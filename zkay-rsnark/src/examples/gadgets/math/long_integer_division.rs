use crate::circuit::auxiliary::long_element;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{Util,BigInteger};

/**
 * This gadget computes q and r such that a = q * b + r, when both operands are represented
 * as long elements. You can check the RSA gadgets/circuit generators for an example.
 * Most of the optimizations that reduce the cost of this step are more visible
 * in the LongElement class methods called by this gadget.
 */
pub struct LongIntegerDivision {
    a: LongElement,
    b: LongElement,
    r: LongElement,
    q: LongElement,
    restrictRange: bool,
    bMinBitwidth: i32,
}
impl LongIntegerDivision {
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
    pub fn new(
        a: LongElement,
        b: LongElement,
        bMinBitwidth: i32,
        restrictRange: bool,
        desc: &Option<String>,
    ) -> Self {
        super(desc);
        buildCircuit();
        Self {
            a,
            b,
            bMinBitwidth,
            restrictRange,
        }
    }
}
impl Gadget for LongIntegerDivision {
    fn buildCircuit() {
        let aBitwidth = std::cmp::max(1, a.getMaxVal(LongElement.CHUNK_BITWIDTH).bits());
        let bBitwidth = std::cmp::max(1, b.getMaxVal(LongElement.CHUNK_BITWIDTH).bits());

        let rBitwidth = std::cmp::min(aBitwidth, bBitwidth);
        let qBitwidth = aBitwidth;

        if bMinBitwidth > 0 {
            qBitwidth = std::cmp::max(1, qBitwidth - bMinBitwidth + 1);
        }

        // length in what follows means the number of chunks
        let rLength = (rBitwidth * 1.0 / LongElement.CHUNK_BITWIDTH).ceil() as i32;
        let qLength = (qBitwidth * 1.0 / LongElement.CHUNK_BITWIDTH).ceil() as i32;

        let rWires = generator.createProverWitnessWireArray(rLength);
        let qWires = generator.createProverWitnessWireArray(qLength);

        let rChunkBitwidths = vec![LongElement.CHUNK_BITWIDTH; rLength];
        let qChunkBitwidths = vec![LongElement.CHUNK_BITWIDTH; qLength];

        if rBitwidth % LongElement.CHUNK_BITWIDTH != 0 {
            rChunkBitwidths[rLength - 1] = rBitwidth % LongElement.CHUNK_BITWIDTH;
        }
        if qBitwidth % LongElement.CHUNK_BITWIDTH != 0 {
            qChunkBitwidths[qLength - 1] = qBitwidth % LongElement.CHUNK_BITWIDTH;
        }

        r = LongElement::new(rWires, rChunkBitwidths);
        q = LongElement::new(qWires, qChunkBitwidths);

        generator.specifyProverWitnessComputation({
            struct Prover;
            impl Instruction for Prover {
                fn evaluate(&self,evaluator: CircuitEvaluator) {
                    let aValue = evaluator.getWireValue(a, LongElement.CHUNK_BITWIDTH);
                    let bValue = evaluator.getWireValue(b, LongElement.CHUNK_BITWIDTH);
                    let rValue = aValue.rem(bValue);
                    let qValue = aValue.div(bValue);

                    evaluator.setWireValue(
                        r.getArray(),
                        Util::split(rValue, LongElement.CHUNK_BITWIDTH),
                    );
                    evaluator.setWireValue(
                        q.getArray(),
                        Util::split(qValue, LongElement.CHUNK_BITWIDTH),
                    );
                }
            }
            Prover
        });

        r.restrictBitwidth();
        q.restrictBitwidth();

        let res = q.mul(b).add(r);

        // implements the improved long integer equality assertion from xjsnark
        res.assertEquality(a);

        if restrictRange {
            r.assertLessThan(b);
        }
    }

    pub fn getQuotient() -> LongElement {
        return q;
    }

    pub fn getRemainder() -> LongElement {
        return r;
    }
}
