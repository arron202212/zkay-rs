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
            wire_type::WireType,
        },
    },
    examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget,
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
    ops::{Add, Mul, Rem, Sub},
};
use zkay_derive::ImplStructNameConfig;

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AESSBoxNaiveLookupGadget {
    pub input: WireType,
    pub output: Vec<Option<WireType>>,
}
impl AESSBoxNaiveLookupGadget {
    pub fn new(
        input: WireType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                output: vec![],
                input,
            },
        );

        _self.build_circuit();
        _self
    }
}
impl Gadget<AESSBoxNaiveLookupGadget> {
    const SBOX: [u8; 256] = Gadget::<AES128CipherGadget>::SBOX;
    fn build_circuit(&mut self) {
        let mut output = self.generator.borrow().get_zero_wire().unwrap();
        for i in 0..256 {
            output = output.add(
                self.t
                    .input
                    .is_equal_toi(i, &None)
                    .muli(Self::SBOX[i as usize] as i64, &None),
            );
        }
        self.t.output = vec![Some(output)];
    }
}
impl GadgetConfig for Gadget<AESSBoxNaiveLookupGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
