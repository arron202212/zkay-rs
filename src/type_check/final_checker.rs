// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    is_instance, ASTCode, ASTType, AssignmentStatement,  Block,
    ConstructorOrFunctionDefinition, ContractDefinition, Expression, IdentifierExpr, IfStatement,
    LocationExpr, StateVariableDeclaration, TupleOrLocationExpr, AST,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;
use std::collections::BTreeMap;

pub fn check_final(ast: AST) {
    let v = FinalVisitor::new();
    v.visit(ast);
}

// class FinalVisitor(AstVisitor)
struct FinalVisitor {
    state_vars_assigned: Option<BTreeMap<AST, bool>>,
}
impl AstVisitor for FinalVisitor {
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
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
        None
    }
}
impl FinalVisitor {
    // pub fn __init__(self)
    //     super().__init__("node-or-children")
    //     self.state_vars_assigned: Optional[Dict[StateVariableDeclaration, bool]] = None
    pub fn new() -> Self {
        Self {
            state_vars_assigned: None,
        }
    }
    pub fn visitContractDefinition(&mut self, ast: ContractDefinition) {
        self.state_vars_assigned = Some(BTreeMap::new());
        for v in &ast.state_variable_declarations {
            if v.is_final() && v.expr() != Expression::None {
                self.state_vars_assigned.as_mut().unwrap().insert(v.clone(), false);
            }
        }

        if ast.constructor_definitions.len() > 0 {
            assert!(ast.constructor_definitions.len() == 1);
            let c = &ast.constructor_definitions[0];
            self.visit(c.body.as_ref().unwrap().get_ast());
        }

        for (sv, assigned) in self.state_vars_assigned.as_ref().unwrap() {
            if !assigned {
                assert!(false, "Did not set all final state variables {}", sv)
            }
        }

        self.state_vars_assigned = None;
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition) {
        assert!(ast.is_function());
    }

    pub fn visitAssignmentStatement(&mut self, ast: AssignmentStatement) {
        self.visit(ast.rhs().unwrap().get_ast());
        if let Some(le) =
            ast.lhs().map(|l|l.identifier_expr())
        {
            let var: &AST = &(*le.unwrap().location_expr_base.target.unwrap()).into();
            if let Some(v) = self.state_vars_assigned.as_mut().unwrap().get_mut(var) {
                assert!(!*v, "Tried to reassign final variable,{:?}", ast);
                *v = true;
            }
        }
    }

    pub fn visitIfStatement(&mut self, ast: IfStatement) {
        self.visit(ast.condition.get_ast());
        let prev = self.state_vars_assigned.as_ref().unwrap().clone();
        self.visit(ast.then_branch.get_ast());
        let then_b = self.state_vars_assigned.as_ref().unwrap().clone();
        self.state_vars_assigned = Some(prev);
        if let Some(else_branch) = &ast.else_branch {
            self.visit(else_branch.get_ast());
        }

        assert!(
            then_b.keys().collect::<Vec<_>>()
                == self.state_vars_assigned.as_ref().unwrap().keys().collect::<Vec<_>>()
        );
        for (var, flag) in &then_b {
                assert!(
                    flag == self.state_vars_assigned.as_ref().unwrap().get(var).unwrap() ,
                    "Final value is not assigned in both branches,{:?}",
                    ast
                );
        }
    }
    pub fn visitIdentifierExpr(&self, ast: IdentifierExpr) {
        if TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(ast.clone())).is_rvalue()
            && self.state_vars_assigned.is_some()
        {
            if let Some(&v) = self
                .state_vars_assigned.as_ref()
                .unwrap()
                .get(&(*ast.location_expr_base.target.clone().unwrap()).into())
            {
                assert!(
                    v,
                    r#"{ast:?} is reading "final" state variable before writing it"#,
                );
            }
        }
    }
}
