#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{AnnotatedTypeName, Expression, IntoAST, Statement, UserDefinedTypeName, AST};
use crate::pointers::parent_setter::set_parents;
use crate::pointers::symbol_table::link_identifiers;
use crate::visitor::visitor::AstVisitor;
use std::collections::BTreeMap;
use zkay_crypto::params::CryptoParams;
// T = TypeVar("T")

pub fn deep_copy(ast: Option<AST>, with_types: bool, with_analysis: bool) -> Option<AST>
// """

    // :param ast
    // :param with_types: (optional)
    // :return: a deep copy of `ast`

    // Only parents and identifiers are updated in the returned ast (e.g., inferred types are not preserved)
    // """
{
    // assert!(isinstance(ast, AST));
    let v = DeepCopyVisitor::new(with_types, with_analysis);
    let mut ast_copy = v.visit(ast.clone().unwrap());
    ast_copy.as_mut().unwrap().ast_base_mut().unwrap().parent =
        ast.unwrap().ast_base_ref().unwrap().parent.clone();
    set_parents(ast_copy.clone().unwrap());
    link_identifiers(ast_copy.as_ref().unwrap());
    ast_copy
}

pub fn replace_expr(
    old_expr: &Expression,
    new_expr: &mut Expression,
    copy_type: bool,
) -> Expression
// """
//     Copies over ast common ast attributes and reruns, parent setter, symbol table, side effect detector
// """
{
    _replace_ast(Some(old_expr.to_ast()), &mut (*new_expr).to_ast());
    if copy_type {
        // new_expr.annotated_type = old_expr.annotated_type;
    }
    new_expr.clone()
}

pub fn _replace_ast(old_ast: Option<AST>, mut new_ast: &mut AST) {
    new_ast.ast_base_mut().unwrap().parent =
        old_ast.as_ref().unwrap().ast_base_ref().unwrap().parent.clone();
    DeepCopyVisitor::copy_ast_fields(old_ast.clone(), &mut new_ast.clone());
    if old_ast.as_ref().unwrap().ast_base_ref().unwrap().parent.is_some() {
        set_parents(new_ast.clone());
        link_identifiers(new_ast);
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
pub struct DeepCopyVisitor {
    with_types: bool,
    with_analysis: bool,
}

impl AstVisitor for DeepCopyVisitor {
    type Return = Option<AST>;
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
    fn get_attr(&self, _name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, _ast: &AST) -> Self::Return {
        None
    }
}
impl DeepCopyVisitor {
    pub fn new(with_types: bool, with_analysis: bool) -> Self
// super().__init__("node-or-children")
        // self.with_types = with_types
        // self.with_analysis = with_analysis
    {
        Self {
            with_types,
            with_analysis,
        }
    }

    // @staticmethod
    pub fn copy_ast_fields(_ast: Option<AST>, _ast_copy: &mut AST) {
        // ast_copy.line = ast.line;
        // ast_copy.column = ast.column;
        // ast_copy.modified_values = ast.modified_values;
        // ast_copy.read_values = ast.read_values;
    }

    pub fn visitChildren(&self, _ast: Option<AST>) -> Option<AST> {
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
        //         assert!(false, "Not copying,{}", k);
        //     }
        // }
        // let mut ast_copy = c(new_fields);
        // self.copy_ast_fields(ast, ast_copy);
        // ast_copy
        None
    }

    pub fn visitAnnotatedTypeName(self, ast: Option<AST>) -> Option<AST> {
        let mut ast_copy = self.visitChildren(ast);
        // ast_copy.had_privacy_annotation = ast.had_privacy_annotation;
        ast_copy
    }

    pub fn visitUserDefinedTypeName(self, ast: UserDefinedTypeName) -> Option<AST> {
        let mut ast_copy = self.visitChildren(Some(ast.to_ast()));
        // ast_copy.target = ast.target;
        ast_copy
    }

    pub fn visitBuiltinFunction(self, ast: Option<AST>) -> Option<AST> {
        let mut ast_copy = self.visitChildren(ast);
        // ast_copy.is_private = ast.is_private;
        // ast_copy.homomorphism = ast.homomorphism;
        ast_copy
    }

    pub fn visitExpression(self, ast: Expression) -> Option<AST> {
        let mut ast_copy = self.visitChildren(Some(ast.to_ast()));
        if self.with_types && ast.annotated_type().is_some() {
            // ast_copy.annotated_type = ast.annotated_type.clone();
        }
        // ast_copy.evaluate_privately = ast.evaluate_privately();
        ast_copy
    }

    pub fn visitStatement(self, ast: Statement) -> Option<AST> {
        let mut ast_copy = self.visitChildren(Some(ast.to_ast()));
        if self.with_analysis {
            // ast_copy.before_analysis = ast.before_analysis();
        }
        ast_copy
    }

    pub fn copy_field(self, field: Option<AST>) -> Option<AST> {
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
        field.clone()
    }
}
