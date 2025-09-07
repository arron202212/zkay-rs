#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::CONFIGS,
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
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Div, Mul, Rem, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

//  * This gadget provides the remainder of a % b.

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

        _self.build_circuit();
        _self
    }
}
impl Gadget<ModGadget> {
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
        // CircuitGenerator::specify_prover_witness_computation(generator.clone(), &|evaluator: &mut CircuitEvaluator| {
        //             let a_value = evaluator.get_wire_value(a);
        //             let b_value = evaluator.get_wire_value(b);
        //             let r_value = a_value.rem(b_value);
        //             evaluator.set_wire_value(r, &r_value);
        //             let q_value = a_value.div(b_value);
        //             evaluator.set_wire_value(q, &q_value);
        //         });
        let prover = crate::impl_prover!(
                                            eval(  a: WireType,
                b: WireType,
                r: WireType,
                q: WireType
                                    )  {
                            impl Instruction for Prover{
                             fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                        let a_value = evaluator.get_wire_value(&self.a);
                                let b_value = evaluator.get_wire_value(&self.b);
                                let r_value = a_value.clone().rem(&b_value);
                                evaluator.set_wire_value(&self.r, &r_value);
                                let q_value = a_value.div(&b_value);
                                evaluator.set_wire_value(&self.q, &q_value);
        Ok(())

                            }
                            }
                                        }
                                    );
        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);
        //     {
        //     struct Prover;
        //     impl Instruction for Prover {
        //         &|evaluator: &mut CircuitEvaluator| {
        //             let a_value = evaluator.get_wire_value(a);
        //             let b_value = evaluator.get_wire_value(b);
        //             let r_value = a_value.rem(b_value);
        //             evaluator.set_wire_value(r, r_value);
        //             let q_value = a_value.div(b_value);
        //             evaluator.set_wire_value(q, q_value);
        //         }
        //     }
        //     Prover
        // });

        r.restrict_bit_length(self.t.bitwidth as u64, &None);
        q.restrict_bit_length(self.t.bitwidth as u64, &None);

        CircuitGenerator::add_one_assertion(
            self.generator.clone(),
            &r.is_less_thans(&b, self.t.bitwidth, &None),
            &None,
        );

        CircuitGenerator::add_equality_assertion(
            self.generator.clone(),
            &q.clone().mul(b).add(&r),
            &a,
            &None,
        );
        (self.t.r, self.t.q) = (vec![Some(r)], q);
    }
}
impl GadgetConfig for Gadget<ModGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.r
    }
}
