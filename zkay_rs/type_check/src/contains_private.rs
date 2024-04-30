#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use rccell::RcCell;
use zkay_ast::ast::{
    is_instance, ASTFlatten, ASTType, AnnotatedTypeName, ExpressionBaseProperty, AST,
};
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;

pub fn contains_private(ast: &ASTFlatten) -> bool {
    let v = ContainsPrivateVisitor::new();
    v.visit(ast);
    let contains_private = *v.contains_private.borrow();
    contains_private
}

// class ContainsPrivateVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct ContainsPrivateVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub contains_private: RcCell<bool>,
}
impl AstVisitor for ContainsPrivateVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, _ast: &AST) -> bool {
        false
    }
    fn get_attr(&self, _name: &ASTType, _ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        Err(eyre::eyre!("unreach"))
    }
}
impl ContainsPrivateVisitor {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.contains_private = False
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
            contains_private: RcCell::new(false),
        }
    }
    pub fn visitAST(&self, ast: &ASTFlatten) {
        if let Some(t) = ast
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
        {
            assert!(is_instance(t, ASTType::AnnotatedTypeName));

            if !t
                .borrow()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .is_all_expr()
            {
                *self.contains_private.borrow_mut() = true;
            }
        }
    }
}
