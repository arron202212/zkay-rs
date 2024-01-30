// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    is_instance, is_instances, ASTChildren, ASTCode, ASTType, AssignmentStatement, BuiltinFunction,
    Expression, FunctionCallExpr, InstanceTarget, InstanceTargetExprUnion, LocationExpr, Parameter,
    StateVariableDeclaration, Statement, TupleExpr, VariableDeclaration, AST,TupleOrLocationExpr,
};
use crate::zkay_ast::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};
use std::collections::BTreeSet;
pub fn has_side_effects(ast: AST) -> bool {
    SideEffectsDetector.visit(ast).is_some()
}

pub fn compute_modified_sets(ast: AST) {
    let v = DirectModificationDetector;
    v.visit(ast);

    let v = IndirectModificationDetector::new();
    v.iterate_until_fixed_point(ast);
}

pub fn check_for_undefined_behavior_due_to_eval_order(ast: AST) {
    EvalOrderUBChecker.visit(ast);
}

// class SideEffectsDetector(AstVisitor)
pub struct SideEffectsDetector;

impl AstVisitor for SideEffectsDetector {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
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
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl SideEffectsDetector {
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) -> bool {
        if is_instance(&ast.func().unwrap(), ASTType::LocationExpr)
            && !ast.is_cast()
            && ast.func().unwrap().target().unwrap().has_side_effects()
        {
            true
        } else {
            self.visitExpression(ast)
        }
    }
    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) -> bool {
        true
    }

    pub fn visitExpression(&self, ast: Expression) -> bool {
        self.visitAST(ast.get_ast())
    }

    pub fn visitStatement(&self, ast: Statement) -> bool {
        self.visitAST(ast.get_ast())
    }

    pub fn visitAST(&self, ast: AST) -> bool {
        ast.children().iter().any(|c| self.visit(c).is_some())
    }
}
// class DirectModificationDetector(FunctionVisitor)
pub struct DirectModificationDetector;

impl FunctionVisitor for DirectModificationDetector {}
impl AstVisitor for DirectModificationDetector {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
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
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl DirectModificationDetector {
    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        self.visitAST(&mut ast.get_ast());
        self.collect_modified_values(&mut ast.get_ast(), ast.lhs().unwrap().into());
    }

    pub fn collect_modified_values(&self, target: &mut AST, expr: AST) {
        if is_instance(&expr, ASTType::TupleExpr) {
            for elem in expr.elements() {
                self.collect_modified_values(target, elem);
            }
        } else {
            let mod_value = InstanceTarget::new(expr);
            if target.modified_values().contains(mod_value) {
                assert!(false,"Undefined behavior due multiple different assignments to the same target in tuple assignment ,{:?}", expr);
            }
            target.modified_values_mut().insert(mod_value);
        }
    }
    pub fn visitLocationExpr(&self, ast: &mut LocationExpr) {
        self.visitAST(&mut (*ast).get_ast());
        let ast1: AST = (*ast.target().unwrap()).into();
        if TupleOrLocationExpr::LocationExpr(*ast).is_rvalue()
            && is_instances(
                &ast.target(),
                vec![
                    ASTType::VariableDeclaration,
                    ASTType::StateVariableDeclaration,
                    ASTType::Parameter,
                ],
            )
        {
            ast.read_values
                .insert(InstanceTargetExprUnion::LocationExpr(ast));
        }
    }
    pub fn visitVariableDeclaration(&self, ast: &mut VariableDeclaration) {
        ast.identifier_declaration_base
            .ast_base
            .modified_values
            .insert(InstanceTarget::new(
                InstanceTargetExprUnion::VariableDeclaration(ast),
            ));
    }

    pub fn visitAST(&self, ast: &mut AST) {
        ast.modified_values_mut().clear();
        ast.read_values_mut().clear();
        for child in ast.children() {
            self.visit(child);
            (*ast.modified_values_mut()) = ast
                .modified_values()
                .union(&child.modified_values)
                .collect();
            (*ast.read_values_mut()) = ast.read_values().union(&child.read_values).collect();
        }
    }
}
// class IndirectModificationDetector(FunctionVisitor)
struct IndirectModificationDetector {
    fixed_point_reached: bool,
}

impl FunctionVisitor for IndirectModificationDetector {}
impl AstVisitor for IndirectModificationDetector {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
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
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
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
        if is_instance(&ast.func, ASTType::LocationExpr) {
            //for now no reference types -> only state could have been modified
            let fdef = ast.func.target;
            let rlen = ast.read_values.len();
            ast.read_values.update(
                fdef.read_values
                    .iter()
                    .filter_map(|v| {
                        if is_instance(&v.target, ASTType::StateVariableDeclaration) {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
            self.fixed_point_reached &= rlen == ast.read_values.len();

            //update modified values if any
            let mlen = ast.modified_values.len();
            for v in fdef.modified_values {
                if is_instance(&v.target, ASTType::StateVariableDeclaration) {
                    ast.modified_values[v] = None;
                }
            }
            self.fixed_point_reached &= mlen == ast.modified_values.len();
        }
    }
    pub fn visitAST(&self, ast: AST) {
        let mlen = ast.modified_values.len();
        let rlen = ast.read_values.len();
        for child in ast.children() {
            self.visit(child);
            ast.modified_values.update(child.modified_values);
            ast.read_values.update(child.read_values);
        }
        self.fixed_point_reached &= mlen == ast.modified_values.len();
        self.fixed_point_reached &= rlen == ast.read_values.len();
    }
}
// class EvalOrderUBChecker(AstVisitor)
struct EvalOrderUBChecker;

impl AstVisitor for EvalOrderUBChecker {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
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
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl EvalOrderUBChecker {
    // @staticmethod
    pub fn visit_child_expressions(parent: AST, exprs: Vec<AST>) {
        if exprs.len() > 1 {
            let mut modset: BTreeSet<_> = exprs[0].modified_values.keys().collect();
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
                    modset = modset.union(diffset).collect();
                }
            }

            for arg in exprs {
                let modset: BTreeSet<_> = arg.modified_values.keys().iter().cloned().collect();
                let other_args = exprs
                    .iter()
                    .filter_map(|e| if e != arg { Some(e) } else { None })
                    .collect();
                for arg2 in other_args {
                    let diffset: BTreeSet<_> = modset.intersection(arg2.read_values).collect();
                    if !diffset.is_empty() {
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
        if is_instance(&ast.func, ASTType::BuiltinFunction) {
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
