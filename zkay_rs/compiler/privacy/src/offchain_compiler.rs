// // from contextlib::contextmanager
// // from datetime::datetime
// use  textwrap::dedent
// // from typing::Dict, List, Optional, ContextManager, Set

// use circuit_generation::circuit_helper::{CircuitHelper, HybridArgumentIdf};
// use zkay_config::config::CFG;
// use zkay_utils::multiline_formatter::MultiLineFormatter;
// use zkay_ast::ast::{ContractDefinition, IntTypeName, SourceUnit, ConstructorOrFunctionDefinition,
//     indent, FunctionCallExpr, IdentifierExpr, BuiltinFunction,
//     StateVariableDeclaration, MemberAccessExpr, IndexExpr, Parameter, TypeName, AnnotatedTypeName, identifier::Identifier,
//     ReturnStatement, EncryptionExpression, MeExpr, Expression, CipherText, Array,
//     AddressTypeName, StructTypeName, HybridArgType, CircuitInputStatement, AddressPayableTypeName,
//     CircuitComputationStatement, VariableDeclaration, Block, KeyLiteralExpr, VariableDeclarationStatement, LocationExpr,
//     PrimitiveCastExpr, EnumDefinition, EnumTypeName, UintTypeName,
//     StatementList, StructDefinition, NumberTypeName, EnterPrivateKeyStatement, ArrayLiteralExpr, NumberLiteralExpr,
//     BoolTypeName};
// use zkay_ast::homomorphism::Homomorphism;
// use zkay_ast::visitors::python_visitor::PythonCodeVisitor;

// pub fn api(name: &str, invoker: &str = "self") -> String
//    { use zkay_transaction::offchain::ApiWrapper;
//     assert! (name in dir(ApiWrapper));
//      format!("{invoker}.api.{name}")}

// PRIV_VALUES_NAME = f"{cfg.reserved_name_prefix}priv"
// IS_EXTERNAL_CALL = f"{cfg.reserved_name_prefix}is_ext"

// SCALAR_FIELD_NAME = "bn128_scalar_field"

// class PythonOffchainVisitor(PythonCodeVisitor)
//     """
//     This visitor generates python code which is able to deploy, connect to and issue transactions for the specified contract.

//     The generated code includes both a class corresponding to the contract, as well as a main function for interactive use.

//     The class has the following two static methods

//     * deploy: Compile all necessary contracts (main contract + libraries), deploy them onto a test chain and return a contract handle.
//     * connect: Get a handle for an already deployed contract (by specifying the on-chain address of the contract). This method automatically verifies the integrity of the remote contract.

//     If the visited AST contains only a single contract, global deploy and connect functions for that contract are also added to the python
//     code.

//     For every zkay function, the class has a corresponding instance method with matching name and (untransformed) signature.
//     To issue a zkay transaction, simply call one of these functions.
//     All private parameters will be encrypted automatically. The function will then simulate solidity execution and circuit computations
//     to obtain all required public circuit inputs. Finally it automatically generates the zero knowledge proof and issues a
//     transformed transaction (encrypted arguments, additional circuit output and proof arguments added).
//     If a require statement fails during simulation, a RequireException is raised.
//     When a state variable is read before it is written in a transaction, its initial value is pulled from the blockchain.
//     Required foreign public keys are also downloaded from the PKI contract on the block chain.

//     The main function simply loads the zkay configuration from the circuit"s manifest, generates encryption keys if necessary
//     and enters an interactive python shell.
//     """

//     pub fn __init__(self, circuits: List[CircuitHelper])
//         super().__init__(False)
//         self.circuits: Dict[ConstructorOrFunctionDefinition, CircuitHelper] = {cg.fct: cg for cg in circuits}

//         self.current_f: Optional[ConstructorOrFunctionDefinition] = None
//         self.current_params: Optional[List[Parameter]] = None
//         self.current_circ: Optional[CircuitHelper] = None
//         self.current_index: List[Expression] = []
//         self.current_index_t: Optional[AnnotatedTypeName] = None

//         self.inside_circuit: bool = False
//         self.flatten_hybrid_args: bool = False

//     @property
//     pub fn _get_forbidden_words(self) -> Set[&str]
//         return super()._get_forbidden_words.union({kw for kw in [
//             # predefined objects
//             "connect", "deploy", "me", "wei_amount",

//             # base class member variables
//             "api", "locals", "state",

//             # base class functions
//             "_scope", "_function_ctx", "default_address", "initialize_keys_for", "use_config_from_manifest", "create_dummy_accounts",

//             # Globals
//             "os", "IntEnum", "Dict", "List", "Tuple", "Optional", "Union", "Any",
//             "my_logging", "CipherValue", "AddressValue", "RandomnessValue", "PublicKeyValue",
//             "ContractSimulator", "RequireException", "help", "annotations"
//         ]})

