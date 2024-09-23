use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::error::Level5Error;
use crate::state::GuardianSet;

pub fn _verify_passcode(ctx: Context<Verify>, passphrase: Vec<u64>) -> Result<()> {
    let guardian_set = &ctx.accounts.guardian_set;

    require!(
        guardian_set.guardian_signatures.len() == passphrase.len(),
        Level5Error::VerificationFailed
    );

    for (reference, x) in guardian_set
        .guardian_signatures
        .iter()
        .zip(passphrase.iter())
    {
        let secret = hash(&x.to_be_bytes()).to_bytes();

        require!(reference.eq(&secret), Level5Error::VerificationFailed);
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(
        seeds = [b"guardian_set",guardian_set.receiver.key().as_ref(),guardian_set.index.to_be_bytes().as_ref()],
        bump
    )]
    pub guardian_set: Account<'info, GuardianSet>,
}
