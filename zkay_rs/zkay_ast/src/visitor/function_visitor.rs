#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{IntoAST, Parameter, SourceUnit};
use crate::visitor::visitor::AstVisitor;
// class FunctionVisitor(AstVisitor)
pub trait FunctionVisitor: AstVisitor {
    // pub fn __init__(self)
    //     super().__init__('node-or-children')
    // fn traversal(&self) -> &'static str {
    //     "node-or-children"
    // }
    fn visitSourceUnit(&self, ast: RcCell<SourceUnit>) {
        for c in &ast.borrow().contracts {
            for cd in &c.constructor_definitions {
                self.visit(cd.clone().into());
            }
            for fd in &c.function_definitions {
                self.visit(fd.clone().into());
            }
        }
    }

    fn visitParameter(&self, _ast: RcCell<Parameter>) {}
}
