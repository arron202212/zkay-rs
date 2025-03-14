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
    AST, ASTBase, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTChildrenCallBack,
    ASTFlatten, ASTFlattenWeak, ASTInstanceOf, ASTType, ArgType, ChildListBuilder, DeepClone,
    FullArgsSpec, FullArgsSpecInit, Immutable, IntoAST,
    annotated_type_name::AnnotatedTypeName,
    expression::{
        Expression, ExpressionASType, ExpressionBase, ExpressionBaseMutRef, ExpressionBaseRef,
        NumberLiteralExpr,
    },
    identifier::IdentifierBaseProperty,
    identifier::{HybridArgumentIdf, Identifier, IdentifierBase},
    identifier_declaration::Parameter,
    is_instance, is_instances,
    statement::IndentBlock,
    statement::{AssignmentStatement, AssignmentStatementBase},
};
use crate::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use crate::global_defs::{GlobalDefs, GlobalVars, array_length_member, global_defs, global_vars};
use crate::homomorphism::{HOMOMORPHISM_STORE, Homomorphism, REHOM_EXPRESSIONS};
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use enum_dispatch::enum_dispatch;
use ethnum::{AsI256, AsU256, I256, U256, i256, int, u256, uint};
use eyre::{Result, eyre};
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
    config::{CFG, ConstructorOrFunctionDefinitionAttr},
    config_user::UserConfig,
    with_context_block, zk_print,
};
use zkay_derive::{
    ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind, ASTVisitorBaseRefImpl,
    EnumDispatchWithDeepClone, EnumDispatchWithFields, ExpressionASTypeImpl, ImplBaseTrait,
    impl_trait, impl_traits,
};
use zkay_transaction_crypto_params::params::CryptoParams;
use zkay_utils::progress_printer::warn_print;
use zkp_u256::{U256 as ZU256, Zero};
// #[enum_dispatch(FullArgsSpec,IntoAST, ASTInstanceOf, TypeNameBaseRef, ASTBaseRef)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
    Mapping(Mapping),
    Array(Array),
    TupleType(TupleType),
    FunctionTypeName(FunctionTypeName),
    Literal(String),
}
impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                    NumberTypeName::NumberLiteralType(s),
                )),
                Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                    NumberTypeName::NumberLiteralType(o),
                )),
            ) => s == o,
            (
                Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(s)),
                Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(o)),
            ) => s == o,
            (
                Self::ElementaryTypeName(ElementaryTypeName::BoolTypeName(s)),
                Self::ElementaryTypeName(ElementaryTypeName::BoolTypeName(o)),
            ) => s == o,
            (Self::ElementaryTypeName(s), Self::ElementaryTypeName(o)) => s == o,
            (
                Self::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(s)),
                Self::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(o)),
            ) => s == o,
            (
                Self::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(s)),
                Self::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(o)),
            ) => s == o,
            (Self::UserDefinedTypeName(s), Self::UserDefinedTypeName(o)) => s == o,
            (Self::Mapping(s), Self::Mapping(o)) => s == o,
            (Self::Array(Array::Proof(s)), Self::Array(Array::Proof(o))) => s == o,
            (Self::Array(Array::Key(s)), Self::Array(Array::Key(o))) => s == o,
            (Self::Array(Array::Randomness(s)), Self::Array(Array::Randomness(o))) => s == o,
            (Self::Array(Array::CipherText(s)), Self::Array(Array::CipherText(o))) => s == o,
            (Self::Array(s), Self::Array(o)) => s == o,
            (Self::TupleType(s), Self::TupleType(o)) => s == o,
            (Self::FunctionTypeName(s), Self::FunctionTypeName(o)) => s == o,
            (Self::Literal(s), Self::Literal(o)) => s == o,
            _ => false,
        }
    }
}
impl IntoAST for TypeName {
    fn into_ast(self) -> AST {
        match self {
            TypeName::ElementaryTypeName(ast) => ast.into_ast(),
            TypeName::UserDefinedTypeName(ast) => ast.into_ast(),
            TypeName::Mapping(ast) => ast.into_ast(),
            TypeName::Array(ast) => ast.into_ast(),
            TypeName::TupleType(ast) => ast.into_ast(),
            TypeName::FunctionTypeName(ast) => ast.into_ast(),
            other => AST::TypeName(other),
        }
    }
}
impl FullArgsSpec for TypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        match self {
            TypeName::ElementaryTypeName(ast) => ast.get_attr(),
            TypeName::UserDefinedTypeName(ast) => ast.get_attr(),
            TypeName::Mapping(ast) => ast.get_attr(),
            TypeName::Array(ast) => ast.get_attr(),
            TypeName::TupleType(ast) => ast.get_attr(),
            TypeName::FunctionTypeName(ast) => ast.get_attr(),
            TypeName::Literal(s) => vec![ArgType::Str(Some(s.clone()))],
        }
    }
}

impl FullArgsSpecInit for TypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        match self {
            TypeName::ElementaryTypeName(ast) => {
                TypeName::ElementaryTypeName(ast.from_fields(fields))
            }
            TypeName::UserDefinedTypeName(ast) => {
                TypeName::UserDefinedTypeName(ast.from_fields(fields))
            }
            TypeName::Mapping(ast) => TypeName::Mapping(ast.from_fields(fields)),
            TypeName::Array(ast) => TypeName::Array(ast.from_fields(fields)),
            TypeName::TupleType(ast) => TypeName::TupleType(ast.from_fields(fields)),
            TypeName::FunctionTypeName(ast) => TypeName::FunctionTypeName(ast.from_fields(fields)),
            TypeName::Literal(_) => self.clone(),
        }
    }
}

impl DeepClone for TypeName {
    fn clone_inner(&self) -> Self {
        match self {
            TypeName::ElementaryTypeName(ast) => TypeName::ElementaryTypeName(ast.clone_inner()),
            TypeName::UserDefinedTypeName(ast) => TypeName::UserDefinedTypeName(ast.clone_inner()),
            TypeName::Mapping(ast) => TypeName::Mapping(ast.clone_inner()),
            TypeName::Array(ast) => TypeName::Array(ast.clone_inner()),
            TypeName::TupleType(ast) => TypeName::TupleType(ast.clone_inner()),
            TypeName::FunctionTypeName(ast) => TypeName::FunctionTypeName(ast.clone_inner()),
            TypeName::Literal(_) => self.clone(),
        }
    }
}

impl ASTInstanceOf for TypeName {
    fn get_ast_type(&self) -> ASTType {
        match self {
            TypeName::ElementaryTypeName(ast) => ast.get_ast_type(),
            TypeName::UserDefinedTypeName(ast) => ast.get_ast_type(),
            TypeName::Mapping(ast) => ast.get_ast_type(),
            TypeName::Array(ast) => ast.get_ast_type(),
            TypeName::TupleType(ast) => ast.get_ast_type(),
            TypeName::FunctionTypeName(ast) => ast.get_ast_type(),
            TypeName::Literal(_) => ASTType::Literal,
        }
    }
}

impl ASTChildren for TypeName {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        match self {
            TypeName::ElementaryTypeName(ast) => ast.process_children(cb),
            TypeName::UserDefinedTypeName(ast) => ast.process_children(cb),
            TypeName::Mapping(ast) => ast.process_children(cb),
            TypeName::Array(ast) => ast.process_children(cb),
            // TypeName::TupleType(ast) => ast.process_children(cb),
            TypeName::FunctionTypeName(ast) => ast.process_children(cb),
            _ => {}
        }
    }
}

impl ASTChildrenCallBack for TypeName {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        match self {
            TypeName::ElementaryTypeName(ast) => ast.process_children_callback(f),
            TypeName::UserDefinedTypeName(ast) => ast.process_children_callback(f),
            TypeName::Mapping(ast) => ast.process_children_callback(f),
            TypeName::Array(ast) => ast.process_children_callback(f),
            // TypeName::TupleType(ast) => ast.process_children_callback(f),
            TypeName::FunctionTypeName(ast) => ast.process_children_callback(f),
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! impl_base_ref_for_typename {
    ($fn_name: ident,$self: ident) => {
        match $self {
            TypeName::ElementaryTypeName(ast) => Some(ast.$fn_name()),
            TypeName::UserDefinedTypeName(ast) => Some(ast.$fn_name()),
            TypeName::Mapping(ast) => Some(ast.$fn_name()),
            TypeName::Array(ast) => Some(ast.$fn_name()),
            TypeName::TupleType(ast) => Some(ast.$fn_name()),
            TypeName::FunctionTypeName(ast) => Some(ast.$fn_name()),
            _ => None,
        }
    };
}
impl TypeName {
    pub fn ast_base_ref(&self) -> Option<RcCell<ASTBase>> {
        impl_base_ref_for_typename!(ast_base_ref, self)
    }
    pub fn ast_base_mut_ref(&mut self) -> Option<RcCell<ASTBase>> {
        impl_base_ref_for_typename!(ast_base_mut_ref, self)
    }

