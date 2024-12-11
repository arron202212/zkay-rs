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
    identifier_declaration::VariableDeclaration,
    is_instance, is_instances,
    type_name::{
        BooleanLiteralType, CombinedPrivacyUnion, DummyAnnotation, ExprUnion, NumberLiteralType,
        NumberLiteralTypeUnion, TypeName,
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
    ASTInstanceOf
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
pub enum Statement {
    CircuitDirectiveStatement(CircuitDirectiveStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ForStatement(ForStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    ReturnStatement(ReturnStatement),
    SimpleStatement(SimpleStatement),
    StatementList(StatementList),
    CircuitStatement(CircuitStatement),
}

#[macro_export]
macro_rules! impl_base_ref_for_statement {
    ($fn_name: ident,$self: ident) => {
        match $self {
            Statement::CircuitDirectiveStatement(ast) => Some(ast.$fn_name()),
            Statement::IfStatement(ast) => Some(ast.$fn_name()),
            Statement::WhileStatement(ast) => Some(ast.$fn_name()),
            Statement::DoWhileStatement(ast) => Some(ast.$fn_name()),
            Statement::ForStatement(ast) => Some(ast.$fn_name()),
            Statement::BreakStatement(ast) => Some(ast.$fn_name()),
            Statement::ContinueStatement(ast) => Some(ast.$fn_name()),
            Statement::ReturnStatement(ast) => Some(ast.$fn_name()),
            Statement::SimpleStatement(ast) => Some(ast.$fn_name()),
            Statement::StatementList(ast) => Some(ast.$fn_name()),
            Statement::CircuitStatement(_) => None,
        }
    };
}

impl Statement {
    pub fn ast_base_ref(&self) -> Option<RcCell<ASTBase>> {
        impl_base_ref_for_statement!(ast_base_ref, self)
    }
    pub fn ast_base_mut_ref(&mut self) -> Option<RcCell<ASTBase>> {
        impl_base_ref_for_statement!(ast_base_mut_ref, self)
    }
    pub fn statement_base_ref(&self) -> Option<&StatementBase> {
        impl_base_ref_for_statement!(statement_base_ref, self)
    }
    pub fn statement_base_mut_ref(&mut self) -> Option<&mut StatementBase> {
        impl_base_ref_for_statement!(statement_base_mut_ref, self)
    }
}

#[enum_dispatch]
pub trait StatementBaseRef: ASTBaseRef {
    fn statement_base_ref(&self) -> &StatementBase;
}
pub trait StatementBaseProperty {
    fn before_analysis(&self) -> &Option<PartitionState<AST>>;
    fn after_analysis(&self) -> &Option<PartitionState<AST>>;
    fn function(&self) -> &Option<ASTFlattenWeak>;
    fn pre_statements(&self) -> &Vec<ASTFlatten>;
}
impl<T: StatementBaseRef> StatementBaseProperty for T {
    fn before_analysis(&self) -> &Option<PartitionState<AST>> {
        &self.statement_base_ref().before_analysis
    }
    fn after_analysis(&self) -> &Option<PartitionState<AST>> {
        &self.statement_base_ref().after_analysis
    }
    fn function(&self) -> &Option<ASTFlattenWeak> {
        &self.statement_base_ref().function
    }
    fn pre_statements(&self) -> &Vec<ASTFlatten> {
        &self.statement_base_ref().pre_statements
    }
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StatementBase {
    pub ast_base: RcCell<ASTBase>,
    pub before_analysis: Option<PartitionState<AST>>,
    pub after_analysis: Option<PartitionState<AST>>,
    pub function: Option<ASTFlattenWeak>,
    pub pre_statements: Vec<ASTFlatten>,
}
impl DeepClone for StatementBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            before_analysis: self.before_analysis.clone(),
            after_analysis: self.after_analysis.clone(),
            function: self.function.clone(),
            pre_statements: self.pre_statements.clone_inner(),
        }
    }
}
impl FullArgsSpec for StatementBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.idf().map(|idf| ASTFlatten::from(idf.clone_inner())),
        )]
    }
}
impl FullArgsSpecInit for StatementBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        StatementBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}
impl StatementBase {
    pub fn new(idf: Option<RcCell<Identifier>>) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new(None, idf, None)),
            before_analysis: None,
            after_analysis: None,
            function: None,
            pre_statements: vec![],
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
    CircuitComputationStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
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
pub enum CircuitDirectiveStatement {
    CircuitComputationStatement(CircuitComputationStatement),
    EnterPrivateKeyStatement(EnterPrivateKeyStatement),
}

