use anchor_lang::{prelude::*, solana_program::bpf_loader_upgradeable};

declare_id!("HUUvPjpnpgMhjg4Z3ki9nVzwMhsy4CzipwsV1V8DiRpr");

#[program]
pub mod initialization_frontrunning {
    use super::*;

    pub fn initialize_insecure(
        ctx: Context<InitializeInsecure>,
        additional_data: u8,
    ) -> Result<()> {
        let global_config = &mut ctx.accounts.global_config;

        global_config.authority = ctx.accounts.signer.key();
        global_config.additional_data = additional_data;

        Ok(())
    }

    pub fn initialize_secure(ctx: Context<InitializeSecure>, additional_data: u8) -> Result<()> {
        let global_config = &mut ctx.accounts.global_config;

        global_config.authority = ctx.accounts.signer.key();
        global_config.additional_data = additional_data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeInsecure<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + GlobalConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeSecure<'info> {
    #[account(
        mut,
        constraint = signer.key() == program_data.upgrade_authority_address.unwrap_or_default()
    )]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + GlobalConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
        seeds = [crate::ID.as_ref()],
        bump,
        seeds::program = bpf_loader_upgradeable::id(),
    )]
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub additional_data: u8,
}
