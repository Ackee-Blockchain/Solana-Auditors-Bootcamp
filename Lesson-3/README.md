# Fuzzing with Trident I

> [!IMPORTANT]
> For more details about the Trident, check the Trident documentation [Trident docs](https://ackee.xyz/trident/docs/latest/)

## Table of Contents
- [Program Issue Overview](#program-issue-overview)
    - [Program Structure](#program-structure)
    - [The Bug](#the-bug)
- [Writing Fuzz Test](#writing-fuzz-test)
    - [Step1 - Initialize Trident](#step-1---initialize-trident)
    - [Step2 - Add required dependencies](#step-2---add-required-dependencies)
    - [Step3 - Add required use statements](#step-3---add-required-use-statements)
    - [Step4 - Specify Genesis Programs](#step-4---specify-genesis-programs)
    - [Step5 - Writing Fuzz Tests](#step-5---write-fuzz-tests)
    - [Step6 - Run and Results](#step-6---run-and-results)


---


## Program Issue Overview

[Program Example](./trident-lesson-part-i/)

### Program Structure

The Program specified within this Lesson has the following structure

#### Asset Account

Program creates within its initialize instruction an Asset Account that has the following fields

1. `authority`: responsible for Asset creation and authority over the Asset.
2. `mint`: tied to the Asset Account and is also initialized within the initialize instruction.
3. `counter`: calculated during initialization.

```rust
#[account]
pub struct Asset {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub counter: u64,
}

impl Asset {
    pub const LEN: usize = 32 + 32 + 8;
}
```

#### Initialize Context

The initialize instruction expects the following accounts

1. `signer`: the new authority over Asset.
2. `asset`: the new asset account that is going to be initialized.
3. `mint`: the mint account tied to asset, also going to be initialized.
4. `metadata_account`: storing additional data for the mint account, going to be initialized using CPI to the Metaplex Metadata Program.
5. rest are programs required for initialization of accounts and the CPI.

> [!TIP]
> For more details about the Metaplex Metadata Program, check the Token Metadata documentation [Token Metadata](https://developers.metaplex.com/token-metadata)

```rust
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    // 1.
    #[account(mut)]
    pub signer: Signer<'info>,

    // 2.
    #[account(
        init,
        payer = signer,
        space = 8 + Asset::LEN,
        seeds = [b"asset",signer.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub asset: Account<'info, Asset>,

    // 3.
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
    )]
    pub mint: Account<'info, Mint>,

    // 4.
    /// CHECK: Will be initialized
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    // 5.
    pub mpl_token_metadata: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
```

#### Initialize Instruction

1. Create Metadata Account with specified Name, Symbol and URI
2. Call `buggy_math_function` which result is going to be assigned into the counter field.

```rust
pub fn _initialize_ix(
    ctx: Context<InitializeContext>,
    input1: u8,
    input2: u8,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    // 1.
    ctx.accounts.create_metadata(name, symbol, uri)?;

    let asset = &mut ctx.accounts.asset;

    asset.authority = ctx.accounts.signer.key();
    asset.mint = ctx.accounts.mint.key();
    // 2.
    asset.counter = buggy_math_function(input1, input2).into();
    Ok(())
}
```

### The Bug

> [!CAUTION]
> The `buggy_math_function` is not correct and contains the following issue.
> ```rust
> pub fn buggy_math_function(input1: u8, input2: u8) -> u8 {
>       let divisor = 254 - input2;
>       input1 / divisor
> }
> ```
> 1. In case of `input2 == 254` -> panic for division by zero occurs.
> 2. In case of `input2 > 254` -> panic for subtraction overflow occurs.



## Writing Fuzz Test

### Step 1 - Initialize Trident

Within the Anchor warkspace, call

```bash
trident init
```

This will initialize new Trident warkspace and prepare all required files and fuzz templates.

> [!tip]
> In case you want to add new fuzz test template, call
> ```bash
> trident fuzz add
> ```

### Step 2 - Add required dependencies

Within the `trident-tests/fuzz_tests/Cargo.toml`, add required dependencies for writing fuzz tests.

> [!tip]
> Most of the time, these are the same dependencies you have specified within the program Cargo.toml.

In our case of provided example we need to add the following dependencies

```toml
[[bin]]
name = "fuzz_0"
path = "fuzz_0/test_fuzz.rs"

[package]
name = "fuzz_tests"
version = "0.1.0"
description = "Created with Trident"
edition = "2021"

[dependencies]
honggfuzz = "0.5.56"
arbitrary = "1.3.0"
assert_matches = "1.4.0"

# --- ADDED ---
anchor-spl = { version = "0.30.1", features = ["metadata"] }

[dependencies.trident-client]
version = "0.7.0"

[dependencies.trident-lesson-part-i]
path = "../../programs/trident-lesson-part-i"

```

### Step 3 - Add required use statements

Within the `trident-tests/fuzz_tests/fuzz_0/accounts_snapshots.rs` add the required use statements.

> [!tip]
> In our case, we work with `Metadata`, `Mint` and `TokenInterface` so we have to add
> ```rust
> use anchor_spl::{metadata::Metadata, token::Mint, token_interface::TokenInterface};
> ```


### Step 4 - Specify Genesis Programs

Within the `trident-tests/fuzz_tests/fuzz_0/test_fuzz.rs`, specify all of the programs that should be included in the Testing Environment.

> [!tip]
> In our case, we use CPI to the Metaplex Token Metadata, so we need to also include this program in genesis

1. Create new FuzzingProgram instance for your program.
2. Create new FuzzingProgram instance for the Metaplex Token Metadata program.
3. Initialize ProgramTest with both FuzzingPrograms specified.

> [!note]
> Notice how the `Metaplex Token Metadata` has the entrypoint fn specified as None. This is because we are including the SBF binary file, which is automatically read from the `trident-genesis` folder.

> [!important]
> Do not forget do specify what types of instructions you want the fuzzer to generate, in this case we want to generate the `FuzzInstruction_trident_lesson_part_i`. If you have multiple programs within your Anchor Environment you can decide which programs instructions should the fuzzer generate.

```rust

// ...
pub type FuzzInstruction = FuzzInstruction_trident_lesson_part_i;
// ...

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // 1.
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_TRIDENT_LESSON_PART_I,
                &PROGRAM_ID_TRIDENT_LESSON_PART_I,
                processor!(convert_entry!(entry_trident_lesson_part_i))
            );

            // 2.
            let fuzzing_program2 = FuzzingProgram::new(
                "metaplex-token-metadata",
                &anchor_spl::metadata::ID,
                None
            );

            // 3.
            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1,fuzzing_program2])
                    .unwrap();

            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_TRIDENT_LESSON_PART_I, &mut client);
        });
    }
}

```
### Step 5 - Write Fuzz Tests


> [!important]
> Writting Fuzz Tests comes down to 3 basic steps
>
> 1. Specifying `FuzzAccounts` which is Storage for Accounts.
> 2. Specifying `get_data()` functions.
> 3. Specifying `get_accounts()` functions.
> 4. Optionaly you can specify Invariants Checks.
>     - Invariants checks are great to compare accounts before instruction and after instruction. So if the instruction passed you can compare if Accounts were updated as expected.

#### Specify `FuzzAccounts` storage

> [!tip]
> It is not necessary to specify the type for all of the Accounts. For example for the well known programs such as System Program, Token Program or Metaplex program we do not need storage. This means within the `get_accounts()` function we do not need to take the programs from the storage.


> [!note]
> In case of metadata_account it can be stored within the storage of type PdaStore. In our example, however, we have only one Instruction so we do not need to store and reuse the metadata_account, moreover instead of specifying seeds by hand we can use the `find_pda` function from the Metaplex.


```rust
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
```

#### Specify `get_data()` function

> [!note]
> In case of Instruction input data we do not need to do anything apart from asigning the correct data field within the InitializeIx struct. This is because we can send the automatically generated random data to the Instruction.


```rust
impl<'info> IxOps<'info> for InitializeIx {
    // ...
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
    // ...
}
```

#### Specify `get_accounts()` function

> [!note]
> In the `get_accounts()` we specify which accounts from the Account Storage to create or reuse, This can be particularly helpful when your program contains multiple Instructions. In that case you can mix the accounts sent to the Instructions and see if there is unauthorized access.

```rust
impl<'info> IxOps<'info> for InitializeIx {

    // ...
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
        }.to_account_metas(None);
        Ok((signers, acc_meta))
        // ...
    }
}



```


### Step 6 - Run and Results

To run the particular Fuzz Test:

```bash
# trident fuzz run fuzz_0
trident fuzz run <FUZZ_TARGET>
```

#### Fuzzing Output

> [!important]
> The output provided by Honggfuzz is as follows
>
> 1. Number of Fuzzing Iterations.
> 2. Feedback Driven Mode = Honggfuzz generates data based on the feedback (i.e. feedback based on Coverage progress).
> 3. Average Iterations per second
> 4. Number of crashes it found (**panics** or failed **invariant checks**)

```bash
------------------------[  0 days 00 hrs 00 mins 01 secs ]----------------------
  Iterations : 688 (out of: 1000 [68%]) # -- 1. --
  Mode [3/3] : Feedback Driven Mode # -- 2. --
      Target : trident-tests/fuzz_tests/fuzzing.....wn-linux-gnu/release/fuzz_0
     Threads : 16, CPUs: 32, CPU%: 1262% [39%/CPU]
       Speed : 680/sec [avg: 688] # -- 3. --
     Crashes : 1 [unique: 1, blocklist: 0, verified: 0] # -- 4. --
    Timeouts : 0 [10 sec]
 Corpus Size : 98, max: 1048576 bytes, init: 0 files
  Cov Update : 0 days 00 hrs 00 mins 00 secs ago
    Coverage : edge: 10345/882951 [1%] pc: 163 cmp: 622547
---------------------------------- [ LOGS ] ------------------/ honggfuzz 2.6 /-
```

#### CrashFile results

To see the results on the found Crashfile.

> [!tip]
> CrashFiles are by default stored within the `trident-tests/fuzz_tests/fuzzing/hfuzz_workspace/<FUZZ_TARGET>`

```bash
trident fuzz run-debug <FUZZ_TARGET> <PATH_TO_CRASHFILE>
```

> [!important]
> In Case of the following error message during debug:
>
> ```text
> :personality set failed: Operation not permitted
> ```
>
> Run the following:
>
> ```bash
> echo 'settings set target.disable-aslr false' >~/.lldbinit
> ```

> [!important]
> In Case of the following error message:
>
> ```text
> mportError: cannot import name 'SBData' from 'lldb' (unknown location)
> ```
>
> Check the solution here [llvm-project-issue](https://github.com/llvm/llvm-project/issues/55575#issuecomment-1247426995)



> [!important]
> The debug output is at current development stage really verbose and contains lldb parts. We are working on improving this experience. In the picture below you can see an example of provided debug output.
>
> 1. Series of Transaction Logs
> 2. Structures of data send within the Instructions
> 3. **Panic** or **Crash**, based on if the Fuzzing panicked within the Solana Program or Invariant Check failed.


![CrashFile execution](../.banner/run-debug.png)
