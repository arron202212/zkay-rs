#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            gadget::GadgetConfig,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::eval::instruction::Instruction;
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;

use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Div, Mul, Rem, Sub};
/**
 * This gadget provides the remainder of a % b.
 *
 *
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ModGadget {
    pub a: WireType,
    pub b: WireType,
    pub r: Vec<Option<WireType>>,
    pub q: WireType,

    pub bitwidth: i32, // bitwidth for both a, b
}
impl ModGadget {
    pub fn new(
        a: WireType,
        b: WireType,
        bitwidth: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                q: a.clone(),
                a,
                b,
                r: vec![],
                bitwidth,
            },
        );
        assert!(bitwidth <= 126, "Bitwidth not supported yet.");

        _self.buildCircuit();
        _self
    }
}
impl Gadget<ModGadget> {
    fn buildCircuit(&mut self) {
        let r = CircuitGenerator::createProverWitnessWire(
            self.generator.clone(),
            &Some("mod result".to_owned()),
        );
        let q = CircuitGenerator::createProverWitnessWire(
            self.generator.clone(),
            &Some("division result".to_owned()),
        );
        let (a, b) = (&self.t.a, &self.t.b);
        // notes about how to use this code block can be found in FieldDivisionGadget
        // generator.specifyProverWitnessComputation( &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a);
        //             let bValue = evaluator.getWireValue(b);
        //             let rValue = aValue.rem(bValue);
        //             evaluator.setWireValue(r, &rValue);
        //             let qValue = aValue.div(bValue);
        //             evaluator.setWireValue(q, &qValue);
        //         });
        let prover = crate::impl_prover!(
                                    eval(  a: WireType,
        b: WireType,
        r: WireType,
        q: WireType
                            )  {
                    impl Instruction for Prover{
                     fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                                let aValue = evaluator.getWireValue(&self.a);
                        let bValue = evaluator.getWireValue(&self.b);
                        let rValue = aValue.clone().rem(&bValue);
                        evaluator.setWireValue(&self.r, &rValue);
                        let qValue = aValue.div(&bValue);
                        evaluator.setWireValue(&self.q, &qValue);


                    }
                    }
                                }
                            );
        self.generators.specifyProverWitnessComputation(prover);
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

        r.restrictBitLength(self.t.bitwidth as u64, &None);
        q.restrictBitLength(self.t.bitwidth as u64, &None);
        self.generator
            .addOneAssertion(&r.isLessThan(&b, self.t.bitwidth, &None), &None);
        self.generator
            .addEqualityAssertion(&q.clone().mul(b).add(&r), &a, &None);
        (self.t.r, self.t.q) = (vec![Some(r)], q);
    }
}
impl GadgetConfig for Gadget<ModGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.r
    }
}
