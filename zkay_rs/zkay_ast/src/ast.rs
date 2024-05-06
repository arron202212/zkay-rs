#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";
// use  typing import List, Dict, Union, Optional, Callable, Set, TypeVar;
use crate::analysis::partition_state::PartitionState;
use crate::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use crate::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use enum_dispatch::enum_dispatch;
use eyre::{eyre, Result};
use lazy_static::lazy_static;
use rccell::{RcCell, WeakCell};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    cell::RefCell,
    cmp::Ordering,
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
    zk_print,
};
use zkay_crypto::params::CryptoParams;
use zkay_derive::{
    impl_trait, impl_traits, ASTChildrenImpl, ASTDebug, ASTFlattenImpl, ASTKind,
    ASTVisitorBaseRefImpl, ExpressionASTypeImpl, ImplBaseTrait,
};
use zkay_utils::progress_printer::warn_print;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct RRWrapper<T: Hash + PartialEq + Eq + Ord + Clone + Debug>(Rc<RefCell<T>>);

impl<T: Hash + PartialEq + Eq + Ord + Clone + Debug> Deref for RRWrapper<T> {
    type Target = Rc<RefCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Hash + PartialEq + Eq + Ord + Clone + Debug> DerefMut for RRWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
impl<T: Hash + PartialEq + Eq + Ord + Clone + Debug> Hash for RRWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl<T: Hash + PartialEq + Eq + Ord + Clone + Debug> RRWrapper<T> {
    pub fn new(inner: T) -> Self {
        RRWrapper(Rc::new(RefCell::new(inner)))
    }
}

pub struct ChildListBuilder {
    pub children: Vec<ASTFlatten>,
}
impl ChildListBuilder {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
    pub fn add_child(&mut self, ast: ASTFlatten) {
        self.children.push(ast);
    }
}

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
pub trait Immutable {
    fn is_immutable(&self) -> bool;
}
impl<T: ASTInstanceOf> ASTInstanceOf for RcCell<T> {
    fn get_ast_type(&self) -> ASTType {
        self.borrow().get_ast_type()
    }
}
impl<T: IntoAST> IntoAST for RcCell<T> {
    fn into_ast(self) -> AST {
        self.borrow().clone().into_ast()
    }
}
pub fn is_instance<T: ASTInstanceOf + IntoAST>(var: &T, ast_type: ASTType) -> bool {
    match ast_type {
        ASTType::IdentifierBase => matches!(var.to_ast(), AST::Identifier(_)),
        ASTType::CommentBase => matches!(var.to_ast(), AST::Comment(_)),
        ASTType::ExpressionBase => matches!(var.to_ast(), AST::Expression(_)),
        ASTType::StatementBase => matches!(var.to_ast(), AST::Statement(_)),
        ASTType::TypeNameBase => matches!(var.to_ast(), AST::TypeName(_)),
        ASTType::IdentifierDeclarationBase => matches!(var.to_ast(), AST::IdentifierDeclaration(_)),
        ASTType::NamespaceDefinitionBase => matches!(var.to_ast(), AST::NamespaceDefinition(_)),
        ASTType::FunctionCallExprBase => matches!(
            var.to_ast(),
            AST::Expression(Expression::FunctionCallExpr(_))
        ),
        ASTType::LiteralExprBase => {
            matches!(var.to_ast(), AST::Expression(Expression::LiteralExpr(_)))
        }
        ASTType::TupleOrLocationExprBase => matches!(
            var.to_ast(),
            AST::Expression(Expression::TupleOrLocationExpr(_))
        ),
        ASTType::ArrayLiteralExprBase => matches!(
            var.to_ast(),
            AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(_)))
        ),
        ASTType::LocationExprBase => matches!(
            var.to_ast(),
            AST::Expression(Expression::TupleOrLocationExpr(
                TupleOrLocationExpr::LocationExpr(_)
            ))
        ),
        ASTType::ReclassifyExprBase => {
            matches!(var.to_ast(), AST::Expression(Expression::ReclassifyExpr(_)))
        }
        ASTType::CircuitDirectiveStatementBase => matches!(
            var.to_ast(),
            AST::Statement(Statement::CircuitDirectiveStatement(_))
        ),
        ASTType::SimpleStatementBase => {
            matches!(var.to_ast(), AST::Statement(Statement::SimpleStatement(_)))
        }
        ASTType::StatementListBase => {
            matches!(var.to_ast(), AST::Statement(Statement::StatementList(_)))
        }
        ASTType::AssignmentStatementBase => matches!(
            var.to_ast(),
            AST::Statement(Statement::SimpleStatement(
                SimpleStatement::AssignmentStatement(_)
            ))
        ),
        ASTType::ElementaryTypeNameBase => {
            matches!(var.to_ast(), AST::TypeName(TypeName::ElementaryTypeName(_)))
        }
        ASTType::UserDefinedTypeNameBase => matches!(
            var.to_ast(),
            AST::TypeName(TypeName::UserDefinedTypeName(_))
        ),
        ASTType::NumberTypeNameBase => matches!(
            var.to_ast(),
            AST::TypeName(TypeName::ElementaryTypeName(
                ElementaryTypeName::NumberTypeName(_)
            ))
        ),
        ASTType::ArrayBase => matches!(var.to_ast(), AST::TypeName(TypeName::Array(_))),
        _ => var.get_ast_type() == ast_type,
    }
}
pub fn is_instances<T: ASTInstanceOf + IntoAST>(var: &T, ast_types: Vec<ASTType>) -> bool {
    ast_types.into_iter().any(|t| is_instance(var, t))
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ASTType {
    ASTBase,
    IdentifierBase,
    CommentBase,
    ExpressionBase,
    StatementBase,
    TypeNameBase,
    AnnotatedTypeName,
    IdentifierDeclarationBase,
    NamespaceDefinitionBase,
    EnumValue,
    SourceUnit,
    Pragma,
    VersionPragma,
    Modifier,
    Homomorphism,
    BlankLine,
    BuiltinFunction,
    FunctionCallExprBase,
    NewExpr,
    PrimitiveCastExpr,
    LiteralExprBase,
    TupleOrLocationExprBase,
    MeExpr,
    AllExpr,
    ReclassifyExpr,
    BooleanLiteralExpr,
    NumberLiteralExpr,
    StringLiteralExpr,
    ArrayLiteralExprBase,
    KeyLiteralExpr,
    TupleExpr,
    LocationExprBase,
    IdentifierExpr,
    MemberAccessExpr,
    IndexExpr,
    SliceExpr,
    ReclassifyExprBase,
    RehomExpr,
    EncryptionExpression,
    HybridArgumentIdf,
    CircuitDirectiveStatementBase,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    BreakStatement,
    ContinueStatement,
    ReturnStatement,
    SimpleStatementBase,
    StatementListBase,
    CircuitComputationStatement,
    EnterPrivateKeyStatement,
    ExpressionStatement,
    RequireStatement,
    AssignmentStatementBase,
    VariableDeclarationStatement,
    CircuitInputStatement,
    Block,
    IndentBlock,
    ElementaryTypeNameBase,
    UserDefinedTypeNameBase,
    Mapping,
    TupleType,
    FunctionTypeName,
    Literal,
    BoolTypeName,
    BooleanLiteralType,
    NumberLiteralType,
    IntTypeName,
    UintTypeName,
    NumberTypeNameBase,
    EnumTypeName,
    EnumValueTypeName,
    StructTypeName,
    ContractTypeName,
    AddressTypeName,
    AddressPayableTypeName,
    CipherText,
    Randomness,
    Key,
    Proof,
    ArrayBase,
    VariableDeclaration,
    Parameter,
    StateVariableDeclaration,
    ConstructorOrFunctionDefinition,
    EnumDefinition,
    StructDefinition,
    ContractDefinition,
    DummyAnnotation,
    CircComment,
    CircIndentBlock,
    CircCall,
    CircVarDecl,
    CircGuardModification,
    CircEncConstraint,
    CircSymmEncConstraint,
    CircEqConstraint,
}

impl fmt::Display for ASTFlatten {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}
impl Immutable for ASTFlatten {
    fn is_immutable(&self) -> bool {
        true
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, EnumIs, EnumTryAs, Hash)]
pub enum ASTFlatten {
    AST(RcCell<AST>),
    Expression(RcCell<Expression>),
    Identifier(RcCell<Identifier>),
    IdentifierBase(RcCell<IdentifierBase>),
    Comment(RcCell<Comment>),
    CommentBase(RcCell<CommentBase>),
    AnnotatedTypeName(RcCell<AnnotatedTypeName>),
    EnumValue(RcCell<EnumValue>),
    SourceUnit(RcCell<SourceUnit>),
    BlankLine(RcCell<BlankLine>),
    BuiltinFunction(RcCell<BuiltinFunction>),
    FunctionCallExprBase(RcCell<FunctionCallExprBase>),
    FunctionCallExpr(RcCell<FunctionCallExpr>),
    NewExpr(RcCell<NewExpr>),
    PrimitiveCastExpr(RcCell<PrimitiveCastExpr>),
    MeExpr(RcCell<MeExpr>),
    AllExpr(RcCell<AllExpr>),
    ReclassifyExpr(RcCell<ReclassifyExpr>),
    LiteralExpr(RcCell<LiteralExpr>),
    BooleanLiteralExpr(RcCell<BooleanLiteralExpr>),
    NumberLiteralExpr(RcCell<NumberLiteralExpr>),
    StringLiteralExpr(RcCell<StringLiteralExpr>),
    ArrayLiteralExprBase(RcCell<ArrayLiteralExprBase>),
    ArrayLiteralExpr(RcCell<ArrayLiteralExpr>),
    KeyLiteralExpr(RcCell<KeyLiteralExpr>),
    TupleOrLocationExpr(RcCell<TupleOrLocationExpr>),
    TupleExpr(RcCell<TupleExpr>),
    IdentifierExpr(RcCell<IdentifierExpr>),
    MemberAccessExpr(RcCell<MemberAccessExpr>),
    LocationExpr(RcCell<LocationExpr>),
    IndexExpr(RcCell<IndexExpr>),
    SliceExpr(RcCell<SliceExpr>),
    ReclassifyExprBase(RcCell<ReclassifyExprBase>),
    RehomExpr(RcCell<RehomExpr>),
    EncryptionExpression(RcCell<EncryptionExpression>),
    HybridArgumentIdf(RcCell<HybridArgumentIdf>),
    Statement(RcCell<Statement>),
    IfStatement(RcCell<IfStatement>),
    WhileStatement(RcCell<WhileStatement>),
    DoWhileStatement(RcCell<DoWhileStatement>),
    ForStatement(RcCell<ForStatement>),
    BreakStatement(RcCell<BreakStatement>),
    ContinueStatement(RcCell<ContinueStatement>),
    ReturnStatement(RcCell<ReturnStatement>),
    StatementListBase(RcCell<StatementListBase>),
    StatementList(RcCell<StatementList>),
    CircuitDirectiveStatement(RcCell<CircuitDirectiveStatement>),
    CircuitComputationStatement(RcCell<CircuitComputationStatement>),
    EnterPrivateKeyStatement(RcCell<EnterPrivateKeyStatement>),
    ExpressionStatement(RcCell<ExpressionStatement>),
    RequireStatement(RcCell<RequireStatement>),
    AssignmentStatementBase(RcCell<AssignmentStatementBase>),
    AssignmentStatement(RcCell<AssignmentStatement>),
    VariableDeclarationStatement(RcCell<VariableDeclarationStatement>),
    CircuitInputStatement(RcCell<CircuitInputStatement>),
    SimpleStatement(RcCell<SimpleStatement>),
    Block(RcCell<Block>),
    IndentBlock(RcCell<IndentBlock>),
    Mapping(RcCell<Mapping>),
    TupleType(RcCell<TupleType>),
    TypeName(RcCell<TypeName>),
    ElementaryTypeName(RcCell<ElementaryTypeName>),
    FunctionTypeName(RcCell<FunctionTypeName>),
    BoolTypeName(RcCell<BoolTypeName>),
    BooleanLiteralType(RcCell<BooleanLiteralType>),
    NumberLiteralType(RcCell<NumberLiteralType>),
    IntTypeName(RcCell<IntTypeName>),
    UintTypeName(RcCell<UintTypeName>),
    NumberTypeNameBase(RcCell<NumberTypeNameBase>),
    NumberTypeName(RcCell<NumberTypeName>),
    EnumTypeName(RcCell<EnumTypeName>),
    EnumValueTypeName(RcCell<EnumValueTypeName>),
    StructTypeName(RcCell<StructTypeName>),
    ContractTypeName(RcCell<ContractTypeName>),
    AddressTypeName(RcCell<AddressTypeName>),
    AddressPayableTypeName(RcCell<AddressPayableTypeName>),
    UserDefinedTypeName(RcCell<UserDefinedTypeName>),
    CipherText(RcCell<CipherText>),
    Randomness(RcCell<Randomness>),
    Key(RcCell<Key>),
    Proof(RcCell<Proof>),
    ArrayBase(RcCell<ArrayBase>),
    Array(RcCell<Array>),
    IdentifierDeclaration(RcCell<IdentifierDeclaration>),
    VariableDeclaration(RcCell<VariableDeclaration>),
    Parameter(RcCell<Parameter>),
    StateVariableDeclaration(RcCell<StateVariableDeclaration>),
    NamespaceDefinition(RcCell<NamespaceDefinition>),
    ConstructorOrFunctionDefinition(RcCell<ConstructorOrFunctionDefinition>),
    EnumDefinition(RcCell<EnumDefinition>),
    StructDefinition(RcCell<StructDefinition>),
    ContractDefinition(RcCell<ContractDefinition>),
    DummyAnnotation(RcCell<DummyAnnotation>),
    CircuitStatement(RcCell<CircuitStatement>),
    CircComment(RcCell<CircComment>),
    CircIndentBlock(RcCell<CircIndentBlock>),
    CircCall(RcCell<CircCall>),
    CircVarDecl(RcCell<CircVarDecl>),
    CircGuardModification(RcCell<CircGuardModification>),
    CircEncConstraint(RcCell<CircEncConstraint>),
    CircSymmEncConstraint(RcCell<CircSymmEncConstraint>),
    CircEqConstraint(RcCell<CircEqConstraint>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ASTFlattenWeak {
    AST(WeakCell<AST>),
    Expression(WeakCell<Expression>),
    Identifier(WeakCell<Identifier>),
    IdentifierBase(WeakCell<IdentifierBase>),
    Comment(WeakCell<Comment>),
    CommentBase(WeakCell<CommentBase>),
    AnnotatedTypeName(WeakCell<AnnotatedTypeName>),
    EnumValue(WeakCell<EnumValue>),
    SourceUnit(WeakCell<SourceUnit>),
    BlankLine(WeakCell<BlankLine>),
    BuiltinFunction(WeakCell<BuiltinFunction>),
    FunctionCallExprBase(WeakCell<FunctionCallExprBase>),
    FunctionCallExpr(WeakCell<FunctionCallExpr>),
    NewExpr(WeakCell<NewExpr>),
    PrimitiveCastExpr(WeakCell<PrimitiveCastExpr>),
    MeExpr(WeakCell<MeExpr>),
    AllExpr(WeakCell<AllExpr>),
    ReclassifyExpr(WeakCell<ReclassifyExpr>),
    LiteralExpr(WeakCell<LiteralExpr>),
    BooleanLiteralExpr(WeakCell<BooleanLiteralExpr>),
    NumberLiteralExpr(WeakCell<NumberLiteralExpr>),
    StringLiteralExpr(WeakCell<StringLiteralExpr>),
    ArrayLiteralExprBase(WeakCell<ArrayLiteralExprBase>),
    ArrayLiteralExpr(WeakCell<ArrayLiteralExpr>),
    KeyLiteralExpr(WeakCell<KeyLiteralExpr>),
    TupleOrLocationExpr(WeakCell<TupleOrLocationExpr>),
    TupleExpr(WeakCell<TupleExpr>),
    IdentifierExpr(WeakCell<IdentifierExpr>),
    MemberAccessExpr(WeakCell<MemberAccessExpr>),
    LocationExpr(WeakCell<LocationExpr>),
    IndexExpr(WeakCell<IndexExpr>),
    SliceExpr(WeakCell<SliceExpr>),
    ReclassifyExprBase(WeakCell<ReclassifyExprBase>),
    RehomExpr(WeakCell<RehomExpr>),
    EncryptionExpression(WeakCell<EncryptionExpression>),
    HybridArgumentIdf(WeakCell<HybridArgumentIdf>),
    Statement(WeakCell<Statement>),
    IfStatement(WeakCell<IfStatement>),
    WhileStatement(WeakCell<WhileStatement>),
    DoWhileStatement(WeakCell<DoWhileStatement>),
    ForStatement(WeakCell<ForStatement>),
    BreakStatement(WeakCell<BreakStatement>),
    ContinueStatement(WeakCell<ContinueStatement>),
    ReturnStatement(WeakCell<ReturnStatement>),
    StatementListBase(WeakCell<StatementListBase>),
    StatementList(WeakCell<StatementList>),
    CircuitDirectiveStatement(WeakCell<CircuitDirectiveStatement>),
    CircuitComputationStatement(WeakCell<CircuitComputationStatement>),
    EnterPrivateKeyStatement(WeakCell<EnterPrivateKeyStatement>),
    ExpressionStatement(WeakCell<ExpressionStatement>),
    RequireStatement(WeakCell<RequireStatement>),
    AssignmentStatementBase(WeakCell<AssignmentStatementBase>),
    AssignmentStatement(WeakCell<AssignmentStatement>),
    VariableDeclarationStatement(WeakCell<VariableDeclarationStatement>),
    CircuitInputStatement(WeakCell<CircuitInputStatement>),
    SimpleStatement(WeakCell<SimpleStatement>),
    Block(WeakCell<Block>),
    IndentBlock(WeakCell<IndentBlock>),
    Mapping(WeakCell<Mapping>),
    TupleType(WeakCell<TupleType>),
    TypeName(WeakCell<TypeName>),
    ElementaryTypeName(WeakCell<ElementaryTypeName>),
    FunctionTypeName(WeakCell<FunctionTypeName>),
    BoolTypeName(WeakCell<BoolTypeName>),
    BooleanLiteralType(WeakCell<BooleanLiteralType>),
    NumberLiteralType(WeakCell<NumberLiteralType>),
    IntTypeName(WeakCell<IntTypeName>),
    UintTypeName(WeakCell<UintTypeName>),
    NumberTypeNameBase(WeakCell<NumberTypeNameBase>),
    NumberTypeName(WeakCell<NumberTypeName>),
    EnumTypeName(WeakCell<EnumTypeName>),
    EnumValueTypeName(WeakCell<EnumValueTypeName>),
    StructTypeName(WeakCell<StructTypeName>),
    ContractTypeName(WeakCell<ContractTypeName>),
    AddressTypeName(WeakCell<AddressTypeName>),
    AddressPayableTypeName(WeakCell<AddressPayableTypeName>),
    UserDefinedTypeName(WeakCell<UserDefinedTypeName>),
    CipherText(WeakCell<CipherText>),
    Randomness(WeakCell<Randomness>),
    Key(WeakCell<Key>),
    Proof(WeakCell<Proof>),
    ArrayBase(WeakCell<ArrayBase>),
    Array(WeakCell<Array>),
    IdentifierDeclaration(WeakCell<IdentifierDeclaration>),
    VariableDeclaration(WeakCell<VariableDeclaration>),
    Parameter(WeakCell<Parameter>),
    StateVariableDeclaration(WeakCell<StateVariableDeclaration>),
    NamespaceDefinition(WeakCell<NamespaceDefinition>),
    ConstructorOrFunctionDefinition(WeakCell<ConstructorOrFunctionDefinition>),
    EnumDefinition(WeakCell<EnumDefinition>),
    StructDefinition(WeakCell<StructDefinition>),
    ContractDefinition(WeakCell<ContractDefinition>),
    DummyAnnotation(WeakCell<DummyAnnotation>),
    CircuitStatement(WeakCell<CircuitStatement>),
    CircComment(WeakCell<CircComment>),
    CircIndentBlock(WeakCell<CircIndentBlock>),
    CircCall(WeakCell<CircCall>),
    CircVarDecl(WeakCell<CircVarDecl>),
    CircGuardModification(WeakCell<CircGuardModification>),
    CircEncConstraint(WeakCell<CircEncConstraint>),
    CircSymmEncConstraint(WeakCell<CircSymmEncConstraint>),
    CircEqConstraint(WeakCell<CircEqConstraint>),
}
impl ASTChildren for ASTFlatten {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        match self {
            Self::AST(astf) => astf.borrow().process_children(cb),
            Self::Expression(astf) => astf.borrow().process_children(cb),
            // Self::Identifier(astf) => astf.borrow().process_children(cb),
            // Self::IdentifierBase(astf) => astf.borrow().process_children(cb),
            // Self::Comment(astf) => astf.borrow().process_children(cb),
            // Self::CommentBase(astf) => astf.borrow().process_children(cb),
            Self::AnnotatedTypeName(astf) => astf.borrow().process_children(cb),
            Self::EnumValue(astf) => astf.borrow().process_children(cb),
            Self::SourceUnit(astf) => astf.borrow().process_children(cb),
            // Self::BlankLine(astf) => astf.borrow().process_children(cb),
            Self::BuiltinFunction(astf) => astf.borrow().process_children(cb),
            Self::FunctionCallExprBase(astf) => astf.borrow().process_children(cb),
            Self::FunctionCallExpr(astf) => astf.borrow().process_children(cb),
            Self::NewExpr(astf) => astf.borrow().process_children(cb),
            Self::PrimitiveCastExpr(astf) => astf.borrow().process_children(cb),
            Self::MeExpr(astf) => astf.borrow().process_children(cb),
            Self::AllExpr(astf) => astf.borrow().process_children(cb),
            Self::ReclassifyExpr(astf) => astf.borrow().process_children(cb),
            Self::LiteralExpr(astf) => astf.borrow().process_children(cb),
            Self::BooleanLiteralExpr(astf) => astf.borrow().process_children(cb),
            Self::NumberLiteralExpr(astf) => astf.borrow().process_children(cb),
            Self::StringLiteralExpr(astf) => astf.borrow().process_children(cb),
            Self::ArrayLiteralExprBase(astf) => astf.borrow().process_children(cb),
            Self::ArrayLiteralExpr(astf) => astf.borrow().process_children(cb),
            Self::KeyLiteralExpr(astf) => astf.borrow().process_children(cb),
            Self::TupleOrLocationExpr(astf) => astf.borrow().process_children(cb),
            Self::TupleExpr(astf) => astf.borrow().process_children(cb),
            Self::IdentifierExpr(astf) => astf.borrow().process_children(cb),
            Self::MemberAccessExpr(astf) => astf.borrow().process_children(cb),
            Self::LocationExpr(astf) => astf.borrow().process_children(cb),
            Self::IndexExpr(astf) => astf.borrow().process_children(cb),
            Self::SliceExpr(astf) => astf.borrow().process_children(cb),
            Self::ReclassifyExprBase(astf) => astf.borrow().process_children(cb),
            Self::RehomExpr(astf) => astf.borrow().process_children(cb),
            Self::EncryptionExpression(astf) => astf.borrow().process_children(cb),
            // Self::HybridArgumentIdf(astf) => astf.borrow().process_children(cb),
            Self::Statement(astf) => astf.borrow().process_children(cb),
            Self::IfStatement(astf) => astf.borrow().process_children(cb),
            Self::WhileStatement(astf) => astf.borrow().process_children(cb),
            Self::DoWhileStatement(astf) => astf.borrow().process_children(cb),
            Self::ForStatement(astf) => astf.borrow().process_children(cb),
            Self::BreakStatement(astf) => astf.borrow().process_children(cb),
            Self::ContinueStatement(astf) => astf.borrow().process_children(cb),
            Self::ReturnStatement(astf) => astf.borrow().process_children(cb),
            Self::StatementListBase(astf) => astf.borrow().process_children(cb),
            Self::StatementList(astf) => astf.borrow().process_children(cb),
            Self::CircuitDirectiveStatement(astf) => astf.borrow().process_children(cb),
            Self::CircuitComputationStatement(astf) => astf.borrow().process_children(cb),
            Self::EnterPrivateKeyStatement(astf) => astf.borrow().process_children(cb),
            Self::ExpressionStatement(astf) => astf.borrow().process_children(cb),
            Self::RequireStatement(astf) => astf.borrow().process_children(cb),
            Self::AssignmentStatementBase(astf) => astf.borrow().process_children(cb),
            Self::AssignmentStatement(astf) => astf.borrow().process_children(cb),
            Self::VariableDeclarationStatement(astf) => astf.borrow().process_children(cb),
            Self::CircuitInputStatement(astf) => astf.borrow().process_children(cb),
            Self::SimpleStatement(astf) => astf.borrow().process_children(cb),
            Self::Block(astf) => astf.borrow().process_children(cb),
            Self::IndentBlock(astf) => astf.borrow().process_children(cb),
            Self::Mapping(astf) => astf.borrow().process_children(cb),
            // Self::TupleType(astf) => astf.borrow().process_children(cb),
            Self::TypeName(astf) => astf.borrow().process_children(cb),
            Self::ElementaryTypeName(astf) => astf.borrow().process_children(cb),
            Self::FunctionTypeName(astf) => astf.borrow().process_children(cb),
            Self::BoolTypeName(astf) => astf.borrow().process_children(cb),
            Self::BooleanLiteralType(astf) => astf.borrow().process_children(cb),
            Self::NumberLiteralType(astf) => astf.borrow().process_children(cb),
            Self::IntTypeName(astf) => astf.borrow().process_children(cb),
            Self::UintTypeName(astf) => astf.borrow().process_children(cb),
            Self::NumberTypeNameBase(astf) => astf.borrow().process_children(cb),
            Self::NumberTypeName(astf) => astf.borrow().process_children(cb),
            Self::EnumTypeName(astf) => astf.borrow().process_children(cb),
            Self::EnumValueTypeName(astf) => astf.borrow().process_children(cb),
            Self::StructTypeName(astf) => astf.borrow().process_children(cb),
            Self::ContractTypeName(astf) => astf.borrow().process_children(cb),
            Self::AddressTypeName(astf) => astf.borrow().process_children(cb),
            Self::AddressPayableTypeName(astf) => astf.borrow().process_children(cb),
            Self::UserDefinedTypeName(astf) => astf.borrow().process_children(cb),
            Self::CipherText(astf) => astf.borrow().process_children(cb),
            Self::Randomness(astf) => astf.borrow().process_children(cb),
            Self::Key(astf) => astf.borrow().process_children(cb),
            Self::Proof(astf) => astf.borrow().process_children(cb),
            Self::ArrayBase(astf) => astf.borrow().process_children(cb),
            Self::Array(astf) => astf.borrow().process_children(cb),
            Self::IdentifierDeclaration(astf) => astf.borrow().process_children(cb),
            Self::VariableDeclaration(astf) => astf.borrow().process_children(cb),
            Self::Parameter(astf) => astf.borrow().process_children(cb),
            Self::StateVariableDeclaration(astf) => astf.borrow().process_children(cb),
            Self::NamespaceDefinition(astf) => astf.borrow().process_children(cb),
            Self::ConstructorOrFunctionDefinition(astf) => astf.borrow().process_children(cb),
            Self::EnumDefinition(astf) => astf.borrow().process_children(cb),
            Self::StructDefinition(astf) => astf.borrow().process_children(cb),
            Self::ContractDefinition(astf) => astf.borrow().process_children(cb),
            Self::DummyAnnotation(astf) => astf.borrow().process_children(cb),
            // Self::CircComment(astf) => astf.borrow().process_children(cb),
            // Self::CircIndentBlock(astf) => astf.borrow().process_children(cb),
            // Self::CircCall(astf) => astf.borrow().process_children(cb),
            // Self::CircVarDecl(astf) => astf.borrow().process_children(cb),
            // Self::CircGuardModification(astf) => astf.borrow().process_children(cb),
            // Self::CircEncConstraint(astf) => astf.borrow().process_children(cb),
            // Self::CircSymmEncConstraint(astf) => astf.borrow().process_children(cb),
            // Self::CircEqConstraint(astf) => astf.borrow().process_children(cb),
            _ => {}
        }
    }
}
impl ASTInstanceOf for ASTFlatten {
    fn get_ast_type(&self) -> ASTType {
        match self {
            Self::AST(astf) => astf.borrow().get_ast_type(),
            Self::Expression(astf) => astf.borrow().get_ast_type(),
            Self::Identifier(astf) => astf.borrow().get_ast_type(),
            Self::IdentifierBase(astf) => astf.borrow().get_ast_type(),
            Self::Comment(astf) => astf.borrow().get_ast_type(),
            Self::CommentBase(astf) => astf.borrow().get_ast_type(),
            Self::AnnotatedTypeName(astf) => astf.borrow().get_ast_type(),
            Self::EnumValue(astf) => astf.borrow().get_ast_type(),
            Self::SourceUnit(astf) => astf.borrow().get_ast_type(),
            Self::BlankLine(astf) => astf.borrow().get_ast_type(),
            Self::BuiltinFunction(astf) => astf.borrow().get_ast_type(),
            Self::FunctionCallExprBase(astf) => astf.borrow().get_ast_type(),
            Self::FunctionCallExpr(astf) => astf.borrow().get_ast_type(),
            Self::NewExpr(astf) => astf.borrow().get_ast_type(),
            Self::PrimitiveCastExpr(astf) => astf.borrow().get_ast_type(),
            Self::MeExpr(astf) => astf.borrow().get_ast_type(),
            Self::AllExpr(astf) => astf.borrow().get_ast_type(),
            Self::ReclassifyExpr(astf) => astf.borrow().get_ast_type(),
            Self::LiteralExpr(astf) => astf.borrow().get_ast_type(),
            Self::BooleanLiteralExpr(astf) => astf.borrow().get_ast_type(),
            Self::NumberLiteralExpr(astf) => astf.borrow().get_ast_type(),
            Self::StringLiteralExpr(astf) => astf.borrow().get_ast_type(),
            Self::ArrayLiteralExprBase(astf) => astf.borrow().get_ast_type(),
            Self::ArrayLiteralExpr(astf) => astf.borrow().get_ast_type(),
            Self::KeyLiteralExpr(astf) => astf.borrow().get_ast_type(),
            Self::TupleOrLocationExpr(astf) => astf.borrow().get_ast_type(),
            Self::TupleExpr(astf) => astf.borrow().get_ast_type(),
            Self::IdentifierExpr(astf) => astf.borrow().get_ast_type(),
            Self::MemberAccessExpr(astf) => astf.borrow().get_ast_type(),
            Self::LocationExpr(astf) => astf.borrow().get_ast_type(),
            Self::IndexExpr(astf) => astf.borrow().get_ast_type(),
            Self::SliceExpr(astf) => astf.borrow().get_ast_type(),
            Self::ReclassifyExprBase(astf) => astf.borrow().get_ast_type(),
            Self::RehomExpr(astf) => astf.borrow().get_ast_type(),
            Self::EncryptionExpression(astf) => astf.borrow().get_ast_type(),
            Self::HybridArgumentIdf(astf) => astf.borrow().get_ast_type(),
            Self::Statement(astf) => astf.borrow().get_ast_type(),
            Self::IfStatement(astf) => astf.borrow().get_ast_type(),
            Self::WhileStatement(astf) => astf.borrow().get_ast_type(),
            Self::DoWhileStatement(astf) => astf.borrow().get_ast_type(),
            Self::ForStatement(astf) => astf.borrow().get_ast_type(),
            Self::BreakStatement(astf) => astf.borrow().get_ast_type(),
            Self::ContinueStatement(astf) => astf.borrow().get_ast_type(),
            Self::ReturnStatement(astf) => astf.borrow().get_ast_type(),
            Self::StatementListBase(astf) => astf.borrow().get_ast_type(),
            Self::StatementList(astf) => astf.borrow().get_ast_type(),
            Self::CircuitDirectiveStatement(astf) => astf.borrow().get_ast_type(),
            Self::CircuitComputationStatement(astf) => astf.borrow().get_ast_type(),
            Self::EnterPrivateKeyStatement(astf) => astf.borrow().get_ast_type(),
            Self::ExpressionStatement(astf) => astf.borrow().get_ast_type(),
            Self::RequireStatement(astf) => astf.borrow().get_ast_type(),
            Self::AssignmentStatementBase(astf) => astf.borrow().get_ast_type(),
            Self::AssignmentStatement(astf) => astf.borrow().get_ast_type(),
            Self::VariableDeclarationStatement(astf) => astf.borrow().get_ast_type(),
            Self::CircuitInputStatement(astf) => astf.borrow().get_ast_type(),
            Self::SimpleStatement(astf) => astf.borrow().get_ast_type(),
            Self::Block(astf) => astf.borrow().get_ast_type(),
            Self::IndentBlock(astf) => astf.borrow().get_ast_type(),
            Self::Mapping(astf) => astf.borrow().get_ast_type(),
            Self::TupleType(astf) => astf.borrow().get_ast_type(),
            Self::TypeName(astf) => astf.borrow().get_ast_type(),
            Self::ElementaryTypeName(astf) => astf.borrow().get_ast_type(),
            Self::FunctionTypeName(astf) => astf.borrow().get_ast_type(),
            Self::BoolTypeName(astf) => astf.borrow().get_ast_type(),
            Self::BooleanLiteralType(astf) => astf.borrow().get_ast_type(),
            Self::NumberLiteralType(astf) => astf.borrow().get_ast_type(),
            Self::IntTypeName(astf) => astf.borrow().get_ast_type(),
            Self::UintTypeName(astf) => astf.borrow().get_ast_type(),
            Self::NumberTypeNameBase(astf) => astf.borrow().get_ast_type(),
            Self::NumberTypeName(astf) => astf.borrow().get_ast_type(),
            Self::EnumTypeName(astf) => astf.borrow().get_ast_type(),
            Self::EnumValueTypeName(astf) => astf.borrow().get_ast_type(),
            Self::StructTypeName(astf) => astf.borrow().get_ast_type(),
            Self::ContractTypeName(astf) => astf.borrow().get_ast_type(),
            Self::AddressTypeName(astf) => astf.borrow().get_ast_type(),
            Self::AddressPayableTypeName(astf) => astf.borrow().get_ast_type(),
            Self::UserDefinedTypeName(astf) => astf.borrow().get_ast_type(),
            Self::CipherText(astf) => astf.borrow().get_ast_type(),
            Self::Randomness(astf) => astf.borrow().get_ast_type(),
            Self::Key(astf) => astf.borrow().get_ast_type(),
            Self::Proof(astf) => astf.borrow().get_ast_type(),
            Self::ArrayBase(astf) => astf.borrow().get_ast_type(),
            Self::Array(astf) => astf.borrow().get_ast_type(),
            Self::IdentifierDeclaration(astf) => astf.borrow().get_ast_type(),
            Self::VariableDeclaration(astf) => astf.borrow().get_ast_type(),
            Self::Parameter(astf) => astf.borrow().get_ast_type(),
            Self::StateVariableDeclaration(astf) => astf.borrow().get_ast_type(),
            Self::NamespaceDefinition(astf) => astf.borrow().get_ast_type(),
            Self::ConstructorOrFunctionDefinition(astf) => astf.borrow().get_ast_type(),
            Self::EnumDefinition(astf) => astf.borrow().get_ast_type(),
            Self::StructDefinition(astf) => astf.borrow().get_ast_type(),
            Self::ContractDefinition(astf) => astf.borrow().get_ast_type(),
            Self::DummyAnnotation(astf) => astf.borrow().get_ast_type(),
            Self::CircuitStatement(astf) => astf.borrow().get_ast_type(),
            Self::CircComment(astf) => astf.borrow().get_ast_type(),
            Self::CircIndentBlock(astf) => astf.borrow().get_ast_type(),
            Self::CircCall(astf) => astf.borrow().get_ast_type(),
            Self::CircVarDecl(astf) => astf.borrow().get_ast_type(),
            Self::CircGuardModification(astf) => astf.borrow().get_ast_type(),
            Self::CircEncConstraint(astf) => astf.borrow().get_ast_type(),
            Self::CircSymmEncConstraint(astf) => astf.borrow().get_ast_type(),
            Self::CircEqConstraint(astf) => astf.borrow().get_ast_type(),
        }
    }
}
impl IntoAST for ASTFlatten {
    fn into_ast(self) -> AST {
        match self {
            Self::AST(astf) => astf.borrow().clone().into_ast(),
            Self::Expression(astf) => astf.borrow().clone().into_ast(),
            Self::Identifier(astf) => astf.borrow().clone().into_ast(),
            Self::IdentifierBase(astf) => astf.borrow().clone().into_ast(),
            Self::Comment(astf) => astf.borrow().clone().into_ast(),
            Self::CommentBase(astf) => astf.borrow().clone().into_ast(),
            Self::AnnotatedTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::EnumValue(astf) => astf.borrow().clone().into_ast(),
            Self::SourceUnit(astf) => astf.borrow().clone().into_ast(),
            Self::BlankLine(astf) => astf.borrow().clone().into_ast(),
            Self::BuiltinFunction(astf) => astf.borrow().clone().into_ast(),
            Self::FunctionCallExprBase(astf) => astf.borrow().clone().into_ast(),
            Self::FunctionCallExpr(astf) => astf.borrow().clone().into_ast(),
            Self::NewExpr(astf) => astf.borrow().clone().into_ast(),
            Self::PrimitiveCastExpr(astf) => astf.borrow().clone().into_ast(),
            Self::MeExpr(astf) => astf.borrow().clone().into_ast(),
            Self::AllExpr(astf) => astf.borrow().clone().into_ast(),
            Self::ReclassifyExpr(astf) => astf.borrow().clone().into_ast(),
            Self::LiteralExpr(astf) => astf.borrow().clone().into_ast(),
            Self::BooleanLiteralExpr(astf) => astf.borrow().clone().into_ast(),
            Self::NumberLiteralExpr(astf) => astf.borrow().clone().into_ast(),
            Self::StringLiteralExpr(astf) => astf.borrow().clone().into_ast(),
            Self::ArrayLiteralExprBase(astf) => astf.borrow().clone().into_ast(),
            Self::ArrayLiteralExpr(astf) => astf.borrow().clone().into_ast(),
            Self::KeyLiteralExpr(astf) => astf.borrow().clone().into_ast(),
            Self::TupleOrLocationExpr(astf) => astf.borrow().clone().into_ast(),
            Self::TupleExpr(astf) => astf.borrow().clone().into_ast(),
            Self::IdentifierExpr(astf) => astf.borrow().clone().into_ast(),
            Self::MemberAccessExpr(astf) => astf.borrow().clone().into_ast(),
            Self::LocationExpr(astf) => astf.borrow().clone().into_ast(),
            Self::IndexExpr(astf) => astf.borrow().clone().into_ast(),
            Self::SliceExpr(astf) => astf.borrow().clone().into_ast(),
            Self::ReclassifyExprBase(astf) => astf.borrow().clone().into_ast(),
            Self::RehomExpr(astf) => astf.borrow().clone().into_ast(),
            Self::EncryptionExpression(astf) => astf.borrow().clone().into_ast(),
            Self::HybridArgumentIdf(astf) => astf.borrow().clone().into_ast(),
            Self::Statement(astf) => astf.borrow().clone().into_ast(),
            Self::IfStatement(astf) => astf.borrow().clone().into_ast(),
            Self::WhileStatement(astf) => astf.borrow().clone().into_ast(),
            Self::DoWhileStatement(astf) => astf.borrow().clone().into_ast(),
            Self::ForStatement(astf) => astf.borrow().clone().into_ast(),
            Self::BreakStatement(astf) => astf.borrow().clone().into_ast(),
            Self::ContinueStatement(astf) => astf.borrow().clone().into_ast(),
            Self::ReturnStatement(astf) => astf.borrow().clone().into_ast(),
            Self::StatementListBase(astf) => astf.borrow().clone().into_ast(),
            Self::StatementList(astf) => astf.borrow().clone().into_ast(),
            Self::CircuitDirectiveStatement(astf) => astf.borrow().clone().into_ast(),
            Self::CircuitComputationStatement(astf) => astf.borrow().clone().into_ast(),
            Self::EnterPrivateKeyStatement(astf) => astf.borrow().clone().into_ast(),
            Self::ExpressionStatement(astf) => astf.borrow().clone().into_ast(),
            Self::RequireStatement(astf) => astf.borrow().clone().into_ast(),
            Self::AssignmentStatementBase(astf) => astf.borrow().clone().into_ast(),
            Self::AssignmentStatement(astf) => astf.borrow().clone().into_ast(),
            Self::VariableDeclarationStatement(astf) => astf.borrow().clone().into_ast(),
            Self::CircuitInputStatement(astf) => astf.borrow().clone().into_ast(),
            Self::SimpleStatement(astf) => astf.borrow().clone().into_ast(),
            Self::Block(astf) => astf.borrow().clone().into_ast(),
            Self::IndentBlock(astf) => astf.borrow().clone().into_ast(),
            Self::Mapping(astf) => astf.borrow().clone().into_ast(),
            Self::TupleType(astf) => astf.borrow().clone().into_ast(),
            Self::TypeName(astf) => astf.borrow().clone().into_ast(),
            Self::ElementaryTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::FunctionTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::BoolTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::BooleanLiteralType(astf) => astf.borrow().clone().into_ast(),
            Self::NumberLiteralType(astf) => astf.borrow().clone().into_ast(),
            Self::IntTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::UintTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::NumberTypeNameBase(astf) => astf.borrow().clone().into_ast(),
            Self::NumberTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::EnumTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::EnumValueTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::StructTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::ContractTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::AddressTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::AddressPayableTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::UserDefinedTypeName(astf) => astf.borrow().clone().into_ast(),
            Self::CipherText(astf) => astf.borrow().clone().into_ast(),
            Self::Randomness(astf) => astf.borrow().clone().into_ast(),
            Self::Key(astf) => astf.borrow().clone().into_ast(),
            Self::Proof(astf) => astf.borrow().clone().into_ast(),
            Self::ArrayBase(astf) => astf.borrow().clone().into_ast(),
            Self::Array(astf) => astf.borrow().clone().into_ast(),
            Self::IdentifierDeclaration(astf) => astf.borrow().clone().into_ast(),
            Self::VariableDeclaration(astf) => astf.borrow().clone().into_ast(),
            Self::Parameter(astf) => astf.borrow().clone().into_ast(),
            Self::StateVariableDeclaration(astf) => astf.borrow().clone().into_ast(),
            Self::NamespaceDefinition(astf) => astf.borrow().clone().into_ast(),
            Self::ConstructorOrFunctionDefinition(astf) => astf.borrow().clone().into_ast(),
            Self::EnumDefinition(astf) => astf.borrow().clone().into_ast(),
            Self::StructDefinition(astf) => astf.borrow().clone().into_ast(),
            Self::ContractDefinition(astf) => astf.borrow().clone().into_ast(),
            Self::DummyAnnotation(astf) => astf.borrow().clone().into_ast(),
            Self::CircuitStatement(astf) => astf.borrow().clone().into_ast(),
            Self::CircComment(astf) => astf.borrow().clone().into_ast(),
            Self::CircIndentBlock(astf) => astf.borrow().clone().into_ast(),
            Self::CircCall(astf) => astf.borrow().clone().into_ast(),
            Self::CircVarDecl(astf) => astf.borrow().clone().into_ast(),
            Self::CircGuardModification(astf) => astf.borrow().clone().into_ast(),
            Self::CircEncConstraint(astf) => astf.borrow().clone().into_ast(),
            Self::CircSymmEncConstraint(astf) => astf.borrow().clone().into_ast(),
            Self::CircEqConstraint(astf) => astf.borrow().clone().into_ast(),
        }
    }
}
impl ASTFlatten {
    pub fn clone_inner(&self) -> ASTFlatten {
        match self {
            Self::AST(astf) => ASTFlatten::AST(RcCell::new(astf.borrow().clone())),
            Self::Expression(astf) => ASTFlatten::Expression(RcCell::new(astf.borrow().clone())),
            Self::Identifier(astf) => ASTFlatten::Identifier(RcCell::new(astf.borrow().clone())),
            Self::IdentifierBase(astf) => {
                ASTFlatten::IdentifierBase(RcCell::new(astf.borrow().clone()))
            }
            Self::Comment(astf) => ASTFlatten::Comment(RcCell::new(astf.borrow().clone())),
            Self::CommentBase(astf) => ASTFlatten::CommentBase(RcCell::new(astf.borrow().clone())),
            Self::AnnotatedTypeName(astf) => {
                ASTFlatten::AnnotatedTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::EnumValue(astf) => ASTFlatten::EnumValue(RcCell::new(astf.borrow().clone())),
            Self::SourceUnit(astf) => ASTFlatten::SourceUnit(RcCell::new(astf.borrow().clone())),
            Self::BlankLine(astf) => ASTFlatten::BlankLine(RcCell::new(astf.borrow().clone())),
            Self::BuiltinFunction(astf) => {
                ASTFlatten::BuiltinFunction(RcCell::new(astf.borrow().clone()))
            }
            Self::FunctionCallExprBase(astf) => {
                ASTFlatten::FunctionCallExprBase(RcCell::new(astf.borrow().clone()))
            }
            Self::FunctionCallExpr(astf) => {
                ASTFlatten::FunctionCallExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::NewExpr(astf) => ASTFlatten::NewExpr(RcCell::new(astf.borrow().clone())),
            Self::PrimitiveCastExpr(astf) => {
                ASTFlatten::PrimitiveCastExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::MeExpr(astf) => ASTFlatten::MeExpr(RcCell::new(astf.borrow().clone())),
            Self::AllExpr(astf) => ASTFlatten::AllExpr(RcCell::new(astf.borrow().clone())),
            Self::ReclassifyExpr(astf) => {
                ASTFlatten::ReclassifyExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::LiteralExpr(astf) => ASTFlatten::LiteralExpr(RcCell::new(astf.borrow().clone())),
            Self::BooleanLiteralExpr(astf) => {
                ASTFlatten::BooleanLiteralExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::NumberLiteralExpr(astf) => {
                ASTFlatten::NumberLiteralExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::StringLiteralExpr(astf) => {
                ASTFlatten::StringLiteralExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::ArrayLiteralExprBase(astf) => {
                ASTFlatten::ArrayLiteralExprBase(RcCell::new(astf.borrow().clone()))
            }
            Self::ArrayLiteralExpr(astf) => {
                ASTFlatten::ArrayLiteralExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::KeyLiteralExpr(astf) => {
                ASTFlatten::KeyLiteralExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::TupleOrLocationExpr(astf) => {
                ASTFlatten::TupleOrLocationExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::TupleExpr(astf) => ASTFlatten::TupleExpr(RcCell::new(astf.borrow().clone())),
            Self::IdentifierExpr(astf) => {
                ASTFlatten::IdentifierExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::MemberAccessExpr(astf) => {
                ASTFlatten::MemberAccessExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::LocationExpr(astf) => {
                ASTFlatten::LocationExpr(RcCell::new(astf.borrow().clone()))
            }
            Self::IndexExpr(astf) => ASTFlatten::IndexExpr(RcCell::new(astf.borrow().clone())),
            Self::SliceExpr(astf) => ASTFlatten::SliceExpr(RcCell::new(astf.borrow().clone())),
            Self::ReclassifyExprBase(astf) => {
                ASTFlatten::ReclassifyExprBase(RcCell::new(astf.borrow().clone()))
            }
            Self::RehomExpr(astf) => ASTFlatten::RehomExpr(RcCell::new(astf.borrow().clone())),
            Self::EncryptionExpression(astf) => {
                ASTFlatten::EncryptionExpression(RcCell::new(astf.borrow().clone()))
            }
            Self::HybridArgumentIdf(astf) => {
                ASTFlatten::HybridArgumentIdf(RcCell::new(astf.borrow().clone()))
            }
            Self::Statement(astf) => ASTFlatten::Statement(RcCell::new(astf.borrow().clone())),
            Self::IfStatement(astf) => ASTFlatten::IfStatement(RcCell::new(astf.borrow().clone())),
            Self::WhileStatement(astf) => {
                ASTFlatten::WhileStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::DoWhileStatement(astf) => {
                ASTFlatten::DoWhileStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::ForStatement(astf) => {
                ASTFlatten::ForStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::BreakStatement(astf) => {
                ASTFlatten::BreakStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::ContinueStatement(astf) => {
                ASTFlatten::ContinueStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::ReturnStatement(astf) => {
                ASTFlatten::ReturnStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::StatementListBase(astf) => {
                ASTFlatten::StatementListBase(RcCell::new(astf.borrow().clone()))
            }
            Self::StatementList(astf) => {
                ASTFlatten::StatementList(RcCell::new(astf.borrow().clone()))
            }
            Self::CircuitDirectiveStatement(astf) => {
                ASTFlatten::CircuitDirectiveStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::CircuitComputationStatement(astf) => {
                ASTFlatten::CircuitComputationStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::EnterPrivateKeyStatement(astf) => {
                ASTFlatten::EnterPrivateKeyStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::ExpressionStatement(astf) => {
                ASTFlatten::ExpressionStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::RequireStatement(astf) => {
                ASTFlatten::RequireStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::AssignmentStatementBase(astf) => {
                ASTFlatten::AssignmentStatementBase(RcCell::new(astf.borrow().clone()))
            }
            Self::AssignmentStatement(astf) => {
                ASTFlatten::AssignmentStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::VariableDeclarationStatement(astf) => {
                ASTFlatten::VariableDeclarationStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::CircuitInputStatement(astf) => {
                ASTFlatten::CircuitInputStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::SimpleStatement(astf) => {
                ASTFlatten::SimpleStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::Block(astf) => ASTFlatten::Block(RcCell::new(astf.borrow().clone())),
            Self::IndentBlock(astf) => ASTFlatten::IndentBlock(RcCell::new(astf.borrow().clone())),
            Self::Mapping(astf) => ASTFlatten::Mapping(RcCell::new(astf.borrow().clone())),
            Self::TupleType(astf) => ASTFlatten::TupleType(RcCell::new(astf.borrow().clone())),
            Self::TypeName(astf) => ASTFlatten::TypeName(RcCell::new(astf.borrow().clone())),
            Self::ElementaryTypeName(astf) => {
                ASTFlatten::ElementaryTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::FunctionTypeName(astf) => {
                ASTFlatten::FunctionTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::BoolTypeName(astf) => {
                ASTFlatten::BoolTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::BooleanLiteralType(astf) => {
                ASTFlatten::BooleanLiteralType(RcCell::new(astf.borrow().clone()))
            }
            Self::NumberLiteralType(astf) => {
                ASTFlatten::NumberLiteralType(RcCell::new(astf.borrow().clone()))
            }
            Self::IntTypeName(astf) => ASTFlatten::IntTypeName(RcCell::new(astf.borrow().clone())),
            Self::UintTypeName(astf) => {
                ASTFlatten::UintTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::NumberTypeNameBase(astf) => {
                ASTFlatten::NumberTypeNameBase(RcCell::new(astf.borrow().clone()))
            }
            Self::NumberTypeName(astf) => {
                ASTFlatten::NumberTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::EnumTypeName(astf) => {
                ASTFlatten::EnumTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::EnumValueTypeName(astf) => {
                ASTFlatten::EnumValueTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::StructTypeName(astf) => {
                ASTFlatten::StructTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::ContractTypeName(astf) => {
                ASTFlatten::ContractTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::AddressTypeName(astf) => {
                ASTFlatten::AddressTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::AddressPayableTypeName(astf) => {
                ASTFlatten::AddressPayableTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::UserDefinedTypeName(astf) => {
                ASTFlatten::UserDefinedTypeName(RcCell::new(astf.borrow().clone()))
            }
            Self::CipherText(astf) => ASTFlatten::CipherText(RcCell::new(astf.borrow().clone())),
            Self::Randomness(astf) => ASTFlatten::Randomness(RcCell::new(astf.borrow().clone())),
            Self::Key(astf) => ASTFlatten::Key(RcCell::new(astf.borrow().clone())),
            Self::Proof(astf) => ASTFlatten::Proof(RcCell::new(astf.borrow().clone())),
            Self::ArrayBase(astf) => ASTFlatten::ArrayBase(RcCell::new(astf.borrow().clone())),
            Self::Array(astf) => ASTFlatten::Array(RcCell::new(astf.borrow().clone())),
            Self::IdentifierDeclaration(astf) => {
                ASTFlatten::IdentifierDeclaration(RcCell::new(astf.borrow().clone()))
            }
            Self::VariableDeclaration(astf) => {
                ASTFlatten::VariableDeclaration(RcCell::new(astf.borrow().clone()))
            }
            Self::Parameter(astf) => ASTFlatten::Parameter(RcCell::new(astf.borrow().clone())),
            Self::StateVariableDeclaration(astf) => {
                ASTFlatten::StateVariableDeclaration(RcCell::new(astf.borrow().clone()))
            }
            Self::NamespaceDefinition(astf) => {
                ASTFlatten::NamespaceDefinition(RcCell::new(astf.borrow().clone()))
            }
            Self::ConstructorOrFunctionDefinition(astf) => {
                ASTFlatten::ConstructorOrFunctionDefinition(RcCell::new(astf.borrow().clone()))
            }
            Self::EnumDefinition(astf) => {
                ASTFlatten::EnumDefinition(RcCell::new(astf.borrow().clone()))
            }
            Self::StructDefinition(astf) => {
                ASTFlatten::StructDefinition(RcCell::new(astf.borrow().clone()))
            }
            Self::ContractDefinition(astf) => {
                ASTFlatten::ContractDefinition(RcCell::new(astf.borrow().clone()))
            }
            Self::DummyAnnotation(astf) => {
                ASTFlatten::DummyAnnotation(RcCell::new(astf.borrow().clone()))
            }
            Self::CircuitStatement(astf) => {
                ASTFlatten::CircuitStatement(RcCell::new(astf.borrow().clone()))
            }
            Self::CircComment(astf) => ASTFlatten::CircComment(RcCell::new(astf.borrow().clone())),
            Self::CircIndentBlock(astf) => {
                ASTFlatten::CircIndentBlock(RcCell::new(astf.borrow().clone()))
            }
            Self::CircCall(astf) => ASTFlatten::CircCall(RcCell::new(astf.borrow().clone())),
            Self::CircVarDecl(astf) => ASTFlatten::CircVarDecl(RcCell::new(astf.borrow().clone())),
            Self::CircGuardModification(astf) => {
                ASTFlatten::CircGuardModification(RcCell::new(astf.borrow().clone()))
            }
            Self::CircEncConstraint(astf) => {
                ASTFlatten::CircEncConstraint(RcCell::new(astf.borrow().clone()))
            }
            Self::CircSymmEncConstraint(astf) => {
                ASTFlatten::CircSymmEncConstraint(RcCell::new(astf.borrow().clone()))
            }
            Self::CircEqConstraint(astf) => {
                ASTFlatten::CircEqConstraint(RcCell::new(astf.borrow().clone()))
            }
        }
    }
    pub fn is_parent_of(&self, child: &ASTFlatten) -> bool {
        let mut e = child.clone();
        let selfs = self.clone();
        while e != selfs && e.ast_base_ref().unwrap().borrow().parent.is_some() {
            let e1 = e
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent
                .as_ref()
                .map(|p| p.clone().upgrade().unwrap())
                .unwrap();
            e = e1;
        }
        e == selfs
    }
    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        match self {
            Self::EnumValue(astf) => astf.borrow().annotated_type.clone(),
            Self::NewExpr(astf) => Some(astf.borrow().annotated_type.borrow().clone()),
            Self::ConstructorOrFunctionDefinition(astf) => astf.borrow().annotated_type.clone(),
            Self::EnumDefinition(astf) => astf.borrow().annotated_type.clone(),
            Self::Expression(astf) => astf
                .borrow()
                .annotated_type()
                .as_ref()
                .map(|a| a.borrow().clone()),
            Self::IdentifierDeclaration(astf) => astf
                .borrow()
                .annotated_type()
                .as_ref()
                .map(|a| a.borrow().clone()),
            _ if matches!(self.to_ast(), AST::Expression(_)) => self
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .map(|a| a.borrow().clone()),
            _ if matches!(self.to_ast(), AST::IdentifierDeclaration(_)) => Some(
                self.to_ast()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .clone(),
            ),
            _ => None,
        }
    }
    pub fn code(&self) -> String {
        match self {
            Self::AST(astf) => astf.borrow().code(),
            Self::Expression(astf) => astf.borrow().code(),
            Self::Identifier(astf) => astf.borrow().code(),
            Self::IdentifierBase(astf) => astf.borrow().code(),
            Self::Comment(astf) => astf.borrow().code(),
            Self::CommentBase(astf) => astf.borrow().code(),
            Self::AnnotatedTypeName(astf) => astf.borrow().code(),
            Self::EnumValue(astf) => astf.borrow().code(),
            Self::SourceUnit(astf) => astf.borrow().code(),
            Self::BlankLine(astf) => astf.borrow().code(),
            Self::BuiltinFunction(astf) => astf.borrow().code(),
            Self::FunctionCallExprBase(astf) => astf.borrow().code(),
            Self::FunctionCallExpr(astf) => astf.borrow().code(),
            Self::NewExpr(astf) => astf.borrow().code(),
            Self::PrimitiveCastExpr(astf) => astf.borrow().code(),
            Self::MeExpr(astf) => astf.borrow().code(),
            Self::AllExpr(astf) => astf.borrow().code(),
            Self::ReclassifyExpr(astf) => astf.borrow().code(),
            Self::LiteralExpr(astf) => astf.borrow().code(),
            Self::BooleanLiteralExpr(astf) => astf.borrow().code(),
            Self::NumberLiteralExpr(astf) => astf.borrow().code(),
            Self::StringLiteralExpr(astf) => astf.borrow().code(),
            Self::ArrayLiteralExprBase(astf) => astf.borrow().code(),
            Self::ArrayLiteralExpr(astf) => astf.borrow().code(),
            Self::KeyLiteralExpr(astf) => astf.borrow().code(),
            Self::TupleOrLocationExpr(astf) => astf.borrow().code(),
            Self::TupleExpr(astf) => astf.borrow().code(),
            Self::IdentifierExpr(astf) => astf.borrow().code(),
            Self::MemberAccessExpr(astf) => astf.borrow().code(),
            Self::LocationExpr(astf) => astf.borrow().code(),
            Self::IndexExpr(astf) => astf.borrow().code(),
            Self::SliceExpr(astf) => astf.borrow().code(),
            Self::ReclassifyExprBase(astf) => astf.borrow().code(),
            Self::RehomExpr(astf) => astf.borrow().code(),
            Self::EncryptionExpression(astf) => astf.borrow().code(),
            Self::HybridArgumentIdf(astf) => astf.borrow().code(),
            Self::Statement(astf) => astf.borrow().code(),
            Self::IfStatement(astf) => astf.borrow().code(),
            Self::WhileStatement(astf) => astf.borrow().code(),
            Self::DoWhileStatement(astf) => astf.borrow().code(),
            Self::ForStatement(astf) => astf.borrow().code(),
            Self::BreakStatement(astf) => astf.borrow().code(),
            Self::ContinueStatement(astf) => astf.borrow().code(),
            Self::ReturnStatement(astf) => astf.borrow().code(),
            Self::StatementListBase(astf) => astf.borrow().code(),
            Self::StatementList(astf) => astf.borrow().code(),
            Self::CircuitDirectiveStatement(astf) => astf.borrow().code(),
            Self::CircuitComputationStatement(astf) => astf.borrow().code(),
            Self::EnterPrivateKeyStatement(astf) => astf.borrow().code(),
            Self::ExpressionStatement(astf) => astf.borrow().code(),
            Self::RequireStatement(astf) => astf.borrow().code(),
            Self::AssignmentStatementBase(astf) => astf.borrow().code(),
            Self::AssignmentStatement(astf) => astf.borrow().code(),
            Self::VariableDeclarationStatement(astf) => astf.borrow().code(),
            Self::CircuitInputStatement(astf) => astf.borrow().code(),
            Self::SimpleStatement(astf) => astf.borrow().code(),
            Self::Block(astf) => astf.borrow().code(),
            Self::IndentBlock(astf) => astf.borrow().code(),
            Self::Mapping(astf) => astf.borrow().code(),
            Self::TupleType(astf) => astf.borrow().code(),
            Self::TypeName(astf) => astf.borrow().code(),
            Self::ElementaryTypeName(astf) => astf.borrow().code(),
            Self::FunctionTypeName(astf) => astf.borrow().code(),
            Self::BoolTypeName(astf) => astf.borrow().code(),
            Self::BooleanLiteralType(astf) => astf.borrow().code(),
            Self::NumberLiteralType(astf) => astf.borrow().code(),
            Self::IntTypeName(astf) => astf.borrow().code(),
            Self::UintTypeName(astf) => astf.borrow().code(),
            Self::NumberTypeNameBase(astf) => astf.borrow().code(),
            Self::NumberTypeName(astf) => astf.borrow().code(),
            Self::EnumTypeName(astf) => astf.borrow().code(),
            Self::EnumValueTypeName(astf) => astf.borrow().code(),
            Self::StructTypeName(astf) => astf.borrow().code(),
            Self::ContractTypeName(astf) => astf.borrow().code(),
            Self::AddressTypeName(astf) => astf.borrow().code(),
            Self::AddressPayableTypeName(astf) => astf.borrow().code(),
            Self::UserDefinedTypeName(astf) => astf.borrow().code(),
            Self::CipherText(astf) => astf.borrow().code(),
            Self::Randomness(astf) => astf.borrow().code(),
            Self::Key(astf) => astf.borrow().code(),
            Self::Proof(astf) => astf.borrow().code(),
            Self::ArrayBase(astf) => astf.borrow().code(),
            Self::Array(astf) => astf.borrow().code(),
            Self::IdentifierDeclaration(astf) => astf.borrow().code(),
            Self::VariableDeclaration(astf) => astf.borrow().code(),
            Self::Parameter(astf) => astf.borrow().code(),
            Self::StateVariableDeclaration(astf) => astf.borrow().code(),
            Self::NamespaceDefinition(astf) => astf.borrow().code(),
            Self::ConstructorOrFunctionDefinition(astf) => astf.borrow().code(),
            Self::EnumDefinition(astf) => astf.borrow().code(),
            Self::StructDefinition(astf) => astf.borrow().code(),
            Self::ContractDefinition(astf) => astf.borrow().code(),
            Self::DummyAnnotation(astf) => astf.borrow().code(),
            _ => String::new(),
            // Self::CircComment(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircIndentBlock(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircCall(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircVarDecl(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircGuardModification(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircEncConstraint(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircSymmEncConstraint(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircEqConstraint(astf) => Some(astf.borrow().ast_base_ref()),
        }
    }
    pub fn ast_base_ref(&self) -> Option<RcCell<ASTBase>> {
        match self {
            Self::AST(astf) => astf.borrow().ast_base_ref(),
            Self::Expression(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Identifier(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::IdentifierBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Comment(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::CommentBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::AnnotatedTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::EnumValue(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::SourceUnit(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::BlankLine(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::BuiltinFunction(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::FunctionCallExprBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::FunctionCallExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::NewExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::PrimitiveCastExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::MeExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::AllExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ReclassifyExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::LiteralExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::BooleanLiteralExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::NumberLiteralExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::StringLiteralExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ArrayLiteralExprBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ArrayLiteralExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::KeyLiteralExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::TupleOrLocationExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::TupleExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::IdentifierExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::MemberAccessExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::LocationExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::IndexExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::SliceExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ReclassifyExprBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::RehomExpr(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::EncryptionExpression(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::HybridArgumentIdf(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Statement(astf) => astf.borrow().ast_base_ref(),
            Self::IfStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::WhileStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::DoWhileStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ForStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::BreakStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ContinueStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ReturnStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::StatementListBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::StatementList(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::CircuitDirectiveStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::CircuitComputationStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::EnterPrivateKeyStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ExpressionStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::RequireStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::AssignmentStatementBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::AssignmentStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::VariableDeclarationStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::CircuitInputStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::SimpleStatement(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Block(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::IndentBlock(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Mapping(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::TupleType(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::TypeName(astf) => astf.borrow().ast_base_ref(),
            Self::ElementaryTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::FunctionTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::BoolTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::BooleanLiteralType(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::NumberLiteralType(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::IntTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::UintTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::NumberTypeNameBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::NumberTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::EnumTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::EnumValueTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::StructTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ContractTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::AddressTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::AddressPayableTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::UserDefinedTypeName(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::CipherText(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Randomness(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Key(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Proof(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ArrayBase(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Array(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::IdentifierDeclaration(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::VariableDeclaration(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::Parameter(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::StateVariableDeclaration(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::NamespaceDefinition(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ConstructorOrFunctionDefinition(astf) => {
                Some(astf.borrow().ast_base_ref().clone())
            }
            Self::EnumDefinition(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::StructDefinition(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::ContractDefinition(astf) => Some(astf.borrow().ast_base_ref().clone()),
            Self::DummyAnnotation(astf) => Some(astf.borrow().ast_base_ref().clone()),
            _ => None,
            // Self::CircComment(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircIndentBlock(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircCall(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircVarDecl(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircGuardModification(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircEncConstraint(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircSymmEncConstraint(astf) => Some(astf.borrow().ast_base_ref()),
            // Self::CircEqConstraint(astf) => Some(astf.borrow().ast_base_ref()),
        }
    }
    pub fn downgrade(self) -> ASTFlattenWeak {
        match self {
            Self::AST(astf) => ASTFlattenWeak::AST(astf.downgrade()),
            Self::Expression(astf) => ASTFlattenWeak::Expression(astf.downgrade()),
            Self::Identifier(astf) => ASTFlattenWeak::Identifier(astf.downgrade()),
            Self::IdentifierBase(astf) => ASTFlattenWeak::IdentifierBase(astf.downgrade()),
            Self::Comment(astf) => ASTFlattenWeak::Comment(astf.downgrade()),
            Self::CommentBase(astf) => ASTFlattenWeak::CommentBase(astf.downgrade()),
            Self::AnnotatedTypeName(astf) => ASTFlattenWeak::AnnotatedTypeName(astf.downgrade()),
            Self::EnumValue(astf) => ASTFlattenWeak::EnumValue(astf.downgrade()),
            Self::SourceUnit(astf) => ASTFlattenWeak::SourceUnit(astf.downgrade()),
            Self::BlankLine(astf) => ASTFlattenWeak::BlankLine(astf.downgrade()),
            Self::BuiltinFunction(astf) => ASTFlattenWeak::BuiltinFunction(astf.downgrade()),
            Self::FunctionCallExprBase(astf) => {
                ASTFlattenWeak::FunctionCallExprBase(astf.downgrade())
            }
            Self::FunctionCallExpr(astf) => ASTFlattenWeak::FunctionCallExpr(astf.downgrade()),
            Self::NewExpr(astf) => ASTFlattenWeak::NewExpr(astf.downgrade()),
            Self::PrimitiveCastExpr(astf) => ASTFlattenWeak::PrimitiveCastExpr(astf.downgrade()),
            Self::MeExpr(astf) => ASTFlattenWeak::MeExpr(astf.downgrade()),
            Self::AllExpr(astf) => ASTFlattenWeak::AllExpr(astf.downgrade()),
            Self::ReclassifyExpr(astf) => ASTFlattenWeak::ReclassifyExpr(astf.downgrade()),
            Self::LiteralExpr(astf) => ASTFlattenWeak::LiteralExpr(astf.downgrade()),
            Self::BooleanLiteralExpr(astf) => ASTFlattenWeak::BooleanLiteralExpr(astf.downgrade()),
            Self::NumberLiteralExpr(astf) => ASTFlattenWeak::NumberLiteralExpr(astf.downgrade()),
            Self::StringLiteralExpr(astf) => ASTFlattenWeak::StringLiteralExpr(astf.downgrade()),
            Self::ArrayLiteralExprBase(astf) => {
                ASTFlattenWeak::ArrayLiteralExprBase(astf.downgrade())
            }
            Self::ArrayLiteralExpr(astf) => ASTFlattenWeak::ArrayLiteralExpr(astf.downgrade()),
            Self::KeyLiteralExpr(astf) => ASTFlattenWeak::KeyLiteralExpr(astf.downgrade()),
            Self::TupleOrLocationExpr(astf) => {
                ASTFlattenWeak::TupleOrLocationExpr(astf.downgrade())
            }
            Self::TupleExpr(astf) => ASTFlattenWeak::TupleExpr(astf.downgrade()),
            Self::IdentifierExpr(astf) => ASTFlattenWeak::IdentifierExpr(astf.downgrade()),
            Self::MemberAccessExpr(astf) => ASTFlattenWeak::MemberAccessExpr(astf.downgrade()),
            Self::LocationExpr(astf) => ASTFlattenWeak::LocationExpr(astf.downgrade()),
            Self::IndexExpr(astf) => ASTFlattenWeak::IndexExpr(astf.downgrade()),
            Self::SliceExpr(astf) => ASTFlattenWeak::SliceExpr(astf.downgrade()),
            Self::ReclassifyExprBase(astf) => ASTFlattenWeak::ReclassifyExprBase(astf.downgrade()),
            Self::RehomExpr(astf) => ASTFlattenWeak::RehomExpr(astf.downgrade()),
            Self::EncryptionExpression(astf) => {
                ASTFlattenWeak::EncryptionExpression(astf.downgrade())
            }
            Self::HybridArgumentIdf(astf) => ASTFlattenWeak::HybridArgumentIdf(astf.downgrade()),
            Self::Statement(astf) => ASTFlattenWeak::Statement(astf.downgrade()),
            Self::IfStatement(astf) => ASTFlattenWeak::IfStatement(astf.downgrade()),
            Self::WhileStatement(astf) => ASTFlattenWeak::WhileStatement(astf.downgrade()),
            Self::DoWhileStatement(astf) => ASTFlattenWeak::DoWhileStatement(astf.downgrade()),
            Self::ForStatement(astf) => ASTFlattenWeak::ForStatement(astf.downgrade()),
            Self::BreakStatement(astf) => ASTFlattenWeak::BreakStatement(astf.downgrade()),
            Self::ContinueStatement(astf) => ASTFlattenWeak::ContinueStatement(astf.downgrade()),
            Self::ReturnStatement(astf) => ASTFlattenWeak::ReturnStatement(astf.downgrade()),
            Self::StatementListBase(astf) => ASTFlattenWeak::StatementListBase(astf.downgrade()),
            Self::StatementList(astf) => ASTFlattenWeak::StatementList(astf.downgrade()),
            Self::CircuitDirectiveStatement(astf) => {
                ASTFlattenWeak::CircuitDirectiveStatement(astf.downgrade())
            }
            Self::CircuitComputationStatement(astf) => {
                ASTFlattenWeak::CircuitComputationStatement(astf.downgrade())
            }
            Self::EnterPrivateKeyStatement(astf) => {
                ASTFlattenWeak::EnterPrivateKeyStatement(astf.downgrade())
            }
            Self::ExpressionStatement(astf) => {
                ASTFlattenWeak::ExpressionStatement(astf.downgrade())
            }
            Self::RequireStatement(astf) => ASTFlattenWeak::RequireStatement(astf.downgrade()),
            Self::AssignmentStatementBase(astf) => {
                ASTFlattenWeak::AssignmentStatementBase(astf.downgrade())
            }
            Self::AssignmentStatement(astf) => {
                ASTFlattenWeak::AssignmentStatement(astf.downgrade())
            }
            Self::VariableDeclarationStatement(astf) => {
                ASTFlattenWeak::VariableDeclarationStatement(astf.downgrade())
            }
            Self::CircuitInputStatement(astf) => {
                ASTFlattenWeak::CircuitInputStatement(astf.downgrade())
            }
            Self::SimpleStatement(astf) => ASTFlattenWeak::SimpleStatement(astf.downgrade()),
            Self::Block(astf) => ASTFlattenWeak::Block(astf.downgrade()),
            Self::IndentBlock(astf) => ASTFlattenWeak::IndentBlock(astf.downgrade()),
            Self::Mapping(astf) => ASTFlattenWeak::Mapping(astf.downgrade()),
            Self::TupleType(astf) => ASTFlattenWeak::TupleType(astf.downgrade()),
            Self::TypeName(astf) => ASTFlattenWeak::TypeName(astf.downgrade()),
            Self::ElementaryTypeName(astf) => ASTFlattenWeak::ElementaryTypeName(astf.downgrade()),
            Self::FunctionTypeName(astf) => ASTFlattenWeak::FunctionTypeName(astf.downgrade()),
            Self::BoolTypeName(astf) => ASTFlattenWeak::BoolTypeName(astf.downgrade()),
            Self::BooleanLiteralType(astf) => ASTFlattenWeak::BooleanLiteralType(astf.downgrade()),
            Self::NumberLiteralType(astf) => ASTFlattenWeak::NumberLiteralType(astf.downgrade()),
            Self::IntTypeName(astf) => ASTFlattenWeak::IntTypeName(astf.downgrade()),
            Self::UintTypeName(astf) => ASTFlattenWeak::UintTypeName(astf.downgrade()),
            Self::NumberTypeNameBase(astf) => ASTFlattenWeak::NumberTypeNameBase(astf.downgrade()),
            Self::NumberTypeName(astf) => ASTFlattenWeak::NumberTypeName(astf.downgrade()),
            Self::EnumTypeName(astf) => ASTFlattenWeak::EnumTypeName(astf.downgrade()),
            Self::EnumValueTypeName(astf) => ASTFlattenWeak::EnumValueTypeName(astf.downgrade()),
            Self::StructTypeName(astf) => ASTFlattenWeak::StructTypeName(astf.downgrade()),
            Self::ContractTypeName(astf) => ASTFlattenWeak::ContractTypeName(astf.downgrade()),
            Self::AddressTypeName(astf) => ASTFlattenWeak::AddressTypeName(astf.downgrade()),
            Self::AddressPayableTypeName(astf) => {
                ASTFlattenWeak::AddressPayableTypeName(astf.downgrade())
            }
            Self::UserDefinedTypeName(astf) => {
                ASTFlattenWeak::UserDefinedTypeName(astf.downgrade())
            }
            Self::CipherText(astf) => ASTFlattenWeak::CipherText(astf.downgrade()),
            Self::Randomness(astf) => ASTFlattenWeak::Randomness(astf.downgrade()),
            Self::Key(astf) => ASTFlattenWeak::Key(astf.downgrade()),
            Self::Proof(astf) => ASTFlattenWeak::Proof(astf.downgrade()),
            Self::ArrayBase(astf) => ASTFlattenWeak::ArrayBase(astf.downgrade()),
            Self::Array(astf) => ASTFlattenWeak::Array(astf.downgrade()),
            Self::IdentifierDeclaration(astf) => {
                ASTFlattenWeak::IdentifierDeclaration(astf.downgrade())
            }
            Self::VariableDeclaration(astf) => {
                ASTFlattenWeak::VariableDeclaration(astf.downgrade())
            }
            Self::Parameter(astf) => ASTFlattenWeak::Parameter(astf.downgrade()),
            Self::StateVariableDeclaration(astf) => {
                ASTFlattenWeak::StateVariableDeclaration(astf.downgrade())
            }
            Self::NamespaceDefinition(astf) => {
                ASTFlattenWeak::NamespaceDefinition(astf.downgrade())
            }
            Self::ConstructorOrFunctionDefinition(astf) => {
                ASTFlattenWeak::ConstructorOrFunctionDefinition(astf.downgrade())
            }
            Self::EnumDefinition(astf) => ASTFlattenWeak::EnumDefinition(astf.downgrade()),
            Self::StructDefinition(astf) => ASTFlattenWeak::StructDefinition(astf.downgrade()),
            Self::ContractDefinition(astf) => ASTFlattenWeak::ContractDefinition(astf.downgrade()),
            Self::DummyAnnotation(astf) => ASTFlattenWeak::DummyAnnotation(astf.downgrade()),
            Self::CircuitStatement(astf) => ASTFlattenWeak::CircuitStatement(astf.downgrade()),
            Self::CircComment(astf) => ASTFlattenWeak::CircComment(astf.downgrade()),
            Self::CircIndentBlock(astf) => ASTFlattenWeak::CircIndentBlock(astf.downgrade()),
            Self::CircCall(astf) => ASTFlattenWeak::CircCall(astf.downgrade()),
            Self::CircVarDecl(astf) => ASTFlattenWeak::CircVarDecl(astf.downgrade()),
            Self::CircGuardModification(astf) => {
                ASTFlattenWeak::CircGuardModification(astf.downgrade())
            }
            Self::CircEncConstraint(astf) => ASTFlattenWeak::CircEncConstraint(astf.downgrade()),
            Self::CircSymmEncConstraint(astf) => {
                ASTFlattenWeak::CircSymmEncConstraint(astf.downgrade())
            }
            Self::CircEqConstraint(astf) => ASTFlattenWeak::CircEqConstraint(astf.downgrade()),
        }
    }
}

impl ASTFlattenWeak {
    pub fn upgrade(self) -> Option<ASTFlatten> {
        match self {
            Self::AST(astf) => astf.upgrade().map(|astf| ASTFlatten::AST(astf)),
            Self::Expression(astf) => astf.upgrade().map(|astf| ASTFlatten::Expression(astf)),
            Self::Identifier(astf) => astf.upgrade().map(|astf| ASTFlatten::Identifier(astf)),
            Self::IdentifierBase(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::IdentifierBase(astf))
            }
            Self::Comment(astf) => astf.upgrade().map(|astf| ASTFlatten::Comment(astf)),
            Self::CommentBase(astf) => astf.upgrade().map(|astf| ASTFlatten::CommentBase(astf)),
            Self::AnnotatedTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::AnnotatedTypeName(astf)),
            Self::EnumValue(astf) => astf.upgrade().map(|astf| ASTFlatten::EnumValue(astf)),
            Self::SourceUnit(astf) => astf.upgrade().map(|astf| ASTFlatten::SourceUnit(astf)),
            Self::BlankLine(astf) => astf.upgrade().map(|astf| ASTFlatten::BlankLine(astf)),
            Self::BuiltinFunction(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::BuiltinFunction(astf))
            }
            Self::FunctionCallExprBase(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::FunctionCallExprBase(astf)),
            Self::FunctionCallExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::FunctionCallExpr(astf)),
            Self::NewExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::NewExpr(astf)),
            Self::PrimitiveCastExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::PrimitiveCastExpr(astf)),
            Self::MeExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::MeExpr(astf)),
            Self::AllExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::AllExpr(astf)),
            Self::ReclassifyExpr(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::ReclassifyExpr(astf))
            }
            Self::LiteralExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::LiteralExpr(astf)),
            Self::BooleanLiteralExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::BooleanLiteralExpr(astf)),
            Self::NumberLiteralExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::NumberLiteralExpr(astf)),
            Self::StringLiteralExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::StringLiteralExpr(astf)),
            Self::ArrayLiteralExprBase(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ArrayLiteralExprBase(astf)),
            Self::ArrayLiteralExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ArrayLiteralExpr(astf)),
            Self::KeyLiteralExpr(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::KeyLiteralExpr(astf))
            }
            Self::TupleOrLocationExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::TupleOrLocationExpr(astf)),
            Self::TupleExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::TupleExpr(astf)),
            Self::IdentifierExpr(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::IdentifierExpr(astf))
            }
            Self::MemberAccessExpr(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::MemberAccessExpr(astf)),
            Self::LocationExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::LocationExpr(astf)),
            Self::IndexExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::IndexExpr(astf)),
            Self::SliceExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::SliceExpr(astf)),
            Self::ReclassifyExprBase(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ReclassifyExprBase(astf)),
            Self::RehomExpr(astf) => astf.upgrade().map(|astf| ASTFlatten::RehomExpr(astf)),
            Self::EncryptionExpression(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::EncryptionExpression(astf)),
            Self::HybridArgumentIdf(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::HybridArgumentIdf(astf)),
            Self::Statement(astf) => astf.upgrade().map(|astf| ASTFlatten::Statement(astf)),
            Self::IfStatement(astf) => astf.upgrade().map(|astf| ASTFlatten::IfStatement(astf)),
            Self::WhileStatement(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::WhileStatement(astf))
            }
            Self::DoWhileStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::DoWhileStatement(astf)),
            Self::ForStatement(astf) => astf.upgrade().map(|astf| ASTFlatten::ForStatement(astf)),
            Self::BreakStatement(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::BreakStatement(astf))
            }
            Self::ContinueStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ContinueStatement(astf)),
            Self::ReturnStatement(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::ReturnStatement(astf))
            }
            Self::StatementListBase(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::StatementListBase(astf)),
            Self::StatementList(astf) => astf.upgrade().map(|astf| ASTFlatten::StatementList(astf)),
            Self::CircuitDirectiveStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircuitDirectiveStatement(astf)),
            Self::CircuitComputationStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircuitComputationStatement(astf)),
            Self::EnterPrivateKeyStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::EnterPrivateKeyStatement(astf)),
            Self::ExpressionStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ExpressionStatement(astf)),
            Self::RequireStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::RequireStatement(astf)),
            Self::AssignmentStatementBase(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::AssignmentStatementBase(astf)),
            Self::AssignmentStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::AssignmentStatement(astf)),
            Self::VariableDeclarationStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::VariableDeclarationStatement(astf)),
            Self::CircuitInputStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircuitInputStatement(astf)),
            Self::SimpleStatement(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::SimpleStatement(astf))
            }
            Self::Block(astf) => astf.upgrade().map(|astf| ASTFlatten::Block(astf)),
            Self::IndentBlock(astf) => astf.upgrade().map(|astf| ASTFlatten::IndentBlock(astf)),
            Self::Mapping(astf) => astf.upgrade().map(|astf| ASTFlatten::Mapping(astf)),
            Self::TupleType(astf) => astf.upgrade().map(|astf| ASTFlatten::TupleType(astf)),
            Self::TypeName(astf) => astf.upgrade().map(|astf| ASTFlatten::TypeName(astf)),
            Self::ElementaryTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ElementaryTypeName(astf)),
            Self::FunctionTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::FunctionTypeName(astf)),
            Self::BoolTypeName(astf) => astf.upgrade().map(|astf| ASTFlatten::BoolTypeName(astf)),
            Self::BooleanLiteralType(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::BooleanLiteralType(astf)),
            Self::NumberLiteralType(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::NumberLiteralType(astf)),
            Self::IntTypeName(astf) => astf.upgrade().map(|astf| ASTFlatten::IntTypeName(astf)),
            Self::UintTypeName(astf) => astf.upgrade().map(|astf| ASTFlatten::UintTypeName(astf)),
            Self::NumberTypeNameBase(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::NumberTypeNameBase(astf)),
            Self::NumberTypeName(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::NumberTypeName(astf))
            }
            Self::EnumTypeName(astf) => astf.upgrade().map(|astf| ASTFlatten::EnumTypeName(astf)),
            Self::EnumValueTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::EnumValueTypeName(astf)),
            Self::StructTypeName(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::StructTypeName(astf))
            }
            Self::ContractTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ContractTypeName(astf)),
            Self::AddressTypeName(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::AddressTypeName(astf))
            }
            Self::AddressPayableTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::AddressPayableTypeName(astf)),
            Self::UserDefinedTypeName(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::UserDefinedTypeName(astf)),
            Self::CipherText(astf) => astf.upgrade().map(|astf| ASTFlatten::CipherText(astf)),
            Self::Randomness(astf) => astf.upgrade().map(|astf| ASTFlatten::Randomness(astf)),
            Self::Key(astf) => astf.upgrade().map(|astf| ASTFlatten::Key(astf)),
            Self::Proof(astf) => astf.upgrade().map(|astf| ASTFlatten::Proof(astf)),
            Self::ArrayBase(astf) => astf.upgrade().map(|astf| ASTFlatten::ArrayBase(astf)),
            Self::Array(astf) => astf.upgrade().map(|astf| ASTFlatten::Array(astf)),
            Self::IdentifierDeclaration(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::IdentifierDeclaration(astf)),
            Self::VariableDeclaration(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::VariableDeclaration(astf)),
            Self::Parameter(astf) => astf.upgrade().map(|astf| ASTFlatten::Parameter(astf)),
            Self::StateVariableDeclaration(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::StateVariableDeclaration(astf)),
            Self::NamespaceDefinition(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::NamespaceDefinition(astf)),
            Self::ConstructorOrFunctionDefinition(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ConstructorOrFunctionDefinition(astf)),
            Self::EnumDefinition(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::EnumDefinition(astf))
            }
            Self::StructDefinition(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::StructDefinition(astf)),
            Self::ContractDefinition(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::ContractDefinition(astf)),
            Self::DummyAnnotation(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::DummyAnnotation(astf))
            }
            Self::CircuitStatement(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircuitStatement(astf)),
            Self::CircComment(astf) => astf.upgrade().map(|astf| ASTFlatten::CircComment(astf)),
            Self::CircIndentBlock(astf) => {
                astf.upgrade().map(|astf| ASTFlatten::CircIndentBlock(astf))
            }
            Self::CircCall(astf) => astf.upgrade().map(|astf| ASTFlatten::CircCall(astf)),
            Self::CircVarDecl(astf) => astf.upgrade().map(|astf| ASTFlatten::CircVarDecl(astf)),
            Self::CircGuardModification(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircGuardModification(astf)),
            Self::CircEncConstraint(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircEncConstraint(astf)),
            Self::CircSymmEncConstraint(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircSymmEncConstraint(astf)),
            Self::CircEqConstraint(astf) => astf
                .upgrade()
                .map(|astf| ASTFlatten::CircEqConstraint(astf)),
        }
    }
}

#[enum_dispatch]
pub trait ASTChildren {
    fn children(&self) -> Vec<ASTFlatten> {
        let mut cb = ChildListBuilder::new();
        self.process_children(&mut cb);
        cb.children.drain(..).collect()
    }

    fn process_children(&self, cb: &mut ChildListBuilder);
}

pub trait IntoStatement: Clone {
    fn into_statement(self) -> Statement;
    fn to_statement(&self) -> Statement {
        self.clone().into_statement()
    }
}
pub trait IntoExpression: Clone {
    fn into_expr(self) -> Expression;
    fn to_expr(&self) -> Expression {
        self.clone().into_expr()
    }
}
impl<T: IntoAST + Clone> IntoExpression for T {
    fn into_expr(self) -> Expression {
        self.into_ast().try_as_expression().unwrap()
    }
}
impl<T: IntoAST + Clone> IntoStatement for T {
    fn into_statement(self) -> Statement {
        self.into_ast().try_as_statement().unwrap()
    }
}
#[enum_dispatch]
pub trait IntoAST: Clone {
    fn to_ast(&self) -> AST {
        self.clone().into_ast()
    }
    fn into_ast(self) -> AST;
}

#[enum_dispatch]
pub trait ASTInstanceOf {
    fn get_ast_type(&self) -> ASTType;
}
trait ASTProperty {
    fn get_idf(&self) -> Option<Identifier>;
    fn get_namespace(&self) -> Option<Vec<Identifier>>;
    fn qualified_name(&self) -> Vec<Identifier> {
        if let Some(idf) = self.get_idf() {
            if let Some(namespace) = self.get_namespace() {
                if !namespace.is_empty() && namespace.last().unwrap() == &idf {
                    namespace.clone()
                } else {
                    namespace.into_iter().chain([idf]).collect()
                }
            } else {
                vec![idf]
            }
        } else {
            vec![]
        }
    }
}

// #[enum_dispatch(IntoAST,ASTInstanceOf,ASTBaseRef)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AST {
    Identifier(Identifier),
    Comment(Comment),
    Expression(Expression),
    Statement(Statement),
    TypeName(TypeName),
    AnnotatedTypeName(AnnotatedTypeName),
    IdentifierDeclaration(IdentifierDeclaration),
    NamespaceDefinition(NamespaceDefinition),
    EnumValue(EnumValue),
    SourceUnit(SourceUnit),
    Pragma(String),
    VersionPragma(String),
    Modifier(String),
    Homomorphism(String),
}
impl IntoAST for AST {
    fn into_ast(self) -> AST {
        match self {
            AST::Identifier(ast) => ast.into_ast(),
            AST::Comment(ast) => ast.into_ast(),
            AST::Expression(ast) => ast.into_ast(),
            AST::Statement(ast) => ast.into_ast(),
            AST::TypeName(ast) => ast.into_ast(),
            AST::AnnotatedTypeName(ast) => ast.into_ast(),
            AST::IdentifierDeclaration(ast) => ast.into_ast(),
            AST::NamespaceDefinition(ast) => ast.into_ast(),
            AST::EnumValue(ast) => ast.into_ast(),
            AST::SourceUnit(ast) => ast.into_ast(),
            _ => self,
        }
    }
}

impl ASTInstanceOf for AST {
    fn get_ast_type(&self) -> ASTType {
        match self {
            AST::Identifier(ast) => ast.get_ast_type(),
            AST::Comment(ast) => ast.get_ast_type(),
            AST::Expression(ast) => ast.get_ast_type(),
            AST::Statement(ast) => ast.get_ast_type(),
            AST::TypeName(ast) => ast.get_ast_type(),
            AST::AnnotatedTypeName(ast) => ast.get_ast_type(),
            AST::IdentifierDeclaration(ast) => ast.get_ast_type(),
            AST::NamespaceDefinition(ast) => ast.get_ast_type(),
            AST::EnumValue(ast) => ast.get_ast_type(),
            AST::SourceUnit(ast) => ast.get_ast_type(),
            AST::Pragma(_) => ASTType::Pragma,
            AST::VersionPragma(_) => ASTType::VersionPragma,
            AST::Modifier(_) => ASTType::Modifier,
            AST::Homomorphism(_) => ASTType::Homomorphism,
        }
    }
}
impl ASTChildren for AST {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        match self {
            AST::Expression(ast) => ast.process_children(cb),
            AST::Statement(ast) => ast.process_children(cb),
            AST::SourceUnit(ast) => ast.process_children(cb),
            // AST::Identifier(ast) => ast.process_children(cb),
            // AST::Comment(ast) => ast.process_children(cb),
            AST::TypeName(ast) => ast.process_children(cb),
            AST::AnnotatedTypeName(ast) => ast.process_children(cb),
            AST::IdentifierDeclaration(ast) => ast.process_children(cb),
            AST::NamespaceDefinition(ast) => ast.process_children(cb),
            AST::EnumValue(ast) => ast.process_children(cb),
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! impl_base_ref {
    ($fn_name: ident,$self: ident) => {
        match $self {
            AST::Identifier(ast) => Some(ast.$fn_name()),
            AST::Comment(ast) => Some(ast.$fn_name()),
            AST::Expression(ast) => Some(ast.$fn_name()),
            AST::Statement(ast) => ast.$fn_name(),
            AST::TypeName(ast) => ast.$fn_name(),
            AST::AnnotatedTypeName(ast) => Some(ast.$fn_name()),
            AST::IdentifierDeclaration(ast) => Some(ast.$fn_name()),
            AST::NamespaceDefinition(ast) => Some(ast.$fn_name()),
            AST::EnumValue(ast) => Some(ast.$fn_name()),
            AST::SourceUnit(ast) => Some(ast.$fn_name()),
            _ => None,
        }
    };
}

impl AST {
    pub fn ast_base_ref(&self) -> Option<RcCell<ASTBase>> {
        impl_base_ref!(ast_base_ref, self)
    }
    // pub fn ast_base_mut_ref(&mut self) -> Option<&mut RcCell<ASTBase>> {
    //     impl_base_ref!(ast_base_ref, self)
    // }

    pub fn text(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&RcCell::new(self.clone()).into())
    }
    // pub fn code(&self) -> String {
    //     let v = CodeVisitor::new(true);
    //     v.visit(&RcCell::new(self.clone()).into())
    // }
    pub fn is_parent_of(&self, child: &ASTFlatten) -> bool {
        let mut e = child.clone();
        let selfs = RcCell::new(self.clone()).into();
        while e != selfs && e.ast_base_ref().unwrap().borrow().parent.is_some() {
            let e1 = e
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent
                .as_ref()
                .map(|p| p.clone().upgrade().unwrap())
                .unwrap();
            e = e1;
        }
        e == selfs
    }

    pub fn bases(_child: ASTType) -> Option<ASTType> {
        match _child {
            ASTType::IdentifierBase
            | ASTType::CommentBase
            | ASTType::ExpressionBase
            | ASTType::StatementBase
            | ASTType::TypeNameBase
            | ASTType::AnnotatedTypeName
            | ASTType::IdentifierDeclarationBase
            | ASTType::NamespaceDefinitionBase
            | ASTType::EnumValue
            | ASTType::SourceUnit
            | ASTType::Pragma
            | ASTType::VersionPragma
            | ASTType::Modifier
            | ASTType::Homomorphism => Some(ASTType::ASTBase),
            ASTType::BlankLine => Some(ASTType::CommentBase),
            ASTType::BuiltinFunction
            | ASTType::FunctionCallExprBase
            | ASTType::PrimitiveCastExpr
            | ASTType::LiteralExprBase
            | ASTType::TupleOrLocationExprBase
            | ASTType::MeExpr
            | ASTType::AllExpr
            | ASTType::ReclassifyExpr
            | ASTType::DummyAnnotation => Some(ASTType::ExpressionBase),
            ASTType::NewExpr => Some(ASTType::FunctionCallExprBase),
            ASTType::BooleanLiteralExpr
            | ASTType::NumberLiteralExpr
            | ASTType::StringLiteralExpr
            | ASTType::ArrayLiteralExprBase => Some(ASTType::LiteralExprBase),
            ASTType::KeyLiteralExpr => Some(ASTType::ArrayLiteralExprBase),
            ASTType::TupleExpr | ASTType::LocationExprBase => {
                Some(ASTType::TupleOrLocationExprBase)
            }
            ASTType::IdentifierExpr
            | ASTType::MemberAccessExpr
            | ASTType::IndexExpr
            | ASTType::SliceExpr => Some(ASTType::LocationExprBase),
            ASTType::ReclassifyExprBase | ASTType::RehomExpr | ASTType::EncryptionExpression => {
                Some(ASTType::ExpressionBase)
            }
            ASTType::HybridArgumentIdf => Some(ASTType::IdentifierBase),
            ASTType::CircuitDirectiveStatementBase
            | ASTType::IfStatement
            | ASTType::WhileStatement
            | ASTType::DoWhileStatement
            | ASTType::ForStatement
            | ASTType::BreakStatement
            | ASTType::ContinueStatement
            | ASTType::ReturnStatement
            | ASTType::SimpleStatementBase
            | ASTType::StatementListBase => Some(ASTType::StatementBase),
            ASTType::CircuitComputationStatement | ASTType::EnterPrivateKeyStatement => {
                Some(ASTType::CircuitDirectiveStatementBase)
            }
            ASTType::ExpressionStatement
            | ASTType::RequireStatement
            | ASTType::AssignmentStatementBase
            | ASTType::VariableDeclarationStatement => Some(ASTType::SimpleStatementBase),
            ASTType::CircuitInputStatement => Some(ASTType::AssignmentStatementBase),
            ASTType::Block | ASTType::IndentBlock => Some(ASTType::StatementListBase),
            ASTType::ElementaryTypeNameBase
            | ASTType::UserDefinedTypeNameBase
            | ASTType::Mapping
            | ASTType::ArrayBase
            | ASTType::TupleType
            | ASTType::FunctionTypeName
            | ASTType::Literal => Some(ASTType::TypeNameBase),
            ASTType::NumberTypeNameBase | ASTType::BoolTypeName | ASTType::BooleanLiteralType => {
                Some(ASTType::ElementaryTypeNameBase)
            }
            ASTType::NumberLiteralType | ASTType::IntTypeName | ASTType::UintTypeName => {
                Some(ASTType::NumberTypeNameBase)
            }
            ASTType::EnumTypeName
            | ASTType::EnumValueTypeName
            | ASTType::StructTypeName
            | ASTType::ContractTypeName
            | ASTType::AddressTypeName
            | ASTType::AddressPayableTypeName => Some(ASTType::UserDefinedTypeNameBase),
            ASTType::CipherText | ASTType::Randomness | ASTType::Key | ASTType::Proof => {
                Some(ASTType::ArrayBase)
            }
            ASTType::VariableDeclaration
            | ASTType::Parameter
            | ASTType::StateVariableDeclaration => Some(ASTType::IdentifierDeclarationBase),
            ASTType::ConstructorOrFunctionDefinition
            | ASTType::EnumDefinition
            | ASTType::StructDefinition
            | ASTType::ContractDefinition => Some(ASTType::NamespaceDefinitionBase),

            ASTType::CircComment
            | ASTType::CircIndentBlock
            | ASTType::CircCall
            | ASTType::CircVarDecl
            | ASTType::CircGuardModification
            | ASTType::CircEncConstraint
            | ASTType::CircSymmEncConstraint
            | ASTType::CircEqConstraint => Some(ASTType::NamespaceDefinitionBase),
            _ => None,
        }
    }
}

use std::fmt;

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}
impl Immutable for AST {
    fn is_immutable(&self) -> bool {
        true
    }
}
#[impl_trait(
    StatementBase,
    TypeNameBase,
    AnnotatedTypeName,
    IdentifierDeclarationBase,
    NamespaceDefinitionBase,
    EnumValue,
    SourceUnit,
    IdentifierBase,
    CommentBase,
    ExpressionBase
)]
#[enum_dispatch]
pub trait ASTBaseRef {
    fn ast_base_ref(&self) -> RcCell<ASTBase>;
}
pub trait ASTBaseProperty {
    fn parent(&self) -> Option<ASTFlattenWeak>;
    fn namespace(&self) -> Option<Vec<WeakCell<Identifier>>>;
    fn names(&self) -> BTreeMap<String, WeakCell<Identifier>>;
    fn line(&self) -> i32;
    fn column(&self) -> i32;
    fn modified_values(&self) -> BTreeSet<InstanceTarget>;
    fn read_values(&self) -> BTreeSet<InstanceTarget>;
}
impl<T: ASTBaseRef> ASTBaseProperty for T {
    fn parent(&self) -> Option<ASTFlattenWeak> {
        self.ast_base_ref().borrow().parent.clone()
    }
    fn namespace(&self) -> Option<Vec<WeakCell<Identifier>>> {
        self.ast_base_ref().borrow().namespace.clone()
    }
    fn names(&self) -> BTreeMap<String, WeakCell<Identifier>> {
        self.ast_base_ref().borrow().names.clone()
    }
    fn line(&self) -> i32 {
        self.ast_base_ref().borrow().line
    }
    fn column(&self) -> i32 {
        self.ast_base_ref().borrow().column
    }
    fn modified_values(&self) -> BTreeSet<InstanceTarget> {
        self.ast_base_ref().borrow().modified_values.clone()
    }
    fn read_values(&self) -> BTreeSet<InstanceTarget> {
        self.ast_base_ref().borrow().read_values.clone()
    }
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ASTBase {
    pub parent: Option<ASTFlattenWeak>,
    pub namespace: Option<Vec<WeakCell<Identifier>>>,
    pub names: BTreeMap<String, WeakCell<Identifier>>,
    pub line: i32,
    pub column: i32,
    pub modified_values: BTreeSet<InstanceTarget>,
    pub read_values: BTreeSet<InstanceTarget>,
}
impl ASTBase {
    pub fn new() -> Self {
        Self {
            parent: None,
            namespace: None,
            names: BTreeMap::new(),
            line: -1,
            column: -1,
            modified_values: BTreeSet::new(),
            read_values: BTreeSet::new(),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &Self) -> bool {
        &expected == &self
    }
}
#[enum_dispatch]
pub trait IdentifierBaseRef: ASTBaseRef {
    fn identifier_base_ref(&self) -> &IdentifierBase;
}
pub trait IdentifierBaseProperty {
    fn name(&self) -> String;
}
impl<T: IdentifierBaseRef> IdentifierBaseProperty for T {
    fn name(&self) -> String {
        self.identifier_base_ref().name.clone()
    }
}

#[derive(
    ImplBaseTrait, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct IdentifierBase {
    pub ast_base: RcCell<ASTBase>,
    pub name: String,
}
impl IntoAST for IdentifierBase {
    fn into_ast(self) -> AST {
        AST::Identifier(Identifier::Identifier(self))
    }
}

impl IdentifierBase {
    pub fn new(name: String) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            name,
        }
    }
    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
    pub fn decl_var(&self, t: &ASTFlatten, expr: Option<ASTFlatten>) -> Statement {
        let t = if is_instance(t, ASTType::TypeNameBase) {
            Some(RcCell::new(AnnotatedTypeName::new(
                t.clone().try_as_type_name(),
                None,
                Homomorphism::non_homomorphic(),
            )))
        } else {
            t.clone().try_as_annotated_type_name()
        };
        assert!(t.is_some());
        let storage_loc = if t
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .borrow()
            .is_primitive_type()
        {
            String::new()
        } else {
            String::from("memory")
        };
        VariableDeclarationStatement::new(
            RcCell::new(VariableDeclaration::new(
                vec![],
                t,
                Some(RcCell::new(Identifier::Identifier(self.clone()))),
                Some(storage_loc),
            )),
            expr,
        )
        .to_statement()
    }
}
impl Immutable for IdentifierBase {
    fn is_immutable(&self) -> bool {
        let p = self.parent().clone().unwrap().upgrade();
        p.is_some()
            && is_instance(p.as_ref().unwrap(), ASTType::StateVariableDeclaration)
            && (p
                .as_ref()
                .unwrap()
                .try_as_state_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .is_final()
                || p.as_ref()
                    .unwrap()
                    .try_as_state_variable_declaration_ref()
                    .unwrap()
                    .borrow()
                    .identifier_declaration_base
                    .is_constant())
    }
}
impl fmt::Display for IdentifierBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[enum_dispatch(IntoAST, ASTFlattenImpl, ASTInstanceOf, CommentBaseRef, ASTBaseRef)]
#[derive(EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Comment {
    Comment(CommentBase),
    BlankLine(BlankLine),
}
impl Comment {
    pub fn code(&self) -> String {
        String::new()
    }
    pub fn text(&self) -> String {
        String::new()
    }
}
#[enum_dispatch]
pub trait CommentBaseRef: ASTBaseRef {
    fn comment_base_ref(&self) -> &CommentBase;
}
pub trait CommentBaseProperty {
    fn text(&self) -> &String;
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
impl IntoAST for CommentBase {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::Comment(self))
    }
}

impl CommentBase {
    pub fn new(text: String) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
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
            RcCell::new(IndentBlock::new(
                block
                    .into_iter()
                    .filter(|b| is_instance(b, ASTType::StatementBase))
                    .collect(),
            ))
            .into(),
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
impl IntoAST for BlankLine {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::BlankLine(self))
    }
}

impl BlankLine {
    pub fn new() -> Self {
        Self {
            comment_base: CommentBase::new(String::new()),
        }
    }
}
#[enum_dispatch(
    ExpressionASType,
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    pub fn explicitly_converted(&self, expected: &RcCell<TypeName>) -> ASTFlatten {
        let mut ret;
        let bool_type = RcCell::new(TypeName::bool_type());
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
            ));
        } else if expected.borrow().is_numeric() && self.instanceof_data_type(&bool_type) {
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
        ret.expression_base.annotated_type = Some(RcCell::new(AnnotatedTypeName::new(
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
        if let Some(ie) = self
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_identifier_expr_ref()
        {
            if let Some(target) = ie.target() {
                let target = target.clone().upgrade().unwrap();
                if let Some(mapping) = target
                    .try_as_type_name_ref()
                    .unwrap()
                    .borrow()
                    .try_as_mapping_ref()
                {
                    return mapping
                        .instantiated_key
                        .as_ref()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .privacy_annotation_label();
                }
                if let Some(id) = target.try_as_identifier_declaration_ref() {
                    return id.borrow().idf().upgrade().map(|f| f.into());
                }
                if let Some(id) = target.try_as_namespace_definition_ref() {
                    return id.borrow().idf().upgrade().map(|p| p.into());
                }
            }
        }

        if self.is_all_expr() || self.is_me_expr() {
            Some(RcCell::new(self.clone()).into())
        } else {
            None
        }
    }
    pub fn instanceof_data_type(&self, expected: &RcCell<TypeName>) -> bool {
        self.annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .borrow()
            .implicitly_convertible_to(expected)
    }
    pub fn unop(&self, op: String) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            RcCell::new(Expression::BuiltinFunction(BuiltinFunction::new(&op))).into(),
            vec![RcCell::new(self.clone()).into()],
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
        ))
    }

    pub fn instance_of(&self, expected: &RcCell<AnnotatedTypeName>) -> Option<String> {
        // """
        // :param expected:
        // :return: true, false, or "make-private"
        // """
        // assert! (isinstance(expected, AnnotatedTypeName))

        let actual = self.annotated_type();

        if !self.instanceof_data_type(&expected.borrow().type_name.as_ref().unwrap()) {
            return Some(String::from("false"));
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
                    if let TypeName::TupleType(_) = *self
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                    {
                        true
                    } else {
                        false
                    } && if let Expression::TupleOrLocationExpr(tole) = &self {
                        if let TupleOrLocationExpr::TupleExpr(_) = tole {
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                );
                Some(
                    (combined_label
                        == self
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .type_name
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .try_as_tuple_type_ref()
                            .unwrap()
                            .types
                            .iter()
                            .map(|t| {
                                CombinedPrivacyUnion::AST(
                                    t.borrow()
                                        .privacy_annotation
                                        .as_ref()
                                        .map(|pa| pa.clone().into()),
                                )
                            })
                            .collect::<Vec<_>>())
                    .to_string(),
                )
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
                Some(String::from("true"))
            } else {
                Some(String::from("make-private"))
            }
        } else {
            Some(String::from("false"))
        }
    }

    pub fn analysis(&self) -> Option<PartitionState<AST>> {
        self.statement()
            .as_ref()
            .map(|statement| {
                statement
                    .clone()
                    .upgrade()
                    .unwrap()
                    .try_as_statement_ref()
                    .unwrap()
                    .borrow()
                    .statement_base_ref()
                    .unwrap()
                    .before_analysis()
                    .clone()
            })
            .flatten()
    }
}
#[enum_dispatch]
pub trait ExpressionBaseRef: ASTBaseRef {
    fn expression_base_ref(&self) -> &ExpressionBase;
}
pub trait ExpressionBaseProperty {
    fn annotated_type(&self) -> &Option<RcCell<AnnotatedTypeName>>;
    fn statement(&self) -> &Option<ASTFlattenWeak>;
    fn evaluate_privately(&self) -> bool;
}
impl<T: ExpressionBaseRef> ExpressionBaseProperty for T {
    fn annotated_type(&self) -> &Option<RcCell<AnnotatedTypeName>> {
        &self.expression_base_ref().annotated_type
    }
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
    pub annotated_type: Option<RcCell<AnnotatedTypeName>>,
    pub statement: Option<ASTFlattenWeak>,
    pub evaluate_privately: bool,
}
impl ExpressionBase {
    pub fn new() -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            annotated_type: None,
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
pub fn builtin_op_fct(op: &str, args: Vec<i32>) -> LiteralUnion {
    match op {
        "+" => LiteralUnion::Number(args[0] + args[1]),
        "-" => LiteralUnion::Number(args[0] - args[1]),
        "**" => LiteralUnion::Number(args[0].pow(args[1] as u32)),
        "*" => LiteralUnion::Number(args[0] * args[1]),
        "/" => LiteralUnion::Number(args[0] / args[1]),
        "%" => LiteralUnion::Number(args[0] % args[1]),
        "sign+" => LiteralUnion::Number(args[0]),
        "sign-" => LiteralUnion::Number(-args[0]),
        "<<" => LiteralUnion::Number(args[0] << args[1]),
        ">>" => LiteralUnion::Number(args[0] >> args[1]),
        "|" => LiteralUnion::Number(args[0] | args[1]),
        "&" => LiteralUnion::Number(args[0] & args[1]),
        "^" => LiteralUnion::Number(args[0] ^ args[1]),
        "~" => LiteralUnion::Number(!args[0]),
        "<" => LiteralUnion::Bool(args[0] < args[1]),
        ">" => LiteralUnion::Bool(args[0] > args[1]),
        "<=" => LiteralUnion::Bool(args[0] <= args[1]),
        ">=" => LiteralUnion::Bool(args[0] >= args[1]),
        "==" => LiteralUnion::Bool(args[0] == args[1]),
        "!=" => LiteralUnion::Bool(args[0] != args[1]),
        "&&" => LiteralUnion::Bool(args[0] != 0 && args[1] != 0),
        "||" => LiteralUnion::Bool(args[0] != 0 || args[1] != 0),
        "!" => LiteralUnion::Bool(!(args[0] != 0)),
        "ite" => LiteralUnion::Number(if args[0] != 0 { args[1] } else { args[2] }),
        "parenthesis" => LiteralUnion::Number(args[0]),
        _ => LiteralUnion::Bool(false),
    }
}

// assert builtin_op_fct.keys() == BUILTIN_FUNCTIONS.keys()
const BINARY_OP: &'static str = "{{}} {op} {{}}";
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
            ("ite", "if {}  {{{}}} else {{{}}}"),
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
            ("ite", "if {}  {{{}}} else {{{}}}"),
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
            let op_arity = BUILTIN_FUNCTIONS[&__hom.op].matches("{}").count() as usize;
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
    pub rerand_using: Option<RcCell<IdentifierExpr>>,
}

impl IntoAST for BuiltinFunction {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::BuiltinFunction(self))
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
            expression_base: ExpressionBase::new(),
            op,
            is_private: false,
            homomorphism: Homomorphism::non_homomorphic(),
            rerand_using: None,
        }
    }

    pub fn format_string(&self, args: &Vec<String>) -> String {
        let op = self.op.as_str();

        match op {
            "sign+" => format!("+{}", args[0]),
            "sign-" => format!("-{}", args[0]),
            "!" => format!("!{}", args[0]),
            "~" => format!("~{}", args[0]),
            "parenthesis" => format!("({})", args[0]),
            "ite" => {
                let (cond, then, else_then) = (args[0].clone(), args[1].clone(), args[2].clone());
                format!("if {cond} {{{then}}} else {{{else_then}}}")
            }
            _ => format!("{} {op} {}", args[0], args[1]),
        }
    }
    pub fn op_func(&self, args: Vec<i32>) -> LiteralUnion {
        builtin_op_fct(&self.op, args)
    }

    pub fn is_arithmetic(&self) -> bool {
        ARITHMETIC.contains_key(&self.op)
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
    pub fn input_types(&self) -> Vec<Option<RcCell<TypeName>>> {
        // :return: None if the type is generic
        let t = if self.is_arithmetic() || self.is_comp() || self.is_bitop() || self.is_shiftop() {
            Some(RcCell::new(TypeName::number_type()))
        } else if self.is_bop() {
            Some(RcCell::new(TypeName::bool_type()))
        } else
        // eq, parenthesis, ite
        {
            None
        };

        vec![t; self.arity() as usize]
    }
    pub fn output_type(&self) -> Option<TypeName> {
        // :return: None if the type is generic
        if self.is_arithmetic() || self.is_bitop() || self.is_shiftop() {
            Some(TypeName::number_type())
        } else if self.is_comp() || self.is_bop() || self.is_eq() {
            Some(TypeName::bool_type())
        } else
        // parenthesis, ite
        {
            None
        }
    }
    pub fn can_be_private(&self) -> bool
// :return: true if operation itself can be run inside a circuit \
        //          for equality and ite it must be checked separately whether the arguments are also supported inside circuits
    {
        &self.op != "**"
    }

    pub fn select_homomorphic_overload(
        &self,
        args: &Vec<ASTFlatten>,
        analysis: Option<PartitionState<AST>>,
    ) -> Option<HomomorphicBuiltinFunction> {
        // """
        // Finds a homomorphic builtin that performs the correct operation and which can be applied
        // on the arguments, if any exist.

        // :return: A HomomorphicBuiltinFunction that can be used to query the required input types and
        //          the resulting output type of the homomorphic operation, or None
        // """

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
        if inaccessible_arg_types.is_empty()
        // Else we would not have selected a homomorphic operation
        {
            // raise ValueError("Cannot select proper homomorphic function if all arguments are public or @me-private")
            assert!(false,"Cannot select proper homomorphic function if all arguments are public or @me-private");
        }
        let elem_type = arg_types
            .iter()
            .map(|a| a.as_ref().unwrap().borrow().type_name.clone().unwrap())
            .reduce(|l, r| l.borrow().combined_type(&r, true).unwrap())
            .unwrap();
        let base_type = AnnotatedTypeName::new(
            Some(elem_type),
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
impl HomomorphicBuiltinFunction {
    pub fn new(target_type: RcCell<AnnotatedTypeName>, public_args: Vec<bool>) -> Self {
        Self {
            target_type,
            public_args,
        }
    }
    pub fn input_types(&self) -> Vec<RcCell<AnnotatedTypeName>> {
        let public_type = AnnotatedTypeName::all(
            self.target_type
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .borrow()
                .clone(),
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
    ExpressionASType,
    ASTChildren,
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
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FunctionCallExpr {
    FunctionCallExpr(FunctionCallExprBase),
    NewExpr(NewExpr),
}

impl FunctionCallExpr {
    pub fn is_cast(&self) -> bool {
        // isinstance(self.func, LocationExpr) && isinstance(self.func.target, (ContractDefinition, EnumDefinition))
        is_instance(self.func(), ASTType::LocationExprBase)
            && is_instances(
                &self
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
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

impl IntoAST for FunctionCallExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(
            FunctionCallExpr::FunctionCallExpr(self),
        ))
    }
}

impl FunctionCallExprBase {
    pub fn new(func: ASTFlatten, args: Vec<ASTFlatten>, sec_start_offset: Option<i32>) -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            func,
            args,
            sec_start_offset,
            public_key: None,
        }
    }
}

impl ASTChildren for FunctionCallExprBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.func.clone().into());
        self.args.iter().for_each(|arg| {
            cb.add_child(arg.clone().into());
        });
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
    pub annotated_type: RcCell<AnnotatedTypeName>,
}
impl IntoAST for NewExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::NewExpr(
            self,
        )))
    }
}

impl NewExpr {
    pub fn new(annotated_type: AnnotatedTypeName, args: Vec<ASTFlatten>) -> Self {
        // assert not isinstance(annotated_type, ElementaryTypeName)
        Self {
            function_call_expr_base: FunctionCallExprBase::new(
                RcCell::new(IdentifierExpr::new(
                    IdentifierExprUnion::String(format!("new {}", annotated_type.code())),
                    None,
                ))
                .into(),
                args,
                None,
            ),
            annotated_type: RcCell::new(annotated_type),
        }
    }
}
impl ASTChildren for NewExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.annotated_type.clone().into());
        self.function_call_expr_base.args.iter().for_each(|arg| {
            cb.add_child(arg.clone().into());
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
    pub elem_type: RcCell<TypeName>,
    pub expr: ASTFlatten,
    pub is_implicit: bool,
}
impl IntoAST for PrimitiveCastExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::PrimitiveCastExpr(self))
    }
}

impl PrimitiveCastExpr {
    pub fn new(elem_type: RcCell<TypeName>, expr: ASTFlatten, is_implicit: bool) -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            elem_type,
            expr,
            is_implicit,
        }
    }
}
impl ASTChildren for PrimitiveCastExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.elem_type.clone().into());
        cb.add_child(self.expr.clone().into());
    }
}

#[enum_dispatch(
    ExpressionASType,
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    LiteralExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl LiteralExprBase {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(),
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
    pub annotated_type: Option<RcCell<AnnotatedTypeName>>,
}
impl IntoAST for BooleanLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::BooleanLiteralExpr(
            self,
        )))
    }
}

impl BooleanLiteralExpr {
    pub fn new(value: bool) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(),
            value,
            annotated_type: Some(RcCell::new(AnnotatedTypeName::new(
                BooleanLiteralType::new(value)
                    .into_ast()
                    .try_as_type_name()
                    .map(RcCell::new),
                None,
                Homomorphism::non_homomorphic(),
            ))),
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
    pub annotated_type: Option<RcCell<AnnotatedTypeName>>,
}

impl IntoAST for NumberLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
            self,
        )))
    }
}

