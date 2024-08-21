use anchor_lang::prelude::{AccountMeta, Pubkey};
use anchor_lang::InstructionData;
use solana_program_test::*;

use rust_tests::entry;
use solana_sdk::instruction::Instruction;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{entrypoint::ProcessInstruction, signer::Signer};

// Constants for program ID, program name, and associated programs.
const PROGRAM_ID: Pubkey = rust_tests::ID_CONST; // Define the program ID constant.
const PROGRAM_NAME: &str = "rust_tests"; // Define the program name.
const MPL_TOKEN_METADATA: &str = "mpl_token_metadata"; // Define the MPL Token Metadata program name.

mod instructions;
mod utils;

// Define the duration for 7 days.
const DAYS_7: i64 = 7 * 24 * 60 * 60;

#[tokio::test]
async fn test_with_rust_1() {
    // Initialize a new ProgramTest instance with the program name, program ID, and entrypoint processor.
    let mut program_test =
        ProgramTest::new(PROGRAM_NAME, PROGRAM_ID, processor!(convert_entry!(entry)));

    // Add the MPL Token Metadata program to the test environment.
    program_test.add_program(MPL_TOKEN_METADATA, mpl_token_metadata::ID, None);

    // Generate new keypairs for the signer and mint.
    let signer = utils::generate_signer();
    let mint = utils::generate_signer();

    // Airdrop some SOL to the signer's account to fund the test transactions.
    utils::airdrop(&mut program_test, signer.pubkey(), 5 * LAMPORTS_PER_SOL);

    // Start the program test context, simulating a Solana runtime.
    let mut program_test_context = program_test.start_with_context().await;

    // Get the vault address associated with the signer and mint using a program-derived address (PDA).
    let vault_address = utils::get_vault_address(&signer.pubkey(), &mint.pubkey());

    // Find the metadata PDA for the mint.
    let (metadata_address, _bump) =
        mpl_token_metadata::accounts::Metadata::find_pda(&mint.pubkey());

    // Define the account metas needed for the Initialize instruction.
    let initialzie_metas = instructions::initialize_metas(
        &signer.pubkey(),
        &mint.pubkey(),
        &vault_address,
        &metadata_address,
    );

    // Define the parameters for the Initialize instruction.
    const NAME: &str = "Name1";
    const SYMBOL: &str = "SMB1";
    const URI: &str = "URI1";

    // Create the Initialize instruction with the necessary data.
    let initialize_data = instructions::initialize_data(DAYS_7, 8, NAME, SYMBOL, URI);

    // Build the actual Solana instruction using the program ID, accounts, and data.
    let ix_initialize = build_ix(&PROGRAM_ID, initialzie_metas, initialize_data.data());

    // Define the signers for the transaction.
    let signers = vec![&signer, &mint];

    // Process the Initialize instruction in the simulated environment.
    let res = utils::process_instruction(
        &mut program_test_context,
        ix_initialize,
        &signer.pubkey(),
        signers,
    )
    .await;

    // Assert that the instruction was successful.
    assert!(res.is_ok());

    // Define the accounts and metas needed for the Read instruction.
    let read_metas = instructions::read_metas(&signer.pubkey(), &vault_address);

    // Create the Read instruction, which attempts to read data from the vault.
    let read_data = instructions::read_data();
    let ix_read = build_ix(&PROGRAM_ID, read_metas, read_data.data());

    // Define the signers for the transaction.
    let signers = vec![&signer];

    // Process the Read instruction, which is expected to fail in this test.
    let res = utils::process_instruction(
        &mut program_test_context,
        ix_read,
        &signer.pubkey(),
        signers,
    )
    .await;

    // Assert that the instruction resulted in an error.
    assert!(res.is_err());
}

#[tokio::test]
async fn test_with_rust_2() {
    // Initialize a new ProgramTest instance without specifying a processor.
    let mut program_test =
        ProgramTest::new(PROGRAM_NAME, PROGRAM_ID, processor!(convert_entry!(entry)));

    // Add the MPL Token Metadata program to the test environment.
    program_test.add_program(MPL_TOKEN_METADATA, mpl_token_metadata::ID, None);

    // Generate new keypairs for the signer and mint.
    let signer = utils::generate_signer();
    let mint = utils::generate_signer();

    // Airdrop some SOL to the signer's account to fund the test transactions.
    utils::airdrop(&mut program_test, signer.pubkey(), 5 * LAMPORTS_PER_SOL);

    // Start the program test context, simulating a Solana runtime.
    let mut program_test_context = program_test.start_with_context().await;

    // Get the vault address associated with the signer and mint using a program-derived address (PDA).
    let vault_address = utils::get_vault_address(&signer.pubkey(), &mint.pubkey());

    // Find the metadata PDA for the mint.
    let (metadata_address, _bump) =
        mpl_token_metadata::accounts::Metadata::find_pda(&mint.pubkey());

    // Define the account metas needed for the Initialize instruction.
    let initialzie_metas = instructions::initialize_metas(
        &signer.pubkey(),
        &mint.pubkey(),
        &vault_address,
        &metadata_address,
    );

    // Define the parameters for the Initialize instruction.
    const NAME: &str = "Name1";
    const SYMBOL: &str = "SMB1";
    const URI: &str = "URI1";

    // Create the Initialize instruction with the necessary data.
    let initialize_data = instructions::initialize_data(DAYS_7, 8, NAME, SYMBOL, URI);

    // Build the actual Solana instruction using the program ID, accounts, and data.
    let ix_initialize = build_ix(&PROGRAM_ID, initialzie_metas, initialize_data.data());

    // Define the signers for the transaction.
    let signers = vec![&signer, &mint];

    // Process the Initialize instruction in the simulated environment.
    let res = utils::process_instruction(
        &mut program_test_context,
        ix_initialize,
        &signer.pubkey(),
        signers,
    )
    .await;

    // Assert that the instruction was successful.
    assert!(res.is_ok());

    // Forward the program test context time by 7 days.
    utils::forward_time(&mut program_test_context, DAYS_7).await;

    // Define the accounts and metas needed for the Read instruction.
    let read_metas = instructions::read_metas(&signer.pubkey(), &vault_address);

    // Create the Read instruction, which attempts to read data from the vault.
    let read_data = instructions::read_data();
    let ix_read = build_ix(&PROGRAM_ID, read_metas, read_data.data());

    // Define the signers for the transaction.
    let signers = vec![&signer];

    // Process the Read instruction, which is expected to succeed after the time delay.
    let res = utils::process_instruction(
        &mut program_test_context,
        ix_read,
        &signer.pubkey(),
        signers,
    )
    .await;

    // Assert that the instruction was successful.
    assert!(res.is_ok());
}

// Function to build the actual Solana instruction using the program ID, accounts, and data.
pub fn build_ix(program_id: &Pubkey, accounts: Vec<AccountMeta>, data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
