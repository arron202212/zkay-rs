#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

//Modified based on https://github.com/fitzgen/derive_is_enum_variant
extern crate heck;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, Expr, Fields, Ident, Lit, Meta, Token};

// #[proc_macro_derive(is_enum_variant, attributes(is_enum_variant))]
// pub fn derive_is_enum_variant(tokens: TokenStream) -> TokenStream {
//     let source = tokens.to_string();

//     let ast = syn::parse_derive_input(&source).expect("should parse input tokens into AST");

//     let expanded = expand_derive_is_enum_variant(&ast);

//     expanded
//         .parse()
//         .expect("should parse expanded output source into tokens")
// }

enum PredicateConfig {
    None,
    Skip,
    Name(String),
}

impl PredicateConfig {
    fn join(self, meta: &syn::Meta) -> Self {
        match meta {
            syn::Meta::Path(ref ident) if ident.is_ident("skip") => match self {
                PredicateConfig::None | PredicateConfig::Skip => PredicateConfig::Skip,
                PredicateConfig::Name(_) => panic!(
                    "Cannot both `#[is_enum_variant(skip)]` and \
                     `#[is_enum_variant(name = \"..\")]`"
                ),
            },
            syn::Meta::NameValue(meta_name_value) if meta_name_value.path.is_ident("name") => {
                let mut s = String::new();
                if let Expr::Lit(expr) = &meta_name_value.value {
                    if let Lit::Str(lit_str) = &expr.lit {
                        s = lit_str.value();
                    }
                }
                if s.chars().any(|c| c != '_' && !c.is_ascii_alphanumeric()) {
                    panic!(
                        "#[is_enum_variant(name = \"..\")] must be provided \
                         a valid identifier"
                    )
                }
                match self {
                    PredicateConfig::None => PredicateConfig::Name(s.to_string()),
                    PredicateConfig::Skip => panic!(
                        "Cannot both `#[is_enum_variant(skip)]` and \
                         `#[is_enum_variant(name = \"..\")]`"
                    ),
                    PredicateConfig::Name(_) => panic!(
                        "Cannot provide more than one \
                         `#[is_enum_variant(name = \"..\")]`"
                    ),
                }
            }
            _ => panic!(
                "Unknown item inside `#[is_enum_variant(..)]`: {:?}",
                "other"
            ),
        }
    }
}

impl<'a> From<&'a Vec<syn::Attribute>> for PredicateConfig {
    fn from(attrs: &'a Vec<syn::Attribute>) -> Self {
        let our_attr = attrs.iter().find(|a| a.path().is_ident("is_enum_variant"));
        our_attr.map_or(PredicateConfig::None, |attr| match attr.meta {
            syn::Meta::List(_) => {
                let nested = attr
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .unwrap();
                nested
                    .iter()
                    //                 .map(|m|
                    //  match *m {
                    //                     syn::Meta::List(ref m) => m,
                    //                     _ => panic!("Invalid #[is_enum_variant] item"),
                    //                 }
                    //     )
                    .fold(PredicateConfig::None, PredicateConfig::join)
            }
            _ => panic!(
                "#[is_enum_variant] must be used with name/value pairs, like \
                 #[is_enum_variant(name = \"..\")]"
            ),
        })
    }
}

pub fn expand_derive_is_enum_variant(ast: &syn::DeriveInput) -> TokenStream {
    let variants = if let syn::Data::Enum(ref variants) = ast.data {
        variants
    } else {
        panic!("#[derive(is_enum_variant)] can only be used with enums");
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let predicates = variants.variants.iter().map(
        |&syn::Variant {
             ref ident,
             ref fields,
             ref attrs,
             ..
         }| {
            let cfg = attrs.into();
            if let PredicateConfig::Skip = cfg {
                return quote! {};
            }

            let variant_name = ident.to_string();
            let doc = format!("Is this `{}` a `{}`?", name, variant_name);

            let predicate_name = if let PredicateConfig::Name(name) = cfg {
                name
            } else {
                let mut name = String::from("is_");
                name.push_str(&variant_name.to_snake_case());
                name
            };
            let predicate_name = format_ident!("{}", predicate_name);

            let data_tokens = match fields {
                Fields::Named(_) => quote! { { .. } },
                Fields::Unnamed(_) => quote! { (..) },
                Fields::Unit => quote! {},
            };

            quote! {
                #[doc = #doc]
                #[inline]
                #[allow(unreachable_patterns)]
                #[allow(dead_code)]
                pub fn #predicate_name(&self) -> bool {
                    match *self {
                        #name :: #ident #data_tokens => true,
                        _ => false,
                    }
                }
            }
        },
    );

    quote! {
        /// # `enum` Variant Predicates
        impl #impl_generics #name #ty_generics #where_clause {
            #(
                #predicates
            )*
        }
    }
    .into()
}

use proc_macro::TokenTree;

pub fn get_name(keyword: &str, item: TokenStream) -> String {
    get_names(&[keyword], item)
}
pub fn get_names(keywords: &[&str], item: TokenStream) -> String {
    let mut struct_name = String::new();
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if keywords.iter().any(|kw| id.to_string() == *kw) {
                    struct_name = it.next().unwrap().to_string();
                    break;
                }
            }
            _ => {}
        }
    }
    if struct_name.is_empty() {
        panic!("no ident found")
    }
    struct_name
}