#[enum_dispatch]
pub trait CircuitDirectiveStatementBaseRef: StatementBaseRef {
    fn circuit_directive_statement_base_ref(&self) -> &CircuitDirectiveStatementBase;
}

#[impl_traits(StatementBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitDirectiveStatementBase {
    pub statement_base: StatementBase,
}
impl DeepClone for CircuitDirectiveStatementBase {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
        }
    }
}
impl FullArgsSpec for CircuitDirectiveStatementBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.idf().map(|idf| ASTFlatten::from(idf.clone_inner())),
        )]
    }
}
impl FullArgsSpecInit for CircuitDirectiveStatementBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        CircuitDirectiveStatementBase::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}
impl CircuitDirectiveStatementBase {
    pub fn new(idf: Option<RcCell<Identifier>>) -> Self {
        Self {
            statement_base: StatementBase::new(idf),
        }
    }
}
#[impl_traits(CircuitDirectiveStatementBase, StatementBase, ASTBase)]
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
pub struct CircuitComputationStatement {
    pub circuit_directive_statement_base: CircuitDirectiveStatementBase,
}
impl DeepClone for CircuitComputationStatement {
    fn clone_inner(&self) -> Self {
        Self {
            circuit_directive_statement_base: self.circuit_directive_statement_base.clone_inner(),
        }
    }
}
impl IntoAST for CircuitComputationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitDirectiveStatement(
            CircuitDirectiveStatement::CircuitComputationStatement(self),
        ))
    }
}
impl FullArgsSpec for CircuitComputationStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.idf().map(|idf| ASTFlatten::from(idf.clone_inner())),
        )]
    }
}
impl FullArgsSpecInit for CircuitComputationStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        CircuitComputationStatement::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|a| a.try_as_identifier()),
        )
    }
}
impl CircuitComputationStatement {
    pub fn new(idf: Option<RcCell<Identifier>>) -> Self {
        Self {
            circuit_directive_statement_base: CircuitDirectiveStatementBase::new(idf),
        }
    }
}
#[impl_traits(CircuitDirectiveStatementBase, StatementBase, ASTBase)]
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
pub struct EnterPrivateKeyStatement {
    pub circuit_directive_statement_base: CircuitDirectiveStatementBase,
    pub crypto_params: CryptoParams,
}
impl DeepClone for EnterPrivateKeyStatement {
    fn clone_inner(&self) -> Self {
        Self {
            circuit_directive_statement_base: self.circuit_directive_statement_base.clone_inner(),
            crypto_params: self.crypto_params.clone(),
        }
    }
}
impl IntoAST for EnterPrivateKeyStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitDirectiveStatement(
            CircuitDirectiveStatement::EnterPrivateKeyStatement(self),
        ))
    }
}
impl FullArgsSpec for EnterPrivateKeyStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::CryptoParams(Some(self.crypto_params.clone()))]
    }
}
impl FullArgsSpecInit for EnterPrivateKeyStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        EnterPrivateKeyStatement::new(fields[0].clone().try_as_crypto_params().flatten().unwrap())
    }
}
impl EnterPrivateKeyStatement {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            circuit_directive_statement_base: CircuitDirectiveStatementBase::new(None),
            crypto_params,
        }
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IfStatement {
    pub statement_base: StatementBase,
    pub condition: ASTFlatten,
    pub then_branch: RcCell<Block>,
    pub else_branch: Option<RcCell<Block>>,
}
impl DeepClone for IfStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
            condition: self.condition.clone_inner(),
            then_branch: self.then_branch.clone_inner(),
            else_branch: self.else_branch.clone_inner(),
        }
    }
}
impl IntoAST for IfStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::IfStatement(self))
    }
}
impl FullArgsSpec for IfStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.condition.clone_inner())),
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.then_branch.clone_inner()))),
            ArgType::ASTFlatten(
                self.else_branch
                    .as_ref()
                    .map(|b| ASTFlatten::from(b.clone_inner())),
            ),
        ]
    }
}
impl FullArgsSpecInit for IfStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        IfStatement::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_block()
                .unwrap(),
            fields[2]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .and_then(|astf| astf.try_as_block()),
        )
    }
}
impl IfStatement {
    pub fn new(
        condition: ASTFlatten,
        then_branch: RcCell<Block>,
        else_branch: Option<RcCell<Block>>,
    ) -> Self {
        Self {
            statement_base: StatementBase::new(None),
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl ASTChildren for IfStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.clone());
        cb.add_child(self.then_branch.clone().into());
        if let Some(else_branch) = &self.else_branch {
            cb.add_child(else_branch.clone().into());
        }
    }
}

