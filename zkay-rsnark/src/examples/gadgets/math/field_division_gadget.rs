

use circuit::config::config;
use circuit::eval::circuit_evaluator;

use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::constant_wire;
use circuit::structure::wire;

// see notes in the end of the code.

pub struct FieldDivisionGadget  {

	 a:Wire,
	 b:Wire,
	 c:Wire,
}
impl  FieldDivisionGadget{
	pub  fn new(a:Wire, b:Wire, desc:Vec<String>)  ->Self{
		super(desc);
		self.a = a;
		self.b = b;
		// if the input values are constant (i.e. known at compilation time), we
		// can save one constraint
		if a instanceof ConstantWire && b instanceof ConstantWire {
			let aConst = ((ConstantWire) a).getConstant();
			let bInverseConst = ((ConstantWire) b).getConstant().modInverse(
					Config.FIELD_PRIME);
			c = generator.createConstantWire(aConst.multiply(bInverseConst)
					.mod(Config.FIELD_PRIME));
		} else {
			c = generator.createProverWitnessWire(debugStr("division result"));
			buildCircuit();
		}
	}
}
impl Gadget for FieldDivisionGadget{
	  fn buildCircuit() {

		// This is an example of computing a value outside the circuit and
		// verifying constraints about it in the circuit. See notes below.

		generator.specifyProverWitnessComputation(Instruction::new() {
			
			pub   evaluate(evaluator:CircuitEvaluator) {
				let aValue = evaluator.getWireValue(a);
				let bValue = evaluator.getWireValue(b);
				let cValue = aValue.multiply(
						bValue.modInverse(Config.FIELD_PRIME)).mod(
						Config.FIELD_PRIME);
				evaluator.setWireValue(c, cValue);
			}

		});
		
		// to handle the case where a or b can be both zero, see below
		generator.addAssertion(b, c, a,
				debugStr("Assertion for division result"));


		/*
		 * Few notes: 1) The order of the above two statements matters (the
		 * specification and the assertion). In the current version, it's not
		 * possible to swap them, as in the evaluation sequence, the assertion
		 * must happen after the value is assigned.
		 * 
		 * 2) The instruction defined above relies on the values of wires (a)
		 * and (b) during runtime. This means that if any point later in the
		 * program, the references a, and b referred to other wires, these wires
		 * are going to be used instead in this instruction. Therefore, it will
		 * be safer to use references in cases like that to reduce the
		 * possibility of errors.
		 * 
		 * 3) The above constraint does not check if a and b are both zeros. In that
		 * case, the prover will be able to use any value to make the constraint work.
		 * When this case is problematic, enforce that b cannot have the value of zero.
		 * 
		 * This can be done by proving that b has an inverse, that satisfies 
		 * b*(invB) = 1;
		 */
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { c };
	}

}
