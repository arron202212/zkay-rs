// """
// This module provides functionality to transform a zkay AST into an equivalent public solidity AST + proof circuits
// """
use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::compiler::privacy::library_contracts::BN128_SCALAR_FIELD;
use crate::compiler::privacy::transformation::internal_call_transformer::{
    compute_transitive_circuit_io_sizes, transform_internal_calls,
};
use crate::compiler::privacy::transformation::zkay_transformer::{
    ZkayCircuitTransformer, ZkayExpressionTransformer, ZkayStatementTransformer,
    ZkayVarDeclTransformer,
};
use crate::config::CFG;
use crate::transaction::crypto::params::CryptoParams;
use crate::zkay_ast::analysis::used_homomorphisms::UsedHomomorphismsVisitor;
use crate::zkay_ast::ast::{
    is_instance, ASTType, AnnotatedTypeName, Array, ArrayBase, ArrayLiteralExpr, BlankLine, Block,
    CipherText, Comment, ConstructorOrFunctionDefinition, ContractDefinition, ContractTypeName,
    Expression, ExpressionStatement, FunctionCallExpr, Identifier, IdentifierBase, IdentifierExpr,
    MeExpr, NewExpr, NumberLiteralExpr, Parameter, PrimitiveCastExpr, PrivacyLabelExpr,
    RequireStatement, ReturnStatement, SourceUnit, StateVariableDeclaration, StatementList,
    StructDefinition, StructTypeName, TupleExpr, TypeName, VariableDeclaration,
    VariableDeclarationStatement, AST,
};
use crate::zkay_ast::pointers::parent_setter::set_parents;
use crate::zkay_ast::pointers::symbol_table::link_identifiers;
use crate::zkay_ast::visitor::deep_copy::deep_copy;
use crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor;
use std::collections::BTreeMap;

pub fn transform_ast<
    V: Clone
        + std::marker::Sync
        + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
