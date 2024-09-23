use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::state::Escrow;

pub fn _initialize(
    ctx: Context<Initialize>,
    guardian_set_index: u32,
    signatures_number: u8,
    expiration_time: i64,
    signatures: Vec<u64>,
    amount_in: u64,
) -> Result<()> {
    let expiration_time_ = match expiration_time {
        0 => 0,
        _ => {
            let now = Clock::get()?;

            now.unix_timestamp
                .checked_add(expiration_time)
                .unwrap_or(i64::MAX)
        }
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.verification_program.to_account_info(),
        verification_program::cpi::accounts::Initialize {
            sender: ctx.accounts.sender.to_account_info(),
            recipient: ctx.accounts.recipient.to_account_info(),
            guardian_set: ctx.accounts.guardian_set.to_account_info(),
            escrow: ctx.accounts.escrow.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
    );

    verification_program::cpi::initialize(
        cpi_context,
        guardian_set_index,
        signatures_number,
        signatures,
    )?;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.sender_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        amount_in,
        ctx.accounts.mint.decimals,
    )?;

    ctx.accounts.escrow.set_inner(Escrow {
        sender: ctx.accounts.sender.key(),
        recipient: ctx.accounts.recipient.key(),
        index: guardian_set_index,
        mint: ctx.accounts.mint.key(),
        amount: amount_in,
        bump: ctx.bumps.escrow,
        expiration_time: expiration_time_,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(guardian_set_index: u32)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is alright as recipient is selected by the sender
    pub recipient: AccountInfo<'info>,
    /// CHECK: This is alright as we will Initialize this in the Instruction
    #[account(
        mut,
        seeds = [
            b"guardian_set",
            recipient.key().as_ref(),
            guardian_set_index.to_be_bytes().as_ref()
        ],
        seeds::program = verification_program.key(),
        bump
    )]
    pub guardian_set: AccountInfo<'info>,
    #[account(
        init,
        payer = sender,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow",recipient.key().as_ref(),sender.key().as_ref(),escrow_token_account.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = sender,
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
    #[account(
        mut,
        token::authority = sender,
        token::mint = mint
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    /// CHECK: this is safe as we make sure that this is checked within the Program
    pub verification_program: AccountInfo<'info>,
}
