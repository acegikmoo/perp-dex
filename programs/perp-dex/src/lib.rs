use anchor_lang::prelude::*;

mod states;
use states::*;
mod instructions;
use instructions::*;
mod error;
use error::*;
mod utils;
use utils::*;

declare_id!("4XBXLs3VgWs9ThtDXT13PxauTxSaGTq9frHvGPAN6TSn");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>, perp_fee: u64) -> Result<()> {
        instructions::initialize_state(ctx, perp_fee)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, account_id: u16) -> Result<()> {
        handle_initialize_user(ctx, account_id)
    }
}
