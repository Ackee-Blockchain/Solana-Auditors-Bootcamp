use anchor_lang::prelude::*;

#[error_code]
pub enum RustTestsError {
    #[msg("Provided incorrect input")]
    IncorrectInput,
    #[msg("Vault Not opened Yet!")]
    NotOpenedYet,
}
