#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, AnnotatedTypeName, ArgType, Expression,
    ExpressionBaseProperty, FullArgsSpec, FullArgsSpecInit, IntoAST, Statement, TypeName,
    UserDefinedTypeName, AST,
};
use crate::global_defs::{array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars};
use crate::pointers::parent_setter::set_parents;
use crate::pointers::symbol_table::link_identifiers;
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::{RcCell, WeakCell};
use std::collections::BTreeMap;
use zkay_crypto::params::CryptoParams;
use zkay_derive::ASTVisitorBaseRefImpl;
// T = TypeVar("T")
// """

// :param ast
// :param with_types: (optional)
// :return: a deep copy of `ast`

// Only parents and identifiers are updated in the returned ast (e.g., inferred types are not preserved)
// """
pub fn deep_copy(
    ast: &ASTFlatten,
    with_types: bool,
    with_analysis: bool,
    global_vars: RcCell<GlobalVars>,
) -> Option<ASTFlatten> {
    // assert!(matches!(ast.to_ast(), AST(_)));
    let v = DeepCopyVisitor::new(with_types, with_analysis);
    let mut ast_copy = v.visit(ast);
    let parent = ast.ast_base_ref().unwrap().borrow().parent().clone();
    ast_copy
        .as_ref()
        .unwrap()
        .ast_base_ref()
        .unwrap()
        .borrow_mut()
        .parent = parent;
    set_parents(ast_copy.as_mut().unwrap());
    // let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
    link_identifiers(ast_copy.as_mut().unwrap(), global_vars.clone());
    ast_copy
}
// """
//     Copies over ast common ast attributes and reruns, parent setter, symbol table, side effect detector
// """
pub fn replace_expr(
    old_expr: &ASTFlatten,
    new_expr: &ASTFlatten,
    copy_type: bool,
    global_vars: RcCell<GlobalVars>,
) -> Option<ASTFlatten> {
    _replace_ast(old_expr, new_expr, global_vars);
    if copy_type {
        new_expr.ast_base_ref().unwrap().borrow_mut().annotated_type = old_expr
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type
            .clone();
    }
    Some(new_expr.clone())
}

pub fn _replace_ast(
    old_ast: &ASTFlatten,
    mut new_ast: &ASTFlatten,
    global_vars: RcCell<GlobalVars>,
) {
    new_ast.ast_base_ref().unwrap().borrow_mut().parent =
        old_ast.ast_base_ref().unwrap().borrow().parent().clone();
    DeepCopyVisitor::copy_ast_fields(old_ast, new_ast);
    if old_ast.ast_base_ref().unwrap().borrow().parent().is_some() {
        set_parents(new_ast);
        // let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        link_identifiers(new_ast, global_vars.clone());
    }
}

const setting_later: [&str; 42] = [
    //General fields
    "line",
    "column",
    "modified_values",
    "read_values",
    //Specialized fields
    "parent",
    "namespace",
    "names",
    "had_privacy_annotation",
    "annotated_type",
    "statement",
    "before_analysis",
    "after_analysis",
    "target",
    "instantiated_key",
    "function",
    "is_private",
    "rerand_using",
    "homomorphism",
    "evaluate_privately",
    "has_side_effects",
    "contains_inlined_function",
    "_size_in_bits",
    "signed",
    "_annotated_type",
    //Function stuff
    "called_functions",
    "is_recursive",
    "has_static_body",
    "can_be_private",
    "requires_verification",
    "requires_verification_when_external",
    "has_side_effects",
    "can_be_external",
    "is_payable",
    "original_body",
    "original_code",
    "return_var_decls",
    "used_homomorphisms",
    "used_crypto_backends",
    "pre_statements",
    "excluded_from_simulation",
    //For array children (ciphertext, key etc.)
    "expr",
    "value_type",
];
// class DeepCopyVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct DeepCopyVisitor {
    pub ast_visitor_base: AstVisitorBase,
    with_types: bool,
    with_analysis: bool,
}