//     pub fn _get_type_constr(self, t: TypeName)
//         if isinstance(t, BoolTypeName)
//             constr = "bool"
//         elif isinstance(t, EnumTypeName)
//             constr = self.visit_list(t.target.qualified_name, sep=".")
//         elif isinstance(t, NumberTypeName)
//             if self.inside_circuit and t.elem_bitwidth == 256
//                 constr = f"uint"
//             else
//                 constr = f"int{t.elem_bitwidth}"
//                 if not t.signed
//                     constr = f"u{constr}"
//         elif isinstance(t, (AddressTypeName, AddressPayableTypeName))
//             constr = "AddressValue"
//         else
//             raise NotImplementedError(f"No python constructor for type {t}")
//         return constr

//     pub fn get_constructor_args_and_params(self, ast: ContractDefinition)
//         if not ast.constructor_definitions
//             return "", ""
//         with self.circuit_ctx(ast.constructor_definitions[0])
//             a, p = "", ""
//             for param in self.current_params
//                 a += f"{self.visit(param.idf)}, "
//                 p += f"{self.visit(param)}, "
//             return a, p

//     pub fn visitSourceUnit(self, ast: SourceUnit)
//         contracts = self.visit_list(ast.contracts)
//         is_payable = ast.contracts[0].constructor_definitions and ast.contracts[0].constructor_definitions[0].is_payable
//         val_param = ", wei_amount=0" if is_payable else ""
//         val_arg = ", wei_amount=wei_amount" if is_payable else ""

//         c_name = self.visit(ast.contracts[0].idf)
//         c_args, c_params = self.get_constructor_args_and_params(ast.contracts[0])

//         # Create skeleton with global functions and main method
//         return dedent(f"""\
//         ###########################################
//         ## THIS CODE WAS GENERATED AUTOMATICALLY ##
//         ## Creation Time: {datetime.now().strftime("%H:%M:%S %d-%b-%Y")}   ##
//         ###########################################
//         from __future__::annotations

//        ::os
//         from enum::IntEnum
//         from typing::Dict, List, Tuple, Optional, Union, Any

//         from zkay::my_logging
//         from zkay.transaction.types::CipherValue, AddressValue, RandomnessValue, PublicKeyValue
//         from zkay.transaction.offchain::{SCALAR_FIELD_NAME}, ContractSimulator, RequireException
//         from zkay.transaction.int_casts::*
//         from zkay.transaction.solidity_math::*
//         from zkay.utils.timer::time_measure

//         me = None

//         """) + contracts + (dedent(f"""

//         pub fn deploy({c_params}*, user: Union[None, bytes, &str] = None{val_param}) -> {c_name}
//             user = me if user is None else user
//             return {c_name}.deploy({c_args}user=user{val_arg})

//         pub fn connect(address: Union[bytes, &str], user: Union[None, bytes, &str] = None) -> {c_name}
//             user = me if user is None else user
//             return {c_name}.connect(address, user=user)

//         pub fn create_dummy_accounts(count: int) -> Union[&str, Tuple[&str, ...]]
//             return ContractSimulator.create_dummy_accounts(count)

//         pub fn help(val=None)
//             if val is None
//                ::sys
//                 ContractSimulator.help(sys.modules[__name__], {c_name}, "{c_name}")
//             else
//                ::builtins
//                 builtins.help(val)

//         """) if len(ast.contracts) == 1 else "") + dedent("""
//         if __name__ == "__main__"
//             ContractSimulator.use_config_from_manifest(os.path.dirname(os.path.realpath(__file__)))
//             me = ContractSimulator.default_address()
//             if me is not None
//                 me = me.val
//            ::code
//             code.interact(local=globals())
//         """)

//     pub fn generate_constructors(self, ast: ContractDefinition) -> String
//         """Generate class constructor (!= contract constructor) and static connect/deploy methods."""

//         # Priv values: private function args plaintext, locally decrypted plaintexts, encryption randomness
//         # State values: if key not in dict -> pull value from chain on read, otherwise retrieve cached value
//         name = self.visit(ast.idf)

//         is_payable = ast.constructor_definitions and ast.constructor_definitions[0].is_payable
//         val_param = ", wei_amount=0" if is_payable else ""
//         val_arg = "wei_amount=wei_amount" if is_payable else ""

//         c_args, c_params = self.get_constructor_args_and_params(ast)

//         if not ast.constructor_definitions
//             deploy_cmd = f"{api("deploy", "c")}([], []{val_arg})"
//         else
//             deploy_cmd = f"c.constructor({c_args}{val_arg})"

