use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{CircuitGenerator,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::hash::sha256_gadget;

pub struct SHA2CircuitGenerator {
    inputWires: Vec<Option<WireType>>,
    sha2Gadget: SHA256Gadget,
}
impl SHA2CircuitGenerator {
    pub fn new(circuitName: String) -> Self {
        super(circuitName);
    }
}
impl CircuitGenerator for SHA2CircuitGenerator {
    fn buildCircuit() {
        // assuming the circuit input will be 64 bytes
        inputWires = createInputWireArray(64);
        // this gadget is not applying any padding.
        sha2Gadget = SHA256Gadget::new(inputWires, 8, 64, false, false);
        let digest = sha2Gadget.getOutputWires();
        makeOutputArray(digest, "digest");

        // ======================================================================
        // To see how padding can be done, and see how the gadget library will save constraints automatically,
        // try the snippet below instead.
        /*
            inputWires = createInputWireArray(3); 	// 3-byte input
            sha2Gadget = SHA256Gadget::new(inputWires, 8, 3, false, true);
            Vec<Option<WireType>> digest = sha2Gadget.getOutputWires();
            makeOutputArray(digest, "digest");
        */
    }

     fn generateSampleInput(evaluator: CircuitEvaluator) {
        let inputStr = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl";
        for i in 0..inputWires.len() {
            evaluator.setWireValue(inputWires[i], inputStr.charAt(i));
        }
    }

  
}
  pub fn main(args: Vec<String>) {
        let generator = SHA2CircuitGenerator::new("sha_256");
        generator.generateCircuit();
        generator.evalCircuit();
        generator.prepFiles();
        generator.runLibsnark();
    }