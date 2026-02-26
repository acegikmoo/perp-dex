use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Amm {
    pub oracle: Pubkey,
    pub base_asset_reserve: u64,
    pub quote_asset_reserve: u64,
    pub last_funding_rate: u64,
    pub last_funding_rate_ts: i64,
    pub amm_price: u64,
    pub k: u64,
    pub oracle_price_weight: u64,
    pub last_oracle_update: i64,
}

impl Amm {
    pub fn get_bid_price(&self) -> u64 {
        self.quote_asset_reserve / self.base_asset_reserve
    }

    pub fn get_ask_price(&self) -> u64 {
        self.base_asset_reserve / self.quote_asset_reserve
    }

    pub fn calculate_quote_for_base_no_limit(&self, base_amount: u64) -> Result<u64> {
        let new_base = self
            .base_asset_reserve
            .checked_sub(base_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        let new_quote = self
            .k
            .checked_div(new_base)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        new_quote
            .checked_sub(self.quote_asset_reserve)
            .ok_or(ErrorCode::ArithmeticOverflow.into())
    }

    pub fn calculate_quote_for_base_with_limit(
        &self,
        base_amount: u64,
        limit_price: u64,
    ) -> Result<u64> {
        if limit_price < self.amm_price {
            return Ok(0);
        }
        self.calculate_quote_for_base_no_limit(base_amount)
    }

    pub fn execute_trade(&mut self, base_amount: u64, quote_amount: u64) -> Result<()> {
        self.base_asset_reserve = self
            .base_asset_reserve
            .checked_sub(base_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        self.quote_asset_reserve = self
            .quote_asset_reserve
            .checked_add(quote_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        self.amm_price = self.quote_asset_reserve / self.base_asset_reserve;
        Ok(())
    }
}
