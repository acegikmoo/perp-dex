use crate::error::ErrorCode;
use crate::states::{Order, PerpMarketMap, State, User, UserMap};
use crate::utils::constraints::can_sign_for_user;
use crate::utils::{OrderType, PositionDirection};
use crate::{
    fill_with_amm, fill_with_match, get_types_of_filling, FullfillmentMethod, OrderStatus,
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct FillOrder<'info> {
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
    #[account(mut, constraint = can_sign_for_user(&filler, &authority)?)]
    pub filler: Account<'info, User>,
    #[account(mut)]
    pub user: Account<'info, User>,
}

pub fn handle_fill_order(ctx: Context<FillOrder>, order_id: Option<u64>) -> Result<()> {
    let (order_id, market_index) = {
        let user = &ctx.accounts.user;
        let id = order_id.unwrap_or_else(|| user.get_last_order_id());
        let mi = user
            .get_order(id)
            .ok_or(ErrorCode::OrderNotFound)?
            .market_index;
        (id, mi)
    };

    let market_map_acc = &ctx.remaining_accounts[0];
    let mut perp_market_map = PerpMarketMap::try_from_slice(&market_map_acc.data.borrow())?;
    let user_map_acc = &ctx.remaining_accounts[1];
    let mut user_map: UserMap = UserMap::try_from_slice(&user_map_acc.data.borrow())?;

    execute_fill(
        &mut ctx.accounts.user,
        &mut ctx.accounts.filler,
        &mut perp_market_map,
        &mut user_map,
        order_id,
        market_index,
    )?;
    Ok(())
}

fn execute_fill(
    user: &mut User,
    _filler: &mut Account<User>,
    perp_market_map: &mut PerpMarketMap,
    maker_map: &mut UserMap,
    order_id: u64,
    market_index: u16,
) -> Result<(u64, u64)> {
    let taker_key = user.authority;

    let order_idx = user
        .orders
        .iter()
        .position(|o| o.order_id == order_id && o.status == OrderStatus::Open)
        .ok_or(ErrorCode::OrderNotFound)?;

    // collect candidate makers
    let makers = collect_makers(
        perp_market_map,
        maker_map,
        &taker_key,
        &user.orders[order_idx],
    )?;

    let limit_price = user.orders[order_idx].price;
    let market = perp_market_map
        .get_ref(market_index)
        .ok_or(ErrorCode::InvalidMarketIndex)?;
    let fill_types =
        get_types_of_filling(&user.orders[order_idx], makers, &market.amm, limit_price)?;

    if fill_types.is_empty() {
        return Ok((0, 0));
    }

    let mut total_base = 0u64;
    let mut total_quote = 0u64;
    let mut maker_fill_map: BTreeMap<Pubkey, i64> = BTreeMap::new();

    for fill_type in &fill_types {
        let mut market = perp_market_map
            .get_mut(market_index)
            .ok_or(ErrorCode::InvalidMarketIndex)?;
        let (b, q) = match fill_type {
            FullfillmentMethod::AMM(lp) => fill_with_amm(user, order_idx, *lp, market)?,
            FullfillmentMethod::Match(mk, mi, mp) => {
                let maker = maker_map.0.get_mut(mk).ok_or(ErrorCode::InvalidMakerKey)?;
                fill_with_match(
                    user,
                    order_idx,
                    limit_price,
                    maker,
                    *mi as usize,
                    *mp,
                    &mut maker_fill_map,
                )?
            }
        };
        total_base = total_base
            .checked_add(b)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        total_quote = total_quote
            .checked_add(q)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }
    Ok((total_base, total_quote))
}

fn collect_makers(
    perp_market_map: &mut PerpMarketMap,
    maker_map: &UserMap,
    taker_key: &Pubkey,
    taker_order: &Order,
) -> Result<Vec<(Pubkey, usize, u64)>> {
    let maker_dir = taker_order.opposite();
    let mut result: Vec<(Pubkey, usize, u64)> = Vec::with_capacity(8);

    for (mk, maker) in maker_map.0.iter() {
        if mk == taker_key {
            continue;
        }
        for (i, order) in maker.orders.iter().enumerate() {
            if order.status != OrderStatus::Open
                || order.market_index != taker_order.market_index
                || order.direction != maker_dir
                || order.order_type != OrderType::Limit
            {
                continue;
            }

            let price = order.price.unwrap_or(0);
            if let Some(tp) = taker_order.price {
                if price > tp {
                    continue;
                } // maker ask too high for taker bid
            }
            // sorted insert by best price for direction
            let insert_at = match result.binary_search_by(|item| match maker_dir {
                PositionDirection::Short => item.2.cmp(&price),
                PositionDirection::Long => price.cmp(&item.2),
            }) {
                Ok(i) | Err(i) => i,
            };
            if insert_at < result.capacity() {
                result.insert(insert_at, (*mk, i, price));
            }
        }
    }
    Ok(result)
}
