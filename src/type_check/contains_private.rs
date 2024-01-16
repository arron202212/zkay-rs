use crate::zkay_ast::ast::{AnnotatedTypeName, AST,is_instance,ASTType};
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
        if let Some(t)=ast.annotated_type() {
                assert(is_instance(t, ASTType::AnnotatedTypeName));

                if !t.privacy_annotation.is_all_expr() {
                    self.contains_private = true;
                }
        }
    }
}
