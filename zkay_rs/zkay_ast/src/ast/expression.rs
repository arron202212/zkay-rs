#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

#[cfg(windows)]
pub const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";
// use  typing import List, Dict, Union, Optional, Callable, Set, TypeVar;
use crate::analysis::partition_state::PartitionState;
use crate::ast::{
    annotated_type_name::AnnotatedTypeName,
    identifier::{HybridArgumentIdf, Identifier, IdentifierBase},
    is_instance, is_instances,
    statement::IndentBlock,
    statement::{
        AssignmentStatement, AssignmentStatementBase, AssignmentStatementBaseProperty,
        StatementBaseProperty,
    },
    type_name::{
        ArrayBaseProperty, BooleanLiteralType, CombinedPrivacyUnion, DummyAnnotation, ExprUnion,
        NumberLiteralType, NumberLiteralTypeUnion, TypeName,
    },
    ASTBase, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTChildrenCallBack,
    ASTFlatten, ASTFlattenWeak, ASTType, ArgType, ChildListBuilder, DeepClone, FullArgsSpec,
    FullArgsSpecInit, Immutable, IntoAST, AST,
};
use crate::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use crate::global_defs::{array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars};
use crate::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use enum_dispatch::enum_dispatch;
use ethnum::{i256, int, u256, uint, AsI256, AsU256, I256, U256};
use eyre::{eyre, Result};
use lazy_static::lazy_static;
use rccell::{RcCell, WeakCell};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex},
    // borrow::Borrow,
};
use strum_macros::{EnumIs, EnumTryAs};
use zkay_config::{
    config::{ConstructorOrFunctionDefinitionAttr, CFG},
    config_user::UserConfig,
    with_context_block, zk_print,
};
use zkay_derive::{
    impl_trait, impl_traits, ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind,
    ASTVisitorBaseRefImpl, EnumDispatchWithDeepClone, EnumDispatchWithFields, ExpressionASTypeImpl,
    ImplBaseTrait,
};
use zkay_transaction_crypto_params::params::CryptoParams;
use zkay_utils::progress_printer::warn_print;
use zkp_u256::{Zero, U256 as ZU256};
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum Expression {
    BuiltinFunction(BuiltinFunction),
    FunctionCallExpr(FunctionCallExpr),
    PrimitiveCastExpr(PrimitiveCastExpr),
    LiteralExpr(LiteralExpr),
    TupleOrLocationExpr(TupleOrLocationExpr),
    MeExpr(MeExpr),
    AllExpr(AllExpr),
    ReclassifyExpr(ReclassifyExpr),
    DummyAnnotation(DummyAnnotation),
}

impl Expression {
    pub fn all_expr() -> Self {
        Expression::AllExpr(AllExpr::new())
    }
    pub fn me_expr(statement: Option<ASTFlatten>) -> Self {
        let mut me_expr = MeExpr::new();
        me_expr.expression_base.statement = statement.map(|s| s.downgrade());
        Expression::MeExpr(me_expr)
    }
    pub fn explicitly_converted(&self, expected: &ASTFlatten) -> ASTFlatten {
        let mut ret;
        let bool_type = RcCell::new(TypeName::bool_type()).into();
        if expected == &bool_type && !self.instanceof_data_type(&bool_type) {
            ret = Some(FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(BuiltinFunction::new("!="))).into(),
                [
                    self.clone(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                        NumberLiteralExpr::new(0, false),
                    )),
                ]
                .into_iter()
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect(),
                None,
                None,
            ));
        } else if expected.to_ast().try_as_type_name().unwrap().is_numeric()
            && self.instanceof_data_type(&bool_type)
        {
            ret = Some(FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(BuiltinFunction::new("ite"))).into(),
                [
                    self.clone(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                        NumberLiteralExpr::new(1, false),
                    )),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                        NumberLiteralExpr::new(0, false),
                    )),
                ]
                .into_iter()
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect(),
                None,
                None,
            ));
        } else {
            let t = self
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .clone();

            if &t == expected {
                return RcCell::new(self.clone()).into();
            }

            // Explicit casts
            assert!(
                is_instance(&t, ASTType::NumberTypeNameBase)
                    && is_instances(
                        expected,
                        vec![
                            ASTType::NumberTypeNameBase,
                            ASTType::AddressTypeName,
                            ASTType::AddressPayableTypeName,
                            ASTType::EnumTypeName,
                        ],
                    )
                    || is_instance(&t, ASTType::AddressTypeName)
                        && is_instance(expected, ASTType::NumberTypeNameBase)
                    || is_instance(&t, ASTType::AddressPayableTypeName)
                        && is_instances(
                            expected,
                            vec![ASTType::NumberTypeNameBase, ASTType::AddressTypeName],
                        )
                    || is_instance(&t, ASTType::EnumTypeName)
                        && is_instance(expected, ASTType::NumberTypeNameBase)
            );
            return Expression::PrimitiveCastExpr(PrimitiveCastExpr::new(
                expected.clone(),
                RcCell::new(self.clone()).into(),
                false,
            ))
            .as_type(&expected.clone().into());
        }
        assert!(ret.is_some());
        let mut ret = ret.unwrap();
        ret.ast_base_mut_ref().borrow_mut().annotated_type =
            Some(RcCell::new(AnnotatedTypeName::new(
                Some(expected.clone()),
                self.annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation
                    .clone(),
                self.annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism
                    .clone(),
            )));
        RcCell::new(ret).into()
    }

    pub fn privacy_annotation_label(&self) -> Option<ASTFlatten> {
        if is_instance(self, ASTType::IdentifierExpr) {
            let ie = self
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap();
            // println!(
            //     "=====target====ptr===={:?}",
            //     ie.ast_base_ref()
            //         .borrow()
            //         .target
            //         .clone()
            //         .unwrap()
            //         .ptr_string()
            // );
            let target = ie
                .ast_base_ref()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap();
            // println!("==privacy_annotation_label===target===instantiated_key==={}=====", target);
            if is_instance(&target, ASTType::Mapping) {
                return target
                    .to_ast()
                    .try_as_type_name_ref()
                    .unwrap()
                    .try_as_mapping_ref()
                    .unwrap()
                    .instantiated_key
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label();
            }

            return target
                .ast_base_ref()
                .unwrap()
                .borrow()
                .idf()
                .map(|f| f.clone_inner().into());
        }

        if self.is_all_expr() || self.is_me_expr() {
            Some(RcCell::new(self.clone()).into())
        } else {
            None
        }
    }
    pub fn instanceof_data_type(&self, expected: &ASTFlatten) -> bool {
        let res = self
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .implicitly_convertible_to(expected);
        // if !res {
        //     // println!(
        //     //     "=====instanceof_data_type==============={:?}====,============={:?}",
        //     //    self
        //     // .annotated_type()
        //     // .as_ref()
        //     // .unwrap()
        //     // .borrow()
        //     // .type_name,
        //     //     expected,
        //     // );
        // }
        res
    }
    pub fn unop(&self, op: String) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            RcCell::new(Expression::BuiltinFunction(BuiltinFunction::new(&op))).into(),
            vec![RcCell::new(self.clone()).into()],
            None,
            None,
        ))
    }

    pub fn binop(&self, op: String, rhs: Expression) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            RcCell::new(Expression::BuiltinFunction(BuiltinFunction::new(&op))).into(),
            [self.clone(), rhs]
                .into_iter()
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect(),
            None,
            None,
        ))
    }

    pub fn ite(&self, e_true: Expression, e_false: Expression) -> FunctionCallExpr {
        let mut bf = BuiltinFunction::new("ite");
        bf.is_private = self
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private();
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            RcCell::new(Expression::BuiltinFunction(bf)).into(),
            [self.clone(), e_true, e_false]
                .into_iter()
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect(),
            None,
            None,
        ))
    }

    // """
    // :param expected:
    // :return: true, false, or "make-private"
    // """
    pub fn instance_of(&self, expected: &RcCell<AnnotatedTypeName>) -> String {
        // assert! (isinstance(expected, AnnotatedTypeName))

        let actual = self.annotated_type();

        if !self.instanceof_data_type(expected.borrow().type_name.as_ref().unwrap()) {
            return String::from("false");
        }

        // check privacy type and homomorphism
        let combined_label = actual
            .as_ref()
            .unwrap()
            .borrow()
            .combined_privacy(self.analysis(), expected);
        if let Some(combined_label) = combined_label {
            if let CombinedPrivacyUnion::Vec(combined_label) = combined_label {
                assert!(
                    matches!(
                        self.annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .type_name
                            .as_ref()
                            .unwrap()
                            .to_ast()
                            .try_as_type_name_ref()
                            .unwrap(),
                        TypeName::TupleType(_)
                    ) && !matches!(
                        self,
                        Expression::TupleOrLocationExpr(TupleOrLocationExpr::TupleExpr(_))
                    )
                );

                (combined_label
                    == self
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .try_as_tuple_type_ref()
                        .unwrap()
                        .types
                        .iter()
                        .map(|t| CombinedPrivacyUnion::AST(t.borrow().privacy_annotation.clone()))
                        .collect::<Vec<_>>())
                .to_string()
            } else if combined_label
                .clone()
                .as_expression()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
                == actual
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label()
            {
                String::from("true")
            } else {
                String::from("make-private")
            }
        } else {
            String::from("false")
        }
    }

    pub fn analysis(&self) -> Option<PartitionState<AST>> {
        self.statement().as_ref().and_then(|statement| {
            statement
                .clone()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .statement_base_ref()
                .unwrap()
                .before_analysis()
                .clone()
        })
    }
}
#[enum_dispatch]
pub trait ExpressionBaseRef: ASTBaseRef {
    fn expression_base_ref(&self) -> &ExpressionBase;
}
pub trait ExpressionBaseProperty {
    fn statement(&self) -> &Option<ASTFlattenWeak>;
    fn evaluate_privately(&self) -> bool;
}
impl<T: ExpressionBaseRef> ExpressionBaseProperty for T {
    fn statement(&self) -> &Option<ASTFlattenWeak> {
        &self.expression_base_ref().statement
    }
    fn evaluate_privately(&self) -> bool {
        self.expression_base_ref().evaluate_privately
    }
}

