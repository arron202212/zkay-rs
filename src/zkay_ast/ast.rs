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
use crate::config::CFG; //, zk_print;
use crate::transaction::crypto::params::CryptoParams;
// use  crate::utils::progress_printer import warn_print;
use crate::zkay_ast::analysis::partition_state::PartitionState;
use crate::zkay_ast::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};

// use crate::zkay_ast::visitor::visitor::AstVisitor;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
// T = TypeVar('T')
struct ChildListBuilder {
    children: Vec<AST>,
}
impl ChildListBuilder {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
    pub fn add_child(&mut self, ast: AST) {
        if let AST::None = ast {
            return;
        }
        self.children.push(ast.clone());
    }
}
// class ChildListBuilder:
//     def __init__(self):
//         self.children = []

//     def add_child(self, ast: AST) -> AST:
//         if ast is not None:
//             self.children.append(ast)
//         return ast

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
pub trait Immutable {
    fn is_immutable(&self) -> bool;
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum AST {
    #[default]
    None,
    Identifier(IdentifierKind),
    Comment(CommentKind),
    Expression(Box<Expression>),
    Statement(Statement),
    TypeName(TypeName),
    // ConstructorOrFunctionDefinition(ConstructorOrFunctionDefinition),
    // ContractDefinition(ContractDefinition),
    // Block(Block),
    AnnotatedTypeName(AnnotatedTypeName),
    IdentifierDeclaration(IdentifierDeclaration),
    // StateVariableDeclaration(StateVariableDeclaration),
    NamespaceDefinition(NamespaceDefinition),
    // EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    SourceUnit(SourceUnit),
    // Parameters(Option<Vec<Parameter>>),
    // Modifiers(Option<Vec<String>>),
    // ReturnParameters(Option<Vec<Parameter>>),
    // BooleanLiteralExpr(BooleanLiteralExpr),
    // StringLiteralExpr(StringLiteralExpr),
    // NumberLiteralExpr(NumberLiteralExpr),
    // TupleExpr(TupleExpr),
    Pragma(String),
    VersionPragma(String),
    Modifier(String),
    Homomorphism(String),
    // AddressTypeName(AddressTypeName),
    // AddressPayableTypeName(AddressPayableTypeName),
    // BoolTypeName(BoolTypeName),
    // IntTypeName(IntTypeName),
    // UintTypeName(UintTypeName),
    // IndexExpr(IndexExpr),
    // LocationExpr(LocationExpr),
    // FunctionCallExpr(FunctionCallExpr),
    // IfStatement(IfStatement),
    // WhileStatement(WhileStatement),
    // DoWhileStatement(DoWhileStatement),
    // ForStatement(ForStatement),
    // SimpleStatement(SimpleStatement),
    // AssignmentStatement(AssignmentStatement),
    // RequireStatement(RequireStatement),
}
trait ASTChildren {
    fn children(&mut self) -> Vec<AST> {
        let mut cb = ChildListBuilder::new();
        self.process_children(&mut cb);
        cb.children.drain(..).collect()
    }

    fn process_children(&mut self, cb: &mut ChildListBuilder);
}

trait ASTCode {
    fn get_ast(&self) -> AST;
    fn code(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(&self.get_ast())
    }
}

impl AST {
    pub fn parent(&self) -> Option<AST> {
        None
    }
    pub fn code(&self) -> String {
        let v = CodeVisitor::new(true);
        v.visit(self)
    }
    pub fn is_parent_of(&self, child: &AST) -> bool {
        let mut e = child.clone();
        let selfs = self.clone();
        while e != selfs && e.parent().is_some() {
            e = e.parent().unwrap().clone();
        }
        e == selfs
    }
}

use std::fmt;
impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::new())
    }
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
}
// class AST:
//     def __init__(self):
//         // set later by parent setter
//         self.parent: Optional[AST] = None
//         self.namespace: Optional[List[Identifier]] = None

//         // Names accessible by AST nodes below this node.
//         // Does not include names already listed by parents.
//         // Maps strings (names) to Identifiers.
//         //
//         // set later by symbol table
//         self.names: Dict[str, Identifier] = {}

//         self.line = -1
//         self.column = -1

//         self.modified_values: OrderedDict[InstanceTarget, None] = OrderedDict()
//         self.read_values: Set[InstanceTarget] = set()

//     def children(self) -> List[AST]:
//         cb = ChildListBuilder()
//         self.process_children(cb.add_child)
//         return cb.children

//     def is_parent_of(self, child: AST) -> bool:
//         e = child
//         while e != self and e.parent is not None:
//             e = e.parent
//         return e == self

//     def override(self: T, **kwargs) -> T:
//         for key, val in kwargs.items():
//             if not hasattr(self, key):
//                 raise ValueError(f'Class "{type(self).__name__}" does not have property "{key}"')
//             setattr(self, key, val)
//         return self

//     def process_children(self, f: Callable[[T], T]):
//         pass

//     def code(self) -> str:
//         v = CodeVisitor()
//         s = v.visit(self)
//         return s

//     @property
//     def qualified_name(self) -> List[Identifier]:
//         if not hasattr(self, 'idf'):
//             return []
//         if self.namespace[-1] == self.idf:
//             return self.namespace
//         else:
//             return self.namespace + [self.idf]

//     def __str__(self):
//         return self.code()

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Identifier {
    pub parent: Option<Box<AST>>,
    pub name: String,
}
impl Identifier {
    fn new(name: String) -> Self {
        Self { parent: None, name }
    }
}
impl Immutable for Identifier {
    fn is_immutable(&self) -> bool {
        if let Some(v) = &self.parent {
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
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
// class Identifier(AST):

//     def __init__(self, name: str):
//         super().__init__()
//         self.name = name

//     @property
//     def is_immutable(self):
//         return isinstance(self.parent, StateVariableDeclaration) and (self.parent.is_final or self.parent.is_constant)

//     def clone(self) -> Identifier:
//         return Identifier(self.name)

//     def decl_var(self, t: Union[TypeName, AnnotatedTypeName], expr: Optional[Expression] = None):
//         if isinstance(t, TypeName):
//             t = AnnotatedTypeName(t)
//         storage_loc = '' if t.type_name.is_primitive_type() else 'memory'
//         return VariableDeclarationStatement(VariableDeclaration([], t, self.clone(), storage_loc), expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum CommentKind {
    #[default]
    None,
    Comment(Comment),
    BlankLine(BlankLine),
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Comment {
    pub text: String,
}
impl Comment {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
// class Comment(AST):

//     def __init__(self, text: str = ''):
//         super().__init__()
//         self.text = text

//     @staticmethod
//     def comment_list(text: str, block: List[AST]) -> List[AST]:
//         return block if not block else [Comment(text)] + block + [BlankLine()]

//     @staticmethod
//     def comment_wrap_block(text: str, block: List[AST]) -> List[AST]:
//         if not block:
//             return block
//         return [Comment(f'{text}'), Comment('{'), IndentBlock(block), Comment('}'), BlankLine()]

// class BlankLine(Comment):
//     def __init__(self):
//         super().__init__()
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BlankLine {
    pub text: String,
}
impl BlankLine {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum Expression {
    BuiltinFunction(Box<BuiltinFunction>),
    FunctionCallExpr(Box<FunctionCallExpr>),
    PrimitiveCastExpr(Box<PrimitiveCastExpr>),
    LiteralExpr(Box<LiteralExpr>),
    TupleOrLocationExpr(Box<TupleOrLocationExpr>),
    MeExpr(MeExpr),
    AllExpr(AllExpr),
    ReclassifyExpr(Box<ReclassifyExprKind>),
    #[serde(rename_all = "camelCase")]
    #[default]
    None,
}
impl Expression {
    fn is_all_expr(&self) -> bool {
        if let Expression::AllExpr(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionBase {
    pub annotated_type: Option<AnnotatedTypeName>,
    pub statement: Option<Statement>,
    pub evaluate_privately: bool,
}
impl ExpressionBase {
    pub fn new() -> Self {
        Self {
            annotated_type: None,
            statement: None,
            evaluate_privately: false,
        }
    }
}
// class Expression(AST):

//     @staticmethod
//     def all_expr():
//         return AllExpr()

//     @staticmethod
//     def me_expr(stmt: Optional[Statement] = None):
//         me = MeExpr()
//         me.statement = stmt
//         return me

//     def explicitly_converted(self: T, expected: TypeName) -> Union[T, FunctionCallExpr]:
//         if expected == TypeName.bool_type() and not self.instanceof_data_type(TypeName.bool_type()):
//             ret = FunctionCallExpr(BuiltinFunction('!='), [self, NumberLiteralExpr(0)])
//         elif expected.is_numeric and self.instanceof_data_type(TypeName.bool_type()):
//             ret = FunctionCallExpr(BuiltinFunction('ite'), [self, NumberLiteralExpr(1), NumberLiteralExpr(0)])
//         else:
//             t = self.annotated_type.type_name

//             if t == expected:
//                 return self

//             // Explicit casts
//             cast = False
//             if isinstance(t, NumberTypeName) and isinstance(expected, (NumberTypeName, AddressTypeName, AddressPayableTypeName, EnumTypeName)):
//                 cast = True
//             elif isinstance(t, AddressTypeName) and isinstance(expected, NumberTypeName):
//                 cast = True
//             elif isinstance(t, AddressPayableTypeName) and isinstance(expected, (NumberTypeName, AddressTypeName)):
//                 cast = True
//             elif isinstance(t, EnumTypeName) and isinstance(expected, NumberTypeName):
//                 cast = True

//             assert cast
//             return PrimitiveCastExpr(expected, self).as_type(expected)

//         ret.annotated_type = AnnotatedTypeName(expected.clone(), self.annotated_type.privacy_annotation.clone(),
//                                                self.annotated_type.homomorphism)
//         return ret

//     def __init__(self):
//         super().__init__()
//         // set later by type checker
//         self.annotated_type: Optional[AnnotatedTypeName] = None
//         // set by expression to statement
//         self.statement: Optional[Statement] = None

//         self.evaluate_privately = False

//     def is_all_expr(self):
//         return self == Expression.all_expr()

//     def is_me_expr(self):
//         return self == Expression.me_expr()

//     def privacy_annotation_label(self):
//         if isinstance(self, IdentifierExpr):
//             if isinstance(self.target, Mapping):
//                 return self.target.instantiated_key.privacy_annotation_label()
//             else:
//                 return self.target.idf
//         elif self.is_all_expr():
//             return self
//         elif self.is_me_expr():
//             return self
//         else:
//             return None

//     def instanceof_data_type(self, expected: TypeName) -> bool:
//         return self.annotated_type.type_name.implicitly_convertible_to(expected)

//     def unop(self, op: str) -> FunctionCallExpr:
//         return FunctionCallExpr(BuiltinFunction(op), [self])

//     def binop(self, op: str, rhs: Expression) -> FunctionCallExpr:
//         return FunctionCallExpr(BuiltinFunction(op), [self, rhs])

//     def ite(self, e_true: Expression, e_false: Expression) -> FunctionCallExpr:
//         return FunctionCallExpr(BuiltinFunction('ite').override(is_private=self.annotated_type.is_private), [self, e_true, e_false])

//     def instanceof(self, expected):
//         """

//         :param expected:
//         :return: True, False, or 'make-private'
//         """
//         assert (isinstance(expected, AnnotatedTypeName))

//         actual = self.annotated_type

//         if not self.instanceof_data_type(expected.type_name):
//             return False

//         // check privacy type and homomorphism
//         combined_label = actual.combined_privacy(self.analysis, expected)
//         if combined_label is None:
//             return False
//         elif isinstance(combined_label, List):
//             assert isinstance(self.annotated_type.type_name, TupleType) and not isinstance(self, TupleExpr)
//             return combined_label == [t.privacy_annotation for t in self.annotated_type.type_name.types]
//         elif combined_label.privacy_annotation_label() == actual.privacy_annotation.privacy_annotation_label():
//             return True
//         else:
//             return 'make-private'

//     def as_type(self: T, t: Union[TypeName, AnnotatedTypeName]) -> T:
//         return self.override(annotated_type=t if isinstance(t, AnnotatedTypeName) else AnnotatedTypeName(t))

//     @property
//     def analysis(self):
//         if self.statement is None:
//             return None
//         else:
//             return self.statement.before_analysis

// builtin_op_fct = {
//     '+': operator.add, '-': operator.sub,
//     '**': operator.pow, '*': operator.mul, '/': operator.floordiv, '%': operator.mod,
//     'sign+': lambda a: a, 'sign-': operator.neg,
//     '<<': operator.lshift, '>>': operator.rshift,
//     '|': operator.or_, '&': operator.and_, '^': operator.xor, '~': operator.inv,
//     '<': operator.lt, '>': operator.gt, '<=': operator.le, '>=': operator.ge,
//     '==': operator.eq, '!=': operator.ne,
//     '&&': lambda a, b: a and b, '||': lambda a, b: a or b, '!': operator.not_,
//     'ite': lambda a, b, c: b if a else c,
//     'parenthesis': lambda a: a
// }
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum LiteralType {
    Bool(bool),
    Number(i32),
    #[default]
    None,
}
pub fn builtin_op_fct(op: &str, args: Vec<i32>) -> LiteralType {
    match op {
        "+" => LiteralType::Number(args[0] + args[1]),
        "-" => LiteralType::Number(args[0] - args[1]),
        "**" => LiteralType::Number(args[0].pow(args[1] as u32)),
        "*" => LiteralType::Number(args[0] * args[1]),
        "/" => LiteralType::Number(args[0] / args[1]),
        "%" => LiteralType::Number(args[0] % args[1]),
        "sign+" => LiteralType::Number(args[0]),
        "sign-" => LiteralType::Number(-args[0]),
        "<<" => LiteralType::Number(args[0] << args[1]),
        ">>" => LiteralType::Number(args[0] >> args[1]),
        "|" => LiteralType::Number(args[0] | args[1]),
        "&" => LiteralType::Number(args[0] & args[1]),
        "^" => LiteralType::Number(args[0] ^ args[1]),
        "~" => LiteralType::Number(!args[0]),
        "<" => LiteralType::Bool(args[0] < args[1]),
        ">" => LiteralType::Bool(args[0] > args[1]),
        "<=" => LiteralType::Bool(args[0] <= args[1]),
        ">=" => LiteralType::Bool(args[0] >= args[1]),
        "==" => LiteralType::Bool(args[0] == args[1]),
        "!=" => LiteralType::Bool(args[0] != args[1]),
        "&&" => LiteralType::Bool(args[0] != 0 && args[1] != 0),
        "||" => LiteralType::Bool(args[0] != 0 || args[1] != 0),
        "!" => LiteralType::Bool(!(args[0] != 0)),
        "ite" => LiteralType::Number(if args[0] != 0 { args[1] } else { args[2] }),
        "parenthesis" => LiteralType::Number(args[0]),
        _ => LiteralType::Bool(false),
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
}

// @dataclass
// class HomomorphicBuiltin:
//     """
//     Just a named tuple that describes an available homomorphic operation.
//     """
//     op: str
//     homomorphism: Homomorphism
//     public_args: List[bool]
//     """
//     A list that describes what arguments are required to be public to be able to use this homomorphic function.
//     """
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BuiltinFunction {
    pub line: i32,
    pub column: i32,
    pub op: String,
    pub is_private: bool,
    pub homomorphism: String,
    pub rerand_using: Option<Box<IdentifierExpr>>,
}
impl BuiltinFunction {
    pub fn new(op: &str) -> Self {
        let op = op.to_string();
        assert!(
            BUILTIN_FUNCTIONS.get(&op).is_some(),
            "{op} is not a known built-in function"
        );
        Self {
            line: -1,
            column: -1,
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
    pub fn arity(&self) -> i32 {
        BUILTIN_FUNCTIONS[&self.op].matches("{}").count() as i32
    }
}
// class BuiltinFunction(Expression):

//     def __init__(self, op: str):
//         super().__init__()
//         self.op = op

//         // set later by type checker
//         self.is_private: bool = False
//         self.homomorphism: Homomorphism = Homomorphism.NON_HOMOMORPHIC

//         // set later by transformation
//         self.rerand_using: Optional['IdentifierExpr'] = None

//         // input validation
//         if op not in BUILTIN_FUNCTIONS:
//             raise ValueError(f'{op} is not a known built-in function')

//     def format_string(self):
//         return BUILTIN_FUNCTIONS[self.op]

//     @property
//     def op_func(self):
//         return builtin_op_fct[self.op]

//     def is_arithmetic(self):
//         return self.op in arithmetic

//     def is_neg_sign(self):
//         return self.op == 'sign-'

//     def is_comp(self):
//         return self.op in comp

//     def is_eq(self):
//         return self.op in eq

//     def is_bop(self):
//         return self.op in bop

//     def is_bitop(self):
//         return self.op in bitop

//     def is_shiftop(self):
//         return self.op in shiftop

//     def is_parenthesis(self):
//         return self.op == 'parenthesis'

//     def is_ite(self):
//         return self.op == 'ite'

//     def has_shortcircuiting(self):
//         return self.is_ite() or self.op == '&&' or self.op == '||'

//     def arity(self):
//         return self.format_string().count('{}')

//     def input_types(self):
//         """

//         :return: None if the type is generic
//         """
//         if self.is_arithmetic():
//             t = TypeName.number_type()
//         elif self.is_comp():
//             t = TypeName.number_type()
//         elif self.is_bop():
//             t = TypeName.bool_type()
//         elif self.is_bitop():
//             t = TypeName.number_type()
//         elif self.is_shiftop():
//             t = TypeName.number_type()
//         else:
//             // eq, parenthesis, ite
//             return None

//         return self.arity() * [t]

//     def output_type(self):
//         """

//         :return: None if the type is generic
//         """
//         if self.is_arithmetic():
//             return TypeName.number_type()
//         elif self.is_comp():
//             return TypeName.bool_type()
//         elif self.is_bop():
//             return TypeName.bool_type()
//         elif self.is_eq():
//             return TypeName.bool_type()
//         elif self.is_bitop():
//             return TypeName.number_type()
//         elif self.is_shiftop():
//             return TypeName.number_type()
//         else:
//             // parenthesis, ite
//             return None

//     def can_be_private(self) -> bool:
//         """

//         :return: true if operation itself can be run inside a circuit \
//                  for equality and ite it must be checked separately whether the arguments are also supported inside circuits
//         """
//         return self.op != '**'

//     def select_homomorphic_overload(self, args: List[Expression], analysis: PartitionState[PrivacyLabelExpr]):
//         """
//         Finds a homomorphic builtin that performs the correct operation and which can be applied
//         on the arguments, if any exist.

//         :return: A HomomorphicBuiltinFunction that can be used to query the required input types and
//                  the resulting output type of the homomorphic operation, or None
//         """

//         // The first inaccessible (not @all, not @me) determines the output type
//         // self.op and the public arguments determine which homomorphic builtin is selected
//         // We may want to rethink this in the future if we also implement other homomorphisms (e.g. multiplicative)

//         arg_types = list(map(lambda x: x.annotated_type, args))
//         inaccessible_arg_types = list(filter(lambda x: not x.is_accessible(analysis), arg_types))
//         if len(inaccessible_arg_types) == 0:  // Else we would not have selected a homomorphic operation
//             raise ValueError('Cannot select proper homomorphic function if all arguments are public or @me-private')
//         elem_type = reduce(lambda l, r: l.combined_type(r, True), map(lambda a: a.type_name, arg_types))
//         base_type = AnnotatedTypeName(elem_type, inaccessible_arg_types[0].privacy_annotation)
//         public_args = list(map(AnnotatedTypeName.is_public, arg_types))

//         for hom in HOMOMORPHIC_BUILTIN_FUNCTIONS:
//             // Can have more public arguments, but not fewer (hom.public_args[i] implies public_args[i])
//             args_match = [(not h) or a for a, h in zip(public_args, hom.public_args)]
//             if self.op == hom.op and all(args_match):
//                 target_type = base_type.with_homomorphism(hom.homomorphism)
//                 return HomomorphicBuiltinFunction(target_type, hom.public_args)
//         if self.op == '*'\
//             and not args[0].annotated_type.is_accessible(analysis)\
//             and not args[1].annotated_type.is_accessible(analysis)\
//             and (isinstance(args[0], ReclassifyExpr) and not isinstance(args[1], ReclassifyExpr)) \
//                 or (isinstance(args[1], ReclassifyExpr) and not isinstance(args[0], ReclassifyExpr)):
//             // special case: private scalar multiplication using additive homomorphism
//             target_type = base_type.with_homomorphism(Homomorphism.ADDITIVE)
//             return HomomorphicBuiltinFunction(target_type, [False, False])
//         else:
//             return None

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HomomorphicBuiltinFunction {
    pub target_type: AnnotatedTypeName,
    pub public_args: Vec<bool>,
}
// class HomomorphicBuiltinFunction:
//     """
//     Describes the required input types and the resulting output type of a homomorphic execution of a BuiltinFunction.
//     """
//     target_type: AnnotatedTypeName
//     public_args: List[bool]

//     def __init__(self, target_type, public_args):
//         self.target_type = target_type
//         self.public_args = public_args

//     def input_types(self):
//         public_type = AnnotatedTypeName.all(self.target_type.type_name)  // same data type, but @all
//         return [public_type if public else self.target_type for public in self.public_args]

//     def output_type(self):
//         return self.target_type
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FunctionCallExprKind {
    FunctionCallExpr(FunctionCallExpr),
    NewExpr(NewExpr),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FunctionCallExpr {
    pub line: i32,
    pub column: i32,
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub sec_start_offset: Option<i32>,
}
impl FunctionCallExpr {
    pub fn new(
        func: Box<Expression>,
        args: Vec<Expression>,
        sec_start_offset: Option<i32>,
    ) -> Self {
        Self {
            line: -1,
            column: -1,
            func,
            args,
            sec_start_offset,
        }
    }
}

impl ASTChildren for FunctionCallExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(self.func.clone()));
        self.args.iter().for_each(|arg| {
            cb.add_child(AST::Expression(Box::new(arg.clone())));
        });
    }
}
// class FunctionCallExpr(Expression):

//     def __init__(self, func: Expression, args: List[Expression], sec_start_offset: Optional[int] = 0):
//         super().__init__()
//         self.func = func
//         self.args = args
//         self.sec_start_offset = sec_start_offset

//     @property
//     def is_cast(self):
//         return isinstance(self.func, LocationExpr) and isinstance(self.func.target, (ContractDefinition, EnumDefinition))

//     def process_children(self, f: Callable[[T], T]):
//         self.func = f(self.func)
//         self.args[:] = map(f, self.args)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NewExpr {
    pub line: i32,
    pub column: i32,
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub sec_start_offset: Option<i32>,
}
impl NewExpr {
    pub fn new(
        func: Box<Expression>,
        args: Vec<Expression>,
        sec_start_offset: Option<i32>,
    ) -> Self {
        Self {
            line: -1,
            column: -1,
            func,
            args,
            sec_start_offset,
        }
    }
}
impl ASTChildren for NewExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(self.func.clone()));
        self.args.iter().for_each(|arg| {
            cb.add_child(AST::Expression(Box::new(arg.clone())));
        });
    }
}
// class NewExpr(FunctionCallExpr):
//     def __init__(self, annotated_type: AnnotatedTypeName, args: List[Expression]):
//         assert not isinstance(annotated_type, ElementaryTypeName)
//         super().__init__(Identifier(f'new {annotated_type.code()}'), args)
//         self.annotated_type = annotated_type

//     def process_children(self, f: Callable[[T], T]):
//         self.annotated_type = f(self.annotated_type)
//         self.args[:] = map(f, self.args)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PrimitiveCastExpr {
    pub elem_type: TypeName,
    pub expr: Box<Expression>,
    pub is_implicit: bool,
}
impl ASTChildren for PrimitiveCastExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::TypeName(self.elem_type.clone()));
        cb.add_child(AST::Expression(self.expr.clone()));
    }
}
// class PrimitiveCastExpr(Expression):
//     def __init__(self, elem_type: TypeName, expr: Expression, is_implicit=False):
//         super().__init__()
//         self.elem_type = elem_type
//         self.expr = expr
//         self.is_implicit = is_implicit

//     def process_children(self, f: Callable[[T], T]):
//         self.elem_type = f(self.elem_type)
//         self.expr = f(self.expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum LiteralExpr {
    BooleanLiteralExpr(Box<BooleanLiteralExpr>),
    NumberLiteralExpr(NumberLiteralExpr),
    StringLiteralExpr(StringLiteralExpr),
    ArrayLiteralExpr(ArrayLiteralExpr),
    #[serde(rename_all = "camelCase")]
    #[default]
    None,
}

// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct LiteralExpr {
//     pub annotated_type: Option<AnnotatedTypeName>,
//     pub statement: Option<Statement>,
//     pub evaluate_privately: bool,
// }
// impl LiteralExpr{
//     pub fn new()->Self{
//         Self{annotated_type:None,statement:None,evaluate_privately:false}
//     }
// }
// class LiteralExpr(Expression):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BooleanLiteralExpr {
    pub value: bool,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
impl BooleanLiteralExpr {
    pub fn new(value: bool) -> Self {
        Self {
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
}
// class BooleanLiteralExpr(LiteralExpr):

//     def __init__(self, value: bool):
//         super().__init__()
//         self.value = value
//         self.annotated_type = AnnotatedTypeName(BooleanLiteralType(self.value))

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NumberLiteralExpr {
    pub line: i32,
    pub column: i32,
    pub value: i32,
    pub was_hex: bool,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
impl NumberLiteralExpr {
    pub fn new(value: i32, was_hex: bool) -> Self {
        Self {
            line: -1,
            column: -1,
            value,
            was_hex,
            annotated_type: Some(Box::new(AnnotatedTypeName::new(
                TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                    NumberTypeName::NumberLiteralType(NumberLiteralType::new(
                        NumberLiteralTypeKind::I32(value),
                    )),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
            ))),
        }
    }
}

// class NumberLiteralExpr(LiteralExpr):

//     def __init__(self, value: int, was_hex: bool = False):
//         super().__init__()
//         self.value = value
//         self.annotated_type = AnnotatedTypeName(NumberLiteralType(self.value))
//         self.was_hex = was_hex

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StringLiteralExpr {
    pub value: String,
}
impl StringLiteralExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
// class StringLiteralExpr(LiteralExpr):

//     def __init__(self, value: str):
//         super().__init__()
//         self.value = value

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ArrayLiteralExpr {
    pub values: Vec<Expression>,
}
impl ASTChildren for ArrayLiteralExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.values.iter().for_each(|value| {
            cb.add_child(AST::Expression(Box::new(value.clone())));
        });
    }
}
// class ArrayLiteralExpr(LiteralExpr):

//     def __init__(self, values: List[Expression]):
//         super().__init__()
//         self.values = values

//     def process_children(self, f: Callable[[T], T]):
//         self.values[:] = map(f, self.values)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct KeyLiteralExpr {
    pub values: Vec<Expression>,
    pub crypto_params: CryptoParams,
}
// class KeyLiteralExpr(ArrayLiteralExpr):

//     def __init__(self, values: List[Expression], crypto_params: CryptoParams):
//         super().__init__(values)
//         self.crypto_params = crypto_params

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum TupleOrLocationExpr {
    TupleExpr(Box<TupleExpr>),
    LocationExpr(Box<LocationExpr>),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleOrLocationExprBase {
    pub expression_base: ExpressionBase,
    pub parent: Option<Box<AST>>,
}
impl TupleOrLocationExprBase {
    pub fn new() -> Self {
        Self {
            expression_base: ExpressionBase::new(),
            parent: None,
        }
    }
}

// class TupleOrLocationExpr(Expression):
//     def is_lvalue(self) -> bool:
//         if isinstance(self.parent, AssignmentStatement):
//             return self == self.parent.lhs
//         if isinstance(self.parent, IndexExpr) and self == self.parent.arr:
//             return self.parent.is_lvalue()
//         if isinstance(self.parent, MemberAccessExpr) and self == self.parent.expr:
//             return self.parent.is_lvalue()
//         if isinstance(self.parent, TupleExpr):
//             return self.parent.is_lvalue()
//         return False

//     def is_rvalue(self) -> bool:
//         return not self.is_lvalue()

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleExpr {
    pub annotated_type: Option<AnnotatedTypeName>,
    pub statement: Option<Statement>,
    pub evaluate_privately: bool,
    elements: Vec<Expression>,
}
impl TupleExpr {
    pub fn new(elements: Vec<Expression>) -> Self {
        Self {
            annotated_type: None,
            statement: None,
            evaluate_privately: false,
            elements,
        }
    }
}
impl ASTChildren for TupleExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        self.elements.iter().for_each(|element| {
            cb.add_child(AST::Expression(Box::new(element.clone())));
        });
    }
}
// class TupleExpr(TupleOrLocationExpr):
//     def __init__(self, elements: List[Expression]):
//         super().__init__()
//         self.elements = elements

//     def process_children(self, f: Callable[[T], T]):
//         self.elements[:] = map(f, self.elements)

//     def assign(self, val: Expression) -> AssignmentStatement:
//         return AssignmentStatement(self, val)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum LocationExpr {
    IdentifierExpr(IdentifierExpr),
    MemberAccessExpr(Box<MemberAccessExpr>),
    IndexExpr(Box<IndexExpr>),
    SliceExpr(Box<SliceExpr>),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LocationExprBase {
    pub tuple_or_location_expr_base: TupleOrLocationExprBase,
    pub target: Option<TargetDefinition>,
}
impl LocationExprBase {
    pub fn new() -> Self {
        Self {
            tuple_or_location_expr_base: TupleOrLocationExprBase::new(),
            target: None,
        }
    }
}
// class LocationExpr(TupleOrLocationExpr):
//     def __init__(self):
//         super().__init__()
//         // set later by symbol table
//         self.target: Optional[TargetDefinition] = None

//     def call(self, member: Union[None, str, Identifier], args: List[Expression]) -> FunctionCallExpr:
//         if member is None:
//             return FunctionCallExpr(self, args)
//         else:
//             member = Identifier(member) if isinstance(member, str) else member.clone()
//             return FunctionCallExpr(MemberAccessExpr(self, member), args)

//     def dot(self, member: Union[str, Identifier]) -> MemberAccessExpr:
//         member = Identifier(member) if isinstance(member, str) else member.clone()
//         return MemberAccessExpr(self, member)

//     def index(self, item: Union[int, Expression]) -> IndexExpr:
//         assert isinstance(self.annotated_type.type_name, (Array, Mapping))
//         if isinstance(item, int):
//             item = NumberLiteralExpr(item)
//         return IndexExpr(self, item).as_type(self.annotated_type.type_name.value_type)

//     def assign(self, val: Expression) -> AssignmentStatement:
//         return AssignmentStatement(self, val)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IdentifierExprUnion {
    String(String),
    Identifier(Identifier),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierExpr {
    pub location_expr_base: LocationExprBase,
    pub idf: Identifier,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
impl IdentifierExpr {
    pub fn new(idf: IdentifierExprUnion, annotated_type: Option<Box<AnnotatedTypeName>>) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            idf: match idf {
                IdentifierExprUnion::Identifier(idf) => idf,
                IdentifierExprUnion::String(idf) => Identifier::new(idf),
                _ => Identifier::new(String::new()),
            },
            annotated_type,
        }
    }
}
impl ASTChildren for IdentifierExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Identifier(IdentifierKind::Identifier(
            self.idf.clone(),
        )));
    }
}
// class IdentifierExpr(LocationExpr):

//     def __init__(self, idf: Union[str, Identifier], annotated_type: Optional[AnnotatedTypeName] = None):
//         super().__init__()
//         self.idf: Identifier = idf if isinstance(idf, Identifier) else Identifier(idf)
//         self.annotated_type = annotated_type

//     def get_annotated_type(self):
//         return self.target.annotated_type

//     def process_children(self, f: Callable[[T], T]):
//         self.idf = f(self.idf)

//     def slice(self, offset: int, size: int, base: Optional[Expression] = None) -> SliceExpr:
//         return SliceExpr(self.clone(), base, offset, size)

//     def clone(self) -> IdentifierExpr:
//         idf = IdentifierExpr(self.idf.clone()).as_type(self.annotated_type)
//         idf.target = self.target
//         return idf

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MemberAccessExpr {
    pub location_expr_base: LocationExprBase,
    pub expr: LocationExpr,
    pub member: Identifier,
}
impl MemberAccessExpr {
    pub fn new(expr: LocationExpr, member: Identifier) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            expr,
            member,
        }
    }
}
impl ASTChildren for MemberAccessExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Box::new(Expression::TupleOrLocationExpr(
            Box::new(TupleOrLocationExpr::LocationExpr(Box::new(
                self.expr.clone(),
            ))),
        ))));
        cb.add_child(AST::Identifier(IdentifierKind::Identifier(
            self.member.clone(),
        )));
    }
}
// class MemberAccessExpr(LocationExpr):
//     def __init__(self, expr: LocationExpr, member: Identifier):
//         super().__init__()
//         assert isinstance(expr, LocationExpr)
//         self.expr = expr
//         self.member = member

//     def process_children(self, f: Callable[[T], T]):
//         self.expr = f(self.expr)
//         self.member = f(self.member)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IndexExpr {
    pub location_expr_base: LocationExprBase,
    pub arr: LocationExpr,
    pub key: Expression,
}
impl IndexExpr {
    pub fn new(arr: LocationExpr, key: Expression) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            arr,
            key,
        }
    }
}
impl ASTChildren for IndexExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Box::new(Expression::TupleOrLocationExpr(
            Box::new(TupleOrLocationExpr::LocationExpr(Box::new(
                self.arr.clone(),
            ))),
        ))));
        cb.add_child(AST::Expression(Box::new(self.key.clone())));
    }
}
// class IndexExpr(LocationExpr):
//     def __init__(self, arr: LocationExpr, key: Expression):
//         super().__init__()
//         assert isinstance(arr, LocationExpr)
//         self.arr = arr
//         self.key = key

