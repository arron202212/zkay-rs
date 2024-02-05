use crate::zkay_ast::ast::{
    is_instance, ASTType, Expression, FunctionCallExpr, IntoAST, LocationExpr, AST,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn contains_private_expr(ast: Option<AST>) -> bool {
    if ast.is_none() {
        return false;
    }
    let v = ContainsPrivVisitor::new();
    v.visit(ast.unwrap());
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
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
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
        if is_instance(&ast.func().unwrap(), ASTType::LocationExpr) && !ast.is_cast() {
            self.contains_private |= ast
                .func()
                .unwrap()
                .target()
                .unwrap()
                .requires_verification();
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