//         sv_constr = []
//         for svd in [sv for sv in ast.state_variable_declarations if isinstance(sv, StateVariableDeclaration) and not sv.idf.name.startswith(cfg.reserved_name_prefix)]
//             t = svd.annotated_type.type_name
//             while not isinstance(t, CipherText) and hasattr(t, "value_type")
//                 t = t.value_type.type_name
//             if isinstance(t, CipherText)
//                 crypto_name = t.crypto_params.crypto_name
//                 constr = f", {self._get_type_constr(t.plain_type.type_name)}, cipher=True, crypto_backend="{crypto_name}""
//             else
//                 constr = f", {self._get_type_constr(t)}"
//             sv_constr.append(f"self.state.decl("{svd.idf.name}"{constr})")

//         mf = MultiLineFormatter() * \
//             "pub fn __init__(self, project_dir: &str, user_addr: AddressValue):" /\
//                 f"super().__init__(project_dir, user_addr, "{ast.idf.name}")" * sv_constr // f"""\

//             @staticmethod
//             pub fn connect(address: Union[bytes, &str], user: Union[&str, bytes], project_dir: &str = os.path.dirname(os.path.realpath(__file__))) -> {name}
//                 c = {name}(project_dir, AddressValue(user))
//                 {api("connect", "c")}(AddressValue(address))
//                 ContractSimulator.initialize_keys_for(user)
//                 return c

//             @staticmethod
//             pub fn deploy({c_params}*, user: Union[&str, bytes]{val_param}, project_dir: &str = os.path.dirname(os.path.realpath(__file__))) -> {name}
//                 c = {name}(project_dir, AddressValue(user))
//                 ContractSimulator.initialize_keys_for(user)
//                 {deploy_cmd}
//                 return c
//             """
//         return indent(f"{mf}\n")

//     @staticmethod
//     pub fn is_special_var(idf: Identifier)
//         return (idf.name.startswith(cfg.reserved_name_prefix)
//                 or idf.name.startswith("_tmp_key_")
//                 or idf.name in ["msg", "block", "tx", "now"])

//     @staticmethod
//     pub fn get_priv_value(idf: &str)
//         """Retrieve value of private circuit variable from private-value dictionary"""
//         return f"{PRIV_VALUES_NAME}["{idf}"]"

//     pub fn get_loc_value(self, arr: Identifier, indices: List[&str]) -> String
//         """Get the location of the given identifier/array element."""
//         if isinstance(arr, HybridArgumentIdf) and arr.arg_type == HybridArgType::PrivCircuitVal and not arr.name.startswith("tmp")
//             # Private circuit values are located in private value dictionary
//             return self.get_priv_value(arr.name)
//         elif isinstance(arr, HybridArgumentIdf) and arr.arg_type == HybridArgType::PubCircuitArg
//             # Public circuit inputs are in the zk_data dict
//             return self.visit(arr.get_loc_expr())
//         else
//             idxvals = "".join([f"[{idx}]" for idx in indices])
//             return f"{self.visit(arr)}{idxvals}"

//     pub fn get_value(self, idf: IdentifierExpr, indices: List[&str])
//         """
//         Get code corresponding to the rvalue location of an identifier or index expression.

//         e.g. idf = x and indices = [some_addr, 5] corresponds to x[some_addr][5]
//         State variable values are downloaded from the chain if their value is not yet present in the local state variable dict.
//         """
//         if self.is_special_var(idf.idf)
//             return self.get_loc_value(idf.idf, indices)
//         elif isinstance(idf.target, StateVariableDeclaration)
//             # If a state variable appears as an rvalue, the value may need to be requested from the blockchain
//             indices = f", {", ".join(indices)}" if indices else ""
//             return f"self.state["{idf.idf.name}"{indices}]"
//         else
//             name = idf.idf
//             if isinstance(idf.target, VariableDeclaration) and not self.inside_circuit
//                 # Local variables are stored in locals dict
//                 name = Identifier(f"self.locals["{idf.idf.name}"]")
//             return self.get_loc_value(name, indices)

//     pub fn visitContractDefinition(self, ast: ContractDefinition)
//         """Generate a python class with methods for each function and constructor definition and nested classes for each enum definition."""
//         enums = self.visit_list(ast.enum_definitions, "\n\n")
//         constr = self.visit_list(ast.constructor_definitions, "\n\n")
//         fcts = self.visit_list(ast.function_definitions, "\n\n")
//         return f"class {self.visit(ast.idf)}(ContractSimulator):\n" + \
//                (f"{indent(enums)}\n\n" if enums else "") + \
//                f"{self.generate_constructors(ast)}" + \
//                (f"{indent(constr)}\n\n" if constr else "") + \
//                (f"{indent(fcts)}\n" if fcts else "")

//     pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition)
//         with self.circuit_ctx(ast)
//             return super().visitConstructorOrFunctionDefinition(ast)

