use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct RevealSecret<'info> {
    pub hacker: Signer<'info>,
    #[account(mut,
        token::mint = mint,
        token::authority = hacker
    )]
    pub hacker_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
}
