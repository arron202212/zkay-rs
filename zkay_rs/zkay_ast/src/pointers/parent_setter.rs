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
    Identifier, IntoAST, NamespaceDefinition, NamespaceDefinitionBaseProperty, SourceUnit,
    Statement, AST,
};
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::{RcCell, WeakCell};
use zkay_derive::ASTVisitorBaseRefImpl;
#[derive(ASTVisitorBaseRefImpl)]
struct ParentSetterVisitor {
    pub ast_visitor_base: AstVisitorBase,
}

impl AstVisitor for ParentSetterVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::SourceUnit
                | ASTType::EnumDefinition
                | ASTType::ContractDefinition
                | ASTType::StructDefinition
                | ASTType::ConstructorOrFunctionDefinition
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::SourceUnit => self.visitSourceUnit(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            ASTType::NamespaceDefinitionBase => self.visitNamespaceDefinition(ast),
            _ => {}
        }
    }

    fn visit_children(&self, ast: &ASTFlatten) -> Self::Return {
        for c in ast.children().iter_mut() {
            // println!("=={:?}==={:?}===children=={:?}======={:?}",ast.get_ast_type(),c.get_ast_type(),ast.to_string(), c.to_string());
            c.ast_base_ref().unwrap().borrow_mut().parent = Some(ast.clone().downgrade());
            c.ast_base_ref().unwrap().borrow_mut().namespace =
                ast.ast_base_ref().unwrap().borrow().namespace().clone();
            self.visit(c);
        }
    }
}

impl ParentSetterVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("pre", false),
        }
    }
    pub fn visitSourceUnit(&self, ast: &ASTFlatten) {
        ast.try_as_source_unit_ref()
            .unwrap()
            .borrow_mut()
            .ast_base
            .borrow_mut()
            .namespace = Some(vec![]);
    }

    pub fn visitNamespaceDefinition(&self, ast: &ASTFlatten) {
        ast.try_as_namespace_definition_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .namespace = Some(
            if let Some(parent) = ast
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .parent()
            {
                parent
                    .upgrade()
                    .unwrap()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .namespace
                    .as_ref()
                    .unwrap()
                    .iter()
                    .cloned()
                    .chain([ast
                        .try_as_namespace_definition_ref()
                        .unwrap()
                        .borrow()
                        .idf()
                        .clone()])
                    .collect()
            } else {
                vec![ast
                    .try_as_namespace_definition_ref()
                    .unwrap()
                    .borrow()
                    .idf()]
            },
        );
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &ASTFlatten) {
        ast.try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow_mut()
            .namespace_definition_base
            .ast_base
            .borrow_mut()
            .namespace = Some(
            if let Some(parent) = &ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .parent
            {
                parent
                    .namespace_definition_base
                    .ast_base
                    .borrow()
                    .namespace()
                    .as_ref()
                    .unwrap()
                    .into_iter()
                    .chain([&ast
                        .try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .borrow()
                        .namespace_definition_base
                        .idf()
                        .clone()])
                    .cloned()
                    .collect()
            } else {
                vec![ast
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow()
                    .namespace_definition_base
                    .idf()
                    .clone()]
            },
        );
    }
}
#[derive(ASTVisitorBaseRefImpl)]
struct ExpressionToStatementVisitor {
    pub ast_visitor_base: AstVisitorBase,
}

impl AstVisitor for ExpressionToStatementVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(name, ASTType::ExpressionBase | ASTType::StatementBase)
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::ExpressionBase => self.visitExpression(ast),
            ASTType::StatementBase => self.visitStatement(ast),
            _ => {}
        }
    }
}

impl ExpressionToStatementVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn visitExpression(&self, ast: &ASTFlatten) {
        let mut parent = Some(ast.clone());
        while let Some(p) = parent.clone() {
            if is_instance(&p, ASTType::StatementBase) {
                break;
            }
            parent = p
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .ast_base_ref()
                .clone()
                .unwrap()
                .parent()
                .as_ref()
                .map(|p| p.clone().upgrade())
                .flatten()
                .clone();
        }
        if parent.is_some() {
            ast.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement = parent.map(|p| p.clone().downgrade());
        }
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) {
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
                .map(|p| p.clone().upgrade())
                .flatten()
                .clone();
        }
        if parent.is_some() {
            ast.try_as_statement_ref()
                .unwrap()
                .borrow_mut()
                .statement_base_mut_ref()
                .unwrap()
                .function = parent.map(|p| p.clone());
        }
    }
}

pub fn set_parents(ast: &ASTFlatten) {
    let mut v = ParentSetterVisitor::new();
    v.visit(ast);
    let mut v = ExpressionToStatementVisitor::new();
    v.visit(ast);
}
