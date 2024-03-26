#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    is_instance, ASTBaseProperty, ASTType, Block, ConstructorOrFunctionDefinition, IntoAST,
    ReturnStatement, StatementListBaseProperty, AST,
}; //, AstException
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_return(ast: &AST) {
    let v = ReturnCheckVisitor::new();
    v.visit(ast);
}
// class ReturnPositionException(AstException):

// pub fn __init__(&self, ast: ReturnStatement):
//     super().__init__("Return statements are only allowed at the end of a function.", ast)

// class ReturnCheckVisitor(AstVisitor):

#[derive(ASTVisitorBaseRefImpl)]
struct ReturnCheckVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitor for ReturnCheckVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::ReturnStatement == name
    }
    fn get_attr(&self, name: &ASTType, ast: &AST) -> Self::Return {
        match name {
            ASTType::ReturnStatement => self.visitReturnStatement(
                ast.try_as_statement_ref()
                    .unwrap()
                    .try_as_return_statement_ref()
                    .unwrap(),
            ),
            _ => {}
        }
    }
}
impl ReturnCheckVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn visitReturnStatement(&self, ast: &ReturnStatement) {
        let container = ast.parent().clone().unwrap();
        // assert!(is_instance(&*container,ASTType::Block));
        let mut ok = true;
        if container
            .try_as_statement_ref()
            .unwrap()
            .try_as_statement_list_ref()
            .unwrap()
            .statements()
            .last()
            .map(|v| v.clone())
            .unwrap()
            != ast.to_ast()
        {
            ok = false;
        }
        if !is_instance(
            &**container.ast_base_ref().unwrap().parent().as_ref().unwrap(),
            ASTType::ConstructorOrFunctionDefinition,
        ) || container
            .ast_base_ref()
            .unwrap()
            .parent()
            .as_ref()
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
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