#[enum_dispatch]
pub trait ExpressionASType {
    fn as_type(&self, t: &ASTFlatten) -> ASTFlatten;
}
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionBase {
    pub ast_base: RcCell<ASTBase>,
    pub statement: Option<ASTFlattenWeak>,
    pub evaluate_privately: bool,
}
impl DeepClone for ExpressionBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            ..self.clone()
        }
    }
}
impl FullArgsSpec for ExpressionBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.annotated_type()
                .as_ref()
                .map(|tn| ASTFlatten::from(tn.clone_inner())),
        )]
    }
}

impl FullArgsSpecInit for ExpressionBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ExpressionBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}
impl ExpressionBase {
    pub fn new(
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
    ) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(annotated_type, idf, None)),
            statement: None,
            evaluate_privately: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum LiteralUnion {
    Bool(bool),
    Number(i32),
}
pub fn builtin_op_fct(op: &str, args: Vec<String>) -> LiteralUnion {
    let parse_int = |arg: &String| arg.parse::<i32>().unwrap();
    let parse_bool = |arg: &String| arg == "true";
    match op {
        "+" => LiteralUnion::Number(parse_int(&args[0]) + parse_int(&args[1])),
        "-" => LiteralUnion::Number(parse_int(&args[0]) - parse_int(&args[1])),
        "**" => LiteralUnion::Number(parse_int(&args[0]).pow(parse_int(&args[1]) as u32)),
        "*" => LiteralUnion::Number(parse_int(&args[0]) * parse_int(&args[1])),
        "/" => LiteralUnion::Number(parse_int(&args[0]) / parse_int(&args[1])),
        "%" => LiteralUnion::Number(parse_int(&args[0]) % parse_int(&args[1])),
        "sign+" => LiteralUnion::Number(parse_int(&args[0])),
        "sign-" => LiteralUnion::Number(-parse_int(&args[0])),
        "<<" => LiteralUnion::Number(parse_int(&args[0]) << parse_int(&args[1])),
        ">>" => LiteralUnion::Number(parse_int(&args[0]) >> parse_int(&args[1])),
        "|" => LiteralUnion::Number(parse_int(&args[0]) | parse_int(&args[1])),
        "&" => LiteralUnion::Number(parse_int(&args[0]) & parse_int(&args[1])),
        "^" => LiteralUnion::Number(parse_int(&args[0]) ^ parse_int(&args[1])),
        "~" => LiteralUnion::Number(!parse_int(&args[0])),
        "<" => LiteralUnion::Bool(parse_int(&args[0]) < parse_int(&args[1])),
        ">" => LiteralUnion::Bool(parse_int(&args[0]) > parse_int(&args[1])),
        "<=" => LiteralUnion::Bool(parse_int(&args[0]) <= parse_int(&args[1])),
        ">=" => LiteralUnion::Bool(parse_int(&args[0]) >= parse_int(&args[1])),
        "==" => LiteralUnion::Bool(parse_int(&args[0]) == parse_int(&args[1])),
        "!=" => LiteralUnion::Bool(parse_int(&args[0]) != parse_int(&args[1])),
        "&&" => LiteralUnion::Bool(parse_bool(&args[0]) && parse_bool(&args[1])),
        "||" => LiteralUnion::Bool(parse_bool(&args[0]) || parse_bool(&args[1])),
        "!" => LiteralUnion::Bool(!(parse_bool(&args[0]))),
        "ite" => LiteralUnion::Number(if args[0] != "0" {
            parse_int(&args[1])
        } else {
            parse_int(&args[2])
        }),
        "parenthesis" => LiteralUnion::Number(parse_int(&args[0])),
        _ => LiteralUnion::Bool(false),
    }
}

// assert builtin_op_fct.keys() == BUILTIN_FUNCTIONS.keys()
const BINARY_OP: &str = "{{}} {op} {{}}";
lazy_static! {
    pub static ref BUILTIN_FUNCTIONS: HashMap<String, String> = {
        let m: HashMap<&'static str, &'static str> = HashMap::from([
            ("**", BINARY_OP),
            ("*", BINARY_OP),
            ("/", BINARY_OP),
            ("%", BINARY_OP),
            ("+", BINARY_OP),
            ("-", BINARY_OP),
            ("sign+", "+{}"),
            ("sign-", "-{}"),
            ("<", BINARY_OP),
            (">", BINARY_OP),
            ("<=", BINARY_OP),
            (">=", BINARY_OP),
            ("==", BINARY_OP),
            ("!=", BINARY_OP),
            ("&&", BINARY_OP),
            ("||", BINARY_OP),
            ("!", "!{}"),
            ("|", BINARY_OP),
            ("&", BINARY_OP),
            ("^", BINARY_OP),
            ("~", "~{}"),
            ("<<", BINARY_OP),
            (">>", BINARY_OP),
            ("parenthesis", "({})"),
            ("ite", "{}?{}:{}"),
        ]);
        m.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    };
    pub static ref ARITHMETIC: HashMap<String, String> = {
        let m: HashMap<&'static str, &'static str> = HashMap::from([
            ("**", "arithmetic"),
            ("*", "arithmetic"),
            ("/", "arithmetic"),
            ("%", "arithmetic"),
            ("+", "arithmetic"),
            ("-", "arithmetic"),
            ("sign+", "arithmetic"),
            ("sign-", "arithmetic"),
            ("<", "comparison"),
            (">", "comparison"),
            ("<=", "comparison"),
            (">=", "comparison"),
            ("==", "equality"),
            ("!=", "equality"),
            ("&&", "boolean_operations"),
            ("||", "boolean_operations"),
            ("!", "boolean_operations"),
            ("|", "bitwise_operations"),
            ("&", "bitwise_operations"),
            ("^", "bitwise_operations"),
            ("~", "bitwise_operations"),
            ("<<", "shift_operations"),
            (">>", "shift_operations"),
            ("parenthesis", "({})"),
            ("ite", "{}?{}:{}"),
        ]);
        m.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    };
}

//     """
//     Just a named tuple that describes an available homomorphic operation.
//     """
//     op: str
//     homomorphism: Homomorphism
//     public_args: List[bool]
//     """
//     A list that describes what arguments are required to be public to be able to use this homomorphic function.
//     """
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HomomorphicBuiltin {
    pub op: String,
    pub homomorphism: String,
    pub public_args: Vec<bool>,
}
impl FullArgsSpec for HomomorphicBuiltin {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Str(Some(self.op.clone())),
            ArgType::Str(Some(self.homomorphism.clone())),
            ArgType::Vec(
                self.public_args
                    .iter()
                    .map(|&pa| ArgType::Bool(pa))
                    .collect(),
            ),
        ]
    }
}

