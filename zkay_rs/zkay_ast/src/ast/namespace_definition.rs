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
    identifier::IdentifierBaseProperty,
    identifier::{HybridArgumentIdf, Identifier, IdentifierBase},
    identifier_declaration::{Parameter, VariableDeclaration},
    is_instance, is_instances,
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
    ASTChildren,
    ASTChildrenCallBack,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    NamespaceDefinitionBaseRef,
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
pub enum NamespaceDefinition {
    ConstructorOrFunctionDefinition(ConstructorOrFunctionDefinition),
    EnumDefinition(EnumDefinition),
    StructDefinition(StructDefinition),
    ContractDefinition(ContractDefinition),
}
#[enum_dispatch]
pub trait NamespaceDefinitionBaseRef: ASTBaseRef {
    fn namespace_definition_base_ref(&self) -> &NamespaceDefinitionBase;
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NamespaceDefinitionBase {
    pub ast_base: RcCell<ASTBase>,
}
impl DeepClone for NamespaceDefinitionBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
        }
    }
}
impl FullArgsSpec for NamespaceDefinitionBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.annotated_type()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for NamespaceDefinitionBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        NamespaceDefinitionBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_annotated_type_name()),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_identifier()),
        )
    }
}
impl NamespaceDefinitionBase {
    pub fn new(
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
    ) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(annotated_type, idf, None)),
        }
    }
}
impl ASTChildren for NamespaceDefinitionBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf() {
            cb.add_child(idf.clone().into());
        }
    }
}
impl ASTChildrenCallBack for NamespaceDefinitionBase {
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
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ConstructorOrFunctionDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub parameters: Vec<RcCell<Parameter>>,
    pub modifiers: Vec<String>,
    pub return_parameters: Vec<RcCell<Parameter>>,
    pub body: Option<RcCell<Block>>,
    pub return_var_decls: Vec<RcCell<VariableDeclaration>>,
    pub original_body: Option<RcCell<Block>>,
    pub called_functions: BTreeSet<RcCell<ConstructorOrFunctionDefinition>>,
    pub is_recursive: bool,
    pub has_static_body: bool,
    pub can_be_private: bool,
    pub used_homomorphisms: Option<BTreeSet<String>>,
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
    pub requires_verification: bool,
    pub requires_verification_when_external: bool,
}
impl DeepClone for ConstructorOrFunctionDefinition {
    fn clone_inner(&self) -> Self {
        Self {
            namespace_definition_base: self.namespace_definition_base.clone_inner(),
            parameters: self.parameters.clone_inner(),
            return_parameters: self.return_parameters.clone_inner(),
            body: self.body.clone_inner(),
            return_var_decls: self.return_var_decls.clone_inner(),
            original_body: self.original_body.clone_inner(),
            called_functions: self
                .called_functions
                .iter()
                .map(|cf| cf.clone_inner())
                .collect(),
            ..self.clone()
        }
    }
}
impl IntoAST for ConstructorOrFunctionDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(self))
    }
}
impl FullArgsSpec for ConstructorOrFunctionDefinition {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
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
            ArgType::ASTFlatten(
                self.body
                    .as_ref()
                    .map(|tn| ASTFlatten::from(tn.clone_inner())),
            ),
        ]
    }
}
impl FullArgsSpecInit for ConstructorOrFunctionDefinition {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ConstructorOrFunctionDefinition::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_identifier()),
            fields[1]
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
            fields[2]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|astf| astf.try_as_str().flatten().unwrap())
                .collect(),
            fields[3]
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
            fields[4]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_block()),
        )
    }
}
impl ConstructorOrFunctionDefinition {
    pub fn new(
        idf: Option<RcCell<Identifier>>,
        parameters: Vec<RcCell<Parameter>>,
        modifiers: Vec<String>,
        return_parameters: Vec<RcCell<Parameter>>,
        body: Option<RcCell<Block>>,
    ) -> Self {
        assert!(
            idf.is_some() && idf.as_ref().unwrap().borrow().name() != "constructor"
                || return_parameters.is_empty()
        );
        let idf = idf.or(Some(RcCell::new(Identifier::Identifier(
            IdentifierBase::new(String::from("constructor")),
        ))));

        let return_var_name = CFG.lock().unwrap().return_var_name();
        let mut return_var_decls: Vec<_> = return_parameters
            .iter()
            .enumerate()
            .map(|(idx, rp)| {
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    rp.borrow().annotated_type().clone_inner(),
                    Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
                        format!("{}_{idx}", return_var_name),
                    )))),
                    rp.borrow()
                        .identifier_declaration_base
                        .storage_location
                        .clone(),
                ))
            })
            .collect();
        return_var_decls.iter_mut().for_each(|vd| {
            vd.borrow_mut()
                .ast_base_mut_ref()
                .borrow_mut()
                .idf
                .as_mut()
                .unwrap()
                .borrow_mut()
                .ast_base_ref()
                .borrow_mut()
                .parent = Some(ASTFlatten::from(vd.clone()).downgrade());
        });
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(
                Some(RcCell::new(AnnotatedTypeName::new(
                    Some(
                        RcCell::new(TypeName::FunctionTypeName(FunctionTypeName::new(
                            parameters.clone(),
                            modifiers.clone(),
                            return_parameters.clone(),
                        )))
                        .into(),
                    ),
                    None,
                    Homomorphism::non_homomorphic(),
                ))),
                idf,
            ),
            parameters,
            modifiers,
            return_parameters,
            body,
            return_var_decls,
            original_body: None,
            called_functions: BTreeSet::new(),
            is_recursive: false,
            has_static_body: true,
            can_be_private: true,
            used_homomorphisms: None,
            used_crypto_backends: None,
            requires_verification: false,
            requires_verification_when_external: false,
        }
    }
    pub fn has_side_effects(&self) -> bool {
        //  not ("pure" in self.modifiers or "view" in self.modifiers)
        !(self.modifiers.contains(&String::from("pure"))
            || self.modifiers.contains(&String::from("view")))
    }

    pub fn can_be_external(&self) -> bool {
        // return not ("private" in self.modifiers or "internal" in self.modifiers)

        !(self.modifiers.contains(&String::from("private"))
            || self.modifiers.contains(&String::from("internal")))
    }

    pub fn is_external(&self) -> bool {
        // return "external" in self.modifiers

        self.modifiers.contains(&String::from("external"))
    }

    pub fn is_payable(&self) -> bool {
        // return "payable" in self.modifiers

        self.modifiers.contains(&String::from("payable"))
    }

    pub fn name(&self) -> String {
        self.idf().as_ref().unwrap().borrow().name().clone()
    }

    pub fn return_type(&self) -> TupleType {
        TupleType::new(
            self.return_parameters
                .iter()
                .filter_map(|p| p.borrow().annotated_type().clone())
                .collect(),
        )
    }
    // return TupleType([p.annotated_type for p in self.parameters])
    pub fn parameter_types(&self) -> TupleType {
        TupleType::new(
            self.parameters
                .iter()
                .filter_map(|p| p.borrow().annotated_type().clone())
                .collect(),
        )
    }

    pub fn is_constructor(&self) -> bool {
        self.idf().as_ref().unwrap().borrow().name().as_str() == "constructor"
    }

    pub fn is_function(&self) -> bool {
        !self.is_constructor()
    }

    pub fn _update_fct_type(&mut self) {
        self.ast_base_mut_ref().borrow_mut().annotated_type =
            Some(RcCell::new(AnnotatedTypeName::new(
                Some(
                    RcCell::new(TypeName::FunctionTypeName(FunctionTypeName::new(
                        self.parameters.clone(),
                        self.modifiers.clone(),
                        self.return_parameters.clone(),
                    )))
                    .into(),
                ),
                None,
                Homomorphism::non_homomorphic(),
            )));
        // AnnotatedTypeName(FunctionTypeName(&self.parameters, self.modifiers, self.return_parameters));
    }
    pub fn add_param(
        &mut self,
        mut t: ASTFlatten,
        idf: IdentifierExprUnion,
        ref_storage_loc: Option<String>,
    ) {
        let ref_storage_loc = ref_storage_loc.unwrap_or(String::from("memory"));
        if is_instance(&t, ASTType::TypeNameBase) {
            t = RcCell::new(AnnotatedTypeName::new(
                Some(t.clone()),
                None,
                Homomorphism::non_homomorphic(),
            ))
            .into();
        };
        let idf = Some(match idf {
            IdentifierExprUnion::String(idf) => {
                RcCell::new(Identifier::Identifier(IdentifierBase::new(idf)))
            }
            IdentifierExprUnion::Identifier(idf) => idf.clone(),
        });
        let storage_loc = if t
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_primitive_type()
        {
            None
        } else {
            Some(ref_storage_loc)
        };
        self.parameters.push(RcCell::new(Parameter::new(
            vec![],
            t.try_as_annotated_type_name(),
            Some(idf.as_ref().unwrap().clone()),
            storage_loc,
        )));
        self._update_fct_type();
    }
}

