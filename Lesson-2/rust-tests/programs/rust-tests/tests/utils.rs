use anchor_lang::{prelude::AccountInfo, Key}; // Import necessary modules.
use solana_program_test::{ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account, clock::Clock, entrypoint::ProgramResult, instruction::Instruction,
    pubkey::Pubkey, signature::Keypair, transaction::Transaction,
};

// Type alias for the entry function pointer used to convert the entry function into a ProcessInstruction function pointer.
pub type ProgramEntry = for<'info> fn(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult;

// Macro to convert the entry function into a ProcessInstruction function pointer.
#[macro_export]
macro_rules! convert_entry {
    ($entry:expr) => {
        // Use unsafe block to perform memory transmutation.
        unsafe { core::mem::transmute::<utils::ProgramEntry, ProcessInstruction>($entry) }
    };
}

const PROGRAM_ID: Pubkey = rust_tests::ID_CONST; // Define the program ID constant.

// Function to generate a new keypair for signing transactions.
pub fn generate_signer() -> solana_sdk::signer::keypair::Keypair {
    solana_sdk::signer::keypair::Keypair::new()
}

// Function to get the vault address associated with the signer and mint.
pub fn get_vault_address(signer: &Pubkey, mint: &Pubkey) -> Pubkey {
    // Find the program-derived address (PDA) for the vault associated with the signer and mint.
    let (address, _bump) = Pubkey::find_program_address(
        &[b"vault", signer.key().as_ref(), mint.key().as_ref()],
        &PROGRAM_ID,
    );
    address
}

// Function to add an account with the specified amount of lamports to the program test.
pub fn airdrop(program_test: &mut ProgramTest, address: Pubkey, amount: u64) {
    program_test.add_account(
        address,
        Account::new(amount, 0, &solana_sdk::system_program::ID),
    );
}

// Function to process an instruction in the program test context and ensure it is finalized.
pub async fn process_instruction(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Pubkey,
    signers: Vec<&Keypair>,
) -> std::result::Result<(), solana_program_test::BanksClientError> {
    // Create a new transaction with the given instruction and payer.
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(payer));

    // Sign the transaction with the provided signers.
    transaction.sign(&signers, program_test_context.last_blockhash);

    // Process the transaction within the program test context with commitment level finalized.
    program_test_context
        .banks_client
        .process_transaction_with_commitment(
            transaction,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
}

// Function to forward the program test context time by a specified number of seconds.
pub async fn forward_time(program_test_context: &mut ProgramTestContext, seconds: i64) {
    // Get the current clock state from the program test context.
    let mut clock = program_test_context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap();

    // Calculate the new timestamp after advancing time.
    let new_timestamp = clock.unix_timestamp + seconds;

    // Update the Clock instance with the new timestamp.
    clock.unix_timestamp = new_timestamp;

    // Update the sysvar in the program test context with the new Clock state.
    program_test_context.set_sysvar(&clock);
}
