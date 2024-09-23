use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::error::Level5Error;
use crate::state::GuardianSet;

pub fn _initialize(
    ctx: Context<Initialize>,
    guardian_set_index: u32,
    _signatures_number: u8,
    signatures: Vec<u64>,
) -> Result<()> {
    require!(signatures.len() >= 4, Level5Error::NotEnoughKeys);

    let mut signatures_hashed: Vec<[u8; 32]> = vec![];

    for x in signatures.iter() {
        let sig = hash(&x.to_be_bytes()).to_bytes();
        signatures_hashed.push(sig);
    }

    ctx.accounts.guardian_set.set_inner(GuardianSet {
        index: guardian_set_index,
        escrow: ctx.accounts.escrow.key(),
        guardian_signatures: signatures_hashed,
        bump: ctx.bumps.guardian_set,
        sender: ctx.accounts.sender.key(),
        receiver: ctx.accounts.recipient.key(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(guardian_set_index: u32,signatures_number: u8,)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK:
    pub recipient: AccountInfo<'info>,
    #[account(
        init,
        payer = sender,
        space = 8 + GuardianSet::compute_size(usize::from(signatures_number)),
        seeds = [b"guardian_set",recipient.key().as_ref(),guardian_set_index.to_be_bytes().as_ref()],
        bump
    )]
    pub guardian_set: Account<'info, GuardianSet>,
    /// CHECK: This is ok as this goes from the CPI
    pub escrow: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
