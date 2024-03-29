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
use crate::circuit_constraints::CircuitStatement;
use crate::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex},
};
use strum_macros::{EnumIs, EnumTryAs};
use zkay_config::{
    config::{ConstructorOrFunctionDefinitionAttr, CFG},
    zk_print,
};
use zkay_crypto::params::CryptoParams;
use zkay_derive::{
    impl_trait, impl_traits, ASTChildrenImpl, ASTDebug, ASTKind, ASTVisitorBaseRefImpl,
    ImplBaseTrait,
};
use zkay_utils::progress_printer::warn_print;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct RRWrapper<T: Hash + PartialEq + Eq + Ord + Clone + Debug>(Arc<RefCell<T>>);

impl<T: Hash + PartialEq + Eq + Ord + Clone + Debug> Deref for RRWrapper<T> {
    type Target = Arc<RefCell<T>>;
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
        RRWrapper(Arc::new(RefCell::new(inner)))
    }
}

pub struct ChildListBuilder {
    pub children: Vec<AST>,
}
impl ChildListBuilder {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
    pub fn add_child(&mut self, ast: AST) {
        self.children.push(ast.clone());
    }
}

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
pub trait Immutable {
    fn is_immutable(&self) -> bool;
}

pub fn is_instance<T: ASTInstanceOf>(var: &T, ast_type: ASTType) -> bool {
    var.get_ast_type() == ast_type
}
pub fn is_instances<T: ASTInstanceOf>(var: &T, ast_types: Vec<ASTType>) -> bool {
    ast_types.iter().any(|t| t == &var.get_ast_type())
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
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
#[enum_dispatch]
pub trait ASTChildren {
    fn children(&mut self) -> Vec<AST> {
        let mut cb = ChildListBuilder::new();
        self.process_children(&mut cb);
        cb.children.drain(..).collect()
    }

    fn process_children(&mut self, cb: &mut ChildListBuilder);
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
    fn qualified_name(&mut self) -> Vec<Identifier> {
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
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
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
    pub fn ast_base_ref(&self) -> Option<&ASTBase> {
        impl_base_ref!(ast_base_ref, self)
    }
    pub fn ast_base_mut_ref(&mut self) -> Option<&mut ASTBase> {
        impl_base_ref!(ast_base_mut_ref, self)
    }

    pub fn text(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&self).unwrap()
    }
    pub fn code(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&self).unwrap()
    }
    pub fn is_parent_of(&self, child: &AST) -> bool {
        let mut e = child.clone();
        let selfs = self.clone();
        while e != selfs
            && e.ast_base_ref()
                .unwrap()
                .parent_namespace
                .as_ref()
                .unwrap()
                .borrow()
                .parent
                .is_some()
        {
            let e1 = e
                .ast_base_ref()
                .unwrap()
                .parent_namespace
                .as_ref()
                .unwrap()
                .borrow()
                .parent
                .as_ref()
                .map(|p| *p.clone())
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
                Some(ASTType::ReclassifyExprBase)
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
    fn ast_base_ref(&self) -> &ASTBase;
}
pub trait ASTBaseProperty {
    fn parent(&self) -> Option<Box<AST>>;
    fn namespace(&self) -> Option<Vec<Identifier>>;
    fn names(&self) -> &BTreeMap<String, Identifier>;
    fn line(&self) -> i32;
    fn column(&self) -> i32;
    fn modified_values(&self) -> &BTreeSet<InstanceTarget>;
    fn read_values(&self) -> &BTreeSet<InstanceTarget>;
}
impl<T: ASTBaseRef> ASTBaseProperty for T {
    fn parent(&self) -> Option<Box<AST>> {
        self.ast_base_ref()
            .parent_namespace
            .as_ref()
            .unwrap()
            .borrow()
            .parent
            .clone()
    }
    fn namespace(&self) -> Option<Vec<Identifier>> {
        self.ast_base_ref()
            .parent_namespace
            .as_ref()
            .unwrap()
            .borrow()
            .namespace
            .clone()
    }
    fn names(&self) -> &BTreeMap<String, Identifier> {
        &self.ast_base_ref().names
    }
    fn line(&self) -> i32 {
        self.ast_base_ref().line
    }
    fn column(&self) -> i32 {
        self.ast_base_ref().column
    }
    fn modified_values(&self) -> &BTreeSet<InstanceTarget> {
        &self.ast_base_ref().modified_values
    }
    fn read_values(&self) -> &BTreeSet<InstanceTarget> {
        &self.ast_base_ref().read_values
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ASTParentNamespace {
    pub parent: Option<Box<AST>>,
    pub namespace: Option<Vec<Identifier>>,
}
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct ASTBase {
    #[serde(skip)]
    pub parent_namespace: Option<RRWrapper<ASTParentNamespace>>,
    pub names: BTreeMap<String, Identifier>,
    pub line: i32,
    pub column: i32,
    pub modified_values: BTreeSet<InstanceTarget>,
    pub read_values: BTreeSet<InstanceTarget>,
}
impl ASTBase {
    pub fn new() -> Self {
        Self {
            parent_namespace: Some(RRWrapper::new(ASTParentNamespace {
                parent: None,
                namespace: None,
            })),
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
    fn name(&self) -> &String;
}
impl<T: IdentifierBaseRef> IdentifierBaseProperty for T {
    fn name(&self) -> &String {
        &self.identifier_base_ref().name
    }
}

#[derive(
    ImplBaseTrait,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct IdentifierBase {
    pub ast_base: ASTBase,
    pub name: String,
}
impl IntoAST for IdentifierBase {
    fn into_ast(self) -> AST {
        AST::Identifier(Identifier::Identifier(self))
    }
}
impl ASTBaseMutRef for IdentifierBase {
    fn ast_base_mut_ref(&mut self) -> &mut ASTBase {
        &mut self.ast_base
    }
}
impl IdentifierBase {
    pub fn new(name: String) -> Self {
        Self {
            ast_base: ASTBase::new(),
            name,
        }
    }

    pub fn decl_var(&self, t: AST, expr: Option<Expression>) -> Statement {
        let t = match t {
            AST::TypeName(t) => Some(AnnotatedTypeName::new(
                Some(t),
                None,
                Homomorphism::non_homomorphic(),
            )),
            AST::AnnotatedTypeName(t) => Some(t),
            _ => None,
        };
        assert!(t.is_some());
        let t = t.unwrap();
        let storage_loc = if t.type_name.as_ref().unwrap().is_primitive_type() {
            String::new()
        } else {
            String::from("memory")
        };
        VariableDeclarationStatement::new(
            VariableDeclaration::new(
                vec![],
                t,
                Identifier::Identifier(self.clone()),
                Some(storage_loc),
            ),
            expr,
        )
        .to_statement()
    }
}
impl Immutable for IdentifierBase {
    fn is_immutable(&self) -> bool {
        if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::StateVariableDeclaration(
            svd,
        ))) = &self.parent().as_ref().map(|p| *p.clone())
        {
            svd.identifier_declaration_base.is_final()
                || svd.identifier_declaration_base.is_constant()
        } else {
            false
        }
    }
}
impl fmt::Display for IdentifierBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[enum_dispatch(IntoAST, ASTInstanceOf, CommentBaseRef, ASTBaseRef, ASTBaseMutRef)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum Comment {
    Comment(CommentBase),
    BlankLine(BlankLine),
}
impl Comment {
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct CommentBase {
    pub ast_base: ASTBase,
    pub text: String,
}
impl IntoAST for CommentBase {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::Comment(self))
    }
}
impl ASTBaseMutRef for CommentBase {
    fn ast_base_mut_ref(&mut self) -> &mut ASTBase {
        &mut self.ast_base
    }
}
impl CommentBase {
    pub fn new(text: String) -> Self {
        Self {
            ast_base: ASTBase::new(),
            text,
        }
    }
    pub fn comment_list(text: String, block: Vec<AST>) -> Vec<AST> {
        if !block.is_empty() {
            block
        } else {
            [AST::Comment(Comment::Comment(CommentBase::new(text)))]
                .into_iter()
                .chain(block)
                .chain([AST::Comment(Comment::BlankLine(BlankLine::new()))])
                .collect()
        }
    }

    pub fn comment_wrap_block(text: String, block: Vec<AST>) -> Vec<AST> {
        if !block.is_empty() {
            block
        } else {
            vec![
                AST::Comment(Comment::Comment(CommentBase::new(text))),
                AST::Comment(Comment::Comment(CommentBase::new(String::from("{")))),
                AST::Statement(Statement::StatementList(StatementList::IndentBlock(
                    IndentBlock::new(
                        block
                            .into_iter()
                            .filter_map(|b| {
                                if let AST::Statement(_) = b {
                                    Some(b)
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ),
                ))),
                AST::Comment(Comment::Comment(CommentBase::new(String::from("}")))),
                AST::Comment(Comment::BlankLine(BlankLine::new())),
            ]
        }
    }
}

#[impl_traits(CommentBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    pub fn me_expr(statement: Option<Box<Statement>>) -> Self {
        let mut me_expr = MeExpr::new();
        me_expr.expression_base.statement = statement;
        Expression::MeExpr(me_expr)
    }
    pub fn explicitly_converted(&self, expected: TypeName) -> AST {
        let mut ret;
        if expected == TypeName::bool_type() && !self.instanceof_data_type(&TypeName::bool_type()) {
            ret = Some(FunctionCallExprBase::new(
                Expression::BuiltinFunction(BuiltinFunction::new("!=")),
                vec![
                    self.clone(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                        NumberLiteralExpr::new(0, false),
                    )),
                ],
                None,
            ));
        } else if expected.is_numeric() && self.instanceof_data_type(&TypeName::bool_type()) {
            ret = Some(FunctionCallExprBase::new(
                Expression::BuiltinFunction(BuiltinFunction::new("ite")),
                vec![
                    self.clone(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                        NumberLiteralExpr::new(1, false),
                    )),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                        NumberLiteralExpr::new(0, false),
                    )),
                ],
                None,
            ));
        } else {
            let t = self
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .unwrap();

            if &**t == &expected {
                return self.clone().into_ast();
            }

            // Explicit casts
            let cast = is_instance(&**t, ASTType::NumberTypeNameBase)
                && is_instances(
                    &expected,
                    vec![
                        ASTType::NumberTypeNameBase,
                        ASTType::AddressTypeName,
                        ASTType::AddressPayableTypeName,
                        ASTType::EnumTypeName,
                    ],
                )
                || is_instance(&**t, ASTType::AddressTypeName)
                    && is_instance(&expected, ASTType::NumberTypeNameBase)
                || is_instance(&**t, ASTType::AddressPayableTypeName)
                    && is_instances(
                        &expected,
                        vec![ASTType::NumberTypeNameBase, ASTType::AddressTypeName],
                    )
                || is_instance(&**t, ASTType::EnumTypeName)
                    && is_instance(&expected, ASTType::NumberTypeNameBase);
            assert!(cast);
            return Expression::PrimitiveCastExpr(PrimitiveCastExpr::new(
                expected.clone(),
                self.clone(),
                false,
            ))
            .as_type(AST::TypeName(expected))
            .into_ast();
        }
        assert!(ret.is_some());
        let mut ret = ret.unwrap();
        ret.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            Some(expected.clone()),
            if let Some(privacy_annotation) =
                &self.annotated_type().as_ref().unwrap().privacy_annotation
            {
                Some(*privacy_annotation.clone())
            } else {
                None
            },
            self.annotated_type().as_ref().unwrap().homomorphism.clone(),
        ));
        ret.into_ast()
    }

    pub fn privacy_annotation_label(&self) -> Option<AST> {
        if let Some(ie) = self
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_identifier_expr_ref()
        {
            if let Some(target) = ie.target() {
                if let Some(mapping) = target.try_as_type_name_ref().unwrap().try_as_mapping_ref() {
                    return mapping
                        .instantiated_key
                        .as_ref()
                        .unwrap()
                        .privacy_annotation_label();
                }
                if let Some(id) = target.try_as_identifier_declaration_ref() {
                    return Some(id.idf().to_ast());
                }
                if let Some(id) = target.try_as_namespace_definition_ref() {
                    return Some(id.idf().to_ast());
                }
            }
        }

        if self.is_all_expr() || self.is_me_expr() {
            Some(AST::Expression(self.clone()))
        } else {
            None
        }
    }
    pub fn instanceof_data_type(&self, expected: &TypeName) -> bool {
        self.annotated_type()
            .as_ref()
            .unwrap()
            .type_name
            .as_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
    }
    pub fn unop(&self, op: String) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            Expression::BuiltinFunction(BuiltinFunction::new(&op)),
            vec![self.clone()],
            None,
        ))
    }

    pub fn binop(&self, op: String, rhs: Expression) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            Expression::BuiltinFunction(BuiltinFunction::new(&op)),
            vec![self.clone(), rhs],
            None,
        ))
    }

    pub fn ite(&self, e_true: Expression, e_false: Expression) -> FunctionCallExpr {
        let mut bf = BuiltinFunction::new("ite");
        bf.is_private = self.annotated_type().as_ref().unwrap().is_private();
        FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
            Expression::BuiltinFunction(bf),
            vec![self.clone(), e_true, e_false],
            None,
        ))
    }

    pub fn instance_of(&self, expected: &AnnotatedTypeName) -> Option<String> {
        // """
        // :param expected:
        // :return: true, false, or "make-private"
        // """
        // assert! (isinstance(expected, AnnotatedTypeName))

        let actual = self.annotated_type();

        if !self.instanceof_data_type(&*expected.type_name.as_ref().unwrap()) {
            return Some(String::from("false"));
        }

        // check privacy type and homomorphism
        let combined_label = actual
            .as_ref()
            .unwrap()
            .combined_privacy(self.analysis(), expected.clone());
        if let Some(combined_label) = combined_label {
            if let CombinedPrivacyUnion::Vec(combined_label) = combined_label {
                assert!(
                    if let TypeName::TupleType(_) = **self
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap()
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
                            .type_name
                            .as_ref()
                            .unwrap()
                            .try_as_tuple_type_ref()
                            .unwrap()
                            .types
                            .iter()
                            .map(|t| {
                                CombinedPrivacyUnion::AST(
                                    t.privacy_annotation.as_ref().map(|pa| *pa.clone()),
                                )
                            })
                            .collect::<Vec<_>>())
                    .to_string(),
                )
            } else if combined_label
                .clone()
                .as_expression()
                .unwrap()
                .privacy_annotation_label()
                == actual
                    .as_ref()
                    .unwrap()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
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

    pub fn as_type(&self, t: AST) -> Self {
        match self {
            Expression::BuiltinFunction(ast) => Expression::BuiltinFunction(ast.as_type(t)),
            Expression::FunctionCallExpr(ast) => Expression::FunctionCallExpr(ast.as_type(t)),
            Expression::PrimitiveCastExpr(ast) => Expression::PrimitiveCastExpr(ast.as_type(t)),
            Expression::LiteralExpr(ast) => Expression::LiteralExpr(ast.as_type(t)),
            Expression::TupleOrLocationExpr(ast) => Expression::TupleOrLocationExpr(ast.as_type(t)),
            Expression::MeExpr(ast) => Expression::MeExpr(ast.as_type(t)),
            Expression::AllExpr(ast) => Expression::AllExpr(ast.as_type(t)),
            Expression::ReclassifyExpr(ast) => Expression::ReclassifyExpr(ast.as_type(t)),
            Expression::DummyAnnotation(ast) => Expression::DummyAnnotation(ast.clone()),
        }
    }

    pub fn analysis(&self) -> Option<PartitionState<AST>> {
        if let Some(statement) = self.statement() {
            statement
                .statement_base_ref()
                .unwrap()
                .before_analysis()
                .clone()
        } else {
            None
        }
    }
}
#[enum_dispatch]
pub trait ExpressionBaseRef: ASTBaseRef {
    fn expression_base_ref(&self) -> &ExpressionBase;
}
pub trait ExpressionBaseProperty {
    fn annotated_type(&self) -> &Option<AnnotatedTypeName>;
    fn statement(&self) -> &Option<Box<Statement>>;
    fn evaluate_privately(&self) -> bool;
}
impl<T: ExpressionBaseRef> ExpressionBaseProperty for T {
    fn annotated_type(&self) -> &Option<AnnotatedTypeName> {
        &self.expression_base_ref().annotated_type
    }
    fn statement(&self) -> &Option<Box<Statement>> {
        &self.expression_base_ref().statement
    }
    fn evaluate_privately(&self) -> bool {
        self.expression_base_ref().evaluate_privately
    }
}
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct ExpressionBase {
    pub ast_base: ASTBase,
    pub annotated_type: Option<AnnotatedTypeName>,
    pub statement: Option<Box<Statement>>,
    pub evaluate_privately: bool,
}
impl ExpressionBase {
    pub fn new() -> Self {
        Self {
            ast_base: ASTBase::new(),
            annotated_type: None,
            statement: None,
            evaluate_privately: false,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub rerand_using: Option<Box<IdentifierExpr>>,
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
    pub fn input_types(&self) -> Vec<Option<TypeName>> {
        // :return: None if the type is generic
        let t = if self.is_arithmetic() {
            Some(TypeName::number_type())
        } else if self.is_comp() {
            Some(TypeName::number_type())
        } else if self.is_bop() {
            Some(TypeName::bool_type())
        } else if self.is_bitop() {
            Some(TypeName::number_type())
        } else if self.is_shiftop() {
            Some(TypeName::number_type())
        } else
        // eq, parenthesis, ite
        {
            None
        };

        vec![t; self.arity() as usize]
    }
    pub fn output_type(&self) -> Option<TypeName> {
        // :return: None if the type is generic
        if self.is_arithmetic() {
            Some(TypeName::number_type())
        } else if self.is_comp() {
            Some(TypeName::bool_type())
        } else if self.is_bop() {
            Some(TypeName::bool_type())
        } else if self.is_eq() {
            Some(TypeName::bool_type())
        } else if self.is_bitop() {
            Some(TypeName::number_type())
        } else if self.is_shiftop() {
            Some(TypeName::number_type())
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
        args: &Vec<Expression>,
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

        let arg_types: Vec<_> = args.iter().map(|x| x.annotated_type()).collect();
        let inaccessible_arg_types: Vec<_> = arg_types
            .iter()
            .filter(|x| !x.as_ref().unwrap().is_accessible(&analysis))
            .collect();
        if inaccessible_arg_types.is_empty()
        // Else we would not have selected a homomorphic operation
        {
            // raise ValueError("Cannot select proper homomorphic function if all arguments are public or @me-private")
            assert!(false,"Cannot select proper homomorphic function if all arguments are public or @me-private");
        }
        let elem_type = *arg_types
            .iter()
            .map(|a| a.as_ref().unwrap().type_name.clone().unwrap())
            .reduce(|l, r| Box::new(l.combined_type(*r, true).unwrap()))
            .unwrap();
        let base_type = AnnotatedTypeName::new(
            Some(elem_type),
            inaccessible_arg_types[0]
                .as_ref()
                .unwrap()
                .privacy_annotation
                .as_ref()
                .map(|pr| *pr.clone()),
            Homomorphism::non_homomorphic(),
        );
        let public_args: Vec<_> = arg_types
            .iter()
            .map(|a| a.as_ref().unwrap().is_public())
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
        let isinstance = |arg: &Expression| {
            if let Expression::ReclassifyExpr(_) = arg {
                true
            } else {
                false
            }
        };
        if self.op == "*"
            && !args[0]
                .clone()
                .annotated_type()
                .as_ref()
                .unwrap()
                .is_accessible(&analysis)
            && !args[1]
                .clone()
                .annotated_type()
                .as_ref()
                .unwrap()
                .is_accessible(&analysis)
            && (isinstance(&args[0]) && !isinstance(&args[1]))
            || (isinstance(&args[1]) && !isinstance(&args[0]))
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
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}

//     Describes the required input types and the resulting output type of a homomorphic execution of a BuiltinFunction.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HomomorphicBuiltinFunction {
    pub target_type: AnnotatedTypeName,
    pub public_args: Vec<bool>,
}
impl HomomorphicBuiltinFunction {
    pub fn new(target_type: AnnotatedTypeName, public_args: Vec<bool>) -> Self {
        Self {
            target_type,
            public_args,
        }
    }
    pub fn input_types(&self) -> Vec<AnnotatedTypeName> {
        let public_type = AnnotatedTypeName::all(*self.target_type.type_name.clone().unwrap());
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
    pub fn output_type(&self) -> AnnotatedTypeName {
        self.target_type.clone()
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    FunctionCallExprBaseRef,
    FunctionCallExprBaseMutRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum FunctionCallExpr {
    FunctionCallExpr(FunctionCallExprBase),
    NewExpr(NewExpr),
}

impl FunctionCallExpr {
    pub fn is_cast(&self) -> bool {
        // isinstance(self.func, LocationExpr) && isinstance(self.func.target, (ContractDefinition, EnumDefinition))
        is_instance(&**self.func(), ASTType::LocationExprBase)
            && is_instances(
                &**self
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap(),
                vec![ASTType::ContractDefinition, ASTType::EnumDefinition],
            )
    }
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            FunctionCallExpr::FunctionCallExpr(ast) => {
                FunctionCallExpr::FunctionCallExpr(ast.as_type(t))
            }
            FunctionCallExpr::NewExpr(ast) => FunctionCallExpr::NewExpr(ast.as_type(t)),
        }
    }
}

#[enum_dispatch]
pub trait FunctionCallExprBaseRef: ExpressionBaseRef {
    fn function_call_expr_base_ref(&self) -> &FunctionCallExprBase;
}
pub trait FunctionCallExprBaseProperty {
    fn func(&self) -> &Box<Expression>;
    fn args(&self) -> &Vec<Expression>;
    fn sec_start_offset(&self) -> &Option<i32>;
    fn public_key(&self) -> &Option<Box<HybridArgumentIdf>>;
}
impl<T: FunctionCallExprBaseRef> FunctionCallExprBaseProperty for T {
    fn func(&self) -> &Box<Expression> {
        &self.function_call_expr_base_ref().func
    }
    fn args(&self) -> &Vec<Expression> {
        &self.function_call_expr_base_ref().args
    }
    fn sec_start_offset(&self) -> &Option<i32> {
        &self.function_call_expr_base_ref().sec_start_offset
    }
    fn public_key(&self) -> &Option<Box<HybridArgumentIdf>> {
        &self.function_call_expr_base_ref().public_key
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ImplBaseTrait,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct FunctionCallExprBase {
    pub expression_base: ExpressionBase,
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub sec_start_offset: Option<i32>,
    pub public_key: Option<Box<HybridArgumentIdf>>,
}

impl IntoAST for FunctionCallExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(
            FunctionCallExpr::FunctionCallExpr(self),
        ))
    }
}

impl FunctionCallExprBase {
    pub fn new(func: Expression, args: Vec<Expression>, sec_start_offset: Option<i32>) -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            func: Box::new(func),
            args,
            sec_start_offset,
            public_key: None,
        }
    }

    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}

impl ASTChildren for FunctionCallExprBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.func.to_ast());
        self.args.iter().for_each(|arg| {
            cb.add_child(arg.to_ast());
        });
    }
}
#[impl_traits(FunctionCallExprBase, ExpressionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NewExpr {
    pub function_call_expr_base: FunctionCallExprBase,
    pub annotated_type: AnnotatedTypeName,
}
impl IntoAST for NewExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::NewExpr(
            self,
        )))
    }
}