    pub fn bool_type() -> Self {
        TypeName::ElementaryTypeName(ElementaryTypeName::BoolTypeName(BoolTypeName::new()))
    }

    pub fn uint_type() -> Self {
        TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::UintTypeName(UintTypeName::new(String::from("uint"))),
        ))
    }

    pub fn number_type() -> Self {
        TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(NumberTypeName::any()))
    }

    pub fn address_type() -> Self {
        TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(AddressTypeName::new()))
    }

    pub fn address_payable_type() -> Self {
        TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(
            AddressPayableTypeName::new(),
        ))
    }

    pub fn cipher_type(plain_type: RcCell<AnnotatedTypeName>, hom: String) -> Self {
        let crypto_params = CryptoParams::new(CFG.lock().unwrap().get_crypto_params(&hom));
        let mut plain_type = plain_type.borrow().clone();
        plain_type.homomorphism = hom; // Just for display purposes
        TypeName::Array(Array::CipherText(CipherText::new(
            Some(RcCell::new(plain_type)),
            crypto_params,
        )))
    }

    pub fn rnd_type(crypto_params: CryptoParams) -> Self {
        TypeName::Array(Array::Randomness(Randomness::new(crypto_params)))
    }

    pub fn key_type(crypto_params: CryptoParams) -> Self {
        TypeName::Array(Array::Key(Key::new(crypto_params)))
    }

    pub fn proof_type() -> Self {
        TypeName::Array(Array::Proof(Proof::new()))
    }

    pub fn dyn_uint_array() -> Self {
        TypeName::Array(Array::Array(ArrayBase::new(
            AnnotatedTypeName::uint_all(),
            None,
            None,
        )))
    }
    // """How many uints this type occupies when serialized."""
    pub fn size_in_uints(&self) -> i32 {
        match self {
            Self::Array(Array::CipherText(ct)) => ct.size_in_uints(),
            Self::Array(a) => a.array_base_ref().size_in_uints(),
            _ => 1,
        }
    }

    pub fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        // raise NotImplementedError()
        match self {
            Self::ElementaryTypeName(ElementaryTypeName::BoolTypeName(blt)) => blt.elem_bitwidth(),
            Self::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(blt)) => {
                blt.elem_bitwidth()
            }
            // Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            //     NumberTypeName::NumberLiteralType(nlt),
            // )) => nlt.elem_bitwidth(),
            Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(nlt)) => {
                nlt.elem_bitwidth()
            }
            Self::UserDefinedTypeName(UserDefinedTypeName::EnumTypeName(nlt)) => {
                nlt.elem_bitwidth()
            }
            Self::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(nlt)) => {
                nlt.elem_bitwidth()
            }
            Self::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(nlt)) => {
                nlt.elem_bitwidth()
            }
            Self::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(nlt)) => {
                nlt.elem_bitwidth()
            }
            Self::Array(nlt) => nlt.elem_bitwidth(),
            _ => {
                println!(
                    "==========unexpected elem_bitwidth=={:?}",
                    self.get_ast_type()
                );
                0
            }
        }
    }
    pub fn is_literals(&self) -> bool {
        is_instances(
            self,
            vec![
                ASTType::NumberLiteralType,
                ASTType::BooleanLiteralType,
                ASTType::EnumValueTypeName,
            ],
        )
    }
    pub fn is_address(&self) -> bool {
        is_instances(
            self,
            vec![ASTType::AddressTypeName, ASTType::AddressPayableTypeName],
        )
    }
    pub fn is_primitive_type(&self) -> bool {
        is_instances(
            self,
            vec![
                ASTType::ElementaryTypeNameBase,
                ASTType::EnumTypeName,
                ASTType::EnumValueTypeName,
                ASTType::AddressTypeName,
                ASTType::AddressPayableTypeName,
            ],
        )
    }
    pub fn is_cipher(&self) -> bool {
        // println!("=====************=====is_cipher===={:?}======",self.get_ast_type());
        is_instance(self, ASTType::CipherText)
    }
    pub fn is_key(&self) -> bool {
        is_instance(self, ASTType::Key)
    }
    pub fn is_randomness(&self) -> bool {
        is_instance(self, ASTType::Randomness)
    }
    pub fn is_numeric(&self) -> bool {
        is_instance(self, ASTType::NumberTypeNameBase)
    }
    pub fn is_boolean(&self) -> bool {
        is_instances(
            self,
            vec![ASTType::BooleanLiteralType, ASTType::BoolTypeName],
        )
    }
    pub fn signed(&self) -> bool {
        is_instance(self, ASTType::NumberTypeNameBase)
            && self
                .try_as_elementary_type_name_ref()
                .unwrap()
                .try_as_number_type_name_ref()
                .unwrap()
                .signed()
    }
    pub fn is_signed_numeric(&self) -> bool {
        self.is_numeric() && self.signed()
    }

    pub fn can_be_private(&self) -> bool {
        self.is_primitive_type() && !(self.is_signed_numeric() && self.elem_bitwidth() == 256)
    }
    pub fn value(&self) -> String {
        match self {
            Self::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(blt)) => blt.value(),
            Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::NumberLiteralType(nlt),
            )) => nlt.value(),
            _ => String::new(),
        }
    }
    pub fn to_abstract_type(&self) -> Option<ASTFlatten> {
        match self {
            Self::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(blt)) => {
                Some(blt.to_abstract_type())
            }
            Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::NumberLiteralType(nlt),
            )) => Some(nlt.to_abstract_type()),
            Self::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(evtn)) => {
                Some(evtn.to_abstract_type())
            }
            _ => None,
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        let res = expected.to_ast().try_as_type_name().unwrap() == *self;
        // if !res {
        //     // println!(
        //     //     "=======implicitly_convertible_to========={:?},========================{:?}",
        //     //     "self", "expected"
        //     // );
        // }
        match self {
            Self::ElementaryTypeName(etn) => match etn {
                ElementaryTypeName::BooleanLiteralType(blt) => {
                    blt.implicitly_convertible_to(expected)
                }
                ElementaryTypeName::NumberTypeName(ntn) => match ntn {
                    NumberTypeName::NumberLiteralType(nlt) => {
                        nlt.implicitly_convertible_to(expected)
                    }
                    NumberTypeName::IntTypeName(itn) => itn.implicitly_convertible_to(expected),
                    NumberTypeName::UintTypeName(utn) => utn.implicitly_convertible_to(expected),
                    _ => ntn.implicitly_convertible_to(expected),
                },
                _ => res,
            },
            Self::UserDefinedTypeName(udt) => match udt {
                UserDefinedTypeName::EnumValueTypeName(evtn) => {
                    evtn.implicitly_convertible_to(expected)
                }
                UserDefinedTypeName::AddressPayableTypeName(aptn) => {
                    aptn.implicitly_convertible_to(expected)
                }
                _ => res,
            },
            Self::TupleType(tt) => tt.implicitly_convertible_to(expected),
            _ => res,
        }
    }
    pub fn compatible_with(self, other_type: &ASTFlatten) -> bool {
        self.implicitly_convertible_to(other_type)
            || other_type
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .implicitly_convertible_to(&RcCell::new(self.clone()).into())
    }
    pub fn combined_type(
        &self,
        other_type: &ASTFlatten,
        _convert_literals: bool,
    ) -> Option<ASTFlatten> {
        // println!(
        //     "=======combined_type======{:?}===={:?}=========",
        //     self.get_ast_type(),
        //     other_type.borrow().get_ast_type()
        // );
        match self {
            Self::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(blt)) => {
                Some(blt.combined_type(other_type, _convert_literals))
            }
            Self::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::NumberLiteralType(nlt),
            )) => Some(nlt.combined_type(other_type, _convert_literals)),
            Self::TupleType(tt) => tt.combined_type(other_type, _convert_literals),
            _ => {
                let selfs = RcCell::new(self.clone()).into();
                if other_type
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .implicitly_convertible_to(&selfs)
                {
                    Some(selfs)
                } else if self.implicitly_convertible_to(other_type) {
                    Some(other_type.clone())
                } else {
                    None
                }
            }
        }
    }
    pub fn combined_type_base(
        &self,
        other_type: &ASTFlatten,
        _convert_literals: bool,
    ) -> Option<ASTFlatten> {
        // println!(
        //     "=======combined_type_base======{:?}===={:?}=========",
        //     self.get_ast_type(),
        //     other_type.borrow().get_ast_type()
        // );

        let selfs = RcCell::new(self.clone()).into();
        if other_type
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .implicitly_convertible_to(&selfs)
        {
            Some(selfs)
        } else if self.implicitly_convertible_to(other_type) {
            Some(other_type.clone())
        } else {
            None
        }
    }

    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> RcCell<AnnotatedTypeName> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(self.clone()).into()),
            if let CombinedPrivacyUnion::AST(expr) = privacy_annotation {
                expr
            } else {
                None
            },
            Homomorphism::non_homomorphic(),
        ))
    }
}
#[enum_dispatch]
pub trait TypeNameBaseRef: ASTBaseRef {
    fn type_name_base_ref(&self) -> &TypeNameBase;
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TypeNameBase {
    pub ast_base: RcCell<ASTBase>,
}
impl DeepClone for TypeNameBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
        }
    }
}
impl FullArgsSpec for TypeNameBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlattenWeak(self.target().clone())]
    }
}
impl FullArgsSpecInit for TypeNameBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        TypeNameBase::new(fields[0].try_as_ast_flatten_weak_ref().unwrap().clone())
    }
}
impl TypeNameBase {
    pub fn new(target: Option<ASTFlattenWeak>) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(None, None, target)),
        }
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ElementaryTypeNameBaseRef,
    TypeNameBaseRef,
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
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum ElementaryTypeName {
    NumberTypeName(NumberTypeName),
    BoolTypeName(BoolTypeName),
    BooleanLiteralType(BooleanLiteralType),
}
impl PartialEq for ElementaryTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.get_ast_type() == other.get_ast_type() && self.name() == other.name()
    }
}
impl PartialEq for ElementaryTypeNameBase {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
#[enum_dispatch]
pub trait ElementaryTypeNameBaseRef: TypeNameBaseRef {
    fn elementary_type_name_base_ref(&self) -> &ElementaryTypeNameBase;
}
pub trait ElementaryTypeNameBaseProperty {
    fn name(&self) -> &String;
}
impl<T: ElementaryTypeNameBaseRef> ElementaryTypeNameBaseProperty for T {
    fn name(&self) -> &String {
        &self.elementary_type_name_base_ref().name
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ImplBaseTrait, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct ElementaryTypeNameBase {
    pub type_name_base: TypeNameBase,
    pub name: String,
}
impl DeepClone for ElementaryTypeNameBase {
    fn clone_inner(&self) -> Self {
        Self {
            type_name_base: self.type_name_base.clone_inner(),
            name: self.name.clone(),
        }
    }
}
impl FullArgsSpec for ElementaryTypeNameBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.name.clone()))]
    }
}
impl FullArgsSpecInit for ElementaryTypeNameBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ElementaryTypeNameBase::new(fields[0].clone().try_as_str().flatten().unwrap())
    }
}
impl ElementaryTypeNameBase {
    pub fn new(name: String) -> Self {
        Self {
            type_name_base: TypeNameBase::new(None),
            name,
        }
    }
}
#[impl_traits(ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash,
)]
pub struct BoolTypeName {
    pub elementary_type_name_base: ElementaryTypeNameBase,
}
impl DeepClone for BoolTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            elementary_type_name_base: self.elementary_type_name_base.clone_inner(),
        }
    }
}
impl PartialEq for BoolTypeName {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for BoolTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::BoolTypeName(self),
        ))
    }
}
impl FullArgsSpec for BoolTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for BoolTypeName {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        BoolTypeName::new()
    }
}
impl BoolTypeName {
    pub fn new() -> Self {
        Self {
            elementary_type_name_base: ElementaryTypeNameBase::new(String::from("bool")),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        // raise NotImplementedError()
        1
    }
}
#[impl_traits(ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash,
)]
pub struct BooleanLiteralType {
    pub elementary_type_name_base: ElementaryTypeNameBase,
}
impl DeepClone for BooleanLiteralType {
    fn clone_inner(&self) -> Self {
        Self {
            elementary_type_name_base: self.elementary_type_name_base.clone_inner(),
        }
    }
}
impl PartialEq for BooleanLiteralType {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for BooleanLiteralType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::BooleanLiteralType(self),
        ))
    }
}
impl FullArgsSpec for BooleanLiteralType {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Bool(
            &self.elementary_type_name_base.name == "true",
        )]
    }
}
impl FullArgsSpecInit for BooleanLiteralType {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        BooleanLiteralType::new(fields[0].clone().try_as_bool().unwrap())
    }
}
impl BooleanLiteralType {
    pub fn new(name: bool) -> Self {
        let mut name = name.to_string();
        name.make_ascii_lowercase();
        Self {
            elementary_type_name_base: ElementaryTypeNameBase::new(name),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        expected.to_ast().try_as_type_name().unwrap()
            == TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(self.clone()))
            || is_instance(expected, ASTType::BoolTypeName)
    }
    pub fn combined_type(&self, other_type: &ASTFlatten, convert_literals: bool) -> ASTFlatten {
        if is_instance(other_type, ASTType::BooleanLiteralType) {
            RcCell::new(if convert_literals {
                TypeName::bool_type()
            } else {
                TypeName::Literal(String::from("lit"))
            })
            .into()
        } else {
            self.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .combined_type_base(other_type, convert_literals)
                .unwrap()
        }
    }
    pub fn value(&self) -> String {
        self.name().clone()
    }
    pub fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        // raise NotImplementedError()
        1
    }
    pub fn to_abstract_type(&self) -> ASTFlatten {
        RcCell::new(TypeName::bool_type()).into()
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    NumberTypeNameBaseRef,
    ElementaryTypeNameBaseRef,
    TypeNameBaseRef,
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
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum NumberTypeName {
    NumberLiteralType(NumberLiteralType),
    IntTypeName(IntTypeName),
    UintTypeName(UintTypeName),
    NumberTypeNameBase(NumberTypeNameBase),
}
impl PartialEq for NumberTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.get_ast_type() == other.get_ast_type() && self.name() == other.name()
    }
}
impl NumberTypeName {
    pub fn any() -> Self {
        NumberTypeName::NumberTypeNameBase(NumberTypeNameBase::new(
            String::new(),
            String::new(),
            true,
            Some(256),
        ))
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(self.clone()))
            == expected.to_ast().try_as_type_name().unwrap()
            || expected.to_ast().try_as_type_name().unwrap().get_ast_type()
                == ASTType::NumberTypeNameBase
    }
}
#[enum_dispatch]
pub trait NumberTypeNameBaseRef: ElementaryTypeNameBaseRef {
    fn number_type_name_base_ref(&self) -> &NumberTypeNameBase;
}
pub trait NumberTypeNameBaseProperty {
    fn prefix(&self) -> &String;
    fn signed(&self) -> bool;
    fn bitwidth(&self) -> Option<i32>;
    fn _size_in_bits(&self) -> i32;
    fn elem_bitwidth(&self) -> i32;
}
impl<T: NumberTypeNameBaseRef> NumberTypeNameBaseProperty for T {
    fn prefix(&self) -> &String {
        &self.number_type_name_base_ref().prefix
    }
    fn signed(&self) -> bool {
        self.number_type_name_base_ref().signed
    }
    fn bitwidth(&self) -> Option<i32> {
        self.number_type_name_base_ref().bitwidth
    }
    fn _size_in_bits(&self) -> i32 {
        self.number_type_name_base_ref()._size_in_bits
    }
    fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        if self._size_in_bits() == 0 {
            256
        } else {
            self._size_in_bits()
        }
    }
}
#[impl_traits(ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
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
pub struct NumberTypeNameBase {
    pub elementary_type_name_base: ElementaryTypeNameBase,
    pub prefix: String,
    pub signed: bool,
    pub bitwidth: Option<i32>,
    pub _size_in_bits: i32,
}
impl DeepClone for NumberTypeNameBase {
    fn clone_inner(&self) -> Self {
        Self {
            elementary_type_name_base: self.elementary_type_name_base.clone_inner(),
            ..self.clone()
        }
    }
}
impl IntoAST for NumberTypeNameBase {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::NumberTypeNameBase(self)),
        ))
    }
}
impl FullArgsSpec for NumberTypeNameBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Str(Some(self.name().clone())),
            ArgType::Str(Some(self.prefix.clone())),
            ArgType::Bool(self.signed),
            ArgType::Int(self.bitwidth),
        ]
    }
}
impl FullArgsSpecInit for NumberTypeNameBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        NumberTypeNameBase::new(
            fields[0].clone().try_as_str().flatten().unwrap(),
            fields[1].clone().try_as_str().flatten().unwrap(),
            fields[2].clone().try_as_bool().unwrap(),
            fields[3].clone().try_as_int().unwrap(),
        )
    }
}
impl NumberTypeNameBase {
    pub fn new(name: String, prefix: String, signed: bool, bitwidth: Option<i32>) -> Self {
        assert!(name.starts_with(&prefix), "{name} {prefix}");
        let prefix_len = prefix.len();
        let _size_in_bits = if let Some(bitwidth) = bitwidth {
            bitwidth
        } else if name.len() > prefix_len {
            name[prefix_len..].parse::<i32>().unwrap()
        } else {
            0
        };
        Self {
            elementary_type_name_base: ElementaryTypeNameBase::new(name),
            prefix,
            signed,
            bitwidth,
            _size_in_bits,
        }
    }

