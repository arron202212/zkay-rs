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
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct DotProductGadget {
    a: Vec<Option<WireType>>,
    b: Vec<Option<WireType>>,
    output: Vec<Option<WireType>>,
}
impl DotProductGadget {
    pub fn new(
        a: Vec<Option<WireType>>,
        b: Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        assert!(a.len() == b.len());
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                a,
                b,
                output: vec![],
            },
        };

        _self.buildCircuit();
        _self
    }
}
impl Gadget<DotProductGadget> {
    fn buildCircuit(&mut self) {
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
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
