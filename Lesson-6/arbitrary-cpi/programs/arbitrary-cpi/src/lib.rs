use anchor_lang::prelude::*;
use arbitrary_cpi_expected::cpi::accounts::{InitializeSecret, VerifyPin};
use arbitrary_cpi_expected::SecretInformation;
use program::ArbitraryCpi;

declare_id!("44C7R1FuGPwn559MhjPBCV5sZjibH5NvByF5KPVuGYXn");

#[program]
pub mod arbitrary_cpi {
    use super::*;

    /// Initialize PIN
    /// This function initializes a secret PIN by calling an external CPI (Cross-Program Invocation)
    /// The four parts of the PIN (pin1, pin2, pin3, pin4) are passed as arguments
    pub fn initialize_secret(
        ctx: Context<InitializeSecretCPI>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Get the account information of the external program
        let cpi_program = ctx.accounts.secret_program.to_account_info();

        // Create the CPI accounts context
        let cpi_accouts = InitializeSecret {
            author: ctx.accounts.author.to_account_info(),
            secret_information: ctx.accounts.secret_information.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        // Create a new CPI context
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accouts);

        // Call the external CPI to initialize the secret with the provided PIN
        arbitrary_cpi_expected::cpi::initialize_secret(cpi_ctx, pin1, pin2, pin3, pin4)?;
        msg!("PIN SET");
        Ok(())
    }

    /// Verify PIN (Insecure)
    /// This function verifies a secret PIN by calling an external CPI
    /// This implementation is labeled "insecure" for demonstration purposes
    pub fn insecure_verify_pin(
        ctx: Context<InsecureVerifyPinCPI>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Get the account information of the external program
        let cpi_program = ctx.accounts.secret_program.to_account_info();

        // Create the CPI accounts context
        let cpi_accouts = VerifyPin {
            author: ctx.accounts.author.to_account_info(),
            secret_information: ctx.accounts.secret_information.to_account_info(),
        };

        // Create a new CPI context
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accouts);

        // Call the external CPI to verify the PIN
        arbitrary_cpi_expected::cpi::verify_pin(cpi_ctx, pin1, pin2, pin3, pin4)?;
        msg!("PIN VERIFIED");
        Ok(())
    }
    /// Verify PIN (Secure)
    /// This function verifies a secret PIN by calling an external CPI
    /// This implementation is labeled "secure" for demonstration purposes
    pub fn secure_verify_pin(
        ctx: Context<SecureVerifyPinCPI>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Get the account information of the external program
        let cpi_program = ctx.accounts.secret_program.to_account_info();

        if cpi_program.key() != arbitrary_cpi_expected::ID {
            return err!(ArbitraryCPIError::CPIProgramIDMismatch);
        }

        // Create the CPI accounts context
        let cpi_accouts = VerifyPin {
            author: ctx.accounts.author.to_account_info(),
            secret_information: ctx.accounts.secret_information.to_account_info(),
        };

        // Create a new CPI context
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accouts);

        // Call the external CPI to verify the PIN
        arbitrary_cpi_expected::cpi::verify_pin(cpi_ctx, pin1, pin2, pin3, pin4)?;
        msg!("PIN VERIFIED");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSecretCPI<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    /// CHECK: we check this in the program
    #[account(mut)]
    pub secret_information: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: we check this in the program
    pub secret_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InsecureVerifyPinCPI<'info> {
    pub author: Signer<'info>,
    /// CHECK: we check this in the program
    pub secret_information: Account<'info, SecretInformation>,
    /// CHECK: we check this in the program
    pub secret_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SecureVerifyPinCPI<'info> {
    pub author: Signer<'info>,
    /// CHECK: we check this in the program
    pub secret_information: Account<'info, SecretInformation>,
    /// CHECK: we check this in the program
    pub secret_program: Program<'info, ArbitraryCpi>,
}

#[error_code]
pub enum ArbitraryCPIError {
    #[msg("Incorrect CPI program ID")]
    CPIProgramIDMismatch,
}
