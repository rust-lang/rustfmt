use proc_macro2::TokenStream;

use crate::args::Args;

pub fn define_config_type_on_struct(
    _args: &Args,
    _st: &syn::ItemStruct,
) -> syn::Result<TokenStream> {
    unimplemented!()
}
