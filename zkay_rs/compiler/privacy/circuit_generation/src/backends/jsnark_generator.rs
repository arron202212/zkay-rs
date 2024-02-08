// """Circuit Generator implementation for the jsnark backend"""

// import os
// from typing import List, Optional, Union, Tuple

use zkay_ast::circuit_constraints::{
    CircCall, CircComment, CircEncConstraint, CircEqConstraint, CircGuardModification,
    CircIndentBlock, CircSymmEncConstraint, CircVarDecl, CircuitStatement,
};
use crate::circuit_generator::{
    CircuitGenerator, CircuitGeneratorBase, VerifyingKeyType,
};
use circuit_helper::circuit_helper::CircuitHelper;
use proving_scheme::backends::{
    gm17::ProvingSchemeGm17, groth16::ProvingSchemeGroth16,
};
use proving_scheme::proving_scheme::{
    G1Point, G2Point, ProvingScheme, VerifyingKeyMeta,
};

use jsnark_interface::jsnark_interface as jsnark;
use jsnark_interface::libsnark_interface as libsnark;
use zkay_utils::helpers::{hash_file, hash_string};
use zkay_utils::helpers::{read_file, save_to_file};
use zkay_ast::ast::{
    indent, is_instance, ASTType, BooleanLiteralExpr, BuiltinFunction, EnumDefinition, Expression,
    FunctionCallExpr, HybridArgumentIdf, IdentifierExpr, IndexExpr, IntoAST, MeExpr,
    MemberAccessExpr, NumberLiteralExpr, PrimitiveCastExpr, TypeName, AST,
};
use zkay_ast::homomorphism::Homomorphism;
use zkay_ast::visitor::visitor::AstVisitor;
use zkay_config::{config::CFG, zk_print};
use std::any::{Any, TypeId};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;
use zkp_u256::Binary;

