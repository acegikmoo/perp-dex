use anchor_lang::prelude::*;

mod states;
use states::*;
mod instructions;
use instructions::*;
mod error;
use error::*;
mod utils;
use utils::*;

declare_id!("QtgyU3YMAjsgbrZuqhNEkgyqyjuqnunSbqbSiZdrLsn");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>, perp_fee: u64) -> Result<()> {
        instructions::initialize_state(ctx, perp_fee)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, account_id: u16) -> Result<()> {
        handle_initialize_user(ctx, account_id)
    }

    pub fn initialize_perp_market(
        ctx: Context<InitializePerpMarket>,
        params: InitializeMarketParams,
    ) -> Result<()> {
        handle_initialize_perp_market(ctx, params)
    }

    pub fn deposit(ctx: Context<Deposit>, market_index: u16, amount: u64) -> Result<()> {
        handle_deposit(ctx, market_index, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, market_index: u16, amount: u64) -> Result<()> {
        handle_withdraw(ctx, market_index, amount)
    }

    pub fn initialize_oracle(
        ctx: Context<InitializeOracle>,
        market_index: u16,
        initial_price: u64,
        confidence_interval: u64,
        max_price_deviation: u64,
    ) -> Result<()> {
        handle_initialize_oracle(
            ctx,
            market_index,
            initial_price,
            confidence_interval,
            max_price_deviation,
        )
    }

    pub fn update_oracle_price(
        ctx: Context<UpdateOraclePrice>,
        market_index: u16,
        new_price: u64,
    ) -> Result<()> {
        handle_update_oracle_price(ctx, market_index, new_price)
    }
}
