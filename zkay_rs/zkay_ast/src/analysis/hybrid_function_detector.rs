#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// use type_check::type_exceptions::TypeException
use crate::ast::{
    is_instance, ASTType, AllExpr, BuiltinFunction, ConstructorOrFunctionDefinition,
    ExpressionBaseMutRef, ExpressionBaseProperty, FunctionCallExpr, FunctionCallExprBaseProperty,
    IntoAST, LocationExpr, LocationExprBaseProperty, PrimitiveCastExpr, ReclassifyExpr, AST,
};
use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitorBase, AstVisitorBaseRef, AstVisitorMut},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn detect_hybrid_functions(ast: &mut AST)
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
impl AstVisitorMut for DirectHybridFunctionDetectionVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn has_attr(&self, name: &ASTType) -> bool {
        false
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        None
    }
}
impl DirectHybridFunctionDetectionVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitReclassifyExpr(&mut self, ast: &mut ReclassifyExpr) {
        if let ReclassifyExpr::ReclassifyExpr(re) = ast {
            re.expression_base
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .function
                .as_mut()
                .unwrap()
                .requires_verification = true;
        }
    }

    pub fn visitPrimitiveCastExpr(&mut self, ast: &mut PrimitiveCastExpr) {
        if ast.expr.evaluate_privately() {
            ast.expression_base
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .function
                .as_mut()
                .unwrap()
                .requires_verification = true;
        } else {
            self.visit_children(&mut ast.to_ast());
        }
    }

    pub fn visitAllExpr(&mut self, _ast: &mut AllExpr) {}
    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction)
            && ast.func().try_as_builtin_function_ref().unwrap().is_private
        {
            ast.expression_base_mut_ref()
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .function
                .as_mut()
                .unwrap()
                .requires_verification = true;
        } else if ast.is_cast() && ast.evaluate_privately() {
            ast.expression_base_mut_ref()
                .statement
                .as_mut()
                .unwrap()
                .as_mut()
                .statement_base_mut_ref()
                .unwrap()
                .function
                .as_mut()
                .unwrap()
                .requires_verification = true;
        } else {
            self.visit_children(&mut ast.to_ast());
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        ast: &mut ConstructorOrFunctionDefinition,
    ) {
        self.visit(&mut ast.body.as_ref().unwrap().to_ast());

        if ast.can_be_external() {
            if ast.requires_verification {
                ast.requires_verification_when_external = true;
            } else {
                for param in &ast.parameters {
                    if param
                        .identifier_declaration_base
                        .annotated_type
                        .is_private()
                    {
                        ast.requires_verification_when_external = true;
                        break;
                    }
                }
            }
        }
    }
}
// class IndirectHybridFunctionDetectionVisitor(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectHybridFunctionDetectionVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectHybridFunctionDetectionVisitor {}
impl AstVisitorMut for IndirectHybridFunctionDetectionVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }

    fn has_attr(&self, name: &ASTType) -> bool {
        false
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        None
    }
}
impl IndirectHybridFunctionDetectionVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        ast: &mut ConstructorOrFunctionDefinition,
    ) {
        if !ast.requires_verification {
            for fct in &ast.called_functions {
                if fct.requires_verification {
                    ast.requires_verification = true;
                    if ast.can_be_external() {
                        ast.requires_verification_when_external = true;
                    }
                    break;
                }
            }
        }
    }
}
// class NonInlineableCallDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct NonInlineableCallDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for NonInlineableCallDetector {}
impl AstVisitorMut for NonInlineableCallDetector {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }

    fn has_attr(&self, name: &ASTType) -> bool {
        false
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        None
    }
}
impl NonInlineableCallDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        if !ast.is_cast() && is_instance(&**ast.func(), ASTType::LocationExprBase) {
            let ast1 = ast
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap();
            assert!(
                !(ast1
                    .try_as_namespace_definition_ref()
                    .unwrap()
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .requires_verification
                    && ast1
                        .try_as_namespace_definition_ref()
                        .unwrap()
                        .try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .is_recursive),
                "Non-inlineable call to recursive private function {:?}",
                ast.func()
            )
        }
        self.visit_children(&mut ast.to_ast());
    }
}
