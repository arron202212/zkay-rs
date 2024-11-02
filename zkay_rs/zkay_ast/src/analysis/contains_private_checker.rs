#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    expression::{
        Expression, ExpressionBaseProperty, FunctionCallExpr, FunctionCallExprBaseProperty,
        LocationExpr,
    },
    is_instance, ASTFlatten, ASTInstanceOf, ASTType, IntoAST, IntoExpression, AST,
};
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::RcCell;
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn contains_private_expr(ast: &ASTFlatten) -> bool {
    // if ast.is_none() {
    //     return false;
    // }
    let mut v = ContainsPrivVisitor::new();
    let _ = v.visit(ast);
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

    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
        matches!(
            name,
            ASTType::FunctionCallExprBase | ASTType::ExpressionBase
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(ast, AST::Expression(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ => Err(eyre::eyre!("unreach")),
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
    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::LocationExprBase,
        ) && !ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .is_cast()
        {
            *self.contains_private.borrow_mut() |= ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .as_ref()
                .unwrap()
                .clone()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification;
        }
        self.visitExpression(ast)
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .evaluate_privately()
        {
            *self.contains_private.borrow_mut() = true;
        }
        self.visitAST(ast)
    }

    pub fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        if *self.contains_private.borrow() {
            return Ok(());
        }
        self.visit_children(ast)
    }
}
