#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use rccell::RcCell;
use std::collections::BTreeSet;
use zkay_config::config::CFG;
// use type_check::type_exceptions::TypeException
use crate::analysis::partition_state::PartitionState;
use crate::ast::{
    is_instance, is_instances, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType,
    AssignmentStatement, BooleanLiteralType, BuiltinFunction, ConstructorOrFunctionDefinition,
    Expression, ExpressionBaseMutRef, ExpressionBaseProperty, ExpressionBaseRef, FunctionCallExpr,
    FunctionCallExprBaseProperty, FunctionTypeName, IfStatement, IndexExpr, IntoAST,
    IntoExpression, LocationExpr, NumberLiteralType, PrimitiveCastExpr, ReclassifyExpr,
    ReclassifyExprBaseProperty, ReturnStatement, SimpleStatement, Statement, StatementBaseProperty,
    StatementList, TupleOrLocationExpr, VariableDeclarationStatement, AST,
};
use crate::visitors::{
    function_visitor::FunctionVisitor,
    visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef},
};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_circuit_compliance(ast: &ASTFlatten) {
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
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::FunctionCallExprBase
                | ASTType::LocationExprBase
                | ASTType::ReclassifyExpr
                | ASTType::AssignmentStatementBase
                | ASTType::VariableDeclarationStatement
                | ASTType::ReturnStatement
                | ASTType::IfStatement
                | ASTType::StatementListBase
                | ASTType::StatementBase
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(
                ast,
                AST::Expression(Expression::TupleOrLocationExpr(
                    TupleOrLocationExpr::LocationExpr(_)
                ))
            )
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Statement(Statement::StatementList(_)))
            || matches!(ast, AST::Statement(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }

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
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.visitStatementList(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl DirectCanBePrivateDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
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
        ) && !ast
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
        {
            let mut can_be_private = ast
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
                .can_be_private();
            if ast
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
                .is_eq()
                || ast
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
                    .is_ite()
            {
                can_be_private &= ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()[1]
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .can_be_private();
            }
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .statement()
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
                .can_be_private &= can_be_private;
            //TODO to relax this for public expressions,
            // public identifiers must use SSA remapped values (since the function is inlined)
        }
        for arg in ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .args()
        {
            self.visit(arg);
        }
        Ok(())
    }

    pub fn visitLocationExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!(
        //     "==visitLocationExpr========{:?}======={ast:?}",
        //     ast.to_string()
        // );
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .annotated_type()
            .as_ref()
            .is_none()
        {
            return Ok(());
        }
        let t = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .clone();
        ast.to_ast()
            .try_as_expression_ref()
            .unwrap()
            .statement()
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
            .function()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .borrow_mut()
            .can_be_private &= t.as_ref().unwrap().borrow().can_be_private();
        self.visit_children(ast)
    }

    pub fn visitReclassifyExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit(ast.try_as_reclassify_expr_ref().unwrap().borrow().expr());
        Ok(())
    }

    pub fn visitAssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    }

    pub fn visitVariableDeclarationStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    }

    pub fn visitReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    }

    pub fn visitStatementList(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        //All other statement types are not supported inside circuit (for now)
        ast.to_ast()
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
            .can_be_private = false;
        Ok(())
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
impl IndirectCanBePrivateDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .can_be_private
        {
            for fct in &ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .called_functions
            {
                if !fct.borrow().can_be_private {
                    ast.try_as_constructor_or_function_definition_ref()
                        .unwrap()
                        .borrow_mut()
                        .can_be_private = false;
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
// class CircuitComplianceChecker(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct CircuitComplianceChecker {
    pub ast_visitor_base: AstVisitorBase,
    pub priv_setter: RcCell<PrivateSetter>,
    pub inside_privif_stmt: RcCell<bool>,
}

impl FunctionVisitor for CircuitComplianceChecker {}
impl AstVisitor for CircuitComplianceChecker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::IndexExpr
                | ASTType::ReclassifyExpr
                | ASTType::FunctionCallExprBase
                | ASTType::PrimitiveCastExpr
                | ASTType::IfStatement
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::SourceUnit => <Self as FunctionVisitor>::visitSourceUnit(self, ast),
            ASTType::Parameter => <Self as FunctionVisitor>::visitParameter(self, ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
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
            priv_setter: RcCell::new(PrivateSetter::new()),
            inside_privif_stmt: RcCell::new(false),
        }
    }
    // @staticmethod
    pub fn should_evaluate_public_expr_in_circuit(expr: &ASTFlatten) -> bool {
        // assert!(expr.annotated_type());
        if CFG
            .lock()
            .unwrap()
            .user_config
            .opt_eval_constexpr_in_circuit()
        {
            if is_instances(
                &*expr
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow(),
                vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
            ) {
                //Expressions for which the value is known at compile time -> embed constant expression value into the circuit
                return true;
            }

            if is_instance(expr, ASTType::PrimitiveCastExpr)
                && is_instances(
                    expr.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_primitive_cast_expr_ref()
                        .unwrap()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
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
        check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(expr);
        // except TypeException
        //     //Cannot evaluate inside circuit -> never do it
        //     return false

        //Could evaluate in circuit, use analysis to determine whether this would be better performance wise
        //(If this avoids unnecessary encryption operations it may be cheaper)
        false
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .evaluate_privately()
        {
            assert!(ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_index_expr_ref()
                .unwrap()
                .key
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_public());
            self.priv_setter.borrow_mut().set_evaluation(
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_index_expr_ref()
                    .unwrap()
                    .key,
                false,
            );
        }
        self.visit_children(ast)
    }

    pub fn visitReclassifyExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(!*self.inside_privif_stmt.borrow()
            || ast.try_as_reclassify_expr_ref().unwrap().borrow().statement().as_ref().unwrap().clone().upgrade().unwrap().try_as_statement_ref().unwrap().borrow().statement_base_ref().unwrap().before_analysis.as_ref().unwrap().same_partition(
                &ast.try_as_reclassify_expr_ref().unwrap().borrow().privacy().try_as_expression_ref().unwrap().borrow().privacy_annotation_label().unwrap().to_ast(),
                &Expression::me_expr(None).to_ast(),
            ),"Revealing information to other parties is not allowed inside private if statements {:?}", ast);
        if ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .expr()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_public()
        {
            let eval_in_public = false;
            // try
            self.priv_setter.borrow_mut().set_evaluation(ast, true);
            // except TypeException
            //     eval_in_public = true
            if eval_in_public
                || !Self::should_evaluate_public_expr_in_circuit(
                    ast.try_as_reclassify_expr_ref().unwrap().borrow().expr(),
                )
            {
                self.priv_setter.borrow_mut().set_evaluation(
                    ast.try_as_reclassify_expr_ref().unwrap().borrow().expr(),
                    false,
                );
            }
        } else {
            self.priv_setter.borrow_mut().set_evaluation(ast, true);
        }
        self.visit(ast.try_as_reclassify_expr_ref().unwrap().borrow().expr());
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
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_private()
        {
            self.priv_setter.borrow_mut().set_evaluation(ast, true);
        }
        self.visit_children(ast)
    }

    pub fn visitPrimitiveCastExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
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
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            self.priv_setter.borrow_mut().set_evaluation(ast, true);
        }
        self.visit_children(ast)
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let old_in_privif_stmt = *self.inside_privif_stmt.borrow();
        if ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_if_statement_ref()
            .unwrap()
            .condition
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            let mut mod_vals = ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .then_branch
                .borrow()
                .statement_list_base
                .statement_base
                .ast_base
                .borrow()
                .modified_values
                .clone();
            if ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .else_branch
                .is_some()
            {
                mod_vals = mod_vals
                    .union(
                        &ast.to_ast()
                            .try_as_statement_ref()
                            .unwrap()
                            .try_as_if_statement_ref()
                            .unwrap()
                            .else_branch
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .statement_list_base
                            .statement_base
                            .ast_base
                            .borrow()
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
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .zkay_type()
                    .type_name
                    .unwrap()
                    .borrow()
                    .is_primitive_type()
                {
                    panic!("Writes to non-primitive type variables are not allowed inside private if statements {:?}", ast)
                }
                if val.in_scope_at(ast)
                    && !ast
                        .to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_if_statement_ref()
                        .unwrap()
                        .statement_base
                        .before_analysis
                        .as_ref()
                        .unwrap()
                        .same_partition(
                            &val.privacy().unwrap().to_ast(),
                            &Expression::me_expr(None).to_ast(),
                        )
                {
                    panic!("If statement with private condition must not contain side effects to variables with owner != me ,{:?}", ast)
                }
            }
            *self.inside_privif_stmt.borrow_mut() = true;
            self.priv_setter.borrow_mut().set_evaluation(ast, true);
        }
        let ret = self.visit_children(ast);
        *self.inside_privif_stmt.borrow_mut() = old_in_privif_stmt;
        ret
    }
}
// class PrivateSetter(FunctionVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct PrivateSetter {
    pub ast_visitor_base: AstVisitorBase,
    pub evaluate_privately: RcCell<Option<bool>>,
}

