# DeBridge Anchor Extensions

A utility crate that provides helpful extensions for the Anchor framework in Solana development.

## Features

- `AccountsCount`: A trait that automatically calculates the number of accounts in Anchor account structs
- Macro support for deriving `AccountsCount` implementations

## Usage

### Basic Usage
```rust
use anchor_lang::prelude::*;
use debridge_anchor_extensions::AccountsCount;

#[derive(Accounts, AccountsCount)]
pub struct MyContext<'info> {
    pub account_info: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```
The *AccountsCount* derive macro will automatically implement the *AccountsCount* trait for your struct, providing a constant *ACCOUNTS_COUNT* that represents the total number of accounts in the struct.

### Nested Contexts
The *AccountsCount* trait automatically handles nested account contexts:
```rust
use anchor_lang::prelude::*;
use debridge_anchor_extensions::AccountsCount;

#[derive(Accounts, AccountsCount)]
pub struct ParentContext<'info> {
    pub signer: Signer<'info>,
    pub nested: ChildContext<'info>,
}

#[derive(Accounts, AccountsCount)]
pub struct ChildContext<'info> {
    pub system_program: Program<'info, System>,
    pub account: AccountInfo<'info>,
}
```
The *ACCOUNTS_COUNT* constant will include accounts from both the parent and child contexts.

### Example
```rust
use anchor_lang::prelude::*;
use debridge_anchor_extensions::AccountsCount;

#[account]
pub struct AccountExample {
    pub foo: usize,
}

#[account(zero_copy)]
pub struct ZeroStruct {
    bar: usize,
}

#[derive(Accounts, AccountsCount)]
pub struct Context<'info> {
    account_info: AccountInfo<'info>,
    unchecked_account: UncheckedAccount<'info>,
    account_loader: AccountLoader<'info, ZeroStruct>,
    sysvar: Sysvar<'info, Rent>,
    account: Account<'info, AccountExample>,
    boxed: Box<Account<'info, AccountExample>>,
    program: Program<'info, System>,
    signer: Signer<'info>,
    system_account: SystemAccount<'info>,
}

// You can access the count using:
// Context::ACCOUNTS_COUNT
```
### How it Works
The *AccountsCount* trait provides a simple interface:
```rust
pub trait AccountsCount {
    const ACCOUNTS_COUNT: usize;
}
```
The derive macro automatically implements this trait for your account structures, calculating the total number of accounts by:
- Counting each account field as 1
- Recursively adding the *ACCOUNTS_COUNT* of any nested account structures
