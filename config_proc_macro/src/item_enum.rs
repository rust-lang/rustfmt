use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

use crate::args::Args;
use crate::attrs::*;
use crate::utils::*;

type Variants = syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>;

/// Defines and implements `config_type` enum.
pub fn define_config_type_on_enum(args: &Args, em: &syn::ItemEnum) -> syn::Result<TokenStream> {
    let syn::ItemEnum {
        vis,
        enum_token,
        ident,
        generics,
        variants,
        ..
    } = em;

    let mod_name_str = format!("__define_config_type_on_enum_{}", ident);
    let mod_name = syn::Ident::new(&mod_name_str, ident.span());
    let variants = fold_quote(variants.iter().map(process_variant), |meta| quote!(#meta,));
    let mut has_serde = false;
    let derives = [
        "std::fmt::Debug",
        "std::clone::Clone",
        "std::marker::Copy",
        "std::cmp::Eq",
        "std::cmp::PartialEq",
        "serde::Serialize",
        "serde::Deserialize",
    ]
    .iter()
    .filter(|d| args.skip_derives().all(|s| !d.ends_with(s)))
    .inspect(|d| has_serde |= d.contains("serde::"))
    .map(|d| syn::parse_str(d).unwrap())
    .collect::<Vec<syn::Path>>();
    let derives = derives
        .is_empty()
        .then(|| quote!())
        .unwrap_or_else(|| quote! { #[derive( #( #derives ),* )] });
    let serde_attr = has_serde
        .then(|| quote!(#[serde(rename_all = "PascalCase")]))
        .unwrap_or_default();

    let impl_doc_hint = impl_doc_hint(&em.ident, &em.variants);
    let impl_from_str = impl_from_str(&em.ident, &em.variants);
    let impl_display = impl_display(&em.ident, &em.variants);

    Ok(quote! {
        #[allow(non_snake_case)]
        mod #mod_name {
            #derives
            #serde_attr
            pub #enum_token #ident #generics { #variants }
            #impl_display
            #impl_doc_hint
            #impl_from_str
        }
        #vis use #mod_name::#ident;
    })
}

/// Remove attributes specific to `config_proc_macro` from enum variant fields.
fn process_variant(variant: &syn::Variant) -> TokenStream {
    let metas = variant
        .attrs
        .iter()
        .filter(|attr| !is_doc_hint(attr) && !is_unstable_variant(attr));
    let attrs = fold_quote(metas, |meta| {
        if let Some(rename) = config_value(meta) {
            quote!(#[serde(rename = #rename)])
        } else {
            quote!(#meta)
        }
    });
    let syn::Variant { ident, fields, .. } = variant;
    quote!(#attrs #ident #fields)
}

/// Return the correct syntax to pattern match on the enum variant, discarding all
/// internal field data.
fn fields_in_variant(variant: &syn::Variant) -> TokenStream {
    // With thanks to https://stackoverflow.com/a/65182902
    match &variant.fields {
        syn::Fields::Unnamed(_) => quote_spanned! { variant.span() => (..) },
        syn::Fields::Unit => quote_spanned! { variant.span() => },
        syn::Fields::Named(_) => quote_spanned! { variant.span() => {..} },
    }
}

fn impl_doc_hint(ident: &syn::Ident, variants: &Variants) -> TokenStream {
    let doc_hint = variants
        .iter()
        .map(doc_hint_of_variant)
        .collect::<Vec<_>>()
        .join("|");
    let doc_hint = format!("[{}]", doc_hint);

    let variant_stables = variants
        .iter()
        .map(|v| (&v.ident, fields_in_variant(&v), !unstable_of_variant(v)));
    let match_patterns = fold_quote(variant_stables, |(v, fields, stable)| {
        quote! {
            #ident::#v #fields => #stable,
        }
    });
    quote! {
        use crate::config::ConfigType;
        impl ConfigType for #ident {
            fn doc_hint() -> String {
                #doc_hint.to_owned()
            }
            fn stable_variant(&self) -> bool {
                match self {
                    #match_patterns
                }
            }
        }
    }
}

fn impl_display(ident: &syn::Ident, variants: &Variants) -> TokenStream {
    let match_patterns = fold_quote(variants, |v| {
        let variant = &v.ident;
        match &v.fields {
            syn::Fields::Unit => {
                let s = config_value_of_variant(v);
                quote! {
                    #ident::#variant => write!(f, "{}", #s),
                }
            }
            syn::Fields::Named(x) if x.named.len() == 1 => {
                let x = &x.named[0];
                quote! {
                    #ident::#variant { #x } => write!(f, "{}", #x),
                }
            }
            syn::Fields::Named(x) => unimplemented!(
                "cannot implement display for variant with {} named fileds",
                x.named.len()
            ),
            syn::Fields::Unnamed(x) if x.unnamed.len() == 1 => {
                quote! {
                    #ident::#variant(x) => write!(f, "{}", x),
                }
            }
            syn::Fields::Unnamed(x) => {
                unimplemented!(
                    "cannot implement display for variant with {} named fileds",
                    x.unnamed.len()
                )
            }
        }
    });
    quote! {
        use std::fmt;
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    #match_patterns
                    _ => unimplemented!(),
                }
            }
        }
    }
}

fn impl_from_str(ident: &syn::Ident, variants: &Variants) -> TokenStream {
    let vs = variants
        .iter()
        .filter(|v| is_unit(v))
        .map(|v| (config_value_of_variant(v), &v.ident));
    let if_patterns = fold_quote(vs, |(s, v)| {
        quote! {
            if #s.eq_ignore_ascii_case(s) {
                return Ok(#ident::#v);
            }
        }
    });
    let mut err_msg = String::from("Bad variant, expected one of:");
    for v in variants.iter().filter(|v| is_unit(v)) {
        err_msg.push_str(&format!(" `{}`", v.ident));
    }

    quote! {
        impl ::std::str::FromStr for #ident {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #if_patterns
                return Err(#err_msg);
            }
        }
    }
}

fn doc_hint_of_variant(variant: &syn::Variant) -> String {
    let mut text = find_doc_hint(&variant.attrs).unwrap_or(variant.ident.to_string());
    if unstable_of_variant(&variant) {
        text.push_str(" (unstable)")
    };
    text
}

fn config_value_of_variant(variant: &syn::Variant) -> String {
    find_config_value(&variant.attrs).unwrap_or(variant.ident.to_string())
}

fn unstable_of_variant(variant: &syn::Variant) -> bool {
    any_unstable_variant(&variant.attrs)
}
