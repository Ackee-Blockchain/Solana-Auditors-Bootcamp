use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{
        set_authority, spl_token_2022::instruction::AuthorityType, transfer_checked, SetAuthority,
        Token2022, TransferChecked,
    },
    token_interface::{Mint, TokenAccount},
};

use crate::{error::AtlantisError, state::Escrow};

pub fn _init_vesting(
    ctx: Context<InitVesting>,
    recipient: Pubkey,
    amount: u64,
    start_at: u64,
    end_at: u64,
    interval: u64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(amount > 0, AtlantisError::InvalidAmount);
    // Validate timestamps order (overflow check)
    require!(end_at > start_at, AtlantisError::InvalidTimeRange);
    // Validate interval
    require!(end_at - start_at > interval, AtlantisError::InvalidInterval);
    require!(interval > 0, AtlantisError::InvalidInterval);

    escrow.amount = amount;
    escrow.start_time = start_at;
    escrow.end_time = end_at;
    escrow.interval = interval;
    escrow.recipient = recipient;
    escrow.mint = ctx.accounts.mint.key();

    let (escrow_pda_authority, _) =
        Pubkey::find_program_address(&[b"ESCROW_PDA_AUTHORITY"], ctx.program_id);

    // Set escrow's token account authority to the program's PDA
    set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.escrow_token_account.to_account_info(),
                current_authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        AuthorityType::AccountOwner,
        Some(escrow_pda_authority),
    )?;

    // Transfer tokens from sender's token account to escrow's token account
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
        amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitVesting<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut,
        token::authority = sender,
        token::mint = mint
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = sender,
        space = 8 + Escrow::LEN,
     )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        token::mint = mint
        // token account authority will be transfered to program PDA during instruction execution
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