impl NewExpr {
    pub fn new(annotated_type: AnnotatedTypeName, args: Vec<Expression>) -> Self {
        // assert not isinstance(annotated_type, ElementaryTypeName)
        Self {
            function_call_expr_base: FunctionCallExprBase::new(
                Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                    LocationExpr::IdentifierExpr(IdentifierExpr::new(
                        IdentifierExprUnion::String(format!(
                            "new {}",
                            annotated_type.to_ast().code()
                        )),
                        None,
                    )),
                )),
                args,
                None,
            ),
            annotated_type,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.function_call_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.function_call_expr_base.expression_base.annotated_type = Some(
                AnnotatedTypeName::new(Some(tn), None, Homomorphism::non_homomorphic()),
            );
        }

        selfs
    }
}
impl ASTChildren for NewExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.annotated_type.to_ast());
        self.function_call_expr_base.args.iter().for_each(|arg| {
            cb.add_child(arg.to_ast());
        });
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PrimitiveCastExpr {
    pub expression_base: ExpressionBase,
    pub elem_type: Box<TypeName>,
    pub expr: Box<Expression>,
    pub is_implicit: bool,
}
impl IntoAST for PrimitiveCastExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::PrimitiveCastExpr(self))
    }
}

impl PrimitiveCastExpr {
    pub fn new(elem_type: TypeName, expr: Expression, is_implicit: bool) -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            elem_type: Box::new(elem_type),
            expr: Box::new(expr),
            is_implicit,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl ASTChildren for PrimitiveCastExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.elem_type.to_ast());
        cb.add_child(self.expr.to_ast());
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    LiteralExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum LiteralExpr {
    BooleanLiteralExpr(BooleanLiteralExpr),
    NumberLiteralExpr(NumberLiteralExpr),
    StringLiteralExpr(StringLiteralExpr),
    ArrayLiteralExpr(ArrayLiteralExpr),
}
impl LiteralExpr {
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            LiteralExpr::BooleanLiteralExpr(ast) => LiteralExpr::BooleanLiteralExpr(ast.as_type(t)),
            LiteralExpr::NumberLiteralExpr(ast) => LiteralExpr::NumberLiteralExpr(ast.as_type(t)),
            LiteralExpr::StringLiteralExpr(ast) => LiteralExpr::StringLiteralExpr(ast.as_type(t)),
            LiteralExpr::ArrayLiteralExpr(ast) => LiteralExpr::ArrayLiteralExpr(ast.as_type(t)),
        }
    }
}
#[enum_dispatch]
pub trait LiteralExprBaseRef: ExpressionBaseRef {
    fn literal_expr_base_ref(&self) -> &LiteralExprBase;
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
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
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct BooleanLiteralExpr {
    pub literal_expr_base: LiteralExprBase,
    pub value: bool,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
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
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                BooleanLiteralType::new(value).into_ast().try_as_type_name(),
                None,
                Homomorphism::non_homomorphic(),
            ))),
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
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
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                NumberLiteralType::new(NumberLiteralTypeUnion::I32(value))
                    .into_ast()
                    .try_as_type_name(),
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
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                NumberLiteralType::new(NumberLiteralTypeUnion::String(value_string))
                    .into_ast()
                    .try_as_type_name(),
                None,
                Homomorphism::non_homomorphic(),
            ))),
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
    pub fn value(&self) -> i32 {
        0
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    ArrayLiteralExprBaseRef,
    LiteralExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum ArrayLiteralExpr {
    ArrayLiteralExpr(ArrayLiteralExprBase),
    KeyLiteralExpr(KeyLiteralExpr),
}
impl ArrayLiteralExpr {
    pub fn values(&self) -> Vec<Expression> {
        vec![]
    }
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            ArrayLiteralExpr::ArrayLiteralExpr(ast) => {
                ArrayLiteralExpr::ArrayLiteralExpr(ast.as_type(t))
            }
            ArrayLiteralExpr::KeyLiteralExpr(ast) => {
                ArrayLiteralExpr::KeyLiteralExpr(ast.as_type(t))
            }
        }
    }
}
#[enum_dispatch]
pub trait ArrayLiteralExprBaseRef: LiteralExprBaseRef {
    fn array_literal_expr_base_ref(&self) -> &ArrayLiteralExprBase;
}

pub trait ArrayLiteralExprBaseProperty {
    fn values(&self) -> &Vec<Expression>;
}
impl<T: ArrayLiteralExprBaseRef> ArrayLiteralExprBaseProperty for T {
    fn values(&self) -> &Vec<Expression> {
        &self.array_literal_expr_base_ref().values
    }
}
#[impl_traits(LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ImplBaseTrait,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct ArrayLiteralExprBase {
    pub literal_expr_base: LiteralExprBase,
    pub values: Vec<Expression>,
}
impl IntoAST for ArrayLiteralExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(
            ArrayLiteralExpr::ArrayLiteralExpr(self),
        )))
    }
}

impl ArrayLiteralExprBase {
    pub fn new(values: Vec<Expression>) -> Self {
        Self {
            literal_expr_base: LiteralExprBase::new(),
            values,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl ASTChildren for ArrayLiteralExprBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.values.iter().for_each(|value| {
            cb.add_child(value.to_ast());
        });
    }
}

#[impl_traits(ArrayLiteralExprBase, LiteralExprBase, ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn new(values: Vec<Expression>, crypto_params: CryptoParams) -> Self {
        Self {
            array_literal_expr_base: ArrayLiteralExprBase::new(values),
            crypto_params,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs
                .array_literal_expr_base
                .literal_expr_base
                .expression_base
                .annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs
                .array_literal_expr_base
                .literal_expr_base
                .expression_base
                .annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    TupleOrLocationExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum TupleOrLocationExpr {
    TupleExpr(TupleExpr),
    LocationExpr(LocationExpr),
}

impl TupleOrLocationExpr {
    pub fn is_lvalue(&self) -> bool {
        let parent = match self {
            TupleOrLocationExpr::TupleExpr(te) => te.parent().clone().map(|p| *p),
            TupleOrLocationExpr::LocationExpr(te) => te.parent().as_ref().map(|p| *p.clone()),
        };
        if let Some(AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(a)),
        ))) = &parent
        {
            return self
                == a.lhs
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap();
        }
        if let Some(AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IndexExpr(ie)),
        ))) = &parent
        {
            if &self
                == &ie
                    .arr
                    .clone()
                    .unwrap()
                    .into_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
            {
                return parent
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .is_lvalue();
            }
        }
        if let Some(AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::MemberAccessExpr(ie)),
        ))) = &parent
        {
            if &self
                == &ie
                    .expr
                    .clone()
                    .unwrap()
                    .into_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
            {
                return parent
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .is_lvalue();
            }
        }
        if let Some(AST::Expression(Expression::TupleOrLocationExpr(parent))) = &parent {
            return parent.is_lvalue();
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
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            TupleOrLocationExpr::TupleExpr(ast) => TupleOrLocationExpr::TupleExpr(ast.as_type(t)),
            TupleOrLocationExpr::LocationExpr(ast) => {
                TupleOrLocationExpr::LocationExpr(ast.as_type(t))
            }
        }
    }
}
#[enum_dispatch]
pub trait TupleOrLocationExprBaseRef: ExpressionBaseRef {
    fn tuple_or_location_expr_base_ref(&self) -> &TupleOrLocationExprBase;
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
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
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleExpr {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub elements: Vec<Expression>,
}

impl IntoAST for TupleExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::TupleExpr(self),
        ))
    }
}

impl TupleExpr {
    pub fn new(elements: Vec<Expression>) -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(),
            elements,
        }
    }
    pub fn assign(&self, val: Expression) -> AssignmentStatement {
        AssignmentStatement::AssignmentStatement(AssignmentStatementBase::new(
            Some(self.to_ast()),
            Some(val),
            None,
        ))
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl ASTChildren for TupleExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.elements.iter().for_each(|element| {
            cb.add_child(element.to_ast());
        });
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    LocationExprBaseRef,
    TupleOrLocationExprBaseRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub enum LocationExpr {
    IdentifierExpr(IdentifierExpr),
    MemberAccessExpr(MemberAccessExpr),
    IndexExpr(IndexExpr),
    SliceExpr(SliceExpr),
}

impl LocationExpr {
    pub fn location_expr_base(&self) -> Option<LocationExprBase> {
        None
    }
    pub fn member_access_expr(&self) -> Option<MemberAccessExpr> {
        None
    }
    pub fn index_expr(&self) -> Option<IndexExpr> {
        None
    }
    pub fn ast_base_mut(&mut self) -> &mut ASTBase {
        match self {
            LocationExpr::IdentifierExpr(ast) => {
                &mut ast
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .ast_base
            }
            LocationExpr::MemberAccessExpr(ast) => {
                &mut ast
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .ast_base
            }
            LocationExpr::IndexExpr(ast) => {
                &mut ast
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .ast_base
            }
            LocationExpr::SliceExpr(ast) => {
                &mut ast
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .ast_base
            }
        }
    }

    pub fn call(&self, member: IdentifierExprUnion, args: Vec<Expression>) -> FunctionCallExpr {
        FunctionCallExpr::FunctionCallExpr(match member {
            IdentifierExprUnion::Identifier(member) => FunctionCallExprBase::new(
                Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                    LocationExpr::MemberAccessExpr(MemberAccessExpr::new(
                        Some(self.clone()),
                        member,
                    )),
                )),
                args,
                None,
            ),
            IdentifierExprUnion::String(member) => FunctionCallExprBase::new(
                Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                    LocationExpr::MemberAccessExpr(MemberAccessExpr::new(
                        Some(self.clone()),
                        Identifier::Identifier(IdentifierBase::new(member)),
                    )),
                )),
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
                MemberAccessExpr::new(Some(self.clone()), member)
            }
            IdentifierExprUnion::String(member) => MemberAccessExpr::new(
                Some(self.clone()),
                Identifier::Identifier(IdentifierBase::new(member)),
            ),
        }
    }

    pub fn index(&self, item: ExprUnion) -> IndexExpr {
        let type_name = match self {
            LocationExpr::IdentifierExpr(ie) => {
                if let Some(annotated_type) = &ie.annotated_type {
                    Some(annotated_type.type_name.clone())
                } else {
                    None
                }
            }
            LocationExpr::MemberAccessExpr(mae) => {
                if let Some(annotated_type) = &mae
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .annotated_type
                {
                    Some(annotated_type.type_name.clone())
                } else {
                    None
                }
            }
            LocationExpr::IndexExpr(ie) => {
                if let Some(annotated_type) = &ie
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .annotated_type
                {
                    Some(annotated_type.type_name.clone())
                } else {
                    None
                }
            }
            LocationExpr::SliceExpr(se) => {
                if let Some(annotated_type) = &se
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .annotated_type
                {
                    Some(annotated_type.type_name.clone())
                } else {
                    None
                }
            }
        };
        let value_type = if let Some(type_name) = type_name {
            match *type_name.unwrap() {
                TypeName::Array(a) => Some(a.value_type().to_ast()),
                TypeName::Mapping(a) => Some(AST::AnnotatedTypeName(*a.value_type)),
                _ => None,
            }
        } else {
            None
        };
        assert!(value_type.is_some());
        let item = match item {
            ExprUnion::I32(item) => Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
                NumberLiteralExpr::new(item, false),
            )),
            ExprUnion::Expression(item) => item,
        };

        IndexExpr::new(Some(self.clone()), item).as_type(value_type.unwrap())
    }
    pub fn assign(&self, val: Expression) -> AssignmentStatement {
        AssignmentStatement::AssignmentStatement(AssignmentStatementBase::new(
            Some(self.clone().into_ast()),
            Some(val),
            None,
        ))
    }
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            LocationExpr::IdentifierExpr(ast) => LocationExpr::IdentifierExpr(ast.as_type(t)),
            LocationExpr::MemberAccessExpr(ast) => LocationExpr::MemberAccessExpr(ast.as_type(t)),
            LocationExpr::IndexExpr(ast) => LocationExpr::IndexExpr(ast.as_type(t)),
            LocationExpr::SliceExpr(ast) => LocationExpr::SliceExpr(ast.as_type(t)),
        }
    }
}
#[enum_dispatch]
pub trait LocationExprBaseRef: TupleOrLocationExprBaseRef {
    fn location_expr_base_ref(&self) -> &LocationExprBase;
}
pub trait LocationExprBaseProperty {
    fn target(&self) -> &Option<Box<AST>>;
}
impl<T: LocationExprBaseRef> LocationExprBaseProperty for T {
    fn target(&self) -> &Option<Box<AST>> {
        &self.location_expr_base_ref().target
    }
}
#[impl_traits(TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct LocationExprBase {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub target: Option<Box<AST>>,
}