    // """Return true if value can be represented by this type"""
    pub fn can_represent(&self, value: i32) -> bool {
        let elem_bitwidth = self.elem_bitwidth() as usize;

        // println!("=========elem_bitwidth============{}",elem_bitwidth);
        assert!(
            elem_bitwidth > 0 && elem_bitwidth <= 256,
            "elem_bitwidth equal zero{}",
            elem_bitwidth
        );
        let i1 = int!("1");
        if self.signed {
            let v = I256::from(value);
            (-(i1 << (elem_bitwidth - 2))) * 2 <= v && v < ((i1 << (elem_bitwidth - 2) - 1) * 2 + 1)
        } else {
            let v = U256::from(value as u32);
            uint!("0") <= v && v < (uint!("1") << elem_bitwidth - 1)
        }
    }
}
#[derive(Debug)]
pub enum NumberLiteralTypeUnion {
    String(String),
    I32(i32),
}
#[impl_traits(NumberTypeNameBase, ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash,
)]
pub struct NumberLiteralType {
    pub number_type_name_base: NumberTypeNameBase,
}
impl DeepClone for NumberLiteralType {
    fn clone_inner(&self) -> Self {
        Self {
            number_type_name_base: self.number_type_name_base.clone_inner(),
        }
    }
}
impl PartialEq for NumberLiteralType {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for NumberLiteralType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::NumberLiteralType(self)),
        ))
    }
}
impl FullArgsSpec for NumberLiteralType {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.name().clone()))]
    }
}
impl FullArgsSpecInit for NumberLiteralType {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        NumberLiteralType::new(NumberLiteralTypeUnion::String(
            fields[0].clone().try_as_str().flatten().unwrap(),
        ))
    }
}
impl NumberLiteralType {
    pub fn new(name: NumberLiteralTypeUnion) -> Self {
        // println!("{name:?}");
        let (iname, uname) = match name {
            NumberLiteralTypeUnion::String(v) => (
                I256::from_str_prefixed(&v).ok(),
                U256::from_str_prefixed(&v).ok(),
            ), //TODO U256
            NumberLiteralTypeUnion::I32(v) => (Some(v.as_i256()), None),
        };
        let blen = if iname.is_some() {
            (I256::BITS - iname.unwrap().leading_zeros()) as i32
        } else {
            (U256::BITS - uname.unwrap().leading_zeros()) as i32
        };
        let (mut signed, mut bitwidth) = (false, blen);
        if iname.is_some() && iname.unwrap() < 0 {
            signed = true;
            if iname.unwrap() != -(1 << (blen - 1)) {
                bitwidth += 1;
            }
        };
        bitwidth = 8i32.max((bitwidth + 7) / 8 * 8);
        assert!(bitwidth <= 256);
        let name = if iname.is_some() {
            iname.unwrap().to_string()
        } else {
            uname.unwrap().to_string()
        };
        let prefix = name.clone();
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, prefix, signed, Some(bitwidth)),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        if expected.to_ast().try_as_type_name().unwrap().is_numeric()
            && !expected.to_ast().try_as_type_name().unwrap().is_literals()
        {
            // Allow implicit conversion only if it fits
            expected
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_elementary_type_name_ref()
                .unwrap()
                .try_as_number_type_name_ref()
                .unwrap()
                .number_type_name_base_ref()
                .can_represent(self.value().parse::<i32>().unwrap())
        } else if expected.to_ast().try_as_type_name().unwrap().is_address()
            && self.number_type_name_base.elem_bitwidth() == 160
            && !self.number_type_name_base.signed
        {
            // Address literal case (fake solidity check will catch the cases where this is too permissive)
            true
        } else {
            NumberTypeName::NumberLiteralType(self.clone()).implicitly_convertible_to(expected)
        }
    }
    pub fn combined_type(&self, other_type: &ASTFlatten, convert_literals: bool) -> ASTFlatten {
        if is_instance(other_type, ASTType::NumberLiteralType) {
            if convert_literals {
                self.to_abstract_type()
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .combined_type(
                        &other_type
                            .to_ast()
                            .try_as_type_name()
                            .unwrap()
                            .try_as_elementary_type_name_ref()
                            .unwrap()
                            .try_as_number_type_name_ref()
                            .unwrap()
                            .try_as_number_literal_type_ref()
                            .unwrap()
                            .to_abstract_type(),
                        convert_literals,
                    )
                    .unwrap()
            } else {
                RcCell::new(TypeName::Literal(String::from("lit"))).into()
            }
        } else {
            self.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .combined_type_base(other_type, convert_literals)
                .unwrap()
        }
    }
    pub fn to_abstract_type(&self) -> ASTFlatten {
        RcCell::new(if self.value().parse::<i32>().unwrap() < 0 {
            TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::IntTypeName(IntTypeName::new(format!(
                    "int{}",
                    self.number_type_name_base.elem_bitwidth()
                ))),
            ))
        } else {
            TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::UintTypeName(UintTypeName::new(format!(
                    "uint{}",
                    self.number_type_name_base.elem_bitwidth()
                ))),
            ))
        })
        .into()
    }
    pub fn value(&self) -> String {
        self.name().clone()
    }
}
#[impl_traits(NumberTypeNameBase, ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
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
pub struct IntTypeName {
    pub number_type_name_base: NumberTypeNameBase,
}
impl DeepClone for IntTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            number_type_name_base: self.number_type_name_base.clone_inner(),
        }
    }
}
impl IntoAST for IntTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::IntTypeName(self)),
        ))
    }
}
impl FullArgsSpec for IntTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.name().clone()))]
    }
}
impl FullArgsSpecInit for IntTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        IntTypeName::new(fields[0].clone().try_as_str().flatten().unwrap())
    }
}
impl IntTypeName {
    pub fn new(name: String) -> Self {
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, String::from("int"), true, None),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        NumberTypeName::IntTypeName(self.clone()).implicitly_convertible_to(expected)
            || (is_instance(expected, ASTType::IntTypeName)
                && expected
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .elem_bitwidth()
                    >= self.number_type_name_base.elem_bitwidth())
    }
}
#[impl_traits(NumberTypeNameBase, ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
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
pub struct UintTypeName {
    pub number_type_name_base: NumberTypeNameBase,
}
impl DeepClone for UintTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            number_type_name_base: self.number_type_name_base.clone_inner(),
        }
    }
}
impl IntoAST for UintTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::UintTypeName(self)),
        ))
    }
}
impl FullArgsSpec for UintTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.name().clone()))]
    }
}
impl FullArgsSpecInit for UintTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        UintTypeName::new(fields[0].clone().try_as_str().flatten().unwrap())
    }
}
impl UintTypeName {
    pub fn new(name: String) -> Self {
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, String::from("uint"), false, None),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        // println!(
        //     "===implicitly_convertible_to=======UintTypeName========={}==={}======{}={}==",
        //     NumberTypeName::UintTypeName(self.clone()).implicitly_convertible_to(expected),
        //     is_instance(expected, ASTType::UintTypeName),
        //     expected.borrow().elem_bitwidth(),
        //     self.number_type_name_base.elem_bitwidth()
        // );
        // Implicitly convert smaller i32 types to larger i32 types
        NumberTypeName::UintTypeName(self.clone()).implicitly_convertible_to(expected)
            || (is_instance(expected, ASTType::UintTypeName)
                && expected
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .elem_bitwidth()
                    >= self.number_type_name_base.elem_bitwidth())
    }
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    UserDefinedTypeNameBaseRef,
    UserDefinedTypeNameBaseMutRef,
    TypeNameBaseRef,
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
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum UserDefinedTypeName {
    EnumTypeName(EnumTypeName),
    EnumValueTypeName(EnumValueTypeName),
    StructTypeName(StructTypeName),
    ContractTypeName(ContractTypeName),
    AddressTypeName(AddressTypeName),
    AddressPayableTypeName(AddressPayableTypeName),
    UserDefinedTypeName(UserDefinedTypeNameBase),
}
impl PartialEq for UserDefinedTypeName {
    fn eq(&self, other: &Self) -> bool {
        // println!("==UserDefinedTypeName==========");
        self.ast_base_ref()
            .borrow()
            .target
            .as_ref()
            .zip(other.ast_base_ref().borrow().target.as_ref())
            .map_or_else(
                || {
                    self.ast_base_ref()
                        .borrow()
                        .target
                        .as_ref()
                        .or(other.ast_base_ref().borrow().target.as_ref())
                        .is_none()
                },
                |(target, other_target)| {
                    target
                        .clone()
                        .upgrade()
                        .unwrap()
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .qualified_name()
                        .iter()
                        .zip(
                            &other_target
                                .clone()
                                .upgrade()
                                .unwrap()
                                .ast_base_ref()
                                .unwrap()
                                .borrow()
                                .qualified_name(),
                        )
                        .all(|e| e.0.borrow().name() == e.1.borrow().name())
                },
            )
    }
}

