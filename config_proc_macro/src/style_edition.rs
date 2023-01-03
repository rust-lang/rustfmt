use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

use crate::attrs;

/// Returns `true` if the given attribute configures the deafult StyleEdition value
pub fn se_default(attr: &syn::Attribute) -> bool {
    attrs::is_attr_path(attr, "se_default")
}

/// Returns `true` if the given attribute configures the deafult value for StyleEdition2015
pub fn se_2015(attr: &syn::Attribute) -> bool {
    attrs::is_attr_path(attr, "se_2015")
}

/// Returns `true` if the given attribute configures the deafult value for StyleEdition2018
pub fn se_2018(attr: &syn::Attribute) -> bool {
    attrs::is_attr_path(attr, "se_2018")
}

/// Returns `true` if the given attribute configures the deafult value for StyleEdition2021
pub fn se_2021(attr: &syn::Attribute) -> bool {
    attrs::is_attr_path(attr, "se_2021")
}

/// Returns `true` if the given attribute configures the deafult for StyleEdition2024
pub fn se_2024(attr: &syn::Attribute) -> bool {
    attrs::is_attr_path(attr, "se_2024")
}

/// Defines `style_edition` on enum or struct.
pub fn define_style_edition(
    defaults: StyleEditionDefault,
    item: syn::Item,
) -> syn::Result<TokenStream> {
    match item {
        syn::Item::Struct(st) => define_style_edition_struct(defaults, st),
        syn::Item::Enum(en) => define_style_edition_enum(defaults, en),
        _ => panic!("Expected enum or struct"),
    }
}

pub struct StyleEditionDefault {
    default: Option<syn::Expr>,
    se2015: Option<syn::Expr>,
    se2018: Option<syn::Expr>,
    se2021: Option<syn::Expr>,
    se2024: Option<syn::Expr>,
}

impl StyleEditionDefault {
    /// a sinlge default for all style editions
    fn single_default(&self) -> bool {
        self.default.is_some()
            && self.se2015.is_none()
            && self.se2018.is_none()
            && self.se2021.is_none()
            && self.se2024.is_none()
    }
    /// Infer the type from the default value
    fn ty_from_default(&self) -> syn::Result<syn::Type> {
        match &self.default {
            Some(syn::Expr::Lit(lit)) => match lit.lit {
                syn::Lit::Bool(_) => {
                    return Ok(syn::TypePath {
                        qself: None,
                        path: path_from_str("bool"),
                    }
                    .into());
                }
                syn::Lit::Int(_) => {
                    return Ok(syn::TypePath {
                        qself: None,
                        path: path_from_str("usize"),
                    }
                    .into());
                }
                _ => {}
            },
            _ => {}
        }
        Err(syn::parse::Error::new(
            Span::call_site(),
            "could not determine type from default value",
        ))
    }

    fn enum_expr_path(varient: &syn::Variant, en: &syn::ItemEnum) -> syn::ExprPath {
        let mut path = path_from_ident(&en.ident);
        path.segments.push(varient.ident.clone().into());
        syn::ExprPath {
            attrs: vec![],
            qself: None,
            path,
        }
    }

    /// Set the style edition based on the the annotated attribute
    /// For example:
    /// ```ignore
    /// #[style_edition]
    /// enum Example {
    ///     #[se_default] // <-- Default style edition
    ///     A,
    ///     #[se_2018] // <-- Explicit override for StypeEdition2018
    ///     B,
    /// }
    /// ```
    fn set_defaults_by_enum_variant_attr(&mut self, en: &syn::ItemEnum) {
        for varient in en.variants.iter() {
            for attr in varient.attrs.iter() {
                if se_default(attr) {
                    self.default.replace(Self::enum_expr_path(varient, en).into());
                    break;
                } else if se_2015(attr) {
                    self.se2015.replace(Self::enum_expr_path(varient, en).into());
                    break;
                } else if se_2018(attr) {
                    self.se2018.replace(Self::enum_expr_path(varient, en).into());
                    break;
                } else if se_2021(attr) {
                    self.se2021.replace(Self::enum_expr_path(varient, en).into());
                    break;
                } else if se_2024(attr) {
                    self.se2024.replace(Self::enum_expr_path(varient, en).into());
                    break;
                }
            }
        }
    }

    /// Set the style edition based on the lhs of the assignment in the attribute
    /// e.g. `#[style_edition(true, se_2015=false)]`
    fn set_by_assignment(&mut self, assignment: &syn::ExprAssign) -> syn::Result<()> {
        match assignment.left.as_ref() {
            syn::Expr::Path(expr) => {
                let se2015 = syn::Ident::new("se_2015", Span::call_site());
                let se2018 = syn::Ident::new("se_2018", Span::call_site());
                let se2021 = syn::Ident::new("se_2021", Span::call_site());
                let se2024 = syn::Ident::new("se_2024", Span::call_site());
                let ident = expr
                    .path
                    .segments
                    .first()
                    .map(|segment| segment.ident.clone())
                    .expect("should be at least one ident");

                if ident == se2015 {
                    self.se2015.replace(*assignment.right.clone());
                    return Ok(());
                } else if ident == se2018 {
                    self.se2018.replace(*assignment.right.clone());
                    return Ok(());
                } else if ident == se2021 {
                    self.se2021.replace(*assignment.right.clone());
                    return Ok(());
                } else if ident == se2024 {
                    self.se2024.replace(*assignment.right.clone());
                    return Ok(());
                }
            }
            _ => {}
        }
        Err(syn::Error::new(
            Span::call_site(),
            format!(
                "Unknown lhs {:?}",
                assignment.left.as_ref().to_token_stream().to_string()
            ),
        ))
    }

