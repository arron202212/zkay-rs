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
    expression::{
        BooleanLiteralExpr, ExpressionASType, ExpressionBaseMutRef, ExpressionBaseProperty,
        IdentifierExpr, IdentifierExprUnion, LocationExpr, NumberLiteralExpr, SliceExpr,
    },
    identifier_declaration::VariableDeclaration,
    is_instance, is_instances,
    statement::{
        AssignmentStatement, AssignmentStatementBase, IndentBlock, Statement,
        StatementBaseProperty, VariableDeclarationStatement,
    },
    type_name::{
        ArrayBase, BooleanLiteralType, CombinedPrivacyUnion, DummyAnnotation, ExprUnion,
        NumberLiteralType, NumberLiteralTypeUnion, TypeName, UintTypeName,
    },
    ASTBase, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTChildrenCallBack,
    ASTFlatten, ASTFlattenWeak, ASTType, ArgType, ChildListBuilder, DeepClone, FullArgsSpec,
    FullArgsSpecInit, Immutable, IntoAST, IntoExpression, IntoStatement, AST,
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
#[derive(EnumIs, EnumTryAs, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IdentifierUnion {
    Identifier(Option<RcCell<Identifier>>),
    String(String),
}
#[enum_dispatch(
    DeepClone,
    FullArgsSpec,
    IntoAST,
    ASTFlattenImpl,
    ASTInstanceOf,
    IdentifierBaseRef,
    IdentifierBaseMutRef,
    ASTBaseRef
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
    pub is_string: bool,
}
impl DeepClone for IdentifierBase {
    fn clone_inner(&self) -> Self {
        Self {
            ast_base: self.ast_base.clone_inner(),
            ..self.clone()
        }
    }
}
impl IntoAST for IdentifierBase {
    fn into_ast(self) -> AST {
        AST::Identifier(Identifier::Identifier(self))
    }
}

