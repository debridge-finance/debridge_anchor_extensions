use anchor_lang::prelude::*;
use anchor_lang::{AccountDeserialize, AccountSerialize, Owner};
pub use debridge_anchor_extensions_macro::*;
use std::mem::transmute;

pub trait AccountsCount {
    const ACCOUNTS_COUNT: usize;
}

pub trait AccountFromInfo {
    fn try_from_info<'a>(info: &AccountInfo<'a>) -> Result<Account<'a, Self>>
    where
        Self: AccountSerialize + AccountDeserialize + Owner + Clone;
    fn try_from_info_unchecked<'a>(info: &AccountInfo<'a>) -> Result<Account<'a, Self>> where
        Self: AccountSerialize + AccountDeserialize + Owner + Clone;
}

impl<T: AccountSerialize + AccountDeserialize + Owner + Clone> AccountFromInfo for T {
    /// Deserializes the given `info` into a `Account`.
    #[inline(never)]
    fn try_from_info<'a>(info: &AccountInfo<'a>) -> Result<Account<'a, T>> {
        Account::try_from(unsafe { transmute(info) })
    }

    /// Deserializes the given `info` into a `Account` without checking
    /// the account discriminator. Be careful when using this and avoid it if
    /// possible.
    #[inline(never)]
    fn try_from_info_unchecked<'a>(info: &AccountInfo<'a>) -> Result<Account<'a, T>> {
        Account::try_from_unchecked(unsafe { transmute(info) })
    }
}
