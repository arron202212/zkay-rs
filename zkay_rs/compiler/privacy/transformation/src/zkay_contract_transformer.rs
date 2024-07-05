#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// """
// This module provides functionality to transform a zkay AST into an equivalent public solidity AST + proof circuits
// """
use crate::internal_call_transformer::{
    compute_transitive_circuit_io_sizes, transform_internal_calls,
};
use crate::zkay_transformer::{
    ZkayCircuitTransformer, ZkayExpressionTransformer, ZkayStatementTransformer,
    ZkayVarDeclTransformer,
};
use circuit_helper::circuit_helper::CircuitHelper;
use privacy::library_contracts::BN128_SCALAR_FIELD;
use rccell::{RcCell, WeakCell};
use std::collections::BTreeMap;
use zkay_ast::analysis::used_homomorphisms::UsedHomomorphismsVisitor;
use zkay_ast::ast::{
    is_instance, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTFlatten, ASTInstanceOf, ASTType,
    AnnotatedTypeName, Array, ArrayBase, ArrayLiteralExpr, ArrayLiteralExprBase,
    AssignmentStatement, AssignmentStatementBase, AssignmentStatementBaseMutRef, BlankLine, Block,
    CipherText, Comment, CommentBase, ConstructorOrFunctionDefinition, ContractDefinition,
    ContractTypeName, ExprUnion, Expression, ExpressionASType, ExpressionStatement,
    FunctionCallExpr, FunctionCallExprBase, HybridArgumentIdf, Identifier, IdentifierBase,
    IdentifierBaseProperty, IdentifierBaseRef, IdentifierDeclaration, IdentifierExpr,
    IdentifierExprUnion, IndexExpr, IntoAST, IntoExpression, IntoStatement, LocationExpr, MeExpr,
    NamespaceDefinition, NewExpr, NumberLiteralExpr, Parameter, PrimitiveCastExpr,
    RequireStatement, ReturnStatement, SourceUnit, StateVariableDeclaration, Statement,
    StatementList, StatementListBase, StructDefinition, StructTypeName, TupleExpr, TypeName,
    UserDefinedTypeName, VariableDeclaration, VariableDeclarationStatement, AST,
};
use zkay_ast::global_defs::{
    array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars,
};
use zkay_ast::pointers::{parent_setter::set_parents, symbol_table::link_identifiers};
use zkay_ast::visitors::{
    deep_copy::deep_copy,
    transformer_visitor::{
        AstTransformerVisitor, AstTransformerVisitorBase, AstTransformerVisitorBaseRef,
        TransformerVisitorEx,
    },
    visitor::AstVisitor,
};

use zkay_config::config::CFG;
use zkay_crypto::params::CryptoParams;
use zkay_derive::AstTransformerVisitorBaseRefImpl;
// """
// Convert zkay to solidity AST + proof circuits

// :param ast: zkay AST
// :return: solidity AST and dictionary which maps all function definitions which require verification
//          to the corresponding circuit helper instance.
// """
pub fn transform_ast(
    ast: Option<ASTFlatten>,
    global_vars: RcCell<GlobalVars>,
) -> (
    ASTFlatten,
    BTreeMap<RcCell<ConstructorOrFunctionDefinition>, RcCell<CircuitHelper>>,
) {
    let zt = ZkayTransformer::new();
    // // println!("=============1======");
    let mut new_ast = zt.visit(ast.as_ref().unwrap());
    //// println!("======2=======1======");
    // restore all parent pointers and identifier targets
    set_parents(new_ast.as_ref().unwrap());
    //// println!("======2===2====1======");
    link_identifiers(new_ast.as_ref().unwrap(), global_vars.clone());
    //// println!("======2====3===1======");
    let circuits = zt.circuits.borrow().clone();
    (new_ast.unwrap(), circuits)
}

// class ZkayTransformer(AstTransformerVisitor)
// """
// Transformer which transforms contract level AST elements (contract, function definitions, constructor definitions)

// Contract level transformations

// * Import public key infrastructure contract and make it available as public constant state variable
// * | Import verification contracts for all functions which require verification and make them available as public constant state variables
//   | Note: This transformations initializes those state variables with address 0, which is a placeholder. 0 is replaced with the \
//           real address upon deployment.
// * Transform state variable declarations with owner != @all (replace type by cipher type)
// * For every function and constructor, the parameters and the body are transformed using the transformers defined in zkay_transformer.py

// To support verification, the functions themselves also need additional transformations

// In the original zkay paper, all the circuit out parameters + the proof are added as additional parameters for all functions which
// require verification.
// This makes it impossible to simply call another function, since every function expects its out arguments + a proof.

// Zkay 2.0 uses an improved design, with the goal of supporting function calls in an elegant way.
// It is based on the following observations

// 1) zk proof verification is only possible in functions which are called externally via a transaction,
//    as it requires offchain simulation to generate a valid zero knowledge proof.
// 2) public functions can be called externally (transaction) as well as internally (called from other function)
// 3) private and internal functions can only be called internally
// 4) public functions which have private arguments, but don"t contain any private expressions in their body (e.g. because they only
//    contain assignments, which are public operations as long as the owner does not change), only need verification if they are called
//    externally (since then the parameters are user supplied and thus their encryption needs to be verified)
// 5) The difference between an external and an internal function can be reduced to argument encryption verification +
//    proof verification via verification contract invocation

// From 1) follows, that the externally called function must also handle the verification of all transitively called functions
// Observations 2), 4) and 5) suggest, that it is sensible to split each public function into two different parts

//  a) An internal function which has the original function body and arguments
//  b) An external function which does argument verification, calls the internal function and finally and invokes the verification contract
//     (=> "External Wrapper function")

// This way, calling a public function from within another function works exactly the same as calling a private/internal function,
// zkay simply has to reroute the call to the internal function.
// It also means, that no resources are wasted when calling a function such as mentioned in 4) from another function, since in that case
// the internal function does not require verification.

// What"s left is how to deal with 1). Zkay 2.0 uses the following solution

// * | If a function is purely public (no private arguments, no private expressions in itself or any transitively called functions)
//   | => No change in signature and no additional transformations
// * If an internal function requires verification (+), 4 additional arguments are added to its signature

//       1) a variable length array where public circuit inputs generated by this function should be stored
//       2) a start index which determines at which index this function should start storing circuit inputs into the in array
//       3) a variable length array containing public circuit outputs required in this function
//       4) a start index which determines where in the uint array the out values for the current function call are located

//   * A struct definition is added to the contract definition, which includes entries for every circuit and input variable with correct types.
//   * At the beginning of the internal function, a variable of that struct type is declared and all circuit output variables from the out array \
//     parameter are deserialized into the struct.
//   * Within the function body, all circuit inputs are stored into the struct and outputs are read from the struct.
//   * At the end of the internal function, all circuit input variables in the struct are serialized into the in array parameter.

//   When a function calls another function which requires verification, the start indices for in and out array are advanced such that
//   they point to the correct sections and the in/out arrays + new start indices are added to the arguments of the call.
//   If the called function does not require verification, it is simply called without any additional arguments.

