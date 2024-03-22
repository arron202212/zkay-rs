#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// use type_check::type_exceptions::TypeException
use crate::analysis::contains_private_checker::contains_private_expr;

use crate::ast::{ASTType, DoWhileStatement, ForStatement, IntoAST, WhileStatement, AST};

use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitorBase, AstVisitorBaseRef, AstVisitorMut},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_loops(ast: &mut AST) {
    // """
    // Checks if loops don't contain private expressions
    // """
    let mut v = LoopChecker::new();
    v.visit(ast);
}

// class LoopChecker(FunctionVisitor)

#[derive(ASTVisitorBaseRefImpl)]
struct LoopChecker {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for LoopChecker {}
impl AstVisitorMut for LoopChecker {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }

    fn has_attr(&self, name: &ASTType) -> bool {
        false
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        None
    }
}
impl LoopChecker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitWhileStatement(&mut self, ast: WhileStatement) {
        assert!(
            !contains_private_expr(Some(ast.condition.to_ast())),
            "Loop condition cannot contain private expressions {:?}",
            ast.condition
        );
        assert!(
            !contains_private_expr(Some(ast.body.to_ast())),
            "Loop body cannot contain private expressions {:?}",
            ast.body
        );
        self.visit_children(&mut ast.to_ast());
    }

    pub fn visitDoWhileStatement(&mut self, ast: DoWhileStatement) {
        assert!(
            !contains_private_expr(Some(ast.condition.to_ast())),
            "Loop condition cannot contain private expressions {:?}",
            ast.condition
        );
        assert!(
            !contains_private_expr(Some(ast.body.to_ast())),
            "Loop body cannot contain private expressions {:?}",
            ast.body
        );
        self.visit_children(&mut ast.to_ast());
    }

    pub fn visitForStatement(&mut self, ast: ForStatement) {
        assert!(
            !contains_private_expr(Some(ast.condition.to_ast())),
            "Loop condition cannot contain private expressions {:?}",
            ast.condition
        );
        assert!(
            !contains_private_expr(Some(ast.body.to_ast())),
            "Loop body cannot contain private expressions {:?}",
            ast.body
        );
        assert!(
            ast.update.is_none() || contains_private_expr(ast.update.as_ref().map(|u| u.to_ast())),
            "Loop update statement cannot contain private expressions {:?}",
            ast.update
        );
        self.visit_children(&mut ast.to_ast());
    }
}
