pub mod trident_lesson_part_i_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitializeIx(InitializeIx),
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIx {
        pub accounts: InitializeIxAccounts,
        pub data: InitializeIxData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIxAccounts {
        pub signer: AccountId,
        pub asset: AccountId,
        pub mint: AccountId,
        pub metadata_account: AccountId,
        pub mpl_token_metadata: AccountId,
        pub system_program: AccountId,
        pub token_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIxData {
        pub input1: u8,
        pub input2: u8,
        pub name: String,
        pub symbol: String,
        pub uri: String,
    }
    impl<'info> IxOps<'info> for InitializeIx {
        type IxData = trident_lesson_part_i::instruction::InitializeIx;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeIxSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = trident_lesson_part_i::instruction::InitializeIx {
                input1: self.data.input1,
                input2: self.data.input2,
                name: self.data.name.clone(),
                symbol: self.data.symbol.clone(),
                uri: self.data.uri.clone(),
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

            let mint = fuzz_accounts.mint.get_or_create_account(
                self.accounts.mint,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let metadata_account =
                anchor_spl::metadata::mpl_token_metadata::accounts::Metadata::find_pda(
                    &mint.pubkey(),
                );

            let asset = fuzz_accounts
                .asset
                .get_or_create_account(
                    self.accounts.asset,
                    &[b"asset", signer.pubkey().as_ref(), mint.pubkey().as_ref()],
                    &trident_lesson_part_i::ID,
                )
                .unwrap();

            let signers = vec![signer.clone(), mint.clone()];
            let acc_meta = trident_lesson_part_i::accounts::InitializeContext {
                signer: signer.pubkey(),
                asset: asset.pubkey,
                mint: mint.pubkey(),
                metadata_account: metadata_account.0,
                mpl_token_metadata: anchor_spl::metadata::ID,
                system_program: solana_sdk::system_program::ID,
                token_program: anchor_spl::token::ID,
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
        // metadata_account: AccountsStorage<PdaStore>,
        mint: AccountsStorage<Keypair>,
        // mpl_token_metadata: AccountsStorage<todo!()>,
        signer: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<todo!()>,
        // token_program: AccountsStorage<todo!()>,
    }
}
