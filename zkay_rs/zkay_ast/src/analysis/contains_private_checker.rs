#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    is_instance, ASTType, Expression, ExpressionBaseProperty, FunctionCallExpr,
    FunctionCallExprBaseProperty, IntoAST, IntoExpression, LocationExpr, LocationExprBaseProperty,
    AST,
};
use crate::visitor::visitor::{AstVisitorBase, AstVisitorBaseRef, AstVisitorMut};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn contains_private_expr(ast: &mut Option<AST>) -> bool {
    if ast.is_none() {
        return false;
    }
    let mut v = ContainsPrivVisitor::new();
    v.visit(ast.as_mut().unwrap());
    v.contains_private
}

// class ContainsPrivVisitor(AstVisitorMut)
// pub fn __init__(self)
//     super().__init__('node-or-children')
//     self.contains_private = False
#[derive(ASTVisitorBaseRefImpl)]
pub struct ContainsPrivVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub contains_private: bool,
}

impl AstVisitorMut for ContainsPrivVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name || &ASTType::ExpressionBase == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_function_call_expr_mut()
                    .unwrap(),
            ),
            ASTType::ExpressionBase => self.visitExpression(ast.try_as_expression_mut().unwrap()),

            _ => {}
        }
    }
}
impl ContainsPrivVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            contains_private: false,
        }
    }
    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::LocationExprBase) && !ast.is_cast() {
            self.contains_private |= ast
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification;
        }
        self.visitExpression(&mut ast.to_expr())
    }

    pub fn visitExpression(&mut self, ast: &mut Expression) {
        if ast.evaluate_privately() {
            self.contains_private = true;
        }
        self.visitAST(&mut ast.to_ast())
    }

    pub fn visitAST(&mut self, ast: &mut AST) {
        if self.contains_private {
            return;
        }
        self.visit_children(ast);
    }
}
