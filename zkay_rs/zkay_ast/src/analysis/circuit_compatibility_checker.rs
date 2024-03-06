#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use std::collections::BTreeSet;
use zkay_config::config::CFG;
// use type_check::type_exceptions::TypeException
use crate::ast::{
    is_instance, is_instances, ASTType, AssignmentStatement, BooleanLiteralType, BuiltinFunction,
    ConstructorOrFunctionDefinition, Expression, ExpressionBaseMutRef, ExpressionBaseProperty,
    FunctionCallExpr, FunctionCallExprBaseProperty, FunctionTypeName, IfStatement, IndexExpr,
    IntoAST, IntoExpression, LocationExpr, NumberLiteralType, PrimitiveCastExpr, ReclassifyExpr,
    ReturnStatement, Statement, StatementList, AST,
};
use crate::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};

pub fn check_circuit_compliance(ast: AST) {
    // """
    // determines for every function whether it can be used inside a circuit
    // """
    let v = DirectCanBePrivateDetector;
    v.visit(ast.clone());

    let v = IndirectCanBePrivateDetector;
    v.visit(ast.clone());

    let v = CircuitComplianceChecker::new();
    v.visit(ast.clone());

    check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(ast)
}
// class DirectCanBePrivateDetector(FunctionVisitor)
pub struct DirectCanBePrivateDetector;

impl FunctionVisitor for DirectCanBePrivateDetector {}
impl AstVisitor for DirectCanBePrivateDetector {
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
impl DirectCanBePrivateDetector {
    pub fn visitFunctionCallExpr(&mut self, mut ast: FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            if !ast.func().is_private() {
                let mut can_be_private = ast.func().can_be_private();
                if ast.func().is_eq() || ast.func().is_ite() {
                    can_be_private &= ast.args()[1]
                        .annotated_type()
                        .unwrap()
                        .type_name
                        .can_be_private();
                }
                ast.expression_base_mut_ref()
                    .statement
                    .as_mut()
                    .unwrap()
                    .function()
                    .unwrap()
                    .can_be_private &= can_be_private;
                //TODO to relax this for public expressions,
                // public identifiers must use SSA remapped values (since the function is inlined)
            }
        }
        for arg in ast.args() {
            self.visit(arg.to_ast());
        }
    }

    pub fn visitLocationExpr(&mut self, mut ast: LocationExpr) {
        let t = &ast.annotated_type().unwrap().type_name;
        ast.expression_base_mut_ref()
            .statement
            .as_mut()
            .unwrap()
            .function()
            .unwrap()
            .can_be_private &= t.can_be_private();
        self.visit_children(&ast.to_ast());
    }

    pub fn visitReclassifyExpr(&mut self, ast: ReclassifyExpr) {
        self.visit(ast.expr().unwrap().to_ast());
    }

    pub fn visitAssignmentStatement(&mut self, ast: AssignmentStatement) {
        self.visit_children(&ast.to_ast());
    }

    pub fn visitVariableDeclarationStatement(&mut self, ast: AssignmentStatement) {
        self.visit_children(&ast.to_ast());
    }

    pub fn visitReturnStatement(&mut self, ast: ReturnStatement) {
        self.visit_children(&ast.to_ast());
    }

    pub fn visitIfStatement(&mut self, ast: IfStatement) {
        self.visit_children(&ast.to_ast());
    }

    pub fn visitStatementList(&mut self, ast: StatementList) {
        self.visit_children(&ast.to_ast());
    }

    pub fn visitStatement(&mut self, ast: &mut Statement) {
        //All other statement types are not supported inside circuit (for now)
        ast.function().unwrap().can_be_private = false;
    }
}
// class IndirectCanBePrivateDetector(FunctionVisitor)
pub struct IndirectCanBePrivateDetector;

impl FunctionVisitor for IndirectCanBePrivateDetector {}
impl AstVisitor for IndirectCanBePrivateDetector {
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
impl IndirectCanBePrivateDetector {
    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        mut ast: ConstructorOrFunctionDefinition,
    ) {
        if ast.can_be_private {
            for fct in ast.called_functions {
                if !fct.can_be_private {
                    ast.can_be_private = false;
                    return;
                }
            }
        }
    }
}
// class CircuitComplianceChecker(FunctionVisitor)
pub struct CircuitComplianceChecker {
    pub priv_setter: PrivateSetter,
    pub inside_privif_stmt: bool,
}

