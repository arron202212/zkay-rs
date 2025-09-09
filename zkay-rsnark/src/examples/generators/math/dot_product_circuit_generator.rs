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
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            },
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::math::dot_product_gadget::DotProductGadget,
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use zkay_derive::ImplStructNameConfig;

crate::impl_struct_name_for!(CircuitGeneratorExtend<DotProductCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct DotProductCircuitGenerator {
    pub a: Vec<Option<WireType>>,
    pub b: Vec<Option<WireType>>,
    pub dimension: i32,
}
impl DotProductCircuitGenerator {
    pub fn new(circuit_name: &str, dimension: i32) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                a: vec![],
                b: vec![],
                dimension,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<DotProductCircuitGenerator> {
    fn build_circuit(&mut self) {
        let a = CircuitGenerator::create_input_wire_array_with_str(
            self.cg(),
            self.t.dimension as usize,
            "Input a",
        );
        let b = CircuitGenerator::create_input_wire_array_with_str(
            self.cg(),
            self.t.dimension as usize,
            "Input b",
        );

        let dot_product_gadget = DotProductGadget::new(a.clone(), b.clone(), &None, self.cg());
        let result = dot_product_gadget.get_output_wires();
        CircuitGenerator::make_output_with_str(
            self.cg(),
            result[0].as_ref().unwrap(),
            "output of dot product a, b",
        );
        (self.t.a, self.t.b) = (a, b);
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        for i in 0..self.t.dimension as usize {
            evaluator.set_wire_valuei(self.t.a[i].as_ref().unwrap(), 10 + i as i64);
            evaluator.set_wire_valuei(self.t.b[i].as_ref().unwrap(), 20 + i as i64);
        }
    }
}
pub fn main(args: Vec<String>) {
    let mut generator = DotProductCircuitGenerator::new("dot_product", 3);
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
