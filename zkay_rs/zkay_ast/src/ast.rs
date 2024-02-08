// use  __future__ import annotations;
// import abc;
// import math;
// import operator;
// import textwrap;
// use  collections import OrderedDict;
// use  dataclasses import dataclass;
// use  enum import IntEnum;
// use  functools import cmp_to_key, reduce;
// use  os import linesep;
#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";
// use  typing import List, Dict, Union, Optional, Callable, Set, TypeVar;
use crate::circuit_constraints::CircuitStatement;
use zkay_crypto::params::CryptoParams;
use zkay_utils::progress_printer::warn_print;
use crate::analysis::partition_state::PartitionState;
use crate::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};
use crate::visitor::visitor::AstVisitor;
use zkay_config::{config::{CFG,ConstructorOrFunctionDefinitionAttr}, zk_print};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
// T = TypeVar('T')
use zkay_derive::ASTKind;

pub struct ChildListBuilder {
    pub children: Vec<AST>,
}
impl ChildListBuilder {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
    pub fn add_child(&mut self, ast: AST) {
        // if ast.is_none() {
        //     return;
        // }
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
// #[mac
// #[macro_export]
// macro_rules! is_instance {
//     ($var: expr,$typ:expr) => {
//         var.get_ast_type() == std::concat_idents(ASTType,($typ))
//     };
// }

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
        self.into_ast().expr().unwrap()
    }
}
impl<T: IntoAST + Clone> IntoStatement for T {
    fn into_statement(self) -> Statement {
        self.into_ast().statement().unwrap()
    }
}
pub trait IntoAST: Clone {
    fn to_ast(&self) -> AST {
        self.clone().into_ast()
    }
    fn into_ast(self) -> AST;
}
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl AST {
    pub fn statement(&self) -> Option<Statement> {
        None
    }
    pub fn builtin_function(&self) -> Option<BuiltinFunction> {
        None
    }
    pub fn location_expr_base(&self) -> Option<LocationExprBase> {
        None
    }
    pub fn namespace_definition(&self) -> Option<NamespaceDefinition> {
        None
    }
    pub fn identifier_declaration_base(&self) -> Option<IdentifierDeclarationBase> {
        None
    }
    pub fn identifier_declaration(&self) -> Option<IdentifierDeclaration> {
        None
    }
    pub fn state_variable_declaration(&self) -> Option<StateVariableDeclaration> {
        None
    }
    pub fn type_name(&self) -> Option<TypeName> {
        None
    }
    pub fn identifier(&self) -> Option<Identifier> {
        None
    }
    pub fn me_expr(&self) -> Option<MeExpr> {
        None
    }
    pub fn tuple_or_location_expr(&self) -> Option<TupleOrLocationExpr> {
        None
    }
    pub fn identifier_expr(&self) -> Option<IdentifierExpr> {
        None
    }
    pub fn location_expr(&self) -> Option<LocationExpr> {
        None
    }
    pub fn tuple_expr(&self) -> Option<TupleExpr> {
        None
    }
    pub fn tuple_expr_mut(&mut self) -> Option<&mut TupleExpr> {
        None
    }
    pub fn parameter(&self) -> Option<&Parameter> {
        None
    }
    pub fn variable_declaration(&self) -> Option<&VariableDeclaration> {
        None
    }
    pub fn source_unit(&self) -> Option<&SourceUnit> {
        None
    }
    pub fn statement_list_base(&self) -> Option<StatementListBase> {
        None
    }
    pub fn statement_list(&self) -> Option<StatementList> {
        None
    }
    pub fn ast_base(&self) -> Option<&ASTBase> {
        None
    }
    pub fn ast_base_mut(&mut self) -> Option<&mut ASTBase> {
        None
    }
    pub fn elements(&self) -> Vec<Expression> {
        vec![]
    }
    pub fn after_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_before_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn privacy_annotation_label(&self) -> Option<AST> {
        None
    }
    pub fn is_final(&self) -> bool {
        false
    }
    pub fn func(&self) -> Option<Expression> {
        None
    }
    pub fn set_func_idf_name(&mut self, name: String) {}
    pub fn to_location_expr(&self) -> Option<LocationExpr> {
        // if let Self::LocationExpr(le) = self {
        //     le.clone()
        // } else {
        None
        // }
    }
    pub fn init(&self) -> Option<SimpleStatement> {
        None
    }
    pub fn expr(&self) -> Option<Expression> {
        if let Self::Expression(expr) = self {
            Some(expr.clone())
        } else {
            None
        }
    }
    pub fn pre_statements(&self) -> Vec<AST> {
        vec![]
    }
    pub fn block(&self) -> Option<Block> {
        if let AST::Statement(Statement::StatementList(StatementList::Block(b))) = self {
            Some(b.clone())
        } else {
            None
        }
    }
    pub fn constructor_or_function_definition(&self) -> Option<ConstructorOrFunctionDefinition> {
        None
    }
    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
    pub fn line(&self) -> i32 {
        0
    }
    pub fn column(&self) -> i32 {
        0
    }
    pub fn contract_definition(&self) -> Option<ContractDefinition> {
        None
    }

    pub fn target() -> Option<AST> {
        None
    }
    pub fn parent(&self) -> Option<AST> {
        None
    }
    pub fn text(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&self)
    }
    pub fn code(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&self)
    }
    pub fn original_code(&self) -> Vec<String> {
        vec![]
    }
    pub fn set_original_code(&mut self, code: Vec<String>) {}
    pub fn is_parent_of(&self, child: &AST) -> bool {
        let mut e = child.clone();
        let selfs = self.clone();
        while e != selfs && e.parent().is_some() {
            e = e.parent().unwrap().clone();
        }
        e == selfs
    }
    pub fn name(&self) -> &'static str {
        ""
    }
    pub fn bases(child: &str) -> &'static str {
        ""
    }
    pub fn names(&self) -> BTreeMap<String, Identifier> {
        BTreeMap::new()
    }
    pub fn idf(&self) -> Option<Identifier> {
        None
    }
    pub fn requires_verification(&self) -> bool {
        false
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
// impl fmt::Display for Option<AST> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.map(|ast|ast.code()))
//     }
// }
// impl Immutable for Option<AST> {
//     fn is_immutable(&self) -> bool {
//         true
//     }
// }
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ASTBase {
    pub parent: Option<Box<AST>>,
    pub namespace: Option<Vec<Identifier>>,
    pub names: BTreeMap<String, Identifier>,
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

#[derive(ASTKind, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierBase {
    pub ast_base: ASTBase,
    pub name: String,
}
impl IntoAST for IdentifierBase {
    fn into_ast(self) -> AST {
        AST::Identifier(Identifier::Identifier(self))
    }
}
// impl ASTInstanceOf for IdentifierBase {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::IdentifierBase
//     }
// }

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
                t,
                None,
                Homomorphism::non_homomorphic(),
            )),
            AST::AnnotatedTypeName(t) => Some(t),
            _ => None,
        };
        assert!(t.is_some());
        let t = t.unwrap();
        let storage_loc = if t.type_name.is_primitive_type() {
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
        if let Some(v) = &self.ast_base.parent {
            if let AST::IdentifierDeclaration(IdentifierDeclaration::StateVariableDeclaration(
                svd,
            )) = &**v
            {
                svd.is_final() || svd.is_constant()
            } else {
                false
            }
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl IntoAST for Comment {
    fn into_ast(self) -> AST {
        match self {
            Comment::Comment(ast) => ast.into_ast(),
            Comment::BlankLine(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for Comment {
    fn get_ast_type(&self) -> ASTType {
        match self {
            Comment::Comment(ast) => ast.get_ast_type(),
            Comment::BlankLine(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CommentBase {
    pub ast_base: ASTBase,
    pub text: String,
}
impl IntoAST for CommentBase {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::Comment(self))
    }
}
impl ASTInstanceOf for CommentBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::CommentBase
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BlankLine {
    pub comment_base: CommentBase,
}
impl IntoAST for BlankLine {
    fn into_ast(self) -> AST {
        AST::Comment(Comment::BlankLine(self))
    }
}
impl ASTInstanceOf for BlankLine {
    fn get_ast_type(&self) -> ASTType {
        ASTType::BlankLine
    }
}
impl BlankLine {
    pub fn new() -> Self {
        Self {
            comment_base: CommentBase::new(String::new()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

impl IntoAST for Expression {
    fn into_ast(self) -> AST {
        match self {
            Expression::BuiltinFunction(ast) => ast.into_ast(),
            Expression::FunctionCallExpr(ast) => ast.into_ast(),
            Expression::PrimitiveCastExpr(ast) => ast.into_ast(),
            Expression::LiteralExpr(ast) => ast.into_ast(),
            Expression::TupleOrLocationExpr(ast) => ast.into_ast(),
            Expression::MeExpr(ast) => ast.into_ast(),
            Expression::AllExpr(ast) => ast.into_ast(),
            Expression::ReclassifyExpr(ast) => ast.into_ast(),
            Expression::DummyAnnotation(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for Expression {
    fn get_ast_type(&self) -> ASTType {
        match self {
            Expression::BuiltinFunction(ast) => ast.get_ast_type(),
            Expression::FunctionCallExpr(ast) => ast.get_ast_type(),
            Expression::PrimitiveCastExpr(ast) => ast.get_ast_type(),
            Expression::LiteralExpr(ast) => ast.get_ast_type(),
            Expression::TupleOrLocationExpr(ast) => ast.get_ast_type(),
            Expression::MeExpr(ast) => ast.get_ast_type(),
            Expression::AllExpr(ast) => ast.get_ast_type(),
            Expression::ReclassifyExpr(ast) => ast.get_ast_type(),
            Expression::DummyAnnotation(ast) => ast.get_ast_type(),
        }
    }
}
impl ASTChildren for Expression {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl Expression {
    pub fn expression_base_mut(&mut self) -> Option<&mut ExpressionBase> {
        None
    }
    pub fn is_eq(&self) -> bool {
        false
    }
    pub fn code(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&self.to_ast())
    }
    pub fn set_parent(&mut self, parent: Option<Box<AST>>) {}
    pub fn parent(&self) -> Option<Box<AST>> {
        None
    }
    pub fn privacy(&self) -> Option<Expression> {
        None
    }
    pub fn line(&self) -> i32 {
        -1
    }
    pub fn set_line(&mut self, line: i32) {}
    pub fn set_column(&mut self, column: i32) {}
    pub fn column(&self) -> i32 {
        -1
    }
    pub fn is_ite(&self) -> bool {
        false
    }
    pub fn set_homomorphism(&mut self, homomorphism: String) {}
    pub fn homomorphism(&self) -> String {
        String::new()
    }
    pub fn set_expr(&mut self, expr: Expression) {}
    pub fn expr(&self) -> Option<Expression> {
        None
    }
    pub fn set_args(&mut self, args: Vec<Expression>) {}
    pub fn args(&self) -> Vec<Expression> {
        vec![]
    }

    pub fn set_func_rerand_using(&mut self, rerand_using: Option<Box<IdentifierExpr>>) {}
    pub fn has_shortcircuiting(&self) -> bool {
        false
    }
    pub fn is_private(&self) -> bool {
        false
    }
    pub fn set_statement_pre_statements(&mut self, pre_statements: Vec<AST>) {}
    pub fn to_location_expr(&self) -> Option<LocationExpr> {
        // if let Self::LocationExpr(le) = self {
        //     le.clone()
        // } else {
        None
        // }
    }
    pub fn member(&self) -> Option<Identifier> {
        None
    }
    pub fn func(&self) -> Option<Expression> {
        None
    }
    pub fn idf(&self) -> Option<Identifier> {
        None
    }
    pub fn evaluate_privately(&self) -> bool {
        false
    }
    pub fn set_evaluate_privately(&self, v: bool) {}
    pub fn elements(&self) -> Vec<Expression> {
        vec![]
    }
    pub fn add_pre_statement(&mut self, statement: Statement) {}
    pub fn set_annotated_type(&mut self, annotated_type: AnnotatedTypeName) {}
    pub fn set_statement(&mut self, statement: Statement) {}
    pub fn target(&self) -> Option<Box<AST>> {
        None
    }
    pub fn rerand_using(&self) -> Option<Box<IdentifierExpr>> {
        None
    }
    pub fn op(&self) -> Option<String> {
        None
    }

    pub fn can_be_private(&self) -> bool {
        false
    }
    pub fn is_shiftop(&self) -> bool {
        false
    }
    pub fn all_expr() -> Self {
        Expression::AllExpr(AllExpr::new())
    }
    pub fn me_expr(statement: Option<Box<Statement>>) -> Self {
        let mut me_expr = MeExpr::new();
        me_expr.expression_base.statement = statement;
        Expression::MeExpr(me_expr)
    }
    pub fn explicitly_converted(&self, expected: TypeName) -> AST {
        let mut ret = None;
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
            let t = self.annotated_type().unwrap().type_name;

            if *t == expected {
                return self.clone().into_ast();
            }

            // Explicit casts
            let cast = is_instance(&*t, ASTType::NumberTypeNameBase)
                && is_instances(
                    &expected,
                    vec![
                        ASTType::NumberTypeNameBase,
                        ASTType::AddressTypeName,
                        ASTType::AddressPayableTypeName,
                        ASTType::EnumTypeName,
                    ],
                )
                || is_instance(&*t, ASTType::AddressTypeName)
                    && is_instance(&expected, ASTType::NumberTypeNameBase)
                || is_instance(&*t, ASTType::AddressPayableTypeName)
                    && is_instances(
                        &expected,
                        vec![ASTType::NumberTypeNameBase, ASTType::AddressTypeName],
                    )
                || is_instance(&*t, ASTType::EnumTypeName)
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
            expected.clone(),
            if let Some(privacy_annotation) = self.annotated_type().unwrap().privacy_annotation {
                Some(*privacy_annotation)
            } else {
                None
            },
            self.annotated_type().unwrap().homomorphism,
        ));
        ret.into_ast()
    }

    pub fn is_all_expr(&self) -> bool {
        if let Expression::AllExpr(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_me_expr(&self) -> bool {
        if let Expression::MeExpr(_) = self {
            true
        } else {
            false
        }
    }
    pub fn privacy_annotation_label(&self) -> Option<AST> {
        let get_target = |target: &Option<Box<AST>>| {
            if let Some(t) = target {
                if let Some(id) = t.identifier_declaration() {
                    return Some(id);
                }
            }
            None
        };
        if let Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
            LocationExpr::IdentifierExpr(ie),
        )) = &self
        {
            if let Some(id) = get_target(&ie.location_expr_base.target) {
                if let TypeName::Mapping(m) =
                    *(*id.identifier_declaration_base().unwrap().annotated_type).type_name
                {
                    if let Some(ik) = m.instantiated_key {
                        return ik.privacy_annotation_label();
                    }
                } else {
                    return Some(AST::Identifier(
                        *id.identifier_declaration_base().unwrap().idf,
                    ));
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
            .unwrap()
            .type_name
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
        bf.is_private = self.annotated_type().unwrap().is_private();
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

        if !self.instanceof_data_type(&*expected.type_name) {
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
                    if let TypeName::TupleType(_) = *self.annotated_type().unwrap().type_name {
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
                            .unwrap()
                            .type_name
                            .types()
                            .unwrap()
                            .iter()
                            .map(|t| {
                                CombinedPrivacyUnion::AST(
                                    t.clone()
                                        .privacy_annotation
                                        .clone()
                                        .map(|pa| AST::Expression(*pa)),
                                )
                            })
                            .collect::<Vec<_>>())
                    .to_string(),
                )
            } else if combined_label.expr().unwrap().privacy_annotation_label()
                == actual
                    .unwrap()
                    .privacy_annotation
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
            statement.before_analysis().clone()
        } else {
            None
        }
    }
    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
    pub fn statement(&self) -> Option<Box<Statement>> {
        None
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
// BUILTIN_FUNCTIONS = {
//     'parenthesis': '({})',
//     'ite': '{} ? {} : {}'
// }

// // arithmetic
// arithmetic = {op: f'{{}} {op} {{}}' for op in ['**', '*', '/', '%', '+', '-']}
// arithmetic.update({'sign+': '+{}', 'sign-': '-{}'})
// // comparison
// comp = {op: f'{{}} {op} {{}}' for op in ['<', '>', '<=', '>=']}
// // equality
// eq = {op: f'{{}} {op} {{}}' for op in ['==', '!=']}
// // boolean operations
// bop = {op: f'{{}} {op} {{}}' for op in ['&&', '||']}
// bop['!'] = '!{}'
// // bitwise operations
// bitop = {op: f'{{}} {op} {{}}' for op in ['|', '&', '^']}
// bitop['~'] = '~{}'
// // shift operations
// shiftop = {op: f'{{}} {op} {{}}' for op in ['<<', '>>']}

// BUILTIN_FUNCTIONS.update(arithmetic)
// BUILTIN_FUNCTIONS.update(comp)
// BUILTIN_FUNCTIONS.update(eq)
// BUILTIN_FUNCTIONS.update(bop)
// BUILTIN_FUNCTIONS.update(bitop)
// BUILTIN_FUNCTIONS.update(shiftop)

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
// HOMOMORPHIC_BUILTIN_FUNCTIONS = [
//     HomomorphicBuiltin('sign+', Homomorphism.ADDITIVE, [False]),
//     HomomorphicBuiltin('sign-', Homomorphism.ADDITIVE, [False]),
//     HomomorphicBuiltin('+', Homomorphism.ADDITIVE, [False, False]),
//     HomomorphicBuiltin('-', Homomorphism.ADDITIVE, [False, False]),
//     HomomorphicBuiltin('*', Homomorphism.ADDITIVE, [True, False]),
//     HomomorphicBuiltin('*', Homomorphism.ADDITIVE, [False, True]),
//     // HomomorphicBuiltin('ite', Homomorphism.ADDITIVE, [False, True, True]),
//     // HomomorphicBuiltin('ite', Homomorphism.ADDITIVE, [True, False, False]),
//     // HomomorphicBuiltin('*', Homomorphism.MULTIPLICATIVE, [False, False]),
// ]

// for __hom in HOMOMORPHIC_BUILTIN_FUNCTIONS:
//     assert __hom.op in builtin_op_fct and __hom.homomorphism != Homomorphism.NON_HOMOMORPHIC
//     op_arity = BUILTIN_FUNCTIONS[__hom.op].count('{}')
//     assert op_arity == len(__hom.public_args)
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
                    && __hom.homomorphism != String::from("NON_HOMOMORPHIC")
            );
            let op_arity = BUILTIN_FUNCTIONS[&__hom.op].matches("{}").count() as usize;
            assert!(op_arity == __hom.public_args.len());
        }
        homomorphic_builtin_functions_internal
    };
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BuiltinFunction {
    pub expression_base: Box<ExpressionBase>,
    pub op: String,
    pub is_private: bool,
    pub homomorphism: String,
    pub rerand_using: Option<Box<IdentifierExpr>>,
}
// impl IntoExpression for BuiltinFunction {
//     fn into_expr(self) -> Expression {
//         Expression::BuiltinFunction(self)
//     }
// }
impl IntoAST for BuiltinFunction {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::BuiltinFunction(self))
    }
}
impl ASTInstanceOf for BuiltinFunction {
    fn get_ast_type(&self) -> ASTType {
        ASTType::BuiltinFunction
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
            expression_base: Box::new(ExpressionBase::new()),
            op,
            is_private: false,
            homomorphism: String::from("NON_HOMOMORPHIC"),
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
        args: Vec<Expression>,
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
            .map(|a| a.as_ref().unwrap().type_name.clone())
            .reduce(|l, r| Box::new(l.combined_type(*r, true).unwrap()))
            .unwrap();
        let base_type = AnnotatedTypeName::new(
            elem_type,
            inaccessible_arg_types[0]
                .clone()
                .unwrap()
                .privacy_annotation
                .map(|pr| *pr),
            String::from("NON_HOMOMORPHIC"),
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
                .unwrap()
                .is_accessible(&analysis)
            && !args[1]
                .clone()
                .annotated_type()
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
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
        let public_type = AnnotatedTypeName::all(*self.target_type.type_name.clone()); // same data type, but @all
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FunctionCallExpr {
    FunctionCallExpr(FunctionCallExprBase),
    NewExpr(NewExpr),
}

impl FunctionCallExpr {
    pub fn evaluate_privately(&self) -> bool {
        false
    }
    pub fn analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_annotated_type(&mut self, annotated_type: AnnotatedTypeName) {}

    pub fn set_func_rerand_using(&mut self, rerand_using: Option<Box<IdentifierExpr>>) {}
    pub fn statement(&self) -> Option<Box<Statement>> {
        None
    }
    pub fn set_public_key(&mut self, public_key: Option<Box<HybridArgumentIdf>>) {}
    pub fn set_func_idf_name(&mut self, name: String) {}
    pub fn set_args(&mut self, args: Vec<Expression>) {}
    pub fn args(&self) -> Vec<Expression> {
        vec![]
    }
    pub fn extend_pre_statements(&mut self, statement: Vec<Statement>) {}

    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
    pub fn is_cast(&self) -> bool {
        false
    }
    pub fn public_key(&self) -> Option<HybridArgumentIdf> {
        None
    }

    pub fn func(&self) -> Option<Expression> {
        None
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
// impl IntoExpression for FunctionCallExpr {
//     fn into_expr(self) -> Expression {
//         match self {
//             FunctionCallExpr::FunctionCallExpr(ast) => ast.to_expr(),
//             FunctionCallExpr::NewExpr(ast) => ast.to_expr(),
//         }
//     }
// }
impl IntoAST for FunctionCallExpr {
    fn into_ast(self) -> AST {
        match self {
            FunctionCallExpr::FunctionCallExpr(ast) => ast.into_ast(),
            FunctionCallExpr::NewExpr(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for FunctionCallExpr {
    fn get_ast_type(&self) -> ASTType {
        match self {
            FunctionCallExpr::FunctionCallExpr(ast) => ast.get_ast_type(),
            FunctionCallExpr::NewExpr(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FunctionCallExprBase {
    pub expression_base: Box<ExpressionBase>,
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub sec_start_offset: Option<i32>,
    pub public_key: Option<Box<HybridArgumentIdf>>,
}
// impl IntoExpression for FunctionCallExprBase {
//     fn into_expr(self) -> Expression {
//         Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(self))
//     }
// }
impl IntoAST for FunctionCallExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::FunctionCallExpr(
            FunctionCallExpr::FunctionCallExpr(self),
        ))
    }
}
impl ASTInstanceOf for FunctionCallExprBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::FunctionCallExprBase
    }
}
impl FunctionCallExprBase {
    pub fn new(func: Expression, args: Vec<Expression>, sec_start_offset: Option<i32>) -> Self {
        Self {
            expression_base: Box::new(ExpressionBase::new()),
            func: Box::new(func),
            args,
            sec_start_offset,
            public_key: None,
        }
    }

    pub fn is_cast(&self) -> bool {
        // isinstance(self.func, LocationExpr) && isinstance(self.func.target, (ContractDefinition, EnumDefinition))
        if let Expression::TupleOrLocationExpr(tole) = *self.func.clone() {
            if let TupleOrLocationExpr::LocationExpr(le) = tole {
                let target = match le {
                    LocationExpr::IdentifierExpr(ie) => ie.location_expr_base.target.clone(),
                    LocationExpr::MemberAccessExpr(ie) => ie.location_expr_base.target.clone(),
                    LocationExpr::IndexExpr(ie) => ie.location_expr_base.target.clone(),
                    LocationExpr::SliceExpr(ie) => ie.location_expr_base.target.clone(),
                    _ => None,
                };
                if target.is_some()
                    && is_instances(
                        &*target.unwrap(),
                        vec![ASTType::ContractDefinition, ASTType::EnumDefinition],
                    )
                {
                    return true;
                }
            }
        }
        false
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}

impl ASTChildren for FunctionCallExprBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(*self.func.clone()));
        self.args.iter().for_each(|arg| {
            cb.add_child(AST::Expression(arg.clone()));
        });
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for NewExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::NewExpr
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
                AnnotatedTypeName::new(tn, None, String::from("NON_HOMOMORPHIC")),
            );
        }

        selfs
    }
}
impl ASTChildren for NewExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::AnnotatedTypeName(self.annotated_type.clone()));
        self.function_call_expr_base.args.iter().for_each(|arg| {
            cb.add_child(AST::Expression(arg.clone()));
        });
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PrimitiveCastExpr {
    pub expression_base: Box<ExpressionBase>,
    pub elem_type: Box<TypeName>,
    pub expr: Box<Expression>,
    pub is_implicit: bool,
}
impl IntoAST for PrimitiveCastExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::PrimitiveCastExpr(self))
    }
}
impl ASTInstanceOf for PrimitiveCastExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::PrimitiveCastExpr
    }
}
impl PrimitiveCastExpr {
    pub fn new(elem_type: TypeName, expr: Expression, is_implicit: bool) -> Self {
        Self {
            expression_base: Box::new(ExpressionBase::new()),
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
impl ASTChildren for PrimitiveCastExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::TypeName(*self.elem_type.clone()));
        cb.add_child(AST::Expression(*self.expr.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl IntoAST for LiteralExpr {
    fn into_ast(self) -> AST {
        match self {
            LiteralExpr::BooleanLiteralExpr(ast) => ast.into_ast(),
            LiteralExpr::NumberLiteralExpr(ast) => ast.into_ast(),
            LiteralExpr::StringLiteralExpr(ast) => ast.into_ast(),
            LiteralExpr::ArrayLiteralExpr(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for LiteralExpr {
    fn get_ast_type(&self) -> ASTType {
        match self {
            LiteralExpr::BooleanLiteralExpr(ast) => ast.get_ast_type(),
            LiteralExpr::NumberLiteralExpr(ast) => ast.get_ast_type(),
            LiteralExpr::StringLiteralExpr(ast) => ast.get_ast_type(),
            LiteralExpr::ArrayLiteralExpr(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BooleanLiteralExpr {
    pub literal_expr_base: Box<LiteralExprBase>,
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
impl ASTInstanceOf for BooleanLiteralExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::BooleanLiteralExpr
    }
}
impl BooleanLiteralExpr {
    pub fn new(value: bool) -> Self {
        Self {
            literal_expr_base: Box::new(LiteralExprBase::new()),
            value,
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(
                    BooleanLiteralType::new(value),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
            ))),
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NumberLiteralExpr {
    pub literal_expr_base: Box<LiteralExprBase>,
    pub value: i32,
    pub value_string: Option<String>,
    pub was_hex: bool,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
// impl IntoExpression for NumberLiteralExpr {
//     fn into_expr(self) -> Expression {
//         Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(self))
//     }
// }
impl IntoAST for NumberLiteralExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
            self,
        )))
    }
}
impl ASTInstanceOf for NumberLiteralExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::NumberLiteralExpr
    }
}
impl NumberLiteralExpr {
    pub fn new(value: i32, was_hex: bool) -> Self {
        Self {
            literal_expr_base: Box::new(LiteralExprBase::new()),
            value,
            value_string: None,
            was_hex,
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                    NumberTypeName::NumberLiteralType(NumberLiteralType::new(
                        NumberLiteralTypeUnion::I32(value),
                    )),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
            ))),
        }
    }
    pub fn new_string(value_string: String) -> Self {
        Self {
            literal_expr_base: Box::new(LiteralExprBase::new()),
            value: 0,
            value_string: Some(value_string.clone()),
            was_hex: false,
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                    NumberTypeName::NumberLiteralType(NumberLiteralType::new(
                        NumberLiteralTypeUnion::String(value_string),
                    )),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
            ))),
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
    pub fn value(&self) -> i32 {
        0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for StringLiteralExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::StringLiteralExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl IntoAST for ArrayLiteralExpr {
    fn into_ast(self) -> AST {
        match self {
            ArrayLiteralExpr::ArrayLiteralExpr(ast) => ast.into_ast(),
            ArrayLiteralExpr::KeyLiteralExpr(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for ArrayLiteralExpr {
    fn get_ast_type(&self) -> ASTType {
        match self {
            ArrayLiteralExpr::ArrayLiteralExpr(ast) => ast.get_ast_type(),
            ArrayLiteralExpr::KeyLiteralExpr(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ArrayLiteralExprBase {
    pub literal_expr_base: Box<LiteralExprBase>,
    pub values: Vec<Expression>,
}
impl IntoAST for ArrayLiteralExprBase {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(
            ArrayLiteralExpr::ArrayLiteralExpr(self),
        )))
    }
}
impl ASTInstanceOf for ArrayLiteralExprBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ArrayLiteralExprBase
    }
}
impl ArrayLiteralExprBase {
    pub fn new(values: Vec<Expression>) -> Self {
        Self {
            literal_expr_base: Box::new(LiteralExprBase::new()),
            values,
        }
    }
    pub fn as_type(&self, t: AST) -> Self {
        let mut selfs = self.clone();
        if let AST::AnnotatedTypeName(at) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(at);
        } else if let AST::TypeName(tn) = t {
            selfs.literal_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
impl ASTChildren for ArrayLiteralExprBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.values.iter().for_each(|value| {
            cb.add_child(AST::Expression(value.clone()));
        });
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for KeyLiteralExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::KeyLiteralExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum TupleOrLocationExpr {
    TupleExpr(TupleExpr),
    LocationExpr(LocationExpr),
}

impl IntoAST for TupleOrLocationExpr {
    fn into_ast(self) -> AST {
        match self {
            TupleOrLocationExpr::TupleExpr(ast) => ast.into_ast(),
            TupleOrLocationExpr::LocationExpr(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for TupleOrLocationExpr {
    fn get_ast_type(&self) -> ASTType {
        match self {
            TupleOrLocationExpr::TupleExpr(ast) => ast.get_ast_type(),
            TupleOrLocationExpr::LocationExpr(ast) => ast.get_ast_type(),
        }
    }
}
impl TupleOrLocationExpr {
    pub fn is_lvalue(&self) -> bool {
        let parent = match self {
            TupleOrLocationExpr::TupleExpr(te) => te
                .tuple_or_location_expr_base
                .expression_base
                .ast_base
                .parent
                .clone()
                .map(|p| *p),
            TupleOrLocationExpr::LocationExpr(te) => te.parent(),
        };
        if let Some(AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(a)),
        ))) = &parent
        {
            return self == &(*a.lhs.as_ref().unwrap()).tuple_or_location_expr().unwrap();
        }
        if let Some(AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IndexExpr(ie)),
        ))) = &parent
        {
            if self
                == &ie
                    .arr
                    .clone()
                    .unwrap()
                    .into_ast()
                    .tuple_or_location_expr()
                    .unwrap()
            {
                return parent
                    .unwrap()
                    .tuple_or_location_expr()
                    .unwrap()
                    .is_lvalue();
            }
        }
        if let Some(AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::MemberAccessExpr(ie)),
        ))) = &parent
        {
            if self
                == &ie
                    .expr
                    .clone()
                    .unwrap()
                    .into_ast()
                    .tuple_or_location_expr()
                    .unwrap()
            {
                return parent
                    .unwrap()
                    .tuple_or_location_expr()
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleExpr {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub elements: Vec<Expression>,
}
// impl IntoExpression for TupleExpr {
//     fn into_expr(self) -> Expression {
//         Expression::TupleOrLocationExpr(
//             TupleOrLocationExpr::TupleExpr(self),
//         )
//     }
// }
impl IntoAST for TupleExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::TupleExpr(self),
        ))
    }
}
impl ASTInstanceOf for TupleExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::TupleExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
impl ASTChildren for TupleExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.elements.iter().for_each(|element| {
            cb.add_child(AST::Expression(element.clone()));
        });
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum LocationExpr {
    IdentifierExpr(IdentifierExpr),
    MemberAccessExpr(MemberAccessExpr),
    IndexExpr(IndexExpr),
    SliceExpr(SliceExpr),
}

impl IntoAST for LocationExpr {
    fn into_ast(self) -> AST {
        match self {
            LocationExpr::IdentifierExpr(ast) => ast.into_ast(),
            LocationExpr::MemberAccessExpr(ast) => ast.into_ast(),
            LocationExpr::IndexExpr(ast) => ast.into_ast(),
            LocationExpr::SliceExpr(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for LocationExpr {
    fn get_ast_type(&self) -> ASTType {
        match self {
            LocationExpr::IdentifierExpr(ast) => ast.get_ast_type(),
            LocationExpr::MemberAccessExpr(ast) => ast.get_ast_type(),
            LocationExpr::IndexExpr(ast) => ast.get_ast_type(),
            LocationExpr::SliceExpr(ast) => ast.get_ast_type(),
        }
    }
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
    pub fn statement(&self) -> Option<Box<Statement>> {
        None
    }
    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
    pub fn parent(&self) -> Option<AST> {
        match self {
            LocationExpr::IdentifierExpr(ie) => ie
                .clone()
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .ast_base
                .parent
                .map(|p| *p),
            LocationExpr::MemberAccessExpr(mae) => mae
                .clone()
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .ast_base
                .parent
                .map(|p| *p),
            LocationExpr::IndexExpr(ie) => ie
                .clone()
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .ast_base
                .parent
                .map(|p| *p),
            LocationExpr::SliceExpr(se) => se
                .clone()
                .location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .ast_base
                .parent
                .map(|p| *p),
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
            match *type_name {
                TypeName::Array(a) => a.value_type().map(|v| v.to_ast()),
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
    pub fn target(&self) -> Option<Box<AST>> {
        None
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierExpr {
    pub location_expr_base: LocationExprBase,
    pub idf: Box<Identifier>,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
// impl IntoExpression for IdentifierExpr {
//     fn into_expr(self) -> Expression {
//         Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
//             LocationExpr::IdentifierExpr(self),
//         ))
//     }
// }
impl IntoAST for IdentifierExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IdentifierExpr(self)),
        ))
    }
}
impl ASTInstanceOf for IdentifierExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::IdentifierExpr
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
        self.location_expr_base
            .target
            .clone()
            .map(|t| t.annotated_type().unwrap())
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            )));
        }

        selfs
    }
}
impl ASTChildren for IdentifierExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Identifier(*self.idf.clone()));
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for MemberAccessExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::MemberAccessExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
impl ASTChildren for MemberAccessExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(*self.expr.clone().unwrap()),
        )));
        cb.add_child(AST::Identifier(*self.member.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for IndexExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::IndexExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
impl ASTChildren for IndexExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(*self.arr.clone().unwrap()),
        )));
        cb.add_child(AST::Expression(*self.key.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for SliceExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::SliceExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MeExpr {
    pub expression_base: ExpressionBase,
    pub name: String,
}
// impl IntoExpression for MeExpr {
//     fn into_expr(self) -> Expression {
//         Expression::MeExpr(self)
//     }
// }
impl IntoAST for MeExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::MeExpr(self))
    }
}
impl ASTInstanceOf for MeExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::MeExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AllExpr {
    pub expression_base: ExpressionBase,
    pub name: String,
}
impl IntoAST for AllExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::AllExpr(self))
    }
}
impl ASTInstanceOf for AllExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::AllExpr
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum ReclassifyExpr {
    ReclassifyExpr(ReclassifyExprBase),
    RehomExpr(RehomExpr),
    EncryptionExpression(EncryptionExpression),
}

impl ReclassifyExpr {
    pub fn statement(&self) -> Option<Box<Statement>> {
        None
    }
    pub fn set_annotated_type(&mut self, annotated_type: AnnotatedTypeName) {}
    pub fn analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_homomorphism(&mut self, homomorphism: String) {}
    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
    pub fn as_type(&self, t: AST) -> Self {
        match self {
            ReclassifyExpr::ReclassifyExpr(ast) => ReclassifyExpr::ReclassifyExpr(ast.as_type(t)),
            ReclassifyExpr::RehomExpr(ast) => ReclassifyExpr::RehomExpr(ast.as_type(t)),
            ReclassifyExpr::EncryptionExpression(ast) => {
                ReclassifyExpr::EncryptionExpression(ast.as_type(t))
            }
        }
    }
    pub fn expr(&self) -> Option<Expression> {
        None
    }
    pub fn privacy(&self) -> Option<Expression> {
        None
    }
    pub fn homomorphism(&self) -> Option<String> {
        None
    }
    pub fn func_name(&self) -> String {
        String::from("reveal")
    }
}
impl IntoAST for ReclassifyExpr {
    fn into_ast(self) -> AST {
        match self {
            ReclassifyExpr::ReclassifyExpr(ast) => ast.into_ast(),
            ReclassifyExpr::RehomExpr(ast) => ast.into_ast(),
            ReclassifyExpr::EncryptionExpression(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for ReclassifyExpr {
    fn get_ast_type(&self) -> ASTType {
        match self {
            ReclassifyExpr::ReclassifyExpr(ast) => ast.get_ast_type(),
            ReclassifyExpr::RehomExpr(ast) => ast.get_ast_type(),
            ReclassifyExpr::EncryptionExpression(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for ReclassifyExprBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ReclassifyExprBase
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
                tn,
                None,
                String::from("NON_HOMOMORPHIC"),
            ));
        }

        selfs
    }
}
impl ASTChildren for ReclassifyExprBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(*self.expr.clone()));
        cb.add_child(AST::Expression(*self.privacy.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RehomExpr {
    pub reclassify_expr_base: ReclassifyExprBase,
}
impl IntoAST for RehomExpr {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::ReclassifyExpr(ReclassifyExpr::RehomExpr(self)))
    }
}
impl ASTInstanceOf for RehomExpr {
    fn get_ast_type(&self) -> ASTType {
        ASTType::RehomExpr
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
                AnnotatedTypeName::new(tn, None, String::from("NON_HOMOMORPHIC")),
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for HybridArgumentIdf {
    fn get_ast_type(&self) -> ASTType {
        ASTType::HybridArgumentIdf
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
                *self
                    .corresponding_priv_expression
                    .as_ref()
                    .unwrap()
                    .annotated_type()
                    .unwrap()
                    .type_name
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
                    .unwrap()
                    .type_name
                    .bool_value(),
            )
            .into_ast()
        } else if self.arg_type == HybridArgType::TmpCircuitVal
            && self.corresponding_priv_expression.is_some()
            && if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::NumberLiteralType(_),
            )) = *self
                .corresponding_priv_expression
                .as_ref()
                .unwrap()
                .annotated_type()
                .unwrap()
                .type_name
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
                    .unwrap()
                    .type_name
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
            idf.ast_base.parent = parent.clone();
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
        if let TypeName::Array(a) = *self.t.clone() {
            SliceExpr::new(
                self.get_loc_expr(None).location_expr(),
                None,
                0,
                self.t.size_in_uints(),
            )
            .arr
            .unwrap()
            .assign(self.serialized_loc.to_expr())
        } else if let Some(base) = &base {
            self.get_loc_expr(None).location_expr().unwrap().assign(
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
                    .expr()
                    .unwrap(),
            )
        } else {
            self.get_loc_expr(None).location_expr().unwrap().assign(
                Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
                    LocationExpr::IndexExpr(
                        LocationExpr::IdentifierExpr(src).index(ExprUnion::I32(start_offset)),
                    ),
                ))
                .explicitly_converted(*self.t.clone())
                .expr()
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
        if let TypeName::Array(t) = *self.t.clone() {
            let loc = self.get_loc_expr(None).location_expr();
            self.serialized_loc
                .arr
                .as_mut()
                .unwrap()
                .assign(SliceExpr::new(loc, None, 0, self.t.size_in_uints()).to_expr())
        } else {
            let expr = self.get_loc_expr(None);
            let expr = if self.t.is_signed_numeric() {
                // Cast to same size uint to prevent sign extension
                expr.expr()
                    .unwrap()
                    .explicitly_converted(TypeName::ElementaryTypeName(
                        ElementaryTypeName::NumberTypeName(NumberTypeName::UintTypeName(
                            UintTypeName::new(format!("uint{}", self.t.elem_bitwidth())),
                        )),
                    ))
            } else if self.t.is_numeric() && self.t.elem_bitwidth() == 256 {
                expr.expr()
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
                expr.expr()
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
                .assign(expr.expr().unwrap())
            } else {
                LocationExpr::IndexExpr(
                    LocationExpr::IdentifierExpr(tgt.clone()).index(ExprUnion::I32(start_offset)),
                )
                .assign(expr.expr().unwrap())
            }
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    pub fn ast_base_mut(&mut self) -> Option<&mut ASTBase> {
        None
    }
    pub fn corresponding_priv_expression(&self) -> Option<Expression> {
        None
    }
    pub fn decl_var(&self, t: AST, expr: Option<Expression>) -> Option<AST> {
        None
    }
    pub fn arg_type(&self) -> HybridArgType {
        HybridArgType::PubContractVal
    }
    pub fn identifier(name: &str) -> Self {
        Self::Identifier(IdentifierBase::new(String::from(name)))
    }
    pub fn name(&self) -> String {
        String::new()
    }
    pub fn parent(&self) -> Option<AST> {
        None
    }
    pub fn t(&self) -> Option<TypeName> {
        None
    }
}
impl IntoAST for Identifier {
    fn into_ast(self) -> AST {
        match self {
            Identifier::Identifier(ast) => ast.into_ast(),
            Identifier::HybridArgumentIdf(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for Identifier {
    fn get_ast_type(&self) -> ASTType {
        match self {
            Identifier::Identifier(ast) => ast.get_ast_type(),
            Identifier::HybridArgumentIdf(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for EncryptionExpression {
    fn get_ast_type(&self) -> ASTType {
        ASTType::EncryptionExpression
    }
}
impl EncryptionExpression {
    pub fn new(expr: Expression, privacy: AST, homomorphism: Option<String>) -> Self {
        let privacy = privacy.expr().unwrap();
        let annotated_type = Some(AnnotatedTypeName::cipher_type(
            expr.annotated_type().unwrap(),
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
                AnnotatedTypeName::new(tn, None, String::from("NON_HOMOMORPHIC")),
            );
        }

        selfs
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl Statement {
    pub fn statement_base_mut(&mut self) -> Option<&mut StatementBase> {
        None
    }
    pub fn statement_base(&self) -> Option<&StatementBase> {
        None
    }

    pub fn after_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_before_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn pre_statements(&self) -> Vec<AST> {
        vec![]
    }
    pub fn add_pre_statement(&mut self, statement: Statement) {}
    pub fn extend_pre_statements(&mut self, statement: Vec<Statement>) {}
    pub fn append_pre_statements(&mut self, statement: &mut Vec<Statement>) {}
    pub fn drain_pre_statements(&mut self) -> Vec<AST> {
        vec![]
    }
    pub fn modified_values(&self) -> BTreeSet<InstanceTarget> {
        BTreeSet::new()
    }
    pub fn function(&self) -> Option<Box<ConstructorOrFunctionDefinition>> {
        None
    }
    pub fn before_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn line(&self) -> i32 {
        0
    }
    pub fn column(&self) -> i32 {
        0
    }
    pub fn original_code(&self) -> Vec<String> {
        vec![]
    }
}
impl IntoAST for Statement {
    fn into_ast(self) -> AST {
        match self {
            Statement::CircuitDirectiveStatement(ast) => ast.into_ast(),
            Statement::IfStatement(ast) => ast.into_ast(),
            Statement::WhileStatement(ast) => ast.into_ast(),
            Statement::DoWhileStatement(ast) => ast.into_ast(),
            Statement::ForStatement(ast) => ast.into_ast(),
            Statement::BreakStatement(ast) => ast.into_ast(),
            Statement::ContinueStatement(ast) => ast.into_ast(),
            Statement::ReturnStatement(ast) => ast.into_ast(),
            Statement::SimpleStatement(ast) => ast.into_ast(),
            Statement::StatementList(ast) => ast.into_ast(),
            Statement::CircuitStatement(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for Statement {
    fn get_ast_type(&self) -> ASTType {
        match self {
            Statement::CircuitDirectiveStatement(ast) => ast.get_ast_type(),
            Statement::IfStatement(ast) => ast.get_ast_type(),
            Statement::WhileStatement(ast) => ast.get_ast_type(),
            Statement::DoWhileStatement(ast) => ast.get_ast_type(),
            Statement::ForStatement(ast) => ast.get_ast_type(),
            Statement::BreakStatement(ast) => ast.get_ast_type(),
            Statement::ContinueStatement(ast) => ast.get_ast_type(),
            Statement::ReturnStatement(ast) => ast.get_ast_type(),
            Statement::SimpleStatement(ast) => ast.get_ast_type(),
            Statement::StatementList(ast) => ast.get_ast_type(),
            Statement::CircuitStatement(ast) => ast.get_ast_type(),
        }
    }
}
impl ASTChildren for Statement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        match self {
            Statement::CircuitDirectiveStatement(ast) => ast.process_children(cb),
            Statement::IfStatement(ast) => ast.process_children(cb),
            Statement::WhileStatement(ast) => ast.process_children(cb),
            Statement::DoWhileStatement(ast) => ast.process_children(cb),
            Statement::ForStatement(ast) => ast.process_children(cb),
            Statement::BreakStatement(ast) => ast.process_children(cb),
            Statement::ContinueStatement(ast) => ast.process_children(cb),
            Statement::ReturnStatement(ast) => ast.process_children(cb),
            Statement::SimpleStatement(ast) => ast.process_children(cb),
            Statement::StatementList(ast) => ast.process_children(cb),
            Statement::CircuitStatement(ast) => ast.process_children(cb),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CircuitDirectiveStatement {
    CircuitComputationStatement(CircuitComputationStatement),
    EnterPrivateKeyStatement(EnterPrivateKeyStatement),
}
impl ASTChildren for CircuitDirectiveStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        match self {
            Self::CircuitComputationStatement(ast) => ast.process_children(cb),
            Self::EnterPrivateKeyStatement(ast) => ast.process_children(cb),
            _ => {}
        }
    }
}
impl IntoAST for CircuitDirectiveStatement {
    fn into_ast(self) -> AST {
        match self {
            CircuitDirectiveStatement::CircuitComputationStatement(ast) => ast.into_ast(),
            CircuitDirectiveStatement::EnterPrivateKeyStatement(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for CircuitDirectiveStatement {
    fn get_ast_type(&self) -> ASTType {
        match self {
            CircuitDirectiveStatement::CircuitComputationStatement(ast) => ast.get_ast_type(),
            CircuitDirectiveStatement::EnterPrivateKeyStatement(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitComputationStatement {
    pub circuit_directive_statement_base: CircuitDirectiveStatementBase,
    pub idf: HybridArgumentIdf,
}

impl ASTChildren for CircuitComputationStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl IntoAST for CircuitComputationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitDirectiveStatement(
            CircuitDirectiveStatement::CircuitComputationStatement(self),
        ))
    }
}
impl ASTInstanceOf for CircuitComputationStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::CircuitComputationStatement
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnterPrivateKeyStatement {
    pub circuit_directive_statement_base: CircuitDirectiveStatementBase,
    pub crypto_params: CryptoParams,
}
impl ASTChildren for EnterPrivateKeyStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl IntoAST for EnterPrivateKeyStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitDirectiveStatement(
            CircuitDirectiveStatement::EnterPrivateKeyStatement(self),
        ))
    }
}
impl ASTInstanceOf for EnterPrivateKeyStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::EnterPrivateKeyStatement
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IfStatement {
    pub statement_base: StatementBase,
    pub condition: Expression,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}
// impl IntoStatement for IfStatement {
//     fn into_statement(self) -> Statement {
//         Statement::IfStatement(self)
//     }
// }
impl IntoAST for IfStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::IfStatement(self))
    }
}
impl ASTInstanceOf for IfStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::IfStatement
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
        cb.add_child(AST::Expression(self.condition.clone()));
        cb.add_child(AST::Statement(Statement::StatementList(
            StatementList::Block(self.then_branch.clone()),
        )));
        cb.add_child(AST::Statement(Statement::StatementList(
            StatementList::Block(self.then_branch.clone()),
        )));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for WhileStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::WhileStatement
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
        cb.add_child(AST::Expression(self.condition.clone()));
        cb.add_child(AST::Statement(Statement::StatementList(
            StatementList::Block(self.body.clone()),
        )));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for DoWhileStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::DoWhileStatement
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
        cb.add_child(AST::Statement(Statement::StatementList(
            StatementList::Block(self.body.clone()),
        )));
        cb.add_child(AST::Expression(self.condition.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for ForStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ForStatement
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
    pub fn ast_base_mut(&mut self) -> Option<&mut ASTBase> {
        None
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
            cb.add_child(AST::Statement(Statement::SimpleStatement(*init.clone())));
        }

        cb.add_child(AST::Expression(self.condition.clone()));
        if let Some(update) = &self.update {
            cb.add_child(AST::Statement(Statement::SimpleStatement(*update.clone())));
        }
        cb.add_child(AST::Statement(Statement::StatementList(
            StatementList::Block(self.body.clone()),
        )));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BreakStatement {
    pub statement_base: StatementBase,
}
impl ASTChildren for BreakStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl IntoAST for BreakStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::BreakStatement(self))
    }
}
impl ASTInstanceOf for BreakStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::BreakStatement
    }
}
impl BreakStatement {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ContinueStatement {
    pub statement_base: StatementBase,
}
impl ASTChildren for ContinueStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl IntoAST for ContinueStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ContinueStatement(self))
    }
}
impl ASTInstanceOf for ContinueStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ContinueStatement
    }
}
impl ContinueStatement {
    pub fn new() -> Self {
        Self {
            statement_base: StatementBase::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReturnStatement {
    pub statement_base: StatementBase,
    pub expr: Option<Expression>,
}
impl IntoAST for ReturnStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::ReturnStatement(self))
    }
}
impl ASTInstanceOf for ReturnStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ReturnStatement
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
        cb.add_child(AST::Expression(self.expr.clone().unwrap()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleStatement {
    ExpressionStatement(ExpressionStatement),
    RequireStatement(RequireStatement),
    AssignmentStatement(AssignmentStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}
// impl IntoStatement for SimpleStatement {
//     fn into_statement(self) -> Statement {
//         match self {
//             SimpleStatement::ExpressionStatement(ast) => ast.into_statement(),
//             SimpleStatement::RequireStatement(ast) => ast.into_statement(),
//             SimpleStatement::AssignmentStatement(ast) => ast.into_statement(),
//             SimpleStatement::VariableDeclarationStatement(ast) => ast.into_statement(),
//         }
//     }
// }
impl ASTChildren for SimpleStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl SimpleStatement {
    pub fn ast_base_mut(&mut self) -> Option<&mut ASTBase> {
        None
    }

    pub fn before_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn after_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_after_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn pre_statements(&self) -> Vec<AST> {
        vec![]
    }
    pub fn set_before_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn set_lhs(&mut self, lhs: AST) {}
    pub fn set_rhs(&mut self, rhs: Expression) {}
}
impl IntoAST for SimpleStatement {
    fn into_ast(self) -> AST {
        match self {
            SimpleStatement::ExpressionStatement(ast) => ast.into_ast(),
            SimpleStatement::RequireStatement(ast) => ast.into_ast(),
            SimpleStatement::AssignmentStatement(ast) => ast.into_ast(),
            SimpleStatement::VariableDeclarationStatement(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for SimpleStatement {
    fn get_ast_type(&self) -> ASTType {
        match self {
            SimpleStatement::ExpressionStatement(ast) => ast.get_ast_type(),
            SimpleStatement::RequireStatement(ast) => ast.get_ast_type(),
            SimpleStatement::AssignmentStatement(ast) => ast.get_ast_type(),
            SimpleStatement::VariableDeclarationStatement(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub expr: Expression,
}
// impl IntoStatement for ExpressionStatement {
//     fn into_statement(self) -> Statement {
//         Statement::SimpleStatement(SimpleStatement::ExpressionStatement(self))
//     }
// }
impl IntoAST for ExpressionStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::ExpressionStatement(self),
        ))
    }
}
impl ASTInstanceOf for ExpressionStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ExpressionStatement
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
        cb.add_child(AST::Expression(self.expr.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RequireStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub condition: Expression,
    pub unmodified_code: String,
}
// impl IntoStatement for RequireStatement {
//     fn into_statement(self) -> Statement {
//         Statement::SimpleStatement(SimpleStatement::RequireStatement(self))
//     }
// }
impl IntoAST for RequireStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::RequireStatement(self),
        ))
    }
}
impl ASTInstanceOf for RequireStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::RequireStatement
    }
}
impl RequireStatement {
    pub fn new(condition: Expression, unmodified_code: Option<String>) -> Self {
        Self {
            simple_statement_base: SimpleStatementBase::new(),
            condition,
            unmodified_code: if let Some(unmodified_code) = unmodified_code {
                unmodified_code
            } else {
                String::new() //self.code()
            },
        }
    }
}
impl ASTChildren for RequireStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(self.condition.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AssignmentStatement {
    AssignmentStatement(AssignmentStatementBase),
    CircuitInputStatement(CircuitInputStatement),
}
// impl IntoStatement for AssignmentStatement {
//     fn into_statement(self) -> Statement {
//         match self {
//             Self::AssignmentStatement(ast) => ast.into_statement(),
//             Self::CircuitInputStatement(ast) => ast.into_statement(),
//         }
//     }
// }
impl ASTChildren for AssignmentStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl AssignmentStatement {
    pub fn before_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_after_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn set_before_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn function(&self) -> Option<Box<ConstructorOrFunctionDefinition>> {
        None
    }
    pub fn modified_values(&self) -> BTreeSet<InstanceTarget> {
        BTreeSet::new()
    }
    pub fn lhs(&self) -> Option<AST> {
        None
    }
    pub fn set_lhs(&mut self, lhs: Option<AST>) {}
    pub fn set_rhs(&mut self, rhs: Option<Expression>) {}
    pub fn set_pre_statements(&mut self, pre_statements: Vec<AST>) {}
    pub fn rhs(&self) -> Option<Expression> {
        None
    }
    pub fn op(&self) -> Option<String> {
        None
    }
}
impl IntoAST for AssignmentStatement {
    fn into_ast(self) -> AST {
        match self {
            AssignmentStatement::AssignmentStatement(ast) => ast.into_ast(),
            AssignmentStatement::CircuitInputStatement(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for AssignmentStatement {
    fn get_ast_type(&self) -> ASTType {
        match self {
            AssignmentStatement::AssignmentStatement(ast) => ast.get_ast_type(),
            AssignmentStatement::CircuitInputStatement(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AssignmentStatementBase {
    pub simple_statement_base: SimpleStatementBase,
    pub lhs: Option<Box<AST>>,
    pub rhs: Option<Expression>,
    pub op: String,
}
// impl IntoStatement for AssignmentStatementBase {
//     fn into_statement(self) -> Statement {
//         Statement::SimpleStatement(SimpleStatement::AssignmentStatement(
//             AssignmentStatement::AssignmentStatement(self),
//         ))
//     }
// }
impl IntoAST for AssignmentStatementBase {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(self)),
        ))
    }
}
impl ASTInstanceOf for AssignmentStatementBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::AssignmentStatementBase
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
        if let Some(te) = self.lhs.clone().unwrap().tuple_expr() {
            cb.add_child(te.into_ast());
        }
        if let Some(le) = self.lhs.clone().unwrap().location_expr() {
            cb.add_child(le.into_ast());
        }

        cb.add_child(self.rhs.clone().unwrap().into_ast());
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitInputStatement {
    pub assignment_statement_base: AssignmentStatementBase,
}
// impl IntoStatement for CircuitInputStatement {
//     fn into_statement(self) -> Statement {
//         Statement::SimpleStatement(SimpleStatement::AssignmentStatement(
//             AssignmentStatement::CircuitInputStatement(self),
//         ))
//     }
// }
impl IntoAST for CircuitInputStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::CircuitInputStatement(self)),
        ))
    }
}
impl ASTInstanceOf for CircuitInputStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::CircuitInputStatement
    }
}
impl CircuitInputStatement {
    pub fn new(lhs: AST, rhs: Expression, op: Option<String>) -> Self {
        Self {
            assignment_statement_base: AssignmentStatementBase::new(Some(lhs), Some(rhs), op),
        }
    }
}
impl ASTChildren for CircuitInputStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(te) = self
            .assignment_statement_base
            .lhs
            .clone()
            .unwrap()
            .tuple_expr()
        {
            cb.add_child(te.into_ast());
        }
        if let Some(le) = self
            .assignment_statement_base
            .lhs
            .clone()
            .unwrap()
            .location_expr()
        {
            cb.add_child(le.into_ast());
        }
        cb.add_child(AST::Expression(
            self.assignment_statement_base.rhs.clone().unwrap(),
        ));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum StatementList {
    Block(Block),
    IndentBlock(IndentBlock),
    StatementList(StatementListBase),
}

impl ASTChildren for StatementList {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {}
}
impl IntoAST for StatementList {
    fn into_ast(self) -> AST {
        match self {
            StatementList::Block(ast) => ast.into_ast(),
            StatementList::IndentBlock(ast) => ast.into_ast(),
            StatementList::StatementList(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for StatementList {
    fn get_ast_type(&self) -> ASTType {
        match self {
            StatementList::Block(ast) => ast.get_ast_type(),
            StatementList::IndentBlock(ast) => ast.get_ast_type(),
            StatementList::StatementList(ast) => ast.get_ast_type(),
        }
    }
}
impl StatementList {
    pub fn ast_base_mut(&mut self) -> Option<&mut ASTBase> {
        None
    }
    pub fn before_analysis(&self) -> Option<PartitionState<AST>> {
        None
    }
    pub fn set_after_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
    pub fn set_statements(&mut self, statements: Vec<AST>) {}
    pub fn statements(&self) -> Vec<AST> {
        vec![]
    }
    pub fn get_item(&self, key: i32) -> AST {
        match self {
            StatementList::Block(sl) => {
                assert!(self.statements().len() > key as usize);
                self.statements()[key as usize].clone()
            }
            StatementList::IndentBlock(sl) => {
                assert!(self.statements().len() > key as usize);
                self.statements()[key as usize].clone()
            }
            StatementList::StatementList(sl) => {
                assert!(self.statements().len() > key as usize);
                self.statements()[key as usize].clone()
            }
        }
    }

    pub fn contains(&self, stmt: &AST) -> bool {
        if self.statements().contains(stmt) {
            return true;
        }
        for s in self.statements() {
            if let AST::Statement(Statement::StatementList(StatementList::Block(sl))) = &s {
                if sl.statement_list_base.statements.contains(stmt) {
                    return true;
                }
            }
            if let AST::Statement(Statement::StatementList(StatementList::IndentBlock(sl))) = &s {
                if sl.statement_list_base.statements.contains(stmt) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for StatementListBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::StatementListBase
    }
}
impl ASTChildren for StatementListBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.statements.iter().for_each(|statement| {
            cb.add_child(statement.clone());
        });
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Block {
    pub statement_list_base: StatementListBase,
    pub was_single_statement: bool,
}
impl IntoAST for Block {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::Block(self)))
    }
}
impl ASTInstanceOf for Block {
    fn get_ast_type(&self) -> ASTType {
        ASTType::Block
    }
}
impl Block {
    pub fn new(statements: Vec<AST>, was_single_statement: bool) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
            was_single_statement,
        }
    }
    pub fn set_before_analysis(&mut self, before_analysis: Option<PartitionState<AST>>) {}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IndentBlock {
    pub statement_list_base: StatementListBase,
}
impl IntoAST for IndentBlock {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::StatementList(StatementList::IndentBlock(self)))
    }
}
impl ASTInstanceOf for IndentBlock {
    fn get_ast_type(&self) -> ASTType {
        ASTType::IndentBlock
    }
}
impl IndentBlock {
    pub fn new(statements: Vec<AST>) -> Self {
        Self {
            statement_list_base: StatementListBase::new(statements, false),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
            TypeName::Literal(ast) => ASTType::Literal,
        }
    }
}
impl TypeName {
    pub fn tuple_type(&self) -> Option<TupleType> {
        None
    }
    pub fn parameters(&self) -> Vec<Parameter> {
        vec![]
    }
    pub fn return_parameters(&self) -> Vec<Parameter> {
        vec![]
    }
    pub fn set_parent(&mut self, parent: Option<Box<AST>>) {}
    pub fn has_key_label(&self) -> bool {
        false
    }
    pub fn value_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
    pub fn names(&self) -> Vec<Identifier> {
        vec![]
    }
    pub fn can_represent(&self, value: i32) -> bool
// """Return true if value can be represented by this type"""
    {
        // let elem_bitwidth = self.elem_bitwidth() as usize;
        // let lo = if self.signed {
        //     -(1 << elem_bitwidth - 1)
        // } else {
        //     0
        // };
        // let hi = if self.signed {
        //     1 << elem_bitwidth - 1
        // } else {
        //     1 << elem_bitwidth
        // };
        // lo <= value && value < hi
        true
    }
    pub fn to_abstract_type(&self) -> TypeName {
        if self.value() < 0 {
            TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::IntTypeName(IntTypeName::new(format!(
                    "i32{}",
                    self.elem_bitwidth()
                ))),
            ))
        } else {
            TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                NumberTypeName::UintTypeName(UintTypeName::new(format!(
                    "uint{}",
                    self.elem_bitwidth()
                ))),
            ))
        }
    }
    pub fn value(&self) -> i32 {
        0
    }
    pub fn bool_value(&self) -> bool {
        false
    }
    pub fn types(&self) -> Option<Vec<AnnotatedTypeName>> {
        None
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
    pub fn is_literal(&self) -> bool
// return isinstance(&self, (NumberLiteralType, BooleanLiteralType, EnumValueTypeName))
    {
        if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(_),
        ))
        | TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(_))
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(_)) = self
        {
            true
        } else {
            false
        }
    }
    pub fn is_address(&self) -> bool
// return isinstance(&self, (AddressTypeName, AddressPayableTypeName))
    {
        if let TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(_))
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(_)) = self
        {
            true
        } else {
            false
        }
    }
    pub fn is_primitive_type(&self) -> bool {
        if let TypeName::ElementaryTypeName(_)
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumTypeName(_))
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(_))
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(_))
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(_)) = self
        {
            true
        } else {
            false
        }
    }
    pub fn is_cipher(&self) -> bool {
        if let TypeName::Array(Array::CipherText(_)) = &self {
            true
        } else {
            false
        }
    }
    pub fn is_key(&self) -> bool {
        if let TypeName::Array(Array::Key(_)) = &self {
            true
        } else {
            false
        }
    }
    pub fn is_randomness(&self) -> bool {
        if let TypeName::Array(Array::Randomness(_)) = &self {
            true
        } else {
            false
        }
    }
    pub fn is_numeric(&self) -> bool {
        if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(_)) = self {
            true
        } else {
            false
        }
    }
    pub fn is_boolean(&self) -> bool {
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(_))
        | TypeName::ElementaryTypeName(ElementaryTypeName::BoolTypeName(_)) = &self
        {
            true
        } else {
            false
        }
    }
    pub fn signed(&self) -> bool {
        false
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
    pub fn combined_type(&self, other_type: TypeName, convert_literals: bool) -> Option<Self> {
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
            self.clone(),
            if let CombinedPrivacyUnion::AST(expr) = privacy_annotation {
                expr.map(|ast| ast.expr().unwrap())
            } else {
                None
            },
            String::from("NON_HOMOMORPHIC"),
        )
    }
    pub fn crypto_params(&self) -> Option<CryptoParams> {
        None
    }
    pub fn op(&self) -> Option<String> {
        None
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TypeNameBase {
    pub ast_base: ASTBase,
}
impl TypeNameBase {
    pub fn new() -> Self {
        Self {
            ast_base: ASTBase::new(),
        }
    }
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        true
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum ElementaryTypeName {
    NumberTypeName(NumberTypeName),
    BoolTypeName(BoolTypeName),
    BooleanLiteralType(BooleanLiteralType),
}

impl IntoAST for ElementaryTypeName {
    fn into_ast(self) -> AST {
        match self {
            ElementaryTypeName::NumberTypeName(ast) => ast.into_ast(),
            ElementaryTypeName::BoolTypeName(ast) => ast.into_ast(),
            ElementaryTypeName::BooleanLiteralType(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for ElementaryTypeName {
    fn get_ast_type(&self) -> ASTType {
        match self {
            ElementaryTypeName::NumberTypeName(ast) => ast.get_ast_type(),
            ElementaryTypeName::BoolTypeName(ast) => ast.get_ast_type(),
            ElementaryTypeName::BooleanLiteralType(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        true
    }
    pub fn combined_type(&self, other_type: TypeName, convert_literals: bool) -> Option<TypeName> {
        None
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for BoolTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::BoolTypeName
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for BooleanLiteralType {
    fn get_ast_type(&self) -> ASTType {
        ASTType::BooleanLiteralType
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
        self.elementary_type_name_base
            .implicitly_convertible_to(expected)
            || if let TypeName::ElementaryTypeName(ElementaryTypeName::BoolTypeName(_)) = &expected
            {
                true
            } else {
                false
            }
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
            self.elementary_type_name_base
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum NumberTypeName {
    NumberLiteralType(NumberLiteralType),
    IntTypeName(IntTypeName),
    UintTypeName(UintTypeName),
    NumberTypeNameBase(NumberTypeNameBase),
}

impl IntoAST for NumberTypeName {
    fn into_ast(self) -> AST {
        match self {
            NumberTypeName::NumberLiteralType(ast) => ast.into_ast(),
            NumberTypeName::IntTypeName(ast) => ast.into_ast(),
            NumberTypeName::UintTypeName(ast) => ast.into_ast(),
            NumberTypeName::NumberTypeNameBase(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for NumberTypeName {
    fn get_ast_type(&self) -> ASTType {
        match self {
            NumberTypeName::NumberLiteralType(ast) => ast.get_ast_type(),
            NumberTypeName::IntTypeName(ast) => ast.get_ast_type(),
            NumberTypeName::UintTypeName(ast) => ast.get_ast_type(),
            NumberTypeName::NumberTypeNameBase(ast) => ast.get_ast_type(),
        }
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
    pub fn can_represent(&self, value: i32) -> bool
// """Return true if value can be represented by this type"""
    {
        // let elem_bitwidth = self.elem_bitwidth() as usize;
        // let lo = if self.signed {
        //     -(1 << elem_bitwidth - 1)
        // } else {
        //     0
        // };
        // let hi = if self.signed {
        //     1 << elem_bitwidth - 1
        // } else {
        //     1 << elem_bitwidth
        // };
        // lo <= value && value < hi
        true
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for NumberTypeNameBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::NumberTypeNameBase
    }
}
impl NumberTypeNameBase {
    pub fn new(name: String, prefix: String, signed: bool, bitwidth: Option<i32>) -> Self {
        assert!(name.starts_with(&prefix));
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
        self.elementary_type_name_base
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for NumberLiteralType {
    fn get_ast_type(&self) -> ASTType {
        ASTType::NumberLiteralType
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
        if expected.is_numeric() && !expected.is_literal()
        // Allow implicit conversion only if it fits
        {
            expected.can_represent(self.value())
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
                    .combined_type(other_type.to_abstract_type(), convert_literals)
                    .unwrap()
            } else {
                TypeName::Literal(String::from("lit"))
            }
        } else {
            self.number_type_name_base
                .elementary_type_name_base
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for IntTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::IntTypeName
    }
}
impl IntTypeName {
    pub fn new(name: String) -> Self {
        Self {
            number_type_name_base: NumberTypeNameBase::new(name, String::from("i32"), true, None),
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for UintTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::UintTypeName
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum UserDefinedTypeName {
    EnumTypeName(EnumTypeName),
    EnumValueTypeName(EnumValueTypeName),
    StructTypeName(StructTypeName),
    ContractTypeName(ContractTypeName),
    AddressTypeName(AddressTypeName),
    AddressPayableTypeName(AddressPayableTypeName),
}
impl UserDefinedTypeName {
    pub fn set_type_name(&mut self, type_name: TypeName) {}
    pub fn parent(&self) -> Option<AST> {
        None
    }
    pub fn set_parent(&mut self, parent: Option<AST>) {
        match self {
            UserDefinedTypeName::EnumTypeName(ref mut ast) => {
                ast.user_defined_type_name_base
                    .type_name_base
                    .ast_base
                    .parent = parent.map(|p| Box::new(p));
            } //UserDefinedTypeName::EnumTypeName(ast)}
            UserDefinedTypeName::EnumValueTypeName(ref mut ast) => {
                ast.user_defined_type_name_base
                    .type_name_base
                    .ast_base
                    .parent = parent.map(|p| Box::new(p));
            } //UserDefinedTypeName::EnumValueTypeName(ast)}
            UserDefinedTypeName::StructTypeName(ref mut ast) => {
                ast.user_defined_type_name_base
                    .type_name_base
                    .ast_base
                    .parent = parent.map(|p| Box::new(p));
            } //UserDefinedTypeName::StructTypeName(ast)}
            UserDefinedTypeName::ContractTypeName(ref mut ast) => {
                ast.user_defined_type_name_base
                    .type_name_base
                    .ast_base
                    .parent = parent.map(|p| Box::new(p));
            } // UserDefinedTypeName::ContractTypeName(ast)}
            UserDefinedTypeName::AddressTypeName(ref mut ast) => {
                ast.user_defined_type_name_base
                    .type_name_base
                    .ast_base
                    .parent = parent.map(|p| Box::new(p));
            } // UserDefinedTypeName::AddressTypeName(ast)}
            UserDefinedTypeName::AddressPayableTypeName(ref mut ast) => {
                ast.user_defined_type_name_base
                    .type_name_base
                    .ast_base
                    .parent = parent.map(|p| Box::new(p));
            } //UserDefinedTypeName::AddressPayableTypeName(ast)}
            // _ => {} //UserDefinedTypeName::default()},
        };
    }
    pub fn target(&self) -> Option<NamespaceDefinition> {
        None
    }
    pub fn set_target(&mut self, type_def: NamespaceDefinition) {
        match self {
            UserDefinedTypeName::EnumTypeName(ref mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.into_ast()));
            } //UserDefinedTypeName::EnumTypeName(ast)}
            UserDefinedTypeName::EnumValueTypeName(ref mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.into_ast()));
            } //UserDefinedTypeName::EnumValueTypeName(ast)}
            UserDefinedTypeName::StructTypeName(ref mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.into_ast()));
            } //UserDefinedTypeName::StructTypeName(ast)}
            UserDefinedTypeName::ContractTypeName(ref mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.into_ast()));
            } // UserDefinedTypeName::ContractTypeName(ast)}
            UserDefinedTypeName::AddressTypeName(ref mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.into_ast()));
            } //UserDefinedTypeName::AddressTypeName(ast)}
            UserDefinedTypeName::AddressPayableTypeName(ref mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.into_ast()));
            } //UserDefinedTypeName::AddressPayableTypeName(ast)}
            // _ => {} //UserDefinedTypeName::default(),
        };
    }
    pub fn names(&self) -> Vec<Identifier> {
        vec![]
    }
}
impl IntoAST for UserDefinedTypeName {
    fn into_ast(self) -> AST {
        match self {
            UserDefinedTypeName::EnumTypeName(ast) => ast.into_ast(),
            UserDefinedTypeName::EnumValueTypeName(ast) => ast.into_ast(),
            UserDefinedTypeName::StructTypeName(ast) => ast.into_ast(),
            UserDefinedTypeName::ContractTypeName(ast) => ast.into_ast(),
            UserDefinedTypeName::AddressTypeName(ast) => ast.into_ast(),
            UserDefinedTypeName::AddressPayableTypeName(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for UserDefinedTypeName {
    fn get_ast_type(&self) -> ASTType {
        match self {
            UserDefinedTypeName::EnumTypeName(ast) => ast.get_ast_type(),
            UserDefinedTypeName::EnumValueTypeName(ast) => ast.get_ast_type(),
            UserDefinedTypeName::StructTypeName(ast) => ast.get_ast_type(),
            UserDefinedTypeName::ContractTypeName(ast) => ast.get_ast_type(),
            UserDefinedTypeName::AddressTypeName(ast) => ast.get_ast_type(),
            UserDefinedTypeName::AddressPayableTypeName(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for EnumTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::EnumTypeName
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for EnumValueTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::EnumValueTypeName
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
                .clone()
                .unwrap()
                .parent()
                .map(|p| p.namespace_definition().unwrap()),
        )))
    }
    pub fn implicitly_convertible_to(&self, expected: &TypeName) -> bool {
        // Implicitly convert smaller i32 types to larger i32 types
        self.user_defined_type_name_base
            .type_name_base
            .implicitly_convertible_to(expected)
            || (if let TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumTypeName(_)) =
                &expected
            {
                true
            } else {
                false
            } && expected.names()
                >= self.user_defined_type_name_base.names
                    [..self.user_defined_type_name_base.names.len() - 1]
                    .to_vec())
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for StructTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::StructTypeName
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for ContractTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ContractTypeName
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for AddressTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::AddressTypeName
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for AddressPayableTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::AddressPayableTypeName
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
        self.user_defined_type_name_base
            .type_name_base
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mapping {
    pub type_name_base: TypeNameBase,
    pub key_type: ElementaryTypeName,
    pub key_label: Option<Identifier>,
    pub value_type: Box<AnnotatedTypeName>,
    pub instantiated_key: Option<Expression>,
}
impl IntoAST for Mapping {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Mapping(self))
    }
}
impl ASTInstanceOf for Mapping {
    fn get_ast_type(&self) -> ASTType {
        ASTType::Mapping
    }
}
impl Mapping {
    pub fn new(
        key_type: ElementaryTypeName,
        key_label: Option<Identifier>,
        value_type: AnnotatedTypeName,
    ) -> Self {
        Self {
            type_name_base: TypeNameBase::new(),
            key_type,
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
        cb.add_child(AST::TypeName(TypeName::ElementaryTypeName(
            self.key_type.clone(),
        )));
        if let Some(idf) = &self.key_label {
            cb.add_child(AST::Identifier(idf.clone()));
        }
        cb.add_child(AST::AnnotatedTypeName(*self.value_type.clone()));
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ExprUnion {
    I32(i32),
    Expression(Expression),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum Array {
    CipherText(CipherText),
    Randomness(Randomness),
    Key(Key),
    Proof(Proof),
    Array(ArrayBase),
}
impl Array {
    pub fn value_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
}
impl IntoAST for Array {
    fn into_ast(self) -> AST {
        match self {
            Array::CipherText(ast) => ast.into_ast(),
            Array::Randomness(ast) => ast.into_ast(),
            Array::Key(ast) => ast.into_ast(),
            Array::Proof(ast) => ast.into_ast(),
            Array::Array(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for Array {
    fn get_ast_type(&self) -> ASTType {
        match self {
            Array::CipherText(ast) => ast.get_ast_type(),
            Array::Randomness(ast) => ast.get_ast_type(),
            Array::Key(ast) => ast.get_ast_type(),
            Array::Proof(ast) => ast.get_ast_type(),
            Array::Array(ast) => ast.get_ast_type(),
        }
    }
}
impl ASTChildren for ArrayBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::AnnotatedTypeName(self.value_type.clone()));
        if let Some(ExprUnion::Expression(expr)) = &self.expr {
            cb.add_child(AST::Expression(expr.clone()));
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for ArrayBase {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ArrayBase
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
        self.value_type.type_name.elem_bitwidth()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CipherText {
    pub array_base: Box<ArrayBase>,
    pub plain_type: AnnotatedTypeName,
    pub crypto_params: CryptoParams,
}
impl IntoAST for CipherText {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::CipherText(self)))
    }
}
impl ASTInstanceOf for CipherText {
    fn get_ast_type(&self) -> ASTType {
        ASTType::CipherText
    }
}
impl CipherText {
    pub fn new(plain_type: AnnotatedTypeName, crypto_params: CryptoParams) -> Self {
        assert!(!plain_type.type_name.is_cipher());
        Self {
            array_base: Box::new(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(Expression::LiteralExpr(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                        crypto_params.cipher_len(),
                        false,
                    )),
                ))),
            )),
            plain_type,
            crypto_params,
        }
    }
    pub fn size_in_uints(&self) -> i32 {
        self.crypto_params.cipher_payload_len()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Randomness {
    pub array_base: Box<ArrayBase>,
    pub crypto_params: CryptoParams,
}
impl IntoAST for Randomness {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Randomness(self)))
    }
}
impl ASTInstanceOf for Randomness {
    fn get_ast_type(&self) -> ASTType {
        ASTType::Randomness
    }
}
impl Randomness {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: Box::new(ArrayBase::new(
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
            )),
            crypto_params,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Key {
    pub array_base: Box<ArrayBase>,
    pub crypto_params: CryptoParams,
}
impl IntoAST for Key {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Key(self)))
    }
}
impl ASTInstanceOf for Key {
    fn get_ast_type(&self) -> ASTType {
        ASTType::Key
    }
}
impl Key {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            array_base: Box::new(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(Expression::LiteralExpr(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                        crypto_params.key_len(),
                        false,
                    )),
                ))),
            )),
            crypto_params,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Proof {
    pub array_base: Box<ArrayBase>,
}
impl IntoAST for Proof {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::Array(Array::Proof(self)))
    }
}
impl ASTInstanceOf for Proof {
    fn get_ast_type(&self) -> ASTType {
        ASTType::Proof
    }
}
impl Proof {
    pub fn new() -> Self {
        Self {
            array_base: Box::new(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                Some(ExprUnion::Expression(Expression::LiteralExpr(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                        CFG.lock().unwrap().proof_len(),
                        false,
                    )),
                ))),
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DummyAnnotation;

impl IntoAST for DummyAnnotation {
    fn into_ast(self) -> AST {
        AST::Expression(Expression::DummyAnnotation(self))
    }
}
impl ASTInstanceOf for DummyAnnotation {
    fn get_ast_type(&self) -> ASTType {
        ASTType::DummyAnnotation
    }
}
impl DummyAnnotation {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum CombinedPrivacyUnion {
    Vec(Vec<CombinedPrivacyUnion>),
    AST(Option<AST>),
}
impl CombinedPrivacyUnion {
    pub fn expr(&self) -> Option<Expression> {
        if let CombinedPrivacyUnion::AST(expr) = &self {
            return expr.clone().map(|expr| expr.expr().unwrap());
        }
        None
    }
}
//     """Does not appear in the syntax, but is necessary for type checking"""
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleType {
    pub type_name_base: TypeNameBase,
    pub types: Vec<AnnotatedTypeName>,
}
impl IntoAST for TupleType {
    fn into_ast(self) -> AST {
        AST::TypeName(TypeName::TupleType(self))
    }
}
impl ASTInstanceOf for TupleType {
    fn get_ast_type(&self) -> ASTType {
        ASTType::TupleType
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
            if let TypeName::TupleType(t) = *t.type_name {
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
                x.type_name.implicitly_convertible_to(&y.type_name)
            })
        } else {
            false
        }
    }

    pub fn compatible_with(&self, other_type: TypeName) -> bool {
        if let TypeName::TupleType(other_type) = other_type {
            self.check_component_wise(&other_type, |x, y| {
                x.type_name.compatible_with(&y.type_name)
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
                                .combined_type(*e2.type_name.clone(), convert_literals)
                                .unwrap()
                                .tuple_type()
                                .unwrap()
                                .into_ast()
                                .type_name()
                                .unwrap(),
                            Some(Expression::DummyAnnotation(DummyAnnotation::new())),
                            String::from("NON_HOMOMORPHIC"),
                        )
                    })
                    .collect(),
            ))
        }
    }
    pub fn annotate(&self, privacy_annotation: CombinedPrivacyUnion) -> CombinedPrivacyUnion {
        CombinedPrivacyUnion::AST(if let CombinedPrivacyUnion::AST(_) = &privacy_annotation {
            Some(AST::AnnotatedTypeName(AnnotatedTypeName::new(
                TypeName::TupleType(TupleType::new(
                    self.types
                        .iter()
                        .map(|t| t.type_name.annotate(privacy_annotation.clone()))
                        .collect(),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
            )))
        } else if let CombinedPrivacyUnion::Vec(privacy_annotation) = &privacy_annotation {
            assert!(self.types.len() == privacy_annotation.len());
            Some(AST::AnnotatedTypeName(AnnotatedTypeName::new(
                TypeName::TupleType(TupleType::new(
                    self.types
                        .iter()
                        .zip(privacy_annotation)
                        .map(|(t, a)| t.type_name.annotate(a.clone()))
                        .collect(),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for FunctionTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::FunctionTypeName
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
            cb.add_child(AST::IdentifierDeclaration(
                IdentifierDeclaration::Parameter(parameter.clone()),
            ));
        });
        self.return_parameters.iter().for_each(|parameter| {
            cb.add_child(AST::IdentifierDeclaration(
                IdentifierDeclaration::Parameter(parameter.clone()),
            ));
        });
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnnotatedTypeName {
    pub ast_base: ASTBase,
    pub type_name: Box<TypeName>,
    pub had_privacy_annotation: bool,
    pub privacy_annotation: Option<Box<Expression>>,
    pub homomorphism: String,
}
impl IntoAST for AnnotatedTypeName {
    fn into_ast(self) -> AST {
        AST::AnnotatedTypeName(self)
    }
}
impl ASTInstanceOf for AnnotatedTypeName {
    fn get_ast_type(&self) -> ASTType {
        ASTType::AnnotatedTypeName
    }
}
impl AnnotatedTypeName {
    pub fn new(
        type_name: TypeName,
        privacy_annotation: Option<Expression>,
        homomorphism: String,
    ) -> Self {
        assert!(
            !(privacy_annotation.is_none()
                || if let Some(Expression::AllExpr(_)) = &privacy_annotation {
                    true
                } else {
                    false
                })
                || homomorphism == String::from("NON_HOMOMORPHIC"),
            "Public type name cannot be homomorphic (got {:?})",
            HOMOMORPHISM_STORE.lock().unwrap().get(&homomorphism)
        );
        Self {
            ast_base: ASTBase::new(),
            type_name: Box::new(type_name),
            had_privacy_annotation: privacy_annotation.as_ref().is_some(),
            privacy_annotation: Some(Box::new(
                if let Some(privacy_annotation) = privacy_annotation {
                    privacy_annotation
                } else {
                    Expression::AllExpr(AllExpr::new())
                },
            )),
            homomorphism,
        }
    }
    pub fn zkay_type(&self) -> Self {
        if let TypeName::Array(Array::CipherText(ct)) = *self.type_name.clone() {
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
        if let (TypeName::TupleType(selfs), TypeName::TupleType(others)) =
            (*self.type_name.clone(), *other.type_name.clone())
        {
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
        let p_expected = other_privacy_annotation.privacy_annotation_label();
        let p_actual = self_privacy_annotation.privacy_annotation_label();
        if let (Some(p_expected), Some(p_actual)) = (p_expected, p_actual) {
            if p_expected == p_actual
                || (analysis.is_some()
                    && analysis
                        .unwrap()
                        .same_partition(&p_expected.into(), &p_actual.into()))
            {
                Some(CombinedPrivacyUnion::AST(Some(AST::Expression(
                    *self_privacy_annotation,
                ))))
            } else if self_privacy_annotation.is_all_expr() {
                Some(CombinedPrivacyUnion::AST(Some(AST::Expression(
                    *other_privacy_annotation,
                ))))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_public(&self) -> bool {
        if let Some(pa) = &self.privacy_annotation {
            pa.is_all_expr()
        } else {
            false
        }
    }

    pub fn is_private(&self) -> bool {
        !self.is_public()
    }
    pub fn is_private_at_me(&self, analysis: &Option<PartitionState<AST>>) -> bool {
        if let Some(p) = &self.privacy_annotation {
            p.is_me_expr()
                || (analysis.is_some()
                    && analysis.clone().unwrap().same_partition(
                        &p.privacy_annotation_label().unwrap().into(),
                        &AST::Expression(Expression::MeExpr(MeExpr::new())).into(),
                    ))
        } else {
            false
        }
    }
    pub fn is_accessible(&self, analysis: &Option<PartitionState<AST>>) -> bool {
        self.is_public() || self.is_private_at_me(analysis)
    }

    pub fn is_address(&self) -> bool {
        if let TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressTypeName(_))
        | TypeName::UserDefinedTypeName(UserDefinedTypeName::AddressPayableTypeName(_)) =
            *self.type_name
        {
            true
        } else {
            false
        }
    }
    pub fn is_cipher(&self) -> bool {
        if let TypeName::Array(Array::CipherText(_)) = *self.type_name {
            true
        } else {
            false
        }
    }
    pub fn with_homomorphism(&self, hom: String) -> Self {
        AnnotatedTypeName::new(
            *self.type_name.clone(),
            self.privacy_annotation.clone().map(|p| *p),
            hom,
        )
    }
    pub fn uint_all() -> Self {
        AnnotatedTypeName::new(TypeName::uint_type(), None, String::from("NON_HOMOMORPHIC"))
    }

    pub fn bool_all() -> Self {
        AnnotatedTypeName::new(TypeName::bool_type(), None, String::from("NON_HOMOMORPHIC"))
    }

    pub fn address_all() -> Self {
        AnnotatedTypeName::new(
            TypeName::address_type(),
            None,
            String::from("NON_HOMOMORPHIC"),
        )
    }

    pub fn cipher_type(plain_type: AnnotatedTypeName, hom: Option<String>) -> Self {
        AnnotatedTypeName::new(
            TypeName::cipher_type(plain_type, hom.unwrap()),
            None,
            String::from("NON_HOMOMORPHIC"),
        )
    }

    pub fn key_type(crypto_params: CryptoParams) -> Self {
        AnnotatedTypeName::new(
            TypeName::key_type(crypto_params),
            None,
            String::from("NON_HOMOMORPHIC"),
        )
    }

    pub fn proof_type() -> Self {
        AnnotatedTypeName::new(
            TypeName::proof_type(),
            None,
            String::from("NON_HOMOMORPHIC"),
        )
    }
    pub fn all(type_name: TypeName) -> Self {
        AnnotatedTypeName::new(
            type_name,
            Some(Expression::all_expr()),
            String::from("NON_HOMOMORPHIC"),
        )
    }
    pub fn me(type_name: TypeName) -> Self {
        AnnotatedTypeName::new(
            type_name,
            Some(Expression::me_expr(None)),
            String::from("NON_HOMOMORPHIC"),
        )
    }
    pub fn array_all(value_type: AnnotatedTypeName, length: Vec<i32>) -> Self {
        let mut t = value_type;
        for &l in &length {
            t = AnnotatedTypeName::new(
                TypeName::Array(Array::Array(ArrayBase::new(t, Some(ExprUnion::I32(l))))),
                None,
                String::from("NON_HOMOMORPHIC"),
            );
        }
        t
    }
}
impl ASTChildren for AnnotatedTypeName {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::TypeName(*self.type_name.clone()));
        if let Some(privacy_annotation) = &self.privacy_annotation {
            cb.add_child(AST::Expression(*privacy_annotation.clone()));
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum IdentifierDeclaration {
    VariableDeclaration(VariableDeclaration),
    Parameter(Parameter),
    StateVariableDeclaration(StateVariableDeclaration),
}
impl IdentifierDeclaration {
    pub fn identifier_declaration_base(&self) -> Option<IdentifierDeclarationBase> {
        None
    }
    pub fn annotated_type(&self) -> Option<AnnotatedTypeName> {
        None
    }
}
impl IntoAST for IdentifierDeclaration {
    fn into_ast(self) -> AST {
        match self {
            IdentifierDeclaration::VariableDeclaration(ast) => ast.into_ast(),
            IdentifierDeclaration::Parameter(ast) => ast.into_ast(),
            IdentifierDeclaration::StateVariableDeclaration(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for IdentifierDeclaration {
    fn get_ast_type(&self) -> ASTType {
        match self {
            IdentifierDeclaration::VariableDeclaration(ast) => ast.get_ast_type(),
            IdentifierDeclaration::Parameter(ast) => ast.get_ast_type(),
            IdentifierDeclaration::StateVariableDeclaration(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
        cb.add_child(AST::AnnotatedTypeName(*self.annotated_type.clone()));
        cb.add_child(AST::Identifier(*self.idf.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclaration {
    pub identifier_declaration_base: Box<IdentifierDeclarationBase>,
}
impl IntoAST for VariableDeclaration {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(self))
    }
}
impl ASTInstanceOf for VariableDeclaration {
    fn get_ast_type(&self) -> ASTType {
        ASTType::VariableDeclaration
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
            identifier_declaration_base: Box::new(IdentifierDeclarationBase::new(
                keywords,
                annotated_type,
                idf,
                storage_location,
            )),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclarationStatement {
    pub simple_statement_base: SimpleStatementBase,
    pub variable_declaration: VariableDeclaration,
    pub expr: Option<Expression>,
}
// impl IntoStatement for VariableDeclarationStatement {
//     fn into_statement(self) -> Statement {
//         Statement::SimpleStatement(SimpleStatement::VariableDeclarationStatement(self))
//     }
// }
impl IntoAST for VariableDeclarationStatement {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::SimpleStatement(
            SimpleStatement::VariableDeclarationStatement(self),
        ))
    }
}
impl ASTInstanceOf for VariableDeclarationStatement {
    fn get_ast_type(&self) -> ASTType {
        ASTType::VariableDeclarationStatement
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
        cb.add_child(AST::IdentifierDeclaration(
            IdentifierDeclaration::VariableDeclaration(self.variable_declaration.clone()),
        ));
        if let Some(expr) = &self.expr {
            cb.add_child(AST::Expression(expr.clone()));
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Parameter {
    pub identifier_declaration_base: Box<IdentifierDeclarationBase>,
}
impl IntoAST for Parameter {
    fn into_ast(self) -> AST {
        AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(self))
    }
}
impl ASTInstanceOf for Parameter {
    fn get_ast_type(&self) -> ASTType {
        ASTType::Parameter
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
            identifier_declaration_base: Box::new(IdentifierDeclarationBase::new(
                keywords,
                annotated_type,
                idf,
                storage_location,
            )),
        }
    }
    pub fn with_changed_storage(&mut self, match_storage: String, new_storage: String) -> Self {
        if self.identifier_declaration_base.storage_location == Some(match_storage) {
            self.identifier_declaration_base.storage_location = Some(new_storage);
        }
        self.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum NamespaceDefinition {
    ConstructorOrFunctionDefinition(ConstructorOrFunctionDefinition),
    EnumDefinition(EnumDefinition),
    StructDefinition(StructDefinition),
    ContractDefinition(ContractDefinition),
}
impl NamespaceDefinition {
    pub fn ast_base_mut(&mut self) -> Option<&mut ASTBase> {
        None
    }
    pub fn namespace_definition_base(&self) -> Option<&NamespaceDefinitionBase> {
        None
    }
    pub fn names(&self) -> BTreeMap<String, Identifier> {
        BTreeMap::new()
    }
    pub fn parent(&self) -> Option<AST> {
        None
    }
}
impl IntoAST for NamespaceDefinition {
    fn into_ast(self) -> AST {
        match self {
            NamespaceDefinition::ConstructorOrFunctionDefinition(ast) => ast.into_ast(),
            NamespaceDefinition::EnumDefinition(ast) => ast.into_ast(),
            NamespaceDefinition::StructDefinition(ast) => ast.into_ast(),
            NamespaceDefinition::ContractDefinition(ast) => ast.into_ast(),
        }
    }
}
impl ASTInstanceOf for NamespaceDefinition {
    fn get_ast_type(&self) -> ASTType {
        match self {
            NamespaceDefinition::ConstructorOrFunctionDefinition(ast) => ast.get_ast_type(),
            NamespaceDefinition::EnumDefinition(ast) => ast.get_ast_type(),
            NamespaceDefinition::StructDefinition(ast) => ast.get_ast_type(),
            NamespaceDefinition::ContractDefinition(ast) => ast.get_ast_type(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
        cb.add_child(AST::Identifier(self.idf.clone()));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for ConstructorOrFunctionDefinition {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ConstructorOrFunctionDefinition
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
            idf.is_some() && idf.as_ref().unwrap().name() != String::from("constructor")
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
                idf.ast_base.parent = Some(Box::new(AST::IdentifierDeclaration(
                    IdentifierDeclaration::VariableDeclaration(vd.clone()),
                )));
                vd.identifier_declaration_base.idf = Box::new(Identifier::Identifier(idf));
            }
        });
        Self {
            namespace_definition_base: NamespaceDefinitionBase::new(idf),
            parameters: parameters.as_ref().unwrap().clone(),
            modifiers: modifiers.as_ref().unwrap().clone(),
            return_parameters: return_parameters.clone(),
            body,
            return_var_decls,
            parent: None,
            original_body: None,
            annotated_type: Some(AnnotatedTypeName::new(
                TypeName::FunctionTypeName(FunctionTypeName::new(
                    parameters.unwrap(),
                    modifiers.unwrap(),
                    return_parameters,
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
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
        &self.namespace_definition_base.idf.name() == "constructor"
    }

    pub fn is_function(&self) -> bool {
        !self.is_constructor()
    }

    pub fn _update_fct_type(&mut self) {
        self.annotated_type = Some(AnnotatedTypeName::new(
            TypeName::FunctionTypeName(FunctionTypeName::new(
                self.parameters.clone(),
                self.modifiers.clone(),
                self.return_parameters.clone(),
            )),
            None,
            String::from("NON_HOMOMORPHIC"),
        ));
        // AnnotatedTypeName(FunctionTypeName(&self.parameters, self.modifiers, self.return_parameters));
    }
    pub fn add_param(&mut self, t: AST, idf: IdentifierExprUnion, ref_storage_loc: Option<String>) {
        let ref_storage_loc = ref_storage_loc.unwrap_or(String::from("memory"));
        let t = if let AST::AnnotatedTypeName(t) = t {
            Some(t)
        } else if let AST::TypeName(t) = t {
            Some(AnnotatedTypeName::new(
                t,
                None,
                String::from("NON_HOMOMORPHIC"),
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
        let storage_loc = if t.as_ref().unwrap().type_name.is_primitive_type() {
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
        // super().process_children(f)
        self.parameters.iter().for_each(|parameter| {
            cb.add_child(AST::IdentifierDeclaration(
                IdentifierDeclaration::Parameter(parameter.clone()),
            ));
        });
        self.return_parameters.iter().for_each(|parameter| {
            cb.add_child(AST::IdentifierDeclaration(
                IdentifierDeclaration::Parameter(parameter.clone()),
            ));
        });
        if let Some(body) = &self.body {
            cb.add_child(AST::Statement(Statement::StatementList(
                StatementList::Block(body.clone()),
            )));
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for StateVariableDeclaration {
    fn get_ast_type(&self) -> ASTType {
        ASTType::StateVariableDeclaration
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
    pub fn is_final(&self) -> bool {
        self.identifier_declaration_base
            .keywords
            .contains(&String::from("final"))
    }
    pub fn is_constant(&self) -> bool {
        self.identifier_declaration_base
            .keywords
            .contains(&String::from("constant"))
    }
}
impl ASTChildren for StateVariableDeclaration {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.identifier_declaration_base.process_children(cb);
        if let Some(expr) = &self.expr {
            cb.add_child(AST::Expression(expr.clone()));
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for EnumValue {
    fn get_ast_type(&self) -> ASTType {
        ASTType::EnumValue
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
            cb.add_child(AST::Identifier(idf.clone()));
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for EnumDefinition {
    fn get_ast_type(&self) -> ASTType {
        ASTType::EnumDefinition
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
            cb.add_child(AST::EnumValue(value.clone()));
        });
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StructDefinition {
    pub namespace_definition_base: NamespaceDefinitionBase,
    pub members: Vec<AST>,
}
impl IntoAST for StructDefinition {
    fn into_ast(self) -> AST {
        AST::NamespaceDefinition(NamespaceDefinition::StructDefinition(self))
    }
}
impl ASTInstanceOf for StructDefinition {
    fn get_ast_type(&self) -> ASTType {
        ASTType::StructDefinition
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for ContractDefinition {
    fn get_ast_type(&self) -> ASTType {
        ASTType::ContractDefinition
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
}
impl ASTChildren for ContractDefinition {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.namespace_definition_base.process_children(cb);
        self.enum_definitions.iter().for_each(|enum_definition| {
            cb.add_child(AST::NamespaceDefinition(
                NamespaceDefinition::EnumDefinition(enum_definition.clone()),
            ));
        });
        self.struct_definitions
            .iter()
            .for_each(|struct_definition| {
                cb.add_child(AST::NamespaceDefinition(
                    NamespaceDefinition::StructDefinition(struct_definition.clone()),
                ));
            });
        self.state_variable_declarations
            .iter()
            .for_each(|state_variable_declarations| {
                cb.add_child(state_variable_declarations.clone());
            });
        self.constructor_definitions
            .iter()
            .for_each(|constructor_definition| {
                cb.add_child(AST::NamespaceDefinition(
                    NamespaceDefinition::ConstructorOrFunctionDefinition(
                        constructor_definition.clone(),
                    ),
                ));
            });
        self.function_definitions
            .iter()
            .for_each(|function_definition| {
                cb.add_child(AST::NamespaceDefinition(
                    NamespaceDefinition::ConstructorOrFunctionDefinition(
                        function_definition.clone(),
                    ),
                ));
            });
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
impl ASTInstanceOf for SourceUnit {
    fn get_ast_type(&self) -> ASTType {
        ASTType::SourceUnit
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
    pub fn get_item(self, key: &String) -> Option<ContractDefinition> {
        if let Some(c_identifier) = self.ast_base.names.get(key) {
            let c = c_identifier.parent();
            if let Some(c) = c {
                if let AST::NamespaceDefinition(NamespaceDefinition::ContractDefinition(c)) = c {
                    return Some(c.clone());
                }
            }
        }
        None
    }
}
impl ASTChildren for SourceUnit {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.contracts.iter().for_each(|contract| {
            cb.add_child(AST::NamespaceDefinition(
                NamespaceDefinition::ContractDefinition(contract.clone()),
            ));
        });
    }
}
impl ConstructorOrFunctionDefinitionAttr for AST {
    fn get_requires_verification_when_external(&self) -> bool {
        if let Some(c) = self.constructor_or_function_definition() {
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
    if let Some(idf) = plabel.identifier() {
        let mut ie = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(idf.clone()),
            Some(Box::new(AnnotatedTypeName::address_all())),
        );
        ie.location_expr_base.target = idf.parent().map(|p| Box::new((p).into()));
        ie.to_expr()
    } else {
        plabel.expr().unwrap()
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
                let v = v.location_expr().unwrap();
                match v.get_ast_type() {
                    ASTType::IdentifierExpr => {
                        vec![v.location_expr_base().unwrap().target.clone(), None]
                    }

                    ASTType::MemberAccessExpr => vec![
                        v.location_expr_base().unwrap().target.clone(),
                        Some(Box::new(AST::Identifier(
                            *v.member_access_expr().unwrap().member,
                        ))),
                    ],
                    ASTType::IndexExpr => vec![
                        v.location_expr_base().unwrap().target.clone(),
                        Some(Box::new(AST::Expression(*v.index_expr().unwrap().key))),
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
        if let TypeName::Mapping(_) = *self.target().unwrap().annotated_type().unwrap().type_name {
            let t = self
                .target()
                .unwrap()
                .annotated_type()
                .unwrap()
                .zkay_type()
                .type_name;
            if t.has_key_label() {
                self.key()
                    .unwrap()
                    .privacy_annotation_label()
                    .map(|x| x.into_ast())
            } else {
                t.value_type()
                    .unwrap()
                    .privacy_annotation
                    .unwrap()
                    .privacy_annotation_label()
                    .map(|x| x.into_ast())
            }
        } else if self.key().is_none() {
            self.target()
                .unwrap()
                .annotated_type()
                .unwrap()
                .zkay_type()
                .privacy_annotation
                .unwrap()
                .privacy_annotation_label()
                .map(|x| x.into_ast())
        } else {
            None
        }
    }

    pub fn in_scope_at(&self, ast: AST) -> bool {
        crate::pointers::symbol_table::SymbolTableLinker::in_scope_at(
            &self.target().unwrap().idf().unwrap(),
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
        stmt.line()
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
    } else if let AST::Statement(ast) = &ast {
        Some(Box::new(ast.clone()))
    } else {
        None
    };

    // Get surrounding function
    let fct = if let Some(stmt) = &stmt {
        stmt.function().map(|f| f.into_ast())
    } else if is_instance(&ast, ASTType::ConstructorOrFunctionDefinition) {
        Some(ast.clone())
    } else {
        None
    };

    // Get surrounding contract
    let mut ctr = fct.clone().or(Some(ast.clone()));
    while ctr.is_some() && !is_instance(ctr.as_ref().unwrap(), ASTType::ContractDefinition) {
        if let Some(p) = ctr.as_ref().unwrap().parent() {
            ctr = Some(p);
        } else {
            break;
        }
    }

    // Get source root
    let mut root = ctr.clone().or(Some(ast.clone()));
    while root.is_some() && !is_instance(root.as_ref().unwrap(), ASTType::SourceUnit) {
        root = root.unwrap().parent();
    }

    let error_msg = if root.is_none() {
        String::from("error")
    } else {
        get_code_error_msg(
            ast.line(),
            ast.column(),
            root.unwrap().original_code(),
            ctr.unwrap().contract_definition(),
            fct.clone()
                .map(|f| f.constructor_or_function_definition().unwrap()),
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
pub struct CodeVisitor {
    pub display_final: bool,
    pub traversal: &'static str,
    pub log: bool,
}
fn a(ast: AST) -> Option<String> {
    Some(String::new())
}
impl AstVisitor for CodeVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
        None
    }
}
type CodeVisitorReturn = String;
impl CodeVisitor {
    pub fn new(display_final: bool) -> Self {
        Self {
            traversal: "node-or-children",
            display_final,
            log: false,
        }
    }
    pub fn visit(&self, ast: &AST) -> CodeVisitorReturn {
        String::new()
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
                Some(selfs.visit(e))
            } else {
                None
            }
        }

        let s: Vec<_> = l.iter().filter_map(|e| handle(self, e)).collect();
        let s = s.concat();
        s
    }

    pub fn visit_single_or_list(&self, v: SingleOrListUnion, mut sep: &str) -> CodeVisitorReturn {
        if sep.is_empty() {
            sep = "\n";
        }
        match v {
            SingleOrListUnion::Vec(v) => self.visit_list(v, sep),
            SingleOrListUnion::String(v) => v,
            SingleOrListUnion::AST(v) => self.visit(&v),
            // _ => String::new(),
        }
    }

    pub fn visit_AST(&self, ast: AST) -> CodeVisitorReturn {
        // should never be called
        // raise NotImplementedError("Did not implement code generation for " + repr(ast))
        unimplemented!("Did not implement code generation for {:?} ", ast);
        String::new()
    }
    pub fn visit_Comment(&self, ast: Comment) -> CodeVisitorReturn {
        if ast.text() == String::new() {
            String::new()
        } else if ast.text().contains(" ") {
            format!("/* {} */", ast.text())
        } else {
            format!("// {}", ast.text())
        }
    }

    pub fn visit_Identifier(&self, ast: Identifier) -> CodeVisitorReturn {
        ast.name().clone()
    }

    pub fn visit_FunctionCallExpr(&self, ast: FunctionCallExpr) -> CodeVisitorReturn {
        if let Some(Expression::BuiltinFunction(func)) = ast.func() {
            let args: Vec<_> = ast
                .args()
                .iter()
                .map(|a| self.visit(&AST::Expression(a.clone())))
                .collect();
            func.format_string(&args)
        } else {
            let f = self.visit(&AST::Expression(ast.func().unwrap()));
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

    pub fn visit_PrimitiveCastExpr(&self, ast: PrimitiveCastExpr) -> CodeVisitorReturn {
        if ast.is_implicit {
            self.visit(&AST::Expression(*ast.expr))
        } else {
            format!(
                "{}({})",
                self.visit(&AST::TypeName(*ast.elem_type)),
                self.visit(&AST::Expression(*ast.expr))
            )
        }
    }

    pub fn visit_BooleanLiteralExpr(&self, ast: BooleanLiteralExpr) -> CodeVisitorReturn {
        ast.value.to_string().to_ascii_lowercase()
    }

    pub fn visit_NumberLiteralExpr(&self, ast: NumberLiteralExpr) -> CodeVisitorReturn {
        if ast.was_hex {
            format!("{:x}", ast.value())
        } else {
            ast.value.to_string()
        }
    }

    pub fn visit_StringLiteralExpr(&self, ast: StringLiteralExpr) -> CodeVisitorReturn {
        format!("\"{}\"", ast.value)
    }

    pub fn visit_ArrayLiteralExpr(&self, ast: ArrayLiteralExpr) -> CodeVisitorReturn {
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

    pub fn visit_TupleExpr(&self, ast: TupleExpr) -> CodeVisitorReturn {
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

    pub fn visit_IdentifierExpr(&self, ast: IdentifierExpr) -> CodeVisitorReturn {
        self.visit(&AST::Identifier(*ast.idf))
    }

    pub fn visit_MemberAccessExpr(&self, ast: MemberAccessExpr) -> CodeVisitorReturn {
        format!(
            "{}.{}",
            self.visit(&AST::Expression(Expression::TupleOrLocationExpr(
                TupleOrLocationExpr::LocationExpr(*ast.expr.unwrap())
            ))),
            self.visit(&AST::Identifier(*ast.member))
        )
    }

    pub fn visit_IndexExpr(&self, ast: IndexExpr) -> CodeVisitorReturn {
        format!(
            "{}[{}]",
            self.visit(&AST::Expression(Expression::TupleOrLocationExpr(
                TupleOrLocationExpr::LocationExpr(*ast.arr.clone().unwrap())
            ))),
            self.visit(&AST::Expression(*ast.key))
        )
    }

    pub fn visit_MeExpr(&self, _: MeExpr) -> CodeVisitorReturn {
        String::from("me")
    }

    pub fn visit_AllExpr(&self, _: AllExpr) -> CodeVisitorReturn {
        String::from("all")
    }

    pub fn visit_ReclassifyExpr(&self, ast: ReclassifyExpr) -> CodeVisitorReturn {
        let e = self.visit(&AST::Expression(ast.expr().unwrap()));
        let p = self.visit(&AST::Expression(ast.privacy().unwrap()));
        let h = HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .get(&ast.homomorphism().unwrap())
            .unwrap()
            .clone();
        format!("reveal{h:?}({e}, {p})")
    }

    pub fn visit_RehomExpr(&self, ast: RehomExpr) -> CodeVisitorReturn {
        let e = self.visit(&AST::Expression(*ast.reclassify_expr_base.expr.clone()));
        format!("{}({e})", ast.func_name())
    }

    pub fn visit_IfStatement(&self, ast: IfStatement) -> CodeVisitorReturn {
        let c = self.visit(&AST::Expression(ast.condition));
        let t = self.visit_single_or_list(
            SingleOrListUnion::AST(AST::Statement(Statement::StatementList(
                StatementList::Block(ast.then_branch),
            ))),
            "",
        );
        let mut ret = format!("if ({c}) {t}");
        if let Some(else_branch) = ast.else_branch {
            let e = self.visit_single_or_list(
                SingleOrListUnion::AST(AST::Statement(Statement::StatementList(
                    StatementList::Block(else_branch),
                ))),
                "",
            );
            ret += format!("\n else {e}").as_str();
        }
        ret
    }

    pub fn visit_WhileStatement(&self, ast: WhileStatement) -> CodeVisitorReturn {
        let c = self.visit(&AST::Expression(ast.condition));
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(AST::Statement(Statement::StatementList(
                StatementList::Block(ast.body),
            ))),
            "",
        );
        format!("while ({c}) {b}")
    }

    pub fn visit_DoWhileStatement(&self, ast: DoWhileStatement) -> CodeVisitorReturn {
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(AST::Statement(Statement::StatementList(
                StatementList::Block(ast.body),
            ))),
            "",
        );
        let c = self.visit(&AST::Expression(ast.condition));
        format!("do {b} while ({c});")
    }

    pub fn visit_ForStatement(&self, ast: ForStatement) -> CodeVisitorReturn {
        let i = if let Some(init) = ast.init {
            format!(
                "{}",
                self.visit_single_or_list(
                    SingleOrListUnion::AST(AST::Statement(Statement::SimpleStatement(*init))),
                    ""
                )
            )
        } else {
            String::from(";")
        };
        let c = self.visit(&AST::Expression(ast.condition));
        let u = if let Some(update) = ast.update {
            format!(
                " {}",
                self.visit_single_or_list(
                    SingleOrListUnion::AST(AST::Statement(Statement::SimpleStatement(*update))),
                    ""
                )
                .replace(";", "")
            )
        } else {
            String::new()
        };
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(AST::Statement(Statement::StatementList(
                StatementList::Block(ast.body),
            ))),
            "",
        );
        format!("for ({i} {c};{u}) {b}")
    }

    pub fn visit_BreakStatement(&self, _: BreakStatement) -> CodeVisitorReturn {
        String::from("break;")
    }

    pub fn visit_ContinueStatement(&self, _: ContinueStatement) -> CodeVisitorReturn {
        String::from("continue;")
    }

    pub fn visit_ReturnStatement(&self, ast: ReturnStatement) -> CodeVisitorReturn {
        if ast.expr.is_none() {
            String::from("return;")
        } else {
            let e = self.visit(&AST::Expression(ast.expr.unwrap()));
            format!("return {e};")
        }
    }

    pub fn visit_ExpressionStatement(&self, ast: ExpressionStatement) -> CodeVisitorReturn {
        self.visit(&AST::Expression(ast.expr)) + ";"
    }

    pub fn visit_RequireStatement(&self, ast: RequireStatement) -> CodeVisitorReturn {
        let c = self.visit(&AST::Expression(ast.condition));
        format!("require({c});")
    }

    pub fn visit_AssignmentStatement(&self, ast: AssignmentStatement) -> CodeVisitorReturn {
        let lhs = ast.lhs().unwrap();
        let mut op = ast.op().unwrap();
        if let Some(asu) = lhs.tuple_expr() {
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
        if let Some(le) = lhs.location_expr() {
            let annotated_type = match le {
                LocationExpr::IdentifierExpr(ie) => {
                    if let Some(at) = ie.annotated_type.clone() {
                        Some(*at)
                    } else {
                        None
                    }
                }
                LocationExpr::MemberAccessExpr(ie) => ie
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .annotated_type
                    .clone(),
                LocationExpr::IndexExpr(ie) => ie
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .annotated_type
                    .clone(),
                LocationExpr::SliceExpr(ie) => ie
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .annotated_type
                    .clone(),
            };
            if let Some(at) = annotated_type {
                if at.is_private() {
                    op = String::new();
                }
            }
        }

        let rhs = if !op.is_empty() {
            ast.rhs().map(|fce| fce.args()[1].clone().expr().unwrap())
        } else {
            ast.rhs()
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
        ) = (lhs.location_expr(), &rhs)
        {
            assert!(lhs.size == rhs.size, "Slice ranges don't have same size");
            let mut s = String::new();
            let (lexpr, rexpr) = (
                self.visit(&AST::Expression(Expression::TupleOrLocationExpr(
                    TupleOrLocationExpr::LocationExpr(*lhs.arr.clone().unwrap()),
                ))),
                self.visit(&AST::Expression(Expression::TupleOrLocationExpr(
                    TupleOrLocationExpr::LocationExpr(*rhs.arr.clone().unwrap()),
                ))),
            );
            let mut lbase = if let Some(base) = &lhs.base {
                format!("{} + ", self.visit(&AST::Expression(*base.clone())))
            } else {
                String::new()
            };
            let mut rbase = if let Some(base) = &rhs.base {
                format!("{} + ", self.visit(&AST::Expression(*base.clone())))
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
            format_string(to_ast(&lhs), self.visit(&AST::Expression(rhs.unwrap())))
        }
    }
    pub fn visit_CircuitDirectiveStatement(
        &self,
        ast: CircuitDirectiveStatement,
    ) -> CodeVisitorReturn {
        String::new()
    }

    fn handle_block(&self, ast: StatementList) -> CodeVisitorReturn {
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

    pub fn visit_StatementList(&self, ast: StatementList) -> CodeVisitorReturn {
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

    pub fn visit_Block(&self, ast: Block) -> CodeVisitorReturn {
        let b = self
            .handle_block(StatementList::Block(ast.clone()))
            .trim_end()
            .to_string();
        if ast.was_single_statement && ast.statement_list_base.statements.len() == 1 {
            b
        } else {
            format!("{{ {b} }}")
        }
    }

    pub fn visit_IndentBlock(&self, ast: IndentBlock) -> CodeVisitorReturn {
        self.handle_block(StatementList::IndentBlock(ast))
    }

    pub fn visit_ElementaryTypeName(&self, ast: ElementaryTypeName) -> CodeVisitorReturn {
        match ast {
            ElementaryTypeName::NumberTypeName(ntn) => match ntn {
                NumberTypeName::NumberLiteralType(nlt) => nlt
                    .number_type_name_base
                    .elementary_type_name_base
                    .name
                    .clone(),
                NumberTypeName::IntTypeName(itn) => itn
                    .number_type_name_base
                    .elementary_type_name_base
                    .name
                    .clone(),
                NumberTypeName::UintTypeName(utn) => utn
                    .number_type_name_base
                    .elementary_type_name_base
                    .name
                    .clone(),
                NumberTypeName::NumberTypeNameBase(antn) => {
                    antn.elementary_type_name_base.name.clone()
                }
                _ => String::new(),
            },
            ElementaryTypeName::BoolTypeName(btn) => btn.elementary_type_name_base.name.clone(),
            ElementaryTypeName::BooleanLiteralType(blt) => {
                blt.elementary_type_name_base.name.clone()
            }
            _ => String::new(),
        }
    }

    pub fn visit_UserDefinedTypeName(&self, ast: UserDefinedTypeName) -> CodeVisitorReturn {
        let names: Vec<_> = (match ast {
            UserDefinedTypeName::EnumTypeName(ast) => ast.user_defined_type_name_base.names,
            UserDefinedTypeName::EnumValueTypeName(ast) => ast.user_defined_type_name_base.names,
            UserDefinedTypeName::StructTypeName(ast) => ast.user_defined_type_name_base.names,
            UserDefinedTypeName::ContractTypeName(ast) => ast.user_defined_type_name_base.names,
            UserDefinedTypeName::AddressTypeName(ast) => ast.user_defined_type_name_base.names,
            UserDefinedTypeName::AddressPayableTypeName(ast) => {
                ast.user_defined_type_name_base.names
            }
            _ => vec![],
        })
        .iter()
        .map(|name| ListUnion::AST(AST::Identifier(name.clone())))
        .collect();
        self.visit_list(names, ".")
    }

    pub fn visit_AddressTypeName(&self, ast: AddressTypeName) -> CodeVisitorReturn {
        String::from("address")
    }

    pub fn visit_AddressPayableTypeName(&self, ast: AddressPayableTypeName) -> CodeVisitorReturn {
        String::from("address payable")
    }

    pub fn visit_AnnotatedTypeName(&self, ast: AnnotatedTypeName) -> CodeVisitorReturn {
        let t = self.visit(&AST::TypeName(*ast.type_name));
        let p = if let Some(privacy_annotation) = ast.privacy_annotation {
            self.visit(&AST::Expression(*privacy_annotation))
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

    pub fn visit_Mapping(&self, ast: Mapping) -> CodeVisitorReturn {
        let k = self.visit(&AST::TypeName(TypeName::ElementaryTypeName(ast.key_type)));
        let label = if let Some(Identifier::Identifier(idf)) = ast.key_label {
            format!(
                "!{}",
                self.visit(&AST::Identifier(Identifier::Identifier(idf)))
            )
        } else {
            if let Some(Identifier::HybridArgumentIdf(key_label)) = ast.key_label {
                format!("/*!{:?}*/", key_label)
            } else {
                String::new()
            }
        };
        let v = self.visit(&AST::AnnotatedTypeName(*ast.value_type));
        format!("mapping({k}{label} => {v})")
    }

    pub fn visit_Array(&self, ast: Array) -> CodeVisitorReturn {
        let value_type = match &ast {
            Array::CipherText(ast) => &ast.array_base.value_type,
            Array::Randomness(ast) => &ast.array_base.value_type,
            Array::Key(ast) => &ast.array_base.value_type,
            Array::Proof(ast) => &ast.array_base.value_type,
            Array::Array(ast) => &ast.value_type,
        };
        let expr = match &ast {
            Array::CipherText(ast) => &ast.array_base.expr,
            Array::Randomness(ast) => &ast.array_base.expr,
            Array::Key(ast) => &ast.array_base.expr,
            Array::Proof(ast) => &ast.array_base.expr,
            Array::Array(ast) => &ast.expr,
        };
        let t = self.visit(&AST::AnnotatedTypeName(value_type.clone()));
        let e = if let Some(ExprUnion::Expression(expr)) = &expr {
            self.visit(&AST::Expression(expr.clone()))
        } else if let Some(ExprUnion::I32(expr)) = &expr {
            expr.to_string()
        } else {
            String::new()
        };
        format!("{t}[{e}]")
    }

    pub fn visit_CipherText(&self, ast: CipherText) -> CodeVisitorReturn {
        let e = self.visit_Array(Array::CipherText(ast.clone()));
        format!("{e}/*{}*/", ast.plain_type.into_ast().code())
    }

    pub fn visit_TupleType(&self, ast: TupleType) -> CodeVisitorReturn {
        let s = self.visit_list(
            ast.types
                .iter()
                .map(|typ| ListUnion::AST(AST::AnnotatedTypeName(typ.clone())))
                .collect(),
            ", ",
        );
        format!("({s})")
    }

    pub fn visit_VariableDeclaration(&self, ast: VariableDeclaration) -> CodeVisitorReturn {
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
        let t = self.visit(&AST::AnnotatedTypeName(
            *ast.identifier_declaration_base.annotated_type,
        ));
        let s = if let Some(storage_location) = ast.identifier_declaration_base.storage_location {
            format!(" {storage_location}")
        } else {
            String::new()
        };
        let i = self.visit(&AST::Identifier(*ast.identifier_declaration_base.idf));
        format!("{k} {t}{s} {i}").trim().to_string()
    }

    pub fn visit_VariableDeclarationStatement(
        &self,
        ast: VariableDeclarationStatement,
    ) -> CodeVisitorReturn {
        let mut s = self.visit(&AST::IdentifierDeclaration(
            IdentifierDeclaration::VariableDeclaration(ast.variable_declaration),
        ));
        if let Some(expr) = ast.expr {
            s += format!(" = {}", self.visit(&AST::Expression(expr))).as_str();
        }
        s += ";";
        s
    }

    pub fn visit_Parameter(&self, ast: Parameter) -> CodeVisitorReturn {
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
        let t = Some(self.visit(&AST::AnnotatedTypeName(
            *ast.identifier_declaration_base.annotated_type,
        )));
        let i = Some(self.visit(&AST::Identifier(*ast.identifier_declaration_base.idf)));
        let description: Vec<_> = [f, t, ast.identifier_declaration_base.storage_location, i]
            .iter()
            .filter_map(|d| d.clone())
            .collect();
        description.join(" ")
    }

    pub fn visit_ConstructorOrFunctionDefinition(
        &self,
        ast: ConstructorOrFunctionDefinition,
    ) -> CodeVisitorReturn {
        let b = if let Some(body) = ast.body {
            self.visit_single_or_list(
                SingleOrListUnion::AST(AST::Statement(Statement::StatementList(
                    StatementList::Block(body),
                ))),
                "",
            )
        } else {
            String::new()
        };
        self.function_definition_to_str(
            ast.namespace_definition_base.idf,
            ast.parameters
                .iter()
                .map(|parameter| ParameterUnion::Parameter(parameter.clone()))
                .collect(),
            ast.modifiers,
            ast.return_parameters,
            b,
        )
    }
    fn function_definition_to_str(
        &self,
        idf: Identifier,
        parameters: Vec<ParameterUnion>,
        modifiers: Vec<String>,
        return_parameters: Vec<Parameter>,
        body: String,
    ) -> CodeVisitorReturn {
        let definition = if idf.name() != String::from("constructor") {
            let i = self.visit(&AST::Identifier(idf));
            format!("function {i}")
        } else {
            String::from("constructor")
        };
        let p = self.visit_list(
            parameters
                .iter()
                .filter_map(|parameter| match parameter {
                    ParameterUnion::Parameter(p) => Some(ListUnion::AST(
                        AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(p.clone())),
                    )),
                    ParameterUnion::String(s) => Some(ListUnion::String(s.clone())),
                    _ => None,
                })
                .collect(),
            ", ",
        );
        let mut m = modifiers.join(" ");
        if !m.is_empty() {
            m = format!(" {m}");
        }
        let mut r = self.visit_list(
            return_parameters
                .iter()
                .map(|p| {
                    ListUnion::AST(AST::IdentifierDeclaration(
                        IdentifierDeclaration::Parameter(p.clone()),
                    ))
                })
                .collect(),
            ", ",
        );
        if !r.is_empty() {
            r = format!(" returns ({r})");
        }

        format!("{definition}({p}){m}{r} {body}")
    }

    pub fn visit_EnumValue(&self, ast: EnumValue) -> CodeVisitorReturn {
        if let Some(idf) = ast.idf {
            self.visit(&AST::Identifier(idf))
        } else {
            String::new()
        }
    }

    pub fn visit_EnumDefinition(&self, ast: EnumDefinition) -> CodeVisitorReturn {
        let values = indent(
            self.visit_list(
                ast.values
                    .iter()
                    .map(|value| ListUnion::AST(AST::EnumValue(value.clone())))
                    .collect(),
                ", ",
            ),
        );
        format!(
            "enum {} {{\n{values}\n}}",
            self.visit(&AST::Identifier(ast.namespace_definition_base.idf))
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
                Some(
                    v1.identifier_declaration_base
                        .annotated_type
                        .type_name
                        .clone(),
                ),
                Some(
                    v2.identifier_declaration_base
                        .annotated_type
                        .type_name
                        .clone(),
                ),
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

    pub fn visit_StructDefinition(&self, ast: StructDefinition) -> CodeVisitorReturn {
        // Define struct with members in order of descending size (to get maximum space savings through packing)
        let mut members_by_descending_size = ast.members.clone();
        members_by_descending_size.sort_by(|v1, v2| Self::__cmp_type_size(v1, v2).reverse());
        let body = indent(
            members_by_descending_size
                .iter()
                .map(|member| self.visit(&member.clone()))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        format!(
            "struct {} {{\n{body}\n}}",
            self.visit(&AST::Identifier(ast.namespace_definition_base.idf))
        )
    }

    pub fn visit_StateVariableDeclaration(
        &self,
        ast: StateVariableDeclaration,
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
        let t = self.visit(&AST::AnnotatedTypeName(
            *ast.identifier_declaration_base.annotated_type,
        ));
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
        let i = self.visit(&AST::Identifier(*ast.identifier_declaration_base.idf));
        let mut ret = format!("{f}{t} {k}{i}").trim().to_string();
        if let Some(expr) = ast.expr {
            ret += &format!(" = {}", self.visit(&AST::Expression(expr)));
        }
        ret + ";"
    }

    // @staticmethod
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

    pub fn visit_ContractDefinition(&self, ast: ContractDefinition) -> CodeVisitorReturn {
        let state_vars = ast
            .state_variable_declarations
            .iter()
            .map(|e| self.visit(&e.clone()))
            .collect::<Vec<_>>(); //[ for e in ast.state_variable_declarations]
        let constructors = ast
            .constructor_definitions
            .iter()
            .map(|e| {
                self.visit(&AST::NamespaceDefinition(
                    NamespaceDefinition::ConstructorOrFunctionDefinition(e.clone()),
                ))
            })
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.constructor_definitions]
        let functions = ast
            .function_definitions
            .iter()
            .map(|e| {
                self.visit(&AST::NamespaceDefinition(
                    NamespaceDefinition::ConstructorOrFunctionDefinition(e.clone()),
                ))
            })
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.function_definitions]
        let enums = ast
            .enum_definitions
            .iter()
            .map(|e| {
                self.visit(&AST::NamespaceDefinition(
                    NamespaceDefinition::EnumDefinition(e.clone()),
                ))
            })
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.enum_definitions]
        let structs = ast
            .struct_definitions
            .iter()
            .map(|e| {
                self.visit(&AST::NamespaceDefinition(
                    NamespaceDefinition::StructDefinition(e.clone()),
                ))
            })
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.struct_definitions]

        Self::contract_definition_to_str(
            ast.namespace_definition_base.idf,
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

    pub fn visit_SourceUnit(&self, ast: SourceUnit) -> CodeVisitorReturn {
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
