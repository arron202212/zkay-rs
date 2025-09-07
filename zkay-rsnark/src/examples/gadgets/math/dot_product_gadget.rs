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
    ops::{Add, Mul, Sub},
};

use rccell::RcCell;
use zkay_derive::ImplStructNameConfig;

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct DotProductGadget {
    pub a: Vec<Option<WireType>>,
    pub b: Vec<Option<WireType>>,
    pub output: Vec<Option<WireType>>,
}
impl DotProductGadget {
    pub fn new(
        a: Vec<Option<WireType>>,
        b: Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(a.len() == b.len());
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                a,
                b,
                output: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<DotProductGadget> {
    fn build_circuit(&mut self) {
        let mut output = self.generator.get_zero_wire();
        for i in 0..self.t.a.len() {
            let product = self.t.a[i].as_ref().unwrap().mulw(
                self.t.b[i].as_ref().unwrap(),
                &Some(format!("Multiply elements # {i}")),
            );
            output = Some(output.clone().unwrap().add(&product));
        }
        self.t.output = vec![output];
    }
}
impl GadgetConfig for Gadget<DotProductGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
