#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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
    examples::gadgets::blockciphers::chaskey_lts128_cipher_gadget::ChaskeyLTS128CipherGadget,
    util::util::{BigInteger, Util},
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
        impl CGTest {
            pub fn new(name: &str) -> CircuitGeneratorExtend<Self> {
                CircuitGeneratorExtend::<Self>::new(
                    name,
                    Self {
                        plaintext: vec![],
                        key: vec![],
                        ciphertext: vec![],
                    },
                )
            }
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let plaintext = CircuitGenerator::create_input_wire_array(self.cg(), 4);
                let key = CircuitGenerator::create_input_wire_array(self.cg(), 4);
                let ciphertext =
                    ChaskeyLTS128CipherGadget::new(plaintext.clone(), key.clone(), self.cg())
                        .get_output_wires()
                        .clone();
                CircuitGenerator::make_output_array(self.cg(), &ciphertext);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                let key_v = [
                    BigInteger::from(0x68e90956u32),
                    BigInteger::from(0x29e3585fu32),
                    BigInteger::from(0x98ecec40u32),
                    BigInteger::from(0x2f9822c5u32),
                ];

                let msg_v = [
                    BigInteger::from(0x262823b8u32),
                    BigInteger::from(0x5e405efdu32),
                    BigInteger::from(0xa901a369u32),
                    BigInteger::from(0xd87aea78u32),
                ];

                for i in 0..self.t.plaintext.len() {
                    evaluator.set_wire_value(self.t.plaintext[i].as_ref().unwrap(), &msg_v[i]);
                }
                for i in 0..self.t.key.len() {
                    evaluator.set_wire_value(self.t.key[i].as_ref().unwrap(), &key_v[i]);
                }
            }
        };
        let mut generator = CGTest::new("Chaskey_Test1");
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let cipher_text = generator.get_out_wires();

        let expeected_cipher_text = [
            BigInteger::from(0x4d8d60d5),
            BigInteger::from(0x7b34bfa2),
            BigInteger::from(0x2f77f8ab),
            BigInteger::from(0x07deeddf),
        ];

        for i in 0..4 {
            assert_eq!(
                evaluator.get_wire_value(cipher_text[i].as_ref().unwrap()),
                expeected_cipher_text[i],
            );
        }
    }
}
