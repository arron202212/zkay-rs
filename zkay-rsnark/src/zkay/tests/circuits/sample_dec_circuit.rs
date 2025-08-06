#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::zkay::zkay_circuit_base;
// use crate::zkay::zkay_type::zk_uint;
use crate::circuit::structure::circuit_generator::CircuitGeneratorExtend;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::zkay_circuit_base::ZkayCircuitBase;

pub struct SampleDecCircuit;
impl SampleDecCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new("zk__Verify_Test_bar", 6, 1, 2, true);
        _self.addCryptoBackend("elgamal", "elgamal", 508);
        _self
    }
}

impl CircuitGeneratorExtend<ZkayCircuitBase<SampleDecCircuit>> {
    fn __zk__bar(&self) {
        self.stepIn("_zk__bar");
        self.addS("secret0_plain_val", 1, ZkUint(32));
        self.addS("zk__in0_cipher_val_R", 1, ZkUint(256));
        self.addIn("zk__in0_cipher_val", 4, ZkUint(256));
        self.addOut("zk__out0_plain_val", 1, ZkUint(32));

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

    fn buildCircuit(&mut self) {
        // super.buildCircuit();
        self.addK("elgamal", "glob_key_Elgamal__me", 2);

        self.__zk__bar();
    }
}

pub fn main(args: Vec<String>) {
    let circuit = SampleDecCircuit::new();
    circuit.run(args);
}
