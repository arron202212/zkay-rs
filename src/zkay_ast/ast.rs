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
// use  typing import List, Dict, Union, Optional, Callable, Set, TypeVar;
use crate::config::CFG; //, zk_print;
use crate::transaction::crypto::params::CryptoParams;
// use  crate::utils::progress_printer import warn_print;
use crate::zkay_ast::analysis::partition_state::PartitionState;
use crate::zkay_ast::homomorphism::Homomorphism;
// use  crate::zkay_ast::visitor::visitor import AstVisitor
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
// T = TypeVar('T')

// class ChildListBuilder:
//     def __init__(self):
//         self.children = []

//     def add_child(self, ast: AST) -> AST:
//         if ast is not None:
//             self.children.append(ast)
//         return ast
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum AST {
    #[default]
    Never,
    Identifier(Identifier),
    Pragma(String),
    VersionPragma(String),
    Comment(Comment),
    Expression(Box<Expression>),
    Statement(Statement),
    TypeName(TypeName),
ConstructorOrFunctionDefinition(ConstructorOrFunctionDefinition),
ContractDefinition(ContractDefinition),
    Block(Option<Block>),
    AnnotatedTypeName(AnnotatedTypeName),
    IdentifierDeclaration(IdentifierDeclaration),
    StateVariableDeclaration(StateVariableDeclaration),
    NamespaceDefinition(NamespaceDefinition),
EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    SourceUnit(SourceUnit),
  Parameters(Option<Vec<Parameter>>),
        Modifiers(Option<Vec<String>>),
        ReturnParameters(Option<Vec<Parameter>>),
BooleanLiteralExpr(BooleanLiteralExpr),
StringLiteralExpr(StringLiteralExpr),
NumberLiteralExpr(NumberLiteralExpr),
TupleExpr(TupleExpr),
Modifier(String),
Homomorphism(String),
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
pub struct ASTStore {
    pub parent: Option<Box<AST>>,
    pub namespace: Option<Vec<Identifier>>,
    pub names: BTreeMap<String, Identifier>,
    pub line: i32,
    pub column: i32,
    pub modified_values: BTreeSet<InstanceTarget>,
    pub read_values: BTreeSet<InstanceTarget>,
}
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
pub trait Immutable {
    fn is_immutable(&self) -> bool;
}
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
            if let AST::StateVariableDeclaration(svd) = &**v {
                svd.is_final() || svd.is_constant()
            } else {
                false
            }
        } else {
            false
        }
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
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Comment {
    pub text: String,
}

// class BlankLine(Comment):
//     def __init__(self):
//         super().__init__()
pub type BlankLine = Comment;

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
    ReclassifyExpr(Box<ReclassifyExpr>),
    #[serde(rename_all = "camelCase")]
    #[default]
    UnhandledExpression,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStore {
    pub annotated_type: Option<AnnotatedTypeName>,
    pub statement: Option<Statement>,
    pub evaluate_privately: bool,
}
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
    Never,
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
    static ref BUILTIN_FUNCTIONS: HashMap<String, String> = {
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
//     assert __hom.op in builtin_op_fct and __hom.homomorphism != Homomorphism.NonHomomorphic
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
    pub op: String,
    pub is_private: bool,
    pub homomorphism: String,
    pub rerand_using: Option<Box<IdentifierExpr>>,
}
// class BuiltinFunction(Expression):

//     def __init__(self, op: str):
//         super().__init__()
//         self.op = op