//     pub fn visitParameter(self, ast: Parameter)
//         if ast.parent.is_external
//             ot = ast.annotated_type.zkay_type
//             if ot is None
//                 t = "Any"
//             elif ot.is_address()
//                 t = "&str"
//             else
//                 t = self.visit(ot.type_name)
//         elif ast.annotated_type is None
//             t = "Any"
//         else
//             t = self.visit(ast.annotated_type.type_name)
//         return f"{self.visit(ast.idf)}: {t}"

//     pub fn handle_function_params(self, ast: ConstructorOrFunctionDefinition, params: List[Parameter])
//         param_str = super().handle_function_params(ast, self.current_params)
//         if ast.is_payable
//             param_str += ", *, wei_amount: int = 0"
//         return param_str

//     @staticmethod
//     pub fn do_if_external(ast: ConstructorOrFunctionDefinition, extern_elems: Optional[List[&str]] = None, intern_elems: Optional[List[&str]] = None) -> String
//         """
//         Wrap the python statements in extern_elems and intern_elems such that extern_elems are only executed if the surrounding function
//         (python function corresponding to ast) is called externally and intern_elems are only executed if it is not called externally.

//         :param ast: the function to which extern_elems and intern_elems belong
//         :param extern_elems: list of python statements to execute when function is called externally
//         :param intern_elems: list of python statements to execute when function is called internally
//         :return: wrapped code
//         """
//         extern_s = ("\n".join(dedent(s) for s in extern_elems if s) if extern_elems else "").strip()
//         intern_s = ("\n".join(dedent(s) for s in intern_elems if s) if intern_elems else "").strip()
//         if ast.is_external
//             return extern_s
//         elif ast.can_be_external
//             if extern_s
//                 ret = f"if {IS_EXTERNAL_CALL}:\n" + indent(extern_s)
//                 if intern_s
//                     ret += f"\nelse:\n" + indent(intern_s)
//                 return ret
//             elif intern_s
//                 return f"if not {IS_EXTERNAL_CALL}:\n" + indent(intern_s)
//         else
//             return intern_s

//     pub fn handle_function_body(self, ast: ConstructorOrFunctionDefinition)
//         """
//         Return offchain simulation python code for the body of function ast.

//         In addition to what the original code does, the generated python code also

//         * checks that internal functions are not called externally
//         * processes arguments (encryption, address wrapping for external calls),
//         * introduces msg, block and tx objects as local variables (populated with current blockchain state)
//         * serializes the public circuit outputs and the private circuit inputs, which are obtained during \
//           simulation into int lists so that they can be passed to the proof generation
//         * generates the NIZK proof (if needed)
//         * calls/issues transaction with transformed arguments ((encrypted) original args, out array, proof)
//           (or deploys the contract in case of the constructor)
//         """
//         preamble_str = ""
//         if ast.is_external
//             preamble_str += f"assert {IS_EXTERNAL_CALL}\n"
//         preamble_str += f"msg, block, tx = {api("get_special_variables")}()\n" \
//                         f"now = block.timestamp\n"
//         circuit = self.current_circ

//         if circuit and circuit.sec_idfs
//             priv_struct = StructDefinition(None, [VariableDeclaration([], AnnotatedTypeName(sec_idf.t), sec_idf) for sec_idf in circuit.sec_idfs])
//             preamble_str += f"\n{PRIV_VALUES_NAME}: Dict[&str, Any] = {self.get_default_value(StructTypeName([], priv_struct))}\n"

//         all_params = ", ".join([f"{self.visit(param.idf)}" for param in self.current_params])
//         if ast.can_be_external
//             # Wrap address strings in AddressValue object for external calls
//             address_params = [self.visit(param.idf) for param in self.current_params if
//                               param.annotated_type.zkay_type.is_address()]
//             if address_params
//                 assign_addr_str = f"{", ".join(address_params)} = {", ".join([f"AddressValue({p})" for p in address_params])}"
//                 preamble_str += f"\n{self.do_if_external(ast, [assign_addr_str])}\n"

//         if ast.can_be_external and circuit
//             # Encrypt parameters and add private circuit inputs (plain + randomness)
//             enc_param_str = ""
//             for arg in self.current_params
//                 if arg.annotated_type.is_cipher()
//                     assert isinstance(arg.annotated_type.type_name, CipherText)
//                     cipher: CipherText = arg.annotated_type.type_name
//                     pname = self.visit(arg.idf)
//                     plain_val = pname
//                     plain_t = cipher.plain_type.type_name
//                     crypto_params = cipher.crypto_params
//                     crypto_str = f"crypto_backend="{crypto_params.crypto_name}""
//                     if plain_t.is_signed_numeric and crypto_params.enc_signed_as_unsigned
//                         plain_val = self.handle_cast(pname, UintTypeName(f"uint{plain_t.elem_bitwidth}"))
//                     enc_param_str += f"{self.get_priv_value(arg.idf.name)} = {plain_val}\n"
//                     if crypto_params.is_symmetric_cipher()
//                         my_pk = f"{api("get_my_pk")}("{crypto_params.crypto_name}")[0]"
//                         enc_expr = f"{api("enc")}({self.get_priv_value(arg.idf.name)}, {crypto_str})"
//                         enc_param_str += f"{pname} = CipherValue({enc_expr}[0][:-1] + ({my_pk}, ), {crypto_str})\n"
//                     else
//                         enc_expr = f"{api("enc")}({self.get_priv_value(arg.idf.name)}, {crypto_str})"
//                         enc_param_str += f"{pname}, {self.get_priv_value(f"{arg.idf.name}_R")} = {enc_expr}\n"

