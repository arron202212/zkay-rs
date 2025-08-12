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
        // _self.addCryptoBackend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleRehomCircuit>> {
    fn __zk__foo(&mut self) {
        self.stepIn("_zk__foo");
        self.addS("secret0_rnd", 1, ZkayType::ZkUint(256));
        self.addS("secret1_plain_x1", 1, ZkayType::ZkUint(32));
        self.addS("zk__in1_cipher_x1_R", 1, ZkayType::ZkUint(256));
        self.addIn("zk__in0_cipher_b1", 4, ZkayType::ZkUint(256));
        self.addIn("zk__in1_cipher_x1", 4, ZkayType::ZkUint(256));
        self.addOut("zk__out0_cipher", 4, ZkayType::ZkUint(256));

        //[ --- b1 * reveal(x1, receiver) ---
        // zk__in0_cipher_b1 = b1
        // secret1_plain_x1 = dec(x1) [zk__in1_cipher_x1]
        self.checkDec(
            "elgamal",
            "secret1_plain_x1",
            "glob_key_Elgamal__me",
            "zk__in1_cipher_x1_R",
            "zk__in1_cipher_x1",
        );
        let o_hom_sshch = self.o_hom_sshch(
            "elgamal",
            "glob_key_Elgamal__receiver",
            &HomomorphicInput::ofv(self.getCipher("zk__in0_cipher_b1").clone()),
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
        self.checkEq("tmp0_cipher", "zk__out0_cipher");
        //] --- b1 * reveal(x1, receiver) ---

        self.stepOut();
    }
}
impl CGConfig for CircuitGeneratorExtend<ZkayCircuitBase<SampleRehomCircuit>> {
    fn buildCircuit(&mut self) {
        self.super_buildCircuit();
        self.addS("x1", 1, ZkayType::ZkUint(32));
        self.addS("x1_R", 1, ZkayType::ZkUint(256));
        self.addKi("elgamal", "glob_key_Elgamal__receiver", 2);
        self.addKi("elgamal", "glob_key_Elgamal__me", 2);
        self.addIn("zk__in2_cipher_x1", 4, ZkayType::ZkUint(256));

        // zk__in2_cipher_x1 = enc(x1, glob_key_Elgamal__me)
        self.checkEnc(
            "elgamal",
            "x1",
            "glob_key_Elgamal__me",
            "x1_R",
            "zk__in2_cipher_x1",
        );
        self.__zk__foo();
    }
    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        self.super_generateSampleInput(evaluator);
    }
    fn prepFiles(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        self.super_prepFiles(circuit_evaluator);
    }
}

pub fn main(args: Vec<String>) {
    let mut circuit = SampleRehomCircuit::new();
    circuit.run(&args);
}
