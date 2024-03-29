#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use heck::ToSnakeCase;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashSet as Set;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Error, Fields, Ident, Token,
};
#[proc_macro_derive(ASTKind)]
pub fn derive_get_ast_type(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
                    let struct_name = it.next().unwrap().to_string();
                    return format!(
                        r#"
impl ASTInstanceOf for {} {{
    fn get_ast_type(&self) -> ASTType {{
        ASTType::{}
    }}
}}
                    "#,
                        struct_name, struct_name
                    )
                    .parse()
                    .unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("no ident found")
}

#[proc_macro_derive(ASTFlattenImpl)]
pub fn derive_ast_flatten_impl(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
                    let struct_name = it.next().unwrap().to_string();
                    return format!(
                        r#"
impl IntoASTFlatten for {} {{
    fn to_ast_flatten<'a>(&'a mut self) -> ASTFlatten<'a>{{
        ASTFlatten::{}(self)
    }}
}}
                    "#,
                        struct_name, struct_name
                    )
                    .parse()
                    .unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("no ident found")
}

#[proc_macro_derive(ASTVisitorBaseRefImpl)]
pub fn derive_ast_visitor_base_ref(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
                    let struct_name = it.next().unwrap().to_string();
                    return format!(
                        r#"
impl AstVisitorBaseRef for {} {{
    fn ast_visitor_base_ref(&self) -> &AstVisitorBase {{
        &self.ast_visitor_base
    }}
}}
                    "#,
                        struct_name
                    )
                    .parse()
                    .unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("no ident found")
}

#[proc_macro_derive(ASTChildrenImpl)]
pub fn derive_ast_children(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
                    let struct_name = it.next().unwrap().to_string();
                    return format!(
                        r#"
impl ASTChildren for {} {{
    fn process_children(&mut self, _cb: &mut ChildListBuilder) {{
        
    }}
 fn process_children_mut<'a>(&'a mut self, cb: &mut Vec<ASTFlatten<'a>>) {{
        
    }}
}}
                    "#,
                        struct_name
                    )
                    .parse()
                    .unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("no ident found")
}

#[proc_macro_derive(ASTDebug)]
pub fn derive_ast_debug(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
                    let struct_name = it.next().unwrap().to_string();
                    return format!(
                        r#"
impl std::fmt::Display for {} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {{
        write!(f, "{{}}", self.to_ast().code())
    }}
}}
                    "#,
                        struct_name
                    )
                    .parse()
                    .unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("no ident found")
}

#[proc_macro_derive(ImplBaseTrait)]
pub fn derive_impl_base_trait(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
                    let struct_name = it.next().unwrap().to_string();
                    let fn_name = struct_name.to_snake_case();
                    let mut impl_traits_str = format!(
                        r#"
impl {}Ref for {} {{
        fn {}_ref(&self)->&{}{{
        self }}
    }}                    "#,
                        struct_name, struct_name, fn_name, struct_name
                    );
                    impl_traits_str += &format!(
                        r#"
#[enum_dispatch]
pub trait {}MutRef {{
        fn {}_mut_ref(&mut self)->&mut {};
 }} 
                  "#,
                        struct_name, fn_name, struct_name
                    );
                    impl_traits_str += &format!(
                        r#"
impl {}MutRef for {} {{
        fn {}_mut_ref(&mut self)->&mut {}{{
        self }}
    }}                    "#,
                        struct_name, struct_name, fn_name, struct_name
                    );
                    return impl_traits_str.parse().unwrap();
                }
            }
            _ => {}
        }
    }
    panic!("no ident found")
}

