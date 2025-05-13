
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;

pub struct SimpleCircuitGenerator extends CircuitGenerator {

	 Vec<Wire> inputs;

	pub  SimpleCircuitGenerator(String circuitName) {
		super(circuitName);
	}

	
	  fn buildCircuit() {

		// declare input array of length 4.
		inputs = createInputWireArray(4);

		// r1 = in0 * in1
		Wire r1 = inputs[0].mul(inputs[1]);

		// r2 = in2 + in3
		Wire r2 = inputs[2].add(inputs[3]);

		// result = (r1+5)*(6*r2)
		Wire result = r1.add(5).mul(r2.mul(6));

		// mark the wire as output
		makeOutput(result);

	}

	
	pub   generateSampleInput(CircuitEvaluator circuitEvaluator) {
		for i in 0..4 {
			circuitEvaluator.setWireValue(inputs[i], i + 1);
		}
	}

	pub    main(args:Vec<String>)  {

		SimpleCircuitGenerator generator = SimpleCircuitGenerator::new("simple_example");
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();
	}

}
