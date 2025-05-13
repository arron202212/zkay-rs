
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::math::dot_product_gadget;

pub struct DotProductCircuitGenerator extends CircuitGenerator {

	 Vec<Wire> a;
	 Vec<Wire> b;
	 i32 dimension;

	pub  DotProductCircuitGenerator(String circuitName, i32 dimension) {
		super(circuitName);
		self.dimension = dimension;
	}

	
	  fn buildCircuit() {

		a = createInputWireArray(dimension, "Input a");
		b = createInputWireArray(dimension, "Input b");

		DotProductGadget dotProductGadget = DotProductGadget::new(a, b);
		Vec<Wire> result = dotProductGadget.getOutputWires();
		makeOutput(result[0], "output of dot product a, b");
	}

	
	pub   generateSampleInput(CircuitEvaluator circuitEvaluator) {

		for i in 0..dimension {
			circuitEvaluator.setWireValue(a[i], 10 + i);
			circuitEvaluator.setWireValue(b[i], 20 + i);
		}
	}

	pub    main(args:Vec<String>)  {

		DotProductCircuitGenerator generator = DotProductCircuitGenerator::new("dot_product", 3);
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();	
	}

}
