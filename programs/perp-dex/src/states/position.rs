use crate::{error::ErrorCode, PositionDirection};
use anchor_lang::prelude::*;
use std::usize;

#[account]
#[derive(InitSpace)]
pub struct PerpPosition {
    pub last_cumulative_funding_rate: i64,
    pub market_index: u64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    pub open_orders: u8,
    pub pnl: i64,
    pub bids: u64,
    pub asks: u64,
    pub collateral: u64,
}

impl PerpPosition {
    pub fn is_available(&self) -> bool {
        self.open_orders == 0
    }
}

impl Default for PerpPosition {
    fn default() -> Self {
        Self {
            last_cumulative_funding_rate: 0,
            market_index: 0,
            base_asset_amount: 0,
            quote_asset_amount: 0,
            open_orders: 0,
            pnl: 0,
            bids: 0,
            asks: 0,
            collateral: 0,
        }
    }
}

pub type PerpPositions = [PerpPosition; 8];

pub fn add_new_position(positions: &mut PerpPositions, market_index: u16) -> Result<usize> {
    let idx = positions
        .iter()
        .position(|p| p.is_available())
        .ok_or(ErrorCode::MaxNumberOfPositions)?;

    positions[idx] = PerpPosition {
        market_index: market_index as u64,
        ..Default::default()
    };
    Ok(idx)
}

pub fn get_position_index(positions: &PerpPositions, market_index: u16) -> Result<usize> {
    positions
        .iter()
        .position(|p| p.market_index == market_index as u64)
        .ok_or(ErrorCode::UserHasNoPositionInMarket.into())
}

pub fn get_forced_position_from_market_index(
    positions: &mut PerpPositions,
    market_index: u16,
) -> Result<usize> {
    match positions
        .iter()
        .position(|p| p.market_index == market_index as u64)
    {
        Some(i) => Ok(i),
        None => add_new_position(positions, market_index),
    }
}

pub fn update_bids_and_asks(
    pos: &mut PerpPosition,
    dir: PositionDirection,
    amount: u64,
) -> Result<()> {
    match dir {
        PositionDirection::Long => pos.bids += amount,
        PositionDirection::Short => pos.asks += amount,
    }
    Ok(())
}
