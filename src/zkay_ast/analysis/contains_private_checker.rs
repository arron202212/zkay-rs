use crate::zkay_ast::ast::{is_instance, ASTType, Expression, FunctionCallExpr, LocationExpr, AST};
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn contains_private_expr(ast: Option<AST>) -> bool {
    if ast.is_none() {
        return false;
    }
    let v = ContainsPrivVisitor::new();
    v.visit(ast);
    v.contains_private
}

// class ContainsPrivVisitor(AstVisitor)
// pub fn __init__(self)
//     super().__init__('node-or-children')
//     self.contains_private = False
pub struct ContainsPrivVisitor {
    pub contains_private: bool,
}
impl ContainsPrivVisitor {
    pub fn new() -> Self {
        Self {
            contains_private: false,
        }
    }
    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if is_instance(&ast.func, ASTType::LocationExpr) && !ast.is_cast {
            self.contains_private |= ast.func.target.requires_verification;
        }
        self.visitExpression(ast)
    }

    pub fn visitExpression(self, ast: Expression) {
        if ast.evaluate_privately {
            self.contains_private = true;
        }
        self.visitAST(ast)
    }

    pub fn visitAST(self, ast: AST) {
        if self.contains_private {
            return;
        }
        self.visitChildren(ast)
    }
}
