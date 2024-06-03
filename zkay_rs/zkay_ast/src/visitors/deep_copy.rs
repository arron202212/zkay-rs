#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    ASTBaseProperty, ASTFlatten, ASTType, AnnotatedTypeName, Expression, ExpressionBaseProperty,
    IntoAST, Statement, UserDefinedTypeName, AST,
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
pub fn deep_copy(ast: &ASTFlatten, with_types: bool, with_analysis: bool) -> Option<ASTFlatten> {
    // assert!(isinstance(ast,AST,ASTFlatten,));
    let v = DeepCopyVisitor::new(with_types, with_analysis);
    let mut ast_copy = v.visit(ast);
    ast_copy
        .as_mut()
        .unwrap()
        .ast_base_ref()
        .unwrap()
        .borrow_mut()
        .parent = ast.ast_base_ref().unwrap().borrow().parent().clone();
    set_parents(ast_copy.as_mut().unwrap());
    let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
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
) -> Option<ASTFlatten> {
    _replace_ast(old_expr, new_expr);
    if copy_type {
        // new_expr.annotated_type = old_expr.annotated_type;
    }
    Some(new_expr.clone())
}

pub fn _replace_ast(old_ast: &ASTFlatten, mut new_ast: &ASTFlatten) {
    new_ast.ast_base_ref().unwrap().borrow_mut().parent =
        old_ast.ast_base_ref().unwrap().borrow().parent().clone();
    DeepCopyVisitor::copy_ast_fields(old_ast, new_ast);
    if old_ast.ast_base_ref().unwrap().borrow().parent().is_some() {
        set_parents(new_ast);
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
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
    fn has_attr(&self, _ast: &AST) -> bool {
        false
    }
    fn get_attr(&self, _name: &ASTType, _ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        Ok(None)
    }
    fn visit_children(&self, _ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        // let c = ast;
        // let args_names = vec![]; //inspect.getfullargspec(c.__init__).args[1..];
        // let new_fields = BTreeMap::new();
        // for arg_name in args_names {
        //     let old_field = getattr(ast, arg_name);
        //     new_fields[arg_name] = self.copy_field(old_field);
        // }

        // for k in ast.keys() {
        //     if !new_fields.contains(&k)
        //         && !self.setting_later.contains(&k)
        //     {
        //         // && !inspect.getfullargspec(c.__bases__[0].__init__).args[1..].contains(&k)
        //         panic!( "Not copying,{}", k);
        //     }
        // }
        // let mut ast_copy = c(new_fields);
        // self.copy_ast_fields(ast, ast_copy);
        // ast_copy
        Ok(None)
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
    pub fn copy_ast_fields(_ast: &ASTFlatten, _ast_copy: &ASTFlatten) {
        // ast_copy.line = ast.line;
        // ast_copy.column = ast.column;
        // ast_copy.modified_values = ast.modified_values;
        // ast_copy.read_values = ast.read_values;
    }

    pub fn visitAnnotatedTypeName(
        self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // let mut ast_copy = self.visit_children(ast);
        // ast_copy.had_privacy_annotation = ast.had_privacy_annotation;
        self.visit_children(ast)
    }

    pub fn visitUserDefinedTypeName(
        self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // let mut ast_copy = self.visit_children(ast);
        // ast_copy.target = ast.target;
        self.visit_children(ast)
    }

    pub fn visitBuiltinFunction(
        self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // let mut ast_copy = self.visit_children(ast);
        // ast_copy.is_private = ast.is_private;
        // ast_copy.homomorphism = ast.homomorphism;
        self.visit_children(ast)
    }

    pub fn visitExpression(self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ast_copy = self.visit_children(ast);
        if self.with_types
            && ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .is_some()
        {
            // ast_copy.annotated_type = ast.annotated_type.clone();
        }
        // ast_copy.evaluate_privately = ast.evaluate_privately();
        ast_copy
    }

    pub fn visitStatement(self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut ast_copy = self.visit_children(ast);
        if self.with_analysis {
            // ast_copy.before_analysis = ast.before_analysis();
        }
        ast_copy
    }

    pub fn copy_field(self, field: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // if field.is_none() {
        //     None
        // } else if isinstance(field, str)
        //     || isinstance(field, int)
        //     || isinstance(field, bool)
        //     || isinstance(field, Enum)
        //     || isinstance(field, CryptoParams)
        // {
        //     field
        // } else if isinstance(field, list) {
        //     field.iter().map(|e| self.copy_field(e)).collect()
        // } else {
        //     self.visit(field)
        // }
        Ok(Some(field.clone()))
    }
}
