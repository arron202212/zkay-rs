// """
// This module defines the verification key, proof and verification contract format for the Groth16 proving scheme

// See "On the Size of Pairing-based Non-interactive Arguments", Jens Groth, IACR-EUROCRYPT-2016
// https://eprint.iacr.org/2016/260
// """

// from typing import List

use crate::config::CFG;

use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::compiler::privacy::library_contracts::{bn128_scalar_field, bn128_scalar_field_bits};
use crate::compiler::privacy::proving_scheme::proving_scheme::{
    G1Point, G2Point, ProvingScheme, VerifyingKey as VK,
};
use crate::utils::multiline_formatter::MultiLineFormatter;

struct VerifyingKey {
    a: G1Point,
    b: G2Point,
    gamma: G2Point,
    delta: G2Point,
    gamma_abc: Vec<G1Point>,
}
impl VerifyingKey {
    // class VerifyingKey(VerifyingKey):
    pub fn new(
        a: G1Point,
        b: G2Point,
        gamma: G2Point,
        delta: G2Point,
        gamma_abc: Vec<G1Point>,
    ) -> Self {
        Self {
            a,
            b,
            gamma,
            delta,
            gamma_abc,
        }
    }
}
impl VK for VerifyingKey {
    // @classmethod
    fn create_dummy_key() -> Self {
        let p1 = G1Point('0', '0');
        let p2 = G2Point('0', '0', '0', '0');
        Self::new(p1.clone(), p2.clone(), p2.clone(), p2, vec![p1.clone(), p1])
    }
}
// class ProvingSchemeGroth16(ProvingScheme):
pub struct ProvingSchemeGroth16;
impl ProvingScheme for ProvingSchemeGm17 {
    const NAME: &'static str = "groth16";
    type VerificationKey = VerifyingKey;

    fn generate_verification_contract(
        self,
        verification_key: VerifyingKey,
        circuit: CircuitHelper,
        primary_inputs: Vec<String>,
        prover_key_hash: bytes,
    ) -> String {
        let vk = verification_key;
        let should_hash = cfg.should_use_hash(circuit);

        let query_length = vk.gamma_abc.len();
        assert!(query_length == len(primary_inputs) + 1);

        assert!(primary_inputs, "No public inputs");
        let first_pi = primary_inputs[0].clone();
        let potentially_overflowing_pi = primary_inputs
            .iter()
            .filter_map(|pi| {
                if !['1', self.hash_var_name].contains(pi) {
                    Some(pi)
                } else {
                    None
                }
            })
            .collect();

        // Verification contract uses the pairing library from ZoKrates (MIT license)
        // https://github.com/Zokrates/ZoKrates/blob/d8cde9e1c060cc654413f01c8414ea4eaa955d87/zokrates_core/src/proof_system/bn128/utils/solidity.rs#L398
        let x = MultiLineFormatter::new("").mul(format!(r#"
        pragma solidity {zkay_solc_version_compatibility};

        import {{ Pairing, G1Point as G1, G2Point as G2 }} from "{verify_libs_contract_filename}";

        contract {get_verification_contract_name} {{"#,zkay_solc_version_compatibility=cfg.zkay_solc_version_compatibility,verify_libs_contract_filename=ProvingScheme.verify_libs_contract_filename,get_verification_contract_name=circuit.get_verification_contract_name())).div(format!(r#"
            using Pairing for G1;
            using Pairing for G2;

            bytes32 public constant {prover_key_hash_name} = 0x{prover_key_hash};
            uint256 constant {snark_scalar_field_var_name} = {bn128_scalar_field};

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

            function getVk() pure internal returns(Vk memory vk) {{"#,prover_key_hash_name=cfg.prover_key_hash_name,prover_key_hash=prover_key_hash.hex(),snark_scalar_field_var_name=self.snark_scalar_field_var_name)).div(format!(r#"
                vk.a_neg = G1({a});
                vk.b = G2({b});
                vk.gamma = G2({gamma});
                vk.delta = G2({delta});"#,a=vk.a.negated(),b=vk.b,gamma=vk.gamma,delta=vk.delta)).mul(
vk.gamma_abc.iter().enumerate().map(|(idx, G )|format!("vk.gamma_abc[{idx}] = G1({g});")).collect()).floordiv(
            format!(r#"
            }}

            function {verification_function_name}(uint[8] memory proof_, uint[] memory {zk_in_name}, uint[] memory {zk_out_name}) public {{"#,verification_function_name=cfg.verification_function_name,zk_in_name=cfg.zk_in_name,zk_out_name=cfg.zk_out_name)).div(format!(r#"
                // Check if input size correct
                require({zk_in_name}.length == {in_size_trans});

                // Check if output size correct
                require({zk_out_name}.length == {out_size_trans});"#, zk_in_name=cfg.zk_in_name,in_size_trans=circuit.in_size_trans,zk_out_name=cfg.zk_out_name,out_size_trans=circuit.out_size_trans)).mul(if potentially_overflowing_pi.is_empty(){String::new()}else{format!("
                \n// Check that inputs do not overflow\n{}\n",
potentially_overflowing_pi.iter().map(|pi| format!("require({pi} < {});",self.snark_scalar_field_var_name)).collect::<Vec<_>>().concat())}).mul(r#"
                // Create proof and vk data structures
                Proof memory proof;
                proof.a = G1(proof_[0], proof_[1]);
                proof.b = G2([proof_[2], proof_[3]], [proof_[4], proof_[5]]);
                proof.c = G1(proof_[6], proof_[7]);
                Vk memory vk = getVk();

                // Compute linear combination of public inputs"#).mul(if should_hash.is_empty(){String::new()}else{
                format!("uint256 {} = uint256(sha256(abi.encodePacked({}, {})) >> {});",self.hash_var_name,cfg.zk_in_name,cfg.zk_out_name,256 - bn128_scalar_field_bits) }).mul(
                format!("G1 memory lc = {};",if first_pi != "1"{format!("vk.gamma_abc[1].scalar_mul({})",first_pi)}  else {String::from("vk.gamma_abc[1]")})).mul(format!(
    "lc = lc.add({}); ",if pi != "1"{format!("vk.gamma_abc[{}].scalar_mul({pi})",idx + 2)}else{
                 primary_inputs[1..].iter().enumerate().map(|(idx, pi )| format!("vk.gamma_abc[{}]",idx+2)).collect::<Vec<_>>().concat()})).mul(r#"
                lc = lc.add(vk.gamma_abc[0]);

                // Verify proof
                require(Pairing.pairingProd4(proof.a, proof.b,
                                lc.negate(), vk.gamma,
                                proof.c.negate(), vk.delta,
                                vk.a_neg, vk.b), "invalid proof");"#).floordiv(
            "}").floordiv(
        "}");

        x.text.clone()
    }
}
