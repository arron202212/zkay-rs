#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// """
// This module defines the different types of abstract circuit statements, which
// are collected by CircuitHelper during the code transformation.

// CircuitStatements specify high-level semantics of a zk-snark circuit.
// They can be compiled to concrete circuit-representations (e.g. java code for jsnark) by a CircuitGenerator.

// To add support for a new zero knowledge backend, one has to implement a CircuitGenerator subclass which provides functionality
// to convert all the different statement types in this file, as well as all the different AST expressions which are allowed inside
// private expressions into a backend specific format.

// To simplify compilation, CircVarDecl is the only CircuitStatement type which can contain an AST Expression.
// All the other statement types operate only on HybridArgumentIdfs, which reference either circuit inputs or temporary circuit variables.
// Thus, when you have e.g. an assignment a@me = b + c, this will generate the circuit statements
// CircVarDecl(new_temporary_idf, phi(b + c))
// CircEncConstraint(new_temporary_idf, rnd, pk, secret_input_enc_b_plus_c)

// Additionally, abstract circuits use static-single assignment, which means that any HybridArgumentIdf can be regarded as a final variable.
// (That's why it is called CircVarDecl rather than CircAssign)
// """
use crate::ast::{
    ASTChildren, ASTFlatten, ASTInstanceOf, ASTType, ChildListBuilder,
    ConstructorOrFunctionDefinition, Expression, HybridArgumentIdf, IntoAST, Statement, AST,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIs, EnumTryAs};
use zkay_derive::{impl_trait, impl_traits, ASTKind, ImplBaseTrait};
// class CircuitStatement(metaclass=ABCMeta)
// pass
#[enum_dispatch(IntoAST, IntoASTFlatten, ASTInstanceOf)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]

pub enum CircuitStatement {
    CircComment(CircComment),
    CircIndentBlock(CircIndentBlock),
    CircCall(CircCall),
    CircVarDecl(CircVarDecl),
    CircGuardModification(CircGuardModification),
    CircEncConstraint(CircEncConstraint),
    CircSymmEncConstraint(CircSymmEncConstraint),
    CircEqConstraint(CircEqConstraint),
}
// impl IntoASTFlatten for CircuitStatement {
//     fn to_ast_flatten<'a>(&'a mut self) -> ASTFlatten<'a> {
//         ASTFlatten::CircuitStatement(self)
//     }
// }
// impl IntoAST for CircuitStatement {
//     fn into_ast(self) -> AST {
//         match self {
//             Self::CircComment(ast) => ast.to_ast(),
//             Self::CircIndentBlock(ast) => ast.to_ast(),
//             Self::CircCall(ast) => ast.to_ast(),
//             Self::CircVarDecl(ast) => ast.to_ast(),
//             Self::CircGuardModification(ast) => ast.to_ast(),
//             Self::CircEncConstraint(ast) => ast.to_ast(),
//             Self::CircSymmEncConstraint(ast) => ast.to_ast(),
//             Self::CircEqConstraint(ast) => ast.to_ast(),
//         }
//     }
// }
// impl ASTInstanceOf for CircuitStatement {
//     fn get_ast_type(&self) -> ASTType {
//         match self {
//             Self::CircComment(ast) => ast.get_ast_type(),
//             Self::CircIndentBlock(ast) => ast.get_ast_type(),
//             Self::CircCall(ast) => ast.get_ast_type(),
//             Self::CircVarDecl(ast) => ast.get_ast_type(),
//             Self::CircGuardModification(ast) => ast.get_ast_type(),
//             Self::CircEncConstraint(ast) => ast.get_ast_type(),
//             Self::CircSymmEncConstraint(ast) => ast.get_ast_type(),
//             Self::CircEqConstraint(ast) => ast.get_ast_type(),
//         }
//     }
// }
impl ASTChildren for CircuitStatement {
    fn process_children(&mut self, _cb: &mut ChildListBuilder) {}
}
// impl ASTChildrenMut for CircuitStatement {
//     fn process_children_mut<'a>(&'a mut self, _cb: &mut Vec<ASTFlatten<'a>>) {}
// }
// class CircComment(CircuitStatement)
// """
// A textual comment, has no impact on circuit semantics (meta statement)

// Implementing transformation for CircComment is recommended (for output readability) but you can also skip them.
// """

// def __init__(self, text: str)
//     super().__init__()
//     self.text = text
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircComment {
    pub text: String,
}
impl IntoAST for CircComment {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(CircuitStatement::CircComment(
            self,
        )))
    }
}
// impl ASTInstanceOf for CircComment {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircComment
//     }
// }
impl CircComment {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

// class CircIndentBlock(CircuitStatement)
// """
// A named block of statements (container meta statement)

// Implementing special transformation for CircIndentBlock itself is recommended (for output readability) but not required.
// In either case, don't forget to include the contained statements in the transformed output!
// """

// def __init__(self, name: str, statements: List[CircuitStatement])
//     super().__init__()
//     self.name = name
//     self.statements = statements
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircIndentBlock {
    pub name: String,
    pub statements: Vec<CircuitStatement>,
}
impl IntoAST for CircIndentBlock {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(
            CircuitStatement::CircIndentBlock(self),
        ))
    }
}
// impl ASTInstanceOf for CircIndentBlock {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircIndentBlock
//     }
// }
impl CircIndentBlock {
    pub fn new(name: String, statements: Vec<CircuitStatement>) -> Self {
        Self { name, statements }
    }
}

