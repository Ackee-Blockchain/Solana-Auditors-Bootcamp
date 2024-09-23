use anchor_lang::prelude::*;

use crate::state::Explorer;

// Initializes a new explorer
pub fn _init_explorer(ctx: Context<InitExplorer>) -> Result<()> {
    let explorer_account = &mut ctx.accounts.explorer_account;
    explorer_account.health = 100;
    explorer_account.experience = 0;
    explorer_account.mana = 100;
    explorer_account.monsters_defeated = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitExplorer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Explorer::LEN,
        seeds = [b"EXPLORER", user.key().as_ref()],
        bump
    )]
    pub explorer_account: Account<'info, Explorer>,

    pub system_program: Program<'info, System>,
}
