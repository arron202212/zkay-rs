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
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::ops::{Add, Div, Mul, Rem, Sub};
use zkay_derive::ImplStructNameConfig;

crate::impl_struct_name_for!(CircuitGeneratorExtend<SimpleCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SimpleCircuitGenerator {
    pub inputs: Vec<Option<WireType>>,
}
impl SimpleCircuitGenerator {
    pub fn new(circuit_name: &str) -> CircuitGeneratorExtend<Self> {
        //super(circuitName);
        CircuitGeneratorExtend::new(circuit_name, Self { inputs: vec![] })
    }
}
impl CGConfig for CircuitGeneratorExtend<SimpleCircuitGenerator> {
    fn build_circuit(&mut self) {
        // declare input array of length 4.
        let inputs = CircuitGenerator::create_input_wire_array(self.cg(), 4);

        // r1 = in0 * in1
        let r1 = inputs[0].clone().unwrap().mul(inputs[1].as_ref().unwrap());

        // r2 = in2 + in3
        let r2 = inputs[2].clone().unwrap().add(inputs[3].as_ref().unwrap());

        // result = (r1+5)*(6*r2)
        let result = r1.add(5).mul(&r2.muli(6, &None));

        // mark the wire as output
        CircuitGenerator::make_output(self.cg(), &result, &None);
        self.t.inputs = inputs;
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        for i in 0..4 {
            evaluator.set_wire_valuei(self.t.inputs[i].as_ref().unwrap(), i as i64 + 1);
        }
    }
}

pub fn main(args: Vec<String>) {
    let mut generator = SimpleCircuitGenerator::new("simple_example");
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
