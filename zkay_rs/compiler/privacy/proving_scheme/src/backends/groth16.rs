#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// """
// This module defines the verification key, proof and verification contract format for the Groth16 proving scheme

// See "On the Size of Pairing-based Non-interactive Arguments", Jens Groth, IACR-EUROCRYPT-2016
// https://eprint.iacr.org/2016/260
// """

// from typing import List
use rccell::RcCell;
use zkay_config::{config::CFG, config_version::Versions};

use crate::proving_scheme::{G1Point, G2Point, ProvingScheme, VerifyingKeyMeta as VK};
use circuit_helper::circuit_helper::CircuitHelper;
use circuit_helper_config::circuit_helper_config::CircuitHelperConfig;
use privacy::library_contracts::BN128_SCALAR_FIELD_BITS;
use zkay_utils::multiline_formatter::MultiLineFormatter;
// use zkp_u256::{Zero, U256};
pub struct VerifyingKey<G1: Default, G2: Default> {
    a: G1,
    b: G2,
    gamma: G2,
    delta: G2,
    gamma_abc: Vec<G1>,
}
impl<G1: Default, G2: Default> VerifyingKey<G1, G2> {
    // class VerifyingKey(VerifyingKey):
    pub fn new(a: G1, b: G2, gamma: G2, delta: G2, gamma_abc: Vec<G1>) -> Self {
        Self {
            a,
            b,
            gamma,
            delta,
            gamma_abc,
        }
    }
}
impl VK for <ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX {
    // where PS:ProvingScheme<VerifyingKeyX=<ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX>,

    type Output = Self;
    type G1 = G1Point;
    type G2 = G2Point;
    // @classmethod
    fn create_dummy_key() -> Self::Output {
        let p1 = Self::G1::default();
        let p2 = Self::G2::default();
        Self::new(p1.clone(), p2.clone(), p2.clone(), p2, vec![p1.clone(), p1])
    }
}
// class ProvingSchemeGroth16(ProvingScheme):
pub struct ProvingSchemeGroth16;
impl ProvingScheme for ProvingSchemeGroth16 {
    const NAME: &'static str = "groth16";
    type VerifyingKeyX = VerifyingKey<G1Point, G2Point>;

