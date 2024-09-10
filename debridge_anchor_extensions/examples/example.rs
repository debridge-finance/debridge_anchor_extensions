use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use debridge_anchor_extensions::AccountsCount;

pub const ID: Pubkey = Pubkey::new_from_array([0u8; 32]);
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

#[derive(Accounts, AccountsCount)]
pub struct NestedContext<'info> {
    account_info: AccountInfo<'info>,
    unchecked_account: UncheckedAccount<'info>,
    account_loader: AccountLoader<'info, ZeroStruct>,
    sysvar: Sysvar<'info, Rent>,
    account: Account<'info, AccountExample>,
    boxed: Box<Account<'info, AccountExample>>,
    program: Program<'info, System>,
    signer: Signer<'info>,
    system_account: SystemAccount<'info>,

    inner: Context<'info>,
}

fn main() {
    assert_eq!(__client_accounts_context::Context::ACCOUNTS_COUNT, 9);
    assert_eq!(
        __client_accounts_nested_context::NestedContext::ACCOUNTS_COUNT,
        18
    );
}