//     def process_children(self, f: Callable[[T], T]):
//         self.arr = f(self.arr)
//         self.key = f(self.key)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SliceExpr {
    pub location_expr_base: LocationExprBase,
    pub arr: LocationExpr,
    pub base: Option<Expression>,
    pub start_offset: i32,
    pub size: i32,
}
impl SliceExpr {
    pub fn new(arr: LocationExpr, base: Option<Expression>, start_offset: i32, size: i32) -> Self {
        Self {
            location_expr_base: LocationExprBase::new(),
            arr,
            base,
            start_offset,
            size,
        }
    }
}
// class SliceExpr(LocationExpr):
//     def __init__(self, arr: LocationExpr, base: Optional[Expression], start_offset: int, size: int):
//         super().__init__()
//         self.arr = arr
//         self.base = base
//         self.start_offset = start_offset
//         self.size = size

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MeExpr {
    name: String,
}
impl Immutable for MeExpr {
    fn is_immutable(&self) -> bool {
        true
    }
}
impl MeExpr {
    pub fn new() -> Self {
        Self {
            name: String::from("me"),
        }
    }
}
// class MeExpr(Expression):
//     name = 'me'

//     @property
//     def is_immutable(self) -> bool:
//         return True

//     def clone(self) -> MeExpr:
//         return MeExpr()

//     def __eq__(self, other):
//         return isinstance(other, MeExpr)

//     def __hash__(self):
//         return hash('me')
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AllExpr {
    name: String,
}
impl AllExpr {
    pub fn new() -> Self {
        Self {
            name: String::from("all"),
        }
    }
}
impl Immutable for AllExpr {
    fn is_immutable(&self) -> bool {
        true
    }
}
// class AllExpr(Expression):
//     name = 'all'

//     @property
//     def is_immutable(self) -> bool:
//         return True

//     def clone(self) -> AllExpr:
//         return AllExpr()

//     def __eq__(self, other):
//         return isinstance(other, AllExpr)

//     def __hash__(self):
//         return hash('all')

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum ReclassifyExprKind {
    ReclassifyExpr(Box<ReclassifyExpr>),
    RehomExpr(Box<RehomExpr>),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReclassifyExpr {
    pub expr: Box<Expression>,
    pub privacy: Expression,
    pub homomorphism: Option<String>,
}
impl ReclassifyExpr {
    pub fn new(expr: Box<Expression>, privacy: Expression, homomorphism: Option<String>) -> Self {
        Self {
            expr,
            privacy,
            homomorphism,
        }
    }
    pub fn func_name(&self) -> String {
        String::from("reveal")
    }
}
impl ASTChildren for ReclassifyExpr {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(self.expr.clone()));
        cb.add_child(AST::Expression(Box::new(self.privacy.clone())));
    }
}
// class ReclassifyExpr(Expression):

//     def __init__(self, expr: Expression, privacy: Expression, homomorphism: Optional[Homomorphism]):
//         super().__init__()
//         self.expr = expr
//         self.privacy = privacy
//         self.homomorphism = homomorphism

//     def process_children(self, f: Callable[[T], T]):
//         self.expr = f(self.expr)
//         self.privacy = f(self.privacy)

//     def func_name(self):
//         return 'reveal'

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RehomExpr {
    pub expr: Box<Expression>,
    pub privacy: Expression,
    pub homomorphism: Option<String>,
}
impl RehomExpr {
    pub fn new(expr: Box<Expression>, homomorphism: Option<String>) -> Self {
        Self {
            expr,
            privacy: Expression::MeExpr(MeExpr::new()),
            homomorphism,
        }
    }
    pub fn func_name(&self) -> String {
        HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .get(self.homomorphism.as_ref().unwrap())
            .unwrap()
            .rehom_expr_name
            .clone()
    }
}
// class RehomExpr(ReclassifyExpr):

//     def __init__(self, expr: Expression, homomorphism: Homomorphism):
//         super().__init__(expr, MeExpr(), homomorphism)

//     def func_name(self):
//         return self.homomorphism.rehom_expr_name

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HybridArgType {
    #[default]
    PrivCircuitVal,
    PubCircuitArg,
    PubContractVal,
    TmpCircuitVal,
}
// class HybridArgType(IntEnum):
//     PrivCircuitVal = 0
//     PubCircuitArg = 1
//     PubContractVal = 2
//     TmpCircuitVal = 3

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HybridArgumentIdf {
    pub name: String,
    pub t: TypeName,
    pub arg_type: HybridArgType,
    pub corresponding_priv_expression: Vec<Expression>,
    pub serialized_loc: SliceExpr,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum IdentifierKind {
    #[default]
    None,
    Identifier(Identifier),
    HybridArgumentIdf(HybridArgumentIdf),
}
// class HybridArgumentIdf(Identifier):
//     def __init__(self, name: str, t: TypeName, arg_type: HybridArgType, corresponding_priv_expression: Optional[Expression] = None):
//         super().__init__(name)
//         self.t = t  // transformed type of this idf
//         if isinstance(t, BooleanLiteralType):
//             self.t = TypeName.bool_type()
//         elif isinstance(t, NumberLiteralType):
//             self.t = t.to_abstract_type()
//         elif isinstance(t, EnumValueTypeName):
//             self.t = t.to_abstract_type()
//         self.arg_type = arg_type
//         self.corresponding_priv_expression = corresponding_priv_expression
//         self.serialized_loc: SliceExpr = SliceExpr(IdentifierExpr(''), None, -1, -1)

//     def get_loc_expr(self, parent=None) -> Union[LocationExpr, NumberLiteralExpr, BooleanLiteralExpr]:
//         if self.arg_type == HybridArgType.TmpCircuitVal and isinstance(self.corresponding_priv_expression.annotated_type.type_name, BooleanLiteralType):
//             return BooleanLiteralExpr(self.corresponding_priv_expression.annotated_type.type_name.value)
//         elif self.arg_type == HybridArgType.TmpCircuitVal and isinstance(self.corresponding_priv_expression.annotated_type.type_name, NumberLiteralType):
//             return NumberLiteralExpr(self.corresponding_priv_expression.annotated_type.type_name.value)
//         else:
//             assert self.arg_type == HybridArgType.PubCircuitArg
//             ma = IdentifierExpr(cfg.zk_data_var_name).dot(self).as_type(self.t)
//             return ma.override(parent=parent, statement=parent if (parent is None or isinstance(parent, Statement)) else parent.statement)

//     def get_idf_expr(self, parent=None) -> IdentifierExpr:
//         ie = IdentifierExpr(self.clone()).as_type(self.t)
//         return ie.override(parent=parent, statement=parent if (parent is None or isinstance(parent, Statement)) else parent.statement)

//     def clone(self) -> HybridArgumentIdf:
//         ha = HybridArgumentIdf(self.name, self.t, self.arg_type, self.corresponding_priv_expression)
//         ha.serialized_loc = self.serialized_loc
//         return ha

//     def _set_serialized_loc(self, idf, base, start_offset):
//         assert self.serialized_loc.start_offset == -1
//         self.serialized_loc.arr = IdentifierExpr(idf)
//         self.serialized_loc.base = base
//         self.serialized_loc.start_offset = start_offset
//         self.serialized_loc.size = self.t.size_in_uints

//     def deserialize(self, source_idf: str, base: Optional[Expression], start_offset: int) -> AssignmentStatement:
//         self._set_serialized_loc(source_idf, base, start_offset)

//         src = IdentifierExpr(source_idf).as_type(Array(AnnotatedTypeName.uint_all()))
//         if isinstance(self.t, Array):
//             return SliceExpr(self.get_loc_expr(), None, 0, self.t.size_in_uints).assign(self.serialized_loc)
//         elif base is not None:
//             return self.get_loc_expr().assign(src.index(base.binop('+', NumberLiteralExpr(start_offset))).explicitly_converted(self.t))
//         else:
//             return self.get_loc_expr().assign(src.index(start_offset).explicitly_converted(self.t))

//     def serialize(self, target_idf: str, base: Optional[Expression], start_offset: int) -> AssignmentStatement:
//         self._set_serialized_loc(target_idf, base, start_offset)

//         tgt = IdentifierExpr(target_idf).as_type(Array(AnnotatedTypeName.uint_all()))
//         if isinstance(self.t, Array):
//             return self.serialized_loc.assign(SliceExpr(self.get_loc_expr(), None, 0, self.t.size_in_uints))
//         else:
//             expr = self.get_loc_expr()
//             if self.t.is_signed_numeric:
//                 // Cast to same size uint to prevent sign extension
//                 expr = expr.explicitly_converted(UintTypeName(f'uint{self.t.elem_bitwidth}'))
//             elif self.t.is_numeric and self.t.elem_bitwidth == 256:
//                 expr = expr.binop('%', IdentifierExpr(cfg.field_prime_var_name)).as_type(self.t)
//             else:
//                 expr = expr.explicitly_converted(TypeName.uint_type())

//             if base is not None:
//                 return tgt.clone().index(base.binop('+', NumberLiteralExpr(start_offset))).assign(expr)
//             else:
//                 return tgt.clone().index(start_offset).assign(expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EncryptionExpression {
    pub expr: Expression,
    pub privacy: PrivacyLabelExpr,
    pub homomorphism: Option<String>,
    pub annotated_type: Option<AnnotatedTypeName>,
}
// class EncryptionExpression(ReclassifyExpr):
//     def __init__(self, expr: Expression, privacy: PrivacyLabelExpr, homomorphism: Homomorphism):
//         if isinstance(privacy, Identifier):
//             privacy = IdentifierExpr(privacy)
//         super().__init__(expr, privacy, homomorphism)
//         self.annotated_type = AnnotatedTypeName.cipher_type(expr.annotated_type, homomorphism)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Statement {
    CircuitDirectiveStatement(Box<CircuitDirectiveStatement>),
    IfStatement(Box<IfStatement>),
    WhileStatement(Box<WhileStatement>),
    DoWhileStatement(Box<DoWhileStatement>),
    ForStatement(Box<ForStatement>),
    BreakStatement(Box<BreakStatement>),
    ContinueStatement(Box<ContinueStatement>),
    ReturnStatement(Box<ReturnStatement>),
    SimpleStatement(Box<SimpleStatement>),
    StatementList(Box<StatementList>),
    // Block(Box<Block>),
    // ExpressionStatement(ExpressionStatement),
    // RequireStatement(RequireStatement),
    // AssignmentStatement(AssignmentStatement),
    // CircuitInputStatement(CircuitInputStatement),
    #[default]
    None,
}
// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct StatementBase {
//     before_analysis: Option<PartitionState<PrivacyLabelExpr>>,
//     after_analysis: Option<PartitionState<PrivacyLabelExpr>>,
//     function: Option<ConstructorOrFunctionDefinition>,
//     pre_statements: Vec<CircuitInputStatement>,
// }
// class Statement(AST):

//     def __init__(self):
//         super().__init__()
//         // set by alias analysis
//         self.before_analysis: Optional[PartitionState[PrivacyLabelExpr]] = None
//         self.after_analysis: Optional[PartitionState[PrivacyLabelExpr]] = None
//         // set by parent setter
//         self.function: Optional[ConstructorOrFunctionDefinition] = None

//         // set by circuit helper
//         self.pre_statements = []

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CircuitDirectiveStatement {
    CircuitComputationStatement(Box<CircuitComputationStatement>),
    EnterPrivateKeyStatement(Box<EnterPrivateKeyStatement>),
    #[default]
    None,
}
// class CircuitDirectiveStatement(Statement):
//     """Invisible statement with instructions for offchain simulator"""
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitComputationStatement {
    pub idf: HybridArgumentIdf,
}
// class CircuitComputationStatement(CircuitDirectiveStatement):
//     def __init__(self, var: HybridArgumentIdf):
//         super().__init__()
//         self.idf = var.clone()

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnterPrivateKeyStatement {
    pub crypto_params: CryptoParams,
}
// class EnterPrivateKeyStatement(CircuitDirectiveStatement):
//     def __init__(self, crypto_params: CryptoParams):
//         super().__init__()
//         self.crypto_params = crypto_params

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}
impl IfStatement {
    pub fn new(condition: Expression, then_branch: Block, else_branch: Option<Block>) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl ASTChildren for IfStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Box::new(self.condition.clone())));
        cb.add_child(AST::Statement(Statement::StatementList(Box::new(
            StatementList::Block(Box::new(self.then_branch.clone())),
        ))));
        cb.add_child(AST::Statement(Statement::StatementList(Box::new(
            StatementList::Block(Box::new(self.then_branch.clone())),
        ))));
    }
}
// class IfStatement(Statement):

//     def __init__(self, condition: Expression, then_branch: Block, else_branch: Optional[Block]):
//         super().__init__()
//         self.condition = condition
//         self.then_branch = then_branch
//         self.else_branch = else_branch

//     def process_children(self, f: Callable[[T], T]):
//         self.condition = f(self.condition)
//         self.then_branch = f(self.then_branch)
//         self.else_branch = f(self.else_branch)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
}
impl WhileStatement {
    pub fn new(condition: Expression, body: Block) -> Self {
        Self { condition, body }
    }
}
impl ASTChildren for WhileStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Box::new(self.condition.clone())));
        cb.add_child(AST::Statement(Statement::StatementList(Box::new(
            StatementList::Block(Box::new(self.body.clone())),
        ))));
    }
}
// class WhileStatement(Statement):
//     def __init__(self, condition: Expression, body: Block):
//         super().__init__()
//         self.condition = condition
//         self.body = body

//     def process_children(self, f: Callable[[T], T]):
//         self.condition = f(self.condition)
//         self.body = f(self.body)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DoWhileStatement {
    pub body: Block,
    pub condition: Expression,
}
impl DoWhileStatement {
    pub fn new(body: Block, condition: Expression) -> Self {
        Self { body, condition }
    }
}
impl ASTChildren for DoWhileStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Statement(Statement::StatementList(Box::new(
            StatementList::Block(Box::new(self.body.clone())),
        ))));
        cb.add_child(AST::Expression(Box::new(self.condition.clone())));
    }
}
// class DoWhileStatement(Statement):
//     def __init__(self, body: Block, condition: Expression):
//         super().__init__()
//         self.body = body
//         self.condition = condition

//     def process_children(self, f: Callable[[T], T]):
//         self.body = f(self.body)
//         self.condition = f(self.condition)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ForStatement {
    pub init: Option<SimpleStatement>,
    pub condition: Expression,
    pub update: Option<SimpleStatement>,
    pub body: Block,
}
impl ForStatement {
    pub fn new(
        init: Option<SimpleStatement>,
        condition: Expression,
        update: Option<SimpleStatement>,
        body: Block,
    ) -> Self {
        Self {
            init,
            condition,
            update,
            body,
        }
    }
}
impl ASTChildren for ForStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(init) = &self.init {
            cb.add_child(AST::Statement(Statement::SimpleStatement(Box::new(
                init.clone(),
            ))));
        }

        cb.add_child(AST::Expression(Box::new(self.condition.clone())));
        if let Some(update) = &self.update {
            cb.add_child(AST::Statement(Statement::SimpleStatement(Box::new(
                update.clone(),
            ))));
        }
        cb.add_child(AST::Statement(Statement::StatementList(Box::new(
            StatementList::Block(Box::new(self.body.clone())),
        ))));
    }
}
// class ForStatement(Statement):
//     def __init__(self, init: Optional[SimpleStatement], condition: Expression, update: Optional[SimpleStatement], body: Block):
//         super().__init__()
//         self.init = init
//         self.condition = condition
//         self.update = update
//         self.body = body

//     def process_children(self, f: Callable[[T], T]):
//         self.init = f(self.init)
//         self.condition = f(self.condition)
//         self.update = f(self.update)
//         self.body = f(self.body)

//     @property
//     def statements(self) -> List[Statement]:
//         return [self.init, self.condition, self.body, self.update]

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BreakStatement {
    before_analysis: Option<PartitionState<PrivacyLabelExpr>>,
    after_analysis: Option<PartitionState<PrivacyLabelExpr>>,
    function: Option<ConstructorOrFunctionDefinition>,
    pre_statements: Vec<CircuitInputStatement>,
}
// class BreakStatement(Statement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ContinueStatement {
    before_analysis: Option<PartitionState<PrivacyLabelExpr>>,
    after_analysis: Option<PartitionState<PrivacyLabelExpr>>,
    function: Option<ConstructorOrFunctionDefinition>,
    pre_statements: Vec<CircuitInputStatement>,
}
// class ContinueStatement(Statement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReturnStatement {
    pub expr: Expression,
}
impl ASTChildren for ReturnStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Box::new(self.expr.clone())));
    }
}
// class ReturnStatement(Statement):

//     def __init__(self, expr: Expression):
//         super().__init__()
//         self.expr = expr

//     def process_children(self, f: Callable[[T], T]):
//         self.expr = f(self.expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleStatement {
    ExpressionStatement(Box<ExpressionStatement>),
    RequireStatement(Box<RequireStatement>),
    AssignmentStatement(Box<AssignmentStatementKind>),
    VariableDeclarationStatement(VariableDeclarationStatement),
    #[default]
    None,
}
// class SimpleStatement(Statement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStatement {
    pub expr: Expression,
}
impl ExpressionStatement {
    pub fn new(expr: Expression) -> Self {
        Self { expr }
    }
}
impl ASTChildren for ExpressionStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::Expression(Box::new(self.expr.clone())));
    }
}
// class ExpressionStatement(SimpleStatement):

//     def __init__(self, expr: Expression):
//         super().__init__()
//         self.expr = expr