impl PartialEq for UserDefinedTypeNameBase {
    fn eq(&self, other: &Self) -> bool {
        self.ast_base_ref()
            .borrow()
            .target
            .as_ref()
            .zip(other.ast_base_ref().borrow().target.as_ref())
            .map_or_else(
                || {
                    self.ast_base_ref()
                        .borrow()
                        .target
                        .as_ref()
                        .or(other.ast_base_ref().borrow().target.as_ref())
                        .is_none()
                },
                |(target, other_target)| {
                    target
                        .clone()
                        .upgrade()
                        .unwrap()
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .qualified_name()
                        .iter()
                        .zip(
                            &other_target
                                .clone()
                                .upgrade()
                                .unwrap()
                                .ast_base_ref()
                                .unwrap()
                                .borrow()
                                .qualified_name(),
                        )
                        .all(|e| e.0.borrow().name() == e.1.borrow().name())
                },
            )
    }
}
#[enum_dispatch]
pub trait UserDefinedTypeNameBaseRef: TypeNameBaseRef {
    fn user_defined_type_name_base_ref(&self) -> &UserDefinedTypeNameBase;
}
pub trait UserDefinedTypeNameBaseProperty {
    fn names(&self) -> &Vec<RcCell<Identifier>>;
    // fn target(&self) -> &Option<ASTFlattenWeak>;
}
impl<T: UserDefinedTypeNameBaseRef> UserDefinedTypeNameBaseProperty for T {
    fn names(&self) -> &Vec<RcCell<Identifier>> {
        &self.user_defined_type_name_base_ref().names
    }
    // fn target(&self) -> &Option<ASTFlattenWeak> {
    //     &self.user_defined_type_name_base_ref().target
    // }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(
    ImplBaseTrait,
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
pub struct UserDefinedTypeNameBase {
    pub type_name_base: TypeNameBase,
    pub names: Vec<RcCell<Identifier>>,
}
impl DeepClone for UserDefinedTypeNameBase {
    fn clone_inner(&self) -> Self {
        Self {
            type_name_base: self.type_name_base.clone_inner(),
            names: self.names.clone_inner(),
        }
    }
}
impl FullArgsSpec for UserDefinedTypeNameBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.names
                    .iter()
                    .map(|name| ArgType::ASTFlatten(Some(ASTFlatten::from(name.clone_inner()))))
                    .collect(),
            ),
            ArgType::ASTFlattenWeak(self.target().clone()),
        ]
    }
}
impl FullArgsSpecInit for UserDefinedTypeNameBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        UserDefinedTypeNameBase::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_identifier()
                        .unwrap()
                })
                .collect(),
            fields[1].clone().try_as_ast_flatten_weak().flatten(),
        )
    }
}
impl UserDefinedTypeNameBase {
    pub fn new(names: Vec<RcCell<Identifier>>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(target),
            names,
        }
    }
}
impl IntoAST for UserDefinedTypeNameBase {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::UserDefinedTypeName(self),
        ))
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
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
pub struct EnumTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl DeepClone for EnumTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            user_defined_type_name_base: self.user_defined_type_name_base.clone_inner(),
        }
    }
}
impl IntoAST for EnumTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::EnumTypeName(self),
        ))
    }
}
impl FullArgsSpec for EnumTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.user_defined_type_name_base
                    .names
                    .iter()
                    .map(|name| ArgType::ASTFlatten(Some(ASTFlatten::from(name.clone_inner()))))
                    .collect(),
            ),
            ArgType::ASTFlattenWeak(self.target().clone()),
        ]
    }
}
impl FullArgsSpecInit for EnumTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        EnumTypeName::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_identifier()
                        .unwrap()
                })
                .collect(),
            fields[1].clone().try_as_ast_flatten_weak().flatten(),
        )
    }
}
impl EnumTypeName {
    pub fn new(names: Vec<RcCell<Identifier>>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(names, target),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        256
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
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
pub struct EnumValueTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl DeepClone for EnumValueTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            user_defined_type_name_base: self.user_defined_type_name_base.clone_inner(),
        }
    }
}
impl IntoAST for EnumValueTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::EnumValueTypeName(self),
        ))
    }
}
impl FullArgsSpec for EnumValueTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.user_defined_type_name_base
                    .names
                    .iter()
                    .map(|name| ArgType::ASTFlatten(Some(ASTFlatten::from(name.clone_inner()))))
                    .collect(),
            ),
            ArgType::ASTFlattenWeak(self.target().clone()),
        ]
    }
}
impl FullArgsSpecInit for EnumValueTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        EnumValueTypeName::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_identifier()
                        .unwrap()
                })
                .collect(),
            fields[1].clone().try_as_ast_flatten_weak().flatten(),
        )
    }
}
impl EnumValueTypeName {
    pub fn new(names: Vec<RcCell<Identifier>>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(names, target),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        256
    }
    pub fn to_abstract_type(&self) -> ASTFlatten {
        let mut names = self.user_defined_type_name_base.names.clone();
        names.pop();
        RcCell::new(
            EnumTypeName::new(
                names,
                self.ast_base_ref()
                    .borrow()
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .parent(),
            )
            .into_ast()
            .try_as_type_name()
            .unwrap(),
        )
        .into()
    }
    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        // println!("==evt=====implicitly_convertible_to={:?}==={:?},{:?},{:?},========={:?}",expected
        //             .borrow()
        //             .try_as_user_defined_type_name_ref()
        //             .unwrap()
        //             .try_as_enum_type_name_ref()
        //             .unwrap()
        //             .user_defined_type_name_base_ref()
        //             .names.iter().zip(&self.user_defined_type_name_base.names
        //                 [..self.user_defined_type_name_base.names.len().saturating_sub(1)])
        //             .all(|(e,s)| {println!("e.borrow().name()==s.borrow().name()======================{:?},================={:?}",e.borrow().name(),s.borrow().name());e.borrow().name()==s.borrow().name()}), &TypeName::UserDefinedTypeName(
        //     UserDefinedTypeName::EnumValueTypeName(self.clone()),
        // ) == &*expected.borrow()
        //     , is_instance(expected, ASTType::EnumTypeName)
        //         , expected
        //             .borrow()
        //             .try_as_user_defined_type_name_ref()
        //             .unwrap()
        //             .try_as_enum_type_name_ref()
        //             .unwrap()
        //             .user_defined_type_name_base_ref()
        //             .names
        //             .clone()
        //             , self.user_defined_type_name_base.names);
        TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(self.clone()))
            == expected.to_ast().try_as_type_name().unwrap()
            || (is_instance(expected, ASTType::EnumTypeName)
                && expected
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_user_defined_type_name_ref()
                    .unwrap()
                    .try_as_enum_type_name_ref()
                    .unwrap()
                    .user_defined_type_name_base_ref()
                    .names
                    .iter()
                    .zip(
                        &self.user_defined_type_name_base.names[..self
                            .user_defined_type_name_base
                            .names
                            .len()
                            .saturating_sub(1)],
                    )
                    .all(|(e, s)| e.borrow().name() == s.borrow().name()))
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
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
pub struct StructTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl DeepClone for StructTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            user_defined_type_name_base: self.user_defined_type_name_base.clone_inner(),
        }
    }
}
impl IntoAST for StructTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::StructTypeName(self),
        ))
    }
}
impl FullArgsSpec for StructTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.user_defined_type_name_base
                    .names
                    .iter()
                    .map(|name| ArgType::ASTFlatten(Some(ASTFlatten::from(name.clone_inner()))))
                    .collect(),
            ),
            ArgType::ASTFlattenWeak(self.target().clone()),
        ]
    }
}
impl FullArgsSpecInit for StructTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        StructTypeName::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_identifier()
                        .unwrap()
                })
                .collect(),
            fields[1].clone().try_as_ast_flatten_weak().flatten(),
        )
    }
}
impl StructTypeName {
    pub fn new(names: Vec<RcCell<Identifier>>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(names, target),
        }
    }
    pub fn to_type_name(&self) -> TypeName {
        TypeName::UserDefinedTypeName(UserDefinedTypeName::StructTypeName(self.clone()))
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
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
pub struct ContractTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl DeepClone for ContractTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            user_defined_type_name_base: self.user_defined_type_name_base.clone_inner(),
        }
    }
}
impl IntoAST for ContractTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::ContractTypeName(self),
        ))
    }
}
impl FullArgsSpec for ContractTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.user_defined_type_name_base
                    .names
                    .iter()
                    .map(|name| ArgType::ASTFlatten(Some(ASTFlatten::from(name.clone_inner()))))
                    .collect(),
            ),
            ArgType::ASTFlattenWeak(self.target().clone()),
        ]
    }
}
impl FullArgsSpecInit for ContractTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ContractTypeName::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_identifier()
                        .unwrap()
                })
                .collect(),
            fields[1].clone().try_as_ast_flatten_weak().flatten(),
        )
    }
}
impl ContractTypeName {
    pub fn new(names: Vec<RcCell<Identifier>>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(names, target),
        }
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash,
)]
pub struct AddressTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl DeepClone for AddressTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            user_defined_type_name_base: self.user_defined_type_name_base.clone_inner(),
        }
    }
}
impl PartialEq for AddressTypeName {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for AddressTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::AddressTypeName(self),
        ))
    }
}
impl FullArgsSpec for AddressTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for AddressTypeName {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        AddressTypeName::new()
    }
}
impl AddressTypeName {
    pub fn new() -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                vec![RcCell::new(Identifier::Identifier(IdentifierBase::new(
                    String::from("<address>"),
                )))],
                None,
            ),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        160
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash,
)]
pub struct AddressPayableTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl DeepClone for AddressPayableTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            user_defined_type_name_base: self.user_defined_type_name_base.clone_inner(),
        }
    }
}
impl PartialEq for AddressPayableTypeName {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for AddressPayableTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::AddressPayableTypeName(self),
        ))
    }
}
impl FullArgsSpec for AddressPayableTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for AddressPayableTypeName {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        AddressPayableTypeName::new()
    }
}
impl AddressPayableTypeName {
    pub fn new() -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                vec![RcCell::new(Identifier::Identifier(IdentifierBase::new(
                    String::from("<address_payable>"),
                )))],
                None,
            ),
        }
    }

    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(self.clone()))
            == expected.to_ast().try_as_type_name().unwrap()
            || expected.to_ast().try_as_type_name().unwrap() == TypeName::address_type()
    }
    pub fn elem_bitwidth(&self) -> i32 {
        160
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum KeyLabelUnion {
    String(String),
    Identifier(Option<Identifier>),
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct Mapping {
    pub type_name_base: TypeNameBase,
    pub key_type: RcCell<TypeName>,
    pub key_label: Option<RcCell<Identifier>>,
    pub value_type: RcCell<AnnotatedTypeName>,
    pub instantiated_key: Option<ASTFlatten>,
}
impl DeepClone for Mapping {
    fn clone_inner(&self) -> Self {
        Self {
            type_name_base: self.type_name_base.clone_inner(),
            key_type: self.key_type.clone_inner(),
            key_label: self.key_label.clone_inner(),
            value_type: self.value_type.clone_inner(),
            instantiated_key: self.instantiated_key.clone_inner(),
        }
    }
}
impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.key_type == other.key_type && self.value_type == other.value_type
    }
}
impl IntoAST for Mapping {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Mapping(self))
    }
}
impl FullArgsSpec for Mapping {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.key_type.clone_inner()))),
            ArgType::ASTFlatten(
                self.key_label
                    .as_ref()
                    .map(|kl| ASTFlatten::from(kl.clone_inner())),
            ),
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.value_type.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for Mapping {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        Mapping::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_type_name()
                .unwrap(),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
            fields[2]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_annotated_type_name()
                .unwrap(),
        )
    }
}
impl Mapping {
    pub fn new(
        key_type: RcCell<TypeName>,
        key_label: Option<RcCell<Identifier>>,
        value_type: RcCell<AnnotatedTypeName>,
    ) -> Self {
        Self {
            type_name_base: TypeNameBase::new(None),
            key_type,
            key_label,
            value_type,
            instantiated_key: None,
        }
    }
    pub fn has_key_label(&self) -> bool {
        self.key_label.is_some()
    }
    pub fn clone_owned(&self, global_vars: RcCell<GlobalVars>) -> Option<ASTFlatten> {
        use crate::visitors::deep_copy::deep_copy;
        deep_copy(
            &RcCell::new(self.clone()).into(),
            false,
            false,
            global_vars.clone(),
        )
    }
}
impl ASTChildren for Mapping {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.key_type.clone().into());
        if let Some(idf) = &self.key_label {
            if is_instance(idf, ASTType::IdentifierBase) {
                cb.add_child(idf.clone().into());
            }
        }
        cb.add_child(self.value_type.clone().into());
    }
}

