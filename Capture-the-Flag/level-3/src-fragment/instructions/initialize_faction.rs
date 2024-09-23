use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::Mint;

use crate::state::Faction;

pub fn _initialize_faction(
    ctx: Context<InitializeFaction>,
    name: String,
    symbol: String,
) -> Result<()> {
    let faction_creator_key = ctx.accounts.faction_creator.key();
    let mint_key = ctx.accounts.mint.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"faction",
        faction_creator_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.faction],
    ]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_2022_extensions::TokenMetadataInitialize {
            token_program_id: ctx.accounts.token_program.to_account_info(),
            metadata: ctx.accounts.mint.to_account_info(),
            update_authority: ctx.accounts.faction.to_account_info(),
            mint_authority: ctx.accounts.faction.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        },
        signer_seeds,
    );

    anchor_spl::token_2022_extensions::token_metadata_initialize(
        cpi_context,
        name,
        symbol,
        "ackee.xyz".to_string(),
    )?;

    let faction = &mut ctx.accounts.faction;

    faction.authority = ctx.accounts.faction_creator.key();
    faction.mint = ctx.accounts.mint.key();
    faction.members_count = 0;
    faction.bump = ctx.bumps.faction;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeFaction<'info> {
    #[account(mut)]
    pub faction_creator: Signer<'info>,
    #[account(
        init,
        space = 8 + Faction::LEN,
        payer = faction_creator,
        seeds = [b"faction",faction_creator.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub faction: Account<'info, Faction>,
    #[account(
        init,
        payer = faction_creator,
        mint::decimals = 0,
        mint::authority = faction,
        mint::freeze_authority = faction,
        extensions::metadata_pointer::metadata_address = mint.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}