>(
    ast: SourceUnit,
) -> (
    SourceUnit,
    BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper<V>>,
)
// """
    // Convert zkay to solidity AST + proof circuits

    // :param ast: zkay AST
    // :return: solidity AST and dictionary which maps all function definitions which require verification
    //          to the corresponding circuit helper instance.
    // """
{
    let zt = ZkayTransformer::new();
    let new_ast = zt.visit(ast);

    // restore all parent pointers and identifier targets
    set_parents(new_ast);
    link_identifiers(new_ast);
    (new_ast, zt.circuits)
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
pub struct ZkayTransformer<
    V: Clone
        + std::marker::Sync
        + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
> {
    circuits: BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper<V>>,
    var_decl_trafo: ZkayVarDeclTransformer<V>,
}
impl<
        V: Clone
            + std::marker::Sync
            + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
    > ZkayTransformer<V>
{
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
        su: &SourceUnit,
        corresponding_circuit: Option<CircuitHelper<V>>,
    )
    // """
    // Import contract "vname" into the given source unit.

    // :param cname: contract name (.sol filename stem must match contract type name)
    // :param su: [SIDE EFFECT] source unit where contract should be imported
    // :param corresponding_circuit: [SIDE EFFECT] if contract is a verification contract, this should be the corresponding circuit helper
    // """
    {
        let import_filename = format!("./{cname}.sol");
        su.used_contracts.push(import_filename);

        if corresponding_circuit.is_some() {
            let c_type =
                ContractTypeName::new(vec![Identifier::Identifier(IdentifierBase::new(cname))]);
            corresponding_circuit.register_verification_contract_metadata(c_type, import_filename);
        }
    }

    pub fn create_contract_variable(cname: &str) -> StateVariableDeclaration
// """Create a public constant state variable with which contract with name "cname" can be accessed"""
    {
        let inst_idf = Identifier::Identifier(IdentifierBase::new(
            CFG.lock().unwrap().get_contract_var_name(cname),
        ));
        let c_type = ContractTypeName::new([Identifier::Identifier(IdentifierBase::new(cname))]);

        let cast_0_to_c = PrimitiveCastExpr::new(c_type, NumberLiteralExpr::new(0));
        StateVariableDeclaration::new(
            AnnotatedTypeName::new(c_type),
            vec!["public", "constant"],
            inst_idf.clone(),
            cast_0_to_c,
        )
    }

    pub fn include_verification_contracts(
        self,
        su: SourceUnit,
        c: ContractDefinition,
    ) -> Vec<StateVariableDeclaration>
// """
        // Import all verification contracts for "c" into "su" and create state variable declarations for all of them + the pki contract.

        // :param su: [SIDE EFFECT] source unit into which contracts should be imported
        // :param c: contract for which verification contracts should be imported
        // :return: list of all constant state variable declarations for the pki contract + all the verification contracts
        // """
    {
        let mut contract_var_decls = vec![];
        for crypto_params in c.used_crypto_backends {
            let contract_name = CFG.lock().unwrap().get_pki_contract_name(crypto_params);
            contract_var_decls.push(self.create_contract_variable(contract_name));
        }

        for f in c
            .constructor_definitions
            .iter()
            .chain(c.function_definitions)
        {
            if f.requires_verification_when_external && f.has_side_effects {
                let name = CFG
                    .lock()
                    .unwrap()
                    .get_verification_contract_name(c.idf.name, f.name);
                self.import_contract(name, su, self.circuits[f]);
                contract_var_decls.push(self.create_contract_variable(name));
            }
        }

        contract_var_decls
    }

    pub fn create_circuit_helper(
        fct: ConstructorOrFunctionDefinition,
        global_owners: Vec<PrivacyLabelExpr>,
        internal_circ: Option<CircuitHelper<V>>,
    )
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
            fct,
            global_owners,
            ZkayExpressionTransformer::new,
            ZkayCircuitTransformer::new,
            internal_circ,
        )
    }

    pub fn visitSourceUnit(self, ast: SourceUnit)
    // Figure out which crypto backends were used
    {
        UsedHomomorphismsVisitor::new().visit(ast);

        for crypto_params in ast.used_crypto_backends {
            self.import_contract(
                CFG.lock().unwrap().get_pki_contract_name(crypto_params),
                ast,
            );
        }

        for c in ast.contracts {
            self.transform_contract(ast, c);
        }

        ast
    }

    pub fn transform_contract(self, su: SourceUnit, c: ContractDefinition) -> ContractDefinition
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
        let all_fcts = c
            .constructor_definitions
            .iter()
            .chain(c.function_definitions);

        // Get list of static owner labels for this contract
        let mut global_owners = vec![Expression::me_expr()];
        for var in c.state_variable_declarations {
            if var.annotated_type.is_address() && (var.is_final || var.is_constant) {
                global_owners.push(var.idf);
            }
        }

        // Backup untransformed function bodies
        for fct in all_fcts {
            fct.original_body = deep_copy(fct.body, true, true);
        }

        // Transform types of normal state variables
        c.state_variable_declarations = self
            .var_decl_trafo
            .visit_list(c.state_variable_declarations);

        // Split into functions which require verification and those which don"t need a circuit helper
        let mut req_ext_fcts = {};
        let (new_fcts, new_constr) = ([], []);
        for fct in all_fcts {
            assert!(is_instance(&fct, ASTType::ConstructorOrFunctionDefinition));
            if fct.requires_verification || fct.requires_verification_when_external {
                self.circuits[fct] = self.create_circuit_helper(fct, global_owners);
            }

            if fct.requires_verification_when_external {
                req_ext_fcts[fct] = fct.parameters.clone();
            } else if fct.is_constructor {
                new_constr.push(fct);
            } else {
                new_fcts.push(fct);
            }
        }

        // Add constant state variables for external contracts and field prime
        let field_prime_decl = StateVariableDeclaration::new(
            AnnotatedTypeName::uint_all(),
            ["public", "constant"],
            Identifier::new(CFG.lock().unwrap().field_prime_var_name),
            NumberLiteralExpr::new(BN128_SCALAR_FIELD),
        );
        let contract_var_decls = self.include_verification_contracts(su, c);
        c.state_variable_declarations = [field_prime_decl, Comment::new("")]
            .into_iter()
            .chain(Comment::comment_list(
                "Helper Contracts",
                contract_var_decls,
            ))
            .chain([Comment::new("User state variables")])
            .chain(c.state_variable_declarations)
            .collect();

        // Transform signatures
        for f in all_fcts {
            f.parameters = self.var_decl_trafo.visit_list(f.parameters);
        }
        for f in c.function_definitions {
            f.return_parameters = self.var_decl_trafo.visit_list(f.return_parameters);
            f.return_var_decls = self.var_decl_trafo.visit_list(f.return_var_decls);
        }

        // Transform bodies
        for fct in all_fcts {
            let gen = self.circuits.get(fct, None);
            fct.body = ZkayStatementTransformer::new(gen).visit(fct.body);
        }

        // Transform (internal) functions which require verification (add the necessary additional parameters and boilerplate code)
        let fcts_with_verification: Vec<_> = all_fcts
            .iter()
            .filter_map(|fct| {
                if fct.requires_verification {
                    Some(fct)
                } else {
                    None
                }
            })
            .collect();
        compute_transitive_circuit_io_sizes(fcts_with_verification, self.circuits);
        transform_internal_calls(fcts_with_verification, self.circuits);
        for f in fcts_with_verification {
            let circuit = self.circuits[f];
            assert!(circuit.requires_verification());
            if circuit.requires_zk_data_struct()
            // Add zk data struct for f to contract
            {
                let zk_data_struct = StructDefinition::new(
                    Identifier::Identifier(IdentifierBase::new(circuit.zk_data_struct_name)),
                    circuit
                        .output_idfs
                        .iter()
                        .chain(circuit.input_idfs)
                        .iter()
                        .map(|idf| {
                            VariableDeclaration::new(
                                vec![],
                                AnnotatedTypeName::new(idf.t),
                                idf.clone(),
                                "",
                            )
                        })
                        .collect(),
                );
                c.struct_definitions.push(zk_data_struct);
            }
            self.create_internal_verification_wrapper(f);
        }

        // Create external wrapper functions where necessary
        for (f, params) in req_ext_fcts {
            let (ext_f, int_f) =
                self.split_into_external_and_internal_fct(f, params, global_owners);
            if ext_f.is_function {
                new_fcts.push(ext_f);
            } else {
                new_constr.push(ext_f);
            }
            new_fcts.push(int_f);
        }

        c.constructor_definitions = new_constr;
        c.function_definitions = new_fcts;
        return c;
    }

    pub fn create_internal_verification_wrapper(self, ast: ConstructorOrFunctionDefinition)
    // """
    // Add the necessary additional parameters and boiler plate code for verification support to the given function.
    // :param ast: [SIDE EFFECT] Internal function which requires verification
    // """
    {
        let circuit = self.circuits[&ast].clone();
        let mut stmts = vec![];

        let symmetric_cipher_used = ast
            .used_crypto_backends
            .iter()
            .any(|backend| backend.is_symmetric_cipher());
        if symmetric_cipher_used && ast.modifiers.contain("pure")
        // Symmetric trafo requires msg.sender access -> change from pure to view
        {
            ast.modifiers = ast
                .modifiers
                .iter()
                .map(|modi| if modi == "pure" { "view" } else { modi })
                .collect();
        }

        // Add additional params
        ast.add_param(
            Array::Array(ArrayBase::new(AnnotatedTypeName::uint_all())),
            CFG.lock().unwrap().zk_in_name,
        );
        ast.add_param(
            AnnotatedTypeName::uint_all(),
            format!("{}_start_idx", CFG.lock().unwrap().zk_in_name),
        );
        ast.add_param(
            Array::Array(ArrayBase::new(AnnotatedTypeName::uint_all())),
            CFG.lock().unwrap().zk_out_name,
        );
        ast.add_param(
            AnnotatedTypeName::uint_all(),
            format!("{}_start_idx", CFG.lock().unwrap().zk_out_name),
        );

        // Verify that in/out parameters have correct size
        let (out_start_idx, in_start_idx) = (
            IdentifierExpr::new(format!("{}_start_idx", CFG.lock().unwrap().zk_out_name)),
            IdentifierExpr::new(format!("{}_start_idx", CFG.lock().unwrap().zk_in_name)),
        );
        let (out_var, in_var) = (
            IdentifierExpr::new(CFG.lock().unwrap().zk_out_name),
            IdentifierExpr::new(CFG.lock().unwrap().zk_in_name)
                .as_type(Array::Array(AnnotatedTypeName::uint_all())),
        );
        stmts.push(RequireStatement::new(
            out_start_idx
                .binop("+", NumberLiteralExpr::new(circuit.out_size_trans))
                .binop("<=", out_var.dot("length")),
        ));
        stmts.push(RequireStatement::new(
            in_start_idx
                .binop("+", NumberLiteralExpr::new(circuit.in_size_trans))
                .binop("<=", in_var.dot("length")),
        ));

        // Declare zk_data struct var (if needed)
        if circuit.requires_zk_data_struct() {
            let zk_struct_type = StructTypeName::new(vec![Identifier::Identifier(
                IdentifierBase::new(circuit.zk_data_struct_name),
            )]);
            stmts.extend(vec![
                Identifier::Identifier(IdentifierBase::new(CFG.lock().unwrap().zk_data_var_name))
                    .decl_var(zk_struct_type),
                BlankLine::new(),
            ]);
        }

        // Declare return variable if necessary
        if ast.return_parameters {
            stmts += Comment::comment_list(
                "Declare return variables",
                ast.return_var_decls
                    .iter()
                    .map(|vd| VariableDeclarationStatement::new(vd))
                    .collect(),
            );
        }

        // Find all me-keys in the in array
        let mut me_key_idx = BTreeMap::new();
        let mut offset = 0;
        for (key_owner, crypto_params) in &circuit.requested_global_keys {
            if key_owner == MeExpr::new() {
                assert!(!me_key_idx.contains(crypto_params));
                me_key_idx.insert(crypto_params, offset);
            }
            offset += crypto_params.key_len;
        }

        // Deserialize out array (if any)
        let mut deserialize_stmts = vec![];
        let mut offset = 0;
        for s in circuit.output_idfs {
            deserialize_stmts.push(s.deserialize(
                CFG.lock().unwrap().zk_out_name,
                out_start_idx,
                offset,
            ));
            if is_instance(&s.t, ASTType::CipherText) && s.t.crypto_params.is_symmetric_cipher()
            // Assign sender field to user-encrypted values if necessary
            // Assumption: s.t.crypto_params.key_len == 1 for all symmetric ciphers
            {
                assert!(
                    me_key_idx.contains(&s.t.crypto_params),
                    "Symmetric cipher but did not request me key"
                );
                let key_idx = me_key_idx[&s.t.crypto_params];
                let sender_key = in_var.index(key_idx);
                let cipher_payload_len = s.t.crypto_params.cipher_payload_len;
                deserialize_stmts.push(
                    s.get_loc_expr()
                        .index(cipher_payload_len)
                        .assign(sender_key),
                );
            }
            offset += s.t.size_in_uints;
        }
        if deserialize_stmts {
            stmts.push(StatementList::new(
                Comment::comment_wrap_block("Deserialize output values", deserialize_stmts),
                true,
            ));
        }

        // Include original transformed function body
        stmts.extend(ast.body.statements);

        // Serialize in parameters to in array (if any)
        let mut serialize_stmts = vec![];
        let mut offset = 0;
        for s in circuit.input_idfs {
            serialize_stmts.extend(vec![s.serialize(
                CFG.lock().unwrap().zk_in_name,
                in_start_idx,
                offset,
            )]);
            offset += s.t.size_in_uints;
        }
        if offset {
            stmts.push(Comment::new(""));
            stmts.extend(Comment::comment_wrap_block(
                "Serialize input values",
                serialize_stmts,
            ));
        }

        // Add return statement at the end if necessary
        // (was previously replaced by assignment to return_var by ZkayStatementTransformer)
        if circuit.has_return_var {
            stmts.push(ReturnStatement::new(TupleExpr::new(
                ast.return_var_decls
                    .iter()
                    .map(|vd| {
                        let mut idf = IdentifierExpr::new(vd.idf.clone());
                        idf.target = vd;
                        idf
                    })
                    .collect(),
            )));
        }

        ast.body.statements = stmts;
    }

    pub fn split_into_external_and_internal_fct(
        self,
        f: ConstructorOrFunctionDefinition,
        mut original_params: Vec<Parameter>,
        global_owners: Vec<PrivacyLabelExpr>,
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
        let new_modifiers = if f.is_function {
            original_params = original_params
                .iter()
                .map(|p| deep_copy(p, true).with_changed_storage("memory", "calldata"))
                .collect();
            vec!["external"]
        } else {
            vec!["public"]
        };
        if f.is_payable {
            new_modifiers.push("payable");
        }

        let mut requires_proof = true;
        if !f.has_side_effects {
            requires_proof = false;
            new_modifiers.push("view");
        }
        let new_f = ConstructorOrFunctionDefinition::new(
            f.idf,
            original_params,
            new_modifiers,
            f.return_parameters,
            Block::new(vec![]),
        );

        // Make original function internal
        f.idf = Identifier::new(CFG.lock().unwrap().get_internal_name(f));
        f.modifiers = f
            .modifiers
            .iter()
            .filter_map(|modi| {
                if modi != "payable" {
                    Some(if modi == "public" { "internal" } else { modi })
                } else {
                    None
                }
            })
            .collect();
        f.requires_verification_when_external = false;

        // Create new circuit for external function
        let circuit = self.create_circuit_helper(new_f, global_owners, self.circuits[f]);
        if !f.requires_verification {
            self.circuits.remove(f);
        }
        self.circuits[new_f] = circuit;

        // Set meta attributes and populate body
        new_f.requires_verification = true;
        new_f.requires_verification_when_external = true;
        new_f.called_functions = f.called_functions;
        new_f.called_functions[f] = None;
        new_f.used_crypto_backends = f.used_crypto_backends;
        new_f.body = self.create_external_wrapper_body(f, circuit, original_params, requires_proof);

        // Add out and proof parameter to external wrapper
        let storage_loc = if new_f.is_function {
            "calldata"
        } else {
            "memory"
        };
        new_f.add_param(
            Array::Array(AnnotatedTypeName::uint_all()),
            Identifier::new(CFG.lock().unwrap().zk_out_name),
            storage_loc,
        );

        if requires_proof {
            new_f.add_param(
                AnnotatedTypeName::proof_type(),
                Identifier::new(CFG.lock().unwrap().proof_param_name),
                storage_loc,
            );
        }

        (new_f, f)
    }

    pub fn create_external_wrapper_body(
        int_fct: ConstructorOrFunctionDefinition,
        ext_circuit: CircuitHelper<V>,
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
        let priv_args = original_params
            .iter()
            .filter_map(|p| {
                if p.annotated_type.is_cipher() {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();
        let args_backends: Vec<_> = priv_args
            .iter()
            .map(|p| p.annotated_type.type_name.crypto_params.clone())
            .collect();
        let mut stmts = vec![];

        for crypto_params in args_backends {
            assert!(int_fct.used_crypto_backends.contains(&crypto_params));
            // If there are any private arguments with homomorphism "hom", we need the public key for that crypto backend
            ext_circuit._require_public_key_for_label_at(
                None,
                Expression::me_expr(),
                crypto_params,
            );
        }
        for crypto_params in CFG.lock().unwrap().all_crypto_params() {
            if crypto_params.is_symmetric_cipher() {
                if ext_circuit
                    .requested_global_keys
                    .contains(&(MeExpr::new(), crypto_params))
                    || args_backends.contains(&crypto_params)
                // Make sure msg.sender"s key pair is available in the circuit
                {
                    stmts += ext_circuit.request_private_key(crypto_params);
                }
            }
        }

        // Verify that out parameter has correct size
        stmts.push(RequireStatement::new(
            IdentifierExpr::new(CFG.lock().unwrap().zk_out_name)
                .dot("length")
                .binop("==", NumberLiteralExpr::New(ext_circuit.out_size_trans)),
        ));

        // IdentifierExpr for array var holding serialized public circuit inputs
        let in_arr_var = IdentifierExpr::new(CFG.lock().unwrap().zk_in_name)
            .as_type(Array::Array(ArrayBase::new(AnnotatedTypeName::uint_all())));

        // Request static public keys
        let mut offset = 0;
        let key_req_stmts = vec![];
        let mut me_key_idx = BTreeMap::new();
        if ext_circuit.requested_global_keys {
            // Ensure that me public key is stored starting at in[0]
            let keys = ext_circuit.requested_global_keys.clone();

            let tmp_keys = BTreeMap::new();
            for crypto_params in int_fct.used_crypto_backends {
                let tmp_key_var = Identifier::Identifier(IdentifierBase::new(format!(
                    "_tmp_key_{}",
                    crypto_params.identifier_name
                )));
                key_req_stmts
                    .push(tmp_key_var.decl_var(AnnotatedTypeName::key_type(crypto_params)));
                tmp_keys.insert(crypto_params, tmp_key_var);
            }
            for (key_owner, crypto_params) in keys {
                let tmp_key_var = tmp_keys[crypto_params];
                let (idf, assignment) = ext_circuit.request_public_key(
                    crypto_params,
                    key_owner,
                    ext_circuit.get_glob_key_name(key_owner, crypto_params),
                );
                assignment.lhs = IdentifierExpr::new(tmp_key_var.clone());
                key_req_stmts.push(assignment);

                // Remember me-keys for later use in symmetrically encrypted keys
                if key_owner == MeExpr::new() {
                    assert!(!me_key_idx.contains(&crypto_params));
                    me_key_idx[crypto_params] = offset;
                }

                // Manually add to circuit inputs
                let key_len = crypto_params.key_len;
                key_req_stmts.push(
                    in_arr_var
                        .slice(offset, key_len)
                        .assign(IdentifierExpr::new(tmp_key_var.clone()).slice(0, key_len)),
                );
                offset += key_len;
                assert!(offset == ext_circuit.in_size);
            }
        }

        // Check encrypted parameters
        let mut param_stmts = vec![];
        for p in original_params {
            // """ * of T_e rule 8 """
            if p.annotated_type.is_cipher() {
                let cipher_payload_len =
                    p.annotated_type.type_name.crypto_params.cipher_payload_len;
                let assign_stmt = in_arr_var
                    .slice(offset, cipher_payload_len)
                    .assign(IdentifierExpr::new(p.idf.clone()).slice(0, cipher_payload_len));
                ext_circuit.ensure_parameter_encryption(&assign_stmt, p);

                // Manually add to circuit inputs
                param_stmts.push(assign_stmt);
                offset += cipher_payload_len;
            }
        }

        // Populate sender field of parameters encrypted with a symmetric cipher
        let mut copy_stmts = vec![];
        for p in original_params {
            if p.annotated_type.is_cipher() {
                let c = &p.annotated_type.type_name;
                assert!(is_instance(c, ASTType::CipherText));
                if c.crypto_params.is_symmetric_cipher() {
                    let sender_key = in_arr_var.index(me_key_idx[c.crypto_params]);
                    let idf = IdentifierExpr::new(p.idf.clone()).as_type(p.annotated_type.clone());
                    let cipher_payload_len = CFG
                        .lock()
                        .unwrap()
                        .get_crypto_params(p.annotated_type.homomorphism)
                        .cipher_payload_len;
                    let lit = ArrayLiteralExpr::new(
                        (0..cipher_payload_len)
                            .map(|i| idf.clone().index(i))
                            .chain([sender_key])
                            .collect(),
                    );
                    copy_stmts.push(VariableDeclarationStatement::new(
                        VariableDeclaration::new(
                            vec![],
                            p.annotated_type.clone(),
                            p.idf.clone(),
                            "memory",
                        ),
                        lit,
                    ));
                }
            }
        }
        if copy_stmts {
            param_stmts += [
                Comment::new(""),
                Comment::new("Copy from calldata to memory and set sender field"),
            ] + copy_stmts;
        }

        // Declare in array
        let new_in_array_expr = NewExpr::new(
            AnnotatedTypeName::new(TypeName::dyn_uint_array()),
            vec![NumberLiteralExpr::new(ext_circuit.in_size_trans)],
        );
        let in_var_decl = in_arr_var
            .idf
            .decl_var(TypeName::dyn_uint_array(), new_in_array_expr);
        stmts.push(in_var_decl);
        stmts.push(Comment::new(""));
        stmts += Comment::comment_wrap_block("Request static public keys", key_req_stmts);
        stmts +=
            Comment::comment_wrap_block("Backup private arguments for verification", param_stmts);

        // Call internal function
        let args = original_params
            .iter()
            .map(|param| IdentifierExpr::new(param.idf.clone()))
            .collect();
        let mut idf = IdentifierExpr::new(int_fct.idf.clone());
        idf.target = int_fct;
        let mut internal_call = FunctionCallExpr::new(idf, args);
        internal_call.sec_start_offset = ext_circuit.priv_in_size;

        if int_fct.requires_verification {
            ext_circuit.call_function(internal_call);
            args.extend(vec![
                in_arr_var.clone(),
                NumberLiteralExpr::new(ext_circuit.in_size),
                IdentifierExpr::new(CFG.lock().unwrap().zk_out_name),
                NumberLiteralExpr::new(ext_circuit.out_size),
            ]);
        }

        let in_call = if int_fct.return_parameters {
            stmts += Comment::comment_list(
                "Declare return variables",
                int_fct
                    .return_var_decls
                    .iter()
                    .map(|vd| VariableDeclarationStatement::new(deep_copy(vd)))
                    .collect(),
            );
            TupleExpr::new(
                int_fct
                    .return_var_decls
                    .iter()
                    .map(|vd| IdentifierExpr::new(vd.idf.clone()))
                    .collect(),
            )
            .assign(internal_call)
        } else {
            ExpressionStatement::new(internal_call)
        };
        stmts.push(Comment::new("Call internal function"));
        stmts.push(in_call);
        stmts.push(Comment::new(""));

        // Call verifier
        if requires_proof && !CFG.lock().unwrap().disable_verification {
            let verifier = IdentifierExpr::new(
                CFG.lock()
                    .unwrap()
                    .get_contract_var_name(ext_circuit.verifier_contract_type.code()),
            );
            let verifier_args = [
                IdentifierExpr::new(CFG.lock().unwrap().proof_param_name),
                IdentifierExpr::new(CFG.lock().unwrap().zk_in_name),
                IdentifierExpr::new(CFG.lock().unwrap().zk_out_name),
            ];
            let verify = ExpressionStatement::new(verifier.call(
                CFG.lock().unwrap().verification_function_name,
                verifier_args,
            ));
            stmts.push(StatementList::new(
                vec![Comment::new("Verify zk proof of execution"), verify],
                true,
            ));
        }

        // Add return statement at the end if necessary
        if int_fct.return_parameters {
            stmts.push(ReturnStatement::new(TupleExpr::new(
                int_fct
                    .return_var_decls
                    .iter()
                    .map(|vd| IdentifierExpr::new(vd.idf.clone()))
                    .collect(),
            )));
        }

        Block::new(stmts)
    }
}
