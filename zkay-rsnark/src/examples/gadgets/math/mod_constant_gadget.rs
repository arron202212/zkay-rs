use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

/**
 * This gadget provides the remainder of a % b, where b is a circuit constant.
 *
 *
 */

pub struct ModConstantGadget {
    a: WireType,
    b: BigInteger,
    r: WireType,
    q: WireType,

    bitwidth: i32, // a's bitwidth
}
impl ModConstantGadget {
    pub fn new(a: WireType, bitwidth: i32, b: BigInteger, desc: Vec<String>) -> Self {
        super(desc);
        self.a = a;
        self.b = b;
        self.bitwidth = bitwidth;
        assert!(
            b.signum() == 1,
            "b must be a positive constant. Signed operations not supported yet."
        );

        assert!(
            bitwidth >= b.bitLength(),
            "a's bitwidth < b's bitwidth -- This gadget is not needed."
        );

        // TODO: add further checks.

        buildCircuit();
    }
}
impl Gadget for ModConstantGadget {
    fn buildCircuit() {
        r = generator.createProverWitnessWire("mod result");
        q = generator.createProverWitnessWire("division result");

        // notes about how to use this code block can be found in FieldDivisionGadget
        generator.specifyProverWitnessComputation(&{
            struct Prover;
            impl Instruction for Prover {
                fn evaluate(&self,evaluator: CircuitEvaluator) {
                    let aValue = evaluator.getWireValue(a);
                    let rValue = aValue.modulo(b);
                    evaluator.setWireValue(r, rValue);
                    let qValue = aValue.divide(b);
                    evaluator.setWireValue(q, qValue);
                }
            }
            Prover
        });

        let bBitwidth = b.bitLength();
        r.restrictBitLength(bBitwidth);
        q.restrictBitLength(bitwidth - bBitwidth + 1);
        generator.addOneAssertion(r.isLessThan(b, bBitwidth));
        generator.addEqualityAssertion(q.mul(b).add(r), a);
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return vec![r];
    }
}
