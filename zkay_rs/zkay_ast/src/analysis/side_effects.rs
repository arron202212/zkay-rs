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
    is_instance, is_instances, ASTBaseMutRef, ASTChildren, ASTFlatten, ASTType,
    AssignmentStatement, AssignmentStatementBaseProperty, BuiltinFunction, Expression,
    FunctionCallExpr, FunctionCallExprBaseProperty, IdentifierDeclaration, InstanceTarget, IntoAST,
    IntoExpression, IntoStatement, LocationExpr, LocationExprBaseProperty, Parameter,
    StateVariableDeclaration, Statement, TupleExpr, TupleOrLocationExpr, VariableDeclaration, AST,
};
use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;

use std::collections::BTreeSet;
pub fn has_side_effects(ast: &ASTFlatten) -> bool {
    SideEffectsDetector::new().visit(ast)
}

pub fn compute_modified_sets(ast: &ASTFlatten) {
    let mut v = DirectModificationDetector::new();
    v.visit(ast);

    let mut v = IndirectModificationDetector::new();
    v.iterate_until_fixed_point(ast);
}

pub fn check_for_undefined_behavior_due_to_eval_order(ast: &ASTFlatten) {
    EvalOrderUBChecker::new().visit(ast);
}

// class SideEffectsDetector(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct SideEffectsDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitor for SideEffectsDetector {
    type Return = bool;
    fn temper_result(&self) -> Self::Return {
        false
    }

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::FunctionCallExprBase
                | ASTType::ExpressionBase
                | ASTType::AssignmentStatementBase
                | ASTType::StatementBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(ast),
            ASTType::ExpressionBase => self.visitExpression(ast),
            ASTType::StatementBase => self.visitStatement(ast),

            _ => false,
        }
    }
}
impl SideEffectsDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) -> bool {
        if is_instance(
            &**ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::LocationExprBase,
        ) && !ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .is_cast()
            && ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .has_side_effects()
        {
            true
        } else {
            self.visitExpression(ast)
        }
    }
    pub fn visitAssignmentStatement(&self, _ast: &ASTFlatten) -> bool {
        true
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> bool {
        self.visitAST(ast)
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) -> bool {
        self.visitAST(ast)
    }

    pub fn visitAST(&self, ast: &ASTFlatten) -> bool {
        ast.children().iter().any(|c| self.visit(&c))
    }
}
// class DirectModificationDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct DirectModificationDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for DirectModificationDetector {}
impl AstVisitor for DirectModificationDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::LocationExprBase
                | ASTType::VariableDeclaration
                | ASTType::AssignmentStatementBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::LocationExprBase => self.visitLocationExpr(ast),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(ast),
            ASTType::VariableDeclaration => self.visitVariableDeclaration(ast),

            _ => {}
        }
    }
}
impl DirectModificationDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) {
        self.visitAST(ast);
        self.collect_modified_values(
            ast,
            &ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .lhs()
                .clone()
                .unwrap()
                .into(),
        );
    }

    pub fn collect_modified_values(&self, target: &ASTFlatten, expr: &ASTFlatten) {
        if is_instance(expr, ASTType::TupleExpr) {
            for elem in &expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_tuple_expr_ref()
                .unwrap()
                .elements
            {
                self.collect_modified_values(target, &elem.clone().into());
            }
        } else {
            let mod_value = InstanceTarget::new(vec![Some((expr.clone()).into())]);
            assert!(!target
                .ast_base_ref()
                .unwrap().borrow()
                .modified_values
                .contains(&mod_value),"Undefined behavior due multiple different assignments to the same target in tuple assignment ,{:?}", expr);

            target
                .ast_base_ref()
                .unwrap()
                .borrow_mut()
                .modified_values
                .insert(mod_value);
        }
    }
    pub fn visitLocationExpr(&self, ast: &ASTFlatten) {
        let ast2: LocationExpr = ast.try_as_location_expr_ref().unwrap().borrow().clone();
        self.visitAST(ast);
        let ast1 = ast
            .try_as_location_expr_ref()
            .unwrap()
            .borrow()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        if TupleOrLocationExpr::LocationExpr(
            ast.try_as_location_expr_ref().unwrap().borrow().clone(),
        )
        .is_rvalue()
            && is_instances(
                &ast1,
                vec![
                    ASTType::VariableDeclaration,
                    ASTType::StateVariableDeclaration,
                    ASTType::Parameter,
                ],
            )
        {
            ast.ast_base_ref()
                .unwrap()
                .borrow_mut()
                .read_values
                .insert(InstanceTarget::new(vec![Some(ast.clone())]));
        }
    }
    pub fn visitVariableDeclaration(&self, ast: &ASTFlatten) {
        ast.try_as_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .ast_base
            .borrow_mut()
            .modified_values
            .insert(InstanceTarget::new(vec![Some(ast.clone())]));
    }

    pub fn visitAST(&self, ast: &ASTFlatten) {
        let mut modified_values = BTreeSet::new();
        let mut read_values = BTreeSet::new();
        for child in ast.children() {
            self.visit(&child);
            modified_values = modified_values
                .union(&child.ast_base_ref().unwrap().borrow().modified_values)
                .cloned()
                .collect();
            read_values = read_values
                .union(&child.ast_base_ref().unwrap().borrow().read_values)
                .cloned()
                .collect();
        }
        ast.ast_base_ref().unwrap().borrow_mut().modified_values = modified_values;
        ast.ast_base_ref().unwrap().borrow_mut().read_values = read_values;
    }
}
// class IndirectModificationDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectModificationDetector {
    pub ast_visitor_base: AstVisitorBase,
    pub fixed_point_reached: RcCell<bool>,
}
impl FunctionVisitor for IndirectModificationDetector {}
impl AstVisitor for IndirectModificationDetector {
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
impl IndirectModificationDetector {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.fixed_point_reached = true
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            fixed_point_reached: RcCell::new(true),
        }
    }
    pub fn iterate_until_fixed_point(&self, ast: &ASTFlatten) {
        loop {
            self.visit(ast);
            if *self.fixed_point_reached.borrow() {
                break;
            } else {
                *self.fixed_point_reached.borrow_mut() = true;
            }
        }
    }

    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        self.visitAST(ast);
        if is_instance(
            &**ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::LocationExprBase,
        ) {
            //for now no reference types -> only state could have been modified
            let mut fdef = ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap();
            let rlen = ast.ast_base_ref().unwrap().borrow().read_values.len();
            ast.ast_base_ref().unwrap().borrow_mut().read_values = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .read_values
                .union(
                    &fdef
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .read_values
                        .iter()
                        .filter_map(|v| {
                            if v.target().is_some()
                                && is_instance(
                                    &v.target().unwrap(),
                                    ASTType::StateVariableDeclaration,
                                )
                            {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .cloned()
                        .collect(),
                )
                .cloned()
                .collect();
            *self.fixed_point_reached.borrow_mut() &=
                rlen == ast.ast_base_ref().unwrap().borrow().read_values.len();

            //update modified values if any
            let mlen = ast.ast_base_ref().unwrap().borrow().modified_values.len();
            for v in &fdef.ast_base_ref().unwrap().borrow().modified_values {
                if is_instance(&v.target().unwrap(), ASTType::StateVariableDeclaration) {
                    ast.ast_base_ref()
                        .unwrap()
                        .borrow_mut()
                        .modified_values
                        .insert(v.clone());
                }
            }
            *self.fixed_point_reached.borrow_mut() &=
                mlen == ast.ast_base_ref().unwrap().borrow().modified_values.len();
        }
    }
    pub fn visitAST(&self, ast: &ASTFlatten) {
        let mlen = ast.ast_base_ref().unwrap().borrow().modified_values.len();
        let rlen = ast.ast_base_ref().unwrap().borrow().read_values.len();
        for child in ast.children().iter_mut() {
            self.visit(child);
            ast.ast_base_ref().unwrap().borrow_mut().modified_values = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .modified_values
                .union(&child.ast_base_ref().unwrap().borrow().modified_values)
                .cloned()
                .collect();
            ast.ast_base_ref().unwrap().borrow_mut().read_values = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .read_values
                .union(&child.ast_base_ref().unwrap().borrow().read_values)
                .cloned()
                .collect();
        }
        *self.fixed_point_reached.borrow_mut() &=
            mlen == ast.ast_base_ref().unwrap().borrow().modified_values.len();
        *self.fixed_point_reached.borrow_mut() &=
            rlen == ast.ast_base_ref().unwrap().borrow().read_values.len();
    }
}
// class EvalOrderUBChecker(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct EvalOrderUBChecker {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitor for EvalOrderUBChecker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::FunctionCallExprBase
                | ASTType::ExpressionBase
                | ASTType::AssignmentStatementBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(ast),
            ASTType::ExpressionBase => self.visitExpression(ast),

            _ => {}
        }
    }
}
impl EvalOrderUBChecker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    // @staticmethod
    pub fn visit_child_expressions(parent: &ASTFlatten, exprs: &Vec<ASTFlatten>) {
        if exprs.len() > 1 {
            let mut modset: BTreeSet<_> = exprs[0]
                .ast_base_ref()
                .unwrap()
                .borrow()
                .modified_values
                .clone();
            for arg in &exprs[1..] {
                let modified_values = arg.ast_base_ref().unwrap().borrow().modified_values.clone();
                let diffset: BTreeSet<_> = modset.intersection(&modified_values).collect();

                assert!(
                    diffset.is_empty(),
                    r#"Undefined behavior due to potential side effect on the same value(s) \"{}\" in multiple expression children.\n"
                                        "Solidity does not guarantee an evaluation order for non-shortcircuit expressions.\n"
                                        "Since zkay requires local simulation for transaction transformation, all semantics must be well-defined. {:?}"#,
                    format!(
                        "{{{}}}",
                        (diffset.into_iter().map(|d| (*d).to_string()))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    parent
                );
                //????  diffset is_empty
                // modset = modset.union(diffset).collect();
            }

            for arg in &*exprs {
                let modset: BTreeSet<_> =
                    arg.ast_base_ref().unwrap().borrow().modified_values.clone();
                let other_args: BTreeSet<_> = exprs
                    .iter()
                    .filter_map(|e| if e != arg { Some(e) } else { None })
                    .collect();
                for arg2 in &other_args {
                    let read_values = arg2.ast_base_ref().unwrap().borrow().read_values.clone();
                    let diffset: BTreeSet<_> = modset.intersection(&read_values).collect();
                    if !diffset.is_empty() {
                        let setstr = format!(
                            r#"{{{}}}"#,
                            diffset
                                .iter()
                                .map(|it| format!(
                                    "{:?}{}",
                                    it.target_key[0].as_ref().unwrap(),
                                    it.target_key[1]
                                        .clone()
                                        .map_or(String::new(), |member| format!(".{:?}", member))
                                ))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                        assert!(
                            false,
                            r#"Undefined behavior due to read of value(s) \"{setstr}\" which might be modified in this subexpression.\n"
                            "Solidity does not guarantee an evaluation order for non-shortcircuit expressions.\n"
                            "Since zkay requires local simulation for transaction transformation, all semantics must be well-defined. {:?}"#,
                            arg
                        );
                    }
                }
            }
        }
    }
    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        if is_instance(
            &**ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::BuiltinFunction,
        ) {
            if ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .borrow()
                .try_as_builtin_function_ref()
                .unwrap()
                .has_shortcircuiting()
            {
                return;
            }
        }
        Self::visit_child_expressions(
            ast,
            &ast.try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .args()
                .into_iter()
                .map(|arg| arg.clone().into())
                .collect(),
        );
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) {
        Self::visit_child_expressions(ast, &ast.children());
    }

    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) {
        Self::visit_child_expressions(ast, &ast.children());
    }
}
