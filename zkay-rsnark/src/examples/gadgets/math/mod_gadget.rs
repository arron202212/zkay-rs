

use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;

/**
 * This gadget provides the remainder of a % b. 
 *
 *
 */

pub struct ModGadget  {

	 a:Wire,
	 b:Wire,
	 r:Wire,
	 q:Wire,

	 bitwidth:i32, // bitwidth for both a, b
}
impl  ModGadget{
	pub  fn new(a:Wire,  Wire b, bitwidth:i32, desc:Vec<String>)  ->Self{
		super(desc);
		self.a = a;
		self.b = b;
		self.bitwidth = bitwidth;
			assert!(bitwidth <= 126,"Bitwidth not supported yet.");
		
		buildCircuit();
	}
}
impl Gadget for ModGadget{
	  fn buildCircuit() {
		
		r = generator.createProverWitnessWire("mod result");
		q = generator.createProverWitnessWire("division result");

		
		// notes about how to use this code block can be found in FieldDivisionGadget
		generator.specifyProverWitnessComputation(& {
            struct Prover;
            impl Instruction  for Prover
			{
			
			pub  fn evaluate(evaluator:CircuitEvaluator ) {
				let aValue = evaluator.getWireValue(a);
				let bValue = evaluator.getWireValue(b);
				let rValue = aValue.mod(bValue);
				evaluator.setWireValue(r, rValue);
				let qValue = aValue.divide(bValue);
				evaluator.setWireValue(q, qValue);
			}
 }
            Prover
		});
		
		r.restrictBitLength(bitwidth);
		q.restrictBitLength(bitwidth);
		generator.addOneAssertion(r.isLessThan(b, bitwidth));
		generator.addEqualityAssertion(q.mul(b).add(r), a);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![r];
	}

}