impl ASTChildrenCallBack for Mapping {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        let kt = f(&self.key_type.clone().into())
            .unwrap()
            .try_as_type_name()
            .unwrap()
            .borrow()
            .clone();
        *self.key_type.borrow_mut() = kt;
        if let Some(idf) = self.key_label.as_ref() {
            if is_instance(idf, ASTType::IdentifierBase) {
                let _idf = f(&idf.clone().into())
                    .unwrap()
                    .try_as_identifier()
                    .unwrap()
                    .borrow()
                    .clone();
                *idf.borrow_mut() = _idf;
            }
        }
        *self.value_type.borrow_mut() = f(&self.value_type.clone().into())
            .unwrap()
            .try_as_annotated_type_name()
            .unwrap()
            .borrow()
            .clone();
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ExprUnion {
    I32(i32),
    Expression(ASTFlatten),
}

#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ArrayBaseRef,
    TypeNameBaseRef,
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
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub enum Array {
    CipherText(CipherText),
    Randomness(Randomness),
    Key(Key),
    Proof(Proof),
    Array(ArrayBase),
}
impl PartialEq for ArrayBase {
    fn eq(&self, other: &Self) -> bool {
        if self.value_type() != other.value_type() {
            return false;
        }
        if self.expr().as_ref().zip(other.expr().as_ref()).map_or_else(
            || self.expr().as_ref().or(other.expr().as_ref()).is_none(),
            |(expr, other_expr)| {
                is_instance(expr, ASTType::NumberLiteralExpr)
                    && is_instance(other_expr, ASTType::NumberLiteralExpr)
                    && expr
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap()
                        .value
                        == other_expr
                            .to_ast()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_literal_expr_ref()
                            .unwrap()
                            .try_as_number_literal_expr_ref()
                            .unwrap()
                            .value
            },
        ) {
            return true;
        }

        false
    }
}
impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        if self.value_type() != other.value_type() {
            return false;
        }
        if self.expr().as_ref().zip(other.expr().as_ref()).map_or_else(
            || self.expr().as_ref().or(other.expr().as_ref()).is_none(),
            |(expr, other_expr)| {
                is_instance(expr, ASTType::NumberLiteralExpr)
                    && is_instance(other_expr, ASTType::NumberLiteralExpr)
                    && expr
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap()
                        .value
                        == other_expr
                            .to_ast()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_literal_expr_ref()
                            .unwrap()
                            .try_as_number_literal_expr_ref()
                            .unwrap()
                            .value
            },
        ) {
            return true;
        }

