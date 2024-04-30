#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    is_instance, ASTFlatten, ASTInstanceOf, ASTType, BuiltinFunction,
    ConstructorOrFunctionDefinition, Expression, ExpressionBaseMutRef, ExpressionBaseProperty,
    ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty, FunctionCallExprBaseRef, IntoAST,
    LocationExpr, LocationExprBaseProperty, NamespaceDefinition, WhileStatement, AST,
};
use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use rccell::RcCell;
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn call_graph_analysis(ast: &ASTFlatten)
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
impl AstVisitor for DirectCalledFunctionDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::FunctionCallExprBase | ASTType::ForStatement | ASTType::WhileStatement
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::WhileStatement => self.visitWhileStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl DirectCalledFunctionDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if !is_instance(
            ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::BuiltinFunction,
        ) && !ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .is_cast()
        {
            assert!(is_instance(
                ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
                ASTType::LocationExprBase
            ));
            let fdef = &ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .clone();
            assert!(fdef
                .clone()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .is_function());
            if let Some(cofd) = fdef
                .clone()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
            {
                let cofd = cofd.clone();
                ast.try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement
                    .as_ref()
                    .unwrap()
                    .clone()
                    .upgrade()
                    .unwrap()
                    .try_as_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .statement_base_mut_ref()
                    .unwrap()
                    .function
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_constructor_or_function_definition_mut()
                    .unwrap()
                    .borrow_mut()
                    .called_functions
                    .insert(RcCell::new(cofd));
            }
        }
        self.visit_children(ast)
    }
    pub fn visitForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_for_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base
            .function
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .borrow_mut()
            .has_static_body = false;
        self.visit_children(ast)
    }
    pub fn visitWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_while_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base
            .function
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .borrow_mut()
            .has_static_body = false;
        self.visit_children(ast)
    }
}
// class IndirectCalledFunctionDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectCalledFunctionDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectCalledFunctionDetector {}
impl AstVisitor for IndirectCalledFunctionDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, ast: &AST) -> bool {
        ASTType::ConstructorOrFunctionDefinition == ast.get_ast_type()
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl IndirectCalledFunctionDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    //Fixed point iteration
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut size = 0;
        let mut leaves = ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .called_functions
            .clone();
        while ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .called_functions
            .len()
            > size
        {
            size = ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .called_functions
                .len();
            leaves = leaves
                .iter()
                .map(|leaf| {
                    leaf.borrow()
                        .called_functions
                        .iter()
                        .filter_map(|fct| {
                            if !ast
                                .try_as_constructor_or_function_definition_ref()
                                .unwrap()
                                .borrow()
                                .called_functions
                                .contains(fct)
                            {
                                Some(fct.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .called_functions = ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .called_functions
                .union(&leaves)
                .cloned()
                .collect();
        }

        if ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .called_functions
            .contains(&ast.try_as_constructor_or_function_definition_ref().unwrap())
        {
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .is_recursive = true;
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .has_static_body = false;
        }
        Ok(())
    }
}
// class IndirectDynamicBodyDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectDynamicBodyDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectDynamicBodyDetector {}
impl AstVisitor for IndirectDynamicBodyDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        ASTType::ConstructorOrFunctionDefinition == ast.get_ast_type()
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            _ => Err(eyre::eyre!("unreach")),
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
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if !ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .has_static_body
        {
            return Ok(());
        }

        for fct in &ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .called_functions
        {
            // This function (directly or indirectly) calls a recursive function
            if !fct.borrow().has_static_body {
                ast.try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow_mut()
                    .has_static_body = false;
                return Ok(());
            }
        }
        Ok(())
    }
}
