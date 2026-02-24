use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct State {
    pub admin: Pubkey,
    pub no_of_markets: u64,
    pub perp_fee: u64,
    pub no_of_users: u64,
    pub bump: u8,
    pub signer: Pubkey,
    pub signer_bump: u8,
}
