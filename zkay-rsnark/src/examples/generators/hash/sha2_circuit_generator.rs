
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::hash::sha256_gadget;

pub struct SHA2CircuitGenerator extends CircuitGenerator {

	 Vec<Wire> inputWires;
	 SHA256Gadget sha2Gadget;

	pub  SHA2CircuitGenerator(String circuitName) {
		super(circuitName);
	}

	
	  fn buildCircuit() {
		
		// assuming the circuit input will be 64 bytes
		inputWires = createInputWireArray(64);
		// this gadget is not applying any padding.
		sha2Gadget = SHA256Gadget::new(inputWires, 8, 64, false, false);
		Vec<Wire> digest = sha2Gadget.getOutputWires();
		makeOutputArray(digest, "digest");
		
		// ======================================================================
		// To see how padding can be done, and see how the gadget library will save constraints automatically, 
		// try the snippet below instead.
		/*
			inputWires = createInputWireArray(3); 	// 3-byte input
			sha2Gadget = SHA256Gadget::new(inputWires, 8, 3, false, true);
			Vec<Wire> digest = sha2Gadget.getOutputWires();
			makeOutputArray(digest, "digest");
		*/
		
	}

	
	pub   generateSampleInput(CircuitEvaluator evaluator) {
		String inputStr = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl";
		for i in 0..inputWires.length {
			evaluator.setWireValue(inputWires[i], inputStr.charAt(i));
		}
	}

	pub    main(args:Vec<String>)  {
		SHA2CircuitGenerator generator = SHA2CircuitGenerator::new("sha_256");
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();
	}

}
