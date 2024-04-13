#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// use type_check::type_exceptions::TypeException
use crate::analysis::contains_private_checker::contains_private_expr;

use crate::ast::{
    ASTFlatten, ASTType, DoWhileStatement, ForStatement, IntoAST, WhileStatement, AST,
};

use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_loops(ast: &ASTFlatten) {
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
impl AstVisitor for LoopChecker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::WhileStatement | ASTType::DoWhileStatement | ASTType::ForStatement
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            _ => {}
        }
    }
}
impl LoopChecker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitWhileStatement(&self, ast: &ASTFlatten) {
        assert!(
            !contains_private_expr(
                &ast.try_as_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .condition
                    .clone()
                    .into()
            ),
            "Loop condition cannot contain private expressions {:?}",
            ast.try_as_while_statement_ref().unwrap().borrow().condition
        );
        assert!(
            !contains_private_expr(
                &ast.try_as_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .body
                    .clone()
                    .into()
            ),
            "Loop body cannot contain private expressions {:?}",
            ast.try_as_while_statement_ref().unwrap().borrow().body
        );
        self.visit_children(ast);
    }

    pub fn visitDoWhileStatement(&self, ast: &ASTFlatten) {
        assert!(
            !contains_private_expr(
                &ast.try_as_do_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .condition
                    .clone()
                    .into()
            ),
            "Loop condition cannot contain private expressions {:?}",
            ast.try_as_do_while_statement_ref()
                .unwrap()
                .borrow()
                .condition
        );
        assert!(
            !contains_private_expr(
                &ast.try_as_do_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .body
                    .clone()
                    .into()
            ),
            "Loop body cannot contain private expressions {:?}",
            ast.try_as_do_while_statement_ref().unwrap().borrow().body
        );
        self.visit_children(ast);
    }

    pub fn visitForStatement(&self, ast: &ASTFlatten) {
        assert!(
            !contains_private_expr(
                &ast.try_as_for_statement_ref()
                    .unwrap()
                    .borrow()
                    .condition
                    .clone()
                    .into()
            ),
            "Loop condition cannot contain private expressions {:?}",
            ast.try_as_for_statement_ref().unwrap().borrow().condition
        );
        assert!(
            !contains_private_expr(
                &ast.try_as_for_statement_ref()
                    .unwrap()
                    .borrow()
                    .body
                    .clone()
                    .into()
            ),
            "Loop body cannot contain private expressions {:?}",
            ast.try_as_for_statement_ref().unwrap().borrow().body
        );
        assert!(
            ast.try_as_for_statement_ref()
                .unwrap()
                .borrow()
                .update
                .is_none()
                || contains_private_expr(
                    &ast.try_as_for_statement_ref()
                        .unwrap()
                        .borrow()
                        .update
                        .clone()
                        .unwrap()
                        .into()
                ),
            "Loop update statement cannot contain private expressions {:?}",
            ast.try_as_for_statement_ref().unwrap().borrow().update
        );
        self.visit_children(ast);
    }
}
