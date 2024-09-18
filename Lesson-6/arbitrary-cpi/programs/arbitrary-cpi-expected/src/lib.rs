use anchor_lang::prelude::*;

declare_id!("8b1MYaKPkJCewTXtUFbrcBznH4Sy7C5AF7BCw8iHXM3u");

#[program]
pub mod arbitrary_cpi_expected {
    use super::*;

    pub fn initialize_secret(
        ctx: Context<InitializeSecret>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        let secret_info = &mut ctx.accounts.secret_information;

        secret_info.author = ctx.accounts.author.key();
        secret_info.pin1 = pin1;
        secret_info.pin2 = pin2;
        secret_info.pin3 = pin3;
        secret_info.pin4 = pin4;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn verify_pin(
        ctx: Context<VerifyPin>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        let secret = &ctx.accounts.secret_information;
        let signer = &ctx.accounts.author.key();

        if secret.author != *signer {
            return err!(ArbitraryCPIExpectedError::UnprivilegedVerification);
        }

        secret.verify_pin(pin1, pin2, pin3, pin4)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSecret<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author,
        space = 8 + SecretInformation::LEN,
        seeds = [b"secret_info",author.key().as_ref()],
        bump,
    )]
    pub secret_information: Account<'info, SecretInformation>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyPin<'info> {
    pub author: Signer<'info>,
    #[account(
        seeds = [b"secret_info",author.key().as_ref()],
        bump,
    )]
    pub secret_information: Account<'info, SecretInformation>,
}

#[account]
pub struct SecretInformation {
    pub author: Pubkey,
    pub pin1: u8,
    pub pin2: u8,
    pub pin3: u8,
    pub pin4: u8,
}

impl SecretInformation {
    pub const LEN: usize = 32 + 4;

    fn verify_pin(&self, pin1: u8, pin2: u8, pin3: u8, pin4: u8) -> Result<()> {
        if self.pin1 != pin1 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        if self.pin2 != pin2 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        if self.pin3 != pin3 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        if self.pin4 != pin4 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        Ok(())
    }
}

#[error_code]
pub enum ArbitraryCPIExpectedError {
    #[msg("Incorrect PIN")]
    IncorrectPIN,
    #[msg("Unprivileged Verification")]
    UnprivilegedVerification,
}
