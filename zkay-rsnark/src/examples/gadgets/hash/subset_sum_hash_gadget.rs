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
            gadget::{Gadget, GadgetConfig},
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
// use crate::circuit::config::config::Configs;
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
// use crate::util::util::{Util,BigInteger};
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SubsetSumHashGadget {
    pub inputWires: Vec<Option<WireType>>,
    pub outWires: Vec<Option<WireType>>,
    pub binaryOutput: bool,
}
use std::sync::OnceLock;
static COEFFS: OnceLock<Vec<Vec<BigInteger>>> = OnceLock::new();
impl SubsetSumHashGadget {
    pub const DIMENSION: i32 = 3; // set to 4 for higher security
    pub const INPUT_LENGTH: i32 = 2 * Self::DIMENSION * 64; //Configs.log2_field_prime as i32; // length in bits
    /**
     * @param ins
     *            The bitwires of the input.
     * @param binaryOutput
     *            Whether the output digest should be splitted into bits or not.
     * @param desc
     */
    pub fn new(
        ins: Vec<Option<WireType>>,
        binaryOutput: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let numBlocks = (ins.len() as f64 * 1.0 / Self::INPUT_LENGTH as f64).ceil() as i32;

        assert!(numBlocks <= 1, "Only one block is supported at this point");

        let rem = (numBlocks * Self::INPUT_LENGTH) as usize - ins.len();

        let mut pad = vec![None; rem];
        for i in 0..pad.len() {
            pad[i] = generator.get_zero_wire(); // TODO: adjust padding
        }
        let inputWires = Util::concat(&ins, &pad);
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                binaryOutput,
                inputWires,
                outWires: vec![],
            },
        };

        _self.buildCircuit();
        _self
    }
}
impl Gadget<SubsetSumHashGadget> {
    fn buildCircuit(&mut self) {
        let (dimension, input_length) = (
            SubsetSumHashGadget::DIMENSION as usize,
            SubsetSumHashGadget::INPUT_LENGTH as usize,
        );
        let COEFFSS = COEFFS.get_or_init(|| {
            let mut tmp = vec![vec![BigInteger::default(); input_length]; dimension];
            for i in 0..dimension {
                for k in 0..input_length {
                    tmp[i][k] = Util::nextRandomBigInteger(&Configs.field_prime);
                }
            }
            tmp
        });
        let mut outDigest = vec![self.generator.get_zero_wire(); dimension];

        for i in 0..dimension {
            for j in 0..input_length {
                let t = self.t.inputWires[j]
                    .as_ref()
                    .unwrap()
                    .mulb(&COEFFSS[i][j], &None);
                outDigest[i] = Some(outDigest[i].clone().unwrap().add(t));
            }
        }
        if !self.t.binaryOutput {
            self.t.outWires = outDigest;
        } else {
            self.t.outWires = vec![None; dimension * Configs.log2_field_prime as usize];
            for i in 0..dimension {
                let bits = outDigest[i]
                    .as_ref()
                    .unwrap()
                    .getBitWiresi(Configs.log2_field_prime, &None)
                    .asArray()
                    .clone();
                for j in 0..bits.len() {
                    self.t.outWires[j + i * Configs.log2_field_prime as usize] = bits[j].clone();
                }
            }
        }
    }
}
impl GadgetConfig for Gadget<SubsetSumHashGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.outWires
    }
}
