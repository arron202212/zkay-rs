#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::InstanceOf;
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget::GadgetConfig;

use crate::circuit::structure::circuit_generator::CreateConstantWire;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::constant_wire;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_type::WireType;
use rccell::{RcCell, WeakCell};
use zkay_derive::ImplStructNameConfig;
// see notes in the end of the code.
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct FieldDivisionGadget {
    a: WireType,
    b: WireType,
    c: Option<WireType>,
    generator: WeakCell<CircuitGenerator>,
}
impl FieldDivisionGadget {
    pub fn new(
        a: WireType,
        b: WireType,
        desc: &Option<String>,
        mut generator: WeakCell<CircuitGenerator>,
    ) -> Self {
        // super(desc);
        let mut _self = Self {
            a,
            b,
            c: None,
            generator: generator.clone(),
        };
        let mut generator = generator.upgrade().unwrap();
        // let mut generator = self.me.clone().unwrap().upgrade().unwrap();

        // if the input values are constant (i.e. known at compilation time), we
        // can save one constraint
        if _self.a.instance_of("ConstantWire") && _self.b.instance_of("ConstantWire") {
            let aConst = _self.a.try_as_constant_ref().unwrap().getConstant();
            let bInverseConst = _self
                .b
                .try_as_constant_ref()
                .unwrap()
                .getConstant()
                .modinv(&Configs.field_prime)
                .unwrap();
            _self.c =
                Some(generator.create_constant_wire(
                    &aConst.mul(bInverseConst).rem(&Configs.field_prime),
                    &None,
                ));
        } else {
            _self.c = Some(generator.createProverWitnessWire(&_self.debugStr("division result")));
            _self.buildCircuit();
        }
        _self
    }

    fn buildCircuit(&mut self) {
        // This is an example of computing a value outside the circuit and
        // verifying constraints about it in the circuit. See notes below.
        let mut generator = self.generator.clone().upgrade().unwrap();

        let (a, b, c) = (&self.a, &self.b, self.c.as_ref().unwrap());
        let prover = crate::impl_prover!(
                        eval(a: WireType,
                        b: WireType,
                        c: WireType)  {
        impl Instruction for Prover{
         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                               let aValue = evaluator.getWireValue(&self.a);
                            let bValue = evaluator.getWireValue(&self.b);
                            let cValue = aValue
                                .mul(bValue.modinv(&Configs.field_prime).unwrap())
                                .rem(&Configs.field_prime);
                            evaluator.setWireValue(&self.c, &cValue);
        }
        }
                    }
                );
        generator.specifyProverWitnessComputation(prover);
        // generator.specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
        //     let aValue = evaluator.getWireValue(self.a.clone());
        //     let bValue = evaluator.getWireValue(self.b.clone());
        //     let cValue = aValue
        //         .mul(bValue.modinv(&Configs.field_prime.clone()).unwrap())
        //         .rem(Configs.field_prime.clone());
        //     evaluator.setWireValue(self.c.clone(), cValue);
        // });
        // {
        //     #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
        //     struct Prover {
        //         a: WireType,
        //         b: WireType,
        //         c: WireType,
        //     }
        //     impl  Instruction for Prover {
        //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        //             let aValue = evaluator.getWireValue(self.a.clone());
        //             let bValue = evaluator.getWireValue(self.b.clone());
        //             let cValue = aValue
        //                 .mul(bValue.modinv(&Configs.field_prime.clone()).unwrap())
        //                 .rem(Configs.field_prime.clone());
        //             evaluator.setWireValue(self.c.clone(), cValue);
        //         }
        //     }
        //     Box::new(Prover {
        //         a: self.a.clone(),
        //         b: self.b.clone(),
        //         c: self.c.clone(),
        //     })
        // });

        // to handle the case where a or b can be both zero, see below
        generator.addAssertion(
            &self.b,
            self.c.as_ref().unwrap(),
            &self.a,
            &self.debugStr("Assertion for division result"),
        );

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
}
impl GadgetConfig for FieldDivisionGadget {
    fn getOutputWires(&self) -> Vec<Option<WireType>> {
        vec![self.c.clone()]
    }
}
