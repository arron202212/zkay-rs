

use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::math::mod_constant_gadget;
use examples::gadgets::math::mod_gadget;


pub struct Mod_Test extends TestCase {

	// TODO; add more tests
	@Test
	pub   testCase1() {

		i32 a = 1262178522;
		i32 b = 257; // b will be an input to the circuit

		CircuitGenerator generator = CircuitGenerator::new("Mod_Test1") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {

				inputWires = createInputWireArray(2);
//				Wire r = ModGadget::new(inputWires[0], (i32) Math.ceil(Math.log10(a) / Math.log10(2)), inputWires[1],
//						(i32) Math.ceil(Math.log10(b) / Math.log10(2))).getOutputWires()[0];
				
				Wire r = ModGadget::new(inputWires[0],  inputWires[1], 32).getOutputWires()[0];
				makeOutput(r);
			}

			
			pub   generateSampleInput(CircuitEvaluator e) {
				e.setWireValue(inputWires[0], a);
				e.setWireValue(inputWires[1], b);

			}
		};

		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
		Wire rWire = generator.getOutWires().get(0);
		assertEquals(evaluator.getWireValue(rWire), BigInteger.valueOf(a % b));
	}
	
	@Test
	pub   testCase2() {

		i32 a = 1262178522;
		i32 b = 257; // b will be a constant

		CircuitGenerator generator = CircuitGenerator::new("Mod_Test2") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {

				inputWires = createInputWireArray(1);
				Wire r = ModConstantGadget::new(inputWires[0], 32, BigInteger.valueOf(b)).getOutputWires()[0];
				makeOutput(r);
			}

			
			pub   generateSampleInput(CircuitEvaluator e) {
				e.setWireValue(inputWires[0], a);
			}
		};

		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
		Wire rWire = generator.getOutWires().get(0);
		assertEquals(evaluator.getWireValue(rWire), BigInteger.valueOf(a % b));
	}
	

}
