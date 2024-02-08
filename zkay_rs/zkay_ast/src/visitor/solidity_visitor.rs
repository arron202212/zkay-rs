use zkay_config::config::CFG;
use crate::ast::{AnnotatedTypeName, CodeVisitor, IntoAST, MeExpr, AST};

pub fn to_solidity(ast: AST) -> String {
    SolidityVisitor::new().code_visitor_base.visit(&ast)
}

// class SolidityVisitor(CodeVisitor)
pub struct SolidityVisitor {
    pub code_visitor_base: CodeVisitor,
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
        self.code_visitor_base.visit(&(*ast.type_name).to_ast())
    }

    pub fn visitMeExpr(self, _: MeExpr) -> String {
        String::from("msg.sender")
    }

    pub fn handle_pragma(self, pragma: String) -> String {
        format!(
            "pragma solidity {};",
            CFG.lock().unwrap().zkay_solc_version_compatibility()
        )
    }
}
