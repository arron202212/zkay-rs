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
        structure::{circuit_generator::{
            CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            addToEvaluationQueue, getActiveCircuitGenerator,
        },
        wire_type::WireType},
    },
    examples::gadgets::blockciphers::chaskey_lts128_cipher_gadget::ChaskeyLTS128CipherGadget,
    util::util::BigInteger,
};

use zkay_derive::ImplStructNameConfig;
// test case from:  https://www.cryptolux.org/index.php/FELICS

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn chaskey128_test_case1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 4 32-bit words
            key: Vec<Option<WireType>>,        // 4 32-bit words
            ciphertext: Vec<Option<WireType>>, // 4 32-bit words
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plaintext = CircuitGenerator::createInputWireArray(self.cg(), 4, &None);
                let key = CircuitGenerator::createInputWireArray(self.cg(), 4, &None);
                let ciphertext = ChaskeyLTS128CipherGadget::new(
                    plaintext.clone(),
                    key.clone(),
                    &None,
                    self.cg(),
                )
                .getOutputWires()
                .clone();
                CircuitGenerator::makeOutputArray(self.cg(), &ciphertext, &None);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let keyV = [
                    BigInteger::from(0x68e90956u32),
                    BigInteger::from(0x29e3585fu32),
                    BigInteger::from(0x98ecec40u32),
                    BigInteger::from(0x2f9822c5u32),
                ];

                let msgV = [
                    BigInteger::from(0x262823b8u32),
                    BigInteger::from(0x5e405efdu32),
                    BigInteger::from(0xa901a369u32),
                    BigInteger::from(0xd87aea78u32),
                ];

                for i in 0..self.t.plaintext.len() {
                    evaluator.setWireValue(self.t.plaintext[i].as_ref().unwrap(), &msgV[i]);
                }
                for i in 0..self.t.key.len() {
                    evaluator.setWireValue(self.t.key[i].as_ref().unwrap(), &keyV[i]);
                }
            }
        };
        let t = CGTest {
            plaintext: vec![],  // 4 32-bit words
            key: vec![],        // 4 32-bit words
            ciphertext: vec![], // 4 32-bit words
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Chaskey_Test1", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();

        let cipherText = generator.get_out_wires();

        let expeectedCiphertext = [
            BigInteger::from(0x4d8d60d5),
            BigInteger::from(0x7b34bfa2),
            BigInteger::from(0x2f77f8ab),
            BigInteger::from(0x07deeddf),
        ];

        for i in 0..4 {
            assert_eq!(
                evaluator.getWireValue(cipherText[i].as_ref().unwrap()),
                expeectedCiphertext[i],
            );
        }
    }
}
