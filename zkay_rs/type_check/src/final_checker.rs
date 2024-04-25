#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use rccell::RcCell;
// use type_check::type_exceptions::TypeException
use std::collections::BTreeMap;
use zkay_ast::ast::{
    is_instance, ASTFlatten, ASTInstanceOf, ASTType, AssignmentStatement,
    AssignmentStatementBaseProperty, Block, ConstructorOrFunctionDefinition, ContractDefinition,
    Expression, IdentifierDeclarationBaseRef, IdentifierExpr, IfStatement, IntoAST, LocationExpr,
    LocationExprBaseProperty, SimpleStatement, StateVariableDeclaration, Statement,
    TupleOrLocationExpr, AST,
};
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_final(ast: &ASTFlatten) {
    let v = FinalVisitor::new();
    v.visit(ast);
}

// class FinalVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct FinalVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub state_vars_assigned: RcCell<Option<BTreeMap<ASTFlatten, bool>>>,
}
impl AstVisitor for FinalVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::ContractDefinition
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::AssignmentStatementBase
                | ASTType::IfStatement
                | ASTType::IdentifierExpr
        ) || matches!(
            ast,
            AST::Statement(Statement::SimpleStatement(
                SimpleStatement::AssignmentStatement(_)
            ))
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::ContractDefinition => self.visitContractDefinition(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
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
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            _ => {}
        }
    }
}
impl FinalVisitor {
    // pub fn __init__(self)
    //     super().__init__("node-or-children")
    //     self.state_vars_assigned: Optional[Dict[StateVariableDeclaration, bool]] = None
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            state_vars_assigned: RcCell::new(None),
        }
    }
    pub fn visitContractDefinition(&self, ast: &ASTFlatten) -> <Self as AstVisitor>::Return {
        *self.state_vars_assigned.borrow_mut() = Some(BTreeMap::new());
        for v in &ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .state_variable_declarations
        {
            if v.try_as_identifier_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base_ref()
                .is_final()
                && v.try_as_identifier_declaration_ref()
                    .unwrap()
                    .borrow()
                    .try_as_state_variable_declaration_ref()
                    .unwrap()
                    .expr
                    .is_some()
            {
                self.state_vars_assigned
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .insert(v.clone(), false);
            }
        }

        if ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .constructor_definitions
            .len()
            > 0
        {
            assert!(
                ast.try_as_contract_definition_ref()
                    .unwrap()
                    .borrow()
                    .constructor_definitions
                    .len()
                    == 1
            );
            let c = &ast
                .try_as_contract_definition_ref()
                .unwrap()
                .borrow()
                .constructor_definitions[0];
            self.visit(&c.borrow().body.clone().unwrap().into());
        }

        for (sv, assigned) in self.state_vars_assigned.borrow().as_ref().unwrap() {
            if !assigned {
                assert!(false, "Did not set all final state variables {}", sv)
            }
        }

        *self.state_vars_assigned.borrow_mut() = None;
    }
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstVisitor>::Return {
        assert!(ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .is_function());
    }

    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) -> <Self as AstVisitor>::Return {
        self.visit(
            &ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .rhs()
                .clone()
                .unwrap()
                .into(),
        );
        if let Some(le) = ast
            .try_as_assignment_statement_ref()
            .unwrap()
            .borrow()
            .lhs()
            .as_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .borrow()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_identifier_expr_ref()
        {
            if let Some(var) = le.location_expr_base.target() {
                if let Some(v) = self
                    .state_vars_assigned
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .get_mut(&var.clone().upgrade().unwrap())
                {
                    assert!(!*v, "Tried to reassign final variable,{:?}", ast);
                    *v = true;
                }
            }
        }
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> <Self as AstVisitor>::Return {
        self.visit(
            &ast.try_as_if_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into(),
        );
        let prev = self.state_vars_assigned.borrow().as_ref().unwrap().clone();
        self.visit(
            &ast.try_as_if_statement_ref()
                .unwrap()
                .borrow()
                .then_branch
                .clone()
                .into(),
        );
        let then_b = self.state_vars_assigned.borrow().as_ref().unwrap().clone();
        *self.state_vars_assigned.borrow_mut() = Some(prev);
        if let Some(else_branch) = &ast.try_as_if_statement_ref().unwrap().borrow().else_branch {
            self.visit(&else_branch.clone().into());
        }

        assert!(
            then_b.keys().collect::<Vec<_>>()
                == self
                    .state_vars_assigned
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .keys()
                    .collect::<Vec<_>>()
        );
        for (var, flag) in &then_b {
            assert!(
                flag == self
                    .state_vars_assigned
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .get(var)
                    .unwrap(),
                "Final value is not assigned in both branches,{:?}",
                ast
            );
        }
    }
    pub fn visitIdentifierExpr(&self, ast: &ASTFlatten) -> <Self as AstVisitor>::Return {
        if TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(
            ast.try_as_identifier_expr_ref().unwrap().borrow().clone(),
        ))
        .is_rvalue()
            && self.state_vars_assigned.borrow().is_some()
        {
            if let Some(&v) = self.state_vars_assigned.borrow().as_ref().unwrap().get(
                &ast.try_as_identifier_expr_ref()
                    .unwrap()
                    .borrow()
                    .location_expr_base
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
            ) {
                assert!(
                    v,
                    r#"{ast:?} is reading "final" state variable before writing it"#,
                );
            }
        }
    }
}
