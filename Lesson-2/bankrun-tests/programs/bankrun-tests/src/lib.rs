use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::initialize::*;
use instructions::read::*;
use state::MintParameters;

declare_id!("8DrDo7W3ih9CZjDDBzwKhJrfKTKUzXhj6P32ohn3ReG1");

#[program]
pub mod bankrun_tests {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        open_time: i64,
        input: u8,
        mint_parameters: MintParameters,
    ) -> Result<()> {
        _initialize(ctx, open_time, input, mint_parameters)
    }

    pub fn read(ctx: Context<Read>) -> Result<()> {
        _read(ctx)
    }
}
