#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        structure::{circuit_generator::CGConfig, circuit_generator::CircuitGeneratorExtend},
    },
    zkay::{
        homomorphic_input::HomomorphicInput,
        zkay_circuit_base::{ZkayCircuitBase, ZkayCircuitBaseConfig},
        zkay_type::ZkayType,
    },
};

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SampleRehomCircuit;
impl SampleRehomCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new(
            "zk__Verify_Test_foo".to_owned(),
            Some("elgamal".to_owned()),
            Some("elgamal".to_owned()),
            508,
            16,
            4,
            5,
            true,
            Self,
        );
        // _self.add_crypto_backend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleRehomCircuit>> {
    fn __zk__foo(&mut self) {
        self.step_in("_zk__foo");
        self.add_s("secret0_rnd", 1, ZkayType::zk_uint(256));
        self.add_s("secret1_plain_x1", 1, ZkayType::zk_uint(32));
        self.add_s("zk__in1_cipher_x1_R", 1, ZkayType::zk_uint(256));
        self.add_in("zk__in0_cipher_b1", 4, ZkayType::zk_uint(256));
        self.add_in("zk__in1_cipher_x1", 4, ZkayType::zk_uint(256));
        self.add_out("zk__out0_cipher", 4, ZkayType::zk_uint(256));

        //[ --- b1 * reveal(x1, receiver) ---
        // zk__in0_cipher_b1 = b1
        // secret1_plain_x1 = dec(x1) [zk__in1_cipher_x1]
        self.check_dec(
            "elgamal",
            "secret1_plain_x1",
            "glob_key_Elgamal__me",
            "zk__in1_cipher_x1_R",
            "zk__in1_cipher_x1",
        );
        let o_hom_sshch = self.o_hom_sshch(
            "elgamal",
            "glob_key_Elgamal__receiver",
            &HomomorphicInput::ofv(self.get_cipher("zk__in0_cipher_b1").clone()),
            '*',
            &HomomorphicInput::of(self.get("secret1_plain_x1")),
        );
        let o_rerand = self.o_rerand(
            &o_hom_sshch,
            "elgamal",
            "glob_key_Elgamal__receiver",
            &self.get("secret0_rnd"),
        );
        self.decl_svt("tmp0_cipher", &o_rerand);
        self.check_eq("tmp0_cipher", "zk__out0_cipher");
        //] --- b1 * reveal(x1, receiver) ---

        self.step_out();
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleRehomCircuit>> {
    fn build_circuit(&mut self) {
        self.super_build_circuit();
        self.add_s("x1", 1, ZkayType::zk_uint(32));
        self.add_s("x1_R", 1, ZkayType::zk_uint(256));
        self.add_ki("elgamal", "glob_key_Elgamal__receiver", 2);
        self.add_ki("elgamal", "glob_key_Elgamal__me", 2);
        self.add_in("zk__in2_cipher_x1", 4, ZkayType::zk_uint(256));

        // zk__in2_cipher_x1 = enc(x1, glob_key_Elgamal__me)
        self.check_enc(
            "elgamal",
            "x1",
            "glob_key_Elgamal__me",
            "x1_R",
            "zk__in2_cipher_x1",
        );
        self.__zk__foo();
    }
    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        self.super_generate_sample_input(evaluator);
    }
    fn prep_files(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.super_prep_files(circuit_evaluator);
    }
}
impl ZkayCircuitBaseConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleRehomCircuit>> {}
pub fn main(args: Vec<String>) {
    let mut circuit = SampleRehomCircuit::new();
    circuit.run(&args);
}