impl FullArgsSpec for IdentifierBase {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![ArgType::Str(Some(self.name.clone()))]
    }
}
impl FullArgsSpecInit for IdentifierBase {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        IdentifierBase::new(fields[0].clone().try_as_str().flatten().unwrap())
    }
}
impl IdentifierBase {
    pub fn new(name: String) -> Self {
        // if "zk__in2_plain_Choice" == name{
        //     println!("==IdentifierBase========zk__in2_plain_Choice========");
        // }
        Self {
            ast_base: RcCell::new(ASTBase::new(None, None, None)),
            name,
            is_string: false,
        }
    }
    pub fn ast_base_ref(&self) -> RcCell<ASTBase> {
        self.ast_base.clone()
    }
    pub fn decl_var(&self, t: &ASTFlatten, expr: Option<ASTFlatten>) -> Statement {
        let t = if is_instance(t, ASTType::TypeNameBase) {
            Some(RcCell::new(AnnotatedTypeName::new(
                Some(t.clone()),
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
            .to_ast()
            .try_as_type_name()
            .unwrap()
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

use num_enum::{FromPrimitive, IntoPrimitive};
#[repr(i32)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, FromPrimitive, IntoPrimitive)]
pub enum HybridArgType {
    #[default]
    PrivCircuitVal,
    PubCircuitArg,
    PubContractVal,
    TmpCircuitVal,
}
#[impl_traits(IdentifierBase, ASTBase)]
#[derive(ASTDebug, ASTFlattenImpl, ASTKind, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HybridArgumentIdf {
    pub identifier_base: IdentifierBase,
    pub t: ASTFlatten,
    pub arg_type: HybridArgType,
    pub corresponding_priv_expression: Option<ASTFlatten>,
    pub serialized_loc: SliceExpr,
}
impl DeepClone for HybridArgumentIdf {
    fn clone_inner(&self) -> Self {
        Self {
            identifier_base: self.identifier_base.clone_inner(),
            t: self.t.clone_inner(),
            arg_type: self.arg_type.clone(),
            corresponding_priv_expression: self.corresponding_priv_expression.clone_inner(),
            serialized_loc: self.serialized_loc.clone_inner(),
        }
    }
}
impl IntoAST for HybridArgumentIdf {
    fn into_ast(self) -> AST {
        AST::Identifier(Identifier::HybridArgumentIdf(self))
    }
}
impl FullArgsSpec for HybridArgumentIdf {
    fn get_attr(&self) -> Vec<ArgType> {
        vec![
            ArgType::Str(Some(self.identifier_base.name.clone())),
            ArgType::ASTFlatten(Some(self.t.clone_inner())),
            ArgType::Int(Some(self.arg_type.clone().into())),
            ArgType::ASTFlatten(
                self.corresponding_priv_expression
                    .as_ref()
                    .map(|a| a.clone_inner()),
            ),
        ]
    }
}
impl FullArgsSpecInit for HybridArgumentIdf {
    fn from_fields(&self, fields: Vec<ArgType>) -> Self {
        HybridArgumentIdf::new(
            fields[0].clone().try_as_str().flatten().unwrap(),
            fields[1].clone().try_as_ast_flatten().flatten().unwrap(),
            HybridArgType::from(fields[2].clone().try_as_int().unwrap().unwrap()),
            fields[3].clone().try_as_ast_flatten().unwrap(),
        )
    }
}
impl HybridArgumentIdf {
    pub fn new(
        name: String,
        mut t: ASTFlatten,
        arg_type: HybridArgType,
        corresponding_priv_expression: Option<ASTFlatten>,
    ) -> Self {
        // println!("==HybridArgumentIdf====new===================={name}");
        // assert!("c_count" != name);
        if is_instance(&t, ASTType::BooleanLiteralType) {
            t = RcCell::new(TypeName::bool_type()).into();
        } else if is_instance(&t, ASTType::NumberLiteralType) {
            let tt = t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_elementary_type_name_ref()
                .unwrap()
                .try_as_number_type_name_ref()
                .unwrap()
                .try_as_number_literal_type_ref()
                .unwrap()
                .to_abstract_type();
            t = tt.clone_inner();
        } else if is_instance(&t, ASTType::EnumValueTypeName) {
            let tt = t
                .to_ast()
                .try_as_type_name()
                .unwrap()
                .try_as_user_defined_type_name_ref()
                .unwrap()
                .try_as_enum_value_type_name_ref()
                .unwrap()
                .to_abstract_type();
            t = tt.clone_inner();
        }

        Self {
            identifier_base: IdentifierBase::new(name),
            t,
            arg_type,
            corresponding_priv_expression: corresponding_priv_expression.clone_inner(),
            serialized_loc: SliceExpr::new(
                Some(
                    RcCell::new(
                        IdentifierExpr::new(IdentifierExprUnion::String(String::new()), None)
                            .into_ast(),
                    )
                    .into(),
                ),
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
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .value()
                    == "true",
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
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .value()
                    .parse::<i32>()
                    .unwrap(),
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
                Identifier::HybridArgumentIdf(self.clone_inner()),
            )))
            .as_type(&self.t.clone_inner().into());
            // println!("===ma==parent={}===",parent.is_some());
            // if ma.code()=="zk__data.zk__out0_plain"{
            //     panic!("zk__data.zk__out0_plain");
            //     }

            ma.ast_base_ref().unwrap().borrow_mut().parent = parent.map(|p| p.clone().downgrade());
            let statement = parent.as_ref().and_then(|&p| {
                if is_instance(p, ASTType::StatementBase) {
                    Some(p.clone().downgrade())
                } else {
                    p.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .statement()
                        .clone()
                }
            });
            if ma.is_identifier_expr() {
                ma.try_as_identifier_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = statement;
            } else if ma.is_member_access_expr() {
                ma.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = statement;
            } else if ma.is_expression() {
                ma.try_as_expression_ref()
                    .unwrap()
                    .borrow_mut()
                    .expression_base_mut_ref()
                    .statement = statement;
            } else if ma.is_ast() {
                ma.try_as_ast_ref()
                    .unwrap()
                    .borrow_mut()
                    .try_as_expression_mut()
                    .unwrap()
                    .expression_base_mut_ref()
                    .statement = statement;
            } else {
                panic!("=======else=============={ma:?}");
            }
            // println!("===statement========{},======={}", file!(), line!());
            ma
        }
    }
    pub fn clones(&self) -> Self {
        let mut ha = Self::new(
            self.name().clone(),
            self.t.clone_inner(),
            self.arg_type.clone(),
            self.corresponding_priv_expression.clone_inner(),
        );
        ha.serialized_loc = self.serialized_loc.clone_inner();
        ha
    }
    pub fn get_idf_expr(&self, parent: Option<&ASTFlatten>) -> Option<ASTFlatten> {
        let mut ie = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(RcCell::new(Identifier::HybridArgumentIdf(
                self.clones(),
            ))),
            None,
        )
        .as_type(&self.t.clone().into());
        // if parent.is_none(){
        //     println!("==get_idf_expr====else=={}",self);
        // }
        ie.ast_base_ref().unwrap().borrow_mut().parent = parent.map(|p| p.clone().downgrade());

        ie.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_expression_mut()
            .unwrap()
            .expression_base_mut_ref()
            .statement = parent.as_ref().and_then(|&p| {
            if is_instance(p, ASTType::StatementBase) {
                Some(p.clone().downgrade())
            } else if is_instance(p, ASTType::ExpressionBase) {
                p.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .statement()
                    .clone()
            } else {
                println!("===statement====else=parent====type====={:?}=====", p);
                // panic!("===statement====else=parent====type====={:?}=====",p);
                None
            }
        });
        // println!("===statement========{},======={}", file!(), line!());

        Some(ie)
    }

    pub fn _set_serialized_loc(
        &mut self,
        idf: String,
        base: Option<ASTFlatten>,
        start_offset: i32,
    ) {
        assert!(self.serialized_loc.start_offset == -1);
        self.serialized_loc.arr = Some(
            RcCell::new(IdentifierExpr::new(IdentifierExprUnion::String(idf), None).into_ast())
                .into(),
        );
        self.serialized_loc.base = base;
        self.serialized_loc.start_offset = start_offset;
        self.serialized_loc.size = self.t.to_ast().try_as_type_name().unwrap().size_in_uints();
    }

    pub fn deserialize(
        &mut self,
        source_idf: String,
        base: Option<ASTFlatten>,
        start_offset: i32,
    ) -> AssignmentStatement {
        self._set_serialized_loc(source_idf.clone(), base.clone(), start_offset);

        let src = IdentifierExpr::new(IdentifierExprUnion::String(source_idf), None).as_type(
            &RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None, None)).into(),
        );
        let loc_expr = self.get_loc_expr(None);
        if is_instance(&self.t, ASTType::ArrayBase) {
            return LocationExpr::SliceExpr(SliceExpr::new(
                Some(loc_expr),
                None,
                0,
                self.t.to_ast().try_as_type_name().unwrap().size_in_uints(),
            ))
            .assign(RcCell::new(self.serialized_loc.clone()).into());
        }
        if let Some(base) = &base {
            loc_expr
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .assign(
                    src.try_as_ast_ref()
                        .unwrap()
                        .borrow()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .index(ExprUnion::Expression(
                            RcCell::new(crate::a_e!(base.to_ast()).binop(
                                String::from("+"),
                                NumberLiteralExpr::new(start_offset, false).into_expr(),
                            ))
                            .into(),
                        ))
                        .to_ast()
                        .try_as_expression()
                        .unwrap()
                        .explicitly_converted(&self.t.clone().into()),
                )
        } else {
            loc_expr
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .assign(
                    LocationExpr::IdentifierExpr(
                        src.try_as_identifier_expr_ref().unwrap().borrow().clone(),
                    )
                    .index(ExprUnion::I32(start_offset))
                    .try_as_expression()
                    .unwrap()
                    .borrow()
                    .explicitly_converted(&self.t.clone().into()),
                )
        }
    }

