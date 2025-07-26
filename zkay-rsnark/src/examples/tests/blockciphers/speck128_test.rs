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
use crate::examples::gadgets::blockciphers::speck128_cipher_gadget;

/**
 * Tests SPECK block cipher @ keysize = 128, blocksize = 128.
 * Test vector obtained from:  https://github.com/inmcm/Simon_Speck_Ciphers/blob/master/Python/SimonSpeckCiphers/tests/test_simonspeck.py
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCase1() {
        let generator = CircuitGenerator::new("Speck128_Test");
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>, // 2 64-bit words
            key: Vec<Option<WireType>>,       // 2 64-bit words
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                plaintext = createInputWireArray(2);
                key = createInputWireArray(2);
                let expandedKey = Speck128CipherGadget.expandKey(key);
                ciphertext = Speck128CipherGadget::new(plaintext, expandedKey).getOutputWires();
                makeOutputArray(ciphertext);
            }

            fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(key[0], BigInteger::new("0706050403020100", 16));
                evaluator.setWireValue(key[1], BigInteger::new("0f0e0d0c0b0a0908", 16));
                evaluator.setWireValue(plaintext[0], BigInteger::new("7469206564616d20", 16));
                evaluator.setWireValue(plaintext[1], BigInteger::new("6c61766975716520", 16));
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let evaluator = generator.getCircuitEvaluator();
        let cipherText = generator.get_out_wires();
        assertEquals(
            evaluator.getWireValue(cipherText.get(0)),
            BigInteger::new("7860fedf5c570d18", 16),
        );
        assertEquals(
            evaluator.getWireValue(cipherText.get(1)),
            BigInteger::new("a65d985179783265", 16),
        );
    }
}