//             enc_param_comment_str = "\n# Encrypt parameters" if enc_param_str else ""
//             enc_param_str = enc_param_str[:-1] if enc_param_str else ""

//             actual_params_assign_str = f"actual_params = [{all_params}]"

//             out_var_decl_str = f"{cfg.zk_out_name}: List[int] = [0 for _ in range({circuit.out_size_trans})]"
//             out_var_decl_str += f"\nactual_params.append({cfg.zk_out_name})"

//             pre_body_code = self.do_if_external(ast, [
//                 enc_param_comment_str,
//                 enc_param_str,
//                 actual_params_assign_str,
//                 out_var_decl_str
//             ])
//         elif ast.can_be_external
//             pre_body_code = f"actual_params = [{all_params}]"
//         else
//             pre_body_code = ""

//         # Simulate public contract to compute in_values (state variable values are pulled from blockchain if necessary)
//         # (out values are also computed when encountered, by locally evaluating and encrypting
//         # the corresponding private expressions)
//         body_str = self.visit(ast.body).strip()

//         serialize_str = ""
//         if circuit is not None
//             if circuit.output_idfs
//                 out_elemwidths = ", ".join([&str(out.t.elem_bitwidth) if out.t.is_primitive_type() else "0" for out in circuit.output_idfs])
//                 serialize_str += f"\n{cfg.zk_out_name}[{cfg.zk_out_name}_start_idx:{cfg.zk_out_name}_start_idx + {circuit.out_size}] = " \
//                                  f"{api("serialize_circuit_outputs")}(zk__data, [{out_elemwidths}])"
//             if circuit.sec_idfs
//                 sec_elemwidths = ", ".join([&str(sec.t.elem_bitwidth) if sec.t.is_primitive_type() else "0" for sec in circuit.sec_idfs])
//                 serialize_str += f"\n{api("serialize_private_inputs")}({PRIV_VALUES_NAME}, [{sec_elemwidths}])"
//         if serialize_str
//             serialize_str = f"\n# Serialize circuit outputs and/or secret circuit inputs\n" + serialize_str.lstrip()

//         body_code = "\n".join(dedent(s) for s in [
//             f"\n## BEGIN Simulate body",
//             body_str,
//             "## END Simulate body",
//             serialize_str,
//         ] if s) + "\n"

//         # Add proof to actual argument list (when required)
//         generate_proof_str = ""
//         fname = f""{ast.name}""
//         if ast.can_be_external and circuit and ast.has_side_effects
//             generate_proof_str += "\n".join(["\n#Generate proof",
//                                              f"proof = {api("gen_proof")}({fname}, {cfg.zk_in_name}, {cfg.zk_out_name})",
//                                              "actual_params.append(proof)"])

//         should_encrypt = ", ".join([&str(p.annotated_type.is_cipher()) for p in self.current_f.parameters])
//         if ast.is_constructor
//             invoke_transact_str = f"""
//             # Deploy contract
//             {api("deploy")}(actual_params, [{should_encrypt}]{", wei_amount=wei_amount" if ast.is_payable else ""})
//             """
//         elif ast.has_side_effects
//             invoke_transact_str = f"""
//             # Invoke public transaction
//             return {api("transact")}({fname}, actual_params, [{should_encrypt}]{", wei_amount=wei_amount" if ast.is_payable else ""})
//             """
//         elif ast.return_parameters
//             constructors = []
//             for retparam in ast.return_parameters
//                 t = retparam.annotated_type.type_name
//                 if isinstance(t, CipherText)
//                     constr = f"(True, "{t.crypto_params.crypto_name}", {self._get_type_constr(t.plain_type.type_name)})"
//                 else
//                     constr = f"(False, None, {self._get_type_constr(t)})"
//                 constructors.append(constr)
//             constructors = f"[{", ".join(constructors)}]"

//             invoke_transact_str = f"""
//             # Call pure/view function and return value
//             return {api("call")}({fname}, actual_params, {constructors})
//             """
//         else
//             invoke_transact_str = ""