impl LocationExprBase {
    pub fn new() -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(),
            target: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IdentifierExprUnion {
    String(String),
    Identifier(Identifier),
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierExpr {
    pub location_expr_base: LocationExprBase,
    pub idf: Box<Identifier>,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}

impl IntoAST for IdentifierExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(self)),
        ))
    }
}

impl IdentifierExpr {
    pub fn new(idf: IdentifierExprUnion, annotated_type: Option<Box<AnnotatedTypeName>>) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            idf: Box::new(match idf {
                IdentifierExprUnion::Identifier(idf) => idf,
                IdentifierExprUnion::String(idf) => {
                    Identifier::Identifier(IdentifierBase::new(idf))
                } // _ => Identifier::Identifier(IdentifierBase::new(String::new())),
            }),
            annotated_type,
        }
    }

    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        self.location_expr_base.target.clone().map(|t| {
            t.try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .unwrap()
                .clone()
        })
    }

    pub fn slice(&self, offset: i32, size: i32, base: Option<Expression>) -> SliceExpr {
        SliceExpr::new(
            Some(LocationExpr::IdentifierExpr(self.clone())),
            base,
            offset,
            size,
        )
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.annotated_type = Some(Box::new(at));
        } else if let AST::TypeName(tn) = t {
            selfs.annotated_type = Some(Box::new(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            )));
        }

        selfs
    }
}
impl ASTChildren for IdentifierExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.idf.to_ast());
    }
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MemberAccessExpr {
    pub location_expr_base: LocationExprBase,
    pub expr: Option<Box<LocationExpr>>,
    pub member: Box<Identifier>,
}
impl IntoAST for MemberAccessExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::MemberAccessExpr(self)),
        ))
    }
}

impl MemberAccessExpr {
    pub fn new(expr: Option<LocationExpr>, member: Identifier) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            expr: expr.map(|expr| Box::new(expr)),
            member: Box::new(member),
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl ASTChildren for MemberAccessExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(expr) = &self.expr {
            cb.add_child(expr.to_ast());
        }
        cb.add_child(self.member.to_ast());
    }
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IndexExpr {
    pub location_expr_base: LocationExprBase,
    pub arr: Option<Box<LocationExpr>>,
    pub key: Box<Expression>,
}
impl IntoAST for IndexExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IndexExpr(self)),
        ))
    }
}

impl IndexExpr {
    pub fn new(arr: Option<LocationExpr>, key: Expression) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            arr: arr.map(|a| Box::new(a)),
            key: Box::new(key),
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl ASTChildren for IndexExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(arr) = &self.arr {
            cb.add_child(arr.to_ast());
        }
        cb.add_child(self.key.to_ast());
    }
}
#[impl_traits(LocationExprBase, TupleOrLocationExprBase, ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct SliceExpr {
    pub location_expr_base: LocationExprBase,
    pub arr: Option<Box<LocationExpr>>,
    pub base: Option<Box<Expression>>,
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
        arr: Option<LocationExpr>,
        base: Option<Expression>,
        start_offset: i32,
        size: i32,
    ) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            arr: arr.map(|a| Box::new(a)),
            base: if let Some(base) = base {
                Some(Box::new(base))
            } else {
                None
            },
            start_offset,
            size,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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

    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl Immutable for MeExpr {
    fn is_immutable(&self) -> bool {
        true
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl Immutable for AllExpr {
    fn is_immutable(&self) -> bool {
        true
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    ReclassifyExprBaseRef,
    ReclassifyExprBaseMutRef,
    ExpressionBaseRef,
    ExpressionBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum ReclassifyExpr {
    ReclassifyExpr(ReclassifyExprBase),
    RehomExpr(RehomExpr),
    EncryptionExpression(EncryptionExpression),
}

impl ReclassifyExpr {
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            ReclassifyExpr::ReclassifyExpr(ast) => ReclassifyExpr::ReclassifyExpr(ast.as_type(t)),
            ReclassifyExpr::RehomExpr(ast) => ReclassifyExpr::RehomExpr(ast.as_type(t)),
            ReclassifyExpr::EncryptionExpression(ast) => {
                ReclassifyExpr::EncryptionExpression(ast.as_type(t))
            }
        }
    }

    pub fn func_name(&self) -> String {
        String::from("reveal")
    }
}

#[enum_dispatch]
pub trait ReclassifyExprBaseRef: ExpressionBaseRef {
    fn reclassify_expr_base_ref(&self) -> &ReclassifyExprBase;
}
pub trait ReclassifyExprBaseProperty {
    fn expr(&self) -> &Box<Expression>;
    fn privacy(&self) -> &Box<Expression>;
    fn homomorphism(&self) -> &Option<String>;
}
impl<T: ReclassifyExprBaseRef> ReclassifyExprBaseProperty for T {
    fn expr(&self) -> &Box<Expression> {
        &self.reclassify_expr_base_ref().expr
    }
    fn privacy(&self) -> &Box<Expression> {
        &self.reclassify_expr_base_ref().privacy
    }
    fn homomorphism(&self) -> &Option<String> {
        &self.reclassify_expr_base_ref().homomorphism
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ImplBaseTrait,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct ReclassifyExprBase {
    pub expression_base: ExpressionBase,
    pub expr: Box<Expression>,
    pub privacy: Box<Expression>,
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
    pub fn new(expr: Expression, privacy: Expression, homomorphism: Option<String>) -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            expr: Box::new(expr),
            privacy: Box::new(privacy),
            homomorphism,
        }
    }
    pub fn func_name(&self) -> String {
        String::from("reveal")
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                Some(tn),
                None,
                Homomorphism::non_homomorphic(),
            ));
        }

        selfs
    }
}
impl ASTChildren for ReclassifyExprBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.expr.to_ast());
        cb.add_child(self.privacy.to_ast());
    }
}
#[impl_traits(ReclassifyExprBase, ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn new(expr: Expression, homomorphism: Option<String>) -> Self {
        Self {
            reclassify_expr_base: ReclassifyExprBase::new(
                expr,
                Expression::MeExpr(MeExpr::new()),
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
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.reclassify_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.reclassify_expr_base.expression_base.annotated_type = Some(
                AnnotatedTypeName::new(Some(tn), None, Homomorphism::non_homomorphic()),
            );
        }

        selfs
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HybridArgType {
    PrivCircuitVal,
    PubCircuitArg,
    PubContractVal,
    TmpCircuitVal,
}
#[impl_traits(IdentifierBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HybridArgumentIdf {
    pub identifier_base: IdentifierBase,
    pub t: Box<TypeName>,
    pub arg_type: HybridArgType,
    pub corresponding_priv_expression: Option<Expression>,
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
        mut t: TypeName,
        arg_type: HybridArgType,
        corresponding_priv_expression: Option<Expression>,
    ) -> Self {
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(_)) = &t {
            t = TypeName::bool_type();
        } else if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(tt),
        )) = &t
        {
            t = tt.to_abstract_type();
        } else if let TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(tt)) = &t
        {
            t = tt.to_abstract_type();
        }

        Self {
            identifier_base: IdentifierBase::new(name),
            t: Box::new(t),
            arg_type,
            corresponding_priv_expression,
            serialized_loc: SliceExpr::new(
                Some(LocationExpr::IdentifierExpr(IdentifierExpr::new(
                    IdentifierExprUnion::String(String::new()),
                    None,
                ))),
                None,
                -1,
                -1,
            ),
        }
    }

    pub fn get_loc_expr(&self, parent: Option<AST>) -> AST {
        if self.arg_type == HybridArgType::TmpCircuitVal
            && self.corresponding_priv_expression.is_some()
            && if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(_)) =
                **self
                    .corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .unwrap()
            {
                true
            } else {
                false
            }
        {
            BooleanLiteralExpr::new(
                self.corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_boolean_literal_type_ref()
                    .unwrap()
                    .value(),
            )
            .into_ast()
        } else if self.arg_type == HybridArgType::TmpCircuitVal
            && self.corresponding_priv_expression.is_some()
            && if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::NumberLiteralType(_),
            )) = **self
                .corresponding_priv_expression
                .as_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .unwrap()
            {
                true
            } else {
                false
            }
        {
            NumberLiteralExpr::new(
                self.corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_number_type_name_ref()
                    .unwrap()
                    .try_as_number_literal_type_ref()
                    .unwrap()
                    .value(),
                false,
            )
            .into_ast()
        } else {
            assert!(self.arg_type == HybridArgType::PubCircuitArg);
            let mut ma = LocationExpr::IdentifierExpr(IdentifierExpr::new(
                IdentifierExprUnion::String(CFG.lock().unwrap().zk_data_var_name()),
                None,
            ))
            .dot(IdentifierExprUnion::Identifier(
                Identifier::HybridArgumentIdf(self.clone()),
            ))
            .as_type(AST::TypeName(*self.t.clone()));
            ma.location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .ast_base
                .parent_namespace
                .as_mut()
                .unwrap()
                .borrow_mut()
                .parent = parent.clone().map(|p| Box::new(p));
            ma.location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .statement = if let Some(AST::Expression(e)) = &parent {
                e.statement().clone()
            } else {
                if let Some(AST::Statement(s)) = parent {
                    Some(Box::new(s))
                } else {
                    None
                }
            };
            LocationExpr::MemberAccessExpr(ma).into_ast()
        }
    }
    pub fn get_idf_expr(&self, parent: &Option<Box<AST>>) -> IdentifierExpr {
        let mut ie = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(Identifier::HybridArgumentIdf(self.clone())),
            None,
        )
        .as_type(AST::TypeName(*self.t.clone()));
        if let Identifier::Identifier(mut idf) = *ie.idf {
            idf.ast_base
                .parent_namespace
                .as_mut()
                .unwrap()
                .borrow_mut()
                .parent = parent.clone();
            ie.idf = Box::new(Identifier::Identifier(idf));
        }

        ie.location_expr_base
            .tuple_or_location_expr_base
            .expression_base
            .statement = if let Some(parent) = &parent {
            if let AST::Expression(e) = *parent.clone() {
                e.statement().clone()
            } else {
                if let AST::Statement(parent) = *parent.clone() {
                    Some(Box::new(parent))
                } else {
                    None
                }
            }
        } else {
            None
        };
        ie
    }

    pub fn _set_serialized_loc(
        &mut self,
        idf: String,
        base: Option<Expression>,
        start_offset: i32,
    ) {
        assert!(self.serialized_loc.start_offset == -1);
        self.serialized_loc.arr = Some(Box::new(LocationExpr::IdentifierExpr(
            IdentifierExpr::new(IdentifierExprUnion::String(idf), None),
        )));
        self.serialized_loc.base = if let Some(base) = base {
            Some(Box::new(base))
        } else {
            None
        };
        self.serialized_loc.start_offset = start_offset;
        self.serialized_loc.size = self.t.size_in_uints();
    }

    pub fn deserialize(
        &mut self,
        source_idf: String,
        base: Option<Expression>,
        start_offset: i32,
    ) -> AssignmentStatement {
        self._set_serialized_loc(source_idf.clone(), base.clone(), start_offset);

        let src = IdentifierExpr::new(IdentifierExprUnion::String(source_idf), None)
            .as_type(ArrayBase::new(AnnotatedTypeName::uint_all(), None).into_ast());
        if let TypeName::Array(_a) = *self.t.clone() {
            SliceExpr::new(
                self.get_loc_expr(None)
                    .try_as_expression()
                    .unwrap()
                    .try_as_tuple_or_location_expr()
                    .unwrap()
                    .try_as_location_expr(),
                None,
                0,
                self.t.size_in_uints(),
            )
            .arr
            .unwrap()
            .assign(self.serialized_loc.to_expr())
        } else if let Some(base) = &base {
            self.get_loc_expr(None)
                .try_as_expression_mut()
                .unwrap()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .assign(
                    LocationExpr::IdentifierExpr(src)
                        .index(ExprUnion::Expression(
                            base.binop(
                                String::from("+"),
                                NumberLiteralExpr::new(start_offset, false).to_expr(),
                            )
                            .to_expr(),
                        ))
                        .to_expr()
                        .explicitly_converted(*self.t.clone())
                        .try_as_expression()
                        .unwrap(),
                )
        } else {
            self.get_loc_expr(None)
                .try_as_expression_mut()
                .unwrap()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .assign(
                    Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                        LocationExpr::IndexExpr(
                            LocationExpr::IdentifierExpr(src).index(ExprUnion::I32(start_offset)),
                        ),
                    ))
                    .explicitly_converted(*self.t.clone())
                    .try_as_expression()
                    .unwrap(),
                )
        }
    }

    pub fn serialize(
        &mut self,
        target_idf: String,
        base: Option<Expression>,
        start_offset: i32,
    ) -> AssignmentStatement {
        self._set_serialized_loc(target_idf.clone(), base.clone(), start_offset);

        let tgt = IdentifierExpr::new(IdentifierExprUnion::String(target_idf), None).as_type(
            AST::TypeName(TypeName::Array(Array::Array(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                None,
            )))),
        );
        if let TypeName::Array(_t) = *self.t.clone() {
            let loc = self
                .get_loc_expr(None)
                .try_as_expression()
                .unwrap()
                .try_as_tuple_or_location_expr()
                .unwrap()
                .try_as_location_expr();
            self.serialized_loc
                .arr
                .as_mut()
                .unwrap()
                .assign(SliceExpr::new(loc, None, 0, self.t.size_in_uints()).to_expr())
        } else {
            let expr = self.get_loc_expr(None);
            let expr = if self.t.is_signed_numeric() {
                // Cast to same size uint to prevent sign extension
                expr.try_as_expression().unwrap().explicitly_converted(
                    TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                        NumberTypeName::UintTypeName(UintTypeName::new(format!(
                            "uint{}",
                            self.t.elem_bitwidth()
                        ))),
                    )),
                )
            } else if self.t.is_numeric() && self.t.elem_bitwidth() == 256 {
                expr.try_as_expression()
                    .unwrap()
                    .binop(
                        String::from("%"),
                        Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                            LocationExpr::IdentifierExpr(IdentifierExpr::new(
                                IdentifierExprUnion::String(
                                    CFG.lock().unwrap().field_prime_var_name(),
                                ),
                                None,
                            )),
                        )),
                    )
                    .as_type(AST::TypeName(*self.t.clone()))
                    .into_ast()
            } else {
                expr.try_as_expression()
                    .unwrap()
                    .explicitly_converted(TypeName::uint_type())
                //if let ExplicitlyConvertedUnion::FunctionCallExpr(fce)={fce}else{FunctionCallExpr::default()}
            };

            if let Some(base) = &base {
                LocationExpr::IndexExpr(
                    LocationExpr::IdentifierExpr(tgt.clone()).index(ExprUnion::Expression(
                        base.binop(
                            String::from("+"),
                            NumberLiteralExpr::new(start_offset, false).to_expr(),
                        )
                        .to_expr(),
                    )),
                )
                .assign(expr.try_as_expression().unwrap())
            } else {
                LocationExpr::IndexExpr(
                    LocationExpr::IdentifierExpr(tgt.clone()).index(ExprUnion::I32(start_offset)),
                )
                .assign(expr.try_as_expression().unwrap())
            }
        }
    }
}
#[enum_dispatch(
    IntoAST,
    ASTInstanceOf,
    IdentifierBaseRef,
    IdentifierBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum Identifier {
    Identifier(IdentifierBase),
    HybridArgumentIdf(HybridArgumentIdf),
}
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone().into_ast().code())
    }
}
impl Identifier {
    pub fn identifier(name: &str) -> Self {
        Self::Identifier(IdentifierBase::new(String::from(name)))
    }
}

#[impl_traits(ReclassifyExprBase, ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct EncryptionExpression {
    pub reclassify_expr_base: ReclassifyExprBase,
    pub annotated_type: Option<AnnotatedTypeName>,
}
impl IntoAST for EncryptionExpression {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(
            ReclassifyExpr::EncryptionExpression(self),
        ))
    }
}

