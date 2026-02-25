use crate::error::ErrorCode;
use crate::utils::constraints::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub market_index: u16,
    pub order_index: u64,
    pub base_asset_amount: u64,
    pub base_asset_amount_filled: u64,
    pub quote_asset_amount_filled: u64,
    pub price: Option<u64>,
    pub direction: PositionDirection,
    pub order_type: OrderType,
    pub leverage: u64,
    pub status: OrderStatus,
    pub order_id: u64,
}

impl Order {
    pub fn is_available(&self) -> bool {
        self.status != OrderStatus::Open
    }

    pub fn opposite(&self) -> PositionDirection {
        match self.direction {
            PositionDirection::Long => PositionDirection::Short,
            PositionDirection::Short => PositionDirection::Long,
        }
    }

    pub fn get_unfilled_base(&self) -> Result<u64> {
        self.base_asset_amount
            .checked_sub(self.base_asset_amount_filled)
            .ok_or(ErrorCode::ArithmeticOverflow.into())
    }
}

impl Default for Order {
    fn default() -> Self {
        Self {
            market_index: 0,
            order_index: 0,
            base_asset_amount: 0,
            base_asset_amount_filled: 0,
            quote_asset_amount_filled: 0,
            price: None,
            direction: PositionDirection::Long,
            order_type: OrderType::Limit,
            leverage: 1,
            status: OrderStatus::Filled,
            order_id: 0,
        }
    }
}
