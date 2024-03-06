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
    IntoAST, LocationExpr, PrimitiveCastExpr, ReclassifyExpr, AST,
};
use crate::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};

pub fn detect_hybrid_functions(ast: AST)
// """
// :param ast
// :return: marks all functions which will require verification
// """
{
    let v = DirectHybridFunctionDetectionVisitor;
    v.visit(ast.clone());

    let v = IndirectHybridFunctionDetectionVisitor;
    v.visit(ast.clone());

    let v = NonInlineableCallDetector;
    v.visit(ast);
}

// class DirectHybridFunctionDetectionVisitor(FunctionVisitor)
pub struct DirectHybridFunctionDetectionVisitor;

impl FunctionVisitor for DirectHybridFunctionDetectionVisitor {}
impl AstVisitor for DirectHybridFunctionDetectionVisitor {
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
impl DirectHybridFunctionDetectionVisitor {
    pub fn visitReclassifyExpr(&self, ast: ReclassifyExpr) {
        if let ReclassifyExpr::ReclassifyExpr(re) = ast {
            re.expression_base
                .statement
                .unwrap()
                .function()
                .unwrap()
                .requires_verification = true;
        }
    }

    pub fn visitPrimitiveCastExpr(&self, mut ast: PrimitiveCastExpr) {
        if ast.expr.evaluate_privately() {
            ast.expression_base
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut()
                .unwrap()
                .function
                .as_mut()
                .unwrap()
                .requires_verification = true;
        } else {
            self.visit_children(&ast.to_ast());
        }
    }

    pub fn visitAllExpr(&self, _ast: AllExpr)
    // pass
    {
    }
    pub fn visitFunctionCallExpr(&self, ast: &mut FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) && ast.func().is_private() {
            ast.expression_base_mut_ref()
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut()
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
                .statement_base_mut()
                .unwrap()
                .function
                .as_mut()
                .unwrap()
                .requires_verification = true;
        } else {
            self.visit_children(&ast.to_ast());
        }
    }
    pub fn visitConstructorOrFunctionDefinition(&self, mut ast: ConstructorOrFunctionDefinition) {
        self.visit(ast.body.as_ref().unwrap().to_ast());

        if ast.can_be_external() {
            if ast.requires_verification {
                ast.requires_verification_when_external = true;
            } else {
                for param in ast.parameters {
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
pub struct IndirectHybridFunctionDetectionVisitor;

impl FunctionVisitor for IndirectHybridFunctionDetectionVisitor {}
impl AstVisitor for IndirectHybridFunctionDetectionVisitor {
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
impl IndirectHybridFunctionDetectionVisitor {
    pub fn visitConstructorOrFunctionDefinition(&self, mut ast: ConstructorOrFunctionDefinition) {
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
pub struct NonInlineableCallDetector;

impl FunctionVisitor for NonInlineableCallDetector {}
impl AstVisitor for NonInlineableCallDetector {
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
impl NonInlineableCallDetector {
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        if !ast.is_cast() && is_instance(&**ast.func(), ASTType::LocationExprBase) {
            let ast1: AST = (*ast.func().target().unwrap()).into();
            assert!(
                !(ast1
                    .constructor_or_function_definition()
                    .unwrap()
                    .requires_verification
                    && ast1
                        .constructor_or_function_definition()
                        .unwrap()
                        .is_recursive),
                "Non-inlineable call to recursive private function {:?}",
                ast.func()
            )
        }
        self.visit_children(&ast.to_ast());
    }
}