impl FullArgsSpecInit for HomomorphicBuiltin {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        HomomorphicBuiltin::new(
            fields[0].clone().try_as_str().flatten().unwrap().as_str(),
            fields[1].clone().try_as_str().flatten().unwrap(),
            fields[2]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_bool().unwrap())
                .collect(),
        )
    }
}
impl HomomorphicBuiltin {
    pub fn new(op: &str, homomorphism: String, public_args: Vec<bool>) -> Self {
        Self {
            op: op.to_string(),
            homomorphism,
            public_args,
        }
    }
}

lazy_static! {
    static ref HOMOMORPHIC_BUILTIN_FUNCTIONS: Vec<HomomorphicBuiltin> = {
        let homomorphic_builtin_functions_internal = vec![
            HomomorphicBuiltin::new("sign+", String::from("ADDITIVE"), vec![false]),
            HomomorphicBuiltin::new("sign-", String::from("ADDITIVE"), vec![false]),
            HomomorphicBuiltin::new("+", String::from("ADDITIVE"), vec![false, false]),
            HomomorphicBuiltin::new("-", String::from("ADDITIVE"), vec![false, false]),
            HomomorphicBuiltin::new("*", String::from("ADDITIVE"), vec![true, false]),
            HomomorphicBuiltin::new("*", String::from("ADDITIVE"), vec![false, true]),
        ];
        for __hom in &homomorphic_builtin_functions_internal {
            assert!(
                BUILTIN_FUNCTIONS.contains_key(&__hom.op)
                    && __hom.homomorphism != Homomorphism::non_homomorphic()
            );
            let op_arity = BUILTIN_FUNCTIONS[&__hom.op].matches("{}").count();
            assert!(op_arity == __hom.public_args.len());
        }
        homomorphic_builtin_functions_internal
    };
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct BuiltinFunction {
    pub expression_base: ExpressionBase,
    pub op: String,
    pub is_private: bool,
    pub homomorphism: String,
    pub rerand_using: Option<RcCell<AST>>,
}
impl DeepClone for BuiltinFunction {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
            rerand_using: self.rerand_using.clone_inner(),
            ..self.clone()
        }
    }
}
impl IntoAST for BuiltinFunction {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::BuiltinFunction(self))
    }
}
impl FullArgsSpec for BuiltinFunction {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.op.clone()))]
    }
}

impl FullArgsSpecInit for BuiltinFunction {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        BuiltinFunction::new(fields[0].clone().try_as_str().flatten().unwrap().as_str())
    }
}
impl BuiltinFunction {
    pub fn new(op: &str) -> Self {
        let op = op.to_string();
        assert!(
            BUILTIN_FUNCTIONS.get(&op).is_some(),
            "{op} is not a known built-in function"
        );
        Self {
            expression_base: ExpressionBase::new(None, None),
            op,
            is_private: false,
            homomorphism: Homomorphism::non_homomorphic(),
            rerand_using: None,
        }
    }

    pub fn format_string(&self, args: &[String]) -> String {
        let op = self.op.as_str();

        match op {
            "sign+" => format!("+{}", args[0]),
            "sign-" => format!("-{}", args[0]),
            "!" => format!("!{}", args[0]),
            "~" => format!("~{}", args[0]),
            "parenthesis" => format!("({})", args[0]),
            "ite" => {
                let (cond, then, else_then) = (args[0].clone(), args[1].clone(), args[2].clone());
                format!("{cond} ? {then} : {else_then}")
            }
            _ => format!("{} {op} {}", args[0], args[1]),
        }
    }
    pub fn op_func(&self, args: Vec<String>) -> LiteralUnion {
        builtin_op_fct(&self.op, args)
    }

    pub fn is_arithmetic(&self) -> bool {
        ARITHMETIC.get(&self.op) == Some(&String::from("arithmetic"))
    }

    pub fn is_neg_sign(&self) -> bool {
        &self.op == "sign-"
    }

    pub fn is_comp(&self) -> bool {
        ARITHMETIC.get(&self.op) == Some(&String::from("comparison"))
    }

    pub fn is_eq(&self) -> bool {
        ARITHMETIC.get(&self.op) == Some(&String::from("equality"))
    }

    pub fn is_bop(&self) -> bool {
        ARITHMETIC.get(&self.op) == Some(&String::from("boolean_operations"))
    }

    pub fn is_bitop(&self) -> bool {
        ARITHMETIC.get(&self.op) == Some(&String::from("bitwise_operations"))
    }

    pub fn is_shiftop(&self) -> bool {
        ARITHMETIC.get(&self.op) == Some(&String::from("shift_operations"))
    }

    pub fn is_parenthesis(&self) -> bool {
        &self.op == "parenthesis"
    }

    pub fn is_ite(&self) -> bool {
        &self.op == "ite"
    }

    pub fn has_shortcircuiting(&self) -> bool {
        self.is_ite() || &self.op == "&&" || &self.op == "||"
    }

    pub fn arity(&self) -> i32 {
        BUILTIN_FUNCTIONS[&self.op].matches("{}").count() as i32
    }
    pub fn input_types(&self) -> Vec<Option<ASTFlatten>> {
        // :return: None if the type is generic
        let t = if self.is_arithmetic() || self.is_comp() {
            Some(RcCell::new(TypeName::number_type()).into())
        } else if self.is_bop() {
            Some(RcCell::new(TypeName::bool_type()).into())
        } else if self.is_bitop() || self.is_shiftop() {
            Some(RcCell::new(TypeName::number_type()).into())
        } else {
            // eq, parenthesis, ite
            None
        };
        // println!(
        //     "====input_types====={},{},{},{}============{:?},==============={:?}",self.is_arithmetic() , self.is_comp() , self.is_bitop() , self.is_shiftop(),
        //     t,
        //     self.arity()
        // );
        vec![t; self.arity() as usize]
    }
    pub fn output_type(&self) -> Option<TypeName> {
        // :return: None if the type is generic
        if self.is_arithmetic() {
            Some(TypeName::number_type())
        } else if self.is_comp() || self.is_bop() || self.is_eq() {
            Some(TypeName::bool_type())
        } else if self.is_bitop() || self.is_shiftop() {
            Some(TypeName::number_type())
        } else {
            // parenthesis, ite
            None
        }
    }
    // :return: true if operation itself can be run inside a circuit \
    //          for equality and ite it must be checked separately whether the arguments are also supported inside circuits
    pub fn can_be_private(&self) -> bool {
        &self.op != "**"
    }

    // """
    // Finds a homomorphic builtin that performs the correct operation and which can be applied
    // on the arguments, if any exist.

    // :return: A HomomorphicBuiltinFunction that can be used to query the required input types and
    //          the resulting output type of the homomorphic operation, or None
    // """
    pub fn select_homomorphic_overload(
        &self,
        args: &[ASTFlatten],
        analysis: Option<PartitionState<AST>>,
    ) -> Option<HomomorphicBuiltinFunction> {
        // The first inaccessible (not @all, not @me) determines the output type
        // self.op and the public arguments determine which homomorphic builtin is selected
        // We may want to rethink this in the future if we also implement other homomorphisms (e.g. multiplicative)

        let arg_types: Vec<_> = args
            .iter()
            .map(|x| {
                x.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .clone()
            })
            .collect();
        let inaccessible_arg_types: Vec<_> = arg_types
            .iter()
            .filter(|x| !x.as_ref().unwrap().borrow().is_accessible(&analysis))
            .collect();
        // Else we would not have selected a homomorphic operation
        // raise ValueError("Cannot select proper homomorphic function if all arguments are public or @me-private")
        assert!(
            !inaccessible_arg_types.is_empty(),
            "Cannot select proper homomorphic function if all arguments are public or @me-private"
        );
        let elem_type = arg_types
            .iter()
            .map(|a| a.as_ref().unwrap().borrow().type_name.clone().unwrap())
            .reduce(|l, r| {
                l.to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .combined_type(&r, true)
                    .unwrap()
            });
        let base_type = AnnotatedTypeName::new(
            elem_type,
            inaccessible_arg_types[0]
                .as_ref()
                .unwrap()
                .borrow()
                .privacy_annotation
                .clone(),
            Homomorphism::non_homomorphic(),
        );
        let public_args: Vec<_> = arg_types
            .iter()
            .map(|a| a.as_ref().unwrap().borrow().is_public())
            .collect();

        for hom in HOMOMORPHIC_BUILTIN_FUNCTIONS.iter() {
            // Can have more public arguments, but not fewer (hom.public_args[i] implies public_args[i])
            if self.op == hom.op
                && public_args
                    .iter()
                    .zip(&hom.public_args)
                    .all(|(&a, &h)| !h || a)
            {
                let target_type = base_type.with_homomorphism(hom.homomorphism.clone());
                return Some(HomomorphicBuiltinFunction::new(
                    target_type,
                    hom.public_args.clone(),
                ));
            }
        }
        if self.op == "*"
            && !args[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_accessible(&analysis)
            && !args[1]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_accessible(&analysis)
            && (is_instance(&args[0], ASTType::ReclassifyExpr)
                ^ is_instance(&args[1], ASTType::ReclassifyExpr))
        {
            // special case: private scalar multiplication using additive homomorphism
            let target_type = base_type.with_homomorphism(Homomorphism::additive());
            {
                Some(HomomorphicBuiltinFunction::new(
                    target_type,
                    vec![false, false],
                ))
            }
        } else {
            None
        }
    }
}

//     Describes the required input types and the resulting output type of a homomorphic execution of a BuiltinFunction.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HomomorphicBuiltinFunction {
    pub target_type: RcCell<AnnotatedTypeName>,
    pub public_args: Vec<bool>,
}
impl DeepClone for HomomorphicBuiltinFunction {
    fn clone_inner(&self) -> Self {
        Self {
            target_type: self.target_type.clone_inner(),
            public_args: self.public_args.clone(),
        }
    }
}
impl FullArgsSpec for HomomorphicBuiltinFunction {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.target_type.clone_inner()))),
            ArgType::Vec(
                self.public_args
                    .iter()
                    .map(|&pa| ArgType::Bool(pa))
                    .collect(),
            ),
        ]
    }
}

