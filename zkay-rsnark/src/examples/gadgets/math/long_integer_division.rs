
use circuit::auxiliary::long_element;
use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::operations::gadget;
use circuit::structure::wire;
use util::util;


/**
 * This gadget computes q and r such that a = q * b + r, when both operands are represented
 * as long elements. You can check the RSA gadgets/circuit generators for an example.
 * Most of the optimizations that reduce the cost of this step are more visible
 * in the LongElement class methods called by this gadget.
 */
abstract class LongIntegerDivision extends Gadget {

	 LongElement a;
	 LongElement b;

	 LongElement r;
	 LongElement q;
	 bool restrictRange;
	 i32 bMinBitwidth;

	/**
	 * @param a
	 * @param b
	 * @param restrictRange
	 * 		if true, the output will be forced to be less than b,
	 * 		otherwise the output remainder will only be guaranteed
	 * 		to have the same bitwidth as b, but not necessarily less
	 * 		than b. The second case is helpful when the purpose is
	 * 		just to reduce the range, while having consistent
	 * 		output. As an example (in a short integer case for
	 * 		simplicity): assume we are interested in this operation
	 * 		3001 % 10. The output should be 1 in normal cases, but
	 * 		to save some operations, we might skip checking that the
	 * 		result is less than the modulus and just check that it
	 * 		has the same bitwidth as the modulus, which we must do
	 * 		anyway since the result is provided as a witness. In
	 * 		that case, the output of this gadget could be 1 or 11,
	 * 		which in some contexts would be ok, e.g. in intermediate
	 * 		operations. See the RSA encryption gadget for an
	 * 		illustration.
	 * @param desc
	 */

	pub  LongIntegerDivision(LongElement a, LongElement b, bool restrictRange, desc:Vec<String>) {
		this(a, b, 0, restrictRange, desc);
	}

	/**
	 * @param a
	 * @param b
	 * @param bMinBitwidth
	 * 		The minimum bitwidth of the second operand
	 * @param restrictRange
	 * 		if true, the output will be forced to be less than b,
	 * 		otherwise the output remainder will only be guaranteed to have
	 * 		the same bitwidth as b, but not necessarily less than b. The
	 * 		second case is helpful when the purpose is just to reduce the
	 * 		range, while having consistent output. As an example (in a
	 * 		short integer case for simplicity): assume we are interested
	 * 		in this operation 3001 % 10. The output should be 1 in normal
	 * 		cases, but to save some operations, we might skip checking
	 * 		that the result is less than the modulus and just check that
	 * 		it has the same bitwidth as the modulus, which we must do
	 * 		anyway since the result is provided as a witness. In that
	 * 		case, the output of this gadget could be 1 or 11, which in
	 * 		some contexts would be ok, e.g. in intermediate operations.
	 * 		See the RSA encryption gadget for an illustration.
	 * @param desc
	 */
	pub  LongIntegerDivision(LongElement a, LongElement b, i32 bMinBitwidth, bool restrictRange,
	                           desc:Vec<String>) {
		super(desc);
		self.a = a;
		self.b = b;
		self.bMinBitwidth = bMinBitwidth;
		self.restrictRange = restrictRange;
		buildCircuit();
	}

	  fn buildCircuit() {

		i32 aBitwidth = Math.max(1, a.getMaxVal(LongElement.CHUNK_BITWIDTH).bitLength());
		i32 bBitwidth = Math.max(1, b.getMaxVal(LongElement.CHUNK_BITWIDTH).bitLength());

		i32 rBitwidth = std::cmp::min(aBitwidth, bBitwidth);
		i32 qBitwidth = aBitwidth;

		if bMinBitwidth > 0 {
			qBitwidth = Math.max(1, qBitwidth - bMinBitwidth + 1);
		}

		// length in what follows means the number of chunks
		i32 rLength = (i32) Math.ceil(rBitwidth * 1.0 / LongElement.CHUNK_BITWIDTH);
		i32 qLength = (i32) Math.ceil(qBitwidth * 1.0 / LongElement.CHUNK_BITWIDTH);

		Vec<Wire> rWires = generator.createProverWitnessWireArray(rLength);
		Vec<Wire> qWires = generator.createProverWitnessWireArray(qLength);

		Vec<i32> rChunkBitwidths = vec![i32::default();rLength];
		Vec<i32> qChunkBitwidths = vec![i32::default();qLength];

		Arrays.fill(rChunkBitwidths, LongElement.CHUNK_BITWIDTH);
		Arrays.fill(qChunkBitwidths, LongElement.CHUNK_BITWIDTH);

		if rBitwidth % LongElement.CHUNK_BITWIDTH != 0 {
			rChunkBitwidths[rLength - 1] = rBitwidth % LongElement.CHUNK_BITWIDTH;
		}
		if qBitwidth % LongElement.CHUNK_BITWIDTH != 0 {
			qChunkBitwidths[qLength - 1] = qBitwidth % LongElement.CHUNK_BITWIDTH;
		}

		r = LongElement::new(rWires, rChunkBitwidths);
		q = LongElement::new(qWires, qChunkBitwidths);

		generator.specifyProverWitnessComputation(Instruction::new() {
			
			pub   evaluate(CircuitEvaluator evaluator) {
				BigInteger aValue = evaluator.getWireValue(a, LongElement.CHUNK_BITWIDTH);
				BigInteger bValue = evaluator.getWireValue(b, LongElement.CHUNK_BITWIDTH);
				BigInteger rValue = aValue.mod(bValue);
				BigInteger qValue = aValue.divide(bValue);

				evaluator.setWireValue(r.getArray(), Util::split(rValue, LongElement.CHUNK_BITWIDTH));
				evaluator.setWireValue(q.getArray(), Util::split(qValue, LongElement.CHUNK_BITWIDTH));
			}
		});

		r.restrictBitwidth();
		q.restrictBitwidth();

		LongElement res = q.mul(b).add(r);

		// implements the improved long integer equality assertion from xjsnark
		res.assertEquality(a);

		if restrictRange {
			r.assertLessThan(b);
		}
	}

	pub  LongElement getQuotient() {
		return q;
	}

	pub  LongElement getRemainder() {
		return r;
	}
}
