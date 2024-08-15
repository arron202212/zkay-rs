#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    ASTChildren, ASTFlatten, ASTInstanceOf, ASTType, Block, HybridArgumentIdf, IntoAST, AST,
};
use dyn_clone::DynClone;
// T = TypeVar("T")
// std::marker::Sync +
pub trait TransformerVisitorEx: DynClone + AstTransformerVisitor {
    fn visitBlock(
        &self,
        _ast: &ASTFlatten,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> eyre::Result<ASTFlatten>;
}
dyn_clone::clone_trait_object!(TransformerVisitorEx);
#[derive(Clone)]
pub struct AstTransformerVisitorBase {
    log: bool,
}
impl AstTransformerVisitorBase {
    pub fn new(log: bool) -> Self {
        Self { log }
    }
}
pub trait AstTransformerVisitorBaseRef {
    fn ast_transformer_visitor_base_ref(&self) -> &AstTransformerVisitorBase;
}
pub trait AstTransformerVisitorBaseProperty {
    fn log(&self) -> bool;
}
impl<T: AstTransformerVisitorBaseRef> AstTransformerVisitorBaseProperty for T {
    fn log(&self) -> bool {
        self.ast_transformer_visitor_base_ref().log
    }
}

// class AstTransformerVisitor
// """
// Visitor which replaces visited AST elements by the corresponding visit functions return value

// The default action when no matching visit function is defined, is to replace the node with itself and to visit
// the children. If a matching visit function is defined, children are not automatically visited.
// (Corresponds to node-or-children traversal order from AstVisitor)
// """

pub trait AstTransformerVisitor: AstTransformerVisitorBaseProperty {
    // fn default() -> Self
    // where
    //     Self: Sized;

    fn visit(&self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        self._visit_internal(ast)
    }
    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool;
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<ASTFlatten>;
    fn visit_list(&self, ast_list: &[ASTFlatten]) -> Vec<ASTFlatten> {
        ast_list.iter().filter_map(|a| self.visit(a)).collect()
    }
    fn visit_children(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        for c in ast.children() {
            self.visit(&c);
        }
        Ok(ast.clone())
    }
    fn _visit_internal(&self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        if self.log() {
            // std::any::type_name::<Option<String>>(),
            // print!("Visiting {:?}", ast);
        }

        self.get_visit_function(&ast.get_ast_type(), ast)
    }

    fn get_visit_function(&self, c: &ASTType, ast: &ASTFlatten) -> Option<ASTFlatten> {
        if self.has_attr(c, &ast.to_ast()) {
            // if self.log() {
            //             print!("==get_attr========= {:?}", ast.get_ast_type());
            //         }
            self.get_attr(c, ast).ok()
        } else if let Some(_c) = AST::bases(c) {
            // if self.log() {
            //             print!("==get_visit_function==bases======{:?}==== {:?}", ast.get_ast_type(),_c);
            //         }
            self.get_visit_function(&_c, ast)
        } else {
            panic!("====get_visit_function====None");
        }
    }

    fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.visit_children(ast)
    }
}
