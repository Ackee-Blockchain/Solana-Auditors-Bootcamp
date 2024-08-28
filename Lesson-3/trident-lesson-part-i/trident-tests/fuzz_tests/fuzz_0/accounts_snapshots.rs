use anchor_lang::prelude::*;
use anchor_spl::{metadata::Metadata, token::Mint, token_interface::TokenInterface};
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeIxSnapshot<'info> {
    pub signer: Signer<'info>,
    pub asset: Option<Account<'info, trident_lesson_part_i::state::Asset>>,
    pub mint: Option<Account<'info, Mint>>,
    pub metadata_account: UncheckedAccount<'info>,
    pub mpl_token_metadata: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
impl<'info> InitializeIxSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let asset: Option<
            anchor_lang::accounts::account::Account<trident_lesson_part_i::state::Asset>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("asset".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("asset".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "asset".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint: Option<anchor_lang::accounts::account::Account<Mint>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("mint".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("mint".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided("mint".to_string()))
                }
            })
            .transpose()
            .unwrap_or(None);
        let metadata_account = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "metadata_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "metadata_account".to_string(),
            ))?;
        let mpl_token_metadata: anchor_lang::accounts::program::Program<Metadata> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "mpl_token_metadata".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "mpl_token_metadata".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("mpl_token_metadata".to_string())
            })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        Ok(Self {
            signer,
            asset,
            mint,
            metadata_account,
            mpl_token_metadata,
            system_program,
            token_program,
        })
    }
}
