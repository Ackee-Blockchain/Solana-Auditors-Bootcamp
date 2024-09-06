use anchor_lang::prelude::*;

declare_id!("3np32v6MbFMhTF9bq8A4bx2yi7cFUGByDW4YzB6jyDVU");

mod instructions;

use instructions::*;
pub mod error;
pub mod state;

#[program]
pub mod trident_lesson_part_ii {
    use super::*;

    pub fn initialize_ix(ctx: Context<InitializeContext>) -> Result<()> {
        _initialize_ix(ctx)
    }
    pub fn update_ix(ctx: Context<UpdateContext>, input1: u8, input2: u8) -> Result<()> {
        _update_ix(ctx, input1, input2)
    }
    pub fn dummy_ix(ctx: Context<DummyContext>) -> Result<()> {
        _dummy_ix(ctx)
    }
}
