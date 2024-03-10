#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    is_instance, ASTType, BuiltinFunction, ConstructorOrFunctionDefinition, ExpressionBaseMutRef,
    ExpressionBaseProperty, ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty, IntoAST,
    LocationExpr, LocationExprBaseProperty, NamespaceDefinition, WhileStatement, AST,FunctionCallExprBaseRef,
};
use crate::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};

pub fn call_graph_analysis(ast: AST)
// """
// determines (indirectly) called functions for every function
// and concludes from that whether a function has a static body
// """
{
    let v = DirectCalledFunctionDetector;
    v.visit(ast.clone());

    let v = IndirectCalledFunctionDetector;
    v.visit(ast.clone());

    let v = IndirectDynamicBodyDetector;
    v.visit(ast);
}
struct DirectCalledFunctionDetector;

// class DirectCalledFunctionDetector(FunctionVisitor)
impl FunctionVisitor for DirectCalledFunctionDetector {}
impl AstVisitor for DirectCalledFunctionDetector {
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
impl DirectCalledFunctionDetector {
    pub fn visitFunctionCallExpr(&self, mut ast: FunctionCallExpr) {
        if !is_instance(&**ast.func(), ASTType::BuiltinFunction) && !ast.is_cast() {
            assert!(is_instance(&**ast.func(), ASTType::LocationExprBase));
            let fdef = ast
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
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
        self.visit_children(&ast.to_ast());
    }
    pub fn visitForStatement(&self, mut ast: ForStatement) {
        ast.statement_base
            .function
            .as_mut()
            .unwrap()
            .has_static_body = false;
        self.visit_children(&ast.to_ast());
    }
    pub fn visitWhileStatement(&self, mut ast: WhileStatement) {
        ast.statement_base
            .function
            .as_mut()
            .unwrap()
            .has_static_body = false;
        self.visit_children(&ast.to_ast());
    }
}
// class IndirectCalledFunctionDetector(FunctionVisitor)
struct IndirectCalledFunctionDetector;
impl FunctionVisitor for IndirectCalledFunctionDetector {}
impl AstVisitor for IndirectCalledFunctionDetector {
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
impl IndirectCalledFunctionDetector {
    pub fn visitConstructorOrFunctionDefinition(&self, mut ast: ConstructorOrFunctionDefinition)
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
pub struct IndirectDynamicBodyDetector;

impl FunctionVisitor for IndirectDynamicBodyDetector {}
impl AstVisitor for IndirectDynamicBodyDetector {
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
impl IndirectDynamicBodyDetector {
    pub fn visitConstructorOrFunctionDefinition(&self, mut ast: ConstructorOrFunctionDefinition) {
        if !ast.has_static_body {
            return;
        }

        for fct in ast.called_functions {
            if !fct.has_static_body
            // This function (directly or indirectly) calls a recursive function
            {
                ast.has_static_body = false;
                return;
            }
        }
    }
}