impl FullArgsSpecInit for HomomorphicBuiltinFunction {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        HomomorphicBuiltinFunction::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name())
                .unwrap(),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_bool().unwrap())
                .collect(),
        )
    }
}
impl HomomorphicBuiltinFunction {
    pub fn new(target_type: RcCell<AnnotatedTypeName>, public_args: Vec<bool>) -> Self {
        Self {
            target_type,
            public_args,
        }
    }
    pub fn input_types(&self) -> Vec<RcCell<AnnotatedTypeName>> {
        // println!("===HomomorphicBuiltinFunction============input_types===============");
        let public_type = AnnotatedTypeName::all(
            self.target_type
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .to_ast()
                .try_as_type_name()
                .unwrap(),
        );
        // same data type, but @all

        //  [public_type if public else self.target_type for public in self.public_args]
        self.public_args
            .iter()
            .map(|&public| {
                if public {
                    public_type.clone()
                } else {
                    self.target_type.clone()
                }
            })
            .collect::<Vec<_>>()
    }
    pub fn output_type(&self) -> RcCell<AnnotatedTypeName> {
        self.target_type.clone()
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    FunctionCallExprBaseRef,
    FunctionCallExprBaseMutRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum FunctionCallExpr {
    FunctionCallExpr(FunctionCallExprBase),
    NewExpr(NewExpr),
}

impl FunctionCallExpr {
    pub fn is_cast(&self) -> bool {
        // isinstance(self.func, LocationExpr) && isinstance(self.func.target, (ContractDefinition, EnumDefinition))
        // println!(
        //     "=={:?}======is_cast==================={:?}",
        //     self.get_ast_type(),
        //     self.func()
        // );
        is_instance(self.func(), ASTType::LocationExprBase)
            && is_instances(
                &self
                    .func()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
                vec![ASTType::ContractDefinition, ASTType::EnumDefinition],
            )
    }
}

#[enum_dispatch]
pub trait FunctionCallExprBaseRef: ExpressionBaseRef {
    fn function_call_expr_base_ref(&self) -> &FunctionCallExprBase;
}
pub trait FunctionCallExprBaseProperty {
    fn func(&self) -> &ASTFlatten;
    fn args(&self) -> &Vec<ASTFlatten>;
    fn sec_start_offset(&self) -> &Option<i32>;
    fn public_key(&self) -> &Option<RcCell<HybridArgumentIdf>>;
}
impl<T: FunctionCallExprBaseRef> FunctionCallExprBaseProperty for T {
    fn func(&self) -> &ASTFlatten {
        &self.function_call_expr_base_ref().func
    }
    fn args(&self) -> &Vec<ASTFlatten> {
        &self.function_call_expr_base_ref().args
    }
    fn sec_start_offset(&self) -> &Option<i32> {
        &self.function_call_expr_base_ref().sec_start_offset
    }
    fn public_key(&self) -> &Option<RcCell<HybridArgumentIdf>> {
        &self.function_call_expr_base_ref().public_key
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ImplBaseTrait,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct FunctionCallExprBase {
    pub expression_base: ExpressionBase,
    pub func: ASTFlatten,
    pub args: Vec<ASTFlatten>,
    pub sec_start_offset: Option<i32>,
    pub public_key: Option<RcCell<HybridArgumentIdf>>,
}
impl DeepClone for FunctionCallExprBase {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
            func: self.func.clone_inner(),
            args: self.args.clone_inner(),
            sec_start_offset: self.sec_start_offset.clone(),
            public_key: self.public_key.clone_inner(),
        }
    }
}
impl IntoAST for FunctionCallExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(
            FunctionCallExpr::FunctionCallExpr(self),
        ))
    }
}
impl FullArgsSpec for FunctionCallExprBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.func.clone_inner())),
            ArgType::Vec(
                self.args
                    .iter()
                    .map(|pa| ArgType::ASTFlatten(Some(pa.clone_inner())))
                    .collect(),
            ),
            ArgType::Int(self.sec_start_offset),
            ArgType::ASTFlatten(
                self.annotated_type()
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
        ]
    }
}

impl FullArgsSpecInit for FunctionCallExprBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        FunctionCallExprBase::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_ast_flatten().flatten().unwrap())
                .collect(),
            fields[2].clone().try_as_int().unwrap(),
            fields[3]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
        )
    }
}
impl FunctionCallExprBase {
    pub fn new(
        func: ASTFlatten,
        args: Vec<ASTFlatten>,
        sec_start_offset: Option<i32>,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
    ) -> Self {
        // args.iter().for_each(|arg|{print!("{:?},{},",arg.get_ast_type(),arg);});
        // println!("=====func====={:?}========{}====",func.get_ast_type(),func);
        Self {
            expression_base: ExpressionBase::new(annotated_type, None),
            func,
            args,
            sec_start_offset,
            public_key: None,
        }
    }
}

impl ASTChildren for FunctionCallExprBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.func.clone());
        self.args.iter().for_each(|arg| {
            cb.add_child(arg.clone());
        });
    }
}

impl ASTChildrenCallBack for FunctionCallExprBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.func.assign(f(&self.func).as_ref().unwrap());
        self.args
            .iter()
            .for_each(|arg| arg.assign(f(arg).as_ref().unwrap()));
    }
}

#[impl_traits(FunctionCallExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct NewExpr {
    pub function_call_expr_base: FunctionCallExprBase,
}
impl DeepClone for NewExpr {
    fn clone_inner(&self) -> Self {
        Self {
            function_call_expr_base: self.function_call_expr_base.clone_inner(),
        }
    }
}
impl IntoAST for NewExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::NewExpr(
            self,
        )))
    }
}
impl FullArgsSpec for NewExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.annotated_type()
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::Vec(
                self.args()
                    .iter()
                    .map(|pa| ArgType::ASTFlatten(Some(pa.clone_inner())))
                    .collect(),
            ),
        ]
    }
}

impl FullArgsSpecInit for NewExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        NewExpr::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_ast_flatten().flatten().unwrap())
                .collect(),
        )
    }
}
impl NewExpr {
    pub fn new(annotated_type: Option<RcCell<AnnotatedTypeName>>, args: Vec<ASTFlatten>) -> Self {
        // assert not isinstance(annotated_type, ElementaryTypeName)
        Self {
            function_call_expr_base: FunctionCallExprBase::new(
                RcCell::new(
                    IdentifierExpr::new(
                        IdentifierExprUnion::String(format!(
                            "new {}",
                            annotated_type.as_ref().unwrap().borrow().code()
                        )),
                        None,
                    )
                    .into_ast(),
                )
                .into(),
                args,
                None,
                annotated_type,
            ),
        }
    }
}
impl ASTChildren for NewExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.annotated_type().clone().unwrap().into());
        self.function_call_expr_base.args.iter().for_each(|arg| {
            cb.add_child(arg.clone());
        });
    }
}
impl ASTChildrenCallBack for NewExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        *self
            .ast_base_ref()
            .borrow()
            .annotated_type
            .as_ref()
            .unwrap()
            .borrow_mut() = f(&self.annotated_type().clone().unwrap().into())
            .and_then(|at| at.try_as_annotated_type_name())
            .unwrap()
            .borrow()
            .clone();
        self.function_call_expr_base.args.iter().for_each(|arg| {
            arg.assign(f(arg).as_ref().unwrap());
        });
    }
}

