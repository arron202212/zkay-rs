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
use crate::zkay::zkay_circuit_base::ZkayCircuitBase;
use crate::zkay::zkay_type::ZkayType;
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
        // _self.addCryptoBackend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleDecCircuit>> {
    fn __zk__bar(&mut self) {
        self.stepIn("_zk__bar");
        self.addS("secret0_plain_val", 1, ZkayType::ZkUint(32));
        self.addS("zk__in0_cipher_val_R", 1, ZkayType::ZkUint(256));
        self.addIn("zk__in0_cipher_val", 4, ZkayType::ZkUint(256));
        self.addOut("zk__out0_plain_val", 1, ZkayType::ZkUint(32));

        //[ --- val ---
        // secret0_plain_val = dec(val) [zk__in0_cipher_val]
        self.checkDec(
            "elgamal",
            "secret0_plain_val",
            "glob_key_Elgamal__me",
            "zk__in0_cipher_val_R",
            "zk__in0_cipher_val",
        );
        self.decl("tmp0_plain", self.get("secret0_plain_val"));
        self.checkEq("tmp0_plain", "zk__out0_plain_val");
        //] --- val ---

        self.stepOut();
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleDecCircuit>> {
    fn buildCircuit(&mut self) {
        self.super_buildCircuit();
        self.addKi("elgamal", "glob_key_Elgamal__me", 2);

        self.__zk__bar();
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        self.super_generateSampleInput(evaluator);
    }
    fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.super_prepFiles(circuit_evaluator);
    }
}

pub fn main(args: Vec<String>) {
    let mut circuit = SampleDecCircuit::new();
    circuit.run(&args);
}
