use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};
use mpl_token_metadata::{
    instructions::{
        CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::DataV2,
    ID as MPL_METADATA_PROGRAM,
};

use crate::{error::RustTestsError, state::MintParameters, state::Vault};

pub fn _initialize(
    ctx: Context<Initialize>,
    open_time: i64,
    input: u8,
    mint_parameters: MintParameters,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    require!(input != 5, RustTestsError::IncorrectInput);

    let time = Clock::get().unwrap();

    let open_time = time.unix_timestamp.checked_add(open_time).unwrap();

    vault.mint = ctx.accounts.mint.key();
    vault.vault_content = input;
    vault.open_time = open_time;
    vault.bump = ctx.bumps.vault;

    ctx.accounts.create_metadata(
        mint_parameters.name,
        mint_parameters.symbol,
        mint_parameters.uri,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Vault::LEN,
        seeds = [b"vault",signer.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = vault,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: AccountInfo here is OK as we are
    /// about to Initialize this Account
    #[account(
        mut,
        seeds = [
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = mpl_token_metadata.key()
    )]
    pub mint_metadata: AccountInfo<'info>,
    pub mpl_token_metadata: Program<'info, MplTokenMetadataProgram>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn create_metadata(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let mint = self.mint.key();
        let signer = self.signer.key();

        let signer_seeds: &[&[&[u8]]] =
            &[&[b"vault", signer.as_ref(), mint.as_ref(), &[self.vault.bump]]];

        let mpl_metadata_program = &self.mpl_token_metadata.to_account_info();
        let metadata = &self.mint_metadata.to_account_info();
        let mint = &self.mint.to_account_info();
        let mint_authority = &self.vault.to_account_info();
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

        cpi_context.invoke_signed(signer_seeds)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MplTokenMetadataProgram;

impl anchor_lang::Id for MplTokenMetadataProgram {
    fn id() -> Pubkey {
        MPL_METADATA_PROGRAM
    }
}
