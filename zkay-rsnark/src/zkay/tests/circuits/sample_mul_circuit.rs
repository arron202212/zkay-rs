#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::circuit::structure::circuit_generator::CircuitGeneratorExtend;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::zkay_circuit_base::ZkayCircuitBase;
use crate::zkay::zkay_type::ZkayType;
#[derive(Debug, Clone)]
pub struct SampleMulCircuit;
impl SampleMulCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new("zk__Verify_Test_foo", 6, 4, 0, true);
        _self.addCryptoBackend("elgamal", "elgamal", 508);
        _self
    }
}
impl CircuitGeneratorExtend<ZkayCircuitBase<SampleMulCircuit>> {
    fn __zk__foo(&self) {
        self.stepIn("_zk__foo");
        self.addIn("zk__in0_cipher_val", 4, ZkayType::ZkUint(256));
        self.addOut("zk__out0_cipher", 4, ZkayType::ZkUint(256));

        //[ --- val * 3 ---
        // zk__in0_cipher_val = val
        self.decl(
            "tmp0_cipher",
            self.o_hom(
                "elgamal",
                "glob_key_Elgamal__owner",
                HomomorphicInput::of(self.getCipher("zk__in0_cipher_val")),
                '*',
                HomomorphicInput::of(self.cast(val(3, ZkayType::ZkUint(8)), ZkayType::ZkUint(32))),
            ),
        );
        self.checkEq("tmp0_cipher", "zk__out0_cipher");
        //] --- val * 3 ---

        self.stepOut();
    }

    fn buildCircuit(&mut self) {
        // super.buildCircuit();
        self.addK("elgamal", "glob_key_Elgamal__owner", 2);

        self.__zk__foo();
    }
}
pub fn main(args: Vec<String>) {
    let circuit = SampleMulCircuit::new();
    circuit.run(args);
}
