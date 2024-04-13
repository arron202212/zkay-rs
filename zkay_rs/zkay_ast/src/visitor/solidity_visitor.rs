#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{ASTFlatten, AnnotatedTypeName, CodeVisitor, IntoAST, MeExpr, AST};
use crate::visitor::visitor::AstVisitor;
use zkay_config::config::CFG;

pub fn to_solidity(ast: &ASTFlatten) -> String {
    SolidityVisitor::new().code_visitor_base.visit(ast)
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
        self.code_visitor_base
            .visit(&ast.type_name.clone().unwrap().into())
    }

    pub fn visitMeExpr(self, _: MeExpr) -> String {
        String::from("msg.sender")
    }

    pub fn handle_pragma(self, _pragma: String) -> String {
        format!(
            "pragma solidity {};",
            CFG.lock().unwrap().zkay_solc_version_compatibility()
        )
    }
}
