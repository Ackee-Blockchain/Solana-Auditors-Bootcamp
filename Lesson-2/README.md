# Integration tests and Unit tests

## Table of Contents
- [Unit tests and Integration tests](#unit-tests-and-integration-tests)
- [Testing using Rust](#testing-using-rust)
- [Testing using Typescript](#testing-using-typescript)
- [Anchor Manifest](#anchor-manifest)
    - [provider](#provider)
    - [scripts](#scripts)
    - [registry](#registry)
    - [workspace](#workspace)
    - [programs](#programs)
    - [test](#test)
    - [test.validator](#test-validator)
    - [toolchain](#toolchain)
    - [clone from different cluster](#clone-from-different-cluster)
- [Forward in Time](#forward-in-time)
- [Best Testing practices](#best-testing-practices)
---


## Unit tests and Integration tests

| **Context**                      | **Unit Tests**                                                   | **Integration Tests**                                              |
|---------------------------------|------------------------------------------------------------------|--------------------------------------------------------------------|
| **Purpose**                     | Validate the correctness of individual functions or small components in isolation. | Test the interaction between multiple components or the entire program within a Solana environment. |
| **Scope**                       | Narrow, focused on specific functions or modules.                | Broad, covers interactions between multiple components. |
| **Complexity**                  | Relatively simple, testing one piece of code at a time.          | More complex, as it tests how different parts of the program work together. |
| **Dependencies**                | Minimal, often mocked or isolated from the rest of the program.  | High, requires a real or near-real Solana environment with actual dependencies. |
| **Execution Time**              | Fast, as it only tests small units of code.                      | Slower, as it involves more components and possibly network interactions. |
| **Test Data**                   | Simplified, often hardcoded or mocked to suit the specific function under test. | Realistic, using data that closely mimics what the program would encounter in production. |
| **Maintenance**                 | Easier to maintain since they focus on specific code segments.   | More complex to maintain as changes in one part of the program can affect multiple tests. |
| **Use Cases**                   | Ensuring specific logic works correctly (e.g., calculating a balance, performing math logic). | Ensuring the overall program behavior is correct (e.g., submitting mutliple transactions, interacting with other programs). |
| **Example**   | Testing if a specific instruction handler correctly processes inputs. | Testing if a series of transactions or interactions between accounts work as expected. |

> [!NOTE]
> The rest of the materials will focus on integration testing.

> [!TIP]
> For more details about the Unit Tests see [Unit Tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html).
> You can also check the [rust-unit-tests example](./rust-unit-tests/programs/rust-unit-tests/src/instructions/utils.rs).

## Testing using Rust

[Example](./rust-tests/)

### Testing workflow

1. Initialize `tests` folder
2. Specify desired `dev-dependencies` within the `Cargo.toml`
3. Include dumped SBF program from desired cluster (for example Mainnet) inside the `tests/fixtures` folder.
4. Write Tests
5. Execute:
    ```bash
    cargo test-sbf
    ```

> [!NOTE]
> There are multiple options where to store dumped program, however `tests/fixtures` is the most straight forward. For more information check [Docs](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTest.html#method.add_program).

> [!IMPORTANT]
> Provided program name in the `add_program(...)` function has to be the same as the dumped `*.so` file (see [Example](./rust-tests/))

> [!TIP]
> By default, the testing session is executed in parallel, meaning all of the tests are executed in parallel, in order to achieve sequential behavior use:
> ```bash
> cargo test-sbf -- --test-threads=1
> ```

> [!TIP]
> To dump program from desired cluster use
> ```bash
> # "-u m" stands for mainnet
> #
> solana program dump -u m <PROGRAM_ID> <PROGRAM_NAME>.so
> ```

### How to write Tests

1. Initialize new Program Test Instance.
2. Optionally, include other programs required for execution (for CPI calls).
3. Create (optionally)multiple wallets.
4. Aidrop funds.
5. Start Context.
6. Create Instructions.
7. Process Transaction.
8. Check the output.

> [!IMPORTANT]
> The example below and also [rust-example](./rust-tests/) adds [Metaplex Metadata Program](https://github.com/metaplex-foundation/mpl-token-metadata) to the Program Test Environment in order to Initialize Metadata for Mint -> common behavior for creating Fungible and Non-Fungible Tokens.

```rust
use solana_program_test::*;

// other required use statements

use rust_tests::entry;

// Constants for program ID, program name, and associated programs.
const PROGRAM_ID: Pubkey = rust_tests::ID_CONST; // Define the program ID constant.
const PROGRAM_NAME: &str = "rust_tests"; // Define the program name.
const MPL_TOKEN_METADATA: &str = "mpl_token_metadata"; // Define the MPL Token Metadata program name.

mod instructions;
mod utils;

#[tokio::test]
async fn test_with_rust_1() {
    // 1. Initialize a new ProgramTest instance with the program name, program ID, and entrypoint processor.
    let mut program_test =
        ProgramTest::new(PROGRAM_NAME, PROGRAM_ID, processor!(convert_entry!(entry)));

    // 2. Add the MPL Token Metadata program to the test environment.
    program_test.add_program(MPL_TOKEN_METADATA, mpl_token_metadata::ID, None);

    // 3. Generate new keypairs for the signer and mint.
    let signer = utils::generate_signer();
    let mint = utils::generate_signer();

    // 4. Airdrop some SOL to the signer's account to fund the test transactions.
    utils::airdrop(&mut program_test, signer.pubkey(), 5 * LAMPORTS_PER_SOL);

    // 5. Start the program test context, simulating a Solana runtime.
    let mut program_test_context = program_test.start_with_context().await;

    // 6. Do stuff here, for example construct Instructions

    // 7. Process the Initialize instruction in the simulated environment.
    let res = utils::process_instruction(
        &mut program_test_context,
        ix_initialize,
        &signer.pubkey(),
        signers,
    )
    .await;

    // 8. Assert that the instruction was successful.
    assert!(res.is_ok());
}
```

## Testing using Typescript

[Example Anchor Tests](./anchor-tests/)

[Example Anchor Tests with Bankrun](./bankrun-tests/)

### Testing Workflow

1. Initialize the `tests` folder within your project.
2. Install the necessary dependencies for testing in TypeScript with Anchor, typically through `npm` or `yarn`.
3. Create or update the TypeScript test file(s) in the `tests` directory.
5. Execute:
    ```bash
    anchor test
    ```
> [!TIP]
> Check the [Anchor Manifest](#anchor-manifest), for more details about setting up the `solana-test-validator` or see the [Reference](https://www.anchor-lang.com/docs/manifest).

### How to Write Tests

1. Generate a new keypair to act as a signer in your tests. This keypair will be used to sign transactions.
2. Airdrop SOL to the signer's account to cover transaction fees and other operations during the test.
3. Write individual test cases using the `it` function from Mocha (or any other testing framework).
4. Use the Anchor methods to send transactions, such as calling program methods, passing necessary accounts and signers, and ensuring the transactions are confirmed.
5. After executing transactions, fetch the relevant on-chain data and assert that it matches the expected results using your testing framework's assertion library.

> [!IMPORTANT]
> The following example shows how the structure for Typescript test can look like for complete example check the [anchor-example](./anchor-tests/).

```typescript
import * as anchor from "@coral-xyz/anchor";  // Import the Anchor library for interacting with Solana programs
import { Program } from "@coral-xyz/anchor";  // Import the Program type from the Anchor library
import { AnchorTests } from '../target/types/anchor_tests';  // Import the type definition for the AnchorTests program

// Describe block for the test suite for the "anchor-tests" program
describe("anchor-tests", () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    const program = anchor.workspace.AnchorTests as anchor.Program<AnchorTests>;

    // 1. Generate a new Keypair that will be used as a signer in the tests
    const signer = anchor.web3.Keypair.generate();

    // 2.
    before('Prepare', async () => {
        // Airdrop SOL to the signer's public key to cover transaction fees and initialization costs
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(signer.publicKey, 5_000_000_000), // Airdropping 5 SOL
            'confirmed'  // Wait for the transaction to be confirmed
        );
    });

    // 3.
    it('Initialize', async () => {
        // 4. Call the initialize method on the program with required arguments, accounts, and signers
        await program.methods.initialize(...).accounts({...}).signers([...]).rpc({ commitment: "confirmed" });

        // 5. Fetch the account data from the on-chain account after initialization
        let accountData = await program.account.dataAccount.fetch(...);
        // Assert that the fetched data matches the expected data
        assert.strictEqual(accountData.someField.toString(), expected_data.toString());
        assert.strictEqual(accountData.someOtherField.toString(), some_other_expected_data.toString());
    });

});

```


## Anchor Manifest

### provider

A wallet and cluster that are used for all commands.

```toml
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

### scripts

The test script is executed by `anchor test`.

> [!TIP]
> Other defined scripts can be run with `anchor run <script>`. But beware, this command will not start the solana-test-validator.


```toml
[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

### registry

The registry that is used in commands related to verifiable builds (e.g. when pushing a verifiable build with `anchor publish`).

```toml
[registry]
url = "https://api.apr.dev"
```

> [!TIP]
> Read more about registry here [Publishing Source](https://www.anchor-lang.com/docs/publishing-source)

### features

This tells the IDL to include seed generation for PDA Accounts. The default is false

```toml
[features]
seeds = true
```

### workspace

#### types

Adds a directory where you want the `<idl>.ts` file to be copied when running `anchor build` or `anchor idl parse`.

> [!TIP]
> This is helpful when you want to keep this file in version control, like when using it on the frontend, which will probably not have access to the target directory generated by anchor.

```toml
[workspace]
types = "app/src/idl/"
```
#### members

Sets the paths relative to the Anchor.toml to all programs in the local workspace, i.e. the path to the Cargo.toml manifest associated with each program that can be compiled by the anchor CLI.

> [!TIP]
> For programs using the standard Anchor workflow, this can be omitted. For programs not written in Anchor but still want to publish, this should be added.



```toml
[workspace]
members = [
    "programs/*",
    "other_place/my_program"
]
```

### programs

The addresses of the programs in the workspace.

> [!TIP]
> More programs = More addresses

```toml
[programs.localnet]
my_program = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
```
### test

#### startup_wait

Increases the time anchor waits for the `solana-test-validator` to start up.

> [!TIP]
> This is, for example, useful if you're cloning (see `test.validator.clone`) with many accounts which increases the validator's startup time.


```toml
[test]
startup_wait = 10000
```

#### upgradeable

Deploys the program to solana-test-validator using `--upgradeable-program`. This makes it possible to test that certain instructions can only be executed by the program's upgrade authority. The initial upgrade authority will be set to `provider.wallet`.

If unspecified or explicitly set to `false`, then the test program will be deployed with `--bpf-program`, disabling upgrades to it.

```toml
[test]
upgradeable = true
```


### test.validator

These options are passed into the options with the same name in the `solana-test-validator` cli (see `solana-test-validator --help`) in commands like `anchor test`.

```toml
[test.validator]
url = "https://api.mainnet-beta.solana.com"     # This is the url of the cluster that accounts are cloned from (See `test.validator.clone`).
warp_slot = "1337"                              # Warp the ledger to `warp_slot` after starting the validator.
slots_per_epoch = "32"                          # Override the number of slots in an epoch (value must be >=32)
rpc_port = 8896                                 # Set JSON RPC on this port, and the next port for the RPC websocket.
limit_ledger_size = "1337"                      # Keep this amount of shreds in root slots.
ledger = "test-ledger"                          # Set ledger location.
gossip_port = 8994                              # Gossip port number for the validator.
gossip_host = "127.0.0.1"                       # Gossip DNS name or IP address for the validator to advertise in gossip.
faucet_sol = "1337"                             # Give the faucet address this much SOL in genesis.
faucet_port = 8995                              # Enable the faucet on this port.
dynamic_port_range = "1337-13337"               # Range to use for dynamically assigned ports.
bind_address = "0.0.0.0"
```
> [!TIP]
> It is better to set `rpc_port`, `gossip_port` and `faucet_port` to different ports to prevent the test validator startup issues.

> [!TIP]
> If for some reason the solana-test-validator does not start (you get the error stating *`Test validator does not look started...`*), do not forget to look in `test-ledger-log.txt` file usually located in `.anchor/test-ledger/test-ledger-log.txt`. It contains information about possible invalid values in your \[test.validator\] config inside `Anchor.toml`.

### toolchain

Override toolchain data in the workspace similar to `rust-toolchain.toml`.

```toml
[toolchain]
anchor_version = "0.30.1"   # `anchor-cli` version to use(requires `avm`)
solana_version = "1.18.17"    # Solana version to use(applies to all Solana tools)
```

### Clone from different cluster

### genesis

Makes commands like `anchor test` start `solana-test-validator` with a given program already loaded.

> [!IMPORTANT]
> This is one way how to use Cross Program Invocation with SBF program dumped for example from Mainnet.

> [!TIP]
> To dump program from desired cluster use
> ```bash
> # "-u m" stands for mainnet
> #
> solana program dump -u m <PROGRAM_ID> <PROGRAM_NAME>.so
> ```

```toml
[[test.genesis]]
address = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"
program = "dex.so" # path to the SBF


[[test.genesis]]
address = "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD"
program = "swap.so" # path to the SBF
upgradeable = true
```

### test.validator.clone

Use this to clone an account from the `test.validator.url` cluster to your local cluster.

> [!IMPORTANT]
> This is another way how to use Cross Program Invocation with SBF program dumped for example from Mainnet.

```toml
[test.validator]
url = "https://api.mainnet-beta.solana.com"


[[test.validator.clone]]
address = "7NL2qWArf2BbEBBH1vTRZCsoNqFATTddH6h8GkVvrLpG"
[[test.validator.clone]]
address = "2RaN5auQwMdg5efgCaVqpETBV8sacWGR8tkK4m9kjo5r"
[[test.validator.clone]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
```
### test.validator.account

Use this to upload an account from a `.json` file.

> [!TIP]
> To dump account from desired cluster use
> ```bash
> # "-u m" stands for mainnet
> #
> solana account -u m <ACCOUNT_ADDRESS> --output json
> ```

```toml
[[test.validator.account]]
address = "Ev8WSPQsGb4wfjybqff5eZNcS3n6HaMsBkMk9suAiuM"
filename = "some_account.json"


[[test.validator.account]]
address = "Ev8WSPQsGb4wfjybqff5eZNcS3n6HaMsBkMk9suAiuM"
filename = "some_other_account.json"
```

## Forward in Time

### Testing using Rust

> [!TIP]
> ProgramTest provides multiple methods that can update the Clock Sysvar values.
> - [warp_to_slot](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html#method.warp_to_slot)
> - [warp_to_epoch](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html#method.warp_to_epoch)
> - [set_sysvar](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTestContext.html#method.set_sysvar)

Forward in time using the `set_sysvar` method (also used in the [rust-example](./rust-tests/)).

```rust
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
```

### Testing using Typescript

#### Option 1

Do not forward in time. Rather, update the deadline threshold to lower number (i.e. few seconds) and use `sleep()` method for the desired amount. This is not the best approach, but it can still be useful.

#### Option 2

Forward in time using [Bankrun](https://kevinheavey.github.io/solana-bankrun/).

The code below shows example (also implemented in the [bankrun-example](./bankrun-tests/))

```ts
// Fetch the Clock Sysvar
let clock = await test_env.context.banksClient.getClock()

// Get the current timestamp
const now = clock.unixTimestamp;

// Calculate desired future unixtimestamp
const in_future_7_days = now + BigInt(7 * 24 * 60 * 60);

// Initialize new Clock Instance
let new_clock = new Clock(clock.slot, clock.epochStartTimestamp, clock.epoch, clock.leaderScheduleEpoch, in_future_7_days);

// Set the new Clock Sysvar
test_env.context.setClock(new_clock);

```

> [!IMPORTANT]
> Even though Bankrun can be used along with the Anchor Framework, it is important to keep in mind that it does not work with the same Environment as the Anchor Framework. Anchor starts the `solana-test-validator`, Bankrun uses ProgramTest under the hood, that means Transactions are processed in two different Environments. However, it might be beneficial to use Bankrun in some standalone test paths.
