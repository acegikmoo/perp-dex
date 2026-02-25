use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic Overlow")]
    ArithmeticOverflow,
    #[msg("Maximum number of Positions")]
    MaxNumberOfPositions,
    #[msg("User has no position in market")]
    UserHasNoPositionInMarket,
}
