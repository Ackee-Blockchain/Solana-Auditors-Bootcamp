use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

declare_id!("7b4eFt9GjjRnHptynRJNdjwXjjroX2kGY35YfQoXgFNo");

#[program]
pub mod pda_privileges {

    use super::*;

    // Initialize the vault and the metadata account
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let metadata_account = &mut ctx.accounts.metadata_account;
        // Set the creator of the metadata account
        metadata_account.creator = ctx.accounts.vault_creator.key();
        Ok(())
    }
    // Insecure withdrawal from the vault
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>) -> Result<()> {
        // Get the amount to be withdrawn
        let amount = ctx.accounts.vault.amount;
        let metadata_account = &mut ctx.accounts.metadata_account;

        // Define the signer seeds for the PDA (Program Derived Address)
        let signer_seeds: &[&[&[u8]]] = &[&[b"metadata_account", metadata_account.creator.as_ref(), &[ctx.bumps.metadata_account]]];

        // Create the CPI context for the token transfer
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.withdraw_destination.to_account_info(),
                authority: metadata_account.to_account_info(),
            },
            signer_seeds,
        );

        // Perform the token transfer
        transfer(cpi_context, amount)?;
        Ok(())
    }
    // Secure withdrawal from the vault
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>) -> Result<()> {
        // Get the amount to be withdrawn
        let amount = ctx.accounts.vault.amount;
        let metadata_account = &mut ctx.accounts.metadata_account;

        // Define the signer seeds for the PDA (Program Derived Address)
        let signer_seeds: &[&[&[u8]]] = &[&[b"metadata_account", metadata_account.creator.as_ref(), &[ctx.bumps.metadata_account]]];

        // Create the CPI context for the token transfer
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.withdraw_destination.to_account_info(),
                authority: metadata_account.to_account_info(),
            },
            signer_seeds,
        );

        // Perform the token transfer
        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    // The vault creator (signer) of the transaction
    #[account(mut)]
    pub vault_creator: Signer<'info>,
    // The token account representing the vault
    #[account(
        init,
        payer = vault_creator,
        associated_token::mint = mint,
        associated_token::authority = metadata_account,
    )]
    pub vault: Account<'info, TokenAccount>,
    // The metadata account, which stores the creator information
    #[account(
        init,
        payer = vault_creator,
        space = 8 + MetadataAccount::LEN,
        seeds = [b"metadata_account",vault_creator.key().as_ref()],
        bump,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    // The mint account
    pub mint: Account<'info, Mint>,
    // System program account
    pub system_program: Program<'info, System>,
    // Token program account
    pub token_program: Program<'info, Token>,
    // Associated token program account
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // The creator (signer) of the transaction
    pub creator: Signer<'info>,
    // The token account representing the vault
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = metadata_account,
    )]
    pub vault: Account<'info, TokenAccount>,
    // The destination token account for withdrawal
    #[account(
        mut,
        token::mint = mint,
    )]
    pub withdraw_destination: Account<'info, TokenAccount>,
    // The metadata account, which stores the creator information
    #[account(
        seeds = [b"metadata_account",metadata_account.creator.key().as_ref()],
        bump,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    // The mint account
    pub mint: Account<'info, Mint>,
    // Token program account
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // The creator (signer) of the transaction
    pub creator: Signer<'info>,
    // The token account representing the vault
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = metadata_account,
    )]
    pub vault: Account<'info, TokenAccount>,
    // The destination token account for withdrawal
    #[account(
        mut,
        token::mint = mint,
    )]
    pub withdraw_destination: Account<'info, TokenAccount>,
    // The metadata account, which stores the creator information and ensures it matches the creator
    #[account(
        seeds = [b"metadata_account",metadata_account.creator.key().as_ref()],
        bump,
        has_one = creator,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    // The mint account
    pub mint: Account<'info, Mint>,
    // Token program account
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct MetadataAccount {
    pub creator: Pubkey,
}

impl MetadataAccount {
    const LEN: usize = 32;
}
