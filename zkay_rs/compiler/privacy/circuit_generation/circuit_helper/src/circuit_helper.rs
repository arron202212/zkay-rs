#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::name_factory::NameFactory;
use crate::name_remapper::CircVarRemapper;
use std::collections::{BTreeMap, BTreeSet};
use type_check::type_checker::TypeCheckVisitor;
use zkay_ast::analysis::partition_state::PartitionState;
use zkay_ast::ast::{
    get_privacy_expr_from_label, is_instance, is_instances, ASTType, AllExpr, AnnotatedTypeName,
    AssignmentStatement, AssignmentStatementBase, AssignmentStatementBaseMutRef,
    AssignmentStatementBaseProperty, Block, BooleanLiteralType, BuiltinFunction,
    CircuitComputationStatement, CircuitInputStatement, ConstructorOrFunctionDefinition,
    ElementaryTypeName, EncryptionExpression, EnterPrivateKeyStatement, ExprUnion, Expression,
    ExpressionBaseMutRef, ExpressionBaseProperty, ExpressionStatement, FunctionCallExpr,
    FunctionCallExprBase, FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty,
    FunctionCallExprBaseRef, HybridArgType, HybridArgumentIdf, Identifier, IdentifierBase,
    IdentifierBaseProperty, IdentifierDeclarationBaseProperty, IdentifierExpr, IdentifierExprUnion,
    IfStatement, IndexExpr, IntoAST, IntoExpression, IntoStatement, KeyLiteralExpr, LocationExpr,
    LocationExprBaseProperty, MeExpr, MemberAccessExpr, NamespaceDefinitionBaseProperty,
    NumberLiteralExpr, NumberLiteralType, NumberTypeName, Parameter, ReturnStatement,
    SimpleStatement, StateVariableDeclaration, Statement, StatementBaseMutRef,
    StatementBaseProperty, StatementBaseRef, TupleExpr, TupleOrLocationExpr, TypeName,
    UserDefinedTypeName, VariableDeclaration, VariableDeclarationStatement, AST,
};
use zkay_ast::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use zkay_ast::homomorphism::Homomorphism;
use zkay_ast::visitor::deep_copy::deep_copy;
use zkay_ast::visitor::transformer_visitor::{AstTransformerVisitor, TransformerVisitorEx};
use zkay_config::config::CFG;
use zkay_crypto::params::CryptoParams;
// class CircuitHelper

// """
// This class is used to construct abstract proof circuits during contract transformation.

// Typically there is one instance of this class for every function which requires verification.
// """
#[derive(Clone)]
pub struct CircuitHelper
where
    Self: Sized,
{
    // Function and verification contract corresponding to this circuit
    pub fct: ConstructorOrFunctionDefinition,
    pub verifier_contract_filename: Option<String>,
    pub verifier_contract_type: Option<UserDefinedTypeName>,
    // Metadata set later by ZkayContractTransformer
    pub has_return_var: bool,
    // Transformer visitors
    pub _expr_trafo: Option<Box<dyn TransformerVisitorEx>>, //AstTransformerVisitor
    pub _circ_trafo: Option<Box<dyn TransformerVisitorEx>>,
    // List of proof circuit statements (assertions and assignments)
    // WARNING: Never assign to let _phi, always access it using the phi property and only mutate it
    pub _phi: Vec<CircuitStatement>,
    // Name factory for private circuit inputs
    pub _secret_input_name_factory: NameFactory,
    // Name factory for temporary internal circuit variables
    pub _circ_temp_name_factory: NameFactory,
    // Name factory for public circuit inputs
    pub _in_name_factory: NameFactory,
    // Name factory for public circuit outputs
    pub _out_name_factory: NameFactory,
    //For a given owner label (idf or me), stores the corresponding assignment of the requested key to the corresponding in variable
    // List of all statically known privacy labels for the contract of which this circuit is part of
    pub static_owner_labels: Vec<AST>,
    // For each statement, cache the generated variable holding the requested public key of a given
    // not-statically-known identifier, to prevent requesting the same key over and over again
    pub _requested_dynamic_pks: BTreeMap<Statement, BTreeMap<Identifier, HybridArgumentIdf>>,
    // The crypto backends for which msg.sender"s secret key must be added to the private circuit inputs
    pub _needed_secret_key: BTreeSet<CryptoParams>,
    // Set of statically known privacy labels (OrderedDict is used to ensure deterministic iteration order)
    pub _global_keys: BTreeSet<(Option<AST>, CryptoParams)>,
    // List of all (non-transitive) calls in let fct"s body to functions which require verification, in AST visiting order
    // This is internally used to compute transitive in/out/privin sizes, but may also be useful when implementing a new
    // circuit generator backend.
    pub function_calls_with_verification: Vec<FunctionCallExpr>,
    // Set (with deterministic order) of all functions which this circuit transitively calls.
    pub transitively_called_functions: BTreeSet<ConstructorOrFunctionDefinition>,
    pub trans_priv_size: i32,
    pub trans_in_size: i32,
    pub trans_out_size: i32,
    // Remapper instance used for SSA simulation
    pub _remapper: CircVarRemapper,
}

