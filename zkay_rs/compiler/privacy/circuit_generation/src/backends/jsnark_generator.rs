#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// """Circuit Generator implementation for the jsnark backend"""

// import os
// from typing import List, Optional, Union, Tuple
use crate::circuit_generator::{CircuitGenerator, CircuitGeneratorBase, VerifyingKeyType};
use circuit_helper::circuit_helper::CircuitHelper;
use proving_scheme::backends::{gm17::ProvingSchemeGm17, groth16::ProvingSchemeGroth16};
use proving_scheme::proving_scheme::{G1Point, G2Point, ProvingScheme, VerifyingKeyMeta};
use rccell::{RcCell, WeakCell};
use zkay_ast::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};

use jsnark_interface::jsnark_interface as jsnark;
use jsnark_interface::libsnark_interface as libsnark;
use std::any::{Any, TypeId};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;
use zkay_ast::ast::{
    indent, is_instance, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, ArrayBaseProperty,
    BooleanLiteralExpr, BuiltinFunction, EnumDefinition, Expression, ExpressionBaseProperty,
    FunctionCallExpr, FunctionCallExprBaseProperty, HybridArgumentIdf, IdentifierBaseProperty,
    IdentifierExpr, IndexExpr, IntoAST, MeExpr, MemberAccessExpr, NumberLiteralExpr,
    PrimitiveCastExpr, TypeName, AST,
};
use zkay_ast::homomorphism::Homomorphism;
use zkay_ast::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_config::{config::CFG, zk_print};
use zkay_derive::ASTVisitorBaseRefImpl;
use zkay_utils::helpers::{hash_file, hash_string};
use zkay_utils::helpers::{read_file, save_to_file};
use zkp_u256::Binary;

pub fn is_type_id_of<S: ?Sized + Any>(s: TypeId) -> bool {
    TypeId::of::<S>() == s
}