impl FunctionVisitor for CircuitComplianceChecker {}
impl AstVisitor for CircuitComplianceChecker {
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
impl CircuitComplianceChecker {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.priv_setter = PrivateSetter()
    //     self.inside_privif_stmt = false
    pub fn new() -> Self {
        Self {
            priv_setter: PrivateSetter::new(),
            inside_privif_stmt: false,
        }
    }
    // @staticmethod
    pub fn should_evaluate_public_expr_in_circuit(expr: Expression) -> bool {
        // assert!(expr.annotated_type());
        if CFG
            .lock()
            .unwrap()
            .user_config
            .opt_eval_constexpr_in_circuit()
        {
            if is_instances(
                &*expr.annotated_type().unwrap().type_name,
                vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
            ) {
                //Expressions for which the value is known at compile time -> embed constant expression value into the circuit
                return true;
            }

            if is_instance(&expr, ASTType::PrimitiveCastExpr)
                && is_instances(
                    &*expr.expr().unwrap().annotated_type().unwrap().type_name,
                    vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
                )
            {
                //Constant casts should also be evaluated inside the circuit
                return true;
            }
        }

        // try
        check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(
            expr.to_ast(),
        );
        // except TypeException
        //     //Cannot evaluate inside circuit -> never do it
        //     return false

        //Could evaluate in circuit, use analysis to determine whether this would be better performance wise
        //(If this avoids unnecessary encryption operations it may be cheaper)
        false
    }

    pub fn visitIndexExpr(&mut self, ast: IndexExpr) {
        if ast
            .location_expr_base
            .tuple_or_location_expr_base
            .expression_base
            .evaluate_privately
        {
            assert!(ast.key.annotated_type().unwrap().is_public());
            self.priv_setter.set_evaluation((*ast.key).to_ast(), false);
        }
        self.visit_children(&ast.to_ast());
    }

    pub fn visitReclassifyExpr(&mut self, mut ast: ReclassifyExpr) {
        assert!(!self.inside_privif_stmt
            || ast.expression_base_mut_ref().statement.as_mut().unwrap().before_analysis().unwrap().same_partition(
                &ast.privacy().unwrap().privacy_annotation_label().unwrap().into(),
                &Expression::me_expr(None).to_ast(),
            ),"Revealing information to other parties is not allowed inside private if statements {:?}", ast);
        if ast.expr().unwrap().annotated_type().unwrap().is_public() {
            let eval_in_public = false;
            // try
            self.priv_setter.set_evaluation(ast.to_ast(), true);
            // except TypeException
            //     eval_in_public = true
            if eval_in_public || !Self::should_evaluate_public_expr_in_circuit(ast.expr().unwrap())
            {
                self.priv_setter
                    .set_evaluation(ast.expr().unwrap().to_ast(), false);
            }
        } else {
            self.priv_setter.set_evaluation(ast.to_ast(), true);
        }
        self.visit(ast.expr().unwrap().to_ast());
    }

    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) && ast.func().is_private() {
            self.priv_setter.set_evaluation(ast.to_ast(), true);
        } else if ast.is_cast() && ast.annotated_type().unwrap().is_private() {
            self.priv_setter.set_evaluation(ast.to_ast(), true);
        }
        self.visit_children(&ast.to_ast());
    }

    pub fn visitPrimitiveCastExpr(&mut self, ast: PrimitiveCastExpr) {
        if ast.expr.annotated_type().unwrap().is_private() {
            self.priv_setter.set_evaluation(ast.to_ast(), true);
        }
        self.visit_children(&ast.to_ast());
    }

    pub fn visitIfStatement(&mut self, ast: IfStatement) {
        let old_in_privif_stmt = self.inside_privif_stmt.clone();
        if ast.condition.annotated_type().unwrap().is_private() {
            let mut mod_vals = ast
                .then_branch
                .statement_list_base
                .statement_base
                .ast_base
                .modified_values
                .clone();
            if ast.else_branch.is_some() {
                mod_vals = mod_vals
                    .union(
                        &ast.else_branch
                            .as_ref()
                            .unwrap()
                            .statement_list_base
                            .statement_base
                            .ast_base
                            .modified_values,
                    )
                    .cloned()
                    .collect();
            }
            for val in mod_vals {
                if !val
                    .target()
                    .unwrap()
                    .annotated_type()
                    .unwrap()
                    .zkay_type()
                    .type_name
                    .is_primitive_type()
                {
                    assert!(false,"Writes to non-primitive type variables are not allowed inside private if statements {:?}", ast)
                }
                if val.in_scope_at(ast.to_ast())
                    && !ast
                        .statement_base
                        .before_analysis
                        .as_ref()
                        .unwrap()
                        .same_partition(
                            &val.privacy().unwrap(),
                            &Expression::me_expr(None).to_ast(),
                        )
                {
                    assert!(false,"If statement with private condition must not contain side effects to variables with owner != me ,{:?}", ast)
                }
            }
            self.inside_privif_stmt = true;
            self.priv_setter.set_evaluation(ast.to_ast(), true);
        }
        self.visit_children(&ast.to_ast());
        self.inside_privif_stmt = old_in_privif_stmt;
    }
}
// class PrivateSetter(FunctionVisitor)
pub struct PrivateSetter {
    evaluate_privately: Option<bool>,
}

