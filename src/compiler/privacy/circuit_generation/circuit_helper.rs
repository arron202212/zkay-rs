use crate::compiler::name_remapper::CircVarRemapper;
use crate::compiler::privacy::circuit_generation::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use crate::compiler::privacy::circuit_generation::name_factory::NameFactory;
use crate::config::CFG;
use crate::transaction::crypto::params::CryptoParams;
use crate::type_check::type_checker::TypeCheckVisitor;
use crate::zkay_ast::analysis::partition_state::PartitionState;
use crate::zkay_ast::ast::{
    get_privacy_expr_from_label, is_instance, is_instances, ASTType, AllExpr, AnnotatedTypeName,
    AssignmentStatement, Block, BooleanLiteralType, BuiltinFunction, CircuitComputationStatement,
    CircuitInputStatement, ConstructorOrFunctionDefinition, EncryptionExpression,
    EnterPrivateKeyStatement, Expression, ExpressionStatement, FunctionCallExpr, HybridArgType,
    HybridArgumentIdf, Identifier, IdentifierBase, IdentifierExpr, IfStatement, IndexExpr,
    KeyLiteralExpr, LocationExpr, MeExpr, MemberAccessExpr, NumberLiteralExpr, NumberLiteralType,
    Parameter, PrivacyLabelExpr, ReturnStatement, StateVariableDeclaration, Statement, TupleExpr,
    TypeName, UserDefinedTypeName, VariableDeclaration, VariableDeclarationStatement,
};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::deep_copy::deep_copy;
use crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor;
use std::collections::{BTreeMap, BTreeSet};
// class CircuitHelper

// """
// This class is used to construct abstract proof circuits during contract transformation.

// Typically there is one instance of this class for every function which requires verification.
// """
pub struct CircuitHelper<T> {
    // Function and verification contract corresponding to this circuit
    fct: ConstructorOrFunctionDefinition,
    verifier_contract_filename: Option<String>,
    verifier_contract_type: Option<UserDefinedTypeName>,
    // Metadata set later by ZkayContractTransformer
    has_return_var: bool,
    // Transformer visitors
    _expr_trafo: T, //AstTransformerVisitor
    _circ_trafo: T,
    // List of proof circuit statements (assertions and assignments)
    // WARNING: Never assign to let _phi, always access it using the phi property and only mutate it
    _phi: Vec<CircuitStatement>,
    // Name factory for private circuit inputs
    _secret_input_name_factory: NameFactory,
    // Name factory for temporary internal circuit variables
    _circ_temp_name_factory: NameFactory,
    // Name factory for public circuit inputs
    _in_name_factory: NameFactory,
    // Name factory for public circuit outputs
    _out_name_factory: NameFactory,
    //For a given owner label (idf or me), stores the corresponding assignment of the requested key to the corresponding in variable
    // List of all statically known privacy labels for the contract of which this circuit is part of
    static_owner_labels: Vec<PrivacyLabelExpr>,
    // For each statement, cache the generated variable holding the requested public key of a given
    // not-statically-known identifier, to prevent requesting the same key over and over again
    _requested_dynamic_pks: BTreeMap<Statement, BTreeMap<Identifier, HybridArgumentIdf>>,
    // The crypto backends for which msg.sender"s secret key must be added to the private circuit inputs
    _needed_secret_key: BTreeSet<CryptoParams>,
    // Set of statically known privacy labels (OrderedDict is used to ensure deterministic iteration order)
    _global_keys: BTreeSet<((Option<MeExpr>, Option<Identifier>), CryptoParams)>,
    // List of all (non-transitive) calls in let fct"s body to functions which require verification, in AST visiting order
    // This is internally used to compute transitive in/out/privin sizes, but may also be useful when implementing a new
    // circuit generator backend.
    function_calls_with_verification: Vec<FunctionCallExpr>,
    // Set (with deterministic order) of all functions which this circuit transitively calls.
    transitively_called_functions: BTreeSet<ConstructorOrFunctionDefinition>,
    trans_priv_size: i32,
    trans_in_size: i32,
    trans_out_size: i32,
    // Remapper instance used for SSA simulation
    _remapper: CircVarRemapper,
}

