// """
// This module defines the verification key, proof and verification contract format for the GM17 proving scheme

// See "Snarky Signatures: Minimal Signatures of Knowledge from Simulation-Extractable SNARKs", Jens Groth and Mary Maller, IACR-CRYPTO-2017
// https://eprint.iacr.org/2017/540
// """

// from typing import List

use crate::config::CFG;

use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::compiler::privacy::library_contracts::{BN128_SCALAR_FIELD, BN128_SCALAR_FIELD_BITS};
use crate::compiler::privacy::proving_scheme::proving_scheme::{
    G1Point, G2Point, ProvingScheme, VerifyingKeyMeta as VK,
};
use crate::utils::multiline_formatter::MultiLineFormatter;

pub struct VerifyingKey<G1: Default, G2: Default> {
    h: G2,
    g_alpha: G1,
    h_beta: G2,
    g_gamma: G1,
    h_gamma: G2,
    query: Vec<G1>,
}
//  class VerifyingKey(VerifyingKey):
impl<G1: Default, G2: Default> VerifyingKey<G1, G2> {
    pub fn new(h: G2, g_alpha: G1, h_beta: G2, g_gamma: G1, h_gamma: G2, query: Vec<G1>) -> Self {
        Self {
            h,
            g_alpha,
            h_beta,
            g_gamma,
            h_gamma,
            query,
        }
    }
}
impl VK for <ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX
// where PS:ProvingScheme<VerifyingKeyX=<ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX>,
{
    type Output = Self;
    type G1 = G1Point;
    type G2 = G2Point;
    // @classmethod
    fn create_dummy_key() -> Self::Output {
        let p1 = Self::G1::default();
        let p2 = Self::G2::default();
        Self::new(
            p2.clone(),
            p1.clone(),
            p2.clone(),
            p1.clone(),
            p2.clone(),
            vec![p1; 2],
        )
    }
}

// class ProvingSchemeGm17(ProvingScheme):
pub struct ProvingSchemeGm17;
impl ProvingScheme for ProvingSchemeGm17 {
    const NAME: &'static str = "gm17";
    type VerifyingKeyX = VerifyingKey<G1Point, G2Point>;

