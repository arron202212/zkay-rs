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
    expression::{
        BooleanLiteralExpr, IdentifierExpr, IdentifierExprUnion, LocationExpr, NumberLiteralExpr,
        SliceExpr,
    },
    identifier::{Identifier, IdentifierBaseProperty},
    is_instance, is_instances,
    statement::{
        AssignmentStatement, AssignmentStatementBase, IndentBlock, Statement,
        VariableDeclarationStatement,
    },
    type_name::{
        ArrayBase, BooleanLiteralType, CombinedPrivacyUnion, DummyAnnotation, ExprUnion,
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
use zkay_crypto::params::CryptoParams;
use zkay_derive::{
    impl_trait, impl_traits, ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind,
    ASTVisitorBaseRefImpl, EnumDispatchWithDeepClone, EnumDispatchWithFields, ExpressionASTypeImpl,
    ImplBaseTrait,
};
use zkay_utils::progress_printer::warn_print;
use zkp_u256::{Zero, U256 as ZU256};
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    IdentifierDeclarationBaseRef,
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
pub enum IdentifierDeclaration {
    VariableDeclaration(VariableDeclaration),
    Parameter(Parameter),
    StateVariableDeclaration(StateVariableDeclaration),
}
#[enum_dispatch]
pub trait IdentifierDeclarationBaseRef: ASTBaseRef {
    fn identifier_declaration_base_ref(&self) -> &IdentifierDeclarationBase;
}
pub trait IdentifierDeclarationBaseProperty {
    fn keywords(&self) -> &Vec<String>;
    fn storage_location(&self) -> &Option<String>;
}
impl<T: IdentifierDeclarationBaseRef> IdentifierDeclarationBaseProperty for T {
    fn keywords(&self) -> &Vec<String> {
        &self.identifier_declaration_base_ref().keywords
    }
    fn storage_location(&self) -> &Option<String> {
        &self.identifier_declaration_base_ref().storage_location
    }
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierDeclarationBase {
    pub ast_base: RcCell<ASTBase>,
    pub keywords: Vec<String>,
    pub storage_location: Option<String>,
}
impl DeepClone for IdentifierDeclarationBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            ..self.clone()
        }
    }
}
impl IdentifierDeclarationBase {
    fn new(
        keywords: Vec<String>,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(annotated_type, idf, None)),
            keywords,
            storage_location,
        }
    }
    pub fn is_final(&self) -> bool {
        self.keywords.contains(&String::from("final"))
    }
    pub fn is_constant(&self) -> bool {
        self.keywords.contains(&String::from("constant"))
    }
}
impl ASTChildren for IdentifierDeclarationBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.annotated_type().clone().unwrap().into());
        if let Some(idf) = &self.idf() {
            // println!("===process_children===IdentifierDeclarationBase========={:?}",idf);
            cb.add_child(idf.clone().into());
        }
    }
}
impl ASTChildrenCallBack for IdentifierDeclarationBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.ast_base_ref().borrow_mut().annotated_type =
            self.annotated_type().as_ref().and_then(|at| {
                f(&at.clone().into()).and_then(|astf| astf.try_as_annotated_type_name())
            });

        self.ast_base_ref().borrow_mut().idf = self
            .idf()
            .as_ref()
            .and_then(|idf| f(&idf.clone().into()).and_then(|astf| astf.try_as_identifier()));
    }
}

#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclaration {
    pub identifier_declaration_base: IdentifierDeclarationBase,
}
impl DeepClone for VariableDeclaration {
    fn clone_inner(&self) -> Self {
        Self {
            identifier_declaration_base: self.identifier_declaration_base.clone_inner(),
        }
    }
}
impl IntoAST for VariableDeclaration {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(self))
    }
}

impl ASTChildren for VariableDeclaration {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.identifier_declaration_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for VariableDeclaration {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.identifier_declaration_base
            .process_children_callback(f);
    }
}