impl CircuitHelper
where
    Self: Sized,
{
    pub fn new(
        fct: ConstructorOrFunctionDefinition,
        static_owner_labels: Vec<AST>,
        expr_trafo_constructor: impl FnOnce(&Self) -> Option<Box<dyn TransformerVisitorEx>>,
        circ_trafo_constructor: impl FnOnce(&Self) -> Option<Box<dyn TransformerVisitorEx>>,
        internal_circuit: Option<&mut Self>,
    ) -> Self
    where
        Self: Sized,
    {
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
        let mut verifier_contract_filename: Option<String> = None;
        let mut verifier_contract_type: Option<UserDefinedTypeName> = None;
        let _expr_trafo = None; //expr_trafo_constructor(&self);
        let _circ_trafo = None; //circ_trafo_constructor(&self);
        let mut _needed_secret_key = BTreeSet::new();
        let mut _global_keys = BTreeSet::new();
        let mut transitively_called_functions = BTreeSet::new();
        let (mut trans_priv_size, mut trans_in_size, mut trans_out_size) = (0, 0, 0); //Set later by transform_internal_calls
        if let Some(internal_circuit) = internal_circuit {
            //Inherit metadata from internal function"s circuit helper
            verifier_contract_filename = internal_circuit.verifier_contract_filename.clone();
            verifier_contract_type = internal_circuit.verifier_contract_type.clone();
            _global_keys = internal_circuit._global_keys.clone();

            trans_priv_size = internal_circuit.priv_in_size_trans();
            trans_in_size = internal_circuit.in_size_trans();
            trans_out_size = internal_circuit.out_size_trans();

            _needed_secret_key = internal_circuit._needed_secret_key.clone();

            if internal_circuit.fct.requires_verification {
                transitively_called_functions =
                    internal_circuit.transitively_called_functions.clone();
                transitively_called_functions.insert(internal_circuit.fct.clone());
            } else {
                assert!(internal_circuit.transitively_called_functions.is_empty());
                transitively_called_functions = BTreeSet::new();
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
            _secret_input_name_factory: NameFactory::new(
                String::from("secret"),
                HybridArgType::PrivCircuitVal,
            ),
            _circ_temp_name_factory: NameFactory::new(
                String::from("tmp"),
                HybridArgType::TmpCircuitVal,
            ),
            _in_name_factory: NameFactory::new(
                CFG.lock().unwrap().zk_in_name(),
                HybridArgType::PubCircuitArg,
            ),
            _out_name_factory: NameFactory::new(
                CFG.lock().unwrap().zk_out_name(),
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
        &mut self,
        contract_type: TypeName,
        import_filename: &str,
    ) {
        self.verifier_contract_type = if let TypeName::UserDefinedTypeName(v) = contract_type {
            Some(v)
        } else {
            None
        };
        self.verifier_contract_filename = Some(import_filename.to_string());
    }

    //Properties #

    pub fn get_verification_contract_name(&self) -> String {
        assert!(self.verifier_contract_type.is_some());
        self.verifier_contract_type
            .as_ref()
            .unwrap()
            .to_ast()
            .code()
    }

    pub fn requires_zk_data_struct(&self) -> bool
// """
        // Return true if a struct needs to be created in the solidity code to store public data (IO) associated with this circuit.

        // A struct is used instead of plain temporary variables to bypass solidity"s stack limit.
        // """
    {
        self.out_size() + self.in_size() > 0
    }

    pub fn zk_data_struct_name(&self) -> String
// """Name of the data struct type"""
    {
        format!(
            "{}_{}",
            CFG.lock().unwrap().zk_struct_prefix(),
            self.fct.name()
        )
    }

    pub fn priv_in_size_trans(&self) -> i32
// """Total size of all private inputs for this circuit (in //uints)"""
    {
        self.priv_in_size() + self.trans_priv_size
    }

    pub fn priv_in_size(&self) -> i32
// """Size of all private inputs required for self.fct only (without called functions, in #uints)"""
    {
        self._secret_input_name_factory.size
    }

    pub fn out_size_trans(&self) -> i32
// """Total size of all public outputs for this circuit (in //uints)"""
    {
        self.out_size() + self.trans_out_size
    }

    pub fn out_size(&self) -> i32
// """Size of all public outputs required for self.fct only (without called functions, in #uints)"""
    {
        self._out_name_factory.size
    }

    pub fn in_size_trans(&self) -> i32
// """Total size of all public inputs for this circuit (in //uints)"""
    {
        self.in_size() + self.trans_in_size
    }

    pub fn in_size(&self) -> i32
// """Size of all public inputs required for self.fct only (without called functions, in #uints)"""
    {
        self._in_name_factory.size
    }

    pub fn output_idfs(&self) -> Vec<HybridArgumentIdf>
// """All public output HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    {
        self._out_name_factory
            .idfs
            .iter()
            .filter_map(|v| {
                if let Identifier::HybridArgumentIdf(h) = v {
                    Some(h.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn input_idfs(&self) -> Vec<HybridArgumentIdf>
// """All public input HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    {
        self._in_name_factory
            .idfs
            .iter()
            .filter_map(|v| {
                if let Identifier::HybridArgumentIdf(h) = v {
                    Some(h.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn sec_idfs(&self) -> Vec<HybridArgumentIdf>
// """All private input HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    {
        self._secret_input_name_factory
            .idfs
            .iter()
            .filter_map(|v| {
                if let Identifier::HybridArgumentIdf(h) = v {
                    Some(h.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn phi(&self) -> Vec<CircuitStatement>
// """List of abstract circuit statements which defines circuit semantics"""
    {
        self._phi.clone()
    }

    pub fn requested_global_keys(&self) -> BTreeSet<(Option<AST>, CryptoParams)>
// """Statically known keys required by this circuit"""
    {
        self._global_keys.clone()
    }

    pub fn public_arg_arrays(&self) -> Vec<(String, i32)>
// """Returns names and lengths of all public parameter uint256 arrays which go into the verifier"""
    {
        vec![
            (
                self._in_name_factory.base_name_factory.base_name.clone(),
                self.in_size_trans(),
            ),
            (
                self._out_name_factory.base_name_factory.base_name.clone(),
                self.out_size_trans(),
            ),
        ]
    }

    pub fn circ_indent_block(&mut self, name: &str)
    // """
    // Return context manager which manages the lifetime of a CircIndentBlock.

    // All statements which are inserted into self.phi during the lifetime of this context manager are automatically wrapped inside
    // a CircIndentBlock statement with the supplied name.
    // """
    {
        let old_len = self.phi().len();
        // yield
        let mut phi = self.phi();
        let post = phi.split_off(old_len);
        phi.push(CircuitStatement::CircIndentBlock(CircIndentBlock::new(
            name.to_string(),
            post,
        )));
        self._phi = phi;
    }

    pub fn guarded(&self, guard_idf: HybridArgumentIdf, is_true: bool)
    // """Return a context manager which manages the lifetime of a guard variable."""
    {
        CircGuardModification::guarded(&mut self.phi(), guard_idf, is_true);
    }

    pub fn get_glob_key_name(label: &AST, crypto_params: &CryptoParams) -> String
// """Return the name of the HybridArgumentIdf which holds the statically known public key for the given privacy label."""
    {
        // assert!(is_instances(
        //     &label,
        //     vec![ASTType::MeExpr, ASTType::Identifier]
        // ));

        let name =
            if let Some(me_expr) = label.try_as_expression_ref().unwrap().try_as_me_expr_ref() {
                me_expr.name.clone()
            } else {
                label.try_as_identifier_ref().unwrap().name().clone()
            };
        format!("glob_key_{}__{}", crypto_params.identifier_name(), name)
    }

    pub fn get_own_secret_key_name(crypto_params: &CryptoParams) -> String {
        format!("glob_sk_{}__me", crypto_params.identifier_name())
    }

    pub fn requires_verification(&self) -> bool
// """ Returns true if the function corresponding to this circuit requires a zk proof verification for correctness """
    {
        let req =
            self.in_size_trans() > 0 || self.out_size_trans() > 0 || self.priv_in_size_trans() > 0;
        assert!(req == self.fct.requires_verification);
        req
    }

    //Solidity-side interface #

    pub fn ensure_parameter_encryption(
        &mut self,
        insert_loc_stmt: &mut Statement,
        param: &Parameter,
    )
    // """
    // Make circuit prove that the encryption of the specified parameter is correct.
    // """
    {
        assert!(param.identifier_declaration_base.annotated_type.is_cipher());

        let plain_idf = self._secret_input_name_factory.add_idf(
            param.identifier_declaration_base.idf.name().clone(),
            *param
                .identifier_declaration_base
                .annotated_type
                .zkay_type()
                .type_name,
            None,
        );
        let name = format!(
            "{}_{}",
            self._in_name_factory.base_name_factory.get_new_name(
                &param.identifier_declaration_base.annotated_type.type_name,
                false
            ),
            param.identifier_declaration_base.idf.name()
        );
        let cipher_idf = self._in_name_factory.add_idf(
            name,
            *param
                .identifier_declaration_base
                .annotated_type
                .type_name
                .clone(),
            None,
        );
        self._ensure_encryption(
            insert_loc_stmt,
            plain_idf,
            Expression::me_expr(None).to_ast(),
            param
                .identifier_declaration_base
                .annotated_type
                .type_name
                .try_as_array_ref()
                .unwrap()
                .try_as_cipher_text_ref()
                .unwrap()
                .crypto_params
                .clone(),
            cipher_idf,
            true,
            false,
        );
    }

    pub fn get_randomness_for_rerand(&mut self, expr: Expression) -> IdentifierExpr {
        let idf = self._secret_input_name_factory.get_new_idf(
            &TypeName::rnd_type(
                expr.annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .try_as_array_ref()
                    .unwrap()
                    .try_as_randomness_ref()
                    .unwrap()
                    .crypto_params
                    .clone(),
            ),
            None,
        );
        IdentifierExpr::new(
            IdentifierExprUnion::Identifier(Identifier::HybridArgumentIdf(idf)),
            None,
        )
    }

    pub fn evaluate_expr_in_circuit(
        &mut self,
        expr: &mut Expression,
        new_privacy: &AST,
        homomorphism: &String,
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
        self.circ_indent_block(&expr.code());
        self._get_circuit_output_for_private_expression(expr, &new_privacy, &homomorphism)
            .unwrap()
    }

    pub fn evaluate_stmt_in_circuit(&mut self, mut ast: Statement) -> SimpleStatement
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
        let mut astmt = SimpleStatement::ExpressionStatement(ExpressionStatement::new(
            NumberLiteralExpr::new(0, false).to_expr(),
        ));
        for var in &ast.ast_base_ref().unwrap().modified_values {
            if var.in_scope_at(ast.to_ast()) {
                astmt =
                    SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(
                        AssignmentStatementBase::new(None, None, None),
                    ));
                break;
            }
        }
        astmt.statement_base_mut_ref().before_analysis =
            ast.statement_base_ref().unwrap().before_analysis().clone();

        //External values written inside statement -> function return values
        let mut ret_params = vec![];
        for var in &ast.ast_base_ref().unwrap().modified_values {
            if var.in_scope_at(ast.to_ast()) {
                //side effect affects location outside statement and has privacy @me
                assert!(ast
                    .statement_base_ref()
                    .unwrap()
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .same_partition(&var.privacy().unwrap(), &Expression::me_expr(None).to_ast()));
                assert!(is_instances(
                    &*var.target().unwrap(),
                    vec![
                        ASTType::Parameter,
                        ASTType::VariableDeclaration,
                        ASTType::StateVariableDeclaration
                    ]
                ));
                let t = var
                    .target()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .zkay_type();
                if !t.type_name.is_primitive_type() {
                    unimplemented!(
                        "Reference types inside private if statements are not supported"
                    );
                }
                let ret_t = AnnotatedTypeName::new(
                    *t.type_name,
                    Some(Expression::me_expr(None).into_ast()),
                    t.homomorphism,
                ); //t, but @me
                let mut idf = IdentifierExpr::new(
                    IdentifierExprUnion::Identifier(
                        *var.target()
                            .unwrap()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .idf()
                            .clone(),
                    ),
                    Some(Box::new(ret_t)),
                );
                idf.location_expr_base.target = var.target();
                let mut ret_param = idf;
                ret_param
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .statement = Some(Box::new(astmt.to_statement()));
                ret_params.push(ret_param);
            }
        }

        //Build the imaginary function
        let mut fdef = ConstructorOrFunctionDefinition::new(
            Some(Identifier::Identifier(IdentifierBase::new(String::from(
                "<stmt_fct>",
            )))),
            Some(vec![]),
            Some(zkay_config::lc_vec_s!["private"]),
            Some(
                ret_params
                    .iter()
                    .map(|ret| {
                        Parameter::new(
                            vec![],
                            *ret.annotated_type.clone().unwrap(),
                            *ret.location_expr_base
                                .target
                                .clone()
                                .unwrap()
                                .try_as_identifier_declaration_ref()
                                .unwrap()
                                .idf()
                                .clone(),
                            None,
                        )
                    })
                    .collect(),
            ),
            Some(Block::new(
                vec![
                    ast.to_ast(),
                    ReturnStatement::new(Some(
                        TupleExpr::new(ret_params.iter().map(|r| r.to_expr()).collect()).to_expr(),
                    ))
                    .to_ast(),
                ],
                false,
            )),
        );
        fdef.original_body = fdef.body.clone();
        fdef.body
            .as_mut()
            .unwrap()
            .statement_list_base
            .statement_base
            .ast_base
            .parent = Some(Box::new(fdef.to_ast()));
        fdef.parent = None; //TODO Statement to ContractDefinition   ast.clone();

        //inline "Call" to the imaginary function
        let mut idf = IdentifierExpr::new(
            IdentifierExprUnion::String(String::from("<stmt_fct>")),
            None,
        );
        idf.location_expr_base.target = Some(Box::new(fdef.to_ast().into()));
        let mut fcall = FunctionCallExprBase::new(idf.to_expr(), vec![], None);
        fcall.expression_base.statement = Some(Box::new(astmt.to_statement()));
        let mut ret_args = self.inline_function_call_into_circuit(&mut fcall);
        assert!(ret_args.is_some());
        let mut ret_args = ret_args.unwrap();
        //Move all return values out of the circuit
        let mut ret_args = if !is_instance(&ret_args, ASTType::TupleExpr) {
            TupleExpr::new(vec![ret_args.try_as_expression().unwrap()]).into_ast()
        } else {
            ret_args
        };
        for ret_arg in ret_args
            .try_as_expression_mut()
            .unwrap()
            .try_as_tuple_or_location_expr_mut()
            .unwrap()
            .try_as_tuple_expr_mut()
            .unwrap()
            .elements
            .iter_mut()
        {
            ret_arg.expression_base_mut_ref().statement = Some(Box::new(astmt.to_statement()));
        }
        let ret_arg_outs: Vec<_> = ret_params
            .iter()
            .zip(
                ret_args
                    .try_as_expression_mut()
                    .unwrap()
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_tuple_expr_mut()
                    .unwrap()
                    .elements
                    .iter_mut(),
            )
            .map(|(ret_param, ret_arg)| {
                self._get_circuit_output_for_private_expression(
                    ret_arg,
                    &Expression::me_expr(None).to_ast(),
                    &ret_param.annotated_type.clone().unwrap().homomorphism,
                )
                .unwrap()
            })
            .collect();

        //Create assignment statement
        if !ret_params.is_empty() {
            astmt
                .try_as_assignment_statement_mut()
                .unwrap()
                .assignment_statement_base_mut_ref()
                .lhs = Some(Box::new(
                TupleExpr::new(ret_params.iter().map(|r| r.to_expr()).collect()).to_ast(),
            ));
            astmt
                .try_as_assignment_statement_mut()
                .unwrap()
                .assignment_statement_base_mut_ref()
                .rhs =
                Some(TupleExpr::new(ret_arg_outs.iter().map(|r| r.to_expr()).collect()).to_expr());
            astmt
        } else {
            assert!(is_instance(&astmt, ASTType::ExpressionStatement));
            astmt
        }
    }
    pub fn invalidate_idf(&mut self, target_idf: &Identifier) {
        if self._remapper.0.is_remapped(target_idf) {
            self._remapper.0.reset_key(target_idf);
        }
    }

    pub fn call_function(&mut self, ast: &FunctionCallExpr)
    // """
    // Include public function call to a function which requires verification in this circuit.

    // :param ast: The function call to include, target function must require verification
    // """
    {
        assert!(
            ast.func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification
        );
        self.function_calls_with_verification.push(ast.clone());
        self._phi.push(CircuitStatement::CircCall(CircCall::new(
            ast.func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .clone(),
        )));
    }

    pub fn request_public_key(
        &mut self,
        crypto_params: &CryptoParams,
        plabel: Option<AST>,
        name: &str,
    ) -> (HybridArgumentIdf, AssignmentStatement) //(Identifier,CircuitInputStatement)
    // """
    // Request key for the address corresponding to plabel from pki infrastructure and add it to the public circuit inputs.

    // :param plabel: privacy label for which to request key
    // :param name: name to use for the HybridArgumentIdf holding the key
    // :return: HybridArgumentIdf containing the requested key and an AssignmentStatement which assigns the key request to the idf location
    // """
    {
        let idf = self._in_name_factory.add_idf(
            name.to_owned(),
            TypeName::key_type(crypto_params.clone()),
            None,
        );
        let pki = IdentifierExpr::new(
            IdentifierExprUnion::String(
                CFG.lock().unwrap().get_contract_var_name(
                    CFG.lock()
                        .unwrap()
                        .get_pki_contract_name(&crypto_params.identifier_name()),
                ),
            ),
            None,
        );
        let privacy_label_expr = get_privacy_expr_from_label(plabel.unwrap());
        let le = idf.get_loc_expr(None);
        let le = if let Some(le) = le
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
        {
            Some(le)
        } else {
            None
        };
        (
            idf,
            le.unwrap().assign(
                LocationExpr::IdentifierExpr(pki)
                    .call(
                        IdentifierExprUnion::String(String::from("getPk")),
                        if let Some(AST::Expression(expr)) = self
                            ._expr_trafo
                            .as_ref()
                            .unwrap()
                            .visit(Some(privacy_label_expr.to_ast()))
                        {
                            vec![expr]
                        } else {
                            vec![]
                        },
                    )
                    .to_expr(),
            ),
        )
    }

    pub fn request_private_key(&mut self, crypto_params: &CryptoParams) -> Vec<AST> {
        assert!(
            self._needed_secret_key.contains(&crypto_params)
                || self
                    .fct
                    .parameters
                    .iter()
                    .filter_map(
                        |p| if p.identifier_declaration_base.annotated_type.is_cipher() {
                            Some(
                                p.identifier_declaration_base
                                    .annotated_type
                                    .type_name
                                    .try_as_array_ref()
                                    .unwrap()
                                    .try_as_key_ref()
                                    .unwrap()
                                    .crypto_params
                                    .clone(),
                            )
                        } else {
                            None
                        }
                    )
                    .collect::<Vec<_>>()
                    .contains(crypto_params)
        );
        let key_name = Self::get_own_secret_key_name(crypto_params);
        self._secret_input_name_factory.add_idf(
            key_name,
            TypeName::key_type(crypto_params.clone()),
            None,
        );
        vec![EnterPrivateKeyStatement::new(crypto_params.clone()).to_ast()]
    }

    //Circuit-side interface #
    pub fn add_to_circuit_inputs(&mut self, expr: &mut Expression) -> HybridArgumentIdf
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
        let privacy = if expr.annotated_type().as_ref().unwrap().is_private() {
            expr.annotated_type()
                .as_ref()
                .unwrap()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .privacy_annotation_label()
        } else {
            Some(Expression::all_expr().to_ast())
        };
        let is_public = privacy == Some(Expression::all_expr().to_ast());

        let expr_text = expr.code();
        let input_expr = if let Some(AST::Expression(expr)) = self
            ._expr_trafo
            .as_ref()
            .unwrap()
            .visit(Some(expr.to_ast()))
        {
            Some(expr)
        } else {
            None
        };
        let t = input_expr
            .as_ref()
            .unwrap()
            .annotated_type()
            .as_ref()
            .unwrap()
            .type_name
            .clone();
        let mut locally_decrypted_idf = None;

        //If expression has literal type -> evaluate it inside the circuit (constant folding will be used)
        //rather than introducing an unnecessary public circuit input (expensive)
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(t)) = *t {
            return self
                ._evaluate_private_expression(input_expr.unwrap(), &t.value().to_string())
                .unwrap();
        } else if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(t),
        )) = *t
        {
            return self
                ._evaluate_private_expression(input_expr.unwrap(), &t.value().to_string())
                .unwrap();
        }

        let mut t_suffix = String::new();
        if is_instance(expr, ASTType::IdentifierExpr)
        //Look in cache before doing expensive move-in
        {
            if self._remapper.0.is_remapped(
                &expr
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .idf(),
            ) {
                return self._remapper.0.get_current(
                    *expr
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .as_ref()
                        .unwrap()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .idf()
                        .clone(),
                    None,
                );
            }

            t_suffix = format!(
                "_{}",
                expr.try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .idf()
                    .name()
            );
        }

        //Generate circuit inputs
        let (return_idf, input_idf) = if is_public {
            let tname = format!(
                "{}{t_suffix}",
                self._in_name_factory
                    .base_name_factory
                    .get_new_name(&*expr.annotated_type().as_ref().unwrap().type_name, false)
            );
            let input_idf = self._in_name_factory.add_idf(
                tname,
                *expr.annotated_type().as_ref().unwrap().type_name.clone(),
                None,
            );
            let return_idf = input_idf.clone();
            self._phi
                .push(CircuitStatement::CircComment(CircComment::new(format!(
                    "{} = {expr_text}",
                    input_idf.identifier_base.name
                ))));
            (return_idf, input_idf)
        } else
        //Encrypted inputs need to be decrypted inside the circuit (i.e. add plain as private input and prove encryption)
        {
            let tname = format!(
                "{}{t_suffix}",
                self._secret_input_name_factory
                    .base_name_factory
                    .get_new_name(&*expr.annotated_type().as_ref().unwrap().type_name, false)
            );
            let locally_decrypted_idf = self._secret_input_name_factory.add_idf(
                tname,
                *expr.annotated_type().as_ref().unwrap().type_name.clone(),
                None,
            );
            let return_idf = locally_decrypted_idf.clone();
            let cipher_t = TypeName::cipher_type(
                input_expr
                    .as_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .clone(),
                expr.annotated_type().as_ref().unwrap().homomorphism.clone(),
            );
            let tname = format!(
                "{}{t_suffix}",
                self._in_name_factory
                    .base_name_factory
                    .get_new_name(&cipher_t, false)
            );
            let input_idf = self._in_name_factory.add_idf(
                tname,
                cipher_t,
                Some(
                    IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(Identifier::HybridArgumentIdf(
                            locally_decrypted_idf,
                        )),
                        None,
                    )
                    .to_expr(),
                ),
            );
            (return_idf, input_idf)
        };

        //Add a CircuitInputStatement to the solidity code, which looks like a normal assignment statement,
        //but also signals the offchain simulator to perform decryption if necessary
        expr.expression_base_mut_ref()
            .statement
            .as_mut()
            .unwrap()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(
                CircuitInputStatement::new(
                    input_idf.get_loc_expr(None).into(),
                    input_expr.unwrap(),
                    None,
                )
                .into_ast(),
            );

        if !is_public {
            //Check if the secret plain input corresponds to the decrypted cipher value
            let crypto_params = CFG
                .lock()
                .unwrap()
                .user_config
                .get_crypto_params(&expr.annotated_type().as_ref().unwrap().homomorphism);
            self._phi
                .push(CircuitStatement::CircComment(CircComment::new(format!(
                    "{:?} = dec({expr_text}) [{}]",
                    locally_decrypted_idf.clone().unwrap(),
                    input_idf.identifier_base.name
                ))));
            let mut statement = *expr.expression_base_mut_ref().statement.clone().unwrap();
            self._ensure_encryption(
                &mut statement,
                locally_decrypted_idf.clone().unwrap(),
                Expression::me_expr(None).to_ast(),
                crypto_params,
                input_idf,
                false,
                true,
            );
            expr.expression_base_mut_ref().statement = Some(Box::new(statement));
        }

        //Cache circuit input for later reuse if possible
        if CFG.lock().unwrap().user_config.opt_cache_circuit_inputs()
            && is_instance(expr, ASTType::IdentifierExpr)
        //TODO: What if a homomorphic variable gets used as both a plain variable and as a ciphertext?
        //      This works for now because we never perform homomorphic operations on variables we can decrypt.
        {
            self._remapper.0.remap(
                *expr
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .idf()
                    .clone(),
                return_idf.clone(),
            );
        }

        return_idf
    }
    pub fn get_remapped_idf_expr(&self, idf: IdentifierExpr) -> IdentifierExpr
// """
        // Get location expression for the most recently assigned value of idf according to the SSA simulation.

        // :param idf: Identifier expression to lookup
        // :return: Either idf itself (not currently remapped)
        //          or a loc expr for the HybridArgumentIdf which references the most recent value of idf
        // """
    {
        let target: Option<AST> = idf.location_expr_base.target.clone().map(|v| (*v).into());
        assert!(target.is_some());
        assert!(!is_instance(&*idf.idf, ASTType::HybridArgumentIdf));
        if self._remapper.0.is_remapped(
            &target
                .as_ref()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .idf()
                .clone(),
        ) {
            let remapped_idf = self._remapper.0.get_current(
                *target
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .idf()
                    .clone(),
                None,
            );
            remapped_idf
                .get_idf_expr(
                    &idf.location_expr_base
                        .tuple_or_location_expr_base
                        .expression_base
                        .ast_base
                        .parent,
                )
                .as_type(AST::AnnotatedTypeName(*idf.annotated_type.unwrap()))
        } else {
            idf
        }
    }
    pub fn create_new_idf_version_from_value(&mut self, orig_idf: Identifier, expr: Expression)
    // """
    // Store expr in a new version of orig_idf (for SSA).

    // :param orig_idf: the identifier which should be updated with a new value
    // :param expr: the updated value
    // :param is_local: whether orig_idf refers to a local variable (as opposed to a state variable)
    // """
    {
        let tmp_var = self._create_temp_var(&orig_idf.name(), expr);
        self._remapper.0.remap(orig_idf, tmp_var);
    }

    pub fn inline_function_call_into_circuit(
        &mut self,
        fcall: &mut FunctionCallExprBase,
    ) -> Option<AST>
// """
        // Inline an entire function call into the current circuit.

        // :param fcall: Function call to inline
        // :return: Expression (1 retval) / TupleExpr (multiple retvals) with return value(s)
        // """
    {
        assert!(
            is_instance(&*fcall.func, ASTType::LocationExprBase)
                && fcall
                    .func
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .is_some()
        );
        let fdef = fcall
            .func
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .clone();
        //with
        self._remapper.0.remap_scope(Some(
            (*fcall
                .func
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .clone())
            .try_as_namespace_definition()
            .unwrap()
            .try_as_constructor_or_function_definition()
            .unwrap()
            .body
            .unwrap(),
        ));

        //with
        if fcall
            .func
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .as_ref()
            .unwrap()
            .clone()
            .try_as_namespace_definition()
            .unwrap()
            .idf()
            .name()
            != "<stmt_fct>"
        {
            self.circ_indent_block(&format!("INLINED {}", fcall.to_ast().code()));
        }

        //Assign all arguments to temporary circuit variables which are designated as the current version of the parameter idfs
        for (param, arg) in fdef
            .as_ref()
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .parameters
            .iter()
            .zip(&fcall.args)
        {
            self._phi
                .push(CircuitStatement::CircComment(CircComment::new(format!(
                    "ARG {}: {}",
                    param.identifier_declaration_base.idf.name(),
                    arg.code()
                ))));
            // with
            self.circ_indent_block("");
            {
                self.create_new_idf_version_from_value(
                    *param.identifier_declaration_base.idf.clone(),
                    arg.clone(),
                );
            }
        }

        //Visit the untransformed target function body to include all statements in this circuit
        let inlined_body = fdef
            .as_ref()
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .original_body
            .clone(); //deep_copy(fdef.original_body, true, true);
        self._circ_trafo
            .as_ref()
            .unwrap()
            .visit(Some(inlined_body.as_ref().unwrap().to_ast()));

        fcall
            .expression_base
            .statement
            .as_mut()
            .unwrap()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .extend(
                inlined_body
                    .unwrap()
                    .statement_list_base
                    .statement_base
                    .pre_statements
                    .iter()
                    .map(|ps| ps.try_as_statement_ref().unwrap().to_ast())
                    .collect::<Vec<_>>(),
            );

        //Create TupleExpr with location expressions corresponding to the function return values as elements
        let ret_idfs: Vec<_> = (*fdef.as_ref().unwrap())
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .return_var_decls
            .iter()
            .map(|vd| {
                self._remapper
                    .0
                    .get_current((*vd.identifier_declaration_base.idf).clone(), None)
            })
            .collect();
        let mut ret = TupleExpr::new(
            ret_idfs
                .iter()
                .map(|idf| {
                    IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(Identifier::HybridArgumentIdf(idf.clone())),
                        None,
                    )
                    .as_type(AST::TypeName((*idf.t).clone()))
                    .to_expr()
                })
                .collect(),
        );

        Some(
            if ret.elements.len() == 1
            //Unpack 1-length tuple
            {
                // ret = if let Expression::TupleOrLocationExpr(TupleOrLocationExpr::TupleExpr(ret))=&ret.elements[0]{ret.clone()}else{TupleExpr::default()};
                ret.elements[0].to_ast()
            } else {
                ret.to_ast()
            },
        )
    }
    pub fn add_assignment_to_circuit(&mut self, ast: &mut AssignmentStatement)
    // """Include private assignment statement in this circuit."""
    {
        self._phi
            .push(CircuitStatement::CircComment(CircComment::new(
                ast.to_ast().code(),
            )));
        self._add_assign(
            ast.lhs()
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .clone(),
            ast.assignment_statement_base_mut_ref()
                .rhs
                .as_mut()
                .unwrap(),
        );
    }

    pub fn add_var_decl_to_circuit(&mut self, ast: &mut VariableDeclarationStatement) {
        self._phi
            .push(CircuitStatement::CircComment(CircComment::new(
                ast.to_ast().code(),
            )));
        if ast.expr.is_none()
        //Default initialization is made explicit for circuit variables
        {
            let t = ast
                .variable_declaration
                .identifier_declaration_base
                .annotated_type
                .type_name
                .clone();
            assert!(t.can_be_private());
            let mut nle = NumberLiteralExpr::new(0, false);
            nle.literal_expr_base.expression_base.ast_base.parent = Some(Box::new(ast.to_ast()));
            nle.literal_expr_base.expression_base.statement = Some(Box::new(ast.to_statement()));
            ast.expr = Some(TypeCheckVisitor::implicitly_converted_to(
                nle.to_expr(),
                &*t,
            ));
        }
        self.create_new_idf_version_from_value(
            *ast.variable_declaration
                .identifier_declaration_base
                .idf
                .clone(),
            ast.expr.clone().unwrap(),
        );
    }

    pub fn add_return_stmt_to_circuit(&mut self, ast: &mut ReturnStatement) {
        self._phi
            .push(CircuitStatement::CircComment(CircComment::new(
                ast.to_ast().code(),
            )));
        assert!(ast.expr.is_some());
        if !is_instance(ast.expr.as_ref().unwrap(), ASTType::TupleExpr) {
            ast.expr = Some(TupleExpr::new(vec![ast.expr.clone().unwrap()]).to_expr());
        }

        for (vd, expr) in ast
            .statement_base
            .function
            .as_mut()
            .unwrap()
            .return_var_decls
            .iter()
            .zip(
                &ast.expr
                    .as_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements,
            )
        {
            //Assign return value to new version of return variable
            self.create_new_idf_version_from_value(
                *vd.identifier_declaration_base.idf.clone(),
                expr.clone(),
            );
        }
    }

    pub fn add_if_statement_to_circuit(&mut self, ast: &mut IfStatement)
    // """Include private if statement in this circuit."""
    {
        //Handle if branch
        // with
        self._remapper.0.remap_scope(None);
        let mut comment = CircComment::new(format!("if ({})", ast.condition.code()));
        self._phi
            .push(CircuitStatement::CircComment(comment.clone()));
        let cond = self._evaluate_private_expression(ast.condition.clone(), "");
        comment.text += &format!(" [{}]", cond.as_ref().unwrap().identifier_base.name);
        self._circ_trafo.as_ref().unwrap().visitBlock(
            Some(ast.then_branch.to_ast()),
            Some(cond.clone().unwrap()),
            Some(true),
        );
        let then_remap = self._remapper.0.get_state();

        //Bubble up nested pre statements
        let mut ps: Vec<_> = ast
            .then_branch
            .statement_list_base
            .statement_base
            .pre_statements
            .drain(..)
            .collect();
        ast.statement_base.pre_statements.append(&mut ps);
        // ast.then_branch.pre_statements = vec![];

        //Handle else branch
        if ast.else_branch.is_some() {
            self._phi
                .push(CircuitStatement::CircComment(CircComment::new(format!(
                    "else [{}]",
                    cond.as_ref().unwrap().identifier_base.name
                ))));
            self._circ_trafo.as_ref().unwrap().visitBlock(
                Some(ast.else_branch.as_ref().unwrap().to_ast()),
                Some(cond.clone().unwrap()),
                Some(false),
            );

            //Bubble up nested pre statements
            let mut ps: Vec<_> = ast
                .else_branch
                .as_ref()
                .unwrap()
                .statement_list_base
                .statement_base
                .pre_statements
                .clone();
            ast.statement_base.pre_statements.append(&mut ps);
            // ast.else_branch.pre_statements = vec![];
        }

        //SSA join branches (if both branches write to same external value -> cond assignment to select correct version)
        // with
        self.circ_indent_block(&format!(
            "JOIN [{}]",
            cond.as_ref().unwrap().identifier_base.name
        ));
        let cond_idf_expr = cond.unwrap().get_idf_expr(&Some(Box::new(ast.to_ast())));
        assert!(is_instance(&cond_idf_expr, ASTType::IdentifierExpr));
        let mut selfs = self.clone();
        self._remapper.0.join_branch(
            ast.clone(),
            cond_idf_expr,
            then_remap,
            // |s: String, e: Expression| -> HybridArgumentIdf { selfs._create_temp_var(&s, e) },
            &mut selfs,
        );
    }
    pub fn add_block_to_circuit(
        &mut self,
        ast: &mut Block,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) {
        assert!(ast
            .statement_list_base
            .statement_base
            .ast_base
            .parent
            .is_some());
        let is_already_scoped = is_instances(
            &*ast
                .statement_list_base
                .statement_base
                .ast_base
                .parent
                .clone()
                .unwrap(),
            vec![
                ASTType::ConstructorOrFunctionDefinition,
                ASTType::IfStatement,
            ],
        );
        self._phi
            .push(CircuitStatement::CircComment(CircComment::new(
                String::from("{"),
            )));
        // with
        self.circ_indent_block("");
        // with
        if let Some(guard_cond) = guard_cond {
            self.guarded(guard_cond, guard_val.unwrap());
        }
        //with
        if !is_already_scoped {
            self._remapper.0.remap_scope(Some(ast.clone()));
        }
        let mut statements = vec![];
        for stmt in ast.statement_list_base.statements.iter_mut() {
            if let AST::Statement(ref mut stmt) = stmt {
                self._circ_trafo
                    .as_ref()
                    .unwrap()
                    .visit(Some((*stmt).to_ast()));
                //Bubble up nested pre statements
                statements.append(
                    &mut stmt
                        .statement_base_mut_ref()
                        .unwrap()
                        .pre_statements
                        .drain(..)
                        .collect::<Vec<_>>(),
                );
            }
            // stmt.pre_statements = vec![];
        }
        ast.statement_list_base
            .statement_base
            .pre_statements
            .append(&mut statements);

        self._phi
            .push(CircuitStatement::CircComment(CircComment::new(
                String::from("}"),
            )));
    }

    //Internal functionality #

    pub fn _get_canonical_privacy_label(
        &self,
        analysis: &PartitionState<AST>,
        privacy: &AST,
    ) -> AST
// """
    // If privacy is equivalent to a static privacy label -> Return the corresponding static label, otherwise itself.

    // :param analysis: analysis state at the statement where expression with the given privacy occurs
    // :param privacy: original privacy label
    // """
    {
        for owner in &self.static_owner_labels {
            if analysis.same_partition(owner, privacy) {
                return owner.clone();
            }
        }
        privacy.clone()
    }

    pub fn _create_temp_var(&mut self, tag: &str, expr: Expression) -> HybridArgumentIdf
// """Assign expression to a fresh temporary circuit variable."""
    {
        self._evaluate_private_expression(expr, &format!("_{tag}"))
            .unwrap()
    }

    pub fn _add_assign(&mut self, lhs: Expression, rhs: &mut Expression)
    // """
    // Simulate an assignment of rhs to lhs inside the circuit.

    // :param lhs: destination
    // :param rhs: source
    // """
    {
        if is_instance(&lhs, ASTType::IdentifierExpr)
        //for now no ref types
        {
            assert!(lhs
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .is_some());
            self.create_new_idf_version_from_value(
                *lhs.try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .idf()
                    .clone(),
                rhs.clone(),
            );
        } else if is_instance(&lhs, ASTType::IndexExpr) {
            // raise NotImplementedError()
            unimplemented!();
        } else {
            assert!(is_instance(&lhs, ASTType::TupleExpr));
            if is_instance(&*rhs, ASTType::FunctionCallExprBase) {
                if let Some(AST::Expression(expr)) =
                    self._circ_trafo.as_ref().unwrap().visit(Some(rhs.to_ast()))
                {
                    *rhs = expr;
                }
            }
            assert!(
                is_instance(&*rhs, ASTType::TupleExpr)
                    && lhs
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_tuple_expr_ref()
                        .unwrap()
                        .elements
                        .len()
                        == rhs
                            .try_as_tuple_or_location_expr_ref()
                            .unwrap()
                            .try_as_tuple_expr_ref()
                            .unwrap()
                            .elements
                            .len()
            );
            for (e_l, e_r) in lhs
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_tuple_expr_ref()
                .unwrap()
                .elements
                .iter()
                .zip(
                    &rhs.try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_tuple_expr_ref()
                        .unwrap()
                        .elements
                        .clone(),
                )
            {
                self._add_assign(e_l.clone(), &mut e_r.clone());
            }
        }
    }

    pub fn _get_circuit_output_for_private_expression(
        &mut self,
        expr: &mut Expression,
        new_privacy: &AST,
        homomorphism: &String,
    ) -> Option<LocationExpr>
// """
        // Add evaluation of expr to the circuit and return the output HybridArgumentIdf corresponding to the evaluation result.

        // Note: has side effects on expr.statement (adds pre_statement)

        // :param expr: [SIDE EFFECT] expression to evaluate
        // :param new_privacy: result owner (determines encryption key)
        // :return: HybridArgumentIdf which references the circuit output containing the result of expr
        // """
    {
        let is_circ_val = is_instance(&*expr, ASTType::IdentifierExpr)
            && is_instance(
                &*expr
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .idf,
                ASTType::HybridArgumentIdf,
            )
            && expr
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .arg_type
                != HybridArgType::PubContractVal;
        let is_hom_comp = is_instance(&(*expr), ASTType::FunctionCallExprBase)
            && is_instance(
                &**(*expr).try_as_function_call_expr_ref().unwrap().func(),
                ASTType::BuiltinFunction,
            )
            && (*expr)
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .homomorphism
                != Homomorphism::non_homomorphic();
        if is_hom_comp
        //Treat a homomorphic operation as a privately evaluated operation on (public) ciphertexts
        {
            expr.try_as_function_call_expr_mut()
                .unwrap()
                .expression_base_mut_ref()
                .annotated_type = Some(AnnotatedTypeName::cipher_type(
                expr.annotated_type().as_ref().unwrap().clone(),
                Some(homomorphism.clone()),
            ));
        }

        let priv_result_idf = if is_circ_val
            || expr.annotated_type().as_ref().unwrap().is_private()
            || (*expr).evaluate_privately()
        {
            self._evaluate_private_expression(expr.clone(), "").unwrap()
        } else
        //For public expressions which should not be evaluated in private, only the result is moved into the circuit
        {
            self.add_to_circuit_inputs(expr)
        };
        let private_expr = priv_result_idf.get_idf_expr(&None);

        let mut t_suffix = String::new();
        if is_instance(expr, ASTType::IdentifierExpr) && !is_circ_val {
            t_suffix += &format!(
                "_{}",
                expr.try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .idf
                    .name()
            );
        }

        let (out_var, new_out_param) = if is_instance(new_privacy, ASTType::AllExpr)
            || expr
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .is_cipher()
        //If the result is public, add an equality constraint to ensure that the user supplied public output
        //is equal to the circuit evaluation result
        {
            let tname = format!(
                "{}{t_suffix}",
                self._out_name_factory
                    .base_name_factory
                    .get_new_name(&*expr.annotated_type().as_ref().unwrap().type_name, false)
            );
            let new_out_param = self._out_name_factory.add_idf(
                tname,
                *expr.annotated_type().as_ref().unwrap().type_name.clone(),
                Some(private_expr.to_expr()),
            );
            self._phi
                .push(CircuitStatement::CircEqConstraint(CircEqConstraint::new(
                    priv_result_idf,
                    new_out_param.clone(),
                )));
            (
                new_out_param
                    .clone()
                    .get_loc_expr(None)
                    .try_as_expression_ref()
                    .unwrap()
                    .explicitly_converted(
                        *expr.annotated_type().as_ref().unwrap().type_name.clone(),
                    ),
                new_out_param,
            )
        } else
        //If the result is encrypted, add an encryption constraint to ensure that the user supplied encrypted output
        //is equal to the correctly encrypted circuit evaluation result
        {
            let new_privacy =
                self._get_canonical_privacy_label(&expr.analysis().unwrap(), new_privacy);
            let privacy_label_expr = get_privacy_expr_from_label(new_privacy.clone());
            let cipher_t = TypeName::cipher_type(
                expr.annotated_type().as_ref().unwrap().clone(),
                homomorphism.clone(),
            );
            let tname = format!(
                "{}{t_suffix}",
                self._out_name_factory
                    .base_name_factory
                    .get_new_name(&cipher_t, false)
            );
            let enc_expr = EncryptionExpression::new(
                private_expr.to_expr(),
                privacy_label_expr.to_ast(),
                Some(homomorphism.clone()),
            );
            let new_out_param =
                self._out_name_factory
                    .add_idf(tname, cipher_t.clone(), Some(enc_expr.to_expr()));
            let crypto_params = CFG
                .lock()
                .unwrap()
                .user_config
                .get_crypto_params(homomorphism);
            self._ensure_encryption(
                expr.expression_base_mut_ref().statement.as_mut().unwrap(),
                priv_result_idf,
                new_privacy,
                crypto_params,
                new_out_param.clone(),
                false,
                false,
            );
            (new_out_param.get_loc_expr(None).into(), new_out_param)
        };

        //Add an invisible CircuitComputationStatement to the solidity code, which signals the offchain simulator,
        //that the value the contained out variable must be computed at this point by simulating expression evaluation
        expr.expression_base_mut_ref()
            .statement
            .as_mut()
            .unwrap()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(CircuitComputationStatement::new(new_out_param).into_ast());
        if let AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(le),
        )) = out_var
        {
            Some(le)
        } else {
            None
        }
    }
    pub fn _evaluate_private_expression(
        &mut self,
        expr: Expression,
        tmp_idf_suffix: &str,
    ) -> Option<HybridArgumentIdf>
// """
        // Evaluate expr in the circuit (if not already done) and store result in a new temporary circuit variable.

        // :param expr: expression to evaluate
        // :param tmp_idf_suffix: name suffix for the new temporary circuit variable
        // :return: temporary circuit variable HybridArgumentIdf which refers to the transformed circuit expression
        // """
    {
        assert!(
            !(is_instance(&expr, ASTType::MemberAccessExpr)
                && is_instance(
                    &*expr
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .clone(),
                    ASTType::HybridArgumentIdf
                ))
        );
        if is_instance(&expr, ASTType::IdentifierExpr)
            && is_instance(
                &*expr
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .idf,
                ASTType::HybridArgumentIdf,
            )
            && expr
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .arg_type
                != HybridArgType::PubContractVal
        //Already evaluated in circuit
        {
            return expr
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .clone()
                .try_as_hybrid_argument_idf();
        }

        let priv_expr = self
            ._circ_trafo
            .as_ref()
            .unwrap()
            .visit(Some(expr.to_ast()));
        let tname = format!(
            "{}{tmp_idf_suffix}",
            self._circ_temp_name_factory.base_name_factory.get_new_name(
                &*priv_expr
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name,
                false
            )
        );
        let tmp_circ_var_idf = self._circ_temp_name_factory.add_idf(
            tname,
            *priv_expr
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .clone(),
            if let Some(AST::Expression(expr)) = &priv_expr {
                Some(expr.clone())
            } else {
                None
            },
        );
        let stmt = CircVarDecl::new(
            tmp_circ_var_idf.clone(),
            priv_expr.unwrap().try_as_expression_ref().unwrap().clone(),
        );
        self._phi.push(CircuitStatement::CircVarDecl(stmt));
        Some(tmp_circ_var_idf)
    }

    pub fn _ensure_encryption(
        &mut self,
        stmt: &mut Statement,
        plain: HybridArgumentIdf,
        new_privacy: AST,
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
            self._require_secret_key(&crypto_params);
            let my_pk = self._require_public_key_for_label_at(
                Some(stmt),
                &Expression::me_expr(None).to_ast(),
                &crypto_params,
            );
            let other_pk = if is_dec {
                self._get_public_key_in_sender_field(stmt, cipher.clone(), crypto_params)
            } else {
                if new_privacy == Expression::me_expr(None).to_ast() {
                    my_pk
                } else {
                    self._require_public_key_for_label_at(Some(stmt), &new_privacy, &crypto_params)
                }
            };

            self._phi
                .push(CircuitStatement::CircComment(CircComment::new(format!(
                    "{} = enc({}, ecdh({}, my_sk))",
                    cipher.identifier_base.name,
                    plain.identifier_base.name,
                    other_pk.identifier_base.name
                ))));
            self._phi.push(CircuitStatement::CircSymmEncConstraint(
                CircSymmEncConstraint::new(plain, other_pk, cipher, is_dec),
            ));
        } else {
            let rnd = self._secret_input_name_factory.add_idf(
                format!(
                    "{}_R",
                    if is_param {
                        plain.clone().identifier_base.name
                    } else {
                        cipher.clone().identifier_base.name
                    }
                ),
                TypeName::rnd_type(crypto_params.clone()),
                None,
            );
            let pk =
                self._require_public_key_for_label_at(Some(stmt), &new_privacy, &crypto_params);
            if !is_dec {
                self._phi
                    .push(CircuitStatement::CircComment(CircComment::new(format!(
                        "{} = enc({}, {})",
                        cipher.identifier_base.name,
                        plain.identifier_base.name,
                        pk.identifier_base.name
                    ))));
            }
            self._phi
                .push(CircuitStatement::CircEncConstraint(CircEncConstraint::new(
                    plain, rnd, pk, cipher, is_dec,
                )));
        }
    }

    pub fn _require_secret_key(&mut self, crypto_params: &CryptoParams) -> HybridArgumentIdf {
        self._needed_secret_key.insert(crypto_params.clone()); //Add to _need_secret_key OrderedDict
        let key_name = Self::get_own_secret_key_name(crypto_params);
        HybridArgumentIdf::new(
            key_name,
            TypeName::key_type(crypto_params.clone()),
            HybridArgType::PrivCircuitVal,
            None,
        )
    }

    pub fn _require_public_key_for_label_at(
        &mut self,
        mut stmt: Option<&mut Statement>,
        privacy: &AST,
        crypto_params: &CryptoParams,
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
        if self.static_owner_labels.contains(&privacy)
        //Statically known privacy -> keep track (all global keys will be requested only once)
        {
            self._global_keys
                .insert((privacy.clone().into(), crypto_params.clone()));
            return HybridArgumentIdf::new(
                Self::get_glob_key_name(&privacy, crypto_params),
                TypeName::key_type(crypto_params.clone()),
                HybridArgType::PubCircuitArg,
                None,
            );
        }
        if stmt.is_some() {
            assert!(
                false,
                "stmt cannot be None if privacy is not guaranteed to be statically known"
            )
        }

        //privacy cannot be MeExpr (is in _static_owner_labels) or AllExpr (has no public key)
        assert!(is_instance(privacy, ASTType::IdentifierBase));

        if let Some(requested_dynamic_pks) =
            self._requested_dynamic_pks.get(&*stmt.as_ref().unwrap())
        {
            if let Some(v) = requested_dynamic_pks.get(&privacy.try_as_identifier_ref().unwrap()) {
                return v.clone();
            }
        } else {
            self._requested_dynamic_pks
                .insert(stmt.as_mut().unwrap().clone(), BTreeMap::new());
        }

        //Dynamic privacy -> always request key on spot and add to local in args
        let name = format!(
            "{}_{}",
            self._in_name_factory
                .base_name_factory
                .get_new_name(&TypeName::key_type(crypto_params.clone()), false),
            privacy.name()
        );
        let (idf, get_key_stmt) =
            self.request_public_key(&crypto_params, privacy.clone().into(), &name);
        stmt.as_mut()
            .unwrap()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(get_key_stmt.to_ast());
        if let Some(requested_dynamic_pks) = self
            ._requested_dynamic_pks
            .get_mut(&*stmt.as_ref().unwrap())
        {
            requested_dynamic_pks.insert(privacy.clone().try_as_identifier().unwrap(), idf.clone());
        }

        idf.clone()
    }

    pub fn _get_public_key_in_sender_field(
        &mut self,
        stmt: &mut Statement,
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
        let key_t = TypeName::key_type(crypto_params.clone());
        let name = format!(
            "{}_sender",
            self._in_name_factory
                .base_name_factory
                .get_new_name(&key_t, false)
        );
        let key_idf = self._in_name_factory.add_idf(name, key_t.clone(), None);
        let cipher_payload_len = crypto_params.cipher_payload_len();
        let key_expr = KeyLiteralExpr::new(
            if let Some(le) = cipher
                .get_loc_expr(Some((*stmt).to_ast()))
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
            {
                vec![le.index(ExprUnion::I32(cipher_payload_len)).to_expr()]
            } else {
                vec![]
            },
            crypto_params,
        )
        .as_type(AST::TypeName(key_t));
        stmt.statement_base_mut_ref().unwrap().pre_statements.push(
            AssignmentStatementBase::new(
                key_idf.get_loc_expr(None).into(),
                Some(key_expr.to_expr()),
                None,
            )
            .into_ast(),
        );
        key_idf
    }
}