impl EncryptionExpression {
    pub fn new(expr: Expression, privacy: AST, homomorphism: Option<String>) -> Self {
        let privacy = privacy.try_as_expression().unwrap();
        let annotated_type = Some(AnnotatedTypeName::cipher_type(
            expr.annotated_type().as_ref().unwrap().clone(),
            homomorphism.clone(),
        ));
        Self {
            reclassify_expr_base: ReclassifyExprBase::new(expr, privacy, homomorphism),
            annotated_type,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.reclassify_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.reclassify_expr_base.expression_base.annotated_type = Some(
                AnnotatedTypeName::new(Some(tn), None, Homomorphism::non_homomorphic()),
            );
        }

        selfs
    }
}
#[enum_dispatch(ASTChildren, IntoAST, ASTInstanceOf)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
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
    pub fn ast_base_ref(&self) -> Option<&ASTBase> {
        impl_base_ref_for_statement!(ast_base_ref, self)
    }
    pub fn ast_base_mut_ref(&mut self) -> Option<&mut ASTBase> {
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
    fn function(&self) -> &Option<Box<ConstructorOrFunctionDefinition>>;
    fn pre_statements(&self) -> &Vec<AST>;
}
impl<T: StatementBaseRef> StatementBaseProperty for T {
    fn before_analysis(&self) -> &Option<PartitionState<AST>> {
        &self.statement_base_ref().before_analysis
    }
    fn after_analysis(&self) -> &Option<PartitionState<AST>> {
        &self.statement_base_ref().after_analysis
    }
    fn function(&self) -> &Option<Box<ConstructorOrFunctionDefinition>> {
        &self.statement_base_ref().function
    }
    fn pre_statements(&self) -> &Vec<AST> {
        &self.statement_base_ref().pre_statements
    }
}

#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct StatementBase {
    pub ast_base: ASTBase,
    pub before_analysis: Option<PartitionState<AST>>,
    pub after_analysis: Option<PartitionState<AST>>,
    pub function: Option<Box<ConstructorOrFunctionDefinition>>,
    pub pre_statements: Vec<AST>,
}
impl StatementBase {
    pub fn new() -> Self {
        Self {
            ast_base: ASTBase::new(),
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
    ASTInstanceOf,
    CircuitComputationStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
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
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct CircuitComputationStatement {
    pub circuit_directive_statement_base: CircuitDirectiveStatementBase,
    pub idf: HybridArgumentIdf,
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
            idf,
        }
    }
}
#[impl_traits(CircuitDirectiveStatementBase, StatementBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IfStatement {
    pub statement_base: StatementBase,
    pub condition: Expression,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}

impl IntoAST for IfStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::IfStatement(self))
    }
}

impl IfStatement {
    pub fn new(condition: Expression, then_branch: Block, else_branch: Option<Block>) -> Self {
        Self {
            statement_base: StatementBase::new(),
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl ASTChildren for IfStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.to_ast());
        cb.add_child(self.then_branch.to_ast());
        cb.add_child(self.then_branch.to_ast());
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WhileStatement {
    pub statement_base: StatementBase,
    pub condition: Expression,
    pub body: Block,
}
impl IntoAST for WhileStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::WhileStatement(self))
    }
}

impl WhileStatement {
    pub fn new(condition: Expression, body: Block) -> Self {
        Self {
            statement_base: StatementBase::new(),
            condition,
            body,
        }
    }
}
impl ASTChildren for WhileStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.to_ast());
        cb.add_child(self.body.to_ast());
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DoWhileStatement {
    pub statement_base: StatementBase,
    pub body: Block,
    pub condition: Expression,
}
impl IntoAST for DoWhileStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::DoWhileStatement(self))
    }
}

impl DoWhileStatement {
    pub fn new(body: Block, condition: Expression) -> Self {
        Self {
            statement_base: StatementBase::new(),
            body,
            condition,
        }
    }
}
impl ASTChildren for DoWhileStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.body.to_ast());
        cb.add_child(self.condition.to_ast());
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ForStatement {
    pub statement_base: StatementBase,
    pub init: Option<Box<SimpleStatement>>,
    pub condition: Expression,
    pub update: Option<Box<SimpleStatement>>,
    pub body: Block,
}
impl IntoAST for ForStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ForStatement(self))
    }
}

impl ForStatement {
    pub fn new(
        init: Option<SimpleStatement>,
        condition: Expression,
        update: Option<SimpleStatement>,
        body: Block,
    ) -> Self {
        Self {
            statement_base: StatementBase::new(),
            init: init.map(|i| Box::new(i)),
            condition,
            update: update.map(|u| Box::new(u)),
            body,
        }
    }

    pub fn statements(&self) -> Vec<Statement> {
        vec![
            self.init.clone().map(|i| i.to_statement()).unwrap(),
            self.condition.to_statement(),
            self.update.clone().map(|u| u.to_statement()).unwrap(),
            self.body.to_statement(),
        ]
    }
}
impl ASTChildren for ForStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(init) = &self.init {
            cb.add_child(init.to_ast());
        }

        cb.add_child(self.condition.to_ast());
        if let Some(update) = &self.update {
            cb.add_child(update.to_ast());
        }
        cb.add_child(self.body.to_ast());
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReturnStatement {
    pub statement_base: StatementBase,
    pub expr: Option<Expression>,
}
impl IntoAST for ReturnStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ReturnStatement(self))
    }
}

impl ReturnStatement {
    pub fn new(expr: Option<Expression>) -> Self {
        Self {
            statement_base: StatementBase::new(),
            expr,
        }
    }
}
impl ASTChildren for ReturnStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(expr) = &self.expr {
            cb.add_child(expr.to_ast());
        }
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    SimpleStatementBaseRef,
    StatementBaseRef,
    StatementBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
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
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
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
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub expr: Expression,
}

impl IntoAST for ExpressionStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::ExpressionStatement(self),
        ))
    }
}

impl ExpressionStatement {
    pub fn new(expr: Expression) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            expr,
        }
    }
}
impl ASTChildren for ExpressionStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.expr.to_ast());
    }
}
#[impl_traits(SimpleStatementBase, StatementBase, ASTBase)]
#[derive(
    ASTDebug, ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct RequireStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub condition: Expression,
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
    pub fn new(condition: Expression, unmodified_code: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            condition,
            unmodified_code: unmodified_code.unwrap_or(String::new()), //self.code()
        }
    }
}
impl ASTChildren for RequireStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.condition.to_ast());
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
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
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
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
    fn lhs(&self) -> &Option<Box<AST>>;
    fn rhs(&self) -> &Option<Expression>;
    fn op(&self) -> &String;
}
impl<T: AssignmentStatementBaseRef> AssignmentStatementBaseProperty for T {
    fn lhs(&self) -> &Option<Box<AST>> {
        &self.assignment_statement_base_ref().lhs
    }
    fn rhs(&self) -> &Option<Expression> {
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct AssignmentStatementBase {
    pub simple_statement_base: SimpleStatementBase,
    pub lhs: Option<Box<AST>>,
    pub rhs: Option<Expression>,
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
    pub fn new(lhs: Option<AST>, rhs: Option<Expression>, op: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            lhs: lhs.map(|l| Box::new(l)),
            rhs,
            op: if let Some(op) = op { op } else { String::new() },
        }
    }
}
impl ASTChildren for AssignmentStatementBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(lhs) = &self.lhs {
            cb.add_child(lhs.to_ast());
        }
        if let Some(rhs) = &self.rhs {
            cb.add_child(rhs.to_ast());
        }
    }
}
#[impl_traits(AssignmentStatementBase, SimpleStatementBase, StatementBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
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

impl CircuitInputStatement {
    pub fn new(lhs: AST, rhs: Expression, op: Option<String>) -> Self {
        Self {
            assignment_statement_base: AssignmentStatementBase::new(Some(lhs), Some(rhs), op),
        }
    }
}

#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    StatementListBaseRef,
    StatementListBaseMutRef,
    StatementBaseRef,
    StatementBaseMutRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
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
    fn statements(&self) -> &Vec<AST>;
    fn excluded_from_simulation(&self) -> bool;
    fn get_item(&self, key: i32) -> AST {
        assert!(self.statements().len() > key as usize);
        self.statements()[key as usize].clone()
    }

    fn contains(&self, stmt: &AST) -> bool {
        if self.statements().contains(stmt) {
            return true;
        }
        for s in self.statements() {
            if is_instance(s, ASTType::StatementListBase) {
                if s.try_as_statement_ref()
                    .unwrap()
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
    fn statements(&self) -> &Vec<AST> {
        &self.statement_list_base_ref().statements
    }
    fn excluded_from_simulation(&self) -> bool {
        self.statement_list_base_ref().excluded_from_simulation
    }
}
#[impl_traits(StatementBase, ASTBase)]
#[derive(
    ImplBaseTrait,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct StatementListBase {
    pub statement_base: StatementBase,
    pub statements: Vec<AST>,
    pub excluded_from_simulation: bool,
}
impl StatementListBase {
    pub fn new(statements: Vec<AST>, excluded_from_simulation: bool) -> Self {
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.statements.iter().for_each(|statement| {
            cb.add_child(statement.clone());
        });
    }
}
#[impl_traits(StatementListBase, StatementBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct Block {
    pub statement_list_base: StatementListBase,
    pub was_single_statement: bool,
}
impl IntoAST for Block {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::Block(self)))
    }
}

impl Block {
    pub fn new(statements: Vec<AST>, was_single_statement: bool) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
            was_single_statement,
        }
    }
}
#[impl_traits(StatementListBase, StatementBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct IndentBlock {
    pub statement_list_base: StatementListBase,
}
impl IntoAST for IndentBlock {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::IndentBlock(self)))
    }
}

impl IndentBlock {
    pub fn new(statements: Vec<AST>) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
        }
    }
}
// #[enum_dispatch(IntoAST, ASTInstanceOf, TypeNameBaseRef, ASTBaseRef)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
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
    pub fn ast_base_ref(&self) -> Option<&ASTBase> {
        impl_base_ref_for_typename!(ast_base_ref, self)
    }
    pub fn ast_base_mut_ref(&mut self) -> Option<&mut ASTBase> {
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

    pub fn cipher_type(plain_type: AnnotatedTypeName, hom: String) -> Self {
        let crypto_params = CFG.lock().unwrap().user_config.get_crypto_params(&hom);
        let mut pt = plain_type.clone();
        pt.homomorphism = hom; // Just for display purposes
        TypeName::Array(Array::CipherText(CipherText::new(pt, crypto_params)))
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

    pub fn size_in_uints(&self) -> i32
// """How many uints this type occupies when serialized."""
    {
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

    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        &expected == &self
    }
    pub fn compatible_with(self, other_type: &TypeName) -> bool {
        self.implicitly_convertible_to(&other_type) || other_type.implicitly_convertible_to(&self)
    }
    pub fn combined_type(&self, other_type: TypeName, _convert_literals: bool) -> Option<Self> {
        if other_type.implicitly_convertible_to(&self) {
            Some(self.clone())
        } else if self.implicitly_convertible_to(&other_type) {
            Some(other_type)
        } else {
            None
        }
    }
    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> AnnotatedTypeName {
        AnnotatedTypeName::new(
            Some(self.clone()),
            if let CombinedPrivacyUnion::AST(expr) = privacy_annotation {
                expr
            } else {
                None
            },
            Homomorphism::non_homomorphic(),
        )
    }
}
#[enum_dispatch]
pub trait TypeNameBaseRef: ASTBaseRef {
    fn type_name_base_ref(&self) -> &TypeNameBase;
}

#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct TypeNameBase {
    pub ast_base: ASTBase,
}
impl TypeNameBase {
    pub fn new() -> Self {
        Self {
            ast_base: ASTBase::new(),
        }
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    ElementaryTypeNameBaseRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || is_instance(expected, ASTType::BoolTypeName)
    }
    pub fn combined_type(&self, other_type: TypeName, convert_literals: bool) -> TypeName {
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(_)) = &other_type
        {
            if convert_literals {
                TypeName::bool_type()
            } else {
                TypeName::Literal(String::from("lit"))
            }
        } else {
            self.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .combined_type(other_type, convert_literals)
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
    pub fn to_abstract_type(&self) -> TypeName {
        TypeName::bool_type()
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    NumberTypeNameBaseRef,
    ElementaryTypeNameBaseRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(_)) =
                &expected
            {
                true
            } else {
                false
            }
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

    pub fn can_represent(&self, value: i32) -> bool
// """Return true if value can be represented by this type"""
    {
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        if expected.is_numeric() && !expected.is_literals()
        // Allow implicit conversion only if it fits
        {
            expected
                .try_as_elementary_type_name_ref()
                .unwrap()
                .try_as_number_type_name_ref()
                .unwrap()
                .number_type_name_base_ref()
                .can_represent(self.value())
        } else if expected.is_address()
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
    pub fn combined_type(&self, other_type: TypeName, convert_literals: bool) -> TypeName {
        if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(_),
        )) = &other_type
        {
            if convert_literals {
                self.to_abstract_type()
                    .combined_type(
                        other_type
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
                TypeName::Literal(String::from("lit"))
            }
        } else {
            self.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .combined_type(other_type, convert_literals)
                .unwrap()
        }
    }
    pub fn to_abstract_type(&self) -> TypeName {
        if self.value() < 0 {
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
        }
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.number_type_name_base
            .implicitly_convertible_to(expected)
            || (if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::IntTypeName(_),
            )) = &expected
            {
                true
            } else {
                false
            } && expected.elem_bitwidth() >= self.number_type_name_base.elem_bitwidth())
    }
}
#[impl_traits(NumberTypeNameBase, ElementaryTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.number_type_name_base
            .implicitly_convertible_to(expected)
            || (if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::UintTypeName(_),
            )) = &expected
            {
                true
            } else {
                false
            } && expected.elem_bitwidth() >= self.number_type_name_base.elem_bitwidth())
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    UserDefinedTypeNameBaseRef,
    UserDefinedTypeNameBaseMutRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    fn names(&self) -> &Vec<Identifier>;
    fn target(&self) -> &Option<Box<AST>>;
}
impl<T: UserDefinedTypeNameBaseRef> UserDefinedTypeNameBaseProperty for T {
    fn names(&self) -> &Vec<Identifier> {
        &self.user_defined_type_name_base_ref().names
    }
    fn target(&self) -> &Option<Box<AST>> {
        &self.user_defined_type_name_base_ref().target
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct UserDefinedTypeNameBase {
    pub type_name_base: TypeNameBase,
    pub names: Vec<Identifier>,
    pub target: Option<Box<AST>>,
}
impl UserDefinedTypeNameBase {
    pub fn new(names: Vec<Identifier>, target: Option<AST>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            names,
            target: target.map(|t| Box::new(t)),
        }
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn new(names: Vec<Identifier>, target: Option<NamespaceDefinition>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                names,
                target.map(|t| t.into_ast()),
            ),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        256
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn new(names: Vec<Identifier>, target: Option<NamespaceDefinition>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                names,
                target.map(|t| t.into_ast()),
            ),
        }
    }
    pub fn elem_bitwidth(&self) -> i32 {
        256
    }
    pub fn to_abstract_type(&self) -> TypeName {
        let mut names = self.user_defined_type_name_base.names.clone();
        names.pop();
        TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumTypeName(EnumTypeName::new(
            names,
            self.user_defined_type_name_base
                .target
                .as_ref()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .parent()
                .as_ref()
                .map(|p| p.try_as_namespace_definition_ref().unwrap().clone()),
        )))
    }
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || (is_instance(expected, ASTType::EnumTypeName)
                && expected
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn new(names: Vec<Identifier>, target: Option<NamespaceDefinition>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                names,
                target.map(|t| t.into_ast()),
            ),
        }
    }
    pub fn to_type_name(&self) -> TypeName {
        TypeName::UserDefinedTypeName(UserDefinedTypeName::StructTypeName(self.clone()))
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    pub fn new(names: Vec<Identifier>, target: Option<NamespaceDefinition>) -> Self {
        Self {
            user_defined_type_name_base: UserDefinedTypeNameBase::new(
                names,
                target.map(|t| t.into_ast()),
            ),
        }
    }
}
#[impl_traits(UserDefinedTypeNameBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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

    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .implicitly_convertible_to(expected)
            || expected == &TypeName::address_type()
    }
    pub fn elem_bitwidth(&self) -> i32 {
        160
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum KeyLabelUnion {
    String(String),
    Identifier(Option<Identifier>),
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mapping {
    pub type_name_base: TypeNameBase,
    pub key_type: Box<TypeName>,
    pub key_label: Option<Identifier>,
    pub value_type: Box<AnnotatedTypeName>,
    pub instantiated_key: Option<Expression>,
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
            key_type: Box::new(key_type),
            key_label,
            value_type: Box::new(value_type),
            instantiated_key: None,
        }
    }
    pub fn has_key_label(&self) -> bool {
        self.key_label.is_some()
    }
}
impl ASTChildren for Mapping {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.key_type.to_ast());
        if let Some(idf) = &self.key_label {
            cb.add_child(idf.to_ast());
        }
        cb.add_child(self.value_type.to_ast());
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ExprUnion {
    I32(i32),
    Expression(Expression),
}

#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    ArrayBaseRef,
    TypeNameBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
