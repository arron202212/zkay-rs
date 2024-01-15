use crate::config::CFG;
use crate::zkay_ast::ast::{AnnotatedTypeName, CodeVisitor, MeExpr, AST};

pub fn to_solidity(ast: AST) {
    SolidityVisitor::new().visit(ast)
}

// class SolidityVisitor(CodeVisitor)
pub struct SolidityVisitor {
    code_visitor_base: CodeVisitor,
}
impl SolidityVisitor {
    // pub fn __init__(self)
    //     // do not display `final` keywords (`final` is not in Solidity fragment)
    //     super().__init__(False)
    pub fn new() -> Self {
        Self {
            code_visitor_base: CodeVisitor::new(false),
        }
    }
    pub fn visitAnnotatedTypeName(self, ast: AnnotatedTypeName) -> String
//only display data type, not privacy annotation
    {
        self.visit(ast.type_name)
    }

    pub fn visitMeExpr(self, _: MeExpr) -> String {
        String::from("msg.sender")
    }

    pub fn handle_pragma(self, pragma: String) -> String {
        format!(
            "pragma solidity {};",
            CFG.lock().unwrap().kay_solc_version_compatibility
        )
    }
}