//         // set later by type checker
//         self.is_private: bool = False
//         self.homomorphism: Homomorphism = Homomorphism.NonHomomorphic

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
pub struct FunctionCallExpr {
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub sec_start_offset: Option<i32>,
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

pub type NewExpr = FunctionCallExpr;

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
// class PrimitiveCastExpr(Expression):
//     def __init__(self, elem_type: TypeName, expr: Expression, is_implicit=False):
//         super().__init__()
//         self.elem_type = elem_type
//         self.expr = expr
//         self.is_implicit = is_implicit

//     def process_children(self, f: Callable[[T], T]):
//         self.elem_type = f(self.elem_type)
//         self.expr = f(self.expr)

pub type LiteralExpr = Expression;
// class LiteralExpr(Expression):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BooleanLiteralExpr {
    pub value: bool,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
impl BooleanLiteralExpr{
    pub fn new(value: bool)->Self{
        Self{value,annotated_type:Some(Box::new(AnnotatedTypeName::new(TypeName::BooleanLiteralType(BooleanLiteralType::new(value)),None,String::from("NON_HOMOMORPHIC"))))}
    }
}
// class BooleanLiteralExpr(LiteralExpr):

//     def __init__(self, value: bool):
//         super().__init__()
//         self.value = value
//         self.annotated_type = AnnotatedTypeName(BooleanLiteralType(self.value))

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NumberLiteralExpr {
    pub value: i32,
    pub was_hex: bool,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
}
impl NumberLiteralExpr{
    pub fn new(value: i32,was_hex: bool)->Self{
        Self{value,was_hex,annotated_type:Some(Box::new(AnnotatedTypeName::new(TypeName::NumberLiteralType(NumberLiteralType::new(NumberLiteralTypeKind::I32(value))),None,String::from("NON_HOMOMORPHIC"))))}
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
impl StringLiteralExpr{
pub fn new(value:String)->Self{
        Self{value}
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
pub struct TupleOrLocationExpr {
    parent: Option<Box<AST>>,
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
    elements: Vec<Expression>,
}
impl TupleExpr{
pub fn new(elements: Vec<Expression>)->Self{
    Self{elements}
}}
// class TupleExpr(TupleOrLocationExpr):
//     def __init__(self, elements: List[Expression]):
//         super().__init__()
//         self.elements = elements

//     def process_children(self, f: Callable[[T], T]):
//         self.elements[:] = map(f, self.elements)

//     def assign(self, val: Expression) -> AssignmentStatement:
//         return AssignmentStatement(self, val)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LocationExpr {
    pub target: Option<TargetDefinition>,
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
pub enum Location {
    IdentifierExpr(IdentifierExpr),
    MemberAccessExpr(MemberAccessExpr),
    IndexExpr(IndexExpr),
    SliceExpr(SliceExpr),
    #[default]
    Never,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IdentifierExprType {
    String(String),
    Identifier(Identifier),
    #[default]
    Never,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IdentifierExpr {
    pub target: Option<TargetDefinition>,
    pub idf: Identifier,
    pub annotated_type: Option<Box<AnnotatedTypeName>>,
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
    pub expr: LocationExpr,
    pub member: Identifier,
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
    pub arr: LocationExpr,
    pub key: Expression,
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
    pub arr: LocationExpr,
    pub base: Option<Expression>,
    pub start_offset: i32,
    pub size: i32,
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
impl AllExpr{
    pub fn new()->Self{
        Self{name:String::from("all")}
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
pub struct ReclassifyExpr {
    pub expr: Box<Expression>,
    pub privacy: Expression,
    pub homomorphism: Option<Homomorphism>,
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

pub type RehomExpr = ReclassifyExpr;

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
    pub homomorphism: Option<Homomorphism>,
    pub annotated_type: Option<AnnotatedTypeName>,
}
// class EncryptionExpression(ReclassifyExpr):
//     def __init__(self, expr: Expression, privacy: PrivacyLabelExpr, homomorphism: Homomorphism):
//         if isinstance(privacy, Identifier):
//             privacy = IdentifierExpr(privacy)
//         super().__init__(expr, privacy, homomorphism)
//         self.annotated_type = AnnotatedTypeName.cipher_type(expr.annotated_type, homomorphism)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Statement {
    before_analysis: Option<PartitionState<PrivacyLabelExpr>>,
    after_analysis: Option<PartitionState<PrivacyLabelExpr>>,
    function: Option<ConstructorOrFunctionDefinition>,
    pre_statements: Vec<CircuitInputStatement>,
}
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

pub type CircuitDirectiveStatement = Statement;
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

pub type BreakStatement = Statement;
// class BreakStatement(Statement):
//     pass

pub type ContinueStatement = Statement;
// class ContinueStatement(Statement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReturnStatement {
    pub expr: Expression,
}
// class ReturnStatement(Statement):

//     def __init__(self, expr: Expression):
//         super().__init__()
//         self.expr = expr

//     def process_children(self, f: Callable[[T], T]):
//         self.expr = f(self.expr)

pub type SimpleStatement = Statement;
// class SimpleStatement(Statement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExpressionStatement {
    pub expr: Expression,
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
    pub unmodified_code: Option<String>,
}
// class RequireStatement(SimpleStatement):

//     def __init__(self, condition: Expression, unmodified_code: Optional[str] = None):
//         super().__init__()
//         self.condition = condition
//         self.unmodified_code = self.code() if unmodified_code is None else unmodified_code

//     def process_children(self, f: Callable[[T], T]):
//         self.condition = f(self.condition)
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AssignmentStatementType {
    TupleExpr(TupleExpr),
    LocationExpr(LocationExpr),
    #[default]
    Never,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AssignmentStatement {
    pub lhs: AssignmentStatementType,
    pub rhs: Expression,
    pub op: Option<String>,
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

pub type CircuitInputStatement = AssignmentStatement;
// class CircuitInputStatement(AssignmentStatement):
//     pass

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StatementList {
    pub statements: Vec<Statement>,
    pub excluded_from_simulation: bool,
}

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
    FunctionTypeName(FunctionTypeName),
    Mapping(Box<Mapping>),
    UserDefinedTypeName(UserDefinedTypeName),
    ElementaryTypeName(ElementaryTypeName),
    NumberLiteralType(NumberLiteralType),
    BooleanLiteralType(BooleanLiteralType),
    String(String),
    #[default]
    Never,
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


#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ElementaryTypeName {
    pub name: String,
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
impl BooleanLiteralType{
    pub fn new(name:bool)->Self{
    let mut  name=name.to_string();
        name.make_ascii_lowercase();
    Self{name}
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
pub struct NumberTypeName {
    name: String,
    prefix: String,
    signed: bool,
    bitwidth: Option<i32>,
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
impl NumberLiteralType{
pub fn new(name:NumberLiteralTypeKind)->Self{
    let name=match name{NumberLiteralTypeKind::String(v)=>v.parse::<i32>().unwrap(),NumberLiteralTypeKind::I32(v)=>v};
    let blen=(i32::BITS-name.leading_zeros()) as i32;
    let (signed,mut bitwidth)=if name<0{
        (true,if name!=-(1<<(blen-1)){blen+1}else{blen})
    }else{(false,blen)};
    bitwidth=8i32.max((bitwidth+7)/8*8);
    assert!(bitwidth<=256);
    let name=name.to_string();
    let prefix=name.clone();
    Self{name,prefix,signed,bitwidth:Some(bitwidth) }
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
// class UintTypeName(NumberTypeName):
//     def __init__(self, name: str = 'uint'):
//         super().__init__(name, 'uint', False)

//     def implicitly_convertible_to(self, expected: TypeName) -> bool:
//         // Implicitly convert smaller uint types to larger uint types
//         return super().implicitly_convertible_to(expected) or (
//                 isinstance(expected, UintTypeName) and expected.elem_bitwidth >= self.elem_bitwidth)

//     def clone(self) -> UintTypeName:
//         return UintTypeName(self.name)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UserDefinedTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
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

pub type EnumTypeName = UserDefinedTypeName;
// class EnumTypeName(UserDefinedTypeName):
//     def clone(self) -> EnumTypeName:
//         return EnumTypeName(self.names.copy(), self.target)

//     @property
//     def elem_bitwidth(self):
//         return 256

pub type EnumValueTypeName = UserDefinedTypeName;
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

pub type StructTypeName = UserDefinedTypeName;
// class StructTypeName(UserDefinedTypeName):
//     def clone(self) -> StructTypeName:
//         return StructTypeName(self.names.copy(), self.target)

pub type ContractTypeName = UserDefinedTypeName;
// class ContractTypeName(UserDefinedTypeName):
//     def clone(self) -> ContractTypeName:
//         return ContractTypeName(self.names.copy(), self.target)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AddressTypeName {
    pub names: Vec<Identifier>,
    pub target: Option<NamespaceDefinition>,
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
    Never,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mapping {
    pub key_type: ElementaryTypeName,
    pub key_label: KeyLabelType,
    pub value_type: Box<AnnotatedTypeName>,
    pub instantiated_key: Option<Expression>,
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
    Never,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Array {
    pub value_type: AnnotatedTypeName,
    pub expr: ExprType,
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

pub type TupleType = TypeName;
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
// class FunctionTypeName(TypeName):
//     def __init__(self, parameters: List[Parameter], modifiers: List[str], return_parameters: List[Parameter]):
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
    type_name: Box<TypeName>,
    had_privacy_annotation: bool,
    privacy_annotation: Option<Expression>,
    homomorphism: String,
}
impl AnnotatedTypeName{
    pub fn  new(type_name:TypeName, 
    privacy_annotation: Option<Expression>,
    homomorphism: String,
    )->Self{

        Self{type_name:Box::new(type_name),had_privacy_annotation:privacy_annotation.is_some(),privacy_annotation:if privacy_annotation.is_some(){privacy_annotation}else{Some(Expression::AllExpr(AllExpr::new()))},homomorphism}
    }
}
// class AnnotatedTypeName(AST):

//     def __init__(self, type_name: TypeName, privacy_annotation: Optional[Expression] = None,
//                  homomorphism: Homomorphism = Homomorphism.NonHomomorphic):
//         super().__init__()
//         self.type_name = type_name
//         self.had_privacy_annotation = privacy_annotation is not None
//         if self.had_privacy_annotation:
//             self.privacy_annotation = privacy_annotation
//         else:
//             self.privacy_annotation = AllExpr()
//         self.homomorphism = homomorphism
//         if self.privacy_annotation == AllExpr() and homomorphism != Homomorphism.NonHomomorphic:
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
pub struct IdentifierDeclaration {
    keywords: Vec<String>,
    annotated_type: Box<AnnotatedTypeName>,
    idf: Identifier,
    storage_location: Option<String>,
}
// class IdentifierDeclaration(AST):

//     def __init__(self, keywords: List[str], annotated_type: AnnotatedTypeName, idf: Identifier, storage_location: Optional[str] = None):
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

pub type VariableDeclaration = IdentifierDeclaration;
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

//     def __init__(self, keywords: List[str], annotated_type: AnnotatedTypeName, idf: Identifier, storage_location: Optional[str] = None):
//         super().__init__(keywords, annotated_type, idf, storage_location)

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VariableDeclarationStatement {
    variable_declaration: VariableDeclaration,
    expr: Option<Expression>,
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

pub type Parameter = IdentifierDeclaration;
// class Parameter(IdentifierDeclaration):

//     def __init__(
//             self,
//             keywords: List[str],
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
pub struct NamespaceDefinition {
    idf: Identifier,
}
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
    name:String,
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
        assert!(idf.is_some() && idf.as_ref().unwrap().name != String::from("constructor") || return_parameters.is_none());
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
        let return_var_decls: Vec<_> = return_parameters
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
        Self{idf,parameters:parameters.unwrap(),modifiers:modifiers.unwrap(),return_parameters,body,return_var_decls,name:String::new(),parent:None, 
        original_body: None,
    annotated_type: None,
    called_functions: BTreeSet::new(),
    is_recursive: false,
    has_static_body: false,
    can_be_private: false,
    used_homomorphisms: None,
    used_crypto_backends: None,
    requires_verification: false,
    requires_verification_when_external: false,
    }
    }
}
pub trait ConstructorOrFunctionDefinitionAttr{
    fn  get_requires_verification_when_external(&self)->bool;
    fn get_name(&self)->String;
}
impl ConstructorOrFunctionDefinitionAttr for ConstructorOrFunctionDefinition{
    fn  get_requires_verification_when_external(&self)->bool{
    self.requires_verification_when_external}
    fn get_name(&self)->String{
    self.name.clone()
        }
}
// class ConstructorOrFunctionDefinition(NamespaceDefinition):

//     def __init__(self, idf: Optional[Identifier], parameters: List[Parameter], modifiers: List[str], return_parameters: Optional[List[Parameter]], body: Block):
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
    keywords: Vec<String>,
    annotated_type: Box<AnnotatedTypeName>,
    idf: Identifier,
    storage_location: Option<String>,
    expr: Option<Expression>,
}

impl StateVariableDeclaration {
    fn is_final(&self) -> bool {
        self.keywords.contains(&String::from("final"))
    }
    fn is_constant(&self) -> bool {
        self.keywords.contains(&String::from("constant"))
    }
}
// class StateVariableDeclaration(IdentifierDeclaration):

//     def __init__(self, annotated_type: AnnotatedTypeName, keywords: List[str], idf: Identifier, expr: Optional[Expression]):
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
impl EnumValue{
    pub fn new(idf: Option<Identifier>)->Self{
        Self{idf,annotated_type:None}
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
impl EnumDefinition{
    pub fn new(idf: Option<Identifier>,values: Vec<EnumValue>)->Self{
        Self{idf,values,annotated_type:None}
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
    pub struct_definitions: Option<Vec<StructDefinition>>,
    pub used_crypto_backends: Option<Vec<CryptoParams>>,
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
}
// class SourceUnit(AST):

//     def __init__(self, pragma_directive: str, contracts: List[ContractDefinition], used_contracts: Optional[List[str]] = None):
//         super().__init__()
//         self.pragma_directive = pragma_directive
//         self.contracts = contracts
//         self.used_contracts = [] if used_contracts is None else used_contracts
//         self.used_homomorphisms: Optional[Set[Homomorphism]] = None
//         self.used_crypto_backends: Optional[List[CryptoParams]] = None

//         self.original_code: List[str] = []

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
    Never,
}
// PrivacyLabelExpr = Union[MeExpr, AllExpr, Identifier]
// TargetDefinition = Union[IdentifierDeclaration, NamespaceDefinition]
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TargetDefinition {
    IdentifierDeclaration(IdentifierDeclaration),
    NamespaceDefinition(NamespaceDefinition),
    #[default]
    Never,
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
    LocationExpr(Location),
    #[default]
    Never,
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

// def indent(s: str):
//     return textwrap.indent(s, cfg.indentation)

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

// class CodeVisitor(AstVisitor):

//     def __init__(self, display_final=True):
//         super().__init__('node-or-children')
//         self.display_final = display_final

//     def visit_list(self, l: List[Union[AST, str]], sep=' '):
//         if l is None:
//             return 'None'

//         def handle(e: Union[AST, str]):
//             if isinstance(e, str):
//                 return e
//             else:
//                 return self.visit(e)

//         s = filter(None.__ne__, [handle(e) for e in l])
//         s = sep.join(s)
//         return s

//     def visit_single_or_list(self, v: Union[List[AST], AST, str], sep=' '):
//         if isinstance(v, List):
//             return self.visit_list(v, sep)
//         elif isinstance(v, str):
//             return v
//         else:
//             return self.visit(v)

//     def visitAST(self, ast: AST):
//         // should never be called
//         raise NotImplementedError("Did not implement code generation for " + repr(ast))

//     def visitComment(self, ast: Comment):
//         if ast.text == '':
//             return ''
//         elif ast.text.find(' ') != -1:
//             return f'/* {ast.text} */'
//         else:
//             return f'// {ast.text}'

//     def visitIdentifier(self, ast: Identifier):
//         return ast.name

//     def visitFunctionCallExpr(self, ast: FunctionCallExpr):
//         if isinstance(ast.func, BuiltinFunction):
//             args = [self.visit(a) for a in ast.args]
//             return ast.func.format_string().format(*args)
//         else:
//             f = self.visit(ast.func)
//             a = self.visit_list(ast.args, ', ')
//             return f'{f}({a})'

//     def visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr):
//         if ast.is_implicit:
//             return self.visit(ast.expr)
//         else:
//             return f'{self.visit(ast.elem_type)}({self.visit(ast.expr)})'

//     def visitBooleanLiteralExpr(self, ast: BooleanLiteralExpr):
//         return str(ast.value).lower()

//     def visitNumberLiteralExpr(self, ast: NumberLiteralExpr):
//         return hex(ast.value) if ast.was_hex else str(ast.value)

//     def visitStringLiteralExpr(self, ast: StringLiteralExpr):
//         return f'\'{ast.value}\''

//     def visitArrayLiteralExpr(self, ast: ArrayLiteralExpr):
//         return f'[{self.visit_list(ast.values, sep=", ")}]'

//     def visitTupleExpr(self, ast: TupleExpr):
//         return f'({self.visit_list(ast.elements, sep=", ")})'

//     def visitIdentifierExpr(self, ast: IdentifierExpr):
//         return self.visit(ast.idf)

//     def visitMemberAccessExpr(self, ast: MemberAccessExpr):
//         return f'{self.visit(ast.expr)}.{self.visit(ast.member)}'

//     def visitIndexExpr(self, ast: IndexExpr):
//         return f'{self.visit(ast.arr)}[{self.visit(ast.key)}]'

//     def visitMeExpr(self, _: MeExpr):
//         return 'me'

//     def visitAllExpr(self, _: AllExpr):
//         return 'all'

//     def visitReclassifyExpr(self, ast: ReclassifyExpr):
//         e = self.visit(ast.expr)
//         p = self.visit(ast.privacy)
//         h = ast.homomorphism or ''
//         return f'reveal{h}({e}, {p})'

//     def visitRehomExpr(self, ast: RehomExpr):
//         e = self.visit(ast.expr)
//         return f'{ast.func_name()}({e})'

//     def visitIfStatement(self, ast: IfStatement):
//         c = self.visit(ast.condition)
//         t = self.visit_single_or_list(ast.then_branch)
//         ret = f'if ({c}) {t}'
//         if ast.else_branch:
//             e = self.visit_single_or_list(ast.else_branch)
//             ret += f'  else {e}'
//         return ret

//     def visitWhileStatement(self, ast: WhileStatement):
//         c = self.visit(ast.condition)
//         b = self.visit_single_or_list(ast.body)
//         ret = f'while ({c}) {b}'
//         return ret

//     def visitDoWhileStatement(self, ast: DoWhileStatement):
//         b = self.visit_single_or_list(ast.body)
//         c = self.visit(ast.condition)
//         ret = f'do {b} while ({c});'
//         return ret

//     def visitForStatement(self, ast: ForStatement):
//         i = ';' if ast.init is None else f'{self.visit_single_or_list(ast.init)}'
//         c = self.visit(ast.condition)
//         u = '' if ast.update is None else f' {self.visit_single_or_list(ast.update).replace(";", "")}'
//         b = self.visit_single_or_list(ast.body)
//         ret = f'for ({i} {c};{u}) {b}'
//         return ret

//     def visitBreakStatement(self, _: BreakStatement):
//         return 'break;'

//     def visitContinueStatement(self, _: ContinueStatement):
//         return 'continue;'

//     def visitReturnStatement(self, ast: ReturnStatement):
//         if ast.expr:
//             e = self.visit(ast.expr)
//             return f'return {e};'
//         else:
//             return 'return;'

//     def visitExpressionStatement(self, ast: ExpressionStatement):
//         return self.visit(ast.expr) + ';'

//     def visitRequireStatement(self, ast: RequireStatement):
//         c = self.visit(ast.condition)
//         return f'require({c});'

//     def visitAssignmentStatement(self, ast: AssignmentStatement):
//         lhs = ast.lhs
//         op = ast.op
//         if ast.lhs.annotated_type is not None and ast.lhs.annotated_type.is_private():
//             op = ''
//         rhs = ast.rhs.args[1] if op else ast.rhs

//         if op.startswith('pre'):
//             op = op[3:]
//             fstr = '{1}{0};'
//         elif op.startswith('post'):
//             op = op[4:]
//             fstr = '{0}{1};'
//         else:
//             fstr = '{} {}= {};'

//         if isinstance(lhs, SliceExpr) and isinstance(rhs, SliceExpr):
//             assert lhs.size == rhs.size, "Slice ranges don't have same size"
//             s = ''
//             lexpr, rexpr = self.visit(lhs.arr), self.visit(rhs.arr)
//             lbase = '' if lhs.base is None else f'{self.visit(lhs.base)} + '
//             rbase = '' if rhs.base is None else f'{self.visit(rhs.base)} + '
//             if lhs.size <= 3:
//                 for i in range(lhs.size):
//                     s += fstr.format(f'{lexpr}[{lbase}{lhs.start_offset + i}]', op,
//                                      f'{rexpr}[{rbase}{rhs.start_offset + i}]') + ' '
//             else:
//                 i = cfg.reserved_name_prefix + 'i'
//                 if lhs.start_offset != 0:
//                     lbase += f'{lhs.start_offset} + '
//                 if rhs.start_offset != 0:
//                     rbase += f'{rhs.start_offset} + '
//                 s += f'for (uint {i} = 0; {i} < {lhs.size}; ++{i}) {{ '
//                 s += indent(fstr.format(f'{lexpr}[{lbase}{i}]', op, f'{rexpr}[{rbase}{i}]')) + ' '
//                 s += '} '
//             return s[:-1]
//         else:
//             lhs = self.visit(lhs)
//             rhs = self.visit(rhs)
//             return fstr.format(lhs, op, rhs)

//     def visitCircuitDirectiveStatement(self, ast: CircuitDirectiveStatement):
//         return None

//     def handle_block(self, ast: StatementList):
//         s = self.visit_list(ast.statements)
//         s = indent(s)
//         return s

//     def visitStatementList(self, ast: StatementList):
//         return self.visit_list(ast.statements)

//     def visitBlock(self, ast: Block):
//         b = self.handle_block(ast).rstrip()
//         if ast.was_single_statement and len(ast.statements) == 1:
//             return b
//         else:
//             return f'{{ {b} }}'

//     def visitIndentBlock(self, ast: IndentBlock):
//         return self.handle_block(ast)

//     def visitElementaryTypeName(self, ast: ElementaryTypeName):
//         return ast.name

//     def visitUserDefinedTypeName(self, ast: UserDefinedTypeName):
//         return self.visit_list(ast.names, '.')

//     def visitAddressTypeName(self, ast: AddressTypeName):
//         return 'address'

//     def visitAddressPayableTypeName(self, ast: AddressPayableTypeName):
//         return 'address payable'

//     def visitAnnotatedTypeName(self, ast: AnnotatedTypeName):
//         t = self.visit(ast.type_name)
//         p = self.visit(ast.privacy_annotation)
//         if ast.had_privacy_annotation:
//             return f'{t}@{p}{ast.homomorphism}'
//         return t

//     def visitMapping(self, ast: Mapping):
//         k = self.visit(ast.key_type)
//         if isinstance(ast.key_label, Identifier):
//             label = '!' + self.visit(ast.key_label)
//         else:
//             label = f'/*!{ast.key_label}*/' if ast.key_label is not None else ''
//         v = self.visit(ast.value_type)
//         return f"mapping({k}{label} => {v})"

//     def visitArray(self, ast: Array):
//         t = self.visit(ast.value_type)
//         if ast.expr is not None:
//             e = self.visit(ast.expr)
//         else:
//             e = ''
//         return f'{t}[{e}]'

//     def visitCipherText(self, ast: CipherText):
//         e = self.visitArray(ast)
//         return f'{e}/*{ast.plain_type.code()}*/'

//     def visitTupleType(self, ast: TupleType):
//         s = self.visit_list(ast.types, ', ')
//         return f'({s})'

//     def visitVariableDeclaration(self, ast: VariableDeclaration):
//         keywords = [k for k in ast.keywords if self.display_final or k != 'final']
//         k = ' '.join(keywords)
//         t = self.visit(ast.annotated_type)
//         s = '' if ast.storage_location is None else f' {ast.storage_location}'
//         i = self.visit(ast.idf)
//         return f'{k} {t}{s} {i}'.strip()

//     def visitVariableDeclarationStatement(self, ast: VariableDeclarationStatement):
//         s = self.visit(ast.variable_declaration)
//         if ast.expr:
//             s += ' = ' + self.visit(ast.expr)
//         s += ';'
//         return s

//     def visitParameter(self, ast: Parameter):
//         if not self.display_final:
//             f = None
//         else:
//             f = 'final' if 'final' in ast.keywords else None
//         t = self.visit(ast.annotated_type)
//         if ast.idf is None:
//             i = None
//         else:
//             i = self.visit(ast.idf)

//         description = [f, t, ast.storage_location, i]
//         description = [d for d in description if d is not None]
//         s = ' '.join(description)
//         return s

//     def visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition):
//         b = self.visit_single_or_list(ast.body)
//         return self.function_definition_to_str(ast.idf, ast.parameters, ast.modifiers, ast.return_parameters, b)

//     def function_definition_to_str(
//             self,
//             idf: Identifier,
//             parameters: List[Union[Parameter, str]],
//             modifiers: List[str],
//             return_parameters: List[Parameter],
//             body: str):
//         if idf.name != 'constructor':
//             i = self.visit(idf)
//             definition = f'function {i}'
//         else:
//             definition = 'constructor'
//         p = self.visit_list(parameters, ', ')
//         m = ' '.join(modifiers)
//         if m != '':
//             m = f' {m}'
//         r = self.visit_list(return_parameters, ', ')
//         if r != '':
//             r = f' returns ({r})'

//         f = f"{definition}({p}){m}{r} {body}"
//         return f

//     def visitEnumValue(self, ast: EnumValue):
//         return self.visit(ast.idf)

//     def visitEnumDefinition(self, ast: EnumDefinition):
//         values = self.visit_list(ast.values, sep=', ')
//         return f'enum {self.visit(ast.idf)} {{ {indent(values)} }}'

//     @staticmethod
//     def __cmp_type_size(v1: VariableDeclaration, v2: VariableDeclaration):
//         t1, t2 = v1.annotated_type.type_name, v2.annotated_type.type_name
//         cmp = (t1.size_in_uints > t2.size_in_uints) - (t1.size_in_uints < t2.size_in_uints)
//         if cmp == 0:
//             cmp = (t1.elem_bitwidth > t2.elem_bitwidth) - (t1.elem_bitwidth < t2.elem_bitwidth)
//         return cmp

//     def visitStructDefinition(self, ast: StructDefinition):
//         // Define struct with members in order of descending size (to get maximum space savings through packing)
//         members_by_descending_size = sorted(ast.members, key=cmp_to_key(self.__cmp_type_size), reverse=True)

//         body = ' '.join([f'{self.visit(member)};' for member in members_by_descending_size])
//         return f'struct {self.visit(ast.idf)} {{ {indent(body)} }}'

//     def visitStateVariableDeclaration(self, ast: StateVariableDeclaration):
//         keywords = [k for k in ast.keywords if self.display_final or k != 'final']
//         f = 'final ' if 'final' in keywords else ''
//         t = self.visit(ast.annotated_type)
//         k = ' '.join([k for k in keywords if k != 'final'])
//         if k != '':
//             k = f'{k} '
//         i = self.visit(ast.idf)
//         ret = f"{f}{t} {k}{i}".strip()
//         if ast.expr:
//             ret += ' = ' + self.visit(ast.expr)
//         return ret + ';'

//     @staticmethod
//     def contract_definition_to_str(
//             idf: Identifier,
//             state_vars: List[str],
//             constructors: List[str],
//             functions: List[str],
//             enums: List[str],
//             structs: List[str]):

//         i = str(idf)
//         structs = '  '.join(structs)
//         enums = '  '.join(enums)
//         state_vars = ' '.join(state_vars)
//         constructors = '  '.join(constructors)
//         functions = '  '.join(functions)
//         body = '  '.join(filter(''.__ne__, [structs, enums, state_vars, constructors, functions]))
//         body = indent(body)
//         return f"contract {i} {{ {body} }}"

//     def visitContractDefinition(self, ast: ContractDefinition):
//         state_vars = [self.visit(e) for e in ast.state_variable_declarations]
//         constructors = [self.visit(e) for e in ast.constructor_definitions]
//         functions = [self.visit(e) for e in ast.function_definitions]
//         enums = [self.visit(e) for e in ast.enum_definitions]
//         structs = [self.visit(e) for e in ast.struct_definitions]

//         return self.contract_definition_to_str(
//             ast.idf,
//             state_vars,
//             constructors,
//             functions,
//             enums,
//             structs)

//     def handle_pragma(self, pragma: str) -> str:
//         return pragma

//     def visitSourceUnit(self, ast: SourceUnit):
//         p = self.handle_pragma(ast.pragma_directive)
//         contracts = self.visit_list(ast.contracts)
//         lfstr = 'import "{}";'
//         return '  '.join(filter(''.__ne__, [p, linesep.join([lfstr.format(uc) for uc in ast.used_contracts]), contracts]))
