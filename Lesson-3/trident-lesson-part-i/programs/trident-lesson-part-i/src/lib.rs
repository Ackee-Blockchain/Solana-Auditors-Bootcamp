use anchor_lang::prelude::*;

declare_id!("6p3iZzv4Q7GHq1AQAHpgEjj5HxBtif5mtxKYsmFWv57M");

mod instructions;

use instructions::*;
pub mod state;

#[program]
pub mod trident_lesson_part_i {
    use super::*;

    pub fn initialize_ix(
        ctx: Context<InitializeContext>,
        input1: u8,
        input2: u8,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        _initialize_ix(ctx, input1, input2, name, symbol, uri)
    }
}