impl FunctionVisitor for PrivateSetter {}
impl AstVisitor for PrivateSetter {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::SourceUnit
                | ASTType::Parameter
                | ASTType::FunctionCallExprBase
                | ASTType::ExpressionBase
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(ast, AST::Expression(_))
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
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),

            _ => Err(eyre::eyre!("unreach")),
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
            evaluate_privately: RcCell::new(None),
        }
    }
    pub fn set_evaluation(&self, ast: &ASTFlatten, evaluate_privately: bool) {
        *self.evaluate_privately.borrow_mut() = Some(evaluate_privately);
        self.visit(ast);
        *self.evaluate_privately.borrow_mut() = None;
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if self.evaluate_privately.borrow().is_some()
            && is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::LocationExprBase,
            )
            && !ast
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
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .as_ref()
                .unwrap()
                .clone()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
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
        self.visitExpression(ast)
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(self.evaluate_privately.borrow().is_some());
        if ast.is_expression() {
            ast.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .evaluate_privately = self.evaluate_privately.borrow().unwrap();
        } else if ast.is_ast() {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_expression_mut()
                .unwrap()
                .expression_base_mut_ref()
                .evaluate_privately = self.evaluate_privately.borrow().unwrap();
        } else if ast.is_reclassify_expr_base() {
            ast.try_as_reclassify_expr_base_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .evaluate_privately = self.evaluate_privately.borrow().unwrap();
        } else if ast.is_primitive_cast_expr() {
            ast.try_as_primitive_cast_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .evaluate_privately = self.evaluate_privately.borrow().unwrap();
        } else if ast.is_location_expr() {
            ast.try_as_location_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .evaluate_privately = self.evaluate_privately.borrow().unwrap();
        } else {
            panic!("==========================={ast:?}");
        }
        self.visit_children(ast)
    }
}
pub fn check_for_nonstatic_function_calls_or_not_circuit_inlineable_in_private_exprs(
    ast: &ASTFlatten,
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
impl NonstaticOrIncompatibilityDetector {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut can_be_private = true;
        let mut has_nonstatic_call = false;
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .evaluate_privately()
            && !ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .is_cast()
        {
            if is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::LocationExprBase,
            ) {
                assert!(ast
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
                    .is_some());
                assert!(is_instance(
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .func()
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .target
                        .as_ref()
                        .unwrap()
                        .clone()
                        .upgrade()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap(),
                    ASTType::FunctionTypeName
                ));
                has_nonstatic_call |= !ast
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
                    .as_ref()
                    .unwrap()
                    .clone()
                    .upgrade()
                    .unwrap()
                    .try_as_namespace_definition_ref()
                    .unwrap()
                    .borrow()
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .has_static_body;
                can_be_private &= ast
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
                    .as_ref()
                    .unwrap()
                    .clone()
                    .upgrade()
                    .unwrap()
                    .try_as_namespace_definition_ref()
                    .unwrap()
                    .borrow()
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .can_be_private;
            } else if is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::BuiltinFunction,
            ) {
                can_be_private &= ast
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
                    .can_be_private()
                    || ast
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .is_literals();
                if ast
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
                    .is_eq()
                    || ast
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
                        .is_ite()
                {
                    can_be_private &= ast
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .args()[1]
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
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
        self.visit_children(ast)
    }
}
