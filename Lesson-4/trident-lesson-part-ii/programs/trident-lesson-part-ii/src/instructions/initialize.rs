use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token_interface::TokenInterface;

use crate::state::{Asset, Config};

pub fn _initialize_ix(ctx: Context<InitializeContext>) -> Result<()> {
    let asset = &mut ctx.accounts.asset;

    asset.authority = ctx.accounts.signer.key();
    asset.mint = ctx.accounts.mint.key();
    asset.counter = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub config_authority: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Asset::LEN,
        seeds = [b"asset",signer.key().as_ref()],
        bump
    )]
    pub asset: Account<'info, Asset>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [b"config",config_authority.key().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
