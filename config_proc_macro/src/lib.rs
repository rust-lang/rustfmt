//! This crate provides a derive macro for `ConfigType`.

#![recursion_limit = "256"]

extern crate proc_macro;

mod attrs;
mod config_type;
mod define_config;
mod item_enum;
mod item_struct;
mod utils;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn config_type(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Item);
    let output = config_type::define_config_type(&input);

    #[cfg(all(test, feature = "debug-with-rustfmt"))]
    {
        utils::debug_with_rustfmt(&output);
    }

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn rustfmt_config(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Item);
    let output = define_config::define_rustfmt_config(input);

    #[cfg(all(test, feature = "debug-with-rustfmt"))]
    {
        utils::debug_with_rustfmt(&output);
    }

    TokenStream::from(output)
}
