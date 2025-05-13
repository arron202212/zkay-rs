

use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;

/**
 * This gadget provides the remainder of a % b. 
 *
 *
 */

pub struct ModGadget extends Gadget {

	 Wire a;
	 Wire b;
	 Wire r;
	 Wire q;

	 i32 bitwidth; // bitwidth for both a, b

	pub  ModGadget(Wire a,  Wire b, i32 bitwidth, desc:Vec<String>) {
		super(desc);
		self.a = a;
		self.b = b;
		self.bitwidth = bitwidth;
		if bitwidth > 126{
			assert!("Bitwidth not supported yet.");
		}
		buildCircuit();
	}

	  fn buildCircuit() {
		
		r = generator.createProverWitnessWire("mod result");
		q = generator.createProverWitnessWire("division result");

		
		// notes about how to use this code block can be found in FieldDivisionGadget
		generator.specifyProverWitnessComputation(Instruction::new() {
			
			pub   evaluate(CircuitEvaluator evaluator) {
				BigInteger aValue = evaluator.getWireValue(a);
				BigInteger bValue = evaluator.getWireValue(b);
				BigInteger rValue = aValue.mod(bValue);
				evaluator.setWireValue(r, rValue);
				BigInteger qValue = aValue.divide(bValue);
				evaluator.setWireValue(q, qValue);
			}

		});
		
		r.restrictBitLength(bitwidth);
		q.restrictBitLength(bitwidth);
		generator.addOneAssertion(r.isLessThan(b, bitwidth));
		generator.addEqualityAssertion(q.mul(b).add(r), a);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { r };
	}

}