    pub fn serialize(
        &mut self,
        target_idf: String,
        base: Option<ASTFlatten>,
        start_offset: i32,
    ) -> AssignmentStatement {
        // if target_idf=="zk__in"{
        // println!("================serialize=========begin====");
        // }
        self._set_serialized_loc(target_idf.clone(), base.clone(), start_offset);

        let tgt = IdentifierExpr::new(IdentifierExprUnion::String(target_idf.clone()), None)
            .as_type(
                &RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None, None)).into(),
            );
        if is_instance(&self.t, ASTType::ArrayBase) {
            let loc = self.get_loc_expr(None);
            let res = LocationExpr::SliceExpr(self.serialized_loc.clone_inner()).assign(
                RcCell::new(SliceExpr::new(
                    Some(loc),
                    None,
                    0,
                    self.t.to_ast().try_as_type_name().unwrap().size_in_uints(),
                ))
                .into(),
            );
            // if target_idf=="zk__in"{
            // println!("================serialize=========array=={}==",res.code());
            // }
            return res;
        }
        let expr = self.get_loc_expr(None);
        let expr = if self
            .t
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_signed_numeric()
        {
            // Cast to same size uint to prevent sign extension
            expr.try_as_expression()
                .unwrap()
                .borrow()
                .explicitly_converted(
                    &RcCell::new(
                        UintTypeName::new(format!(
                            "uint{}",
                            self.t.to_ast().try_as_type_name().unwrap().elem_bitwidth()
                        ))
                        .into_ast()
                        .try_as_type_name()
                        .unwrap(),
                    )
                    .into(),
                )
        } else if self.t.to_ast().try_as_type_name().unwrap().is_numeric()
            && self.t.to_ast().try_as_type_name().unwrap().elem_bitwidth() == 256
        {
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
            // println!("==========================={expr:?}");
            expr.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .explicitly_converted(&RcCell::new(TypeName::uint_type()).into())
            //if let ExplicitlyConvertedUnion::FunctionCallExpr(fce)={fce}else{FunctionCallExpr::default()}
        };

        if let Some(base) = &base {
            LocationExpr::IndexExpr(
                tgt.try_as_ast_ref()
                    .unwrap()
                    .borrow()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .index(ExprUnion::Expression(
                        RcCell::new(base.to_ast().try_as_expression_ref().unwrap().binop(
                            String::from("+"),
                            NumberLiteralExpr::new(start_offset, false).into_expr(),
                        ))
                        .into(),
                    ))
                    .try_as_ast_ref()
                    .unwrap()
                    .borrow()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_index_expr_ref()
                    .unwrap()
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
