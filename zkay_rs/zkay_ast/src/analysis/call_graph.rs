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
    ExpressionBaseRef, ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty,
    FunctionCallExprBaseRef, IntoAST, LocationExpr, LocationExprBaseProperty, NamespaceDefinition,
    WhileStatement, AST,
};
use crate::visitors::{
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
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::FunctionCallExprBase
                | ASTType::ForStatement
                | ASTType::WhileStatement
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
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
        // let fce=ast.try_as_expression_ref().unwrap().borrow().try_as_function_call_expr_ref().unwrap().clone();
        // println!("====visitFunctionCallExpr============{:?}",ast);
        if !is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::BuiltinFunction,
        ) && !ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .is_cast()
        {
            assert!(is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::LocationExprBase
            ));
            let fdef = &ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
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
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .is_function());
            if let Some(cofd) = fdef
                .clone()
                .upgrade()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .map(|d| d.borrow().clone())
            {
                let cofd = cofd.clone();
                // println!(
                //     "====.upgrade()=======function=============={:?}",
                //     ast.to_ast()
                //         .try_as_expression_ref()
                //         .unwrap()
                //         .expression_base_ref()
                //         .statement
                //         .as_ref()
                //         .unwrap()
                //         .clone()
                //         .upgrade()
                //         .unwrap()
                //         .get_ast_type()
                // );

                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .expression_base_ref()
                    .statement
                    .as_ref()
                    .unwrap()
                    .clone()
                    .upgrade()
                    .unwrap()
                    .to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .statement_base_ref()
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
        ast.to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
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
        // println!("=======visitWhileStatement================{:?}",ast);
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_while_statement_mut()
            .unwrap()
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
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit | ASTType::Parameter | ASTType::ConstructorOrFunctionDefinition
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
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
                .flat_map(|leaf| {
                    leaf.borrow()
                        .called_functions
                        .iter()
                        .filter(|fct| {
                            !ast.try_as_constructor_or_function_definition_ref()
                                .unwrap()
                                .borrow()
                                .called_functions
                                .contains(fct)
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .collect();
            let cf = ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .called_functions
                .union(&leaves)
                .cloned()
                .collect();
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .called_functions = cf;
        }

        if ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .called_functions
            .contains(ast.try_as_constructor_or_function_definition_ref().unwrap())
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
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit | ASTType::Parameter | ASTType::ConstructorOrFunctionDefinition
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
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

        let v = ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .called_functions
            .iter()
            .any(|fct| !fct.borrow().has_static_body);
        if v {
            // This function (directly or indirectly) calls a recursive function
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .has_static_body = false;
        }
        Ok(())
    }
}
