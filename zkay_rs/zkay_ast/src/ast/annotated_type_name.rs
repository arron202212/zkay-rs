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
use zkay_crypto::params::CryptoParams;
use zkay_derive::{
    impl_trait, impl_traits, ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind,
    ASTVisitorBaseRefImpl, EnumDispatchWithDeepClone, EnumDispatchWithFields, ExpressionASTypeImpl,
    ImplBaseTrait,
};
use zkay_utils::progress_printer::warn_print;
use zkp_u256::{Zero, U256 as ZU256};

use crate::ast::{
    expression::{AllExpr, Expression, MeExpr},
    is_instance, is_instances,
    type_name::{Array, ArrayBase, CombinedPrivacyUnion, ExprUnion, TypeName},
    ASTBase, ASTChildren, ASTChildrenCallBack, ASTFlatten, ASTType, ArgType, ChildListBuilder,
    DeepClone, FullArgsSpec, FullArgsSpecInit, IntoAST, AST,
};

#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct AnnotatedTypeName {
    pub ast_base: RcCell<ASTBase>,
    pub type_name: Option<ASTFlatten>,
    pub had_privacy_annotation: bool,
    pub privacy_annotation: Option<ASTFlatten>,
    pub homomorphism: String,
}
impl DeepClone for AnnotatedTypeName {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            type_name: self.type_name.clone_inner(),
            had_privacy_annotation: self.had_privacy_annotation,
            privacy_annotation: self.privacy_annotation.clone_inner(),
            homomorphism: self.homomorphism.clone(),
        }
    }
}
impl PartialEq for AnnotatedTypeName {
    fn eq(&self, other: &Self) -> bool {
        // println!("{:?}===,{:?},=*****===={:?}===={:?},************{:?}===,{:?},=====",self.type_name,other.type_name , self.privacy_annotation,other.privacy_annotation , self.homomorphism,other.homomorphism);
        // println!("{:?}===,{:?},=*****===={:?}======",self.type_name==other.type_name , self.privacy_annotation==other.privacy_annotation , self.homomorphism==other.homomorphism);

        self.type_name == other.type_name
            && self.privacy_annotation == other.privacy_annotation
            && self.homomorphism == other.homomorphism
    }
}

