use anchor_lang::prelude::*;

declare_id!("cKSu3X7yQgvJMYCwimDN6NVkQUisgynx4EzujEoj2iB");

#[program]
pub mod anchor_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
