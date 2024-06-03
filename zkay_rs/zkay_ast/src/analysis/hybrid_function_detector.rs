#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use rccell::RcCell;
// use type_check::type_exceptions::TypeException
use crate::ast::{
    is_instance, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, AllExpr, BuiltinFunction,
    ConstructorOrFunctionDefinition, Expression, ExpressionBaseMutRef, ExpressionBaseProperty,
    FunctionCallExpr, FunctionCallExprBaseProperty, IntoAST, LocationExpr,
    LocationExprBaseProperty, PrimitiveCastExpr, ReclassifyExpr, AST,
};
use crate::visitors::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn detect_hybrid_functions(ast: &ASTFlatten)
// """
// :param ast
// :return: marks all functions which will require verification
// """
{
    let mut v = DirectHybridFunctionDetectionVisitor::new();
    v.visit(ast);

    let mut v = IndirectHybridFunctionDetectionVisitor::new();
    v.visit(ast);

    let mut v = NonInlineableCallDetector::new();
    v.visit(ast);
}

// class DirectHybridFunctionDetectionVisitor(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct DirectHybridFunctionDetectionVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for DirectHybridFunctionDetectionVisitor {}
impl AstVisitor for DirectHybridFunctionDetectionVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::ReclassifyExpr
                | ASTType::PrimitiveCastExpr
                | ASTType::AllExpr
                | ASTType::FunctionCallExprBase
                | ASTType::ConstructorOrFunctionDefinition
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            ASTType::AllExpr => self.visitAllExpr(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl DirectHybridFunctionDetectionVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitReclassifyExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if is_instance(ast, ASTType::ReclassifyExpr) {
            println!("======*********************=============");
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement
                .as_mut()
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
                .requires_verification = true;
        }
        Ok(())
    }

    pub fn visitPrimitiveCastExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ret = Ok(());
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_primitive_cast_expr_ref()
            .unwrap()
            .expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .evaluate_privately()
        {
            println!("======*********************=============");
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .statement()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_statement_ref()
                .unwrap()
                .borrow()
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
                .requires_verification = true;
        } else {
            ret = self.visit_children(ast);
        }
        ret
    }

    pub fn visitAllExpr(&self, _ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(())
    }
    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let flag = is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::BuiltinFunction,
        ) && ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .func()
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_builtin_function_ref()
            .unwrap()
            .is_private
            || ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .is_cast()
                && ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .evaluate_privately();
        if flag {
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .statement()
                .clone()
                .unwrap()
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
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .requires_verification = true;
            return Ok(());
        }

        self.visit_children(ast)
    }
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit(
            &ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .body
                .clone()
                .unwrap()
                .into(),
        );

        if ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .can_be_external()
        {
            if ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .requires_verification
            {
                ast.try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow_mut()
                    .requires_verification_when_external = true;
            } else {
                let is_private = ast
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow()
                    .parameters
                    .iter()
                    .any(|param| {
                        param
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .is_private()
                    });

                if is_private {
                    ast.try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .borrow_mut()
                        .requires_verification_when_external = true;
                }
            }
        }
        Ok(())
    }
}
// class IndirectHybridFunctionDetectionVisitor(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectHybridFunctionDetectionVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectHybridFunctionDetectionVisitor {}
impl AstVisitor for IndirectHybridFunctionDetectionVisitor {
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
impl IndirectHybridFunctionDetectionVisitor {
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
            .requires_verification
        {
            for fct in &ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .called_functions
            {
                if fct.borrow().requires_verification {
                    ast.try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .borrow_mut()
                        .requires_verification = true;
                    if ast
                        .try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .borrow()
                        .can_be_external()
                    {
                        ast.try_as_constructor_or_function_definition_ref()
                            .unwrap()
                            .borrow_mut()
                            .requires_verification_when_external = true;
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}
// class NonInlineableCallDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct NonInlineableCallDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for NonInlineableCallDetector {}
impl AstVisitor for NonInlineableCallDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(ast.get_ast_type(), ASTType::SourceUnit | ASTType::Parameter)
            || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
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

            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl NonInlineableCallDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if !ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .is_cast()
            && is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::LocationExprBase,
            )
        {
            let ast1 = ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap();
            assert!(
                !(ast1
                    .to_ast()
                    .try_as_namespace_definition_ref()
                    .unwrap()
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .requires_verification
                    && ast1
                        .to_ast()
                        .try_as_namespace_definition_ref()
                        .unwrap()
                        .try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .is_recursive),
                "Non-inlineable call to recursive private function {:?}",
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func()
            )
        }
        self.visit_children(ast)
    }
}
