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
pub struct SampleMulCircuit;
impl SampleMulCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new(
            "zk__Verify_Test_foo".to_owned(),
            Some("elgamal".to_owned()),
            Some("elgamal".to_owned()),
            508,
            6,
            4,
            0,
            true,
            Self,
        );
        // _self.add_crypto_backend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleMulCircuit>> {
    fn __zk__foo(&mut self) {
        self.step_in("_zk__foo");
        self.add_in("zk__in0_cipher_val", 4, ZkayType::zk_uint(256));
        self.add_out("zk__out0_cipher", 4, ZkayType::zk_uint(256));

        //[ --- val * 3 ---
        // zk__in0_cipher_val = val
        let o_hom_sshch = self.o_hom_sshch(
            "elgamal",
            "glob_key_Elgamal__owner",
            &HomomorphicInput::ofv(self.get_cipher("zk__in0_cipher_val").clone()),
            '*',
            &HomomorphicInput::of(
                self.cast(&self.val_iz(3, ZkayType::zk_uint(8)), ZkayType::zk_uint(32)),
            ),
        );
        self.decl_svt("tmp0_cipher", &o_hom_sshch);
        self.check_eq("tmp0_cipher", "zk__out0_cipher");
        //] --- val * 3 ---

        self.step_out();
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleMulCircuit>> {
    fn build_circuit(&mut self) {
        self.super_build_circuit();
        self.add_ki("elgamal", "glob_key_Elgamal__owner", 2);

        self.__zk__foo();
    }
    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        self.super_generate_sample_input(evaluator);
    }
    fn prep_files(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.super_prep_files(circuit_evaluator);
    }
}
impl ZkayCircuitBaseConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleMulCircuit>> {}
pub fn main(args: Vec<String>) {
    let mut circuit = SampleMulCircuit::new();
    circuit.run(&args);
}