//         post_body_code = self.do_if_external(ast, [
//             generate_proof_str,
//             invoke_transact_str
//         ], [f"return {", ".join([f"{cfg.return_var_name}_{idx}" for idx in range(len(ast.return_parameters))])}"
//             if ast.is_function and ast.requires_verification and ast.return_parameters else None])

//         code = "\n\n".join(s.strip() for s in [
//             f"assert not {IS_EXTERNAL_CALL}" if not ast.can_be_external else None,
//             dedent(preamble_str),
//             pre_body_code,
//             body_code,
//             post_body_code
//         ] if s)

//         func_ctx_params = []
//         if circuit
//             func_ctx_params.append(&str(circuit.priv_in_size_trans))
//         if ast.is_payable
//             func_ctx_params.append("wei_amount=wei_amount")
//         if ast.can_be_external
//             func_ctx_params.append(f"name={fname}")
//             code = "with time_measure("transaction_full", skip=not zk__is_ext):\n" + indent(code)
//         code = f"with self._function_ctx({", ".join(func_ctx_params)}) as {IS_EXTERNAL_CALL}:\n" + indent(code)
//         return code

//     pub fn visitStatementList(self, ast: StatementList)
//         if ast.excluded_from_simulation
//             return None
//         else
//             return super().visitStatementList(ast)

//     pub fn visitBlock(self, ast: Block)
//         # Introduce a new virtual local scope when visiting a block
//         ret = super().visitBlock(ast)
//         return f"with self._scope():\n{indent(ret)}"

//     pub fn visitReturnStatement(self, ast: ReturnStatement)
//         if not ast.function.requires_verification
//             return self.do_if_external(ast.function, None, [super().visitReturnStatement(ast)])
//         else
//             return None

//     pub fn visitCircuitInputStatement(self, ast: CircuitInputStatement)
//         """
//         Generate code which assigns the specified value to a circuit input variable.

//         If the circuit input is encrypted, this will also generate code to add a decrypted
//         version + the corresponding randomness to the private circuit input dict.
//         """
//         in_decrypt = ""
//         in_idf = ast.lhs.member
//         assert isinstance(in_idf, HybridArgumentIdf)
//         if in_idf.corresponding_priv_expression is not None
//             plain_idf_name = self.get_priv_value(in_idf.corresponding_priv_expression.idf.name)
//             constr = self._get_type_constr(in_idf.t.plain_type.type_name)
//             crypto_params = in_idf.t.crypto_params
//             crypto_str = f"crypto_backend="{crypto_params.crypto_name}""
//             dec_call = f"{api("dec")}({self.visit(in_idf.get_loc_expr())}, {constr}, {crypto_str})"
//             if crypto_params.is_symmetric_cipher()
//                 in_decrypt += f"\n{plain_idf_name}, _ = {dec_call}"
//             else
//                 in_decrypt += f"\n{plain_idf_name}, {self.get_priv_value(f"{in_idf.name}_R")} = {dec_call}"
//         return self.visitAssignmentStatement(ast) + in_decrypt

//     pub fn visitCircuitComputationStatement(self, ast: CircuitComputationStatement)
//         """
//         Generate code which simulates the evaluation of a private expression.

//         The expression is evaluated with finite field semantics and its (encrypted) result
//         is assigned to the corresponding circuit output variable.
//         """
//         out_initializations = ""
//         out_idf = ast.idf
//         out_val = out_idf.corresponding_priv_expression
//         if isinstance(out_val, EncryptionExpression)
//             cipher_loc = self.visit(out_idf.get_loc_expr())
//             if cfg.is_symmetric_cipher(out_val.homomorphism)
//                 s = cipher_loc
//             else
//                 s = f"{cipher_loc}, {self.get_priv_value(f"{out_idf.name}_R")}"
//         else
//             s = f"{self.visit(out_idf.get_loc_expr())}"

//         with self.circuit_computation(flatten_hybrid_args=True)
//             rhs = self.visit(out_val)
//         if not out_idf.t.is_cipher()
//             rhs = self.handle_cast(rhs, out_idf.t)
//         elif out_idf.t.crypto_params.is_symmetric_cipher()
//             crypto_backend = out_idf.t.crypto_params.crypto_name
//             my_pk = f"{api("get_my_pk")}("{crypto_backend}")[0]"
//             rhs += f"[0][:-1] + ({my_pk}, )"
//             rhs = f"CipherValue({rhs}, crypto_backend="{crypto_backend}")"
//         s = f"{s} = {rhs}"
//         out_initializations += f"{s}\n"
//         return out_initializations

//     pub fn visitEnterPrivateKeyStatement(self, ast: EnterPrivateKeyStatement)
//         assert self.current_circ
//         key_var_name = self.current_circ.get_own_secret_key_name(ast.crypto_params)
//         return f"{PRIV_VALUES_NAME}["{key_var_name}"] = {api("get_my_sk")}("{ast.crypto_params.crypto_name}")"

