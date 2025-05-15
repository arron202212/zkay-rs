

use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::math::mod_constant_gadget;
use examples::gadgets::math::mod_gadget;


pub struct Mod_Test  {

	// TODO; add more tests
	
	pub   testCase1() {

let a = 1262178522;
let b = 257; // b will be an input to the circuit

		CircuitGenerator generator = CircuitGenerator::new("Mod_Test1") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {

				inputWires = createInputWireArray(2);
//				Wire r = ModGadget::new(inputWires[0], (i32) Math.ceil(Math.log10(a) / Math.log10(2)), inputWires[1],
//						(i32) Math.ceil(Math.log10(b) / Math.log10(2))).getOutputWires()[0];
				
let r = ModGadget::new(inputWires[0],  inputWires[1], 32).getOutputWires()[0];
				makeOutput(r);
			}

			
			pub  fn generateSampleInput(CircuitEvaluator e) {
				e.setWireValue(inputWires[0], a);
				e.setWireValue(inputWires[1], b);

			}
		};

		generator.generateCircuit();
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
let rWire = generator.getOutWires().get(0);
		assertEquals(evaluator.getWireValue(rWire), BigInteger.valueOf(a % b));
	}
	
	
	pub   testCase2() {

let a = 1262178522;
let b = 257; // b will be a constant

		CircuitGenerator generator = CircuitGenerator::new("Mod_Test2") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {

				inputWires = createInputWireArray(1);
let r = ModConstantGadget::new(inputWires[0], 32, BigInteger.valueOf(b)).getOutputWires()[0];
				makeOutput(r);
			}

			
			pub  fn generateSampleInput(CircuitEvaluator e) {
				e.setWireValue(inputWires[0], a);
			}
		};

		generator.generateCircuit();
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
let rWire = generator.getOutWires().get(0);
		assertEquals(evaluator.getWireValue(rWire), BigInteger.valueOf(a % b));
	}
	

}