//     def process_children(self, f: Callable[[T], T]):
//         self.expr = f(self.expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RequireStatement {
    pub condition: Expression,
    pub unmodified_code: String,
}
impl RequireStatement {
    pub fn new(condition: Expression, unmodified_code: Option<String>) -> Self {
        Self {
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
        cb.add_child(AST::Expression(Box::new(self.condition.clone())));
    }
}
// class RequireStatement(SimpleStatement):

//     def __init__(self, condition: Expression, unmodified_code: Optional[str] = None):
//         super().__init__()
//         self.condition = condition
//         self.unmodified_code = self.code() if unmodified_code is None else unmodified_code

//     def process_children(self, f: Callable[[T], T]):
//         self.condition = f(self.condition)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AssignmentStatementKind {
    AssignmentStatement(AssignmentStatement),
    CircuitInputStatement(CircuitInputStatement),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AssignmentStatement {
    pub lhs: AssignmentStatementUnion,
    pub rhs: Expression,
    pub op: String,
}
impl AssignmentStatement {
    pub fn new(lhs: AssignmentStatementUnion, rhs: Expression, op: Option<String>) -> Self {
        Self {
            lhs,
            rhs,
            op: if let Some(op) = op { op } else { String::new() },
        }
    }
}
impl ASTChildren for AssignmentStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        match &self.lhs {
            AssignmentStatementUnion::TupleExpr(te) => {
                cb.add_child(AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                    Box::new(TupleOrLocationExpr::TupleExpr(Box::new(te.clone()))),
                ))))
            }
            AssignmentStatementUnion::LocationExpr(le) => {
                cb.add_child(AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                    Box::new(TupleOrLocationExpr::LocationExpr(Box::new(le.clone()))),
                ))))
            }
            _ => {}
        };
        cb.add_child(AST::Expression(Box::new(self.rhs.clone())));
    }
}
// class AssignmentStatement(SimpleStatement):

//     def __init__(self, lhs: Union[TupleExpr, LocationExpr], rhs: Expression, op: Optional[str] = None):
//         super().__init__()
//         self.lhs = lhs
//         self.rhs = rhs
//         self.op = '' if op is None else op

//     def process_children(self, f: Callable[[T], T]):
//         self.lhs = f(self.lhs)
//         self.rhs = f(self.rhs)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AssignmentStatementUnion {
    TupleExpr(TupleExpr),
    LocationExpr(LocationExpr),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircuitInputStatement {
    pub lhs: AssignmentStatementUnion,
    pub rhs: Expression,
    pub op: String,
}
impl CircuitInputStatement {
    pub fn new(lhs: AssignmentStatementUnion, rhs: Expression, op: Option<String>) -> Self {
        Self {
            lhs,
            rhs,
            op: if let Some(op) = op { op } else { String::new() },
        }
    }
}
impl ASTChildren for CircuitInputStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        match &self.lhs {
            AssignmentStatementUnion::TupleExpr(te) => {
                cb.add_child(AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                    Box::new(TupleOrLocationExpr::TupleExpr(Box::new(te.clone()))),
                ))))
            }
            AssignmentStatementUnion::LocationExpr(le) => {
                cb.add_child(AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                    Box::new(TupleOrLocationExpr::LocationExpr(Box::new(le.clone()))),
                ))))
            }
            _ => {}
        };
        cb.add_child(AST::Expression(Box::new(self.rhs.clone())));
    }
}
// class CircuitInputStatement(AssignmentStatement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum StatementList {
    Block(Box<Block>),
    IndentBlock(Box<IndentBlock>),
    #[default]
    None,
}
// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct StatementList {
//     pub statements: Vec<Statement>,
//     pub excluded_from_simulation: bool,
// }
// impl ASTChildren for StatementList {
//     fn process_children(&mut self, cb: &mut ChildListBuilder) {
//         self.statements.iter().for_each(|statement| {
//             cb.add_child(AST::Statement(statement.clone()));
//         });
//     }
// }
// class StatementList(Statement):
//     def __init__(self, statements: List[Statement], excluded_from_simulation: bool = False):
//         super().__init__()
//         self.statements = statements
//         self.excluded_from_simulation = excluded_from_simulation

//         // Special case, if processing a statement returns a list of statements,
//         // all statements will be integrated into this block

//     def process_children(self, f: Callable[[T], T]):
//         new_stmts = []
//         for idx, stmt in enumerate(self.statements):
//             new_stmt = f(stmt)
//             if new_stmt is not None:
//                 if isinstance(new_stmt, List):
//                     new_stmts += new_stmt
//                 else:
//                     new_stmts.append(new_stmt)
//         self.statements = new_stmts

//     def __getitem__(self, key: int) -> Statement:
//         return self.statements[key]

//     def __contains__(self, stmt: Statement):
//         if stmt in self.statements:
//             return True
//         for s in self.statements:
//             if isinstance(s, StatementList) and stmt in s:
//                 return True
//         return False

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub excluded_from_simulation: bool,
    pub was_single_statement: bool,
}
impl Block {
    pub fn new(statements: Vec<Statement>, was_single_statement: bool) -> Self {
        Self {
            statements,
            excluded_from_simulation: false,
            was_single_statement,
        }
    }
}
// class Block(StatementList):
//     def __init__(self, statements: List[Statement], was_single_statement=False):
//         super().__init__(statements)
//         self.was_single_statement = was_single_statement

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IndentBlock {
    pub statements: Vec<Statement>,
}
// class IndentBlock(StatementList):
//     def __init__(self, statements: List[Statement]):
//         super().__init__(statements)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
    Mapping(Box<Mapping>),
    Array(Array),
    TupleType(TupleType),
    FunctionTypeName(FunctionTypeName),
    // NumberLiteralType(NumberLiteralType),
    // BooleanLiteralType(BooleanLiteralType),
    // String(String),
    #[default]
    None,
}
impl TypeName {
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
        TypeName::Array(Array::Key(Box::new(Key::new(crypto_params))))
    }

    pub fn proof_type() -> Self {
        TypeName::Array(Array::Proof(Proof::new()))
    }

    pub fn dyn_uint_array() -> Self {
        TypeName::Array(Array::ArrayBase(ArrayBase::new(
            AnnotatedTypeName::uint_all(),
            ExprType::None,
        )))
    }
    pub fn is_cipher(&self) -> bool {
        // if let TypeName::Array(Array::CipherText(_)) = self.type_name {
        //     true
        // } else {
        false
        // }
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
}
// class TypeName(AST):
//     __metaclass__ = abc.ABCMeta

//     @staticmethod
//     def bool_type():
//         return BoolTypeName()

//     @staticmethod
//     def uint_type():
//         return UintTypeName()

//     @staticmethod
//     def number_type():
//         return NumberTypeName.any()

//     @staticmethod
//     def address_type():
//         return AddressTypeName()

//     @staticmethod
//     def address_payable_type():
//         return AddressPayableTypeName()

//     @staticmethod
//     def cipher_type(plain_type: AnnotatedTypeName, hom: Homomorphism):
//         crypto_params = cfg.get_crypto_params(hom)
//         type = plain_type.clone()
//         type.homomorphism = hom  // Just for display purposes
//         return CipherText(type, crypto_params)

//     @staticmethod
//     def rnd_type(crypto_params: CryptoParams):
//         return Randomness(crypto_params)

//     @staticmethod
//     def key_type(crypto_params: CryptoParams):
//         return Key(crypto_params)

//     @staticmethod
//     def proof_type():
//         return Proof()

//     @staticmethod
//     def dyn_uint_array():
//         return Array(AnnotatedTypeName.uint_all())

//     @property
//     def size_in_uints(self):
//         """How many uints this type occupies when serialized."""
//         return 1

//     @property
//     def elem_bitwidth(self) -> int:
//         // Bitwidth, only defined for primitive types
//         raise NotImplementedError()

//     @property
//     def is_literal(self) -> bool:
//         return isinstance(self, (NumberLiteralType, BooleanLiteralType, EnumValueTypeName))

//     def is_address(self) -> bool:
//         return isinstance(self, (AddressTypeName, AddressPayableTypeName))

//     def is_primitive_type(self) -> bool:
//         return isinstance(self, (ElementaryTypeName, EnumTypeName, EnumValueTypeName, AddressTypeName, AddressPayableTypeName))

//     def is_cipher(self) -> bool:
//         return isinstance(self, CipherText)

//     def is_key(self) -> bool:
//         return isinstance(self, Key)

//     def is_randomness(self) -> bool:
//         return isinstance(self, Randomness)

//     @property
//     def is_numeric(self) -> bool:
//         return isinstance(self, NumberTypeName)

//     @property
//     def is_boolean(self) -> bool:
//         return isinstance(self, (BooleanLiteralType, BoolTypeName))

//     @property
//     def is_signed_numeric(self) -> bool:
//         return self.is_numeric and self.signed

//     def can_be_private(self) -> bool:
//         return self.is_primitive_type() and not (self.is_signed_numeric and self.elem_bitwidth == 256)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         assert isinstance(expected, TypeName)
//         return expected == self

//     def compatible_with(self, other_type: TypeName) -> bool:
//         assert isinstance(other_type, TypeName)
//         return self.implicitly_convertible_to(other_type) or other_type.implicitly_convertible_to(self)

//     def combined_type(self, other_type: TypeName, convert_literals: bool):
//         if other_type.implicitly_convertible_to(self):
//             return self
//         elif self.implicitly_convertible_to(other_type):
//             return other_type
//         return None

//     def annotate(self, privacy_annotation):
//         return AnnotatedTypeName(self, privacy_annotation)

//     def clone(self) -> TypeName:
//         raise NotImplementedError()

//     def __eq__(self, other):
//         raise NotImplementedError()

// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct ElementaryTypeName {
//     pub name: String,
// }
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum ElementaryTypeName {
    NumberTypeName(NumberTypeName),
    BoolTypeName(BoolTypeName),
    BooleanLiteralType(BooleanLiteralType),
    #[default]
    None,
}
// class ElementaryTypeName(TypeName):

//     def __init__(self, name: str):
//         super().__init__()
//         self.name = name

//     def clone(self) -> ElementaryTypeName:
//         return ElementaryTypeName(self.name)

//     def __eq__(self, other):
//         return isinstance(other, ElementaryTypeName) and self.name == other.name

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BoolTypeName {
    name: String,
}
impl BoolTypeName {
    pub fn new() -> Self {
        Self {
            name: String::from("bool"),
        }
    }
}
// class BoolTypeName(ElementaryTypeName):
//     def __init__(self, name='bool'):
//         super().__init__(name)

//     def clone(self) -> BoolTypeName:
//         return BoolTypeName()

//     @property
//     def elem_bitwidth(self):
//         return 1

//     def __eq__(self, other):
//         return isinstance(other, BoolTypeName)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BooleanLiteralType {
    name: String,
}
impl BooleanLiteralType {
    pub fn new(name: bool) -> Self {
        let mut name = name.to_string();
        name.make_ascii_lowercase();
        Self { name }
    }
}
// class BooleanLiteralType(ElementaryTypeName):
//     def __init__(self, name: bool):
//         super().__init__(str(name).lower())

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         return super().implicitly_convertible_to(expected) or isinstance(expected, BoolTypeName)

//     def combined_type(self, other_type: TypeName, convert_literals: bool):
//         if isinstance(other_type, BooleanLiteralType):
//             return TypeName.bool_type() if convert_literals else 'lit'
//         else:
//             return super().combined_type(other_type, convert_literals)

//     @property
//     def value(self):
//         return self.name == 'true'

//     @property
//     def elem_bitwidth(self):
//         return 1

//     def to_abstract_type(self):
//         return TypeName.bool_type()

//     def clone(self) -> BooleanLiteralType:
//         return BooleanLiteralType(self.value)

//     def __eq__(self, other):
//         return isinstance(other, BooleanLiteralType)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum NumberTypeName {
    NumberLiteralType(NumberLiteralType),
    IntTypeName(IntTypeName),
    UintTypeName(UintTypeName),
    AnyNumberTypeName(AnyNumberTypeName),
    #[default]
    None,
}
impl NumberTypeName {
    pub fn any() -> Self {
        NumberTypeName::AnyNumberTypeName(AnyNumberTypeName::new(
            String::new(),
            String::new(),
            true,
            Some(256),
        ))
    }
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnyNumberTypeName {
    name: String,
    prefix: String,
    signed: bool,
    bitwidth: Option<i32>,
    _size_in_bits: i32,
}
impl AnyNumberTypeName {
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
            name,
            prefix,
            signed,
            bitwidth,
            _size_in_bits,
        }
    }
}
// class NumberTypeName(ElementaryTypeName):
//     def __init__(self, name: str, prefix: str, signed: bool, bitwidth=None):
//         assert name.startswith(prefix)
//         prefix_len = len(prefix)
//         super().__init__(name)
//         if bitwidth is None:
//             self._size_in_bits = int(name[prefix_len:]) if len(name) > prefix_len else 0
//         else:
//             self._size_in_bits = bitwidth
//         self.signed = signed

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         return super().implicitly_convertible_to(expected) or type(expected) == NumberTypeName

//     @staticmethod
//     def any():
//         return NumberTypeName('', '', True, 256)

//     @property
//     def elem_bitwidth(self):
//         return 256 if self._size_in_bits == 0 else self._size_in_bits

//     def can_represent(self, value: int):
//         """Return true if value can be represented by this type"""
//         lo = - (1 << self.elem_bitwidth - 1) if self.signed else 0
//         hi = (1 << self.elem_bitwidth - 1) if self.signed else (1 << self.elem_bitwidth)
//         return lo <= value < hi

//     def __eq__(self, other):
//         return isinstance(other, NumberTypeName) and self.name == other.name
pub enum NumberLiteralTypeKind {
    String(String),
    I32(i32),
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NumberLiteralType {
    name: String,
    prefix: String,
    signed: bool,
    bitwidth: Option<i32>,
}
impl NumberLiteralType {
    pub fn new(name: NumberLiteralTypeKind) -> Self {
        let name = match name {
            NumberLiteralTypeKind::String(v) => v.parse::<i32>().unwrap(),
            NumberLiteralTypeKind::I32(v) => v,
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
            name,
            prefix,
            signed,
            bitwidth: Some(bitwidth),
        }
    }
}
// class NumberLiteralType(NumberTypeName):
//     def __init__(self, name: Union[str, int]):
//         name = int(name) if isinstance(name, str) else name
//         blen = name.bit_length()
//         if name < 0:
//             signed = True
//             bitwidth = blen + 1 if name != -(1 << (blen-1)) else blen
//         else:
//             signed = False
//             bitwidth = blen
//         bitwidth = max(int(math.ceil(bitwidth / 8.0)) * 8, 8)
//         assert 8 <= bitwidth <= 256 and bitwidth % 8 == 0

//         name = str(name)
//         super().__init__(name, name, signed, bitwidth)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         if expected.is_numeric and not expected.is_literal:
//             // Allow implicit conversion only if it fits
//             return expected.can_represent(self.value)
//         elif expected.is_address() and self.elem_bitwidth == 160 and not self.signed:
//             // Address literal case (fake solidity check will catch the cases where this is too permissive)
//             return True
//         return super().implicitly_convertible_to(expected)

//     def combined_type(self, other_type: TypeName, convert_literals: bool):
//         if isinstance(other_type, NumberLiteralType):
//             return self.to_abstract_type().combined_type(other_type.to_abstract_type(), convert_literals) if convert_literals else 'lit'
//         else:
//             return super().combined_type(other_type, convert_literals)

//     def to_abstract_type(self):
//         if self.value < 0:
//             return IntTypeName(f'int{self.elem_bitwidth}')
//         else:
//             return UintTypeName(f'uint{self.elem_bitwidth}')

//     @property
//     def value(self):
//         return int(self.name)

//     def clone(self) -> NumberLiteralType:
//         return NumberLiteralType(self.value)

//     def __eq__(self, other):
//         return isinstance(other, NumberLiteralType)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IntTypeName {
    name: String,
    prefix: String,
    signed: bool,
    bitwidth: Option<i32>,
}
impl IntTypeName {
    pub fn new(name: String) -> Self {
        Self {
            name,
            prefix: String::from("int"),
            signed: true,
            bitwidth: None,
        }
    }
}
// class IntTypeName(NumberTypeName):
//     def __init__(self, name: str = 'int'):
//         super().__init__(name, 'int', True)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         // Implicitly convert smaller int types to larger int types
//         return super().implicitly_convertible_to(expected) or (
//                 isinstance(expected, IntTypeName) and expected.elem_bitwidth >= self.elem_bitwidth)

//     def clone(self) -> IntTypeName:
//         return IntTypeName(self.name)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UintTypeName {
    name: String,
    prefix: String,
    signed: bool,
    bitwidth: Option<i32>,
}
impl UintTypeName {
    pub fn new(name: String) -> Self {
        Self {
            name,
            prefix: String::from("uint"),
            signed: false,
            bitwidth: None,
        }
    }
}
// class UintTypeName(NumberTypeName):
//     def __init__(self, name: str = 'uint'):
//         super().__init__(name, 'uint', False)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         // Implicitly convert smaller uint types to larger uint types
//         return super().implicitly_convertible_to(expected) or (
//                 isinstance(expected, UintTypeName) and expected.elem_bitwidth >= self.elem_bitwidth)

