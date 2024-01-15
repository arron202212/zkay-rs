// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    AssignmentStatement, Block, ConstructorOrFunctionDefinition, ContractDefinition,
    IdentifierExpr, IfStatement, StateVariableDeclaration,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn check_final(ast: AST) {
    let v = FinalVisitor();
    v.visit(ast);
}

// class FinalVisitor(AstVisitor)
struct FinalVisitor {
    state_vars_assigned: Option<BTreeMap<StateVariableDeclaration, bool>>,
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
    pub fn visitContractDefinition(self, ast: ContractDefinition) {
        self.state_vars_assigned = {};
        for v in ast.state_variable_declarations {
            if v.is_final && v.expr.is_none() {
                self.state_vars_assigned[v] = False;
            }
        }

        if len(ast.constructor_definitions) > 0 {
            assert!(len(ast.constructor_definitions) == 1);
            c = ast.constructor_definitions[0];
            self.visit(c.body);
        }

        for (sv, assigned) in self.state_vars_assigned.items() {
            if !assigned {
                assert!(false, "Did not set all final state variables {}", sv)
            }
        }

        self.state_vars_assigned = None;
    }
    pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) {
        assert!(ast.is_function);
    }

    pub fn visitAssignmentStatement(self, ast: AssignmentStatement) {
        self.visit(ast.rhs);
        if isinstance(ast.lhs, IdentifierExpr) {
            let var = &ast.lhs.target;
            if self.state_vars_assigned.contains(var) {
                {
                    if self.state_vars_assigned[var] {
                        assert!(false, "Tried to reassign final variable,{:?}", ast);
                    }
                }

                self.state_vars_assigned[var] = True;
            }
        }
    }

    pub fn visitIfStatement(self, ast: IfStatement) {
        self.visit(ast.condition);
        prev = self.state_vars_assigned.copy();
        self.visit(ast.then_branch);
        then_b = self.state_vars_assigned.copy();
        self.state_vars_assigned = prev;
        if ast.else_branch.is_some() {
            self.visit(ast.else_branch);
        }

        assert!(then_b.keys() == self.state_vars_assigned.keys());
        for var in then_b.keys() {
            if then_b[var] != self.state_vars_assigned[var] {
                assert!(
                    false,
                    "Final value is not assigned in both branches,{:?}",
                    ast
                );
            }
        }
    }
    pub fn visitIdentifierExpr(self, ast: IdentifierExpr) {
        if ast.is_rvalue() && self.state_vars_assigned.is_some() {
            if self.state_vars_assigned.contains(ast.target)
                && !self.state_vars_assigned[ast.target]
            {
                assert!(
                    false,
                    r#"{ast:?} is reading "final" state variable before writing it"#,
                );
            }
        }
    }
}