#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct PrimitiveCastExpr {
    pub expression_base: ExpressionBase,
    pub elem_type: ASTFlatten,
    pub expr: ASTFlatten,
    pub is_implicit: bool,
}
impl DeepClone for PrimitiveCastExpr {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
            elem_type: self.elem_type.clone_inner(),
            expr: self.expr.clone_inner(),
            is_implicit: self.is_implicit,
        }
    }
}
impl IntoAST for PrimitiveCastExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::PrimitiveCastExpr(self))
    }
}
impl FullArgsSpec for PrimitiveCastExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.elem_type.clone_inner())),
            ArgType::ASTFlatten(Some(self.expr.clone_inner())),
            ArgType::Bool(self.is_implicit),
        ]
    }
}

impl FullArgsSpecInit for PrimitiveCastExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        PrimitiveCastExpr::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[2].clone().try_as_bool().unwrap(),
        )
    }
}
impl PrimitiveCastExpr {
    pub fn new(elem_type: ASTFlatten, expr: ASTFlatten, is_implicit: bool) -> Self {
        Self {
            expression_base: ExpressionBase::new(None, None),
            elem_type,
            expr,
            is_implicit,
        }
    }
}
impl ASTChildren for PrimitiveCastExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.elem_type.clone());
        cb.add_child(self.expr.clone());
    }
}
impl ASTChildrenCallBack for PrimitiveCastExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.elem_type.assign(f(&self.elem_type).as_ref().unwrap());
        self.expr.assign(f(&self.expr).as_ref().unwrap());
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    LiteralExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum LiteralExpr {
    BooleanLiteralExpr(BooleanLiteralExpr),
    NumberLiteralExpr(NumberLiteralExpr),
    StringLiteralExpr(StringLiteralExpr),
    ArrayLiteralExpr(ArrayLiteralExpr),
}

#[enum_dispatch]
pub trait LiteralExprBaseRef: ExpressionBaseRef {
    fn literal_expr_base_ref(&self) -> &LiteralExprBase;
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LiteralExprBase {
    pub expression_base: ExpressionBase,
}
impl DeepClone for LiteralExprBase {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
        }
    }
}
impl FullArgsSpec for LiteralExprBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.annotated_type()
                .map(|tn| ASTFlatten::from(tn.clone_inner())),
        )]
    }
}

impl FullArgsSpecInit for LiteralExprBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        LiteralExprBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
        )
    }
}
impl LiteralExprBase {
    pub fn new(annotated_type: Option<RcCell<AnnotatedTypeName>>) -> Self {
        Self {
            expression_base: ExpressionBase::new(annotated_type, None),
        }
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct BooleanLiteralExpr {
    pub literal_expr_base: LiteralExprBase,
    pub value: bool,
}
impl DeepClone for BooleanLiteralExpr {
    fn clone_inner(&self) -> Self {
        Self {
            literal_expr_base: self.literal_expr_base.clone_inner(),
            value: self.value,
        }
    }
}
impl IntoAST for BooleanLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::BooleanLiteralExpr(
            self,
        )))
    }
}
impl FullArgsSpec for BooleanLiteralExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Bool(self.value)]
    }
}

impl FullArgsSpecInit for BooleanLiteralExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        BooleanLiteralExpr::new(fields[0].clone().try_as_bool().unwrap())
    }
}
impl BooleanLiteralExpr {
    pub fn new(value: bool) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(Some(RcCell::new(AnnotatedTypeName::new(
                BooleanLiteralType::new(value)
                    .into_ast()
                    .try_as_type_name()
                    .map(|tn| RcCell::new(tn).into()),
                None,
                Homomorphism::non_homomorphic(),
            )))),
            value,
        }
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct NumberLiteralExpr {
    pub literal_expr_base: LiteralExprBase,
    pub value: i32,
    pub value_string: Option<String>,
    pub was_hex: bool,
}
impl DeepClone for NumberLiteralExpr {
    fn clone_inner(&self) -> Self {
        Self {
            literal_expr_base: self.literal_expr_base.clone_inner(),
            ..self.clone()
        }
    }
}
impl IntoAST for NumberLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
            self,
        )))
    }
}
impl FullArgsSpec for NumberLiteralExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        if let Some(value) = &self.value_string {
            vec![
                ArgType::Str(Some(value.clone())),
                ArgType::Bool(self.was_hex),
            ]
        } else {
            vec![ArgType::Int(Some(self.value)), ArgType::Bool(self.was_hex)]
        }
    }
}

impl FullArgsSpecInit for NumberLiteralExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        if fields[0].is_str() {
            NumberLiteralExpr::new_string(fields[0].clone().try_as_str().flatten().unwrap())
        } else {
            NumberLiteralExpr::new(
                fields[0].clone().try_as_int().flatten().unwrap(),
                fields[1].clone().try_as_bool().unwrap(),
            )
        }
    }
}
impl NumberLiteralExpr {
    pub fn new(value: i32, was_hex: bool) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(Some(RcCell::new(AnnotatedTypeName::new(
                NumberLiteralType::new(NumberLiteralTypeUnion::I32(value))
                    .into_ast()
                    .try_as_type_name()
                    .map(|tn| RcCell::new(tn).into()),
                None,
                Homomorphism::non_homomorphic(),
            )))),
            value,
            value_string: None,
            was_hex,
        }
    }
    pub fn new_string(value_string: String) -> Self {
        // println!(
        //     "=value_string====={}==============",
        //     U256::from_str_prefixed(&value_string).unwrap().to_string()
        // );
        Self {
            literal_expr_base: LiteralExprBase::new(Some(RcCell::new(AnnotatedTypeName::new(
                NumberLiteralType::new(NumberLiteralTypeUnion::String(value_string.clone()))
                    .into_ast()
                    .try_as_type_name()
                    .map(|tn| RcCell::new(tn).into()),
                None,
                Homomorphism::non_homomorphic(),
            )))),
            value: 0,
            value_string: Some(U256::from_str_prefixed(&value_string).unwrap().to_string()),
            was_hex: false,
        }
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct StringLiteralExpr {
    pub literal_expr_base: LiteralExprBase,
    pub value: String,
}
impl DeepClone for StringLiteralExpr {
    fn clone_inner(&self) -> Self {
        Self {
            literal_expr_base: self.literal_expr_base.clone_inner(),
            value: self.value.clone(),
        }
    }
}
impl IntoAST for StringLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::StringLiteralExpr(
            self,
        )))
    }
}
impl FullArgsSpec for StringLiteralExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.value.clone()))]
    }
}

impl FullArgsSpecInit for StringLiteralExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        StringLiteralExpr::new(fields[0].clone().try_as_str().flatten().unwrap())
    }
}
impl StringLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(None),
            value,
        }
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ArrayLiteralExprBaseRef,
    LiteralExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum ArrayLiteralExpr {
    ArrayLiteralExpr(ArrayLiteralExprBase),
    KeyLiteralExpr(KeyLiteralExpr),
}

#[enum_dispatch]
pub trait ArrayLiteralExprBaseRef: LiteralExprBaseRef {
    fn array_literal_expr_base_ref(&self) -> &ArrayLiteralExprBase;
}

pub trait ArrayLiteralExprBaseProperty {
    fn values(&self) -> &Vec<ASTFlatten>;
}
impl<T: ArrayLiteralExprBaseRef> ArrayLiteralExprBaseProperty for T {
    fn values(&self) -> &Vec<ASTFlatten> {
        &self.array_literal_expr_base_ref().values
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ImplBaseTrait,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct ArrayLiteralExprBase {
    pub literal_expr_base: LiteralExprBase,
    pub values: Vec<ASTFlatten>,
}
impl DeepClone for ArrayLiteralExprBase {
    fn clone_inner(&self) -> Self {
        Self {
            literal_expr_base: self.literal_expr_base.clone_inner(),
            values: self.values.clone_inner(),
        }
    }
}
impl IntoAST for ArrayLiteralExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(
            ArrayLiteralExpr::ArrayLiteralExpr(self),
        )))
    }
}
impl FullArgsSpec for ArrayLiteralExprBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Vec(
            self.values
                .iter()
                .map(|pa| ArgType::ASTFlatten(Some(pa.clone_inner())))
                .collect(),
        )]
    }
}

