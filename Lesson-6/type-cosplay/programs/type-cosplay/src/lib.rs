use anchor_lang::prelude::*;

declare_id!("J7Q6qZK86gGobrZbyHMBD5Wx9etRfGQK4sZ4XaWPnhfu");

#[program]
pub mod type_cosplay {
    use super::*;

    pub fn insecure_user_read(ctx: Context<InsecureTypeCosplay>) -> Result<()> {
        let user = User::try_from_slice(&ctx.accounts.user.data.borrow())?;

        if user.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::InvalidAccountData.into());
        }

        msg!(
            "The Age of the User: {} is: {}",
            ctx.accounts.authority.key(),
            user.age
        );
        Ok(())
    }
    pub fn secure_user_read(ctx: Context<SecureTypeCosplay>) -> Result<()> {
        let user = &ctx.accounts.user;
        msg!(
            "The Age of the User: {} is: {}",
            ctx.accounts.authority.key(),
            user.age
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureTypeCosplay<'info> {
    /// CHECK: unsafe, does not check the Account type
    pub user: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureTypeCosplay<'info> {
    #[account(
        has_one = authority,
    )]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

#[account]
pub struct User {
    pub authority: Pubkey,
    pub metadata_account: Pubkey,
    pub age: u32,
}

impl User {
    pub const LEN: usize = 32 + 32 + 4;
}

#[account]

pub struct UserMetadata {
    pub authority: Pubkey,
    pub user_account: Pubkey,
    pub pin1: u8,
    pub pin2: u8,
    pub pin3: u8,
    pub pin4: u8,
}

impl UserMetadata {
    pub const LEN: usize = 32 + 32 + 1 + 1 + 1 + 1;
}
