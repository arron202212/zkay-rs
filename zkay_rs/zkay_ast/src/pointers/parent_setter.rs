#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    is_instance, ASTBaseMutRef, ASTBaseProperty, ASTChildren, ASTFlatten, ASTInstanceOf, ASTType,
    ConstructorOrFunctionDefinition, Expression, ExpressionBaseMutRef, Identifier, IntoAST,
    NamespaceDefinition, NamespaceDefinitionBaseProperty, SourceUnit, Statement, AST,
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
    fn get_attr(&mut self, _name: &ASTType, ast: &mut AST) -> Self::Return {
        match ast {
            AST::SourceUnit(_) => self.visitSourceUnit(ast.try_as_source_unit_mut().unwrap()),
            AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(_)) => {
                self.visitConstructorOrFunctionDefinition(
                    ast.try_as_namespace_definition_mut()
                        .unwrap()
                        .try_as_constructor_or_function_definition_mut()
                        .unwrap(),
                )
            }
            AST::NamespaceDefinition(_) => {
                self.visitNamespaceDefinition(ast.try_as_namespace_definition_mut().unwrap())
            }
            _ => {}
        }
    }

    fn visit_children(&mut self, ast: &mut AST) -> Self::Return {
        for c in ast.children().iter_mut() {
            // println!("=={:?}==={:?}===children=={:?}======={:?}",ast.get_ast_type(),c.get_ast_type(),ast.to_string(), c.to_string());
            c.ast_base_mut_ref().unwrap().parent = Some(Box::new(ast.clone()));
            c.ast_base_mut_ref().unwrap().namespace =
                ast.ast_base_ref().unwrap().namespace().clone();
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
    pub fn visitSourceUnit(&self, ast: &mut SourceUnit) {
        ast.ast_base.namespace = Some(vec![]);
    }

    pub fn visitNamespaceDefinition(&self, ast: &mut NamespaceDefinition) {
        ast.ast_base_mut_ref().namespace = Some(if let Some(parent) = ast.parent() {
            parent
                .ast_base_ref()
                .unwrap()
                .namespace
                .as_ref()
                .unwrap()
                .iter()
                .cloned()
                .chain([ast.idf().clone()])
                .collect()
        } else {
            vec![ast.idf()]
        });
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &mut ConstructorOrFunctionDefinition) {
        ast.namespace_definition_base.ast_base.namespace =
            Some(if let Some(parent) = &ast.parent {
                parent
                    .namespace_definition_base
                    .ast_base
                    .namespace()
                    .as_ref()
                    .unwrap()
                    .into_iter()
                    .chain([&ast.namespace_definition_base.idf().clone()])
                    .cloned()
                    .collect()
            } else {
                vec![ast.namespace_definition_base.idf().clone()]
            });
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
    fn get_attr(&mut self, _name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match ast {
            AST::Expression(_) => self.visitExpression(ast.try_as_expression_mut().unwrap()),
            AST::Statement(_) => self.visitStatement(ast.try_as_statement_mut().unwrap()),
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
    pub fn visitExpression(&self, ast: RcCell<Expression>) {
        let mut parent = Some(ast.to_ast());
        while let Some(p) = &parent {
            if let AST::Statement(_) = p {
                break;
            }
            parent = p
                .ast_base_ref()
                .unwrap()
                .parent()
                .as_ref()
                .map(|p| *p.clone());
        }
        if parent.is_some() {
            ast.expression_base_mut_ref().statement =
                parent.map(|p| Box::new(p.try_as_statement().unwrap()));
        }
    }

    pub fn visitStatement(&self, ast: RcCell<Statement>) {
        let mut parent = Some(ast.to_ast());
        while let Some(p) = &parent {
            if is_instance(p, ASTType::ConstructorOrFunctionDefinition) {
                break;
            }
            parent = p
                .ast_base_ref()
                .unwrap()
                .parent()
                .as_ref()
                .map(|p| *p.clone());
        }
        if parent.is_some() {
            ast.statement_base_mut_ref().unwrap().function = parent.map(|p| {
                Box::new(
                    p.try_as_namespace_definition()
                        .unwrap()
                        .try_as_constructor_or_function_definition()
                        .unwrap(),
                )
            });
        }
    }
}

pub fn set_parents(ast: &ASTFlatten) {
    let mut v = ParentSetterVisitor::new();
    v.visit(ast);
    let mut v = ExpressionToStatementVisitor::new();
    v.visit(ast);
}
