use anchor_lang::prelude::*;

mod error;

declare_id!("8LNL4B82BqGkszA4s671Th3orHxwp5eaiveXTr8Ae1s1");

mod instructions;
mod state;

use error::Level5Error;
use instructions::*;

#[program]
pub mod verification_program {

    use super::*;

    #[access_control(pre_ix_initialize(
        &ctx,
        guardian_set_index,
        signatures_number,
        &signatures,
    ))]
    pub fn initialize(
        ctx: Context<Initialize>,
        guardian_set_index: u32,
        signatures_number: u8,
        signatures: Vec<u64>,
    ) -> Result<()> {
        _initialize(ctx, guardian_set_index, signatures_number, signatures)
    }
    pub fn verify_passcode(ctx: Context<Verify>, passphrase: Vec<u64>) -> Result<()> {
        _verify_passcode(ctx, passphrase)
    }
}

pub fn pre_ix_initialize(
    _ctx: &Context<Initialize>,
    _guardian_set_index: u32,
    signatures_number: u8,
    signatures: &[u64],
) -> Result<()> {
    require!(
        signatures_number as usize == signatures.len(),
        Level5Error::LengthsDoNotCorrespond
    );

    Ok(())
}