impl NumberLiteralExpr {
    pub fn new(value: i32, was_hex: bool) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(),
            value,
            value_string: None,
            was_hex,
            annotated_type: Some(RcCell::new(AnnotatedTypeName::new(
                NumberLiteralType::new(NumberLiteralTypeUnion::I32(value))
                    .into_ast()
                    .try_as_type_name()
                    .map(RcCell::new),
                None,
                Homomorphism::non_homomorphic(),
            ))),
        }
    }
    pub fn new_string(value_string: String) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(),
            value: 0,
            value_string: Some(value_string.clone()),
            was_hex: true,
            annotated_type: Some(RcCell::new(AnnotatedTypeName::new(
                NumberLiteralType::new(NumberLiteralTypeUnion::String(value_string))
                    .into_ast()
                    .try_as_type_name()
                    .map(RcCell::new),
                None,
                Homomorphism::non_homomorphic(),
            ))),
        }
    }

    pub fn value(&self) -> i32 {
        0
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
impl IntoAST for StringLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::StringLiteralExpr(
            self,
        )))
    }
}

impl StringLiteralExpr {
    pub fn new(value: String) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(),
            value,
        }
    }
}
#[enum_dispatch(
    ExpressionASType,
    ASTChildren,
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
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl IntoAST for ArrayLiteralExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(
            ArrayLiteralExpr::ArrayLiteralExpr(self),
        )))
    }
}

