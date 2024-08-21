use anchor_lang::prelude::{Pubkey, ToAccountMetas};

// Function to define the account metas needed for the Read instruction.
pub fn read_metas(
    signer: &Pubkey,
    vault: &Pubkey,
) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
    rust_tests::accounts::Read {
        signer: *signer,
        vault: *vault,
    }
    .to_account_metas(None)
}

// Function to define the data for the Read instruction.
pub fn read_data() -> rust_tests::instruction::Read {
    rust_tests::instruction::Read {}
}
