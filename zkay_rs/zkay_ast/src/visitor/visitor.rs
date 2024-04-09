#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{ASTChildren, ASTFlatten, ASTInstanceOf, ASTType, AST};

pub struct AstVisitorBase {
    pub traversal: String,
    pub log: bool,
}

impl AstVisitorBase {
    pub fn new(traversal: &str, log: bool) -> Self {
        Self {
            traversal: String::from(if traversal.is_empty() {
                "post"
            } else {
                traversal
            }),
            log,
        }
    }
}
pub trait AstVisitorBaseRef {
    fn ast_visitor_base_ref(&self) -> &AstVisitorBase;
}
pub trait AstVisitorBaseProperty {
    fn traversal(&self) -> &String;
    fn log(&self) -> bool;
}
impl<T: AstVisitorBaseRef> AstVisitorBaseProperty for T {
    fn traversal(&self) -> &String {
        &self.ast_visitor_base_ref().traversal
    }
    fn log(&self) -> bool {
        self.ast_visitor_base_ref().log
    }
}
pub trait AstVisitor: AstVisitorBaseProperty {
    type Return;
    fn visit(&self, ast: &ASTFlatten) -> Self::Return {
        self._visit_internal(ast).unwrap()
    }
    fn has_attr(&self, name: &ASTType) -> bool;
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return;
    fn temper_result(&self) -> Self::Return;
    fn _visit_internal(&self, ast: &ASTFlatten) -> Option<Self::Return> {
        if self.log() {
            // std::any::type_name::<Option<String>>(),
            print!("Visiting {:?}", ast);
        }
        let mut ret = None;
        let mut ret_children = None;

        if self.traversal() == "post" {
            ret_children = Some(self.visit_children(&ast));
        }
        let f = self.get_visit_function(ast.get_ast_type(), &ast);
        if f.is_some() {
            ret = f;
        } else if self.traversal() == "node-or-children" {
            ret_children = Some(self.visit_children(&ast));
        }
        if self.traversal() == "pre" {
            ret_children = Some(self.visit_children(&ast));
        }
        if ret.is_some() {
            // Some(ret)
            ret
        } else if ret_children.is_some() {
            ret_children
        } else {
            None
        }
    }

    fn get_visit_function(&self, c: ASTType, ast: &ASTFlatten) -> Option<Self::Return>
// std::any::type_name::<Option<String>>(),
    {
        // let _visitor_function = c; // String::from("visit") +
        if self.has_attr(&c) {
            return Some(self.get_attr(&c, ast));
        } else if let Some(c) = AST::bases(c) {
            let f = self.get_visit_function(c, ast);
            if f.is_some() {
                return f;
            }
        }
        None
    }
    fn visit_children(&self, ast: &ASTFlatten) -> Self::Return {
        let mut ast = ast.clone();
        for c in ast.children() {
            self.visit(&c);
        }
        self.temper_result()
    }
}
