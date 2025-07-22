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
use examples::gadgets::blockciphers::chaskey_lts128_cipher_gadget;

// test case from:  https://www.cryptolux.org/index.php/FELICS

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCase1() {
        let generator = CircuitGenerator::new("Chaskey_Test1");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 4 32-bit words
            key: Vec<Option<WireType>>,        // 4 32-bit words
            ciphertext: Vec<Option<WireType>>, // 4 32-bit words
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                plaintext = createInputWireArray(4);
                key = createInputWireArray(4);
                ciphertext = ChaskeyLTS128CipherGadget::new(plaintext, key).getOutputWires();
                makeOutputArray(ciphertext);
            }

            pub fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                let keyV = [
                    BigInteger::from(0x68e90956L),
                    BigInteger::from(0x29e3585fL),
                    BigInteger::from(0x98ecec40L),
                    BigInteger::from(0x2f9822c5L),
                ];

                let msgV = [
                    BigInteger::from(0x262823b8L),
                    BigInteger::from(0x5e405efdL),
                    BigInteger::from(0xa901a369L),
                    BigInteger::from(0xd87aea78L),
                ];

                for i in 0..plaintext.len() {
                    evaluator.setWireValue(plaintext[i], msgV[i]);
                }
                for i in 0..key.len() {
                    evaluator.setWireValue(key[i], keyV[i]);
                }
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let evaluator = generator.getCircuitEvaluator();
        let cipherText = generator.get_out_wires();

        let expeectedCiphertext = [
            BigInteger::from(0x4d8d60d5L),
            BigInteger::from(0x7b34bfa2L),
            BigInteger::from(0x2f77f8abL),
            BigInteger::from(0x07deeddfL),
        ];

        for i in 0..4 {
            assertEquals(
                evaluator.getWireValue(cipherText.get(i)),
                expeectedCiphertext[i],
            );
        }
    }
}
