use anchor_lang::prelude::*;

#[error_code]
pub enum Level3Error {
    #[msg("You need to obtain secret from the previous level first!")]
    IncorrectSecret,
}
