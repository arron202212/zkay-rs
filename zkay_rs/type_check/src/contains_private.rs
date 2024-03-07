#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use zkay_ast::ast::{is_instance, ASTType, AnnotatedTypeName, AST};
use zkay_ast::visitor::visitor::AstVisitor;

pub fn contains_private(ast: AST) -> bool {
    let v = ContainsPrivateVisitor::new();
    v.visit(ast);
    v.contains_private
}

// class ContainsPrivateVisitor(AstVisitor)
pub struct ContainsPrivateVisitor {
    pub contains_private: bool,
}
impl AstVisitor for ContainsPrivateVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, _name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, _ast: &AST) -> Self::Return {
        None
    }
}
impl ContainsPrivateVisitor {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.contains_private = False
    pub fn new() -> Self {
        Self {
            contains_private: false,
        }
    }
    pub fn visitAST(&mut self, ast: AST) {
        if let Some(t) = ast.annotated_type() {
            assert!(is_instance(&t, ASTType::AnnotatedTypeName));

            if !t.privacy_annotation.unwrap().try_as_expression_ref().unwrap().is_all_expr() {
                self.contains_private = true;
            }
        }
    }
}