impl ArrayLiteralExprBase {
    pub fn new(values: Vec<ASTFlatten>) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(),
            values,
        }
    }
}
impl ASTChildren for ArrayLiteralExprBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.values.iter().for_each(|value| {
            cb.add_child(value.clone().into());
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
impl IntoAST for KeyLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(
            ArrayLiteralExpr::KeyLiteralExpr(self),
        )))
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
    ExpressionASType,
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    TupleOrLocationExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
                == &*parent
                    .as_ref()
                    .unwrap()
                    .try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow()
                    .lhs()
                    .as_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow();
        }
        if is_instance(parent.as_ref().unwrap(), ASTType::IndexExpr) {
            if self
                == &TupleOrLocationExpr::LocationExpr(
                    parent
                        .as_ref()
                        .unwrap()
                        .try_as_index_expr_ref()
                        .unwrap()
                        .borrow()
                        .arr
                        .clone()
                        .unwrap()
                        .borrow()
                        .clone(),
                )
            {
                return parent
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .is_lvalue();
            }
        }
        if is_instance(parent.as_ref().unwrap(), ASTType::MemberAccessExpr) {
            if self
                == &TupleOrLocationExpr::LocationExpr(
                    parent
                        .as_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .borrow()
                        .expr
                        .clone()
                        .unwrap()
                        .borrow()
                        .clone(),
                )
            {
                return parent
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .is_lvalue();
            }
        }
        if is_instance(parent.as_ref().unwrap(), ASTType::TupleExpr) {
            return parent
                .as_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .is_lvalue();
        }

        // if isinstance(self.parent, AssignmentStatement)
        //     return self == self.parent.lhs
        // if isinstance(self.parent, IndexExpr) and self == self.parent.arr:
        //     return self.parent.is_lvalue()
        // if isinstance(self.parent, MemberAccessExpr) and self == self.parent.expr:
        //     return self.parent.is_lvalue()
        // if isinstance(self.parent, TupleExpr):
        //     return self.parent.is_lvalue()
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
impl TupleOrLocationExprBase {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(),
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

impl IntoAST for TupleExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::TupleExpr(self),
        ))
    }
}

