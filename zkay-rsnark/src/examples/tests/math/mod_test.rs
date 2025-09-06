#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire_type::WireType,
        },
    },
    examples::gadgets::math::mod_gadget::ModGadget,
    util::util::BigInteger,
};
use zkay_derive::ImplStructNameConfig;
#[cfg(test)]
mod test {
    use super::*;

    // TODO; add more tests

    #[test]
    pub fn mod_test_case1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const a: i64 = 1262178522;
            const b: i64 = 257; // b will be an input to the circuit
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires = CircuitGenerator::create_input_wire_array(self.cg(), 2, &None);
                //				WireType r = ModGadget::new(input_wires[0], (i32) Math.ceil(Math.log10(a) / Math.log10(2)), input_wires[1],
                //						(i32) Math.ceil(Math.log10(b) / Math.log10(2))).get_output_wires()[0];

                let r = ModGadget::new(
                    input_wires[0].clone().unwrap(),
                    input_wires[1].clone().unwrap(),
                    32,
                    &None,
                    self.cg(),
                )
                .get_output_wires()[0]
                    .clone();
                CircuitGenerator::make_output(self.cg(), r.as_ref().unwrap(), &None);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuei(self.t.input_wires[0].as_ref().unwrap(), CGTest::a);
                evaluator.set_wire_valuei(self.t.input_wires[1].as_ref().unwrap(), CGTest::b);
            }
        };
        let t = CGTest {
            input_wires: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Mod_Test1", t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let rWire = generator.get_out_wires()[0].clone();
        assert_eq!(
            evaluator.get_wire_value(rWire.as_ref().unwrap()),
            BigInteger::from(CGTest::a % CGTest::b)
        );
    }

    #[test]
    pub fn mod_test_case2() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const a: i64 = 1262178522;
            const b: i64 = 257; //  b will be a constant
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires = CircuitGenerator::create_input_wire_array(self.cg(), 1, &None);
                let r = ModConstantGadget::new(
                    input_wires[0].clone().unwrap(),
                    32,
                    BigInteger::from(CGTest::b),
                    &None,
                    self.cg(),
                )
                .get_output_wires()[0]
                    .clone();
                CircuitGenerator::make_output(self.cg(), r.as_ref().unwrap(), &None);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuei(self.t.input_wires[0].as_ref().unwrap(), CGTest::a);
            }
        };
        let t = CGTest {
            input_wires: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Mod_Test2", t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let rWire = generator.get_out_wires()[0].clone();
        assert_eq!(
            evaluator.get_wire_value(rWire.as_ref().unwrap()),
            BigInteger::from(CGTest::a % CGTest::b)
        );
    }
}