impl<T> CircuitHelper<T> {
    pub fn new(
        fct: ConstructorOrFunctionDefinition,
        static_owner_labels: Vec<PrivacyLabelExpr>,
        expr_trafo_constructor: impl FnOnce(&Self) -> T,
        circ_trafo_constructor: impl FnOnce(&Self) -> T,
        internal_circuit: &mut Option<CircuitHelper<T>>,
    ) -> Self {
        // """
        // Create a new CircuitHelper instance

        // :param fct: The function which is associated with this proof circuit
        // :param static_owner_labels: A list of all static privacy labels for this contract
        //                             (i.e. MeExpr + Identifiers of all final address state variables)
        // :param expr_trafo_constructor: Constructor of ZkayExpressionTransformer (cyclic dependency)
        // :param circ_trafo_constructor: Constructor fo ZkayCircuitTransformer (cyclic dependency)
        // :param internal_circuit [Optional]: When creating the external wrapper function (see ZkayContractTransformer),
        //                                     this should point to the CircuitHelper of the corresponding internal function.
        //                                     This circuit will then be initialized with the internal circuits data.
        // """

        // super().__init__()
        let verifier_contract_filename: Option<str> = None;
        let verifier_contract_type: Option<UserDefinedTypeName> = None;
        let _expr_trafo: T = AstTransformerVisitor::default(); //expr_trafo_constructor(&self);
        let _circ_trafo: T = AstTransformerVisitor::default(); //circ_trafo_constructor(&self);
        let mut _needed_secret_key = BTreeMap::new();
        let mut _global_keys = BTreeMap::new();
        let transitively_called_functions = BTreeMap::new();
        let (mut trans_priv_size, mut trans_in_size, mut trans_out_size) = (0, 0, 0); //Set later by transform_internal_calls
        if let Some(mut internal_circuit) = internal_circuit {
            //Inherit metadata from internal function"s circuit helper
            verifier_contract_filename = internal_circuit.verifier_contract_filename.take();
            verifier_contract_type = internal_circuit.verifier_contract_type.take();
            _global_keys = internal_circuit._global_keys.take();

            trans_priv_size = internal_circuit.priv_in_size_trans;
            trans_in_size = internal_circuit.in_size_trans;
            trans_out_size = internal_circuit.out_size_trans;

            _needed_secret_key = internal_circuit._needed_secret_key;

            if internal_circuit.fct.requires_verification.is_some() {
                transitively_called_functions =
                    internal_circuit.transitively_called_functions.clone();
                transitively_called_functions.insert(internal_circuit.fct.clone());
            } else {
                assert!(internal_circuit.transitively_called_functions.is_none());
                transitively_called_functions = Some(BTreeSet::new());
            }
        }

        let mut selfs = Self {
            fct,
            verifier_contract_filename,
            verifier_contract_type,
            has_return_var: false,
            _expr_trafo,
            _circ_trafo,
            _phi: vec![],
            _secret_input_name_factory: NameFactory::new("secret", HybridArgType::PrivCircuitVal),
            _circ_temp_name_factory: NameFactory::new("tmp", HybridArgType::TmpCircuitVal),
            _in_name_factory: NameFactory::new(
                CFG.lock().unwrap().zk_in_name,
                HybridArgType::PubCircuitArg,
            ),
            _out_name_factory: NameFactory::new(
                CFG.lock().unwrap().zk_out_name,
                HybridArgType::PubCircuitArg,
            ),
            static_owner_labels,
            _requested_dynamic_pks: BTreeMap::new(),
            _needed_secret_key,
            _global_keys,
            function_calls_with_verification: vec![],
            transitively_called_functions,
            trans_priv_size,
            trans_in_size,
            trans_out_size,
            _remapper: CircVarRemapper::new(),
        };
        selfs._expr_trafo = expr_trafo_constructor(&selfs);
        selfs._circ_trafo = circ_trafo_constructor(&selfs);
        selfs
    }
    pub fn register_verification_contract_metadata(
        &self,
        contract_type: TypeName,
        import_filename: &str,
    ) {
        self.verifier_contract_type = contract_type;
        self.verifier_contract_filename = import_filename;
    }

    //Properties #

    pub fn get_verification_contract_name(self) -> String {
        assert!(self.verifier_contract_type.is_some());
        self.verifier_contract_type.code()
    }

    pub fn requires_zk_data_struct(self) -> bool
// """
        // Return true if a struct needs to be created in the solidity code to store public data (IO) associated with this circuit.

        // A struct is used instead of plain temporary variables to bypass solidity"s stack limit.
        // """
    {
        self.out_size + self.in_size > 0
    }

    pub fn zk_data_struct_name(self) -> String
// """Name of the data struct type"""
    {
        format!("{}_{}", CFG.lock().unwrap().zk_struct_prefix, self.fct.name)
    }

    pub fn priv_in_size_trans(self) -> i32
// """Total size of all private inputs for this circuit (in //uints)"""
    {
        self.priv_in_size + self.trans_priv_size
    }

    pub fn priv_in_size(self) -> i32
// """Size of all private inputs required for self.fct only (without called functions, in #uints)"""
    {
        self._secret_input_name_factory.size
    }

    pub fn out_size_trans(self) -> i32
// """Total size of all public outputs for this circuit (in //uints)"""
    {
        self.out_size + self.trans_out_size
    }

    pub fn out_size(self) -> i32
// """Size of all public outputs required for self.fct only (without called functions, in #uints)"""
    {
        self._out_name_factory.size
    }

    pub fn in_size_trans(self) -> i32
// """Total size of all public inputs for this circuit (in //uints)"""
    {
        self.in_size + self.trans_in_size
    }

    pub fn in_size(self) -> i32
// """Size of all public inputs required for self.fct only (without called functions, in #uints)"""
    {
        self._in_name_factory.size
    }

    pub fn output_idfs(self) -> Vec<HybridArgumentIdf>
// """All public output HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    {
        self._out_name_factory.idfs.clone()
    }

    pub fn input_idfs(self) -> Vec<HybridArgumentIdf>
// """All public input HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    {
        self._in_name_factory.idfs.clone()
    }

    pub fn sec_idfs(self) -> Vec<HybridArgumentIdf>
// """All private input HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    {
        self._secret_input_name_factory.idfs.clone()
    }

    pub fn phi(self) -> Vec<CircuitStatement>
// """List of abstract circuit statements which defines circuit semantics"""
    {
        self._phi.clone()
    }

    pub fn requested_global_keys(
        self,
    ) -> BTreeSet<((Option<MeExpr>, Option<Identifier>), CryptoParams)>
// """Statically known keys required by this circuit"""
    {
        self._global_keys.clone()
    }

    pub fn public_arg_arrays(self) -> Vec<(String, i32)>
// """Returns names and lengths of all public parameter uint256 arrays which go into the verifier"""
    {
        [
            (self._in_name_factory.base_name, self.in_size_trans),
            (self._out_name_factory.base_name, self.out_size_trans),
        ]
    }

    pub fn circ_indent_block(&self, name: &str)
    // """
    // Return context manager which manages the lifetime of a CircIndentBlock.

    // All statements which are inserted into self.phi during the lifetime of this context manager are automatically wrapped inside
    // a CircIndentBlock statement with the supplied name.
    // """
    {
        let old_len = self.phi.len();
        // yield
        self.phi = self.phi[..old_len] + [CircIndentBlock::new(name, self.phi[old_len..])];
    }

    pub fn guarded(&self, guard_idf: HybridArgumentIdf, is_true: bool)
    // """Return a context manager which manages the lifetime of a guard variable."""
    {
        CircGuardModification::guarded(self.phi, guard_idf, is_true);
    }

    pub fn get_glob_key_name(label: PrivacyLabelExpr, crypto_params: CryptoParams) -> String
