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
use crate::examples::gadgets::blockciphers::aes128_cipher_gadget;
use crate::examples::gadgets::blockciphers::sbox::aes_s_box_gadget_optimized2;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCase1() {
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "ae2d8a571e03ac9c9eb76fac45af8e51"
        // ciphertext: "f5d3d58503b9699de785895a96fdbaaf"

        // testing all available sBox implementations
        for sboxOption in AES128CipherGadget.SBoxOption.values() {
            // AES128CipherGadget.sBoxOption = sboxOption;
            let generator = CircuitGenerator::new("AES128_Test1_" + sboxOption);
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                plaintext: Vec<Option<WireType>>,  // 16 bytes
                key: Vec<Option<WireType>>,        // 16 bytes
                ciphertext: Vec<Option<WireType>>, // 16 bytes
            }
            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    plaintext = createInputWireArray(16);
                    key = createInputWireArray(16);
                    let expandedKey = AES128CipherGadget.expandKey(key);
                    ciphertext = AES128CipherGadget::new(plaintext, expandedKey).getOutputWires();
                    makeOutputArray(ciphertext);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    let keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
                    let msgV = BigInteger::new("ae2d8a571e03ac9c9eb76fac45af8e51", 16);

                    let keyArray = keyV.toByteArray();
                    let msgArray = msgV.toByteArray();
                    msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                    keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                    for i in 0..plaintext.len() {
                        evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
                    }
                    for i in 0..key.len() {
                        evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
                    }
                }
            }

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            let cipherText = generator.get_out_wires();

            let result = BigInteger::new("f5d3d58503b9699de785895a96fdbaaf", 16);

            let resultArray = result.toByteArray();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assertEquals(
                    evaluator.getWireValue(cipherText.get(i)),
                    BigInteger::from((resultArray[i] + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn testCase2() {
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "6bc1bee22e409f96e93d7e117393172a"
        // ciphertext: "3ad77bb40d7a3660a89ecaf32466ef97"

        // testing all available sBox implementations
        for sboxOption in AES128CipherGadget.SBoxOption.values() {
            // AES128CipherGadget.sBoxOption = sboxOption;
            let generator = CircuitGenerator::new("AES128_Test2_" + sboxOption);
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                plaintext: Vec<Option<WireType>>,  // 16 bytes
                key: Vec<Option<WireType>>,        // 16 bytes
                ciphertext: Vec<Option<WireType>>, // 16 bytes
            }
            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    plaintext = createInputWireArray(16);
                    key = createInputWireArray(16);
                    let expandedKey = AES128CipherGadget.expandKey(key);
                    ciphertext = AES128CipherGadget::new(plaintext, expandedKey).getOutputWires();
                    makeOutputArray(ciphertext);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    let keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
                    let msgV = BigInteger::new("6bc1bee22e409f96e93d7e117393172a", 16);

                    let keyArray = keyV.toByteArray();
                    let msgArray = msgV.toByteArray();
                    msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                    keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                    for i in 0..plaintext.len() {
                        evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
                    }
                    for i in 0..key.len() {
                        evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
                    }
                }
            };

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            let cipherText = generator.get_out_wires();

            let result = BigInteger::new("3ad77bb40d7a3660a89ecaf32466ef97", 16);

            // expected output:0xf5d3d58503b9699de785895a96fdbaaf

            let resultArray = result.toByteArray();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assertEquals(
                    evaluator.getWireValue(cipherText.get(i)),
                    BigInteger::from((resultArray[i] + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn testCase3() {
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "6bc1bee22e409f96e93d7e117393172a"
        // ciphertext: "3ad77bb40d7a3660a89ecaf32466ef97"

        // testing all available sBox implementations
        for sboxOption in AES128CipherGadget.SBoxOption.values() {
            // AES128CipherGadget.sBoxOption = sboxOption;
            let generator = CircuitGenerator::new("AES128_Test3_" + sboxOption);
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                plaintext: Vec<Option<WireType>>,  // 16 bytes
                key: Vec<Option<WireType>>,        // 16 bytes
                ciphertext: Vec<Option<WireType>>, // 16 bytes
            }
            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    self.t.plaintext = createInputWireArray(16);
                    self.t.key = createInputWireArray(16);
                    let expandedKey = AES128CipherGadget.expandKey(key);
                    ciphertext = AES128CipherGadget::new(plaintext, expandedKey).getOutputWires();
                    makeOutputArray(ciphertext);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    let keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
                    let msgV = BigInteger::new("30c81c46a35ce411e5fbc1191a0a52ef", 16);

                    let keyArray = keyV.toByteArray();
                    let msgArray = msgV.toByteArray();
                    msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                    keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                    for i in 0..plaintext.len() {
                        evaluator.setWireValue(plaintext[i], msgArray[i] & 0xff);
                    }
                    for i in 0..key.len() {
                        evaluator.setWireValue(key[i], keyArray[i] & 0xff);
                    }
                }
            }

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            let cipherText = generator.get_out_wires();

            let result = BigInteger::new("43b1cd7f598ece23881b00e3ed030688", 16);

            let resultArray = result.toByteArray();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assertEquals(
                    evaluator.getWireValue(cipherText.get(i)),
                    BigInteger::from((resultArray[i] + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn testCase4() {
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "30c81c46a35ce411e5fbc1191a0a52ef"
        // ciphertext: "43b1cd7f598ece23881b00e3ed030688"

        // testing all available sBox implementations
        for sboxOption in AES128CipherGadget.SBoxOption.values() {
            // AES128CipherGadget.sBoxOption = sboxOption;
            let generator = CircuitGenerator::new("AES128_Test4_" + sboxOption);
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                plaintext: Vec<Option<WireType>>,  // 16 bytes
                key: Vec<Option<WireType>>,        // 16 bytes
                ciphertext: Vec<Option<WireType>>, // 16 bytes
            }
            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    plaintext = createInputWireArray(16);
                    key = createInputWireArray(16);
                    let expandedKey = AES128CipherGadget.expandKey(key);
                    ciphertext = AES128CipherGadget::new(plaintext, expandedKey).getOutputWires();
                    makeOutputArray(ciphertext);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    let keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
                    let msgV = BigInteger::new("f69f2445df4f9b17ad2b417be66c3710", 16);

                    let keyArray = keyV.toByteArray();
                    let msgArray = msgV.toByteArray();
                    msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                    keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                    for i in 0..plaintext.len() {
                        evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
                    }
                    for i in 0..key.len() {
                        evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
                    }
                }
            }

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            let cipherText = generator.get_out_wires();

            let result = BigInteger::new("7b0c785e27e8ad3f8223207104725dd4", 16);

            let resultArray = result.toByteArray();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assertEquals(
                    evaluator.getWireValue(cipherText.get(i)),
                    BigInteger::from((resultArray[i] + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn testCustomSboxImplementation() {
        AES128CipherGadget.sBoxOption = AES128CipherGadget.SBoxOption.OPTIMIZED2;
        for b in 0..=15 {
            AESSBoxGadgetOptimized2.setBitCount(b);
            AESSBoxGadgetOptimized2.solveLinearSystems();
            let generator = CircuitGenerator::new("AES128_Test_SBox_Parametrization_" + b);
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                plaintext: Vec<Option<WireType>>,  // 16 bytes
                key: Vec<Option<WireType>>,        // 16 bytes
                ciphertext: Vec<Option<WireType>>, // 16 bytes
            }
            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    plaintext = createInputWireArray(16);
                    key = createInputWireArray(16);
                    let expandedKey = AES128CipherGadget.expandKey(key);
                    ciphertext = AES128CipherGadget::new(plaintext, expandedKey).getOutputWires();
                    makeOutputArray(ciphertext);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    let keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
                    let msgV = BigInteger::new("f69f2445df4f9b17ad2b417be66c3710", 16);

                    let keyArray = keyV.toByteArray();
                    let msgArray = msgV.toByteArray();
                    msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                    keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                    for i in 0..plaintext.len() {
                        evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
                    }
                    for i in 0..key.len() {
                        evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
                    }
                }
            };

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            let cipherText = generator.get_out_wires();

            let result = BigInteger::new("7b0c785e27e8ad3f8223207104725dd4", 16);

            let resultArray = result.toByteArray();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assertEquals(
                    evaluator.getWireValue(cipherText.get(i)),
                    BigInteger::from((resultArray[i] + 256) % 256),
                );
            }
        }
    }
}