impl TupleExpr {
    pub fn new(elements: Vec<ASTFlatten>) -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(),
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

#[enum_dispatch(
    ExpressionASType,
    ASTChildren,
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
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum LocationExpr {
    IdentifierExpr(IdentifierExpr),
    MemberAccessExpr(MemberAccessExpr),
    IndexExpr(IndexExpr),
    SliceExpr(SliceExpr),
}

impl LocationExpr {
    pub fn call(&self, member: IdentifierExprUnion, args: Vec<ASTFlatten>) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(match member {
            IdentifierExprUnion::Identifier(member) => FunctionCallExprBase::new(
                RcCell::new(MemberAccessExpr::new(
                    Some(RcCell::new(self.clone())),
                    member,
                ))
                .into(),
                args,
                None,
            ),
            IdentifierExprUnion::String(member) => FunctionCallExprBase::new(
                RcCell::new(MemberAccessExpr::new(
                    Some(RcCell::new(self.clone())),
                    RcCell::new(Identifier::Identifier(IdentifierBase::new(member))),
                ))
                .into(),
                args,
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
        match member {
            IdentifierExprUnion::Identifier(member) => {
                MemberAccessExpr::new(Some(RcCell::new(self.clone())), member)
            }
            IdentifierExprUnion::String(member) => MemberAccessExpr::new(
                Some(RcCell::new(self.clone())),
                RcCell::new(Identifier::Identifier(IdentifierBase::new(member))),
            ),
        }
    }

    pub fn index(&self, item: ExprUnion) -> ASTFlatten {
        let type_name = self
            .annotated_type()
            .as_ref()
            .map(|t| t.borrow().type_name.clone());
        let value_type = type_name
            .map(|type_name| match type_name.map(|t| t.borrow().clone()) {
                Some(TypeName::Array(a)) => Some(a.value_type().clone().into()),
                Some(TypeName::Mapping(a)) => Some(a.value_type.clone().into()),
                _ => None,
            })
            .flatten();
        assert!(value_type.is_some());
        let item = match item {
            ExprUnion::I32(item) => RcCell::new(NumberLiteralExpr::new(item, false)).into(),
            ExprUnion::Expression(item) => item,
        };

        IndexExpr::new(Some(self.clone()), item).as_type(&value_type.unwrap())
    }
    pub fn assign(&self, val: ASTFlatten) -> AssignmentStatement {
        AssignmentStatement::AssignmentStatement(AssignmentStatementBase::new(
            Some(RcCell::new(self.clone()).into()),
            Some(val),
            None,
        ))
    }
}
#[enum_dispatch]
pub trait LocationExprBaseRef: TupleOrLocationExprBaseRef {
    fn location_expr_base_ref(&self) -> &LocationExprBase;
}
pub trait LocationExprBaseProperty {
    fn target(&self) -> &Option<ASTFlattenWeak>;
}
impl<T: LocationExprBaseRef> LocationExprBaseProperty for T {
    fn target(&self) -> &Option<ASTFlattenWeak> {
        &self.location_expr_base_ref().target
    }
}
#[impl_traits(TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LocationExprBase {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub target: Option<ASTFlattenWeak>,
    pub target_rc: Option<ASTFlatten>,
}

impl LocationExprBase {
    pub fn new() -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(),
            target: None,
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
    pub idf: Option<RcCell<Identifier>>,
    pub annotated_type: Option<RcCell<AnnotatedTypeName>>,
}

impl IntoAST for IdentifierExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(self)),
        ))
    }
}

