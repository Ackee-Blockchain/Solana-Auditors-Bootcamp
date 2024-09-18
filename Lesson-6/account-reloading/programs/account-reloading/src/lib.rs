use anchor_lang::prelude::*;
use update_account::Metadata;

declare_id!("FkeyV5F9SVuq6hbMqtxrHqX1MDHrwQ2jsNug1MaBN9jf");

#[program]
pub mod account_reloading {
    use super::*;

    pub fn update_cpi_noreload(ctx: Context<UpdateCPI>, new_input: u8) -> Result<()> {
        msg!(
            "Updated Metadata input - Before: {}",
            &ctx.accounts.metadata.input
        );

        let cpi_context = CpiContext::new(
            ctx.accounts.update_account.to_account_info(),
            update_account::cpi::accounts::Update {
                authority: ctx.accounts.authority.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
            },
        );

        update_account::cpi::update(cpi_context, new_input)?;

        msg!(
            "Updated Metadata input - After: {}",
            &ctx.accounts.metadata.input
        );

        Ok(())
    }

    pub fn update_cpi_reload(ctx: Context<UpdateCPI>, new_input: u8) -> Result<()> {
        msg!(
            "Updated Metadata input - Before: {}",
            &ctx.accounts.metadata.input
        );

        let cpi_context = CpiContext::new(
            ctx.accounts.update_account.to_account_info(),
            update_account::cpi::accounts::Update {
                authority: ctx.accounts.authority.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
            },
        );

        update_account::cpi::update(cpi_context, new_input)?;

        ctx.accounts.metadata.reload()?;

        msg!(
            "Updated Metadata input - After: {}",
            &ctx.accounts.metadata.input
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateCPI<'info> {
    pub authority: Signer<'info>,
    #[account{
        mut,
        seeds = [b"metadata",authority.key().as_ref()],
        seeds::program = update_account::ID,
        bump,
    }]
    pub metadata: Account<'info, Metadata>,
    pub update_account: Program<'info, UpdateAccountProgram>,
}

pub struct UpdateAccountProgram;

impl anchor_lang::Id for UpdateAccountProgram {
    fn id() -> Pubkey {
        update_account::ID
    }
}
