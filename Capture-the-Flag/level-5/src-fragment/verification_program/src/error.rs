use anchor_lang::prelude::*;

#[error_code]
pub enum Level5Error {
    #[msg("Number of signatures needs to be the same as Vector Length!")]
    LengthsDoNotCorrespond,
    #[msg("Not enough keys!")]
    NotEnoughKeys,
    #[msg("Verification Failed!")]
    VerificationFailed,
    #[msg("Expiration Time cannot be in the past!")]
    PastNotAllowed,
}
