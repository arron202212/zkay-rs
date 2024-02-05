use crate::zkay_ast::ast::{IntoAST, Parameter, SourceUnit};
use crate::zkay_ast::visitor::visitor::AstVisitor;
// class FunctionVisitor(AstVisitor)
pub trait FunctionVisitor: AstVisitor {
    // pub fn __init__(self)
    //     super().__init__('node-or-children')
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn visitSourceUnit(&self, ast: SourceUnit) {
        for c in ast.contracts {
            for cd in c.constructor_definitions {
                self.visit(cd.to_ast());
            }
            for fd in c.function_definitions {
                self.visit(fd.to_ast());
            }
        }
    }

    fn visitParameter(&self, ast: Parameter) {}
}
