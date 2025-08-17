#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::CircuitGeneratorExtend;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::zkay_circuit_base::{ZkayCircuitBase, ZkayCircuitBaseConfig};
use crate::zkay::zkay_type::ZkayType;
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SampleEncCircuit;
impl SampleEncCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new(
            "zk__Verify_Test_foo".to_owned(),
            Some("elgamal".to_owned()),
            Some("elgamal".to_owned()),
            508,
            6,
            8,
            1,
            true,
            Self,
        );
        // _self.addCryptoBackend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleEncCircuit>> {
    fn __zk__foo(&mut self) {
        self.stepIn("_zk__foo");
        self.addS("zk__out0_cipher_R", 1, ZkayType::ZkUint(256));
        self.addIn("zk__in0_cipher_val", 4, ZkayType::ZkUint(256));
        self.addOut("zk__out0_cipher", 4, ZkayType::ZkUint(256));
        self.addOut("zk__out1_cipher", 4, ZkayType::ZkUint(256));

        //[ --- val + reveal<+>(3, owner) ---
        // zk__in0_cipher_val = val
        //[ --- 3 ---
        self.decl(
            "tmp0_plain",
            self.cast(&self.val_iz(3, ZkayType::ZkUint(8)), ZkayType::ZkUint(32)),
        );
        // zk__out0_cipher = enc(tmp0_plain, glob_key_Elgamal__owner)
        self.checkEnc(
            "elgamal",
            "tmp0_plain",
            "glob_key_Elgamal__owner",
            "zk__out0_cipher_R",
            "zk__out0_cipher",
        );
        //] --- 3 ---
        let o_hom_sshch = self.o_hom_sshch(
            "elgamal",
            "glob_key_Elgamal__owner",
            &HomomorphicInput::ofv(self.getCipher("zk__in0_cipher_val").clone()),
            '+',
            &HomomorphicInput::ofv(self.getCipher("zk__out0_cipher").clone()),
        );
        self.decl_svt("tmp1_cipher", &o_hom_sshch);
        self.checkEq("tmp1_cipher", "zk__out1_cipher");
        //] --- val + reveal<+>(3, owner) ---

        self.stepOut();
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleEncCircuit>> {
    fn buildCircuit(&mut self) {
        println!("======buildCircuit====={}", file!());
        self.super_buildCircuit();
        self.addKi("elgamal", "glob_key_Elgamal__owner", 2);

        self.__zk__foo();
    }
    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        self.super_generateSampleInput(evaluator);
    }
    fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.super_prepFiles(circuit_evaluator);
    }
}
impl ZkayCircuitBaseConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleEncCircuit>> {}
pub fn main(args: Vec<String>) {
    let mut circuit = SampleEncCircuit::new();
    println!("======SampleEncCircuit===run=========");
    circuit.run(&args);
}
