use anchor_lang::prelude::*;

#[error_code]
pub enum AtlantisError {
    InvalidAmount,
    InvalidTimeRange,
    InvalidInterval,
    Overflow,
    Underflow,
    #[msg("You need to obtain secrets from the previous level first!")]
    IncorrectSecrets,
}