// class CircCall(CircuitStatement)
// """
// Represents a public function call to a function which requires verification

// This statement is only used for function calls OUTSIDE private expressions
// (as function calls INSIDE private expressions are always fully inlined).
// The generated public solidity code will also contain this function call.

// The semantics of this statement can be described as:
// "Include all the circuit statements which are generated by the body of the called function such that all circuit inputs and outputs
// which are generated by those statements are not shared between different invocations unless they are provably equivalent."

// IMPORTANT:
// It is up to the backend implementation to ensure that fresh inputs and outputs are generated for each function invocation.
// E.g. if we have a function a(x) { k@me = x }
// if we have a(1), a(2), which leads to  CircCall(a); CircCall(a).
// Then the second CircCall must generate a fresh circuit input for x and a fresh circuit output for k.

// In the case of the jsnark backend, the java code of the jsnark wrapper handles this complexity internally by creating different
// namespaces for different function invocations, which simplifies the visible part of the java circuit representation
// and also the Jsnark Circuit Generator.
// """

// def __init__(self, fct: ConstructorOrFunctionDefinition)
//     super().__init__()
//     self.fct = fct
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircCall {
    pub fct: ConstructorOrFunctionDefinition,
}
impl IntoAST for CircCall {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(CircuitStatement::CircCall(
            self,
        )))
    }
}

// impl ASTInstanceOf for CircCall {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircCall
//     }
// }
impl CircCall {
    pub fn new(fct: ConstructorOrFunctionDefinition) -> Self {
        Self { fct }
    }
}

// class CircVarDecl(CircuitStatement)
// """
// Represents an assignment of a private expression to a temporary circuit variable

// The circuit generator must ensure that the statements which come after this have access to the result of expr
// under the name lhs
// """

// def __init__(self, lhs: HybridArgumentIdf, expr: Expression)
//     super().__init__()
//     self.lhs = lhs
//     self.expr = expr
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircVarDecl {
    pub lhs: HybridArgumentIdf,
    pub expr: Expression,
}

impl IntoAST for CircVarDecl {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(CircuitStatement::CircVarDecl(
            self,
        )))
    }
}
// impl ASTInstanceOf for CircVarDecl {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircVarDecl
//     }
// }
impl CircVarDecl {
    pub fn new(lhs: HybridArgumentIdf, expr: Expression) -> Self {
        Self { lhs, expr }
    }
}

// class CircGuardModification(CircuitStatement)
// """
// Enters and leaves scopes protected by a guard condition (e.g. if statement blocks protected by if condition)

// A guard scope starts with a CircGuardModification(guard_cond, is_true) statements,
// and ends with a corresponding CircGuardModification(None) statement
// (like a stack, CircGuardModification(None) always ends the last opened scope)

// The circuit generator must ensure that any assertion statement ASSERT(COND) which is added inside a guarded scope (guard_var, is_true),
// is transformed into ``ASSERT((previous_guard_constraint && (guard_var == is_true)) => COND)``

// The circuit generator must also ensure that any CircVarDecl(var, expr) which corresponds to a real assignment to a variable
// (i.e. an AssignmentStatement inside a private expression (function call) or if statement),
// is transformed from ``var = expr`` into ``var = (previous_guard_constraint && (guard_var == is_true)) ? expr : var``
// """

// def __init__(self, new_cond: Optional[HybridArgumentIdf], is_true: Optional[bool] = None)
//     super().__init__()
//     self.new_cond = new_cond
//     self.is_true = is_true
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircGuardModification {
    pub new_cond: Option<HybridArgumentIdf>,
    pub is_true: Option<bool>,
}
impl IntoAST for CircGuardModification {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(
            CircuitStatement::CircGuardModification(self),
        ))
    }
}
// impl ASTInstanceOf for CircGuardModification {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircGuardModification
//     }
// }
impl CircGuardModification {
    pub fn new(new_cond: Option<HybridArgumentIdf>, is_true: Option<bool>) -> Self {
        Self { new_cond, is_true }
    }

    // @staticmethod
    // @contextmanager
    pub fn guarded(phi: &mut Vec<CircuitStatement>, guard_idf: HybridArgumentIdf, is_true: bool)
    // """
    // Return a context manager which manages the lifetime of a guarded scope.

