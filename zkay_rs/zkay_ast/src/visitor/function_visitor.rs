#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{IntoAST, Parameter, SourceUnit};
use crate::visitor::visitor::AstVisitorMut;
// class FunctionVisitor(AstVisitor)
pub trait FunctionVisitor: AstVisitorMut {
    // pub fn __init__(self)
    //     super().__init__('node-or-children')
    // fn traversal(&self) -> &'static str {
    //     "node-or-children"
    // }
    fn visitSourceUnit(&mut self, ast: &mut SourceUnit) {
        for c in ast.contracts.iter_mut() {
            for cd in &c.constructor_definitions {
                self.visit(&mut cd.to_ast());
            }
            for fd in &c.function_definitions {
                self.visit(&mut fd.to_ast());
            }
        }
    }

    fn visitParameter(&mut self, _ast: &mut Parameter) {}
}