pub enum Array {
    CipherText(CipherText),
    Randomness(Randomness),
    Key(Key),
    Proof(Proof),
    Array(ArrayBase),
}

impl ASTChildren for ArrayBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.value_type.to_ast());
        if let Some(ExprUnion::Expression(expr)) = &self.expr {
            cb.add_child(expr.to_ast());
        }
    }
}
#[enum_dispatch]
pub trait ArrayBaseRef: TypeNameBaseRef {
    fn array_base_ref(&self) -> &ArrayBase;
}

pub trait ArrayBaseProperty {
    fn value_type(&self) -> &AnnotatedTypeName;
    fn expr(&self) -> &Option<ExprUnion>;
}
impl<T: ArrayBaseRef> ArrayBaseProperty for T {
    fn value_type(&self) -> &AnnotatedTypeName {
        &self.array_base_ref().value_type
    }
    fn expr(&self) -> &Option<ExprUnion> {
        &self.array_base_ref().expr
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(
    ImplBaseTrait,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct ArrayBase {
    pub type_name_base: TypeNameBase,
    pub value_type: AnnotatedTypeName,
    pub expr: Option<ExprUnion>,
}
impl IntoAST for ArrayBase {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Array(self)))
    }
}

impl ArrayBase {
    pub fn new(value_type: AnnotatedTypeName, expr: Option<ExprUnion>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            value_type,
            expr: if let Some(ExprUnion::I32(expr)) = expr {
                Some(ExprUnion::Expression(
                    NumberLiteralExpr::new(expr, false).to_expr(),
                ))
            } else {
                expr
            },
        }
    }
    pub fn size_in_uints(&self) -> i32 {
        if let Some(ExprUnion::Expression(Expression::LiteralExpr(le))) = &self.expr {
            if let LiteralExpr::NumberLiteralExpr(expr) = le.clone() {
                return expr.value.clone();
            }
        }
        -1
    }

    pub fn elem_bitwidth(&self) -> i32 {
        self.value_type.type_name.as_ref().unwrap().elem_bitwidth()
    }
}

#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct CipherText {
    pub array_base: ArrayBase,
    pub plain_type: AnnotatedTypeName,
    pub crypto_params: CryptoParams,
}
impl IntoAST for CipherText {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::CipherText(self)))
    }
}

impl CipherText {
    pub fn new(plain_type: AnnotatedTypeName, crypto_params: CryptoParams) -> Self {
        assert!(!plain_type.type_name.as_ref().unwrap().is_cipher());
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(Expression::LiteralExpr(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                        crypto_params.cipher_len(),
                        false,
                    )),
                ))),
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
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct Randomness {
    pub array_base: ArrayBase,
    pub crypto_params: CryptoParams,
}
impl IntoAST for Randomness {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Randomness(self)))
    }
}

impl Randomness {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                if let Some(randomness_len) = crypto_params.randomness_len() {
                    Some(ExprUnion::Expression(Expression::LiteralExpr(
                        LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                            randomness_len,
                            false,
                        )),
                    )))
                } else {
                    None
                },
            ),
            crypto_params,
        }
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct Key {
    pub array_base: ArrayBase,
    pub crypto_params: CryptoParams,
}
impl IntoAST for Key {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Key(self)))
    }
}

impl Key {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(Expression::LiteralExpr(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                        crypto_params.key_len(),
                        false,
                    )),
                ))),
            ),
            crypto_params,
        }
    }
}
#[impl_traits(ArrayBase, TypeNameBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct Proof {
    pub array_base: ArrayBase,
}
impl IntoAST for Proof {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Proof(self)))
    }
}

impl Proof {
    pub fn new() -> Self {
        Self {
            array_base: ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(Expression::LiteralExpr(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                        CFG.lock().unwrap().proof_len(),
                        false,
                    )),
                ))),
            ),
        }
    }
}
#[impl_traits(ExpressionBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum CombinedPrivacyUnion {
    Vec(Vec<CombinedPrivacyUnion>),
    AST(Option<AST>),
}
impl CombinedPrivacyUnion {
    pub fn as_expression(self) -> Option<Expression> {
        if let CombinedPrivacyUnion::AST(Some(AST::Expression(expr))) = self {
            Some(expr)
        } else {
            None
        }
    }
}
//     """Does not appear in the syntax, but is necessary for type checking"""
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleType {
    pub type_name_base: TypeNameBase,
    pub types: Vec<AnnotatedTypeName>,
}
impl IntoAST for TupleType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::TupleType(self))
    }
}

impl TupleType {
    pub fn new(types: Vec<AnnotatedTypeName>) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            types,
        }
    }
    pub fn ensure_tuple(t: Option<AnnotatedTypeName>) -> TupleType {
        if let Some(t) = t {
            if let TypeName::TupleType(t) = &**t.type_name.as_ref().unwrap() {
                t.clone()
            } else {
                TupleType::new(vec![t.clone()])
            }
        } else {
            TupleType::empty()
        }
    }

    pub fn len(&self) -> i32 {
        self.types.len() as i32
    }

    pub fn get_item(&self, i: i32) -> AnnotatedTypeName {
        self.types[i as usize].clone()
    }

    pub fn check_component_wise(
        &self,
        other: &Self,
        f: impl FnOnce(AnnotatedTypeName, AnnotatedTypeName) -> bool + std::marker::Copy,
    ) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if !f(self.get_item(i), other.get_item(i)) {
                    return false;
                }
            }
            true
        }
    }

    pub fn implicitly_convertible_to(&self, expected: TypeName) -> bool {
        if let TypeName::TupleType(expected) = expected {
            self.check_component_wise(&expected, |x, y| {
                x.type_name
                    .unwrap()
                    .implicitly_convertible_to(&y.type_name.unwrap())
            })
        } else {
            false
        }
    }

    pub fn compatible_with(&self, other_type: TypeName) -> bool {
        if let TypeName::TupleType(other_type) = other_type {
            self.check_component_wise(&other_type, |x, y| {
                x.type_name.unwrap().compatible_with(&y.type_name.unwrap())
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
                        AnnotatedTypeName::new(
                            e1.type_name
                                .as_ref()
                                .unwrap()
                                .combined_type(*e2.type_name.clone().unwrap(), convert_literals)
                                .unwrap()
                                .try_as_tuple_type()
                                .unwrap()
                                .into_ast()
                                .try_as_type_name(),
                            Some(Expression::DummyAnnotation(DummyAnnotation::new()).into_ast()),
                            Homomorphism::non_homomorphic(),
                        )
                    })
                    .collect(),
            ))
        }
    }
    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> CombinedPrivacyUnion {
        CombinedPrivacyUnion::AST(if let CombinedPrivacyUnion::AST(_) = &privacy_annotation {
            Some(AST::AnnotatedTypeName(AnnotatedTypeName::new(
                Some(TypeName::TupleType(TupleType::new(
                    self.types
                        .iter()
                        .map(|t| {
                            t.type_name
                                .as_ref()
                                .unwrap()
                                .annotate(privacy_annotation.clone())
                        })
                        .collect(),
                ))),
                None,
                Homomorphism::non_homomorphic(),
            )))
        } else if let CombinedPrivacyUnion::Vec(privacy_annotation) = &privacy_annotation {
            assert!(self.types.len() == privacy_annotation.len());
            Some(AST::AnnotatedTypeName(AnnotatedTypeName::new(
                Some(TypeName::TupleType(TupleType::new(
                    self.types
                        .iter()
                        .zip(privacy_annotation)
                        .map(|(t, a)| t.type_name.as_ref().unwrap().annotate(a.clone()))
                        .collect(),
                ))),
                None,
                Homomorphism::non_homomorphic(),
            )))
        } else {
            None
        })
    }
    pub fn perfect_privacy_match(&self, other: &Self) -> bool {
        fn privacy_match(selfs: AnnotatedTypeName, other: AnnotatedTypeName) -> bool {
            selfs.privacy_annotation == other.privacy_annotation
        }

        self.check_component_wise(other, privacy_match)
    }

    pub fn empty() -> TupleType {
        TupleType::new(vec![])
    }
}
#[impl_traits(TypeNameBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FunctionTypeName {
    pub type_name_base: TypeNameBase,
    pub parameters: Vec<Parameter>,
    pub modifiers: Vec<String>,
    pub return_parameters: Vec<Parameter>,
}
impl IntoAST for FunctionTypeName {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::FunctionTypeName(self))
    }
}

impl FunctionTypeName {
    pub fn new(
        parameters: Vec<Parameter>,
        modifiers: Vec<String>,
        return_parameters: Vec<Parameter>,
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.to_ast());
        });
        self.return_parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.to_ast());
        });
    }
}

#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnnotatedTypeName {
    pub ast_base: ASTBase,
    pub type_name: Option<Box<TypeName>>,
    pub had_privacy_annotation: bool,
    pub privacy_annotation: Option<Box<AST>>,
    pub homomorphism: String,
}
impl IntoAST for AnnotatedTypeName {
    fn into_ast(self) -> AST {
        AST::AnnotatedTypeName(self)
    }
}
impl ASTBaseMutRef for AnnotatedTypeName {
    fn ast_base_mut_ref(&mut self) -> &mut ASTBase {
        &mut self.ast_base
    }
}
impl AnnotatedTypeName {
    pub fn new(
        type_name: Option<TypeName>,
        privacy_annotation: Option<AST>,
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
            ast_base: ASTBase::new(),
            type_name: type_name.map(|t| Box::new(t)),
            had_privacy_annotation: privacy_annotation.as_ref().is_some(),
            privacy_annotation: privacy_annotation.map(|p| Box::new(p)).or(Some(Box::new(
                Expression::AllExpr(AllExpr::new()).into_ast(),
            ))),
            homomorphism,
        }
    }
    pub fn zkay_type(&self) -> Self {
        if let TypeName::Array(Array::CipherText(ct)) = *self.type_name.clone().unwrap() {
            ct.plain_type.clone()
        } else {
            self.clone()
        }
    }
    pub fn combined_privacy(
        &self,
        analysis: Option<PartitionState<AST>>,
        other: AnnotatedTypeName,
    ) -> Option<CombinedPrivacyUnion> {
        if let (TypeName::TupleType(selfs), TypeName::TupleType(others)) = (
            *self.type_name.clone().unwrap(),
            *other.type_name.clone().unwrap(),
        ) {
            assert!(selfs.types.len() == others.types.len());
            return Some(CombinedPrivacyUnion::Vec(
                selfs
                    .types
                    .iter()
                    .zip(others.types)
                    .filter_map(|(e1, e2)| e1.combined_privacy(analysis.clone(), e2))
                    .collect(),
            ));
        }
        if self.homomorphism != other.homomorphism && !self.is_public() {
            return None;
        }
        if other.privacy_annotation.is_none() || self.privacy_annotation.is_none() {
            return None;
        }
        let (other_privacy_annotation, self_privacy_annotation) = (
            other.privacy_annotation.clone().unwrap(),
            self.privacy_annotation.clone().unwrap(),
        );
        let p_expected = other_privacy_annotation
            .try_as_expression_ref()
            .unwrap()
            .privacy_annotation_label();
        let p_actual = self_privacy_annotation
            .try_as_expression_ref()
            .unwrap()
            .privacy_annotation_label();
        if let (Some(p_expected), Some(p_actual)) = (p_expected, p_actual) {
            if p_expected == p_actual
                || (analysis.is_some()
                    && analysis
                        .unwrap()
                        .same_partition(&p_expected.into(), &p_actual.into()))
            {
                Some(CombinedPrivacyUnion::AST(Some(*self_privacy_annotation)))
            } else if self_privacy_annotation
                .try_as_expression_ref()
                .unwrap()
                .is_all_expr()
            {
                Some(CombinedPrivacyUnion::AST(Some(*other_privacy_annotation)))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_public(&self) -> bool {
        if let Some(AST::Expression(pa)) = &self.privacy_annotation.as_ref().map(|pa| *pa.clone()) {
            pa.is_all_expr()
        } else {
            false
        }
    }

    pub fn is_private(&self) -> bool {
        !self.is_public()
    }
    pub fn is_private_at_me(&self, analysis: &Option<PartitionState<AST>>) -> bool {
        if let Some(AST::Expression(p)) = &self.privacy_annotation.as_ref().map(|pa| *pa.clone()) {
            p.is_me_expr()
                || (analysis.is_some()
                    && analysis.as_ref().unwrap().same_partition(
                        &p.privacy_annotation_label().unwrap().into(),
                        &MeExpr::new().into_ast(),
                    ))
        } else {
            false
        }
    }
    pub fn is_accessible(&self, analysis: &Option<PartitionState<AST>>) -> bool {
        self.is_public() || self.is_private_at_me(analysis)
    }

    pub fn is_address(&self) -> bool {
        is_instances(
            &**self.type_name.as_ref().unwrap(),
            vec![ASTType::AddressTypeName, ASTType::AddressPayableTypeName],
        )
    }
    pub fn is_cipher(&self) -> bool {
        is_instance(&**self.type_name.as_ref().unwrap(), ASTType::CipherText)
    }
    pub fn with_homomorphism(&self, hom: String) -> Self {
        AnnotatedTypeName::new(
            self.type_name.as_ref().map(|t| *t.clone()),
            self.privacy_annotation.clone().map(|p| *p),
            hom,
        )
    }
    pub fn uint_all() -> Self {
        AnnotatedTypeName::new(
            Some(TypeName::uint_type()),
            None,
            Homomorphism::non_homomorphic(),
        )
    }

    pub fn bool_all() -> Self {
        AnnotatedTypeName::new(
            Some(TypeName::bool_type()),
            None,
            Homomorphism::non_homomorphic(),
        )
    }

    pub fn address_all() -> Self {
        AnnotatedTypeName::new(
            Some(TypeName::address_type()),
            None,
            Homomorphism::non_homomorphic(),
        )
    }

    pub fn cipher_type(plain_type: AnnotatedTypeName, hom: Option<String>) -> Self {
        AnnotatedTypeName::new(
            Some(TypeName::cipher_type(plain_type, hom.unwrap())),
            None,
            Homomorphism::non_homomorphic(),
        )
    }

    pub fn key_type(crypto_params: CryptoParams) -> Self {
        AnnotatedTypeName::new(
            Some(TypeName::key_type(crypto_params)),
            None,
            Homomorphism::non_homomorphic(),
        )
    }

    pub fn proof_type() -> Self {
        AnnotatedTypeName::new(
            Some(TypeName::proof_type()),
            None,
            Homomorphism::non_homomorphic(),
        )
    }
    pub fn all(type_name: TypeName) -> Self {
        AnnotatedTypeName::new(
            Some(type_name),
            Some(Expression::all_expr().into_ast()),
            Homomorphism::non_homomorphic(),
        )
    }
    pub fn me(type_name: TypeName) -> Self {
        AnnotatedTypeName::new(
            Some(type_name),
            Some(Expression::me_expr(None).into_ast()),
            Homomorphism::non_homomorphic(),
        )
    }
    pub fn array_all(value_type: AnnotatedTypeName, length: Vec<i32>) -> Self {
        let mut t = value_type;
        for &l in &length {
            t = AnnotatedTypeName::new(
                Some(TypeName::Array(Array::Array(ArrayBase::new(
                    t,
                    Some(ExprUnion::I32(l)),
                )))),
                None,
                Homomorphism::non_homomorphic(),
            );
        }
        t
    }
}
impl ASTChildren for AnnotatedTypeName {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(type_name) = &self.type_name {
            cb.add_child(type_name.to_ast());
        }
        if let Some(privacy_annotation) = &self.privacy_annotation {
            cb.add_child(privacy_annotation.to_ast());
        }
    }
}
#[enum_dispatch(
    ASTChildren,
    IntoAST,
    ASTInstanceOf,
    IdentifierDeclarationBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    fn annotated_type(&self) -> &Box<AnnotatedTypeName>;
    fn idf(&self) -> &Box<Identifier>;
    fn storage_location(&self) -> &Option<String>;
}
impl<T: IdentifierDeclarationBaseRef> IdentifierDeclarationBaseProperty for T {
    fn keywords(&self) -> &Vec<String> {
        &self.identifier_declaration_base_ref().keywords
    }
    fn annotated_type(&self) -> &Box<AnnotatedTypeName> {
        &self.identifier_declaration_base_ref().annotated_type
    }
    fn idf(&self) -> &Box<Identifier> {
        &self.identifier_declaration_base_ref().idf
    }
    fn storage_location(&self) -> &Option<String> {
        &self.identifier_declaration_base_ref().storage_location
    }
}

#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct IdentifierDeclarationBase {
    pub ast_base: ASTBase,
    pub keywords: Vec<String>,
    pub annotated_type: Box<AnnotatedTypeName>,
    pub idf: Box<Identifier>,
    pub storage_location: Option<String>,
}
impl IdentifierDeclarationBase {
    fn new(
        keywords: Vec<String>,
        annotated_type: AnnotatedTypeName,
        idf: Identifier,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            ast_base: ASTBase::new(),
            keywords,
            annotated_type: Box::new(annotated_type),
            idf: Box::new(idf),
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.annotated_type.to_ast());
        cb.add_child(self.idf.to_ast());
    }
}
#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct VariableDeclaration {
    pub identifier_declaration_base: IdentifierDeclarationBase,
}
impl IntoAST for VariableDeclaration {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(self))
    }
}

