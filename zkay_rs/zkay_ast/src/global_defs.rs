#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// # BUILTIN SPECIAL TYPE DEFINITIONS
use crate::ast::{
    ASTBaseMutRef, ASTBaseRef, ASTFlatten, AnnotatedTypeName, Block,
    ConstructorOrFunctionDefinition, FunctionTypeName, Identifier, IdentifierBase, IntoAST,
    NamespaceDefinitionBaseProperty, Parameter, StateVariableDeclaration, StructDefinition,
    StructTypeName, TypeName, UserDefinedTypeName, VariableDeclaration,
};
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
        AnnotatedTypeName::uint_all(),
        Identifier::identifier("length"),
        None,
    ))
    .into()
}

pub fn global_defs() -> GlobalDefs {
    GlobalDefs::new()
}
pub fn global_vars() -> GlobalVars {
    GlobalVars::new()
}
// lazy_static! {
// pub static ref ARRAY_LENGTH_MEMBER: Arc<Mutex<VariableDeclaration>> = Arc::new(Mutex::new(VariableDeclaration::new(
//     vec![],
//     AnnotatedTypeName::uint_all(),
//     Identifier::identifier("length"),
//     None
// )));
// pub static ref global_defs(): GlobalDefs = GlobalDefs::new();
// pub static ref global_vars(): GlobalVars = GlobalVars::new();
// }
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
            vec![VariableDeclaration::new(
                vec![],
                AnnotatedTypeName::uint_all(),
                Identifier::identifier("balance"),
                None,
            )
            .to_ast()],
        ));
        set_parents(&address_struct.clone().into());

        let mut address_payable_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<address_payable>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("balance"),
                    None,
                )
                .to_ast(),
                ConstructorOrFunctionDefinition::new(
                    Identifier::identifier("send"),
                    Some(vec![RcCell::new(Parameter::new(
                        vec![],
                        AnnotatedTypeName::uint_all(),
                        Identifier::identifier(""),
                        None,
                    ))]),
                    Some(vec![String::from("public")]),
                    Some(vec![RcCell::new(Parameter::new(
                        vec![],
                        AnnotatedTypeName::bool_all(),
                        Identifier::identifier(""),
                        None,
                    ))]),
                    Some(Block::new(vec![], false)),
                )
                .to_ast(),
                ConstructorOrFunctionDefinition::new(
                    Identifier::identifier("transfer"),
                    Some(vec![RcCell::new(Parameter::new(
                        vec![],
                        AnnotatedTypeName::uint_all(),
                        Identifier::identifier(""),
                        None,
                    ))]),
                    Some(vec![String::from("public")]),
                    Some(vec![]),
                    Some(Block::new(vec![], false)),
                )
                .to_ast(),
            ],
        ));
        address_payable_struct.borrow_mut().members[1]
            .borrow_mut()
            .try_as_namespace_definition_mut()
            .unwrap()
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .can_be_private = false;
        address_payable_struct.borrow_mut().members[2]
            .borrow_mut()
            .try_as_namespace_definition_mut()
            .unwrap()
            .try_as_constructor_or_function_definition_mut()
            .unwrap()
            .can_be_private = false;
        set_parents(&address_payable_struct.clone().into());

        let msg_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<msg>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::address_payable_type())),
                        None,
                        String::from("NON_HOMOMORPHISM"),
                    )),
                    Identifier::identifier("sender"),
                    None,
                )
                .to_ast(),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("value"),
                    None,
                )
                .to_ast(),
            ],
        ));
        set_parents(&msg_struct.clone().into());

        let block_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<block>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::address_payable_type())),
                        None,
                        String::from("NON_HOMOMORPHISM"),
                    )),
                    Identifier::identifier("coinbase"),
                    None,
                )
                .to_ast(),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("difficulty"),
                    None,
                )
                .to_ast(),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("gaslimit"),
                    None,
                )
                .to_ast(),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("number"),
                    None,
                )
                .to_ast(),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("timestamp"),
                    None,
                )
                .to_ast(),
            ],
        ));
        set_parents(&block_struct.clone().into());

        let tx_struct = RcCell::new(StructDefinition::new(
            Identifier::identifier("<tx>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("gasprice"),
                    None,
                )
                .to_ast(),
                VariableDeclaration::new(
                    vec![],
                    RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::address_payable_type())),
                        None,
                        String::from("NON_HOMOMORPHISM"),
                    )),
                    Identifier::identifier("origin"),
                    None,
                )
                .to_ast(),
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
}
// class GlobalVars:
impl GlobalVars {
    pub fn new() -> Self {
        let mut msg = RcCell::new(StateVariableDeclaration::new(
            AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![global_defs()
                        .msg_struct
                        .borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .clone()],
                    Some(ASTFlatten::from(global_defs().msg_struct.clone()).downgrade()),
                )
                .to_type_name(),
            ),
            vec![],
            Identifier::identifier("msg"),
            None,
        ));
        msg.borrow_mut()
            .identifier_declaration_base
            .idf
            .as_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(msg.clone()).downgrade());

        let mut block = RcCell::new(StateVariableDeclaration::new(
            AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![global_defs()
                        .block_struct
                        .borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .clone()],
                    Some(ASTFlatten::from(global_defs().block_struct.clone()).downgrade()),
                )
                .to_type_name(),
            ),
            vec![],
            Identifier::identifier("block"),
            None,
        ));
        block
            .borrow_mut()
            .identifier_declaration_base
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(block.clone()).downgrade());

        let mut tx = RcCell::new(StateVariableDeclaration::new(
            AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![global_defs()
                        .tx_struct
                        .borrow()
                        .namespace_definition_base
                        .idf()
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .clone()],
                    Some(ASTFlatten::from(global_defs().tx_struct.clone()).downgrade()),
                )
                .to_type_name(),
            ),
            vec![],
            Identifier::identifier("tx"),
            None,
        ));
        tx.borrow_mut()
            .identifier_declaration_base
            .idf
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(tx.clone()).downgrade());

        let mut now = RcCell::new(StateVariableDeclaration::new(
            AnnotatedTypeName::uint_all(),
            vec![],
            Identifier::identifier("now"),
            None,
        ));
        now.borrow_mut()
            .identifier_declaration_base
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
