#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    is_instance, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTFlatten,
    ASTInstanceOf, ASTType, ConstructorOrFunctionDefinition, Expression, ExpressionBaseMutRef,
    Identifier, IntoAST, NamespaceDefinition, SourceUnit, Statement, StatementBaseMutRef, AST,
};
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::{RcCell, WeakCell};
use zkay_derive::ASTVisitorBaseRefImpl;
#[derive(ASTVisitorBaseRefImpl)]
struct ParentSetterVisitor {
    pub ast_visitor_base: AstVisitorBase,
}

impl AstVisitor for ParentSetterVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit | ASTType::ConstructorOrFunctionDefinition
        ) || matches!(ast, AST::NamespaceDefinition(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => self.visitSourceUnit(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            _ if matches!(ast.to_ast(), AST::NamespaceDefinition(_)) => {
                self.visitNamespaceDefinition(ast)
            }
            _ => Err(eyre::eyre!("unreach")),
        }
    }

    fn visit_children(&self, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        for c in ast.children() {
            c.ast_base_ref().unwrap().borrow_mut().parent = Some(ast.clone().downgrade());
            c.ast_base_ref().unwrap().borrow_mut().namespace =
                ast.ast_base_ref().unwrap().borrow().namespace().clone();
            // println!("========================{:?},{:?}",ast.get_ast_type(),c.get_ast_type());
            // println!(
            //         "=0000000={:?}==={:?}===children=={:?}======={:?}",
            //         ast.get_ast_type(),
            //         c.get_ast_type(),
            //         ast.to_string(),
            //         c.to_string()
            //     );
            self.visit(&c); //stack overflow TODO
        }
        Ok(())
    }
}

impl ParentSetterVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("pre", false),
        }
    }
    pub fn visitSourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_source_unit_ref()
            .unwrap()
            .borrow_mut()
            .ast_base
            .borrow_mut()
            .namespace = Some(vec![]);
        Ok(())
    }

    pub fn visitNamespaceDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitNamespaceDefinition==========={:?}", ast);
        let mut ast = ast.to_ast();
        let namespace = ast
            .try_as_namespace_definition_ref()
            .unwrap()
            .parent()
            .map(|parent| {
                let mut p: Vec<_> = parent
                    .upgrade()
                    .unwrap()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .namespace
                    .clone()
                    .unwrap();
                p.push(
                    ast.try_as_namespace_definition_ref()
                        .unwrap()
                        .idf()
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                );
                p
            })
            .or(Some(vec![ast
                .try_as_namespace_definition_ref()
                .unwrap()
                .idf()
                .as_ref()
                .unwrap()
                .downgrade()]));

        ast.try_as_namespace_definition_ref()
            .unwrap()
            .ast_base_ref()
            .borrow_mut()
            .namespace = namespace;
        Ok(())
    }

    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let namespace = ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .parent
            .as_ref()
            .map(|parent| {
                let mut p: Vec<_> = parent
                    .namespace_definition_base
                    .ast_base
                    .borrow()
                    .namespace()
                    .as_ref()
                    .unwrap()
                    .to_vec();
                p.push(
                    ast.try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .borrow()
                        .idf()
                        .clone()
                        .unwrap()
                        .downgrade(),
                );
                p
            })
            .or(Some(vec![ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .idf()
                .clone()
                .unwrap()
                .downgrade()]));

        ast.try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow_mut()
            .namespace_definition_base
            .ast_base
            .borrow_mut()
            .namespace = namespace;
        Ok(())
    }
}
#[derive(ASTVisitorBaseRefImpl)]
struct ExpressionToStatementVisitor {
    pub ast_visitor_base: AstVisitorBase,
}

impl AstVisitor for ExpressionToStatementVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::ExpressionBase | ASTType::StatementBase
        ) || matches!(ast, AST::Expression(_))
            || matches!(ast, AST::Statement(_))
    }
    fn get_attr(
        &self,
        name: &ASTType,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        match name {
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}

impl ExpressionToStatementVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut parent = Some(ast.clone());
        while let Some(p) = parent.clone() {
            if is_instance(&p, ASTType::StatementBase) {
                break;
            }

            //   println!("=====visitExpression===========s========={:?}",p.try_as_expression_ref()  .unwrap()
            //                 .borrow()
            //                 .ast_base_ref()
            //                 .clone());
            parent = p
                .to_ast()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .as_ref()
                .and_then(|p| p.clone().upgrade())
                .clone();
        }
        if parent.is_some() {
            // if ast.get_ast_type() == ASTType::IdentifierExpr {
            //     // println!(
            //     //     "=====statement========={:?}==========={:?}",
            //     //     parent.as_ref().unwrap().to_string(),
            //     //     parent.as_ref().unwrap().get_ast_type()
            //     // );
            // }
            if ast.is_expression() {
                ast.try_as_expression_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = parent.map(|p| p.clone().downgrade());
                //      println!("=====statement=====is_expression===={:?}===========",ast.try_as_expression_ref()
                // .unwrap()
                // .borrow_mut()
                // .expression_base_mut_ref()
                // .statement.is_some());
            } else if ast.is_location_expr() {
                ast.try_as_location_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = parent.map(|p| p.clone().downgrade());
            } else if ast.is_tuple_or_location_expr() {
                ast.try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = parent.map(|p| p.clone().downgrade());
            } else if ast.is_member_access_expr() {
                ast.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = parent.map(|p| p.clone().downgrade());
            } else if ast.is_index_expr() {
                ast.try_as_index_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = parent.map(|p| p.clone().downgrade());
            } else {
                panic!("===================else======={ast:?}");
            }
        }
        Ok(())
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut parent = Some(ast.clone());
        while let Some(p) = parent.clone() {
            if is_instance(&p, ASTType::ConstructorOrFunctionDefinition) {
                break;
            }
            parent = p
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .as_ref()
                .and_then(|p| p.clone().upgrade())
                .clone();
        }
        if parent.is_some() {
            // println!(
            //     "=====visitStatement====get_ast_type===={:?}",
            //     ast.get_ast_type()
            // );
            if ast.is_block() {
                ast.try_as_block_ref()
                    .unwrap()
                    .borrow_mut()
                    .statement_base_mut_ref()
                    .function = parent.map(|p| p.clone().downgrade());
            } else if ast.is_ast() {
                ast.try_as_ast_ref()
                    .unwrap()
                    .borrow_mut()
                    .try_as_statement_mut()
                    .unwrap()
                    .statement_base_mut_ref()
                    .unwrap()
                    .function = parent.map(|p| p.clone().downgrade());
            } else if ast.is_simple_statement() {
                ast.try_as_simple_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .statement_base_mut_ref()
                    .function = parent.map(|p| p.clone().downgrade());
            } else {
                // println!(
                //     "=====visitStatement=======else==={:?}=====",
                //     ast.get_ast_type()
                // );
                eyre::bail!("=========else===========");
            }
        }
        Ok(())
    }
}

pub fn set_parents(ast: &ASTFlatten) {
    let v = ParentSetterVisitor::new();
    v.visit(ast);
    let v = ExpressionToStatementVisitor::new();
    v.visit(ast);
}