impl FullArgsSpecInit for ArrayLiteralExprBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ArrayLiteralExprBase::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_ast_flatten().flatten().unwrap())
                .collect(),
        )
    }
}
impl ArrayLiteralExprBase {
    pub fn new(values: Vec<ASTFlatten>) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(None),
            values,
        }
    }
}
impl ASTChildren for ArrayLiteralExprBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.values.iter().for_each(|value| {
            cb.add_child(value.clone());
        });
    }
}
impl ASTChildrenCallBack for ArrayLiteralExprBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.values.iter().for_each(|value| {
            value.assign(f(value).as_ref().unwrap());
        });
    }
}
#[impl_traits(ArrayLiteralExprBase, LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct KeyLiteralExpr {
    pub array_literal_expr_base: ArrayLiteralExprBase,
    pub crypto_params: CryptoParams,
}
impl DeepClone for KeyLiteralExpr {
    fn clone_inner(&self) -> Self {
        Self {
            array_literal_expr_base: self.array_literal_expr_base.clone_inner(),
            crypto_params: self.crypto_params.clone(),
        }
    }
}
impl IntoAST for KeyLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(
            ArrayLiteralExpr::KeyLiteralExpr(self),
        )))
    }
}
impl FullArgsSpec for KeyLiteralExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.values()
                    .iter()
                    .map(|pa| ArgType::ASTFlatten(Some(pa.clone_inner())))
                    .collect(),
            ),
            ArgType::CryptoParams(Some(self.crypto_params.clone())),
        ]
    }
}
impl FullArgsSpecInit for KeyLiteralExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        KeyLiteralExpr::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_ast_flatten().flatten().unwrap())
                .collect(),
            fields[1].clone().try_as_crypto_params().flatten().unwrap(),
        )
    }
}
impl KeyLiteralExpr {
    pub fn new(values: Vec<ASTFlatten>, crypto_params: CryptoParams) -> Self {
        Self {
            array_literal_expr_base: ArrayLiteralExprBase::new(values),
            crypto_params,
        }
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    TupleOrLocationExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum TupleOrLocationExpr {
    TupleExpr(TupleExpr),
    LocationExpr(LocationExpr),
}

impl TupleOrLocationExpr {
    pub fn is_lvalue(&self) -> bool {
        let parent = match self {
            TupleOrLocationExpr::TupleExpr(te) => te.parent().clone().map(|p| p.upgrade().unwrap()),
            TupleOrLocationExpr::LocationExpr(te) => {
                te.parent().as_ref().map(|p| p.clone().upgrade().unwrap())
            }
        };
        assert!(parent.is_some());
        if is_instance(parent.as_ref().unwrap(), ASTType::AssignmentStatementBase) {
            return self
                == parent
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_assignment_statement_ref()
                    .unwrap()
                    .lhs()
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap();
        }
        if is_instance(parent.as_ref().unwrap(), ASTType::IndexExpr)
            && self.to_ast()
                == parent
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_index_expr_ref()
                    .unwrap()
                    .arr
                    .clone()
                    .unwrap()
                    .borrow()
                    .clone()
        {
            return parent
                .unwrap()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .is_lvalue();
        }
        if is_instance(parent.as_ref().unwrap(), ASTType::MemberAccessExpr)
            && self.to_ast()
                == parent
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .expr
                    .clone()
                    .unwrap()
                    .borrow()
                    .clone()
        {
            return parent
                .unwrap()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .is_lvalue();
        }
        if is_instance(parent.as_ref().unwrap(), ASTType::TupleExpr) {
            return parent
                .as_ref()
                .unwrap()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .is_lvalue();
        }

        false
    }

    pub fn is_rvalue(&self) -> bool {
        !self.is_lvalue()
    }
}
#[enum_dispatch]
pub trait TupleOrLocationExprBaseRef: ExpressionBaseRef {
    fn tuple_or_location_expr_base_ref(&self) -> &TupleOrLocationExprBase;
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleOrLocationExprBase {
    pub expression_base: ExpressionBase,
}
impl DeepClone for TupleOrLocationExprBase {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
        }
    }
}
impl FullArgsSpec for TupleOrLocationExprBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(self.idf().map(|idf| ASTFlatten::from(idf.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for TupleOrLocationExprBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        TupleOrLocationExprBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}
impl TupleOrLocationExprBase {
    pub fn new(
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
    ) -> Self {
        Self {
            expression_base: ExpressionBase::new(annotated_type, idf),
        }
    }
}
#[impl_traits(TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct TupleExpr {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub elements: Vec<ASTFlatten>,
}
impl DeepClone for TupleExpr {
    fn clone_inner(&self) -> Self {
        Self {
            tuple_or_location_expr_base: self.tuple_or_location_expr_base.clone_inner(),
            elements: self.elements.clone_inner(),
        }
    }
}
impl IntoAST for TupleExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::TupleExpr(self),
        ))
    }
}
impl FullArgsSpec for TupleExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Vec(
            self.elements
                .iter()
                .map(|pa| ArgType::ASTFlatten(Some(pa.clone_inner())))
                .collect(),
        )]
    }
}

impl FullArgsSpecInit for TupleExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        TupleExpr::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_ast_flatten().flatten().unwrap())
                .collect(),
        )
    }
}
impl TupleExpr {
    pub fn new(elements: Vec<ASTFlatten>) -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(None, None),
            elements,
        }
    }
    pub fn assign(&self, val: ASTFlatten) -> AssignmentStatement {
        AssignmentStatement::AssignmentStatement(AssignmentStatementBase::new(
            Some(RcCell::new(self.clone()).into()),
            Some(val),
            None,
        ))
    }
}
impl ASTChildren for TupleExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.elements.iter().for_each(|element| {
            cb.add_child(element.clone());
        });
    }
}
impl ASTChildrenCallBack for TupleExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.elements.iter().for_each(|element| {
            element.assign(f(element).as_ref().unwrap());
        });
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    LocationExprBaseRef,
    LocationExprBaseMutRef,
    TupleOrLocationExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum LocationExpr {
    IdentifierExpr(IdentifierExpr),
    MemberAccessExpr(MemberAccessExpr),
    IndexExpr(IndexExpr),
    SliceExpr(SliceExpr),
}

impl LocationExpr {
    pub fn call(&self, member: IdentifierExprUnion, args: Vec<ASTFlatten>) -> FunctionCallExpr {
        //  println!("====call============{:?}==========",self.get_ast_type());
        FunctionCallExpr::FunctionCallExpr(match member {
            IdentifierExprUnion::Identifier(member) => FunctionCallExprBase::new(
                RcCell::new(MemberAccessExpr::new(
                    Some(RcCell::new(self.to_ast())),
                    member,
                ))
                .into(),
                args,
                None,
                None,
            ),
            IdentifierExprUnion::String(member) => FunctionCallExprBase::new(
                RcCell::new(MemberAccessExpr::new(
                    Some(RcCell::new(self.to_ast())),
                    RcCell::new(Identifier::Identifier(IdentifierBase::new(member))),
                ))
                .into(),
                args,
                None,
                None,
            ),
            // _ => FunctionCallExprBase::new(
            //     Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(self.clone())),
            //     args,
            //     None,
            // ),
        })
    }

    pub fn dot(&self, member: IdentifierExprUnion) -> MemberAccessExpr {
        // println!("====dot============{:?}==========",self.get_ast_type());
        match member {
            IdentifierExprUnion::Identifier(member) => {
                MemberAccessExpr::new(Some(RcCell::new(self.to_ast())), member)
            }
            IdentifierExprUnion::String(member) => MemberAccessExpr::new(
                Some(RcCell::new(self.to_ast())),
                RcCell::new(Identifier::Identifier(IdentifierBase::new(member))),
            ),
        }
    }

    pub fn index(&self, item: ExprUnion) -> ASTFlatten {
        // println!("=====index========annotated_type=========={:?}",self
        //     .ast_base_ref().borrow().annotated_type);
        assert!(is_instances(
            self.ast_base_ref()
                .borrow()
                .annotated_type
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap(),
            vec![ASTType::ArrayBase, ASTType::Mapping]
        ));
        let value_type = self
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .and_then(|t| {
                t.to_ast().try_as_type_name().and_then(|t| match t {
                    TypeName::Array(a) => Some(a.value_type().clone().into()),
                    TypeName::Mapping(a) => Some(a.value_type.clone().into()),
                    _ => None,
                })
            });
        assert!(
            value_type.is_some(),
            "====value_type===is none==of  type name======={:?}",
            self.annotated_type().as_ref().unwrap().borrow().type_name
        );
        let item = match item {
            ExprUnion::I32(item) => RcCell::new(NumberLiteralExpr::new(item, false)).into(),
            ExprUnion::Expression(item) => item,
        };

        IndexExpr::new(Some(RcCell::new(self.to_ast())), item).as_type(value_type.as_ref().unwrap())
    }
    pub fn assign(&self, val: ASTFlatten) -> AssignmentStatement {
        AssignmentStatement::AssignmentStatement(AssignmentStatementBase::new(
            Some(RcCell::new(self.to_ast()).into()),
            Some(val),
            None,
        ))
    }
}
#[enum_dispatch]
pub trait LocationExprBaseRef: TupleOrLocationExprBaseRef {
    fn location_expr_base_ref(&self) -> &LocationExprBase;
}

