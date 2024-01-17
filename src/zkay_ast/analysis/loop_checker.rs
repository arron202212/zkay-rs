// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::analysis::contains_private_checker::contains_private_expr;
use crate::zkay_ast::ast::{DoWhileStatement, ForStatement, WhileStatement,AST,};
use crate::zkay_ast::visitor::function_visitor::FunctionVisitor;

pub fn check_loops(ast: AST) {
    // """
    // Checks if loops don't contain private expressions
    // """
    let v = LoopChecker::new();
    v.visit(ast);
}

// class LoopChecker(FunctionVisitor)
pub struct LoopChecker;
impl LoopChecker {
    pub fn visitWhileStatement(self, ast: WhileStatement) {
        if contains_private_expr(ast.condition) {
            assert!(
                false,
                "Loop condition cannot contain private expressions {:?}",
                ast.condition
            )
        }
        if contains_private_expr(ast.body) {
            assert!(
                false,
                "Loop body cannot contain private expressions {:?}",
                ast.body
            )
        }
        self.visitChildren(ast);
    }

    pub fn visitDoWhileStatement(self, ast: DoWhileStatement) {
        if contains_private_expr(ast.condition) {
            assert!(
                false,
                "Loop condition cannot contain private expressions {:?}",
                ast.condition
            )
        }
        if contains_private_expr(ast.body) {
            assert!(
                false,
                "Loop body cannot contain private expressions {:?}",
                ast.body
            )
        }
        self.visitChildren(ast);
    }

    pub fn visitForStatement(self, ast: ForStatement) {
        if contains_private_expr(ast.condition) {
            assert!(
                false,
                "Loop condition cannot contain private expressions {:?}",
                ast.condition
            )
        }
        if contains_private_expr(ast.body) {
            assert!(
                false,
                "Loop body cannot contain private expressions {:?}",
                ast.body
            )
        }
        if ast.update.is_some() && contains_private_expr(ast.update) {
            assert!(
                false,
                "Loop update statement cannot contain private expressions {:?}",
                ast.update
            )
        }
        self.visitChildren(ast);
    }
}