impl FullArgsSpec for VariableDeclaration {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.keywords()
                    .iter()
                    .map(|kw| ArgType::Str(Some(kw.clone())))
                    .collect(),
            ),
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
            ArgType::Str(self.storage_location().clone()),
        ]
    }
}
impl FullArgsSpecInit for VariableDeclaration {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        VariableDeclaration::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| astf.try_as_str().flatten().unwrap())
                .collect(),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[2]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
            fields[3].clone().try_as_str().unwrap(),
        )
    }
}
impl VariableDeclaration {
    pub fn new(
        keywords: Vec<String>,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            identifier_declaration_base: IdentifierDeclarationBase::new(
                keywords,
                annotated_type,
                idf,
                storage_location,
            ),
        }
    }
}

#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Parameter {
    pub identifier_declaration_base: IdentifierDeclarationBase,
}
impl DeepClone for Parameter {
    fn clone_inner(&self) -> Self {
        Self {
            identifier_declaration_base: self.identifier_declaration_base.clone_inner(),
        }
    }
}
impl IntoAST for Parameter {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(self))
    }
}
impl ASTChildren for Parameter {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.identifier_declaration_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for Parameter {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.identifier_declaration_base
            .process_children_callback(f);
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ParameterUnion {
    Parameter(RcCell<Parameter>),
    String(String),
}
impl FullArgsSpec for Parameter {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.keywords()
                    .iter()
                    .map(|kw| ArgType::Str(Some(kw.clone())))
                    .collect(),
            ),
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
            ArgType::Str(self.storage_location().clone()),
        ]
    }
}
impl FullArgsSpecInit for Parameter {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        Parameter::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| astf.try_as_str().flatten().unwrap())
                .collect(),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[2]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_identifier()),
            fields[3].clone().try_as_str().unwrap(),
        )
    }
}
impl Parameter {
    pub fn new(
        keywords: Vec<String>,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            identifier_declaration_base: IdentifierDeclarationBase::new(
                keywords,
                annotated_type,
                idf,
                storage_location,
            ),
        }
    }
    pub fn with_changed_storage(&mut self, match_storage: String, new_storage: String) -> Self {
        if self.identifier_declaration_base.storage_location == Some(match_storage) {
            self.identifier_declaration_base.storage_location = Some(new_storage);
        }
        self.clone()
    }
}

#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateVariableDeclaration {
    pub identifier_declaration_base: IdentifierDeclarationBase,
    pub expr: Option<ASTFlatten>,
}
impl DeepClone for StateVariableDeclaration {
    fn clone_inner(&self) -> Self {
        Self {
            identifier_declaration_base: self.identifier_declaration_base.clone_inner(),
            expr: self.expr.clone_inner(),
        }
    }
}
impl IntoAST for StateVariableDeclaration {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::StateVariableDeclaration(self))
    }
}
impl FullArgsSpec for StateVariableDeclaration {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::Vec(
                self.keywords()
                    .iter()
                    .map(|kw| ArgType::Str(Some(kw.clone())))
                    .collect(),
            ),
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
            ArgType::ASTFlatten(self.expr.as_ref().map(|e| e.clone_inner())),
        ]
    }
}
impl FullArgsSpecInit for StateVariableDeclaration {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        StateVariableDeclaration::new(
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
                .map(|astf| astf.try_as_str().flatten().unwrap())
                .collect(),
            fields[2]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_identifier()),
            fields[3].clone().try_as_ast_flatten().flatten(),
        )
    }
}
impl StateVariableDeclaration {
    pub fn new(
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        keywords: Vec<String>,
        idf: Option<RcCell<Identifier>>,
        expr: Option<ASTFlatten>,
    ) -> Self {
        Self {
            identifier_declaration_base: IdentifierDeclarationBase::new(
                keywords,
                annotated_type,
                idf,
                None,
            ),
            expr,
        }
    }
}
impl ASTChildren for StateVariableDeclaration {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.identifier_declaration_base.process_children(cb);
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone());
        }
    }
}
impl ASTChildrenCallBack for StateVariableDeclaration {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.identifier_declaration_base
            .process_children_callback(f);
        if self.expr.is_some() {
            self.expr
                .as_ref()
                .unwrap()
                .assign(f(self.expr.as_ref().unwrap()).as_ref().unwrap());
        }
    }
}
