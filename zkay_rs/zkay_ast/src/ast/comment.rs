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
    statement::IndentBlock, ASTBase, ASTBaseMutRef, ASTBaseRef, ASTFlatten, ArgType, DeepClone,
    FullArgsSpec, FullArgsSpecInit, IntoAST, AST,
};
use crate::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use crate::global_defs::{array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars};
use crate::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};
use crate::visitors::code_visitor::CodeVisitorBase;
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use enum_dispatch::enum_dispatch;
use ethnum::{i256, int, u256, uint, AsI256, AsU256, I256, U256};
use eyre::{eyre, Result};
use lazy_static::lazy_static;
use rccell::{RcCell, WeakCell};
use serde::{Deserialize, Deserializer, Serialize};
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
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    CommentBaseRef,
    ASTBaseRef
)]
#[derive(
    EnumDispatchWithFields, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum Comment {
    Comment(CommentBase),
    BlankLine(BlankLine),
}

#[enum_dispatch]
pub trait CommentBaseRef: ASTBaseRef {
    fn comment_base_ref(&self) -> &CommentBase;
}
pub trait CommentBaseProperty {
    fn text(&self) -> &String;
    fn code(&self) -> String {
        self.text().clone()
    }
}
impl<T: CommentBaseRef> CommentBaseProperty for T {
    fn text(&self) -> &String {
        &self.comment_base_ref().text
    }
}

#[derive(
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
pub struct CommentBase {
    pub ast_base: RcCell<ASTBase>,
    pub text: String,
}
impl DeepClone for CommentBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            text: self.text.clone(),
        }
    }
}
impl IntoAST for CommentBase {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::Comment(self))
    }
}
impl FullArgsSpec for CommentBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.text.clone()))]
    }
}

impl FullArgsSpecInit for CommentBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        CommentBase::new(fields[0].clone().try_as_str().flatten().unwrap())
    }
}
impl CommentBase {
    pub fn new(text: String) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(None, None, None)),
            text,
        }
    }
    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
    pub fn comment_list(text: String, mut block: Vec<ASTFlatten>) -> Vec<ASTFlatten> {
        if !block.is_empty() {
            block.insert(0, RcCell::new(CommentBase::new(text)).into());
            block.push(RcCell::new(BlankLine::new()).into());
        }
        block
    }

    pub fn comment_wrap_block(text: String, block: Vec<ASTFlatten>) -> Vec<ASTFlatten> {
        if block.is_empty() {
            return block;
        }
        vec![
            RcCell::new(CommentBase::new(text)).into(),
            RcCell::new(CommentBase::new(String::from("{"))).into(),
            RcCell::new(IndentBlock::new(block)).into(),
            RcCell::new(CommentBase::new(String::from("}"))).into(),
            RcCell::new(BlankLine::new()).into(),
        ]
    }
}

#[impl_traits(CommentBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BlankLine {
    pub comment_base: CommentBase,
}
impl DeepClone for BlankLine {
    fn clone_inner(&self) -> Self {
        Self {
            comment_base: self.comment_base.clone_inner(),
        }
    }
}
impl IntoAST for BlankLine {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::BlankLine(self))
    }
}
impl FullArgsSpec for BlankLine {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}

impl FullArgsSpecInit for BlankLine {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        BlankLine::new()
    }
}
impl BlankLine {
    pub fn new() -> Self {
        Self {
            comment_base: CommentBase::new(String::new()),
        }
    }
}