// """Return the name of the HybridArgumentIdf which holds the statically known public key for the given privacy label."""
    {
        assert!(is_instances(
            &label,
            vec![ASTType::MeExpr, ASTType::Identifier]
        ));
        format!("glob_key_{}__{}", crypto_params.identifier_name, label.name)
    }

    pub fn get_own_secret_key_name(crypto_params: CryptoParams) -> String {
        format!("glob_sk_{}__me", crypto_params.identifier_name)
    }

    pub fn requires_verification(self) -> bool
// """ Returns true if the function corresponding to this circuit requires a zk proof verification for correctness """
    {
        let req = self.in_size_trans > 0 || self.out_size_trans > 0 || self.priv_in_size_trans > 0;
        assert!(req == self.fct.requires_verification);
        req
    }

    //Solidity-side interface #

    pub fn ensure_parameter_encryption(&self, insert_loc_stmt: Statement, param: Parameter)
    // """
    // Make circuit prove that the encryption of the specified parameter is correct.
    // """
    {
        assert!(param.annotated_type.is_cipher());

        let plain_idf = self
            ._secret_input_name_factory
            .add_idf(param.idf.name, param.annotated_type.zkay_type.type_name);
        let name = format!(
            "{}_{}",
            self._in_name_factory
                .get_new_name(param.annotated_type.type_name),
            param.idf.name
        );
        let cipher_idf = self
            ._in_name_factory
            .add_idf(name, param.annotated_type.type_name);
        self._ensure_encryption(
            insert_loc_stmt,
            plain_idf,
            Expression::me_expr(),
            param.annotated_type.type_name.crypto_params,
            cipher_idf,
            true,
            false,
        );
    }

    pub fn get_randomness_for_rerand(&self, expr: Expression) -> IdentifierExpr {
        let idf = self
            ._secret_input_name_factory
            .get_new_idf(TypeName::rnd_type(
                expr.annotated_type.type_name.crypto_params,
            ));
        IdentifierExpr::new(idf)
    }

    pub fn evaluate_expr_in_circuit(
        &self,
        expr: Expression,
        new_privacy: PrivacyLabelExpr,
        homomorphism: Homomorphism,
    ) -> LocationExpr
// """
        // Evaluate private expression and return result as a fresh out variable.
        // Roughly corresponds to out() from paper
        // Note: This function has side effects on expr.statement (adds a pre_statement)
        // :param expr: [SIDE EFFECT] The expression which should be evaluated privately
        // :param new_privacy: The circuit output should be encrypted for this owner (or plain if "all")
        // :return: Location expression which references the encrypted circuit result
        // """
    {
        self.circ_indent_block(expr.code());
        self._get_circuit_output_for_private_expression(expr, new_privacy, homomorphism)
    }

    pub fn evaluate_stmt_in_circuit(&self, ast: Statement) -> AssignmentStatement
