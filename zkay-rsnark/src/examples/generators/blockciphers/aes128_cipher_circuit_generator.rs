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
            circuit_generator::{CGConfig, CGInstance, CircuitGenerator, CircuitGeneratorExtend},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget,
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use zkay_derive::ImplStructNameConfig;

// A sample usage of the Aes128 block cipher gadget
crate::impl_struct_name_for!(CircuitGeneratorExtend<AES128CipherCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AES128CipherCircuitGenerator {
    pub inputs: Vec<Option<WireType>>,
    pub key: Vec<Option<WireType>>,
    pub outputs: Vec<Option<WireType>>,
    pub gadget: Option<Gadget<AES128CipherGadget>>,
}
impl AES128CipherCircuitGenerator {
    pub fn new(circuit_name: &str) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::<Self>::new(
            circuit_name,
            Self {
                inputs: vec![],
                key: vec![],
                outputs: vec![],
                gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<AES128CipherCircuitGenerator> {
    fn build_circuit(&mut self) {
        self.t.inputs = CircuitGenerator::create_input_wire_array(self.cg(), 16); // in bytes
        self.t.key = CircuitGenerator::create_input_wire_array(self.cg(), 16); // in bytes

        let expanded_key = Gadget::<AES128CipherGadget>::expandKey(&self.t.key, &self.cg);
        let gadget = AES128CipherGadget::new(self.t.inputs.clone(), expanded_key, &None, self.cg());
        self.t.outputs = gadget.get_output_wires().clone();
        for o in &self.t.outputs {
            CircuitGenerator::make_output(self.cg(), o.as_ref().unwrap());
        }
        self.t.gadget = Some(gadget);
    }

    fn generate_sample_input(&self, circuit_evaluator: &mut CircuitEvaluator) {
        let key_v = Util::parse_big_int_x("2b7e151628aed2a6abf7158809cf4f3c");
        let msg_v = Util::parse_big_int_x("ae2d8a571e03ac9c9eb76fac45af8e51");

        // expected output:0xf5d3d58503b9699de785895a96fdbaaf

        let mut key_array = key_v.to_bytes_be().1.clone();
        let mut msg_array = msg_v.to_bytes_be().1.clone();
        msg_array = msg_array[msg_array.len() - 16..].to_vec();
        key_array = key_array[key_array.len() - 16..].to_vec();

        for i in 0..msg_array.len() {
            circuit_evaluator.set_wire_valuei(
                self.t.inputs[i].as_ref().unwrap(),
                (msg_array[i] as i64 & 0xff),
            );
        }

        for i in 0..key_array.len() {
            circuit_evaluator.set_wire_valuei(
                self.t.key[i].as_ref().unwrap(),
                (key_array[i] as i64 & 0xff),
            );
        }
    }
}

pub fn main(args: Vec<String>) {
    use std::sync::atomic::{self, AtomicBool, Ordering};
    //CONFIGS.hex_output_enabled = true;
    crate::circuit::config::config::ATOMIC_HEX_OUTPUT_ENABLED.store(true, Ordering::Relaxed);
    let mut generator = AES128CipherCircuitGenerator::new("AES_Circuit");
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
