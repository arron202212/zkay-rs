#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::constant_wire;
use crate::circuit::structure::wire_type::WireType;
use zkay_derive::ImplStructNameConfig;
// see notes in the end of the code.
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct FieldDivisionGadget {
    a: WireType,
    b: WireType,
    c: WireType,
}
impl FieldDivisionGadget {
    fn new(a: WireType, b: WireType, desc: &String) -> Self {
        // super(desc);
        let mut _self = Self {
            a,
            b,
            c: WireType::default(),
        };
        let generator = CircuitGenerator::getActiveCircuitGenerator().unwrap();
        // if the input values are constant (i.e. known at compilation time), we
        // can save one constraint
        if _self.a.instance_of("ConstantWire") && _self.b.instance_of("ConstantWire") {
            let aConst = _self.a.getConstant();
            let bInverseConst = _self
                .b
                .getConstant()
                .modinv(&Configs.get().unwrap().field_prime.clone())
                .unwrap();
            _self.c = generator.createConstantWire(
                aConst
                    .mul(bInverseConst)
                    .rem(Configs.get().unwrap().field_prime.clone()),
                &String::new(),
            );
        } else {
            _self.c =
                generator.createProverWitnessWire(&_self.debugStr("division result".to_owned()));
            _self.buildCircuit();
        }
        _self
    }

    fn buildCircuit(&mut self) {
        // This is an example of computing a value outside the circuit and
        // verifying constraints about it in the circuit. See notes below.
        let generator = CircuitGenerator::getActiveCircuitGenerator().unwrap();
        generator.specifyProverWitnessComputation({
            #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
            struct Prover {
                a: WireType,
                b: WireType,
                c: WireType,
            }
            impl Instruction for Prover {
                fn evaluate(&self, evaluator: CircuitEvaluator) {
                    let aValue = evaluator.getWireValue(self.a.clone());
                    let bValue = evaluator.getWireValue(self.b.clone());
                    let cValue = aValue
                        .mul(
                            bValue
                                .modinv(&Configs.get().unwrap().field_prime.clone())
                                .unwrap(),
                        )
                        .rem(Configs.get().unwrap().field_prime.clone());
                    evaluator.setWireValue(self.c.clone(), cValue);
                }
            }
            Box::new(Prover {
                a: self.a.clone(),
                b: self.b.clone(),
                c: self.c.clone(),
            })
        });

        // to handle the case where a or b can be both zero, see below
        generator.addAssertion(
            self.b.clone(),
            self.c.clone(),
            self.a.clone(),
            &self.debugStr("Assertion for division result".to_owned()),
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
        return vec![Some(self.c.clone())];
    }
}