// """
        // Evaluate an entire statement privately.

        // This works by turning the statement into an assignment statement where the

        // * lhs is a tuple of all external locations (defined outside statement), which are modified inside the statement
        // * rhs is the return value of an inlined function call expression to a virtual function where body = the statement + return statement \
        //   which returns a tuple of the most recent SSA version of all modified locations

        // Note: Modifying external locations which are not owned by @me inside the statement is illegal (would leak information).
        // Note: At the moment, this is only used for if statements with a private condition.

        // :param ast: the statement to evaluate inside the circuit
        // :return: AssignmentStatement as described above
        // """
    {
        let mut astmt = ExpressionStatement::new(NumberLiteralExpr::new(0));
        for var in ast.modified_values {
            if var.in_scope_at(ast) {
                astmt = AssignmentStatement::new(None, None);
                break;
            }
        }

        astmt.before_analysis = ast.before_analysis.clone();

        //External values written inside statement -> function return values
        let mut ret_params = vec![];
        for var in ast.modified_values {
            if var.in_scope_at(ast) {
                //side effect affects location outside statement and has privacy @me
                assert!(ast
                    .before_analysis
                    .same_partition(var.privacy, Expression::me_expr()));
                assert!(is_instances(
                    &var.target,
                    vec![
                        ASTType::Parameter,
                        ASTType::VariableDeclaration,
                        ASTType::StateVariableDeclaration
                    ]
                ));
                let t = var.target.annotated_type.zkay_type;
                if !t.type_name.is_primitive_type() {
                    unimplemented!(
                        "Reference types inside private if statements are not supported"
                    );
                }
                let ret_t =
                    AnnotatedTypeName::New(t.type_name, Expression::me_expr(), t.homomorphism); //t, but @me
                let mut idf = IdentifierExpr::new(var.target.idf.clone(), ret_t);
                idf.target = var.target.clone();
                let mut ret_param = idf;
                ret_param.statement = astmt;
                ret_params.push(ret_param);
            }
        }

        //Build the imaginary function
        let mut fdef = ConstructorOrFunctionDefinition::new(
            Identifier::Identifier(IdentifierBase::new(String::from("<stmt_fct>"))),
            vec![],
            crate::lc_vec_s!["private"],
            ret_params
                .iter()
                .map(|ret| Parameter::new(vec![], ret.annotated_type, ret.target.idf))
                .collect(),
            Block::new(vec![
                ast,
                ReturnStatement::new(TupleExpr::new(ret_params)).get_ast(),
            ]),
        );
        fdef.original_body = fdef.body.clone();
        fdef.body.parent = fdef.clone();
        fdef.parent = ast.clone();

        //inline "Call" to the imaginary function
        let mut idf = IdentifierExpr::new("<stmt_fct>");
        idf.target = fdef;
        let mut fcall = FunctionCallExpr::new(idf, vec![]);
        fcall.statement = astmt;
        let mut ret_args = self.inline_function_call_into_circuit(fcall);

        //Move all return values out of the circuit
        if !is_instance(&ret_args, ASTType::TupleExpr) {
            ret_args = TupleExpr::new(vec![ret_args]);
        }
        for ret_arg in ret_args.elements {
            ret_arg.statement = astmt;
        }
        let ret_arg_outs: Vec<_> = ret_params
            .iter()
            .zip(&ret_args.elements)
            .map(|(ret_param, ret_arg)| {
                self._get_circuit_output_for_private_expression(
                    ret_arg,
                    Expression::me_expr(),
                    ret_param.annotated_type.homomorphism,
                )
            })
            .collect();

        //Create assignment statement
        if ret_params {
            astmt.lhs = TupleExpr::new(ret_params.iter().cloned().collect());
            astmt.rhs = TupleExpr::new(ret_arg_outs);
            astmt
        } else {
            assert!(is_instance(&astmt, ASTType::ExpressionStatement));
            astmt
        }
    }
    pub fn invalidate_idf(&self, target_idf: Identifier) {
        if self._remapper.is_remapped(target_idf) {
            self._remapper.reset_key(target_idf);
        }
    }

    pub fn call_function(&self, ast: FunctionCallExpr)
    // """
    // Include public function call to a function which requires verification in this circuit.

    // :param ast: The function call to include, target function must require verification
    // """
    {
        assert!(ast.func.target.requires_verification);
        self.function_calls_with_verification.push(ast);
        self.phi.push(CircCall::new(ast.func.target));
    }

    pub fn request_public_key(
        &self,
        crypto_params: CryptoParams,
        plabel: (Option<MeExpr>, Option<Identifier>),
        name: &str,
    )
    // """
    // Request key for the address corresponding to plabel from pki infrastructure and add it to the public circuit inputs.

    // :param plabel: privacy label for which to request key
    // :param name: name to use for the HybridArgumentIdf holding the key
    // :return: HybridArgumentIdf containing the requested key and an AssignmentStatement which assigns the key request to the idf location
    // """
    {
        let idf = self
            ._in_name_factory
            .add_idf(name, TypeName::key_type(crypto_params));
        let pki = IdentifierExpr::new(
            CFG.lock()
                .unwrap()
                .get_contract_var_name(CFG.lock().unwrap().get_pki_contract_name(crypto_params)),
        );
        let privacy_label_expr = get_privacy_expr_from_label(plabel);
        (
            idf,
            idf.get_loc_expr()
                .assign(pki.call("getPk", [self._expr_trafo.visit(privacy_label_expr)])),
        )
    }

    pub fn request_private_key(&self, crypto_params: CryptoParams) -> Vec<Statement> {
        assert!(
            self._needed_secret_key.contains(crypto_params)
                || self
                    .fct
                    .parameters
                    .iter()
                    .filter_map(|p| if p.annotated_type.is_cipher() {
                        p.annotated_type.type_name.crypto_params.clone()
                    } else {
                        None
                    })
                    .collect::<Vec<_>>()
                    .contains(crypto_params)
        );
        let key_name = self.get_own_secret_key_name(crypto_params);
        self._secret_input_name_factory
            .add_idf(key_name, TypeName::key_type(crypto_params));
        return vec![EnterPrivateKeyStatement::new(crypto_params)];
    }

    //Circuit-side interface #
    pub fn add_to_circuit_inputs(&self, expr: Expression) -> HybridArgumentIdf
