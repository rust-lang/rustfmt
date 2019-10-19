use std::convert::{From, TryFrom};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use thiserror::Error;

pub fn define_rustfmt_config(input: syn::Item) -> TokenStream {
    match input {
        syn::Item::Struct(st) => define_rustfmt_config_on_struct(st),
        _ => panic!("expected struct"),
    }
}

fn define_rustfmt_config_on_struct(st: syn::ItemStruct) -> TokenStream {
    let attrs = &st.attrs;
    let vis = &st.vis;
    let name = &st.ident;
    let field_iter = st.fields.iter();
    let non_config_fields = field_iter
        .clone()
        .filter(|f| f.attrs.iter().find(|a| is_rustfmt_config_attr(a)).is_none());
    let config_defs: Vec<RustfmtConfigDef> = field_iter
        .filter_map(|f| RustfmtConfigDef::try_from(f).ok())
        .collect::<Vec<_>>();

    let fields = config_defs.iter().map(RustfmtConfigDef::to_field);
    let getters = config_defs.iter().map(RustfmtConfigDef::to_getter);
    let setters = config_defs.iter().map(RustfmtConfigDef::to_setter);
    let is_sets = config_defs.iter().map(RustfmtConfigDef::to_is_set);
    let is_stables = config_defs.iter().map(RustfmtConfigDef::to_is_stable);
    let validates = config_defs.iter().map(RustfmtConfigDef::to_validate);
    let override_arms = config_defs.iter().map(RustfmtConfigDef::to_override_arm);
    let is_valid_name_arms = config_defs
        .iter()
        .map(RustfmtConfigDef::to_is_valid_name_arm);
    let is_default_arms = config_defs.iter().map(RustfmtConfigDef::to_is_default_arm);
    let hash_set_inserts = config_defs.iter().map(RustfmtConfigDef::to_hash_set_insert);
    let max_name_len = config_defs
        .iter()
        .map(|cfg_def| cfg_def.name.to_string().len())
        .max()
        .unwrap();
    let print_docs = config_defs
        .iter()
        .map(|cfg_def| cfg_def.to_print_doc(max_name_len));
    let explicit_defaults = config_defs
        .iter()
        .map(RustfmtConfigDef::to_explicit_default);

    quote! {
        #(#attrs)*
        #vis struct #name {
            #(#non_config_fields),*,
            #(#fields),*
        }

        #[derive(Debug, thiserror::Error)]
        pub enum ConfigError {
            UnstableConfigOption(Vec<&'static str>),
            TomlParseError(toml::de::Error),
            TomlSerializationError(toml::ser::Error),
            InvalidKeyValuePair(String, String),
        }

        impl std::fmt::Display for ConfigError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    ConfigError::UnstableConfigOption(unstables) => {
                        write!(f, "unstable options: [")?;
                        let mut first = true;
                        for unstable in unstables {
                            if !first {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}", unstable)?;
                            first = false;
                        }
                        write!(f, "]")
                    }
                    ConfigError::TomlParseError(e) => write!(f, "failed to parse a config file: {}", e),
                    ConfigError::TomlSerializationError(e) => {
                        write!(f, "failed to serialize a config file: {}", e)
                    }
                    ConfigError::InvalidKeyValuePair(key, val) => {
                        write!(f, "{} is not valid value for {}", val, key)
                    }
                }
            }
        }


        impl #name {
            #(#getters)*
            #(#setters)*
            #(#is_stables)*
            #(#is_sets)*

            pub fn override_value(&mut self, key: &str, val: &str) -> Result<(), ConfigError> {
                match key {
                    #(#override_arms)*,
                    _ => return Err(ConfigError::InvalidKeyValuePair(
                        key.to_owned(),
                        val.to_owned()),
                    ),
                }
                Ok(())
            }

            pub(crate) fn validate(&self) -> Result<(), ConfigError> {
                if is_nightly_channel!() {
                    return Ok(());
                }

                let mut errors = vec![];
                #(#validates)*
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(ConfigError::UnstableConfigOption(errors))
                }
            }

            pub fn print_docs(
                out: &mut dyn std::io::Write,
                include_unstable: bool,
            ) -> std::io::Result<()> {
                #(#print_docs)*
                Ok(())
            }

            pub fn is_valid_key_val(_key: &str, _val: &str) -> bool { true }

            pub fn all_options() -> Config {
                Config {
                    #(#explicit_defaults),*,
                    ..Default::default()
                }
            }

            pub fn is_valid_name(name: &str) -> bool {
                match name {
                    #(#is_valid_name_arms),*,
                    _ => false,
                }
            }

            #[cfg(test)]
            pub fn is_default(&self, key: &str) -> bool {
                match key {
                    #(#is_default_arms),*,
                    _ => false,
                }
            }

            #[cfg(test)]
            pub fn hash_set() -> std::collections::HashSet<&'static str> {
                let mut hashset = std::collections::HashSet::new();
                #(#hash_set_inserts)*
                hashset
            }
        }
    }
}