        false
    }
}

impl ASTChildren for ArrayBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.value_type.clone().into());
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone());
        }
    }
}

impl ASTChildrenCallBack for ArrayBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        *self.value_type.borrow_mut() = f(&self.value_type.clone().into())
            .unwrap()
            .try_as_annotated_type_name()
            .unwrap()
            .borrow()
            .clone();
        self.expr
            .as_ref()
            .unwrap()
            .assign(f(self.expr.as_ref().unwrap()).as_ref().unwrap());
    }
}

#[enum_dispatch]
pub trait ArrayBaseRef: TypeNameBaseRef {
    fn array_base_ref(&self) -> &ArrayBase;
}

pub trait ArrayBaseProperty {
    fn value_type(&self) -> &RcCell<AnnotatedTypeName>;
    fn expr(&self) -> &Option<ASTFlatten>;
    fn elem_bitwidth(&self) -> i32;
    fn crypto_params(&self) -> &Option<CryptoParams>;
}
impl<T: ArrayBaseRef> ArrayBaseProperty for T {
    fn value_type(&self) -> &RcCell<AnnotatedTypeName> {
        &self.array_base_ref().value_type
    }
    fn expr(&self) -> &Option<ASTFlatten> {
        &self.array_base_ref().expr
    }
    fn elem_bitwidth(&self) -> i32 {
        self.value_type()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .elem_bitwidth()
    }
    fn crypto_params(&self) -> &Option<CryptoParams> {
        &self.array_base_ref().crypto_params
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(
    ImplBaseTrait, ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash,
)]
pub struct ArrayBase {
    pub type_name_base: TypeNameBase,
    pub value_type: RcCell<AnnotatedTypeName>,
    pub expr: Option<ASTFlatten>,
    pub crypto_params: Option<CryptoParams>,
}
impl DeepClone for ArrayBase {
    fn clone_inner(&self) -> Self {
        Self {
            type_name_base: self.type_name_base.clone_inner(),
            value_type: self.value_type.clone_inner(),
            expr: self.expr.clone_inner(),
            crypto_params: self.crypto_params.clone(),
        }
    }
}
impl IntoAST for ArrayBase {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Array(self)))
    }
}
impl FullArgsSpec for ArrayBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.value_type.clone_inner()))),
            ArgType::ASTFlatten(self.expr.as_ref().map(|a| a.clone_inner())),
            ArgType::CryptoParams(self.crypto_params.clone()),
        ]
    }
}
impl FullArgsSpecInit for ArrayBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ArrayBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_annotated_type_name()
                .unwrap(),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .unwrap()
                .map(|a| ExprUnion::Expression(a.clone())),
            fields[2].clone().try_as_crypto_params().flatten(),
        )
    }
}
impl ArrayBase {
    pub fn new(
        value_type: RcCell<AnnotatedTypeName>,
        expr: Option<ExprUnion>,
        crypto_params: Option<CryptoParams>,
    ) -> Self {
        Self {
            type_name_base: TypeNameBase::new(None),
            value_type,
            expr: expr.map(|_expr| match _expr {
                ExprUnion::I32(exp) => RcCell::new(NumberLiteralExpr::new(exp, false)).into(),
                ExprUnion::Expression(exp) => exp,
            }),
            crypto_params,
        }
    }
    pub fn size_in_uints(&self) -> i32 {
        if self.expr.is_some()
            && is_instance(self.expr.as_ref().unwrap(), ASTType::NumberLiteralExpr)
        {
            // println!("{:?}",self.expr);
            return self
                .expr
                .as_ref()
                .unwrap()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_literal_expr_ref()
                .unwrap()
                .try_as_number_literal_expr_ref()
                .unwrap()
                .value;
        }
        -1
    }
}

