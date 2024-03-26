#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// use type_check::type_exceptions::TypeException
use crate::ast::{
    is_instance, is_instances, ASTBaseMutRef, ASTChildren, ASTType, AssignmentStatement,
    AssignmentStatementBaseProperty, BuiltinFunction, Expression, FunctionCallExpr,
    FunctionCallExprBaseProperty, IdentifierDeclaration, InstanceTarget, IntoAST, IntoExpression,
    IntoStatement, LocationExpr, LocationExprBaseProperty, Parameter, StateVariableDeclaration,
    Statement, TupleExpr, TupleOrLocationExpr, VariableDeclaration, AST,
};
use crate::visitor::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef, AstVisitorMut},
};
use zkay_derive::ASTVisitorBaseRefImpl;

use std::collections::BTreeSet;
pub fn has_side_effects(ast: AST) -> bool {
    SideEffectsDetector::new().visit(&ast)
}

pub fn compute_modified_sets(ast: &mut AST) {
    let mut v = DirectModificationDetector::new();
    v.visit(ast);

    let mut v = IndirectModificationDetector::new();
    v.iterate_until_fixed_point(ast);
}

pub fn check_for_undefined_behavior_due_to_eval_order(ast: &mut AST) {
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
        &ASTType::FunctionCallExprBase == name
            || &ASTType::ExpressionBase == name
            || &ASTType::AssignmentStatementBase == name
            || &ASTType::StatementBase == name
    }
    fn get_attr(&self, name: &ASTType, ast: &AST) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap(),
            ),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(
                ast.try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_assignment_statement_ref()
                    .unwrap(),
            ),
            ASTType::ExpressionBase => self.visitExpression(ast.try_as_expression_ref().unwrap()),
            ASTType::StatementBase => self.visitStatement(ast.try_as_statement_ref().unwrap()),

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
    pub fn visitFunctionCallExpr(&self, ast: &FunctionCallExpr) -> bool {
        if is_instance(&**ast.func(), ASTType::LocationExprBase)
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
            true
        } else {
            self.visitExpression(&ast.to_expr())
        }
    }
    pub fn visitAssignmentStatement(&self, _ast: &AssignmentStatement) -> bool {
        true
    }

    pub fn visitExpression(&self, ast: &Expression) -> bool {
        self.visitAST(&ast.to_ast())
    }

    pub fn visitStatement(&self, ast: &Statement) -> bool {
        self.visitAST(&ast.to_ast())
    }

    pub fn visitAST(&self, ast: &AST) -> bool {
        let mut ast = ast.clone();
        ast.children().iter().any(|c| self.visit(&c))
    }
}
// class DirectModificationDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct DirectModificationDetector {
    pub ast_visitor_base: AstVisitorBase,
}
impl FunctionVisitor for DirectModificationDetector {}
impl AstVisitorMut for DirectModificationDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::LocationExprBase == name
            || &ASTType::VariableDeclaration == name
            || &ASTType::AssignmentStatementBase == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::LocationExprBase => self.visitLocationExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap(),
            ),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap()
                    .try_as_assignment_statement_mut()
                    .unwrap(),
            ),
            ASTType::VariableDeclaration => self.visitVariableDeclaration(
                ast.try_as_identifier_declaration_mut()
                    .unwrap()
                    .try_as_variable_declaration_mut()
                    .unwrap(),
            ),

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
    pub fn visitAssignmentStatement(&mut self, ast: &mut AssignmentStatement) {
        self.visitAST(&mut ast.to_ast());
        self.collect_modified_values(&mut ast.to_ast(), *ast.lhs().clone().unwrap());
    }

    pub fn collect_modified_values(&self, target: &mut AST, expr: AST) {
        if is_instance(&expr, ASTType::TupleExpr) {
            for elem in &expr
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_tuple_expr_ref()
                .unwrap()
                .elements
            {
                self.collect_modified_values(target, elem.to_ast());
            }
        } else {
            let mod_value = InstanceTarget::new(vec![Some(Box::new(expr.to_ast()))]);
            if target
                .ast_base_ref()
                .unwrap()
                .modified_values
                .contains(&mod_value)
            {
                assert!(false,"Undefined behavior due multiple different assignments to the same target in tuple assignment ,{:?}", expr);
            }
            target
                .ast_base_mut_ref()
                .unwrap()
                .modified_values
                .insert(mod_value);
        }
    }
    pub fn visitLocationExpr(&mut self, ast: &mut LocationExpr) {
        let ast2: LocationExpr = ast.clone();
        self.visitAST(&mut (*ast).to_ast());
        let ast1 = ast.target().as_ref().unwrap();
        if TupleOrLocationExpr::LocationExpr(ast.clone()).is_rvalue()
            && is_instances(
                &**ast1,
                vec![
                    ASTType::VariableDeclaration,
                    ASTType::StateVariableDeclaration,
                    ASTType::Parameter,
                ],
            )
        {
            ast.ast_base_mut_ref()
                .read_values
                .insert(InstanceTarget::new(vec![Some(Box::new(ast2.to_ast()))]));
        }
    }
    pub fn visitVariableDeclaration(&self, ast: &mut VariableDeclaration) {
        ast.identifier_declaration_base
            .ast_base
            .modified_values
            .insert(InstanceTarget::new(vec![Some(Box::new(ast.to_ast()))]));
    }

    pub fn visitAST(&mut self, ast: &mut AST) {
        let mut modified_values = BTreeSet::new();
        let mut read_values = BTreeSet::new();
        for child in ast.children().iter_mut() {
            self.visit(child);
            modified_values = modified_values
                .union(&child.ast_base_mut_ref().unwrap().modified_values)
                .cloned()
                .collect();
            read_values = read_values
                .union(&child.ast_base_ref().unwrap().read_values)
                .cloned()
                .collect();
        }
        ast.ast_base_mut_ref().unwrap().modified_values = modified_values;
        ast.ast_base_mut_ref().unwrap().read_values = read_values;
    }
}
// class IndirectModificationDetector(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct IndirectModificationDetector {
    pub ast_visitor_base: AstVisitorBase,
    pub fixed_point_reached: bool,
}
impl FunctionVisitor for IndirectModificationDetector {}
impl AstVisitorMut for IndirectModificationDetector {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_function_call_expr_mut()
                    .unwrap(),
            ),

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
            fixed_point_reached: true,
        }
    }
    pub fn iterate_until_fixed_point(&mut self, ast: &mut AST) {
        loop {
            self.visit(ast);
            if self.fixed_point_reached {
                break;
            } else {
                self.fixed_point_reached = true;
            }
        }
    }

    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        self.visitAST(&mut ast.to_ast());
        if is_instance(&**ast.func(), ASTType::LocationExprBase) {
            //for now no reference types -> only state could have been modified
            let mut fdef = ast
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap();
            let mut ast = ast.to_ast();
            let rlen = ast.ast_base_ref().unwrap().read_values.len();
            ast.ast_base_mut_ref().unwrap().read_values = ast
                .ast_base_mut_ref()
                .unwrap()
                .read_values
                .union(
                    &fdef
                        .ast_base_ref()
                        .unwrap()
                        .read_values
                        .iter()
                        .filter_map(|v| {
                            if v.target().is_some()
                                && is_instance(
                                    &v.target().map(|t| *t).unwrap(),
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
            self.fixed_point_reached &= rlen == ast.ast_base_ref().unwrap().read_values.len();

            //update modified values if any
            let mlen = ast.ast_base_ref().unwrap().modified_values.len();
            for v in &fdef.ast_base_ref().unwrap().modified_values {
                if is_instance(
                    &v.target().map(|t| *t).unwrap(),
                    ASTType::StateVariableDeclaration,
                ) {
                    ast.ast_base_mut_ref()
                        .unwrap()
                        .modified_values
                        .insert(v.clone());
                }
            }
            self.fixed_point_reached &= mlen == ast.ast_base_ref().unwrap().modified_values.len();
        }
    }
    pub fn visitAST(&mut self, ast: &mut AST) {
        let mlen = ast.ast_base_ref().unwrap().modified_values.len();
        let rlen = ast.ast_base_ref().unwrap().read_values.len();
        for child in ast.children().iter_mut() {
            self.visit(child);
            ast.ast_base_mut_ref().unwrap().modified_values = ast
                .ast_base_mut_ref()
                .unwrap()
                .modified_values
                .union(&child.ast_base_ref().unwrap().modified_values)
                .cloned()
                .collect();
            ast.ast_base_mut_ref().unwrap().read_values = ast
                .ast_base_mut_ref()
                .unwrap()
                .read_values
                .union(&child.ast_base_ref().unwrap().read_values)
                .cloned()
                .collect();
        }
        self.fixed_point_reached &= mlen == ast.ast_base_ref().unwrap().modified_values.len();
        self.fixed_point_reached &= rlen == ast.ast_base_ref().unwrap().read_values.len();
    }
}
// class EvalOrderUBChecker(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct EvalOrderUBChecker {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitorMut for EvalOrderUBChecker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name
            || &ASTType::ExpressionBase == name
            || &ASTType::AssignmentStatementBase == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_function_call_expr_mut()
                    .unwrap(),
            ),
            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap()
                    .try_as_assignment_statement_mut()
                    .unwrap(),
            ),
            ASTType::ExpressionBase => self.visitExpression(ast.try_as_expression_mut().unwrap()),

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
    pub fn visit_child_expressions(parent: AST, exprs: &mut Vec<AST>) {
        if exprs.len() > 1 {
            let mut modset: BTreeSet<_> = exprs[0].ast_base_ref().unwrap().modified_values.clone();
            for arg in &exprs[1..] {
                let modified_values = arg.ast_base_ref().unwrap().modified_values.clone();
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
                let modset: BTreeSet<_> = arg.ast_base_ref().unwrap().modified_values.clone();
                let other_args: BTreeSet<_> = exprs
                    .iter()
                    .filter_map(|e| if e != arg { Some(e) } else { None })
                    .collect();
                for arg2 in &other_args {
                    let diffset: BTreeSet<_> = modset
                        .intersection(&arg2.ast_base_ref().unwrap().read_values)
                        .collect();
                    if !diffset.is_empty() {
                        let setstr = format!(
                            r#"{{{}}}"#,
                            diffset
                                .iter()
                                .map(|it| format!(
                                    "{:?}{}",
                                    it.target_key[0].as_ref().unwrap(),
                                    if let Some(member) = it.target_key[1].clone().map(|t| *t) {
                                        format!(".{:?}", member)
                                    } else {
                                        String::new()
                                    }
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
    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            if ast
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .has_shortcircuiting()
            {
                return;
            }
        }
        Self::visit_child_expressions(
            ast.to_ast(),
            &mut ast.args().into_iter().map(|v| v.to_ast()).collect(),
        );
    }

    pub fn visitExpression(&mut self, ast: &mut Expression) {
        Self::visit_child_expressions(ast.to_ast(), &mut ast.children());
    }

    pub fn visitAssignmentStatement(&mut self, ast: &mut AssignmentStatement) {
        Self::visit_child_expressions(ast.to_ast(), &mut ast.children());
    }
}