    fn generate_verification_contract(
        verification_key: <ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX,
        circuit: &CircuitHelper,
        primary_inputs: Vec<String>,
        prover_key_hash: Vec<u8>,
    ) -> String {
        let vk = verification_key;
        let should_hash = CFG
            .lock()
            .unwrap()
            .should_use_hash(circuit.trans_in_size + circuit.trans_out_size);

        let query_length = vk.query.len();
        assert!(query_length == primary_inputs.len() + 1);

        assert!(!primary_inputs.is_empty(), "No public inputs");
        let first_pi = &primary_inputs[0];
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

        //Verification contract uses the pairing library from ZoKrates (MIT license)
        //https://github.com/Zokrates/ZoKrates/blob/d8cde9e1c060cc654413f01c8414ea4eaa955d87/zokrates_core/src/proof_system/bn128/utils/solidity.rs#L398
        let x = MultiLineFormatter::new("").mul(format!(r#"
        pragma solidity {zkay_solc_version_compatibility};

        import {{ Pairing, G1Point as G1, G2Point as G2 }} from "{verify_libs_contract_filename}";

        contract {get_verification_contract_name} {{"#,zkay_solc_version_compatibility=CFG.lock().unwrap().zkay_solc_version_compatibility(),verify_libs_contract_filename=<Self as ProvingScheme>::verify_libs_contract_filename(),get_verification_contract_name=circuit.get_verification_contract_name())).truediv(format!(r#"
            using Pairing for G1;
            using Pairing for G2;

            bytes32 public constant {prover_key_hash_name} = 0x{prover_key_hash};
            uint256 constant {snark_scalar_field_var_name} = {BN128_SCALAR_FIELD:?};

            struct Proof {{
                G1 a;
                G2 b;
                G1 c;
            }}

            struct Vk {{
                G2 h;
                G1 g_alpha;
                G2 h_beta;
                G1 g_gamma_neg;
                G2 h_gamma;
                G1[{query_length}] query;
            }}

            function getVk() pure internal returns(Vk memory vk) {{"#,prover_key_hash_name=CFG.lock().unwrap().prover_key_hash_name(),prover_key_hash=hex::encode(prover_key_hash),snark_scalar_field_var_name=<Self as ProvingScheme>::snark_scalar_field_var_name())).truediv(format!(r#"
                vk.h = G2({h});
                vk.g_alpha = G1({g_alpha});
                vk.h_beta = G2({h_beta});
                vk.g_gamma_neg = G1({g_gamma});
                vk.h_gamma = G2({h_gamma});"#,h=vk.h,g_alpha=vk.g_alpha,h_beta=vk.h_beta,g_gamma=vk.g_gamma.negated(),h_gamma=vk.h_gamma)).mul(vk.query.iter().enumerate().map(|(idx, q )|format!("vk.query[{idx}] = G1({q});")).collect()).floordiv(
            format!(r#"
            }}

            function {verification_function_name}(uint[8] memory proof_, uint[] memory {zk_in_name}, uint[] memory {zk_out_name}) public {{"#,verification_function_name=CFG.lock().unwrap().verification_function_name(),zk_in_name=CFG.lock().unwrap().zk_in_name(),zk_out_name=CFG.lock().unwrap().zk_out_name())).truediv(format!(r#"
                // Check if input size correct
                require({zk_in_name}.length == {in_size_trans}, "Wrong public input length");

                // Check if output size correct
                require({zk_out_name}.length == {out_size_trans}, "Wrong public output length");"#, zk_in_name=CFG.lock().unwrap().zk_in_name(),in_size_trans=circuit.in_size_trans(),zk_out_name=CFG.lock().unwrap().zk_out_name(),out_size_trans=circuit.out_size_trans())).mul(if potentially_overflowing_pi.is_empty(){String::new()}else{format!("
                \n// Check that inputs do not overflow\n{}\n",
potentially_overflowing_pi.iter().map(|pi| format!("require({pi} < {});",<Self as ProvingScheme>::snark_scalar_field_var_name())).collect::<Vec<_>>().concat())}).mul(String::from(r#"
                // Create proof and vk data structures
                Proof memory proof;
                proof.a = G1(proof_[0], proof_[1]);
                proof.b = G2([proof_[2], proof_[3]], [proof_[4], proof_[5]]);
                proof.c = G1(proof_[6], proof_[7]);
                Vk memory vk = getVk();

                // Compute linear combination of public inputs"#)).mul(if should_hash{String::new()}else{
                format!("uint256 {} = uint256(sha256(abi.encodePacked({}, {})) >> {});",<Self as ProvingScheme>::hash_var_name(),CFG.lock().unwrap().zk_in_name(),CFG.lock().unwrap().zk_out_name(),256usize - *BN128_SCALAR_FIELD_BITS) }).mul(
                format!("G1 memory lc = {};",if first_pi != "1"{format!("vk.query[1].scalar_mul({})",first_pi)}  else {String::from("vk.query[1]")})).mul(
                 primary_inputs[1..].iter().enumerate().map(|(idx, pi )| format!(
    "lc = lc.add({}); ",format!("vk.query[{}]{}",idx+2,if pi != "1"{format!(".scalar_mul({pi})")}else{String::new()}))).collect::<Vec<_>>().concat()).mul(r#"
                lc = lc.add(vk.query[0]);

                // Verify proof
                require(Pairing.pairingProd2(proof.a, vk.h_gamma,
                                             vk.g_gamma_neg, proof.b), "invalid proof 1/2");
                require(Pairing.pairingProd4(vk.g_alpha, vk.h_beta,
                                             lc, vk.h_gamma,
                                             proof.c, vk.h,
                                             proof.a.add(vk.g_alpha).negate(), proof.b.add(vk.h_beta)), "invalid proof 2/2");"#.to_string()).floordiv(
             String::from("}")).floordiv(
        String::from("}"));

        x.str()
    }
}