fn is_rustfmt_config_attr(attr: &syn::Attribute) -> bool {
    attr.path.segments.len() == 2
        && attr.path.segments[0].ident == "rustfmt"
        && attr.path.segments[1].ident == "config"
}

#[derive(Error, Debug)]
enum RustfmtConfigConvertError {
    #[error("rustfmt::config not found")]
    AttributeNotFound,
    #[error("rustfmt::config must be list")]
    AttributeNotList,
    #[error("rustfmt::config list must consists of meta lists with a single argument")]
    InvalidListItems,
    #[error("rustfmt::config list does not have default")]
    NoDefault,
}

struct RustfmtConfigDef<'a> {
    name: &'a syn::Ident,
    ty: &'a syn::Type,
    default_value: syn::NestedMeta,
    attrs: &'a [syn::Attribute],
    stable_version: Option<syn::LitStr>,
    custom_setter: Option<syn::Path>,
    doc_comment: String,
}

impl<'a> RustfmtConfigDef<'a> {
    fn to_field(&self) -> TokenStream {
        let name = self.name;
        let ty = self.ty;
        let attrs = self.attrs.iter().filter(|a| is_rustfmt_config_attr(a));

        quote! { #(#attrs)* #name: Option<#ty> }
    }

    // FIXME: Avoid cloning if possible.
    fn to_getter(&self) -> TokenStream {
        let name = self.name;
        let ty = self.ty;
        let default = &self.default_value;
        let doc = format!("Returns the current {}.", name);

        quote! {
            #[doc = #doc]
            pub fn #name(&self) -> #ty {
                self.#name.clone().unwrap_or(#default)
            }
        }
    }

    fn setter_name(name: &syn::Ident) -> syn::Ident {
        format_ident!("set_{}", name)
    }

