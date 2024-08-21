use anchor_lang::prelude::*;

use crate::{
    error::RustTestsError,
    state::{ReadEvent, Vault},
};

pub fn _read(ctx: Context<Read>) -> Result<()> {
    let vault = &ctx.accounts.vault;

    let time = Clock::get().unwrap();

    require!(
        time.unix_timestamp >= vault.open_time,
        RustTestsError::NotOpenedYet
    );

    emit!(ReadEvent {
        vault_content: vault.vault_content,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Read<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"vault",signer.key().as_ref(),vault.mint.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
}