//     def clone(self) -> UintTypeName:
//         return UintTypeName(self.name)

// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct UserDefinedTypeName {
//     pub names: Vec<Identifier>,
//     pub target: Option<NamespaceDefinition>,
// }
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum UserDefinedTypeName {
    EnumTypeName(EnumTypeName),
    EnumValueTypeName(EnumValueTypeName),
    StructTypeName(Box<StructTypeName>),
    ContractTypeName(ContractTypeName),
    AddressTypeName(AddressTypeName),
    AddressPayableTypeName(AddressPayableTypeName),
    #[default]
    None,
}
// class UserDefinedTypeName(TypeName):
//     def __init__(self, names: List[Identifier], target: Optional[NamespaceDefinition] = None):
//         super().__init__()
//         self.names = names
//         self.target = target

//     def clone(self) -> UserDefinedTypeName:
//         return UserDefinedTypeName(self.names.copy(), self.target)

//     def __eq__(self, other):
//         return isinstance(other, UserDefinedTypeName) and all(e[0].name == e[1].name for e in zip(self.target.qualified_name, other.target.qualified_name))

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
}
// class EnumTypeName(UserDefinedTypeName):
//     def clone(self) -> EnumTypeName:
//         return EnumTypeName(self.names.copy(), self.target)

//     @property
//     def elem_bitwidth(self):
//         return 256

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumValueTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
}
// class EnumValueTypeName(UserDefinedTypeName):
//     @property
//     def elem_bitwidth(self):
//         return 256

//     def clone(self) -> EnumValueTypeName:
//         return EnumValueTypeName(self.names.copy(), self.target)

//     def to_abstract_type(self):
//         return EnumTypeName(self.names[:-1], self.target.parent)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         return super().implicitly_convertible_to(expected) or (isinstance(expected, EnumTypeName) and expected.names == self.names[:-1])

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StructTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
}
// class StructTypeName(UserDefinedTypeName):
//     def clone(self) -> StructTypeName:
//         return StructTypeName(self.names.copy(), self.target)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ContractTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
}
// class ContractTypeName(UserDefinedTypeName):
//     def clone(self) -> ContractTypeName:
//         return ContractTypeName(self.names.copy(), self.target)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AddressTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
}
impl AddressTypeName {
    pub fn new() -> Self {
        Self {
            names: vec![Identifier::new(String::from("<address>"))],
            target: None,
        }
    }
}
// class AddressTypeName(UserDefinedTypeName):
//     def __init__(self):
//         super().__init__([Identifier('<address>')], None)

//     @property
//     def elem_bitwidth(self):
//         return 160

//     def clone(self) -> UserDefinedTypeName:
//         return AddressTypeName()

//     def __eq__(self, other):
//         return isinstance(other, AddressTypeName)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AddressPayableTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
}
impl AddressPayableTypeName {
    pub fn new() -> Self {
        Self {
            names: vec![Identifier::new(String::from("<address_payable>"))],
            target: None,
        }
    }
}
// class AddressPayableTypeName(UserDefinedTypeName):
//     def __init__(self):
//         super().__init__([Identifier('<address_payable>')], None)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         // Implicit conversions
//         return super().implicitly_convertible_to(expected) or expected == TypeName.address_type()

//     @property
//     def elem_bitwidth(self):
//         return 160

//     def clone(self) -> UserDefinedTypeName:
//         return AddressPayableTypeName()

//     def __eq__(self, other):
//         return isinstance(other, AddressPayableTypeName)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum KeyLabelType {
    String(String),
    Identifier(Option<Identifier>),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mapping {
    pub idf: Identifier,
    pub key_type: ElementaryTypeName,
    pub key_label: KeyLabelType,
    pub value_type: Box<AnnotatedTypeName>,
    pub instantiated_key: Option<Expression>,
}
impl ASTChildren for Mapping {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::TypeName(TypeName::ElementaryTypeName(
            self.key_type.clone(),
        )));
        if let KeyLabelType::Identifier(Some(idf)) = &self.key_label {
            cb.add_child(AST::Identifier(IdentifierKind::Identifier(
                self.idf.clone(),
            )));
        }
        cb.add_child(AST::AnnotatedTypeName(*self.value_type.clone()));
    }
}
// class Mapping(TypeName):

//     def __init__(self, key_type: ElementaryTypeName, key_label: Optional[Identifier], value_type: AnnotatedTypeName):
//         super().__init__()
//         self.key_type = key_type
//         self.key_label: Union[str, Optional[Identifier]] = key_label
//         self.value_type = value_type
//         // set by type checker: instantiation of the key by IndexExpr
//         self.instantiated_key: Optional[Expression] = None

//     def process_children(self, f: Callable[[T], T]):
//         self.key_type = f(self.key_type)
//         if isinstance(self.key_label, Identifier):
//             self.key_label = f(self.key_label)
//         self.value_type = f(self.value_type)

//     def clone(self) -> Mapping:
//         from zkay.zkay_ast.visitor.deep_copy import deep_copy
//         return deep_copy(self)

//     @property
//     def has_key_label(self):
//         return self.key_label is not None

//     def __eq__(self, other):
//         if isinstance(other, Mapping):
//             return self.key_type == other.key_type and self.value_type == other.value_type
//         else:
//             return False
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ExprType {
    I32(i32),
    Expression(Expression),
    #[default]
    None,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum Array {
    CipherText(CipherText),
    Randomness(Randomness),
    Key(Box<Key>),
    Proof(Proof),
    ArrayBase(ArrayBase),
    #[default]
    None,
}
impl ASTChildren for ArrayBase {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::AnnotatedTypeName(self.value_type.clone()));
        if let ExprType::Expression(expr) = &self.expr {
            cb.add_child(AST::Expression(Box::new(expr.clone())));
        }
    }
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ArrayBase {
    pub value_type: AnnotatedTypeName,
    pub expr: ExprType,
}
impl ArrayBase {
    pub fn new(value_type: AnnotatedTypeName, expr: ExprType) -> Self {
        Self {
            value_type,
            expr: if let ExprType::I32(expr) = expr {
                ExprType::Expression(Expression::LiteralExpr(Box::new(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(expr, false)),
                )))
            } else {
                expr
            },
        }
    }
}

// class Array(TypeName):

//     def __init__(self, value_type: AnnotatedTypeName, expr: Union[int, Expression] = None):
//         super().__init__()
//         self.value_type = value_type
//         self.expr = NumberLiteralExpr(expr) if isinstance(expr, int) else expr

//     def process_children(self, f: Callable[[T], T]):
//         self.value_type = f(self.value_type)
//         self.expr = f(self.expr)

//     def clone(self) -> Array:
//         return Array(self.value_type.clone(), self.expr)

//     @property
//     def size_in_uints(self):
//         if self.expr is None or not isinstance(self.expr, NumberLiteralExpr):
//             return -1
//         else:
//             return self.expr.value

//     @property
//     def elem_bitwidth(self):
//         return self.value_type.type_name.elem_bitwidth

//     def __eq__(self, other):
//         if not isinstance(other, Array):
//             return False
//         if self.value_type != other.value_type:
//             return False
//         if isinstance(self.expr, NumberLiteralExpr) and isinstance(other.expr, NumberLiteralExpr) and self.expr.value == other.expr.value:
//             return True
//         if self.expr is None and other.expr is None:
//             return True
//         return False

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CipherText {
    pub value_type: AnnotatedTypeName,
    pub expr: ExprType,
    pub plain_type: AnnotatedTypeName,
    pub crypto_params: CryptoParams,
}
impl CipherText {
    pub fn new(plain_type: AnnotatedTypeName, crypto_params: CryptoParams) -> Self {
        assert!(!plain_type.type_name.is_cipher());
        Self {
            value_type: AnnotatedTypeName::uint_all(),
            expr: ExprType::Expression(Expression::LiteralExpr(Box::new(
                LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                    crypto_params.cipher_len(),
                    false,
                )),
            ))),
            plain_type,
            crypto_params,
        }
    }
}
// class CipherText(Array):
//     def __init__(self, plain_type: AnnotatedTypeName, crypto_params: CryptoParams):
//         assert not plain_type.type_name.is_cipher()
//         super().__init__(AnnotatedTypeName.uint_all(), NumberLiteralExpr(crypto_params.cipher_len))
//         self.plain_type = plain_type
//         self.crypto_params = crypto_params

//     @property
//     def size_in_uints(self):
//         return self.crypto_params.cipher_payload_len

//     def clone(self) -> CipherText:
//         return CipherText(self.plain_type, self.crypto_params)

//     def __eq__(self, other):
//         return (isinstance(other, CipherText)
//                 and (self.plain_type is None or self.plain_type == other.plain_type)
//                 and self.crypto_params == other.crypto_params)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Randomness {
    pub value_type: AnnotatedTypeName,
    pub expr: ExprType,
    pub crypto_params: CryptoParams,
}
impl Randomness {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            value_type: AnnotatedTypeName::uint_all(),
            expr: if let Some(randomness_len) = crypto_params.randomness_len() {
                ExprType::Expression(Expression::LiteralExpr(Box::new(
                    LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(randomness_len, false)),
                )))
            } else {
                ExprType::None
            },
            crypto_params,
        }
    }
}
// class Randomness(Array):
//     def __init__(self, crypto_params: CryptoParams):
//         if crypto_params.randomness_len is None:
//             super().__init__(AnnotatedTypeName.uint_all(), None)
//         else:
//             super().__init__(AnnotatedTypeName.uint_all(), NumberLiteralExpr(crypto_params.randomness_len))
//         self.crypto_params = crypto_params

//     def clone(self) -> Randomness:
//         return Randomness(self.crypto_params)

//     def __eq__(self, other):
//         return isinstance(other, Randomness) and self.crypto_params == other.crypto_params

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Key {
    pub value_type: AnnotatedTypeName,
    pub expr: ExprType,
    pub crypto_params: CryptoParams,
}
impl Key {
    pub fn new(crypto_params: CryptoParams) -> Self {
        Self {
            value_type: AnnotatedTypeName::uint_all(),
            expr: ExprType::Expression(Expression::LiteralExpr(Box::new(
                LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                    crypto_params.key_len(),
                    false,
                )),
            ))),
            crypto_params,
        }
    }
}
// class Key(Array):
//     def __init__(self, crypto_params: CryptoParams):
//         super().__init__(AnnotatedTypeName.uint_all(), NumberLiteralExpr(crypto_params.key_len))
//         self.crypto_params = crypto_params

//     def clone(self) -> Key:
//         return Key(self.crypto_params)

//     def __eq__(self, other):
//         return isinstance(other, Key) and self.crypto_params == other.crypto_params

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Proof {
    pub value_type: AnnotatedTypeName,
    pub expr: ExprType,
}
impl Proof {
    pub fn new() -> Self {
        Self {
            value_type: AnnotatedTypeName::uint_all(),
            expr: ExprType::Expression(Expression::LiteralExpr(Box::new(
                LiteralExpr::NumberLiteralExpr(NumberLiteralExpr::new(
                    CFG.lock().unwrap().proof_len(),
                    false,
                )),
            ))),
        }
    }
}
// class Proof(Array):
//     def __init__(self):
//         super().__init__(AnnotatedTypeName.uint_all(), NumberLiteralExpr(cfg.proof_len))

//     def clone(self) -> Proof:
//         return Proof()

//     def __eq__(self, other):
//         return isinstance(other, Proof)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DummyAnnotation;
// class DummyAnnotation:
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TupleType {
    pub types: Vec<AnnotatedTypeName>,
}
impl TupleType {
    pub fn new(types: Vec<AnnotatedTypeName>) -> Self {
        Self { types }
    }
}
// class TupleType(TypeName):
//     """Does not appear in the syntax, but is necessary for type checking"""

//     @staticmethod
//     def ensure_tuple(t: AnnotatedTypeName):
//         if t is None:
//             return TupleType.empty()
//         elif isinstance(t.type_name, TupleType):
//             return t
//         else:
//             return TupleType([t])

//     def __init__(self, types: List[AnnotatedTypeName]):
//         super().__init__()
//         self.types = types

//     def __len__(self):
//         return len(self.types)

//     def __iter__(self):
//         """Make this class iterable, by iterating over its types."""
//         return self.types.__iter__()

//     def __getitem__(self, i: int):
//         return self.types[i]

//     def check_component_wise(self, other, f):
//         if isinstance(other, TupleType):
//             if len(self) != len(other):
//                 return False
//             else:
//                 for i in range(len(self)):
//                     if not f(self[i], other[i]):
//                         return False
//                 return True
//         else:
//             return False

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         return self.check_component_wise(expected, lambda x, y: x.type_name.implicitly_convertible_to(y.type_name))

//     def compatible_with(self, other_type: TypeName) -> bool:
//         return self.check_component_wise(other_type, lambda x, y: x.type_name.compatible_with(y.type_name))

//     def combined_type(self, other_type: TupleType, convert_literals: bool):
//         if not isinstance(other_type, TupleType) or len(self.types) != len(other_type.types):
//             return None
//         return TupleType([AnnotatedTypeName(e1.type_name.combined_type(e2.type_name, convert_literals), DummyAnnotation()) for e1, e2 in zip(self.types, other_type.types)])

//     def annotate(self, privacy_annotation):
//         if isinstance(privacy_annotation, Expression):
//             return AnnotatedTypeName(TupleType([t.type_name.annotate(privacy_annotation) for t in self.types]))
//         else:
//             assert len(self.types) == len(privacy_annotation)
//             return AnnotatedTypeName(TupleType([t.type_name.annotate(a) for t, a in zip(self.types, privacy_annotation)]))

//     def perfect_privacy_match(self, other):
//         def privacy_match(self: AnnotatedTypeName, other: AnnotatedTypeName):
//             return self.privacy_annotation == other.privacy_annotation

//         self.check_component_wise(other, privacy_match)

//     def clone(self) -> TupleType:
//         return TupleType(list(map(AnnotatedTypeName.clone, self.types)))

//     @staticmethod
//     def empty() -> TupleType:
//         return TupleType([])

//     def __eq__(self, other):
//         return self.check_component_wise(other, lambda x, y: x == y)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FunctionTypeName {
    parameters: Vec<Parameter>,
    modifiers: Vec<String>,
    return_parameters: Vec<Parameter>,
}
impl FunctionTypeName {
    pub fn new(
        parameters: Vec<Parameter>,
        modifiers: Vec<String>,
        return_parameters: Vec<Parameter>,
    ) -> Self {
        Self {
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
// class FunctionTypeName(TypeName):
//     def __init__(self, parameters: List[Parameter], modifiers: Vec<String>, return_parameters: List[Parameter]):
//         super().__init__()
//         self.parameters = parameters
//         self.modifiers = modifiers
//         self.return_parameters = return_parameters

//     def process_children(self, f: Callable[[T], T]):
//         self.parameters[:] = map(f, self.parameters)
//         self.return_parameters[:] = map(f, self.return_parameters)

//     def clone(self) -> FunctionTypeName:
//         // TODO deep copy if required
//         return FunctionTypeName(self.parameters, self.modifiers, self.return_parameters)

//     def __eq__(self, other):
//         return isinstance(other, FunctionTypeName) and self.parameters == other.parameters and \
//                self.modifiers == other.modifiers and self.return_parameters == other.return_parameters

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnnotatedTypeName {
    pub type_name: Box<TypeName>,
    pub had_privacy_annotation: bool,
    pub privacy_annotation: Option<Expression>,
    pub homomorphism: String,
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
            type_name: Box::new(type_name),
            had_privacy_annotation: privacy_annotation.as_ref().is_some(),
            privacy_annotation: if privacy_annotation.is_some() {
                privacy_annotation
            } else {
                Some(Expression::AllExpr(AllExpr::new()))
            },
            homomorphism,
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
        self.is_public()
    }
    pub fn uint_all() -> AnnotatedTypeName {
        AnnotatedTypeName::new(TypeName::uint_type(), None, String::from("NON_HOMOMORPHIC"))
    }
    pub fn is_cipher(&self) -> bool {
        if let TypeName::Array(Array::CipherText(_)) = *self.type_name {
            true
        } else {
            false
        }
    }
}
impl ASTChildren for AnnotatedTypeName {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::TypeName(*self.type_name.clone()));
        if let Some(privacy_annotation) = &self.privacy_annotation {
            cb.add_child(AST::Expression(Box::new(privacy_annotation.clone())));
        }
    }
}
impl ASTCode for AnnotatedTypeName {
    fn get_ast(&self) -> AST {
        AST::AnnotatedTypeName(self.clone())
    }
}
// class AnnotatedTypeName(AST):

//     def __init__(self, type_name: TypeName, privacy_annotation: Optional[Expression] = None,
//                  homomorphism: Homomorphism = Homomorphism.NON_HOMOMORPHIC):
//         super().__init__()
//         self.type_name = type_name
//         self.had_privacy_annotation = privacy_annotation is not None
//         if self.had_privacy_annotation:
//             self.privacy_annotation = privacy_annotation
//         else:
//             self.privacy_annotation = AllExpr()
//         self.homomorphism = homomorphism
//         if self.privacy_annotation == AllExpr() and homomorphism != Homomorphism.NON_HOMOMORPHIC:
//             raise ValueError(f'Public type name cannot be homomorphic (got {homomorphism.type_annotation})')

