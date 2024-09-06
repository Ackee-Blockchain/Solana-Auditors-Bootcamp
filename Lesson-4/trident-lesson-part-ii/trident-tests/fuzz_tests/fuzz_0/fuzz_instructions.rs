pub mod trident_lesson_part_ii_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::InstructionError,
        native_token::LAMPORTS_PER_SOL,
        transaction::TransactionError,
    };
    use trident_client::fuzzing::anchor_lang::AccountSerialize;

    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitializeIx(InitializeIx),
        UpdateIx(UpdateIx),
        DummyIx(DummyIx),
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIx {
        pub accounts: InitializeIxAccounts,
        pub data: InitializeIxData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIxAccounts {
        pub signer: AccountId,
        pub config_authority: AccountId,
        pub asset: AccountId,
        pub mint: AccountId,
        pub config: AccountId,
        pub system_program: AccountId,
        pub token_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIxData {}
    #[derive(Arbitrary, Debug)]
    pub struct UpdateIx {
        pub accounts: UpdateIxAccounts,
        pub data: UpdateIxData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateIxAccounts {
        pub signer: AccountId,
        pub asset: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateIxData {
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=100))]
        pub input1: u8,
        // #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(120..=200))]
        pub input2: u8,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DummyIx {
        pub accounts: DummyIxAccounts,
        pub data: DummyIxData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DummyIxAccounts {
        pub signer: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DummyIxData {}
    impl<'info> IxOps<'info> for InitializeIx {
        type IxData = trident_lesson_part_ii::instruction::InitializeIx;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeIxSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = trident_lesson_part_ii::instruction::InitializeIx {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
            // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
            // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
            // Client Methods

            // create config authority account
            let config_authority = client.set_account(10 * LAMPORTS_PER_SOL);

            // find config address
            let (config_address, _config_bump) = Pubkey::try_find_program_address(
                &[b"config", config_authority.pubkey().as_ref()],
                &trident_lesson_part_ii::ID,
            )
            .unwrap();

            // set config fields
            let config = trident_lesson_part_ii::state::Config {
                authority: config_authority.pubkey(),
            };

            // serialize config into vector
            let mut data: Vec<u8> = vec![];
            config.try_serialize(&mut data).unwrap();

            // set Custom Config Account
            client.set_account_custom(
                &config_address,
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL,
                    data,
                    trident_lesson_part_ii::ID,
                    false,
                    0,
                ),
            );
            // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
            // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
            // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*

            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let mint = fuzz_accounts.mint.get_or_create_account(
                self.accounts.mint,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let asset = fuzz_accounts
                .asset
                .get_or_create_account(
                    self.accounts.asset,
                    &[b"asset", signer.pubkey().as_ref()],
                    &trident_lesson_part_ii::ID,
                )
                .unwrap();

            let signers = vec![signer.clone(), mint.clone(), config_authority.clone()];
            let acc_meta = trident_lesson_part_ii::accounts::InitializeContext {
                signer: signer.pubkey(),
                config_authority: config_authority.pubkey(),
                asset: asset.pubkey,
                config: config_address,
                mint: mint.pubkey(),
                system_program: solana_sdk::system_program::ID,
                token_program: anchor_spl::token::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpdateIx {
        type IxData = trident_lesson_part_ii::instruction::UpdateIx;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateIxSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = trident_lesson_part_ii::instruction::UpdateIx {
                input1: self.data.input1,
                input2: self.data.input2,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let asset = fuzz_accounts
                .asset
                .get_or_create_account(
                    self.accounts.asset,
                    &[b"asset", signer.pubkey().as_ref()],
                    &trident_lesson_part_ii::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = trident_lesson_part_ii::accounts::UpdateContext {
                signer: signer.pubkey(),
                asset: asset.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
        // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        // Invariant Checks
        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            match post_ix.asset.counter.checked_sub(pre_ix.asset.counter) {
                Some(difference) => {
                    if difference > 5 {
                        return Err(FuzzingError::Custom(5));
                    }
                    Ok(())
                }
                None => Ok(()),
            }
        }
        // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        // -*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        // Custom Error Handler
        fn tx_error_handler(
            &self,
            e: FuzzClientErrorWithOrigin,
            ix_data: Self::IxData,
            _pre_ix_acc_infos: &'info mut [Option<AccountInfo<'info>>],
        ) -> Result<(), FuzzClientErrorWithOrigin> {
            let client_e = e.client_error;
            match client_e {
                FuzzClientError::BanksError(banks_e) => match banks_e.unwrap() {
                    TransactionError::InstructionError(_, InstructionError::Custom(6000)) => {
                        if ix_data.input2 >= 254 {
                            panic!("Expected Error returned")
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                },
                _ => Ok(()),
            }
        }
    }
    impl<'info> IxOps<'info> for DummyIx {
        type IxData = trident_lesson_part_ii::instruction::DummyIx;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DummyIxSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = trident_lesson_part_ii::instruction::DummyIx {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let signers = vec![signer.clone()];
            let acc_meta = trident_lesson_part_ii::accounts::DummyContext {
                signer: signer.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        asset: AccountsStorage<PdaStore>,
        mint: AccountsStorage<Keypair>,
        signer: AccountsStorage<Keypair>,
        // _system_program: AccountsStorage<ProgramStore>,
        // _token_program: AccountsStorage<ProgramStore>,
    }
}
