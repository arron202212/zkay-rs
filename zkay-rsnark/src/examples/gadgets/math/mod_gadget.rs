use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

/**
 * This gadget provides the remainder of a % b.
 *
 *
 */

pub struct ModGadget {
    a: WireType,
    b: WireType,
    r: WireType,
    q: WireType,

    bitwidth: i32, // bitwidth for both a, b
}
impl ModGadget {
    pub fn new(a: WireType, b: WireType, bitwidth: i32, desc: &Option<String>) -> Self {
        super(desc);
        self.a = a;
        self.b = b;
        self.bitwidth = bitwidth;
        assert!(bitwidth <= 126, "Bitwidth not supported yet.");

        buildCircuit();
    }
}
impl Gadget for ModGadget {
    fn buildCircuit() {
        r = generator.createProverWitnessWire("mod result");
        q = generator.createProverWitnessWire("division result");

        // notes about how to use this code block can be found in FieldDivisionGadget
        generator.specifyProverWitnessComputation( &|evaluator: &mut CircuitEvaluator| {
                    let aValue = evaluator.getWireValue(a);
                    let bValue = evaluator.getWireValue(b);
                    let rValue = aValue.rem(bValue);
                    evaluator.setWireValue(r, rValue);
                    let qValue = aValue.div(bValue);
                    evaluator.setWireValue(q, qValue);
                });
        //     {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a);
        //             let bValue = evaluator.getWireValue(b);
        //             let rValue = aValue.rem(bValue);
        //             evaluator.setWireValue(r, rValue);
        //             let qValue = aValue.div(bValue);
        //             evaluator.setWireValue(q, qValue);
        //         }
        //     }
        //     Prover
        // });

        r.restrictBitLength(bitwidth);
        q.restrictBitLength(bitwidth);
        generator.addOneAssertion(r.isLessThan(b, bitwidth));
        generator.addEqualityAssertion(q.mul(b).add(r), a);
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return vec![r];
    }
}