#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct CipherText {
    pub array_base: ArrayBase,
    pub plain_type: Option<RcCell<AnnotatedTypeName>>,
}
impl DeepClone for CipherText {
    fn clone_inner(&self) -> Self {
        Self {
            array_base: self.array_base.clone_inner(),
            plain_type: self.plain_type.clone_inner(),
        }
    }
}
impl PartialEq for CipherText {
    fn eq(&self, other: &Self) -> bool {
        (self.plain_type.is_none() || self.plain_type == other.plain_type)
            && self.array_base.crypto_params == other.array_base.crypto_params
    }
}
impl IntoAST for CipherText {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::CipherText(self)))
    }
}

impl ASTChildren for CipherText {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.array_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for CipherText {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.array_base.process_children_callback(f);
    }
}

impl FullArgsSpec for CipherText {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.plain_type
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::CryptoParams(self.crypto_params().clone()),
        ]
    }
}
impl FullArgsSpecInit for CipherText {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        CipherText::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[1].clone().try_as_crypto_params().flatten().unwrap(),
        )
    }
}
impl CipherText {
    pub fn new(plain_type: Option<RcCell<AnnotatedTypeName>>, crypto_params: CryptoParams) -> Self {
        assert!(!plain_type.as_ref().map_or(false, |pt| {
            pt.borrow()
                .type_name
                .as_ref()
                .unwrap()
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .is_cipher()
        }));
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(
                    RcCell::new(NumberLiteralExpr::new(crypto_params.cipher_len(), false)).into(),
                )),
                Some(crypto_params),
            ),
            plain_type,
        }
    }
    pub fn size_in_uints(&self) -> i32 {
        self.array_base
            .crypto_params
            .as_ref()
            .unwrap()
            .cipher_payload_len()
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct Randomness {
    pub array_base: ArrayBase,
}
impl DeepClone for Randomness {
    fn clone_inner(&self) -> Self {
        Self {
            array_base: self.array_base.clone_inner(),
        }
    }
}
impl PartialEq for Randomness {
    fn eq(&self, other: &Self) -> bool {
        self.array_base.crypto_params == other.array_base.crypto_params
    }
}
impl IntoAST for Randomness {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Randomness(self)))
    }
}
impl ASTChildren for Randomness {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.array_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for Randomness {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.array_base.process_children_callback(f);
    }
}

impl FullArgsSpec for Randomness {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::CryptoParams(self.crypto_params().clone())]
    }
}
impl FullArgsSpecInit for Randomness {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        Randomness::new(fields[0].clone().try_as_crypto_params().flatten().unwrap())
    }
}
impl Randomness {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                crypto_params.randomness_len().map(|randomness_len| {
                    ExprUnion::Expression(
                        RcCell::new(NumberLiteralExpr::new(randomness_len, false)).into(),
                    )
                }),
                Some(crypto_params),
            ),
        }
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct Key {
    pub array_base: ArrayBase,
}
impl DeepClone for Key {
    fn clone_inner(&self) -> Self {
        Self {
            array_base: self.array_base.clone_inner(),
        }
    }
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.array_base.crypto_params == other.array_base.crypto_params
    }
}
impl IntoAST for Key {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Key(self)))
    }
}

impl ASTChildren for Key {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.array_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for Key {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.array_base.process_children_callback(f);
    }
}

impl FullArgsSpec for Key {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::CryptoParams(self.crypto_params().clone())]
    }
}
impl FullArgsSpecInit for Key {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        Key::new(fields[0].clone().try_as_crypto_params().flatten().unwrap())
    }
}
impl Key {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(
                    RcCell::new(NumberLiteralExpr::new(crypto_params.key_len(), false)).into(),
                )),
                Some(crypto_params),
            ),
        }
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct Proof {
    pub array_base: ArrayBase,
}
impl DeepClone for Proof {
    fn clone_inner(&self) -> Self {
        Self {
            array_base: self.array_base.clone_inner(),
        }
    }
}
impl PartialEq for Proof {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl IntoAST for Proof {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Proof(self)))
    }
}
impl ASTChildren for Proof {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.array_base.process_children(cb);
    }
}
impl ASTChildrenCallBack for Proof {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.array_base.process_children_callback(f);
    }
}