impl ConstructorOrFunctionDefinitionAttr for ConstructorOrFunctionDefinition {
    fn get_requires_verification_when_external(&self) -> bool {
        self.requires_verification_when_external
    }
    fn get_name(&self) -> String {
        self.name().clone()
    }
}
impl ASTChildren for ConstructorOrFunctionDefinition {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.clone().into());
        });
        self.return_parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.clone().into());
        });
        if let Some(body) = &self.body {
            // println!("======body============={:?}",body);
            cb.add_child(body.clone().into());
        }
    }
}

impl ASTChildrenCallBack for ConstructorOrFunctionDefinition {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.namespace_definition_base.process_children_callback(f);
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

        *self.body.as_ref().unwrap().borrow_mut() = self
            .body
            .as_ref()
            .and_then(|body| f(&body.clone().into()).and_then(|astf| astf.try_as_block()))
            .unwrap()
            .borrow()
            .clone();
    }
}

#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub values: Vec<RcCell<EnumValue>>,
}
impl DeepClone for EnumDefinition {
    fn clone_inner(&self) -> Self {
        Self {
            namespace_definition_base: self.namespace_definition_base.clone_inner(),
            values: self.values.clone_inner(),
        }
    }
}
impl IntoAST for EnumDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::EnumDefinition(self))
    }
}
impl FullArgsSpec for EnumDefinition {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
            ArgType::Vec(
                self.values
                    .iter()
                    .map(|v| ArgType::ASTFlatten(Some(ASTFlatten::from(v.clone_inner()))))
                    .collect(),
            ),
        ]
    }
}
impl FullArgsSpecInit for EnumDefinition {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        EnumDefinition::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| {
                    f.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_enum_value()
                        .unwrap()
                })
                .collect(),
        )
    }
}
impl EnumDefinition {
    pub fn new(idf: Option<RcCell<Identifier>>, values: Vec<RcCell<EnumValue>>) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(None, idf),
            values,
        }
    }
}

