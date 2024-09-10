pub use debridge_anchor_extensions_macro::*;

pub trait AccountsCount {
    const ACCOUNTS_COUNT: usize;
}

#[cfg(feature = "anchor30")]
pub use conv::TryIntoAccount;
#[cfg(feature = "anchor30")]
mod conv {
    use anchor_lang::{
        error::Error,
        prelude::{Account, AccountInfo},
        AccountDeserialize, AccountSerialize, Owner,
    };

    pub trait TryIntoAccount {
        type Error;
        fn try_into_account<T: AccountSerialize + AccountDeserialize + Clone + Owner>(
            &self,
        ) -> std::result::Result<Account<T>, Self::Error>;

        fn try_into_account_unchecked<T: AccountSerialize + AccountDeserialize + Clone + Owner>(
            &self,
        ) -> std::result::Result<Account<T>, Self::Error>;
    }

    impl<'info> TryIntoAccount for AccountInfo<'info> {
        type Error = Error;

        fn try_into_account<T: AccountSerialize + AccountDeserialize + Clone + Owner>(
            &self,
        ) -> std::result::Result<Account<T>, Self::Error> {
            Account::try_from(unsafe { std::mem::transmute(self) })
        }

        fn try_into_account_unchecked<T: AccountSerialize + AccountDeserialize + Clone + Owner>(
            &self,
        ) -> std::result::Result<Account<T>, Self::Error> {
            Account::try_from_unchecked(unsafe { std::mem::transmute(self) })
        }
    }
}
