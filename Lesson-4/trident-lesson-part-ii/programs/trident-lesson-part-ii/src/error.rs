use anchor_lang::prelude::*;

#[error_code]
pub enum ExampleError {
    #[msg("This number is incorrect")]
    InvalidInputNumber,
}
