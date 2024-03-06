#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    is_instance, ASTType, Block, ConstructorOrFunctionDefinition, IntoAST, ReturnStatement, AST,
}; //, AstException
use crate::visitor::visitor::AstVisitor;

pub fn check_return(ast: &AST) {
    let v = ReturnCheckVisitor;
    v.visit(ast.clone());
}
// class ReturnPositionException(AstException):

// pub fn __init__(&self, ast: ReturnStatement):
//     super().__init__("Return statements are only allowed at the end of a function.", ast)

// class ReturnCheckVisitor(AstVisitor):

struct ReturnCheckVisitor;
impl AstVisitor for ReturnCheckVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, _name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, _ast: &AST) -> Self::Return {
        None
    }
}
impl ReturnCheckVisitor {
    pub fn visitReturnStatement(&self, ast: &mut ReturnStatement) {
        let container = ast.statement_base.ast_base.parent.clone().unwrap();
        // assert!(is_instance(&*container,ASTType::Block));
        let mut ok = true;
        if container
            .statement_list_base()
            .unwrap()
            .statements
            .last()
            .map(|v| v.clone())
            .unwrap()
            != ast.to_ast()
        {
            ok = false;
        }
        if !is_instance(
            &container.parent().unwrap(),
            ASTType::ConstructorOrFunctionDefinition,
        ) || container
            .parent()
            .unwrap()
            .constructor_or_function_definition()
            .unwrap()
            .is_constructor()
        {
            ok = false;
        }
        // raise ReturnPositionException(ast)}
        assert!(
            ok,
            "Return statements are only allowed at the end of a function. {:?}",
            ast,
        );
    }
}
