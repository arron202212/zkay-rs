use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

/**
 * This gadget does not apply any lookups in the circuit. Instead, it verifies
 * the solution using the AES S-Box properties.
 * (Might need to be revisited in
 * the future to include other ways that have better circuit representation).
 *
 */

pub struct AESSBoxComputeGadget {
    input: WireType,
    inverse: WireType,
    output: WireType,
}
impl AESSBoxComputeGadget {
    pub fn new(input: WireType, desc: &Option<String>) -> Self {
        super(desc);
        self.input = input;
        buildCircuit();
    }
}
impl Gadget for AESSBoxComputeGadget {
    fn buildCircuit() {
        inverse = generator.createProverWitnessWire();

        generator.specifyProverWitnessComputation( &|evaluator: &mut CircuitEvaluator| {
                    let p = evaluator.getWireValue(input).intValue();
                    let q = findInv(p);
                    evaluator.setWireValue(inverse, &q);
                });
        // &{
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let p = evaluator.getWireValue(input).intValue();
        //             let q = findInv(p);
        //             evaluator.setWireValue(inverse, q);
        //         }
        //     }
        //     Prover
        // });

        inverse.restrictBitLength(8);
        let v = gmul(input, inverse);
        generator.addAssertion(
            v.sub(generator.get_one_wire()),
            input.add(inverse),
            generator.get_zero_wire(),
        );
        let constant = generator.createConstantWire(0x63L);
        output = constant.xorBitwise(inverse, 8);
        output = output.xorBitwise(inverse.rotateLeft(8, 1), 8);
        output = output.xorBitwise(inverse.rotateLeft(8, 2), 8);
        output = output.xorBitwise(inverse.rotateLeft(8, 3), 8);
        output = output.xorBitwise(inverse.rotateLeft(8, 4), 8);
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        vec![output]
    }

    fn gmul(a: WireType, b: WireType) -> WireType {
        let p = generator.get_zero_wire();
        for counter in 0..8 {
            let tmp = p.xorBitwise(a, 8);
            let bit = b.getBitWires(8).get(0);
            p = p.add(bit.mul(tmp.sub(p)));

            let bit2 = a.getBitWires(8).get(7);
            a = a.shl(8, 1);

            let tmp2 = a.xorBitwise(generator.createConstantWire(0x1bL), 8);
            a = a.add(bit2.mul(tmp2.sub(a)));
            b = b.shiftRight(8, 1);
        }
        p
    }

    fn gmul(a: i32, b: i32) -> i32 {
        let p = 0;
        for j in 0..8 {
            if (b & 1) != 0 {
                p ^= a;
            }
            a <<= 1;
            if (a & 0x100) != 0 {
                a ^= 0x11b;
            }
            b >>= 1;
        }
        p
    }

    fn findInv(a: i32) -> i32 {
        if a == 0 {
            return 0;
        }
        for i in 0..256 {
            if gmul(i, a) == 1 {
                return i;
            }
        }
        -1
    }
}
