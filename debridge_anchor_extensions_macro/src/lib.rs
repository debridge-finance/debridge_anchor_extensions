extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use anchor_syn::{AccountField, AccountsStruct, CompositeField};
use heck::ToSnakeCase;

fn get_symbol_full_path(s: &CompositeField) -> proc_macro2::TokenStream {
    format!(
        "__client_accounts_{0}::{1}",
        s.symbol.to_snake_case(),
        s.symbol,
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(AccountsCount)]
pub fn accounts_count_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let accs = parse_macro_input!(input as AccountsStruct);
    let name = &accs.ident;
    let path: proc_macro2::TokenStream = format!(
        "__client_accounts_{}::{}",
        accs.ident.to_string().to_snake_case(),
        name,
    )
    .parse()
    .unwrap();
    let accounts_count = accs.fields.iter().fold(
        vec![quote! { 0 }],
        |mut accounts_count, f: &AccountField| {
            match f {
                AccountField::CompositeField(s) => {
                    let symbol: proc_macro2::TokenStream = get_symbol_full_path(s);
                    accounts_count.push(quote! {
                        <#symbol as debridge_anchor_extensions::AccountsCount>::ACCOUNTS_COUNT
                    });
                }
                AccountField::Field(_) => {
                    accounts_count.push(quote! { 1 });
                }
            };
            accounts_count
        },
    );
    let token_stream = quote! {
        #[automatically_derived]
        impl debridge_anchor_extensions::AccountsCount for #path {
            const ACCOUNTS_COUNT: usize = #(#accounts_count)+*;
        }
    };

    TokenStream::from(token_stream)
}
