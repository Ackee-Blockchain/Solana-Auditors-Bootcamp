use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use error::AtlantisError;

declare_id!("D51vhx6jAbBtQVwo1fcYr7RMKQKAAnSUy6v7vRCHCZL3");

mod error;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod level_4 {
    use super::*;

    pub fn init_vesting(
        ctx: Context<InitVesting>,
        secret: String,
        recipient: Pubkey,
        amount: u64,
        start_at: u64,
        end_at: u64,
        interval: u64,
    ) -> Result<()> {
        let secrets = hash(secret.as_bytes()).to_string();
        if secrets.ne("2X6mirNNEJLEU9kSiH2nxLxikmhbyzVr8PWwsY1QKAYE") {
            return err!(AtlantisError::IncorrectSecrets);
        }
        _init_vesting(ctx, recipient, amount, start_at, end_at, interval)
    }
    pub fn withdraw_unlocked(ctx: Context<WithdrawUnlocked>) -> Result<()> {
        _withdraw_unlocked(ctx)
    }
    pub fn reveal_secret(ctx: Context<RevealSecret>) -> Result<()> {
        _reveal_secret(ctx)
    }
}
