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
    ASTFlatten, ASTInstanceOf, ASTType, DoWhileStatement, ForStatement, IntoAST, WhileStatement,
    AST,
};

use crate::visitors::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_loops(ast: &ASTFlatten) {
    // """
    // Checks if loops don't contain private expressions
    // """
    let mut v = LoopChecker::new();
    let _=v.visit(ast);
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

    fn has_attr(&self, name: &ASTType, _ast: &AST) -> bool {
        matches!(
            name,
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::WhileStatement
                | ASTType::DoWhileStatement
                | ASTType::ForStatement
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl LoopChecker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            !contains_private_expr(&ast.try_as_while_statement_ref().unwrap().borrow().condition),
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
        self.visit_children(ast)
    }

    pub fn visitDoWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            !contains_private_expr(
                &ast.try_as_do_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .condition
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
        self.visit_children(ast)
    }

    pub fn visitForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            !contains_private_expr(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .condition
            ),
            "Loop condition cannot contain private expressions {:?}",
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .condition
        );
        assert!(
            !contains_private_expr(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .body
                    .clone()
                    .into()
            ),
            "Loop body cannot contain private expressions {:?}",
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .body
        );
        assert!(
            !(ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .update
                .is_some()
                && contains_private_expr(
                    &ast.to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_for_statement_ref()
                        .unwrap()
                        .update
                        .clone()
                        .unwrap()
                        .into()
                )),
            "Loop update statement cannot contain private expressions {:?}",
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .update
        );
        self.visit_children(ast)
    }
}
