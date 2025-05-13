
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
pub struct LongIntegerModInverseGadget extends Gadget {

	 LongElement a; // the value to be inverted
	 LongElement m; // the modulus
	 bool restrictRange; // whether to enforce that a^(-1) < m
	 LongElement inverse;

	pub  LongIntegerModInverseGadget(LongElement a, LongElement m, bool restrictRange, desc:Vec<String>) {
		super(desc);
		self.a = a;
		self.m = m;
		self.restrictRange = restrictRange;
		buildCircuit();
	}

	  fn buildCircuit() {
		Vec<Wire> inverseWires = generator.createProverWitnessWireArray(m.getSize());
		inverse = LongElement::new(inverseWires, m.getCurrentBitwidth());
		Vec<Wire> quotientWires = generator.createProverWitnessWireArray(m.getSize());
		LongElement quotient = LongElement::new(quotientWires, m.getCurrentBitwidth());

		generator.specifyProverWitnessComputation(Instruction::new() {
			
			pub   evaluate(CircuitEvaluator evaluator) {
				BigInteger aValue = evaluator.getWireValue(a, LongElement.CHUNK_BITWIDTH);
				BigInteger mValue = evaluator.getWireValue(m, LongElement.CHUNK_BITWIDTH);
				BigInteger inverseValue = aValue.modInverse(mValue);
				BigInteger quotientValue = aValue.multiply(inverseValue).divide(mValue);

				evaluator.setWireValue(inverseWires, Util::split(inverseValue, LongElement.CHUNK_BITWIDTH));
				evaluator.setWireValue(quotientWires, Util::split(quotientValue, LongElement.CHUNK_BITWIDTH));
			}
		});

		inverse.restrictBitwidth();
		quotient.restrictBitwidth();

		// a * a^(-1) = 1   (mod m)
		// <=> Exist q:  a * a^(-1) = q * m + 1
		LongElement product = a.mul(inverse);
		LongElement oneModM = quotient.mul(m).add(1);
		product.assertEquality(oneModM);

		if restrictRange {
			inverse.assertLessThan(m);
		}
	}

	pub  LongElement getResult() {
		return inverse;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return inverse.getArray();
	}
}
