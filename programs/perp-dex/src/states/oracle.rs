use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Oracle {
    pub market_index: u16,
    pub authority: Pubkey,
    pub price: u64,
    pub last_update_ts: i64,
    pub confidence_interval: u64,
    pub max_price_deviation: u64,
    pub bump: u8,
}

impl Oracle {
    pub fn update_price(&mut self, new_price: u64, authority: &Pubkey) -> Result<()> {
        require!(self.authority == *authority, ErrorCode::Unauthorized);
        let deviation = self.calculate_price_deviation(new_price);
        require!(
            deviation <= self.max_price_deviation,
            ErrorCode::PriceDeviationTooHigh
        );
        self.price = new_price;
        self.last_update_ts = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn calculate_price_deviation(&self, new_price: u64) -> u64 {
        if self.price == 0 {
            return 0;
        }
        let deviation = if new_price > self.price {
            new_price - self.price
        } else {
            self.price - new_price
        };
        (deviation * 10000) / self.price
    }

    pub fn is_price_stale(&self) -> bool {
        let ts = Clock::get().map(|c| c.unix_timestamp).unwrap_or(0);
        ts - self.last_update_ts > self.confidence_interval as i64
    }

    pub fn get_price(&self) -> Result<u64> {
        require!(!self.is_price_stale(), ErrorCode::StaleOraclePrice);
        Ok(self.price)
    }
}
