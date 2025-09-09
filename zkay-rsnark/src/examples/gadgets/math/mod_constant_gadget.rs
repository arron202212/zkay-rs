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

use num_bigint::Sign;
use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

//  * This gadget provides the remainder of a % b, where b is a circuit constant.

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct ModConstantGadget {
    pub a: WireType,
    pub b: BigInteger,
    pub r: Vec<Option<WireType>>,
    pub q: WireType,
    pub bitwidth: i32, // a's bitwidth
}
impl ModConstantGadget {
    #[inline]
    pub fn new(
        a: WireType,
        bitwidth: i32,
        b: BigInteger,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        Self::new_with_option(a, bitwidth, b, &None, generator)
    }
    pub fn new_with_option(
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
        let r = CircuitGenerator::create_prover_witness_wire_with_str(
            self.generator.clone(),
            "mod result",
        );
        let q = CircuitGenerator::create_prover_witness_wire_with_str(
            self.generator.clone(),
            "division result",
        );
        let (a, b) = (&self.t.a, &self.t.b);

        let prover = crate::impl_prover!(
                                            eval(  a: WireType,
                b: BigInteger,
                r: WireType,
                q: WireType
                                    )  {
                            impl Instruction for Prover{
                             fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                        let a_value = evaluator.get_wire_value(&self.a);
                                let r_value = a_value.clone().rem(&self.b);
                                evaluator.set_wire_value(&self.r, &r_value);
                                let q_value = a_value.div(&self.b);
                                evaluator.set_wire_value(&self.q, &q_value);
        Ok(())

                            }
                            }
                                        }
                                    );
        CircuitGenerator::specify_prover_witness_computation(self.generator.clone(), prover);

        let b_bitwidth = b.bits();
        r.restrict_bit_length(b_bitwidth);
        q.restrict_bit_length(self.t.bitwidth as u64 - b_bitwidth + 1);

        CircuitGenerator::add_one_assertion(
            self.generator.clone(),
            &r.is_less_thanb(&b, b_bitwidth as i32),
        );

        CircuitGenerator::add_equality_assertion(self.generator.clone(), &q.mulb(&b).add(&r), &a);
        (self.t.r, self.t.q) = (vec![Some(r)], q);
    }
}
impl GadgetConfig for Gadget<ModConstantGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.r
    }
}