    // :param phi: list which stores all circuit statements for a particular circuit
    // :param guard_idf: HybridArgumentIdf which references the guard condition
    // :param is_true: assertions and assignments inside the guarded scope are ignored unless guard_idf is equal to is_true at
    //                 proof generation time
    // """
    {
        phi.push(CircuitStatement::CircGuardModification(
            CircGuardModification::new(Some(guard_idf), Some(is_true)),
        ));
        // yield
        phi.push(CircuitStatement::CircGuardModification(
            CircGuardModification::new(None, None),
        ));
    }
}
// class CircEncConstraint(CircuitStatement)
// """
// Depending on is_dec, either represents an encryption or a decryption constraint

// Both types are generally modelled using the constraint ``enc(plain, pk, rnd) == cipher``

// IMPORTANT FOR SECURITY:
// To support solidity's default initialization semantics for encrypted variables, a cipher value of 0 is always decrypted to
// the plain value 0. To ensure correctness, the circuit should thus reject user-supplied (private input) cipher values equal to 0.
// (When a legitimate encryption operation produces cipher = 0 during simulation (extremely unlikely), it should be repeated with a different randomness)

// For encryption (user supplies the cipher text as private input)
//  => the generated circuit must prove that enc(plain, pk, rnd) == cipher AND that cipher != 0
// For decryption (user supplies the plain text as private input)
//  => the generated circuit must prove that enc(plain, pk, rnd) == cipher OR (cipher == 0 AND plain == 0 AND rnd == 0)
// """

// def __init__(self, plain: HybridArgumentIdf, rnd: HybridArgumentIdf, pk: HybridArgumentIdf, cipher: HybridArgumentIdf, is_dec: bool)
//     super().__init__()
//     self.plain = plain
//     self.rnd = rnd
//     self.pk = pk
//     self.cipher = cipher
//     self.is_dec = is_dec # True if this is an inverted decryption
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircEncConstraint {
    pub plain: HybridArgumentIdf,
    pub rnd: HybridArgumentIdf,
    pub pk: HybridArgumentIdf,
    pub cipher: HybridArgumentIdf,
    pub is_dec: bool,
}
impl IntoAST for CircEncConstraint {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(
            CircuitStatement::CircEncConstraint(self),
        ))
    }
}
// impl ASTInstanceOf for CircEncConstraint {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircEncConstraint
//     }
// }
impl CircEncConstraint {
    pub fn new(
        plain: HybridArgumentIdf,
        rnd: HybridArgumentIdf,
        pk: HybridArgumentIdf,
        cipher: HybridArgumentIdf,
        is_dec: bool,
    ) -> Self {
        Self {
            plain,
            rnd,
            pk,
            cipher,
            is_dec,
        }
    }
}

// class CircSymmEncConstraint(CircuitStatement)
// """
// ECDH+Symmetric encryption constraint

// Verifies that:
// iv_cipher == enc(plain, ecdh(other_pk, my_sk), iv)

// (circuit also verifies globally that my_pk = DeriveEcPk(my_sk))
// """

// def __init__(self, plain: HybridArgumentIdf, other_pk: HybridArgumentIdf, iv_cipher: HybridArgumentIdf, is_dec: bool)
//     super().__init__()
//     self.plain = plain
//     self.other_pk = other_pk
//     self.iv_cipher = iv_cipher
//     self.is_dec = is_dec # True if this is an inverted decryption
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircSymmEncConstraint {
    pub plain: HybridArgumentIdf,
    pub other_pk: HybridArgumentIdf,
    pub iv_cipher: HybridArgumentIdf,
    pub is_dec: bool,
}
impl IntoAST for CircSymmEncConstraint {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(
            CircuitStatement::CircSymmEncConstraint(self),
        ))
    }
}
// impl ASTInstanceOf for CircSymmEncConstraint {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircSymmEncConstraint
//     }
// }
impl CircSymmEncConstraint {
    pub fn new(
        plain: HybridArgumentIdf,
        other_pk: HybridArgumentIdf,
        iv_cipher: HybridArgumentIdf,
        is_dec: bool,
    ) -> Self {
        Self {
            plain,
            other_pk,
            iv_cipher,
            is_dec,
        }
    }
}

// class CircEqConstraint(CircuitStatement)
// """
// Represents a simple equality constraint
// """

// def __init__(self, tgt: HybridArgumentIdf, val: HybridArgumentIdf)
//     super().__init__()
//     self.tgt = tgt
//     self.val = val
#[derive(ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CircEqConstraint {
    pub tgt: HybridArgumentIdf,
    pub val: HybridArgumentIdf,
}
impl IntoAST for CircEqConstraint {
    fn into_ast(self) -> AST {
        AST::Statement(Statement::CircuitStatement(
            CircuitStatement::CircEqConstraint(self),
        ))
    }
}
// impl ASTInstanceOf for CircEqConstraint {
//     fn get_ast_type(&self) -> ASTType {
//         ASTType::CircEqConstraint
//     }
// }
impl CircEqConstraint {
    pub fn new(tgt: HybridArgumentIdf, val: HybridArgumentIdf) -> Self {
        Self { tgt, val }
    }
}