impl ASTChildrenCallBack for IfStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.condition.assign(f(&self.condition).as_ref().unwrap());
        *self.then_branch.borrow_mut() = f(&self.then_branch.clone().into())
            .unwrap()
            .try_as_block()
            .unwrap()
            .borrow()
            .clone();
        *self.else_branch.as_ref().unwrap().borrow_mut() = self
            .else_branch
            .as_ref()
            .and_then(|b| f(&b.clone().into()).and_then(|astf| astf.try_as_block()))
            .unwrap()
            .borrow()
            .clone();
    }
}

#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WhileStatement {
    pub statement_base: StatementBase,
    pub condition: ASTFlatten,
    pub body: RcCell<Block>,
}
impl DeepClone for WhileStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
            condition: self.condition.clone_inner(),
            body: self.body.clone_inner(),
        }
    }
}
impl IntoAST for WhileStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::WhileStatement(self))
    }
}
impl FullArgsSpec for WhileStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.condition.clone_inner())),
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.body.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for WhileStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        WhileStatement::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_block()
                .unwrap(),
        )
    }
}
impl WhileStatement {
    pub fn new(condition: ASTFlatten, body: RcCell<Block>) -> Self {
        Self {
            statement_base: StatementBase::new(None),
            condition,
            body,
        }
    }
}
impl ASTChildren for WhileStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.clone());
        cb.add_child(self.body.clone().into());
    }
}

impl ASTChildrenCallBack for WhileStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.condition.assign(f(&self.condition).as_ref().unwrap());
        *self.body.borrow_mut() = f(&self.body.clone().into())
            .unwrap()
            .try_as_block()
            .unwrap()
            .borrow()
            .clone();
    }
}