// """
        // Add the provided expression to the public circuit inputs.

        // Roughly corresponds to in() from paper

        // If expr is encrypted (privacy != @all), this function also automatically ensures that the circuit has access to
        // the correctly decrypted expression value in the form of a new private circuit input.

        // If expr is an IdentifierExpr, its value will be cached
        // (i.e. when the same identifier is needed again as a circuit input, its value will be retrieved from cache rather \
        //  than adding an expensive redundant input. The cache is invalidated as soon as the identifier is overwritten in public code)

        // Note: This function has side effects on expr.statement (adds a pre_statement)

        // :param expr: [SIDE EFFECT] expression which should be made available inside the circuit as an argument
        // :return: HybridArgumentIdf which references the plaintext value of the newly added input
        // """
    {
        let privacy = if expr.annotated_type.is_private() {
            expr.annotated_type
                .privacy_annotation
                .privacy_annotation_label()
        } else {
            Expression::all_expr()
        };
        let is_public = privacy == Expression::all_expr();

        let expr_text = expr.code();
        let input_expr = self._expr_trafo.visit(expr);
        let t = input_expr.annotated_type.type_name;
        let mut locally_decrypted_idf = None;

        //If expression has literal type -> evaluate it inside the circuit (constant folding will be used)
        //rather than introducing an unnecessary public circuit input (expensive)
        if is_instance(&t, ASTType::BooleanLiteralType) {
            return self._evaluate_private_expression(input_expr, t.value.to_string());
        } else if is_instance(&t, ASTType::NumberLiteralType) {
            return self._evaluate_private_expression(input_expr, t.value.to_string());
        }

        let mut t_suffix = String::new();
        if is_instance(&expr, ASTType::IdentifierExpr)
        //Look in cache before doing expensive move-in
        {
            if self._remapper.is_remapped(expr.target.idf) {
                return self._remapper.get_current(expr.target.idf);
            }

            t_suffix = format!("_{}", expr.idf.name);
        }

        //Generate circuit inputs
        let (return_idf, input_idf) = if is_public {
            let tname = format!(
                "{}{t_suffix}",
                self._in_name_factory
                    .get_new_name(expr.annotated_type.type_name)
            );
            let input_idf = self
                ._in_name_factory
                .add_idf(tname, expr.annotated_type.type_name);
            let return_idf = input_idf.clone();
            self._phi.push(CircComment::new(format!(
                "{} = {expr_text}",
                input_idf.name
            )));
            (return_idf, input_idf)
        } else
        //Encrypted inputs need to be decrypted inside the circuit (i.e. add plain as private input and prove encryption)
        {
            let tname = format!(
                "{}{t_suffix}",
                self._secret_input_name_factory
                    .get_new_name(expr.annotated_type.type_name)
            );
            let locally_decrypted_idf = self
                ._secret_input_name_factory
                .add_idf(tname, expr.annotated_type.type_name);
            let return_idf = locally_decrypted_idf.clone();
            let cipher_t =
                TypeName::cipher_type(input_expr.annotated_type, expr.annotated_type.homomorphism);
            let tname = format!("{}{t_suffix}", self._in_name_factory.get_new_name(cipher_t));
            let input_idf = self._in_name_factory.add_idf(
                tname,
                cipher_t,
                IdentifierExpr::new(locally_decrypted_idf),
            );
            (return_idf, input_idf)
        };

        //Add a CircuitInputStatement to the solidity code, which looks like a normal assignment statement,
        //but also signals the offchain simulator to perform decryption if necessary
        expr.statement
            .pre_statements
            .push(CircuitInputStatement::new(
                input_idf.get_loc_expr(),
                input_expr,
            ));

        if !is_public {
            //Check if the secret plain input corresponds to the decrypted cipher value
            let crypto_params = CFG
                .lock()
                .unwrap()
                .get_crypto_params(expr.annotated_type.homomorphism);
            self._phi.push(CircComment::new(format!(
                "{locally_decrypted_idf} = dec({expr_text}) [{}]",
                input_idf.name
            )));
            self._ensure_encryption(
                expr.statement,
                locally_decrypted_idf,
                Expression::me_expr(),
                crypto_params,
                input_idf,
                false,
                true,
            );
        }

        //Cache circuit input for later reuse if possible
        if CFG.lock().unwrap().opt_cache_circuit_inputs
            && is_instance(&expr, ASTType::IdentifierExpr)
        //TODO: What if a homomorphic variable gets used as both a plain variable and as a ciphertext?
        //      This works for now because we never perform homomorphic operations on variables we can decrypt.
        {
            self._remapper.remap(expr.target.idf, return_idf);
        }

        return_idf
    }
    pub fn get_remapped_idf_expr(&self, idf: IdentifierExpr) -> LocationExpr
// """
        // Get location expression for the most recently assigned value of idf according to the SSA simulation.

        // :param idf: Identifier expression to lookup
        // :return: Either idf itself (not currently remapped)
        //          or a loc expr for the HybridArgumentIdf which references the most recent value of idf
        // """
    {
        assert!(idf.target.is_some());
        assert!(!is_instance(&idf.idf, ASTType::HybridArgumentIdf));
        if self._remapper.is_remapped(idf.target.idf) {
            let remapped_idf = self._remapper.get_current(idf.target.idf);
            remapped_idf
                .get_idf_expr(idf.parent)
                .as_type(idf.annotated_type)
        } else {
            idf
        }
    }
    pub fn create_new_idf_version_from_value(&self, orig_idf: Identifier, expr: Expression)
    // """
    // Store expr in a new version of orig_idf (for SSA).

    // :param orig_idf: the identifier which should be updated with a new value
    // :param expr: the updated value
    // :param is_local: whether orig_idf refers to a local variable (as opposed to a state variable)
    // """
    {
        let tmp_var = self._create_temp_var(orig_idf.name, expr);
        self._remapper.remap(orig_idf, tmp_var);
    }

    pub fn inline_function_call_into_circuit(
        &self,
        fcall: &FunctionCallExpr,
    ) -> (Option<Expression>, Option<TupleExpr>)
