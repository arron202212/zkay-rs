// """Circuit Generator implementation for the jsnark backend"""

// import os
// from typing import List, Optional, Union, Tuple

use crate::compiler::privacy::circuit_generation::circuit_constraints::{
    CircCall, CircComment, CircGuardModification, CircIndentBlock, CircSymmEncConstraint,
};
use crate::compiler::privacy::circuit_generation::circuit_generator::CircuitGenerator;
use crate::compiler::privacy::circuit_generation::circuit_helper::{
    CircEncConstraint, CircEqConstraint, CircVarDecl, CircuitHelper, CircuitStatement,
    HybridArgumentIdf,
};
use crate::compiler::privacy::proving_scheme::backends::gm17::ProvingSchemeGm17;
use crate::compiler::privacy::proving_scheme::backends::groth16::ProvingSchemeGroth16;
use crate::compiler::privacy::proving_scheme::proving_scheme::{
    G1Point, G2Point, ProvingScheme, VerifyingKeyMeta,
};
use crate::jsnark_interface::jsnark_interface as jsnark;
use crate::jsnark_interface::libsnark_interface as libsnark;
use crate::utils::helpers::{hash_file, hash_string};
use crate::utils::helpers::{read_file, save_to_file};
use crate::zkay_ast::ast::{
    indent, BooleanLiteralExpr, BuiltinFunction, EnumDefinition, Expression, FunctionCallExpr,
    IdentifierExpr, IndexExpr, MeExpr, MemberAccessExpr, NumberLiteralExpr, PrimitiveCastExpr,
    TypeName,is_instance,ASTType,
};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::visitor::AstVisitor;
use crate::{config::CFG, zk_print};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
pub fn _get_t(t: (Option<TypeName>, Option<Expression>))
// """Return the corresponding jsnark type name for a given type or expression."""
{
    if is_instance(&t,ASTType::Expression) {
        t = t.annotated_type.type_name;
    }
    let bits = t.elem_bitwidth;
    if t.elem_bitwidth == 1 {
        "ZkBool"
    }
    if t.is_signed_numeric {
        format!(r#"ZkInt({bits})"#)
    } else {
        format!(r#"ZkUint({bits})"#)
    }
}

// class JsnarkVisitor(AstVisitor)
pub struct JsnarkVisitor {
    phi: Vec<CircuitStatement>,
}
impl JsnarkVisitor
// """Visitor which compiles CircuitStatements and Expressions down to java code compatible with a custom jsnark wrapper."""
{
    pub fn new(phi: Vec<CircuitStatement>)
    // super().__init__("node-or-children", false)
    {
        Self { phi }
    }

    pub fn visitCircuit(self) -> Vec<String> {
        self.phi.iter().map(|constr| self.visit(constr)).collect()
    }

    pub fn visitCircComment(self, stmt: CircComment) {
        if stmt.text {
            format!(r#"// {}"#, stmt.text)
        } else {
            String::new()
        }
    }

    pub fn visitCircIndentBlock(self, stmt: CircIndentBlock) {
        let stmts = list(map(self.visit, stmt.statements));
        if stmt.name {
            format!(
                r#"//[ --- {name} ---\n {} \n //] --- {name} ---\n"#,
                indent(stmts.join("\n")), name = stmt.name
            )
        } else {
            indent(stmts.join("\n"))
        }
    }

    pub fn visitCircCall(self, stmt: CircCall) {
        format!(r#"_{}();"#, stmt.fct.name)
    }

    pub fn visitCircVarDecl(self, stmt: CircVarDecl) {
        format!(r#"decl("{}", {});"#, stmt.lhs.name, self.visit(stmt.expr))
    }

    pub fn visitCircEqConstraint(self, stmt: CircEqConstraint) {
        assert!(stmt.tgt.t.size_in_uints == stmt.val.t.size_in_uints);
        format!(r#"checkEq("{}", "{}");"#, stmt.tgt.name, stmt.val.name)
    }

    pub fn visitCircEncConstraint(self, stmt: CircEncConstraint) {
        assert!(stmt.cipher.t.is_cipher());
        assert!(stmt.pk.t.is_key());
        assert!(stmt.rnd.t.is_randomness());
        assert!(
            stmt.cipher.t.crypto_params == stmt.pk.t.crypto_params
                && stmt.pk.t.crypto_params == stmt.rnd.t.crypto_params
        );
        let backend = stmt.pk.t.crypto_params.crypto_name;

        format!(
            r#"check{}("{backend}", "{}", "{}", "{}", "{}");"#,
            if stmt.is_dec { "Dec" } else { "Enc" },
            stmt.plain.name,
            stmt.pk.name,
            stmt.rnd.name,
            stmt.cipher.name
        )
    }
    pub fn visitCircSymmEncConstraint(self, stmt: CircSymmEncConstraint) -> String {
        assert!(stmt.iv_cipher.t.is_cipher());
        assert!(stmt.other_pk.t.is_key());
        assert!(stmt.iv_cipher.t.crypto_params == stmt.other_pk.t.crypto_params);
        let backend = stmt.other_pk.t.crypto_params.crypto_name;
        format!(
            r#"checkSymm{}("{backend}", "{}", "{}", "{}");"#,
            if stmt.is_dec { "Dec" } else { "Enc" },
            stmt.plain.name,
            stmt.other_pk.name,
            stmt.iv_cipher.name
        )
    }
    pub fn visitCircGuardModification(self, stmt: CircGuardModification) {
        if let Some(new_cond) = stmt.new_cond {
            format!(
                r#"addGuard("{}", {});"#,
                stmt.new_cond.name,
                stmt.is_true.to_string().to_ascii_lowercase()
            )
        } else {
            "popGuard();"
        }
    }

    pub fn visitBooleanLiteralExpr(self, ast: BooleanLiteralExpr) {
        format!(r#"val({})"#, ast.value.to_string().to_ascii_lowercase())
    }

    pub fn visitNumberLiteralExpr(self, ast: NumberLiteralExpr) {
        let t = _get_t(ast);
        if ast.value < (1 << 31) {
            format!(r#"val({}, {t})"#, ast.value)
        } else {
            format!(r#"val("{}", {t})"#, ast.value)
        }
    }

    pub fn visitIdentifierExpr(self, ast: IdentifierExpr) {
        if is_instance(&ast.idf,ASTType:: HybridArgumentIdf) && ast.idf.t.is_cipher() {
            format!(r#"getCipher("{}")"#, ast.idf.name)
        } else {
            format!(r#"get("{}")"#, ast.idf.name)
        }
    }

    pub fn visitMemberAccessExpr(self, ast: MemberAccessExpr) {
        assert!(is_instance(&ast.member,ASTType:: HybridArgumentIdf));
        if ast.member.t.is_cipher() {
            format!(r#"getCipher("{}")"#, ast.member.name)
        } else {
            assert!(ast.member.t.size_in_uints == 1);
            format!(r#"get("{}")"#, ast.member.name)
        }
    }

    pub fn visitIndexExpr(self, ast: IndexExpr) {
        unimplemented!();
    }

    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if is_instance(&ast.func,ASTType:: BuiltinFunction) {
            assert!(ast.func.can_be_private());
            let mut args: Vec<_> = ast.args.iter().map(self.visit).collect();
            if ast.func.is_shiftop() {
                assert!(ast.args[1].annotated_type.type_name.is_literal);
                args[1] = ast.args[1].annotated_type.type_name.value
            }

            let mut op = ast.func.op.clone();
            op = if op == "sign-" { "-" } else { op };
            if op == "sign+" {
                unimplemented!()
            }
            let homomorphism = ast.func.homomorphism.clone();
            let (f_start,crypto_backend,public_key_name) = if homomorphism == Homomorphism::non_homomorphic() {
                (String::from("o_("),String::new(),String::new())
            } else {
                let crypto_backend = CFG.lock().unwrap().get_crypto_params(homomorphism).crypto_name;
                let public_key_name = ast.public_key.name;

                args = args
                    .iter()
                    .map(|arg| format!(r#"HomomorphicInput.of({arg})"#))
                    .collect();
                (format!(r#"o_hom("{crypto_backend}", "{public_key_name}", "#),crypto_backend,public_key_name)
            };

            if op == "ite" {
                format!(
                    r#"{f_start}{{{}}}, "?", {{{}}}, ":", {{{}}})"#,
                    args[0], args[1], args[2]
                )
            } else if op == "parenthesis" {
                String::from("({})")
            } else {
                let o = if len(op) == 1 {
                    format!(r#"'{op}'"#)
                } else {
                    format!(r#""{op}""#)
                };
                if len(args) == 1 {
                    format!(r#"{f_start}{o}, {{{}}})"#, args[0])
                } else {
                    assert!(len(args) == 2);
                    if op == "*" && ast.func.rerand_using.is_some() {
                        // re-randomize homomorphic scalar multiplication
                        let rnd = self.visit(ast.func.rerand_using);
                        format!(
                            r#"o_rerand({f_start}{{{}}}, {o}, {{{}}}), "{crypto_backend}", "{public_key_name}", {rnd})"#, args[0], args[1]
                        )
                    } else {
                        format!(r#"{f_start}{{{}}}, {o}, {{{}}})"#, args[0], args[1])
                    }
                }
            }
        } else if ast.is_cast && is_instance(&ast.func.target,ASTType:: EnumDefinition) {
            assert!(ast.annotated_type.type_name.elem_bitwidth == 256);
            return self.handle_cast(self.visit(ast.args[0]), TypeName::uint_type());
        }

        assert!(
            false,
            "Unsupported function {} inside circuit",
            ast.func.code()
        );
    }

    pub fn visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr) {
        self.handle_cast(self.visit(ast.expr), ast.elem_type)
    }

    pub fn handle_cast(self, wire: String, t: TypeName) {
        format!(r#"cast({wire}, {})"#, _get_t(t))
    }
}

pub fn add_function_circuit_arguments(circuit: CircuitHelper)
// """Generate java code which adds circuit IO as described by circuit"""
{
    let mut input_init_stmts = vec![];
    for sec_input in circuit.sec_idfs {
        input_init_stmts.push(format!(
            r#"addS("{}", {}, {});"#,
            sec_input.name,
            sec_input.t.size_in_uints,
            _get_t(sec_input.t)
        ));
    }

    for pub_input in circuit.input_idfs {
        input_init_stmts.push(if pub_input.t.is_key() {
            let backend = pub_input.t.crypto_params.crypto_name;
            format!(
                r#"addK("{backend}", "{}", {});"#,
                pub_input.name, pub_input.t.size_in_uints
            )
        } else {
            format!(
                r#"addIn("{}", {}, {});"#,
                pub_input.name,
                pub_input.t.size_in_uints,
                _get_t(pub_input.t)
            )
        });
    }
    for pub_output in circuit.output_idfs {
        input_init_stmts.push(format!(
            r#"addOut("{}", {}, {});"#,
            pub_output.name,
            pub_output.t.size_in_uints,
            _get_t(pub_output.t)
        ));
    }

    let sec_input_names = circuit
        .sec_idfs
        .iter()
        .map(|sec_input| sec_input.name.clone())
        .collect();
    for crypto_params in CFG.lock().unwrap().all_crypto_params() {
        let pk_name = circuit.get_glob_key_name(MeExpr::new(), crypto_params);
        let sk_name = circuit.get_own_secret_key_name(crypto_params);
        if crypto_params.is_symmetric_cipher() && sec_input_names.contains(sk_name) {
            assert!(circuit
                .input_idfs
                .iter()
                .map(|pub_input| pub_input.name.clone())
                .collect()
                .contains(pk_name));
            input_init_stmts.push(format!(
                r#"setKeyPair("{}", "{pk_name}", "{sk_name}");"#,
                crypto_params.crypto_name
            ));
        }
    }

    return input_init_stmts;
}

// class JsnarkGenerator(CircuitGenerator)
pub struct JsnarkGenerator {
    circuit_generator_base: CircuitGenerator,
}

impl JsnarkGenerator {
    pub fn new(circuits: Vec<CircuitHelper>, proving_scheme: ProvingScheme, output_dir: String) {
        Self {
            circuit_generator_base: CircuitGenerator::new(
                circuits,
                proving_scheme,
                output_dir,
                false,
            ),
        }
    }

    pub fn _generate_zkcircuit(self, import_keys: bool, circuit: CircuitHelper) -> bool
//Create output directory
    {
        let output_dir = self._get_circuit_output_dir(circuit);
        if let Err(_) | Ok(false) = output_dir.try_exists() {
            std::fs::create_dir_all(output_dir).expect("{}", output_dir);
        }

        //Generate java code to add used crypto backends by calling addCryptoBackend
        let mut crypto_init_stmts = vec![];
        for params in circuit.fct.used_crypto_backends {
            let init_stmt = format!(
                r#"addCryptoBackend("{}", "{}", {});"#,
                params.crypto_name, params.crypto_name, params.key_bits
            );
            crypto_init_stmts.push(init_stmt);
        }

        //Generate java code for all functions which are transitively called by the fct corresponding to this circuit
        //(outside private expressions)
        let mut fdefs = vec![];
        for fct in circuit.transitively_called_functions.keys() {
            let target_circuit = self.circuits[fct];
            let body_stmts = JsnarkVisitor(target_circuit.phi).visitCircuit();

            let body = [format!(r#"stepIn("{}");"#, fct.name)]
                .into_iter()
                .chain(add_function_circuit_arguments(target_circuit))
                .chain([""])
                .chain(body_stmts)
                .chain(["stepOut();"])
                .collect::<Vec<_>>()
                .join("\n");
            let fdef = format!(
                r#"private void _{name}() {{\n {body} \n}}"#,
                body = indent(body),
                name = fct.name
            );
            fdefs.push(format!(r#"{fdef}"#))
        }

        //Generate java code for the function corresponding to this circuit
        let input_init_stmts = add_function_circuit_arguments(circuit);
        let constraints = JsnarkVisitor::new()(circuit.phi).visitCircuit();

        //Inject the function definitions into the java template
        let code = jsnark::get_jsnark_circuit_class_str(
            circuit,
            crypto_init_stmts,
            fdefs,
            input_init_stmts.iter().chain([""]).chain(constraints).collect(),
        );

        //Compute combined hash of the current jsnark interface jar and of the contents of the java file
        let hashfile = output_dir.join(format!(
            r#"{}.hash"#,
            CFG.lock().unwrap().jsnark_circuit_classname
        ));
        let digest = hex::encode(hash_string(
            &(jsnark::CIRCUIT_BUILDER_JAR_HASH + &code + &CFG.lock().unwrap().proving_scheme)
        ));
        let oldhash = if let Ok(true) = hashfile.try_exists() {
            read_file(hashfile)
        } else {
            String::new()
        };

        //Invoke jsnark compilation if either the jsnark-wrapper or the current circuit was modified (based on hash comparison)
        if oldhash != digest
            || output_dir
                .join("circuit.arith")
                .try_exists()
                .map_or(false, |v| v)
        {
            if !import_keys
            //Remove old keys
            {
                for f in self._get_vk_and_pk_paths(circuit) {
                    if Path::new(f).try_exists().map_or(false,|v|v) {
                        std::fs::remove_file(f);
                    }
                }
            }
            jsnark::compile_circuit(output_dir, code);
            save_to_file(None, hashfile, digest);
            true
        } else {
            zk_print(format!(
                r#"Circuit \"{}\" not modified, skipping compilation"#,
                circuit.get_verification_contract_name()
            ));
            false
        }
    }
    pub fn _generate_keys(self, circuit: CircuitHelper)
    //Invoke the custom libsnark interface to generate keys
    {
        let output_dir = self._get_circuit_output_dir(circuit);
        libsnark::generate_keys(output_dir, output_dir, self.proving_scheme.name);
    }

    // @classmethod
    pub fn get_vk_and_pk_filenames() -> Vec<String> {
        ["verification.key", "proving.key", "verification.key.bin"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    pub fn _parse_verification_key(self, circuit: CircuitHelper) -> impl VerifyingKeyMeta {
        let f = File::open(self._get_vk_and_pk_paths(circuit)[0]).expect("");
        // data = iter(f.read().splitlines());
        let buf = BufReader::new(f);
        let data = buf.lines();
        if is_instance(&self.proving_scheme,ASTType:: ProvingSchemeGroth16) {
            let a = G1Point::from_it(data);
            let b = G2Point::from_it(data);
            let gamma = G2Point::from_it(data);
            let delta = G2Point::from_it(data);
            let query_len = int(next(data));
            let gamma_abc = vec![None; query_len];
            for idx in 0..query_len {
                gamma_abc.insert(idx,G1Point::from_it(data));
            }
            return ProvingSchemeGroth16.VerifyingKey(a, b, gamma, delta, gamma_abc);
        } else if is_instance(&self.proving_scheme,ASTType:: ProvingSchemeGm17) {
            let h = G2Point::from_it(data);
            let g_alpha = G1Point::from_it(data);
            let h_beta = G2Point::from_it(data);
            let g_gamma = G1Point::from_it(data);
            let h_gamma = G2Point::from_it(data);
            let query_len = data.next().unwrap();
            let query = vec![None; query_len];
            for idx in 0..query_len {
                query.insert(idx,G1Point::from_it(data));
            }
            return ProvingSchemeGm17.VerifyingKey(h, g_alpha, h_beta, g_gamma, h_gamma, query)
        } else {
            unimplemented!()
        }
    }

    pub fn _get_prover_key_hash(self, circuit: CircuitHelper) -> Vec<u8> {
        hash_file(self._get_vk_and_pk_paths(circuit)[1])
    }

    pub fn _get_primary_inputs(self, circuit: CircuitHelper) -> Vec<String>
//Jsnark requires an additional public input with the value 1 as first input
    {
        [String::from("1")].into_iter().chain(self.circuit_generator_base._get_primary_inputs(circuit)).collect()
    }
}
