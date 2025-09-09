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
            gadget::{Gadget, GadgetConfig},
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

use rccell::RcCell;
use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Sub},
};
use zkay_derive::ImplStructNameConfig;

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SubsetSumHashGadget {
    pub input_wires: Vec<Option<WireType>>,
    pub out_wires: Vec<Option<WireType>>,
    pub binary_output: bool,
}
use std::sync::OnceLock;
static COEFFS: OnceLock<Vec<Vec<BigInteger>>> = OnceLock::new();
impl SubsetSumHashGadget {
    pub const DIMENSION: i32 = 3; // set to 4 for higher security
    pub const INPUT_LENGTH: i32 = 2 * Self::DIMENSION * 64; //CONFIGS.log2_field_prime as i32; // length in bits

    //@param ins
    //           The bitwires of the input.
    //@param binary_output
    //           Whether the output digest should be splitted into bits or not.
    //@param desc

    pub fn new(
        ins: Vec<Option<WireType>>,
        binary_output: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let num_blocks = (ins.len() as f64 * 1.0 / Self::INPUT_LENGTH as f64).ceil() as i32;

        assert!(num_blocks <= 1, "Only one block is supported at this point");

        let rem = (num_blocks * Self::INPUT_LENGTH) as usize - ins.len();

        let mut pad = vec![None; rem];
        for i in 0..pad.len() {
            pad[i] = generator.get_zero_wire(); // TODO: adjust padding
        }
        let input_wires = Util::concat(&ins, &pad);
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                binary_output,
                input_wires,
                out_wires: vec![],
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<SubsetSumHashGadget> {
    fn build_circuit(&mut self) {
        let (dimension, input_length) = (
            SubsetSumHashGadget::DIMENSION as usize,
            SubsetSumHashGadget::INPUT_LENGTH as usize,
        );
        let coeffss = COEFFS.get_or_init(|| {
            let mut tmp = vec![vec![BigInteger::default(); input_length]; dimension];
            for i in 0..dimension {
                for k in 0..input_length {
                    tmp[i][k] = Util::next_random_big_integer(&CONFIGS.field_prime);
                }
            }
            tmp
        });
        let mut out_digest = vec![self.generator.get_zero_wire(); dimension];

        for i in 0..dimension {
            for j in 0..input_length {
                let t = self.t.input_wires[j].as_ref().unwrap().mulb(&coeffss[i][j]);
                out_digest[i] = Some(out_digest[i].clone().unwrap().add(t));
            }
        }
        if !self.t.binary_output {
            self.t.out_wires = out_digest;
        } else {
            self.t.out_wires = vec![None; dimension * CONFIGS.log2_field_prime as usize];
            for i in 0..dimension {
                let bits = out_digest[i]
                    .as_ref()
                    .unwrap()
                    .get_bit_wiresi(CONFIGS.log2_field_prime)
                    .as_array()
                    .clone();
                for j in 0..bits.len() {
                    self.t.out_wires[j + i * CONFIGS.log2_field_prime as usize] = bits[j].clone();
                }
            }
        }
    }
}
impl GadgetConfig for Gadget<SubsetSumHashGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.out_wires
    }
}