#[impl_traits(TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LocationExprBase {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub target_rc: Option<ASTFlatten>,
}
impl DeepClone for LocationExprBase {
    fn clone_inner(&self) -> Self {
        Self {
            tuple_or_location_expr_base: self.tuple_or_location_expr_base.clone_inner(),
            target_rc: self.target_rc.clone_inner(),
        }
    }
}
impl FullArgsSpec for LocationExprBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(self.idf().map(|idf| ASTFlatten::from(idf.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for LocationExprBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        LocationExprBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}

impl LocationExprBase {
    pub fn new(
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
    ) -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(annotated_type, idf),
            target_rc: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IdentifierExprUnion {
    String(String),
    Identifier(RcCell<Identifier>),
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct IdentifierExpr {
    pub location_expr_base: LocationExprBase,
}
impl DeepClone for IdentifierExpr {
    fn clone_inner(&self) -> Self {
        Self {
            location_expr_base: self.location_expr_base.clone_inner(),
        }
    }
}
impl IntoAST for IdentifierExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(self)),
        ))
    }
}
impl FullArgsSpec for IdentifierExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.idf().map(|idf| ASTFlatten::from(idf.clone_inner()))),
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
        ]
    }
}
impl FullArgsSpecInit for IdentifierExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        IdentifierExpr::new(
            IdentifierExprUnion::Identifier(
                fields[0]
                    .clone()
                    .try_as_ast_flatten()
                    .flatten()
                    .unwrap()
                    .try_as_identifier()
                    .unwrap(),
            ),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
        )
    }
}

impl IdentifierExpr {
    pub fn new(
        idf: IdentifierExprUnion,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
    ) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(
                annotated_type,
                Some(match idf {
                    IdentifierExprUnion::Identifier(idf) => {
                        // print!("=idfname==={:?},",idf.borrow().name());
                        idf
                    }
                    IdentifierExprUnion::String(idf) => {
                        // print!("=idfname==={:?},",idf);
                        RcCell::new(Identifier::Identifier(IdentifierBase::new(idf)))
                    }
                }),
            ),
        }
    }

    pub fn get_annotated_type(&self) -> Option<AnnotatedTypeName> {
        self.ast_base_ref()
            .borrow()
            .target
            .clone()
            .unwrap()
            .upgrade()
            .map(|t| {
                // println!("==t===={:?}",t);
                t.try_as_variable_declaration_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .clone()
            })
    }

    pub fn slice(&self, offset: i32, size: i32, base: Option<ASTFlatten>) -> SliceExpr {
        SliceExpr::new(Some(RcCell::new(self.clone()).into()), base, offset, size)
    }
}
impl ASTChildren for IdentifierExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf() {
            cb.add_child(idf.clone().into());
        }
    }
}

impl ASTChildrenCallBack for IdentifierExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        if let Some(idf) = &self.idf() {
            self.ast_base_ref().borrow_mut().idf =
                f(&idf.clone().into()).and_then(|_idf| _idf.try_as_identifier());
        }
    }
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct MemberAccessExpr {
    pub location_expr_base: LocationExprBase,
    pub expr: Option<RcCell<AST>>,
    pub member: RcCell<Identifier>,
}
impl DeepClone for MemberAccessExpr {
    fn clone_inner(&self) -> Self {
        Self {
            location_expr_base: self.location_expr_base.clone_inner(),
            expr: self.expr.clone_inner(),
            member: self.member.clone_inner(),
        }
    }
}
impl IntoAST for MemberAccessExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::MemberAccessExpr(self)),
        ))
    }
}
impl FullArgsSpec for MemberAccessExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.expr
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.member.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for MemberAccessExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        MemberAccessExpr::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_ast()),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_identifier()
                .unwrap(),
        )
    }
}
impl MemberAccessExpr {
    pub fn new(expr: Option<RcCell<AST>>, member: RcCell<Identifier>) -> Self {
        // println!(
        //     "=MemberAccessExpr===new==={:?}========={:?}==========",
        //     expr.as_ref().map(|ex| {
        //         // print!("=asttype=={:?}", ex.borrow().get_ast_type());
        //         ex.borrow()
        //             .ast_base_ref()
        //             .borrow()
        //             .idf()
        //             .as_ref()
        //             .map(|idf| idf.borrow().name())
        //     }),
        //     member.borrow().name()
        // );
        Self {
            location_expr_base: LocationExprBase::new(None, None),
            expr,
            member,
        }
    }
}
impl ASTChildren for MemberAccessExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(expr) = &self.expr {
            //   println!("===MemberAccessExpr===process_children============{:?}=====",expr.get_ast_type());
            cb.add_child(expr.clone().into());
        }
        cb.add_child(self.member.clone().into());
    }
}

impl ASTChildrenCallBack for MemberAccessExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        let expr = self
            .expr
            .as_ref()
            .and_then(|expr| f(&expr.clone().into()).and_then(|astf| astf.try_as_ast()))
            .unwrap()
            .borrow()
            .clone();
        *self.expr.as_ref().unwrap().borrow_mut() = expr;
        let member = f(&self.member.clone().into())
            .unwrap()
            .try_as_identifier()
            .unwrap()
            .borrow()
            .clone();
        *self.member.borrow_mut() = member;
    }
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct IndexExpr {
    pub location_expr_base: LocationExprBase,
    pub arr: Option<RcCell<AST>>,
    pub key: ASTFlatten,
}
impl DeepClone for IndexExpr {
    fn clone_inner(&self) -> Self {
        Self {
            location_expr_base: self.location_expr_base.clone_inner(),
            arr: self.arr.clone_inner(),
            key: self.key.clone_inner(),
        }
    }
}
impl IntoAST for IndexExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IndexExpr(self)),
        ))
    }
}
impl FullArgsSpec for IndexExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.arr
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(Some(self.key.clone_inner())),
        ]
    }
}
impl FullArgsSpecInit for IndexExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        IndexExpr::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_ast()),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
        )
    }
}
impl IndexExpr {
    pub fn new(arr: Option<RcCell<AST>>, key: ASTFlatten) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(None, None),
            arr,
            key,
        }
    }
}
impl ASTChildren for IndexExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(arr) = &self.arr {
            cb.add_child(arr.clone().into());
        }
        cb.add_child(self.key.clone());
    }
}
impl ASTChildrenCallBack for IndexExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        *self.arr.as_ref().unwrap().borrow_mut() = self
            .arr
            .as_ref()
            .and_then(|a| f(&a.clone().into()).and_then(|astf| astf.try_as_ast()))
            .unwrap()
            .borrow()
            .clone();
        self.key.assign(f(&self.key).as_ref().unwrap());
    }
}

