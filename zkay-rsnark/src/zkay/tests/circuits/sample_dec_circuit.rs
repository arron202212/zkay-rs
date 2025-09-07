#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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
pub struct SampleDecCircuit;
impl SampleDecCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new(
            "zk__Verify_Test_bar".to_owned(),
            Some("elgamal".to_owned()),
            Some("elgamal".to_owned()),
            508,
            6,
            1,
            2,
            true,
            Self,
        );
        // _self.add_crypto_backend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleDecCircuit>> {
    fn __zk__bar(&mut self) {
        self.step_in("_zk__bar");
        self.add_s("secret0_plain_val", 1, ZkayType::zk_uint(32));
        self.add_s("zk__in0_cipher_val_R", 1, ZkayType::zk_uint(256));
        self.add_in("zk__in0_cipher_val", 4, ZkayType::zk_uint(256));
        self.add_out("zk__out0_plain_val", 1, ZkayType::zk_uint(32));

        //[ --- val ---
        // secret0_plain_val = dec(val) [zk__in0_cipher_val]
        self.check_dec(
            "elgamal",
            "secret0_plain_val",
            "glob_key_Elgamal__me",
            "zk__in0_cipher_val_R",
            "zk__in0_cipher_val",
        );
        self.decl("tmp0_plain", self.get("secret0_plain_val"));
        self.check_eq("tmp0_plain", "zk__out0_plain_val");
        //] --- val ---

        self.step_out();
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleDecCircuit>> {
    fn build_circuit(&mut self) {
        self.super_build_circuit();
        self.add_ki("elgamal", "glob_key_Elgamal__me", 2);

        self.__zk__bar();
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        self.super_generate_sample_input(evaluator);
    }
    fn prep_files(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.super_prep_files(circuit_evaluator);
    }
}
impl ZkayCircuitBaseConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleDecCircuit>> {}
pub fn main(args: Vec<String>) {
    let mut circuit = SampleDecCircuit::new();
    circuit.run(&args);
}