pub fn is_type_id_of<S: ?Sized + Any>(s: TypeId) -> bool {
    TypeId::of::<S>() == s
}
pub fn _get_t(mut t: Option<AST>) -> String
// """Return the corresponding jsnark type name for a given type or expression."""
{
    let t = t.unwrap();
    let t = if let Some(t) = t.expr() {
        Some(*t.annotated_type().unwrap().type_name)
    } else {
        t.type_name()
    };
    assert!(t.is_some());
    let t = t.unwrap();
    let bits = t.elem_bitwidth();
    if bits == 1 {
        return String::from("ZkBool");
    }
    if t.is_signed_numeric() {
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
    pub fn new(phi: Vec<CircuitStatement>) -> Self
// super().__init__("node-or-children", false)
    {
        Self { phi }
    }
    pub fn visit(&self, _: AST) -> String {
        String::new()
    }
    pub fn visitCircuit(&self) -> Vec<String> {
        self.phi
            .iter()
            .map(|constr| self.visit(constr.to_ast()))
            .collect()
    }

    pub fn visitCircComment(&self, stmt: CircComment) -> String {
        if !stmt.text.is_empty() {
            format!(r#"// {}"#, stmt.text)
        } else {
            String::new()
        }
    }

    pub fn visitCircIndentBlock(&self, stmt: CircIndentBlock) -> String {
        let stmts: Vec<_> = stmt
            .statements
            .iter()
            .map(|s| self.visit(s.to_ast()))
            .collect();
        if !stmt.name.is_empty() {
            format!(
                r#"//[ --- {name} ---\n {} \n //] --- {name} ---\n"#,
                indent(stmts.join("\n")),
                name = stmt.name
            )
        } else {
            indent(stmts.join("\n"))
        }
    }

    pub fn visitCircCall(&self, stmt: CircCall) -> String {
        format!(r#"_{}();"#, stmt.fct.name())
    }

    pub fn visitCircVarDecl(&self, stmt: CircVarDecl) -> String {
        format!(
            r#"decl("{}", {});"#,
            stmt.lhs.identifier_base.name,
            self.visit(stmt.expr.to_ast())
        )
    }

    pub fn visitCircEqConstraint(&self, stmt: CircEqConstraint) -> String {
        assert!(stmt.tgt.t.size_in_uints() == stmt.val.t.size_in_uints());
        format!(
            r#"checkEq("{}", "{}");"#,
            stmt.tgt.identifier_base.name, stmt.val.identifier_base.name
        )
    }

    pub fn visitCircEncConstraint(&self, stmt: CircEncConstraint) -> String {
        assert!(stmt.cipher.t.is_cipher());
        assert!(stmt.pk.t.is_key());
        assert!(stmt.rnd.t.is_randomness());
        assert!(
            stmt.cipher.t.crypto_params() == stmt.pk.t.crypto_params()
                && stmt.pk.t.crypto_params() == stmt.rnd.t.crypto_params()
        );
        let backend = stmt.pk.t.crypto_params().unwrap().crypto_name;

        format!(
            r#"check{}("{backend}", "{}", "{}", "{}", "{}");"#,
            if stmt.is_dec { "Dec" } else { "Enc" },
            stmt.plain.identifier_base.name,
            stmt.pk.identifier_base.name,
            stmt.rnd.identifier_base.name,
            stmt.cipher.identifier_base.name
        )
    }
    pub fn visitCircSymmEncConstraint(&self, stmt: CircSymmEncConstraint) -> String {
        assert!(stmt.iv_cipher.t.is_cipher());
        assert!(stmt.other_pk.t.is_key());
        assert!(stmt.iv_cipher.t.crypto_params() == stmt.other_pk.t.crypto_params());
        let backend = stmt.other_pk.t.crypto_params().unwrap().crypto_name;
        format!(
            r#"checkSymm{}("{backend}", "{}", "{}", "{}");"#,
            if stmt.is_dec { "Dec" } else { "Enc" },
            stmt.plain.identifier_base.name,
            stmt.other_pk.identifier_base.name,
            stmt.iv_cipher.identifier_base.name
        )
    }
    pub fn visitCircGuardModification(&self, stmt: CircGuardModification) -> String {
        if let Some(new_cond) = &stmt.new_cond {
            format!(
                r#"addGuard("{}", {});"#,
                stmt.new_cond.unwrap().identifier_base.name,
                stmt.is_true
                    .map_or(String::new(), |v| v.to_string().to_ascii_lowercase())
            )
        } else {
            String::from("popGuard();")
        }
    }

    pub fn visitBooleanLiteralExpr(&self, ast: BooleanLiteralExpr) -> String {
        format!(r#"val({})"#, ast.value.to_string().to_ascii_lowercase())
    }

    pub fn visitNumberLiteralExpr(&self, ast: NumberLiteralExpr) -> String {
        let t = _get_t(Some(ast.to_ast()));
        if ast.value < (1 << 31) {
            format!(r#"val({}, {t})"#, ast.value)
        } else {
            format!(r#"val("{}", {t})"#, ast.value)
        }
    }

    pub fn visitIdentifierExpr(&self, ast: IdentifierExpr) -> String {
        if is_instance(&*ast.idf, ASTType::HybridArgumentIdf) && ast.idf.t().unwrap().is_cipher() {
            format!(r#"getCipher("{}")"#, ast.idf.name())
        } else {
            format!(r#"get("{}")"#, ast.idf.name())
        }
    }

    pub fn visitMemberAccessExpr(&self, ast: MemberAccessExpr) -> String {
        assert!(is_instance(&*ast.member, ASTType::HybridArgumentIdf));
        if ast.member.t().unwrap().is_cipher() {
            format!(r#"getCipher("{}")"#, ast.member.name())
        } else {
            assert!(ast.member.t().unwrap().size_in_uints() == 1);
            format!(r#"get("{}")"#, ast.member.name())
        }
    }

    pub fn visitIndexExpr(&self, ast: IndexExpr) {
        unimplemented!();
    }

    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) -> String {
        if is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction) {
            assert!(ast.func().unwrap().can_be_private());
            let mut args: Vec<_> = ast
                .args()
                .iter()
                .map(|arg| self.visit(arg.to_ast()))
                .collect();
            if ast.func().unwrap().is_shiftop() {
                assert!(ast.args()[1]
                    .annotated_type()
                    .unwrap()
                    .type_name
                    .is_literal());
                args[1] = ast.args()[1]
                    .annotated_type()
                    .unwrap()
                    .type_name
                    .value()
                    .to_string()
            }

            let mut op = &ast.func().unwrap().op().unwrap();
            let op = if op == "sign-" { "-" } else { op };
            if op == "sign+" {
                unimplemented!()
            }
            let homomorphism = ast.func().unwrap().homomorphism();
            let (f_start, crypto_backend, public_key_name) =
                if homomorphism == Homomorphism::non_homomorphic() {
                    (String::from("o_("), String::new(), String::new())
                } else {
                    let crypto_backend = CFG
                        .lock()
                        .unwrap()
                        .user_config
                        .get_crypto_params(&homomorphism)
                        .crypto_name;
                    let public_key_name = ast.public_key().unwrap().identifier_base.name;

                    args = args
                        .iter()
                        .map(|arg| format!(r#"HomomorphicInput.of({arg})"#))
                        .collect();
                    (
                        format!(r#"o_hom("{crypto_backend}", "{public_key_name}", "#),
                        crypto_backend,
                        public_key_name,
                    )
                };

            return if op == "ite" {
                format!(
                    r#"{f_start}{{{}}}, "?", {{{}}}, ":", {{{}}})"#,
                    args[0], args[1], args[2]
                )
            } else if op == "parenthesis" {
                String::from("({})")
            } else {
                let o = if op.len() == 1 {
                    format!(r#"'{op}'"#)
                } else {
                    format!(r#""{op}""#)
                };
                if args.len() == 1 {
                    format!(r#"{f_start}{o}, {{{}}})"#, args[0])
                } else {
                    assert!(args.len() == 2);
                    if op == "*" && ast.func().unwrap().rerand_using().is_some() {
                        // re-randomize homomorphic scalar multiplication
                        let rnd = self.visit(ast.func().unwrap().rerand_using().unwrap().to_ast());
                        format!(
                            r#"o_rerand({f_start}{{{}}}, {o}, {{{}}}), "{crypto_backend}", "{public_key_name}", {rnd})"#,
                            args[0], args[1]
                        )
                    } else {
                        format!(r#"{f_start}{{{}}}, {o}, {{{}}})"#, args[0], args[1])
                    }
                }
            };
        } else if ast.is_cast()
            && is_instance(
                &ast.func()
                    .unwrap()
                    .target()
                    .map(|v| Into::<AST>::into(*v))
                    .unwrap(),
                ASTType::EnumDefinition,
            )
        {
            assert!(ast.annotated_type().unwrap().type_name.elem_bitwidth() == 256);
            return self.handle_cast(self.visit(ast.args()[0].to_ast()), TypeName::uint_type());
        }

        // assert!(
        //     false,
        //     "Unsupported function {} inside circuit",
        //     ast.func().unwrap().code()
        // );
        String::new()
    }

    pub fn visitPrimitiveCastExpr(&self, ast: PrimitiveCastExpr) -> String {
        self.handle_cast(self.visit(ast.expr.to_ast()), *ast.elem_type)
    }

    pub fn handle_cast(&self, wire: String, t: TypeName) -> String {
        format!(r#"cast({wire}, {})"#, _get_t(Some(t.to_ast())))
    }
}

pub fn add_function_circuit_arguments(circuit: &CircuitHelper) -> Vec<String>
// """Generate java code which adds circuit IO as described by circuit"""
{
    let mut input_init_stmts = vec![];
    for sec_input in circuit.sec_idfs() {
        input_init_stmts.push(format!(
            r#"addS("{}", {}, {});"#,
            sec_input.identifier_base.name,
            sec_input.t.size_in_uints(),
            _get_t(Some(sec_input.t.to_ast()))
        ));
    }

    for pub_input in circuit.input_idfs() {
        input_init_stmts.push(if pub_input.t.is_key() {
            let backend = pub_input.t.crypto_params().unwrap().crypto_name;
            format!(
                r#"addK("{backend}", "{}", {});"#,
                pub_input.identifier_base.name,
                pub_input.t.size_in_uints()
            )
        } else {
            format!(
                r#"addIn("{}", {}, {});"#,
                pub_input.identifier_base.name,
                pub_input.t.size_in_uints(),
                _get_t(Some(pub_input.t.to_ast()))
            )
        });
    }
    for pub_output in circuit.output_idfs() {
        input_init_stmts.push(format!(
            r#"addOut("{}", {}, {});"#,
            pub_output.identifier_base.name,
            pub_output.t.size_in_uints(),
            _get_t(Some(pub_output.t.to_ast()))
        ));
    }

    let sec_input_names: Vec<_> = circuit
        .sec_idfs()
        .iter()
        .map(|sec_input| sec_input.identifier_base.name.clone())
        .collect();
    for crypto_params in &CFG.lock().unwrap().user_config.all_crypto_params() {
        let pk_name = CircuitHelper::get_glob_key_name(&MeExpr::new().to_ast(), crypto_params);
        let sk_name = CircuitHelper::get_own_secret_key_name(&crypto_params);
        if crypto_params.is_symmetric_cipher() && sec_input_names.contains(&sk_name) {
            assert!(circuit
                .input_idfs()
                .iter()
                .map(|pub_input| pub_input.identifier_base.name.clone())
                .collect::<Vec<_>>()
                .contains(&pk_name));
            input_init_stmts.push(format!(
                r#"setKeyPair("{}", "{pk_name}", "{sk_name}");"#,
                crypto_params.crypto_name
            ));
        }
    }

    input_init_stmts
}

// class JsnarkGenerator(CircuitGenerator)
pub struct JsnarkGenerator
//<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
{
    pub circuit_generator_base: CircuitGeneratorBase, //<T, VK>,
}

impl JsnarkGenerator
//<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
{
    pub fn new(circuits: Vec<CircuitHelper>, proving_scheme: String, output_dir: String) -> Self {
        Self {
            circuit_generator_base: CircuitGeneratorBase::new(
                circuits,
                proving_scheme,
                output_dir,
                false,
            ),
        }
    }

    pub fn _generate_zkcircuit(&self, import_keys: bool, circuit: &CircuitHelper) -> bool
//Create output directory
    {
        let p = self.circuit_generator_base._get_circuit_output_dir(circuit);
        let output_dir = Path::new(&p);
        if let Err(_) | Ok(false) = output_dir.try_exists() {
            std::fs::create_dir_all(output_dir).expect(output_dir.to_str().unwrap());
        }

        //Generate java code to add used crypto backends by calling addCryptoBackend
        let mut crypto_init_stmts = vec![];
        for params in &circuit.fct.used_crypto_backends.clone().unwrap() {
            let init_stmt = format!(
                r#"addCryptoBackend("{}", "{}", {});"#,
                params.crypto_name,
                params.crypto_name,
                params.key_bits()
            );
            crypto_init_stmts.push(init_stmt);
        }

        //Generate java code for all functions which are transitively called by the fct corresponding to this circuit
        //(outside private expressions)
        let mut fdefs = vec![];
        for fct in &circuit.transitively_called_functions {
            let target_circuit = &self.circuit_generator_base.circuits[fct];
            let body_stmts = JsnarkVisitor::new(target_circuit.phi()).visitCircuit();

            let body = [format!(r#"stepIn("{}");"#, fct.name())]
                .into_iter()
                .chain(add_function_circuit_arguments(target_circuit))
                .chain([String::new()])
                .chain(body_stmts)
                .chain([(String::from("stepOut();"))])
                .collect::<Vec<_>>()
                .join("\n");
            let fdef = format!(
                r#"private void _{name}() {{\n {body} \n}}"#,
                body = indent(body),
                name = fct.name()
            );
            fdefs.push(format!(r#"{fdef}"#))
        }

        //Generate java code for the function corresponding to this circuit
        let input_init_stmts = add_function_circuit_arguments(circuit);
        let constraints = JsnarkVisitor::new(circuit.phi()).visitCircuit();

        //Inject the function definitions into the java template
        let code = jsnark::get_jsnark_circuit_class_str(
            circuit,
            crypto_init_stmts,
            fdefs,
            input_init_stmts
                .iter()
                .cloned()
                .chain([String::new()])
                .chain(constraints)
                .collect(),
        );

        //Compute combined hash of the current jsnark interface jar and of the contents of the java file
        let hashfile = output_dir.join(format!(
            r#"{}.hash"#,
            CFG.lock().unwrap().jsnark_circuit_classname()
        ));
        let digest = hex::encode(hash_string(
            &(jsnark::CIRCUIT_BUILDER_JAR_HASH.to_string()
                + &code
                + &CFG.lock().unwrap().user_config.proving_scheme()),
        ));
        let oldhash = if let Ok(true) = hashfile.try_exists() {
            read_file(hashfile.to_str().unwrap())
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
                for f in self.circuit_generator_base._get_vk_and_pk_paths(circuit) {
                    if Path::new(&f).try_exists().map_or(false, |v| v) {
                        std::fs::remove_file(f);
                    }
                }
            }
            jsnark::compile_circuit(output_dir.to_str().unwrap(), &code);
            save_to_file(None, hashfile.to_str().unwrap(), &digest);
            true
        } else {
            zk_print!(
                r#"Circuit \"{}\" not modified, skipping compilation"#,
                circuit.get_verification_contract_name()
            );
            false
        }
    }
    pub fn _generate_keys(&self, circuit: &CircuitHelper)
    //Invoke the custom libsnark interface to generate keys
    {
        let output_dir = self.circuit_generator_base._get_circuit_output_dir(circuit);
        libsnark::generate_keys(
            &output_dir,
            &output_dir,
            &self.circuit_generator_base.proving_scheme,
        );
    }

    // @classmethod
    pub fn get_vk_and_pk_filenames() -> Vec<String> {
        ["verification.key", "proving.key", "verification.key.bin"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    pub fn _parse_verification_key(&self, circuit: &CircuitHelper) -> Option<VerifyingKeyType> {
        let p = &self.circuit_generator_base._get_vk_and_pk_paths(circuit)[0];
        let f = File::open(p).expect("");
        // data = iter(f.read().splitlines());
        let buf = BufReader::new(f);
        let mut data = buf.lines();
        if self.circuit_generator_base.proving_scheme.type_id()
            == TypeId::of::<ProvingSchemeGroth16>()
        {
            let a = G1Point::from_it(&mut data);
            let b = G2Point::from_it(&mut data);
            let gamma = G2Point::from_it(&mut data);
            let delta = G2Point::from_it(&mut data);
            let query_len = data.next().unwrap().unwrap().parse::<usize>().unwrap();
            let mut gamma_abc = vec![G1Point::default(); query_len];
            for idx in 0..query_len {
                gamma_abc.insert(idx, G1Point::from_it(&mut data));
            }
            return Some(VerifyingKeyType::ProvingSchemeGroth16(
                <ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX::new(
                    a, b, gamma, delta, gamma_abc,
                ),
            ));
        } else if self.circuit_generator_base.proving_scheme.type_id()
            == TypeId::of::<ProvingSchemeGm17>()
        {
            let h = G2Point::from_it(&mut data);
            let g_alpha = G1Point::from_it(&mut data);
            let h_beta = G2Point::from_it(&mut data);
            let g_gamma = G1Point::from_it(&mut data);
            let h_gamma = G2Point::from_it(&mut data);
            let query_len = data.next().unwrap().unwrap().parse::<usize>().unwrap();
            let mut query = vec![G1Point::default(); query_len];
            for idx in 0..query_len {
                query.insert(idx, G1Point::from_it(&mut data));
            }
            return Some(VerifyingKeyType::ProvingSchemeGm17(
                <ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX::new(
                    h, g_alpha, h_beta, g_gamma, h_gamma, query,
                ),
            ));
        }
        // else {
        //     unimplemented!()
        // }
        None
    }

    pub fn _get_prover_key_hash(&self, circuit: &CircuitHelper) -> Vec<u8> {
        hash_file(
            &self.circuit_generator_base._get_vk_and_pk_paths(circuit)[1],
            0,
        )
    }

    pub fn _get_primary_inputs(&self, circuit: &CircuitHelper) -> Vec<String>
//Jsnark requires an additional public input with the value 1 as first input
    {
        [String::from("1")]
            .into_iter()
            .chain(self.circuit_generator_base._get_primary_inputs(circuit))
            .collect()
    }
}
