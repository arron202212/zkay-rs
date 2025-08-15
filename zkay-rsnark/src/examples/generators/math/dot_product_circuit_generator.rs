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
use crate::examples::gadgets::math::dot_product_gadget::DotProductGadget;
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
    fn buildCircuit(&mut self) {
        let a = self.createInputWireArray(self.t.dimension as usize, &Some("Input a".to_owned()));
        let b = self.createInputWireArray(self.t.dimension as usize, &Some("Input b".to_owned()));

        let dotProductGadget = DotProductGadget::new(a.clone(), b.clone(), &None, self.cg());
        let result = dotProductGadget.getOutputWires();
        self.makeOutput(
            result[0].as_ref().unwrap(),
            &Some("output of dot product a, b".to_owned()),
        );
        (self.t.a, self.t.b) = (a, b);
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        for i in 0..self.t.dimension as usize {
            evaluator.setWireValuei(self.t.a[i].as_ref().unwrap(), 10 + i as i64);
            evaluator.setWireValuei(self.t.b[i].as_ref().unwrap(), 20 + i as i64);
        }
    }
}
pub fn main(args: Vec<String>) {
    let mut generator = DotProductCircuitGenerator::new("dot_product", 3);
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit().ok();
    generator.prepFiles(evaluator);
    generator.runLibsnark();
}