struct Args {
    vars: Vec<Ident>,
}
impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}
#[proc_macro_attribute]
pub fn impl_traits(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = attr.clone();
    let mut args = parse_macro_input!(args as Args);
    let mut struct_name = String::new();
    let items = item.to_string();
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "struct" {
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

    let mut at = args.vars.iter();
    let mut impls = items + "\n";
    let mut struct_vairent = String::from("self");
    while let Some(base_struct_name) = at.next() {
        let base_struct_name = base_struct_name.to_string();
        let fn_name = base_struct_name.to_snake_case();
        struct_vairent += ".";
        struct_vairent += &fn_name;

        let s = format!(
            "impl {}Ref for {} {{
        fn {}_ref(&self)->&{}{{
        &{} }}
    }}\n",
            base_struct_name, struct_name, fn_name, base_struct_name, struct_vairent
        );
        impls += &s;
        let s = format!(
            "impl {}MutRef for {} {{
        fn {}_mut_ref(&mut self)->&mut {}{{
        &mut {} }}
    }}\n",
            base_struct_name, struct_name, fn_name, base_struct_name, struct_vairent
        );
        impls += &s;
    }
    impls.parse().unwrap()
}

#[proc_macro_attribute]
pub fn impl_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = attr.clone();
    let mut args = parse_macro_input!(args as Args);
    let mut trait_name = String::new();
    let items = item.to_string();
    let mut it = item.into_iter();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(id) => {
                if id.to_string() == "trait" {
                    trait_name = it.next().unwrap().to_string();
                    break;
                }
            }
            _ => {}
        }
    }
    let fn_name = trait_name.to_snake_case();
    let struct_ref = if trait_name.len() < 3 {
        &trait_name
    } else {
        &trait_name[..trait_name.len() - 3]
    };
    let struct_vairent = if fn_name.len() < 4 {
        &fn_name
    } else {
        &fn_name[..fn_name.len() - 4]
    };
    let mut at = args.vars.iter();
    // let mut at = at.trim_matches('"');
    // let mut at = at.split(",");
    let mut impls = items + "\n";
    while let Some(struct_name) = at.next() {
        let struct_name = struct_name.to_string();
        let s = format!(
            "impl {} for {} {{
        fn {}(&self)->&{}{{
        &self.{} }}
    }}\n",
            trait_name, struct_name, fn_name, struct_ref, struct_vairent
        );
        impls += &s;
    }
    impls.parse().unwrap()
}

#[proc_macro_derive(is_enum_variant, attributes(is_enum_variant))]
pub fn derive_is_enum_variant(tokens: TokenStream) -> TokenStream {
    // let source = tokens.to_string();

    let ast = parse_macro_input!(tokens as DeriveInput);
    // syn::parse_derive_input(&source).expect("should parse input tokens into AST");
    let expanded = zkay_derive_core::expand_derive_is_enum_variant(&ast);

    expanded
    // .parse()
    // .expect("should parse expanded output source into tokens")
}

extern crate proc_macro;

// use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
// use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

use convert_case::{Case, Casing};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ref name = input.ident;
    let ref data = input.data;

    let mut variant_checker_functions;

    match data {
        Data::Enum(data_enum) => {
            variant_checker_functions = TokenStream2::new();

            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;
                let fields_in_variant = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                let mut is_variant_func_name =
                    format_ident!("is_{}", variant_name.to_string().to_case(Case::Snake));
                is_variant_func_name.set_span(variant_name.span());

                let mut as_variant_func_name =
                    format_ident!("as_{}", variant_name.to_string().to_case(Case::Snake));
                as_variant_func_name.set_span(variant_name.span());

                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    fn #is_variant_func_name(&self) -> bool {
                        match self {
                            #name::#variant_name #fields_in_variant => true,
                            _ => false,
                        }
                    }
                });
                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    fn #as_variant_func_name(&self) -> Option<& #variant_name> {
                        match self {
                            #name::#variant_name #fields_in_variant => true,
                            _ => false,
                        }
                    }
                });
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #variant_checker_functions
        }
    };

    TokenStream::from(expanded)
}

use syn::{Field, FieldsUnnamed};

// fn is_field_struct(field: &Field) -> bool {
//     if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &field.ty {
//         unnamed.first().map_or(false, |f| f.is_struct())
//     } else {
//         false
//     }
// }

// fn main() {
//     let input: DeriveInput = /* Some input */;
//     for field in input.fields {
//         if is_field_struct(field) {
//             // Do something
//         }
//     }
// }
