#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    is_instance, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, Block,
    ConstructorOrFunctionDefinition, IntoAST, ReturnStatement, StatementListBaseProperty, AST,
}; //, AstException
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_return(ast: &ASTFlatten) {
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
    fn has_attr(&self, ast: &AST) -> bool {
        ASTType::ReturnStatement == ast.get_ast_type()
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl ReturnCheckVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn visitReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("========visitReturnStatement========================{:?}",ast);
        let container = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_return_statement_ref()
            .unwrap()
            .parent()
            .unwrap()
            .upgrade()
            .unwrap();
        assert!(is_instance(&container, ASTType::Block));
        let mut ok = true;
        if container
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_statement_list_ref()
            .unwrap()
            .statements()
            .last()
            .unwrap()
            != ast
        {
            ok = false;
        }
        if !is_instance(
            &container
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap(),
            ASTType::ConstructorOrFunctionDefinition,
        ) || container
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .parent()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .is_constructor()
        {
            ok = false;
        }
        // raise ReturnPositionException(ast)}
        eyre::ensure!(
            ok,
            "Return statements are only allowed at the end of a function. {:?}",
            ast,
        );
        Ok(())
    }
}