//     def process_children(self, f: Callable[[T], T]):
//         self.type_name = f(self.type_name)
//         self.privacy_annotation = f(self.privacy_annotation)

//     def clone(self) -> AnnotatedTypeName:
//         assert self.privacy_annotation is not None
//         at = AnnotatedTypeName(self.type_name.clone(), self.privacy_annotation.clone(), self.homomorphism)
//         at.had_privacy_annotation = self.had_privacy_annotation
//         return at

//     @property
//     def zkay_type(self) -> AnnotatedTypeName:
//         if isinstance(self.type_name, CipherText):
//             return self.type_name.plain_type
//         else:
//             return self

//     def __eq__(self, other):
//         if isinstance(other, AnnotatedTypeName):
//             return (self.type_name == other.type_name and
//                     self.privacy_annotation == other.privacy_annotation and
//                     self.homomorphism == other.homomorphism)
//         else:
//             return False

//     def combined_privacy(self, analysis: PartitionState[PrivacyLabelExpr], other: AnnotatedTypeName):
//         if isinstance(self.type_name, TupleType):
//             assert isinstance(other.type_name, TupleType) and len(self.type_name.types) == len(other.type_name.types)
//             return [e1.combined_privacy(analysis, e2) for e1, e2 in zip(self.type_name.types, other.type_name.types)]

//         if self.homomorphism != other.homomorphism and not self.is_public():
//             return None

//         p_expected = other.privacy_annotation.privacy_annotation_label()
//         p_actual = self.privacy_annotation.privacy_annotation_label()
//         if p_expected and p_actual:
//             if p_expected == p_actual or (analysis is not None and analysis.same_partition(p_expected, p_actual)):
//                 return self.privacy_annotation.clone()
//             elif self.privacy_annotation.is_all_expr():
//                 return other.privacy_annotation.clone()
//         else:
//             return None

//     def is_public(self):
//         return self.privacy_annotation.is_all_expr()

//     def is_private(self):
//         return not self.is_public()

//     def is_private_at_me(self, analysis: PartitionState[PrivacyLabelExpr]):
//         p = self.privacy_annotation
//         return p.is_me_expr() or (analysis is not None and analysis.same_partition(p.privacy_annotation_label(), MeExpr()))

//     def is_accessible(self, analysis: PartitionState[PrivacyLabelExpr]):
//         return self.is_public() or self.is_private_at_me(analysis)

//     def is_address(self) -> bool:
//         return isinstance(self.type_name, (AddressTypeName, AddressPayableTypeName))

//     def is_cipher(self) -> bool:
//         return isinstance(self.type_name, CipherText)

//     def with_homomorphism(self, hom: Homomorphism):
//         return AnnotatedTypeName(self.type_name, self.privacy_annotation, hom)

//     @staticmethod
//     def uint_all():
//         return AnnotatedTypeName(TypeName.uint_type())

//     @staticmethod
//     def bool_all():
//         return AnnotatedTypeName(TypeName.bool_type())

//     @staticmethod
//     def address_all():
//         return AnnotatedTypeName(TypeName.address_type())

//     @staticmethod
//     def cipher_type(plain_type: AnnotatedTypeName, hom: Homomorphism):
//         return AnnotatedTypeName(TypeName.cipher_type(plain_type, hom))

//     @staticmethod
//     def key_type(crypto_params: CryptoParams):
//         return AnnotatedTypeName(TypeName.key_type(crypto_params))

//     @staticmethod
//     def proof_type():
//         return AnnotatedTypeName(TypeName.proof_type())

//     @staticmethod
//     def all(type: TypeName):
//         return AnnotatedTypeName(type, Expression.all_expr())

//     @staticmethod
//     def me(type: TypeName):
//         return AnnotatedTypeName(type, Expression.me_expr())

//     @staticmethod
//     def array_all(value_type: AnnotatedTypeName, *length: int):
//         t = value_type
//         for l in length:
//             t = AnnotatedTypeName(Array(t, NumberLiteralExpr(l)))
//         return t

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum IdentifierDeclaration {
    #[default]
    None,
    VariableDeclaration(VariableDeclaration),
    Parameter(Parameter),
    StateVariableDeclaration(StateVariableDeclaration),
}
// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct IdentifierDeclaration {
//     keywords: Vec<String>,
//     annotated_type: Box<AnnotatedTypeName>,
//     idf: Identifier,
//     storage_location: Option<String>,
// }
// impl IdentifierDeclaration {
//     fn new(
//         keywords: Vec<String>,
//         annotated_type: Box<AnnotatedTypeName>,
//         idf: Identifier,
//         storage_location: Option<String>,
//     ) -> Self {
//         Self {
//             keywords,
//             annotated_type,
//             idf,
//             storage_location,
//         }
//     }
// }
// impl ASTChildren for IdentifierDeclaration {
//     fn process_children(&mut self, cb: &mut ChildListBuilder) {
//         cb.add_child(AST::AnnotatedTypeName(self.annotated_type.clone()));
//         cb.add_child(AST::Identifier(self.idf.clone()));
//     }
// }

// class IdentifierDeclaration(AST):

//     def __init__(self, keywords: Vec<String>, annotated_type: AnnotatedTypeName, idf: Identifier, storage_location: Optional[str] = None):
//         super().__init__()
//         self.keywords = keywords
//         self.annotated_type = annotated_type
//         self.idf = idf
//         self.storage_location = storage_location

//     @property
//     def is_final(self) -> bool:
//         return 'final' in self.keywords

//     @property
//     def is_constant(self) -> bool:
//         return 'constant' in self.keywords

//     def process_children(self, f: Callable[[T], T]):
//         self.annotated_type = f(self.annotated_type)
//         self.idf = f(self.idf)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclaration {
    keywords: Vec<String>,
    annotated_type: Box<AnnotatedTypeName>,
    idf: Identifier,
    storage_location: Option<String>,
}
impl VariableDeclaration {
    fn new(
        keywords: Vec<String>,
        annotated_type: Box<AnnotatedTypeName>,
        idf: Identifier,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            keywords,
            annotated_type,
            idf,
            storage_location,
        }
    }
}
// class VariableDeclaration(IdentifierDeclaration):

//     def __init__(self, keywords: Vec<String>, annotated_type: AnnotatedTypeName, idf: Identifier, storage_location: Optional[str] = None):
//         super().__init__(keywords, annotated_type, idf, storage_location)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclarationStatement {
    variable_declaration: VariableDeclaration,
    expr: Option<Expression>,
}
impl ASTChildren for VariableDeclarationStatement {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        cb.add_child(AST::IdentifierDeclaration(
            IdentifierDeclaration::VariableDeclaration(self.variable_declaration.clone()),
        ));
        if let Some(expr) = &self.expr {
            cb.add_child(AST::Expression(Box::new(expr.clone())));
        }
    }
}
impl VariableDeclarationStatement {
    pub fn new(variable_declaration: VariableDeclaration, expr: Option<Expression>) -> Self {
        Self {
            variable_declaration,
            expr,
        }
    }
}
// class VariableDeclarationStatement(SimpleStatement):

//     def __init__(self, variable_declaration: VariableDeclaration, expr: Optional[Expression] = None):
//         """

//         :param variable_declaration:
//         :param expr: can be None
//         """
//         super().__init__()
//         self.variable_declaration = variable_declaration
//         self.expr = expr

//     def process_children(self, f: Callable[[T], T]):
//         self.variable_declaration = f(self.variable_declaration)
//         self.expr = f(self.expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Parameter {
    keywords: Vec<String>,
    annotated_type: Box<AnnotatedTypeName>,
    idf: Identifier,
    storage_location: Option<String>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ParameterKind {
    Parameter(Parameter),
    String(String),
    #[default]
    None,
}
impl Parameter {
    pub fn new(
        keywords: Vec<String>,
        annotated_type: Box<AnnotatedTypeName>,
        idf: Identifier,
        storage_location: Option<String>,
    ) -> Self {
        Self {
            keywords,
            annotated_type,
            idf,
            storage_location,
        }
    }
}
// class Parameter(IdentifierDeclaration):

//     def __init__(
//             self,
//             keywords: Vec<String>,
//             annotated_type: AnnotatedTypeName,
//             idf: Identifier,
//             storage_location: Optional[str] = None):
//         super().__init__(keywords, annotated_type, idf, storage_location)

//     def copy(self) -> Parameter:
//         return Parameter(self.keywords, self.annotated_type.clone(), self.idf.clone() if self.idf else None, self.storage_location)

//     def with_changed_storage(self, match_storage: str, new_storage: str) -> Parameter:
//         if self.storage_location == match_storage:
//             self.storage_location = new_storage
//         return self
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum NamespaceDefinition {
    #[default]
    None,
    ConstructorOrFunctionDefinition(ConstructorOrFunctionDefinition),
    EnumDefinition(EnumDefinition),
    StructDefinition(StructDefinition),
    ContractDefinition(ContractDefinition),
}
// #[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub struct NamespaceDefinition {
//     idf: Identifier,
// }
// impl NamespaceDefinition {
//     pub fn new(idf: Identifier) -> Self {
//         Self { idf }
//     }
// }
// impl ASTChildren for NamespaceDefinition {
//     fn process_children(&mut self, cb: &mut ChildListBuilder) {
//         cb.add_child(AST::Identifier(self.idf.clone()));
//     }
// }
// class NamespaceDefinition(AST):
//     def __init__(self, idf: Identifier):
//         super().__init__()
//         self.idf = idf

//     def process_children(self, f: Callable[[T], T]):
//         oldidf = self.idf
//         self.idf = f(self.idf)
//         assert oldidf == self.idf // must be readonly

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ConstructorOrFunctionDefinition {
    name: String,
    idf: Identifier,
    parameters: Vec<Parameter>,
    modifiers: Vec<String>,
    return_parameters: Vec<Parameter>,
    body: Option<Block>,
    return_var_decls: Vec<VariableDeclaration>,
    parent: Option<ContractDefinition>,
    original_body: Option<Block>,
    annotated_type: Option<AnnotatedTypeName>,
    called_functions: BTreeSet<ConstructorOrFunctionDefinition>,
    is_recursive: bool,
    has_static_body: bool,
    can_be_private: bool,
    used_homomorphisms: Option<BTreeSet<Homomorphism>>,
    used_crypto_backends: Option<Vec<CryptoParams>>,
    requires_verification: bool,
    requires_verification_when_external: bool,
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
            idf.is_some() && idf.as_ref().unwrap().name != String::from("constructor")
                || return_parameters.is_none()
        );
        let idf = if let Some(idf) = idf {
            idf
        } else {
            Identifier {
                parent: None,
                name: String::from("constructor"),
            }
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
                    rp.annotated_type.clone(),
                    Identifier::new(format!("{}_{idx}", CFG.lock().unwrap().return_var_name())),
                    rp.storage_location.clone(),
                )
            })
            .collect();
        return_var_decls.iter_mut().for_each(|vd| {
            vd.idf.parent = Some(Box::new(AST::IdentifierDeclaration(
                IdentifierDeclaration::VariableDeclaration(vd.clone()),
            )));
        });
        Self {
            idf,
            parameters: parameters.as_ref().unwrap().clone(),
            modifiers: modifiers.as_ref().unwrap().clone(),
            return_parameters: return_parameters.clone(),
            body,
            return_var_decls,
            name: String::new(),
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
}
pub trait ConstructorOrFunctionDefinitionAttr {
    fn get_requires_verification_when_external(&self) -> bool;
    fn get_name(&self) -> String;
}
impl ConstructorOrFunctionDefinitionAttr for ConstructorOrFunctionDefinition {
    fn get_requires_verification_when_external(&self) -> bool {
        self.requires_verification_when_external
    }
    fn get_name(&self) -> String {
        self.name.clone()
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
            cb.add_child(AST::Statement(Statement::StatementList(Box::new(
                StatementList::Block(Box::new(body.clone())),
            ))));
        }
    }
}
// class ConstructorOrFunctionDefinition(NamespaceDefinition):

//     def __init__(self, idf: Optional[Identifier], parameters: List[Parameter], modifiers: Vec<String>, return_parameters: Optional[List[Parameter]], body: Block):
//         assert (idf is not None and idf.name != 'constructor') or not return_parameters
//         if idf is None:
//             idf = Identifier('constructor')
//         super().__init__(idf)
//         self.parameters = parameters
//         self.modifiers = modifiers
//         self.body = body
//         self.return_parameters = [] if return_parameters is None else return_parameters

//         self.return_var_decls: List[VariableDeclaration] = [
//             VariableDeclaration([], rp.annotated_type, Identifier(f'{cfg.return_var_name}_{idx}'), rp.storage_location)
//             for idx, rp in enumerate(self.return_parameters)
//         ]
//         for vd in self.return_var_decls:
//             vd.idf.parent = vd

//         // specify parent type
//         self.parent: Optional[ContractDefinition] = None
//         self.original_body: Optional[Block] = None

//         // Set function type
//         self.annotated_type = None
//         self._update_fct_type()

//         // Analysis properties
//         self.called_functions: OrderedDict[ConstructorOrFunctionDefinition, None] = OrderedDict()
//         self.is_recursive = False
//         self.has_static_body = True
//         self.can_be_private = True
//         self.used_homomorphisms: Optional[Set[Homomorphism]] = None
//         self.used_crypto_backends: Optional[List[CryptoParams]] = None

//         // True if this function contains private expressions
//         self.requires_verification = False

//         // True if this function is public and either requires verification or has private arguments
//         self.requires_verification_when_external = False

//     @property
//     def has_side_effects(self) -> bool:
//         return not ('pure' in self.modifiers or 'view' in self.modifiers)

//     @property
//     def can_be_external(self) -> bool:
//         return not ('private' in self.modifiers or 'internal' in self.modifiers)

//     @property
//     def is_external(self) -> bool:
//         return 'external' in self.modifiers

//     @property
//     def is_payable(self) -> bool:
//         return 'payable' in self.modifiers

//     @property
//     def name(self) -> str:
//         return self.idf.name

//     @property
//     def return_type(self) -> TupleType:
//         return TupleType([p.annotated_type for p in self.return_parameters])

//     @property
//     def parameter_types(self) -> TupleType:
//         return TupleType([p.annotated_type for p in self.parameters])

//     @property
//     def is_constructor(self) -> bool:
//         return self.idf.name == 'constructor'

//     @property
//     def is_function(self) -> bool:
//         return not self.is_constructor

//     def _update_fct_type(self):
//         self.annotated_type = AnnotatedTypeName(FunctionTypeName(self.parameters, self.modifiers, self.return_parameters))

//     def process_children(self, f: Callable[[T], T]):
//         super().process_children(f)
//         self.parameters[:] = map(f, self.parameters)
//         self.return_parameters[:] = map(f, self.return_parameters)
//         self.body = f(self.body)

//     def add_param(self, t: Union[TypeName, AnnotatedTypeName], idf: Union[str, Identifier], ref_storage_loc: str = 'memory'):
//         t = t if isinstance(t, AnnotatedTypeName) else AnnotatedTypeName(t)
//         idf = Identifier(idf) if isinstance(idf, str) else idf.clone()
//         storage_loc = '' if t.type_name.is_primitive_type() else ref_storage_loc
//         self.parameters.append(Parameter([], t, idf, storage_loc))
//         self._update_fct_type()
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StateVariableDeclaration {
    annotated_type: Box<AnnotatedTypeName>,
    keywords: Vec<String>,
    idf: Identifier,
    storage_location: Option<String>,
    expr: Option<Expression>,
}

impl StateVariableDeclaration {
    pub fn new(
        annotated_type: Box<AnnotatedTypeName>,
        keywords: Vec<String>,
        idf: Identifier,
        expr: Option<Expression>,
    ) -> Self {
        Self {
            annotated_type,
            keywords,
            idf,
            expr,
            storage_location: None,
        }
    }
    pub fn is_final(&self) -> bool {
        self.keywords.contains(&String::from("final"))
    }
    pub fn is_constant(&self) -> bool {
        self.keywords.contains(&String::from("constant"))
    }
}
impl ASTChildren for StateVariableDeclaration {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        // super().process_children(f)
        if let Some(expr) = &self.expr {
            cb.add_child(AST::Expression(Box::new(expr.clone())));
        }
    }
}
// class StateVariableDeclaration(IdentifierDeclaration):

//     def __init__(self, annotated_type: AnnotatedTypeName, keywords: Vec<String>, idf: Identifier, expr: Optional[Expression]):
//         super().__init__(keywords, annotated_type, idf)
//         self.expr = expr

//     def process_children(self, f: Callable[[T], T]):
//         super().process_children(f)
//         self.expr = f(self.expr)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumValue {
    idf: Option<Identifier>,
    annotated_type: Option<AnnotatedTypeName>,
}
impl EnumValue {
    pub fn new(idf: Option<Identifier>) -> Self {
        Self {
            idf,
            annotated_type: None,
        }
    }
}
impl ASTChildren for EnumValue {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        if let Some(idf) = &self.idf {
            cb.add_child(AST::Identifier(IdentifierKind::Identifier(idf.clone())));
        }
    }
}
// class EnumValue(AST):
//     def __init__(self, idf: Identifier):
//         super().__init__()
//         self.idf = idf
//         self.annotated_type: Optional[AnnotatedTypeName] = None

