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
use zkay_derive_core::{get_name, get_names};
#[proc_macro_derive(ExpressionASTypeImpl)]
pub fn derive_expression_as_type(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"
impl ExpressionASType for {struct_name} {{
    fn as_type(&self, t: &ASTFlatten) -> ASTFlatten {{
        let mut selfs = self.clone();
        if is_instance(t, ASTType::AnnotatedTypeName) {{
            selfs.ast_base_mut_ref().borrow_mut().annotated_type = t.clone().try_as_annotated_type_name();
        }} else  {{
            selfs.ast_base_mut_ref().borrow_mut().annotated_type =
                Some(RcCell::new(AnnotatedTypeName::new(
                    Some(t.clone()),
                    None,
                    Homomorphism::non_homomorphic(),
                )));
        }}

        ASTFlatten::from(RcCell::new(selfs.clone()))
    }}
}}
                    "#
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(ASTKind)]
pub fn derive_get_ast_type(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
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
    .unwrap()
}

#[proc_macro_derive(ASTFlattenImpl)]
pub fn derive_ast_flatten_impl(item: TokenStream) -> TokenStream {
    let struct_name = get_names(&["struct", "enum"], item);
    format!(
        r#"
impl {struct_name} {{
 pub fn code(&self) -> String {{
        let v = CodeVisitorBase::new(true);
        v.visit(&RcCell::new(self.clone()).into())
    }}
}}

                    "#
    )
    .parse()
    .unwrap()
}

// impl From<RcCell<{struct_name}>> for ASTFlatten {{
//     fn from(f:RcCell<{struct_name}>) -> ASTFlatten{{
//         ASTFlatten::{struct_name}(f)
//     }}
// }}
#[proc_macro_derive(ASTVisitorBaseRefImpl)]
pub fn derive_ast_visitor_base_ref(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
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
    .unwrap()
}

#[proc_macro_derive(AstTransformerVisitorBaseRefImpl)]
pub fn derive_ast_transformer_visitor_base_ref(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"

impl AstTransformerVisitorBaseRef for {} {{
    fn ast_transformer_visitor_base_ref(&self) -> &AstTransformerVisitorBase {{
        &self.ast_transformer_visitor_base
    }}
}}
                    "#,
        struct_name
    )
    .parse()
    .unwrap()
}
#[proc_macro_derive(ASTChildrenImpl)]
pub fn derive_ast_children(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"
impl ASTChildren for {} {{
    fn process_children(&self, _cb: &mut ChildListBuilder) {{
        
    }}
                }}
                    "#,
        struct_name
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(MyPartialEqImpl)]
pub fn derive_my_partial_eq(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"
impl MyPartialEq for {} {{
    fn my_eq(&self, other: &Self)-> bool{{
        self==other
    }}
                }}
                    "#,
        struct_name
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(ASTNoneIdentifierPropertyImpl)]
pub fn derive_ast_none_identifier_property(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"
impl ASTPropertyIdentifier for {} {{
    fn get_option_idf(&self)-> Option<Identifier> {{
        None
    }}
                }}
                    "#,
        struct_name
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(ASTIdentifierPropertyImpl)]
pub fn derive_ast_identifier_property(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"
impl ASTPropertyIdentifier for {} {{
    fn get_option_idf(&self)-> Option<Identifier> {{
        self.idf().clone().upgrade().map(|f|f.borrow().clone())
    }}
                }}
                    "#,
        struct_name
    )
    .parse()
    .unwrap()
}
#[proc_macro_derive(ASTDebug)]
pub fn derive_ast_debug(item: TokenStream) -> TokenStream {
    let struct_name = get_name("struct", item);
    format!(
        r#"

impl std::fmt::Display for {} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {{
        write!(f, "{{}}", self.code())
    }}
}}
                    "#,
        struct_name
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(ImplBaseTrait)]
pub fn derive_impl_base_trait(item: TokenStream) -> TokenStream {
    let mut struct_name = get_name("struct", item);

    let fn_name = struct_name.to_snake_case();
    let mut impl_traits_str = format!(
        r#"
impl {struct_name}Ref for {struct_name} {{
        fn {fn_name}_ref(&self)->{}{{
        {} }}
    }}                    "#,
        if &struct_name == "ASTBase" {
            "RcCell<ASTBase>".to_owned()
        } else {
            "&".to_owned() + &struct_name
        },
        if &struct_name == "ASTBase" {
            "RcCell::new(self.clone())"
        } else {
            "self"
        }
    );
    impl_traits_str += &format!(
        r#"
#[enum_dispatch]
pub trait {struct_name}MutRef {{
        fn {fn_name}_mut_ref(&mut self)->{};
 }} 
                  "#,
        if &struct_name == "ASTBase" {
            "RcCell<ASTBase>".to_owned()
        } else {
            "&mut ".to_owned() + &struct_name
        }
    );
    impl_traits_str += &format!(
        r#"
impl {struct_name}MutRef for {struct_name} {{
        fn {fn_name}_mut_ref(&mut self)->{}{{
        {} }}
    }}                    "#,
        if &struct_name == "ASTBase" {
            "RcCell<ASTBase>".to_owned()
        } else {
            "&mut ".to_owned() + &struct_name
        },
        if &struct_name == "ASTBase" {
            "RcCell::new(self.clone())"
        } else {
            "self"
        }
    );
    impl_traits_str.parse().unwrap()
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
    let items = item.to_string();
    let struct_name = get_name("struct", item);

    let mut at = args.vars.iter();
    let mut impls = items + "\n";
    let mut struct_vairent = String::from("self");
    for base_struct_name in at {
        let base_struct_name = base_struct_name.to_string();
        let fn_name = base_struct_name.to_snake_case();
        struct_vairent += ".";
        struct_vairent += &fn_name;

        let s = format!(
            "impl {base_struct_name}Ref for {struct_name} {{
        fn {fn_name}_ref(&self)->{}{{
            {} }}
    }}\n",
            if &base_struct_name == "ASTBase" {
                "RcCell<ASTBase>".to_owned()
            } else {
                "&".to_owned() + &base_struct_name
            },
            if &base_struct_name == "ASTBase" {
                struct_vairent.clone() + ".clone()"
            } else {
                "&".to_owned() + &struct_vairent
            },
        );
        impls += &s;
        let s = format!(
            "impl {base_struct_name}MutRef for {struct_name} {{
        fn {fn_name}_mut_ref(&mut self)-> {}{{
             {} }}
    }}\n",
            if &base_struct_name == "ASTBase" {
                "RcCell<ASTBase>".to_owned()
            } else {
                "&mut ".to_owned() + &base_struct_name
            },
            if &base_struct_name == "ASTBase" {
                struct_vairent.clone() + ".clone()"
            } else {
                "&mut ".to_owned() + &struct_vairent
            },
        );
        impls += &s;
    }
    impls.parse().unwrap()
}

