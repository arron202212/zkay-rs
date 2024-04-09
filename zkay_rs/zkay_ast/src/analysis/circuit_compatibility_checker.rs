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
use crate::analysis::partition_state::PartitionState;
use crate::ast::{
    is_instance, is_instances, ASTFlatten, ASTType, AssignmentStatement, BooleanLiteralType,
    BuiltinFunction, ConstructorOrFunctionDefinition, Expression, ExpressionBaseMutRef,
    ExpressionBaseProperty, FunctionCallExpr, FunctionCallExprBaseProperty, FunctionTypeName,
    IfStatement, IndexExpr, IntoAST, IntoExpression, LocationExpr, LocationExprBaseProperty,
    NumberLiteralType, PrimitiveCastExpr, ReclassifyExpr, ReclassifyExprBaseProperty,
    ReturnStatement, Statement, StatementBaseProperty, StatementList, VariableDeclarationStatement,
    AST,
};
use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_circuit_compliance(ast: &mut AST) {
    // """
    // determines for every function whether it can be used inside a circuit
    // """
    let mut v = DirectCanBePrivateDetector::new();
    v.visit(ast);

    let mut v = IndirectCanBePrivateDetector::new();
    v.visit(ast);

    let mut v = CircuitComplianceChecker::new();
    v.visit(ast);

    check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(ast)
}
// class DirectCanBePrivateDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct DirectCanBePrivateDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for DirectCanBePrivateDetector {}
impl AstVisitor for DirectCanBePrivateDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::FunctionCallExprBase
                | ASTType::LocationExprBase
                | ASTType::ReclassifyExpr
                | ASTType::AssignmentStatementBase
                | ASTType::VariableDeclarationStatement
                | ASTType::ReturnStatement
                | ASTType::IfStatement
                | ASTType::StatementListBase
                | ASTType::StatementBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),
            ASTType::LocationExprBase => self.visitLocationExpr(ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::StatementListBase => self.visitStatementList(ast),
            ASTType::StatementBase => self.visitStatement(ast),

            _ => {}
        }
    }
}
impl DirectCanBePrivateDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            if !ast.func().try_as_builtin_function_ref().unwrap().is_private {
                let mut can_be_private = ast
                    .func()
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .can_be_private();
                if ast.func().try_as_builtin_function_ref().unwrap().is_eq()
                    || ast.func().try_as_builtin_function_ref().unwrap().is_ite()
                {
                    can_be_private &= ast.args()[1]
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .can_be_private();
                }
                ast.expression_base_mut_ref()
                    .statement
                    .as_mut()
                    .unwrap()
                    .statement_base_mut_ref()
                    .unwrap()
                    .function
                    .as_mut()
                    .unwrap()
                    .can_be_private &= can_be_private;
                //TODO to relax this for public expressions,
                // public identifiers must use SSA remapped values (since the function is inlined)
            }
        }
        for arg in ast.args() {
            self.visit(arg.into());
        }
    }

    pub fn visitLocationExpr(&self, ast: &ASTFlatten) {
        let t = &ast.annotated_type().as_ref().unwrap().type_name;
        ast.expression_base_mut_ref()
            .statement
            .as_mut()
            .unwrap()
            .statement_base_mut_ref()
            .unwrap()
            .function
            .as_mut()
            .unwrap()
            .can_be_private &= t.as_ref().unwrap().can_be_private();
        self.visit_children(ast);
    }

    pub fn visitReclassifyExpr(&self, ast: &ASTFlatten) {
        self.visit(&ast.expr().into());
    }

    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) {
        self.visit_children(ast);
    }

    pub fn visitVariableDeclarationStatement(&self, ast: &ASTFlatten) {
        self.visit_children(ast);
    }

    pub fn visitReturnStatement(&self, ast: &ASTFlatten) {
        self.visit_children(ast);
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) {
        self.visit_children(ast);
    }

    pub fn visitStatementList(&self, ast: &ASTFlatten) {
        self.visit_children(ast);
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) {
        //All other statement types are not supported inside circuit (for now)
        ast.statement_base_mut_ref()
            .unwrap()
            .function
            .as_mut()
            .unwrap()
            .can_be_private = false;
    }
}
// class IndirectCanBePrivateDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectCanBePrivateDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for IndirectCanBePrivateDetector {}
impl AstVisitor for IndirectCanBePrivateDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::ConstructorOrFunctionDefinition == name
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            _ => {}
        }
    }
}
impl IndirectCanBePrivateDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: &ASTFlatten) {
        if ast.can_be_private {
            for fct in &ast.called_functions {
                if !fct.can_be_private {
                    ast.can_be_private = false;
                    return;
                }
            }
        }
    }
}
// class CircuitComplianceChecker(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct CircuitComplianceChecker {
    pub ast_visitor_base: AstVisitorBase,
    pub priv_setter: PrivateSetter,
    pub inside_privif_stmt: bool,
}

