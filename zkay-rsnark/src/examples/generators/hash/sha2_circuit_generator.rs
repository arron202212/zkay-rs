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
            circuit_generator::{CGConfig, CircuitGenerator,CGInstance, CircuitGeneratorExtend},
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
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::structure::circuit_generator::{
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
//     getActiveCircuitGenerator,
// };
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget::SHA256Gadget;
use zkay_derive::ImplStructNameConfig;
crate::impl_struct_name_for!(CircuitGeneratorExtend<SHA2CircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct SHA2CircuitGenerator {
    inputWires: Vec<Option<WireType>>,
    sha2Gadget: Option<Gadget<SHA256Gadget>>,
}
impl SHA2CircuitGenerator {
    pub fn new(circuit_name: &str) -> CircuitGeneratorExtend<Self> {
        //super(circuitName);
        CircuitGeneratorExtend::<Self>::new(
            circuit_name,
            Self {
                inputWires: vec![],
                sha2Gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<SHA2CircuitGenerator> {
    fn buildCircuit(&mut self) {
        // assuming the circuit input will be 64 bytes
        let inputWires = self.createInputWireArray(64,&None);
        // this gadget is not applying any padding.
        let sha2Gadget = SHA256Gadget::new(inputWires.clone(), 8, 64, false, false,&None,self.cg());
        let digest = sha2Gadget.getOutputWires();
        self.makeOutputArray(digest, &Some("digest".to_owned()));

        // ======================================================================
        // To see how padding can be done, and see how the gadget library will save constraints automatically,
        // try the snippet below instead.
        /*
            inputWires = createInputWireArray(3); 	// 3-byte input
            sha2Gadget = SHA256Gadget::new(inputWires, 8, 3, false, true);
            Vec<Option<WireType>> digest = sha2Gadget.getOutputWires();
            makeOutputArray(digest, "digest");
        */
        (self.t.inputWires, self.t.sha2Gadget) = (inputWires, Some(sha2Gadget));
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        let inputStr = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl";
        for i in 0..self.t.inputWires.len() {
            evaluator.setWireValuei(self.t.inputWires[i].as_ref().unwrap(), inputStr[i] as i64);
        }
    }
}
pub fn main(args: Vec<String>) {
    let mut generator = SHA2CircuitGenerator::new("sha_256");
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit();
    generator.prepFiles(Some(evaluator));
    generator.runLibsnark();
}
