#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use zkay_ast::ast::{
    is_instance, ASTFlatten, ASTType, AnnotatedTypeName, ExpressionBaseProperty, AST,
};
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;

pub fn contains_private(ast: AST) -> bool {
    let v = ContainsPrivateVisitor::new();
    v.visit(&ast);
    v.contains_private
}

// class ContainsPrivateVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct ContainsPrivateVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub contains_private: bool,
}
impl AstVisitor for ContainsPrivateVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, _name: &ASTType) -> bool {
        false
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {}
}
impl ContainsPrivateVisitor {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.contains_private = False
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
            contains_private: false,
        }
    }
    pub fn visitAST(&self, ast: &ASTFlatten) {
        if let Some(t) = ast.try_as_expression_ref().unwrap().annotated_type() {
            assert!(is_instance(t, ASTType::AnnotatedTypeName));

            if !t
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .is_all_expr()
            {
                self.contains_private = true;
            }
        }
    }
}
