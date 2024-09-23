use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;

use error::Level5Error;
use instructions::*;

use anchor_lang::solana_program::hash::hash;

declare_id!("HoyRuVgqiRHeCJBDSCobmoS8TwnMP3YvMWjKcZfCvMyw");

#[program]
pub mod level_5 {

    use super::*;

    #[access_control(pre_ix_initialize(
        &ctx,
        guardian_set_index,
        signatures_number,
        expiration_time,
        &signatures,
        amount_in,
    ))]
    pub fn initialize(
        ctx: Context<Initialize>,
        guardian_set_index: u32,
        signatures_number: u8,
        expiration_time: i64,
        signatures: Vec<u64>,
        amount_in: u64,
    ) -> Result<()> {
        _initialize(
            ctx,
            guardian_set_index,
            signatures_number,
            expiration_time,
            signatures,
            amount_in,
        )
    }
    #[access_control(pre_ix_withdraw(&ctx,))]
    pub fn withdraw(ctx: Context<Withdraw>, passphrase: Vec<u64>) -> Result<()> {
        _withdraw(ctx, passphrase)
    }
    pub fn obtain_secret(ctx: Context<ObtainSecret>) -> Result<()> {
        let secret = hash(secret.as_bytes()).to_string();
        if secret.ne("9gok9BzW8NxNhTvFEJCaoKsnTw7eS9JQAreiWw1d3fhH") {
            return err!(Level5Error::IncorrectSecrets);
        }
        _obtain_secret(ctx)
    }
}

pub fn pre_ix_initialize(
    ctx: &Context<Initialize>,
    _guardian_set_index: u32,
    signatures_number: u8,
    expiration_time: i64,
    signatures: &[u64],
    amount_in: u64,
) -> Result<()> {
    let sender = &ctx.accounts.sender_token_account;

    require!(
        signatures_number as usize == signatures.len(),
        Level5Error::LengthsDoNotCorrespond
    );

    require!(sender.amount >= amount_in, Level5Error::NotEnoughFunds);

    require!(expiration_time >= 0, Level5Error::PastNotAllowed);

    Ok(())
}

pub fn pre_ix_withdraw(ctx: &Context<Withdraw>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    match escrow.expiration_time {
        0 => {}
        _ => {
            let now = Clock::get()?;
            let expiration = escrow.expiration_time;

            require!(
                expiration >= now.unix_timestamp,
                Level5Error::WithdrawWindowPassed
            );
        }
    };
    Ok(())
}
