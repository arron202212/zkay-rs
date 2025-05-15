use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;

// A sample usage of the AES128 block cipher gadget
pub struct AES128CipherCircuitGenerator {
    inputs: Vec<Wire>,
    key: Vec<Wire>,
    outputs: Vec<Wire>,
    gadget: AES128CipherGadget,
}
impl AES128CipherCircuitGenerator {
    pub fn new(circuitName: String) -> Self {
        super(circuitName);
    }
}
impl CircuitGenerator for AES128CipherCircuitGenerator {
    fn buildCircuit() {
        inputs = createInputWireArray(16); // in bytes
        key = createInputWireArray(16); // in bytes

        let expandedKey = AES128CipherGadget.expandKey(key);
        gadget = AES128CipherGadget::new(inputs, expandedKey, "");
        outputs = gadget.getOutputWires();
        for o in outputs {
            makeOutput(o);
        }
    }

    pub fn generateSampleInput(circuitEvaluator: CircuitEvaluator) {
        let keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
        let msgV = BigInteger::new("ae2d8a571e03ac9c9eb76fac45af8e51", 16);

        // expected output:0xf5d3d58503b9699de785895a96fdbaaf

        let keyArray = keyV.toByteArray();
        let msgArray = msgV.toByteArray();
        msgArray = msgArray[msgArray.len() - 16..].to_vec();
        keyArray = keyArray[keyArray.len() - 16..].to_vec();

        for i in 0..msgArray.len() {
            circuitEvaluator.setWireValue(inputs[i], (msgArray[i] & 0xff));
        }

        for i in 0..keyArray.len() {
            circuitEvaluator.setWireValue(key[i], (keyArray[i] & 0xff));
        }
    }

    pub fn main(args: Vec<String>) {
        Config.hexOutputEnabled = true;
        let generator = AES128CipherCircuitGenerator::new("AES_Circuit");
        generator.generateCircuit();
        generator.evalCircuit();
        generator.prepFiles();
        generator.runLibsnark();
    }
}
