use anchor_lang::prelude::*;

declare_id!("7xFkBW5msPRF1cHcm5LESpUMfsZALh2heLUEDuPPY6RP");

#[program]
pub mod update_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: u8) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        metadata.authority = ctx.accounts.authority.key();
        metadata.input = input;
        metadata.bump = ctx.bumps.metadata;

        Ok(())
    }
    pub fn update(ctx: Context<Update>, input: u8) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        metadata.input = input;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        space = 8 + Metadata::INIT_SPACE,
        payer = authority,
        seeds = [b"metadata",authority.key().as_ref()],
        bump,
    )]
    pub metadata: Account<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"metadata",authority.key().as_ref()],
        bump = metadata.bump,
        has_one = authority,
    )]
    pub metadata: Account<'info, Metadata>,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct Metadata {
    pub authority: Pubkey,
    pub input: u8,
    pub bump: u8,
}
