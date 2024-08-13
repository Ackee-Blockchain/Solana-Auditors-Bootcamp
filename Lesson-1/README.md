# Lesson 1 - Advanced Anchor

## Table of Contents
- [Attribute Macro #[program]](#attribute-macro-program)
- [Derive Macro Accounts](#derive-macro-accounts)
- [Attribute Macro #[account]](#attribute-macro-account)
- [Anchor Expand](#anchor-expand)
- [Account Types](#account-types)
    - [Account](#account)
    - [AccountInfo](#accountinfo)
    - [AccountLoader](#accountloader)
    - [Boxed](#boxed)
    - [Interface](#interface)
    - [InterfaceAccount](#interfaceaccount)
    - [Option](#option)
    - [Program](#program)
    - [Signer](#signer)
    - [SystemAccount](#systemaccount)
    - [Sysvar](#sysvar)
    - [UncheckedAccount](#uncheckedaccount)
- [Account Constraints](#account-constraints)
    - [Normal](#normal)
    - [Solana Program Library](#solana-program-library)
- [Account Space](#account-space)
---


## Attribute Macro #[program]

[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/attr.program.html)

Program Intruction Entry dispatch

1. `entry` function.
2. `try_entry` function.
3. **PROGRAM_ID** and sufficient length for Instruction **DISCRIMINATOR** check.
4. Strip first 8 bytes as Instruction **DISCRIMINATOR**.
5. **Dispatch** Instruction based on the Instruction **DISCRIMINATOR**.
6. Call Accounts deserialization and check specified constraints.

```rust

/// 1.
entrypoint!(entry);


pub fn entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}

/// 2.
fn try_entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    /// 3.
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    /// 3.
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}

fn dispatch<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    /// 4.
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    use anchor_lang::Discriminator;
    /// 5.
    match sighash {
        instruction::Initialize::DISCRIMINATOR => {
            __private::__global::initialize(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {
            __private::__idl::__idl_dispatch(program_id, accounts, &ix_data)
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}


mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl { ... }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Initialize");

            ...

            /// 6.
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;

            ...

        }
    }
}

```

## Derive Macro `Accounts`

[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)


Implements an Accounts deserializer on the given struct. Can provide further functionality through the use of account constraints.

1. Invoked from the `__private::__global::initialize` function.
2. Call `try_accounts` function implementation for the given Account Type.
    - For the Data Accounts, perform Deserialization
        - `try_accounts` for the Data Account type
            - [try_accounts](https://github.com/coral-xyz/anchor/blob/dc6ac2d631fd759339c67a25996c9570bb1e71df/lang/src/accounts/account.rs#L340)
3. If specified perform Account Constraints check (for example if the `init` is specified -> Initialize the Account).
4. Generate CPI Structs (not present in the code snippet below, can be seen in the `anchor_solana-expanded.rs`).

- `try_accounts` implementation for the Account Type **Signer**
    - [try_accounts](https://github.com/coral-xyz/anchor/blob/dc6ac2d631fd759339c67a25996c9570bb1e71df/lang/src/accounts/signer.rs#L59)
- `try_accounts` implementation for the Account Type **Program**
    - [try_accounts](https://github.com/coral-xyz/anchor/blob/dc6ac2d631fd759339c67a25996c9570bb1e71df/lang/src/accounts/program.rs#L144)

```rust

...

impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize<'info>
where
    'info: 'info,
{
    /// 1.
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut InitializeBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        /// 2.
        let signer: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("signer"))?;

        /// 2.
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;

        /// 3. If Account Constraints are specified

        Ok(Initialize {
            signer,
            system_program,
        })
    }
}

...

```


## Attribute Macro `#[account]`

[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html)

An attribute for a data structure representing a Solana account. Gnerates implementation for the given Traits

- AccountSerialize
- AccountDeserialize
- AnchorSerialize
- AnchorDeserialize
- Clone
- Discriminator
- Owner



```rust

/// For Reference - Original Definition
/// #[account]
/// pub struct DataAccount {
///     pub authority: Pubkey,
///     pub counter: u64,
/// }

/// Implemnentation of the AccountSerialize
impl anchor_lang::AccountSerialize for DataAccount {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> anchor_lang::Result<()> {
        if writer.write_all(&[85, 240, 182, 158, 76, 7, 18, 233]).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        if AnchorSerialize::serialize(self, writer).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(())
    }
}

/// Implemnentation of the AccountDeserialize
impl anchor_lang::AccountDeserialize for DataAccount {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [85, 240, 182, 158, 76, 7, 18, 233].len() {
            return Err(
                anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
            );
        }
        let given_disc = &buf[..8];
        if &[85, 240, 182, 158, 76, 7, 18, 233] != given_disc {
            /// Error below is too long -> removed
            return Err( ... )
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}

/// Implemnentation of the Discriminator
impl anchor_lang::Discriminator for DataAccount {
    const DISCRIMINATOR: [u8; 8] = [85, 240, 182, 158, 76, 7, 18, 233];
}

/// Implemnentation of the DataAccount
impl anchor_lang::Owner for DataAccount {
    fn owner() -> Pubkey {
        crate::ID
    }
}

```

### Anchor’s Internal Discriminator

[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/trait.Discriminator.html)

Consider a program that manages two types of accounts, Account A and Account B. Both accounts are owned by the same program and have identical fields. Now, suppose you have an instruction called foo that is designed to only operate on Account A.

However, a user mistakenly passes Account B as an argument to the foo instruction. Given that Account B shares the same owner and the same fields as Account A, how can the program detect this mistake and throw an error?

This is where the discriminator comes into play. It uniquely identifies the type of an account. Even though Account A and Account B are structurally identical and share the same owner, they have different discriminators.

> [!NOTE]
> The length of the DISCRIMINATOR is 8 bytes.


### Zero Copy Deserialization

[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html#zero-copy-deserialization)

> [!WARNING]
> Zero copy deserialization is an experimental feature. It’s recommended to use it only when necessary, i.e., when you have extremely large accounts that cannot be Borsh deserialized without hitting stack or heap limits.


To enable zero-copy-deserialization, one can pass in the zero_copy argument to the macro as follows:
```rust
#[account(zero_copy)]
```

> [!NOTE]
> Other than being more efficient, the most salient benefit this provides is the ability to define account types larger than the max stack or heap size. When using borsh, the account has to be copied and deserialized into a new data structure and thus is constrained by stack and heap limits imposed by the BPF VM. With zero copy deserialization, all bytes from the account’s backing RefCell<&mut [u8]> are simply re-interpreted as a reference to the data structure. No allocations or copies necessary. Hence the ability to get around stack and heap limitations.

## Anchor Expand
It is possible to see all of the Anchor macros expanded.

> [!TIP]
> - Use `anchor expand` command to see all of the Macros expanded within your Solana program.

## Account Types

### Account

[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/account.rs)

- Wrapper around AccountInfo that verifies program ownership and deserializes underlying data into a Rust type.

```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: Account<'info, CustomAccount>,
}
```

### AccountInfo

[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/account_info.rs)

> [!CAUTION]
> This type does not perform any validation checks.

- Raw AccountInfo.

```rust
#[derive(Accounts)]
pub struct Context<'info> {
    /// CHECK: AccountInfo is Uunchecked Account
    pub account: AccountInfo<'info>,
}
```

### AccountLoader

[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/account_loader.rs)

- Type facilitating on demand zero copy deserialization.

> [!NOTE]
> Note that using accounts in this way is distinctly different from using, for example, the Account. Namely, one must call
>
> - load_init after initializing an account (this will ignore the missing account discriminator that gets added only after the user’s instruction code)
> - load when the account is not mutable
> - load_mut when the account is mutable



```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: AccountLoader<'info, CustomAccount>,
}
```

### Boxed

[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/boxed.rs)

- Box type to save stack space.
- Sometimes accounts are too large for the stack, leading to stack violations.

```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: Box<Account<'info, CustomAccount>>,
}
```

### Interface

[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/interface.rs)

- Type validating that the account is one of a set of given Programs

> [!NOTE]
> Program ID validation is from the set of Program IDs.

> [!TIP]
> Example of [TokenInterface](https://github.com/coral-xyz/anchor/blob/dc6ac2d631fd759339c67a25996c9570bb1e71df/spl/src/token_interface.rs#L75)

```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub program: Interface<'info, TokenInterface>,
}
```

### InterfaceAccount
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/interface_account.rs)

- Wrapper around AccountInfo that verifies program ownership (from set) and deserializes underlying data into a Rust type.

> [!NOTE]
> Owner validation is from the set of Program IDs.

> [!TIP]
> Example of [Mint Account](https://github.com/coral-xyz/anchor/blob/dc6ac2d631fd759339c67a25996c9570bb1e71df/spl/src/token_interface.rs#L58)

```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: InterfaceAccount<'info, TokenAccount>,
}
```

### Option
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/option.rs)

- Option type for optional accounts.
```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: Option<Account<'info, CustomAccount>>,
}
```

### Program
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/program.rs)

- Type validating that the account is the given Program.
```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub program: Program<'info, System>,
}
```

### Signer
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/signer.rs)

- Type validating that the account signed the transaction. No other ownership or type checks are done.
```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: Signer<'info>,
}
```

### SystemAccount
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/system_account.rs)

- Type validating that the account is owned by the system program.
```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub account: SystemAccount<'info>,
}
```

### Sysvar
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/sysvar.rs)

- Type validating that the account is a sysvar and deserializing it.
```rust
#[derive(Accounts)]
pub struct Context<'info> {
    pub sysvar: Sysvar<'info, Clock>,
}
```

### UncheckedAccount
[Source](https://github.com/coral-xyz/anchor/blob/master/lang/src/accounts/unchecked_account.rs)

> [!CAUTION]
> This type does not perform any validation checks.

- Explicit wrapper for AccountInfo types to emphasize that no checks are performed.

```rust
#[derive(Accounts)]
pub struct Context<'info> {
    /// CHECK: Explicit wrapper for AccountInfo type to emphasize that no checks are performed
    pub account: UncheckedAccount<'info>,
}
```


## Account Constraints

### Normal
[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html#normal-constraints)

#### mut

- Checks the given account is mutable.
- Custom errors are supported via @.

```rust
#[account(mut)]
```

#### init, space, payer

- Creates the account via a CPI to the system program and initializes it (sets its account discriminator).
- Marks the account as mutable and is mutually exclusive with mut.
- Makes the account rent exempt unless skipped with rent_exempt = skip.

```rust
#[account(
    init,
    payer = <target_account>,
    space = <num_bytes>
)]
```

#### init_if_needed
- Exact same functionality as the init constraint but only runs if the account does not exist yet.
- If the account does exist, it still checks whether the given init constraints are correct, e.g. that the account has the expected amount of space and, if it's a PDA, the correct seeds etc.

> [!CAUTION]
> You need to make sure you properly protect yourself against re-initialization attacks, i.e if the Account is already initialized you have to make sure fields of the Account will not be reset to initial state.

```rust
#[account(
    init_if_needed,
    payer = <target_account>,
    space = <num_bytes>
)]

```

#### seeds

- Checks that given account is a PDA derived from the currently executing program, the seeds, and if provided, the bump. If not provided, anchor uses the canonical bump.
- Add seeds::program = <expr> to derive the PDA from a different program than the currently executing one.

```rust
#[account(
    seeds = <seeds>,
    bump = <expr>,
    seeds::program = <expr>
)]
```

#### has_one

- Checks the target_account field on the account matches the key of the target_account field in the Accounts struct.
- Custom errors are supported via @.

```rust
#[account(
    has_one = <target_account> @ <custom_error>
)]
```

#### address

- Checks the account key matches the pubkey.
- Custom errors are supported via @.

```rust
#[account(
    address = <expr> @ <custom_error>
)]
```

#### owner

- Checks the account owner matches expr.
- Custom errors are supported via @.

```rust
#[account(
    owner = <expr> @ <custom_error>
)]
```

#### executable

- Checks the account is executable (i.e. the account is a program).
- You may want to use the Program type instead.

```rust
#[account(executable)]
```

#### rent_exempt

- Enforces rent exemption with = enforce.
- Skips rent exemption check that would normally be done through other constraints with = skip, e.g. when used with the zero constraint

```rust
#[account(rent_exempt = skip)]

#[account(rent_exempt = enforce)]
```

#### zero

- Checks the account DISCRIMINATOR is zero.
- Enforces rent exemption unless skipped with rent_exempt = skip.

> [!TIP]
> Use this constraint if you want to create an account in a previous instruction and then initialize it in your instruction instead of using init. This is necessary for accounts that are larger than 10 Kibibyte because those accounts cannot be created via a CPI (which is what init would do).

```rust
#[account(zero)]
```

#### close

- Closes the account by:
    - Sending the lamports to the specified account
    - Assigning the owner to the System Program
    - Resetting the data of the account

- Requires mut to exist on the account.

```rust
#[account(close = <target_account>)]
```

#### constraint

- Constraint that checks whether the given expression evaluates to true.
- Use this when no other constraint fits your use case.

```rust
#[account(constraint = <expr> @ <custom_error>)]

```

#### realloc


- The account must be marked as mut and applied to either Account or AccountLoader types.
- Change in data length is additive -> lamports are transfered from the realloc::payer.
- Change in data length is subtractive -> lamports are transfered from the data account to the realloc::payer.
- The realloc::zero constraint is required in order to determine whether the new memory should be zero initialized after reallocation. Read the documentation on the [AccountInfo::realloc](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html#method.realloc) function to understand the caveats regarding compute units when providing true/false to this flag.

> [!WARNING]
> The manual use of `AccountInfo::realloc` is discouraged in favor of the `realloc` constraint group due to the lack of native runtime checks to prevent reallocation over the `MAX_PERMITTED_DATA_INCREASE` limit (which can unintentionally cause account data overwrite other accounts). The constraint group also ensure account reallocation idempotency but checking and restricting duplicate account reallocation within a single ix.

```rust
#[account(
    realloc = <space>,
    realloc::payer = <target>,
    realloc::zero = <bool>
)]
```

### Solana Program Library
[Docs](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html#spl-constraints)

#### Token Account

- Can be used as a check or with init to create a token account with the given mint address and authority.
- When used as a check, it's possible to only specify a subset of the constraints.

```rust
use anchor_spl::{mint, token::{TokenAccount, Mint, Token}};
...

#[account(
    init,
    payer = payer,
    token::mint = mint,
    token::authority = payer,
)]
pub token: Account<'info, TokenAccount>,
#[account(address = mint::USDC)]
pub mint: Account<'info, Mint>,
#[account(mut)]
pub payer: Signer<'info>,
pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>
```

#### Mint

- Can be used as a check or with init to create a mint account with the given mint decimals and mint authority.
- The freeze authority is optional when used with init.
- When used as a check, it's possible to only specify a subset of the constraints.


```rust
use anchor_spl::token::{Mint, Token};
...

#[account(
    init,
    payer = payer,
    mint::decimals = 9,
    mint::authority = payer,
)]
pub mint_one: Account<'info, Mint>,
#[account(
    init,
    payer = payer,
    mint::decimals = 9,
    mint::authority = payer,
    mint::freeze_authority = payer
)]
pub mint_two: Account<'info, Mint>,
#[account(mut)]
pub payer: Signer<'info>,
pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>
```

#### Associated Token Account

- Can be used as a standalone as a check or with init to create an associated token account with the given mint address and authority.

```rust
use anchor_spl::{
    associated_token::AssociatedToken,
    mint,
    token::{TokenAccount, Mint, Token}
};
...

#[account(
    init,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = payer,
)]
pub token: Account<'info, TokenAccount>,
#[account(
    associated_token::mint = mint,
    associated_token::authority = payer,
)]
pub second_token: Account<'info, TokenAccount>,
#[account(address = mint::USDC)]
pub mint: Account<'info, Mint>,
#[account(mut)]
pub payer: Signer<'info>,
pub token_program: Program<'info, Token>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>
```

#### Token Program override

The token_program can optionally be overridden.

```rust
#[account(
    mint::token_program = token_program,
)]
pub mint_account: InterfaceAccount<'info, Mint>,
#[account(
    token::token_program = token_program,
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,
pub token_program: Interface<'info, TokenInterface>,
```

## Account Space

### Space Reference
[Docs](https://book.anchor-lang.com/anchor_references/space.html#space-reference)

| Type | Size (Bytes) | Description |
|------------------|----------------------------------------------------------------------------------------------------------|-------------|
| `bool` | 1 | Would only require 1 bit but still uses 1 byte |
| `u8`/`i8` | 1 | |
| `u16`/`i16` | 2 | |
| `u32`/`i32` | 4 | |
| `u64`/`i64` | 8 | |
| `u128`/`i128` | 16 | |
| `[T; amount]` | `space(T) * amount` | e.g. `space([u16; 32]) = 2 * 32 = 64` |
| `Pubkey` | 32 | |
| `Vec<T>` | `4 + (space(T) * amount)` | |
|`String` | `4 + length of string in bytes` | |
| `Option<T>` | `1 + space(T)` | |
| `Enum` | `1 + Largest Variant Size` | e.g. `Enum { A, B { val: u8 }, C { val: u16 } } -> 1 + space(u16) = 3` |
| `f32` | 4 | |
| `f64` | 8 | |


### Resizing Program Space

> [!TIP]
> - Use `realloc` constraint if you want to increase/decrease size of Data Account.
> - Use `solana program extend <PROGRAM_ID> <MORE_BYTES>` CLI command if you are not able to deploy your program due to insufficient space for deployment ("account data too small for instruction").
