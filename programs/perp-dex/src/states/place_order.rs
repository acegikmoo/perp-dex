use crate::error::ErrorCode;
use crate::states::order::Order;
use crate::states::perp_market_map::PerpMarketMap;
use crate::states::position::*;
use crate::states::state::State;
use crate::states::user::User;
use crate::utils::constants::*;
use crate::utils::constraints::{can_sign_for_user, OrderParams, OrderStatus};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub state: Account<'info, State>,
    #[account(mut, constraint = can_sign_for_user(&user, &authority)?)]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

pub fn handle_place_order(ctx: Context<PlaceOrder>, params: OrderParams) -> Result<()> {
    let map_account = &ctx.remaining_accounts[0];
    let perp_market_map: PerpMarketMap = PerpMarketMap::try_from_slice(&map_account.data.borrow())?;
    place_order(
        params,
        &perp_market_map,
        &ctx.accounts.state,
        &mut ctx.accounts.user,
    )
}

pub fn place_order(
    params: OrderParams,
    perp_market_map: &PerpMarketMap,
    _state: &State,
    user: &mut User,
) -> Result<()> {
    require!(
        params.base_asset_amount >= MIN_ORDER_AMOUNT,
        ErrorCode::InvalidAmount
    );
    require!(
        params.leverage >= MIN_LEVERAGE && params.leverage <= MAX_LEVERAGE,
        ErrorCode::InvalidLeverage
    );
    require!(params.price > 0, ErrorCode::InvalidPrice);
    require!(
        params.market_index <= MAX_MARKET_INDEX,
        ErrorCode::InvalidMarketIndex
    );

    // validate market exists
    let _market = perp_market_map
        .get_ref(params.market_index)
        .ok_or(ErrorCode::InvalidMarketIndex)?;

    let slot = user
        .orders
        .iter()
        .position(|o| o.is_available())
        .ok_or(ErrorCode::MaxNumberOfOrders)?;

    let pos_idx = get_position_index(&user.perp_positions, params.market_index)
        .or_else(|_| add_new_position(&mut user.perp_positions, params.market_index))?;

    let order_id = user.next_order_id;

    user.orders[slot] = Order {
        market_index: params.market_index,
        order_index: slot as u64,
        base_asset_amount: params.base_asset_amount,
        base_asset_amount_filled: 0,
        quote_asset_amount_filled: 0,
        price: Some(params.price),
        direction: params.direction,
        order_type: params.order_type,
        leverage: params.leverage,
        status: OrderStatus::Open,
        order_id,
    };

    user.perp_positions[pos_idx].open_orders += 1;
    user.next_order_id += 1;
    user.open_orders += 1;
    update_bids_and_asks(
        &mut user.perp_positions[pos_idx],
        params.direction,
        params.base_asset_amount,
    )?;
    Ok(())
}