impl VariableDeclaration {
    pub fn new(
        keywords: Vec<String>,
        annotated_type: AnnotatedTypeName,
        idf: Identifier,
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
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclarationStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub variable_declaration: VariableDeclaration,
    pub expr: Option<Expression>,
}

impl IntoAST for VariableDeclarationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::VariableDeclarationStatement(self),
        ))
    }
}

impl VariableDeclarationStatement {
    pub fn new(variable_declaration: VariableDeclaration, expr: Option<Expression>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            variable_declaration,
            expr,
        }
    }
}
impl ASTChildren for VariableDeclarationStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.variable_declaration.to_ast());
        if let Some(expr) = &self.expr {
            cb.add_child(expr.to_ast());
        }
    }
}
#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(
    ASTChildrenImpl,
    ASTKind,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
)]
pub struct Parameter {
    pub identifier_declaration_base: IdentifierDeclarationBase,
}
impl IntoAST for Parameter {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(self))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ParameterUnion {
    Parameter(Parameter),
    String(String),
}
impl Parameter {
    pub fn new(
        keywords: Vec<String>,
        annotated_type: AnnotatedTypeName,
        idf: Identifier,
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
    ASTInstanceOf,
    NamespaceDefinitionBaseRef,
    ASTBaseRef,
    ASTBaseMutRef
)]
#[derive(
    EnumIs, EnumTryAs, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
#[serde(untagged)]
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
    fn idf(&self) -> &Identifier;
}
impl<T: NamespaceDefinitionBaseRef> NamespaceDefinitionBaseProperty for T {
    fn idf(&self) -> &Identifier {
        &self.namespace_definition_base_ref().idf
    }
}

#[derive(
    ImplBaseTrait, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct NamespaceDefinitionBase {
    pub ast_base: ASTBase,
    pub idf: Identifier,
}
impl NamespaceDefinitionBase {
    pub fn new(idf: Identifier) -> Self {
        Self {
            ast_base: ASTBase::new(),
            idf,
        }
    }
}
impl ASTChildren for NamespaceDefinitionBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(self.idf.to_ast());
    }
}
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ConstructorOrFunctionDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub parameters: Vec<Parameter>,
    pub modifiers: Vec<String>,
    pub return_parameters: Vec<Parameter>,
    pub body: Option<Block>,
    pub return_var_decls: Vec<VariableDeclaration>,
    pub parent: Option<ContractDefinition>,
    pub original_body: Option<Block>,
    pub annotated_type: Option<AnnotatedTypeName>,
    pub called_functions: BTreeSet<ConstructorOrFunctionDefinition>,
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
        idf: Option<Identifier>,
        parameters: Option<Vec<Parameter>>,
        modifiers: Option<Vec<String>>,
        return_parameters: Option<Vec<Parameter>>,
        body: Option<Block>,
    ) -> Self {
        assert!(
            idf.is_some() && idf.as_ref().unwrap().name() != "constructor"
                || return_parameters.is_none()
        );
        let idf = if let Some(idf) = idf {
            idf
        } else {
            Identifier::Identifier(IdentifierBase::new(String::from("constructor")))
        };
        let return_parameters = if let Some(return_parameters) = return_parameters {
            return_parameters
        } else {
            vec![]
        };
        let mut return_var_decls: Vec<_> = return_parameters
            .iter()
            .enumerate()
            .map(|(idx, rp)| {
                VariableDeclaration::new(
                    vec![],
                    *rp.identifier_declaration_base.annotated_type.clone(),
                    Identifier::Identifier(IdentifierBase::new(format!(
                        "{}_{idx}",
                        CFG.lock().unwrap().return_var_name()
                    ))),
                    rp.identifier_declaration_base.storage_location.clone(),
                )
            })
            .collect();
        return_var_decls.iter_mut().for_each(|mut vd| {
            if let Identifier::Identifier(mut idf) = *vd.identifier_declaration_base.idf.clone() {
                idf.ast_base
                    .parent_namespace
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .parent = Some(Box::new(AST::IdentifierDeclaration(
                    IdentifierDeclaration::VariableDeclaration(vd.clone()),
                )));
                vd.identifier_declaration_base.idf = Box::new(Identifier::Identifier(idf));
            }
        });
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf),
            parameters: parameters.clone().unwrap_or(vec![]),
            modifiers: modifiers.clone().unwrap_or(vec![]),
            return_parameters: return_parameters.clone(),
            body,
            return_var_decls,
            parent: None,
            original_body: None,
            annotated_type: Some(AnnotatedTypeName::new(
                Some(TypeName::FunctionTypeName(FunctionTypeName::new(
                    parameters.unwrap_or(vec![]),
                    modifiers.unwrap_or(vec![]),
                    return_parameters,
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
        self.namespace_definition_base.idf.name().clone()
    }

    pub fn return_type(&self) -> TupleType {
        TupleType::new(
            self.return_parameters
                .iter()
                .map(|p| *p.identifier_declaration_base.annotated_type.clone())
                .collect(),
        )
    }

    pub fn parameter_types(&self) -> TupleType
// return TupleType([p.annotated_type for p in self.parameters])
    {
        TupleType::new(
            self.parameters
                .iter()
                .map(|p| *p.identifier_declaration_base.annotated_type.clone())
                .collect(),
        )
    }

    pub fn is_constructor(&self) -> bool {
        self.namespace_definition_base.idf.name().as_str() == "constructor"
    }

    pub fn is_function(&self) -> bool {
        !self.is_constructor()
    }

    pub fn _update_fct_type(&mut self) {
        self.annotated_type = Some(AnnotatedTypeName::new(
            Some(TypeName::FunctionTypeName(FunctionTypeName::new(
                self.parameters.clone(),
                self.modifiers.clone(),
                self.return_parameters.clone(),
            ))),
            None,
            Homomorphism::non_homomorphic(),
        ));
        // AnnotatedTypeName(FunctionTypeName(&self.parameters, self.modifiers, self.return_parameters));
    }
    pub fn add_param(&mut self, t: AST, idf: IdentifierExprUnion, ref_storage_loc: Option<String>) {
        let ref_storage_loc = ref_storage_loc.unwrap_or(String::from("memory"));
        let t = if let AST::AnnotatedTypeName(t) = t {
            Some(t)
        } else if let AST::TypeName(t) = t {
            Some(AnnotatedTypeName::new(
                Some(t),
                None,
                Homomorphism::non_homomorphic(),
            ))
        } else {
            None
        };
        let idf = if let IdentifierExprUnion::String(idf) = idf {
            Some(Identifier::Identifier(IdentifierBase::new(idf)))
        } else if let IdentifierExprUnion::Identifier(idf) = idf {
            Some(idf.clone())
        } else {
            None
        };
        let storage_loc = if t
            .as_ref()
            .unwrap()
            .type_name
            .as_ref()
            .unwrap()
            .is_primitive_type()
        {
            None
        } else {
            Some(ref_storage_loc)
        };
        self.parameters.push(Parameter::new(
            vec![],
            t.unwrap(),
            idf.unwrap(),
            storage_loc,
        ));
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.to_ast());
        });
        self.return_parameters.iter().for_each(|parameter| {
            cb.add_child(parameter.to_ast());
        });
        if let Some(body) = &self.body {
            cb.add_child(body.to_ast());
        }
    }
}
#[impl_traits(IdentifierDeclarationBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateVariableDeclaration {
    pub identifier_declaration_base: IdentifierDeclarationBase,
    pub expr: Option<Expression>,
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
        annotated_type: AnnotatedTypeName,
        keywords: Vec<String>,
        idf: Identifier,
        expr: Option<Expression>,
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.identifier_declaration_base.process_children(cb);
        if let Some(expr) = &self.expr {
            cb.add_child(expr.to_ast());
        }
    }
}

#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumValue {
    pub ast_base: ASTBase,
    pub idf: Option<Identifier>,
    pub annotated_type: Option<AnnotatedTypeName>,
}
impl IntoAST for EnumValue {
    fn into_ast(self) -> AST {
        AST::EnumValue(self)
    }
}
impl ASTBaseMutRef for EnumValue {
    fn ast_base_mut_ref(&mut self) -> &mut ASTBase {
        &mut self.ast_base
    }
}
impl EnumValue {
    pub fn new(idf: Option<Identifier>) -> Self {
        Self {
            ast_base: ASTBase::new(),
            idf,
            annotated_type: None,
        }
    }
    pub fn qualified_name(&mut self) -> Vec<Identifier> {
        vec![]
    }
}
impl ASTChildren for EnumValue {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf {
            cb.add_child(idf.to_ast());
        }
    }
}
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub values: Vec<EnumValue>,
    pub annotated_type: Option<AnnotatedTypeName>,
}
impl IntoAST for EnumDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::EnumDefinition(self))
    }
}

impl EnumDefinition {
    pub fn new(idf: Option<Identifier>, values: Vec<EnumValue>) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf.unwrap()),
            values,
            annotated_type: None,
        }
    }
    pub fn qualified_name(&mut self) -> Vec<Identifier> {
        vec![]
    }
}

impl ASTChildren for EnumDefinition {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.values.iter().for_each(|value| {
            cb.add_child(value.to_ast());
        });
    }
}
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StructDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub members: Vec<AST>,
}
impl IntoAST for StructDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::StructDefinition(self))
    }
}

impl StructDefinition {
    pub fn new(idf: Identifier, members: Vec<AST>) -> Self {
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.members.iter().for_each(|member| {
            cb.add_child(member.clone());
        });
    }
}
#[impl_traits(NamespaceDefinitionBase, ASTBase)]
#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub state_variable_declarations: Vec<AST>,
    pub constructor_definitions: Vec<ConstructorOrFunctionDefinition>,
    pub function_definitions: Vec<ConstructorOrFunctionDefinition>,
    pub enum_definitions: Vec<EnumDefinition>,
    pub struct_definitions: Vec<StructDefinition>,
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
}
impl IntoAST for ContractDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::ContractDefinition(self))
    }
}

impl ContractDefinition {
    pub fn new(
        idf: Option<Identifier>,
        state_variable_declarations: Vec<AST>,
        constructor_definitions: Vec<ConstructorOrFunctionDefinition>,
        function_definitions: Vec<ConstructorOrFunctionDefinition>,
        enum_definitions: Vec<EnumDefinition>,
        struct_definitions: Option<Vec<StructDefinition>>,
        used_crypto_backends: Option<Vec<CryptoParams>>,
    ) -> Self {
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf.unwrap()),
            state_variable_declarations,
            constructor_definitions,
            function_definitions,
            enum_definitions,
            struct_definitions: if let Some(struct_definitions) = struct_definitions {
                struct_definitions
            } else {
                vec![]
            },
            used_crypto_backends,
        }
    }
    pub fn get_item(&self, key: &String) -> Option<AST> {
        if key == "constructor" {
            if self.constructor_definitions.len() == 0 {
                // # return empty constructor
                let mut c = ConstructorOrFunctionDefinition::new(None, None, None, None, None);
                c.ast_base_mut_ref()
                    .parent_namespace
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .parent = Some(Box::new(self.to_ast()));
                Some(c.into_ast())
            } else if self.constructor_definitions.len() == 1 {
                Some(self.constructor_definitions[0].to_ast())
            } else {
                // assert!(false,"Multiple constructors exist");
                None
            }
        } else {
            let d_identifier = self.names().get(key).unwrap();
            d_identifier.parent().as_ref().map(|p| *p.clone())
        }
    }
}
impl ASTChildren for ContractDefinition {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.enum_definitions.iter().for_each(|enum_definition| {
            cb.add_child(enum_definition.to_ast());
        });
        self.struct_definitions
            .iter()
            .for_each(|struct_definition| {
                cb.add_child(struct_definition.to_ast());
            });
        self.state_variable_declarations
            .iter()
            .for_each(|state_variable_declarations| {
                cb.add_child(state_variable_declarations.clone());
            });
        self.constructor_definitions
            .iter()
            .for_each(|constructor_definition| {
                cb.add_child(constructor_definition.to_ast());
            });
        self.function_definitions
            .iter()
            .for_each(|function_definition| {
                cb.add_child(function_definition.to_ast());
            });
    }
}

#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub ast_base: ASTBase,
    pub pragma_directive: String,
    pub contracts: Vec<ContractDefinition>,
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
impl ASTBaseMutRef for SourceUnit {
    fn ast_base_mut_ref(&mut self) -> &mut ASTBase {
        &mut self.ast_base
    }
}
impl SourceUnit {
    pub fn new(
        pragma_directive: String,
        contracts: Vec<ContractDefinition>,
        used_contracts: Option<Vec<String>>,
    ) -> Self {
        Self {
            ast_base: ASTBase::new(),
            pragma_directive,
            contracts,
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
    pub fn get_item(&self, key: &String) -> Option<ContractDefinition> {
        if let Some(c_identifier) = self.ast_base.names.get(key) {
            if let Some(AST::NamespaceDefinition(NamespaceDefinition::ContractDefinition(c))) =
                c_identifier.parent().as_ref().map(|p| *p.clone())
            {
                return Some(c.clone());
            }
        }
        None
    }
}
impl ASTChildren for SourceUnit {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.contracts.iter().for_each(|contract| {
            cb.add_child(contract.to_ast());
        });
    }
}
impl ConstructorOrFunctionDefinitionAttr for AST {
    fn get_requires_verification_when_external(&self) -> bool {
        if let Some(c) = self
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
        {
            c.requires_verification_when_external
        } else {
            false
        }
    }
    fn get_name(&self) -> String {
        String::new()
    }
}
pub fn get_privacy_expr_from_label(plabel: AST) -> Expression
// """Turn privacy label into expression (i.e. Identifier -> IdentifierExpr, Me and All stay the same)."""
{
    if let Some(idf) = plabel.try_as_identifier_ref() {
        let mut ie = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(idf.clone()),
            Some(Box::new(AnnotatedTypeName::address_all())),
        );
        ie.location_expr_base.target = idf.parent().clone();
        ie.to_expr()
    } else {
        plabel.try_as_expression().unwrap()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct InstanceTarget {
    pub target_key: Vec<Option<Box<AST>>>,
}
impl fmt::Display for InstanceTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.target_key)
    }
}
impl InstanceTarget {
    pub fn new(expr: Vec<Option<Box<AST>>>) -> Self {
        let target_key = if expr.len() == 2 {
            expr
        } else {
            let v = expr[0].clone().map(|e| *e).unwrap();
            if is_instance(&v, ASTType::VariableDeclaration) {
                vec![expr[0].clone(), None]
            } else if is_instance(&v, ASTType::LocationExprBase) {
                let v = v
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap();
                match v.get_ast_type() {
                    ASTType::IdentifierExpr => {
                        vec![v.location_expr_base_ref().target.clone(), None]
                    }

                    ASTType::MemberAccessExpr => vec![
                        v.location_expr_base_ref().target.clone(),
                        Some(Box::new(AST::Identifier(
                            *v.try_as_member_access_expr_ref().unwrap().member.clone(),
                        ))),
                    ],
                    ASTType::IndexExpr => vec![
                        v.location_expr_base_ref().target.clone(),
                        Some(Box::new(AST::Expression(
                            *v.try_as_index_expr_ref().unwrap().key.clone(),
                        ))),
                    ],
                    _ => vec![None; 2],
                }
            } else {
                vec![None; 2]
            }
        };
        assert!(is_instances(
            &target_key[0].clone().map(|k| *k).unwrap(),
            vec![
                ASTType::VariableDeclaration,
                ASTType::Parameter,
                ASTType::StateVariableDeclaration
            ]
        ));
        Self { target_key }
    }

    pub fn target(&self) -> Option<Box<AST>> {
        if !self.target_key.is_empty() {
            self.target_key[0].clone()
        } else {
            None
        }
    }

    pub fn key(&self) -> Option<AST> {
        if self.target_key.len() > 1 {
            self.target_key[1].clone().map(|t| *t)
        } else {
            None
        }
    }

    pub fn privacy(&self) -> Option<AST> {
        if self.key().is_none()
            && !is_instance(
                &**self
                    .target()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
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
                .annotated_type()
                .as_ref()
                .unwrap()
                .zkay_type()
                .privacy_annotation
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .privacy_annotation_label()
                .map(|x| x.into_ast())
        } else {
            let t = self
                .target()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .unwrap()
                .zkay_type()
                .type_name
                .unwrap();
            assert!(is_instance(&*t, ASTType::Mapping));

            if t.try_as_mapping_ref().unwrap().has_key_label() {
                self.key()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label()
                    .map(|x| x.into_ast())
            } else {
                t.try_as_mapping_ref()
                    .unwrap()
                    .value_type
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label()
                    .map(|x| x.into_ast())
            }
        }
    }

