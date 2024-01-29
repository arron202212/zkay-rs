// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::analysis::contains_private_checker::contains_private_expr;
use crate::zkay_ast::ast::{DoWhileStatement, ForStatement, WhileStatement, AST};
use crate::zkay_ast::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};

pub fn check_loops(ast: AST) {
    // """
    // Checks if loops don't contain private expressions
    // """
    let v = LoopChecker;
    v.visit(ast);
}

// class LoopChecker(FunctionVisitor)
pub struct LoopChecker;

impl FunctionVisitor for LoopChecker {}
impl AstVisitor for LoopChecker {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl LoopChecker {
    pub fn visitWhileStatement(self, ast: WhileStatement) {
        assert!(
            !contains_private_expr(Some(ast.condition.get_ast())),
            "Loop condition cannot contain private expressions {:?}",
            ast.condition
        );
        assert!(
            !contains_private_expr(Some(ast.body.get_ast())),
            "Loop body cannot contain private expressions {:?}",
            ast.body
        );
        self.visit_children(ast);
    }

    pub fn visitDoWhileStatement(self, ast: DoWhileStatement) {
        assert!(
            !contains_private_expr(Some(ast.condition.get_ast())),
            "Loop condition cannot contain private expressions {:?}",
            ast.condition
        );
        assert!(
            !contains_private_expr(Some(ast.body.get_ast())),
            "Loop body cannot contain private expressions {:?}",
            ast.body
        );
        self.visit_children(ast);
    }

    pub fn visitForStatement(self, ast: ForStatement) {
        assert!(
            !contains_private_expr(ast.condition.get_ast()),
            "Loop condition cannot contain private expressions {:?}",
            ast.condition
        );
        assert!(
            !contains_private_expr(Some(ast.body.get_ast())),
            "Loop body cannot contain private expressions {:?}",
            ast.body
        );
        assert!(
            ast.update.is_none() || contains_private_expr(ast.update.map(|u| u.get_ast())),
            "Loop update statement cannot contain private expressions {:?}",
            ast.update
        );
        self.visit_children(ast);
    }
}
