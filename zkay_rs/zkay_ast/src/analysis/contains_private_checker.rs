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
use crate::visitor::visitor::AstVisitor;

pub fn contains_private_expr(ast: Option<AST>) -> bool {
    if ast.is_none() {
        return false;
    }
    let v = ContainsPrivVisitor::new();
    v.visit(ast.as_ref().unwrap());
    v.contains_private
}

// class ContainsPrivVisitor(AstVisitor)
// pub fn __init__(self)
//     super().__init__('node-or-children')
//     self.contains_private = False
pub struct ContainsPrivVisitor {
    pub contains_private: bool,
}

impl AstVisitor for ContainsPrivVisitor {
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
    fn has_attr(&self, name: &ASTType) -> bool {
        false
    }
    fn get_attr(&self, name: &ASTType, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl ContainsPrivVisitor {
    pub fn new() -> Self {
        Self {
            contains_private: false,
        }
    }
    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
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
        self.visitExpression(ast.to_expr())
    }

    pub fn visitExpression(&mut self, ast: Expression) {
        if ast.evaluate_privately() {
            self.contains_private = true;
        }
        self.visitAST(ast.to_ast())
    }

    pub fn visitAST(&self, ast: AST) {
        if self.contains_private {
            return;
        }
        self.visit_children(&ast);
    }
}