    fn to_setter(&self) -> TokenStream {
        let name = self.name;
        let setter_name = Self::setter_name(name);
        let ty = self.ty;
        let doc = format!("Set {} to the given value.", name);

        if let Some(ref setter) = self.custom_setter {
            quote! {
                #[doc = #doc]
                pub fn #setter_name(&mut self, val: #ty) {
                    self.#setter(val);
                }
            }
        } else {
            quote! {
                #[doc = #doc]
                pub fn #setter_name(&mut self, new_val: #ty) -> Option<#ty> {
                    self.#name.replace(new_val)
                }
            }
        }
    }

    fn to_is_set(&self) -> TokenStream {
        let name = self.name;
        let is_set_name = format_ident!("is_{}_set", name);
        let doc = format!("Return true if {} has been explicitly set.", name);

        quote! {
            #[doc = #doc]
            pub fn #is_set_name(&self) -> bool {
                self.#name.is_some()
            }
        }
    }

    fn is_stable_name(&self) -> syn::Ident {
        format_ident!("is_{}_stable", self.name)
    }

    fn to_is_stable(&self) -> TokenStream {
        let name = self.name;
        let is_stable_name = self.is_stable_name();
        let doc = format!("Returns true if {} is stable", name);
        let is_stable = self.stable_version.is_some();

        quote! {
            #[doc = #doc]
            pub fn #is_stable_name() -> bool {
                #is_stable
            }
        }
    }

    fn to_validate(&self) -> TokenStream {
        let is_set_name = format_ident!("is_{}_set", self.name);
        let is_stable_name = self.is_stable_name();
        let name = format!("{}", self.name);
        quote! {
            if self.#is_set_name() && !Self::#is_stable_name() {
                errors.push(#name);
            }
        }
    }

    fn to_explicit_default(&self) -> TokenStream {
        let name = self.name;
        let default = &self.default_value;
        quote! {
            #name: Some(#default)
        }
    }

    fn to_override_arm(&self) -> TokenStream {
        let pattern = format!("{}", self.name);
        let ty = self.ty;
        let setter = Self::setter_name(self.name);

        quote! {
            #pattern => match val.parse::<#ty>() {
                Ok(v) => {
                    self.#setter(v);
                }
                Err(e) => return Err(ConfigError::InvalidKeyValuePair(
                    key.to_owned(),
                    val.to_owned()),
                ),
            }
        }
    }

    fn to_is_valid_name_arm(&self) -> TokenStream {
        let pattern = format!("{}", self.name);
        quote! {
            #pattern => true
        }
    }

    fn to_is_default_arm(&self) -> TokenStream {
        let pattern = format!("{}", self.name);
        let name = self.name;
        quote! {
            #pattern => self.#name.is_none()
        }
    }

    fn to_hash_set_insert(&self) -> TokenStream {
        let name = format!("{}", self.name);
        quote! {
            hashset.insert(#name);
        }
    }

    fn to_print_doc(&self, max_name_len: usize) -> TokenStream {
        let is_stable = self.is_stable_name();
        let name = format!("{}", self.name);
        let space_len = max_name_len - name.len();
        let spaces = " ".repeat(space_len);
        let next_line_spaces = " ".repeat(max_name_len - 1);
        let default_value = &self.default_value;
        let ty = self.ty;
        let unstable = if self.stable_version.is_none() {
            " (unstable)"
        } else {
            ""
        };
        let config_doc = &self.doc_comment;

        quote! {
            if Self::#is_stable() || include_unstable {
                write!(out, #spaces)?;
                write!(out, #name)?;
                writeln!(out, " {} Default: {}{}", <#ty>::doc_hint(), #default_value, #unstable)?;
                write!(out, #next_line_spaces)?;
                write!(out, " ")?;
                writeln!(out, #config_doc)?;
                writeln!(out)?;
            }
        }
    }
}

impl<'a> From<&'a RustfmtConfigDef<'a>> for ConfigDefault<'a> {
    fn from(def: &'a RustfmtConfigDef<'a>) -> Self {
        ConfigDefault {
            name: def.name,
            default_value: &def.default_value,
        }
    }
}

struct ConfigDefault<'a> {
    name: &'a syn::Ident,
    default_value: &'a syn::NestedMeta,
}

impl<'a> ToTokens for ConfigDefault<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let default_value = &self.default_value;
        let name = &self.name;
        tokens.extend(quote! {
            #name: #default_value
        });
    }
}

impl<'a> TryFrom<&'a syn::Field> for RustfmtConfigDef<'a> {
    type Error = RustfmtConfigConvertError;

    fn try_from(f: &'a syn::Field) -> Result<Self, Self::Error> {
        let name = f.ident.as_ref().expect("field without name");
        let meta =
            extract_rustfmt_config_attr(f).ok_or(RustfmtConfigConvertError::AttributeNotFound)?;
        let nested_meta_list = match meta {
            syn::Meta::List(meta_list) => meta_list,
            _ => return Err(RustfmtConfigConvertError::AttributeNotList),
        };
        let config_values = nested_meta_list
            .nested
            .iter()
            .map(|nested| match nested {
                syn::NestedMeta::Lit(..) => Err(RustfmtConfigConvertError::InvalidListItems),
                syn::NestedMeta::Meta(nested_meta) => match nested_meta {
                    syn::Meta::List(inner_list) if inner_list.nested.len() == 1 => {
                        Ok((&inner_list.path, inner_list.nested.first().unwrap()))
                    }
                    _ => Err(RustfmtConfigConvertError::InvalidListItems),
                },
            })
            .collect::<Result<Vec<_>, _>>()?;

        let &default_value = config_values
            .iter()
            .find_map(|(p, q)| if p.is_ident("default") { Some(q) } else { None })
            .ok_or(RustfmtConfigConvertError::NoDefault)?;
        let custom_setter = config_values
            .iter()
            .find_map(|(p, q)| if p.is_ident("setter") { Some(*q) } else { None })
            .cloned()
            .and_then(nested_meta_to_path);
        let stable_version = config_values
            .iter()
            .find_map(|(p, q)| if p.is_ident("stable") { Some(*q) } else { None })
            .cloned()
            .and_then(|nm| nested_meta_to_lit_str(nm));
        let doc_comment = f
            .attrs
            .iter()
            .filter_map(doc_comment)
            .collect::<Vec<String>>()
            .join("");
        if doc_comment.trim().is_empty() {
            panic!("doc comment for {} does not exist", name)
        }

        Ok(RustfmtConfigDef {
            name,
            default_value: default_value.clone(),
            ty: &f.ty,
            attrs: &f.attrs,
            stable_version,
            custom_setter,
            doc_comment,
        })
    }
}

fn doc_comment(attr: &syn::Attribute) -> Option<String> {
    match attr.parse_meta().ok()? {
        syn::Meta::NameValue(syn::MetaNameValue {
            lit: syn::Lit::Str(lit_str),
            ..
        }) => Some(lit_str.value()),
        _ => None,
    }
}

fn nested_meta_to_lit_str(nm: syn::NestedMeta) -> Option<syn::LitStr> {
    match nm {
        syn::NestedMeta::Lit(syn::Lit::Str(lit_str)) => Some(lit_str),
        _ => None,
    }
}

fn nested_meta_to_path(nm: syn::NestedMeta) -> Option<syn::Path> {
    match nm {
        syn::NestedMeta::Meta(syn::Meta::Path(p)) => Some(p),
        _ => None,
    }
}

fn extract_rustfmt_config_attr(f: &syn::Field) -> Option<syn::Meta> {
    f.attrs.iter().find_map(|attr| {
        if is_rustfmt_config_attr(attr) {
            attr.parse_meta().ok()
        } else {
            None
        }
    })
}
