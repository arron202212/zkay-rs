#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
    addToEvaluationQueue, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::mod_constant_gadget::ModConstantGadget;
use crate::examples::gadgets::math::mod_gadget::ModGadget;
use crate::util::util::BigInteger;
use zkay_derive::ImplStructNameConfig;
#[cfg(test)]
mod test {
    use super::*;

    // TODO; add more tests

    #[test]
    pub fn mod_test_case1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const a: i64 = 1262178522;
            const b: i64 = 257; // b will be an input to the circuit
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = CircuitGenerator::createInputWireArray(self.cg(), 2, &None);
                //				WireType r = ModGadget::new(inputWires[0], (i32) Math.ceil(Math.log10(a) / Math.log10(2)), inputWires[1],
                //						(i32) Math.ceil(Math.log10(b) / Math.log10(2))).getOutputWires()[0];

                let r = ModGadget::new(
                    inputWires[0].clone().unwrap(),
                    inputWires[1].clone().unwrap(),
                    32,
                    &None,
                    self.cg(),
                )
                .getOutputWires()[0]
                    .clone();
                CircuitGenerator::makeOutput(self.cg(), r.as_ref().unwrap(), &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuei(self.t.inputWires[0].as_ref().unwrap(), CGTest::a);
                evaluator.setWireValuei(self.t.inputWires[1].as_ref().unwrap(), CGTest::b);
            }
        };
        let t = CGTest { inputWires: vec![] };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Mod_Test1", t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let rWire = generator.get_out_wires()[0].clone();
        assert_eq!(
            evaluator.getWireValue(rWire.as_ref().unwrap()),
            BigInteger::from(CGTest::a % CGTest::b)
        );
    }

    #[test]
    pub fn mod_test_case2() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const a: i64 = 1262178522;
            const b: i64 = 257; //  b will be a constant
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = CircuitGenerator::createInputWireArray(self.cg(), 1, &None);
                let r = ModConstantGadget::new(
                    inputWires[0].clone().unwrap(),
                    32,
                    BigInteger::from(CGTest::b),
                    &None,
                    self.cg(),
                )
                .getOutputWires()[0]
                    .clone();
                CircuitGenerator::makeOutput(self.cg(), r.as_ref().unwrap(), &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuei(self.t.inputWires[0].as_ref().unwrap(), CGTest::a);
            }
        };
        let t = CGTest { inputWires: vec![] };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Mod_Test2", t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let rWire = generator.get_out_wires()[0].clone();
        assert_eq!(
            evaluator.getWireValue(rWire.as_ref().unwrap()),
            BigInteger::from(CGTest::a % CGTest::b)
        );
    }
}