impl ASTChildren for EnumDefinition {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.values.iter().for_each(|value| {
            cb.add_child(value.clone().into());
        });
    }
}
impl ASTChildrenCallBack for EnumDefinition {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.namespace_definition_base.process_children_callback(f);
        self.values.iter().for_each(|value| {
            *value.borrow_mut() = f(&value.clone().into())
                .unwrap()
                .try_as_enum_value()
                .unwrap()
                .borrow()
                .clone();
        });
    }
}
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StructDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub members: Vec<ASTFlatten>,
}
impl DeepClone for StructDefinition {
    fn clone_inner(&self) -> Self {
        Self {
            namespace_definition_base: self.namespace_definition_base.clone_inner(),
            members: self.members.clone_inner(),
        }
    }
}
impl IntoAST for StructDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::StructDefinition(self))
    }
}
impl FullArgsSpec for StructDefinition {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
            ArgType::Vec(
                self.members
                    .iter()
                    .map(|m| ArgType::ASTFlatten(Some(m.clone_inner())))
                    .collect(),
            ),
        ]
    }
}
impl FullArgsSpecInit for StructDefinition {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        StructDefinition::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|v| v.try_as_ast_flatten().flatten().unwrap())
                .collect(),
        )
    }
}
impl StructDefinition {
    pub fn new(idf: Option<RcCell<Identifier>>, members: Vec<ASTFlatten>) -> Self {
        // members.iter().for_each(|m|{print!("=member==={:?}",m.get_ast_type())});
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(None, idf),
            members,
        }
    }
    pub fn to_namespace_definition(&self) -> NamespaceDefinition {
        NamespaceDefinition::StructDefinition(self.clone())
    }
}
impl ASTChildren for StructDefinition {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.members.iter().for_each(|member| {
            cb.add_child(member.clone());
        });
    }
}
impl ASTChildrenCallBack for StructDefinition {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.namespace_definition_base.process_children_callback(f);
        self.members.iter().for_each(|member| {
            member.assign(f(member).as_ref().unwrap());
        });
    }
}
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]