#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DoWhileStatement {
    pub statement_base: StatementBase,
    pub body: RcCell<Block>,
    pub condition: ASTFlatten,
}
impl DeepClone for DoWhileStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
            body: self.body.clone_inner(),
            condition: self.condition.clone_inner(),
        }
    }
}
impl IntoAST for DoWhileStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::DoWhileStatement(self))
    }
}
impl FullArgsSpec for DoWhileStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.body.clone_inner()))),
            ArgType::ASTFlatten(Some(self.condition.clone_inner())),
        ]
    }
}
impl FullArgsSpecInit for DoWhileStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        DoWhileStatement::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_block()
                .unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
        )
    }
}
impl DoWhileStatement {
    pub fn new(body: RcCell<Block>, condition: ASTFlatten) -> Self {
        Self {
            statement_base: StatementBase::new(None),
            body,
            condition,
        }
    }
}
impl ASTChildren for DoWhileStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.body.clone().into());
        cb.add_child(self.condition.clone());
    }
}
impl ASTChildrenCallBack for DoWhileStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        *self.body.borrow_mut() = f(&self.body.clone().into())
            .unwrap()
            .try_as_block()
            .unwrap()
            .borrow()
            .clone();
        self.condition.assign(f(&self.condition).as_ref().unwrap());
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ForStatement {
    pub statement_base: StatementBase,
    pub init: Option<RcCell<SimpleStatement>>,
    pub condition: ASTFlatten,
    pub update: Option<RcCell<SimpleStatement>>,
    pub body: RcCell<Block>,
}
impl DeepClone for ForStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
            init: self.init.clone_inner(),
            condition: self.condition.clone_inner(),
            update: self.update.clone_inner(),
            body: self.body.clone_inner(),
        }
    }
}
impl IntoAST for ForStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ForStatement(self))
    }
}
impl FullArgsSpec for ForStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(
                self.init
                    .as_ref()
                    .map(|b| ASTFlatten::from(b.clone_inner())),
            ),
            ArgType::ASTFlatten(Some(self.condition.clone_inner())),
            ArgType::ASTFlatten(
                self.update
                    .as_ref()
                    .map(|b| ASTFlatten::from(b.clone_inner())),
            ),
            ArgType::ASTFlatten(Some(ASTFlatten::from(self.body.clone_inner()))),
        ]
    }
}
impl FullArgsSpecInit for ForStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ForStatement::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .map(|astf| astf.try_as_simple_statement().unwrap()),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[2]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .map(|astf| astf.try_as_simple_statement().unwrap()),
            fields[3]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_block()
                .unwrap(),
        )
    }
}
impl ForStatement {
    pub fn new(
        init: Option<RcCell<SimpleStatement>>,
        condition: ASTFlatten,
        update: Option<RcCell<SimpleStatement>>,
        body: RcCell<Block>,
    ) -> Self {
        Self {
            statement_base: StatementBase::new(None),
            init,
            condition,
            update,
            body,
        }
    }

    pub fn statements(&self) -> Vec<ASTFlatten> {
        [
            self.init.as_ref().map(|i| i.clone().into()),
            Some(self.condition.clone()),
            self.update.as_ref().map(|u| u.clone().into()),
            Some(self.body.clone().into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
impl ASTChildren for ForStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(init) = &self.init {
            cb.add_child(init.clone().into());
        }

        cb.add_child(self.condition.clone());
        if let Some(update) = &self.update {
            cb.add_child(update.clone().into());
        }
        cb.add_child(self.body.clone().into());
    }
}

impl ASTChildrenCallBack for ForStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        *self.init.as_ref().unwrap().borrow_mut() = self
            .init
            .as_ref()
            .and_then(|ss| f(&ss.clone().into()).and_then(|astf| astf.try_as_simple_statement()))
            .unwrap()
            .borrow()
            .clone();
        self.condition.assign(f(&self.condition).as_ref().unwrap());
        *self.update.as_ref().unwrap().borrow_mut() = self
            .update
            .as_ref()
            .and_then(|ss| f(&ss.clone().into()).and_then(|astf| astf.try_as_simple_statement()))
            .unwrap()
            .borrow()
            .clone();
        *self.body.borrow_mut() = f(&self.body.clone().into())
            .unwrap()
            .try_as_block()
            .unwrap()
            .borrow()
            .clone();
    }
}

#[impl_traits(StatementBase, ASTBase)]
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
pub struct BreakStatement {
    pub statement_base: StatementBase,
}
impl DeepClone for BreakStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
        }
    }
}
impl IntoAST for BreakStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::BreakStatement(self))
    }
}
impl FullArgsSpec for BreakStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for BreakStatement {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        BreakStatement::new()
    }
}
impl BreakStatement {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(None),
        }
    }
}
#[impl_traits(StatementBase, ASTBase)]
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
pub struct ContinueStatement {
    pub statement_base: StatementBase,
}
impl DeepClone for ContinueStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
        }
    }
}
impl IntoAST for ContinueStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ContinueStatement(self))
    }
}
impl FullArgsSpec for ContinueStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for ContinueStatement {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        ContinueStatement::new()
    }
}
impl ContinueStatement {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(None),
        }
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReturnStatement {
    pub statement_base: StatementBase,
    pub expr: Option<ASTFlatten>,
}
impl DeepClone for ReturnStatement {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
            expr: self.expr.clone_inner(),
        }
    }
}
impl IntoAST for ReturnStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ReturnStatement(self))
    }
}
impl FullArgsSpec for ReturnStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(
            self.expr.as_ref().map(|e| e.clone_inner()),
        )]
    }
}
impl FullArgsSpecInit for ReturnStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ReturnStatement::new(fields[0].clone().try_as_ast_flatten().flatten())
    }
}
impl ReturnStatement {
    pub fn new(expr: Option<ASTFlatten>) -> Self {
        Self {
            statement_base: StatementBase::new(None),
            expr,
        }
    }
}
impl ASTChildren for ReturnStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone());
        }
    }
}

