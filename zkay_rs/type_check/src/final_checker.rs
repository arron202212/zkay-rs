#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// use type_check::type_exceptions::TypeException
use std::collections::BTreeMap;
use zkay_ast::ast::{
    is_instance, ASTFlatten, ASTType, AssignmentStatement, AssignmentStatementBaseProperty, Block,
    ConstructorOrFunctionDefinition, ContractDefinition, Expression, IdentifierDeclarationBaseRef,
    IdentifierExpr, IfStatement, IntoAST, LocationExpr, LocationExprBaseProperty,
    StateVariableDeclaration, TupleOrLocationExpr, AST,
};
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn check_final(ast: AST) {
    let v = FinalVisitor::new();
    v.visit(&ast);
}

// class FinalVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
struct FinalVisitor {
    pub ast_visitor_base: AstVisitorBase,
    state_vars_assigned: Option<BTreeMap<AST, bool>>,
}
impl AstVisitor for FinalVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn has_attr(&self, name: &ASTType) -> bool {
        matches! {name,
         ASTType::ContractDefinition|
         ASTType::ConstructorOrFunctionDefinition|
         ASTType::AssignmentStatement|
         ASTType::IfStatement|
         ASTType::IdentifierExpr
        }
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::ContractDefinition => self.visitContractDefinition(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            ASTType::AssignmentStatement => self.visitAssignmentStatement(ast),
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
            state_vars_assigned: None,
        }
    }
    pub fn visitContractDefinition(&self, ast: &ASTFlatten) {
        self.state_vars_assigned = Some(BTreeMap::new());
        for v in &ast.state_variable_declarations {
            if v.try_as_identifier_declaration_ref()
                .unwrap()
                .identifier_declaration_base_ref()
                .is_final()
                && v.try_as_identifier_declaration_ref()
                    .unwrap()
                    .try_as_state_variable_declaration_ref()
                    .unwrap()
                    .expr
                    .is_some()
            {
                self.state_vars_assigned
                    .as_mut()
                    .unwrap()
                    .insert(v.clone(), false);
            }
        }

        if ast.constructor_definitions.len() > 0 {
            assert!(ast.constructor_definitions.len() == 1);
            let c = &ast.constructor_definitions[0];
            self.visit(c.body.clone().into());
        }

        for (sv, assigned) in self.state_vars_assigned.as_ref().unwrap() {
            if !assigned {
                assert!(false, "Did not set all final state variables {}", sv)
            }
        }

        self.state_vars_assigned = None;
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: &ASTFlatten) {
        assert!(ast.is_function());
    }

    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) {
        self.visit(ast.rhs().clone().into());
        if let Some(le) = ast
            .lhs()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_identifier_expr_ref()
        {
            if let Some(var) = le.location_expr_base.target() {
                if let Some(v) = self.state_vars_assigned.as_mut().unwrap().get_mut(&var) {
                    assert!(!*v, "Tried to reassign final variable,{:?}", ast);
                    *v = true;
                }
            }
        }
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) {
        self.visit(ast.condition.clone().into());
        let prev = self.state_vars_assigned.as_ref().unwrap().clone();
        self.visit(ast.then_branch.clone().into());
        let then_b = self.state_vars_assigned.as_ref().unwrap().clone();
        self.state_vars_assigned = Some(prev);
        if let Some(else_branch) = &ast.else_branch {
            self.visit(else_branch.clone().into());
        }

        assert!(
            then_b.keys().collect::<Vec<_>>()
                == self
                    .state_vars_assigned
                    .as_ref()
                    .unwrap()
                    .keys()
                    .collect::<Vec<_>>()
        );
        for (var, flag) in &then_b {
            assert!(
                flag == self.state_vars_assigned.as_ref().unwrap().get(var).unwrap(),
                "Final value is not assigned in both branches,{:?}",
                ast
            );
        }
    }
    pub fn visitIdentifierExpr(&self, ast: &ASTFlatten) {
        if TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(ast.clone())).is_rvalue()
            && self.state_vars_assigned.is_some()
        {
            if let Some(&v) = self.state_vars_assigned.as_ref().unwrap().get(
                &(ast
                    .location_expr_base
                    .target
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .clone())
                .into(),
            ) {
                assert!(
                    v,
                    r#"{ast:?} is reading "final" state variable before writing it"#,
                );
            }
        }
    }
}
