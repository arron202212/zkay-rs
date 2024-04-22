#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::name_factory::NameFactory;
use crate::name_remapper::CircVarRemapper;
use rccell::{RcCell, WeakCell};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};
use type_check::type_checker::TypeCheckVisitor;
use zkay_ast::analysis::partition_state::PartitionState;
use zkay_ast::ast::{
    get_privacy_expr_from_label, is_instance, is_instances, ASTBaseMutRef, ASTFlatten, ASTType,
    AllExpr, AnnotatedTypeName, AssignmentStatement, AssignmentStatementBase,
    AssignmentStatementBaseMutRef, AssignmentStatementBaseProperty, AssignmentStatementBaseRef,
    Block, BooleanLiteralType, BuiltinFunction, CircuitComputationStatement, CircuitInputStatement,
    ConstructorOrFunctionDefinition, ElementaryTypeName, EncryptionExpression,
    EnterPrivateKeyStatement, ExprUnion, Expression, ExpressionBaseMutRef, ExpressionBaseProperty,
    ExpressionBaseRef, ExpressionStatement, FunctionCallExpr, FunctionCallExprBase,
    FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty, FunctionCallExprBaseRef,
    HybridArgType, HybridArgumentIdf, Identifier, IdentifierBase, IdentifierBaseProperty,
    IdentifierDeclarationBaseProperty, IdentifierExpr, IdentifierExprUnion, IfStatement, IndexExpr,
    IntoAST, IntoExpression, IntoStatement, KeyLiteralExpr, LocationExpr, LocationExprBaseProperty,
    MeExpr, MemberAccessExpr, NamespaceDefinitionBaseProperty, NumberLiteralExpr,
    NumberLiteralType, NumberTypeName, Parameter, ReturnStatement, SimpleStatement,
    StateVariableDeclaration, Statement, StatementBaseMutRef, StatementBaseProperty,
    StatementBaseRef, TupleExpr, TupleOrLocationExpr, TypeName, UserDefinedTypeName,
    VariableDeclaration, VariableDeclarationStatement, AST,
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
    pub fct: RcCell<ConstructorOrFunctionDefinition>,
    pub verifier_contract_filename: Option<String>,
    pub verifier_contract_type: Option<UserDefinedTypeName>,
    // Metadata set later by ZkayContractTransformer
    pub has_return_var: bool,
    // Transformer visitors
    pub _expr_trafo: Option<Box<dyn TransformerVisitorEx>>, //AstTransformerVisitor
    pub _circ_trafo: Option<Box<dyn TransformerVisitorEx>>,
    // List of proof circuit statements (assertions and assignments)
    // WARNING: Never assign to let _phi, always access it using the phi property and only mutate it
    pub _phi: Vec<RcCell<CircuitStatement>>,
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
    pub static_owner_labels: Vec<ASTFlatten>,
    // For each statement, cache the generated variable holding the requested public key of a given
    // not-statically-known identifier, to prevent requesting the same key over and over again
    pub _requested_dynamic_pks:
        BTreeMap<RcCell<Statement>, BTreeMap<RcCell<Identifier>, HybridArgumentIdf>>,
    // The crypto backends for which msg.sender"s secret key must be added to the private circuit inputs
    pub _needed_secret_key: BTreeSet<CryptoParams>,
    // Set of statically known privacy labels (OrderedDict is used to ensure deterministic iteration order)
    pub _global_keys: BTreeSet<(Option<ASTFlatten>, CryptoParams)>,
    // List of all (non-transitive) calls in let fct"s body to functions which require verification, in AST visiting order
    // This is internally used to compute transitive in/out/privin sizes, but may also be useful when implementing a new
    // circuit generator backend.
    pub function_calls_with_verification: Vec<RcCell<FunctionCallExpr>>,
    // Set (with deterministic order) of all functions which this circuit transitively calls.
    pub transitively_called_functions: BTreeSet<RcCell<ConstructorOrFunctionDefinition>>,
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
    pub fn new(
        fct: RcCell<ConstructorOrFunctionDefinition>,
        static_owner_labels: Vec<ASTFlatten>,
        expr_trafo_constructor: impl FnOnce(&RcCell<Self>) -> Option<Box<dyn TransformerVisitorEx>>,
        circ_trafo_constructor: impl FnOnce(&RcCell<Self>) -> Option<Box<dyn TransformerVisitorEx>>,
        internal_circuit: Option<WeakCell<Self>>,
    ) -> Self
    where
        Self: Sized,
    {
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
            let internal_circuit = internal_circuit.upgrade().unwrap();
            //Inherit metadata from internal function"s circuit helper
            verifier_contract_filename =
                internal_circuit.borrow().verifier_contract_filename.clone();
            verifier_contract_type = internal_circuit.borrow().verifier_contract_type.clone();
            _global_keys = internal_circuit.borrow()._global_keys.clone();

            trans_priv_size = internal_circuit.borrow().priv_in_size_trans();
            trans_in_size = internal_circuit.borrow().in_size_trans();
            trans_out_size = internal_circuit.borrow().out_size_trans();

            _needed_secret_key = internal_circuit.borrow()._needed_secret_key.clone();

            if internal_circuit.borrow().fct.borrow().requires_verification {
                transitively_called_functions = internal_circuit
                    .borrow()
                    .transitively_called_functions
                    .clone();
                transitively_called_functions.insert(internal_circuit.borrow().fct.clone());
            } else {
                assert!(internal_circuit
                    .borrow()
                    .transitively_called_functions
                    .is_empty());
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
        let selfsrc = RcCell::new(selfs.clone());
        selfs._expr_trafo = expr_trafo_constructor(&selfsrc);
        selfs._circ_trafo = circ_trafo_constructor(&selfsrc);
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
    // """
    // Return true if a struct needs to be created in the solidity code to store public data (IO) associated with this circuit.

    // A struct is used instead of plain temporary variables to bypass solidity"s stack limit.
    // """
    pub fn requires_zk_data_struct(&self) -> bool {
        self.out_size() + self.in_size() > 0
    }

    // """Name of the data struct type"""
    pub fn zk_data_struct_name(&self) -> String {
        format!(
            "{}_{}",
            CFG.lock().unwrap().zk_struct_prefix(),
            self.fct.borrow().name()
        )
    }
    // """Total size of all private inputs for this circuit (in //uints)"""
    pub fn priv_in_size_trans(&self) -> i32 {
        self.priv_in_size() + self.trans_priv_size
    }
    // """Size of all private inputs required for self.fct only (without called functions, in #uints)"""
    pub fn priv_in_size(&self) -> i32 {
        self._secret_input_name_factory.size
    }
    // """Total size of all public outputs for this circuit (in //uints)"""
    pub fn out_size_trans(&self) -> i32 {
        self.out_size() + self.trans_out_size
    }
    // """Size of all public outputs required for self.fct only (without called functions, in #uints)"""
    pub fn out_size(&self) -> i32 {
        self._out_name_factory.size
    }
    // """Total size of all public inputs for this circuit (in //uints)"""
    pub fn in_size_trans(&self) -> i32 {
        self.in_size() + self.trans_in_size
    }
    // """Size of all public inputs required for self.fct only (without called functions, in #uints)"""
    pub fn in_size(&self) -> i32 {
        self._in_name_factory.size
    }
    // """All public output HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    pub fn output_idfs(&self) -> Vec<HybridArgumentIdf> {
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
    // """All public input HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    pub fn input_idfs(&self) -> Vec<HybridArgumentIdf> {
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
    // """All private input HybridArgumentIdfs (for self.fct only, w/o called functions)"""
    pub fn sec_idfs(&self) -> Vec<HybridArgumentIdf> {
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
    // """List of abstract circuit statements which defines circuit semantics"""

    pub fn phi(&self) -> Vec<RcCell<CircuitStatement>> {
        self._phi.clone()
    }
    // """Statically known keys required by this circuit"""

    pub fn requested_global_keys(&self) -> BTreeSet<(Option<ASTFlatten>, CryptoParams)> {
        self._global_keys.clone()
    }
    // """Returns names and lengths of all public parameter uint256 arrays which go into the verifier"""
    pub fn public_arg_arrays(&self) -> Vec<(String, i32)> {
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
    // """
    // Return context manager which manages the lifetime of a CircIndentBlock.

    // All statements which are inserted into self.phi during the lifetime of this context manager are automatically wrapped inside
    // a CircIndentBlock statement with the supplied name.
    // """
    pub fn circ_indent_block(&mut self, name: &str) {
        let old_len = self.phi().len();
        // yield
        let mut phi = self.phi();
        let post = phi.split_off(old_len);
        phi.push(RcCell::new(CircuitStatement::CircIndentBlock(
            CircIndentBlock::new(name.to_string(), post),
        )));
        self._phi = phi;
    }
    // """Return a context manager which manages the lifetime of a guard variable."""

    pub fn guarded(&self, guard_idf: HybridArgumentIdf, is_true: bool) {
        CircGuardModification::guarded(&mut self.phi(), guard_idf, is_true);
    }
    // """Return the name of the HybridArgumentIdf which holds the statically known public key for the given privacy label."""

    pub fn get_glob_key_name(label: &ASTFlatten, crypto_params: &CryptoParams) -> String {
        // assert!(is_instances(
        //     &label,
        //     vec![ASTType::MeExpr, ASTType::Identifier]
        // ));

        let name = if let Some(me_expr) = label
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_me_expr_ref()
        {
            me_expr.name.clone()
        } else {
            label
                .try_as_identifier_ref()
                .unwrap()
                .borrow()
                .name()
                .clone()
        };
        format!("glob_key_{}__{}", crypto_params.identifier_name(), name)
    }

    pub fn get_own_secret_key_name(crypto_params: &CryptoParams) -> String {
        format!("glob_sk_{}__me", crypto_params.identifier_name())
    }
    // """ Returns true if the function corresponding to this circuit requires a zk proof verification for correctness """
    pub fn requires_verification(&self) -> bool {
        let req =
            self.in_size_trans() > 0 || self.out_size_trans() > 0 || self.priv_in_size_trans() > 0;
        assert!(req == self.fct.borrow().requires_verification);
        req
    }

    //Solidity-side interface #
    // """
    // Make circuit prove that the encryption of the specified parameter is correct.
    // """
    pub fn ensure_parameter_encryption(
        &mut self,
        insert_loc_stmt: &ASTFlatten,
        param: &RcCell<Parameter>,
    ) {
        assert!(param
            .borrow()
            .identifier_declaration_base
            .annotated_type
            .borrow()
            .is_cipher());

        let plain_idf = self._secret_input_name_factory.add_idf(
            param
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .name()
                .clone(),
            param
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .borrow()
                .zkay_type()
                .type_name
                .as_ref()
                .unwrap(),
            None,
        );
        let name = format!(
            "{}_{}",
            self._in_name_factory.base_name_factory.get_new_name(
                param
                    .borrow()
                    .identifier_declaration_base
                    .annotated_type
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap(),
                false
            ),
            param
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .name()
        );
        let cipher_idf = self._in_name_factory.add_idf(
            name,
            param
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .borrow()
                .type_name
                .as_ref()
                .unwrap(),
            None,
        );
        self._ensure_encryption(
            insert_loc_stmt,
            plain_idf,
            &RcCell::new(Expression::me_expr(None)).into(),
            param
                .borrow()
                .identifier_declaration_base
                .annotated_type
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
                .clone(),
            cipher_idf,
            true,
            false,
        );
    }

    pub fn get_randomness_for_rerand(&mut self, expr: &ASTFlatten) -> RcCell<IdentifierExpr> {
        let idf = self._secret_input_name_factory.get_new_idf(
            &RcCell::new(TypeName::rnd_type(
                expr.try_as_expression_ref()
                    .unwrap()
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
                    .try_as_randomness_ref()
                    .unwrap()
                    .crypto_params
                    .clone(),
            )),
            None,
        );
        RcCell::new(IdentifierExpr::new(
            IdentifierExprUnion::Identifier(RcCell::new(Identifier::HybridArgumentIdf(idf))),
            None,
        ))
    }
    // """
    // Evaluate private expression and return result as a fresh out variable.
    // Roughly corresponds to out() from paper
    // Note: This function has side effects on expr.statement (adds a pre_statement)
    // :param expr: [SIDE EFFECT] The expression which should be evaluated privately
    // :param new_privacy: The circuit output should be encrypted for this owner (or plain if "all")
    // :return: Location expression which references the encrypted circuit result
    // """
    pub fn evaluate_expr_in_circuit(
        &mut self,
        expr: &ASTFlatten,
        new_privacy: &ASTFlatten,
        homomorphism: &String,
    ) -> Option<ASTFlatten> {
        self.circ_indent_block(
            &expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .to_ast()
                .code(),
        );
        self._get_circuit_output_for_private_expression(expr, &new_privacy, &homomorphism)
    }
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
    pub fn evaluate_stmt_in_circuit(&mut self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        let mut astmt = SimpleStatement::ExpressionStatement(ExpressionStatement::new(
            RcCell::new(NumberLiteralExpr::new(0, false).to_expr()).into(),
        ));
        for var in &ast
            .try_as_statement_ref()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .modified_values
        {
            if var.in_scope_at(ast) {
                astmt =
                    SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(
                        AssignmentStatementBase::new(None, None, None),
                    ));
                break;
            }
        }
        astmt.statement_base_mut_ref().before_analysis = ast
            .try_as_statement_ref()
            .unwrap()
            .borrow()
            .statement_base_ref()
            .unwrap()
            .before_analysis()
            .clone();

        //External values written inside statement -> function return values
        let mut ret_params = vec![];
        for var in &ast
            .try_as_statement_ref()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .modified_values
        {
            if var.in_scope_at(ast) {
                //side effect affects location outside statement and has privacy @me
                assert!(ast
                    .try_as_statement_ref()
                    .unwrap()
                    .borrow()
                    .statement_base_ref()
                    .unwrap()
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .same_partition(
                        &var.privacy().unwrap(),
                        &RcCell::new(Expression::me_expr(None)).into()
                    ));
                assert!(is_instances(
                    &var.target().unwrap(),
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
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .zkay_type();
                if !t.type_name.as_ref().unwrap().borrow().is_primitive_type() {
                    unimplemented!(
                        "Reference types inside private if statements are not supported"
                    );
                }
                let ret_t = AnnotatedTypeName::new(
                    t.type_name.clone(),
                    Some(RcCell::new(Expression::me_expr(None)).into()),
                    t.homomorphism,
                ); //t, but @me
                let mut idf = IdentifierExpr::new(
                    IdentifierExprUnion::Identifier(
                        var.target()
                            .unwrap()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .borrow()
                            .idf()
                            .upgrade()
                            .unwrap(),
                    ),
                    Some(RcCell::new(ret_t)),
                );
                idf.location_expr_base.target = var.target().map(|p| p.downgrade());
                let mut ret_param = idf;
                ret_param
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .statement =
                    Some(ASTFlatten::from(RcCell::new(astmt.to_statement())).downgrade());
                ret_params.push(ret_param);
            }
        }

        //Build the imaginary function
        let mut fdef = ConstructorOrFunctionDefinition::new(
            Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(
                String::from("<stmt_fct>"),
            )))),
            Some(vec![]),
            Some(zkay_config::lc_vec_s!["private"]),
            Some(
                ret_params
                    .iter()
                    .map(|ret| {
                        RcCell::new(Parameter::new(
                            vec![],
                            ret.annotated_type.clone().unwrap(),
                            ret.location_expr_base
                                .target
                                .clone()
                                .unwrap()
                                .upgrade()
                                .unwrap()
                                .try_as_identifier_declaration_ref()
                                .unwrap()
                                .borrow()
                                .idf()
                                .upgrade(),
                            None,
                        ))
                    })
                    .collect(),
            ),
            Some(Block::new(
                vec![
                    ast.clone(),
                    RcCell::new(ReturnStatement::new(Some(
                        RcCell::new(
                            TupleExpr::new(
                                ret_params
                                    .iter()
                                    .map(|r| RcCell::new(r.to_expr()).into())
                                    .collect(),
                            )
                            .to_expr(),
                        )
                        .into(),
                    )))
                    .into(),
                ],
                false,
            )),
        );
        fdef.original_body = fdef.body.clone();
        fdef.parent = None; //TODO Statement to ContractDefinition   ast.clone();
        let fdef = RcCell::new(fdef);
        fdef.borrow_mut()
            .body
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .parent = Some(ASTFlatten::from(fdef.clone()).downgrade());
        //inline "Call" to the imaginary function
        let mut idf = IdentifierExpr::new(
            IdentifierExprUnion::String(String::from("<stmt_fct>")),
            None,
        );
        idf.location_expr_base.target_rc = Some(ASTFlatten::from(fdef));
        idf.location_expr_base.target = idf
            .location_expr_base
            .target_rc
            .as_ref()
            .map(|t| t.clone().downgrade());
        let mut fcall = FunctionCallExprBase::new(RcCell::new(idf.to_expr()).into(), vec![], None);
        fcall.expression_base.statement =
            Some(ASTFlatten::from(RcCell::new(astmt.to_statement())).downgrade());
        let mut ret_args = self.inline_function_call_into_circuit(&RcCell::new(fcall).into());
        assert!(ret_args.is_some());
        let mut ret_args = ret_args.unwrap();
        //Move all return values out of the circuit
        let mut ret_args = if !is_instance(&ret_args, ASTType::TupleExpr) {
            RcCell::new(TupleExpr::new(vec![ret_args.into()]).into_expr()).into()
        } else {
            ret_args
        };
        for ret_arg in ret_args
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .borrow_mut()
            .try_as_tuple_expr_mut()
            .unwrap()
            .elements
            .iter_mut()
        {
            ret_arg
                .try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement = Some(ASTFlatten::from(RcCell::new(astmt.to_statement())).downgrade());
        }
        let ret_arg_outs: Vec<_> = ret_params
            .iter()
            .zip(
                ret_args
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .try_as_tuple_expr_mut()
                    .unwrap()
                    .elements
                    .iter_mut(),
            )
            .filter_map(|(ret_param, ret_arg)| {
                self._get_circuit_output_for_private_expression(
                    ret_arg,
                    &RcCell::new(Expression::me_expr(None)).into(),
                    &ret_param
                        .annotated_type
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .homomorphism,
                )
            })
            .collect();

        //Create assignment statement
        if !ret_params.is_empty() {
            astmt
                .try_as_assignment_statement_mut()
                .unwrap()
                .assignment_statement_base_mut_ref()
                .lhs = Some(
                RcCell::new(
                    TupleExpr::new(
                        ret_params
                            .iter()
                            .map(|r| RcCell::new(r.to_expr()).into())
                            .collect(),
                    )
                    .into_expr(),
                )
                .into(),
            );
            astmt
                .try_as_assignment_statement_mut()
                .unwrap()
                .assignment_statement_base_mut_ref()
                .rhs = Some(RcCell::new(TupleExpr::new(ret_arg_outs).to_expr()).into());
        } else {
            assert!(is_instance(&astmt, ASTType::ExpressionStatement));
        }
        Some(RcCell::new(astmt).into())
    }
    pub fn invalidate_idf(&mut self, target_idf: &RcCell<Identifier>) {
        if self._remapper.0.is_remapped(target_idf) {
            self._remapper.0.reset_key(target_idf);
        }
    }
    // """
    // Include public function call to a function which requires verification in this circuit.

    // :param ast: The function call to include, target function must require verification
    // """
    pub fn call_function(&mut self, ast: &ASTFlatten) {
        assert!(
            ast.try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification
        );
        self.function_calls_with_verification
            .push(ast.try_as_function_call_expr_ref().unwrap().clone());
        self._phi
            .push(RcCell::new(CircuitStatement::CircCall(CircCall::new(
                ast.try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_namespace_definition_ref()
                    .unwrap()
                    .borrow()
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .clone(),
            ))));
    }

    // """
    // Request key for the address corresponding to plabel from pki infrastructure and add it to the public circuit inputs.

    // :param plabel: privacy label for which to request key
    // :param name: name to use for the HybridArgumentIdf holding the key
    // :return: HybridArgumentIdf containing the requested key and an AssignmentStatement which assigns the key request to the idf location
    // """
    pub fn request_public_key(
        &mut self,
        crypto_params: &CryptoParams,
        plabel: Option<ASTFlatten>,
        name: &str,
    ) -> (HybridArgumentIdf, AssignmentStatement) //(Identifier,CircuitInputStatement)
    {
        let idf = self._in_name_factory.add_idf(
            name.to_owned(),
            &RcCell::new(TypeName::key_type(crypto_params.clone())),
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
        let le = idf
            .get_loc_expr(None)
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .assign(
                RcCell::new(
                    LocationExpr::IdentifierExpr(pki)
                        .call(
                            IdentifierExprUnion::String(String::from("getPk")),
                            self._expr_trafo
                                .as_ref()
                                .unwrap()
                                .visit(&privacy_label_expr.clone().into())
                                .map_or(vec![], |expr| vec![expr]),
                        )
                        .to_expr(),
                )
                .into(),
            );
        (idf, le)
    }

    pub fn request_private_key(&mut self, crypto_params: &CryptoParams) -> Vec<ASTFlatten> {
        assert!(
            self._needed_secret_key.contains(&crypto_params)
                || self
                    .fct
                    .borrow()
                    .parameters
                    .iter()
                    .filter_map(|p| {
                        if p.borrow()
                            .identifier_declaration_base
                            .annotated_type
                            .borrow()
                            .is_cipher()
                        {
                            Some(
                                p.borrow()
                                    .identifier_declaration_base
                                    .annotated_type
                                    .borrow()
                                    .type_name
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
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
                    })
                    .collect::<Vec<_>>()
                    .contains(crypto_params)
        );
        let key_name = Self::get_own_secret_key_name(crypto_params);
        self._secret_input_name_factory.add_idf(
            key_name,
            &RcCell::new(TypeName::key_type(crypto_params.clone())),
            None,
        );
        vec![RcCell::new(EnterPrivateKeyStatement::new(crypto_params.clone())).into()]
    }

    //Circuit-side interface #
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
    pub fn add_to_circuit_inputs(&mut self, expr: &ASTFlatten) -> HybridArgumentIdf {
        let privacy = if expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            expr.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
        } else {
            Some(RcCell::new(Expression::all_expr()).into())
        };
        let is_public = privacy == Some(RcCell::new(Expression::all_expr()).into());

        let expr_text = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .to_ast()
            .code();
        let input_expr = self
            ._expr_trafo
            .as_ref()
            .unwrap()
            .visit(&expr.clone().into());
        let t = input_expr
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
            .clone();
        let mut locally_decrypted_idf = None;

        //If expression has literal type -> evaluate it inside the circuit (constant folding will be used)
        //rather than introducing an unnecessary public circuit input (expensive)
        if is_instance(t.as_ref().unwrap(), ASTType::BooleanLiteralType) {
            return self
                ._evaluate_private_expression(
                    input_expr.as_ref().unwrap(),
                    &t.as_ref()
                        .unwrap()
                        .borrow()
                        .try_as_elementary_type_name_ref()
                        .unwrap()
                        .try_as_boolean_literal_type_ref()
                        .unwrap()
                        .value()
                        .to_string(),
                )
                .unwrap();
        } else if is_instance(t.as_ref().unwrap(), ASTType::NumberLiteralType) {
            return self
                ._evaluate_private_expression(
                    input_expr.as_ref().unwrap(),
                    &t.as_ref()
                        .unwrap()
                        .borrow()
                        .try_as_elementary_type_name_ref()
                        .unwrap()
                        .try_as_number_type_name_ref()
                        .unwrap()
                        .try_as_number_literal_type_ref()
                        .unwrap()
                        .value()
                        .to_string(),
                )
                .unwrap();
        }

        let mut t_suffix = String::new();
        if is_instance(expr, ASTType::IdentifierExpr)
        //Look in cache before doing expensive move-in
        {
            if self._remapper.0.is_remapped(
                &expr
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .upgrade()
                    .unwrap(),
            ) {
                return self._remapper.0.get_current(
                    &expr
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .borrow()
                        .idf()
                        .upgrade()
                        .unwrap(),
                    None,
                );
            }

            t_suffix = format!(
                "_{}",
                expr.try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .name()
            );
        }

        //Generate circuit inputs
        let (return_idf, input_idf) = if is_public {
            let tname = format!(
                "{}{t_suffix}",
                self._in_name_factory.base_name_factory.get_new_name(
                    &expr
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
                    false
                )
            );
            let input_idf = self._in_name_factory.add_idf(
                tname,
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap(),
                None,
            );
            let return_idf = input_idf.clone();
            self._phi.push(RcCell::new(CircuitStatement::CircComment(
                CircComment::new(format!("{} = {expr_text}", input_idf.identifier_base.name)),
            )));
            (return_idf, input_idf)
        } else
        //Encrypted inputs need to be decrypted inside the circuit (i.e. add plain as private input and prove encryption)
        {
            let tname = format!(
                "{}{t_suffix}",
                self._secret_input_name_factory
                    .base_name_factory
                    .get_new_name(
                        &*expr
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
                        false
                    )
            );
            let locally_decrypted_idf = self._secret_input_name_factory.add_idf(
                tname,
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap(),
                None,
            );
            let return_idf = locally_decrypted_idf.clone();
            let cipher_t = RcCell::new(TypeName::cipher_type(
                input_expr
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .clone(),
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism
                    .clone(),
            ));
            let tname = format!(
                "{}{t_suffix}",
                self._in_name_factory
                    .base_name_factory
                    .get_new_name(&cipher_t, false)
            );
            let input_idf = self._in_name_factory.add_idf(
                tname,
                &cipher_t,
                Some(
                    &RcCell::new(
                        IdentifierExpr::new(
                            IdentifierExprUnion::Identifier(RcCell::new(
                                Identifier::HybridArgumentIdf(locally_decrypted_idf),
                            )),
                            None,
                        )
                        .to_expr(),
                    )
                    .into(),
                ),
            );
            (return_idf, input_idf)
        };

        //Add a CircuitInputStatement to the solidity code, which looks like a normal assignment statement,
        //but also signals the offchain simulator to perform decryption if necessary
        expr.try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .statement
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(
                RcCell::new(CircuitInputStatement::new(
                    input_idf.get_loc_expr(None),
                    input_expr.unwrap(),
                    None,
                ))
                .into(),
            );

        if !is_public {
            //Check if the secret plain input corresponds to the decrypted cipher value
            let crypto_params = CFG.lock().unwrap().user_config.get_crypto_params(
                &expr
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism,
            );
            self._phi.push(RcCell::new(CircuitStatement::CircComment(
                CircComment::new(format!(
                    "{:?} = dec({expr_text}) [{}]",
                    locally_decrypted_idf.clone().unwrap(),
                    input_idf.identifier_base.name
                )),
            )));
            let mut statement = expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .expression_base_ref()
                .statement
                .clone()
                .unwrap();
            self._ensure_encryption(
                &statement.clone().upgrade().unwrap(),
                locally_decrypted_idf.clone().unwrap(),
                &RcCell::new(Expression::me_expr(None)).into(),
                crypto_params,
                input_idf,
                false,
                true,
            );
            expr.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement = Some(statement);
        }

        //Cache circuit input for later reuse if possible
        if CFG.lock().unwrap().user_config.opt_cache_circuit_inputs()
            && is_instance(expr, ASTType::IdentifierExpr)
        //TODO: What if a homomorphic variable gets used as both a plain variable and as a ciphertext?
        //      This works for now because we never perform homomorphic operations on variables we can decrypt.
        {
            self._remapper.0.remap(
                &expr
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .upgrade()
                    .unwrap(),
                return_idf.clone(),
            );
        }

        return_idf
    }
    // """
    // Get location expression for the most recently assigned value of idf according to the SSA simulation.

    // :param idf: Identifier expression to lookup
    // :return: Either idf itself (not currently remapped)
    //          or a loc expr for the HybridArgumentIdf which references the most recent value of idf
    // """
    pub fn get_remapped_idf_expr(&self, idf: ASTFlatten) -> ASTFlatten {
        let target = idf
            .try_as_identifier_expr_ref()
            .unwrap()
            .borrow()
            .location_expr_base
            .target
            .clone()
            .map(|t| t.upgrade())
            .flatten();
        assert!(target.is_some());
        assert!(!is_instance(
            idf.try_as_identifier_expr_ref()
                .unwrap()
                .borrow()
                .idf
                .as_ref()
                .unwrap(),
            ASTType::HybridArgumentIdf
        ));
        if self._remapper.0.is_remapped(
            &target
                .as_ref()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .borrow()
                .idf()
                .upgrade()
                .unwrap(),
        ) {
            let remapped_idf = self._remapper.0.get_current(
                &target
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .upgrade()
                    .unwrap(),
                None,
            );
            remapped_idf
                .get_idf_expr(
                    idf.try_as_identifier_expr_ref()
                        .unwrap()
                        .borrow()
                        .location_expr_base
                        .tuple_or_location_expr_base
                        .expression_base
                        .ast_base
                        .borrow()
                        .parent
                        .clone()
                        .unwrap()
                        .upgrade()
                        .as_ref(),
                )
                .as_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .borrow()
                .as_type(
                    &idf.try_as_identifier_expr_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type
                        .clone()
                        .unwrap()
                        .into(),
                )
        } else {
            idf.clone()
        }
    }
    // """
    // Store expr in a new version of orig_idf (for SSA).

    // :param orig_idf: the identifier which should be updated with a new value
    // :param expr: the updated value
    // :param is_local: whether orig_idf refers to a local variable (as opposed to a state variable)
    // """
    pub fn create_new_idf_version_from_value(
        &mut self,
        orig_idf: &RcCell<Identifier>,
        expr: &ASTFlatten,
    ) {
        let tmp_var = self._create_temp_var(&orig_idf.borrow().name(), expr);
        self._remapper.0.remap(orig_idf, tmp_var);
    }

    // """
    // Inline an entire function call into the current circuit.

    // :param fcall: Function call to inline
    // :return: Expression (1 retval) / TupleExpr (multiple retvals) with return value(s)
    // """
    pub fn inline_function_call_into_circuit(&mut self, fcall: &ASTFlatten) -> Option<ASTFlatten> {
        assert!(
            is_instance(
                fcall
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .func(),
                ASTType::LocationExprBase
            ) && fcall
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .is_some()
        );
        let fdef = fcall
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .func()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .borrow()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .clone();
        //with
        self._remapper.0.remap_scope(Some(
            &fcall
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .body
                .clone()
                .unwrap()
                .into(),
        ));

        //with
        if fcall
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .func()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .borrow()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_namespace_definition()
            .unwrap()
            .borrow()
            .idf()
            .upgrade()
            .unwrap()
            .borrow()
            .name()
            != "<stmt_fct>"
        {
            self.circ_indent_block(&format!(
                "INLINED {}",
                fcall
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .to_ast()
                    .code()
            ));
        }

        //Assign all arguments to temporary circuit variables which are designated as the current version of the parameter idfs
        for (param, arg) in fdef
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .borrow()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .parameters
            .iter()
            .zip(
                fcall
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .args(),
            )
        {
            self._phi.push(RcCell::new(CircuitStatement::CircComment(
                CircComment::new(format!(
                    "ARG {}: {}",
                    param
                        .borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name(),
                    arg.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .to_ast()
                        .code()
                )),
            )));
            // with
            self.circ_indent_block("");
            {
                self.create_new_idf_version_from_value(
                    &param
                        .borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .clone(),
                    arg,
                );
            }
        }

        //Visit the untransformed target function body to include all statements in this circuit
        let inlined_body = fdef
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .borrow()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .original_body
            .clone(); //deep_copy(fdef.original_body, true, true);
        self._circ_trafo
            .as_ref()
            .unwrap()
            .visit(&inlined_body.clone().unwrap().into());

        fcall
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .statement
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .extend(
                inlined_body
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .pre_statements()
                    .clone(),
            );

        //Create TupleExpr with location expressions corresponding to the function return values as elements
        let ret_idfs: Vec<_> = fdef
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .borrow()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .return_var_decls
            .iter()
            .map(|vd| {
                self._remapper.0.get_current(
                    vd.borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap(),
                    None,
                )
            })
            .collect();
        let mut ret = TupleExpr::new(
            ret_idfs
                .iter()
                .map(|idf| {
                    IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(RcCell::new(
                            Identifier::HybridArgumentIdf(idf.clone()),
                        )),
                        None,
                    )
                    .as_type(&idf.t.clone().into())
                })
                .collect(),
        );

        Some(if ret.elements.len() == 1 {
            //Unpack 1-length tuple
            // ret = if let Expression::TupleOrLocationExpr(TupleOrLocationExpr::TupleExpr(ret))=&ret.elements[0]{ret.clone()}else{TupleExpr::default()};
            ret.elements[0].clone()
        } else {
            RcCell::new(ret.into_expr()).into()
        })
    }
    // """Include private assignment statement in this circuit."""

    pub fn add_assignment_to_circuit(&mut self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        self._phi.push(RcCell::new(CircuitStatement::CircComment(
            CircComment::new(
                ast.try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow()
                    .to_ast()
                    .code(),
            ),
        )));
        self._add_assign(
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .lhs()
                .as_ref()
                .unwrap(),
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .assignment_statement_base_ref()
                .rhs
                .as_ref()
                .unwrap(),
        );
        Some(ast.clone())
    }

    pub fn add_var_decl_to_circuit(&mut self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        self._phi.push(RcCell::new(CircuitStatement::CircComment(
            CircComment::new(
                ast.try_as_variable_declaration_statement_ref()
                    .unwrap()
                    .borrow()
                    .to_ast()
                    .code(),
            ),
        )));
        if ast
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .borrow()
            .expr
            .is_none()
        {
            //Default initialization is made explicit for circuit variables

            let t = ast
                .try_as_variable_declaration_statement_ref()
                .unwrap()
                .borrow()
                .variable_declaration
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .borrow()
                .type_name
                .clone();
            assert!(t.as_ref().unwrap().borrow().can_be_private());
            let mut nle = NumberLiteralExpr::new(0, false);
            nle.ast_base_mut_ref().borrow_mut().parent = Some(ast.clone().downgrade());
            nle.literal_expr_base.expression_base.statement = Some(ast.clone().downgrade());
            ast.try_as_variable_declaration_statement_ref()
                .unwrap()
                .borrow_mut()
                .expr = Some(TypeCheckVisitor::implicitly_converted_to(
                &RcCell::new(nle.to_expr()).into(),
                t.as_ref().unwrap(),
            ));
        }
        self.create_new_idf_version_from_value(
            ast.try_as_variable_declaration_statement_ref()
                .unwrap()
                .borrow()
                .variable_declaration
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap(),
            ast.try_as_variable_declaration_statement_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap(),
        );
        Some(ast.clone())
    }

    pub fn add_return_stmt_to_circuit(&mut self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        self._phi.push(RcCell::new(CircuitStatement::CircComment(
            CircComment::new(
                ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .to_ast()
                    .code(),
            ),
        )));
        assert!(ast
            .try_as_return_statement_ref()
            .unwrap()
            .borrow()
            .expr
            .is_some());
        if !is_instance(
            ast.try_as_return_statement_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap(),
            ASTType::TupleExpr,
        ) {
            ast.try_as_return_statement_ref().unwrap().borrow_mut().expr = Some(
                RcCell::new(
                    TupleExpr::new(vec![ast
                        .try_as_return_statement_ref()
                        .unwrap()
                        .borrow()
                        .expr
                        .clone()
                        .unwrap()])
                    .to_expr(),
                )
                .into(),
            );
        }

        for (vd, expr) in ast
            .try_as_return_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base
            .function
            .as_mut()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .return_var_decls
            .iter()
            .zip(
                &ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements,
            )
        {
            //Assign return value to new version of return variable
            self.create_new_idf_version_from_value(
                vd.borrow()
                    .identifier_declaration_base
                    .idf
                    .as_ref()
                    .unwrap(),
                &expr,
            );
        }
        Some(ast.clone())
    }
    // """Include private if statement in this circuit."""
    pub fn add_if_statement_to_circuit(&mut self, ast: &ASTFlatten) -> Option<ASTFlatten> {
        //Handle if branch
        // with
        self._remapper.0.remap_scope(None);
        let mut comment = CircComment::new(format!(
            "if ({})",
            ast.try_as_if_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .to_ast()
                .code()
        ));
        self._phi
            .push(RcCell::new(CircuitStatement::CircComment(comment.clone())));
        let cond = self._evaluate_private_expression(
            &ast.try_as_if_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone(),
            "",
        );
        comment.text += &format!(" [{}]", cond.as_ref().unwrap().identifier_base.name);
        self._circ_trafo.as_ref().unwrap().visitBlock(
            Some(
                ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow()
                    .then_branch
                    .clone()
                    .into(),
            ),
            Some(cond.clone().unwrap()),
            Some(true),
        );
        let then_remap = self._remapper.0.get_state();

        //Bubble up nested pre statements
        let mut ps: Vec<_> = ast
            .try_as_if_statement_ref()
            .unwrap()
            .borrow_mut()
            .then_branch
            .borrow_mut()
            .statement_list_base
            .statement_base
            .pre_statements
            .drain(..)
            .collect();
        ast.try_as_if_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base
            .pre_statements
            .append(&mut ps);
        // ast.then_branch.pre_statements = vec![];

        //Handle else branch
        if ast
            .try_as_if_statement_ref()
            .unwrap()
            .borrow()
            .else_branch
            .is_some()
        {
            self._phi.push(RcCell::new(CircuitStatement::CircComment(
                CircComment::new(format!(
                    "else [{}]",
                    cond.as_ref().unwrap().identifier_base.name
                )),
            )));
            self._circ_trafo.as_ref().unwrap().visitBlock(
                Some(
                    ast.try_as_if_statement_ref()
                        .unwrap()
                        .borrow()
                        .else_branch
                        .as_ref()
                        .unwrap()
                        .clone()
                        .into(),
                ),
                Some(cond.clone().unwrap()),
                Some(false),
            );

            //Bubble up nested pre statements
            let mut ps: Vec<_> = ast
                .try_as_if_statement_ref()
                .unwrap()
                .borrow()
                .else_branch
                .as_ref()
                .unwrap()
                .borrow()
                .statement_list_base
                .statement_base
                .pre_statements
                .clone();
            ast.try_as_if_statement_ref()
                .unwrap()
                .borrow_mut()
                .statement_base
                .pre_statements
                .append(&mut ps);
            // ast.else_branch.pre_statements = vec![];
        }

        //SSA join branches (if both branches write to same external value -> cond assignment to select correct version)
        // with
        self.circ_indent_block(&format!(
            "JOIN [{}]",
            cond.as_ref().unwrap().identifier_base.name
        ));
        let cond_idf_expr = cond.unwrap().get_idf_expr(Some(ast));
        assert!(is_instance(
            cond_idf_expr.as_ref().unwrap(),
            ASTType::IdentifierExpr
        ));
        let mut selfs = RcCell::new(self.clone());
        self._remapper.0.join_branch(
            ast,
            cond_idf_expr.as_ref().unwrap(),
            then_remap,
            // |s: String, e: Expression| -> HybridArgumentIdf { selfs._create_temp_var(&s, e) },
            &selfs,
        );
        Some(ast.clone())
    }
    pub fn add_block_to_circuit(
        &mut self,
        ast: &ASTFlatten,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> Option<ASTFlatten> {
        assert!(ast
            .try_as_block_ref()
            .unwrap()
            .borrow()
            .statement_list_base
            .statement_base
            .ast_base
            .borrow()
            .parent
            .is_some());
        let is_already_scoped = is_instances(
            &ast.try_as_block_ref()
                .unwrap()
                .borrow()
                .statement_list_base
                .statement_base
                .ast_base
                .borrow()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap(),
            vec![
                ASTType::ConstructorOrFunctionDefinition,
                ASTType::IfStatement,
            ],
        );
        self._phi.push(RcCell::new(CircuitStatement::CircComment(
            CircComment::new(String::from("{")),
        )));
        // with
        self.circ_indent_block("");
        // with
        if let Some(guard_cond) = guard_cond {
            self.guarded(guard_cond, guard_val.unwrap());
        }
        //with
        if !is_already_scoped {
            self._remapper.0.remap_scope(Some(ast));
        }
        let mut statements = vec![];
        for stmt in ast
            .try_as_block_ref()
            .unwrap()
            .borrow_mut()
            .statement_list_base
            .statements
            .iter_mut()
        {
            if is_instance(stmt, ASTType::StatementBase) {
                self._circ_trafo
                    .as_ref()
                    .unwrap()
                    .visit(&stmt.clone().into());
                //Bubble up nested pre statements
                statements.append(
                    &mut stmt
                        .try_as_statement_mut()
                        .unwrap()
                        .borrow_mut()
                        .statement_base_mut_ref()
                        .unwrap()
                        .pre_statements
                        .drain(..)
                        .collect::<Vec<_>>(),
                );
            }
            // stmt.pre_statements = vec![];
        }
        ast.try_as_block_ref()
            .unwrap()
            .borrow_mut()
            .statement_list_base
            .statement_base
            .pre_statements
            .append(&mut statements);

        self._phi.push(RcCell::new(CircuitStatement::CircComment(
            CircComment::new(String::from("}")),
        )));
        Some(ast.clone())
    }

    //Internal functionality #
    // """
    // If privacy is equivalent to a static privacy label -> Return the corresponding static label, otherwise itself.

    // :param analysis: analysis state at the statement where expression with the given privacy occurs
    // :param privacy: original privacy label
    // """
    pub fn _get_canonical_privacy_label(
        &self,
        analysis: &PartitionState<ASTFlatten>,
        privacy: &ASTFlatten,
    ) -> ASTFlatten {
        for owner in &self.static_owner_labels {
            if analysis.same_partition(owner, privacy) {
                return owner.clone();
            }
        }
        privacy.clone()
    }
    // """Assign expression to a fresh temporary circuit variable."""
    pub fn _create_temp_var(&mut self, tag: &str, expr: &ASTFlatten) -> HybridArgumentIdf {
        self._evaluate_private_expression(expr, &format!("_{tag}"))
            .unwrap()
    }
    // """
    // Simulate an assignment of rhs to lhs inside the circuit.

    // :param lhs: destination
    // :param rhs: source
    // """
    pub fn _add_assign(&mut self, lhs: &ASTFlatten, rhs: &ASTFlatten) {
        if is_instance(lhs, ASTType::IdentifierExpr) {
            //for now no ref types
            assert!(lhs
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .is_some());
            self.create_new_idf_version_from_value(
                &lhs.try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .upgrade()
                    .unwrap(),
                &rhs,
            );
        } else if is_instance(lhs, ASTType::IndexExpr) {
            // raise NotImplementedError()
            unimplemented!();
        } else {
            assert!(is_instance(lhs, ASTType::TupleExpr));
            if is_instance(&*rhs, ASTType::FunctionCallExprBase) {
                if let Some(expr) = self
                    ._circ_trafo
                    .as_ref()
                    .unwrap()
                    .visit(&rhs.clone().into())
                {
                    *rhs.try_as_function_call_expr_ref().unwrap().borrow_mut() = expr
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow()
                        .clone();
                }
            }
            assert!(
                is_instance(rhs, ASTType::TupleExpr)
                    && lhs
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_tuple_expr_ref()
                        .unwrap()
                        .elements
                        .len()
                        == rhs
                            .try_as_tuple_or_location_expr_ref()
                            .unwrap()
                            .borrow()
                            .try_as_tuple_expr_ref()
                            .unwrap()
                            .elements
                            .len()
            );
            for (e_l, e_r) in lhs
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_expr_ref()
                .unwrap()
                .elements
                .iter()
                .zip(
                    &rhs.try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_tuple_expr_ref()
                        .unwrap()
                        .elements,
                )
            {
                self._add_assign(e_l, &e_r);
            }
        }
    }
    // """
    // Add evaluation of expr to the circuit and return the output HybridArgumentIdf corresponding to the evaluation result.

    // Note: has side effects on expr.statement (adds pre_statement)

    // :param expr: [SIDE EFFECT] expression to evaluate
    // :param new_privacy: result owner (determines encryption key)
    // :return: HybridArgumentIdf which references the circuit output containing the result of expr
    // """
    pub fn _get_circuit_output_for_private_expression(
        &mut self,
        expr: &ASTFlatten,
        new_privacy: &ASTFlatten,
        homomorphism: &String,
    ) -> Option<ASTFlatten> {
        let is_circ_val = is_instance(expr, ASTType::IdentifierExpr)
            && is_instance(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .idf
                    .as_ref()
                    .unwrap(),
                ASTType::HybridArgumentIdf,
            )
            && expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .arg_type
                != HybridArgType::PubContractVal;
        let is_hom_comp = is_instance(expr, ASTType::FunctionCallExprBase)
            && is_instance(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::BuiltinFunction,
            )
            && expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .homomorphism
                != Homomorphism::non_homomorphic();
        if is_hom_comp {
            //Treat a homomorphic operation as a privately evaluated operation on (public) ciphertexts

            expr.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_function_call_expr_mut()
                .unwrap()
                .expression_base_mut_ref()
                .annotated_type = Some(AnnotatedTypeName::cipher_type(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .clone(),
                Some(homomorphism.clone()),
            ));
        }

        let priv_result_idf = if is_circ_val
            || expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_private()
            || expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .evaluate_privately()
        {
            self._evaluate_private_expression(expr, "").unwrap()
        } else {
            //For public expressions which should not be evaluated in private, only the result is moved into the circuit

            self.add_to_circuit_inputs(expr)
        };
        let private_expr = priv_result_idf.get_idf_expr(None);

        let mut t_suffix = String::new();
        if is_instance(expr, ASTType::IdentifierExpr) && !is_circ_val {
            t_suffix += &format!(
                "_{}",
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .name()
            );
        }

        let (out_var, new_out_param) = if is_instance(new_privacy, ASTType::AllExpr)
            || expr
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
                .borrow()
                .is_cipher()
        {
            //If the result is public, add an equality constraint to ensure that the user supplied public output
            //is equal to the circuit evaluation result
            let tname = format!(
                "{}{t_suffix}",
                self._out_name_factory.base_name_factory.get_new_name(
                    expr.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap(),
                    false
                )
            );
            let new_out_param = self._out_name_factory.add_idf(
                tname,
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap(),
                private_expr.as_ref(),
            );
            self._phi
                .push(RcCell::new(CircuitStatement::CircEqConstraint(
                    CircEqConstraint::new(priv_result_idf, new_out_param.clone()),
                )));
            (
                new_out_param
                    .clone()
                    .get_loc_expr(None)
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .explicitly_converted(
                        expr.try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .type_name
                            .as_ref()
                            .unwrap(),
                    ),
                new_out_param,
            )
        } else {
            //If the result is encrypted, add an encryption constraint to ensure that the user supplied encrypted output
            //is equal to the correctly encrypted circuit evaluation result
            let new_privacy = self._get_canonical_privacy_label(
                &expr
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .analysis()
                    .unwrap(),
                new_privacy,
            );
            let privacy_label_expr = get_privacy_expr_from_label(new_privacy.clone());
            let cipher_t = RcCell::new(TypeName::cipher_type(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .clone(),
                homomorphism.clone(),
            ));
            let tname = format!(
                "{}{t_suffix}",
                self._out_name_factory
                    .base_name_factory
                    .get_new_name(&cipher_t, false)
            );
            let enc_expr = EncryptionExpression::new(
                private_expr.clone().unwrap(),
                privacy_label_expr.clone(),
                Some(homomorphism.clone()),
            );
            let new_out_param = self._out_name_factory.add_idf(
                tname,
                &cipher_t,
                Some(&RcCell::new(enc_expr.to_expr()).into()),
            );
            let crypto_params = CFG
                .lock()
                .unwrap()
                .user_config
                .get_crypto_params(homomorphism);
            self._ensure_encryption(
                &expr
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .expression_base_ref()
                    .statement
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
                priv_result_idf,
                &new_privacy,
                crypto_params,
                new_out_param.clone(),
                false,
                false,
            );
            (new_out_param.get_loc_expr(None).into(), new_out_param)
        };

        //Add an invisible CircuitComputationStatement to the solidity code, which signals the offchain simulator,
        //that the value the contained out variable must be computed at this point by simulating expression evaluation
        expr.try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .statement
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_statement_mut()
            .unwrap()
            .borrow_mut()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(RcCell::new(CircuitComputationStatement::new(new_out_param)).into());
        if is_instance(&out_var, ASTType::LocationExprBase) {
            Some(out_var)
        } else {
            None
        }
    }
    // """
    // Evaluate expr in the circuit (if not already done) and store result in a new temporary circuit variable.

    // :param expr: expression to evaluate
    // :param tmp_idf_suffix: name suffix for the new temporary circuit variable
    // :return: temporary circuit variable HybridArgumentIdf which refers to the transformed circuit expression
    // """
    pub fn _evaluate_private_expression(
        &mut self,
        expr: &ASTFlatten,
        tmp_idf_suffix: &str,
    ) -> Option<HybridArgumentIdf> {
        assert!(
            !(is_instance(expr, ASTType::MemberAccessExpr)
                && is_instance(
                    &*expr
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .clone(),
                    ASTType::HybridArgumentIdf
                ))
        );
        if is_instance(expr, ASTType::IdentifierExpr)
            && is_instance(
                &*expr
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow(),
                ASTType::HybridArgumentIdf,
            )
            && expr
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .arg_type
                != HybridArgType::PubContractVal
        //Already evaluated in circuit
        {
            return expr
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .borrow()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .clone()
                .try_as_hybrid_argument_idf();
        }

        let priv_expr = self._circ_trafo.as_ref().unwrap().visit(expr);
        let tname = format!(
            "{}{tmp_idf_suffix}",
            self._circ_temp_name_factory.base_name_factory.get_new_name(
                &*priv_expr
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
                false
            )
        );
        let tmp_circ_var_idf = self._circ_temp_name_factory.add_idf(
            tname,
            priv_expr
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
            priv_expr.as_ref(),
        );
        let stmt = CircVarDecl::new(
            tmp_circ_var_idf.clone(),
            priv_expr.unwrap().try_as_expression_ref().unwrap().clone(),
        );
        self._phi
            .push(RcCell::new(CircuitStatement::CircVarDecl(stmt)));
        Some(tmp_circ_var_idf)
    }
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
    pub fn _ensure_encryption(
        &mut self,
        stmt: &ASTFlatten,
        plain: HybridArgumentIdf,
        new_privacy: &ASTFlatten,
        crypto_params: CryptoParams,
        cipher: HybridArgumentIdf,
        is_param: bool,
        is_dec: bool,
    ) {
        //Need a different set of keys for hybrid-encryption (ecdh-based) backends
        if crypto_params.is_symmetric_cipher() {
            self._require_secret_key(&crypto_params);
            let my_pk = self._require_public_key_for_label_at(
                Some(stmt),
                &RcCell::new(Expression::me_expr(None)).into(),
                &crypto_params,
            );
            let other_pk = if is_dec {
                self._get_public_key_in_sender_field(stmt, cipher.clone(), crypto_params)
            } else {
                if new_privacy == &RcCell::new(Expression::me_expr(None)).into() {
                    my_pk
                } else {
                    self._require_public_key_for_label_at(Some(stmt), &new_privacy, &crypto_params)
                }
            };

            self._phi.push(RcCell::new(CircuitStatement::CircComment(
                CircComment::new(format!(
                    "{} = enc({}, ecdh({}, my_sk))",
                    cipher.identifier_base.name,
                    plain.identifier_base.name,
                    other_pk.identifier_base.name
                )),
            )));
            self._phi
                .push(RcCell::new(CircuitStatement::CircSymmEncConstraint(
                    CircSymmEncConstraint::new(plain, other_pk, cipher, is_dec),
                )));
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
                &RcCell::new(TypeName::rnd_type(crypto_params.clone())),
                None,
            );
            let pk =
                self._require_public_key_for_label_at(Some(stmt), &new_privacy, &crypto_params);
            if !is_dec {
                self._phi.push(RcCell::new(CircuitStatement::CircComment(
                    CircComment::new(format!(
                        "{} = enc({}, {})",
                        cipher.identifier_base.name,
                        plain.identifier_base.name,
                        pk.identifier_base.name
                    )),
                )));
            }
            self._phi
                .push(RcCell::new(CircuitStatement::CircEncConstraint(
                    CircEncConstraint::new(plain, rnd, pk, cipher, is_dec),
                )));
        }
    }

    pub fn _require_secret_key(&mut self, crypto_params: &CryptoParams) -> HybridArgumentIdf {
        self._needed_secret_key.insert(crypto_params.clone()); //Add to _need_secret_key OrderedDict
        let key_name = Self::get_own_secret_key_name(crypto_params);
        HybridArgumentIdf::new(
            key_name,
            RcCell::new(TypeName::key_type(crypto_params.clone())),
            HybridArgType::PrivCircuitVal,
            None,
        )
    }
    // """
    // Make circuit helper aware, that the key corresponding to privacy is required at stmt.

    // If privacy is not a statically known label, the key is requested on spot.
    // Otherwise the label is added to the global key set.
    // The keys in that set are requested only once at the start of the external wrapper function, to improve efficiency.
    // Note: This function has side effects on stmt (adds a pre_statement)
    // :return: HybridArgumentIdf which references the key
    // """
    pub fn _require_public_key_for_label_at(
        &mut self,
        stmt: Option<&ASTFlatten>,
        privacy: &ASTFlatten,
        crypto_params: &CryptoParams,
    ) -> HybridArgumentIdf {
        //Statically known privacy -> keep track (all global keys will be requested only once)
        if self.static_owner_labels.contains(&privacy) {
            self._global_keys
                .insert((privacy.clone().into(), crypto_params.clone()));
            return HybridArgumentIdf::new(
                Self::get_glob_key_name(&privacy, crypto_params),
                RcCell::new(TypeName::key_type(crypto_params.clone())),
                HybridArgType::PubCircuitArg,
                None,
            );
        }
        assert!(
            stmt.is_none(),
            "stmt cannot be None if privacy is not guaranteed to be statically known"
        );

        //privacy cannot be MeExpr (is in _static_owner_labels) or AllExpr (has no public key)
        assert!(is_instance(privacy, ASTType::IdentifierBase));

        if let Some(requested_dynamic_pks) = self
            ._requested_dynamic_pks
            .get(stmt.as_ref().unwrap().try_as_statement_ref().unwrap())
        {
            if let Some(v) = requested_dynamic_pks.get(&privacy.try_as_identifier_ref().unwrap()) {
                return v.clone();
            }
        } else {
            self._requested_dynamic_pks.insert(
                stmt.as_ref()
                    .unwrap()
                    .try_as_statement_ref()
                    .unwrap()
                    .clone(),
                BTreeMap::new(),
            );
        }

        //Dynamic privacy -> always request key on spot and add to local in args
        let name = format!(
            "{}_{}",
            self._in_name_factory.base_name_factory.get_new_name(
                &RcCell::new(TypeName::key_type(crypto_params.clone())),
                false
            ),
            privacy.try_as_identifier_ref().unwrap().borrow().name()
        );
        let (idf, get_key_stmt) =
            self.request_public_key(&crypto_params, privacy.clone().into(), &name);
        stmt.as_ref()
            .unwrap()
            .try_as_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(RcCell::new(get_key_stmt).into());
        if let Some(requested_dynamic_pks) = self
            ._requested_dynamic_pks
            .get_mut(stmt.as_ref().unwrap().try_as_statement_ref().unwrap())
        {
            requested_dynamic_pks.insert(privacy.clone().try_as_identifier().unwrap(), idf.clone());
        }

        idf.clone()
    }
    // """
    // Ensure the circuit has access to the public key stored in cipher"s sender field.

    // Note: This function has side effects on stmt [adds a pre-statement]

    // :param stmt [SIDE EFFECT]: statement in which this private expression occurs
    // :param cipher: HybridArgumentIdf which references the cipher value
    // :return: HybridArgumentIdf which references the key in cipher"s sender field (or 0 if none)
    // """
    pub fn _get_public_key_in_sender_field(
        &mut self,
        stmt: &ASTFlatten,
        cipher: HybridArgumentIdf,
        crypto_params: CryptoParams,
    ) -> HybridArgumentIdf {
        let key_t = RcCell::new(TypeName::key_type(crypto_params.clone()));
        let name = format!(
            "{}_sender",
            self._in_name_factory
                .base_name_factory
                .get_new_name(&key_t, false)
        );
        let key_idf = self._in_name_factory.add_idf(name, &key_t, None);
        let cipher_payload_len = crypto_params.cipher_payload_len();
        let key_expr = KeyLiteralExpr::new(
            if let Some(le) = cipher
                .get_loc_expr(Some(stmt))
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
            {
                vec![le.index(ExprUnion::I32(cipher_payload_len))]
            } else {
                vec![]
            },
            crypto_params,
        )
        .as_type(&key_t.clone().into());
        stmt.try_as_statement_ref()
            .unwrap()
            .borrow_mut()
            .statement_base_mut_ref()
            .unwrap()
            .pre_statements
            .push(
                RcCell::new(AssignmentStatementBase::new(
                    Some(RcCell::new(key_idf.clone()).into()),
                    Some(key_expr),
                    None,
                ))
                .into(),
            );
        key_idf
    }
}