// """
        // Inline an entire function call into the current circuit.

        // :param fcall: Function call to inline
        // :return: Expression (1 retval) / TupleExpr (multiple retvals) with return value(s)
        // """
    {
        assert!(is_instance(&fcall.func, ASTType::LocationExpr) && fcall.func.target.is_some());
        let fdef = fcall.func.target.clone();
        //with
        self._remapper.remap_scope(fcall.func.target.body);

        //with
        if fcall.func.target.idf.name == "<stmt_fct>" {
            {}
        } else {
            self.circ_indent_block(format!("INLINED {}", fcall.code()));
        }

        //Assign all arguments to temporary circuit variables which are designated as the current version of the parameter idfs
        for (param, arg) in fdef.parameters.iter().zip(&fcall.args) {
            self.phi.push(CircComment::new(format!(
                "ARG {}: {}",
                param.idf.name,
                arg.code()
            )));
            // with
            self.circ_indent_block();
            {
                self.create_new_idf_version_from_value(param.idf, arg);
            }
        }

        //Visit the untransformed target function body to include all statements in this circuit
        let inlined_body = deep_copy(fdef.original_body, true, true);
        self._circ_trafo.visit(inlined_body);
        fcall.statement.pre_statements += inlined_body.pre_statements;

        //Create TupleExpr with location expressions corresponding to the function return values as elements
        let ret_idfs = fdef
            .return_var_decls
            .iter()
            .map(|vd| self._remapper.get_current(vd.idf))
            .collect();
        let mut ret = TupleExpr::new(
            ret_idfs
                .iter()
                .map(|idf| IdentifierExpr::new(idf.clone()).as_type(idf.t))
                .collect(),
        );

        if ret.elements.len() == 1
        //Unpack 1-length tuple
        {
            ret = ret.elements[0];
        }
        ret
    }
    pub fn add_assignment_to_circuit(&self, ast: AssignmentStatement)
    // """Include private assignment statement in this circuit."""
    {
        self.phi.push(CircComment::new(ast.code()));
        self._add_assign(ast.lhs, ast.rhs);
    }

    pub fn add_var_decl_to_circuit(&self, ast: VariableDeclarationStatement) {
        self.phi.push(CircComment::new(ast.code()));
        if ast.expr.is_none()
        //Default initialization is made explicit for circuit variables
        {
            let t = ast.variable_declaration.annotated_type.type_name.clone();
            assert!(t.can_be_private());
            let mut nle = NumberLiteralExpr::new(0);
            nle.parent = ast;
            nle.statement = ast;
            ast.expr = TypeCheckVisitor::implicitly_converted_to(nle, t);
        }
        self.create_new_idf_version_from_value(ast.variable_declaration.idf, ast.expr);
    }

    pub fn add_return_stmt_to_circuit(&self, ast: ReturnStatement) {
        self.phi.push(CircComment::new(ast.code()));
        assert!(ast.expr.is_some());
        if !is_instance(&ast.expr, ASTType::TupleExpr) {
            ast.expr = TupleExpr::new(vec![ast.expr.clone()]);
        }

        for (vd, expr) in ast.function.return_var_decls.iter().zip(&ast.expr.elements) {
            //Assign return value to new version of return variable
            self.create_new_idf_version_from_value(vd.idf, expr);
        }
    }

    pub fn add_if_statement_to_circuit(&self, ast: IfStatement)
    // """Include private if statement in this circuit."""
    {
        //Handle if branch
        // with
        self._remapper.remap_scope();
        let mut comment = CircComment::new(format!("if ({})", ast.condition.code()));
        self._phi.push(comment.clone());
        let cond = self._evaluate_private_expression(ast.condition);
        comment.text += format!(" [{}]", cond.name);
        self._circ_trafo.visitBlock(ast.then_branch, cond, true);
        let then_remap = self._remapper.get_state();

        //Bubble up nested pre statements
        ast.pre_statements.extend(ast.then_branch.pre_statements);
        ast.then_branch.pre_statements = vec![];

        //Handle else branch
        if ast.else_branch.is_some() {
            self._phi
                .push(CircComment::new(format!("else [{}]", cond.name)));
            self._circ_trafo.visitBlock(ast.else_branch, cond, false);

            //Bubble up nested pre statements
            ast.pre_statements += ast.else_branch.pre_statements;
            ast.else_branch.pre_statements = [];
        }

        //SSA join branches (if both branches write to same external value -> cond assignment to select correct version)
        // with
        self.circ_indent_block(format!("JOIN [{}]", cond.name));
        let cond_idf_expr = cond.get_idf_expr(ast);
        assert!(is_instance(&cond_idf_expr, ASTType::IdentifierExpr));
        self._remapper
            .join_branch(ast, cond_idf_expr, then_remap, self._create_temp_var);
    }
    pub fn add_block_to_circuit(
        &self,
        ast: Block,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) {
        assert!(ast.parent.is_some());
        let is_already_scoped = is_instances(
            &ast.parent,
            vec![
                ASTType::ConstructorOrFunctionDefinition,
                ASTType::IfStatement,
            ],
        );
        self.phi.push(CircComment::new("{"));
        // with
        self.circ_indent_block();
        // with
        if let Some(guard_cond) = guard_cond {
            self.guarded(guard_cond, guard_val);
        }
        //with
        if !is_already_scoped {
            self._remapper.remap_scope(ast);
        }
        for stmt in ast.statements {
            self._circ_trafo.visit(stmt);
            //Bubble up nested pre statements
            ast.pre_statements += stmt.pre_statements;
            stmt.pre_statements = vec![];
        }

        self.phi.push(CircComment::new("}"));
    }

    //Internal functionality #

    pub fn _get_canonical_privacy_label<P: Ord>(
        &self,
        analysis: &PartitionState<P>,
        privacy: &PrivacyLabelExpr,
    )
    // """
    // If privacy is equivalent to a static privacy label -> Return the corresponding static label, otherwise itself.

    // :param analysis: analysis state at the statement where expression with the given privacy occurs
    // :param privacy: original privacy label
    // """
    {
        for owner in self._static_owner_labels {
            if analysis.same_partition(owner, privacy) {
                return owner;
            }
        }
        privacy
    }

    pub fn _create_temp_var(&self, tag: &str, expr: Expression) -> HybridArgumentIdf
// """Assign expression to a fresh temporary circuit variable."""
    {
        self._evaluate_private_expression(expr, format!("_{tag}"))
    }

    pub fn _add_assign(&self, lhs: Expression, rhs: Expression)
    // """
    // Simulate an assignment of rhs to lhs inside the circuit.

    // :param lhs: destination
    // :param rhs: source
    // """
    {
        if is_instance(&lhs, ASTType::IdentifierExpr)
        //for now no ref types
        {
            assert!(lhs.target.is_some());
            self.create_new_idf_version_from_value(lhs.target.idf, rhs);
        } else if is_instance(&lhs, ASTType::IndexExpr) {
            // raise NotImplementedError()
            unimplemented!();
        } else {
            assert!(is_instance(&lhs, ASTType::TupleExpr));
            if is_instance(&rhs, ASTType::FunctionCallExpr) {
                rhs = self._circ_trafo.visit(rhs);
            }
            assert!(
                is_instance(&rhs, ASTType::TupleExpr) && lhs.elements.len() == rhs.elements.len()
            );
            for (e_l, e_r) in lhs.elements.iter().zip(&rhs.elements) {
                self._add_assign(e_l, e_r);
            }
        }
    }

    pub fn _get_circuit_output_for_private_expression(
        &self,
        expr: Expression,
        new_privacy: PrivacyLabelExpr,
        homomorphism: Homomorphism,
    ) -> LocationExpr