#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct SliceExpr {
    pub location_expr_base: LocationExprBase,
    pub arr: Option<ASTFlatten>,
    pub base: Option<ASTFlatten>,
    pub start_offset: i32,
    pub size: i32,
}
impl DeepClone for SliceExpr {
    fn clone_inner(&self) -> Self {
        let &Self {
            start_offset, size, ..
        } = self;
        Self {
            location_expr_base: self.location_expr_base.clone_inner(),
            arr: self.arr.clone_inner(),
            base: self.base.clone_inner(),
            start_offset,
            size,
        }
    }
}
impl IntoAST for SliceExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::SliceExpr(self)),
        ))
    }
}
impl FullArgsSpec for SliceExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.arr.as_ref().map(|a| a.clone_inner())),
            ArgType::ASTFlatten(self.base.as_ref().map(|a| a.clone_inner())),
            ArgType::Int(Some(self.start_offset)),
            ArgType::Int(Some(self.size)),
        ]
    }
}
impl FullArgsSpecInit for SliceExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        SliceExpr::new(
            fields[0].clone().try_as_ast_flatten().unwrap(),
            fields[1].clone().try_as_ast_flatten().unwrap(),
            fields[2].clone().try_as_int().flatten().unwrap(),
            fields[3].clone().try_as_int().flatten().unwrap(),
        )
    }
}
impl SliceExpr {
    pub fn new(
        arr: Option<ASTFlatten>,
        base: Option<ASTFlatten>,
        start_offset: i32,
        size: i32,
    ) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(None, None),
            arr,
            base,
            start_offset,
            size,
        }
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct MeExpr {
    pub expression_base: ExpressionBase,
    pub name: String,
}
impl DeepClone for MeExpr {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
            name: self.name.clone(),
        }
    }
}
impl PartialEq for MeExpr {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for MeExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::MeExpr(self))
    }
}
impl FullArgsSpec for MeExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for MeExpr {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        MeExpr::new()
    }
}
impl MeExpr {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(None, None),
            name: String::from("me"),
        }
    }
}
impl Immutable for MeExpr {
    fn is_immutable(&self) -> bool {
        true
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTChildrenImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct AllExpr {
    pub expression_base: ExpressionBase,
    pub name: String,
}
impl DeepClone for AllExpr {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
            name: self.name.clone(),
        }
    }
}
impl PartialEq for AllExpr {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for AllExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::AllExpr(self))
    }
}
impl FullArgsSpec for AllExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for AllExpr {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        AllExpr::new()
    }
}
impl AllExpr {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(None, None),
            name: String::from("all"),
        }
    }
}
impl Immutable for AllExpr {
    fn is_immutable(&self) -> bool {
        true
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ExpressionASType,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ReclassifyExprBaseRef,
    ReclassifyExprBaseMutRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumDispatchWithFields,
    ASTFlattenImpl,
    EnumIs,
    EnumTryAs,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum ReclassifyExpr {
    ReclassifyExpr(ReclassifyExprBase),
    RehomExpr(RehomExpr),
    EncryptionExpression(EncryptionExpression),
}

impl ReclassifyExpr {
    pub fn func_name(&self) -> String {
        if let Self::RehomExpr(rhe) = self {
            rhe.func_name()
        } else {
            String::from("reveal")
        }
    }
}

#[enum_dispatch]
pub trait ReclassifyExprBaseRef: ExpressionBaseRef {
    fn reclassify_expr_base_ref(&self) -> &ReclassifyExprBase;
}
pub trait ReclassifyExprBaseProperty {
    fn expr(&self) -> &ASTFlatten;
    fn privacy(&self) -> &ASTFlatten;
    fn homomorphism(&self) -> &Option<String>;
}
impl<T: ReclassifyExprBaseRef> ReclassifyExprBaseProperty for T {
    fn expr(&self) -> &ASTFlatten {
        &self.reclassify_expr_base_ref().expr
    }
    fn privacy(&self) -> &ASTFlatten {
        &self.reclassify_expr_base_ref().privacy
    }
    fn homomorphism(&self) -> &Option<String> {
        &self.reclassify_expr_base_ref().homomorphism
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ImplBaseTrait,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct ReclassifyExprBase {
    pub expression_base: ExpressionBase,
    pub expr: ASTFlatten,
    pub privacy: ASTFlatten,
    pub homomorphism: Option<String>,
}
impl DeepClone for ReclassifyExprBase {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
            expr: self.expr.clone_inner(),
            privacy: self.privacy.clone_inner(),
            homomorphism: self.homomorphism.clone(),
        }
    }
}
impl IntoAST for ReclassifyExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(ReclassifyExpr::ReclassifyExpr(
            self,
        )))
    }
}
impl FullArgsSpec for ReclassifyExprBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.expr.clone_inner())),
            ArgType::ASTFlatten(Some(self.privacy.clone_inner())),
            ArgType::Str(self.homomorphism.clone()),
            ArgType::ASTFlatten(
                self.annotated_type()
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
        ]
    }
}
impl FullArgsSpecInit for ReclassifyExprBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ReclassifyExprBase::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[2].clone().try_as_str().unwrap(),
            fields[3]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
        )
    }
}

impl ReclassifyExprBase {
    pub fn new(
        expr: ASTFlatten,
        privacy: ASTFlatten,
        homomorphism: Option<String>,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
    ) -> Self {
        // println!(
        //     "======ReclassifyExprBase====new==expr.get_ast_type==={:?}====={:?}",
        //     expr.to_string(),
        //     expr.get_ast_type()
        // );
        // if expr.to_string() == "reveal(Choice.none, me)" {
        //     panic!(
        //         "==ReclassifyExprBase====new====reveal(Choice.none, me)===={}==",
        //         expr.to_string()
        //     );
        // }
        Self {
            expression_base: ExpressionBase::new(annotated_type, None),
            expr,
            privacy,
            homomorphism,
        }
    }
    pub fn func_name(&self) -> String {
        if let Some(homomorphism) = &self.homomorphism {
            HOMOMORPHISM_STORE
                .lock()
                .unwrap()
                .get(homomorphism)
                .unwrap()
                .rehom_expr_name
                .clone()
        } else {
            String::from("reveal")
        }
    }
}
impl ASTChildren for ReclassifyExprBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.expr.clone());
        cb.add_child(self.privacy.clone());
    }
}
impl ASTChildrenCallBack for ReclassifyExprBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.expr.assign(f(&self.expr).as_ref().unwrap());
        self.privacy.assign(f(&self.privacy).as_ref().unwrap());
    }
}
#[impl_traits(ReclassifyExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct RehomExpr {
    pub reclassify_expr_base: ReclassifyExprBase,
}
impl DeepClone for RehomExpr {
    fn clone_inner(&self) -> Self {
        Self {
            reclassify_expr_base: self.reclassify_expr_base.clone_inner(),
        }
    }
}
impl IntoAST for RehomExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(ReclassifyExpr::RehomExpr(self)))
    }
}
impl ASTChildren for RehomExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.reclassify_expr_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for RehomExpr {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.reclassify_expr_base.process_children_callback(f);
    }
}

impl FullArgsSpec for RehomExpr {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.expr().clone_inner())),
            ArgType::Str(self.homomorphism().clone()),
        ]
    }
}
impl FullArgsSpecInit for RehomExpr {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        RehomExpr::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1].clone().try_as_str().unwrap(),
        )
    }
}
impl RehomExpr {
    pub fn new(expr: ASTFlatten, homomorphism: Option<String>) -> Self {
        // println!("==RehomExpr=====new========{expr}");
        // assert!( expr.to_string()!="c_count");

        Self {
            reclassify_expr_base: ReclassifyExprBase::new(
                expr,
                RcCell::new(Expression::MeExpr(MeExpr::new())).into(),
                homomorphism,
                None,
            ),
        }
    }
    pub fn func_name(&self) -> String {
        HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .get(self.reclassify_expr_base.homomorphism.as_ref().unwrap())
            .unwrap()
            .rehom_expr_name
            .clone()
    }
}

#[impl_traits(ReclassifyExprBase, ExpressionBase, ASTBase)]
#[derive(
    ExpressionASTypeImpl,
    ASTDebug,
    ASTFlattenImpl,
    ASTKind,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct EncryptionExpression {
    pub reclassify_expr_base: ReclassifyExprBase,
}
impl DeepClone for EncryptionExpression {
    fn clone_inner(&self) -> Self {
        Self {
            reclassify_expr_base: self.reclassify_expr_base.clone_inner(),
        }
    }
}
impl IntoAST for EncryptionExpression {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(
            ReclassifyExpr::EncryptionExpression(self),
        ))
    }
}
impl ASTChildren for EncryptionExpression {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.reclassify_expr_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for EncryptionExpression {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.reclassify_expr_base.process_children_callback(f);
    }
}

impl FullArgsSpec for EncryptionExpression {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.expr().clone_inner())),
            ArgType::ASTFlatten(Some(self.privacy().clone_inner())),
            ArgType::Str(self.homomorphism().clone()),
        ]
    }
}
impl FullArgsSpecInit for EncryptionExpression {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        EncryptionExpression::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[2].clone().try_as_str().unwrap(),
        )
    }
}
impl EncryptionExpression {
    pub fn new(expr: ASTFlatten, privacy: ASTFlatten, homomorphism: Option<String>) -> Self {
        let annotated_type = Some(AnnotatedTypeName::cipher_type(
            expr.ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .clone_inner(),
            homomorphism.clone(),
        ));
        Self {
            reclassify_expr_base: ReclassifyExprBase::new(
                expr,
                privacy,
                homomorphism,
                annotated_type,
            ),
        }
    }
}