pub struct ContractDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub state_variable_declarations: Vec<ASTFlatten>,
    pub constructor_definitions: Vec<RcCell<ConstructorOrFunctionDefinition>>,
    pub function_definitions: Vec<RcCell<ConstructorOrFunctionDefinition>>,
    pub enum_definitions: Vec<RcCell<EnumDefinition>>,
    pub struct_definitions: Vec<RcCell<StructDefinition>>,
    pub used_crypto_backends: Vec<CryptoParams>,
}
impl DeepClone for ContractDefinition {
    fn clone_inner(&self) -> Self {
        Self {
            namespace_definition_base: self.namespace_definition_base.clone_inner(),
            state_variable_declarations: self.state_variable_declarations.clone_inner(),
            constructor_definitions: self.constructor_definitions.clone_inner(),
            function_definitions: self.function_definitions.clone_inner(),
            enum_definitions: self.enum_definitions.clone_inner(),
            struct_definitions: self.struct_definitions.clone_inner(),
            used_crypto_backends: self.used_crypto_backends.clone(),
        }
    }
}
impl IntoAST for ContractDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::ContractDefinition(self))
    }
}
impl FullArgsSpec for ContractDefinition {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.idf().map(|tn| ASTFlatten::from(tn.clone_inner()))),
            ArgType::Vec(
                self.state_variable_declarations
                    .iter()
                    .map(|s| ArgType::ASTFlatten(Some(s.clone_inner())))
                    .collect(),
            ),
            ArgType::Vec(
                self.constructor_definitions
                    .iter()
                    .map(|c| ArgType::ASTFlatten(Some(ASTFlatten::from(c.clone_inner()))))
                    .collect(),
            ),
            ArgType::Vec(
                self.function_definitions
                    .iter()
                    .map(|c| ArgType::ASTFlatten(Some(ASTFlatten::from(c.clone_inner()))))
                    .collect(),
            ),
            ArgType::Vec(
                self.enum_definitions
                    .iter()
                    .map(|c| ArgType::ASTFlatten(Some(ASTFlatten::from(c.clone_inner()))))
                    .collect(),
            ),
            ArgType::Vec(
                self.struct_definitions
                    .iter()
                    .map(|c| ArgType::ASTFlatten(Some(ASTFlatten::from(c.clone_inner()))))
                    .collect(),
            ),
            ArgType::Vec(
                self.used_crypto_backends
                    .iter()
                    .map(|c| ArgType::CryptoParams(Some(c.clone())))
                    .collect(),
            ),
        ]
    }
}
impl FullArgsSpecInit for ContractDefinition {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ContractDefinition::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
            fields[1]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| f.try_as_ast_flatten().flatten().unwrap())
                .collect(),
            fields[2]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| {
                    f.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_constructor_or_function_definition()
                        .unwrap()
                })
                .collect(),
            fields[3]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| {
                    f.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_constructor_or_function_definition()
                        .unwrap()
                })
                .collect(),
            fields[4]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| {
                    f.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_enum_definition()
                        .unwrap()
                })
                .collect(),
            fields[5]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| {
                    f.try_as_ast_flatten()
                        .flatten()
                        .unwrap()
                        .try_as_struct_definition()
                        .unwrap()
                })
                .collect(),
            fields[6]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|f| f.try_as_crypto_params().flatten().unwrap())
                .collect(),
        )
    }
}
impl ContractDefinition {
    pub fn new(
        idf: Option<RcCell<Identifier>>,
        state_variable_declarations: Vec<ASTFlatten>,
        constructor_definitions: Vec<RcCell<ConstructorOrFunctionDefinition>>,
        function_definitions: Vec<RcCell<ConstructorOrFunctionDefinition>>,
        enum_definitions: Vec<RcCell<EnumDefinition>>,
        struct_definitions: Vec<RcCell<StructDefinition>>,
        used_crypto_backends: Vec<CryptoParams>,
    ) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(None, idf),
            state_variable_declarations,
            constructor_definitions,
            function_definitions,
            enum_definitions,
            struct_definitions,
            used_crypto_backends,
        }
    }
    pub fn get_item(&self, key: &String) -> Option<ASTFlatten> {
        // //println!("=======get_item============");
        if key == "constructor" {
            if self.constructor_definitions.is_empty() {
                // # return empty constructor
                let mut c =
                    ConstructorOrFunctionDefinition::new(None, vec![], vec![], vec![], None);
                c.ast_base_mut_ref().borrow_mut().parent =
                    Some(ASTFlatten::from(RcCell::new(self.clone())).downgrade());
                Some(RcCell::new(c).into())
            } else if self.constructor_definitions.len() == 1 {
                Some(self.constructor_definitions[0].clone().into())
            } else {
                // panic!("Multiple constructors exist");
                None
            }
        } else {
            let names = self.names();
            let d_identifier = names.get(key).unwrap();
            d_identifier
                .upgrade()
                .unwrap()
                .borrow()
                .parent()
                .as_ref()
                .map(|p| p.clone().upgrade().unwrap())
        }
    }
}
impl ASTChildren for ContractDefinition {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.enum_definitions.iter().for_each(|enum_definition| {
            cb.add_child(enum_definition.clone().into());
        });
        self.struct_definitions
            .iter()
            .for_each(|struct_definition| {
                cb.add_child(struct_definition.clone().into());
            });
        self.state_variable_declarations
            .iter()
            .for_each(|state_variable_declarations| {
                cb.add_child(state_variable_declarations.clone());
            });
        self.constructor_definitions
            .iter()
            .for_each(|constructor_definition| {
                cb.add_child(constructor_definition.clone().into());
            });
        self.function_definitions
            .iter()
            .for_each(|function_definition| {
                cb.add_child(function_definition.clone().into());
            });
    }
}