impl FullArgsSpec for Proof {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for Proof {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        Proof::new()
    }
}
impl Proof {
    pub fn new() -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(
                    RcCell::new(NumberLiteralExpr::new(
                        CFG.lock().unwrap().proof_len(),
                        false,
                    ))
                    .into(),
                )),
                None,
            ),
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
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct DummyAnnotation {
    pub expression_base: ExpressionBase,
}
impl DeepClone for DummyAnnotation {
    fn clone_inner(&self) -> Self {
        Self {
            expression_base: self.expression_base.clone_inner(),
        }
    }
}
impl IntoAST for DummyAnnotation {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::DummyAnnotation(self))
    }
}
impl FullArgsSpec for DummyAnnotation {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for DummyAnnotation {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        DummyAnnotation::new()
    }
}
impl DummyAnnotation {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(None, None),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CombinedPrivacyUnion {
    Vec(Vec<CombinedPrivacyUnion>),
    AST(Option<ASTFlatten>),
}
impl CombinedPrivacyUnion {
    pub fn as_expression(self) -> Option<ASTFlatten> {
        if let CombinedPrivacyUnion::AST(expr) = self {
            expr
        } else {
            None
        }
    }
}
//     """Does not appear in the syntax, but is necessary for type checking"""
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct TupleType {
    pub type_name_base: TypeNameBase,
    pub types: Vec<RcCell<AnnotatedTypeName>>,
}
impl DeepClone for TupleType {
    fn clone_inner(&self) -> Self {
        Self {
            type_name_base: self.type_name_base.clone_inner(),
            types: self.types.clone_inner(),
        }
    }
}
impl PartialEq for TupleType {
    fn eq(&self, other: &Self) -> bool {
        self.check_component_wise(
            &RcCell::new(TypeName::TupleType(other.clone())).into(),
            |x, y| x == y,
        )
    }
}
impl IntoAST for TupleType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::TupleType(self))
    }
}
impl FullArgsSpec for TupleType {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Vec(
            self.types
                .iter()
                .map(|tn| ArgType::ASTFlatten(Some(ASTFlatten::from(tn.clone_inner()))))
                .collect(),
        )]
    }
}
impl FullArgsSpecInit for TupleType {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        TupleType::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_annotated_type_name()
                        .unwrap()
                })
                .collect(),
        )
    }
}
impl TupleType {
    pub fn new(types: Vec<RcCell<AnnotatedTypeName>>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(None),
            types,
        }
    }
    pub fn ensure_tuple(t: Option<AnnotatedTypeName>) -> TupleType {
        if let Some(t) = t {
            if let Some(TypeName::TupleType(t)) = t
                .type_name
                .as_ref()
                .and_then(|t| t.to_ast().try_as_type_name())
            {
                t.clone()
            } else {
                TupleType::new(vec![RcCell::new(t.clone())])
            }
        } else {
            TupleType::empty()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> i32 {
        self.types.len() as i32
    }

    pub fn get_item(&self, i: i32) -> RcCell<AnnotatedTypeName> {
        self.types[i as usize].clone()
    }

    pub fn check_component_wise(
        &self,
        other: &ASTFlatten,
        f: impl FnOnce(RcCell<AnnotatedTypeName>, RcCell<AnnotatedTypeName>) -> bool + std::marker::Copy,
    ) -> bool {
        if !is_instance(other, ASTType::TupleType) {
            return false;
        }
        if self.len()
            != other
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_tuple_type_ref()
                .unwrap()
                .len()
        {
            return false;
        }
        (0..self.len()).all(|i| {
            f(
                self.get_item(i),
                other
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_tuple_type_ref()
                    .unwrap()
                    .get_item(i),
            )
        })
    }

    pub fn implicitly_convertible_to(&self, expected: &ASTFlatten) -> bool {
        self.check_component_wise(expected, |x, y| {
            x.borrow()
                .type_name
                .as_ref()
                .unwrap()
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .implicitly_convertible_to(y.borrow().type_name.as_ref().unwrap())
        })
    }

    pub fn compatible_with(&self, other_type: ASTFlatten) -> bool {
        if other_type
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_tuple_type()
        {
            self.check_component_wise(&other_type, |x, y| {
                x.borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .compatible_with(y.borrow().type_name.as_ref().unwrap())
            })
        } else {
            false
        }
    }

    pub fn combined_type(
        &self,
        other_type: &ASTFlatten,
        convert_literals: bool,
    ) -> Option<ASTFlatten> {
        if self.types.len()
            != other_type
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_tuple_type_ref()
                .unwrap()
                .types
                .len()
        {
            None
        } else {
            Some(
                RcCell::new(TypeName::TupleType(TupleType::new(
                    self.types
                        .iter()
                        .zip(
                            &other_type
                                .to_ast()
                                .try_as_type_name()
                                .unwrap()
                                .try_as_tuple_type_ref()
                                .unwrap()
                                .types,
                        )
                        .map(|(e1, e2)| {
                            RcCell::new(
                                AnnotatedTypeName::new(
                                    e1.borrow()
                                        .type_name
                                        .as_ref()
                                        .unwrap()
                                        .to_ast()
                                        .try_as_type_name()
                                        .unwrap()
                                        .combined_type(
                                            e2.borrow().type_name.as_ref().unwrap(),
                                            convert_literals,
                                        ),
                                    Some(
                                        RcCell::new(Expression::DummyAnnotation(
                                            DummyAnnotation::new(),
                                        ))
                                        .into(),
                                    ),
                                    Homomorphism::non_homomorphic(),
                                )
                                .into(),
                            )
                        })
                        .collect(),
                )))
                .into(),
            )
        }
    }
    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> CombinedPrivacyUnion {
        CombinedPrivacyUnion::AST(match privacy_annotation {
            CombinedPrivacyUnion::AST(_) => Some(
                RcCell::new(AnnotatedTypeName::new(
                    Some(
                        RcCell::new(TypeName::TupleType(TupleType::new(
                            self.types
                                .iter()
                                .map(|t| {
                                    t.borrow()
                                        .type_name
                                        .as_ref()
                                        .unwrap()
                                        .to_ast()
                                        .try_as_type_name()
                                        .unwrap()
                                        .annotate(privacy_annotation.clone())
                                })
                                .collect(),
                        )))
                        .into(),
                    ),
                    None,
                    Homomorphism::non_homomorphic(),
                ))
                .into(),
            ),
            CombinedPrivacyUnion::Vec(privacy_annotation) => {
                assert!(self.types.len() == privacy_annotation.len());
                Some(
                    RcCell::new(AnnotatedTypeName::new(
                        Some(
                            RcCell::new(TypeName::TupleType(TupleType::new(
                                self.types
                                    .iter()
                                    .zip(privacy_annotation)
                                    .map(|(t, a)| {
                                        t.borrow()
                                            .type_name
                                            .as_ref()
                                            .unwrap()
                                            .to_ast()
                                            .try_as_type_name()
                                            .unwrap()
                                            .annotate(a.clone())
                                    })
                                    .collect(),
                            )))
                            .into(),
                        ),
                        None,
                        Homomorphism::non_homomorphic(),
                    ))
                    .into(),
                )
            }
        })
    }
    pub fn perfect_privacy_match(&self, other: &Self) -> bool {
        fn privacy_match(
            selfs: RcCell<AnnotatedTypeName>,
            other: RcCell<AnnotatedTypeName>,
        ) -> bool {
            selfs.borrow().privacy_annotation == other.borrow().privacy_annotation
        }

        self.check_component_wise(
            &RcCell::new(TypeName::TupleType(other.clone())).into(),
            privacy_match,
        )
    }

    pub fn empty() -> TupleType {
        TupleType::new(vec![])
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct FunctionTypeName {
    pub type_name_base: TypeNameBase,
    pub parameters: Vec<RcCell<Parameter>>,
    pub modifiers: Vec<String>,
    pub return_parameters: Vec<RcCell<Parameter>>,
}
impl DeepClone for FunctionTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            type_name_base: self.type_name_base.clone_inner(),
            parameters: self.parameters.clone_inner(),
            modifiers: self.modifiers.clone(),
            return_parameters: self.return_parameters.clone_inner(),
        }
    }
}
impl PartialEq for FunctionTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.parameters == other.parameters
            && self.modifiers == other.modifiers
            && self.return_parameters == other.return_parameters
    }
}
impl IntoAST for FunctionTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::FunctionTypeName(self))
    }
}
impl FullArgsSpec for FunctionTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.parameters
                    .iter()
                    .map(|tn| ArgType::ASTFlatten(Some(ASTFlatten::from(tn.clone_inner()))))
                    .collect(),
            ),
            ArgType::Vec(
                self.modifiers
                    .iter()
                    .map(|tn| ArgType::Str(Some(tn.clone())))
                    .collect(),
            ),
            ArgType::Vec(
                self.return_parameters
                    .iter()
                    .map(|tn| ArgType::ASTFlatten(Some(ASTFlatten::from(tn.clone_inner()))))
                    .collect(),
            ),
        ]
    }
}
impl FullArgsSpecInit for FunctionTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        FunctionTypeName::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_parameter()
                        .unwrap()
                })
                .collect(),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| astf.try_as_str().flatten().unwrap())
                .collect(),
            fields[2]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_parameter()
                        .unwrap()
                })
                .collect(),
        )
    }
}
impl FunctionTypeName {
    pub fn new(
        parameters: Vec<RcCell<Parameter>>,
        modifiers: Vec<String>,
        return_parameters: Vec<RcCell<Parameter>>,
    ) -> Self {
        Self {
            type_name_base: TypeNameBase::new(None),
            parameters,
            modifiers,
            return_parameters,
        }
    }
}
impl ASTChildren for FunctionTypeName {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.clone().into());
        });
        self.return_parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.clone().into());
        });
    }
}

impl ASTChildrenCallBack for FunctionTypeName {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.parameters.iter().for_each(|parameter| {
            *parameter.borrow_mut() = f(&parameter.clone().into())
                .unwrap()
                .try_as_parameter()
                .unwrap()
                .borrow()
                .clone();
        });
        self.return_parameters.iter().for_each(|parameter| {
            *parameter.borrow_mut() = f(&parameter.clone().into())
                .unwrap()
                .try_as_parameter()
                .unwrap()
                .borrow()
                .clone();
        });
    }
}