    pub fn in_scope_at(&self, ast: AST) -> bool {
        crate::pointers::symbol_table::SymbolTableLinker::in_scope_at(
            &**self
                .target()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .idf(),
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
        stmt.ast_base_ref().unwrap().line
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
        ast.statement().clone()
    } else if let AST::Statement(ast) = &ast {
        Some(Box::new(ast.clone()))
    } else {
        None
    };

    // Get surrounding function
    let fct = if let Some(stmt) = &stmt {
        stmt.statement_base_ref()
            .unwrap()
            .function
            .as_ref()
            .map(|f| f.to_ast())
    } else if is_instance(&ast, ASTType::ConstructorOrFunctionDefinition) {
        Some(ast.clone())
    } else {
        None
    };

    // Get surrounding contract
    let mut ctr = fct.clone().or(Some(ast.clone()));
    while ctr.is_some() && !is_instance(ctr.as_ref().unwrap(), ASTType::ContractDefinition) {
        if let Some(p) = ctr
            .as_ref()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .parent()
            .as_ref()
        {
            ctr = Some(*p.clone());
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
            .parent()
            .as_ref()
            .map(|p| *p.clone());
    }

    let error_msg = if root.is_none() {
        String::from("error")
    } else {
        get_code_error_msg(
            ast.ast_base_ref().unwrap().line,
            ast.ast_base_ref().unwrap().column,
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
            stmt.clone().map(|s| *s),
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ListUnion {
    AST(AST),
    String(String),
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SingleOrListUnion {
    Vec(Vec<ListUnion>),
    AST(AST),
    String(String),
}
#[derive(ASTVisitorBaseRefImpl)]
pub struct CodeVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub display_final: bool,
}

impl AstVisitor for CodeVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::ASTBase == name
            || &ASTType::CommentBase == name
            || &ASTType::IdentifierBase == name
            || &ASTType::FunctionCallExprBase == name
            || &ASTType::PrimitiveCastExpr == name
            || &ASTType::BooleanLiteralExpr == name
            || &ASTType::NumberLiteralExpr == name
            || &ASTType::StringLiteralExpr == name
            || &ASTType::ArrayLiteralExprBase == name
            || &ASTType::TupleExpr == name
            || &ASTType::IdentifierExpr == name
            || &ASTType::MemberAccessExpr == name
            || &ASTType::IndexExpr == name
            || &ASTType::MeExpr == name
            || &ASTType::AllExpr == name
            || &ASTType::ReclassifyExpr == name
            || &ASTType::RehomExpr == name
            || &ASTType::IfStatement == name
            || &ASTType::WhileStatement == name
            || &ASTType::DoWhileStatement == name
            || &ASTType::ForStatement == name
            || &ASTType::BreakStatement == name
            || &ASTType::ContinueStatement == name
            || &ASTType::ReturnStatement == name
            || &ASTType::ExpressionStatement == name
            || &ASTType::RequireStatement == name
            || &ASTType::AssignmentStatementBase == name
            || &ASTType::CircuitDirectiveStatementBase == name
            || &ASTType::StatementListBase == name
            || &ASTType::Block == name
            || &ASTType::IndentBlock == name
            || &ASTType::ElementaryTypeNameBase == name
            || &ASTType::UserDefinedTypeNameBase == name
            || &ASTType::AddressTypeName == name
            || &ASTType::AddressPayableTypeName == name
            || &ASTType::AnnotatedTypeName == name
            || &ASTType::Mapping == name
            || &ASTType::ArrayBase == name
            || &ASTType::CipherText == name
            || &ASTType::TupleType == name
            || &ASTType::VariableDeclaration == name
            || &ASTType::VariableDeclarationStatement == name
            || &ASTType::Parameter == name
            || &ASTType::ConstructorOrFunctionDefinition == name
            || &ASTType::EnumValue == name
            || &ASTType::EnumDefinition == name
            || &ASTType::StructDefinition == name
            || &ASTType::StateVariableDeclaration == name
            || &ASTType::ContractDefinition == name
            || &ASTType::SourceUnit == name
    }
    fn get_attr(&self, name: &ASTType, ast: &AST) -> Self::Return {
        match *name {
            ASTType::ASTBase => Some(self.visit_AST(ast)),
            ASTType::CommentBase => Some(self.visit_Comment(ast.try_as_comment_ref().unwrap())),
            ASTType::IdentifierBase => {
                Some(self.visit_Identifier(ast.try_as_identifier_ref().unwrap()))
            }
            ASTType::FunctionCallExprBase => Some(
                self.visit_FunctionCallExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::PrimitiveCastExpr => Some(
                self.visit_PrimitiveCastExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_primitive_cast_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::BooleanLiteralExpr => Some(
                self.visit_BooleanLiteralExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_boolean_literal_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::NumberLiteralExpr => Some(
                self.visit_NumberLiteralExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::StringLiteralExpr => Some(
                self.visit_StringLiteralExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_string_literal_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ArrayLiteralExprBase => Some(
                self.visit_ArrayLiteralExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_array_literal_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::TupleExpr => Some(
                self.visit_TupleExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_tuple_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::IdentifierExpr => Some(
                self.visit_IdentifierExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_identifier_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::MemberAccessExpr => Some(
                self.visit_MemberAccessExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::IndexExpr => Some(
                self.visit_IndexExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_index_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::MeExpr => Some(
                self.visit_MeExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_me_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::AllExpr => Some(
                self.visit_AllExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_all_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ReclassifyExpr => Some(
                self.visit_ReclassifyExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_reclassify_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::RehomExpr => Some(
                self.visit_RehomExpr(
                    ast.try_as_expression_ref()
                        .unwrap()
                        .try_as_reclassify_expr_ref()
                        .unwrap()
                        .try_as_rehom_expr_ref()
                        .unwrap(),
                ),
            ),
            ASTType::IfStatement => Some(
                self.visit_IfStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_if_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::WhileStatement => Some(
                self.visit_WhileStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_while_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::DoWhileStatement => Some(
                self.visit_DoWhileStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_do_while_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ForStatement => Some(
                self.visit_ForStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_for_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::BreakStatement => Some(
                self.visit_BreakStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_break_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ContinueStatement => Some(
                self.visit_ContinueStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_continue_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ReturnStatement => Some(
                self.visit_ReturnStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_return_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ExpressionStatement => Some(
                self.visit_ExpressionStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_expression_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::RequireStatement => Some(
                self.visit_RequireStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_require_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::AssignmentStatementBase => Some(
                self.visit_AssignmentStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_assignment_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::CircuitDirectiveStatementBase => Some(
                self.visit_CircuitDirectiveStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_circuit_directive_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::StatementListBase => Some(
                self.visit_StatementList(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_statement_list_ref()
                        .unwrap(),
                ),
            ),
            ASTType::Block => Some(
                self.visit_Block(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_statement_list_ref()
                        .unwrap()
                        .try_as_block_ref()
                        .unwrap(),
                ),
            ),
            ASTType::IndentBlock => Some(
                self.visit_IndentBlock(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_statement_list_ref()
                        .unwrap()
                        .try_as_indent_block_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ElementaryTypeNameBase => Some(
                self.visit_ElementaryTypeName(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_elementary_type_name_ref()
                        .unwrap(),
                ),
            ),
            ASTType::UserDefinedTypeNameBase => Some(
                self.visit_UserDefinedTypeName(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_user_defined_type_name_ref()
                        .unwrap(),
                ),
            ),
            ASTType::AddressTypeName => Some(
                self.visit_AddressTypeName(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_user_defined_type_name_ref()
                        .unwrap()
                        .try_as_address_type_name_ref()
                        .unwrap(),
                ),
            ),
            ASTType::AddressPayableTypeName => Some(
                self.visit_AddressPayableTypeName(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_user_defined_type_name_ref()
                        .unwrap()
                        .try_as_address_payable_type_name_ref()
                        .unwrap(),
                ),
            ),
            ASTType::AnnotatedTypeName => {
                Some(self.visit_AnnotatedTypeName(ast.try_as_annotated_type_name_ref().unwrap()))
            }
            ASTType::Mapping => Some(
                self.visit_Mapping(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_mapping_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ArrayBase => Some(
                self.visit_Array(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_array_ref()
                        .unwrap(),
                ),
            ),
            ASTType::CipherText => Some(
                self.visit_CipherText(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_array_ref()
                        .unwrap()
                        .try_as_cipher_text_ref()
                        .unwrap(),
                ),
            ),
            ASTType::TupleType => Some(
                self.visit_TupleType(
                    ast.try_as_type_name_ref()
                        .unwrap()
                        .try_as_tuple_type_ref()
                        .unwrap(),
                ),
            ),
            ASTType::VariableDeclaration => Some(
                self.visit_VariableDeclaration(
                    ast.try_as_identifier_declaration_ref()
                        .unwrap()
                        .try_as_variable_declaration_ref()
                        .unwrap(),
                ),
            ),
            ASTType::VariableDeclarationStatement => Some(
                self.visit_VariableDeclarationStatement(
                    ast.try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_variable_declaration_statement_ref()
                        .unwrap(),
                ),
            ),
            ASTType::Parameter => Some(
                self.visit_Parameter(
                    ast.try_as_identifier_declaration_ref()
                        .unwrap()
                        .try_as_parameter_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ConstructorOrFunctionDefinition => Some(
                self.visit_ConstructorOrFunctionDefinition(
                    ast.try_as_namespace_definition_ref()
                        .unwrap()
                        .try_as_constructor_or_function_definition_ref()
                        .unwrap(),
                ),
            ),
            ASTType::EnumValue => Some(self.visit_EnumValue(ast.try_as_enum_value_ref().unwrap())),
            ASTType::EnumDefinition => Some(
                self.visit_EnumDefinition(
                    ast.try_as_namespace_definition_ref()
                        .unwrap()
                        .try_as_enum_definition_ref()
                        .unwrap(),
                ),
            ),
            ASTType::StructDefinition => Some(
                self.visit_StructDefinition(
                    ast.try_as_namespace_definition_ref()
                        .unwrap()
                        .try_as_struct_definition_ref()
                        .unwrap(),
                ),
            ),
            ASTType::StateVariableDeclaration => Some(
                self.visit_StateVariableDeclaration(
                    ast.try_as_identifier_declaration_ref()
                        .unwrap()
                        .try_as_state_variable_declaration_ref()
                        .unwrap(),
                ),
            ),
            ASTType::ContractDefinition => Some(
                self.visit_ContractDefinition(
                    ast.try_as_namespace_definition_ref()
                        .unwrap()
                        .try_as_contract_definition_ref()
                        .unwrap(),
                ),
            ),
            ASTType::SourceUnit => {
                Some(self.visit_SourceUnit(ast.try_as_source_unit_ref().unwrap()))
            }
            _ => None,
        }
    }
}
type CodeVisitorReturn = String;
impl CodeVisitor {
    pub fn new(display_final: bool) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            display_final,
        }
    }

    pub fn visit_list(&self, l: Vec<ListUnion>, mut sep: &str) -> CodeVisitorReturn {
        if sep.is_empty() {
            sep = "\n";
        }
        if l.is_empty() {
            return String::new();
        }

        fn handle(selfs: &CodeVisitor, e: &ListUnion) -> Option<String> {
            if let ListUnion::String(e) = e {
                Some(e.to_owned())
            } else if let ListUnion::AST(e) = e {
                selfs.visit(e)
            } else {
                None
            }
        }

        let s: Vec<_> = l.iter().filter_map(|e| handle(self, e)).collect();
        let s = s.join(sep);
        s
    }

    pub fn visit_single_or_list(&self, v: SingleOrListUnion, mut sep: &str) -> CodeVisitorReturn {
        if sep.is_empty() {
            sep = "\n";
        }
        match v {
            SingleOrListUnion::Vec(v) => self.visit_list(v, sep),
            SingleOrListUnion::String(v) => v,
            SingleOrListUnion::AST(v) => self.visit(&v).unwrap(),
        }
    }

    pub fn visit_AST(&self, _ast: &AST) -> CodeVisitorReturn {
        // should never be called
        // raise NotImplementedError("Did not implement code generation for " + repr(ast))
        // unimplemented!("Did not implement code generation for {:?} ", ast);
        String::new()
    }
    pub fn visit_Comment(&self, ast: &Comment) -> CodeVisitorReturn {
        if ast.text() == String::new() {
            String::new()
        } else if ast.text().contains(" ") {
            format!("/* {} */", ast.text())
        } else {
            format!("// {}", ast.text())
        }
    }

    pub fn visit_Identifier(&self, ast: &Identifier) -> CodeVisitorReturn {
        ast.name().clone()
    }

    pub fn visit_FunctionCallExpr(&self, ast: &FunctionCallExpr) -> CodeVisitorReturn {
        if let Expression::BuiltinFunction(func) = &**ast.func() {
            let args: Vec<_> = ast
                .args()
                .iter()
                .map(|a| self.visit(&AST::Expression(a.clone())).unwrap())
                .collect();
            func.format_string(&args)
        } else {
            let f = self.visit(&ast.func().to_ast()).unwrap();
            let a = self.visit_list(
                ast.args()
                    .iter()
                    .map(|arg| ListUnion::AST(AST::Expression(arg.clone())))
                    .collect(),
                ", ",
            );
            format!("{f}({a})")
        }
    }

    pub fn visit_PrimitiveCastExpr(&self, ast: &PrimitiveCastExpr) -> CodeVisitorReturn {
        if ast.is_implicit {
            self.visit(&ast.expr.to_ast()).unwrap()
        } else {
            format!(
                "{}({})",
                self.visit(&ast.elem_type.to_ast()).unwrap(),
                self.visit(&ast.expr.to_ast()).unwrap()
            )
        }
    }

    pub fn visit_BooleanLiteralExpr(&self, ast: &BooleanLiteralExpr) -> CodeVisitorReturn {
        ast.value.to_string().to_ascii_lowercase()
    }

    pub fn visit_NumberLiteralExpr(&self, ast: &NumberLiteralExpr) -> CodeVisitorReturn {
        if ast.was_hex {
            format!("{:x}", ast.value())
        } else {
            ast.value.to_string()
        }
    }

    pub fn visit_StringLiteralExpr(&self, ast: &StringLiteralExpr) -> CodeVisitorReturn {
        format!("\"{}\"", ast.value)
    }

    pub fn visit_ArrayLiteralExpr(&self, ast: &ArrayLiteralExpr) -> CodeVisitorReturn {
        format!(
            "[{}]",
            self.visit_list(
                ast.values()
                    .iter()
                    .map(|value| ListUnion::AST(AST::Expression(value.clone())))
                    .collect(),
                ", "
            )
        )
    }

    pub fn visit_TupleExpr(&self, ast: &TupleExpr) -> CodeVisitorReturn {
        format!(
            "({})",
            self.visit_list(
                ast.elements
                    .iter()
                    .map(|element| ListUnion::AST(AST::Expression(element.clone())))
                    .collect(),
                ", "
            )
        )
    }

    pub fn visit_IdentifierExpr(&self, ast: &IdentifierExpr) -> CodeVisitorReturn {
        self.visit(&ast.idf.to_ast()).unwrap()
    }

    pub fn visit_MemberAccessExpr(&self, ast: &MemberAccessExpr) -> CodeVisitorReturn {
        format!(
            "{}.{}",
            self.visit(&ast.expr.as_ref().unwrap().to_ast()).unwrap(),
            self.visit(&ast.member.to_ast()).unwrap()
        )
    }

    pub fn visit_IndexExpr(&self, ast: &IndexExpr) -> CodeVisitorReturn {
        format!(
            "{}[{}]",
            self.visit(&ast.arr.as_ref().unwrap().to_ast()).unwrap(),
            self.visit(&ast.key.to_ast()).unwrap()
        )
    }

    pub fn visit_MeExpr(&self, _: &MeExpr) -> CodeVisitorReturn {
        String::from("me")
    }

    pub fn visit_AllExpr(&self, _: &AllExpr) -> CodeVisitorReturn {
        String::from("all")
    }

    pub fn visit_ReclassifyExpr(&self, ast: &ReclassifyExpr) -> CodeVisitorReturn {
        let e = self.visit(&ast.expr().to_ast()).unwrap();
        let p = self.visit(&ast.privacy().to_ast()).unwrap();
        let h = HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .get(ast.homomorphism().as_ref().unwrap())
            .unwrap()
            .clone();
        format!("reveal{h:?}({e}, {p})")
    }

    pub fn visit_RehomExpr(&self, ast: &RehomExpr) -> CodeVisitorReturn {
        let e = self.visit(&ast.reclassify_expr_base.expr.to_ast()).unwrap();
        format!("{}({e})", ast.func_name())
    }

    pub fn visit_IfStatement(&self, ast: &IfStatement) -> CodeVisitorReturn {
        let c = self.visit(&ast.condition.to_ast()).unwrap();
        let t = self.visit_single_or_list(SingleOrListUnion::AST(ast.then_branch.to_ast()), "");
        let mut ret = format!("if ({c}) {t}");
        if let Some(else_branch) = &ast.else_branch {
            let e = self.visit_single_or_list(SingleOrListUnion::AST(else_branch.to_ast()), "");
            ret += format!("\n else {e}").as_str();
        }
        ret
    }

    pub fn visit_WhileStatement(&self, ast: &WhileStatement) -> CodeVisitorReturn {
        let c = self.visit(&ast.condition.to_ast()).unwrap();
        let b = self.visit_single_or_list(SingleOrListUnion::AST(ast.body.to_ast()), "");
        format!("while ({c}) {b}")
    }

    pub fn visit_DoWhileStatement(&self, ast: &DoWhileStatement) -> CodeVisitorReturn {
        let b = self.visit_single_or_list(SingleOrListUnion::AST(ast.body.to_ast()), "");
        let c = self.visit(&ast.condition.to_ast()).unwrap();
        format!("do {b} while ({c});")
    }

    pub fn visit_ForStatement(&self, ast: &ForStatement) -> CodeVisitorReturn {
        let i = if let Some(init) = &ast.init {
            format!(
                "{}",
                self.visit_single_or_list(SingleOrListUnion::AST(init.to_ast()), "")
            )
        } else {
            String::from(";")
        };
        let c = self.visit(&ast.condition.to_ast()).unwrap();
        let u = if let Some(update) = &ast.update {
            format!(
                " {}",
                self.visit_single_or_list(SingleOrListUnion::AST(update.to_ast()), "")
                    .replace(";", "")
            )
        } else {
            String::new()
        };
        let b = self.visit_single_or_list(SingleOrListUnion::AST(ast.body.to_ast()), "");
        format!("for ({i} {c};{u}) {b}")
    }

    pub fn visit_BreakStatement(&self, _: &BreakStatement) -> CodeVisitorReturn {
        String::from("break;")
    }

    pub fn visit_ContinueStatement(&self, _: &ContinueStatement) -> CodeVisitorReturn {
        String::from("continue;")
    }

    pub fn visit_ReturnStatement(&self, ast: &ReturnStatement) -> CodeVisitorReturn {
        if ast.expr.is_none() {
            String::from("return;")
        } else {
            let e = self.visit(&ast.expr.as_ref().unwrap().to_ast()).unwrap();
            format!("return {e};")
        }
    }

    pub fn visit_ExpressionStatement(&self, ast: &ExpressionStatement) -> CodeVisitorReturn {
        self.visit(&ast.expr.to_ast()).unwrap() + ";"
    }

    pub fn visit_RequireStatement(&self, ast: &RequireStatement) -> CodeVisitorReturn {
        let c = self.visit(&ast.condition.to_ast()).unwrap();
        format!("require({c});")
    }

    pub fn visit_AssignmentStatement(&self, ast: &AssignmentStatement) -> CodeVisitorReturn {
        let lhs = ast.lhs();
        let mut op = ast.op().clone();
        if let Some(asu) = lhs
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_tuple_expr_ref()
        {
            if let Some(at) = &asu
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type
            {
                if at.is_private() {
                    op = String::new();
                }
            }
        }
        if let Some(le) = lhs
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
        {
            let annotated_type = match le {
                LocationExpr::IdentifierExpr(ie) => {
                    ie.annotated_type.as_ref().map(|at| *at.clone())
                }
                LocationExpr::MemberAccessExpr(ie) => ie.annotated_type().clone(),
                LocationExpr::IndexExpr(ie) => ie.annotated_type().clone(),
                LocationExpr::SliceExpr(ie) => ie.annotated_type().clone(),
            };
            if let Some(at) = annotated_type {
                if at.is_private() {
                    op = String::new();
                }
            }
        }

        let rhs = if !op.is_empty() {
            ast.rhs()
                .clone()
                .map(|fce| fce.try_as_function_call_expr_ref().unwrap().args()[1].clone())
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
        if let (
            Some(LocationExpr::SliceExpr(lhs)),
            Some(Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                LocationExpr::SliceExpr(rhs),
            ))),
        ) = (
            lhs.as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref(),
            &rhs,
        ) {
            assert!(lhs.size == rhs.size, "Slice ranges don't have same size");
            let mut s = String::new();
            let (lexpr, rexpr) = (
                self.visit(&lhs.arr.as_ref().unwrap().to_ast()).unwrap(),
                self.visit(&rhs.arr.as_ref().unwrap().to_ast()).unwrap(),
            );
            let mut lbase = if let Some(base) = &lhs.base {
                format!("{} + ", self.visit(&base.to_ast()).unwrap())
            } else {
                String::new()
            };
            let mut rbase = if let Some(base) = &rhs.base {
                format!("{} + ", self.visit(&base.to_ast()).unwrap())
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
            let to_ast = |hs| self.visit(hs).unwrap();
            format_string(
                to_ast(&*lhs.as_ref().unwrap()),
                self.visit(&rhs.unwrap().to_ast()).unwrap(),
            )
        }
    }
    pub fn visit_CircuitDirectiveStatement(
        &self,
        _ast: &CircuitDirectiveStatement,
    ) -> CodeVisitorReturn {
        String::new()
    }

    fn handle_block(&self, ast: &StatementList) -> CodeVisitorReturn {
        match ast {
            StatementList::Block(block) => indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone()))
                        .collect(),
                    "",
                ),
            ),
            StatementList::IndentBlock(block) => indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone()))
                        .collect(),
                    "",
                ),
            ),
            _ => String::new(),
        }
    }

    pub fn visit_StatementList(&self, ast: &StatementList) -> CodeVisitorReturn {
        match ast {
            StatementList::Block(block) => indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone()))
                        .collect(),
                    "",
                ),
            ),
            StatementList::IndentBlock(block) => indent(
                self.visit_list(
                    block
                        .statement_list_base
                        .statements
                        .iter()
                        .map(|statement| ListUnion::AST(statement.clone()))
                        .collect(),
                    "",
                ),
            ),
            _ => String::new(),
        }
    }

    pub fn visit_Block(&self, ast: &Block) -> CodeVisitorReturn {
        let b = self
            .handle_block(&StatementList::Block(ast.clone()))
            .trim_end()
            .to_string();
        if ast.was_single_statement && ast.statement_list_base.statements.len() == 1 {
            b
        } else {
            format!("{{ {b} }}")
        }
    }

    pub fn visit_IndentBlock(&self, ast: &IndentBlock) -> CodeVisitorReturn {
        self.handle_block(ast.to_statement().try_as_statement_list_ref().unwrap())
    }

    pub fn visit_ElementaryTypeName(&self, ast: &ElementaryTypeName) -> CodeVisitorReturn {
        ast.name().clone()
    }

    pub fn visit_UserDefinedTypeName(&self, ast: &UserDefinedTypeName) -> CodeVisitorReturn {
        let names: Vec<_> = ast
            .user_defined_type_name_base_ref()
            .names
            .iter()
            .map(|name| ListUnion::AST(AST::Identifier(name.clone())))
            .collect();
        self.visit_list(names, ".")
    }

    pub fn visit_AddressTypeName(&self, _ast: &AddressTypeName) -> CodeVisitorReturn {
        String::from("address")
    }

    pub fn visit_AddressPayableTypeName(&self, _ast: &AddressPayableTypeName) -> CodeVisitorReturn {
        String::from("address payable")
    }

    pub fn visit_AnnotatedTypeName(&self, ast: &AnnotatedTypeName) -> CodeVisitorReturn {
        let t = self
            .visit(&ast.type_name.as_ref().unwrap().to_ast())
            .unwrap();
        let p = if let Some(privacy_annotation) = &ast.privacy_annotation {
            self.visit(&*privacy_annotation).unwrap()
        } else {
            String::new()
        };

        if ast.had_privacy_annotation {
            format!(
                "{t}@{p}{:?}",
                HOMOMORPHISM_STORE
                    .lock()
                    .unwrap()
                    .get(&ast.homomorphism)
                    .unwrap()
            )
        } else {
            t
        }
    }

    pub fn visit_Mapping(&self, ast: &Mapping) -> CodeVisitorReturn {
        let k = self.visit(&ast.key_type.to_ast()).unwrap();
        let label = if let Some(Identifier::Identifier(idf)) = &ast.key_label {
            format!("!{}", self.visit(&idf.to_ast()).unwrap())
        } else {
            if let Some(Identifier::HybridArgumentIdf(key_label)) = &ast.key_label {
                format!("/*!{:?}*/", key_label)
            } else {
                String::new()
            }
        };
        let v = self.visit(&ast.value_type.to_ast()).unwrap();
        format!("mapping({k}{label} => {v})")
    }

    pub fn visit_Array(&self, ast: &Array) -> CodeVisitorReturn {
        let value_type = ast.value_type().clone();
        let expr = ast.expr().clone();
        let t = self.visit(&value_type.to_ast()).unwrap();
        let e = if let Some(ExprUnion::Expression(expr)) = &expr {
            self.visit(&expr.to_ast()).unwrap()
        } else if let Some(ExprUnion::I32(expr)) = &expr {
            expr.to_string()
        } else {
            String::new()
        };
        format!("{t}[{e}]")
    }

    pub fn visit_CipherText(&self, ast: &CipherText) -> CodeVisitorReturn {
        let e = self.visit_Array(&Array::CipherText(ast.clone()));
        format!("{e}/*{}*/", ast.plain_type.to_ast().code())
    }

    pub fn visit_TupleType(&self, ast: &TupleType) -> CodeVisitorReturn {
        let s = self.visit_list(
            ast.types
                .iter()
                .map(|typ| ListUnion::AST(typ.to_ast()))
                .collect(),
            ", ",
        );
        format!("({s})")
    }

    pub fn visit_VariableDeclaration(&self, ast: &VariableDeclaration) -> CodeVisitorReturn {
        let keywords: Vec<_> = ast
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
        let t = self
            .visit(&ast.identifier_declaration_base.annotated_type.to_ast())
            .unwrap();
        let s = if let Some(storage_location) = &ast.identifier_declaration_base.storage_location {
            format!(" {storage_location}")
        } else {
            String::new()
        };
        let i = self
            .visit(&ast.identifier_declaration_base.idf.to_ast())
            .unwrap();
        format!("{k} {t}{s} {i}").trim().to_string()
    }

    pub fn visit_VariableDeclarationStatement(
        &self,
        ast: &VariableDeclarationStatement,
    ) -> CodeVisitorReturn {
        let mut s = self.visit(&ast.variable_declaration.to_ast()).unwrap();
        if let Some(expr) = &ast.expr {
            s += format!(" = {}", self.visit(&expr.to_ast()).unwrap()).as_str();
        }
        s += ";";
        s
    }

    pub fn visit_Parameter(&self, ast: &Parameter) -> CodeVisitorReturn {
        let final_string = String::from("final");
        let f = if !self.display_final {
            None
        } else {
            if ast
                .identifier_declaration_base
                .keywords
                .contains(&final_string)
            {
                Some(final_string)
            } else {
                None
            }
        };
        let t = self.visit(&ast.identifier_declaration_base.annotated_type.to_ast());
        let i = self.visit(&ast.identifier_declaration_base.idf.to_ast());
        let description: Vec<_> = [
            f,
            t,
            ast.identifier_declaration_base.storage_location.clone(),
            i,
        ]
        .iter()
        .filter_map(|d| d.clone())
        .collect();
        description.join(" ")
    }

    pub fn visit_ConstructorOrFunctionDefinition(
        &self,
        ast: &ConstructorOrFunctionDefinition,
    ) -> CodeVisitorReturn {
        let b = if let Some(body) = &ast.body {
            self.visit_single_or_list(SingleOrListUnion::AST(body.to_ast()), "")
        } else {
            String::new()
        };
        self.function_definition_to_str(
            &ast.namespace_definition_base.idf,
            ast.parameters
                .iter()
                .map(|parameter| ParameterUnion::Parameter(parameter.clone()))
                .collect(),
            &ast.modifiers,
            &ast.return_parameters,
            &b,
        )
    }
    fn function_definition_to_str(
        &self,
        idf: &Identifier,
        parameters: Vec<ParameterUnion>,
        modifiers: &Vec<String>,
        return_parameters: &Vec<Parameter>,
        body: &String,
    ) -> CodeVisitorReturn {
        let definition = if idf.name() != "constructor" {
            let i = self.visit(&idf.to_ast()).unwrap();
            format!("function {i}")
        } else {
            String::from("constructor")
        };
        let p = self.visit_list(
            parameters
                .iter()
                .filter_map(|parameter| match parameter {
                    ParameterUnion::Parameter(p) => Some(ListUnion::AST(p.to_ast())),
                    ParameterUnion::String(s) => Some(ListUnion::String(s.clone())),
                })
                .collect(),
            ", ",
        );
        let mut m = modifiers.clone().join(" ");
        if !m.is_empty() {
            m = format!(" {m}");
        }
        let mut r = self.visit_list(
            return_parameters
                .iter()
                .map(|p| ListUnion::AST(p.to_ast()))
                .collect(),
            ", ",
        );
        if !r.is_empty() {
            r = format!(" returns ({r})");
        }

        format!("{definition}({p}){m}{r} {body}")
    }

    pub fn visit_EnumValue(&self, ast: &EnumValue) -> CodeVisitorReturn {
        if let Some(idf) = &ast.idf {
            self.visit(&idf.to_ast()).unwrap()
        } else {
            String::new()
        }
    }

    pub fn visit_EnumDefinition(&self, ast: &EnumDefinition) -> CodeVisitorReturn {
        let values = indent(
            self.visit_list(
                ast.values
                    .iter()
                    .map(|value| ListUnion::AST(value.to_ast()))
                    .collect(),
                ", ",
            ),
        );
        format!(
            "enum {} {{\n{values}\n}}",
            self.visit(&ast.namespace_definition_base.idf.to_ast())
                .unwrap()
        )
    }

    // @staticmethod
    fn __cmp_type_size(v1: &AST, v2: &AST) -> Ordering {
        let (t1, t2) = if let (
            AST::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(v1)),
            AST::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(v2)),
        ) = (v1, v2)
        {
            (
                v1.identifier_declaration_base
                    .annotated_type
                    .type_name
                    .clone(),
                v2.identifier_declaration_base
                    .annotated_type
                    .type_name
                    .clone(),
            )
        } else {
            (None, None)
        };
        if t1.is_none() || t2.is_none() {
            return Ordering::Equal;
        }
        let (t1, t2) = (t1.unwrap(), t2.unwrap());
        let mut cmp = if t1.size_in_uints() > t2.size_in_uints() {
            1
        } else {
            0
        } - if t1.size_in_uints() < t2.size_in_uints() {
            1
        } else {
            0
        };
        if cmp == 0 {
            cmp = if t1.elem_bitwidth() > t2.elem_bitwidth() {
                1
            } else {
                0
            } - if t1.elem_bitwidth() < t2.elem_bitwidth() {
                1
            } else {
                0
            };
        }
        if cmp != 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    pub fn visit_StructDefinition(&self, ast: &StructDefinition) -> CodeVisitorReturn {
        // Define struct with members in order of descending size (to get maximum space savings through packing)
        let mut members_by_descending_size = ast.members.clone();
        members_by_descending_size.sort_by(|v1, v2| Self::__cmp_type_size(v1, v2).reverse());
        let body = indent(
            members_by_descending_size
                .iter()
                .map(|member| self.visit(member).unwrap())
                .collect::<Vec<_>>()
                .join("\n"),
        );
        format!(
            "struct {} {{\n{body}\n}}",
            self.visit(&ast.namespace_definition_base.idf.to_ast())
                .unwrap()
        )
    }

    pub fn visit_StateVariableDeclaration(
        &self,
        ast: &StateVariableDeclaration,
    ) -> CodeVisitorReturn {
        let final_string = String::from("final");
        let keywords: Vec<_> = ast
            .identifier_declaration_base
            .keywords
            .iter()
            .filter_map(|k| {
                if self.display_final || k != &final_string {
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
        let t = self
            .visit(&ast.identifier_declaration_base.annotated_type.to_ast())
            .unwrap();
        let mut k = ast
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
        let i = self
            .visit(&ast.identifier_declaration_base.idf.to_ast())
            .unwrap();
        let mut ret = format!("{f}{t} {k}{i}").trim().to_string();
        if let Some(expr) = &ast.expr {
            ret += &format!(" = {}", self.visit(&expr.to_ast()).unwrap());
        }
        ret + ";"
    }

    fn contract_definition_to_str(
        idf: Identifier,
        state_vars: Vec<String>,
        constructors: Vec<String>,
        functions: Vec<String>,
        enums: Vec<String>,
        structs: Vec<String>,
    ) -> CodeVisitorReturn {
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
        format!("contract {i} {{\n{body}\n}}")
    }

    pub fn visit_ContractDefinition(&self, ast: &ContractDefinition) -> CodeVisitorReturn {
        let state_vars = ast
            .state_variable_declarations
            .iter()
            .map(|e| self.visit(&e.clone()).unwrap())
            .collect::<Vec<_>>(); //[ for e in ast.state_variable_declarations]
        let constructors = ast
            .constructor_definitions
            .iter()
            .map(|e| self.visit(&e.to_ast()).unwrap())
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.constructor_definitions]
        let functions = ast
            .function_definitions
            .iter()
            .map(|e| self.visit(&e.to_ast()).unwrap())
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.function_definitions]
        let enums = ast
            .enum_definitions
            .iter()
            .map(|e| self.visit(&e.to_ast()).unwrap())
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.enum_definitions]
        let structs = ast
            .struct_definitions
            .iter()
            .map(|e| self.visit(&e.to_ast()).unwrap())
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.struct_definitions]

        Self::contract_definition_to_str(
            ast.namespace_definition_base.idf.clone(),
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

    pub fn visit_SourceUnit(&self, ast: &SourceUnit) -> CodeVisitorReturn {
        let p = self.handle_pragma(ast.pragma_directive.clone());
        let contracts = self.visit_list(
            ast.contracts
                .iter()
                .map(|contract| {
                    ListUnion::AST(AST::NamespaceDefinition(
                        NamespaceDefinition::ContractDefinition(contract.clone()),
                    ))
                })
                .collect(),
            "",
        );
        let lfstr = |uc| format!("import \"{}\";", uc);
        //  "\n\n".join(filter("".__ne__, [p, linesep.join([lfstr.format(uc) for uc in ast.used_contracts]), contracts]))
        [
            p,
            ast.used_contracts
                .iter()
                .map(|uc| lfstr(uc))
                .collect::<Vec<_>>()
                .join(LINE_ENDING),
            contracts,
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
    }
}
