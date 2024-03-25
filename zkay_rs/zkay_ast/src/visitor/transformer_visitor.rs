#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{Block, HybridArgumentIdf, AST};
use dyn_clone::DynClone;
// T = TypeVar("T")
// std::marker::Sync +
pub trait TransformerVisitorEx: DynClone +  AstTransformerVisitor {}
dyn_clone::clone_trait_object!(TransformerVisitorEx);
pub struct AstTransformerVisitorBase {
    log: bool,
}
pub trait AstTransformerVisitor {
    fn default() -> Self
    where
        Self: Sized;
    fn visit(&self, ast: Option<AST>) -> Option<AST>;
    fn visitBlock(
        &self,
        ast: Option<AST>,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> Option<AST>;
    fn visit_list(&self, ast_list: Vec<AST>) -> Vec<Option<AST>> {
        ast_list
            .iter()
            .map(|a| self.visit(Some(a.clone())))
            .collect()
    }
    fn visit_children(&self, mut _ast: Option<AST>) -> Option<AST> {
        // ast.process_children(self.visit);
        // ast
        None
    }

    fn _visit_internal(&self, _ast: Option<AST>) -> Option<AST> {
        None
    }
}
// class AstTransformerVisitor
// """
// Visitor which replaces visited AST elements by the corresponding visit functions return value

// The default action when no matching visit function is defined, is to replace the node with itself and to visit
// the children. If a matching visit function is defined, children are not automatically visited.
// (Corresponds to node-or-children traversal order from AstVisitor)
// """

impl AstTransformerVisitorBase {
    pub fn new(log: bool) -> Self {
        Self { log }
    }

    pub fn visit_children<T>(&self, mut ast: T) -> T {
        // ast.process_children(self.visit);
        ast
    }

    pub fn _visit_internal(&self, ast: Option<AST>) -> Option<AST> {
        if ast.is_none() {
            return ast;
        }

        if self.log {
            println!("Visiting {:?}", ast);
        }
        self.get_visit_function(ast)
    }

    pub fn get_visit_function(&self, c: Option<AST>) -> Option<AST> {
        // let visitor_function = "visit" + c.name();
        // if hasattr(&self, visitor_function) {
        //     return getattr(&self, visitor_function);
        // } else {
        //     for base in c.bases() {
        //         let f = self.get_visit_function(base);
        //         if f.is_some() {
        //             return f;
        //         }
        //     }
        // }
        // assert!(false);
        c
    }

    pub fn visitAST(&self, ast: Option<AST>) -> Option<AST> {
        self.visit_children(ast)
    }
}
impl AstTransformerVisitor for AstTransformerVisitorBase {
    fn default() -> Self {
        Self::new(false)
    }

    fn visit(&self, ast: Option<AST>) -> Option<AST> {
        self._visit_internal(ast)
    }
    fn visitBlock(
        &self,
        ast: Option<AST>,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> Option<AST> {
        self.visit_children(ast)
    }
}
