use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::instructions::{
    CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
    CreateMetadataAccountV3InstructionArgs,
};
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::Mint;
use anchor_spl::token_interface::TokenInterface;

use crate::state::Asset;

const MAGIC_NUMBER: u8 = 254;

pub fn _initialize_ix(
    ctx: Context<InitializeContext>,
    input1: u8,
    input2: u8,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    ctx.accounts.create_metadata(name, symbol, uri)?;

    let asset = &mut ctx.accounts.asset;

    asset.authority = ctx.accounts.signer.key();
    asset.mint = ctx.accounts.mint.key();
    asset.counter = buggy_math_function(input1, input2).into();
    Ok(())
}

pub fn buggy_math_function(input1: u8, input2: u8) -> u8 {
    let divisor = MAGIC_NUMBER - input2;
    input1 / divisor
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Asset::LEN,
        seeds = [b"asset",signer.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub asset: Account<'info, Asset>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Will be initialized
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    pub mpl_token_metadata: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeContext<'info> {
    pub fn create_metadata(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let mpl_metadata_program = &self.mpl_token_metadata.to_account_info();
        let metadata = &self.metadata_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let mint_authority = &self.signer.to_account_info();
        let payer = &self.signer.to_account_info();
        let system_program = &self.system_program.to_account_info();

        let cpi_context = CreateMetadataAccountV3Cpi::new(
            mpl_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority,
                payer,
                update_authority: (system_program, false), // second value sets if the account is also signer
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                is_mutable: false,
                collection_details: None,
            },
        );

        cpi_context.invoke()?;

        Ok(())
    }
}
