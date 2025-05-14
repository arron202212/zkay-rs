
use circuit::auxiliary::long_element;
use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;
use util::util;



/**
 * This gadget computes the modular multiplicative inverse a^(-1) mod m,
 * where a and m are LongElements.
 * If restrictRange is set to true, the output will be the sole inverse a^(-1)
 * for which a < m holds. If restrictRange is false, the inverse may be any
 * value x for which ax = 1 mod m holds.
 * It is the responsibility of the caller to ensure that a and m are
 * relatively co-prime, i.e. the modular inverse actually exists.
 */
pub struct LongIntegerModInverseGadget  {

	 a:LongElement, // the value to be inverted
	 m:LongElement, // the modulus
	 restrictRange:bool, // whether to enforce that a^(-1) < m
	 inverse:LongElement,
}
impl  LongIntegerModInverseGadget{
	pub  fn new(a:LongElement, m:LongElement, bool restrictRange, desc:Vec<String>)  ->Self{
		super(desc);
		self.a = a;
		self.m = m;
		self.restrictRange = restrictRange;
		buildCircuit();
	}
}
impl Gadget for LongIntegerModInverseGadget{
	  fn buildCircuit() {
		let inverseWires = generator.createProverWitnessWireArray(m.getSize());
		inverse = LongElement::new(inverseWires, m.getCurrentBitwidth());
		let quotientWires = generator.createProverWitnessWireArray(m.getSize());
		let quotient = LongElement::new(quotientWires, m.getCurrentBitwidth());

		generator.specifyProverWitnessComputation(Instruction::new() {
			
			pub   evaluate(evaluator:CircuitEvaluator) {
				let aValue = evaluator.getWireValue(a, LongElement.CHUNK_BITWIDTH);
				let mValue = evaluator.getWireValue(m, LongElement.CHUNK_BITWIDTH);
				let inverseValue = aValue.modInverse(mValue);
				let quotientValue = aValue.multiply(inverseValue).divide(mValue);

				evaluator.setWireValue(inverseWires, Util::split(inverseValue, LongElement.CHUNK_BITWIDTH));
				evaluator.setWireValue(quotientWires, Util::split(quotientValue, LongElement.CHUNK_BITWIDTH));
			}
		});

		inverse.restrictBitwidth();
		quotient.restrictBitwidth();

		// a * a^(-1) = 1   (mod m)
		// <=> Exist q:  a * a^(-1) = q * m + 1
		let product = a.mul(inverse);
		let oneModM = quotient.mul(m).add(1);
		product.assertEquality(oneModM);

		if restrictRange {
			inverse.assertLessThan(m);
		}
	}

	pub fn getResult()-> LongElement {
		return inverse;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return inverse.getArray();
	}
}