impl IdentifierExpr {
    pub fn new(
        idf: IdentifierExprUnion,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
    ) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            idf: Some(match idf {
                IdentifierExprUnion::Identifier(idf) => idf,
                IdentifierExprUnion::String(idf) => {
                    RcCell::new(Identifier::Identifier(IdentifierBase::new(idf)))
                }
            }),
            annotated_type,
        }
    }

    pub fn get_annotated_type(&self) -> Option<AnnotatedTypeName> {
        self.location_expr_base
            .target
            .clone()
            .unwrap()
            .upgrade()
            .map(|t| {
                t.try_as_identifier_declaration_ref()
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
        SliceExpr::new(
            Some(RcCell::new(LocationExpr::IdentifierExpr(self.clone()))),
            base,
            offset,
            size,
        )
    }
}
impl ASTChildren for IdentifierExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf {
            cb.add_child(idf.clone().into());
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
    pub expr: Option<RcCell<LocationExpr>>,
    pub member: RcCell<Identifier>,
}
impl IntoAST for MemberAccessExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::MemberAccessExpr(self)),
        ))
    }
}

impl MemberAccessExpr {
    pub fn new(expr: Option<RcCell<LocationExpr>>, member: RcCell<Identifier>) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            expr,
            member,
        }
    }
}
impl ASTChildren for MemberAccessExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone().into());
        }
        cb.add_child(self.member.clone().into());
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
    pub arr: Option<RcCell<LocationExpr>>,
    pub key: ASTFlatten,
}
impl IntoAST for IndexExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IndexExpr(self)),
        ))
    }
}

impl IndexExpr {
    pub fn new(arr: Option<LocationExpr>, key: ASTFlatten) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            arr: arr.map(RcCell::new),
            key,
        }
    }
}
impl ASTChildren for IndexExpr {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(arr) = &self.arr {
            cb.add_child(arr.clone().into());
        }
        cb.add_child(self.key.clone().into());
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
    pub arr: Option<RcCell<LocationExpr>>,
    pub base: Option<ASTFlatten>,
    pub start_offset: i32,
    pub size: i32,
}
impl IntoAST for SliceExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::SliceExpr(self)),
        ))
    }
}

impl SliceExpr {
    pub fn new(
        arr: Option<RcCell<LocationExpr>>,
        base: Option<ASTFlatten>,
        start_offset: i32,
        size: i32,
    ) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
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
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct MeExpr {
    pub expression_base: ExpressionBase,
    pub name: String,
}

impl IntoAST for MeExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::MeExpr(self))
    }
}

impl MeExpr {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(),
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
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct AllExpr {
    pub expression_base: ExpressionBase,
    pub name: String,
}
impl IntoAST for AllExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::AllExpr(self))
    }
}

impl AllExpr {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(),
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
    ExpressionASType,
    ASTChildren,
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
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ReclassifyExpr {
    ReclassifyExpr(ReclassifyExprBase),
    RehomExpr(RehomExpr),
    EncryptionExpression(EncryptionExpression),
}

impl ReclassifyExpr {
    pub fn func_name(&self) -> String {
        String::from("reveal")
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
impl IntoAST for ReclassifyExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(ReclassifyExpr::ReclassifyExpr(
            self,
        )))
    }
}

impl ReclassifyExprBase {
    pub fn new(expr: ASTFlatten, privacy: ASTFlatten, homomorphism: Option<String>) -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            expr,
            privacy,
            homomorphism,
        }
    }
    pub fn func_name(&self) -> String {
        String::from("reveal")
    }
}
impl ASTChildren for ReclassifyExprBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.expr.clone().into());
        cb.add_child(self.privacy.clone().into());
    }
}

#[impl_traits(ReclassifyExprBase, ExpressionBase, ASTBase)]
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
pub struct RehomExpr {
    pub reclassify_expr_base: ReclassifyExprBase,
}
impl IntoAST for RehomExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(ReclassifyExpr::RehomExpr(self)))
    }
}

impl RehomExpr {
    pub fn new(expr: ASTFlatten, homomorphism: Option<String>) -> Self {
        Self {
            reclassify_expr_base: ReclassifyExprBase::new(
                expr,
                RcCell::new(Expression::MeExpr(MeExpr::new())).into(),
                homomorphism,
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HybridArgType {
    PrivCircuitVal,
    PubCircuitArg,
    PubContractVal,
    TmpCircuitVal,
}
#[impl_traits(IdentifierBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HybridArgumentIdf {
    pub identifier_base: IdentifierBase,
    pub t: RcCell<TypeName>,
    pub arg_type: HybridArgType,
    pub corresponding_priv_expression: Option<ASTFlatten>,
    pub serialized_loc: SliceExpr,
}
impl IntoAST for HybridArgumentIdf {
    fn into_ast(self) -> AST {
        AST::Identifier(Identifier::HybridArgumentIdf(self))
    }
}

impl HybridArgumentIdf {
    pub fn new(
        name: String,
        mut t: RcCell<TypeName>,
        arg_type: HybridArgType,
        corresponding_priv_expression: Option<ASTFlatten>,
    ) -> Self {
        if is_instance(&t, ASTType::BooleanLiteralType) {
            t = RcCell::new(TypeName::bool_type());
        } else if is_instance(&t, ASTType::NumberLiteralType) {
            let tt = t
                .borrow()
                .try_as_elementary_type_name_ref()
                .unwrap()
                .try_as_number_type_name_ref()
                .unwrap()
                .try_as_number_literal_type_ref()
                .unwrap()
                .to_abstract_type();
            t = tt;
        } else if is_instance(&t, ASTType::EnumValueTypeName) {
            let tt = t
                .borrow()
                .try_as_user_defined_type_name_ref()
                .unwrap()
                .try_as_enum_value_type_name_ref()
                .unwrap()
                .to_abstract_type();
            t = tt;
        }

        Self {
            identifier_base: IdentifierBase::new(name),
            t,
            arg_type,
            corresponding_priv_expression,
            serialized_loc: SliceExpr::new(
                Some(RcCell::new(LocationExpr::IdentifierExpr(
                    IdentifierExpr::new(IdentifierExprUnion::String(String::new()), None),
                ))),
                None,
                -1,
                -1,
            ),
        }
    }

    pub fn get_loc_expr(&self, parent: Option<&ASTFlatten>) -> ASTFlatten {
        if self.arg_type == HybridArgType::TmpCircuitVal
            && self.corresponding_priv_expression.is_some()
            && is_instance(
                &*self
                    .corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow(),
                ASTType::BooleanLiteralType,
            )
        {
            RcCell::new(BooleanLiteralExpr::new(
                self.corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_boolean_literal_type_ref()
                    .unwrap()
                    .value(),
            ))
            .into()
        } else if self.arg_type == HybridArgType::TmpCircuitVal
            && self.corresponding_priv_expression.is_some()
            && is_instance(
                self.corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap(),
                ASTType::NumberLiteralType,
            )
        {
            RcCell::new(NumberLiteralExpr::new(
                self.corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_number_type_name_ref()
                    .unwrap()
                    .try_as_number_literal_type_ref()
                    .unwrap()
                    .value(),
                false,
            ))
            .into()
        } else {
            assert!(self.arg_type == HybridArgType::PubCircuitArg);
            let mut ma = LocationExpr::IdentifierExpr(IdentifierExpr::new(
                IdentifierExprUnion::String(CFG.lock().unwrap().zk_data_var_name()),
                None,
            ))
            .dot(IdentifierExprUnion::Identifier(RcCell::new(
                Identifier::HybridArgumentIdf(self.clone()),
            )))
            .as_type(&self.t.clone().into());
            ma.ast_base_ref().unwrap().borrow_mut().parent =
                parent.clone().map(|p| p.clone().downgrade());
            ma.try_as_identifier_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement = parent
                .as_ref()
                .map(|&p| {
                    if is_instance(p, ASTType::ExpressionBase) {
                        p.try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .statement()
                            .clone()
                    } else {
                        Some(p.clone().downgrade())
                    }
                })
                .flatten();
            ma
        }
    }
    pub fn get_idf_expr(&self, parent: Option<&ASTFlatten>) -> Option<ASTFlatten> {
        let mut ie = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(RcCell::new(Identifier::HybridArgumentIdf(
                self.clone(),
            ))),
            None,
        )
        .as_type(&self.t.clone().into());

        ie.try_as_identifier_expr_ref()
            .unwrap()
            .borrow_mut()
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = parent.clone().map(|p| p.clone().downgrade());

        ie.try_as_identifier_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .statement = parent
            .as_ref()
            .map(|&p| {
                if is_instance(p, ASTType::ExpressionBase) {
                    p.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .statement()
                        .clone()
                } else {
                    Some(p.clone().downgrade())
                }
            })
            .flatten();
        Some(ie)
    }

    pub fn _set_serialized_loc(
        &mut self,
        idf: String,
        base: Option<ASTFlatten>,
        start_offset: i32,
    ) {
        assert!(self.serialized_loc.start_offset == -1);
        self.serialized_loc.arr = Some(RcCell::new(LocationExpr::IdentifierExpr(
            IdentifierExpr::new(IdentifierExprUnion::String(idf), None),
        )));
        self.serialized_loc.base = base;
        self.serialized_loc.start_offset = start_offset;
        self.serialized_loc.size = self.t.borrow().size_in_uints();
    }

    pub fn deserialize(
        &mut self,
        source_idf: String,
        base: Option<ASTFlatten>,
        start_offset: i32,
    ) -> AssignmentStatement {
        self._set_serialized_loc(source_idf.clone(), base.clone(), start_offset);

        let src = IdentifierExpr::new(IdentifierExprUnion::String(source_idf), None)
            .as_type(&RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into());
        if let TypeName::Array(_a) = self.t.borrow().clone() {
            SliceExpr::new(
                self.get_loc_expr(None).try_as_location_expr(),
                None,
                0,
                self.t.borrow().size_in_uints(),
            )
            .arr
            .unwrap()
            .borrow_mut()
            .assign(RcCell::new(self.serialized_loc.clone()).into())
        } else if let Some(base) = &base {
            self.get_loc_expr(None)
                .try_as_expression_mut()
                .unwrap()
                .borrow_mut()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .assign(
                    LocationExpr::IdentifierExpr(
                        src.try_as_identifier_expr_ref().unwrap().borrow().clone(),
                    )
                    .index(ExprUnion::Expression(
                        RcCell::new(base.try_as_expression_ref().unwrap().borrow().binop(
                            String::from("+"),
                            NumberLiteralExpr::new(start_offset, false).into_expr(),
                        ))
                        .into(),
                    ))
                    .try_as_expression()
                    .unwrap()
                    .borrow()
                    .explicitly_converted(&self.t),
                )
        } else {
            self.get_loc_expr(None)
                .try_as_expression_mut()
                .unwrap()
                .borrow_mut()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .assign(
                    LocationExpr::IdentifierExpr(
                        src.try_as_identifier_expr_ref().unwrap().borrow().clone(),
                    )
                    .index(ExprUnion::I32(start_offset))
                    .try_as_expression()
                    .unwrap()
                    .borrow()
                    .explicitly_converted(&self.t),
                )
        }
    }

    pub fn serialize(
        &mut self,
        target_idf: String,
        base: Option<ASTFlatten>,
        start_offset: i32,
    ) -> AssignmentStatement {
        self._set_serialized_loc(target_idf.clone(), base.clone(), start_offset);

        let tgt = IdentifierExpr::new(IdentifierExprUnion::String(target_idf), None)
            .as_type(&RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into());
        if let TypeName::Array(_t) = self.t.borrow().clone() {
            let loc = self.get_loc_expr(None);
            self.serialized_loc
                .arr
                .as_mut()
                .unwrap()
                .borrow_mut()
                .assign(
                    RcCell::new(SliceExpr::new(
                        loc.try_as_location_expr(),
                        None,
                        0,
                        self.t.borrow().size_in_uints(),
                    ))
                    .into(),
                )
        } else {
            let expr = self.get_loc_expr(None);
            let expr = if self.t.borrow().is_signed_numeric() {
                // Cast to same size uint to prevent sign extension
                expr.try_as_expression()
                    .unwrap()
                    .borrow()
                    .explicitly_converted(&RcCell::new(
                        UintTypeName::new(format!("uint{}", self.t.borrow().elem_bitwidth()))
                            .into_ast()
                            .try_as_type_name()
                            .unwrap(),
                    ))
            } else if self.t.borrow().is_numeric() && self.t.borrow().elem_bitwidth() == 256 {
                expr.try_as_expression()
                    .unwrap()
                    .borrow()
                    .binop(
                        String::from("%"),
                        IdentifierExpr::new(
                            IdentifierExprUnion::String(CFG.lock().unwrap().field_prime_var_name()),
                            None,
                        )
                        .into_expr(),
                    )
                    .as_type(&self.t.clone().into())
            } else {
                expr.try_as_expression()
                    .unwrap()
                    .borrow()
                    .explicitly_converted(&RcCell::new(TypeName::uint_type()))
                //if let ExplicitlyConvertedUnion::FunctionCallExpr(fce)={fce}else{FunctionCallExpr::default()}
            };

            if let Some(base) = &base {
                LocationExpr::IndexExpr(
                    LocationExpr::IdentifierExpr(
                        tgt.try_as_identifier_expr_ref().unwrap().borrow().clone(),
                    )
                    .index(ExprUnion::Expression(
                        RcCell::new(base.try_as_expression_ref().unwrap().borrow().binop(
                            String::from("+"),
                            NumberLiteralExpr::new(start_offset, false).into_expr(),
                        ))
                        .into(),
                    ))
                    .try_as_index_expr_ref()
                    .unwrap()
                    .borrow()
                    .clone(),
                )
                .assign(expr)
            } else {
                LocationExpr::IndexExpr(
                    LocationExpr::IdentifierExpr(
                        tgt.try_as_identifier_expr_ref().unwrap().borrow().clone(),
                    )
                    .index(ExprUnion::I32(start_offset))
                    .try_as_index_expr_ref()
                    .unwrap()
                    .borrow()
                    .clone(),
                )
                .assign(expr)
            }
        }
    }
}
#[enum_dispatch(
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    IdentifierBaseRef,
    IdentifierBaseMutRef,
    ASTBaseRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Identifier {
    Identifier(IdentifierBase),
    HybridArgumentIdf(HybridArgumentIdf),
}
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Identifier::Identifier(idf) => &idf.name,
                Identifier::HybridArgumentIdf(idf) => &idf.identifier_base.name,
            }
        )
    }
}
impl Identifier {
    pub fn identifier(name: &str) -> Option<RcCell<Self>> {
        Some(RcCell::new(Self::Identifier(IdentifierBase::new(
            String::from(name),
        ))))
    }
}

#[impl_traits(ReclassifyExprBase, ExpressionBase, ASTBase)]
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
pub struct EncryptionExpression {
    pub reclassify_expr_base: ReclassifyExprBase,
    pub annotated_type: Option<RcCell<AnnotatedTypeName>>,
}
impl IntoAST for EncryptionExpression {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(
            ReclassifyExpr::EncryptionExpression(self),
        ))
    }
}

impl EncryptionExpression {
    pub fn new(expr: ASTFlatten, privacy: ASTFlatten, homomorphism: Option<String>) -> Self {
        let annotated_type = Some(AnnotatedTypeName::cipher_type(
            expr.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .clone(),
            homomorphism.clone(),
        ));
        Self {
            reclassify_expr_base: ReclassifyExprBase::new(expr, privacy, homomorphism),
            annotated_type,
        }
    }
}
#[enum_dispatch(ASTChildren, IntoAST, ASTFlattenImpl, ASTInstanceOf)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl StatementBase {
    pub fn new() -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            before_analysis: None,
            after_analysis: None,
            function: None,
            pre_statements: vec![],
        }
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    CircuitComputationStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl CircuitDirectiveStatementBase {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(),
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
    pub idf: Option<RcCell<HybridArgumentIdf>>,
}

impl IntoAST for CircuitComputationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitDirectiveStatement(
            CircuitDirectiveStatement::CircuitComputationStatement(self),
        ))
    }
}

impl CircuitComputationStatement {
    pub fn new(idf: HybridArgumentIdf) -> Self {
        Self {
            circuit_directive_statement_base: CircuitDirectiveStatementBase::new(),
            idf: Some(RcCell::new(idf)),
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

impl IntoAST for EnterPrivateKeyStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitDirectiveStatement(
            CircuitDirectiveStatement::EnterPrivateKeyStatement(self),
        ))
    }
}

impl EnterPrivateKeyStatement {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            circuit_directive_statement_base: CircuitDirectiveStatementBase::new(),
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

impl IntoAST for IfStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::IfStatement(self))
    }
}

impl IfStatement {
    pub fn new(condition: ASTFlatten, then_branch: Block, else_branch: Option<Block>) -> Self {
        Self {
            statement_base: StatementBase::new(),
            condition,
            then_branch: RcCell::new(then_branch),
            else_branch: else_branch.map(RcCell::new),
        }
    }
}
impl ASTChildren for IfStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.clone().into());
        cb.add_child(self.then_branch.clone().into());
        if let Some(else_branch) = &self.else_branch {
            cb.add_child(else_branch.clone().into());
        }
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WhileStatement {
    pub statement_base: StatementBase,
    pub condition: ASTFlatten,
    pub body: RcCell<Block>,
}
impl IntoAST for WhileStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::WhileStatement(self))
    }
}

impl WhileStatement {
    pub fn new(condition: ASTFlatten, body: Block) -> Self {
        Self {
            statement_base: StatementBase::new(),
            condition,
            body: RcCell::new(body),
        }
    }
}
impl ASTChildren for WhileStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.clone().into());
        cb.add_child(self.body.clone().into());
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DoWhileStatement {
    pub statement_base: StatementBase,
    pub body: RcCell<Block>,
    pub condition: ASTFlatten,
}
impl IntoAST for DoWhileStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::DoWhileStatement(self))
    }
}

impl DoWhileStatement {
    pub fn new(body: Block, condition: ASTFlatten) -> Self {
        Self {
            statement_base: StatementBase::new(),
            body: RcCell::new(body),
            condition,
        }
    }
}
impl ASTChildren for DoWhileStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.body.clone().into());
        cb.add_child(self.condition.clone().into());
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
impl IntoAST for ForStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ForStatement(self))
    }
}