impl FunctionVisitor for CircuitComplianceChecker {}
impl AstVisitor for CircuitComplianceChecker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::IndexExpr
                | ASTType::ReclassifyExpr
                | ASTType::FunctionCallExprBase
                | ASTType::PrimitiveCastExpr
                | ASTType::IfStatement
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            _ => {}
        }
    }
}
impl CircuitComplianceChecker {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.priv_setter = PrivateSetter()
    //     self.inside_privif_stmt = false
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
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
                &**expr
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .unwrap(),
                vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
            ) {
                //Expressions for which the value is known at compile time -> embed constant expression value into the circuit
                return true;
            }

            if is_instance(&expr, ASTType::PrimitiveCastExpr)
                && is_instances(
                    &**expr
                        .try_as_primitive_cast_expr_ref()
                        .unwrap()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap(),
                    vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
                )
            {
                //Constant casts should also be evaluated inside the circuit
                return true;
            }
        }

        // try
        check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(
            &mut expr.to_ast(),
        );
        // except TypeException
        //     //Cannot evaluate inside circuit -> never do it
        //     return false

        //Could evaluate in circuit, use analysis to determine whether this would be better performance wise
        //(If this avoids unnecessary encryption operations it may be cheaper)
        false
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) {
        if ast
            .location_expr_base
            .tuple_or_location_expr_base
            .expression_base
            .evaluate_privately
        {
            assert!(ast.key.annotated_type().as_ref().unwrap().is_public());
            self.priv_setter
                .set_evaluation(&mut (*ast.key).to_ast(), false);
        }
        self.visit_children(ast);
    }

    pub fn visitReclassifyExpr(&self, ast: &ASTFlatten) {
        assert!(!self.inside_privif_stmt
            || ast.statement().as_ref().unwrap().statement_base_ref().unwrap().before_analysis.as_ref().unwrap().same_partition(
                &ast.privacy().privacy_annotation_label().unwrap(),
                &Expression::me_expr(None).to_ast(),
            ),"Revealing information to other parties is not allowed inside private if statements {:?}", ast);
        if ast.expr().annotated_type().as_ref().unwrap().is_public() {
            let eval_in_public = false;
            // try
            self.priv_setter.set_evaluation(&mut ast.to_ast(), true);
            // except TypeException
            //     eval_in_public = true
            if eval_in_public || !Self::should_evaluate_public_expr_in_circuit(*ast.expr().clone())
            {
                self.priv_setter
                    .set_evaluation(&mut ast.expr().to_ast(), false);
            }
        } else {
            self.priv_setter.set_evaluation(&mut ast.to_ast(), true);
        }
        self.visit(ast.expr().into());
    }

    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction)
            && ast.func().try_as_builtin_function_ref().unwrap().is_private
        {
            self.priv_setter.set_evaluation(&mut ast.to_ast(), true);
        } else if ast.is_cast() && ast.annotated_type().as_ref().unwrap().is_private() {
            self.priv_setter.set_evaluation(&mut ast.to_ast(), true);
        }
        self.visit_children(ast);
    }

    pub fn visitPrimitiveCastExpr(&self, ast: &ASTFlatten) {
        if ast.expr.annotated_type().as_ref().unwrap().is_private() {
            self.priv_setter.set_evaluation(&mut ast.to_ast(), true);
        }
        self.visit_children(ast);
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) {
        let old_in_privif_stmt = self.inside_privif_stmt.clone();
        if ast
            .condition
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_private()
        {
            let mut mod_vals = ast
                .then_branch
                .borrow()
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
                            .borrow()
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
                    .try_as_expression_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .zkay_type()
                    .type_name
                    .unwrap()
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
            self.priv_setter.set_evaluation(ast, true);
        }
        self.visit_children(ast);
        self.inside_privif_stmt = old_in_privif_stmt;
    }
}
// class PrivateSetter(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct PrivateSetter {
    pub ast_visitor_base: AstVisitorBase,
    pub evaluate_privately: Option<bool>,
}

