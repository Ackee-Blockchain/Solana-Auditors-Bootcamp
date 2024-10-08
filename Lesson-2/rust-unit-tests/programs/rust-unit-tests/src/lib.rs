use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::initialize::*;
use instructions::read::*;
use state::MintParameters;

declare_id!("AdKKJTjnZYRbNNtgm3jW7BSZPTC98GfVpcB6K6XCqU8L");

#[program]
pub mod rust_unit_tests {

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
