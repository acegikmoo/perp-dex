use crate::error::ErrorCode;
use crate::states::{Amm, User};
use crate::states::{Order, PerpMarket};
use crate::utils::OrderStatus;
use crate::utils::{FullfillmentMethod, PositionDirection};
use anchor_lang::prelude::*;
use std::cmp::min;
use std::collections::BTreeMap;

pub fn get_types_of_filling(
    order: &Order,
    maker_id_index_price: Vec<(Pubkey, usize, u64)>,
    amm: &Amm,
    limit_price: Option<u64>,
) -> Result<Vec<FullfillmentMethod>> {
    let mut fills = Vec::with_capacity(8);
    let maker_direction = order.opposite();

    let mut amm_price = match maker_direction {
        PositionDirection::Long => amm.get_bid_price(),
        PositionDirection::Short => amm.get_bid_price(),
    };

    for (maker_key, maker_idx, maker_price) in maker_id_index_price {
        let taker_crosses = match limit_price {
            Some(p) => does_order_cross(&maker_direction, maker_price, p),
            None => true,
        };

        if !taker_crosses {
            continue;
        }

        let maker_beats_amm = match maker_direction {
            PositionDirection::Long => maker_price < amm_price,
            PositionDirection::Short => maker_price > amm_price,
        };

        if !maker_beats_amm {
            fills.push(FullfillmentMethod::AMM(Some(maker_price)));
            amm_price = maker_price;
        }
        fills.push(FullfillmentMethod::Match(
            maker_key,
            maker_idx as u16,
            maker_price,
        ));

        if fills.len() >= 6 {
            break;
        }
    }
    let amm_crosses = match limit_price {
        Some(p) => does_order_cross(&maker_direction, amm_price, p),
        None => true,
    };
    if amm_crosses {
        fills.push(FullfillmentMethod::AMM(None));
    }
    Ok(fills)
}

// crossing
pub fn does_order_cross(
    maker_direction: &PositionDirection,
    maker_price: u64,
    limit_price: u64,
) -> bool {
    match maker_direction {
        PositionDirection::Long => limit_price > maker_price,
        PositionDirection::Short => limit_price < maker_price,
    }
}

// match
pub fn fill_with_match(
    taker: &mut User,
    taker_idx: usize,
    taker_limit: Option<u64>,
    maker: &mut User,
    maker_idx: usize,
    maker_price: u64,
    fill_map: &mut BTreeMap<Pubkey, i64>,
) -> Result<(u64, u64)> {
    require!(
        taker.orders[taker_idx].opposite() == maker.orders[maker_idx].direction,
        ErrorCode::InvalidDirection
    );
    let taker_limit = taker_limit.unwrap_or(maker_price);
    let maker_dir = maker.orders[maker_idx].direction;

    if !does_order_cross(&maker_dir, maker_price, taker_limit) {
        return Ok((0, 0));
    }

    let taker_unfilled = taker.orders[taker_idx].get_unfilled_base()?;
    let maker_unfilled = maker.orders[maker_idx].get_unfilled_base()?;
    let (base_filled, quote_filled) =
        calculate_fill_by_match(maker_unfilled, maker_price, taker_unfilled)?;

    if base_filled > 0 {
        update_maker_fills_map(fill_map, &maker.authority.key(), maker_dir, base_filled)?;
    }
    update_order_after_filling(&mut maker.orders[maker_idx], base_filled, quote_filled)?;
    update_order_after_filling(&mut taker.orders[taker_idx], base_filled, quote_filled)?;
    Ok((base_filled, quote_filled))
}

pub fn fill_with_amm(
    user: &mut User,
    order_idx: usize,
    limit_price: Option<u64>,
    market: &mut PerpMarket,
) -> Result<(u64, u64)> {
    let base = user.orders[order_idx].base_asset_amount;
    let quote = match limit_price {
        Some(p) => market.amm.calculate_quote_for_base_with_limit(base, p)?,
        None => market.amm.calculate_quote_for_base_no_limit(base)?,
    };
    if quote == 0 {
        return Ok((0, 0));
    }

    market.amm.execute_trade(base, quote)?;
    update_order_after_filling(&mut user.orders[order_idx], base, quote)?;
    Ok((base, quote))
}

pub fn update_order_after_filling(order: &mut Order, base: u64, quote: u64) -> Result<()> {
    order.base_asset_amount_filled = order
        .base_asset_amount_filled
        .checked_add(base)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    order.quote_asset_amount_filled = order
        .quote_asset_amount_filled
        .checked_add(quote)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    if order.get_unfilled_base()? == 0 {
        order.status = OrderStatus::Filled;
    }
    Ok(())
}

pub fn calculate_fill_by_match(
    maker_base: u64,
    maker_price: u64,
    taker_base: u64,
) -> Result<(u64, u64)> {
    let base = min(maker_base, taker_base);
    let quote = base
        .checked_mul(maker_price)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    Ok((base, quote))
}

fn update_maker_fills_map(
    map: &mut BTreeMap<Pubkey, i64>,
    key: &Pubkey,
    dir: PositionDirection,
    fill: u64,
) -> Result<()> {
    let signed = match dir {
        PositionDirection::Long => fill as i64,
        PositionDirection::Short => -(fill as i64),
    };
    if let Some(v) = map.get_mut(key) {
        *v = v.checked_add(signed).ok_or(ErrorCode::ArithmeticOverflow)?;
    } else {
        map.insert(*key, signed);
    }
    Ok(())
}