// """
        // Add evaluation of expr to the circuit and return the output HybridArgumentIdf corresponding to the evaluation result.

        // Note: has side effects on expr.statement (adds pre_statement)

        // :param expr: [SIDE EFFECT] expression to evaluate
        // :param new_privacy: result owner (determines encryption key)
        // :return: HybridArgumentIdf which references the circuit output containing the result of expr
        // """
    {
        let is_circ_val = is_instance(&expr, ASTType::IdentifierExpr)
            && is_instance(&expr.idf, ASTType::HybridArgumentIdf)
            && expr.idf.arg_type != HybridArgType::PUB_CONTRACT_VAL;
        let is_hom_comp = is_instance(&expr, ASTType::FunctionCallExpr)
            && is_instance(&expr.func, ASTType::BuiltinFunction)
            && expr.func.homomorphism != Homomorphism::non_homomorphic();
        if is_hom_comp
        //Treat a homomorphic operation as a privately evaluated operation on (public) ciphertexts
        {
            expr.annotated_type = AnnotatedTypeName::cipher_type(expr.annotated_type, homomorphism);
        }

        let priv_result_idf =
            if is_circ_val || expr.annotated_type.is_private() || expr.evaluate_privately {
                self._evaluate_private_expression(expr);
            } else
            //For public expressions which should not be evaluated in private, only the result is moved into the circuit
            {
                self.add_to_circuit_inputs(expr)
            };
        let private_expr = priv_result_idf.get_idf_expr();

        let t_suffix = String::new();
        if is_instance(&expr, ASTType::IdentifierExpr) && !is_circ_val {
            t_suffix += &format!("_{}", expr.idf.name);
        }

        let (out_var, new_out_param) = if is_instance(&new_privacy, ASTType::AllExpr)
            || expr.annotated_type.type_name.is_cipher()
        //If the result is public, add an equality constraint to ensure that the user supplied public output
        //is equal to the circuit evaluation result
        {
            let tname = format!(
                "{}{t_suffix}",
                self._out_name_factory
                    .get_new_name(expr.annotated_type.type_name)
            );
            let new_out_param =
                self._out_name_factory
                    .add_idf(tname, expr.annotated_type.type_name, private_expr);
            self._phi
                .push(CircEqConstraint::new(priv_result_idf, new_out_param));
            (
                new_out_param
                    .get_loc_expr()
                    .explicitly_converted(expr.annotated_type.type_name),
                new_out_param,
            )
        } else
        //If the result is encrypted, add an encryption constraint to ensure that the user supplied encrypted output
        //is equal to the correctly encrypted circuit evaluation result
        {
            let new_privacy = self._get_canonical_privacy_label(expr.analysis, new_privacy);
            let privacy_label_expr = get_privacy_expr_from_label(new_privacy);
            let cipher_t = TypeName::cipher_type(expr.annotated_type, homomorphism);
            let tname = format!(
                "{}{t_suffix}",
                self._out_name_factory.get_new_name(cipher_t)
            );
            let enc_expr =
                EncryptionExpression::new(private_expr, privacy_label_expr, homomorphism);
            let new_out_param = self._out_name_factory.add_idf(tname, cipher_t, enc_expr);
            let crypto_params = CFG.lock().unwrap().get_crypto_params(homomorphism);
            self._ensure_encryption(
                expr.statement,
                priv_result_idf,
                new_privacy,
                crypto_params,
                new_out_param,
                false,
                false,
            );
            (new_out_param.get_loc_expr(), new_out_param)
        };

        //Add an invisible CircuitComputationStatement to the solidity code, which signals the offchain simulator,
        //that the value the contained out variable must be computed at this point by simulating expression evaluation
        expr.statement
            .pre_statements
            .push(CircuitComputationStatement::new(new_out_param));
        out_var
    }
    pub fn _evaluate_private_expression(
        &self,
        expr: Expression,
        tmp_idf_suffix: &str,
    ) -> HybridArgumentIdf
