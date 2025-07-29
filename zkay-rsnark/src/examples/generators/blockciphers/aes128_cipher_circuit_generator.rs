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
            circuit_generator::{CGConfig, CGInstance, CircuitGenerator, CircuitGeneratorExtend},
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
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::structure::circuit_generator::{
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
//     getActiveCircuitGenerator,
// };
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget::AES128CipherGadget;
use zkay_derive::ImplStructNameConfig;
// A sample usage of the AES128 block cipher gadget
crate::impl_struct_name_for!(CircuitGeneratorExtend<AES128CipherCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct AES128CipherCircuitGenerator {
    inputs: Vec<Option<WireType>>,
    key: Vec<Option<WireType>>,
    outputs: Vec<Option<WireType>>,
    gadget: Option<Gadget<AES128CipherGadget>>,
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
    fn buildCircuit(&mut self) {
        self.t.inputs = self.createInputWireArray(16, &None); // in bytes
        self.t.key = self.createInputWireArray(16, &None); // in bytes

        let expandedKey = Gadget::<AES128CipherGadget>::expandKey(&self.t.key, &self.cg);
        let gadget = AES128CipherGadget::new(self.t.inputs.clone(), expandedKey, &None, self.cg());
        self.t.outputs = gadget.getOutputWires().clone();
        for o in &self.t.outputs {
            self.makeOutput(o.as_ref().unwrap(), &None);
        }
        self.t.gadget = Some(gadget);
    }

    fn generateSampleInput(&self, circuitEvaluator: &mut CircuitEvaluator) {
        let keyV = BigInteger::parse_bytes(b"2b7e151628aed2a6abf7158809cf4f3c", 16).unwrap();
        let msgV = BigInteger::parse_bytes(b"ae2d8a571e03ac9c9eb76fac45af8e51", 16).unwrap();

        // expected output:0xf5d3d58503b9699de785895a96fdbaaf

        let mut keyArray = keyV.to_bytes_be().1.clone();
        let mut msgArray = msgV.to_bytes_be().1.clone();
        msgArray = msgArray[msgArray.len() - 16..].to_vec();
        keyArray = keyArray[keyArray.len() - 16..].to_vec();

        for i in 0..msgArray.len() {
            circuitEvaluator.setWireValuei(
                self.t.inputs[i].as_ref().unwrap(),
                (msgArray[i] as i64 & 0xff),
            );
        }

        for i in 0..keyArray.len() {
            circuitEvaluator
                .setWireValuei(self.t.key[i].as_ref().unwrap(), (keyArray[i] as i64 & 0xff));
        }
    }
}

pub fn main(args: Vec<String>) {
    use std::sync::atomic::{self, AtomicBool, Ordering};
    //Configs.hex_output_enabled = true;
    crate::circuit::config::config::atomic_hex_output_enabled.store(true, Ordering::Relaxed);
    let mut generator = AES128CipherCircuitGenerator::new("AES_Circuit");
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit().ok();
    generator.prepFiles(evaluator);
    generator.runLibsnark();
}