impl ASTChildrenCallBack for ReturnStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.expr
            .as_ref()
            .unwrap()
            .assign(f(self.expr.as_ref().unwrap()).as_ref().unwrap());
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
    SimpleStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
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
pub enum SimpleStatement {
    ExpressionStatement(ExpressionStatement),
    RequireStatement(RequireStatement),
    AssignmentStatement(AssignmentStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}

#[enum_dispatch]
pub trait SimpleStatementBaseRef: StatementBaseRef {
    fn simple_statement_base_ref(&self) -> &SimpleStatementBase;
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SimpleStatementBase {
    pub statement_base: StatementBase,
}
impl DeepClone for SimpleStatementBase {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
        }
    }
}
impl FullArgsSpec for SimpleStatementBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![]
    }
}
impl FullArgsSpecInit for SimpleStatementBase {
    fn from_fields(&self, _fields: Vec<ArgType>) -> Self {
        SimpleStatementBase::new()
    }
}
impl SimpleStatementBase {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(None),
        }
    }
}
#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub expr: ASTFlatten,
}
impl DeepClone for ExpressionStatement {
    fn clone_inner(&self) -> Self {
        Self {
            simple_statement_base: self.simple_statement_base.clone_inner(),
            expr: self.expr.clone_inner(),
        }
    }
}
impl IntoAST for ExpressionStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::ExpressionStatement(self),
        ))
    }
}
impl FullArgsSpec for ExpressionStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::ASTFlatten(Some(self.expr.clone_inner()))]
    }
}
impl FullArgsSpecInit for ExpressionStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        ExpressionStatement::new(fields[0].clone().try_as_ast_flatten().flatten().unwrap())
    }
}
impl ExpressionStatement {
    pub fn new(expr: ASTFlatten) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            expr,
        }
    }
}
impl ASTChildren for ExpressionStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.expr.clone());
    }
}

impl ASTChildrenCallBack for ExpressionStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.expr.assign(f(&self.expr).as_ref().unwrap());
    }
}

#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RequireStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub condition: ASTFlatten,
    pub unmodified_code: String,
}
impl DeepClone for RequireStatement {
    fn clone_inner(&self) -> Self {
        Self {
            simple_statement_base: self.simple_statement_base.clone_inner(),
            condition: self.condition.clone_inner(),
            unmodified_code: self.unmodified_code.clone(),
        }
    }
}
impl IntoAST for RequireStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::RequireStatement(self),
        ))
    }
}
impl FullArgsSpec for RequireStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(self.condition.clone_inner())),
            ArgType::Str(Some(self.unmodified_code.clone())),
        ]
    }
}
impl FullArgsSpecInit for RequireStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        RequireStatement::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1].clone().try_as_str().unwrap(),
        )
    }
}
impl RequireStatement {
    pub fn new(condition: ASTFlatten, unmodified_code: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            condition,
            unmodified_code: unmodified_code.unwrap_or_default(), //self.code()
        }
    }
}
impl ASTChildren for RequireStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.clone());
    }
}

impl ASTChildrenCallBack for RequireStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.condition.assign(f(&self.condition).as_ref().unwrap());
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
    AssignmentStatementBaseRef,
    AssignmentStatementBaseMutRef,
    SimpleStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
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
pub enum AssignmentStatement {
    AssignmentStatement(AssignmentStatementBase),
    CircuitInputStatement(CircuitInputStatement),
}

