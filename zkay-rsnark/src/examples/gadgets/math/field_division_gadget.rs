#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::circuit::InstanceOf;
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget::{Gadget, GadgetConfig};

use crate::circuit::structure::circuit_generator::CreateConstantWire;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
    get_active_circuit_generator,
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

use std::fs::File;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct FieldDivisionGadget {
    pub a: WireType,
    pub b: WireType,
    pub c: Vec<Option<WireType>>,
}
impl FieldDivisionGadget {
    pub fn new(
        a: WireType,
        b: WireType,
        desc: &Option<String>,
        mut generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let start = std::time::Instant::now();
        let mut _self = Gadget::<Self>::new(generator.clone(), desc, Self { a, b, c: vec![] });

        // let mut generator = self.me.clone().unwrap().upgrade().unwrap();

        // if the input values are constant (i.e. known at compilation time), we
        // can save one constraint
        if _self.t.a.instance_of("ConstantWire") && _self.t.b.instance_of("ConstantWire") {
            let aConst = _self.t.a.try_as_constant_ref().unwrap().get_constant();

            let bInverseConst = _self
                .t
                .b
                .try_as_constant_ref()
                .unwrap()
                .get_constant()
                .modinv(&Configs.field_prime)
                .unwrap();

            _self.t.c = vec![Some(generator.create_constant_wire(
                &aConst.mul(bInverseConst).rem(&Configs.field_prime),
                &None,
            ))];
        } else {
            let debug_str = _self.debug_str("division result");

            let pww = CircuitGenerator::create_prover_witness_wire(generator, &debug_str);

            _self.t.c = vec![Some(pww)];

            _self.build_circuit();
        }
        _self
    }
}
impl Gadget<FieldDivisionGadget> {
    fn build_circuit(&mut self) {
        let start = std::time::Instant::now();
        // This is an example of computing a value outside the circuit and
        // verifying constraints about it in the circuit. See notes below.

        let (a, b, c) = (&self.t.a, &self.t.b, self.t.c[0].as_ref().unwrap());
        let prover = crate::impl_prover!(
                                eval(a: WireType,
                                b: WireType,
                                c: WireType)  {
                impl Instruction for Prover{
                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                       let aValue = evaluator.get_wire_value(&self.a);
                                    let bValue = evaluator.get_wire_value(&self.b);
                                    // println!("===bValue==={}====={bValue}====",self.b);
                                    let cValue = aValue
                                        .mul(bValue.modinv(&Configs.field_prime).unwrap())
                                        .rem(&Configs.field_prime);
                                    evaluator.set_wire_value(&self.c, &cValue);
        Ok(())
                }
                }
                            }
                        );

        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);

        // CircuitGenerator::specify_prover_witness_computation(generator.clone(),&|evaluator: &mut CircuitEvaluator| {
        //     let aValue = evaluator.get_wire_value(self.t.a.clone());
        //     let bValue = evaluator.get_wire_value(self.t.b.clone());
        //     let cValue = aValue
        //         .mul(bValue.modinv(&Configs.field_prime.clone()).unwrap())
        //         .rem(&Configs.field_prime);
        //     evaluator.set_wire_value(self.t.c.clone(), cValue);
        // });
        // {
        //     #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
        //     struct Prover {
        //         a: WireType,
        //         b: WireType,
        //         c: WireType,
        //     }
        //     impl  Instruction for Prover {
        //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
        //             let aValue = evaluator.get_wire_value(self.t.a.clone());
        //             let bValue = evaluator.get_wire_value(self.t.b.clone());
        //             let cValue = aValue
        //                 .mul(bValue.modinv(&Configs.field_prime.clone()).unwrap())
        //                 .rem(&Configs.field_prime);
        //             evaluator.set_wire_value(self.t.c.clone(), cValue);
        //         }
        //     }
        //     Box::new(Prover {
        //         a: self.t.a.clone(),
        //         b: self.t.b.clone(),
        //         c: self.t.c.clone(),
        //     })
        // });

        let generator = self.generator.clone();

        let debug_str = self.debug_str("Assertion for division result");

        // to handle the case where a or b can be both zero, see below
        CircuitGenerator::add_assertion(generator, b, c, a, &debug_str);

        //Few notes: 1) The order of the above two statements matters (the
        //specification and the assertion). In the current version, it's not
        //possible to swap them, as in the evaluation sequence, the assertion
        //must happen after the value is assigned.
        //
        //2) The instruction defined above relies on the values of wires (a)
        //and (b) during runtime. This means that if any point later in the
        //program, the references a, and b referred to other wires, these wires
        //are going to be used instead in this instruction. Therefore, it will
        //be safer to use references in cases like that to reduce the
        //possibility of errors.
        //
        //3) The above constraint does not check if a and b are both zeros. In that
        //case, the prover will be able to use any value to make the constraint work.
        //When this case is problematic, enforce that b cannot have the value of zero.
        //
        //This can be done by proving that b has an inverse, that satisfies
        //b*(invB) = 1;
    }
}
impl GadgetConfig for Gadget<FieldDivisionGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.c
    }
}