// """Return the corresponding jsnark type name for a given type or expression."""
pub fn _get_t(mut t: Option<ASTFlatten>) -> String {
    let t = t.and_then(|t| {
        if t.is_expression() {
            t.ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone()
        } else {
            Some(t)
        }
    });
    assert!(t.is_some());
    let t = t.unwrap();
    let bits = t.to_ast().try_as_type_name().unwrap().elem_bitwidth();
    if bits == 1 {
        return String::from("ZkBool");
    }
    if t.to_ast().try_as_type_name().unwrap().is_signed_numeric() {
        format!(r#"ZkInt({bits})"#)
    } else {
        format!(r#"ZkUint({bits})"#)
    }
}

// class JsnarkVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct JsnarkVisitor {
    pub ast_visitor_base: AstVisitorBase,
    phi: Vec<RcCell<CircuitStatement>>,
}
impl AstVisitor for JsnarkVisitor {
    type Return = String;
    fn temper_result(&self) -> Self::Return {
        String::new()
    }
    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
        matches!(
            name,
            ASTType::CircComment
                | ASTType::CircIndentBlock
                | ASTType::CircCall
                | ASTType::CircVarDecl
                | ASTType::CircEqConstraint
                | ASTType::CircEncConstraint
                | ASTType::CircSymmEncConstraint
                | ASTType::CircGuardModification
                | ASTType::BooleanLiteralExpr
                | ASTType::NumberLiteralExpr
                | ASTType::IdentifierExpr
                | ASTType::MemberAccessExpr
                | ASTType::IndexExpr
                | ASTType::FunctionCallExprBase
                | ASTType::PrimitiveCastExpr
        ) || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            // ASTType::Circuit=>self.visitCircuit(ast),
            ASTType::CircComment => self.visitCircComment(ast),
            ASTType::CircIndentBlock => self.visitCircIndentBlock(ast),
            ASTType::CircCall => self.visitCircCall(ast),
            ASTType::CircVarDecl => self.visitCircVarDecl(ast),
            ASTType::CircEqConstraint => self.visitCircEqConstraint(ast),
            ASTType::CircEncConstraint => self.visitCircEncConstraint(ast),
            ASTType::CircSymmEncConstraint => self.visitCircSymmEncConstraint(ast),
            ASTType::CircGuardModification => self.visitCircGuardModification(ast),
            ASTType::BooleanLiteralExpr => self.visitBooleanLiteralExpr(ast),
            ASTType::NumberLiteralExpr => self.visitNumberLiteralExpr(ast),
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
// """Visitor which compiles CircuitStatements and Expressions down to java code compatible with a custom jsnark wrapper."""
impl JsnarkVisitor {
    pub fn new(phi: Vec<RcCell<CircuitStatement>>) -> Self {
        // super().__init__("node-or-children", false)
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
            phi,
        }
    }
    pub fn visitCircuit(&self) -> Vec<String> {
        self.phi
            .iter()
            .map(|constr| self.visit(&constr.clone().into()))
            .collect()
    }

    pub fn visitCircComment(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitCircComment====={:?}",stmt);
        Ok(
            if !stmt
                .try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_comment_ref()
                .unwrap()
                .text
                .is_empty()
            {
                format!(
                    r#"// {}"#,
                    stmt.try_as_circuit_statement_ref()
                        .unwrap()
                        .borrow()
                        .try_as_circ_comment_ref()
                        .unwrap()
                        .text
                )
            } else {
                String::new()
            },
        )
    }

    pub fn visitCircIndentBlock(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitCircIndentBlock====={:?}",stmt);
        let stmts: Vec<_> = stmt
            .try_as_circuit_statement_ref()
            .unwrap()
            .borrow()
            .try_as_circ_indent_block_ref()
            .unwrap()
            .statements
            .iter()
            .map(|s| self.visit(&s.clone().into()))
            .collect();
        Ok(
            if !stmt
                .try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_indent_block_ref()
                .unwrap()
                .name
                .is_empty()
            {
                format!(
                    r#"//[ --- {name} ---\n {} \n //] --- {name} ---\n"#,
                    indent(stmts.join("\n")),
                    name = stmt
                        .try_as_circuit_statement_ref()
                        .unwrap()
                        .borrow()
                        .try_as_circ_indent_block_ref()
                        .unwrap()
                        .name
                )
            } else {
                indent(stmts.join("\n"))
            },
        )
    }

    pub fn visitCircCall(&self, stmt: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            r#"_{}();"#,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_call_ref()
                .unwrap()
                .fct
                .name()
        ))
    }

    pub fn visitCircVarDecl(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!(
            "======visitCircVarDecl============={:?}",
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_var_decl_ref()
                .unwrap()
                .expr
                .get_ast_type()
        );
        Ok(format!(
            r#"decl("{}", {});"#,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_var_decl_ref()
                .unwrap()
                .lhs
                .identifier_base
                .name,
            self.visit(
                &stmt
                    .try_as_circuit_statement_ref()
                    .unwrap()
                    .borrow()
                    .try_as_circ_var_decl_ref()
                    .unwrap()
                    .expr
                    .clone()
                    .into()
            )
        ))
    }

    pub fn visitCircEqConstraint(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_eq_constraint_ref()
                .unwrap()
                .tgt
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .size_in_uints()
                == stmt
                    .try_as_circuit_statement_ref()
                    .unwrap()
                    .borrow()
                    .try_as_circ_eq_constraint_ref()
                    .unwrap()
                    .val
                    .t
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .size_in_uints()
        );
        Ok(format!(
            r#"checkEq("{}", "{}");"#,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_eq_constraint_ref()
                .unwrap()
                .tgt
                .identifier_base
                .name,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_eq_constraint_ref()
                .unwrap()
                .val
                .identifier_base
                .name
        ))
    }

    pub fn visitCircEncConstraint(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(stmt
            .try_as_circuit_statement_ref()
            .unwrap()
            .borrow()
            .try_as_circ_enc_constraint_ref()
            .unwrap()
            .cipher
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_cipher());
        assert!(stmt
            .try_as_circuit_statement_ref()
            .unwrap()
            .borrow()
            .try_as_circ_enc_constraint_ref()
            .unwrap()
            .pk
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_key());
        assert!(stmt
            .try_as_circuit_statement_ref()
            .unwrap()
            .borrow()
            .try_as_circ_enc_constraint_ref()
            .unwrap()
            .rnd
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_randomness());
        assert!(
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_enc_constraint_ref()
                .unwrap()
                .cipher
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_array_ref()
                .unwrap()
                .crypto_params()
                .as_ref()
                .unwrap()
                == stmt
                    .try_as_circuit_statement_ref()
                    .unwrap()
                    .borrow()
                    .try_as_circ_enc_constraint_ref()
                    .unwrap()
                    .pk
                    .t
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_array_ref()
                    .unwrap()
                    .crypto_params()
                    .as_ref()
                    .unwrap()
                && stmt
                    .try_as_circuit_statement_ref()
                    .unwrap()
                    .borrow()
                    .try_as_circ_enc_constraint_ref()
                    .unwrap()
                    .pk
                    .t
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_array_ref()
                    .unwrap()
                    .crypto_params()
                    .as_ref()
                    .unwrap()
                    == stmt
                        .try_as_circuit_statement_ref()
                        .unwrap()
                        .borrow()
                        .try_as_circ_enc_constraint_ref()
                        .unwrap()
                        .rnd
                        .t
                        .to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .try_as_array_ref()
                        .unwrap()
                        .crypto_params()
                        .as_ref()
                        .unwrap()
        );
        let backend = stmt
            .try_as_circuit_statement_ref()
            .unwrap()
            .borrow()
            .try_as_circ_enc_constraint_ref()
            .unwrap()
            .pk
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .try_as_array_ref()
            .unwrap()
            .crypto_params()
            .as_ref()
            .unwrap()
            .crypto_name
            .clone();

        Ok(format!(
            r#"check{}("{backend}", "{}", "{}", "{}", "{}");"#,
            if stmt
                .try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_enc_constraint_ref()
                .unwrap()
                .is_dec
            {
                "Dec"
            } else {
                "Enc"
            },
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_enc_constraint_ref()
                .unwrap()
                .plain
                .identifier_base
                .name,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_enc_constraint_ref()
                .unwrap()
                .pk
                .identifier_base
                .name,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_enc_constraint_ref()
                .unwrap()
                .rnd
                .identifier_base
                .name,
            stmt.try_as_circuit_statement_ref()
                .unwrap()
                .borrow()
                .try_as_circ_enc_constraint_ref()
                .unwrap()
                .cipher
                .identifier_base
                .name
        ))
    }
    pub fn visitCircSymmEncConstraint(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(stmt
            .try_as_circ_symm_enc_constraint_ref()
            .unwrap()
            .borrow()
            .iv_cipher
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_cipher());
        assert!(stmt
            .try_as_circ_symm_enc_constraint_ref()
            .unwrap()
            .borrow()
            .other_pk
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_key());
        assert!(
            stmt.try_as_circ_symm_enc_constraint_ref()
                .unwrap()
                .borrow()
                .iv_cipher
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_array_ref()
                .unwrap()
                .crypto_params()
                .as_ref()
                .unwrap()
                == stmt
                    .try_as_circ_symm_enc_constraint_ref()
                    .unwrap()
                    .borrow()
                    .other_pk
                    .t
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_array_ref()
                    .unwrap()
                    .crypto_params()
                    .as_ref()
                    .unwrap()
        );
        let backend = stmt
            .try_as_circ_symm_enc_constraint_ref()
            .unwrap()
            .borrow()
            .other_pk
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .try_as_array_ref()
            .unwrap()
            .crypto_params()
            .as_ref()
            .unwrap()
            .crypto_name
            .clone();
        Ok(format!(
            r#"checkSymm{}("{backend}", "{}", "{}", "{}");"#,
            if stmt
                .try_as_circ_symm_enc_constraint_ref()
                .unwrap()
                .borrow()
                .is_dec
            {
                "Dec"
            } else {
                "Enc"
            },
            stmt.try_as_circ_symm_enc_constraint_ref()
                .unwrap()
                .borrow()
                .plain
                .identifier_base
                .name,
            stmt.try_as_circ_symm_enc_constraint_ref()
                .unwrap()
                .borrow()
                .other_pk
                .identifier_base
                .name,
            stmt.try_as_circ_symm_enc_constraint_ref()
                .unwrap()
                .borrow()
                .iv_cipher
                .identifier_base
                .name
        ))
    }
    pub fn visitCircGuardModification(
        &self,
        stmt: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if let Some(_new_cond) = &stmt
                .try_as_circ_guard_modification_ref()
                .unwrap()
                .borrow()
                .new_cond
            {
                format!(
                    r#"addGuard("{}", {});"#,
                    stmt.try_as_circ_guard_modification_ref()
                        .unwrap()
                        .borrow()
                        .new_cond
                        .as_ref()
                        .unwrap()
                        .identifier_base
                        .name,
                    stmt.try_as_circ_guard_modification_ref()
                        .unwrap()
                        .borrow()
                        .is_true
                        .to_string()
                        .to_ascii_lowercase()
                )
            } else {
                String::from("popGuard();")
            },
        )
    }

    pub fn visitBooleanLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            r#"val({})"#,
            ast.try_as_boolean_literal_type_ref()
                .unwrap()
                .borrow()
                .value()
                .to_string()
                .to_ascii_lowercase()
        ))
    }

    pub fn visitNumberLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("======visitNumberLiteralExpr=============={:?}",ast);
        let t = _get_t(Some(ast.clone()));
        Ok(
            if ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_literal_expr_ref()
                .unwrap()
                .try_as_number_literal_expr_ref()
                .unwrap()
                .value
                < (1 << 31)
            {
                format!(
                    r#"val({}, {t})"#,
                    ast.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap()
                        .value
                )
            } else {
                format!(
                    r#"val("{}", {t})"#,
                    ast.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap()
                        .value
                )
            },
        )
    }

    pub fn visitIdentifierExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!("====visitIdentifierExpr================");
        Ok(
            if is_instance(
                ast.ast_base_ref().unwrap().borrow().idf.as_ref().unwrap(),
                ASTType::HybridArgumentIdf,
            ) && ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .is_cipher()
            {
                format!(
                    r#"getCipher("{}")"#,
                    ast.ast_base_ref()
                        .unwrap()
                        .borrow()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                )
            } else {
                panic!("=====visitIdentifierExpr=============");
                println!(
                    "====visitIdentifierExpr=========get==={}====",
                    ast.ast_base_ref()
                        .unwrap()
                        .borrow()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                );
                format!(
                    r#"get("{}")"#,
                    ast.ast_base_ref()
                        .unwrap()
                        .borrow()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                )
            },
        )
    }

    pub fn visitMemberAccessExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(is_instance(
            &ast.try_as_member_access_expr_ref().unwrap().borrow().member,
            ASTType::HybridArgumentIdf
        ));
        Ok(
            if ast
                .try_as_member_access_expr_ref()
                .unwrap()
                .borrow()
                .member
                .borrow()
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .is_cipher()
            {
                format!(
                    r#"getCipher("{}")"#,
                    ast.try_as_member_access_expr_ref()
                        .unwrap()
                        .borrow()
                        .member
                        .borrow()
                        .name()
                )
            } else {
                assert!(
                    ast.try_as_member_access_expr_ref()
                        .unwrap()
                        .borrow()
                        .member
                        .borrow()
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .t
                        .to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .size_in_uints()
                        == 1
                );
                format!(
                    r#"get("{}")"#,
                    ast.try_as_member_access_expr_ref()
                        .unwrap()
                        .borrow()
                        .member
                        .borrow()
                        .name()
                )
            },
        )
    }
    // #[allow(unreachable_code)]
    pub fn visitIndexExpr(&self, _ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // unimplemented!();
        Err(eyre::eyre!("unimplemented"))
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!(
            "======visitFunctionCallExpr=============={:?}",
            ast.try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .get_ast_type()
        );
        if is_instance(
            ast.try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::BuiltinFunction,
        ) {
            assert!(ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_builtin_function_ref()
                .unwrap()
                .can_be_private());
            let mut args: Vec<_> = ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()
                .iter()
                .map(|arg| self.visit(&arg.clone().into()))
                .collect();
            if ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_builtin_function_ref()
                .unwrap()
                .is_shiftop()
            {
                assert!(ast
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()[1]
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
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .is_literals());
                args[1] = ast
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()[1]
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
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_number_type_name_ref()
                    .unwrap()
                    .try_as_number_literal_type_ref()
                    .unwrap()
                    .value()
                    .to_string()
            }

            let mut op = &ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_builtin_function_ref()
                .unwrap()
                .op
                .clone();
            let op = if op == "sign-" { "-" } else { op };
            if op == "sign+" {
                // unimplemented!()
                eyre::bail!("unimplemented")
            }
            let homomorphism = ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_builtin_function_ref()
                .unwrap()
                .homomorphism
                .clone();
            let (f_start, crypto_backend, public_key_name) =
                if homomorphism == Homomorphism::non_homomorphic() {
                    (String::from("o_("), String::new(), String::new())
                } else {
                    let crypto_backend = CFG
                        .lock()
                        .unwrap()
                        .user_config
                        .get_crypto_params(&homomorphism)
                        .crypto_name; // String::from("elgamal");
                    let public_key_name = ast
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .public_key()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .identifier_base
                        .name
                        .clone();

                    args = args
                        .iter()
                        .map(|arg| format!(r#"HomomorphicInput.of({arg})"#))
                        .collect();
                    (
                        format!(r#"o_hom("{crypto_backend}", "{public_key_name}", "#),
                        crypto_backend,
                        public_key_name,
                    )
                };

            return Ok(if op == "ite" {
                format!(
                    r#"{f_start}{{{}}}, "?", {{{}}}, ":", {{{}}})"#,
                    args[0], args[1], args[2]
                )
            } else if op == "parenthesis" {
                format!(r#"({})"#, args[0])
            } else {
                let o = if op.len() == 1 {
                    format!("'{op}'")
                } else {
                    format!(r#""{op}""#)
                };
                if args.len() == 1 {
                    format!(r#"{f_start}{o}, {})"#, args[0])
                } else {
                    assert!(args.len() == 2);
                    if op == "*"
                        && ast
                            .try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .try_as_function_call_expr_ref()
                            .unwrap()
                            .func()
                            .try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .try_as_builtin_function_ref()
                            .unwrap()
                            .rerand_using
                            .is_some()
                    {
                        // re-randomize homomorphic scalar multiplication
                        let rnd = self.visit(
                            &ast.try_as_expression_ref()
                                .unwrap()
                                .borrow()
                                .try_as_function_call_expr_ref()
                                .unwrap()
                                .func()
                                .try_as_expression_ref()
                                .unwrap()
                                .borrow()
                                .try_as_builtin_function_ref()
                                .unwrap()
                                .rerand_using
                                .clone()
                                .unwrap()
                                .into(),
                        );
                        format!(
                            r#"o_rerand({f_start}{}, {o}, {}), "{crypto_backend}", "{public_key_name}", {rnd})"#,
                            args[0], args[1]
                        )
                    } else {
                        format!(r#"{f_start}{}, {o},{})"#, args[0], args[1])
                    }
                }
            });
        } else if ast
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_function_call_expr_ref()
            .unwrap()
            .is_cast()
            && is_instance(
                &ast.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
                ASTType::EnumDefinition,
            )
        {
            assert!(
                ast.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .elem_bitwidth()
                    == 256
            );
            return self.handle_cast(
                self.visit(
                    &ast.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .args()[0]
                        .clone()
                        .into(),
                ),
                &RcCell::new(TypeName::uint_type()).into(),
            );
        }

        // assert!(
        //     false,
        //     "Unsupported function {} inside circuit",
        //     ast.func().code()
        // );
        Err(eyre::eyre!(
            "Unsupported function {} inside circuit",
            ast.try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .code()
        ))
    }

    pub fn visitPrimitiveCastExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!(
            "======visitPrimitiveCastExpr===={}=========={:?}",
            ast,
            ast.try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_primitive_cast_expr_ref()
                .unwrap()
                .expr
                .get_ast_type()
        );
        let wire = self.visit(
            &ast.try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_primitive_cast_expr_ref()
                .unwrap()
                .expr
                .clone()
                .into(),
        );
        println!("=========wire======={wire}");
        self.handle_cast(
            wire,
            &ast.try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_primitive_cast_expr_ref()
                .unwrap()
                .elem_type,
        )
    }

    pub fn handle_cast(
        &self,
        wire: String,
        t: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!("=handle_cast=======wire========{wire}");
        Ok(format!(r#"cast({wire}, {})"#, _get_t(Some(t.clone()))))
    }
}
// """Generate java code which adds circuit IO as described by circuit"""
pub fn add_function_circuit_arguments(circuit: &RcCell<CircuitHelper>) -> Vec<String> {
    let mut input_init_stmts = vec![];
    for sec_input in circuit.borrow().sec_idfs() {
        input_init_stmts.push(format!(
            r#"addS("{}", {}, {});"#,
            sec_input.identifier_base.name,
            sec_input
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .size_in_uints(),
            _get_t(Some(sec_input.t.clone()))
        ));
    }

    for pub_input in circuit.borrow().input_idfs() {
        input_init_stmts.push(
            if pub_input.t.to_ast().try_as_type_name().unwrap().is_key() {
                let backend = pub_input
                    .t
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .try_as_array_ref()
                    .unwrap()
                    .crypto_params()
                    .as_ref()
                    .unwrap()
                    .crypto_name
                    .clone();
                format!(
                    r#"addK("{backend}", "{}", {});"#,
                    pub_input.identifier_base.name,
                    pub_input
                        .t
                        .to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .size_in_uints()
                )
            } else {
                format!(
                    r#"addIn("{}", {}, {});"#,
                    pub_input.identifier_base.name,
                    pub_input
                        .t
                        .to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .size_in_uints(),
                    _get_t(Some(pub_input.t.clone()))
                )
            },
        );
    }
    for pub_output in circuit.borrow().output_idfs() {
        input_init_stmts.push(format!(
            r#"addOut("{}", {}, {});"#,
            pub_output.identifier_base.name,
            pub_output
                .t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .size_in_uints(),
            _get_t(Some(pub_output.t.clone()))
        ));
    }

    let sec_input_names: Vec<_> = circuit
        .borrow()
        .sec_idfs()
        .iter()
        .map(|sec_input| sec_input.identifier_base.name.clone())
        .collect();
    let all_crypto_params = CFG.lock().unwrap().user_config.all_crypto_params();
    for crypto_params in &all_crypto_params {
        let pk_name =
            CircuitHelper::get_glob_key_name(&RcCell::new(MeExpr::new()).into(), crypto_params);
        let sk_name = CircuitHelper::get_own_secret_key_name(&crypto_params);
        if crypto_params.is_symmetric_cipher() && sec_input_names.contains(&sk_name) {
            assert!(circuit
                .borrow()
                .input_idfs()
                .iter()
                .map(|pub_input| pub_input.identifier_base.name.clone())
                .collect::<Vec<_>>()
                .contains(&pk_name));
            input_init_stmts.push(format!(
                r#"setKeyPair("{}", "{pk_name}", "{sk_name}");"#,
                crypto_params.crypto_name
            ));
        }
    }

    input_init_stmts
}

// class JsnarkGenerator(CircuitGenerator)
//<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
pub struct JsnarkGenerator {
    pub circuit_generator_base: CircuitGeneratorBase, //<T, VK>,
}

//<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
impl JsnarkGenerator {
    pub fn new(
        circuits: Vec<RcCell<CircuitHelper>>,
        proving_scheme: String,
        output_dir: String,
    ) -> Self {
        Self {
            circuit_generator_base: CircuitGeneratorBase::new(
                circuits,
                proving_scheme,
                output_dir,
                false,
            ),
        }
    }
}
impl CircuitGenerator for JsnarkGenerator {
    fn base(&self) -> &CircuitGeneratorBase {
        &self.circuit_generator_base
    }
    //Create output directory
    fn _generate_zkcircuit(&self, import_keys: bool, circuit: &RcCell<CircuitHelper>) -> bool {
        let p = self.circuit_generator_base._get_circuit_output_dir(circuit);
        let output_dir = Path::new(&p);
        if let Err(_) | Ok(false) = output_dir.try_exists() {
            std::fs::create_dir_all(output_dir).expect(output_dir.to_str().unwrap());
        }
        //Generate java code to add used crypto backends by calling addCryptoBackend
        let mut crypto_init_stmts = vec![];
        for params in &circuit
            .borrow()
            .fct
            .borrow()
            .used_crypto_backends
            .clone()
            .unwrap()
        {
            let init_stmt = format!(
                r#"addCryptoBackend("{}", "{}", {});"#,
                params.crypto_name,
                params.crypto_name,
                params.key_bits()
            );
            crypto_init_stmts.push(init_stmt);
        }
        //Generate java code for all functions which are transitively called by the fct corresponding to this circuit
        //(outside private expressions)
        let mut fdefs = vec![];
        for fct in &circuit.borrow().transitively_called_functions {
            let target_circuit = &self.circuit_generator_base.circuits[fct];
            let body_stmts = JsnarkVisitor::new(target_circuit.borrow().phi()).visitCircuit();

            let body = [format!(r#"stepIn("{}");"#, fct.borrow().name())]
                .into_iter()
                .chain(add_function_circuit_arguments(target_circuit))
                .chain([String::new()])
                .chain(body_stmts)
                .chain([(String::from("stepOut();"))])
                .collect::<Vec<_>>()
                .join("\n");
            let fdef = format!(
                "private void _{name}() {{\n {body} \n}}",
                body = indent(body),
                name = fct.borrow().name()
            );
            fdefs.push(format!(r#"{fdef}"#))
        }
        //Generate java code for the function corresponding to this circuit
        let input_init_stmts = add_function_circuit_arguments(circuit);
        let constraints = JsnarkVisitor::new(circuit.borrow().phi()).visitCircuit();
        //Inject the function definitions into the java template
        let code = jsnark::get_jsnark_circuit_class_str(
            circuit,
            crypto_init_stmts,
            fdefs,
            input_init_stmts
                .iter()
                .cloned()
                .chain([String::new()])
                .chain(constraints)
                .collect(),
        );

        //Compute combined hash of the current jsnark interface jar and of the contents of the java file
        let hashfile = output_dir.join(format!(
            r#"{}.hash"#,
            CFG.lock().unwrap().jsnark_circuit_classname()
        ));
        println!("========");
        let proving_scheme = CFG.lock().unwrap().user_config.proving_scheme(); //String::from("groth16");//
        println!("====1====");
        let jar_hash_string =
            jsnark::CIRCUIT_BUILDER_JAR_HASH.to_string() + &code + &proving_scheme;
        println!("====1==0==");
        let hs = hash_string(&jar_hash_string);
        println!("====1=1===");
        let digest = hex::encode(hs);
        println!("===2=====");
        let oldhash = if let Ok(true) = hashfile.try_exists() {
            read_file(hashfile.to_str().unwrap())
        } else {
            String::new()
        };

        //Invoke jsnark compilation if either the jsnark-wrapper or the current circuit was modified (based on hash comparison)
        if oldhash != digest
            || output_dir
                .join("circuit.arith")
                .try_exists()
                .map_or(false, |v| v)
        {
            if !import_keys {
                //Remove old keys
                for f in self.circuit_generator_base._get_vk_and_pk_paths(circuit) {
                    if Path::new(&f).try_exists().map_or(false, |v| v) {
                        let _ = std::fs::remove_file(f);
                    }
                }
            }
            jsnark::compile_circuit(output_dir.to_str().unwrap(), &code);
            save_to_file(None, hashfile.to_str().unwrap(), &digest);
            true
        } else {
            zk_print!(
                r#"Circuit \"{}\" not modified, skipping compilation"#,
                circuit.borrow().get_verification_contract_name()
            );
            false
        }
    }
    //Invoke the custom libsnark interface to generate keys
    fn _generate_keys(&self, circuit: &RcCell<CircuitHelper>) {
        let output_dir = self.circuit_generator_base._get_circuit_output_dir(circuit);
        libsnark::generate_keys(
            &output_dir,
            &output_dir,
            &self.circuit_generator_base.proving_scheme,
        );
    }

    // @classmethod
    fn get_vk_and_pk_filenames() -> Vec<String> {
        ["verification.key", "proving.key", "verification.key.bin"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    fn _parse_verification_key(&self, circuit: &RcCell<CircuitHelper>) -> Option<VerifyingKeyType> {
        let p = &self.circuit_generator_base._get_vk_and_pk_paths(circuit)[0];
        let f = File::open(p).expect("");
        // data = iter(f.read().splitlines());
        let buf = BufReader::new(f);
        let mut data = buf.lines();
        if self.circuit_generator_base.proving_scheme.type_id()
            == TypeId::of::<ProvingSchemeGroth16>()
        {
            let a = G1Point::from_it(&mut data);
            let b = G2Point::from_it(&mut data);
            let gamma = G2Point::from_it(&mut data);
            let delta = G2Point::from_it(&mut data);
            let query_len = data.next().unwrap().unwrap().parse::<usize>().unwrap();
            let mut gamma_abc = vec![G1Point::default(); query_len];
            for idx in 0..query_len {
                gamma_abc.insert(idx, G1Point::from_it(&mut data));
            }
            return Some(VerifyingKeyType::ProvingSchemeGroth16(
                <ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX::new(
                    a, b, gamma, delta, gamma_abc,
                ),
            ));
        } else if self.circuit_generator_base.proving_scheme.type_id()
            == TypeId::of::<ProvingSchemeGm17>()
        {
            let h = G2Point::from_it(&mut data);
            let g_alpha = G1Point::from_it(&mut data);
            let h_beta = G2Point::from_it(&mut data);
            let g_gamma = G1Point::from_it(&mut data);
            let h_gamma = G2Point::from_it(&mut data);
            let query_len = data.next().unwrap().unwrap().parse::<usize>().unwrap();
            let mut query = vec![G1Point::default(); query_len];
            for idx in 0..query_len {
                query.insert(idx, G1Point::from_it(&mut data));
            }
            return Some(VerifyingKeyType::ProvingSchemeGm17(
                <ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX::new(
                    h, g_alpha, h_beta, g_gamma, h_gamma, query,
                ),
            ));
        }
        // else {
        //     unimplemented!()
        // }
        None
    }

    fn _get_prover_key_hash(&self, circuit: &RcCell<CircuitHelper>) -> Vec<u8> {
        hash_file(
            &self.circuit_generator_base._get_vk_and_pk_paths(circuit)[1],
            0,
        )
    }

    //Jsnark requires an additional public input with the value 1 as first input
    fn _get_primary_inputs(&self, circuit: &RcCell<CircuitHelper>) -> Vec<String> {
        println!("==_get_primary_inputs========");
        [String::from("1")]
            .into_iter()
            .chain(self.circuit_generator_base._get_primary_inputs(circuit))
            .collect()
    }
}
