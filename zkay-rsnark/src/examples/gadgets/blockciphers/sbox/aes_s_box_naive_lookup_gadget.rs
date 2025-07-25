#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::GadgetConfig;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
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
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget;
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Rem, Sub};
use zkay_derive::ImplStructNameConfig;

#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AESSBoxNaiveLookupGadget {
    input: WireType,
    output: Vec<Option<WireType>>,
}
impl AESSBoxNaiveLookupGadget {
    pub fn new(
        input: WireType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self> {
            generator,
            description: desc.as_ref().map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                output: vec![],
                input,
            },
        };

        _self.buildCircuit();
        _self
    }
}
impl Gadget<AESSBoxNaiveLookupGadget> {
    const SBox: [u8;256]  = Gadget::<AES128CipherGadget>::SBox;
    fn buildCircuit(&mut self) {
        let mut output = self.generator.borrow().get_zero_wire().unwrap();
        for i in 0..256 {
            output = output.add(
                self.t
                    .input
                    .isEqualToi(i, &None)
                    .muli(Self::SBox[i as usize] as i64, &None),
            );
        }
        self.t.output = vec![Some(output)];
    }
}
impl GadgetConfig for Gadget<AESSBoxNaiveLookupGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
