use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;

use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget;
use crate::circuit::structure::constant_wire;
use crate::circuit::structure::wire_type::WireType;

// see notes in the end of the code.

pub struct FieldDivisionGadget {
    a: WireType,
    b: WireType,
    c: WireType,
}
impl FieldDivisionGadget {
    pub fn new(a: WireType, b: WireType, desc: Vec<String>) -> Self {
        super(desc);
        self.a = a;
        self.b = b;
        // if the input values are constant (i.e. known at compilation time), we
        // can save one constraint
        if a.instance_of(ConstantWire) && b.instance_of(ConstantWire) {
            let aConst = a.getConstant();
            let bInverseConst = b.getConstant().modInverse(Configs.get().unwrap().field_prime);
            c = generator
                .createConstantWire(aConst.mul(bInverseConst).modulo(Configs.get().unwrap().field_prime));
        } else {
            c = generator.createProverWitnessWire(debugStr("division result"));
            buildCircuit();
        }
    }
}
impl Gadget for FieldDivisionGadget {
    fn buildCircuit() {
        // This is an example of computing a value outside the circuit and
        // verifying constraints about it in the circuit. See notes below.

        generator.specifyProverWitnessComputation({
            struct Prover;
            impl Instruction for Prover {
                fn evaluate(&self,evaluator: CircuitEvaluator) {
                    let aValue = evaluator.getWireValue(a);
                    let bValue = evaluator.getWireValue(b);
                    let cValue = aValue
                        .mul(bValue.modInverse(Configs.get().unwrap().field_prime))
                        .modulo(Configs.get().unwrap().field_prime);
                    evaluator.setWireValue(c, cValue);
                }
            }
            Prover
        });

        // to handle the case where a or b can be both zero, see below
        generator.addAssertion(b, c, a, debugStr("Assertion for division result"));

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

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return vec![c];
    }
}
