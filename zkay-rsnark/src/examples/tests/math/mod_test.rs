#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::math::mod_constant_gadget;
use examples::gadgets::math::mod_gadget;
#[cfg(test)]
mod test {
    use super::*;

    // TODO; add more tests

    #[test]
    pub fn testCase1() {
        let a = 1262178522;
        let b = 257; // b will be an input to the circuit

        let generator = CircuitGenerator::new("Mod_Test1");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                inputWires = createInputWireArray(2);
                //				WireType r = ModGadget::new(inputWires[0], (i32) Math.ceil(Math.log10(a) / Math.log10(2)), inputWires[1],
                //						(i32) Math.ceil(Math.log10(b) / Math.log10(2))).getOutputWires()[0];

                let r = ModGadget::new(inputWires[0], inputWires[1], 32).getOutputWires()[0];
                makeOutput(r);
            }

            pub fn generateSampleInput(e: &mut CircuitEvaluator) {
                e.setWireValue(inputWires[0], a);
                e.setWireValue(inputWires[1], b);
            }
        };

        generator.generateCircuit();
        let evaluator = CircuitEvaluator::new("CGTest");
        generator.generateSampleInput(evaluator);
        evaluator.evaluate();
        let rWire = generator.get_out_wires().get(0);
        assertEquals(evaluator.getWireValue(rWire), BigInteger::from(a % b));
    }

    #[test]
    pub fn testCase2() {
        let a = 1262178522;
        let b = 257; // b will be a constant

        let generator = CircuitGenerator::new("Mod_Test2");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                inputWires = createInputWireArray(1);
                let r = ModConstantGadget::new(inputWires[0], 32, BigInteger::from(b))
                    .getOutputWires()[0];
                makeOutput(r);
            }

            pub fn generateSampleInput(e: &mut CircuitEvaluator) {
                e.setWireValue(inputWires[0], a);
            }
        };

        generator.generateCircuit();
        let evaluator = CircuitEvaluator::new("CGTest");
        generator.generateSampleInput(evaluator);
        evaluator.evaluate();
        let rWire = generator.get_out_wires().get(0);
        assertEquals(evaluator.getWireValue(rWire), BigInteger::from(a % b));
    }
}