impl IntoAST for AnnotatedTypeName {
    fn into_ast(self) -> AST {
        AST::AnnotatedTypeName(self)
    }
}
impl FullArgsSpec for AnnotatedTypeName {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.type_name.as_ref().map(|t| t.clone_inner())),
            ArgType::ASTFlatten(self.privacy_annotation.as_ref().map(|t| t.clone_inner())),
            ArgType::Str(Some(self.homomorphism.clone())),
        ]
    }
}
impl FullArgsSpecInit for AnnotatedTypeName {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        AnnotatedTypeName::new(
            fields[0].clone().try_as_ast_flatten().flatten(),
            fields[1].clone().try_as_ast_flatten().flatten(),
            fields[2].clone().try_as_str().flatten().unwrap(),
        )
    }
}
impl AnnotatedTypeName {
    pub fn new(
        type_name: Option<ASTFlatten>,
        mut privacy_annotation: Option<ASTFlatten>,
        homomorphism: String,
    ) -> Self {
        // println!("==AnnotatedTypeName::new====={type_name:?}======");
        let had_privacy_annotation = privacy_annotation.as_ref().is_some();
        privacy_annotation = privacy_annotation.or(Some(
            RcCell::new(Expression::AllExpr(AllExpr::new())).into(),
        ));
        assert!(
            !(privacy_annotation.is_some()
                && is_instance(privacy_annotation.as_ref().unwrap(), ASTType::AllExpr)
                && homomorphism != Homomorphism::non_homomorphic()),
            "Public type name cannot be homomorphic (got {:?}),{:?}",
            HOMOMORPHISM_STORE.lock().unwrap().get(&homomorphism),
            homomorphism
        );
        Self {
            ast_base: RcCell::new(ASTBase::new(None, None, None)),
            type_name,
            had_privacy_annotation,
            privacy_annotation,
            homomorphism,
        }
    }

    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
    pub fn zkay_type(&self) -> Self {
        if let Some(TypeName::Array(Array::CipherText(ct))) = self
            .type_name
            .as_ref()
            .and_then(|t| t.to_ast().try_as_type_name())
        {
            ct.plain_type.as_ref().unwrap().borrow().clone()
        } else {
            self.clone()
        }
    }
    pub fn combined_privacy(
        &self,
        analysis: Option<PartitionState<AST>>,
        other: &RcCell<AnnotatedTypeName>,
    ) -> Option<CombinedPrivacyUnion> {
        if let (Some(TypeName::TupleType(selfs)), Some(TypeName::TupleType(others))) = (
            self.type_name
                .as_ref()
                .and_then(|t| t.to_ast().try_as_type_name()),
            other
                .borrow()
                .type_name
                .as_ref()
                .and_then(|t| t.to_ast().try_as_type_name()),
        ) {
            assert!(selfs.types.len() == others.types.len());
            return Some(CombinedPrivacyUnion::Vec(
                selfs
                    .types
                    .iter()
                    .zip(others.types.clone())
                    .filter_map(|(e1, e2)| e1.borrow().combined_privacy(analysis.clone(), &e2))
                    .collect(),
            ));
        }
        if self.homomorphism != other.borrow().homomorphism && !self.is_public() {
            return None;
        }
        if other.borrow().privacy_annotation.is_none() || self.privacy_annotation.is_none() {
            return None;
        }
        let (other_privacy_annotation, self_privacy_annotation) = (
            other.borrow().privacy_annotation.clone().unwrap(),
            self.privacy_annotation.clone().unwrap(),
        );
        let p_expected = other_privacy_annotation
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .privacy_annotation_label();
        let p_actual = self_privacy_annotation
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .privacy_annotation_label();
        if let (Some(p_expected), Some(p_actual)) = (p_expected, p_actual) {
            if p_expected == p_actual
                || (analysis.is_some()
                    && analysis
                        .unwrap()
                        .same_partition(&p_expected.to_ast(), &p_actual.to_ast()))
            {
                Some(CombinedPrivacyUnion::AST(Some(
                    self_privacy_annotation.clone(),
                )))
            } else if self_privacy_annotation
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .is_all_expr()
            {
                Some(CombinedPrivacyUnion::AST(Some(
                    other_privacy_annotation.clone(),
                )))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_public(&self) -> bool {
        self.privacy_annotation.as_ref().map_or(false, |pa| {
            pa.try_as_expression_ref()
                .map_or(false, |expr| expr.borrow().is_all_expr())
        })
    }

    pub fn is_private(&self) -> bool {
        !self.is_public()
    }
    pub fn is_private_at_me(&self, analysis: &Option<PartitionState<AST>>) -> bool {
        self.privacy_annotation.as_ref().map_or(false, |pa| {
            pa.try_as_expression_ref().map_or(false, |p| {
                p.borrow().is_me_expr()
                    || (analysis.is_some()
                        && analysis.as_ref().unwrap().same_partition(
                            &p.borrow().privacy_annotation_label().unwrap().to_ast(),
                            &MeExpr::new().into_ast(),
                        ))
            })
        })
    }
    pub fn is_accessible(&self, analysis: &Option<PartitionState<AST>>) -> bool {
        self.is_public() || self.is_private_at_me(analysis)
    }

    pub fn is_address(&self) -> bool {
        is_instances(
            self.type_name.as_ref().unwrap(),
            vec![ASTType::AddressTypeName, ASTType::AddressPayableTypeName],
        )
    }
    pub fn is_cipher(&self) -> bool {
        // println!("=======type_name=====*******===get_ast_type========{:?}",self.type_name.as_ref().unwrap().get_ast_type());
        is_instance(self.type_name.as_ref().unwrap(), ASTType::CipherText)
    }
    pub fn with_homomorphism(&self, hom: String) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            self.type_name.clone(),
            self.privacy_annotation.clone(),
            hom,
        ))
    }
    pub fn uint_all() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::uint_type()).into()),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn bool_all() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::bool_type()).into()),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn address_all() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::address_type()).into()),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn cipher_type(plain_type: RcCell<AnnotatedTypeName>, hom: Option<String>) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::cipher_type(plain_type, hom.unwrap())).into()),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn key_type(crypto_params: CryptoParams) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::key_type(crypto_params)).into()),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn proof_type() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::proof_type()).into()),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }
    pub fn all(type_name: TypeName) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(type_name).into()),
            Some(RcCell::new(Expression::all_expr()).into()),
            Homomorphism::non_homomorphic(),
        ))
    }
    pub fn me(type_name: TypeName) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(type_name).into()),
            Some(RcCell::new(Expression::me_expr(None)).into()),
            Homomorphism::non_homomorphic(),
        ))
    }
    pub fn array_all(value_type: RcCell<AnnotatedTypeName>, length: Vec<i32>) -> RcCell<Self> {
        let mut t = value_type;
        for &l in &length {
            t = RcCell::new(AnnotatedTypeName::new(
                Some(
                    RcCell::new(TypeName::Array(Array::Array(ArrayBase::new(
                        t,
                        Some(ExprUnion::I32(l)),
                        None,
                    ))))
                    .into(),
                ),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }
        t
    }
    pub fn clone_owned(&self, global_vars: RcCell<GlobalVars>) -> Self {
        assert!(self.privacy_annotation.is_some());
        let mut at = Self::new(
            if is_instance(self.type_name.as_ref().unwrap(), ASTType::Mapping) {
                self.type_name
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_mapping_ref()
                    .unwrap()
                    .clone_owned(global_vars)
            } else {
                self.type_name.clone()
            },
            self.privacy_annotation.as_ref().map(|pa| pa.clone_inner()),
            self.homomorphism.clone(),
        );
        at.had_privacy_annotation = self.had_privacy_annotation;
        at
    }
}
impl ASTChildren for AnnotatedTypeName {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(type_name) = &self.type_name {
            cb.add_child(type_name.clone());
        }
        if let Some(privacy_annotation) = &self.privacy_annotation {
            cb.add_child(privacy_annotation.clone());
        }
    }
}
impl ASTChildrenCallBack for AnnotatedTypeName {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.type_name
            .as_ref()
            .unwrap()
            .assign(f(self.type_name.as_ref().unwrap()).as_ref().unwrap());
        self.privacy_annotation.as_ref().unwrap().assign(
            f(self.privacy_annotation.as_ref().unwrap())
                .as_ref()
                .unwrap(),
        );
    }
}
