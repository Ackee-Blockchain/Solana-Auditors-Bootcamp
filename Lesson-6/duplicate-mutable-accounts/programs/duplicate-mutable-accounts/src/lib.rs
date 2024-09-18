use anchor_lang::prelude::*;

declare_id!("HK5TJtQq6Jr2t173yCGETEwSGfbFG27Un2CYi6LoY9gK");

const FEE_BPS: u64 = 1200; // Fee basis points (1200 BPS = 12%)
const BPS: u64 = 10000; // Basis points in a percent (10000 BPS = 100%)

#[program]
pub mod duplicate_mutable_accounts {
    use super::*;

    // Initialize a new vault
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.owner = ctx.accounts.creator.key();
        vault.amount = 0;

        Ok(())
    }

    // Initialize a fee vault
    pub fn initialize_fee_vault(ctx: Context<InitializeFeeVault>) -> Result<()> {
        let fee_vault = &mut ctx.accounts.vault;

        fee_vault.owner = ctx.accounts.authority.key();
        fee_vault.amount = 0;

        Ok(())
    }

    // Deposit an amount into the vault
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.amount = vault.amount.checked_add(amount).unwrap();
        Ok(())
    }

    // Insecure atomic trade between two vaults
    pub fn insecure_atomic_trade(ctx: Context<AtomicTrade>, transfer_amount: u64) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        let vault_a = &mut ctx.accounts.vault_a;
        let vault_b = &mut ctx.accounts.vault_b;

        // Issue: No check to ensure vault_a and vault_b are different accounts
        // If they are the same, it will lead to logical errors and unintended state changes

        // Calculate the fee
        let fee = transfer_amount
            .checked_mul(FEE_BPS)
            .unwrap()
            .checked_div(BPS)
            .unwrap();

        // Calculate the amount after deducting the fee
        let fee_deducted = transfer_amount.checked_sub(fee).unwrap();

        msg!("Vault A amount before: {}", vault_a.amount);
        msg!("Vault B amount before: {}", vault_b.amount);
        msg!("Fee Vault amount before: {}", fee_vault.amount);

        // Update the amounts in the respective vaults
        fee_vault.amount = fee_vault.amount.checked_add(fee).unwrap();
        vault_a.amount = vault_a.amount.checked_add(fee_deducted).unwrap();
        vault_b.amount = vault_b.amount.checked_sub(fee_deducted).unwrap();

        msg!("Vault A amount after: {}", vault_a.amount);
        msg!("Vault B amount after: {}", vault_b.amount);
        msg!("Fee Vault amount after: {}", fee_vault.amount);

        Ok(())
    }

    // Secure atomic trade between two vaults
    pub fn secure_atomic_trade(ctx: Context<AtomicTrade>, transfer_amount: u64) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        let vault_a = &mut ctx.accounts.vault_a;
        let vault_b = &mut ctx.accounts.vault_b;

        // Ensure vault_a and vault_b are different accounts
        if vault_a.key() == vault_b.key() {
            return err!(AtomicTradeError::DuplicateVaults);
        }

        // Calculate the fee
        let fee = transfer_amount
            .checked_mul(FEE_BPS)
            .unwrap()
            .checked_div(BPS)
            .unwrap();

        // Calculate the amount after deducting the fee
        let fee_deducted = transfer_amount.checked_sub(fee).unwrap();

        msg!("Vault A amount before: {}", vault_a.amount);
        msg!("Vault B amount before: {}", vault_b.amount);
        msg!("Fee Vault amount before: {}", fee_vault.amount);

        // Update the amounts in the respective vaults
        fee_vault.amount = fee_vault.amount.checked_add(fee).unwrap();
        vault_a.amount = vault_a.amount.checked_add(fee_deducted).unwrap();
        vault_b.amount = vault_b.amount.checked_sub(fee_deducted).unwrap();

        msg!("Vault A amount: {}", vault_a.amount);
        msg!("Vault B amount: {}", vault_b.amount);
        msg!("Fee Vault amount: {}", fee_vault.amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeFeeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::LEN,
        seeds = [b"fee_vault"],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space= 8 + Vault::LEN,
        seeds = [b"vault",creator.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"vault",owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
}

#[derive(Accounts)]
pub struct AtomicTrade<'info> {
    pub signer_a: Signer<'info>,
    pub signer_b: Signer<'info>,
    #[account(
        mut,
        constraint = vault_a.owner == signer_a.key(),
        // This can also resolve the security Issue !!
        // constraint = vault_a.key() != vault_b.key() @ AtomicTradeError::DuplicateVaults,
        seeds = [b"vault",signer_a.key().as_ref()],
        bump
    )]
    pub vault_a: Account<'info, Vault>,
    #[account(
        mut,
        constraint = vault_b.owner == signer_b.key(),
        seeds = [b"vault",signer_b.key().as_ref()],
        bump
    )]
    pub vault_b: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub amount: u64,
}

impl Vault {
    const LEN: usize = 32 + 8;
}

#[error_code]
pub enum AtomicTradeError {
    #[msg("Receiver and Sender address cannot be the same!")]
    DuplicateVaults,
}
