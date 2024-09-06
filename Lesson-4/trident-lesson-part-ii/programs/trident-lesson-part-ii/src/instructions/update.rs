use anchor_lang::prelude::*;

use crate::{error::ExampleError, state::Asset};

const MAGIC_NUMBER: u8 = 254;

pub fn _update_ix(ctx: Context<UpdateContext>, input1: u8, input2: u8) -> Result<()> {
    let asset = &mut ctx.accounts.asset;

    asset.counter = buggy_math_function(input1, input2)?.into();
    Ok(())
}

pub fn buggy_math_function(input1: u8, input2: u8) -> Result<u8> {
    if input2 >= MAGIC_NUMBER {
        return err!(ExampleError::InvalidInputNumber);
    }
    let divisor = MAGIC_NUMBER.checked_sub(input2).unwrap();
    Ok(input1.checked_div(divisor).unwrap())
}

#[derive(Accounts)]
pub struct UpdateContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"asset",signer.key().as_ref()],
        bump
    )]
    pub asset: Account<'info, Asset>,
}
