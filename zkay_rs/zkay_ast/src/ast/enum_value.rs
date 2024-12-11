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
    identifier::Identifier, statement::IndentBlock, ASTBase, ASTBaseMutRef, ASTBaseProperty,
    ASTBaseRef, ASTChildren, ASTChildrenCallBack, ASTFlatten, ArgType, ChildListBuilder, DeepClone,
    FullArgsSpec, FullArgsSpecInit, IntoAST, AST,
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

#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumValue {
    pub ast_base: RcCell<ASTBase>,
}
impl DeepClone for EnumValue {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
        }
    }
}
impl IntoAST for EnumValue {
    fn into_ast(self) -> AST {
        AST::EnumValue(self)
    }
}
impl FullArgsSpec for EnumValue {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.idf().map(|tn| ASTFlatten::from(tn.clone_inner())),
        )]
    }
}
impl FullArgsSpecInit for EnumValue {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        EnumValue::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}
impl EnumValue {
    pub fn new(idf: Option<RcCell<Identifier>>) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(None, idf, None)),
        }
    }
}
impl ASTChildren for EnumValue {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf() {
            cb.add_child(idf.clone().into());
        }
    }
}
impl ASTChildrenCallBack for EnumValue {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.ast_base_ref().borrow_mut().idf = self
            .idf()
            .as_ref()
            .and_then(|idf| f(&idf.clone().into()).and_then(|astf| astf.try_as_identifier()));
    }
}
