use anchor_lang::prelude::*;

#[error_code]
pub enum Level5Error {
    #[msg("Withdraw window passed!")]
    WithdrawWindowPassed,
    #[msg("Number of signatures needs to be the same as Vector Length!")]
    LengthsDoNotCorrespond,
    #[msg("Not enough funds!")]
    NotEnoughFunds,
    #[msg("Expiration Time cannot be in the past!")]
    PastNotAllowed,
    #[msg("You need to obtain secrets from the previous level first!")]
    IncorrectSecrets,
}