impl FunctionVisitor for PrivateSetter {}
impl AstVisitor for PrivateSetter {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::FunctionCallExprBase | ASTType::ExpressionBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_function_call_expr_mut()
                    .unwrap(),
            ),
            ASTType::ExpressionBase => self.visitExpression(ast.try_as_expression_mut().unwrap()),

            _ => {}
        }
    }
}
impl PrivateSetter {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.evaluate_privately = None
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            evaluate_privately: None,
        }
    }
    pub fn set_evaluation(&self, ast: &ASTFlatten, evaluate_privately: bool) {
        self.evaluate_privately = Some(evaluate_privately);
        self.visit(ast);
        self.evaluate_privately = None;
    }

    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        if self.evaluate_privately.is_some()
            && is_instance(&**ast.func(), ASTType::LocationExprBase)
            && !ast.is_cast()
            && (*ast
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap())
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .has_side_effects()
        {
            assert!(
                false,
                "Expressions with side effects are not allowed inside private expressions {:?}",
                ast
            )
        }
        self.visitExpression(ast);
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) {
        assert!(self.evaluate_privately.is_some());
        ast.expression_base_mut_ref().evaluate_privately = self.evaluate_privately.unwrap();
        self.visit_children(&ast);
    }
}
pub fn check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(
    ast: &mut AST,
) {
    NonstaticOrIncompatibilityDetector::new().visit(ast);
}

// class NonstaticOrIncompatibilityDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct NonstaticOrIncompatibilityDetector {
    pub ast_visitor_base: AstVisitorBase,
}

impl FunctionVisitor for NonstaticOrIncompatibilityDetector {}
impl AstVisitor for NonstaticOrIncompatibilityDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),

            _ => {}
        }
    }
}
impl NonstaticOrIncompatibilityDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        let mut can_be_private = true;
        let mut has_nonstatic_call = false;
        if ast.evaluate_privately() && !ast.is_cast() {
            if is_instance(&**ast.func(), ASTType::LocationExprBase) {
                assert!(ast
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .is_some());
                assert!(is_instance(
                    &**ast
                        .func()
                        .borrow()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .as_ref()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap(),
                    ASTType::FunctionTypeName
                ));
                has_nonstatic_call |= !(*ast
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap())
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .has_static_body;
                can_be_private &= (ast
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap())
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .can_be_private;
            } else if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
                can_be_private &= ast
                    .func()
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .can_be_private()
                    || ast
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .is_literal();
                if ast.func().try_as_builtin_function_ref().unwrap().is_eq()
                    || ast.func().try_as_builtin_function_ref().unwrap().is_ite()
                {
                    can_be_private &= ast.args()[1]
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .can_be_private();
                }
            }
        }
        assert!(
                !has_nonstatic_call,
                "Function calls to non static functions are not allowed inside private expressions ,{:?}",
                ast
            );
        assert!(can_be_private,
                "Calls to functions with operations which cannot be expressed as a circuit are not allowed inside private expressions {:?}", ast);
        self.visit_children(ast);
    }
}
