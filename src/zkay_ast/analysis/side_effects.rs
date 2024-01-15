// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    AssignmentStatement, BuiltinFunction, Expression, FunctionCallExpr, InstanceTarget,
    LocationExpr, Parameter, StateVariableDeclaration, Statement, TupleExpr, VariableDeclaration,
    AST,
};
use crate::zkay_ast::visitor::function_visitor::FunctionVisitor;
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn has_side_effects(ast: AST) -> bool {
    SideEffectsDetector::new().visit(ast)
}

pub fn compute_modified_sets(ast: AST) {
    let v = DirectModificationDetector::new();
    v.visit(ast);

    let v = IndirectModificationDetector::new();
    v.iterate_until_fixed_point(ast);
}

pub fn check_for_undefined_behavior_due_to_eval_order(ast: AST) {
    EvalOrderUBChecker::new().visit(ast);
}

// class SideEffectsDetector(AstVisitor)
pub struct SideEffectsDetector;
impl SideEffectsDetector {
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        if isinstance(ast.func, LocationExpr) && !ast.is_cast && ast.func.target.has_side_effects {
            true
        } else {
            self.visitExpression(ast)
        }
    }
    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        true
    }

    pub fn visitExpression(&self, ast: Expression) {
        self.visitAST(ast)
    }

    pub fn visitStatement(&self, ast: Statement) {
        self.visitAST(ast)
    }

    pub fn visitAST(&self, ast: AST) {
        any(map(self.visit, ast.children()))
    }
}
// class DirectModificationDetector(FunctionVisitor)
pub struct DirectModificationDetector;
impl DirectModificationDetector {
    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        self.visitAST(ast);
        self.collect_modified_values(ast, ast.lhs);
    }

    pub fn collect_modified_values(
        &self,
        target: (Option<Expression>, Option<Statement>),
        expr: (Option<TupleExpr>, Option<LocationExpr>),
    ) {
        if isinstance(expr, TupleExpr) {
            for elem in expr.elements {
                self.collect_modified_values(target, elem);
            }
        } else {
            mod_value = InstanceTarget(expr);
            if target.modified_values.contains(mod_value) {
                assert!(false,"Undefined behavior due multiple different assignments to the same target in tuple assignment ,{:?}", expr);
            }
            target.modified_values[mod_value] = None;
        }
    }
    pub fn visitLocationExpr(&self, ast: LocationExpr) {
        self.visitAST(ast);
        if ast.is_rvalue()
            && isinstance(
                ast.target,
                (VariableDeclaration, StateVariableDeclaration, Parameter),
            )
        {
            ast.read_values.add(InstanceTarget(ast));
        }
    }
    pub fn visitVariableDeclaration(&self, ast: VariableDeclaration) {
        ast.modified_values[InstanceTarget(ast)] = None;
    }

    pub fn visitAST(&self, ast: AST) {
        ast.modified_values.clear();
        ast.read_values.clear();
        for child in ast.children() {
            self.visit(child);
            ast.modified_values.update(child.modified_values);
            ast.read_values.update(child.read_values);
        }
    }
}
// class IndirectModificationDetector(FunctionVisitor)
struct IndirectModificationDetector {
    fixed_point_reached: bool,
}
impl IndirectModificationDetector {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.fixed_point_reached = true
    pub fn new() -> Self {
        Self {
            fixed_point_reached: true,
        }
    }
    pub fn iterate_until_fixed_point(&self, ast: AST) {
        loop {
            self.visit(ast);
            if self.fixed_point_reached {
                break;
            } else {
                self.fixed_point_reached = true;
            }
        }
    }

    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        self.visitAST(ast);
        if isinstance(ast.func, LocationExpr) {
            //for now no reference types -> only state could have been modified
            let fdef = ast.func.target;
            let rlen = len(ast.read_values);
            ast.read_values.update(
                fdef.read_values
                    .iter()
                    .filter_map(|v| {
                        if isinstance(v.target, StateVariableDeclaration) {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
            self.fixed_point_reached &= rlen == len(ast.read_values);

            //update modified values if any
            let mlen = len(ast.modified_values);
            for v in fdef.modified_values {
                if isinstance(v.target, StateVariableDeclaration) {
                    ast.modified_values[v] = None;
                }
            }
            self.fixed_point_reached &= mlen == len(ast.modified_values);
        }
    }
    pub fn visitAST(&self, ast: AST) {
        let mlen = len(ast.modified_values);
        let rlen = len(ast.read_values);
        for child in ast.children() {
            self.visit(child);
            ast.modified_values.update(child.modified_values);
            ast.read_values.update(child.read_values);
        }
        self.fixed_point_reached &= mlen == len(ast.modified_values);
        self.fixed_point_reached &= rlen == len(ast.read_values);
    }
}
// class EvalOrderUBChecker(AstVisitor)
struct EvalOrderUBChecker;
impl EvalOrderUBChecker {
    // @staticmethod
    pub fn visit_child_expressions(parent: AST, exprs: Vec<AST>) {
        if len(exprs) > 1 {
            modset = set(exprs[0].modified_values.keys());
            for arg in exprs[1..] {
                let diffset = modset.intersection(arg.modified_values);
                if !diffset.is_empty() {
                    let setstr = format!(
                        "{{{}}}",
                        (diffset.iter().map(String::from))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    assert!(
                        false,
                        r#"Undefined behavior due to potential side effect on the same value(s) \"{setstr}\" in multiple expression children.\n"
                                        "Solidity does not guarantee an evaluation order for non-shortcircuit expressions.\n"
                                        "Since zkay requires local simulation for transaction transformation, all semantics must be well-defined. {:?}"#,
                        parent
                    )
                } else {
                    modset.update(diffset);
                }
            }

            for arg in exprs {
                let modset: BTreeSet<_> = arg.modified_values.keys().iter().cloned().collect();
                let other_args = exprs
                    .iter()
                    .filter_map(|e| if e != arg { Some(e) } else { None })
                    .collect();
                for arg2 in other_args {
                    diffset = modset.intersection(arg2.read_values);
                    if diffset {
                        let setstr = format!(
                            r#"{{{}}}"#,
                            diffset
                                .iter()
                                .map(|(val, member)| format!(
                                    "{val}{}",
                                    if member { &format!(".{member}") } else { "" }
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
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        if isinstance(ast.func, BuiltinFunction) {
            if ast.func.has_shortcircuiting() {
                return;
            }
        }
        self.visit_child_expressions(ast, ast.args);
    }

    pub fn visitExpression(&self, ast: Expression) {
        self.visit_child_expressions(ast, ast.children());
    }

    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        self.visit_child_expressions(ast, ast.children());
    }
}