//   | (+) An internal function requires verification if it contains private expressions.
//   |     *Note*: a function body can contain private variables (!= @all) without containing private expressions, \
//               since assignment of encrypted variables with the same owner is a public operation.

// * If a function is an external wrapper, 2 additional arguments are added to its signature

//       1) a variable length array containing public circuit outputs for the function itself and all transitively called functions
//          If we have a call hierarchy like this:

//             func f()
//                 calls g(x) which calls h(x)
//                 calls h(x)

//          then the layout in the output array (same for the input array defined below) will be: f outs | g outs | h outs | h outs
//          (i.e. the current functions circuit outputs come first, followed by those of the called functions in the function call order
//          (according to AST traversal order))
//       2) a zero knowledge proof

//   * At the beginning of the external wrapper function a dynamic array is allocated which is large enough to store all circuit inputs
//     from all transitively called functions.
//   * Next all encrypted arguments are stored in the in array (since the circuit will verify the encryption)
//   * Then the wrapper requests all statically known public keys (key for me or for a final address state variable), required by any
//     of the transitively called functions, and also stores them in the in array.
//   * The corresponding internal function is then called.
//     If it requires verification, the newly allocated in array + the out array parameter + initial start indices
//     (0 for out array, after last key for in array) are added as additional arguments.
//   * Finally the verification contract is invoked to verify the proof (the in array was populated by the called functions themselves).
// """
#[derive(Clone, AstTransformerVisitorBaseRefImpl)]
pub struct ZkayTransformer {
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    circuits: RcCell<BTreeMap<RcCell<ConstructorOrFunctionDefinition>, RcCell<CircuitHelper>>>,
    var_decl_trafo: ZkayVarDeclTransformer,
}
impl AstTransformerVisitor for ZkayTransformer {
    // fn default() -> Self {
    //     Self::new()
    // }

