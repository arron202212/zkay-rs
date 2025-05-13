
use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;

/**
 * This gadget does not apply any lookups in the circuit. Instead, it verifies
 * the solution using the AES S-Box properties.
 * (Might need to be revisited in
 * the future to include other ways that have better circuit representation).
 *
 */

pub struct AESSBoxComputeGadget extends Gadget {

	 Wire input;
	 Wire inverse;
	 Wire output;

	pub  AESSBoxComputeGadget(Wire input, desc:Vec<String>) {
		super(desc);
		self.input = input;
		buildCircuit();
	}

	  fn buildCircuit() {
		inverse = generator.createProverWitnessWire();

		generator.addToEvaluationQueue(Instruction::new() {

			
			pub   evaluate(CircuitEvaluator evaluator) {
				i32 p = evaluator.getWireValue(input).intValue(); 
				i32 q = findInv(p);
				evaluator.setWireValue(inverse, q);

			}
		});

		inverse.restrictBitLength(8);
		Wire v = gmul(input, inverse);
		generator.addAssertion(v.sub(generator.getOneWire()),
				input.add(inverse), generator.getZeroWire());
		Wire constant = generator.createConstantWire(0x63L);
		output = constant.xorBitwise(inverse, 8);
		output = output.xorBitwise(inverse.rotateLeft(8, 1), 8);
		output = output.xorBitwise(inverse.rotateLeft(8, 2), 8);
		output = output.xorBitwise(inverse.rotateLeft(8, 3), 8);
		output = output.xorBitwise(inverse.rotateLeft(8, 4), 8);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { output };
	}

	 Wire gmul(Wire a, Wire b) {
		Wire p = generator.getZeroWire();
		i32 counter;
		for counter in 0..8
			Wire tmp = p.xorBitwise(a, 8);
			Wire bit = b.getBitWires(8).get(0);
			p = p.add(bit.mul(tmp.sub(p)));

			Wire bit2 = a.getBitWires(8).get(7);
			a = a.shiftLeft(8, 1);

			Wire tmp2 = a.xorBitwise(generator.createConstantWire(0x1bL), 8);
			a = a.add(bit2.mul(tmp2.sub(a)));
			b = b.shiftRight(8, 1);
		}
		return p;
	}

	 i32 gmul(i32 a, i32 b) {
		i32 p = 0;
		i32 j;
		for j in 0..8
			if (b & 1) != 0
				p ^= a;
			a <<= 1;
			if (a & 0x100) != 0
				a ^= 0x11b;
			b >>= 1;
		}
		return p;
	}

	 i32 findInv(i32 a) {
		if a == 0
			return 0;
		for i in 0..256 {
			if gmul(i, a) == 1 {
				return i;
			}
		}
		return -1;
	}
}