impl ASTChildrenCallBack for ContractDefinition {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.namespace_definition_base.process_children_callback(f);
        self.enum_definitions.iter().for_each(|enum_definition| {
            *enum_definition.borrow_mut() = f(&enum_definition.clone().into())
                .unwrap()
                .try_as_enum_definition()
                .unwrap()
                .borrow()
                .clone();
        });
        self.struct_definitions
            .iter()
            .for_each(|struct_definition| {
                *struct_definition.borrow_mut() = f(&struct_definition.clone().into())
                    .unwrap()
                    .try_as_struct_definition()
                    .unwrap()
                    .borrow()
                    .clone();
            });
        self.state_variable_declarations
            .iter()
            .for_each(|state_variable_declarations| {
                state_variable_declarations
                    .assign(f(state_variable_declarations).as_ref().unwrap());
            });
        self.constructor_definitions
            .iter()
            .for_each(|constructor_definition| {
                *constructor_definition.borrow_mut() = f(&constructor_definition.clone().into())
                    .unwrap()
                    .try_as_constructor_or_function_definition()
                    .unwrap()
                    .borrow()
                    .clone();
            });
        self.function_definitions
            .iter()
            .for_each(|function_definition| {
                *function_definition.borrow_mut() = f(&function_definition.clone().into())
                    .unwrap()
                    .try_as_constructor_or_function_definition()
                    .unwrap()
                    .borrow()
                    .clone();
            });
    }
}
