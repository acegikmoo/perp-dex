use anchor_lang::prelude::*;

#[constant]
// Maximum allowed user leverage
pub const MAX_LEVERAGE: u64 = 100;

// Minimum permitted leverage (1x = fully collateralized)
pub const MIN_LEVERAGE: u64 = 1;

// Default leverage applied if unspecified
pub const DEFAULT_LEVERAGE: u64 = 1;

// Minimum base asset size per order
pub const MIN_ORDER_AMOUNT: u64 = 1;

// Cap on concurrent open orders per user
pub const MAX_ORDERS_PER_USER: usize = 16;

// Cap on active market positions per user
pub const MAX_POSITIONS_PER_USER: usize = 8;

// Upper bound on supported market index
pub const MAX_MARKET_INDEX: u16 = 255;
