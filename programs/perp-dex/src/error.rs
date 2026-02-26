use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic Overlow")]
    ArithmeticOverflow,
    #[msg("Maximum number of Positions")]
    MaxNumberOfPositions,
    #[msg("User has no position in market")]
    UserHasNoPositionInMarket,
    #[msg("Invalid Market Index")]
    InvalidMarketIndex,
    #[msg("Invalid Leverage")]
    InvalidLeverage,
    #[msg("Invalid Amount")]
    InvalidAmount,
}
