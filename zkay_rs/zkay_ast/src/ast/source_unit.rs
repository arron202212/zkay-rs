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
    enum_value::EnumValue,
    expression::IdentifierExprUnion,
    identifier::{HybridArgumentIdf, Identifier, IdentifierBase},
    identifier_declaration::{Parameter, VariableDeclaration},
    is_instance, is_instances,
    namespace_definition::ContractDefinition,
    statement::{AssignmentStatement, AssignmentStatementBase, Block, IndentBlock},
    type_name::{
        BooleanLiteralType, CombinedPrivacyUnion, DummyAnnotation, ExprUnion, FunctionTypeName,
        NumberLiteralType, NumberLiteralTypeUnion, TupleType, TypeName,
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

#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SourceUnit {
    pub ast_base: RcCell<ASTBase>,
    pub pragma_directive: String,
    pub contracts: Vec<RcCell<ContractDefinition>>,
    pub used_contracts: Vec<String>,
    pub used_homomorphisms: Option<BTreeSet<String>>,
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
    pub original_code: Vec<String>,
}
impl DeepClone for SourceUnit {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            contracts: self.contracts.clone_inner(),
            ..self.clone()
        }
    }
}
impl IntoAST for SourceUnit {
    fn into_ast(self) -> AST {
        AST::SourceUnit(self)
    }
}
impl FullArgsSpec for SourceUnit {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Str(Some(self.pragma_directive.clone())),
            ArgType::Vec(
                self.contracts
                    .iter()
                    .map(|tn| ArgType::ASTFlatten(Some(ASTFlatten::from(tn.clone_inner()))))
                    .collect(),
            ),
            ArgType::Vec(
                self.used_contracts
                    .iter()
                    .map(|u| ArgType::Str(Some(u.clone())))
                    .collect(),
            ),
        ]
    }
}
impl FullArgsSpecInit for SourceUnit {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        SourceUnit::new(
            fields[0].clone().try_as_str().flatten().unwrap(),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| {
                    astf.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_contract_definition()
                        .unwrap()
                })
                .collect(),
            fields[2]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|v| v.try_as_str().flatten().unwrap())
                .collect(),
        )
    }
}
impl SourceUnit {
    pub fn new(
        pragma_directive: String,
        contracts: Vec<RcCell<ContractDefinition>>,
        used_contracts: Vec<String>,
    ) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(None, None, None)),
            pragma_directive,
            contracts,
            used_contracts,
            used_homomorphisms: None,
            used_crypto_backends: None,
            original_code: vec![],
        }
    }
    pub fn get_item(&self, key: &String) -> Option<ASTFlatten> {
        self.ast_base
            .borrow()
            .names()
            .get(key)
            .and_then(|c_identifier| {
                c_identifier
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .parent()
                    .as_ref()
                    .and_then(|p| p.clone().upgrade())
            })
    }
    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
}
impl ASTChildren for SourceUnit {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.contracts.iter().for_each(|contract| {
            cb.add_child(contract.clone().into());
        });
    }
}
impl ASTChildrenCallBack for SourceUnit {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.contracts.iter().for_each(|contract| {
            *contract.borrow_mut() = f(&contract.clone().into())
                .unwrap()
                .try_as_contract_definition()
                .unwrap()
                .borrow()
                .clone();
        });
    }
}
