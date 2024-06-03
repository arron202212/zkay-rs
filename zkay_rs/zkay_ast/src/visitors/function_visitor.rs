#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{ASTFlatten, IntoAST, Parameter, SourceUnit};
use crate::visitors::visitor::AstVisitor;
use rccell::RcCell;
// class FunctionVisitor(AstVisitor)
pub trait FunctionVisitor: AstVisitor {
    // pub fn __init__(self)
    //     super().__init__('node-or-children')
    // fn traversal(&self) -> &'static str {
    //     "node-or-children"
    // }
    fn visitSourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        for c in ast
            .try_as_source_unit_ref()
            .unwrap()
            .borrow()
            .contracts
            .clone()
        {
            for cd in c.borrow().constructor_definitions.clone() {
                self.visit(&cd.clone().into());
            }
            for fd in c.borrow().function_definitions.clone() {
                self.visit(&fd.clone().into());
            }
        }
        Ok(<Self as AstVisitor>::temper_result(self))
    }

    fn visitParameter(&self, _ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(<Self as AstVisitor>::temper_result(self))
    }
}
