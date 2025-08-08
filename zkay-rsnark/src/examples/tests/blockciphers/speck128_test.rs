#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::{Gadget, GadgetConfig};
use crate::circuit::structure::circuit_generator::{
    CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
    addToEvaluationQueue, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::blockciphers::speck128_cipher_gadget::Speck128CipherGadget;
use crate::util::util::BigInteger;
use zkay_derive::ImplStructNameConfig;
/**
 * Tests SPECK block cipher @ keysize = 128, blocksize = 128.
 * Test vector obtained from:  https://github.com/inmcm/Simon_Speck_Ciphers/blob/master/Python/SimonSpeckCiphers/tests/test_simonspeck.py
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn speck128_test_case1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>, // 2 64-bit words
            key: Vec<Option<WireType>>,       // 2 64-bit words
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plaintext = self.createInputWireArray(2, &None);
                let key = self.createInputWireArray(2, &None);
                let expandedKey = Gadget::<Speck128CipherGadget>::expandKey(&key, &self.cg);
                let ciphertext =
                    Speck128CipherGadget::new(plaintext.clone(), expandedKey, &None, self.cg())
                        .getOutputWires()
                        .clone();
                self.makeOutputArray(&ciphertext, &None);
                (self.t.plaintext, self.t.key) = (plaintext, key);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(
                    self.t.key[0].as_ref().unwrap(),
                    BigInteger::parse_bytes(b"0706050403020100", 16)
                        .as_ref()
                        .unwrap(),
                );
                evaluator.setWireValue(
                    self.t.key[1].as_ref().unwrap(),
                    BigInteger::parse_bytes(b"0f0e0d0c0b0a0908", 16)
                        .as_ref()
                        .unwrap(),
                );
                evaluator.setWireValue(
                    self.t.plaintext[0].as_ref().unwrap(),
                    BigInteger::parse_bytes(b"7469206564616d20", 16)
                        .as_ref()
                        .unwrap(),
                );
                evaluator.setWireValue(
                    self.t.plaintext[1].as_ref().unwrap(),
                    BigInteger::parse_bytes(b"6c61766975716520", 16)
                        .as_ref()
                        .unwrap(),
                );
            }
        };
        let t = CGTest {
            plaintext: vec![], // 2 64-bit words
            key: vec![],       // 2 64-bit words
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Speck128_Test", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();

        let cipherText = generator.get_out_wires();
        assert_eq!(
            evaluator.getWireValue(cipherText[0].as_ref().unwrap()),
            BigInteger::parse_bytes(b"7860fedf5c570d18", 16).unwrap(),
        );
        assert_eq!(
            evaluator.getWireValue(cipherText[1].as_ref().unwrap()),
            BigInteger::parse_bytes(b"a65d985179783265", 16).unwrap(),
        );
    }
}
