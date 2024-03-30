#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    is_instance, ASTType, BuiltinFunction, ConstructorOrFunctionDefinition, ExpressionBaseMutRef,
    ExpressionBaseProperty, ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty,
    FunctionCallExprBaseRef, IntoAST, LocationExpr, LocationExprBaseProperty, NamespaceDefinition,
    WhileStatement, AST,
};
use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitorBase, AstVisitorBaseRef, AstVisitorMut},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn call_graph_analysis(ast: &mut AST)
// """
// determines (indirectly) called functions for every function
// and concludes from that whether a function has a static body
// """
{
    let mut v = DirectCalledFunctionDetector::new();
    v.visit(ast);

    let mut v = IndirectCalledFunctionDetector::new();
    v.visit(ast);

    let mut v = IndirectDynamicBodyDetector::new();
    v.visit(ast);
}
#[derive(ASTVisitorBaseRefImpl)]
struct DirectCalledFunctionDetector {
    pub ast_visitor_base: AstVisitorBase,
}

// class DirectCalledFunctionDetector(FunctionVisitor)
impl FunctionVisitor for DirectCalledFunctionDetector {}
impl AstVisitorMut for DirectCalledFunctionDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name
            || &ASTType::ForStatement == name
            || &ASTType::WhileStatement == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_function_call_expr_mut()
                    .unwrap(),
            ),
            ASTType::ForStatement => self.visitForStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_for_statement_mut()
                    .unwrap(),
            ),
            ASTType::WhileStatement => self.visitWhileStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_while_statement_mut()
                    .unwrap(),
            ),
            _ => {}
        }
    }
}
impl DirectCalledFunctionDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        if !is_instance(&**ast.func(), ASTType::BuiltinFunction) && !ast.is_cast() {
            assert!(is_instance(&**ast.func(), ASTType::LocationExprBase));
            let fdef = ast
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .unwrap();
            assert!(fdef
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .is_function());
            if let Some(cofd) = fdef
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
            {
                let cofd = cofd.clone();
                ast.expression_base_mut_ref()
                    .statement
                    .as_mut()
                    .unwrap()
                    .statement_base_mut_ref()
                    .unwrap()
                    .function
                    .as_mut()
                    .unwrap()
                    .called_functions
                    .insert(cofd);
            }
        }
        self.visit_children(&mut ast.to_ast());
    }
    pub fn visitForStatement(&mut self, ast: &mut ForStatement) {
        ast.statement_base
            .function
            .as_mut()
            .unwrap()
            .has_static_body = false;
        self.visit_children(&mut ast.to_ast());
    }
    pub fn visitWhileStatement(&mut self, ast: &mut WhileStatement) {
        ast.statement_base
            .function
            .as_mut()
            .unwrap()
            .has_static_body = false;
        self.visit_children(&mut ast.to_ast());
    }
}
// class IndirectCalledFunctionDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectCalledFunctionDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectCalledFunctionDetector {}
impl AstVisitorMut for IndirectCalledFunctionDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::ConstructorOrFunctionDefinition == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::ConstructorOrFunctionDefinition => self.visitConstructorOrFunctionDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_constructor_or_function_definition_mut()
                    .unwrap(),
            ),
            _ => {}
        }
    }
}
impl IndirectCalledFunctionDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        ast: &mut ConstructorOrFunctionDefinition,
    )
    //Fixed point iteration
    {
        let mut size = 0;
        let mut leaves = ast.called_functions.clone();
        while ast.called_functions.len() > size {
            size = ast.called_functions.len();
            leaves = leaves
                .iter()
                .map(|leaf| {
                    leaf.called_functions
                        .iter()
                        .filter_map(|fct| {
                            if !ast.called_functions.contains(fct) {
                                Some(fct.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            ast.called_functions = ast.called_functions.union(&leaves).cloned().collect();
        }

        if ast.called_functions.contains(&ast) {
            ast.is_recursive = true;
            ast.has_static_body = false;
        }
    }
}
// class IndirectDynamicBodyDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectDynamicBodyDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectDynamicBodyDetector {}
impl AstVisitorMut for IndirectDynamicBodyDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::ConstructorOrFunctionDefinition == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::ConstructorOrFunctionDefinition => self.visitConstructorOrFunctionDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_constructor_or_function_definition_mut()
                    .unwrap(),
            ),
            _ => {}
        }
    }
}
impl IndirectDynamicBodyDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        ast: &mut ConstructorOrFunctionDefinition,
    ) {
        if !ast.has_static_body {
            return;
        }

        for fct in &ast.called_functions {
            if !fct.has_static_body
            // This function (directly or indirectly) calls a recursive function
            {
                ast.has_static_body = false;
                return;
            }
        }
    }
}
