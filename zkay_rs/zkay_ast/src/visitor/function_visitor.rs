#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{ASTFlatten, IntoAST, Parameter, SourceUnit};
use crate::visitor::visitor::AstVisitor;
use rccell::RcCell;
// class FunctionVisitor(AstVisitor)
pub trait FunctionVisitor: AstVisitor {
    // pub fn __init__(self)
    //     super().__init__('node-or-children')
    // fn traversal(&self) -> &'static str {
    //     "node-or-children"
    // }
    fn visitSourceUnit(&self, ast: &ASTFlatten) {
        for c in &ast.try_as_source_unit_ref().unwrap().borrow().contracts {
            for cd in &c.borrow().constructor_definitions {
                self.visit(&cd.clone().into());
            }
            for fd in &c.borrow().function_definitions {
                self.visit(&fd.clone().into());
            }
        }
    }

    fn visitParameter(&self, _ast: &ASTFlatten) {}
}