#[enum_dispatch]
pub trait AssignmentStatementBaseRef: SimpleStatementBaseRef {
    fn assignment_statement_base_ref(&self) -> &AssignmentStatementBase;
}
pub trait AssignmentStatementBaseProperty {
    fn lhs(&self) -> &Option<ASTFlatten>;
    fn rhs(&self) -> &Option<ASTFlatten>;
    fn op(&self) -> &String;
}
impl<T: AssignmentStatementBaseRef> AssignmentStatementBaseProperty for T {
    fn lhs(&self) -> &Option<ASTFlatten> {
        &self.assignment_statement_base_ref().lhs
    }
    fn rhs(&self) -> &Option<ASTFlatten> {
        &self.assignment_statement_base_ref().rhs
    }
    fn op(&self) -> &String {
        &self.assignment_statement_base_ref().op
    }
}
#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(
    ASTDebug,
    ImplBaseTrait,
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
pub struct AssignmentStatementBase {
    pub simple_statement_base: SimpleStatementBase,
    pub lhs: Option<ASTFlatten>,
    pub rhs: Option<ASTFlatten>,
    pub op: String,
}
impl DeepClone for AssignmentStatementBase {
    fn clone_inner(&self) -> Self {
        Self {
            simple_statement_base: self.simple_statement_base.clone_inner(),
            lhs: self.lhs.clone_inner(),
            rhs: self.rhs.clone_inner(),
            op: self.op.clone(),
        }
    }
}
impl IntoAST for AssignmentStatementBase {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(self)),
        ))
    }
}
impl FullArgsSpec for AssignmentStatementBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.lhs().as_ref().map(|e| e.clone_inner())),
            ArgType::ASTFlatten(self.rhs().as_ref().map(|e| e.clone_inner())),
            ArgType::Str(Some(self.op().clone())),
        ]
    }
}
impl FullArgsSpecInit for AssignmentStatementBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        AssignmentStatementBase::new(
            fields[0].clone().try_as_ast_flatten().flatten(),
            fields[1].clone().try_as_ast_flatten().flatten(),
            fields[2].clone().try_as_str().unwrap(),
        )
    }
}
impl AssignmentStatementBase {
    pub fn new(lhs: Option<ASTFlatten>, rhs: Option<ASTFlatten>, op: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            lhs,
            rhs,
            op: op.unwrap_or_default(),
        }
    }
}

impl ASTChildren for AssignmentStatementBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(lhs) = &self.lhs {
            cb.add_child(lhs.clone());
        }
        if let Some(rhs) = &self.rhs {
            cb.add_child(rhs.clone());
        }
    }
}

impl ASTChildrenCallBack for AssignmentStatementBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.lhs
            .as_ref()
            .unwrap()
            .assign(f(self.lhs.as_ref().unwrap()).as_ref().unwrap());
        self.rhs
            .as_ref()
            .unwrap()
            .assign(f(self.rhs.as_ref().unwrap()).as_ref().unwrap());
    }
}

