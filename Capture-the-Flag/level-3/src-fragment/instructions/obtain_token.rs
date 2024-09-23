use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{mint_to, MintTo, Token2022},
    token_interface::{Mint, TokenAccount},
};

use crate::state::Faction;

pub fn _obtain_faction_token(ctx: Context<ObtainFactionToken>) -> Result<()> {
    let faction_creator_key = ctx.accounts.faction.authority.key();
    let mint_key = ctx.accounts.mint.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"faction",
        faction_creator_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.faction],
    ]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.new_member_token_account.to_account_info(),
            authority: ctx.accounts.faction.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_context, 1)?;
    Ok(())
}

#[derive(Accounts)]
pub struct ObtainFactionToken<'info> {
    pub faction_authority: Signer<'info>,
    #[account(
        seeds = [b"faction",faction_authority.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub faction: Account<'info, Faction>,
    #[account(
        mut,
        mint::decimals = 0,
        mint::authority = faction,
        mint::freeze_authority = faction,
        extensions::metadata_pointer::metadata_address = mint.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub new_member: Signer<'info>,
    #[account(
        init,
        payer = new_member,
        associated_token::authority = new_member,
        associated_token::mint = mint,
    )]
    pub new_member_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