// """
        // Evaluate expr in the circuit (if not already done) and store result in a new temporary circuit variable.

        // :param expr: expression to evaluate
        // :param tmp_idf_suffix: name suffix for the new temporary circuit variable
        // :return: temporary circuit variable HybridArgumentIdf which refers to the transformed circuit expression
        // """
    {
        assert!(
            !(is_instance(&expr, ASTType::MemberAccessExpr)
                && is_instance(&expr.member, ASTType::HybridArgumentIdf))
        );
        if is_instance(&expr, ASTType::IdentifierExpr)
            && is_instance(&expr.idf, ASTType::HybridArgumentIdf)
            && expr.idf.arg_type != HybridArgType::PUB_CONTRACT_VAL
        //Already evaluated in circuit
        {
            return expr.idf.clone();
        }

        let priv_expr = self._circ_trafo.visit(expr);
        let tname = format!(
            "{}{tmp_idf_suffix}",
            self._circ_temp_name_factory
                .get_new_name(priv_expr.annotated_type.type_name)
        );
        let tmp_circ_var_idf = self._circ_temp_name_factory.add_idf(
            tname,
            priv_expr.annotated_type.type_name,
            priv_expr,
        );
        let stmt = CircVarDecl::new(tmp_circ_var_idf.clone(), priv_expr);
        self.phi.push(stmt);
        tmp_circ_var_idf
    }

    pub fn _ensure_encryption(
        &self,
        stmt: Statement,
        plain: HybridArgumentIdf,
        new_privacy: PrivacyLabelExpr,
        crypto_params: CryptoParams,
        cipher: HybridArgumentIdf,
        is_param: bool,
        is_dec: bool,
    )
    // """
    // Make sure that cipher = enc(plain, getPk(new_privacy), priv_user_provided_rnd).

    // This automatically requests necessary keys and adds a circuit input for the randomness.

    // Note: This function adds pre-statements to stmt

    // :param stmt [SIDE EFFECT]: the statement which contains the expression which requires this encryption
    // :param plain: circuit variable referencing the plaintext value
    // :param new_privacy: privacy label corresponding to the destination key address
    // :param cipher: circuit variable referencing the encrypted value
    // :param is_param: whether cipher is a function parameter
    // :param is_dec: whether this is a decryption operation (user supplied plain) as opposed to an encryption operation (user supplied cipher)
    // """
    {
        if crypto_params.is_symmetric_cipher()
        //Need a different set of keys for hybrid-encryption (ecdh-based) backends
        {
            self._require_secret_key(crypto_params);
            let my_pk =
                self._require_public_key_for_label_at(stmt, Expression::me_expr(), crypto_params);
            let other_pk = if is_dec {
                self._get_public_key_in_sender_field(stmt, cipher, crypto_params)
            } else {
                if new_privacy == Expression::me_expr() {
                    my_pk
                } else {
                    self._require_public_key_for_label_at(stmt, new_privacy, crypto_params)
                }
            };

            self.phi.push(CircComment::new(format!(
                "{} = enc({}, ecdh({}, my_sk))",
                cipher.name, plain.name, other_pk.name
            )));
            self._phi
                .push(CircSymmEncConstraint::new(plain, other_pk, cipher, is_dec));
        } else {
            let rnd = self._secret_input_name_factory.add_idf(
                format!("{}_R", if is_param { plain.name } else { cipher.name }),
                TypeName::rnd_type(crypto_params),
            );
            let pk = self._require_public_key_for_label_at(stmt, new_privacy, crypto_params);
            if !is_dec {
                self.phi.push(CircComment::new(format!(
                    "{} = enc({}, {})",
                    cipher.name, plain.name, pk.name
                )));
            }
            self._phi
                .push(CircEncConstraint::new(plain, rnd, pk, cipher, is_dec));
        }
    }

    pub fn _require_secret_key(&self, crypto_params: CryptoParams) -> HybridArgumentIdf {
        self._needed_secret_key[crypto_params] = None; //Add to _need_secret_key OrderedDict
        let key_name = self.get_own_secret_key_name(crypto_params);
        HybridArgumentIdf::new(
            key_name,
            TypeName::key_type(crypto_params),
            HybridArgType::PrivCircuitVal,
        )
    }

    pub fn _require_public_key_for_label_at(
        &self,
        stmt: Option<Statement>,
        privacy: PrivacyLabelExpr,
        crypto_params: CryptoParams,
    ) -> HybridArgumentIdf
// """
        // Make circuit helper aware, that the key corresponding to privacy is required at stmt.

        // If privacy is not a statically known label, the key is requested on spot.
        // Otherwise the label is added to the global key set.
        // The keys in that set are requested only once at the start of the external wrapper function, to improve efficiency.

        // Note: This function has side effects on stmt (adds a pre_statement)

        // :return: HybridArgumentIdf which references the key
        // """
    {
        if self._static_owner_labels.contains(privacy)
        //Statically known privacy -> keep track (all global keys will be requested only once)
        {
            self._global_keys[(privacy, crypto_params)] = None;
            HybridArgumentIdf::new(
                self.get_glob_key_name(privacy, crypto_params),
                TypeName::key_type(crypto_params),
                HybridArgType::PubCircuitArg,
            )
        }
        if stmt.is_some() {
            assert!(
                false,
                "stmt cannot be None if privacy is not guaranteed to be statically known"
            )
        }

        //privacy cannot be MeExpr (is in _static_owner_labels) or AllExpr (has no public key)
        assert!(is_instance(&privacy, ASTType::Identifier));

        if !self._requested_dynamic_pks.contains(stmt) {
            self._requested_dynamic_pks[stmt] = {};
        }
        let requested_dynamic_pks = self._requested_dynamic_pks[stmt];
        if requested_dynamic_pks.contains(privacy) {
            return requested_dynamic_pks[privacy];
        }

        //Dynamic privacy -> always request key on spot and add to local in args
        let name = format!(
            "{}_{}",
            self._in_name_factory
                .get_new_name(TypeName::key_type(crypto_params)),
            privacy.name
        );
        let (idf, get_key_stmt) = self.request_public_key(crypto_params, privacy, name);
        stmt.pre_statements.push(get_key_stmt);
        requested_dynamic_pks.insert(privacy, idf);
        idf
    }

    pub fn _get_public_key_in_sender_field(
        &self,
        stmt: Statement,
        cipher: HybridArgumentIdf,
        crypto_params: CryptoParams,
    ) -> HybridArgumentIdf
// """
        // Ensure the circuit has access to the public key stored in cipher"s sender field.

        // Note: This function has side effects on stmt [adds a pre-statement]

        // :param stmt [SIDE EFFECT]: statement in which this private expression occurs
        // :param cipher: HybridArgumentIdf which references the cipher value
        // :return: HybridArgumentIdf which references the key in cipher"s sender field (or 0 if none)
        // """
    {
        let key_t = TypeName::key_type(crypto_params);
        let name = format!("{}_sender", self._in_name_factory.get_new_name(key_t));
        let key_idf = self._in_name_factory.add_idf(name, key_t);
        let cipher_payload_len = crypto_params.cipher_payload_len;
        let key_expr = KeyLiteralExpr::new(
            vec![cipher.get_loc_expr(stmt).index(cipher_payload_len)],
            crypto_params,
        )
        .as_type(key_t);
        stmt.pre_statements
            .push(AssignmentStatement::new(key_idf.get_loc_expr(), key_expr));
        key_idf
    }
}