//     pub fn visitEncryptionExpression(self, ast: EncryptionExpression)
//         priv_str = "msg.sender" if isinstance(ast.privacy, MeExpr) else self.visit(ast.privacy.clone())
//         crypto_params = cfg.get_crypto_params(ast.homomorphism)
//         crypto_str = f"crypto_backend="{crypto_params.crypto_name}""
//         plain = self.visit(ast.expr)
//         plain_t = ast.expr.annotated_type.type_name
//         if plain_t.is_signed_numeric and crypto_params.enc_signed_as_unsigned
//             plain = self.handle_cast(plain, UintTypeName(f"uint{plain_t.elem_bitwidth}"))
//         return f"{api("enc")}({plain}, {priv_str}, {crypto_str})"

//     pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr)
//         if isinstance(ast.func, BuiltinFunction) and (ast.func.is_arithmetic() or ast.func.op == "~")
//             # For arithmetic operations, need to simulate finite integer semantics (since python has arbitrary precision ints)
//             t = ast.annotated_type.type_name if ast.annotated_type is not None else TypeName::uint_type()
//             res = super().visitFunctionCallExpr(ast)
//             if not t.is_literal and ast.func.homomorphism == Homomorphism.NonHomomorphic
//                 # Use cast for correct overflow behavior according to type
//                 res = self.handle_cast(res, t)
//             return res
//         elif isinstance(ast.func, BuiltinFunction) and ast.func.is_comp() and self.inside_circuit
//             # Inside circuit, only comparisons with values using less than 252 bits are valid
//             # -> perform additional check
//             args = [f"{api("range_checked")}({self.visit(a)})" for a in ast.args]
//             return ast.func.format_string().format(*args)
//         elif ast.is_cast
//             return self.handle_cast(self.visit(ast.args[0]), ast.func.target.annotated_type.type_name)
//         elif isinstance(ast.func, LocationExpr) and ast.func.target is not None and ast.func.target.requires_verification
//             # Function calls to functions which require verification need to be treated differently
//             # (called function has a different priv-value dictionary)
//             f = self.visit(ast.func)
//             a = self.visit_list(ast.args, ", ")
//             return f"{api("call_fct")}({ast.sec_start_offset}, self.{f}, {a})"

//         return super().visitFunctionCallExpr(ast)

//     pub fn visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr)
//         if ast.is_implicit and not self.inside_circuit
//             e = self.visit(ast.expr)
//             if isinstance(ast.expr, NumberLiteralExpr) and ast.annotated_type.is_address()
//                 # Special case when implicitly casting address literal to address
//                 return f"AddressValue({e})"
//             else
//                 # Implicit casts in public code can be ignored, since they have to effect on the value
//                 return e
//         else
//             return self.handle_cast(self.visit(ast.expr), ast.elem_type)

//     pub fn visitKeyLiteralExpr(self, ast: KeyLiteralExpr)
//         return f"PublicKeyValue({super().visitArrayLiteralExpr(ast)}, crypto_backend="{ast.crypto_params.crypto_name}")"

//     pub fn handle_cast(self, expr: &str, t: TypeName) -> String
//         """Return python expr which corresponds to expr converted to type t."""
//         constr = self._get_type_constr(t)
//         return f"{constr}({expr})"

//     pub fn visitMemberAccessExpr(self, ast: MemberAccessExpr)
//         assert not isinstance(ast.target, StateVariableDeclaration), "State member accesses not handled"

//         if isinstance(ast.expr.target, EnumDefinition)
//             return f"{self.visit_list(ast.expr.target.qualified_name, sep=".")}.{self.visit(ast.member)}"

//         if ast.member.name == "length" and isinstance(ast.expr.target.annotated_type.type_name, Array)
//             return f"len({self.visit(ast.expr)})"

//         if self.current_index
//             indices = list(reversed(self.current_index))
//             self.current_index, self.current_index_t = [], None
//             indices = [self.visit(idx) for idx in indices]
//         else
//             indices = []

//         if isinstance(ast.member, HybridArgumentIdf)
//             e = f"{self.visit(ast.expr)}["{ast.member.name}"]"
//         else
//             e = super().visitMemberAccessExpr(ast)
//         return self.get_loc_value(Identifier(e), indices)

//     pub fn visitIdentifierExpr(self, ast: IdentifierExpr)
//         # Special identifiers
//         pki_inst_names = {f"{cfg.get_pki_contract_name(params)}_inst": params for params in cfg.all_crypto_params()}
//         if ast.idf.name in pki_inst_names and not ast.is_lvalue()
//             crypto_params = pki_inst_names[ast.idf.name]
//             return f"{api("get_keystore")}("{crypto_params.crypto_name}")"
//         elif ast.idf.name == cfg.field_prime_var_name
//             assert ast.is_rvalue()
//             return f"{SCALAR_FIELD_NAME}"

