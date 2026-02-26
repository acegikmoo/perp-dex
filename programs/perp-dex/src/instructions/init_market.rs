use crate::states::*;
use crate::utils::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

use crate::utils::InitializeMarketParams;
use anchor_spl::token::{Mint, Token, TokenAccount};


#[derive(Accounts)]
pub struct InitializePerpMarket<'info> {
    #[account(
        init, 
        payer = admin,
        space = 8 + PerpMarket::INIT_SPACE,
        seeds = [b"perp_market", state.no_of_markets.to_le_bytes().as_ref()],
        bump
    )]
    pub market: Account<'info, PerpMarket>,

    pub perp_market_mint: Account<'info, Mint>,

    #[account(
        init, payer = admin,
        seeds = [b"perp_market_vault", state.no_of_markets.to_le_bytes().as_ref()],
        bump,
        token::mint = perp_market_mint,
        token::authority = drift_signer,
    )]
    pub perp_market_vault: Account<'info, TokenAccount>,

    #[account(mut, constraint = drift_signer.key() == state.signer.key())]
    /// CHECK: validate by state.signer constraint
    pub drift_signer: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub state: Account<'info, State>,

    /// CHECK: stored in market.amm.oracle
    pub oracle: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_initialize_perp_market(
    ctx: Context<InitializePerpMarket>,
    params: InitializeMarketParams,
) -> Result<()> {
    require!(params.market_index == ctx.accounts.state.no_of_markets, ErrorCode::InvalidMarketIndex);
    require!(params.max_leverage <= MAX_LEVERAGE, ErrorCode::InvalidLeverage);
    require!(params.base_asset_reserve > 0, ErrorCode::InvalidAmount);
    require!(params.quote_asset_reserve > 0, ErrorCode::InvalidAmount);

    let clock = Clock::get()?;
    let market = &mut ctx.accounts.market;
    market.market_index = params.market_index;
    market.authority = ctx.accounts.admin.key();
    market.liquidator_fee = params.liquidator_fee;
    market.max_leverage = params.max_leverage;
    market.margin_ratio_initial = params.margin_ratio_initial;
    market.margin_ratio_maintainance = params.margin_ratio_maintainance;
    market.amm = Amm {
        oracle: ctx.accounts.oracle.key(),
        base_asset_reserve: params.base_asset_reserve,
        quote_asset_reserve: params.quote_asset_reserve,
        last_funding_rate: 0,
        last_funding_rate_ts: clock.unix_timestamp,
        amm_price: params.quote_asset_reserve / params.base_asset_reserve,
        k: params.base_asset_reserve.checked_mul(params.quote_asset_reserve).ok_or(ErrorCode::ArithmeticOverflow)?,
        oracle_price_weight: 5000,
        last_oracle_update: clock.unix_timestamp,
    };
    market.bump = ctx.bumps.market;
    ctx.accounts.state.no_of_markets += 1;
    Ok(())
}
