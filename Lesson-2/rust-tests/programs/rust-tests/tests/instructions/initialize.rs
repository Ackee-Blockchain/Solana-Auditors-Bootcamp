use anchor_lang::prelude::{Pubkey, ToAccountMetas};

// Function to define the account metas needed for the Initialize instruction.
pub fn initialize_metas(
    signer: &Pubkey,
    mint: &Pubkey,
    vault: &Pubkey,
    metadata_address: &Pubkey,
) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
    rust_tests::accounts::Initialize {
        signer: *signer,
        vault: *vault,
        mint: *mint,
        mint_metadata: *metadata_address,
        mpl_token_metadata: mpl_token_metadata::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: solana_sdk::system_program::ID,
    }
    .to_account_metas(None)
}

// Function to define the data for the Initialize instruction.
pub fn initialize_data(
    open_time: i64,
    input: u8,
    name: &str,
    symbol: &str,
    uri: &str,
) -> rust_tests::instruction::Initialize {
    // Create a struct representing the parameters of the mint.
    let mint_parameters = rust_tests::state::MintParameters {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
    };

    // Return the Initialize instruction with the input and mint parameters.
    rust_tests::instruction::Initialize {
        open_time,
        input,
        mint_parameters,
    }
}