    fn generate_verification_contract<C: CircuitHelperConfig>(
        verification_key: <ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX,
        circuit: &C,
        primary_inputs: Vec<String>,
        prover_key_hash: Vec<u8>,
    ) -> String {
        let vk = verification_key;
        let should_hash = CFG
            .unwrap()
            .should_use_hash(circuit.trans_in_size() + circuit.trans_out_size());
        println!(
            "=====len============{},{}",
            vk.gamma_abc.len(),
            primary_inputs.len()
        );
        let query_length = vk.gamma_abc.len();
        assert!(query_length == primary_inputs.len() + 1);

        assert!(!primary_inputs.is_empty(), "No public inputs");
        let first_pi = primary_inputs[0].clone();
        let potentially_overflowing_pi: Vec<_> = primary_inputs
            .iter()
            .filter_map(|pi| {
                if ![String::from("1"), <Self as ProvingScheme>::hash_var_name()].contains(pi) {
                    Some(pi)
                } else {
                    None
                }
            })
            .collect();

        // Verification contract uses the pairing library from ZoKrates (MIT license)
        // https://github.com/Zokrates/ZoKrates/blob/d8cde9e1c060cc654413f01c8414ea4eaa955d87/zokrates_core/src/proof_system/bn128/utils/solidity.rs#L398
        let zkay_solc_version_compatibility = CFG.lock().unwrap().zkay_solc_version_compatibility();
        let verify_libs_contract_filename =
            <Self as ProvingScheme>::verify_libs_contract_filename();
        let get_verification_contract_name = circuit.get_verification_contract_name();
        let prover_key_hash_name = CFG.lock().unwrap().prover_key_hash_name();
        let prover_key_hash = hex::encode(prover_key_hash);
        let snark_scalar_field_var_name = <Self as ProvingScheme>::snark_scalar_field_var_name();
        let zk_in_name = CFG.lock().unwrap().zk_in_name();
        let in_size_trans = circuit.in_size_trans();
        let zk_out_name = CFG.lock().unwrap().zk_out_name();
        let out_size_trans = circuit.out_size_trans();
        let verification_function_name = CFG.lock().unwrap().verification_function_name();
        let bn128_scalar_field_value: &str =
            "21888242871839275222246405745257275088548364400416034343698204186575808495617"; //BN128_SCALAR_FIELD.clone();
        // println!("===ddddd====={}",line!());
        let x = MultiLineFormatter::new("").mul(format!(r#"
        pragma solidity {zkay_solc_version_compatibility};

        import {{ Pairing, G1Point as G1, G2Point as G2 }} from "{verify_libs_contract_filename}";

        contract {get_verification_contract_name} {{"#)).truediv(format!(r#"
            using Pairing for G1;
            using Pairing for G2;

            bytes32 public constant {prover_key_hash_name} = 0x{prover_key_hash};
            uint256 constant {snark_scalar_field_var_name} = {bn128_scalar_field_value};

            struct Proof {{
                G1 a;
                G2 b;
                G1 c;
            }}

            struct Vk {{
                G1 a_neg;
                G2 b;
                G2 gamma;
                G2 delta;
                G1[{query_length}] gamma_abc;
            }}

            function getVk() pure internal returns(Vk memory vk) {{"#)).truediv(format!(r#"
                vk.a_neg = G1({a});
                vk.b = G2({b});
                vk.gamma = G2({gamma});
                vk.delta = G2({delta});"#,a=vk.a.negated(),b=vk.b,gamma=vk.gamma,delta=vk.delta)).mul(
vk.gamma_abc.iter().enumerate().map(|(idx, g )|format!("vk.gamma_abc[{idx}] = G1({g});\n")).collect()).floordiv(
            format!(r#"
            }}

            function {verification_function_name}(uint[8] memory proof_, uint[] memory {zk_in_name}, uint[] memory {zk_out_name}) public {{"#)).truediv(format!(r#"
                // Check if input size correct
                require({zk_in_name}.length == {in_size_trans});

                // Check if output size correct
                require({zk_out_name}.length == {out_size_trans});"#)).mul(if potentially_overflowing_pi.is_empty(){String::new()}else{format!("
                \n// Check that inputs do not overflow\n{}\n",
potentially_overflowing_pi.iter().map(|pi| format!("require({pi} < {});",<Self as ProvingScheme>::snark_scalar_field_var_name())).collect::<Vec<_>>().concat())}).mul(r#"
                // Create proof and vk data structures
                Proof memory proof;
                proof.a = G1(proof_[0], proof_[1]);
                proof.b = G2([proof_[2], proof_[3]], [proof_[4], proof_[5]]);
                proof.c = G1(proof_[6], proof_[7]);
                Vk memory vk = getVk();

                // Compute linear combination of public inputs"#.to_string()).mul(if should_hash{
                format!("uint256 {} = uint256(sha256(abi.encodePacked({}, {})) >> {});",<Self as ProvingScheme>::hash_var_name(),zk_in_name,zk_out_name,256 - *BN128_SCALAR_FIELD_BITS) }else{String::new()}).mul(
                format!("G1 memory lc = {};",if first_pi != "1"{format!("vk.gamma_abc[1].scalar_mul({})",first_pi)}  else {String::from("vk.gamma_abc[1]")})).mul( primary_inputs[1..].iter().enumerate().map(|(idx, pi )| format!(
    "lc = lc.add({}); ",format!("vk.gamma_abc[{}]{}",idx+2,if pi != "1"{format!(".scalar_mul({pi})")}else{String::new()}))).collect::<Vec<_>>().concat()).mul(r#"
                lc = lc.add(vk.gamma_abc[0]);

                // Verify proof
                require(Pairing.pairingProd4(proof.a, proof.b,
                                lc.negate(), vk.gamma,
                                proof.c.negate(), vk.delta,
                                vk.a_neg, vk.b), "invalid proof");"#.to_string()).floordiv(
            String::from("}")).floordiv(
        String::from("}"));

        x.text.clone()
    }
}