#[impl_traits(AssignmentStatementBase, SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitInputStatement {
    pub assignment_statement_base: AssignmentStatementBase,
}
impl DeepClone for CircuitInputStatement {
    fn clone_inner(&self) -> Self {
        Self {
            assignment_statement_base: self.assignment_statement_base.clone_inner(),
        }
    }
}
impl IntoAST for CircuitInputStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::CircuitInputStatement(self)),
        ))
    }
}
impl ASTChildren for CircuitInputStatement {
    fn process_children(&self, _cb: &mut ChildListBuilder) {
        // self.assignment_statement_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for CircuitInputStatement {
    fn process_children_callback(
        &self,
        _f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        // self.assignment_statement_base.process_children_callback(f);
    }
}

impl FullArgsSpec for CircuitInputStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(self.lhs().as_ref().map(|e| e.clone_inner())),
            ArgType::ASTFlatten(self.rhs().as_ref().map(|e| e.clone_inner())),
            ArgType::Str(Some(self.op().clone())),
        ]
    }
}
impl FullArgsSpecInit for CircuitInputStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        CircuitInputStatement::new(
            fields[0].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
            fields[2].clone().try_as_str().unwrap(),
        )
    }
}
impl CircuitInputStatement {
    pub fn new(lhs: ASTFlatten, rhs: ASTFlatten, op: Option<String>) -> Self {
        Self {
            assignment_statement_base: AssignmentStatementBase::new(Some(lhs), Some(rhs), op),
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
    StatementListBaseRef,
    StatementListBaseMutRef,
    StatementBaseRef,
    StatementBaseMutRef,
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
pub enum StatementList {
    Block(Block),
    IndentBlock(IndentBlock),
    StatementList(StatementListBase),
}

#[enum_dispatch]
pub trait StatementListBaseRef: StatementBaseRef {
    fn statement_list_base_ref(&self) -> &StatementListBase;
}
pub trait StatementListBaseProperty {
    fn statements(&self) -> &Vec<ASTFlatten>;
    fn excluded_from_simulation(&self) -> bool;
    fn get_item(&self, key: i32) -> ASTFlatten {
        assert!(self.statements().len() > key as usize);
        self.statements()[key as usize].clone()
    }

    fn contains(&self, stmt: &ASTFlatten) -> bool {
        if self.statements().contains(stmt) {
            return true;
        }
        for s in self.statements() {
            if is_instance(s, ASTType::StatementListBase)
                && s.try_as_statement_ref()
                    .unwrap()
                    .borrow()
                    .try_as_statement_list_ref()
                    .unwrap()
                    .contains(stmt)
            {
                return true;
            }
        }
        false
    }
}
impl<T: StatementListBaseRef> StatementListBaseProperty for T {
    fn statements(&self) -> &Vec<ASTFlatten> {
        &self.statement_list_base_ref().statements
    }
    fn excluded_from_simulation(&self) -> bool {
        self.statement_list_base_ref().excluded_from_simulation
    }
}
#[impl_traits(StatementBase, ASTBase)]
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
pub struct StatementListBase {
    pub statement_base: StatementBase,
    pub statements: Vec<ASTFlatten>,
    pub excluded_from_simulation: bool,
}
impl DeepClone for StatementListBase {
    fn clone_inner(&self) -> Self {
        Self {
            statement_base: self.statement_base.clone_inner(),
            statements: self.statements.clone_inner(),
            excluded_from_simulation: self.excluded_from_simulation,
        }
    }
}
impl FullArgsSpec for StatementListBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.statements
                    .iter()
                    .map(|s| ArgType::ASTFlatten(Some(s.clone_inner())))
                    .collect(),
            ),
            ArgType::Bool(self.excluded_from_simulation),
        ]
    }
}
impl FullArgsSpecInit for StatementListBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        StatementListBase::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|arg| arg.try_as_ast_flatten().flatten().unwrap())
                .collect(),
            fields[1].clone().try_as_bool().unwrap(),
        )
    }
}
impl StatementListBase {
    pub fn new(statements: Vec<ASTFlatten>, excluded_from_simulation: bool) -> Self {
        // if statements
        //     .iter()
        //     .any(|s| s.get_ast_type() == ASTType::StatementListBase)
        // {
        //     println!(
        //         "==StatementListBase=======new==========StatementListBase===={}=",
        //         line!()
        //     );
        // }
        Self {
            statement_base: StatementBase::new(None),
            statements,
            excluded_from_simulation,
        }
    }
}
impl IntoAST for StatementListBase {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::StatementList(self)))
    }
}

impl ASTChildren for StatementListBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.statements.iter().for_each(|statement| {
            cb.add_child(statement.clone());
        });
    }
}

impl ASTChildrenCallBack for StatementListBase {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.statements.iter().for_each(|statement| {
            statement.assign(f(statement).as_ref().unwrap());
        });
    }
}

