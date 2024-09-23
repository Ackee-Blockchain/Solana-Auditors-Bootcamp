use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount},
};

use crate::state::Escrow;

pub fn _withdraw(ctx: Context<Withdraw>, passphrase: Vec<u64>) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.verification_program.to_account_info(),
        verification_program::cpi::accounts::Verify {
            guardian_set: ctx.accounts.guardian_set.to_account_info(),
        },
    );

    // We make sure to call the correct Verification Program
    verification_program::cpi::verify_passcode(cpi_context, passphrase)?;

    let seeds = &[
        b"ESCROW_PDA_AUTHORITY".as_ref(),
        &[ctx.bumps.escrow_pda_authority],
    ];

    let amount = ctx.accounts.escrow.amount;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.escrow_pda_authority.to_account_info(),
            },
        )
        .with_signer(&[&seeds[..]]),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,
    #[account(
        init_if_needed,
        payer = recipient,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: correctness of the Address is ensured, no way to check the DISCRIMINATOR
    /// Will be checked in the CPI Program.
    #[account(
        seeds = [
            b"guardian_set",
            recipient.key().as_ref(),
            escrow.index.to_be_bytes().as_ref()
        ],
        seeds::program = verification_program.key(),
        owner = verification_program.key(),
        bump
    )]
    pub guardian_set: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"escrow",escrow.recipient.key().as_ref(),escrow.sender.key().as_ref(),escrow_token_account.key().as_ref()],
        bump = escrow.bump,
        has_one = mint,
        close = recipient,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = escrow_pda_authority,
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: we do not read or write to this account
    #[account(
        seeds = [b"ESCROW_PDA_AUTHORITY"],
        bump
    )]
    pub escrow_pda_authority: AccountInfo<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: this is safe as we make sure that this is checked within the Program
    pub verification_program: AccountInfo<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