    fn quote(&self, name: &syn::Ident) -> TokenStream {
        let default = self.default.as_ref();
        let se2015 = self.se2015.as_ref().map(|expr| {
            quote! {
                if #name == crate::config::StyleEdition::Edition2015 {
                    return #expr;
                }
            }
        });
        let se2018 = self.se2018.as_ref().map(|expr| {
            quote! {
                if #name == crate::config::StyleEdition::Edition2018 {
                    return #expr;
                }
            }
        });
        let se2021 = self.se2021.as_ref().map(|expr| {
            quote! {
                if #name == crate::config::StyleEdition::Edition2021 {
                    return #expr;
                }
            }
        });
        let se2024 = self.se2024.as_ref().map(|expr| {
            quote! {
                if #name == crate::config::StyleEdition::Edition2024 {
                    return #expr;
                }
            }
        });
        quote! {
            #se2015
            #se2018
            #se2021
            #se2024
            #default
        }
    }
}

fn path_from_str(s: &str) -> syn::Path {
    syn::Path::from(syn::Ident::new(s, Span::call_site()))
}

fn path_from_ident(ident: &syn::Ident) -> syn::Path {
    syn::Path::from(ident.clone())
}

impl Default for StyleEditionDefault {
    fn default() -> Self {
        Self {
            default: None,
            se2015: None,
            se2018: None,
            se2021: None,
            se2024: None,
        }
    }
}

/// Parse StyleEdition values from attribute macro.
/// For example: `#[style_edition(100)]`, which sets the defaul to 100 for all style edtions
/// or `#[style_edition(false, se_2024=true)]`, which sets the default for all style editions except
/// `StyleEdition2024` to false, and explicitly sets `StyleEdition2024=true`
impl Parse for StyleEditionDefault {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut se_default = StyleEditionDefault::default();
        if input.is_empty() {
            return Ok(se_default);
        }
        let defaults = input.parse_terminated(syn::Expr::parse, Token![,])?;
        for (idx, pair) in defaults.into_pairs().enumerate() {
            let expr = pair.into_value();
            match &expr {
                syn::Expr::Assign(assign) => {
                    if idx == 0 {
                        se_default.default.replace(*assign.right.to_owned());
                        continue;
                    }
                    se_default.set_by_assignment(assign)?;
                }
                syn::Expr::Lit(_) if idx == 0 => {
                    se_default.default.replace(expr);
                }
                syn::Expr::Path(_) if idx == 0 => {
                    se_default.default.replace(expr);
                }
                _ => {
                    return Err(syn::parse::Error::new(
                        expr.span(),
                        format!(
                            "Can't create a style edition default from the expr: {:?}",
                            expr.to_token_stream().to_string()
                        ),
                    ));
                }
            }
        }
        Ok(se_default)
    }
}

fn define_style_edition_struct(
    defaults: StyleEditionDefault,
    st: syn::ItemStruct,
) -> syn::Result<TokenStream> {
    let ty = defaults.ty_from_default()?;
    let ident = st.ident.clone();
    define_style_edition_inner(defaults, ty, ident, st.into())
}

fn define_style_edition_enum(
    mut defaults: StyleEditionDefault,
    mut en: syn::ItemEnum,
) -> syn::Result<TokenStream> {
    let ty = syn::TypePath {
        qself: None,
        path: syn::Path::from(en.ident.clone()),
    };

    let ident = en.ident.clone();
    defaults.set_defaults_by_enum_variant_attr(&en);
    for mut variant in en.variants.iter_mut() {
        remove_style_edition_attrs(&mut variant);
    }
    define_style_edition_inner(defaults, ty.into(), ident, en.into())
}

/// Remove attributes specific to `style_edition` from enum variant fields.
/// These attributes are only used as markers to help us generate `StyleEditionDefault`
/// trait implementations. They should be removed to avoid compilation errors.
fn remove_style_edition_attrs(variant: &mut syn::Variant) {
    let metas = variant
        .attrs
        .iter()
        .filter(|attr| {
            !se_default(attr)
                && !se_2015(attr)
                && !se_2018(attr)
                && !se_2021(attr)
                && !se_2024(attr)
        })
        .cloned()
        .collect();

    variant.attrs = metas;
}

fn define_style_edition_inner(
    defaults: StyleEditionDefault,
    ty: syn::Type,
    ident: syn::Ident,
    item: syn::Item,
) -> syn::Result<TokenStream> {
    if defaults.default.is_none() {
        return Err(syn::Error::new(
            Span::call_site(),
            format!("Missing default style edition value for {:?}", ident),
        ));
    }

    let name = if defaults.single_default() {
        syn::Ident::new("_se", Span::call_site())
    } else {
        syn::Ident::new("style_edition", Span::call_site())
    };

    let value = defaults.quote(&name);

    Ok(quote! {
        #item

        impl crate::config::StyleEditionDefault for #ident {
            type ConfigType = #ty;
            fn style_edition_default(#name: crate::config::StyleEdition) -> Self::ConfigType {
                #value
            }
        }
    })
}
