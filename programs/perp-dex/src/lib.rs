pub mod constants;
pub use constants::*;
pub mod error;
pub mod instructions;
pub use instructions::*;
pub mod states;
pub use states::*;

use anchor_lang::prelude::*;

declare_id!("4XBXLs3VgWs9ThtDXT13PxauTxSaGTq9frHvGPAN6TSn");

#[program]
pub mod perp_dex {
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>, perp_fee: u64) -> Result<()> {
        instructions::initialize_state(ctx, perp_fee)
    }
}
