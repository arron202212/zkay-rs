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
    examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget},
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use zkay_derive::ImplStructNameConfig;
crate::impl_struct_name_for!(CircuitGeneratorExtend<SHA2CircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SHA2CircuitGenerator {
    pub input_wires: Vec<Option<WireType>>,
    pub sha2_gadget: Option<Gadget<SHA256Gadget<Base>>>,
}
impl SHA2CircuitGenerator {
    pub fn new(circuit_name: &str) -> CircuitGeneratorExtend<Self> {
        //super(circuitName);
        CircuitGeneratorExtend::<Self>::new(
            circuit_name,
            Self {
                input_wires: vec![],
                sha2_gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<SHA2CircuitGenerator> {
    fn build_circuit(&mut self) {
        // assuming the circuit input will be 64 bytes
        let input_wires = CircuitGenerator::create_input_wire_array(self.cg(), 64);
        // this gadget is not applying any padding.
        let sha2_gadget =
            SHA256Gadget::new(input_wires.clone(), 8, 64, false, false, self.cg(), Base);
        let digest = sha2_gadget.get_output_wires();
        CircuitGenerator::make_output_array_with_str(self.cg(), digest, "digest");

        // ======================================================================
        // To see how padding can be done, and see how the gadget library will save constraints automatically,
        // try the snippet below instead.

        // input_wires = create_input_wire_array(3); 	// 3-byte input
        // sha2_gadget = SHA256Gadget::new(input_wires, 8, 3, false, true);
        // Vec<Option<WireType>> digest = sha2_gadget.get_output_wires();
        // make_output_array(digest, "digest");

        (self.t.input_wires, self.t.sha2_gadget) = (input_wires, Some(sha2_gadget));
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        let input_str = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl";
        for i in 0..self.t.input_wires.len() {
            evaluator.set_wire_valuei(self.t.input_wires[i].as_ref().unwrap(), input_str[i] as i64);
        }
    }
}
pub fn main(args: Vec<String>) {
    let mut generator = SHA2CircuitGenerator::new("sha_256");
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
