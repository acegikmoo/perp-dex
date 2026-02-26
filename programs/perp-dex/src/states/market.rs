use crate::states::Amm;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PerpMarket {
    pub market_index: u64,
    pub authority: Pubkey,
    pub liquidator_fee: u64,
    pub max_leverage: u64,
    pub margin_ratio_initial: u64,
    pub margin_ratio_maintainance: u64,
    pub amm: Amm,
    pub bump: u8,
}