//     def process_children(self, f: Callable[[T], T]):
//         self.idf = f(self.idf)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumDefinition {
    idf: Option<Identifier>,
    values: Vec<EnumValue>,
    annotated_type: Option<AnnotatedTypeName>,
}
impl EnumDefinition {
    pub fn new(idf: Option<Identifier>, values: Vec<EnumValue>) -> Self {
        Self {
            idf,
            values,
            annotated_type: None,
        }
    }
}

impl ASTChildren for EnumDefinition {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        // super().process_children(f)
        // cb.add_child(AST::Expression(Box::new(self.func.clone())));
        self.values.iter().for_each(|value| {
            cb.add_child(AST::EnumValue(value.clone()));
        });
    }
}
// class EnumDefinition(NamespaceDefinition):
//     def __init__(self, idf: Identifier, values: List[EnumValue]):
//         super().__init__(idf)
//         self.values = values

//         self.annotated_type: Optional[AnnotatedTypeName] = None

//     def process_children(self, f: Callable[[T], T]):
//         super().process_children(f)
//         self.values[:] = map(f, self.values)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StructDefinition {
    idf: Identifier,
    members: Vec<VariableDeclaration>,
}
impl StructDefinition {
    pub fn new(idf: Identifier, members: Vec<VariableDeclaration>) -> Self {
        Self { idf, members }
    }
}
impl ASTChildren for StructDefinition {
    fn process_children(&mut self, cb: &mut ChildListBuilder) {
        // super().process_children(f)
        // cb.add_child(AST::Expression(Box::new(self.func.clone()));
        self.members.iter().for_each(|member| {
            cb.add_child(AST::IdentifierDeclaration(
                IdentifierDeclaration::VariableDeclaration(member.clone()),
            ));
        });
    }
}

// class StructDefinition(NamespaceDefinition):
//     def __init__(self, idf: Identifier, members: List[VariableDeclaration]):
//         super().__init__(idf)
//         self.members = members