impl AstVisitor for DeepCopyVisitor {
    type Return = Option<ASTFlatten>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::AnnotatedTypeName | ASTType::BuiltinFunction
        ) || matches!(ast, AST::TypeName(TypeName::UserDefinedTypeName(_)))
            || matches!(ast, AST::Expression(_))
            || matches!(ast, AST::Statement(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::AnnotatedTypeName => self.visitAnnotatedTypeName(ast),
            ASTType::BuiltinFunction => self.visitBuiltinFunction(ast),
            _ if matches!(
                ast.to_ast(),
                AST::TypeName(TypeName::UserDefinedTypeName(_))
            ) =>
            {
                self.visitUserDefinedTypeName(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),
            _ => eyre::bail!(String::new()),
        }
    }
    fn visit_children(&self, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        // let c = ast;
        let new_fields: Vec<_> = ast
            .get_attr()
            .iter()
            .map(|old_field| self.copy_field(old_field))
            .collect();

        // for k in ast.keys() {
        //     if !new_fields.contains(&k)
        //         && !self.setting_later.contains(&k)
        //     {
        //         // && !inspect.getfullargspec(c.__bases__[0].__init__).args[1..].contains(&k)
        //         panic!( "Not copying,{}", k);
        //     }
        // }
        let mut ast_copy = ast.from_fields(new_fields);
        Self::copy_ast_fields(ast, &ast_copy);
        Ok(Some(ast_copy))
    }
}
impl DeepCopyVisitor {
    pub fn new(with_types: bool, with_analysis: bool) -> Self {
        // super().__init__("node-or-children")
        // self.with_types = with_types
        // self.with_analysis = with_analysis
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            with_types,
            with_analysis,
        }
    }

    // @staticmethod
    pub fn copy_ast_fields(ast: &ASTFlatten, ast_copy: &ASTFlatten) {
        ast_copy.ast_base_ref().unwrap().borrow_mut().line =
            ast.ast_base_ref().unwrap().borrow().line;
        ast_copy.ast_base_ref().unwrap().borrow_mut().column =
            ast.ast_base_ref().unwrap().borrow().column;
        ast_copy
            .ast_base_ref()
            .unwrap()
            .borrow_mut()
            .modified_values = ast.ast_base_ref().unwrap().borrow().modified_values.clone();
        ast_copy.ast_base_ref().unwrap().borrow_mut().read_values =
            ast.ast_base_ref().unwrap().borrow().read_values.clone();
    }

    pub fn visitAnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ast_copy = self.visit_children(ast);
        ast_copy
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow_mut()
            .had_privacy_annotation = ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .had_privacy_annotation;
        ast_copy
    }

    pub fn visitUserDefinedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ast_copy = self.visit_children(ast);
        ast_copy
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow_mut()
            .target = ast.ast_base_ref().unwrap().borrow().target.clone();
        ast_copy
    }

    pub fn visitBuiltinFunction(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("==visitBuiltinFunction==========={ast:?}");
        let mut ast_copy = self.visit_children(ast);
        let is_private = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_builtin_function_ref()
            .unwrap()
            .is_private;
        let homomorphism = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_builtin_function_ref()
            .unwrap()
            .homomorphism
            .clone();
        if ast_copy
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .is_builtin_function()
        {
            ast_copy
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow_mut()
                .is_private = is_private;
            ast_copy
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow_mut()
                .homomorphism = homomorphism;
        } else if ast_copy.as_ref().unwrap().as_ref().unwrap().is_expression() {
            ast_copy
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_builtin_function_mut()
                .unwrap()
                .is_private = is_private;
            ast_copy
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_builtin_function_mut()
                .unwrap()
                .homomorphism = homomorphism;
        } else {
            panic!("==============else==========={ast:?}");
        }

        ast_copy
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ast_copy = self.visit_children(ast);
        if self.with_types
            && ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type
                .is_some()
        {
            ast_copy
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow_mut()
                .annotated_type = ast.ast_base_ref().unwrap().borrow().annotated_type.clone();
        }
        ast_copy
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .set_expression_base_mut_ref_property(|expr| {
                expr.evaluate_privately = ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .evaluate_privately();
            });

        ast_copy
    }

    pub fn visitStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ast_copy = self.visit_children(ast);
        if self.with_analysis {
            ast_copy
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .set_statement_base_mut_ref_property(|stmt| {
                    stmt.before_analysis = ast
                        .to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .statement_base_ref()
                        .unwrap()
                        .before_analysis
                        .clone();
                });
        }
        ast_copy
    }

    pub fn copy_field(&self, field: &ArgType) -> ArgType {
        match field {
            ArgType::Str(_)
            | ArgType::Int(_)
            | ArgType::Bool(_)
            | ArgType::CryptoParams(_)
            | ArgType::ASTFlattenWeak(_) => field.clone(),
            ArgType::Vec(v) => ArgType::Vec(v.iter().map(|a| self.copy_field(a)).collect()),
            ArgType::ASTFlatten(astf) => {
                ArgType::ASTFlatten(astf.as_ref().and_then(|_astf| self.visit(_astf)))
            }
        }
    }
}
