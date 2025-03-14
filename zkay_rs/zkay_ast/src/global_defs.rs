#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// # BUILTIN SPECIAL TYPE DEFINITIONS
use crate::ast::{
    ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTFlatten, DeepClone, IntoAST,
    annotated_type_name::AnnotatedTypeName,
    identifier::{Identifier, IdentifierBase},
    identifier_declaration::{Parameter, StateVariableDeclaration, VariableDeclaration},
    namespace_definition::{ConstructorOrFunctionDefinition, StructDefinition},
    statement::Block,
    type_name::{FunctionTypeName, StructTypeName, TypeName, UserDefinedTypeName},
};
use crate::homomorphism::Homomorphism;
use crate::pointers::parent_setter::set_parents;
use lazy_static::lazy_static;
use rccell::{RcCell, WeakCell};
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
// lazy_static! {
// pub static ref VARIABLE_DECLARATIONS_CACHE:Mutex<BTreeSet<RcCell<VariableDeclaration>>>=Mutex::new(BTreeSet::new());
pub fn array_length_member() -> ASTFlatten {
    RcCell::new(VariableDeclaration::new(
        vec![],
        Some(AnnotatedTypeName::uint_all()),
        Identifier::identifier("length"),
        None,
    ))
    .into()
}

pub fn global_defs() -> GlobalDefs {
    GlobalDefs::new()
}
pub fn global_vars(global_defs: RcCell<GlobalDefs>) -> GlobalVars {
    GlobalVars::new(global_defs)
}
lazy_static! {
// pub static ref ARRAY_LENGTH_MEMBER: Arc<Mutex<VariableDeclaration>> = Arc::new(Mutex::new(VariableDeclaration::new(
//     vec![],
//     AnnotatedTypeName::uint_all(),
//     Identifier::identifier("length"),
//     None
// )));
// pub static ref GLOBAL_DEFSS: GlobalDefs = GlobalDefs::new();
// pub static ref GLOBAL_VARSS: GlobalVars = GlobalVars::new();
}
pub struct GlobalDefs {
    address_struct: RcCell<StructDefinition>,
    address_payable_struct: RcCell<StructDefinition>,
    msg_struct: RcCell<StructDefinition>,
    block_struct: RcCell<StructDefinition>,
    tx_struct: RcCell<StructDefinition>,
}
// class GlobalDefs:
// # gasleft: FunctionDefinition = FunctionDefinition(
// #     idf=Identifier::identifier("gasleft"),
// #     parameters=[],
// #     modifiers=[],
// #     return_parameters=[Parameter([], annotated_type=AnnotatedTypeName::uint_all(), idf=Identifier::identifier(String::new()))],
// #     body=Block([])
// # )
// # gasleft.idf.parent = gasleft
impl GlobalDefs {
    pub fn new() -> Self {
        let address_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<address>"),
            vec![
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("balance"),
                    None,
                ))
                .into(),
            ],
        ));
        set_parents(&address_struct.clone().into());

        let mut address_payable_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<address_payable>"),
            vec![
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("balance"),
                    None,
                ))
                .into(),
                RcCell::new(ConstructorOrFunctionDefinition::new(
                    Identifier::identifier("send"),
                    vec![RcCell::new(Parameter::new(
                        vec![],
                        Some(AnnotatedTypeName::uint_all()),
                        Identifier::identifier(""),
                        None,
                    ))],
                    vec![String::from("public")],
                    vec![RcCell::new(Parameter::new(
                        vec![],
                        Some(AnnotatedTypeName::bool_all()),
                        Identifier::identifier(""),
                        None,
                    ))],
                    Some(RcCell::new(Block::new(vec![], false))),
                ))
                .into(),
                RcCell::new(ConstructorOrFunctionDefinition::new(
                    Identifier::identifier("transfer"),
                    vec![RcCell::new(Parameter::new(
                        vec![],
                        Some(AnnotatedTypeName::uint_all()),
                        Identifier::identifier(""),
                        None,
                    ))],
                    vec![String::from("public")],
                    vec![],
                    Some(RcCell::new(Block::new(vec![], false))),
                ))
                .into(),
            ],
        ));
        // println!("=====members[1]============{:?}====", address_payable_struct.borrow().members[1]);
        address_payable_struct.borrow_mut().members[1]
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .borrow_mut()
            .can_be_private = false;
        address_payable_struct.borrow_mut().members[2]
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .borrow_mut()
            .can_be_private = false;
        set_parents(&address_payable_struct.clone().into());

        let msg_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<msg>"),
            vec![
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::address_payable_type()).into()),
                        None,
                        Homomorphism::non_homomorphic(),
                    ))),
                    Identifier::identifier("sender"),
                    None,
                ))
                .into(),
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("value"),
                    None,
                ))
                .into(),
            ],
        ));
        set_parents(&msg_struct.clone().into());

        let block_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<block>"),
            vec![
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::address_payable_type()).into()),
                        None,
                        Homomorphism::non_homomorphic(),
                    ))),
                    Identifier::identifier("coinbase"),
                    None,
                ))
                .into(),
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("difficulty"),
                    None,
                ))
                .into(),
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("gaslimit"),
                    None,
                ))
                .into(),
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("number"),
                    None,
                ))
                .into(),
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("timestamp"),
                    None,
                ))
                .into(),
            ],
        ));
        set_parents(&block_struct.clone().into());

        let tx_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<tx>"),
            vec![
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(AnnotatedTypeName::uint_all()),
                    Identifier::identifier("gasprice"),
                    None,
                ))
                .into(),
                RcCell::new(VariableDeclaration::new(
                    vec![],
                    Some(RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::address_payable_type()).into()),
                        None,
                        Homomorphism::non_homomorphic(),
                    ))),
                    Identifier::identifier("origin"),
                    None,
                ))
                .into(),
            ],
        ));
        set_parents(&tx_struct.clone().into());
        Self {
            address_struct,
            address_payable_struct,
            msg_struct,
            block_struct,
            tx_struct,
        }
    }
    pub fn vars(&self) -> Vec<RcCell<StructDefinition>> {
        vec![
            self.address_struct.clone(),
            self.address_payable_struct.clone(),
            self.msg_struct.clone(),
            self.block_struct.clone(),
            self.tx_struct.clone(),
        ]
    }
}