    fn has_attr(&self, ast: &AST) -> bool {
        //println!("======has_attr========={:?}", ast.get_ast_type());
        matches!(ast.get_ast_type(), ASTType::SourceUnit | ASTType::ASTBase)
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        //println!("======get_attr========={:?}", name);
        match name {
            ASTType::SourceUnit => self.visitSourceUnit(ast),
            ASTType::ASTBase => <Self as AstTransformerVisitor>::visitAST(self, ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl ZkayTransformer {
    // pub fn __init__(self)
    //     super().__init__()
    //     self.circuits: Dict[ConstructorOrFunctionDefinition, CircuitHelper] = {}
    //     """Abstract circuits for all functions which require verification"""

    //     self.var_decl_trafo = ZkayVarDeclTransformer()
    //     """Transformer for state variable declarations and parameters"""
    pub fn new() -> Self {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            circuits: RcCell::new(BTreeMap::new()),
            var_decl_trafo: ZkayVarDeclTransformer::new(),
        }
    }
    // """
    // Import contract "vname" into the given source unit.

    // :param cname: contract name (.sol filename stem must match contract type name)
    // :param su: [SIDE EFFECT] source unit where contract should be imported
    // :param corresponding_circuit: [SIDE EFFECT] if contract is a verification contract, this should be the corresponding circuit helper
    // """
    pub fn import_contract(
        cname: &str,
        su: &ASTFlatten,
        corresponding_circuit: Option<&RcCell<CircuitHelper>>,
    ) {
        let import_filename = format!("./{cname}.sol");
        su.try_as_source_unit_ref()
            .unwrap()
            .borrow_mut()
            .used_contracts
            .push(import_filename.clone());

        if let Some(corresponding_circuit) = corresponding_circuit {
            let c_type = ContractTypeName::new(
                vec![Identifier::Identifier(IdentifierBase::new(
                    cname.to_string(),
                ))],
                None,
            );
            corresponding_circuit
                .borrow()
                .register_verification_contract_metadata(
                    TypeName::UserDefinedTypeName(UserDefinedTypeName::ContractTypeName(
                        c_type.clone(),
                    )),
                    &import_filename,
                );
        }
    }
    // """Create a public constant state variable with which contract with name "cname" can be accessed"""
    pub fn create_contract_variable(cname: &str) -> ASTFlatten {
        let inst_idf = Identifier::Identifier(IdentifierBase::new(
            CFG.lock().unwrap().get_contract_var_name(cname.to_string()),
        ));
        let c_type = ContractTypeName::new(
            vec![Identifier::Identifier(IdentifierBase::new(
                cname.to_string(),
            ))],
            None,
        );

        let cast_0_to_c = RcCell::new(PrimitiveCastExpr::new(
            RcCell::new(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::ContractTypeName(c_type.clone()),
            )),
            RcCell::new(NumberLiteralExpr::new(0, false)).into(),
            false,
        ));
        RcCell::new(StateVariableDeclaration::new(
            Some(RcCell::new(AnnotatedTypeName::new(
                Some(RcCell::new(TypeName::UserDefinedTypeName(
                    UserDefinedTypeName::ContractTypeName(c_type.clone()),
                ))),
                None,
                String::from("NON_HOMOMORPHIC"),
            ))),
            zkay_config::lc_vec_s!["public", "constant"],
            Some(RcCell::new(inst_idf.clone())),
            Some(cast_0_to_c.into()),
        ))
        .into()
    }
    // """
    // Import all verification contracts for "c" into "su" and create state variable declarations for all of them + the pki contract.

    // :param su: [SIDE EFFECT] source unit into which contracts should be imported
    // :param c: contract for which verification contracts should be imported
    // :return: list of all constant state variable declarations for the pki contract + all the verification contracts
    // """
    pub fn include_verification_contracts(
        &self,
        su: &ASTFlatten,
        c: &RcCell<ContractDefinition>,
    ) -> Vec<ASTFlatten> {
        //// println!("-====================");
        let mut contract_var_decls = vec![];
        for crypto_params in c.borrow().used_crypto_backends.clone().unwrap() {
            let contract_name = CFG
                .lock()
                .unwrap()
                .get_pki_contract_name(&crypto_params.identifier_name());
            // // println!("-=============111=======");
            contract_var_decls.push(Self::create_contract_variable(&contract_name));
        }
        // // println!("-====111====line=========={}==",line!());
        for f in c
            .borrow()
            .constructor_definitions
            .iter()
            .chain(&c.borrow().function_definitions)
        {
            // // println!("-====for====line=========={}==",line!());
            if f.borrow().requires_verification_when_external && f.borrow().has_side_effects() {
                // // println!("-====for====line=====lock==bef==={}==",line!());
                let name = CFG.lock().unwrap().get_verification_contract_name(
                    c.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    f.borrow().name(),
                );
                // // println!("-====for====line====lock======{}==",line!());
                Self::import_contract(&name, su, self.circuits.borrow().get(&f));
                contract_var_decls.push(Self::create_contract_variable(&name));
            }
        }
        // // println!("-===end=====line=========={}==",line!());
        contract_var_decls
    }

    // """
    // Create circuit helper for the given function.

    // :param fct: function for which to create a circuit
    // :param global_owners: list of all statically known privacy labels (me + final address state variables)
    // :param internal_circ: the circuit of the internal function on which to base this circuit
    //                       (only used when creating the circuit of the external wrapper function)
    // :return: new circuit helper
    // """
    pub fn create_circuit_helper(
        fct: &RcCell<ConstructorOrFunctionDefinition>,
        global_owners: Vec<ASTFlatten>,
        internal_circ: Option<&RcCell<CircuitHelper>>,
    ) -> RcCell<CircuitHelper> {
        //// println!("=====create_circuit_helper==before=={}=", line!());
        CircuitHelper::new(
            fct.clone(),
            global_owners,
            |ch: &WeakCell<CircuitHelper>| {
                //// println!("=====create_circuit_helper==before=={}=", line!());
                Some(Box::new(ZkayExpressionTransformer::new(Some(ch.clone()))))
            },
            |ch: &WeakCell<CircuitHelper>| {
                //// println!("=====create_circuit_helper==before=={}=", line!());
                Some(Box::new(ZkayCircuitTransformer::new(Some(ch.clone()))))
            },
            internal_circ.map(|c| c.downgrade()),
        )
    }
    // Figure out which crypto backends were used
    pub fn visitSourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        //// println!("==visitSourceUnit=========={:?}",ast.get_ast_type());
        UsedHomomorphismsVisitor::new().visit(ast);
        // println!("=====visitSourceUnit======{}=={}==", file!(), line!());
        let used_crypto_backends = ast
            .try_as_source_unit_ref()
            .unwrap()
            .borrow()
            .used_crypto_backends
            .as_ref()
            .unwrap()
            .clone();
        for crypto_params in used_crypto_backends {
            Self::import_contract(
                &CFG.lock()
                    .unwrap()
                    .get_pki_contract_name(&crypto_params.identifier_name()),
                ast,
                None,
            );
        }
        //println!("=====visitSourceUnit======2====");
        let mut contracts = ast
            .try_as_source_unit_ref()
            .unwrap()
            .borrow()
            .contracts
            .clone();
        for c in &contracts {
            // println!("=====visitSourceUnit====={:?}===", c.get_ast_type());
            self.transform_contract(ast, c);
        }
        // println!("=====visitSourceUnit======3====");
        ast.try_as_source_unit_ref().unwrap().borrow_mut().contracts = contracts;
        // panic!("=-===");
        Ok(ast.clone())
    }
    // """
    // Transform an entire zkay contract into a public solidity contract.

    // This

    // * transforms state variables, function bodies and signatures
    // * import verification contracts
    // * adds zk_data structs for each function with verification \
    //   (to store circuit I/O, to bypass solidity stack limit and allow for easy assignment of array variables),
    // * creates external wrapper functions for all public functions which require verification
    // * adds circuit IO serialization/deserialization code from/to zk_data struct to all functions which require verification.

    // :param su: [SIDE EFFECTS] Source unit of which this contract is part of
    // :param c: [SIDE EFFECTS] The contract to transform
    // :return: The contract itself
    // """
    pub fn transform_contract(
        &self,
        su: &ASTFlatten,
        c: &RcCell<ContractDefinition>,
    ) -> RcCell<ContractDefinition> {
        // println!("=====transform_contract===={}=", line!());
        let mut all_fcts: Vec<_> = c
            .borrow()
            .constructor_definitions
            .iter()
            .chain(&c.borrow().function_definitions)
            .cloned()
            .collect();

        // Get list of static owner labels for this contract
        let mut global_owners = vec![RcCell::new(Expression::me_expr(None)).into()];
        for var in &c.borrow().state_variable_declarations {
            if is_instance(var, ASTType::StateVariableDeclaration) {
                if var
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_address()
                    && (var
                        .to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .try_as_state_variable_declaration_ref()
                        .unwrap()
                        .identifier_declaration_base
                        .is_final()
                        || var
                            .to_ast()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .try_as_state_variable_declaration_ref()
                            .unwrap()
                            .identifier_declaration_base
                            .is_constant())
                {
                    global_owners.push(
                        var.to_ast()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .try_as_state_variable_declaration_ref()
                            .unwrap()
                            .idf()
                            .clone()
                            .unwrap()
                            .into(),
                    );
                }
            }
        }
        // println!("=====transform_contract===={}=", line!());
        // Backup untransformed function bodies
        for fct in &all_fcts {
            let body = fct.borrow().body.clone(); //deep_copy(fct.body, true, true);
            fct.borrow_mut().original_body = body;
        }

        // Transform types of normal state variables
        let state_variable_declarations = self.var_decl_trafo.visit_list(
            &c.borrow()
                .state_variable_declarations
                .iter()
                .cloned()
                .map(Into::<ASTFlatten>::into)
                .collect::<Vec<_>>(),
        );
        c.borrow_mut().state_variable_declarations = state_variable_declarations;
        // println!("=====transform_contract===={}=", line!());
        // Split into functions which require verification and those which don"t need a circuit helper
        let mut req_ext_fcts =
            BTreeMap::<RcCell<ConstructorOrFunctionDefinition>, Vec<RcCell<Parameter>>>::new();
        let (mut new_fcts, mut new_constr) = (vec![], vec![]);
        for fct in &all_fcts {
            assert!(is_instance(fct, ASTType::ConstructorOrFunctionDefinition));
            if fct.borrow().requires_verification
                || fct.borrow().requires_verification_when_external
            {
                //println!("=====transform_contract==before=={}=", line!());
                self.circuits.borrow_mut().insert(
                    fct.clone(),
                    Self::create_circuit_helper(fct, global_owners.clone(), None),
                );
                //println!("=====transform_contract==after=={}=", line!());
            }

            if fct.borrow().requires_verification_when_external {
                req_ext_fcts.insert(fct.clone(), fct.borrow().parameters.clone());
            } else if fct.borrow().is_constructor() {
                new_constr.push(fct.clone());
            } else {
                new_fcts.push(fct.clone());
            }
        }
        // println!("=====transform_contract===={}=", line!());
        // Add constant state variables for external contracts and field prime
        let field_prime_decl = StateVariableDeclaration::new(
            Some(AnnotatedTypeName::uint_all()),
            zkay_config::lc_vec_s!["public", "constant"],
            Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
                CFG.lock().unwrap().field_prime_var_name(),
            )))),
            Some(
                RcCell::new(NumberLiteralExpr::new_string(
                    BN128_SCALAR_FIELD.to_hex_string(),
                ))
                .into(),
            ),
        );
        let contract_var_decls = self.include_verification_contracts(su, &*c);
        let mut svd = vec![
            RcCell::new(field_prime_decl).into(),
            RcCell::new(CommentBase::new(String::new())).into(),
        ];
        svd.extend(CommentBase::comment_list(
            String::from("Helper Contracts"),
            contract_var_decls,
        ));
        svd.push(RcCell::new(CommentBase::new(String::from("User state variables"))).into());
        svd.extend(
            c.borrow_mut()
                .state_variable_declarations
                .drain(..)
                .collect::<Vec<_>>(),
        );
        c.borrow_mut().state_variable_declarations = svd;
        // println!("=====transform_contract===={}=", line!());
        // Transform signatures
        for f in all_fcts.iter_mut() {
            let parameters: Vec<_> = self
                .var_decl_trafo
                .visit_list(
                    &f.borrow()
                        .parameters
                        .iter()
                        .map(|p| p.clone().into())
                        .collect::<Vec<_>>(),
                )
                .into_iter()
                .filter(|p| is_instance(p, ASTType::Parameter))
                .filter_map(|p| p.try_as_parameter())
                .collect();
            f.borrow_mut().parameters = parameters;
        }
        // println!("=====transform_contract===={}=", line!());
        for f in c.borrow_mut().function_definitions.iter_mut() {
            let return_parameters: Vec<_> = self
                .var_decl_trafo
                .visit_list(
                    &f.borrow()
                        .return_parameters
                        .iter()
                        .map(|p| p.clone().into())
                        .collect::<Vec<_>>(),
                )
                .into_iter()
                .filter(|p| is_instance(p, ASTType::Parameter))
                .filter_map(|p| p.try_as_parameter())
                .collect();
            f.borrow_mut().return_parameters = return_parameters;
            let return_var_decls: Vec<_> = self
                .var_decl_trafo
                .visit_list(
                    &f.borrow()
                        .return_var_decls
                        .iter()
                        .map(|p| p.clone().into())
                        .collect::<Vec<_>>(),
                )
                .into_iter()
                .filter_map(|p| p.try_as_variable_declaration())
                .collect();
            f.borrow_mut().return_var_decls = return_var_decls;
        }
        // println!("=====transform_contract===={}=", line!());
        // Transform bodies
        for fct in all_fcts.iter_mut() {
            if let Some(circuit) = self.circuits.borrow().get(fct) {
                let body = fct.borrow().body.clone();
                fct.borrow_mut().body =
                    ZkayStatementTransformer::new(Some(circuit.clone().downgrade()))
                        .visit(&body.clone().unwrap().into())
                        .and_then(|b| b.try_as_block());
            }
        }
        // println!("=====transform_contract===={}=", line!());
        // Transform (internal) functions which require verification (add the necessary additional parameters and boilerplate code)
        let mut fcts_with_verification: Vec<_> = all_fcts
            .iter()
            .filter(|fct| fct.borrow().requires_verification)
            .cloned()
            .collect();
        compute_transitive_circuit_io_sizes(&mut fcts_with_verification, &self.circuits);
        // println!("=====transform_contract===={}=", line!());
        transform_internal_calls(&fcts_with_verification, &self.circuits);
        // println!("=====transform_contract===={}=", line!());
        for f in &fcts_with_verification {
            let circuit = self.circuits.borrow()[&f].clone();
            assert!(circuit.borrow().requires_verification());
            // println!("=====transform_contract===={}=", line!());
            if circuit.borrow().requires_zk_data_struct() {
                // Add zk data struct for f to contract
                let output_idfs = circuit
                    .borrow()
                    .output_idfs()
                    .iter()
                    .chain(&circuit.borrow().input_idfs())
                    .map(|idf| {
                        RcCell::new(VariableDeclaration::new(
                            vec![],
                            Some(RcCell::new(AnnotatedTypeName::new(
                                Some(idf.t.clone()),
                                None,
                                String::from("NON_HOMOMORPHIC"),
                            ))),
                            Some(RcCell::new(Identifier::HybridArgumentIdf(idf.clone()))),
                            None,
                        ))
                        .into()
                    })
                    .collect();
                // println!("=====transform_contract===={}=", line!());
                let zk_data_struct = StructDefinition::new(
                    Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
                        circuit.borrow().zk_data_struct_name(),
                    )))),
                    output_idfs,
                );
                // println!(
                //     "==44444444444===transform_contract={}==={}=",
                //     line!(),
                //     circuit.borrow().zk_data_struct_name()
                // );
                c.borrow_mut()
                    .struct_definitions
                    .push(RcCell::new(zk_data_struct));
            }
            // println!("=====transform_contract===={}=", line!());
            self.create_internal_verification_wrapper(f);
            // println!("=====transform_contract===={}=", line!());
        }
        // println!("=====transform_contract===={}=", line!());
        // Create external wrapper functions where necessary
        for (f, params) in req_ext_fcts.iter_mut() {
            let (ext_f, int_f) =
                self.split_into_external_and_internal_fct(f, params, global_owners.clone());
            if ext_f.borrow().is_function() {
                new_fcts.push(ext_f);
            } else {
                new_constr.push(ext_f);
            }
            new_fcts.push(int_f);
        }
        // println!("=====transform_contract===={}=", line!());
        c.borrow_mut().constructor_definitions = new_constr;
        c.borrow_mut().function_definitions = new_fcts;
        c.clone()
    }
    // """
    // Add the necessary additional parameters and boiler plate code for verification support to the given function.
    // :param ast: [SIDE EFFECT] Internal function which requires verification
    // """
    pub fn create_internal_verification_wrapper(
        &self,
        ast: &RcCell<ConstructorOrFunctionDefinition>,
    ) {
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        let circuit = self.circuits.borrow()[&ast].clone();
        let mut stmts = vec![];

        let symmetric_cipher_used = ast
            .borrow()
            .used_crypto_backends
            .as_ref()
            .unwrap()
            .iter()
            .any(|backend| backend.is_symmetric_cipher());
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        if symmetric_cipher_used && ast.borrow().modifiers.contains(&String::from("pure")) {
            // Symmetric trafo requires msg.sender access -> change from pure to view

            ast.borrow_mut().modifiers = ast
                .borrow()
                .modifiers
                .iter()
                .map(|modi| (if modi == "pure" { "view" } else { modi }).to_string())
                .collect();
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Add additional params
        ast.borrow_mut().add_param(
            RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into(),
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
            None,
        );
        ast.borrow_mut().add_param(
            AnnotatedTypeName::uint_all().into(),
            IdentifierExprUnion::String(format!("{}_start_idx", CFG.lock().unwrap().zk_in_name())),
            None,
        );
        ast.borrow_mut().add_param(
            RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into(),
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
            None,
        );
        ast.borrow_mut().add_param(
            AnnotatedTypeName::uint_all().into(),
            IdentifierExprUnion::String(format!("{}_start_idx", CFG.lock().unwrap().zk_out_name())),
            None,
        );
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Verify that in/out parameters have correct size
        let out_start_idx = IdentifierExpr::new(
            IdentifierExprUnion::String(format!("{}_start_idx", CFG.lock().unwrap().zk_out_name())),
            None,
        );
        let in_start_idx = IdentifierExpr::new(
            IdentifierExprUnion::String(format!("{}_start_idx", CFG.lock().unwrap().zk_in_name())),
            None,
        );
        let out_var = IdentifierExpr::new(
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
            None,
        );
        let in_var = IdentifierExpr::new(
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
            None,
        )
        .as_type(&RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into());
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        stmts.push(
            RcCell::new(RequireStatement::new(
                RcCell::new(
                    out_start_idx
                        .to_expr()
                        .binop(
                            String::from("+"),
                            NumberLiteralExpr::new(circuit.borrow().out_size_trans(), false)
                                .into_expr(),
                        )
                        .into_expr()
                        .binop(
                            String::from("<="),
                            LocationExpr::IdentifierExpr(out_var)
                                .dot(IdentifierExprUnion::String(String::from("length")))
                                .into_expr(),
                        ),
                )
                .into(),
                None,
            ))
            .into(),
        );
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        stmts.push(
            RcCell::new(RequireStatement::new(
                RcCell::new(
                    in_start_idx
                        .to_expr()
                        .binop(
                            String::from("+"),
                            NumberLiteralExpr::new(circuit.borrow().in_size_trans(), false)
                                .into_expr(),
                        )
                        .into_expr()
                        .binop(
                            String::from("<="),
                            LocationExpr::IdentifierExpr(
                                in_var
                                    .clone()
                                    .try_as_identifier_expr()
                                    .unwrap()
                                    .borrow()
                                    .clone(),
                            )
                            .dot(IdentifierExprUnion::String(String::from("length")).into())
                            .into_expr(),
                        ),
                )
                .into(),
                None,
            ))
            .into(),
        );
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Declare zk_data struct var (if needed)
        if circuit.borrow().requires_zk_data_struct() {
            // println!(
            //     "===@@@@@@@@@@@@@@@@@@@@@@@@==create_internal_verification_wrapper=={}=={}=",
            //     line!(),
            //     circuit.borrow().zk_data_struct_name()
            // );
            let zk_struct_type = StructTypeName::new(
                vec![Identifier::Identifier(IdentifierBase::new(
                    circuit.borrow().zk_data_struct_name(),
                ))],
                None,
            );
            let mut idf = IdentifierBase::new(CFG.lock().unwrap().zk_data_var_name());
            // println!(
            //     "===###############################==create_internal_verification_wrapper={}==={}=",
            //     line!(),
            //     CFG.lock().unwrap().zk_data_var_name()
            // );
            stmts.extend(vec![
                RcCell::new(
                    idf.decl_var(
                        &RcCell::new(TypeName::UserDefinedTypeName(
                            UserDefinedTypeName::StructTypeName(zk_struct_type),
                        ))
                        .into(),
                        None,
                    ),
                )
                .into(),
                RcCell::new(BlankLine::new()).into(),
            ]);
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Declare return variable if necessary
        if !ast.borrow().return_parameters.is_empty() {
            stmts.extend(CommentBase::comment_list(
                String::from("Declare return variables"),
                ast.borrow()
                    .return_var_decls
                    .iter()
                    .map(|vd| {
                        RcCell::new(VariableDeclarationStatement::new(vd.clone(), None)).into()
                    })
                    .collect(),
            ));
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Find all me-keys in the in array
        let mut me_key_idx = BTreeMap::new();
        let mut offset = 0;
        for (key_owner, crypto_params) in circuit.borrow().requested_global_keys() {
            if is_instance(&key_owner.unwrap(), ASTType::MeExpr) {
                //== MeExpr::new()
                assert!(!me_key_idx.contains_key(&crypto_params));
                me_key_idx.insert(crypto_params.clone(), offset);
            }
            offset += crypto_params.key_len();
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Deserialize out array (if any)
        let mut deserialize_stmts = vec![];
        let mut offset = 0;
        let zk_out_name = CFG.lock().unwrap().zk_out_name();
        for s in circuit.borrow().output_idfs().iter_mut() {
            deserialize_stmts.push(s.deserialize(
                zk_out_name.clone(),
                Some(RcCell::new(out_start_idx.clone()).into()),
                offset,
            ));
            //println!("=====create_internal_verification_wrapper===={}=", line!());
            if is_instance(&s.t, ASTType::CipherText)
                && s.t
                    .borrow()
                    .try_as_array_ref()
                    .unwrap()
                    .try_as_cipher_text_ref()
                    .unwrap()
                    .crypto_params
                    .is_symmetric_cipher()
            {
                // Assign sender field to user-encrypted values if necessary
                // Assumption: s.t.crypto_params.key_len == 1 for all symmetric ciphers
                assert!(
                    me_key_idx.contains_key(
                        &s.t.borrow()
                            .try_as_array_ref()
                            .unwrap()
                            .try_as_cipher_text_ref()
                            .unwrap()
                            .crypto_params
                    ),
                    "Symmetric cipher but did not request me key"
                );
                let key_idx = me_key_idx[&s
                    .t
                    .borrow()
                    .try_as_array_ref()
                    .unwrap()
                    .try_as_cipher_text_ref()
                    .unwrap()
                    .crypto_params];
                let sender_key = LocationExpr::IdentifierExpr(
                    in_var
                        .clone()
                        .try_as_identifier_expr()
                        .unwrap()
                        .borrow()
                        .clone(),
                )
                .index(ExprUnion::I32(key_idx));
                let cipher_payload_len =
                    s.t.borrow()
                        .try_as_array_ref()
                        .unwrap()
                        .try_as_cipher_text_ref()
                        .unwrap()
                        .crypto_params
                        .cipher_payload_len();
                deserialize_stmts.push(
                    s.get_loc_expr(None)
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .index(ExprUnion::I32(cipher_payload_len))
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .assign(sender_key),
                );
            }
            offset += s.t.borrow().size_in_uints();
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        if !deserialize_stmts.is_empty() {
            stmts.push(
                RcCell::new(Statement::StatementList(StatementList::StatementList(
                    StatementListBase::new(
                        CommentBase::comment_wrap_block(
                            String::from("Deserialize output values"),
                            deserialize_stmts
                                .into_iter()
                                .map(RcCell::new)
                                .map(Into::<ASTFlatten>::into)
                                .collect(),
                        ),
                        true,
                    ),
                )))
                .into(),
            );
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Include original transformed function body
        stmts.extend(
            ast.borrow()
                .body
                .as_ref()
                .unwrap()
                .borrow()
                .statement_list_base
                .statements
                .iter()
                .map(|s| s.clone_inner())
                .collect::<Vec<_>>(),
        );
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Serialize in parameters to in array (if any)
        let mut serialize_stmts = vec![];
        let mut offset = 0;
        let zk_in_name = CFG.lock().unwrap().zk_in_name();
        for s in circuit.borrow_mut().input_idfs().iter_mut() {
            serialize_stmts.push(s.serialize(
                zk_in_name.clone(),
                Some(RcCell::new(in_start_idx.clone()).into()),
                offset,
            ));
            offset += s.t.borrow().size_in_uints();
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        if offset != 0 {
            stmts.push(RcCell::new(CommentBase::new(String::new())).into());
            stmts.extend(CommentBase::comment_wrap_block(
                String::from("Serialize input values"),
                serialize_stmts
                    .into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
            ));
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // Add return statement at the end if necessary
        // (was previously replaced by assignment to return_var by ZkayStatementTransformer)
        if circuit.borrow().has_return_var {
            stmts.push(
                RcCell::new(ReturnStatement::new(Some(
                    RcCell::new(TupleExpr::new(
                        ast.borrow()
                            .return_var_decls
                            .iter()
                            .map(|vd| {
                                let mut idf = IdentifierExpr::new(
                                    IdentifierExprUnion::Identifier(
                                        vd.borrow().idf().clone().unwrap(),
                                    ),
                                    None,
                                );
                                idf.ast_base_ref().borrow_mut().target =
                                    Some(ASTFlatten::from(vd.clone()).downgrade());
                                RcCell::new(idf).into()
                            })
                            .collect(),
                    ))
                    .into(),
                )))
                .into(),
            );
        }
        //println!("=====create_internal_verification_wrapper===={}=", line!());
        // if ast.get_ast_type() == ASTType::StatementListBase {
        //     if stmts
        //         .iter()
        //         .any(|s| s.get_ast_type() == ASTType::StatementListBase)
        //     {
        //         println!(
        //             "==StatementListBase=======ct==========StatementListBase===={}=",
        //             line!()
        //         );
        //     }
        // }
        ast.borrow_mut()
            .body
            .as_mut()
            .unwrap()
            .borrow_mut()
            .statement_list_base
            .statements = stmts;
    }
    // """
    // Take public function f and split it into an internal function and an external wrapper function.

    // :param f: [SIDE EFFECT] function to split (at least requires_verification_if_external)
    // :param original_params: list of transformed function parameters without additional parameters added due to transformation
    // :param global_owners: list of static labels (me + final address state variable identifiers)
    // :return: Tuple of newly created external and internal function definitions
    // """
    pub fn split_into_external_and_internal_fct(
        &self,
        f: &RcCell<ConstructorOrFunctionDefinition>,
        original_params: &mut Vec<RcCell<Parameter>>,
        global_owners: Vec<ASTFlatten>,
    ) -> (
        RcCell<ConstructorOrFunctionDefinition>,
        RcCell<ConstructorOrFunctionDefinition>,
    ) {
        assert!(f.borrow().requires_verification_when_external);

        // Create new empty function with same parameters as original -> external wrapper
        let mut new_modifiers = if f.borrow().is_function() {
            *original_params = original_params
                .iter()
                .map(|p| {
                    let mut pp = deep_copy(&p.clone().into(), true, false)
                        .unwrap()
                        .to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .try_as_parameter_ref()
                        .unwrap()
                        .clone();
                    RcCell::new(
                        pp.with_changed_storage(String::from("memory"), String::from("calldata"))
                            .clone(),
                    )
                })
                .collect();
            zkay_config::lc_vec_s!["external"]
        } else {
            zkay_config::lc_vec_s!["public"]
        };
        if f.borrow().is_payable() {
            new_modifiers.push(String::from("payable"));
        }

        let mut requires_proof = true;
        if !f.borrow().has_side_effects() {
            requires_proof = false;
            new_modifiers.push(String::from("view"));
        }
        let mut new_f = ConstructorOrFunctionDefinition::new(
            f.borrow().idf().clone(),
            Some(original_params.clone()),
            Some(new_modifiers),
            Some(f.borrow().return_parameters.clone()),
            Some(Block::new(vec![], false)),
        );

        // Make original function internal
        let idf = Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
            CFG.lock().unwrap().get_internal_name(&f.borrow().clone()),
        ))));
        f.borrow_mut().ast_base_mut_ref().borrow_mut().idf = idf;
        let modifiers = f
            .borrow()
            .modifiers
            .iter()
            .filter_map(|modi| {
                (modi != "payable")
                    .then(|| (if modi == "public" { "internal" } else { modi }).to_string())
            })
            .collect();
        f.borrow_mut().modifiers = modifiers;
        f.borrow_mut().requires_verification_when_external = false;
        let mut new_f = RcCell::new(new_f);
        // Create new circuit for external function
        let circuit =
            Self::create_circuit_helper(&new_f, global_owners, self.circuits.borrow().get(f));
        if !f.borrow().requires_verification {
            self.circuits.borrow_mut().remove(f);
        }

        // Set meta attributes and populate body
        new_f.borrow_mut().requires_verification = true;
        new_f.borrow_mut().requires_verification_when_external = true;
        new_f.borrow_mut().called_functions = f.borrow().called_functions.clone();
        new_f.borrow_mut().called_functions.insert(f.clone());
        new_f.borrow_mut().used_crypto_backends = f.borrow().used_crypto_backends.clone();
        new_f.borrow_mut().body = Some(Self::create_external_wrapper_body(
            f.clone(),
            &circuit,
            original_params.clone(),
            requires_proof.clone(),
        ));

        // Add out and proof parameter to external wrapper
        let storage_loc = if new_f.borrow().is_function() {
            "calldata"
        } else {
            "memory"
        };
        new_f.borrow_mut().add_param(
            RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into(),
            IdentifierExprUnion::Identifier(RcCell::new(Identifier::Identifier(
                IdentifierBase::new(CFG.lock().unwrap().zk_out_name()),
            ))),
            Some(storage_loc.to_string()),
        );

        if requires_proof {
            new_f.borrow_mut().add_param(
                AnnotatedTypeName::proof_type().into(),
                IdentifierExprUnion::Identifier(RcCell::new(Identifier::Identifier(
                    IdentifierBase::new(CFG.lock().unwrap().proof_param_name()),
                ))),
                Some(storage_loc.to_string()),
            );
        }
        self.circuits.borrow_mut().insert(new_f.clone(), circuit);
        (new_f, f.clone())
    }
    // """
    // Return Block with external wrapper function body.

    // :param int_fct: corresponding internal function
    // :param ext_circuit: [SIDE EFFECT] circuit helper of the external wrapper function
    // :param original_params: list of transformed function parameters without additional parameters added due to transformation
    // :return: body with wrapper code
    // """
    pub fn create_external_wrapper_body(
        int_fct: RcCell<ConstructorOrFunctionDefinition>,
        ext_circuit: &RcCell<CircuitHelper>,
        original_params: Vec<RcCell<Parameter>>,
        requires_proof: bool,
    ) -> RcCell<Block> {
        //println!("==================={}=={}==========", file!(), line!());
        let priv_args: Vec<_> = original_params
            .iter()
            .filter(|p| {
                p.borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_cipher()
            })
            .cloned()
            .collect();
        let args_backends: Vec<_> = priv_args
            .iter()
            .map(|p| {
                p.borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_array_ref()
                    .unwrap()
                    .try_as_cipher_text_ref()
                    .unwrap()
                    .crypto_params
                    .clone()
            })
            .collect();
        let mut stmts = vec![];

        for crypto_params in &args_backends {
            assert!(int_fct
                .borrow()
                .used_crypto_backends
                .as_ref()
                .unwrap()
                .contains(&crypto_params));
            // If there are any private arguments with homomorphism "hom", we need the public key for that crypto backend
            ext_circuit.borrow_mut()._require_public_key_for_label_at(
                None,
                &RcCell::new(Expression::me_expr(None)).into(),
                &crypto_params,
            );
        }
        let all_crypto_params = CFG.lock().unwrap().user_config.all_crypto_params();
        for crypto_params in all_crypto_params {
            if crypto_params.is_symmetric_cipher()
                && (ext_circuit.borrow().requested_global_keys().contains(&(
                    Some(RcCell::new(MeExpr::new()).into()),
                    crypto_params.clone(),
                )) || args_backends.contains(&crypto_params))
            {
                // Make sure msg.sender"s key pair is available in the circuit
                stmts.extend(ext_circuit.borrow().request_private_key(&crypto_params));
            }
        }

        // Verify that out parameter has correct size
        stmts.push(
            RcCell::new(RequireStatement::new(
                RcCell::new(
                    LocationExpr::IdentifierExpr(IdentifierExpr::new(
                        IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                        None,
                    ))
                    .dot(IdentifierExprUnion::String(String::from("length")))
                    .into_expr()
                    .binop(
                        String::from("=="),
                        NumberLiteralExpr::new(ext_circuit.borrow().out_size_trans(), false)
                            .into_expr(),
                    ),
                )
                .into(),
                None,
            ))
            .into(),
        );

        // IdentifierExpr for array var holding serialized public circuit inputs
        let in_arr_var = IdentifierExpr::new(
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
            None,
        )
        .as_type(&RcCell::new(ArrayBase::new(AnnotatedTypeName::uint_all(), None)).into());

        // Request static public keys
        let mut offset = 0;
        let mut key_req_stmts = vec![];
        let mut me_key_idx = BTreeMap::new();
        if !ext_circuit.borrow().requested_global_keys().is_empty() {
            // Ensure that me public key is stored starting at in[0]
            let keys = ext_circuit.borrow().requested_global_keys();

            let mut tmp_keys = BTreeMap::new();
            for crypto_params in int_fct.borrow().used_crypto_backends.clone().unwrap() {
                let tmp_key_var =
                    IdentifierBase::new(format!("_tmp_key_{}", crypto_params.identifier_name()));

                key_req_stmts.push(
                    RcCell::new(tmp_key_var.decl_var(
                        &AnnotatedTypeName::key_type(crypto_params.clone()).into(),
                        None,
                    ))
                    .into(),
                );
                tmp_keys.insert(crypto_params.clone(), tmp_key_var);
            }
            for (key_owner, crypto_params) in keys {
                let tmp_key_var = &tmp_keys[&crypto_params];
                let (_idf, mut assignment) = ext_circuit.borrow_mut().request_public_key(
                    &crypto_params,
                    key_owner.clone(),
                    &CircuitHelper::get_glob_key_name(&key_owner.as_ref().unwrap(), &crypto_params),
                );
                assignment.assignment_statement_base_mut_ref().lhs = Some(
                    RcCell::new(IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(RcCell::new(Identifier::Identifier(
                            tmp_key_var.clone(),
                        ))),
                        None,
                    ))
                    .into(),
                );
                key_req_stmts.push(RcCell::new(assignment).into());

                // Remember me-keys for later use in symmetrically encrypted keys
                if is_instance(&key_owner.unwrap(), ASTType::MeExpr) {
                    //== MeExpr::new()
                    assert!(!me_key_idx.contains_key(&crypto_params));
                    me_key_idx.insert(crypto_params.clone(), offset);
                }

                // Manually add to circuit inputs
                let key_len = crypto_params.key_len();
                key_req_stmts.push(
                    RcCell::new(
                        LocationExpr::IdentifierExpr(
                            in_arr_var
                                .try_as_identifier_expr_ref()
                                .unwrap()
                                .borrow()
                                .slice(offset, key_len, None)
                                .arr
                                .as_ref()
                                .unwrap()
                                .try_as_identifier_expr_ref()
                                .unwrap()
                                .borrow()
                                .clone(),
                        )
                        .assign(
                            RcCell::new(
                                IdentifierExpr::new(
                                    IdentifierExprUnion::Identifier(RcCell::new(
                                        Identifier::Identifier(tmp_key_var.clone()),
                                    )),
                                    None,
                                )
                                .slice(0, key_len, None),
                            )
                            .into(),
                        ),
                    )
                    .into(),
                );
                offset += key_len;
                assert!(offset == ext_circuit.borrow().in_size());
            }
        }

        // Check encrypted parameters
        let mut param_stmts = vec![];
        for p in &original_params {
            // """ * of T_e rule 8 """
            if p.borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_cipher()
            {
                let cipher_payload_len = p
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_array_ref()
                    .unwrap()
                    .try_as_cipher_text_ref()
                    .unwrap()
                    .crypto_params
                    .cipher_payload_len();
                let assign_stmt = LocationExpr::IdentifierExpr(
                    in_arr_var
                        .try_as_identifier_expr_ref()
                        .unwrap()
                        .borrow()
                        .slice(offset, cipher_payload_len, None)
                        .arr
                        .as_ref()
                        .unwrap()
                        .try_as_identifier_expr_ref()
                        .unwrap()
                        .borrow()
                        .clone(),
                )
                .assign(
                    RcCell::new(
                        IdentifierExpr::new(
                            IdentifierExprUnion::Identifier(p.borrow().idf().clone().unwrap()),
                            None,
                        )
                        .slice(0, cipher_payload_len, None),
                    )
                    .into(),
                );
                let assign_stmt = RcCell::new(assign_stmt).into();
                ext_circuit
                    .borrow_mut()
                    .ensure_parameter_encryption(&assign_stmt, p);

                // Manually add to circuit inputs
                param_stmts.push(assign_stmt);
                offset += cipher_payload_len;
            }
        }

        // Populate sender field of parameters encrypted with a symmetric cipher
        let mut copy_stmts = vec![];
        for p in &original_params {
            if p.borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_cipher()
            {
                let c = p
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .clone();
                assert!(is_instance(c.as_ref().unwrap(), ASTType::CipherText));
                if c.as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_array_ref()
                    .unwrap()
                    .try_as_cipher_text_ref()
                    .unwrap()
                    .crypto_params
                    .is_symmetric_cipher()
                {
                    let sender_key = LocationExpr::IdentifierExpr(
                        in_arr_var
                            .try_as_identifier_expr_ref()
                            .unwrap()
                            .borrow()
                            .clone(),
                    )
                    .index(ExprUnion::I32(
                        me_key_idx[&c
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .try_as_array_ref()
                            .unwrap()
                            .try_as_cipher_text_ref()
                            .unwrap()
                            .crypto_params],
                    ));
                    let idf = IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(p.borrow().idf().clone().unwrap()),
                        None,
                    )
                    .as_type(&p.borrow().annotated_type().clone().unwrap().into());
                    let cipher_payload_len = CFG
                        .lock()
                        .unwrap()
                        .user_config
                        .get_crypto_params(
                            &p.borrow()
                                .annotated_type()
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .homomorphism,
                        )
                        .cipher_payload_len();
                    let lit = ArrayLiteralExpr::ArrayLiteralExpr(ArrayLiteralExprBase::new(
                        (0..cipher_payload_len)
                            .map(|i| {
                                LocationExpr::IdentifierExpr(
                                    idf.try_as_identifier_expr_ref().unwrap().borrow().clone(),
                                )
                                .index(ExprUnion::I32(i))
                            })
                            .chain([sender_key])
                            .collect(),
                    ));
                    copy_stmts.push(
                        RcCell::new(VariableDeclarationStatement::new(
                            RcCell::new(VariableDeclaration::new(
                                vec![],
                                p.borrow().annotated_type().clone(),
                                p.borrow().idf().clone(),
                                None,
                            )),
                            Some(RcCell::new(lit).into()),
                        ))
                        .into(),
                    );
                }
            }
        }
        if !copy_stmts.is_empty() {
            param_stmts.extend([
                RcCell::new(CommentBase::new(String::new())).into(),
                RcCell::new(CommentBase::new(String::from(
                    "Copy from calldata to memory and set sender field",
                )))
                .into(),
            ]);
            param_stmts.extend(copy_stmts);
        }

        // Declare in array
        let new_in_array_expr = NewExpr::new(
            AnnotatedTypeName::new(
                Some(RcCell::new(TypeName::dyn_uint_array())),
                None,
                zkay_ast::homomorphism::Homomorphism::non_homomorphic(),
            ),
            vec![RcCell::new(NumberLiteralExpr::new(
                ext_circuit.borrow().in_size_trans(),
                false,
            ))
            .into()],
        );
        let in_var_decl = in_arr_var
            .ast_base_ref()
            .unwrap()
            .borrow()
            .idf
            .as_ref()
            .unwrap()
            .borrow()
            .identifier_base_ref()
            .decl_var(
                &RcCell::new(TypeName::dyn_uint_array()).into(),
                Some(RcCell::new(new_in_array_expr).into()),
            );
        stmts.push(RcCell::new(in_var_decl).into());
        stmts.push(RcCell::new(CommentBase::new(String::new())).into());
        stmts.extend(CommentBase::comment_wrap_block(
            String::from("Request static public keys"),
            key_req_stmts,
        ));
        stmts.extend(CommentBase::comment_wrap_block(
            String::from("Backup private arguments for verification"),
            param_stmts,
        ));

        // Call internal function
        let mut args: Vec<_> = original_params
            .iter()
            .map(|param| {
                RcCell::new(IdentifierExpr::new(
                    IdentifierExprUnion::Identifier(param.borrow().idf().clone().unwrap()),
                    None,
                ))
                .into()
            })
            .collect();
        let mut idf = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(int_fct.borrow().idf().clone().unwrap()),
            None,
        );
        idf.ast_base_ref().borrow_mut().target =
            Some(ASTFlatten::from(int_fct.clone()).downgrade());
        let mut internal_call =
            FunctionCallExprBase::new(RcCell::new(idf).into(), args.clone(), None, None);
        internal_call.sec_start_offset = Some(ext_circuit.borrow().priv_in_size());
        let mut internal_call = RcCell::new(FunctionCallExpr::FunctionCallExpr(internal_call));
        if int_fct.borrow().requires_verification {
            ext_circuit
                .borrow_mut()
                .call_function(&internal_call.clone().into());
            args.append(&mut vec![
                in_arr_var.clone(),
                RcCell::new(NumberLiteralExpr::new(
                    ext_circuit.borrow().in_size(),
                    false,
                ))
                .into(),
                RcCell::new(IdentifierExpr::new(
                    IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                    None,
                ))
                .into(),
                RcCell::new(NumberLiteralExpr::new(
                    ext_circuit.borrow().out_size(),
                    false,
                ))
                .into(),
            ]);
        }

        let in_call: ASTFlatten = if !int_fct.borrow().return_parameters.is_empty() {
            stmts.extend(CommentBase::comment_list(
                String::from("Declare return variables"),
                int_fct
                    .borrow()
                    .return_var_decls
                    .iter()
                    .map(|vd| {
                        RcCell::new(VariableDeclarationStatement::new(
                            deep_copy(&vd.clone().into(), false, false)
                                .unwrap()
                                .try_as_variable_declaration()
                                .unwrap(),
                            None,
                        ))
                        .into()
                    })
                    .collect(),
            ));
            RcCell::new(
                TupleExpr::new(
                    int_fct
                        .borrow()
                        .return_var_decls
                        .iter()
                        .map(|vd| {
                            RcCell::new(IdentifierExpr::new(
                                IdentifierExprUnion::Identifier(vd.borrow().idf().clone().unwrap()),
                                None,
                            ))
                            .into()
                        })
                        .collect(),
                )
                .assign(internal_call.into()),
            )
            .into()
        } else {
            RcCell::new(ExpressionStatement::new(internal_call.into())).into()
        };
        stmts.push(RcCell::new(CommentBase::new(String::from("Call internal function"))).into());
        stmts.push(in_call.into());
        stmts.push(RcCell::new(CommentBase::new(String::new())).into());

        // Call verifier
        let disable_verification = CFG.lock().unwrap().user_config.disable_verification();
        if requires_proof && !disable_verification {
            let verifier = IdentifierExpr::new(
                IdentifierExprUnion::String(
                    CFG.lock().unwrap().get_contract_var_name(
                        ext_circuit
                            .borrow()
                            .verifier_contract_type
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .code(),
                    ),
                ),
                None,
            );
            let proof_param_name =
                IdentifierExprUnion::String(CFG.lock().unwrap().proof_param_name());
            let zk_in_name = IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name());
            let zk_out_name = IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name());
            let verifier_args: Vec<_> = [proof_param_name, zk_in_name, zk_out_name]
                .into_iter()
                .map(|name| IdentifierExpr::new(name, None))
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect();
            let verify = ExpressionStatement::new(
                RcCell::new(LocationExpr::IdentifierExpr(verifier).call(
                    IdentifierExprUnion::String(CFG.lock().unwrap().verification_function_name()),
                    verifier_args,
                ))
                .into(),
            );
            stmts.push(
                RcCell::new(StatementListBase::new(
                    vec![
                        RcCell::new(CommentBase::new(String::from(
                            "Verify zk proof of execution",
                        )))
                        .into(),
                        RcCell::new(verify).into(),
                    ],
                    true,
                ))
                .into(),
            );
        }

        // Add return statement at the end if necessary
        if !int_fct.borrow().return_parameters.is_empty() {
            stmts.push(
                RcCell::new(ReturnStatement::new(Some(
                    RcCell::new(TupleExpr::new(
                        int_fct
                            .borrow()
                            .return_var_decls
                            .iter()
                            .map(|vd| {
                                RcCell::new(IdentifierExpr::new(
                                    IdentifierExprUnion::Identifier(
                                        vd.borrow().idf().clone().unwrap(),
                                    ),
                                    None,
                                ))
                            })
                            .map(Into::<ASTFlatten>::into)
                            .collect(),
                    ))
                    .into(),
                )))
                .into(),
            );
        }

        RcCell::new(Block::new(stmts, false))
    }
}
