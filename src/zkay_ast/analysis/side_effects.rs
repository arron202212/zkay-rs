// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    is_instance, is_instances, ASTChildren, ASTCode, ASTType, AssignmentStatement, BuiltinFunction,
    Expression, FunctionCallExpr, IdentifierDeclaration, InstanceTarget, 
    LocationExpr, Parameter, StateVariableDeclaration, Statement,  TupleExpr,
    TupleOrLocationExpr, VariableDeclaration, AST,
};
use crate::zkay_ast::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};
use std::collections::BTreeSet;
pub fn has_side_effects(ast: AST) -> bool {
    SideEffectsDetector.visit(ast).is_some()
}

pub fn compute_modified_sets(ast: AST) {
    let v = DirectModificationDetector;
    v.visit(ast.clone());

    let mut v = IndirectModificationDetector::new();
    v.iterate_until_fixed_point(ast.clone());
}

pub fn check_for_undefined_behavior_due_to_eval_order(ast: AST) {
    EvalOrderUBChecker.visit(ast);
}

// class SideEffectsDetector(AstVisitor)
pub struct SideEffectsDetector;

impl AstVisitor for SideEffectsDetector {
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
impl SideEffectsDetector {
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) -> bool {
        if is_instance(&ast.func().unwrap(), ASTType::LocationExpr)
            && !ast.is_cast()
            && (*ast.func().unwrap().target().unwrap()).constructor_or_function_definition().unwrap().has_side_effects()
        {
            true
        } else {
            self.visitExpression(ast.to_expr())
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

    pub fn visitAST(&self, mut ast: AST) -> bool {
        ast.children()
            .iter()
            .any(|c| self.visit(c.clone()).is_some())
    }
}
// class DirectModificationDetector(FunctionVisitor)
pub struct DirectModificationDetector;

impl FunctionVisitor for DirectModificationDetector {}
impl AstVisitor for DirectModificationDetector {
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
impl DirectModificationDetector {
    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        self.visitAST(&mut ast.get_ast());
        self.collect_modified_values(&mut ast.get_ast(), ast.lhs().unwrap().into());
    }

    pub fn collect_modified_values(&self, target: &mut AST, expr: AST) {
        if is_instance(&expr, ASTType::TupleExpr) {
            for elem in expr.elements() {
                self.collect_modified_values(target, elem.get_ast());
            }
        } else {
            let mod_value = InstanceTarget::new(vec![Some(Box::new(expr.get_ast()))]);
            if target
                .ast_base()
                .unwrap()
                .modified_values
                .contains(&mod_value)
            {
                assert!(false,"Undefined behavior due multiple different assignments to the same target in tuple assignment ,{:?}", expr);
            }
            target
                .ast_base_mut()
                .unwrap()
                .modified_values
                .insert(mod_value);
        }
    }
    pub fn visitLocationExpr(&self, ast: &mut LocationExpr) {
        let ast2:LocationExpr=ast.clone();
        self.visitAST(&mut (*ast).get_ast());
        let ast1: AST = (*ast.target().unwrap()).into();
        if TupleOrLocationExpr::LocationExpr(ast.clone()).is_rvalue()
            && is_instances(
                &ast1,
                vec![
                    ASTType::VariableDeclaration,
                    ASTType::StateVariableDeclaration,
                    ASTType::Parameter,
                ],
            )
        {
            ast.ast_base_mut().read_values.insert(InstanceTarget::new(
               vec![ Some(Box::new(ast2.get_ast()))]
            ));
        }
    }
    pub fn visitVariableDeclaration(&self, ast: &mut VariableDeclaration) {
        ast.identifier_declaration_base
            .ast_base
            .modified_values
            .insert(InstanceTarget::new(
                vec![Some(Box::new(ast.get_ast()))]
            ));
    }

    pub fn visitAST(&self, ast: &mut AST) {
        let mut modified_values = BTreeSet::new();
        let mut read_values = BTreeSet::new();
        for child in ast.children().iter_mut() {
            self.visit(child.clone());
            modified_values = modified_values
                .union(&child.ast_base_mut().unwrap().modified_values)
                .cloned()
                .collect();
            read_values = read_values
                .union(&child.ast_base().unwrap().read_values)
                .cloned()
                .collect();
        }
        ast.ast_base_mut().unwrap().modified_values = modified_values;
        ast.ast_base_mut().unwrap().read_values = read_values;
    }
}
// class IndirectModificationDetector(FunctionVisitor)
struct IndirectModificationDetector {
    fixed_point_reached: bool,
}

impl FunctionVisitor for IndirectModificationDetector {}
impl AstVisitor for IndirectModificationDetector {
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
impl IndirectModificationDetector {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.fixed_point_reached = true
    pub fn new() -> Self {
        Self {
            fixed_point_reached: true,
        }
    }
    pub fn iterate_until_fixed_point(&mut self, ast: AST) {
        loop {
            self.visit(ast.clone());
            if self.fixed_point_reached {
                break;
            } else {
                self.fixed_point_reached = true;
            }
        }
    }

    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
        self.visitAST(ast.get_ast());
        if is_instance(&ast.func().unwrap(), ASTType::LocationExpr) {
            //for now no reference types -> only state could have been modified
            let mut fdef: AST = (*ast.func().unwrap().target().unwrap()).into();
            let mut ast = ast.get_ast();
            let rlen = ast.ast_base().unwrap().read_values.len();
            ast.ast_base_mut().unwrap().read_values = ast
                .ast_base_mut()
                .unwrap()
                .read_values
                .union(
                    &fdef
                        .ast_base()
                        .unwrap()
                        .read_values
                        .iter()
                        .filter_map(|v| {
                            if  v.target().is_some() && is_instance(&v.target().map(|t| *t).unwrap(),ASTType::StateVariableDeclaration)
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
            self.fixed_point_reached &= rlen == ast.ast_base().unwrap().read_values.len();

            //update modified values if any
            let mlen = ast.ast_base().unwrap().modified_values.len();
            for v in &fdef.ast_base().unwrap().modified_values {
                if is_instance(&v.target().map(|t| *t).unwrap(),ASTType::StateVariableDeclaration)
                {
                    ast.ast_base_mut()
                        .unwrap()
                        .modified_values
                        .insert(v.clone());
                }
            }
            self.fixed_point_reached &= mlen == ast.ast_base().unwrap().modified_values.len();
        }
    }
    pub fn visitAST(&mut self, mut ast: AST) {
        let mlen = ast.ast_base().unwrap().modified_values.len();
        let rlen = ast.ast_base().unwrap().read_values.len();
        for child in ast.children().iter_mut() {
            self.visit(child.clone());
            ast.ast_base_mut().unwrap().modified_values = ast
                .ast_base_mut()
                .unwrap()
                .modified_values
                .union(&child.ast_base().unwrap().modified_values)
                .cloned()
                .collect();
            ast.ast_base_mut().unwrap().read_values = ast
                .ast_base_mut()
                .unwrap()
                .read_values
                .union(&child.ast_base().unwrap().read_values)
                .cloned()
                .collect();
        }
        self.fixed_point_reached &= mlen == ast.ast_base().unwrap().modified_values.len();
        self.fixed_point_reached &= rlen == ast.ast_base().unwrap().read_values.len();
    }
}
// class EvalOrderUBChecker(AstVisitor)
struct EvalOrderUBChecker;

impl AstVisitor for EvalOrderUBChecker {
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
impl EvalOrderUBChecker {
    // @staticmethod
    pub fn visit_child_expressions(parent: AST, mut exprs: Vec<AST>) {
        if exprs.len() > 1 {
            let mut modset: BTreeSet<_> = exprs[0].ast_base().unwrap().modified_values.clone();
            for arg in &exprs[1..] {
                let modified_values=arg.ast_base().unwrap().modified_values.clone();
                let diffset: BTreeSet<_> = modset
                    .intersection(&modified_values)
                    .collect();

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

            for arg in &exprs {
                let modset: BTreeSet<_> = arg.ast_base().unwrap().modified_values.clone();
                let other_args: BTreeSet<_> = exprs
                    .iter()
                    .filter_map(|e| if e != arg { Some(e) } else { None })
                    .collect();
                for arg2 in &other_args {
                    let diffset: BTreeSet<_> = modset
                        .intersection(&arg2.ast_base().unwrap().read_values)
                        .collect();
                    if !diffset.is_empty() {
                        let setstr = format!(
                            r#"{{{}}}"#,
                            diffset
                                .iter()
                                .map(
                                    |
                                         it,
                                     | format!(
                                        "{:?}{}",
                                        it.target_key[0].as_ref().unwrap(),
                                        if let Some(member) = it.target_key[1].clone().map(|t|*t) {
                                            format!(".{:?}", member)
                                        } else {
                                            String::new()
                                        }
                                    )
                                )
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
        if is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction) {
            if ast.func().unwrap().has_shortcircuiting() {
                return;
            }
        }
        Self::visit_child_expressions(
            ast.get_ast(),
            ast.args().into_iter().map(|v| v.get_ast()).collect(),
        );
    }

    pub fn visitExpression(&mut self, mut ast: Expression) {
        Self::visit_child_expressions(ast.get_ast(), ast.children());
    }

    pub fn visitAssignmentStatement(&mut self, mut ast: AssignmentStatement) {
        Self::visit_child_expressions(ast.get_ast(), ast.children());
    }
}