//     def process_children(self, f: Callable[[T], T]):
//         super().process_children(f)
//         self.members[:] = map(f, self.members)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub idf: Option<Identifier>,
    pub state_variable_declarations: Vec<StateVariableDeclaration>,
    pub constructor_definitions: Vec<ConstructorOrFunctionDefinition>,
    pub function_definitions: Vec<ConstructorOrFunctionDefinition>,
    pub enum_definitions: Vec<EnumDefinition>,
    pub struct_definitions: Vec<StructDefinition>,
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
}
impl ContractDefinition {
    pub fn new(
        idf: Option<Identifier>,
        state_variable_declarations: Vec<StateVariableDeclaration>,
        constructor_definitions: Vec<ConstructorOrFunctionDefinition>,
        function_definitions: Vec<ConstructorOrFunctionDefinition>,
        enum_definitions: Vec<EnumDefinition>,
        struct_definitions: Option<Vec<StructDefinition>>,
        used_crypto_backends: Option<Vec<CryptoParams>>,
    ) -> Self {
        Self {
            idf,
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
        // super().process_children(f)
        // cb.add_child(AST::Expression(self.func.clone()));
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
                cb.add_child(AST::IdentifierDeclaration(
                    IdentifierDeclaration::StateVariableDeclaration(
                        state_variable_declarations.clone(),
                    ),
                ));
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
// class ContractDefinition(NamespaceDefinition):

//     def __init__(
//             self,
//             idf: Identifier,
//             state_variable_declarations: List[StateVariableDeclaration],
//             constructor_definitions: List[ConstructorOrFunctionDefinition],
//             function_definitions: List[ConstructorOrFunctionDefinition],
//             enum_definitions: List[EnumDefinition],
//             struct_definitions: Optional[List[StructDefinition]] = None):
//         super().__init__(idf)
//         self.state_variable_declarations = state_variable_declarations
//         self.constructor_definitions = constructor_definitions
//         self.function_definitions = function_definitions
//         self.enum_definitions = enum_definitions
//         self.struct_definitions = [] if struct_definitions is None else struct_definitions
//         self.used_crypto_backends: Optional[List[CryptoParams]] = None

//     def process_children(self, f: Callable[[T], T]):
//         super().process_children(f)
//         self.enum_definitions[:] = map(f, self.enum_definitions)
//         self.struct_definitions[:] = map(f, self.struct_definitions)
//         self.state_variable_declarations[:] = map(f, self.state_variable_declarations)
//         self.constructor_definitions[:] = map(f, self.constructor_definitions)
//         self.function_definitions[:] = map(f, self.function_definitions)

//     def __getitem__(self, key: str):
//         if key == 'constructor':
//             if len(self.constructor_definitions) == 0:
//                 // return empty constructor
//                 c = ConstructorOrFunctionDefinition(None, [], [], None, Block([]))
//                 c.parent = self
//                 return c
//             elif len(self.constructor_definitions) == 1:
//                 return self.constructor_definitions[0]
//             else:
//                 raise ValueError('Multiple constructors exist')
//         else:
//             d_identifier = self.names[key]
//             return d_identifier.parent

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pragma_directive: String,
    contracts: Vec<ContractDefinition>,
    used_contracts: Vec<String>,
    pub used_homomorphisms: Option<BTreeSet<Homomorphism>>,
    used_crypto_backends: Option<Vec<CryptoParams>>,
    original_code: Vec<String>,
}
impl SourceUnit {
    pub fn new(
        pragma_directive: String,
        contracts: Vec<ContractDefinition>,
        used_contracts: Option<Vec<String>>,
    ) -> Self {
        Self {
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
// class SourceUnit(AST):

//     def __init__(self, pragma_directive: str, contracts: List[ContractDefinition], used_contracts: Optional[Vec<String>] = None):
//         super().__init__()
//         self.pragma_directive = pragma_directive
//         self.contracts = contracts
//         self.used_contracts = [] if used_contracts is None else used_contracts
//         self.used_homomorphisms: Optional[Set[Homomorphism]] = None
//         self.used_crypto_backends: Optional[List[CryptoParams]] = None

//         self.original_code: Vec<String> = []

//     def process_children(self, f: Callable[[T], T]):
//         self.contracts[:] = map(f, self.contracts)

//     def __getitem__(self, key: str):
//         c_identifier = self.names[key]
//         c = c_identifier.parent
//         assert (isinstance(c, ContractDefinition))
//         return c

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum PrivacyLabelExpr {
    MeExpr(MeExpr),
    AllExpr(AllExpr),
    Identifier(Identifier),
    #[default]
    None,
}
// PrivacyLabelExpr = Union[MeExpr, AllExpr, Identifier]
// TargetDefinition = Union[IdentifierDeclaration, NamespaceDefinition]
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TargetDefinition {
    IdentifierDeclaration(IdentifierDeclaration),
    NamespaceDefinition(NamespaceDefinition),
    #[default]
    None,
}

// def get_privacy_expr_from_label(plabel: PrivacyLabelExpr):
//     """Turn privacy label into expression (i.e. Identifier -> IdentifierExpr, Me and All stay the same)."""
//     if isinstance(plabel, Identifier):
//         return IdentifierExpr(plabel.clone(), AnnotatedTypeName.address_all()).override(target=plabel.parent)
//     else:
//         return plabel.clone()
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum InstanceTargetExprType {
    Tuple((Option<TargetDefinition>, Option<AST>)),
    VariableDeclaration(VariableDeclaration),
    LocationExpr(LocationExpr),
    #[default]
    None,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct InstanceTarget {
    target_key: (Option<TargetDefinition>, Option<AST>),
}
// class InstanceTarget(tuple):
//     def __new__(cls, expr: Union[tuple, VariableDeclaration, LocationExpr]):
//         if isinstance(expr, tuple):
//             // copy constructor
//             target_key = expr[:]
//         else:
//             target_key = [None, None]
//             if isinstance(expr, VariableDeclaration):
//                 target_key[0] = expr
//             elif isinstance(expr, IdentifierExpr):
//                 target_key[0] = expr.target
//             elif isinstance(expr, MemberAccessExpr):
//                 target_key[0] = expr.expr.target
//                 target_key[1] = expr.member.clone()
//             else:
//                 assert isinstance(expr, IndexExpr)
//                 target_key[0] = expr.arr.target
//                 target_key[1] = expr.key

//         assert isinstance(target_key[0], (VariableDeclaration, Parameter, StateVariableDeclaration))
//         return super(InstanceTarget, cls).__new__(cls, target_key)

//     def __eq__(self, other):
//         return isinstance(other, type(self)) and super().__eq__(other)

//     def __hash__(self):
//         return hash(self[:])

//     @property
//     def target(self) -> IdentifierDeclaration:
//         return self[0]

//     @property
//     def key(self) -> Optional[Union[Identifier, Expression]]:
//         return self[1]

//     @property
//     def privacy(self) -> PrivacyLabelExpr:
//         if self.key is None or not isinstance(self.target.annotated_type.type_name, Mapping):
//             return self.target.annotated_type.zkay_type.privacy_annotation.privacy_annotation_label()
//         else:
//             t = self.target.annotated_type.zkay_type.type_name
//             assert isinstance(t, Mapping)
//             if t.has_key_label:
//                 return self.key.privacy_annotation_label()
//             else:
//                 t.value_type.privacy_annotation.privacy_annotation_label()

//     def in_scope_at(self, ast: AST) -> bool:
//         from zkay.zkay_ast.pointers.symbol_table import SymbolTableLinker
//         return SymbolTableLinker.in_scope_at(self.target.idf, ast)

// // UTIL FUNCTIONS

fn indent(s: String) -> String {
    format!("{}{}", CFG.lock().unwrap().user_config.indentation(), s)
}

// // EXCEPTIONS

// fn get_code_error_msg(line: i32, column: i32, code: Vec<String>, ctr: Option<ContractDefinition>,
//                        fct: Option<ConstructorOrFunctionDefinition>, stmt: Option<Statement>)->String{
//     // Print Location
//     let mut error_msg = format!("At line: {line};{column}");

//     // If error location is outside code bounds, only show line;col
//     if line <= 0 || column <= 0 || line > code.len() as i32
//        { return error_msg
//         }

//     if fct.is_some()
//       {  assert!( ctr.is_some());
//         error_msg += ", in function \"{fct.name}\" of contract \"{ctr.idf.name}\"";}
//     else if ctr.is_some():
//       {  error_msg += ", in contract \"{ctr.idf.name}\"";}
//     error_msg += " ";

//     let start_line =  if let Some(stmt)=stmt {stmt.line} else {line};
//     if start_line != -1
//         {
//         for line in range(start_line, line + 1):
//             {// replace tabs with 4 spaces for consistent output
//             let mut orig_line: String = code[line - 1].clone();
//             orig_line = orig_line.replace("\t", "    ");
//             error_msg += format!("{orig_line} ");
//             }

//         let affected_line = &code[line - 1];
//         let loc_string:String = affected_line[:column - 1].chars().map(|c| (if c == "\t"  {"----"}  else {"-"}).to_string() ).collect::<Vec<String>>().concat() ;
//         format!( "{error_msg}{loc_string}/")
//         }
//     else
//         { error_msg}
// }

// def get_ast_exception_msg(ast: AST, msg: str):
//     // Get surrounding statement
//     if isinstance(ast, Expression):
//         stmt = ast.statement
//     elif isinstance(ast, Statement):
//         stmt = ast
//     else:
//         stmt = None

//     // Get surrounding function
//     if stmt is not None:
//         fct = stmt.function
//     elif isinstance(ast, ConstructorOrFunctionDefinition):
//         fct = ast
//     else:
//         fct = None

//     // Get surrounding contract
//     ctr = ast if fct is None else fct
//     while ctr is not None and not isinstance(ctr, ContractDefinition):
//         ctr = ctr.parent

//     // Get source root
//     root = ast if ctr is None else ctr
//     while root is not None and not isinstance(root, SourceUnit):
//         root = root.parent

//     if root is None:
//         error_msg = 'error'
//     else:
//         error_msg = get_code_error_msg(ast.line, ast.column, root.original_code, ctr, fct, stmt)

//     return f' {error_msg}  {msg}'

// def issue_compiler_warning(ast: AST, warning_type: str, msg: str):
//     if cfg.is_unit_test:
//         return
//     with warn_print():
//         zk_print(f' \nWARNING: {warning_type}{get_ast_exception_msg(ast, msg)}')

// class AstException(Exception):
//     """Generic exception for errors in an AST"""

//     def __init__(self, msg, ast):
//         super().__init__(get_ast_exception_msg(ast, msg))

// // CODE GENERATION
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ListKind {
    AST(AST),
    String(String),
    #[default]
    None,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SingleOrListKind {
    Vec(Vec<ListKind>),
    AST(AST),
    String(String),
    #[default]
    None,
}
pub struct CodeVisitor {
    display_final: bool,
    traversal: &'static str,
    log: bool,
}

// class CodeVisitor(AstVisitor):

//     def __init__(self, display_final=True):
//         super().__init__('node-or-children')
//         self.display_final = display_final
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
    pub fn visit_list(&self, l: Vec<ListKind>, mut sep: &str) -> CodeVisitorReturn {
        if sep.is_empty() {
            sep = "\n";
        }
        if l.is_empty() {
            return String::new();
        }

        fn handle(selfs: &CodeVisitor, e: &ListKind) -> Option<String> {
            if let ListKind::String(e) = e {
                Some(e.to_owned())
            } else if let ListKind::AST(e) = e {
                Some(selfs.visit(e))
            } else {
                None
            }
        }

        let s: Vec<_> = l.iter().filter_map(|e| handle(self, e)).collect();
        let s = s.concat();
        s
    }

    pub fn visit_single_or_list(&self, v: SingleOrListKind, mut sep: &str) -> CodeVisitorReturn {
        if sep.is_empty() {
            sep = "\n";
        }
        match v {
            SingleOrListKind::Vec(v) => self.visit_list(v, sep),
            SingleOrListKind::String(v) => v,
            SingleOrListKind::AST(v) => self.visit(&v),
            _ => String::new(),
        }
    }

    pub fn visit_AST(&self, ast: AST) -> CodeVisitorReturn {
        // should never be called
        // raise NotImplementedError("Did not implement code generation for " + repr(ast))
        unimplemented!("Did not implement code generation for {:?} ", ast);
        String::new()
    }
    pub fn visit_Comment(&self, ast: Comment) -> CodeVisitorReturn {
        if ast.text == String::new() {
            String::new()
        } else if ast.text.contains(" ") {
            format!("/* {} */", ast.text)
        } else {
            format!("// {}", ast.text)
        }
    }

    pub fn visit_Identifier(&self, ast: Identifier) -> CodeVisitorReturn {
        ast.name.clone()
    }

    pub fn visit_FunctionCallExpr(&self, ast: FunctionCallExpr) -> CodeVisitorReturn {
        if let Expression::BuiltinFunction(func) = *ast.func {
            let args: Vec<_> = ast
                .args
                .iter()
                .map(|a| self.visit(&AST::Expression(Box::new(a.clone()))))
                .collect();
            func.format_string(&args)
        } else {
            let f = self.visit(&AST::Expression(ast.func));
            let a = self.visit_list(
                ast.args
                    .iter()
                    .map(|arg| ListKind::AST(AST::Expression(Box::new(arg.clone()))))
                    .collect(),
                ", ",
            );
            format!("{f}({a})")
        }
    }

    pub fn visit_PrimitiveCastExpr(&self, ast: PrimitiveCastExpr) -> CodeVisitorReturn {
        if ast.is_implicit {
            self.visit(&AST::Expression(ast.expr.clone()))
        } else {
            format!(
                "{}({})",
                self.visit(&AST::TypeName(ast.elem_type)),
                self.visit(&AST::Expression(ast.expr))
            )
        }
    }

    pub fn visit_BooleanLiteralExpr(&self, ast: BooleanLiteralExpr) -> CodeVisitorReturn {
        ast.value.to_string().to_ascii_lowercase()
    }

    pub fn visit_NumberLiteralExpr(&self, ast: NumberLiteralExpr) -> CodeVisitorReturn {
        if ast.was_hex {
            format!("{:x}", ast.value)
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
                ast.values
                    .iter()
                    .map(|value| ListKind::AST(AST::Expression(Box::new(value.clone()))))
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
                    .map(|element| ListKind::AST(AST::Expression(Box::new(element.clone()))))
                    .collect(),
                ", "
            )
        )
    }

    pub fn visit_IdentifierExpr(&self, ast: IdentifierExpr) -> CodeVisitorReturn {
        self.visit(&AST::Identifier(IdentifierKind::Identifier(ast.idf)))
    }

    pub fn visit_MemberAccessExpr(&self, ast: MemberAccessExpr) -> CodeVisitorReturn {
        format!(
            "{}.{}",
            self.visit(&AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                Box::new(TupleOrLocationExpr::LocationExpr(Box::new(ast.expr)))
            )))),
            self.visit(&AST::Identifier(IdentifierKind::Identifier(ast.member)))
        )
    }

    pub fn visit_IndexExpr(&self, ast: IndexExpr) -> CodeVisitorReturn {
        format!(
            "{}[{}]",
            self.visit(&AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                Box::new(TupleOrLocationExpr::LocationExpr(Box::new(ast.arr)))
            )))),
            self.visit(&AST::Expression(Box::new(ast.key)))
        )
    }

    pub fn visit_MeExpr(&self, _: MeExpr) -> CodeVisitorReturn {
        String::from("me")
    }

    pub fn visit_AllExpr(&self, _: AllExpr) -> CodeVisitorReturn {
        String::from("all")
    }

    pub fn visit_ReclassifyExpr(&self, ast: ReclassifyExpr) -> CodeVisitorReturn {
        let e = self.visit(&AST::Expression(ast.expr));
        let p = self.visit(&AST::Expression(Box::new(ast.privacy)));
        let h = HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .get(&ast.homomorphism.unwrap_or(String::from("NON_HOMOMORPHIC")))
            .unwrap()
            .clone();
        format!("reveal{h:?}({e}, {p})")
    }

    pub fn visit_RehomExpr(&self, ast: RehomExpr) -> CodeVisitorReturn {
        let e = self.visit(&AST::Expression(ast.expr.clone()));
        format!("{}({e})", ast.func_name())
    }

    pub fn visit_IfStatement(&self, ast: IfStatement) -> CodeVisitorReturn {
        let c = self.visit(&AST::Expression(Box::new(ast.condition)));
        let t = self.visit_single_or_list(
            SingleOrListKind::AST(AST::Statement(Statement::StatementList(Box::new(
                StatementList::Block(Box::new(ast.then_branch)),
            )))),
            "",
        );
        let mut ret = format!("if ({c}) {t}");
        if let Some(else_branch) = ast.else_branch {
            let e = self.visit_single_or_list(
                SingleOrListKind::AST(AST::Statement(Statement::StatementList(Box::new(
                    StatementList::Block(Box::new(else_branch)),
                )))),
                "",
            );
            ret += format!("\n else {e}").as_str();
        }
        ret
    }

    pub fn visit_WhileStatement(&self, ast: WhileStatement) -> CodeVisitorReturn {
        let c = self.visit(&AST::Expression(Box::new(ast.condition)));
        let b = self.visit_single_or_list(
            SingleOrListKind::AST(AST::Statement(Statement::StatementList(Box::new(
                StatementList::Block(Box::new(ast.body)),
            )))),
            "",
        );
        format!("while ({c}) {b}")
    }

    pub fn visit_DoWhileStatement(&self, ast: DoWhileStatement) -> CodeVisitorReturn {
        let b = self.visit_single_or_list(
            SingleOrListKind::AST(AST::Statement(Statement::StatementList(Box::new(
                StatementList::Block(Box::new(ast.body)),
            )))),
            "",
        );
        let c = self.visit(&AST::Expression(Box::new(ast.condition)));
        format!("do {b} while ({c});")
    }

    pub fn visit_ForStatement(&self, ast: ForStatement) -> CodeVisitorReturn {
        let i = if let Some(init) = ast.init {
            format!(
                "{}",
                self.visit_single_or_list(
                    SingleOrListKind::AST(AST::Statement(Statement::SimpleStatement(Box::new(
                        init
                    )))),
                    ""
                )
            )
        } else {
            String::from(";")
        };
        let c = self.visit(&AST::Expression(Box::new(ast.condition)));
        let u = if let Some(update) = ast.update {
            format!(
                " {}",
                self.visit_single_or_list(
                    SingleOrListKind::AST(AST::Statement(Statement::SimpleStatement(Box::new(
                        update
                    )))),
                    ""
                )
                .replace(";", "")
            )
        } else {
            String::new()
        };
        let b = self.visit_single_or_list(
            SingleOrListKind::AST(AST::Statement(Statement::StatementList(Box::new(
                StatementList::Block(Box::new(ast.body)),
            )))),
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
        if let Expression::None = ast.expr {
            String::from("return;")
        } else {
            let e = self.visit(&AST::Expression(Box::new(ast.expr)));
            format!("return {e};")
        }
    }

    pub fn visit_ExpressionStatement(&self, ast: ExpressionStatement) -> CodeVisitorReturn {
        self.visit(&AST::Expression(Box::new(ast.expr))) + ";"
    }

    pub fn visit_RequireStatement(&self, ast: RequireStatement) -> CodeVisitorReturn {
        let c = self.visit(&AST::Expression(Box::new(ast.condition)));
        format!("require({c});")
    }

    pub fn visit_AssignmentStatement(&self, ast: AssignmentStatement) -> CodeVisitorReturn {
        let lhs = ast.lhs.clone();
        let mut op = ast.op.clone();
        match ast.lhs {
            AssignmentStatementUnion::TupleExpr(asu) => {
                if let Some(at) = asu.annotated_type {
                    if at.is_private() {
                        op = String::new();
                    }
                }
            }
            AssignmentStatementUnion::LocationExpr(le) => {
                let annotated_type = match le {
                    LocationExpr::IdentifierExpr(ie) => {
                        if let Some(at) = ie.annotated_type {
                            Some(*at)
                        } else {
                            None
                        }
                    }
                    LocationExpr::MemberAccessExpr(ie) => {
                        ie.location_expr_base
                            .tuple_or_location_expr_base
                            .expression_base
                            .annotated_type
                    }
                    LocationExpr::IndexExpr(ie) => {
                        ie.location_expr_base
                            .tuple_or_location_expr_base
                            .expression_base
                            .annotated_type
                    }
                    LocationExpr::SliceExpr(ie) => {
                        ie.location_expr_base
                            .tuple_or_location_expr_base
                            .expression_base
                            .annotated_type
                    }
                    _ => None,
                };
                if let Some(at) = annotated_type {
                    if at.is_private() {
                        op = String::new();
                    }
                }
            }
            _ => {}
        }
        let rhs = if !op.is_empty() {
            if let Expression::FunctionCallExpr(fce) = ast.rhs {
                fce.args[1].clone()
            } else {
                Expression::default()
            }
        } else {
            ast.rhs.clone()
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
        if let (lhs, Expression::TupleOrLocationExpr(rhs)) = (&lhs, &rhs) {
            if let (
                AssignmentStatementUnion::LocationExpr(lhs),
                TupleOrLocationExpr::LocationExpr(rhs),
            ) = (lhs, *rhs.clone())
            {
                if let (LocationExpr::SliceExpr(lhs), LocationExpr::SliceExpr(rhs)) = (lhs, *rhs) {
                    assert!(lhs.size == rhs.size, "Slice ranges don't have same size");
                    let mut s = String::new();
                    let (lexpr, rexpr) = (
                        self.visit(&AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                            Box::new(TupleOrLocationExpr::LocationExpr(Box::new(lhs.arr.clone()))),
                        )))),
                        self.visit(&AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                            Box::new(TupleOrLocationExpr::LocationExpr(Box::new(rhs.arr.clone()))),
                        )))),
                    );
                    let mut lbase = if let Some(base) = &lhs.base {
                        format!(
                            "{} + ",
                            self.visit(&AST::Expression(Box::new(base.clone())))
                        )
                    } else {
                        String::new()
                    };
                    let mut rbase = if let Some(base) = rhs.base {
                        format!("{} + ", self.visit(&AST::Expression(Box::new(base))))
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
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            let to_ast = |hs| match hs {
                AssignmentStatementUnion::TupleExpr(te) => {
                    self.visit(&AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                        Box::new(TupleOrLocationExpr::TupleExpr(Box::new(te.clone()))),
                    ))))
                }
                AssignmentStatementUnion::LocationExpr(le) => {
                    self.visit(&AST::Expression(Box::new(Expression::TupleOrLocationExpr(
                        Box::new(TupleOrLocationExpr::LocationExpr(Box::new(le.clone()))),
                    ))))
                }
                _ => String::new(),
            };
            format_string(to_ast(lhs), self.visit(&AST::Expression(Box::new(rhs))))
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
                        .statements
                        .iter()
                        .map(|statement| ListKind::AST(AST::Statement(statement.clone())))
                        .collect(),
                    "",
                ),
            ),
            StatementList::IndentBlock(block) => indent(
                self.visit_list(
                    block
                        .statements
                        .iter()
                        .map(|statement| ListKind::AST(AST::Statement(statement.clone())))
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
                        .statements
                        .iter()
                        .map(|statement| ListKind::AST(AST::Statement(statement.clone())))
                        .collect(),
                    "",
                ),
            ),
            StatementList::IndentBlock(block) => indent(
                self.visit_list(
                    block
                        .statements
                        .iter()
                        .map(|statement| ListKind::AST(AST::Statement(statement.clone())))
                        .collect(),
                    "",
                ),
            ),
            _ => String::new(),
        }
    }

    pub fn visit_Block(&self, ast: Block) -> CodeVisitorReturn {
        let b = self
            .handle_block(StatementList::Block(Box::new(ast.clone())))
            .trim_end()
            .to_string();
        if ast.was_single_statement && ast.statements.len() == 1 {
            b
        } else {
            format!("{{ {b} }}")
        }
    }

    pub fn visit_IndentBlock(&self, ast: IndentBlock) -> CodeVisitorReturn {
        self.handle_block(StatementList::IndentBlock(Box::new(ast)))
    }

    pub fn visit_ElementaryTypeName(&self, ast: ElementaryTypeName) -> CodeVisitorReturn {
        match ast {
            ElementaryTypeName::NumberTypeName(ntn) => match ntn {
                NumberTypeName::NumberLiteralType(nlt) => nlt.name.clone(),
                NumberTypeName::IntTypeName(itn) => itn.name.clone(),
                NumberTypeName::UintTypeName(utn) => utn.name.clone(),
                NumberTypeName::AnyNumberTypeName(antn) => antn.name.clone(),
                _ => String::new(),
            },
            ElementaryTypeName::BoolTypeName(btn) => btn.name.clone(),
            ElementaryTypeName::BooleanLiteralType(blt) => blt.name.clone(),
            _ => String::new(),
        }
    }

    pub fn visit_UserDefinedTypeName(&self, ast: UserDefinedTypeName) -> CodeVisitorReturn {
        let names: Vec<_> = (match ast {
            UserDefinedTypeName::EnumTypeName(ast) => ast.names,
            UserDefinedTypeName::EnumValueTypeName(ast) => ast.names,
            UserDefinedTypeName::StructTypeName(ast) => ast.names,
            UserDefinedTypeName::ContractTypeName(ast) => ast.names,
            UserDefinedTypeName::AddressTypeName(ast) => ast.names,
            UserDefinedTypeName::AddressPayableTypeName(ast) => ast.names,
            _ => vec![],
        })
        .iter()
        .map(|name| ListKind::AST(AST::Identifier(IdentifierKind::Identifier(name.clone()))))
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
            self.visit(&AST::Expression(Box::new(privacy_annotation)))
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
        let label = if let KeyLabelType::Identifier(idf) = ast.key_label {
            if let Some(idf) = idf {
                format!(
                    "!{}",
                    self.visit(&AST::Identifier(IdentifierKind::Identifier(idf)))
                )
            } else {
                String::new()
            }
        } else {
            if let KeyLabelType::String(key_label) = ast.key_label {
                format!("/*!{}*/", key_label)
            } else {
                String::new()
            }
        };
        let v = self.visit(&AST::AnnotatedTypeName(*ast.value_type));
        format!("mapping({k}{label} => {v})")
    }

    pub fn visit_Array(&self, ast: Array) -> CodeVisitorReturn {
        let dat = AnnotatedTypeName::default();
        let value_type = match &ast {
            Array::CipherText(ast) => &ast.value_type,
            Array::Randomness(ast) => &ast.value_type,
            Array::Key(ast) => &ast.value_type,
            Array::Proof(ast) => &ast.value_type,
            Array::ArrayBase(ast) => &ast.value_type,
            _ => &dat,
        };
        let et = ExprType::default();
        let expr = match &ast {
            Array::CipherText(ast) => &ast.expr,
            Array::Randomness(ast) => &ast.expr,
            Array::Key(ast) => &ast.expr,
            Array::Proof(ast) => &ast.expr,
            Array::ArrayBase(ast) => &ast.expr,
            _ => &et,
        };
        let t = self.visit(&AST::AnnotatedTypeName(value_type.clone()));
        let e = if let ExprType::Expression(expr) = &expr {
            self.visit(&AST::Expression(Box::new(expr.clone())))
        } else if let ExprType::I32(expr) = &expr {
            expr.to_string()
        } else {
            String::new()
        };
        format!("{t}[{e}]")
    }

    pub fn visit_CipherText(&self, ast: CipherText) -> CodeVisitorReturn {
        let e = self.visit_Array(Array::CipherText(ast.clone()));
        format!("{e}/*{}*/", ast.plain_type.code())
    }

    pub fn visit_TupleType(&self, ast: TupleType) -> CodeVisitorReturn {
        let s = self.visit_list(
            ast.types
                .iter()
                .map(|typ| ListKind::AST(AST::AnnotatedTypeName(typ.clone())))
                .collect(),
            ", ",
        );
        format!("({s})")
    }

    pub fn visit_VariableDeclaration(&self, ast: VariableDeclaration) -> CodeVisitorReturn {
        let keywords: Vec<_> = ast
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
        let t = self.visit(&AST::AnnotatedTypeName(*ast.annotated_type));
        let s = if let Some(storage_location) = ast.storage_location {
            format!(" {storage_location}")
        } else {
            String::new()
        };
        let i = self.visit(&AST::Identifier(IdentifierKind::Identifier(ast.idf)));
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
            s += format!(" = {}", self.visit(&AST::Expression(Box::new(expr)))).as_str();
        }
        s += ";";
        s
    }

    pub fn visit_Parameter(&self, ast: Parameter) -> CodeVisitorReturn {
        let final_string = String::from("final");
        let f = if !self.display_final {
            None
        } else {
            if ast.keywords.contains(&final_string) {
                Some(final_string)
            } else {
                None
            }
        };
        let t = Some(self.visit(&AST::AnnotatedTypeName(*ast.annotated_type)));
        let i = Some(self.visit(&AST::Identifier(IdentifierKind::Identifier(ast.idf))));
        let description: Vec<_> = [f, t, ast.storage_location, i]
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
                SingleOrListKind::AST(AST::Statement(Statement::StatementList(Box::new(
                    StatementList::Block(Box::new(body)),
                )))),
                "",
            )
        } else {
            String::new()
        };
        self.function_definition_to_str(
            ast.idf,
            ast.parameters
                .iter()
                .map(|parameter| ParameterKind::Parameter(parameter.clone()))
                .collect(),
            ast.modifiers,
            ast.return_parameters,
            b,
        )
    }
    fn function_definition_to_str(
        &self,
        idf: Identifier,
        parameters: Vec<ParameterKind>,
        modifiers: Vec<String>,
        return_parameters: Vec<Parameter>,
        body: String,
    ) -> CodeVisitorReturn {
        let definition = if idf.name != String::from("constructor") {
            let i = self.visit(&AST::Identifier(IdentifierKind::Identifier(idf)));
            format!("function {i}")
        } else {
            String::from("constructor")
        };
        let p = self.visit_list(
            parameters
                .iter()
                .filter_map(|parameter| match parameter {
                    ParameterKind::Parameter(p) => Some(ListKind::AST(AST::IdentifierDeclaration(
                        IdentifierDeclaration::Parameter(p.clone()),
                    ))),
                    ParameterKind::String(s) => Some(ListKind::String(s.clone())),
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
                    ListKind::AST(AST::IdentifierDeclaration(
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
            self.visit(&AST::Identifier(IdentifierKind::Identifier(idf)))
        } else {
            String::new()
        }
    }

    pub fn visit_EnumDefinition(&self, ast: EnumDefinition) -> CodeVisitorReturn {
        let values = indent(
            self.visit_list(
                ast.values
                    .iter()
                    .map(|value| ListKind::AST(AST::EnumValue(value.clone())))
                    .collect(),
                ", ",
            ),
        );
        format!(
            "enum {} {{\n{values}\n}}",
            if let Some(idf) = ast.idf {
                self.visit(&AST::Identifier(IdentifierKind::Identifier(idf)))
            } else {
                String::new()
            }
        )
    }

    // @staticmethod
    fn __cmp_type_size(v1: &VariableDeclaration, v2: &VariableDeclaration) -> Ordering {
        let (t1, t2) = (&v1.annotated_type.type_name, &v2.annotated_type.type_name);
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
                .map(|member| {
                    self.visit(&AST::IdentifierDeclaration(
                        IdentifierDeclaration::VariableDeclaration(member.clone()),
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );
        format!(
            "struct {} {{\n{body}\n}}",
            self.visit(&AST::Identifier(IdentifierKind::Identifier(ast.idf)))
        )
    }

    pub fn visit_StateVariableDeclaration(
        &self,
        ast: StateVariableDeclaration,
    ) -> CodeVisitorReturn {
        let final_string = String::from("final");
        let keywords: Vec<_> = ast
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
        let t = self.visit(&AST::AnnotatedTypeName(*ast.annotated_type));
        let mut k = ast
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
        let i = self.visit(&AST::Identifier(IdentifierKind::Identifier(ast.idf)));
        let mut ret = format!("{f}{t} {k}{i}").trim().to_string();
        if let Some(expr) = ast.expr {
            ret += &format!(" = {}", self.visit(&AST::Expression(Box::new(expr))));
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
            .map(|e| {
                self.visit(&AST::IdentifierDeclaration(
                    IdentifierDeclaration::StateVariableDeclaration(e.clone()),
                ))
            })
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
            ast.idf.unwrap(),
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
                    ListKind::AST(AST::NamespaceDefinition(
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
