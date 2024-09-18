use anchor_lang::prelude::*;

declare_id!("3U8eyhCS6ksULVZws9VkZ6Cc3Q5VjL3sjGezQQjM77uA");

#[program]
pub mod signer_authorization {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, data: u8) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.authority = *ctx.accounts.authority.key;
    escrow.data = data;

    Ok(())
  }
  pub fn insecure_authorization(ctx: Context<InsecureAuthorization>, data: u8) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.data = data;

    msg!("Data: {}", escrow.data);

    Ok(())
  }
  pub fn secure_authorization(ctx: Context<SecureAuthorization>, data: u8) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.data = data;

    msg!("Data: {}", escrow.data);

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(init, payer = authority, space = 8 + Escrow::LEN, seeds = [b"escrow".as_ref()], bump)]
  pub escrow: Account<'info, Escrow>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InsecureAuthorization<'info> {
  pub authority: Signer<'info>,
  /// CHECK: This is not correct
  #[account(
    mut,
    seeds = [b"escrow".as_ref()],
    bump
  )]
  pub escrow: Account<'info, Escrow>,
}

#[derive(Accounts)]
pub struct SecureAuthorization<'info> {
  pub authority: Signer<'info>,
  #[account(
    mut,
    seeds = [b"escrow".as_ref()],
    bump,
    has_one = authority
  )]
  pub escrow: Account<'info, Escrow>,
}

#[account]
pub struct Escrow {
  pub authority: Pubkey,
  pub data: u8,
}

impl Escrow {
  pub const LEN: usize = 32 + 1; // Pubkey + u8
}
