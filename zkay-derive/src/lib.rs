use proc_macro::{TokenStream, TokenTree};

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
