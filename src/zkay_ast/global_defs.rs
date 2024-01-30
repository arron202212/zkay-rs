// # BUILTIN SPECIAL TYPE DEFINITIONS
use crate::zkay_ast::ast::{
    AnnotatedTypeName, Block, ConstructorOrFunctionDefinition, FunctionTypeName, Identifier,
    IdentifierBase, Parameter, StateVariableDeclaration, StructDefinition, StructTypeName,
    TypeName, UserDefinedTypeName, VariableDeclaration,
};
use crate::zkay_ast::pointers::parent_setter::set_parents;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref ARRAY_LENGTH_MEMBER: VariableDeclaration = VariableDeclaration::new(
        vec![],
        AnnotatedTypeName::uint_all(),
        Identifier::identifier(IdentifierBase::new(String::from("length"))),
        None
    );
    pub static ref GLOBAL_DEFS: GlobalDefs = GlobalDefs::new();
    pub static ref GLOBAL_VARS: GlobalVars = GlobalVars::new();
}
pub struct GlobalDefs {
    address_struct: StructDefinition,
    address_payable_struct: StructDefinition,
    msg_struct: StructDefinition,
    block_struct: StructDefinition,
    tx_struct: StructDefinition,
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
        let address_struct: StructDefinition = StructDefinition::new(
            Identifier::identifier("<address>"),
            vec![VariableDeclaration::new(
                vec![],
                AnnotatedTypeName::uint_all(),
                Identifier::identifier("balance"),
            )],
        );
        set_parents(address_struct);

        let mut address_payable_struct: StructDefinition = StructDefinition::new(
            Identifier::identifier("<address_payable>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("balance"),
                ),
                ConstructorOrFunctionDefinition::new(
                    Some(Identifier::identifier("send")),
                    Some(vec![Parameter::new(
                        vec![],
                        AnnotatedTypeName::uint_all(),
                        Identifier::identifier(String::new()),
                        None,
                    )]),
                    Some(vec![String::from("public")]),
                    Some(vec![Parameter::new(
                        vec![],
                        AnnotatedTypeName::bool_all(),
                        Identifier::identifier(String::new()),
                        None,
                    )]),
                    Some(Block::new(vec![], false)),
                ),
                ConstructorOrFunctionDefinition::new(
                    Some(Identifier::identifier("transfer")),
                    Some(vec![Parameter::new(
                        vec![],
                        AnnotatedTypeName::uint_all(),
                        Identifier::identifier(String::new()),
                        None,
                    )]),
                    Some(vec![String::from("public")]),
                    Some(vec![]),
                    Some(Block::new(vec![], false)),
                ),
            ],
        );
        address_payable_struct.members[1].can_be_private = false;
        address_payable_struct.members[2].can_be_private = false;
        set_parents(address_payable_struct);

        let msg_struct: StructDefinition = StructDefinition::new(
            Identifier::identifier("<msg>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::new(TypeName::address_payable_type()),
                    Identifier::identifier("sender"),
                ),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("value"),
                ),
            ],
        );
        set_parents(msg_struct);

        let block_struct: StructDefinition = StructDefinition::new(
            Identifier::identifier("<block>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::new(TypeName::address_payable_type()),
                    Identifier::identifier("coinbase"),
                ),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("difficulty"),
                ),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("gaslimit"),
                ),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("number"),
                ),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("timestamp"),
                ),
            ],
        );
        set_parents(block_struct);

        let tx_struct: StructDefinition = StructDefinition::new(
            Identifier::identifier("<tx>"),
            vec![
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::uint_all(),
                    Identifier::identifier("gasprice"),
                ),
                VariableDeclaration::new(
                    vec![],
                    AnnotatedTypeName::new(TypeName::address_payable_type()),
                    Identifier::identifier("origin"),
                ),
            ],
        );
        set_parents(tx_struct);
        Self {
            address_struct,
            address_payable_struct,
            msg_struct,
            block_struct,
            tx_struct,
        }
    }
    pub fn vars(&self) -> Vec<StructDefinition> {
        vec![
            self.address_struct.clone(),
            self.address_payable_struct.clone(),
            self.msg_struct.clone(),
            self.block_struct.clone(),
            self.tx_struct.clone(),
        ]
    }
}

struct GlobalVars {
    msg: StateVariableDeclaration,
    block: StateVariableDeclaration,
    tx: StateVariableDeclaration,
    now: StateVariableDeclaration,
}
// class GlobalVars:
impl GlobalVars {
    pub fn new() -> Self {
        let mut msg: StateVariableDeclaration = StateVariableDeclaration::new(
            AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![GLOBAL_DEFS.msg_struct.idf.clone()],
                    Some(GLOBAL_DEFS.msg_struct.to_namespace_definition()),
                )
                .to_type_name(),
            ),
            vec![],
            Identifier::identifier("msg"),
            None,
        );
        msg.idf.parent = msg;

        let mut block: StateVariableDeclaration = StateVariableDeclaration::new(
            AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![GLOBAL_DEFS.block_struct.idf.clone()],
                    GLOBAL_DEFS.block_struct.to_namespace_definition(),
                )
                .to_type_name(),
            ),
            vec![],
            Identifier::identifier("block"),
            None,
        );
        block.idf.parent = block;

        let mut tx: StateVariableDeclaration = StateVariableDeclaration::new(
            AnnotatedTypeName::all(
                StructTypeName::new(
                    vec![GLOBAL_DEFS.tx_struct.idf.clone()],
                    GLOBAL_DEFS.tx_struct.to_namespace_definition(),
                )
                .to_type_name(),
            ),
            vec![],
            Identifier::identifier("tx"),
            None,
        );
        tx.idf.parent = tx;

        let mut now: StateVariableDeclaration = StateVariableDeclaration::new(
            AnnotatedTypeName::uint_all(),
            vec![],
            Identifier::identifier("now"),
            None,
        );
        now.idf.parent = now;
        Self {
            msg,
            block,
            tx,
            now,
        }
    }
    pub fn vars(&self) -> Vec<StateVariableDeclaration> {
        vec![
            self.msg.clone(),
            self.block.clone(),
            self.tx.clone(),
            self.now.clone(),
        ]
    }
}
