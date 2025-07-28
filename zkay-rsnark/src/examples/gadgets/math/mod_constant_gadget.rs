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
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::{ConstantWire, new_constant},
            variable_bit_wire::VariableBitWire,
            variable_wire::{VariableWire, new_variable},
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

use num_bigint::Sign;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Div, Mul, Rem, Sub};
/**
 * This gadget provides the remainder of a % b, where b is a circuit constant.
 *
 *
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ModConstantGadget {
    a: WireType,
    b: BigInteger,
    r: Vec<Option<WireType>>,
    q: WireType,
    bitwidth: i32, // a's bitwidth
}
impl ModConstantGadget {
    pub fn new(
        a: WireType,
        bitwidth: i32,
        b: BigInteger,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(
            b.sign() == Sign::Plus,
            "b must be a positive constant. Signed operations not supported yet."
        );

        assert!(
            bitwidth as u64 >= b.bits(),
            "a's bitwidth < b's bitwidth -- This gadget is not needed."
        );
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                q: a.clone(),
                a,
                b,
                r: vec![],
                bitwidth,
            },
        };

        // TODO: add further checks.
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ModConstantGadget> {
    fn buildCircuit(&mut self) {
        let r = self
            .generator
            .createProverWitnessWire(&Some("mod result".to_owned()));
        let q = self
            .generator
            .createProverWitnessWire(&Some("division result".to_owned()));
        let (a, b) = (&self.t.a, &self.t.b);
        // notes about how to use this code block can be found in FieldDivisionGadget
        // generator.specifyProverWitnessComputation(  &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a);
        //             let rValue = aValue.rem(b);
        //             evaluator.setWireValue(r, &rValue);
        //             let qValue = aValue.divide(b);
        //             evaluator.setWireValue(q, &qValue);
        //         });

        let prover = crate::impl_prover!(
                                    eval(  a: WireType,
        b: BigInteger,
        r: WireType,
        q: WireType
                            )  {
                    impl Instruction for Prover{
                     fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                                let aValue = evaluator.getWireValue(&self.a);
                        let rValue = aValue.clone().rem(&self.b);
                        evaluator.setWireValue(&self.r, &rValue);
                        let qValue = aValue.div(&self.b);
                        evaluator.setWireValue(&self.q, &qValue);


                    }
                    }
                                }
                            );
        self.generator.specifyProverWitnessComputation(prover);
        // {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.getWireValue(a);
        //             let rValue = aValue.rem(b);
        //             evaluator.setWireValue(r, rValue);
        //             let qValue = aValue.divide(b);
        //             evaluator.setWireValue(q, qValue);
        //         }
        //     }
        //     Prover
        // });

        let bBitwidth = b.bits();
        r.restrictBitLength(bBitwidth, &None);
        q.restrictBitLength(self.t.bitwidth as u64 - bBitwidth + 1, &None);
        self.generator
            .addOneAssertion(&r.isLessThanb(&b, bBitwidth as i32, &None), &None);
        self.generator
            .addEqualityAssertion(&q.mulb(&b, &None).add(&r), &a, &None);
        (self.t.r, self.t.q) = (vec![Some(r)], q);
    }
}
impl GadgetConfig for Gadget<ModConstantGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.r
    }
}