pub struct GlobalVars {
    msg: RcCell<StateVariableDeclaration>,
    block: RcCell<StateVariableDeclaration>,
    tx: RcCell<StateVariableDeclaration>,
    now: RcCell<StateVariableDeclaration>,
    pub global_defs: RcCell<GlobalDefs>,
}
// class GlobalVars:
impl GlobalVars {
    pub fn new(global_defs: RcCell<GlobalDefs>) -> Self {
        let mut msg = RcCell::new(StateVariableDeclaration::new(
            Some(AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![RcCell::new(
                        global_defs
                            .borrow()
                            .msg_struct
                            .borrow()
                            .idf()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .clone_inner(),
                    )],
                    Some(ASTFlatten::from(global_defs.borrow().msg_struct.clone()).downgrade()),
                )
                .to_type_name(),
            )),
            vec![],
            Identifier::identifier("msg"),
            None,
        ));
        msg.borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(msg.clone()).downgrade());

        let mut block = RcCell::new(StateVariableDeclaration::new(
            Some(AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![RcCell::new(
                        global_defs
                            .borrow()
                            .block_struct
                            .borrow()
                            .idf()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .clone_inner(),
                    )],
                    Some(ASTFlatten::from(global_defs.borrow().block_struct.clone()).downgrade()),
                )
                .to_type_name(),
            )),
            vec![],
            Identifier::identifier("block"),
            None,
        ));
        block
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(block.clone()).downgrade());

        let mut tx = RcCell::new(StateVariableDeclaration::new(
            Some(AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![RcCell::new(
                        global_defs
                            .borrow()
                            .tx_struct
                            .borrow()
                            .idf()
                            .unwrap()
                            .borrow()
                            .clone_inner(),
                    )],
                    Some(ASTFlatten::from(global_defs.borrow().tx_struct.clone()).downgrade()),
                )
                .to_type_name(),
            )),
            vec![],
            Identifier::identifier("tx"),
            None,
        ));
        tx.borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(tx.clone()).downgrade());

        let mut now = RcCell::new(StateVariableDeclaration::new(
            Some(AnnotatedTypeName::uint_all()),
            vec![],
            Identifier::identifier("now"),
            None,
        ));
        now.borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(now.clone()).downgrade());
        Self {
            msg,
            block,
            tx,
            now,
            global_defs,
        }
    }
    pub fn vars(&self) -> Vec<RcCell<StateVariableDeclaration>> {
        vec![
            self.msg.clone(),
            self.block.clone(),
            self.tx.clone(),
            self.now.clone(),
        ]
    }
}