//         if self.current_index
//             # This identifier is the beginning of an Index expression e.g. idf[1][2] or idf[me]
//             indices, t = list(reversed(self.current_index)), self.current_index_t
//             self.current_index, self.current_index_t = [], None
//             indices = [self.visit(idx) for idx in indices]
//         elif self.inside_circuit and isinstance(ast.idf, HybridArgumentIdf) and ast.idf.corresponding_priv_expression is not None and self.flatten_hybrid_args
//             return self.visit(ast.idf.corresponding_priv_expression)
//         else
//             indices, t = [], ast.target.annotated_type if isinstance(ast.target, StateVariableDeclaration) else None

//         return self.get_value(ast, indices)

//     pub fn visitIndexExpr(self, ast: IndexExpr)
//         """
//         Convert an index expression.

//         Since Index.arr can be an IndexExpr, it is possible that this IndexExpr is actually part of a nested index expression.
//         e.g. when we have x[p][i], this will be parsed as IndexExpr(IndexExpr(x, p), i) and the outer IndexExpr will be visited first.
//         This is problematic in the case where x is a state variable, since the value has to be requested from the chain
//         using the call x(p, i).
//         One has to recursively visit all the IndexExpr.arr children to know which state variable to call, as the Index expressions
//         are basically visited in reverse order.

//         At the moment, this problem is solved by constructing the full, combined index expression in reverse order
//         (by keeping track of all index keys and their types in the list self.current_index until IndexExpr.arr is an IdentifierExpr, which terminates the recursion/nesting.
//         Evaluation of IndexExpr.key for all encountered IndexExpr is also delayed until then, since nested IndexExpr in the key expressions would otherwise break the current_index array).
//         """
//         if self.current_index_t is None
//             self.current_index_t = ast.target.annotated_type
//         self.current_index.append(ast.key)
//         return self.visit(ast.arr)

//     pub fn get_default_value(self, t: TypeName)
//         if isinstance(t, (AddressTypeName, AddressPayableTypeName))
//             return "AddressValue(0)"
//         else
//             return super().get_default_value(t)

//     pub fn visitVariableDeclarationStatement(self, ast: VariableDeclarationStatement)
//         if self.is_special_var(ast.variable_declaration.idf)
//             return super().visitVariableDeclarationStatement(ast)
//         else
//             s = ast.variable_declaration.idf.name
//             e = self.handle_var_decl_expr(ast)
//             return f"self.locals.decl("{s}", {e})"

//     pub fn handle_var_decl_expr(self, ast: VariableDeclarationStatement) -> String
//         ret = super().handle_var_decl_expr(ast)
//         decl_type = ast.variable_declaration.annotated_type.type_name
//         if decl_type.is_cipher() and isinstance(ast.expr, ArrayLiteralExpr)
//             ret = f"CipherValue({ret}, crypto_backend="{decl_type.crypto_params.crypto_name}")"
//         return ret

//     # Types with special wrapper classes

//     pub fn visitCipherText(self, _)
//         return "CipherValue"

//     pub fn visitKey(self, _)
//         return "PublicKeyValue"

//     pub fn visitRandomness(self, _)
//         return "RandomnessValue"

//     pub fn visitAddressTypeName(self, ast: AddressTypeName)
//         return "AddressValue"

//     @contextmanager
//     pub fn circuit_ctx(self, ast: ConstructorOrFunctionDefinition) -> ContextManager
//         """
//         Return a context manager which sets the sets the current function, circuit and parameter fields to match the specified function.

//         :param ast: function definition which will be visited within this context
//         :return: context manager
//         """
//         self.current_f = ast
//         self.current_circ = self.circuits.get(ast, None)
//         if self.current_circ and self.current_f.can_be_external
//             self.current_params = [p for p in ast.parameters if p.idf.name != cfg.zk_out_name and p.idf.name != cfg.proof_param_name]
//         else
//             self.current_params = ast.parameters.copy()
//         yield
//         self.current_f, self.current_circ, self.current_params = None, None, None

//     @contextmanager
//     pub fn circuit_computation(self, flatten_hybrid_args: bool = False) -> ContextManager
//         // """
//         // Return a context manager which enables the inside_circuit flag and sets the flatten_hybrid_args flag as specified
//         // for the duration of its lifetime.

//         // :param flatten_hybrid_args: if true, all encountered HybridArgumentIdfs which have a private expression associated with them are
//         //                             replaced by that private expression (recursively) during the lifetime of this context manager.
//         // :return: context manager
//         // """
//         assert not self.inside_circuit
//         self.inside_circuit = True
//         old_fp = self.flatten_hybrid_args
//         self.flatten_hybrid_args = flatten_hybrid_args
//         yield
//         assert self.inside_circuit
//         self.inside_circuit = False
//         self.flatten_hybrid_args = old_fp
