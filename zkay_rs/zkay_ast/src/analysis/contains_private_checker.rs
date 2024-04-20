#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    is_instance, ASTFlatten, ASTType, Expression, ExpressionBaseProperty, FunctionCallExpr,
    FunctionCallExprBaseProperty, IntoAST, IntoExpression, LocationExpr, LocationExprBaseProperty,
    AST,
};
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::RcCell;
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn contains_private_expr(ast: &ASTFlatten) -> bool {
    // if ast.is_none() {
    //     return false;
    // }
    let mut v = ContainsPrivVisitor::new();
    v.visit(ast);
    let contains_private = *v.contains_private.borrow();
    contains_private
}

// class ContainsPrivVisitor(AstVisitor)
// pub fn __init__(self)
//     super().__init__('node-or-children')
//     self.contains_private = False
#[derive(ASTVisitorBaseRefImpl)]
pub struct ContainsPrivVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub contains_private: RcCell<bool>,
}

impl AstVisitor for ContainsPrivVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::FunctionCallExprBase | ASTType::ExpressionBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),
            ASTType::ExpressionBase => self.visitExpression(ast),
            _ => {}
        }
    }
}
impl ContainsPrivVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            contains_private: RcCell::new(false),
        }
    }
    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        if is_instance(
            ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::LocationExprBase,
        ) && !ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .is_cast()
        {
            *self.contains_private.borrow_mut() |= ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .clone()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification;
        }
        self.visitExpression(ast)
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) {
        if ast
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .evaluate_privately()
        {
            *self.contains_private.borrow_mut() = true;
        }
        self.visitAST(ast)
    }

    pub fn visitAST(&self, ast: &ASTFlatten) {
        if *self.contains_private.borrow() {
            return;
        }
        self.visit_children(ast);
    }
}
