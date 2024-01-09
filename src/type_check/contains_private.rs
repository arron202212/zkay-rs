use crate::zkay_ast::ast::{AnnotatedTypeName, AST};
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn contains_private(ast: AST) -> bool {
    let v = ContainsPrivateVisitor::new();
    v.visit(ast);
    v.contains_private
}

// class ContainsPrivateVisitor(AstVisitor)
pub struct ContainsPrivateVisitor;
impl ContainsPrivateVisitor {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.contains_private = False
    pub fn new() -> Self {
        Self {
            contains_private: false,
        }
    }
    pub fn visitAST(self, ast: AST) {
        if hasattr(ast, "annotated_type") {
            let t = ast.annotated_type;
            if t.is_some() {
                assert(isinstance(t, AnnotatedTypeName));

                if !t.privacy_annotation.is_all_expr() {
                    self.contains_private = True;
                }
            }
        }
    }
}