impl FunctionVisitor for PrivateSetter {}
impl AstVisitor for PrivateSetter {
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
impl PrivateSetter {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.evaluate_privately = None
    pub fn new() -> Self {
        Self {
            evaluate_privately: None,
        }
    }
    pub fn set_evaluation(&mut self, ast: AST, evaluate_privately: bool) {
        self.evaluate_privately = Some(evaluate_privately);
        self.visit(ast);
        self.evaluate_privately = None;
    }

    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
        if self.evaluate_privately.is_some()
            && is_instance(&**ast.func(), ASTType::LocationExprBase)
            && !ast.is_cast()
            && (*ast.func().target().unwrap())
                .constructor_or_function_definition()
                .unwrap()
                .has_side_effects()
        {
            assert!(
                false,
                "Expressions with side effects are not allowed inside private expressions {:?}",
                ast
            )
        }
        self.visitExpression(ast.to_expr());
    }

    pub fn visitExpression(&mut self, ast: Expression) {
        assert!(self.evaluate_privately.is_some());
        ast.set_evaluate_privately(self.evaluate_privately.unwrap());
        self.visit_children(&ast.to_ast());
    }
}
pub fn check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(ast: AST) {
    NonstaticOrIncompatibilityDetector.visit(ast);
}

// class NonstaticOrIncompatibilityDetector(FunctionVisitor)
pub struct NonstaticOrIncompatibilityDetector;

impl FunctionVisitor for NonstaticOrIncompatibilityDetector {}
impl AstVisitor for NonstaticOrIncompatibilityDetector {
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
impl NonstaticOrIncompatibilityDetector {
    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
        let mut can_be_private = true;
        let mut has_nonstatic_call = false;
        if ast.evaluate_privately() && !ast.is_cast() {
            if is_instance(&**ast.func(), ASTType::LocationExprBase) {
                assert!(ast.func().target().is_some());
                assert!(is_instance(
                    &*ast
                        .func()
                        .target()
                        .unwrap()
                        .annotated_type()
                        .unwrap()
                        .type_name,
                    ASTType::FunctionTypeName
                ));
                has_nonstatic_call |= !(*ast.func().target().unwrap())
                    .constructor_or_function_definition()
                    .unwrap()
                    .has_static_body;
                can_be_private &= (ast.func().target().unwrap())
                    .constructor_or_function_definition()
                    .unwrap()
                    .can_be_private;
            } else if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
                can_be_private &= ast.func().can_be_private()
                    || ast.annotated_type().unwrap().type_name.is_literal();
                if ast.func().is_eq() || ast.func().is_ite() {
                    can_be_private &= ast.args()[1]
                        .annotated_type()
                        .unwrap()
                        .type_name
                        .can_be_private();
                }
            }
        }
        if has_nonstatic_call {
            assert!(
                false,
                "Function calls to non static functions are not allowed inside private expressions ,{:?}",
                ast
            );
        }
        if !can_be_private {
            assert!(false,
                "Calls to functions with operations which cannot be expressed as a circuit are not allowed inside private expressions {:?}", ast);
        }
        self.visit_children(&ast.to_ast());
    }
}
