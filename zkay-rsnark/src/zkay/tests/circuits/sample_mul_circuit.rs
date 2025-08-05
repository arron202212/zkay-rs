use zkay::homomorphic_input;
use zkay::zkay_circuit_base;
use zkay::zkay_type::zk_bool;
use zkay::zkay_type::zk_int;
use zkay::zkay_type::zk_uint;

pub struct SampleMulCircuit;
impl SampleMulCircuit {
    pub fn new() -> CircuitGeneratorExtend<ZkayCircuitBase<Self>> {
        let mut _self = ZkayCircuitBase::<Self>::new("zk__Verify_Test_foo", 6, 4, 0, true);
        _self.addCryptoBackend("elgamal", "elgamal", 508);
        _self
    }
}
impl CircuitGeneratorExtend<ZkayCircuitBase<SampleMulCircuit>> {
    fn __zk__foo() {
        stepIn("_zk__foo");
        addIn("zk__in0_cipher_val", 4, ZkUint(256));
        addOut("zk__out0_cipher", 4, ZkUint(256));

        //[ --- val * 3 ---
        // zk__in0_cipher_val = val
        decl(
            "tmp0_cipher",
            o_hom(
                "elgamal",
                "glob_key_Elgamal__owner",
                HomomorphicInput.of(getCipher("zk__in0_cipher_val")),
                '*',
                HomomorphicInput.of(cast(val(3, ZkUint(8)), ZkUint(32))),
            ),
        );
        checkEq("tmp0_cipher", "zk__out0_cipher");
        //] --- val * 3 ---

        stepOut();
    }

    fn buildCircuit(&mut self) {
        // super.buildCircuit();
        addK("elgamal", "glob_key_Elgamal__owner", 2);

        __zk__foo();
    }
}
pub fn main(args: Vec<String>) {
    let circuit = SampleMulCircuit::new();
    circuit.run(args);
}
