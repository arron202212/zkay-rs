

use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;

/**
 * This gadget provides the remainder of a % b, where b is a circuit constant.
 *
 *
 */

pub struct ModConstantGadget  {

	 a:Wire,
	 b:BigInteger,
	 r:Wire,
	 q:Wire,

	 bitwidth:i32, // a's bitwidth
}
impl  ModConstantGadget{
	pub  fn new(a:Wire, i32 bitwidth, b:BigInteger, desc:Vec<String>)  ->Self{
		super(desc);
		self.a = a;
		self.b = b;
		self.bitwidth = bitwidth;
		if b.signum() != 1{
			assert!("b must be a positive constant. Signed operations not supported yet.");
		}
		if bitwidth < b.bitLength(){
			assert!("a's bitwidth < b's bitwidth -- This gadget is not needed.");
		}
		// TODO: add further checks.
		
		buildCircuit();
	}
}
impl Gadget for ModConstantGadget{
	  fn buildCircuit() {
		
		r = generator.createProverWitnessWire("mod result");
		q = generator.createProverWitnessWire("division result");

		// notes about how to use this code block can be found in FieldDivisionGadget
		generator.specifyProverWitnessComputation(Instruction::new() {
			
			pub   evaluate(evaluator:CircuitEvaluator) {
				let aValue = evaluator.getWireValue(a);
				let rValue = aValue.mod(b);
				evaluator.setWireValue(r, rValue);
				let qValue = aValue.divide(b);
				evaluator.setWireValue(q, qValue);
			}

		});
		
		let bBitwidth = b.bitLength();
		r.restrictBitLength(bBitwidth);
		q.restrictBitLength(bitwidth - bBitwidth + 1);
		generator.addOneAssertion(r.isLessThan(b, bBitwidth));
		generator.addEqualityAssertion(q.mul(b).add(r), a);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { r };
	}

}
