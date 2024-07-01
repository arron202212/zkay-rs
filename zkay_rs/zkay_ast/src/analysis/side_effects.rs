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
    is_instance, is_instances, ASTBaseMutRef, ASTChildren, ASTFlatten, ASTInstanceOf, ASTType,
    AssignmentStatement, AssignmentStatementBaseProperty, BuiltinFunction, Expression,
    FunctionCallExpr, FunctionCallExprBaseProperty, IdentifierDeclaration, InstanceTarget, IntoAST,
    IntoExpression, IntoStatement, LocationExpr, Parameter, SimpleStatement,
    StateVariableDeclaration, Statement, TupleExpr, TupleOrLocationExpr, VariableDeclaration, AST,
};
use crate::visitors::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;

use std::collections::{BTreeSet, HashSet};
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

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::FunctionCallExprBase
                | ASTType::ExpressionBase
                | ASTType::AssignmentStatementBase
                | ASTType::StatementBase
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Expression(_))
            || matches!(ast, AST::Statement(_))
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
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),

            _ => Ok(false),
        }
    }
}
impl SideEffectsDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let fce = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .clone();

        if is_instance(fce.func(), ASTType::LocationExprBase)
            && !fce.is_cast()
            && fce
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .has_side_effects()
        {
            Ok(true)
        } else {
            self.visitExpression(ast)
        }
    }
    pub fn visitAssignmentStatement(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(true)
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visitAST(ast)
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visitAST(ast)
    }

    pub fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(ast.children().iter().any(|c| self.visit(c)))
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

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::LocationExprBase
                | ASTType::VariableDeclaration
                | ASTType::AssignmentStatementBase
        ) || matches!(
            ast,
            AST::Expression(Expression::TupleOrLocationExpr(
                TupleOrLocationExpr::LocationExpr(_)
            ))
        ) || matches!(
            ast,
            AST::Statement(Statement::SimpleStatement(
                SimpleStatement::AssignmentStatement(_)
            ))
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::TupleOrLocationExpr(
                    TupleOrLocationExpr::LocationExpr(_)
                ))
            ) =>
            {
                self.visitLocationExpr(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            ASTType::VariableDeclaration => self.visitVariableDeclaration(ast),

            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl DirectModificationDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitAssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visitAST(ast)?;
        Self::collect_modified_values(
            ast,
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_assignment_statement_ref()
                .unwrap()
                .lhs()
                .as_ref()
                .unwrap(),
        );
        Ok(())
    }

    pub fn collect_modified_values(target: &ASTFlatten, expr: &ASTFlatten) {
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
                Self::collect_modified_values(target, &elem.clone());
            }
        } else {
            let mod_value = InstanceTarget::new(vec![Some(expr.clone())]);
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
    pub fn visitLocationExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visitAST(ast)?;
        // println!("=======visitLocationExpr====================={:?}",ast);
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .is_rvalue()
            && is_instances(
                ast.ast_base_ref()
                    .unwrap()
                    .borrow()
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .as_ref()
                    .unwrap(),
                vec![
                    ASTType::VariableDeclaration,
                    ASTType::StateVariableDeclaration,
                    ASTType::Parameter,
                ],
            )
        {
            let it = InstanceTarget::new(vec![Some(ast.clone())]);
            ast.ast_base_ref()
                .unwrap()
                .borrow_mut()
                .read_values
                .insert(it);
        }
        Ok(())
    }
    pub fn visitVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let a = InstanceTarget::new(vec![Some(ast.clone())]);
        ast.try_as_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .ast_base
            .borrow_mut()
            .modified_values
            .insert(a);
        Ok(())
    }

    pub fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
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
        Ok(())
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
        for _ in 0..10 {
            self.visit(ast);
            if *self.fixed_point_reached.borrow() {
                return;
            } else {
                *self.fixed_point_reached.borrow_mut() = true;
            }
        }
        println!("========iterate_until_fixed_point=======break======failed===========");
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visitAST(ast)?;
        if is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::LocationExprBase,
        ) {
            //for now no reference types -> only state could have been modified
            let mut fdef = ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap();
            let rlen = ast.ast_base_ref().unwrap().borrow().read_values.len();
            let rv = ast
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
                        .filter(|v| {
                            v.target().is_some()
                                && is_instance(
                                    &v.target().unwrap(),
                                    ASTType::StateVariableDeclaration,
                                )
                        })
                        .cloned()
                        .collect(),
                )
                .cloned()
                .collect();
            ast.ast_base_ref().unwrap().borrow_mut().read_values = rv;
            // println!("==before===fixed_point_reached=={rlen}=={}==={:?}",ast.ast_base_ref().unwrap().borrow().read_values.len(),*self.fixed_point_reached.borrow() );
            *self.fixed_point_reached.borrow_mut() &=
                rlen == ast.ast_base_ref().unwrap().borrow().read_values.len();
            // println!("==after===fixed_point_reached========{:?}",*self.fixed_point_reached.borrow() );
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
            // println!("==before===fixed_point_reached=={mlen}==={}==={:?}",ast.ast_base_ref().unwrap().borrow().modified_values.len(),*self.fixed_point_reached.borrow() );
            *self.fixed_point_reached.borrow_mut() &=
                mlen == ast.ast_base_ref().unwrap().borrow().modified_values.len();
            // println!("==after===fixed_point_reached========{:?}",*self.fixed_point_reached.borrow() );
        }
        Ok(())
    }
    pub fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mlen = ast.ast_base_ref().unwrap().borrow().modified_values.len();
        let rlen = ast.ast_base_ref().unwrap().borrow().read_values.len();
        for child in ast.children() {
            self.visit(&child);
            let mv: BTreeSet<_> = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .modified_values
                .union(&child.ast_base_ref().unwrap().borrow().modified_values)
                .cloned()
                .collect();
            ast.ast_base_ref().unwrap().borrow_mut().modified_values = mv;
            // println!("{:?},======read_values========{:?}",ast
            //     .ast_base_ref()
            //     .unwrap()
            //     .borrow()
            //     .read_values,child.ast_base_ref().unwrap().borrow().read_values);
            let rv: BTreeSet<_> = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .read_values
                .union(&child.ast_base_ref().unwrap().borrow().read_values)
                .cloned()
                .collect();
            ast.ast_base_ref().unwrap().borrow_mut().read_values = rv;
        }
        let v = (mlen == ast.ast_base_ref().unwrap().borrow().modified_values.len())
            & (rlen == ast.ast_base_ref().unwrap().borrow().read_values.len());
        // println!("==before===fixed_point_reached=={mlen}={}=={rlen}={}=={:?}", ast.ast_base_ref().unwrap().borrow().modified_values.len(),ast.ast_base_ref().unwrap().borrow().read_values.len(),*self.fixed_point_reached.borrow() );
        *self.fixed_point_reached.borrow_mut() &= v;
        // println!("==after===fixed_point_reached========{:?}",*self.fixed_point_reached.borrow() );
        Ok(())
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

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::FunctionCallExprBase
                | ASTType::ExpressionBase
                | ASTType::AssignmentStatementBase
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Expression(_))
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
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),

            _ => Err(eyre::eyre!("unreach")),
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
    pub fn visit_child_expressions(
        parent: &ASTFlatten,
        exprs: &Vec<ASTFlatten>,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
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

            for arg in exprs {
                let modset: BTreeSet<_> =
                    arg.ast_base_ref().unwrap().borrow().modified_values.clone();
                let other_args: BTreeSet<_> = exprs.iter().filter(|e| *e != arg).cloned().collect();
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
        Ok(())
    }
    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if is_instance(
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
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_builtin_function_ref()
            .unwrap()
            .has_shortcircuiting()
        {
            return Ok(());
        }
        Self::visit_child_expressions(
            ast,
            &ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()
                .to_vec(),
        )
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Self::visit_child_expressions(ast, &ast.children())
    }

    pub fn visitAssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Self::visit_child_expressions(ast, &ast.children())
    }
}
