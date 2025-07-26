#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget;
use crate::util::util::{BigInteger, Util};

/**
 * Tests SHA256 standard cases.
 *
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCase1() {
        let inputStr = "";
        let expectedDigest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let generator = CircuitGenerator::new("SHA2_Test1");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                inputWires = createInputWireArray(inputStr.len());
                let digest = SHA256Gadget::new(inputWires, 8, inputStr.len(), false, true, "")
                    .getOutputWires();
                makeOutputArray(digest);
            }

            fn generateSampleInput(e: &mut CircuitEvaluator) {
                // no input needed
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let evaluator = generator.getCircuitEvaluator();

        let outDigest = "";
        for w in generator.get_out_wires() {
            outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
        }
        assertEquals(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase2() {
        let inputStr = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let expectedDigest = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";

        let generator = CircuitGenerator::new("SHA2_Test2");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                inputWires = createInputWireArray(inputStr.len());
                let digest = SHA256Gadget::new(inputWires, 8, inputStr.len(), false, true, "")
                    .getOutputWires();
                makeOutputArray(digest);
            }

            fn generateSampleInput(e: &mut CircuitEvaluator) {
                for i in 0..inputStr.len() {
                    e.setWireValue(inputWires[i], inputStr.charAt(i));
                }
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let evaluator = generator.getCircuitEvaluator();

        let outDigest = "";
        for w in generator.get_out_wires() {
            outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
        }
        assertEquals(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase3() {
        let inputStr = "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
        let expectedDigest = "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1";

        let generator = CircuitGenerator::new("SHA2_Test3");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                inputWires = createInputWireArray(inputStr.len());
                let digest = SHA256Gadget::new(inputWires, 8, inputStr.len(), false, true, "")
                    .getOutputWires();
                makeOutputArray(digest);
            }

            fn generateSampleInput(e: &mut CircuitEvaluator) {
                for i in 0..inputStr.len() {
                    e.setWireValue(inputWires[i], inputStr.charAt(i));
                }
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let evaluator = generator.getCircuitEvaluator();

        let outDigest = "";
        for w in generator.get_out_wires() {
            outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
        }
        assertEquals(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase4() {
        let inputStr = "abc";
        let expectedDigest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

        let generator = CircuitGenerator::new("SHA2_Test4");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                inputWires = createInputWireArray(inputStr.len());
                let digest = SHA256Gadget::new(inputWires, 8, inputStr.len(), false, true, "")
                    .getOutputWires();
                makeOutputArray(digest);
            }

            fn generateSampleInput(e: &mut CircuitEvaluator) {
                for i in 0..inputStr.len() {
                    e.setWireValue(inputWires[i], inputStr.charAt(i));
                }
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let evaluator = generator.getCircuitEvaluator();

        let outDigest = "";
        for w in generator.get_out_wires() {
            outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
        }
        assertEquals(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase5() {
        let inputStr = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let expectedDigest = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";

        // Testing different settings of the bitWidthPerInputElement parameter
        // wordSize = # of bytes per input wire

        for wordSize in 1..=Configs.log2_field_prime / 8 - 1 {
            let numBytesPerInputWire = wordSize;

            let generator = CircuitGenerator::new("SHA2_Test5");

            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                inputWires: Vec<Option<WireType>>,
            }
            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    inputWires = createInputWireArray(
                        inputStr.len() / numBytesPerInputWire
                            + if inputStr.len() % numBytesPerInputWire != 0 {
                                1
                            } else {
                                0
                            },
                    );
                    let digest = SHA256Gadget::new(
                        inputWires,
                        8 * numBytesPerInputWire,
                        inputStr.len(),
                        false,
                        true,
                        "",
                    )
                    .getOutputWires();
                    makeOutputArray(digest);
                }

                fn generateSampleInput(e: &mut CircuitEvaluator) {
                    for i in 0..inputWires.len() {
                        let sum = BigInteger::ZERO;
                        for j in i * numBytesPerInputWire
                            ..j < inputStr.len().min((i + 1) * numBytesPerInputWire)
                        {
                            let v = BigInteger::from(inputStr.charAt(j));
                            sum = sum.add(v.shl((j % numBytesPerInputWire) * 8));
                        }
                        e.setWireValue(inputWires[i], sum);
                    }
                }
            };

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();

            let outDigest = "";
            for w in generator.get_out_wires() {
                outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
            }
            assertEquals(outDigest, expectedDigest);
        }
    }
}