#[impl_traits(StatementListBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Block {
    pub statement_list_base: StatementListBase,
    pub was_single_statement: bool,
}
impl DeepClone for Block {
    fn clone_inner(&self) -> Self {
        Self {
            statement_list_base: self.statement_list_base.clone_inner(),
            was_single_statement: self.was_single_statement,
        }
    }
}
impl IntoAST for Block {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::Block(self)))
    }
}
impl ASTChildren for Block {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.statement_list_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for Block {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.statement_list_base.process_children_callback(f);
    }
}

impl FullArgsSpec for Block {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Vec(
                self.statements()
                    .iter()
                    .map(|s| ArgType::ASTFlatten(Some(s.clone_inner())))
                    .collect(),
            ),
            ArgType::Bool(self.was_single_statement),
        ]
    }
}
impl FullArgsSpecInit for Block {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        Block::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|a| a.try_as_ast_flatten().flatten().unwrap())
                .collect(),
            fields[1].clone().try_as_bool().unwrap(),
        )
    }
}
impl Block {
    pub fn new(statements: Vec<ASTFlatten>, was_single_statement: bool) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
            was_single_statement,
        }
    }
}
#[impl_traits(StatementListBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IndentBlock {
    pub statement_list_base: StatementListBase,
}
impl DeepClone for IndentBlock {
    fn clone_inner(&self) -> Self {
        Self {
            statement_list_base: self.statement_list_base.clone_inner(),
        }
    }
}
impl IntoAST for IndentBlock {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::IndentBlock(self)))
    }
}
impl ASTChildren for IndentBlock {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.statement_list_base.process_children(cb);
    }
}

impl ASTChildrenCallBack for IndentBlock {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        self.statement_list_base.process_children_callback(f);
    }
}

impl FullArgsSpec for IndentBlock {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Vec(
            self.statements()
                .iter()
                .map(|s| ArgType::ASTFlatten(Some(s.clone_inner())))
                .collect(),
        )]
    }
}
impl FullArgsSpecInit for IndentBlock {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        IndentBlock::new(
            fields[0]
                .clone()
                .try_as_vec()
                .unwrap()
                .into_iter()
                .map(|a| a.clone().try_as_ast_flatten().flatten().unwrap())
                .collect(),
        )
    }
}
impl IndentBlock {
    pub fn new(statements: Vec<ASTFlatten>) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
        }
    }
}

#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclarationStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub variable_declaration: RcCell<VariableDeclaration>,
    pub expr: Option<ASTFlatten>,
}
impl DeepClone for VariableDeclarationStatement {
    fn clone_inner(&self) -> Self {
        Self {
            simple_statement_base: self.simple_statement_base.clone_inner(),
            variable_declaration: self.variable_declaration.clone_inner(),
            expr: self.expr.clone_inner(),
        }
    }
}
impl IntoAST for VariableDeclarationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::VariableDeclarationStatement(self),
        ))
    }
}
impl FullArgsSpec for VariableDeclarationStatement {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::ASTFlatten(Some(ASTFlatten::from(
                self.variable_declaration.clone_inner(),
            ))),
            ArgType::ASTFlatten(self.expr.as_ref().map(|e| e.clone_inner())),
        ]
    }
}
impl FullArgsSpecInit for VariableDeclarationStatement {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        VariableDeclarationStatement::new(
            fields[0]
                .clone()
                .try_as_ast_flatten()
                .flatten()
                .unwrap()
                .try_as_variable_declaration()
                .unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten(),
        )
    }
}
impl VariableDeclarationStatement {
    pub fn new(
        variable_declaration: RcCell<VariableDeclaration>,
        expr: Option<ASTFlatten>,
    ) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            variable_declaration,
            expr,
        }
    }
}
impl ASTChildren for VariableDeclarationStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.variable_declaration.clone().into());
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone());
        }
    }
}

impl ASTChildrenCallBack for VariableDeclarationStatement {
    fn process_children_callback(
        &self,
        f: impl Fn(&ASTFlatten) -> Option<ASTFlatten> + std::marker::Copy,
    ) {
        *self.variable_declaration.borrow_mut() = f(&self.variable_declaration.clone().into())
            .unwrap()
            .try_as_variable_declaration()
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
trait MyPartialEq {
    fn my_eq(&self, other: &Self) -> bool;
}
