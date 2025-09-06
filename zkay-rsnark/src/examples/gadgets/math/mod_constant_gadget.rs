#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
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
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use num_bigint::Sign;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Div, Mul, Rem, Sub};

//  * This gadget provides the remainder of a % b, where b is a circuit constant.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ModConstantGadget {
    pub a: WireType,
    pub b: BigInteger,
    pub r: Vec<Option<WireType>>,
    pub q: WireType,
    pub bitwidth: i32, // a's bitwidth
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

        // TODO: add further checks.
        _self.build_circuit();
        _self
    }
}
impl Gadget<ModConstantGadget> {
    fn build_circuit(&mut self) {
        let r = CircuitGenerator::create_prover_witness_wire(
            self.generator.clone(),
            &Some("mod result".to_owned()),
        );
        let q = CircuitGenerator::create_prover_witness_wire(
            self.generator.clone(),
            &Some("division result".to_owned()),
        );
        let (a, b) = (&self.t.a, &self.t.b);
        // notes about how to use this code block can be found in FieldDivisionGadget
        // CircuitGenerator::specify_prover_witness_computation(generator.clone(),  &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.get_wire_value(a);
        //             let rValue = aValue.rem(b);
        //             evaluator.set_wire_value(r, &rValue);
        //             let qValue = aValue.divide(b);
        //             evaluator.set_wire_value(q, &qValue);
        //         });

        let prover = crate::impl_prover!(
                                            eval(  a: WireType,
                b: BigInteger,
                r: WireType,
                q: WireType
                                    )  {
                            impl Instruction for Prover{
                             fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                        let aValue = evaluator.get_wire_value(&self.a);
                                let rValue = aValue.clone().rem(&self.b);
                                evaluator.set_wire_value(&self.r, &rValue);
                                let qValue = aValue.div(&self.b);
                                evaluator.set_wire_value(&self.q, &qValue);
        Ok(())

                            }
                            }
                                        }
                                    );
        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);
        // {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let aValue = evaluator.get_wire_value(a);
        //             let rValue = aValue.rem(b);
        //             evaluator.set_wire_value(r, rValue);
        //             let qValue = aValue.divide(b);
        //             evaluator.set_wire_value(q, qValue);
        //         }
        //     }
        //     Prover
        // });

        let bBitwidth = b.bits();
        r.restrict_bit_length(bBitwidth, &None);
        q.restrict_bit_length(self.t.bitwidth as u64 - bBitwidth + 1, &None);

        CircuitGenerator::add_one_assertion(
            self.generator.clone(),
            &r.is_less_thanb(&b, bBitwidth as i32, &None),
            &None,
        );

        CircuitGenerator::add_equality_assertion(
            self.generator.clone(),
            &q.mulb(&b, &None).add(&r),
            &a,
            &None,
        );
        (self.t.r, self.t.q) = (vec![Some(r)], q);
    }
}
impl GadgetConfig for Gadget<ModConstantGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.r
    }
}