impl ForStatement {
    pub fn new(
        init: Option<SimpleStatement>,
        condition: ASTFlatten,
        update: Option<SimpleStatement>,
        body: Block,
    ) -> Self {
        Self {
            statement_base: StatementBase::new(),
            init: init.map(RcCell::new),
            condition,
            update: update.map(RcCell::new),
            body: RcCell::new(body),
        }
    }

    pub fn statements(&self) -> Vec<Statement> {
        vec![
            self.init
                .as_ref()
                .map(|i| i.borrow().to_statement())
                .unwrap(),
            self.condition
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .to_statement(),
            self.update
                .as_ref()
                .map(|u| u.borrow().to_statement())
                .unwrap(),
            self.body.borrow().to_statement(),
        ]
    }
}
impl ASTChildren for ForStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(init) = &self.init {
            cb.add_child(init.clone().into());
        }

        cb.add_child(self.condition.clone().into());
        if let Some(update) = &self.update {
            cb.add_child(update.clone().into());
        }
        cb.add_child(self.body.clone().into());
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

impl IntoAST for BreakStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::BreakStatement(self))
    }
}

impl BreakStatement {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(),
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

impl IntoAST for ContinueStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ContinueStatement(self))
    }
}

impl ContinueStatement {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(),
        }
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReturnStatement {
    pub statement_base: StatementBase,
    pub expr: Option<ASTFlatten>,
}
impl IntoAST for ReturnStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ReturnStatement(self))
    }
}

impl ReturnStatement {
    pub fn new(expr: Option<ASTFlatten>) -> Self {
        Self {
            statement_base: StatementBase::new(),
            expr,
        }
    }
}
impl ASTChildren for ReturnStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone().into());
        }
    }
}

#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    SimpleStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

impl SimpleStatementBase {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(),
        }
    }
}
#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub expr: ASTFlatten,
}

impl IntoAST for ExpressionStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::ExpressionStatement(self),
        ))
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
        cb.add_child(self.expr.clone().into());
    }
}

#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RequireStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub condition: ASTFlatten,
    pub unmodified_code: String,
}

impl IntoAST for RequireStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::RequireStatement(self),
        ))
    }
}

impl RequireStatement {
    pub fn new(condition: ASTFlatten, unmodified_code: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            condition,
            unmodified_code: unmodified_code.unwrap_or(String::new()), //self.code()
        }
    }
}
impl ASTChildren for RequireStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.clone().into());
    }
}

#[enum_dispatch(
    ASTChildren,
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
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

impl IntoAST for AssignmentStatementBase {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(self)),
        ))
    }
}

impl AssignmentStatementBase {
    pub fn new(lhs: Option<ASTFlatten>, rhs: Option<ASTFlatten>, op: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            lhs,
            rhs,
            op: op.unwrap_or(String::new()),
        }
    }
}
impl ASTChildren for AssignmentStatementBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(lhs) = &self.lhs {
            cb.add_child(lhs.clone().into());
        }
        if let Some(rhs) = &self.rhs {
            cb.add_child(rhs.clone().into());
        }
    }
}
#[impl_traits(AssignmentStatementBase, SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitInputStatement {
    pub assignment_statement_base: AssignmentStatementBase,
}

impl IntoAST for CircuitInputStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::CircuitInputStatement(self)),
        ))
    }
}
impl ASTChildren for CircuitInputStatement {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.assignment_statement_base.process_children(cb);
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
    ASTChildren,
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
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
            if is_instance(s, ASTType::StatementListBase) {
                if s.try_as_statement_ref()
                    .unwrap()
                    .borrow()
                    .try_as_statement_list_ref()
                    .unwrap()
                    .contains(stmt)
                {
                    return true;
                }
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
impl StatementListBase {
    pub fn new(statements: Vec<ASTFlatten>, excluded_from_simulation: bool) -> Self {
        Self {
            statement_base: StatementBase::new(),
            statements,
            excluded_from_simulation,
        }
    }
}
impl IntoAST for StatementListBase {
    fn into_ast(self) -> AST {
        StatementList::StatementList(self).into_ast()
    }
}

impl ASTChildren for StatementListBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        self.statements.iter().for_each(|statement| {
            cb.add_child(statement.clone());
        });
    }
}

#[impl_traits(StatementListBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Block {
    pub statement_list_base: StatementListBase,
    pub was_single_statement: bool,
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

impl IndentBlock {
    pub fn new(statements: Vec<ASTFlatten>) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
        }
    }
}
// #[enum_dispatch(IntoAST, ASTInstanceOf, TypeNameBaseRef, ASTBaseRef)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
    Mapping(Mapping),
    Array(Array),
    TupleType(TupleType),
    FunctionTypeName(FunctionTypeName),
    Literal(String),
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
// impl TypeName {
//     pub fn to_ast_flatten<'a>(&'a mut self) -> Option<ASTFlatten<'a>> {
//         match self {
//             TypeName::ElementaryTypeName(ast) => Some(ast.to_ast_flatten()),
//             TypeName::UserDefinedTypeName(ast) => Some(ast.to_ast_flatten()),
//             TypeName::Mapping(ast) => Some(ast.to_ast_flatten()),
//             TypeName::Array(ast) => Some(ast.to_ast_flatten()),
//             TypeName::TupleType(ast) => Some(ast.to_ast_flatten()),
//             TypeName::FunctionTypeName(ast) => Some(ast.to_ast_flatten()),
//             _ => None,
//         }
//     }
// }
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
        let crypto_params = CFG.lock().unwrap().user_config.get_crypto_params(&hom);
        plain_type.borrow_mut().homomorphism = hom; // Just for display purposes
        TypeName::Array(Array::CipherText(CipherText::new(
            plain_type,
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
        )))
    }
    // """How many uints this type occupies when serialized."""
    pub fn size_in_uints(&self) -> i32 {
        1
    }

    pub fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        // raise NotImplementedError()
        1
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

    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        &*expected.borrow() == self
    }
    pub fn compatible_with(self, other_type: &RcCell<TypeName>) -> bool {
        self.implicitly_convertible_to(&other_type)
            || other_type
                .borrow()
                .implicitly_convertible_to(&RcCell::new(self.clone()))
    }
    pub fn combined_type(
        &self,
        other_type: &RcCell<TypeName>,
        _convert_literals: bool,
    ) -> Option<RcCell<Self>> {
        let selfs = RcCell::new(self.clone());
        if other_type.borrow().implicitly_convertible_to(&selfs) {
            Some(selfs)
        } else if self.implicitly_convertible_to(&other_type) {
            Some(other_type.clone())
        } else {
            None
        }
    }
    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> RcCell<AnnotatedTypeName> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(self.clone())),
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
impl TypeNameBase {
    pub fn new() -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
        }
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ElementaryTypeNameBaseRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ElementaryTypeName {
    NumberTypeName(NumberTypeName),
    BoolTypeName(BoolTypeName),
    BooleanLiteralType(BooleanLiteralType),
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
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ElementaryTypeNameBase {
    pub type_name_base: TypeNameBase,
    pub name: String,
}
impl ElementaryTypeNameBase {
    pub fn new(name: String) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            name,
        }
    }
}
#[impl_traits(ElementaryTypeNameBase, TypeNameBase, ASTBase)]
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
pub struct BoolTypeName {
    pub elementary_type_name_base: ElementaryTypeNameBase,
}
impl IntoAST for BoolTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::BoolTypeName(self),
        ))
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
pub struct BooleanLiteralType {
    pub elementary_type_name_base: ElementaryTypeNameBase,
}
impl IntoAST for BooleanLiteralType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::BooleanLiteralType(self),
        ))
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
    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || is_instance(expected, ASTType::BoolTypeName)
    }
    pub fn combined_type(
        &self,
        other_type: RcCell<TypeName>,
        convert_literals: bool,
    ) -> RcCell<TypeName> {
        if is_instance(&other_type, ASTType::BooleanLiteralType) {
            RcCell::new(if convert_literals {
                TypeName::bool_type()
            } else {
                TypeName::Literal(String::from("lit"))
            })
        } else {
            self.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .combined_type(&other_type, convert_literals)
                .unwrap()
        }
    }
    pub fn value(&self) -> bool {
        &self.elementary_type_name_base.name == "true"
    }
    pub fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        // raise NotImplementedError()
        1
    }
    pub fn to_abstract_type(&self) -> RcCell<TypeName> {
        RcCell::new(TypeName::bool_type())
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    NumberTypeNameBaseRef,
    ElementaryTypeNameBaseRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum NumberTypeName {
    NumberLiteralType(NumberLiteralType),
    IntTypeName(IntTypeName),
    UintTypeName(UintTypeName),
    NumberTypeNameBase(NumberTypeNameBase),
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
impl IntoAST for NumberTypeNameBase {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::NumberTypeNameBase(self)),
        ))
    }
}

impl NumberTypeNameBase {
    pub fn new(name: String, prefix: String, signed: bool, bitwidth: Option<i32>) -> Self {
        assert!(name.starts_with(&prefix), "{name} {prefix}");
        let prefix_len = prefix.len();
        let _size_in_bits = if let Some(bitwidth) = bitwidth {
            bitwidth
        } else {
            if name.len() > prefix_len {
                name[prefix_len..].parse::<i32>().unwrap()
            } else {
                0
            }
        };
        Self {
            elementary_type_name_base: ElementaryTypeNameBase::new(name),
            prefix,
            signed,
            bitwidth,
            _size_in_bits,
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || is_instance(expected, ASTType::NumberTypeNameBase)
    }
    pub fn elem_bitwidth(&self) -> i32 {
        // Bitwidth, only defined for primitive types
        // raise NotImplementedError()
        if self._size_in_bits == 0 {
            256
        } else {
            self._size_in_bits
        }
    }
    // """Return true if value can be represented by this type"""
    pub fn can_represent(&self, value: i32) -> bool {
        let elem_bitwidth = self.elem_bitwidth() as usize;
        let lo = if self.signed {
            -(1 << elem_bitwidth - 1)
        } else {
            0
        };
        let hi = if self.signed {
            1 << elem_bitwidth - 1
        } else {
            1 << elem_bitwidth
        };
        lo <= value && value < hi
    }
}
pub enum NumberLiteralTypeUnion {
    String(String),
    I32(i32),
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
pub struct NumberLiteralType {
    pub number_type_name_base: NumberTypeNameBase,
}
impl IntoAST for NumberLiteralType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::NumberLiteralType(self)),
        ))
    }
}

impl NumberLiteralType {
    pub fn new(name: NumberLiteralTypeUnion) -> Self {
        let name = match name {
            NumberLiteralTypeUnion::String(v) => v.parse::<i32>().unwrap(), //TODO U256
            NumberLiteralTypeUnion::I32(v) => v,
        };
        let blen = (i32::BITS - name.leading_zeros()) as i32;
        let (signed, mut bitwidth) = if name < 0 {
            (
                true,
                if name != -(1 << (blen - 1)) {
                    blen + 1
                } else {
                    blen
                },
            )
        } else {
            (false, blen)
        };
        bitwidth = 8i32.max((bitwidth + 7) / 8 * 8);
        assert!(bitwidth <= 256);
        let name = name.to_string();
        let prefix = name.clone();
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, prefix, signed, Some(bitwidth)),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        if expected.borrow().is_numeric() && !expected.borrow().is_literals()
        // Allow implicit conversion only if it fits
        {
            expected
                .borrow()
                .try_as_elementary_type_name_ref()
                .unwrap()
                .try_as_number_type_name_ref()
                .unwrap()
                .number_type_name_base_ref()
                .can_represent(self.value())
        } else if expected.borrow().is_address()
            && self.number_type_name_base.elem_bitwidth() == 160
            && !self.number_type_name_base.signed
        // Address literal case (fake solidity check will catch the cases where this is too permissive)
        {
            true
        } else {
            self.number_type_name_base
                .implicitly_convertible_to(expected)
        }
    }
    pub fn combined_type(
        &self,
        other_type: RcCell<TypeName>,
        convert_literals: bool,
    ) -> RcCell<TypeName> {
        if is_instance(&other_type, ASTType::NumberLiteralType) {
            if convert_literals {
                self.to_abstract_type()
                    .borrow()
                    .combined_type(
                        &other_type
                            .borrow()
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
                RcCell::new(TypeName::Literal(String::from("lit")))
            }
        } else {
            self.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .combined_type(&other_type, convert_literals)
                .unwrap()
        }
    }
    pub fn to_abstract_type(&self) -> RcCell<TypeName> {
        RcCell::new(if self.value() < 0 {
            TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::IntTypeName(IntTypeName::new(format!(
                    "i32{}",
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
    }
    pub fn value(&self) -> i32 {
        self.number_type_name_base
            .elementary_type_name_base
            .name
            .parse::<i32>()
            .unwrap()
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
impl IntoAST for IntTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::IntTypeName(self)),
        ))
    }
}

impl IntTypeName {
    pub fn new(name: String) -> Self {
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, String::from("int"), true, None),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.number_type_name_base
            .implicitly_convertible_to(expected)
            || is_instance(expected, ASTType::IntTypeName)
                && expected.borrow().elem_bitwidth() >= self.number_type_name_base.elem_bitwidth()
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
impl IntoAST for UintTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::ElementaryTypeName(
            ElementaryTypeName::NumberTypeName(NumberTypeName::UintTypeName(self)),
        ))
    }
}

impl UintTypeName {
    pub fn new(name: String) -> Self {
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, String::from("uint"), false, None),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.number_type_name_base
            .implicitly_convertible_to(expected)
            || is_instance(expected, ASTType::UintTypeName)
                && expected.borrow().elem_bitwidth() >= self.number_type_name_base.elem_bitwidth()
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    UserDefinedTypeNameBaseRef,
    UserDefinedTypeNameBaseMutRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum UserDefinedTypeName {
    EnumTypeName(EnumTypeName),
    EnumValueTypeName(EnumValueTypeName),
    StructTypeName(StructTypeName),
    ContractTypeName(ContractTypeName),
    AddressTypeName(AddressTypeName),
    AddressPayableTypeName(AddressPayableTypeName),
}

#[enum_dispatch]
pub trait UserDefinedTypeNameBaseRef: TypeNameBaseRef {
    fn user_defined_type_name_base_ref(&self) -> &UserDefinedTypeNameBase;
}
pub trait UserDefinedTypeNameBaseProperty {
    fn names(&self) -> &Vec<RcCell<Identifier>>;
    fn target(&self) -> &Option<ASTFlattenWeak>;
}
impl<T: UserDefinedTypeNameBaseRef> UserDefinedTypeNameBaseProperty for T {
    fn names(&self) -> &Vec<RcCell<Identifier>> {
        &self.user_defined_type_name_base_ref().names
    }
    fn target(&self) -> &Option<ASTFlattenWeak> {
        &self.user_defined_type_name_base_ref().target
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UserDefinedTypeNameBase {
    pub type_name_base: TypeNameBase,
    pub names: Vec<RcCell<Identifier>>,
    pub target: Option<ASTFlattenWeak>,
}
impl UserDefinedTypeNameBase {
    pub fn new(names: Vec<Identifier>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            names: names.into_iter().map(RcCell::new).collect(),
            target,
        }
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
impl IntoAST for EnumTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::EnumTypeName(self),
        ))
    }
}

impl EnumTypeName {
    pub fn new(names: Vec<Identifier>, target: Option<ASTFlattenWeak>) -> Self {
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
impl IntoAST for EnumValueTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::EnumValueTypeName(self),
        ))
    }
}

impl EnumValueTypeName {
    pub fn new(names: Vec<Identifier>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(names, target),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        256
    }
    pub fn to_abstract_type(&self) -> RcCell<TypeName> {
        let mut names: Vec<_> = self
            .user_defined_type_name_base
            .names
            .iter()
            .map(|name| name.borrow().clone())
            .collect();
        names.pop();
        RcCell::new(
            EnumTypeName::new(
                names,
                self.user_defined_type_name_base
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
    }
    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || (is_instance(expected, ASTType::EnumTypeName)
                && expected
                    .borrow()
                    .try_as_user_defined_type_name_ref()
                    .unwrap()
                    .try_as_enum_type_name_ref()
                    .unwrap()
                    .user_defined_type_name_base_ref()
                    .names
                    .clone()
                    >= self.user_defined_type_name_base.names
                        [..self.user_defined_type_name_base.names.len() - 1]
                        .to_vec())
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
impl IntoAST for StructTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::StructTypeName(self),
        ))
    }
}

impl StructTypeName {
    pub fn new(names: Vec<Identifier>, target: Option<ASTFlattenWeak>) -> Self {
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
impl IntoAST for ContractTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::ContractTypeName(self),
        ))
    }
}

impl ContractTypeName {
    pub fn new(names: Vec<Identifier>, target: Option<ASTFlattenWeak>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(names, target),
        }
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
pub struct AddressTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl IntoAST for AddressTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::AddressTypeName(self),
        ))
    }
}

impl AddressTypeName {
    pub fn new() -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                vec![Identifier::Identifier(IdentifierBase::new(String::from(
                    "<address>",
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
pub struct AddressPayableTypeName {
    pub user_defined_type_name_base: UserDefinedTypeNameBase,
}
impl IntoAST for AddressPayableTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::UserDefinedTypeName(
            UserDefinedTypeName::AddressPayableTypeName(self),
        ))
    }
}

impl AddressPayableTypeName {
    pub fn new() -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                vec![Identifier::Identifier(IdentifierBase::new(String::from(
                    "<address_payable>",
                )))],
                None,
            ),
        }
    }

    pub fn implicitly_convertible_to(&self, expected: &RcCell<TypeName>) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || &*expected.borrow() == &TypeName::address_type()
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
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mapping {
    pub type_name_base: TypeNameBase,
    pub key_type: RcCell<TypeName>,
    pub key_label: Option<RcCell<Identifier>>,
    pub value_type: RcCell<AnnotatedTypeName>,
    pub instantiated_key: Option<ASTFlatten>,
}
impl IntoAST for Mapping {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Mapping(self))
    }
}

impl Mapping {
    pub fn new(
        key_type: TypeName,
        key_label: Option<Identifier>,
        value_type: AnnotatedTypeName,
    ) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            key_type: RcCell::new(key_type),
            key_label: key_label.map(|kl| RcCell::new(kl)),
            value_type: RcCell::new(value_type),
            instantiated_key: None,
        }
    }
    pub fn has_key_label(&self) -> bool {
        self.key_label.is_some()
    }
}
impl ASTChildren for Mapping {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.key_type.clone().into());
        if let Some(idf) = &self.key_label {
            cb.add_child(idf.clone().into());
        }
        cb.add_child(self.value_type.clone().into());
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ExprUnion {
    I32(i32),
    Expression(ASTFlatten),
}

#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    ArrayBaseRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Array {
    CipherText(CipherText),
    Randomness(Randomness),
    Key(Key),
    Proof(Proof),
    Array(ArrayBase),
}

impl ASTChildren for ArrayBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        cb.add_child(self.value_type.clone().into());
        if let Some(expr) = &self.expr {
            cb.add_child(expr.clone().into());
        }
    }
}

#[enum_dispatch]
pub trait ArrayBaseRef: TypeNameBaseRef {
    fn array_base_ref(&self) -> &ArrayBase;
}

pub trait ArrayBaseProperty {
    fn value_type(&self) -> &RcCell<AnnotatedTypeName>;
    fn expr(&self) -> &Option<ASTFlatten>;
}
impl<T: ArrayBaseRef> ArrayBaseProperty for T {
    fn value_type(&self) -> &RcCell<AnnotatedTypeName> {
        &self.array_base_ref().value_type
    }
    fn expr(&self) -> &Option<ASTFlatten> {
        &self.array_base_ref().expr
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
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
pub struct ArrayBase {
    pub type_name_base: TypeNameBase,
    pub value_type: RcCell<AnnotatedTypeName>,
    pub expr: Option<ASTFlatten>,
}
impl IntoAST for ArrayBase {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Array(self)))
    }
}

impl ArrayBase {
    pub fn new(value_type: RcCell<AnnotatedTypeName>, expr: Option<ExprUnion>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            value_type,
            expr: expr.map(|_expr| match _expr {
                ExprUnion::I32(exp) => RcCell::new(NumberLiteralExpr::new(exp, false)).into(),
                ExprUnion::Expression(exp) => exp,
            }),
        }
    }
    pub fn size_in_uints(&self) -> i32 {
        if is_instance(self.expr.as_ref().unwrap(), ASTType::NumberLiteralExpr) {
            return self
                .expr
                .as_ref()
                .unwrap()
                .try_as_literal_expr_ref()
                .unwrap()
                .borrow()
                .try_as_number_literal_expr_ref()
                .unwrap()
                .value
                .clone();
        }
        -1
    }

    pub fn elem_bitwidth(&self) -> i32 {
        self.value_type
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .borrow()
            .elem_bitwidth()
    }
}

#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CipherText {
    pub array_base: ArrayBase,
    pub plain_type: RcCell<AnnotatedTypeName>,
    pub crypto_params: CryptoParams,
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

impl CipherText {
    pub fn new(plain_type: RcCell<AnnotatedTypeName>, crypto_params: CryptoParams) -> Self {
        assert!(!plain_type
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .borrow()
            .is_cipher());
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(
                    RcCell::new(NumberLiteralExpr::new(crypto_params.cipher_len(), false)).into(),
                )),
            ),
            plain_type,
            crypto_params,
        }
    }
    pub fn size_in_uints(&self) -> i32 {
        self.crypto_params.cipher_payload_len()
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Randomness {
    pub array_base: ArrayBase,
    pub crypto_params: CryptoParams,
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

impl Randomness {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                if let Some(randomness_len) = crypto_params.randomness_len() {
                    Some(ExprUnion::Expression(
                        RcCell::new(NumberLiteralExpr::new(randomness_len, false)).into(),
                    ))
                } else {
                    None
                },
            ),
            crypto_params,
        }
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Key {
    pub array_base: ArrayBase,
    pub crypto_params: CryptoParams,
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

impl Key {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(
                    RcCell::new(NumberLiteralExpr::new(crypto_params.key_len(), false)).into(),
                )),
            ),
            crypto_params,
        }
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Proof {
    pub array_base: ArrayBase,
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
impl IntoAST for DummyAnnotation {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::DummyAnnotation(self))
    }
}

impl DummyAnnotation {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(),
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
            expr.map(|exp| exp.into())
        } else {
            None
        }
    }
}
//     """Does not appear in the syntax, but is necessary for type checking"""
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleType {
    pub type_name_base: TypeNameBase,
    pub types: Vec<RcCell<AnnotatedTypeName>>,
}
impl IntoAST for TupleType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::TupleType(self))
    }
}

impl TupleType {
    pub fn new(types: Vec<RcCell<AnnotatedTypeName>>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            types,
        }
    }
    pub fn ensure_tuple(t: Option<AnnotatedTypeName>) -> TupleType {
        if let Some(t) = t {
            if let Some(TypeName::TupleType(t)) = t.type_name.as_ref().map(|t| t.borrow().clone()) {
                t.clone()
            } else {
                TupleType::new(vec![RcCell::new(t.clone())])
            }
        } else {
            TupleType::empty()
        }
    }

    pub fn len(&self) -> i32 {
        self.types.len() as i32
    }

    pub fn get_item(&self, i: i32) -> RcCell<AnnotatedTypeName> {
        self.types[i as usize].clone()
    }

    pub fn check_component_wise(
        &self,
        other: &RcCell<TypeName>,
        f: impl FnOnce(RcCell<AnnotatedTypeName>, RcCell<AnnotatedTypeName>) -> bool + std::marker::Copy,
    ) -> bool {
        if self.len() != other.borrow().try_as_tuple_type_ref().unwrap().len() {
            false
        } else {
            for i in 0..self.len() {
                if !f(
                    self.get_item(i),
                    other.borrow().try_as_tuple_type_ref().unwrap().get_item(i),
                ) {
                    return false;
                }
            }
            true
        }
    }

    pub fn implicitly_convertible_to(&self, expected: RcCell<TypeName>) -> bool {
        if expected.borrow().is_tuple_type() {
            self.check_component_wise(&expected, |x, y| {
                x.borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .implicitly_convertible_to(y.borrow().type_name.as_ref().unwrap())
            })
        } else {
            false
        }
    }

    pub fn compatible_with(&self, other_type: RcCell<TypeName>) -> bool {
        if other_type.borrow().is_tuple_type() {
            self.check_component_wise(&other_type, |x, y| {
                x.borrow()
                    .type_name
                    .clone()
                    .unwrap()
                    .borrow()
                    .clone()
                    .compatible_with(y.borrow().type_name.as_ref().unwrap())
            })
        } else {
            false
        }
    }

    pub fn combined_type(
        &self,
        other_type: TupleType,
        convert_literals: bool,
    ) -> Option<TupleType> {
        if self.types.len() != other_type.types.len() {
            None
        } else {
            Some(TupleType::new(
                self.types
                    .iter()
                    .zip(&other_type.types)
                    .map(|(e1, e2)| {
                        RcCell::new(AnnotatedTypeName::new(
                            e1.borrow()
                                .type_name
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .combined_type(
                                    e2.borrow().type_name.as_ref().unwrap(),
                                    convert_literals,
                                ),
                            Some(
                                RcCell::new(Expression::DummyAnnotation(DummyAnnotation::new()))
                                    .into(),
                            ),
                            Homomorphism::non_homomorphic(),
                        ))
                    })
                    .collect(),
            ))
        }
    }
    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> CombinedPrivacyUnion {
        CombinedPrivacyUnion::AST(match privacy_annotation {
            CombinedPrivacyUnion::AST(_) => Some(
                RcCell::new(AnnotatedTypeName::new(
                    Some(RcCell::new(TypeName::TupleType(TupleType::new(
                        self.types
                            .iter()
                            .map(|t| {
                                t.borrow()
                                    .type_name
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .annotate(privacy_annotation.clone())
                            })
                            .collect(),
                    )))),
                    None,
                    Homomorphism::non_homomorphic(),
                ))
                .into(),
            ),
            CombinedPrivacyUnion::Vec(privacy_annotation) => {
                assert!(self.types.len() == privacy_annotation.len());
                Some(
                    RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::TupleType(TupleType::new(
                            self.types
                                .iter()
                                .zip(privacy_annotation)
                                .map(|(t, a)| {
                                    t.borrow()
                                        .type_name
                                        .as_ref()
                                        .unwrap()
                                        .borrow()
                                        .annotate(a.clone())
                                })
                                .collect(),
                        )))),
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
            &RcCell::new(TypeName::TupleType(other.clone())),
            privacy_match,
        )
    }

    pub fn empty() -> TupleType {
        TupleType::new(vec![])
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FunctionTypeName {
    pub type_name_base: TypeNameBase,
    pub parameters: Vec<RcCell<Parameter>>,
    pub modifiers: Vec<String>,
    pub return_parameters: Vec<RcCell<Parameter>>,
}
impl IntoAST for FunctionTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::FunctionTypeName(self))
    }
}

impl FunctionTypeName {
    pub fn new(
        parameters: Vec<RcCell<Parameter>>,
        modifiers: Vec<String>,
        return_parameters: Vec<RcCell<Parameter>>,
    ) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
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

#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnnotatedTypeName {
    pub ast_base: RcCell<ASTBase>,
    pub type_name: Option<RcCell<TypeName>>,
    pub had_privacy_annotation: bool,
    pub privacy_annotation: Option<ASTFlatten>,
    pub homomorphism: String,
}
impl IntoAST for AnnotatedTypeName {
    fn into_ast(self) -> AST {
        AST::AnnotatedTypeName(self)
    }
}

