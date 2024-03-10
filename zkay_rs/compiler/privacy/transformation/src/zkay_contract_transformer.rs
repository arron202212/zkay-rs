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
use std::collections::BTreeMap;
use zkay_ast::analysis::used_homomorphisms::UsedHomomorphismsVisitor;
use zkay_ast::ast::{
    is_instance, ASTType, AnnotatedTypeName, Array, ArrayBase, ArrayLiteralExpr,
    ArrayLiteralExprBase, AssignmentStatement, AssignmentStatementBase,
    AssignmentStatementBaseMutRef, BlankLine, Block, CipherText, Comment, CommentBase,
    ConstructorOrFunctionDefinition, ContractDefinition, ContractTypeName, ExprUnion, Expression,
    ExpressionStatement, FunctionCallExpr, FunctionCallExprBase, HybridArgumentIdf, Identifier,
    IdentifierBase, IdentifierBaseProperty, IdentifierBaseRef, IdentifierDeclaration,
    IdentifierExpr, IdentifierExprUnion, IndexExpr, IntoAST, IntoExpression, IntoStatement,
    LocationExpr, MeExpr, NamespaceDefinition, NewExpr, NumberLiteralExpr, Parameter,
    PrimitiveCastExpr, RequireStatement, ReturnStatement, SourceUnit, StateVariableDeclaration,
    Statement, StatementList, StatementListBase, StructDefinition, StructTypeName, TupleExpr,
    TypeName, UserDefinedTypeName, VariableDeclaration, VariableDeclarationStatement, AST,
};
use zkay_ast::pointers::parent_setter::set_parents;
use zkay_ast::pointers::symbol_table::link_identifiers;
use zkay_ast::visitor::deep_copy::deep_copy;
use zkay_ast::visitor::transformer_visitor::{AstTransformerVisitor, TransformerVisitorEx};
use zkay_config::config::CFG;
use zkay_crypto::params::CryptoParams;
pub fn transform_ast(
    ast: Option<AST>,
) -> (
    AST,
    BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
)
// """
    // Convert zkay to solidity AST + proof circuits

    // :param ast: zkay AST
    // :return: solidity AST and dictionary which maps all function definitions which require verification
    //          to the corresponding circuit helper instance.
    // """
{
    let zt = ZkayTransformer::new();
    let new_ast = zt.visit(Some(ast.unwrap().to_ast()));

    // restore all parent pointers and identifier targets
    set_parents(new_ast.clone().unwrap());
    link_identifiers(&new_ast.as_ref().unwrap());
    (new_ast.unwrap(), zt.circuits)
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
#[derive(Clone)]
pub struct ZkayTransformer {
    circuits: BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
    var_decl_trafo: ZkayVarDeclTransformer,
}
impl AstTransformerVisitor for ZkayTransformer {
    fn default() -> Self {
        Self::new()
    }

    fn visit(&self, _ast: Option<AST>) -> Option<AST> {
        // self._visit_internal(ast)
        None
    }
    fn visitBlock(
        &self,
        _ast: Option<AST>,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> Option<AST> {
        // self.visit_children(ast)
        None
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
            circuits: BTreeMap::new(),
            var_decl_trafo: ZkayVarDeclTransformer::new(),
        }
    }

    pub fn import_contract(
        cname: &str,
        su: &mut SourceUnit,
        corresponding_circuit: Option<&mut CircuitHelper>,
    )
    // """
    // Import contract "vname" into the given source unit.

    // :param cname: contract name (.sol filename stem must match contract type name)
    // :param su: [SIDE EFFECT] source unit where contract should be imported
    // :param corresponding_circuit: [SIDE EFFECT] if contract is a verification contract, this should be the corresponding circuit helper
    // """
    {
        let import_filename = format!("./{cname}.sol");
        su.used_contracts.push(import_filename.clone());

        if let Some(corresponding_circuit) = corresponding_circuit {
            let c_type = ContractTypeName::new(
                vec![Identifier::Identifier(IdentifierBase::new(
                    cname.to_string(),
                ))],
                None,
            );
            corresponding_circuit.register_verification_contract_metadata(
                TypeName::UserDefinedTypeName(UserDefinedTypeName::ContractTypeName(
                    c_type.clone(),
                )),
                &import_filename,
            );
        }
    }

    pub fn create_contract_variable(cname: &str) -> StateVariableDeclaration
// """Create a public constant state variable with which contract with name "cname" can be accessed"""
    {
        let inst_idf = Identifier::Identifier(IdentifierBase::new(
            CFG.lock().unwrap().get_contract_var_name(cname.to_string()),
        ));
        let c_type = ContractTypeName::new(
            vec![Identifier::Identifier(IdentifierBase::new(
                cname.to_string(),
            ))],
            None,
        );

        let cast_0_to_c = PrimitiveCastExpr::new(
            TypeName::UserDefinedTypeName(UserDefinedTypeName::ContractTypeName(c_type.clone())),
            NumberLiteralExpr::new(0, false).to_expr(),
            false,
        );
        StateVariableDeclaration::new(
            AnnotatedTypeName::new(
                TypeName::UserDefinedTypeName(UserDefinedTypeName::ContractTypeName(
                    c_type.clone(),
                )),
                None,
                String::from("NON_HOMOMORPHIC"),
            ),
            zkay_config::lc_vec_s!["public", "constant"],
            inst_idf.clone(),
            Some(cast_0_to_c.to_expr()),
        )
    }

    pub fn include_verification_contracts(
        &mut self,
        su: &mut SourceUnit,
        c: &ContractDefinition,
    ) -> Vec<StateVariableDeclaration>
// """
        // Import all verification contracts for "c" into "su" and create state variable declarations for all of them + the pki contract.

        // :param su: [SIDE EFFECT] source unit into which contracts should be imported
        // :param c: contract for which verification contracts should be imported
        // :return: list of all constant state variable declarations for the pki contract + all the verification contracts
        // """
    {
        let mut contract_var_decls = vec![];
        for crypto_params in c.used_crypto_backends.clone().unwrap() {
            let contract_name = CFG
                .lock()
                .unwrap()
                .get_pki_contract_name(&crypto_params.identifier_name());
            contract_var_decls.push(Self::create_contract_variable(&contract_name));
        }

        for f in c
            .constructor_definitions
            .iter()
            .chain(&c.function_definitions)
        {
            if f.requires_verification_when_external && f.has_side_effects() {
                let name = CFG.lock().unwrap().get_verification_contract_name(
                    c.namespace_definition_base.idf.name().clone(),
                    f.name(),
                );
                Self::import_contract(&name, su, self.circuits.get_mut(&f));
                contract_var_decls.push(Self::create_contract_variable(&name));
            }
        }

        contract_var_decls
    }

    pub fn create_circuit_helper(
        fct: &ConstructorOrFunctionDefinition,
        global_owners: Vec<AST>,
        internal_circ: Option<&mut CircuitHelper>,
    ) -> CircuitHelper
// """
    // Create circuit helper for the given function.

    // :param fct: function for which to create a circuit
    // :param global_owners: list of all statically known privacy labels (me + final address state variables)
    // :param internal_circ: the circuit of the internal function on which to base this circuit
    //                       (only used when creating the circuit of the external wrapper function)
    // :return: new circuit helper
    // """
    {
        CircuitHelper::new(
            fct.clone(),
            global_owners,
            |ch: &CircuitHelper| {
                Some(Box::new(ZkayExpressionTransformer::new(Some(Box::new(
                    ch.clone(),
                )))))
            },
            |ch: &CircuitHelper| {
                Some(Box::new(ZkayCircuitTransformer::new(Some(Box::new(
                    ch.clone(),
                )))))
            },
            internal_circ,
        )
    }

    pub fn visitSourceUnit(&mut self, ast: &mut SourceUnit) -> SourceUnit
// Figure out which crypto backends were used
    {
        UsedHomomorphismsVisitor::new().visit(&mut ast.to_ast());

        for crypto_params in ast.clone().used_crypto_backends.unwrap() {
            Self::import_contract(
                &CFG.lock()
                    .unwrap()
                    .get_pki_contract_name(&crypto_params.identifier_name()),
                ast,
                None,
            );
        }
        let mut contracts = ast.contracts.clone();
        for c in contracts.iter_mut() {
            self.transform_contract(ast, c);
        }
        ast.contracts = contracts;
        ast.clone()
    }

    pub fn transform_contract(
        &mut self,
        su: &mut SourceUnit,
        c: &mut ContractDefinition,
    ) -> ContractDefinition
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
    {
        let mut all_fcts: Vec<_> = c
            .constructor_definitions
            .iter()
            .chain(&c.function_definitions)
            .cloned()
            .collect();

        // Get list of static owner labels for this contract
        let mut global_owners = vec![Expression::me_expr(None)];
        for var in &c.state_variable_declarations {
            if let AST::IdentifierDeclaration(IdentifierDeclaration::StateVariableDeclaration(
                var,
            )) = var
            {
                if var.identifier_declaration_base.annotated_type.is_address()
                    && (var.identifier_declaration_base.is_final()
                        || var.identifier_declaration_base.is_constant())
                {
                    global_owners.push((*var.identifier_declaration_base.idf).to_expr());
                }
            }
        }

        // Backup untransformed function bodies
        for fct in all_fcts.iter_mut() {
            fct.original_body = fct.body.clone(); //deep_copy(fct.body, true, true);
        }

        // Transform types of normal state variables
        c.state_variable_declarations = self
            .var_decl_trafo
            .visit_list(c.state_variable_declarations.clone())
            .into_iter()
            .filter_map(|a| a)
            .collect();

        // Split into functions which require verification and those which don"t need a circuit helper
        let mut req_ext_fcts = BTreeMap::<ConstructorOrFunctionDefinition, Vec<Parameter>>::new();
        let (mut new_fcts, mut new_constr) = (vec![], vec![]);
        for fct in &all_fcts {
            assert!(is_instance(fct, ASTType::ConstructorOrFunctionDefinition));
            if fct.requires_verification || fct.requires_verification_when_external {
                self.circuits.insert(
                    fct.clone(),
                    Self::create_circuit_helper(
                        fct,
                        global_owners
                            .clone()
                            .into_iter()
                            .map(|e| e.to_ast())
                            .collect(),
                        None,
                    ),
                );
            }

            if fct.requires_verification_when_external {
                req_ext_fcts.insert(fct.clone(), fct.parameters.clone());
            } else if fct.is_constructor() {
                new_constr.push(fct.clone());
            } else {
                new_fcts.push(fct.clone());
            }
        }

        // Add constant state variables for external contracts and field prime
        let field_prime_decl = StateVariableDeclaration::new(
            AnnotatedTypeName::uint_all(),
            zkay_config::lc_vec_s!["public", "constant"],
            Identifier::Identifier(IdentifierBase::new(
                CFG.lock().unwrap().field_prime_var_name(),
            )),
            Some(NumberLiteralExpr::new_string(BN128_SCALAR_FIELD.to_hex_string()).to_expr()),
        );
        let contract_var_decls: Vec<_> = self
            .include_verification_contracts(su, &*c)
            .into_iter()
            .map(|v| v.to_ast())
            .collect();
        let mut svd = vec![
            field_prime_decl.to_ast(),
            CommentBase::new(String::new()).to_ast(),
        ];
        svd.extend(CommentBase::comment_list(
            String::from("Helper Contracts"),
            contract_var_decls,
        ));
        svd.extend(vec![
            CommentBase::new(String::from("User state variables")).to_ast()
        ]);
        svd.extend(c.state_variable_declarations.clone());
        c.state_variable_declarations = svd;

        // Transform signatures
        for f in all_fcts.iter_mut() {
            f.parameters = self
                .var_decl_trafo
                .visit_list(f.parameters.iter().map(|p| p.to_ast()).collect())
                .into_iter()
                .filter_map(|p| {
                    if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(p))) = p
                    {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect();
        }
        for f in c.function_definitions.iter_mut() {
            f.return_parameters = self
                .var_decl_trafo
                .visit_list(f.return_parameters.iter().map(|p| p.to_ast()).collect())
                .into_iter()
                .filter_map(|p| {
                    if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(p))) = p
                    {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect();
            f.return_var_decls = self
                .var_decl_trafo
                .visit_list(f.return_var_decls.iter().map(|p| p.to_ast()).collect())
                .into_iter()
                .filter_map(|p| {
                    p.map(|v| {
                        v.try_as_identifier_declaration_ref()
                            .unwrap()
                            .try_as_variable_declaration_ref()
                            .unwrap()
                            .clone()
                    })
                })
                .collect();
        }

        // Transform bodies
        for fct in all_fcts.iter_mut() {
            let gen = self.circuits.get(fct);
            fct.body =
                if let Some(AST::Statement(Statement::StatementList(StatementList::Block(b)))) =
                    ZkayStatementTransformer::new(Some(Box::new(gen.unwrap().clone())))
                        .visit(Some(fct.body.as_ref().unwrap().to_ast()))
                {
                    Some(b)
                } else {
                    None
                };
        }

        // Transform (internal) functions which require verification (add the necessary additional parameters and boilerplate code)
        let mut fcts_with_verification: Vec<_> = all_fcts
            .iter()
            .filter_map(|fct| {
                if fct.requires_verification {
                    Some(fct.clone())
                } else {
                    None
                }
            })
            .collect();
        compute_transitive_circuit_io_sizes(&mut fcts_with_verification, &mut self.circuits);
        transform_internal_calls(&mut fcts_with_verification, &mut self.circuits);
        for f in fcts_with_verification.iter_mut() {
            let circuit = self.circuits[&*f].clone();
            assert!(circuit.requires_verification());
            if circuit.requires_zk_data_struct()
            // Add zk data struct for f to contract
            {
                let zk_data_struct = StructDefinition::new(
                    Identifier::Identifier(IdentifierBase::new(circuit.zk_data_struct_name())),
                    circuit
                        .output_idfs()
                        .iter()
                        .chain(&circuit.input_idfs())
                        .map(|idf| {
                            VariableDeclaration::new(
                                vec![],
                                AnnotatedTypeName::new(
                                    *idf.t.clone(),
                                    None,
                                    String::from("NON_HOMOMORPHIC"),
                                ),
                                Identifier::HybridArgumentIdf(idf.clone()),
                                None,
                            )
                            .to_ast()
                        })
                        .collect(),
                );
                c.struct_definitions.push(zk_data_struct);
            }
            self.create_internal_verification_wrapper(f);
        }

        // Create external wrapper functions where necessary
        for (f, params) in req_ext_fcts.iter_mut() {
            let mut f = f.clone();
            let (ext_f, int_f) = self.split_into_external_and_internal_fct(
                &mut f,
                params,
                global_owners
                    .clone()
                    .into_iter()
                    .map(|e| e.to_ast())
                    .collect(),
            );
            if ext_f.is_function() {
                new_fcts.push(ext_f);
            } else {
                new_constr.push(ext_f);
            }
            new_fcts.push(int_f);
        }

        c.constructor_definitions = new_constr;
        c.function_definitions = new_fcts;
        c.clone()
    }

    pub fn create_internal_verification_wrapper(&self, ast: &mut ConstructorOrFunctionDefinition)
    // """
    // Add the necessary additional parameters and boiler plate code for verification support to the given function.
    // :param ast: [SIDE EFFECT] Internal function which requires verification
    // """
    {
        let circuit = self.circuits[&ast].clone();
        let mut stmts = vec![];

        let symmetric_cipher_used = ast
            .used_crypto_backends
            .as_ref()
            .unwrap()
            .iter()
            .any(|backend| backend.is_symmetric_cipher());
        if symmetric_cipher_used && ast.modifiers.contains(&String::from("pure"))
        // Symmetric trafo requires msg.sender access -> change from pure to view
        {
            ast.modifiers = ast
                .modifiers
                .iter()
                .map(|modi| (if modi == "pure" { "view" } else { modi }).to_string())
                .collect();
        }

        // Add additional params
        ast.add_param(
            AST::TypeName(TypeName::Array(Array::Array(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                None,
            )))),
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
            None,
        );
        ast.add_param(
            AST::AnnotatedTypeName(AnnotatedTypeName::uint_all()),
            IdentifierExprUnion::String(format!("{}_start_idx", CFG.lock().unwrap().zk_in_name())),
            None,
        );
        ast.add_param(
            AST::TypeName(TypeName::Array(Array::Array(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                None,
            )))),
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
            None,
        );
        ast.add_param(
            AST::AnnotatedTypeName(AnnotatedTypeName::uint_all()),
            IdentifierExprUnion::String(format!("{}_start_idx", CFG.lock().unwrap().zk_out_name())),
            None,
        );

        // Verify that in/out parameters have correct size
        let (out_start_idx, in_start_idx) = (
            IdentifierExpr::new(
                IdentifierExprUnion::String(format!(
                    "{}_start_idx",
                    CFG.lock().unwrap().zk_out_name()
                )),
                None,
            ),
            IdentifierExpr::new(
                IdentifierExprUnion::String(format!(
                    "{}_start_idx",
                    CFG.lock().unwrap().zk_in_name()
                )),
                None,
            ),
        );
        let (out_var, in_var) = (
            IdentifierExpr::new(
                IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                None,
            ),
            IdentifierExpr::new(
                IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
                None,
            )
            .as_type(AST::TypeName(TypeName::Array(Array::Array(
                ArrayBase::new(AnnotatedTypeName::uint_all(), None),
            )))),
        );
        stmts.push(
            RequireStatement::new(
                out_start_idx
                    .to_expr()
                    .binop(
                        String::from("+"),
                        NumberLiteralExpr::new(circuit.out_size_trans(), false).to_expr(),
                    )
                    .to_expr()
                    .binop(
                        String::from("<="),
                        LocationExpr::IdentifierExpr(out_var)
                            .dot(IdentifierExprUnion::String(String::from("length")))
                            .to_expr(),
                    )
                    .to_expr(),
                None,
            )
            .to_ast(),
        );
        stmts.push(
            RequireStatement::new(
                in_start_idx
                    .to_expr()
                    .binop(
                        String::from("+"),
                        NumberLiteralExpr::new(circuit.in_size_trans(), false).to_expr(),
                    )
                    .to_expr()
                    .binop(
                        String::from("<="),
                        LocationExpr::IdentifierExpr(in_var.clone())
                            .dot(IdentifierExprUnion::String(String::from("length")).into())
                            .to_expr(),
                    )
                    .to_expr(),
                None,
            )
            .to_ast(),
        );

        // Declare zk_data struct var (if needed)
        if circuit.requires_zk_data_struct() {
            let zk_struct_type = StructTypeName::new(
                vec![Identifier::Identifier(IdentifierBase::new(
                    circuit.zk_data_struct_name(),
                ))],
                None,
            );
            let mut idf = IdentifierBase::new(CFG.lock().unwrap().zk_data_var_name());
            idf.decl_var(
                AST::TypeName(TypeName::UserDefinedTypeName(
                    UserDefinedTypeName::StructTypeName(zk_struct_type),
                )),
                None,
            );
            stmts.extend(vec![
                Identifier::Identifier(idf).to_ast(),
                BlankLine::new().to_ast(),
            ]);
        }

        // Declare return variable if necessary
        if !ast.return_parameters.is_empty() {
            stmts.extend(CommentBase::comment_list(
                String::from("Declare return variables"),
                ast.return_var_decls
                    .iter()
                    .map(|vd| VariableDeclarationStatement::new(vd.clone(), None).to_ast())
                    .collect(),
            ));
        }

        // Find all me-keys in the in array
        let mut me_key_idx = BTreeMap::new();
        let mut offset = 0;
        for (key_owner, crypto_params) in circuit.requested_global_keys() {
            if is_instance(&key_owner.unwrap(), ASTType::MeExpr) {
                //== MeExpr::new()
                assert!(!me_key_idx.contains_key(&crypto_params));
                me_key_idx.insert(crypto_params.clone(), offset);
            }
            offset += crypto_params.key_len();
        }

        // Deserialize out array (if any)
        let mut deserialize_stmts = vec![];
        let mut offset = 0;
        for s in circuit.output_idfs().iter_mut() {
            deserialize_stmts.push(s.deserialize(
                CFG.lock().unwrap().zk_out_name(),
                Some(out_start_idx.to_expr()),
                offset,
            ));
            if is_instance(&*s.t, ASTType::CipherText)
                && s.t.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap().crypto_params.is_symmetric_cipher()
            // Assign sender field to user-encrypted values if necessary
            // Assumption: s.t.crypto_params.key_len == 1 for all symmetric ciphers
            {
                assert!(
                    me_key_idx.contains_key(&s.t.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap().crypto_params),
                    "Symmetric cipher but did not request me key"
                );
                let key_idx = me_key_idx[&s.t.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap().crypto_params];
                let sender_key =
                    LocationExpr::IdentifierExpr(in_var.clone()).index(ExprUnion::I32(key_idx));
                let cipher_payload_len = s.t.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap().crypto_params.cipher_payload_len();
                deserialize_stmts.push(
                    LocationExpr::IndexExpr(
                        s.get_loc_expr(None)
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_tuple_or_location_expr_ref()
                            .unwrap()
                            .try_as_location_expr_ref()
                            .unwrap()
                            .index(ExprUnion::I32(cipher_payload_len)),
                    )
                    .assign(sender_key.to_expr()),
                );
            }
            offset += s.t.size_in_uints();
        }
        if !deserialize_stmts.is_empty() {
            stmts.push(
                Statement::StatementList(StatementList::StatementList(StatementListBase::new(
                    CommentBase::comment_wrap_block(
                        String::from("Deserialize output values"),
                        deserialize_stmts.into_iter().map(|v| v.to_ast()).collect(),
                    ),
                    true,
                )))
                .to_ast(),
            );
        }

        // Include original transformed function body
        stmts.extend(
            ast.body
                .as_ref()
                .unwrap()
                .statement_list_base
                .statements
                .clone(),
        );

        // Serialize in parameters to in array (if any)
        let mut serialize_stmts = vec![];
        let mut offset = 0;
        for s in circuit.input_idfs().iter_mut() {
            serialize_stmts.extend(vec![s.serialize(
                CFG.lock().unwrap().zk_in_name(),
                Some(in_start_idx.to_expr()),
                offset,
            )]);
            offset += s.t.size_in_uints();
        }
        if offset != 0 {
            stmts.push(CommentBase::new(String::new()).to_ast());
            stmts.extend(CommentBase::comment_wrap_block(
                String::from("Serialize input values"),
                serialize_stmts.into_iter().map(|v| v.to_ast()).collect(),
            ));
        }

        // Add return statement at the end if necessary
        // (was previously replaced by assignment to return_var by ZkayStatementTransformer)
        if circuit.has_return_var {
            stmts.push(
                ReturnStatement::new(Some(
                    TupleExpr::new(
                        ast.return_var_decls
                            .iter()
                            .map(|vd| {
                                let mut idf = IdentifierExpr::new(
                                    IdentifierExprUnion::Identifier(
                                        *vd.identifier_declaration_base.idf.clone(),
                                    ),
                                    None,
                                );
                                idf.location_expr_base.target = Some(Box::new(vd.to_ast()));
                                idf.to_expr()
                            })
                            .collect(),
                    )
                    .to_expr(),
                ))
                .to_ast(),
            );
        }

        ast.body.as_mut().unwrap().statement_list_base.statements = stmts;
    }

    pub fn split_into_external_and_internal_fct(
        &mut self,
        f: &mut ConstructorOrFunctionDefinition,
        original_params: &mut Vec<Parameter>,
        global_owners: Vec<AST>,
    ) -> (
        ConstructorOrFunctionDefinition,
        ConstructorOrFunctionDefinition,
    )
// """
        // Take public function f and split it into an internal function and an external wrapper function.

        // :param f: [SIDE EFFECT] function to split (at least requires_verification_if_external)
        // :param original_params: list of transformed function parameters without additional parameters added due to transformation
        // :param global_owners: list of static labels (me + final address state variable identifiers)
        // :return: Tuple of newly created external and internal function definitions
        // """
    {
        assert!(f.requires_verification_when_external);

        // Create new empty function with same parameters as original -> external wrapper
        let mut new_modifiers = if f.is_function() {
            *original_params = original_params
                .iter()
                .map(|p| {
                    let mut pp = deep_copy(Some(p.to_ast()), true, false)
                        .unwrap()
                        .try_as_identifier_declaration()
                        .unwrap()
                        .try_as_parameter()
                        .unwrap()
                        .clone();
                    pp.with_changed_storage(String::from("memory"), String::from("calldata"))
                        .clone()
                })
                .collect();
            zkay_config::lc_vec_s!["external"]
        } else {
            zkay_config::lc_vec_s!["public"]
        };
        if f.is_payable() {
            new_modifiers.push(String::from("payable"));
        }

        let mut requires_proof = true;
        if !f.has_side_effects() {
            requires_proof = false;
            new_modifiers.push(String::from("view"));
        }
        let mut new_f = ConstructorOrFunctionDefinition::new(
            Some(f.namespace_definition_base.idf.clone()),
            Some(original_params.clone()),
            Some(new_modifiers),
            Some(f.return_parameters.clone()),
            Some(Block::new(vec![], false)),
        );

        // Make original function internal
        f.namespace_definition_base.idf = Identifier::Identifier(IdentifierBase::new(
            CFG.lock().unwrap().get_internal_name(f),
        ));
        f.modifiers = f
            .modifiers
            .iter()
            .filter_map(|modi| {
                if modi != "payable" {
                    Some((if modi == "public" { "internal" } else { modi }).to_string())
                } else {
                    None
                }
            })
            .collect();
        f.requires_verification_when_external = false;

        // Create new circuit for external function
        let circuit = Self::create_circuit_helper(&new_f, global_owners, self.circuits.get_mut(f));
        if !f.requires_verification {
            self.circuits.remove(f);
        }

        // Set meta attributes and populate body
        new_f.requires_verification = true;
        new_f.requires_verification_when_external = true;
        new_f.called_functions = f.called_functions.clone();
        new_f.called_functions.insert(f.clone());
        new_f.used_crypto_backends = f.used_crypto_backends.clone();
        new_f.body = Some(Self::create_external_wrapper_body(
            f.clone(),
            circuit.clone(),
            original_params.clone(),
            requires_proof.clone(),
        ));

        // Add out and proof parameter to external wrapper
        let storage_loc = if new_f.is_function() {
            "calldata"
        } else {
            "memory"
        };
        new_f.add_param(
            AST::TypeName(TypeName::Array(Array::Array(ArrayBase::new(
                AnnotatedTypeName::uint_all(),
                None,
            )))),
            IdentifierExprUnion::Identifier(Identifier::Identifier(IdentifierBase::new(
                CFG.lock().unwrap().zk_out_name(),
            ))),
            Some(storage_loc.to_string()),
        );

        if requires_proof {
            new_f.add_param(
                AST::AnnotatedTypeName(AnnotatedTypeName::proof_type()),
                IdentifierExprUnion::Identifier(Identifier::Identifier(IdentifierBase::new(
                    CFG.lock().unwrap().proof_param_name(),
                ))),
                Some(storage_loc.to_string()),
            );
        }
        self.circuits.insert(new_f.clone(), circuit);
        (new_f, f.clone())
    }

    pub fn create_external_wrapper_body(
        int_fct: ConstructorOrFunctionDefinition,
        mut ext_circuit: CircuitHelper,
        original_params: Vec<Parameter>,
        requires_proof: bool,
    ) -> Block
// """
        // Return Block with external wrapper function body.

        // :param int_fct: corresponding internal function
        // :param ext_circuit: [SIDE EFFECT] circuit helper of the external wrapper function
        // :param original_params: list of transformed function parameters without additional parameters added due to transformation
        // :return: body with wrapper code
        // """
    {
        let priv_args: Vec<_> = original_params
            .iter()
            .filter_map(|p| {
                if p.identifier_declaration_base.annotated_type.is_cipher() {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();
        let args_backends: Vec<_> = priv_args
            .iter()
            .map(|p| {
                p.identifier_declaration_base
                    .annotated_type
                    .type_name.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap()
                    .crypto_params.clone()
            })
            .collect();
        let mut stmts = vec![];

        for crypto_params in &args_backends {
            assert!(int_fct
                .used_crypto_backends
                .as_ref()
                .unwrap()
                .contains(&crypto_params));
            // If there are any private arguments with homomorphism "hom", we need the public key for that crypto backend
            ext_circuit._require_public_key_for_label_at(
                None,
                &Expression::me_expr(None).to_ast(),
                crypto_params,
            );
        }
        for crypto_params in CFG.lock().unwrap().user_config.all_crypto_params() {
            if crypto_params.is_symmetric_cipher() {
                if ext_circuit
                    .requested_global_keys()
                    .contains(&(Some(MeExpr::new().to_ast()), crypto_params.clone()))
                    || args_backends.contains(&crypto_params)
                // Make sure msg.sender"s key pair is available in the circuit
                {
                    stmts.extend(ext_circuit.request_private_key(&crypto_params));
                }
            }
        }

        // Verify that out parameter has correct size
        stmts.push(
            RequireStatement::new(
                LocationExpr::IdentifierExpr(IdentifierExpr::new(
                    IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                    None,
                ))
                .dot(IdentifierExprUnion::String(String::from("length")))
                .to_expr()
                .binop(
                    String::from("=="),
                    NumberLiteralExpr::new(ext_circuit.out_size_trans(), false).to_expr(),
                )
                .to_expr(),
                None,
            )
            .to_ast(),
        );

        // IdentifierExpr for array var holding serialized public circuit inputs
        let in_arr_var = IdentifierExpr::new(
            IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
            None,
        )
        .as_type(AST::TypeName(TypeName::Array(Array::Array(
            ArrayBase::new(AnnotatedTypeName::uint_all(), None),
        ))));

        // Request static public keys
        let mut offset = 0;
        let mut key_req_stmts = vec![];
        let mut me_key_idx = BTreeMap::new();
        if !ext_circuit.clone().requested_global_keys().is_empty() {
            // Ensure that me public key is stored starting at in[0]
            let keys = ext_circuit.requested_global_keys();

            let mut tmp_keys = BTreeMap::new();
            for crypto_params in int_fct.used_crypto_backends.clone().unwrap() {
                let tmp_key_var =
                    IdentifierBase::new(format!("_tmp_key_{}", crypto_params.identifier_name()));

                key_req_stmts.push(
                    tmp_key_var
                        .decl_var(
                            AST::AnnotatedTypeName(AnnotatedTypeName::key_type(
                                crypto_params.clone(),
                            )),
                            None,
                        )
                        .to_ast(),
                );
                tmp_keys.insert(crypto_params.clone(), tmp_key_var);
            }
            for (key_owner, crypto_params) in keys {
                let tmp_key_var = &tmp_keys[&crypto_params];
                let (_idf, mut assignment) = ext_circuit.clone().request_public_key(
                    &crypto_params,
                    key_owner.clone(),
                    &CircuitHelper::get_glob_key_name(&key_owner.as_ref().unwrap(), &crypto_params),
                );
                assignment.assignment_statement_base_mut_ref().lhs = Some(Box::new(
                    IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(Identifier::Identifier(
                            tmp_key_var.clone(),
                        )),
                        None,
                    )
                    .to_ast(),
                ));
                key_req_stmts.push(assignment.to_ast());

                // Remember me-keys for later use in symmetrically encrypted keys
                if is_instance(&key_owner.unwrap(), ASTType::MeExpr) {
                    //== MeExpr::new()
                    assert!(!me_key_idx.contains_key(&crypto_params));
                    me_key_idx.insert(crypto_params.clone(), offset);
                }

                // Manually add to circuit inputs
                let key_len = crypto_params.key_len();
                key_req_stmts.push(
                    in_arr_var
                        .slice(offset, key_len, None)
                        .arr
                        .unwrap()
                        .assign(
                            IdentifierExpr::new(
                                IdentifierExprUnion::Identifier(Identifier::Identifier(
                                    tmp_key_var.clone(),
                                )),
                                None,
                            )
                            .slice(0, key_len, None)
                            .to_expr(),
                        )
                        .to_ast(),
                );
                offset += key_len;
                assert!(offset == ext_circuit.in_size());
            }
        }

        // Check encrypted parameters
        let mut param_stmts = vec![];
        for p in &original_params {
            // """ * of T_e rule 8 """
            if p.identifier_declaration_base.annotated_type.is_cipher() {
                let cipher_payload_len = p
                    .identifier_declaration_base
                    .annotated_type
                    .type_name.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap()
                    .crypto_params
                    .cipher_payload_len();
                let assign_stmt = in_arr_var
                    .slice(offset, cipher_payload_len, None)
                    .arr
                    .unwrap()
                    .assign(
                        IdentifierExpr::new(
                            IdentifierExprUnion::Identifier(
                                *p.identifier_declaration_base.idf.clone(),
                            ),
                            None,
                        )
                        .slice(0, cipher_payload_len, None)
                        .to_expr(),
                    );
                ext_circuit.ensure_parameter_encryption(&mut assign_stmt.to_statement(), p);

                // Manually add to circuit inputs
                param_stmts.push(assign_stmt.to_ast());
                offset += cipher_payload_len;
            }
        }

        // Populate sender field of parameters encrypted with a symmetric cipher
        let mut copy_stmts = vec![];
        for p in &original_params {
            if p.identifier_declaration_base.annotated_type.is_cipher() {
                let c = p
                    .identifier_declaration_base
                    .annotated_type
                    .type_name
                    .clone();
                assert!(is_instance(&*c, ASTType::CipherText));
                if c.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap().crypto_params.is_symmetric_cipher() {
                    let sender_key = LocationExpr::IdentifierExpr(in_arr_var.clone())
                        .index(ExprUnion::I32(me_key_idx[&c.try_as_array_ref().unwrap().try_as_cipher_text_ref().unwrap().crypto_params]));
                    let idf = IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(*p.identifier_declaration_base.idf.clone()),
                        None,
                    )
                    .as_type(AST::AnnotatedTypeName(
                        *p.identifier_declaration_base.annotated_type.clone(),
                    ));
                    let cipher_payload_len = CFG
                        .lock()
                        .unwrap()
                        .user_config
                        .get_crypto_params(
                            &p.identifier_declaration_base.annotated_type.homomorphism,
                        )
                        .cipher_payload_len();
                    let lit = ArrayLiteralExpr::ArrayLiteralExpr(ArrayLiteralExprBase::new(
                        (0..cipher_payload_len)
                            .map(|i| {
                                LocationExpr::IdentifierExpr(idf.clone())
                                    .index(ExprUnion::I32(i))
                                    .to_expr()
                            })
                            .chain([sender_key.to_expr()])
                            .collect(),
                    ));
                    copy_stmts.push(
                        VariableDeclarationStatement::new(
                            VariableDeclaration::new(
                                vec![],
                                *p.identifier_declaration_base.annotated_type.clone(),
                                *p.identifier_declaration_base.idf.clone(),
                                None,
                            ),
                            Some(lit.to_expr()),
                        )
                        .to_ast(),
                    );
                }
            }
        }
        if !copy_stmts.is_empty() {
            param_stmts.extend(vec![
                CommentBase::new(String::new()).to_ast(),
                CommentBase::new(String::from(
                    "Copy from calldata to memory and set sender field",
                ))
                .to_ast(),
            ]);
            param_stmts.extend(copy_stmts);
        }

        // Declare in array
        let new_in_array_expr = NewExpr::new(
            AnnotatedTypeName::new(TypeName::dyn_uint_array(), None, String::from("NON_")),
            vec![NumberLiteralExpr::new(ext_circuit.in_size_trans(), false).to_expr()],
        );
        let in_var_decl = (*in_arr_var.idf.clone()).identifier_base_ref().decl_var(
            AST::TypeName(TypeName::dyn_uint_array()),
            Some(new_in_array_expr.to_expr()),
        );
        stmts.push(in_var_decl.into_ast());
        stmts.push(CommentBase::new(String::new()).to_ast());
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
                IdentifierExpr::new(
                    IdentifierExprUnion::Identifier(*param.identifier_declaration_base.idf.clone()),
                    None,
                )
                .to_expr()
            })
            .collect();
        let mut idf = IdentifierExpr::new(
            IdentifierExprUnion::Identifier(int_fct.namespace_definition_base.idf.clone()),
            None,
        );
        idf.location_expr_base.target = Some(Box::new(int_fct.to_ast()));
        let mut internal_call = FunctionCallExprBase::new(idf.to_expr(), args.clone(), None);
        internal_call.sec_start_offset = Some(ext_circuit.priv_in_size());

        if int_fct.requires_verification {
            ext_circuit.call_function(&FunctionCallExpr::FunctionCallExpr(internal_call.clone()));
            args.extend(vec![
                in_arr_var.to_expr(),
                NumberLiteralExpr::new(ext_circuit.in_size(), false).to_expr(),
                IdentifierExpr::new(
                    IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                    None,
                )
                .to_expr(),
                NumberLiteralExpr::new(ext_circuit.out_size(), false).to_expr(),
            ]);
        }

        let in_call = if !int_fct.return_parameters.is_empty() {
            stmts.extend(CommentBase::comment_list(
                String::from("Declare return variables"),
                int_fct
                    .return_var_decls
                    .iter()
                    .map(|vd| {
                        VariableDeclarationStatement::new(
                            deep_copy(Some(vd.to_ast()), false, false)
                                .unwrap()
                                .try_as_identifier_declaration_ref()
                                .unwrap()
                                .try_as_variable_declaration_ref()
                                .unwrap()
                                .clone(),
                            None,
                        )
                        .to_ast()
                    })
                    .collect(),
            ));
            TupleExpr::new(
                int_fct
                    .return_var_decls
                    .iter()
                    .map(|vd| {
                        IdentifierExpr::new(
                            IdentifierExprUnion::Identifier(
                                *vd.identifier_declaration_base.idf.clone(),
                            ),
                            None,
                        )
                        .to_expr()
                    })
                    .collect(),
            )
            .assign(internal_call.to_expr())
            .to_ast()
        } else {
            ExpressionStatement::new(internal_call.to_expr()).to_ast()
        };
        stmts.push(CommentBase::new(String::from("Call internal function")).to_ast());
        stmts.push(in_call);
        stmts.push(CommentBase::new(String::new()).to_ast());

        // Call verifier
        if requires_proof && !CFG.lock().unwrap().user_config.disable_verification() {
            let verifier = IdentifierExpr::new(
                IdentifierExprUnion::String(
                    CFG.lock().unwrap().get_contract_var_name(
                        ext_circuit
                            .clone()
                            .verifier_contract_type
                            .unwrap()
                            .to_ast()
                            .code(),
                    ),
                ),
                None,
            );
            let verifier_args = vec![
                IdentifierExpr::new(
                    IdentifierExprUnion::String(CFG.lock().unwrap().proof_param_name()),
                    None,
                )
                .to_expr(),
                IdentifierExpr::new(
                    IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
                    None,
                )
                .to_expr(),
                IdentifierExpr::new(
                    IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                    None,
                )
                .to_expr(),
            ];
            let verify = ExpressionStatement::new(
                LocationExpr::IdentifierExpr(verifier)
                    .call(
                        IdentifierExprUnion::String(
                            CFG.lock().unwrap().verification_function_name(),
                        ),
                        verifier_args,
                    )
                    .to_expr(),
            );
            stmts.push(
                StatementListBase::new(
                    vec![
                        CommentBase::new(String::from("Verify zk proof of execution")).to_ast(),
                        verify.to_ast(),
                    ],
                    true,
                )
                .to_ast(),
            );
        }

        // Add return statement at the end if necessary
        if !int_fct.return_parameters.is_empty() {
            stmts.push(
                ReturnStatement::new(Some(
                    TupleExpr::new(
                        int_fct
                            .return_var_decls
                            .iter()
                            .map(|vd| {
                                IdentifierExpr::new(
                                    IdentifierExprUnion::Identifier(
                                        *vd.identifier_declaration_base.idf.clone(),
                                    ),
                                    None,
                                )
                                .to_expr()
                            })
                            .collect(),
                    )
                    .to_expr(),
                ))
                .to_ast(),
            );
        }

        Block::new(stmts, false)
    }
}