#[proc_macro_attribute]
pub fn impl_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = attr.clone();
    let mut args = parse_macro_input!(args as Args);
    let items = item.to_string();
    let mut trait_name = get_name("trait", item);
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
    for struct_name in at {
        let struct_name = struct_name.to_string();
        let s = format!(
            "impl {trait_name} for {struct_name} {{
        fn {fn_name}(&self)->{}{{
        {} }}
    }}\n",
            if struct_ref == "ASTBase" {
                "RcCell<ASTBase>".to_owned()
            } else {
                "&".to_owned() + struct_ref
            },
            if struct_ref == "ASTBase" {
                "self.ast_base.clone()".to_owned()
            } else {
                "&self.".to_owned() + struct_vairent
            },
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
    // .parse()
    // .expect("should parse expanded output source into tokens")
    zkay_derive_core::expand_derive_is_enum_variant(&ast)
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

#[proc_macro_derive(EnumDispatchWithDeepClone)]
pub fn derive_enum_dispatch_with_deep_clone(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let mut variant_checker_functions;

    match data {
        Data::Enum(data_enum) => {
            variant_checker_functions = TokenStream2::new();

            for variant in &data_enum.variants {
                let variant_name = &variant.ident;

                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    #name::#variant_name (ast) => #name::#variant_name(ast.clone_inner()),
                });
            }
        }
        _ => return derive_error!("EnumDispatchWithDeepClone is only implemented for enums"),
    };

    let expanded = quote! {
        impl DeepClone for #name  {
            fn clone_inner(&self) -> Self {
            match self
            {
                #variant_checker_functions
            }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(EnumDispatchWithFields)]
pub fn derive_enum_dispatch_with_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let mut variant_checker_functions;

    match data {
        Data::Enum(data_enum) => {
            variant_checker_functions = TokenStream2::new();

            for variant in &data_enum.variants {
                let variant_name = &variant.ident;

                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    #name::#variant_name (ast) => #name::#variant_name(ast.from_fields(fields)),
                });
            }
        }
        _ => return derive_error!("EnumDispatchWithFields is only implemented for enums"),
    };

    let expanded = quote! {
        impl FullArgsSpecInit for #name  {
            fn from_fields(&self,fields: Vec<ArgType>) -> Self {
            match self
            {
                #variant_checker_functions
            }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let mut variant_checker_functions;

    match data {
        Data::Enum(data_enum) => {
            variant_checker_functions = TokenStream2::new();

            for variant in &data_enum.variants {
                let variant_name = &variant.ident;
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