impl AnnotatedTypeName {
    pub fn new(
        type_name: Option<RcCell<TypeName>>,
        privacy_annotation: Option<ASTFlatten>,
        homomorphism: String,
    ) -> Self {
        assert!(
            !(privacy_annotation.is_some()
                && is_instance(privacy_annotation.as_ref().unwrap(), ASTType::AllExpr)
                && homomorphism != Homomorphism::non_homomorphic()),
            "Public type name cannot be homomorphic (got {:?})",
            HOMOMORPHISM_STORE.lock().unwrap().get(&homomorphism)
        );
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            type_name,
            had_privacy_annotation: privacy_annotation.as_ref().is_some(),
            privacy_annotation: privacy_annotation.or(Some(
                RcCell::new(Expression::AllExpr(AllExpr::new())).into(),
            )),
            homomorphism,
        }
    }
    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
    pub fn zkay_type(&self) -> Self {
        if let Some(TypeName::Array(Array::CipherText(ct))) =
            self.type_name.as_ref().map(|t| t.borrow().clone())
        {
            ct.plain_type.borrow().clone()
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
            self.type_name.as_ref().map(|t| t.borrow().clone()),
            other
                .borrow()
                .type_name
                .as_ref()
                .map(|t| t.borrow().clone()),
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
            Some(RcCell::new(TypeName::uint_type())),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn bool_all() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::bool_type())),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn address_all() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::address_type())),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn cipher_type(plain_type: RcCell<AnnotatedTypeName>, hom: Option<String>) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::cipher_type(plain_type, hom.unwrap()))),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn key_type(crypto_params: CryptoParams) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::key_type(crypto_params))),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }

    pub fn proof_type() -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::proof_type())),
            None,
            Homomorphism::non_homomorphic(),
        ))
    }
    pub fn all(type_name: TypeName) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(type_name)),
            Some(RcCell::new(Expression::all_expr()).into()),
            Homomorphism::non_homomorphic(),
        ))
    }
    pub fn me(type_name: TypeName) -> RcCell<Self> {
        RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(type_name)),
            Some(RcCell::new(Expression::me_expr(None)).into()),
            Homomorphism::non_homomorphic(),
        ))
    }
    pub fn array_all(value_type: RcCell<AnnotatedTypeName>, length: Vec<i32>) -> RcCell<Self> {
        let mut t = value_type;
        for &l in &length {
            t = RcCell::new(AnnotatedTypeName::new(
                Some(RcCell::new(TypeName::Array(Array::Array(ArrayBase::new(
                    t,
                    Some(ExprUnion::I32(l)),
                ))))),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }
        t
    }
}
impl ASTChildren for AnnotatedTypeName {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(type_name) = &self.type_name {
            cb.add_child(type_name.clone().into());
        }
        if let Some(privacy_annotation) = &self.privacy_annotation {
            cb.add_child(privacy_annotation.clone().into());
        }
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    IdentifierDeclarationBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    fn annotated_type(&self) -> &Option<RcCell<AnnotatedTypeName>>;
    fn idf(&self) -> WeakCell<Identifier>;
    fn storage_location(&self) -> &Option<String>;
}
impl<T: IdentifierDeclarationBaseRef> IdentifierDeclarationBaseProperty for T {
    fn keywords(&self) -> &Vec<String> {
        &self.identifier_declaration_base_ref().keywords
    }
    fn annotated_type(&self) -> &Option<RcCell<AnnotatedTypeName>> {
        &self.identifier_declaration_base_ref().annotated_type
    }
    fn idf(&self) -> WeakCell<Identifier> {
        self.identifier_declaration_base_ref()
            .idf
            .as_ref()
            .unwrap()
            .downgrade()
    }
    fn storage_location(&self) -> &Option<String> {
        &self.identifier_declaration_base_ref().storage_location
    }
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierDeclarationBase {
    pub ast_base: RcCell<ASTBase>,
    pub keywords: Vec<String>,
    pub annotated_type: Option<RcCell<AnnotatedTypeName>>,
    pub idf: Option<RcCell<Identifier>>,
    pub storage_location: Option<String>,
}
impl IdentifierDeclarationBase {
    fn new(
        keywords: Vec<String>,
        annotated_type: Option<RcCell<AnnotatedTypeName>>,
        idf: Option<RcCell<Identifier>>,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            keywords,
            annotated_type: annotated_type,
            idf,
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
        cb.add_child(self.annotated_type.clone().unwrap().into());
        if let Some(idf) = &self.idf {
            // println!("===process_children===IdentifierDeclarationBase========={:?}",idf);
            cb.add_child(idf.clone().into());
        }
    }
}

#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclaration {
    pub identifier_declaration_base: IdentifierDeclarationBase,
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
#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclarationStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub variable_declaration: RcCell<VariableDeclaration>,
    pub expr: Option<ASTFlatten>,
}

impl IntoAST for VariableDeclarationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::VariableDeclarationStatement(self),
        ))
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
            cb.add_child(expr.clone().into());
        }
    }
}

#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Parameter {
    pub identifier_declaration_base: IdentifierDeclarationBase,
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ParameterUnion {
    Parameter(RcCell<Parameter>),
    String(String),
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
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    NamespaceDefinitionBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(ASTFlattenImpl, EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
pub trait NamespaceDefinitionBaseProperty {
    fn idf(&self) -> WeakCell<Identifier>;
}
impl<T: NamespaceDefinitionBaseRef> NamespaceDefinitionBaseProperty for T {
    fn idf(&self) -> WeakCell<Identifier> {
        self.namespace_definition_base_ref()
            .idf
            .as_ref()
            .unwrap()
            .downgrade()
    }
}

#[derive(ImplBaseTrait, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NamespaceDefinitionBase {
    pub ast_base: RcCell<ASTBase>,
    pub idf: Option<RcCell<Identifier>>,
}
impl NamespaceDefinitionBase {
    pub fn new(idf: Option<RcCell<Identifier>>) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            idf,
        }
    }
}
impl ASTChildren for NamespaceDefinitionBase {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf {
            cb.add_child(idf.clone().into());
        }
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
    pub parent: Option<ContractDefinition>,
    pub original_body: Option<RcCell<Block>>,
    pub annotated_type: Option<AnnotatedTypeName>,
    pub called_functions: BTreeSet<RcCell<ConstructorOrFunctionDefinition>>,
    pub is_recursive: bool,
    pub has_static_body: bool,
    pub can_be_private: bool,
    pub used_homomorphisms: Option<BTreeSet<String>>,
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
    pub requires_verification: bool,
    pub requires_verification_when_external: bool,
}
impl IntoAST for ConstructorOrFunctionDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(
            self.clone(),
        ))
    }
}

impl ConstructorOrFunctionDefinition {
    pub fn new(
        idf: Option<RcCell<Identifier>>,
        parameters: Option<Vec<RcCell<Parameter>>>,
        modifiers: Option<Vec<String>>,
        return_parameters: Option<Vec<RcCell<Parameter>>>,
        body: Option<Block>,
    ) -> Self {
        assert!(
            idf.is_some() && idf.as_ref().unwrap().borrow().name() != "constructor"
                || return_parameters.is_none()
        );
        let idf = idf.or(Some(RcCell::new(Identifier::Identifier(
            IdentifierBase::new(String::from("constructor")),
        ))));
        let return_parameters = if let Some(return_parameters) = return_parameters {
            return_parameters
        } else {
            vec![]
        };
        let mut return_var_decls: Vec<_> = return_parameters
            .iter()
            .enumerate()
            .map(|(idx, rp)| {
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    rp.borrow()
                        .identifier_declaration_base
                        .annotated_type
                        .clone(),
                    Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
                        format!("{}_{idx}", CFG.lock().unwrap().return_var_name()),
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
                .identifier_declaration_base
                .idf
                .as_mut()
                .unwrap()
                .borrow_mut()
                .ast_base_ref()
                .borrow_mut()
                .parent = Some(ASTFlatten::from(vd.clone()).downgrade());
        });
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf),
            parameters: parameters.clone().map_or(vec![], |p| p),
            modifiers: modifiers.clone().unwrap_or(vec![]),
            return_parameters: return_parameters.clone(),
            body: body.map(RcCell::new),
            return_var_decls,
            parent: None,
            original_body: None,
            annotated_type: Some(AnnotatedTypeName::new(
                Some(RcCell::new(TypeName::FunctionTypeName(
                    FunctionTypeName::new(
                        parameters.clone().unwrap_or(vec![]),
                        modifiers.unwrap_or(vec![]),
                        return_parameters,
                    ),
                ))),
                None,
                Homomorphism::non_homomorphic(),
            )),
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

    pub fn can_be_external(&self) -> bool
// return not ("private" in self.modifiers or "internal" in self.modifiers)
    {
        !(self.modifiers.contains(&String::from("private"))
            || self.modifiers.contains(&String::from("internal")))
    }

    pub fn is_external(&self) -> bool
// return "external" in self.modifiers
    {
        self.modifiers.contains(&String::from("external"))
    }

    pub fn is_payable(&self) -> bool
// return "payable" in self.modifiers
    {
        self.modifiers.contains(&String::from("payable"))
    }

    pub fn name(&self) -> String {
        self.namespace_definition_base
            .idf
            .as_ref()
            .unwrap()
            .borrow()
            .name()
            .clone()
    }

    pub fn return_type(&self) -> TupleType {
        TupleType::new(
            self.return_parameters
                .iter()
                .filter_map(|p| {
                    p.borrow()
                        .identifier_declaration_base
                        .annotated_type
                        .clone()
                })
                .collect(),
        )
    }
    // return TupleType([p.annotated_type for p in self.parameters])
    pub fn parameter_types(&self) -> TupleType {
        TupleType::new(
            self.parameters
                .iter()
                .filter_map(|p| {
                    p.borrow()
                        .identifier_declaration_base
                        .annotated_type
                        .clone()
                })
                .collect(),
        )
    }

    pub fn is_constructor(&self) -> bool {
        self.namespace_definition_base
            .idf
            .as_ref()
            .unwrap()
            .borrow()
            .name()
            .as_str()
            == "constructor"
    }

    pub fn is_function(&self) -> bool {
        !self.is_constructor()
    }

    pub fn _update_fct_type(&mut self) {
        self.annotated_type = Some(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::FunctionTypeName(
                FunctionTypeName::new(
                    self.parameters.clone(),
                    self.modifiers.clone(),
                    self.return_parameters.clone(),
                ),
            ))),
            None,
            Homomorphism::non_homomorphic(),
        ));
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
                t.try_as_type_name(),
                None,
                Homomorphism::non_homomorphic(),
            ))
            .into();
        };
        let idf = if let IdentifierExprUnion::String(idf) = idf {
            Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
                idf,
            ))))
        } else if let IdentifierExprUnion::Identifier(idf) = idf {
            Some(idf.clone())
        } else {
            None
        };
        let storage_loc = if t
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .borrow()
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

#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateVariableDeclaration {
    pub identifier_declaration_base: IdentifierDeclarationBase,
    pub expr: Option<ASTFlatten>,
}
impl IntoAST for StateVariableDeclaration {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::StateVariableDeclaration(
            self.clone(),
        ))
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
            cb.add_child(expr.clone().into());
        }
    }
}

#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumValue {
    pub ast_base: RcCell<ASTBase>,
    pub idf: Option<RcCell<Identifier>>,
    pub annotated_type: Option<AnnotatedTypeName>,
}
impl IntoAST for EnumValue {
    fn into_ast(self) -> AST {
        AST::EnumValue(self)
    }
}

impl EnumValue {
    pub fn new(idf: Option<Identifier>) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            idf: idf.map(|f| RcCell::new(f)),
            annotated_type: None,
        }
    }
    pub fn qualified_name(&self) -> Vec<Identifier> {
        vec![]
    }
    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
}
impl ASTChildren for EnumValue {
    fn process_children(&self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf {
            cb.add_child(idf.clone().into());
        }
    }
}

#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub values: Vec<RcCell<EnumValue>>,
    pub annotated_type: Option<AnnotatedTypeName>,
}
impl IntoAST for EnumDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::EnumDefinition(self))
    }
}

impl EnumDefinition {
    pub fn new(idf: Option<RcCell<Identifier>>, values: Vec<EnumValue>) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf),
            values: values.into_iter().map(RcCell::new).collect(),
            annotated_type: None,
        }
    }
    pub fn qualified_name(&self) -> Vec<Identifier> {
        vec![]
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

#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StructDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub members: Vec<ASTFlatten>,
}
impl IntoAST for StructDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::StructDefinition(self))
    }
}

impl StructDefinition {
    pub fn new(idf: Option<RcCell<Identifier>>, members: Vec<ASTFlatten>) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf),
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
            cb.add_child(member.clone().into());
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
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
}
impl IntoAST for ContractDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::ContractDefinition(self))
    }
}

impl ContractDefinition {
    pub fn new(
        idf: Option<RcCell<Identifier>>,
        state_variable_declarations: Vec<ASTFlatten>,
        constructor_definitions: Vec<ConstructorOrFunctionDefinition>,
        function_definitions: Vec<ConstructorOrFunctionDefinition>,
        enum_definitions: Vec<EnumDefinition>,
        struct_definitions: Option<Vec<StructDefinition>>,
        used_crypto_backends: Option<Vec<CryptoParams>>,
    ) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf),
            state_variable_declarations: state_variable_declarations.into_iter().collect(),
            constructor_definitions: constructor_definitions
                .into_iter()
                .map(RcCell::new)
                .collect(),
            function_definitions: function_definitions.into_iter().map(RcCell::new).collect(),
            enum_definitions: enum_definitions.into_iter().map(RcCell::new).collect(),
            struct_definitions: struct_definitions.map_or(vec![], |struct_definitions| {
                struct_definitions.into_iter().map(RcCell::new).collect()
            }),
            used_crypto_backends,
        }
    }
    pub fn get_item(&self, key: &String) -> Option<ASTFlatten> {
        // //println!("=======get_item============");
        if key == "constructor" {
            if self.constructor_definitions.len() == 0 {
                // # return empty constructor
                let mut c = ConstructorOrFunctionDefinition::new(None, None, None, None, None);
                c.ast_base_mut_ref().borrow_mut().parent =
                    Some(ASTFlatten::from(RcCell::new(self.clone())).downgrade());
                Some(RcCell::new(c).into())
            } else if self.constructor_definitions.len() == 1 {
                Some(self.constructor_definitions[0].clone().into())
            } else {
                // assert!(false,"Multiple constructors exist");
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
                cb.add_child(state_variable_declarations.clone().into());
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
impl IntoAST for SourceUnit {
    fn into_ast(self) -> AST {
        AST::SourceUnit(self)
    }
}

impl SourceUnit {
    pub fn new(
        pragma_directive: String,
        contracts: Vec<ContractDefinition>,
        used_contracts: Option<Vec<String>>,
    ) -> Self {
        Self {
            ast_base: RcCell::new(ASTBase::new()),
            pragma_directive,
            contracts: contracts.into_iter().map(RcCell::new).collect(),
            used_contracts: if let Some(used_contracts) = used_contracts {
                used_contracts
            } else {
                vec![]
            },
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
            .map(|c_identifier| {
                c_identifier
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .parent()
                    .as_ref()
                    .map(|p| p.clone().upgrade())
            })
            .flatten()
            .flatten()
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

impl ConstructorOrFunctionDefinitionAttr for AST {
    fn get_requires_verification_when_external(&self) -> bool {
        self.try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .map_or(false, |c| c.requires_verification_when_external)
    }
    fn get_name(&self) -> String {
        self.try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .map_or(String::new(), |c| c.name().clone())
    }
}
impl ConstructorOrFunctionDefinitionAttr for ASTFlatten {
    fn get_requires_verification_when_external(&self) -> bool {
        let v = self
            .try_as_ast_ref()
            .map(|a| {
                a.borrow()
                    .clone()
                    .try_as_namespace_definition()
                    .map(|a| a.try_as_constructor_or_function_definition())
            })
            .flatten()
            .flatten()
            .map_or(false, |c| c.requires_verification_when_external);
        v || self
            .try_as_constructor_or_function_definition_ref()
            .map_or(false, |c| c.borrow().requires_verification_when_external)
    }
    fn get_name(&self) -> String {
        let v = self
            .try_as_ast_ref()
            .map(|a| {
                a.borrow()
                    .clone()
                    .try_as_namespace_definition()
                    .map(|a| a.try_as_constructor_or_function_definition())
            })
            .flatten()
            .flatten()
            .map_or(String::new(), |c| c.name().clone());
        v + &self
            .try_as_constructor_or_function_definition_ref()
            .map_or(String::new(), |c| c.borrow().name().clone())
    }
}
// """Turn privacy label into expression (i.e. Identifier -> IdentifierExpr, Me and All stay the same)."""
pub fn get_privacy_expr_from_label(plabel: ASTFlatten) -> ASTFlatten {
    if plabel.is_identifier() {
        let mut ie = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(plabel.try_as_identifier_ref().unwrap().clone()),
            Some(AnnotatedTypeName::address_all()),
        );
        ie.location_expr_base.target = plabel.try_as_identifier_ref().unwrap().borrow().parent();
        RcCell::new(ie).into()
    } else {
        plabel
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct InstanceTarget {
    pub target_key: Vec<Option<ASTFlatten>>,
}
impl fmt::Display for InstanceTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.target_key)
    }
}
impl InstanceTarget {
    pub fn new(expr: Vec<Option<ASTFlatten>>) -> Self {
        let target_key = if expr.len() == 2 {
            expr
        } else {
            let v = expr[0].clone().unwrap();
            if is_instance(&v, ASTType::VariableDeclaration) {
                vec![expr[0].clone(), None]
            } else if is_instance(&v, ASTType::LocationExprBase) {
                let v = v
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .clone();
                match v.get_ast_type() {
                    ASTType::IdentifierExpr => {
                        vec![
                            v.location_expr_base_ref().target.clone().unwrap().upgrade(),
                            None,
                        ]
                    }

                    ASTType::MemberAccessExpr => vec![
                        v.location_expr_base_ref().target.clone().unwrap().upgrade(),
                        Some(
                            v.try_as_member_access_expr_ref()
                                .unwrap()
                                .member
                                .clone()
                                .into(),
                        ),
                    ],
                    ASTType::IndexExpr => vec![
                        v.location_expr_base_ref().target.clone().unwrap().upgrade(),
                        Some(v.try_as_index_expr_ref().unwrap().key.clone().into()),
                    ],
                    _ => vec![None; 2],
                }
            } else {
                vec![None; 2]
            }
        };
        assert!(is_instances(
            &target_key[0].clone().unwrap(),
            vec![
                ASTType::VariableDeclaration,
                ASTType::Parameter,
                ASTType::StateVariableDeclaration
            ]
        ));
        Self { target_key }
    }

    pub fn target(&self) -> Option<ASTFlatten> {
        if !self.target_key.is_empty() {
            self.target_key[0].clone()
        } else {
            None
        }
    }

    pub fn key(&self) -> Option<ASTFlatten> {
        if self.target_key.len() > 1 {
            self.target_key[1].clone()
        } else {
            None
        }
    }

    pub fn privacy(&self) -> Option<ASTFlatten> {
        if self.key().is_none()
            && !is_instance(
                self.target()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap(),
                ASTType::Mapping,
            )
        {
            self.target()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .zkay_type()
                .privacy_annotation
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
        } else {
            let t = self
                .target()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .zkay_type()
                .type_name
                .unwrap();
            assert!(is_instance(&t, ASTType::Mapping));

            if t.borrow().try_as_mapping_ref().unwrap().has_key_label() {
                self.key()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label()
                    .map(|x| x.clone().into())
            } else {
                t.borrow()
                    .try_as_mapping_ref()
                    .unwrap()
                    .value_type
                    .borrow()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label()
                    .map(|x| x.clone().into())
            }
        }
    }

    pub fn in_scope_at(&self, ast: &ASTFlatten) -> bool {
        crate::pointers::symbol_table::SymbolTableLinker::in_scope_at(
            self.target()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .borrow()
                .idf()
                .upgrade()
                .as_ref()
                .unwrap(),
            ast,
        )
    }
}
// // UTIL FUNCTIONS

pub fn indent(s: String) -> String {
    format!("{}{}", CFG.lock().unwrap().user_config.indentation(), s)
}

// // EXCEPTIONS

pub fn get_code_error_msg(
    line: i32,
    column: i32,
    code: Vec<String>,
    ctr: Option<ContractDefinition>,
    fct: Option<ConstructorOrFunctionDefinition>,
    stmt: Option<Statement>,
) -> String {
    // Print Location
    let mut error_msg = format!("At line: {line};{column}");

    // If error location is outside code bounds, only show line;col
    if line <= 0 || column <= 0 || line > code.len() as i32 {
        return error_msg;
    }

    if fct.is_some() {
        assert!(ctr.is_some());
        error_msg += ", in function \"{fct.name}\" of contract \"{ctr.idf.name}\"";
    } else if ctr.is_some() {
        error_msg += ", in contract \"{ctr.idf.name}\"";
    }
    error_msg += " ";

    let start_line = if let Some(stmt) = stmt {
        stmt.ast_base_ref().unwrap().borrow().line
    } else {
        line
    };
    if start_line != -1 {
        for line in start_line..line + 1 {
            // replace tabs with 4 spaces for consistent output
            let mut orig_line: String = code[line as usize - 1].clone();
            orig_line = orig_line.replace("\t", "    ");
            error_msg += &format!("{orig_line} ");
        }

        let affected_line = &code[line as usize - 1];
        let loc_string: String = affected_line[..column as usize - 1]
            .chars()
            .map(|c| (if c == '\t' { "----" } else { "-" }).to_string())
            .collect::<Vec<String>>()
            .concat();
        format!("{error_msg}{loc_string}/")
    } else {
        error_msg
    }
}

pub fn get_ast_exception_msg(ast: AST, msg: String) -> String {
    // Get surrounding statement
    let stmt = if let AST::Expression(ast) = &ast {
        ast.statement()
            .as_ref()
            .map(|p| p.clone().upgrade().map(|a| a.try_as_statement()))
            .flatten()
            .flatten()
            .map(|p| p.borrow().clone())
    } else if let AST::Statement(ast) = &ast {
        Some(ast.clone())
    } else {
        None
    };

    // Get surrounding function
    let fct = if let Some(stmt) = &stmt {
        stmt.statement_base_ref()
            .unwrap()
            .function
            .clone()
            .unwrap()
            .upgrade()
            .map(|f| {
                f.try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow()
                    .to_ast()
            })
    } else if is_instance(&ast, ASTType::ConstructorOrFunctionDefinition) {
        Some(ast.clone())
    } else {
        None
    };

    // Get surrounding contract
    let mut ctr = fct.clone().or(Some(ast.clone()));
    while ctr.is_some() && !is_instance(&ctr.clone().unwrap(), ASTType::ContractDefinition) {
        if let Some(p) = ctr
            .clone()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .parent()
            .as_ref()
        {
            ctr = p
                .clone()
                .upgrade()
                .map(|p| p.try_as_ast())
                .flatten()
                .map(|p| p.borrow().clone());
        } else {
            break;
        }
    }

    // Get source root
    let mut root = ctr.clone().or(Some(ast.clone()));
    while root.is_some() && !is_instance(root.as_ref().unwrap(), ASTType::SourceUnit) {
        root = root
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .parent()
            .as_ref()
            .map(|p| p.clone().upgrade())
            .flatten()
            .map(|p| p.try_as_ast())
            .flatten()
            .map(|p| p.borrow().clone());
    }

    let error_msg = if root.is_none() {
        String::from("error")
    } else {
        get_code_error_msg(
            ast.ast_base_ref().unwrap().borrow().line,
            ast.ast_base_ref().unwrap().borrow().column,
            root.unwrap()
                .try_as_source_unit_ref()
                .unwrap()
                .original_code
                .clone(),
            ctr.unwrap()
                .try_as_namespace_definition()
                .unwrap()
                .try_as_contract_definition(),
            fct.clone().map(|f| {
                f.try_as_namespace_definition()
                    .unwrap()
                    .try_as_constructor_or_function_definition()
                    .unwrap()
            }),
            stmt.clone().map(|s| s),
        )
    };

    format!(" {error_msg}  {msg}")
}

pub fn issue_compiler_warning(ast: AST, warning_type: String, msg: String) {
    if CFG.lock().unwrap().is_unit_test() {
        return;
    }
    warn_print();
    zk_print!(
        " \nWARNING: {warning_type}{}",
        get_ast_exception_msg(ast, msg)
    );
}
pub struct AstException(pub String);
impl AstException {
    pub fn new(ast: AST, msg: String) -> Self {
        Self(get_ast_exception_msg(ast, msg))
    }
}

// // CODE GENERATION
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ListUnion {
    AST(ASTFlatten),
    String(String),
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SingleOrListUnion {
    Vec(Vec<ListUnion>),
    AST(ASTFlatten),
    String(String),
}
#[derive(ASTVisitorBaseRefImpl)]
pub struct CodeVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub display_final: bool,
}

impl AstVisitor for CodeVisitor {
    type Return = String;
    fn temper_result(&self) -> Self::Return {
        String::new()
    }
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::ASTBase
                | ASTType::CommentBase
                | ASTType::IdentifierBase
                | ASTType::FunctionCallExprBase
                | ASTType::PrimitiveCastExpr
                | ASTType::BooleanLiteralExpr
                | ASTType::NumberLiteralExpr
                | ASTType::StringLiteralExpr
                | ASTType::ArrayLiteralExprBase
                | ASTType::TupleExpr
                | ASTType::IdentifierExpr
                | ASTType::MemberAccessExpr
                | ASTType::IndexExpr
                | ASTType::MeExpr
                | ASTType::AllExpr
                | ASTType::ReclassifyExpr
                | ASTType::RehomExpr
                | ASTType::IfStatement
                | ASTType::WhileStatement
                | ASTType::DoWhileStatement
                | ASTType::ForStatement
                | ASTType::BreakStatement
                | ASTType::ContinueStatement
                | ASTType::ReturnStatement
                | ASTType::ExpressionStatement
                | ASTType::RequireStatement
                | ASTType::AssignmentStatementBase
                | ASTType::CircuitDirectiveStatementBase
                | ASTType::StatementListBase
                | ASTType::Block
                | ASTType::IndentBlock
                | ASTType::ElementaryTypeNameBase
                | ASTType::UserDefinedTypeNameBase
                | ASTType::AddressTypeName
                | ASTType::AddressPayableTypeName
                | ASTType::AnnotatedTypeName
                | ASTType::Mapping
                | ASTType::ArrayBase
                | ASTType::CipherText
                | ASTType::TupleType
                | ASTType::VariableDeclaration
                | ASTType::VariableDeclarationStatement
                | ASTType::Parameter
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::EnumValue
                | ASTType::EnumDefinition
                | ASTType::StructDefinition
                | ASTType::StateVariableDeclaration
                | ASTType::ContractDefinition
                | ASTType::SourceUnit
        ) || matches!(ast, AST::Comment(_))
            || matches!(ast, AST::Identifier(_))
            || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(
                ast,
                AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(_)))
            )
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Statement(Statement::CircuitDirectiveStatement(_)))
            || matches!(ast, AST::Statement(Statement::StatementList(_)))
            || matches!(ast, AST::TypeName(TypeName::UserDefinedTypeName(_)))
            || matches!(ast, AST::TypeName(TypeName::Array(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::ASTBase => self.visit_AST(ast),
            _ if matches!(ast.to_ast(), AST::Comment(_)) => self.visit_Comment(ast),
            _ if matches!(ast.to_ast(), AST::Identifier(_)) => self.visit_Identifier(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visit_FunctionCallExpr(ast)
            }
            ASTType::PrimitiveCastExpr => self.visit_PrimitiveCastExpr(ast),
            ASTType::BooleanLiteralExpr => self.visit_BooleanLiteralExpr(ast),
            ASTType::NumberLiteralExpr => self.visit_NumberLiteralExpr(ast),
            ASTType::StringLiteralExpr => self.visit_StringLiteralExpr(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(_)))
            ) =>
            {
                self.visit_ArrayLiteralExpr(ast)
            }
            ASTType::TupleExpr => self.visit_TupleExpr(ast),
            ASTType::IdentifierExpr => self.visit_IdentifierExpr(ast),
            ASTType::MemberAccessExpr => self.visit_MemberAccessExpr(ast),
            ASTType::IndexExpr => self.visit_IndexExpr(ast),
            ASTType::MeExpr => self.visit_MeExpr(ast),
            ASTType::AllExpr => self.visit_AllExpr(ast),
            ASTType::ReclassifyExpr => self.visit_ReclassifyExpr(ast),
            ASTType::RehomExpr => self.visit_RehomExpr(ast),
            ASTType::IfStatement => self.visit_IfStatement(ast),
            ASTType::WhileStatement => self.visit_WhileStatement(ast),
            ASTType::DoWhileStatement => self.visit_DoWhileStatement(ast),
            ASTType::ForStatement => self.visit_ForStatement(ast),
            ASTType::BreakStatement => self.visit_BreakStatement(ast),
            ASTType::ContinueStatement => self.visit_ContinueStatement(ast),
            ASTType::ReturnStatement => self.visit_ReturnStatement(ast),
            ASTType::ExpressionStatement => self.visit_ExpressionStatement(ast),
            ASTType::RequireStatement => self.visit_RequireStatement(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visit_AssignmentStatement(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::CircuitDirectiveStatement(_))
            ) =>
            {
                self.visit_CircuitDirectiveStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.visit_StatementList(ast)
            }
            ASTType::Block => self.visit_Block(ast),
            ASTType::IndentBlock => self.visit_IndentBlock(ast),
            _ if matches!(ast.to_ast(), AST::TypeName(TypeName::ElementaryTypeName(_))) => {
                self.visit_ElementaryTypeName(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::TypeName(TypeName::UserDefinedTypeName(_))
            ) =>
            {
                self.visit_UserDefinedTypeName(ast)
            }
            ASTType::AddressTypeName => self.visit_AddressTypeName(ast),
            ASTType::AddressPayableTypeName => self.visit_AddressPayableTypeName(ast),
            ASTType::AnnotatedTypeName => self.visit_AnnotatedTypeName(ast),
            ASTType::Mapping => self.visit_Mapping(ast),
            _ if matches!(ast.to_ast(), AST::TypeName(TypeName::Array(_))) => self.visit_Array(ast),
            ASTType::CipherText => self.visit_CipherText(ast),
            ASTType::TupleType => self.visit_TupleType(ast),
            ASTType::VariableDeclaration => self.visit_VariableDeclaration(ast),
            ASTType::VariableDeclarationStatement => self.visit_VariableDeclarationStatement(ast),
            ASTType::Parameter => self.visit_Parameter(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visit_ConstructorOrFunctionDefinition(ast)
            }
            ASTType::EnumValue => self.visit_EnumValue(ast),
            ASTType::EnumDefinition => self.visit_EnumDefinition(ast),
            ASTType::StructDefinition => self.visit_StructDefinition(ast),
            ASTType::StateVariableDeclaration => self.visit_StateVariableDeclaration(ast),
            ASTType::ContractDefinition => self.visit_ContractDefinition(ast),
            ASTType::SourceUnit => self.visit_SourceUnit(ast),
            _ => Ok(String::new()),
        }
    }
}
impl CodeVisitor {
    pub fn new(display_final: bool) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            display_final,
        }
    }

    pub fn visit_list(
        &self,
        l: Vec<ListUnion>,
        mut sep: &str,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if sep.is_empty() {
            sep = "\n";
        }
        if l.is_empty() {
            return Ok(String::new());
        }

        fn handle(selfs: &CodeVisitor, e: &ListUnion) -> String {
            match e {
                ListUnion::String(e) => e.to_owned(),
                ListUnion::AST(e) => selfs.visit(e),
            }
        }

        let s: Vec<_> = l.iter().map(|e| handle(self, e)).collect();
        let s = s.join(sep);
        Ok(s)
    }

    pub fn visit_single_or_list(
        &self,
        v: SingleOrListUnion,
        mut sep: &str,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if sep.is_empty() {
            sep = "\n";
        }
        match v {
            SingleOrListUnion::Vec(v) => self.visit_list(v, sep),
            SingleOrListUnion::String(v) => Ok(v),
            SingleOrListUnion::AST(v) => Ok(self.visit(&v)),
        }
    }

    pub fn visit_AST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // should never be called
        // raise NotImplementedError("Did not implement code generation for " + repr(ast))
        // unimplemented!("Did not implement code generation for {:?} ", ast);
        // //println!("=======visit_AST==============");

        Err(eyre!("Did not implement code generation for {:?} ", ast))
    }
    pub fn visit_Comment(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if ast.try_as_comment_ref().unwrap().borrow().text() == String::new() {
                String::new()
            } else if ast
                .try_as_comment_ref()
                .unwrap()
                .borrow()
                .text()
                .contains(" ")
            {
                format!(
                    "/* {} */",
                    ast.try_as_comment_ref().unwrap().borrow().text()
                )
            } else {
                format!("// {}", ast.try_as_comment_ref().unwrap().borrow().text())
            },
        )
    }

    pub fn visit_Identifier(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(ast.try_as_identifier_ref().unwrap().borrow().name().clone())
    }

    pub fn visit_FunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::BuiltinFunction,
            ) {
                let args: Vec<_> = ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()
                    .iter()
                    .map(|a| self.visit(&a.clone().into()))
                    .collect();
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func()
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .format_string(&args)
            } else {
                let f = self.visit(
                    &ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .func()
                        .clone()
                        .into(),
                );
                let a = self.visit_list(
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .args()
                        .iter()
                        .map(|arg| ListUnion::AST(arg.clone().into()))
                        .collect(),
                    ", ",
                )?;
                format!("{f}({a})")
            },
        )
    }

    pub fn visit_PrimitiveCastExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if ast
                .try_as_primitive_cast_expr_ref()
                .unwrap()
                .borrow()
                .is_implicit
            {
                self.visit(
                    &ast.try_as_primitive_cast_expr_ref()
                        .unwrap()
                        .borrow()
                        .expr
                        .clone()
                        .into(),
                )
            } else {
                format!(
                    "{}({})",
                    self.visit(
                        &ast.try_as_primitive_cast_expr_ref()
                            .unwrap()
                            .borrow()
                            .elem_type
                            .clone()
                            .into()
                    ),
                    self.visit(
                        &ast.try_as_primitive_cast_expr_ref()
                            .unwrap()
                            .borrow()
                            .expr
                            .clone()
                            .into()
                    )
                )
            },
        )
    }

    pub fn visit_BooleanLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(ast
            .try_as_boolean_literal_expr_ref()
            .unwrap()
            .borrow()
            .value
            .to_string()
            .to_ascii_lowercase())
    }

    pub fn visit_NumberLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("======visit_NumberLiteralExpr==============={:?}",ast);
        Ok(
            if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_literal_expr_ref()
                .unwrap()
                .try_as_number_literal_expr_ref()
                .unwrap()
                .was_hex
            {
                format!(
                    "{:x}",
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap()
                        .value
                )
            } else {
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_literal_expr_ref()
                    .unwrap()
                    .try_as_number_literal_expr_ref()
                    .unwrap()
                    .value
                    .to_string()
            },
        )
    }

    pub fn visit_StringLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "\"{}\"",
            ast.try_as_string_literal_expr_ref().unwrap().borrow().value
        ))
    }

    pub fn visit_ArrayLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "[{}]",
            self.visit_list(
                ast.try_as_array_literal_expr_ref()
                    .unwrap()
                    .borrow()
                    .values()
                    .iter()
                    .map(|value| ListUnion::AST(value.clone().into()))
                    .collect(),
                ", "
            )?
        ))
    }

    pub fn visit_TupleExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "({})",
            self.visit_list(
                ast.try_as_tuple_expr_ref()
                    .unwrap()
                    .borrow()
                    .elements
                    .iter()
                    .map(|element| ListUnion::AST(element.clone().into()))
                    .collect(),
                ", "
            )?
        ))
    }

    pub fn visit_IdentifierExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("======visit_IdentifierExpr========={:?}", ast);
        Ok(self.visit(
            &ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        ))
    }

    pub fn visit_MemberAccessExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "{}.{}",
            self.visit(
                &ast.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            ),
            self.visit(
                &ast.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow()
                    .member
                    .clone()
                    .into()
            )
        ))
    }

    pub fn visit_IndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("=======visit_IndexExpr================={:?}",ast);
        Ok(format!(
            "{}[{}]",
            self.visit(
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_index_expr_ref()
                    .unwrap()
                    .arr
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            ),
            self.visit(
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_index_expr_ref()
                    .unwrap()
                    .key
                    .clone()
                    .into()
            )
        ))
    }

    pub fn visit_MeExpr(&self, _: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("me"))
    }

    pub fn visit_AllExpr(&self, _: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("all"))
    }

    pub fn visit_ReclassifyExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let e = self.visit(
            &ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .expr()
                .clone()
                .into(),
        );
        let p = self.visit(
            &ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .privacy()
                .clone()
                .into(),
        );
        let h = HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .get(
                ast.try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism()
                    .as_ref()
                    .unwrap(),
            )
            .unwrap()
            .clone();
        Ok(format!("reveal{h:?}({e}, {p})"))
    }

    pub fn visit_RehomExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let e = self.visit(
            &ast.try_as_rehom_expr_ref()
                .unwrap()
                .borrow()
                .reclassify_expr_base
                .expr
                .clone()
                .into(),
        );
        Ok(format!(
            "{}({e})",
            ast.try_as_rehom_expr_ref().unwrap().borrow().func_name()
        ))
    }

    pub fn visit_IfStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_IfStatement=========={:?}",ast);
        let c = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .condition
                .clone()
                .into(),
        );
        let t = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_if_statement_ref()
                    .unwrap()
                    .then_branch
                    .clone()
                    .into(),
            ),
            "",
        )?;
        let mut ret = format!("if ({c}) {t}");
        if let Some(else_branch) = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_if_statement_ref()
            .unwrap()
            .else_branch
        {
            let e =
                self.visit_single_or_list(SingleOrListUnion::AST(else_branch.clone().into()), "")?;
            ret += format!("\n else {e}").as_str();
        }
        Ok(ret)
    }

    pub fn visit_WhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let c = self.visit(
            &ast.try_as_while_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into(),
        );
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.try_as_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .body
                    .clone()
                    .into(),
            ),
            "",
        )?;
        Ok(format!("while ({c}) {b}"))
    }

    pub fn visit_DoWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.try_as_do_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .body
                    .clone()
                    .into(),
            ),
            "",
        )?;
        let c = self.visit(
            &ast.try_as_do_while_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into(),
        );
        Ok(format!("do {b} while ({c});"))
    }

    pub fn visit_ForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let i = if let Some(init) = &ast.try_as_for_statement_ref().unwrap().borrow().init {
            format!(
                "{}",
                self.visit_single_or_list(SingleOrListUnion::AST(init.clone().into()), "")?
            )
        } else {
            String::from(";")
        };
        let c = self.visit(
            &ast.try_as_for_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into(),
        );
        let u = if let Some(update) = &ast.try_as_for_statement_ref().unwrap().borrow().update {
            format!(
                " {}",
                self.visit_single_or_list(SingleOrListUnion::AST(update.clone().into()), "")?
                    .replace(";", "")
            )
        } else {
            String::new()
        };
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.try_as_for_statement_ref()
                    .unwrap()
                    .borrow()
                    .body
                    .clone()
                    .into(),
            ),
            "",
        )?;
        Ok(format!("for ({i} {c};{u}) {b}"))
    }

    pub fn visit_BreakStatement(
        &self,
        _: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("break;"))
    }

    pub fn visit_ContinueStatement(
        &self,
        _: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("continue;"))
    }

    pub fn visit_ReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if ast
                .try_as_return_statement_ref()
                .unwrap()
                .borrow()
                .expr
                .is_none()
            {
                String::from("return;")
            } else {
                let e = self.visit(
                    &ast.try_as_return_statement_ref()
                        .unwrap()
                        .borrow()
                        .expr
                        .as_ref()
                        .unwrap()
                        .clone()
                        .into(),
                );
                format!("return {e};")
            },
        )
    }

    pub fn visit_ExpressionStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        //  println!("===visit_ExpressionStatement=========={:?}",ast);
        Ok(self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_expression_statement_ref()
                .unwrap()
                .expr
                .clone()
                .into(),
        ) + ";")
    }

    pub fn visit_RequireStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("====visit_RequireStatement=========={:?}", ast);
        let c = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_require_statement_ref()
                .unwrap()
                .condition
                .clone()
                .into(),
        );
        Ok(format!("require({c});"))
    }

    pub fn visit_AssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("====visit_AssignmentStatement=========={:?}", ast.is_ast());
        let ast = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .clone();
        let lhs = ast.lhs().clone();
        let mut op = ast.op().clone();
        if lhs
            .as_ref()
            .unwrap()
            .to_ast()
            .try_as_expression_ref()
            .as_ref()
            .map_or(false, |asu| {
                asu.annotated_type()
                    .as_ref()
                    .map_or(false, |at| at.borrow().is_private())
            })
        {
            op = String::new();
        }

        let rhs = if !op.is_empty() {
            ast.rhs().clone().map(|fce| {
                // println!("=====fce==========={:?}=====",fce);
                fce.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()[1]
                    .clone()
            })
        } else {
            ast.rhs().clone()
        };

        let fstr = if op.starts_with("pre") {
            op = op[3..].to_string();
            "{1}{0};"
        } else if op.starts_with("post") {
            op = op[4..].to_string();
            "{0}{1};"
        } else {
            "{} {}= {};"
        };
        let format_string = |ls, rs| match fstr {
            "{1}{0};" => format!("{1}{0};", ls, rs),
            "{0}{1};" => format!("{0}{1};", ls, rs),
            _ => format!("{} {}= {};", ls, op, rs),
        };
        Ok(
            if is_instance(lhs.as_ref().unwrap(), ASTType::SliceExpr)
                && is_instance(rhs.as_ref().unwrap(), ASTType::SliceExpr)
            {
                let (lhs, rhs) = (
                    lhs.as_ref()
                        .unwrap()
                        .try_as_ast_ref()
                        .unwrap()
                        .borrow()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_slice_expr_ref()
                        .unwrap()
                        .clone(),
                    rhs.as_ref()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_slice_expr_ref()
                        .unwrap()
                        .clone(),
                );
                assert!(lhs.size == rhs.size, "Slice ranges don't have same size");
                let mut s = String::new();
                let (lexpr, rexpr) = (
                    self.visit(&lhs.arr.as_ref().unwrap().clone().into()),
                    self.visit(&rhs.arr.as_ref().unwrap().clone().into()),
                );
                let mut lbase = if let Some(base) = &lhs.base {
                    format!("{} + ", self.visit(&base.clone().into()))
                } else {
                    String::new()
                };
                let mut rbase = if let Some(base) = &rhs.base {
                    format!("{} + ", self.visit(&base.clone().into()))
                } else {
                    String::new()
                };
                if lhs.size <= 3 {
                    for i in 0..lhs.size {
                        s += &format_string(
                            format!("{lexpr}[{lbase}{}]", lhs.start_offset + i),
                            format!("{rexpr}[{rbase}{}]", rhs.start_offset + i),
                        );
                        s += "\n";
                    }
                } else {
                    let i = CFG.lock().unwrap().reserved_name_prefix() + "i";
                    if lhs.start_offset != 0 {
                        lbase += &format!("{} + ", lhs.start_offset);
                    }
                    if rhs.start_offset != 0 {
                        rbase += &format!("{} + ", rhs.start_offset);
                    }
                    s += format!("for (uint {i} = 0; {i} < {}; ++{i}) {{\n", lhs.size).as_str();
                    s += &indent(format_string(
                        format!("{lexpr}[{lbase}{i}]"),
                        format!("{rexpr}[{rbase}{i}]"),
                    ));
                    s += "\n";
                    s += "}\n";
                }
                s[..s.len() - 1].to_string()
            } else {
                let to_ast = |hs| self.visit(hs);
                format_string(
                    to_ast(&lhs.as_ref().unwrap().clone().into()),
                    self.visit(&rhs.clone().unwrap().into()),
                )
            },
        )
    }
    pub fn visit_CircuitDirectiveStatement(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::new())
    }

    fn handle_block(&self, ast: &StatementList) -> eyre::Result<<Self as AstVisitor>::Return> {
        match ast {
            StatementList::Block(block) => Ok(indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone_inner()))
                        .collect(),
                    "",
                )?,
            )),
            StatementList::IndentBlock(block) => Ok(indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone_inner()))
                        .collect(),
                    "",
                )?,
            )),
            _ => Err(eyre::eyre!("unreach ")),
        }
    }

    pub fn visit_StatementList(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("======visit_StatementList==============={:?}",ast);
        match &*ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_statement_list_ref()
            .unwrap()
        {
            StatementList::Block(block) => Ok(indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone_inner()))
                        .collect(),
                    "",
                )?,
            )),
            StatementList::IndentBlock(block) => Ok(indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone_inner()))
                        .collect(),
                    "",
                )?,
            )),
            _ => Err(eyre::eyre!("unreach ")),
        }
    }

    pub fn visit_Block(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let b = self
            .handle_block(&StatementList::Block(
                ast.try_as_block_ref().unwrap().borrow().clone(),
            ))?
            .trim_end()
            .to_string();
        Ok(
            if ast
                .try_as_block_ref()
                .unwrap()
                .borrow()
                .was_single_statement
                && ast
                    .try_as_block_ref()
                    .unwrap()
                    .borrow()
                    .statement_list_base
                    .statements
                    .len()
                    == 1
            {
                b
            } else {
                format!("{{ {b} }}")
            },
        )
    }

    pub fn visit_IndentBlock(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.handle_block(
            ast.try_as_indent_block_ref()
                .unwrap()
                .borrow()
                .to_statement()
                .try_as_statement_list_ref()
                .unwrap(),
        )
    }

    pub fn visit_ElementaryTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(ast
            .try_as_elementary_type_name_ref()
            .unwrap()
            .borrow()
            .name()
            .clone())
    }

    pub fn visit_UserDefinedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let names: Vec<_> = ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_user_defined_type_name_ref()
            .unwrap()
            .user_defined_type_name_base_ref()
            .names
            .iter()
            .map(|name| ListUnion::AST(name.clone().into()))
            .collect();
        self.visit_list(names, ".")
    }

    pub fn visit_AddressTypeName(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("address"))
    }

    pub fn visit_AddressPayableTypeName(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("address payable"))
    }

    pub fn visit_AnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let t = self.visit(
            &ast.try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        );
        let p = if let Some(privacy_annotation) = &ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .privacy_annotation
        {
            self.visit(&privacy_annotation.clone().into())
        } else {
            String::new()
        };

        Ok(
            if ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .had_privacy_annotation
            {
                format!(
                    "{t}@{p}{:?}",
                    HOMOMORPHISM_STORE
                        .lock()
                        .unwrap()
                        .get(
                            &ast.try_as_annotated_type_name_ref()
                                .unwrap()
                                .borrow()
                                .homomorphism
                        )
                        .unwrap()
                )
            } else {
                t
            },
        )
    }

    pub fn visit_Mapping(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("=========visit_Mapping================={:?}",ast);
        let k = self.visit(
            &ast.try_as_type_name_ref()
                .unwrap()
                .borrow()
                .try_as_mapping_ref()
                .unwrap()
                .key_type
                .clone()
                .into(),
        );
        let label = if let Some(idf) = &ast
            .try_as_type_name_ref()
            .unwrap()
            .borrow()
            .try_as_mapping_ref()
            .unwrap()
            .key_label
        {
            format!("!{}", self.visit(&idf.clone().into()))
        } else {
            if let Some(Identifier::HybridArgumentIdf(key_label)) = &ast
                .try_as_type_name_ref()
                .unwrap()
                .borrow()
                .try_as_mapping_ref()
                .unwrap()
                .key_label
                .as_ref()
                .map(|kl| (*kl.borrow()).clone())
            {
                format!("/*!{:?}*/", key_label)
            } else {
                String::new()
            }
        };
        let v = self.visit(
            &ast.try_as_type_name_ref()
                .unwrap()
                .borrow()
                .try_as_mapping_ref()
                .unwrap()
                .value_type
                .clone()
                .into(),
        );
        Ok(format!("mapping({k}{label} => {v})"))
    }

    pub fn visit_Array(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let value_type = ast
            .try_as_array_ref()
            .unwrap()
            .borrow()
            .value_type()
            .clone();
        let expr = ast.try_as_array_ref().unwrap().borrow().expr().clone();
        let t = self.visit(&value_type.clone().into());
        let e = expr.clone().map_or(String::new(), |_expr| {
            self.visit(&expr.clone().unwrap().into())
        });
        Ok(format!("{t}[{e}]"))
    }

    pub fn visit_CipherText(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let e = self.visit_Array(&ast.try_as_cipher_text_ref().unwrap().clone().into())?;
        Ok(format!(
            "{e}/*{}*/",
            ast.try_as_cipher_text_ref()
                .unwrap()
                .borrow()
                .plain_type
                .borrow()
                .to_ast()
                .code()
        ))
    }

    pub fn visit_TupleType(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let s = self.visit_list(
            ast.try_as_tuple_type_ref()
                .unwrap()
                .borrow()
                .types
                .iter()
                .map(|typ| ListUnion::AST(typ.clone().into()))
                .collect(),
            ", ",
        )?;
        Ok(format!("({s})"))
    }

    pub fn visit_VariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let keywords: Vec<_> = ast
            .try_as_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .filter_map(|k| {
                if self.display_final || k != "final" {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();
        let k = keywords.join(" ");
        let t = self.visit(
            &ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .clone()
                .unwrap()
                .into(),
        );
        let s = if let Some(storage_location) = &ast
            .try_as_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .storage_location
        {
            format!(" {storage_location}")
        } else {
            String::new()
        };
        let i = self.visit(
            &ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        );
        Ok(format!("{k} {t}{s} {i}").trim().to_string())
    }

    pub fn visit_VariableDeclarationStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_VariableDeclarationStatement=========={:?}", ast);
        let mut s = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_variable_declaration_statement_ref()
                .unwrap()
                .variable_declaration
                .clone()
                .into(),
        );
        if let Some(expr) = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .expr
        {
            s += format!(" = {}", self.visit(&expr.clone().into())).as_str();
        }
        s += ";";
        Ok(s)
    }

    pub fn visit_Parameter(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let final_string = String::from("final");
        let f = if !self.display_final {
            None
        } else {
            if ast
                .try_as_parameter_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .keywords
                .contains(&final_string)
            {
                Some(final_string)
            } else {
                None
            }
        };
        let t = self.visit(
            &ast.try_as_parameter_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .clone()
                .unwrap()
                .into(),
        );
        let i = self.visit(
            &ast.try_as_parameter_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        );
        let description: Vec<_> = [
            f,
            Some(t),
            ast.try_as_parameter_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .storage_location
                .clone(),
            Some(i),
        ]
        .iter()
        .filter_map(|d| d.clone())
        .collect();
        Ok(description.join(" "))
    }

    pub fn visit_ConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let b = if let Some(body) = &ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .body
        {
            self.visit_single_or_list(SingleOrListUnion::AST(body.clone().into()), "")?
        } else {
            String::new()
        };
        self.function_definition_to_str(
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .namespace_definition_base
                .idf
                .as_ref()
                .unwrap(),
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .parameters
                .iter()
                .map(|parameter| ParameterUnion::Parameter(parameter.clone()))
                .collect(),
            &ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .modifiers,
            &ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .return_parameters,
            &b,
        )
    }
    fn function_definition_to_str(
        &self,
        idf: &RcCell<Identifier>,
        parameters: Vec<ParameterUnion>,
        modifiers: &Vec<String>,
        return_parameters: &Vec<RcCell<Parameter>>,
        body: &String,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let definition = if idf.borrow().name() != "constructor" {
            let i = self.visit(&idf.clone().into());
            format!("function {i}")
        } else {
            String::from("constructor")
        };
        // //println!("{:?}", parameters);
        let p = self.visit_list(
            parameters
                .iter()
                .filter_map(|parameter| match parameter {
                    ParameterUnion::Parameter(p) => Some(ListUnion::AST(p.clone().into())),
                    ParameterUnion::String(s) => Some(ListUnion::String(s.clone())),
                })
                .collect(),
            ", ",
        )?;
        let mut m = modifiers.clone().join(" ");
        if !m.is_empty() {
            m = format!(" {m}");
        }
        let mut r = self.visit_list(
            return_parameters
                .iter()
                .map(|p| ListUnion::AST(p.clone().into()))
                .collect(),
            ", ",
        )?;
        if !r.is_empty() {
            r = format!(" returns ({r})");
        }

        Ok(format!("{definition}({p}){m}{r} {body}"))
    }

    pub fn visit_EnumValue(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if let Some(idf) = &ast.try_as_enum_value_ref().unwrap().borrow().idf {
                self.visit(&idf.clone().into())
            } else {
                String::new()
            },
        )
    }

    pub fn visit_EnumDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let values = indent(
            self.visit_list(
                ast.try_as_enum_definition_ref()
                    .unwrap()
                    .borrow()
                    .values
                    .iter()
                    .map(|value| ListUnion::AST(value.clone().into()))
                    .collect(),
                ", ",
            )?,
        );
        Ok(format!(
            "enum {} {{\n{values}\n}}",
            self.visit(
                &ast.try_as_enum_definition_ref()
                    .unwrap()
                    .borrow()
                    .namespace_definition_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            )
        ))
    }

    // @staticmethod
    fn __cmp_type_size(v1: &ASTFlatten, v2: &ASTFlatten) -> Ordering {
        match (
            v1.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone(),
            v2.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone(),
        ) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(t1), Some(t2)) => {
                let cmp = t1
                    .borrow()
                    .size_in_uints()
                    .cmp(&t2.borrow().size_in_uints());
                if cmp == Ordering::Equal {
                    t1.borrow()
                        .elem_bitwidth()
                        .cmp(&t2.borrow().elem_bitwidth())
                } else {
                    cmp
                }
            }
        }
    }

    pub fn visit_StructDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // Define struct with members in order of descending size (to get maximum space savings through packing)
        let mut members_by_descending_size = ast
            .try_as_struct_definition_ref()
            .unwrap()
            .borrow()
            .members
            .clone();
        members_by_descending_size.sort_by(|v1, v2| Self::__cmp_type_size(v1, v2).reverse());
        let body = indent(
            members_by_descending_size
                .iter()
                .map(|member| self.visit(&member.clone().into()))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        Ok(format!(
            "struct {} {{\n{body}\n}}",
            self.visit(
                &ast.try_as_struct_definition_ref()
                    .unwrap()
                    .borrow()
                    .namespace_definition_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            )
        ))
    }

    pub fn visit_StateVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let final_string = String::from("final");

        let keywords: Vec<_> = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .cloned()
            .filter_map(|k| {
                if self.display_final || &k != &final_string {
                    Some(k)
                } else {
                    None
                }
            })
            .collect();
        let f = if keywords.contains(&&final_string) {
            final_string.clone()
        } else {
            String::new()
        };
        let t = self.visit(
            &ast.try_as_state_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .clone()
                .unwrap()
                .into(),
        );
        let mut k = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .filter_map(|k| {
                if k != &final_string {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !k.is_empty() {
            k = format!("{k} ");
        }
        let i = self.visit(
            &ast.try_as_state_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        );
        let mut ret = format!("{f}{t} {k}{i}").trim().to_string();
        if let Some(expr) = &ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .expr
        {
            ret += &format!(" = {}", self.visit(&expr.clone().into()));
        }
        Ok(ret + ";")
    }

    fn contract_definition_to_str(
        idf: Identifier,
        state_vars: Vec<String>,
        constructors: Vec<String>,
        functions: Vec<String>,
        enums: Vec<String>,
        structs: Vec<String>,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let i = idf.to_string();
        let structs = structs.join("\n\n");
        let enums = enums.join("\n\n");
        let state_vars = state_vars.join("\n\n");
        let constructors = constructors.join("\n\n");
        let functions = functions.join("\n\n");
        let mut body = [structs, enums, state_vars, constructors, functions]
            .into_iter()
            .filter_map(|s| if !s.is_empty() { Some(s) } else { None })
            .collect::<Vec<_>>()
            .join("\n\n");
        body = indent(body);
        Ok(format!("contract {i} {{\n{body}\n}}"))
    }

    pub fn visit_ContractDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let state_vars = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .state_variable_declarations
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[ for e in ast.state_variable_declarations]
        let constructors = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .constructor_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.constructor_definitions]
        let functions = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .function_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.function_definitions]
        let enums = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .enum_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.enum_definitions]
        let structs = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .struct_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.struct_definitions]

        Self::contract_definition_to_str(
            ast.try_as_contract_definition_ref()
                .unwrap()
                .borrow()
                .namespace_definition_base
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .clone(),
            state_vars,
            constructors,
            functions,
            enums,
            structs,
        )
    }

    fn handle_pragma(&self, pragma: String) -> String {
        pragma
    }

    pub fn visit_SourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let p = self.handle_pragma(
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .pragma_directive
                .clone(),
        );
        let contracts = self.visit_list(
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .contracts
                .iter()
                .map(|contract| ListUnion::AST(contract.clone().into()))
                .collect(),
            "",
        )?;
        let lfstr = |uc| format!("import \"{}\";", uc);
        //  "\n\n".join(filter("".__ne__, [p, linesep.join([lfstr.format(uc) for uc in ast.used_contracts]), contracts]))
        Ok([
            p,
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .used_contracts
                .iter()
                .map(|uc| lfstr(uc))
                .collect::<Vec<_>>()
                .join(LINE_ENDING),
            contracts,
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n"))
    }
}
