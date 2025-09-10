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
        auxiliary::long_element::LongElement,
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::{
        gadgets::rsa::{
            rsa_encryption_oaep_gadget::RSAEncryptionOAEPGadget,
            rsa_encryption_v1_5_gadget::RSAEncryptionV1_5_Gadget,
        },
        generators::rsa::rsa_util::RSAUtil,
    },
    util::util::{BigInteger, Util},
};

use std::ops::{Add, Mul, Shl, Sub};
use zkay_derive::ImplStructNameConfig;
// Tests RSA PKCS #1, V1.5
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_encryption_different_key_lengths() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_message: Vec<Option<WireType>>,
            randomness: Vec<Option<WireType>>,
            cipher_text: Vec<Option<WireType>>,
            rsa_modulus: Option<LongElement>,
            rsa_encryption_v1_5_gadget: Option<Gadget<RSAEncryptionV1_5_Gadget>>,
            rsa_key_length: i64,
            rsa_modulus_value: BigInteger,
        }
        impl CGTest {
            const plain_text: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let plain_text_length = CGTest::plain_text.len();
                let mut input_message = CircuitGenerator::create_prover_witness_wire_array(
                    self.cg(),
                    plain_text_length,
                ); // in bytes
                for i in 0..plain_text_length {
                    input_message[i].as_ref().unwrap().restrict_bit_length(8);
                }

                let mut rsa_modulus = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let mut randomness = CircuitGenerator::create_prover_witness_wire_array(
                    self.cg(),
                    Gadget::<RSAEncryptionV1_5_Gadget>::get_expected_randomness_length(
                        self.t.rsa_key_length as i32,
                        plain_text_length as i32,
                    ) as usize,
                );
                let mut rsa_encryption_v1_5_gadget = RSAEncryptionV1_5_Gadget::new(
                    rsa_modulus.clone(),
                    input_message.clone(),
                    randomness.clone(),
                    self.t.rsa_key_length as i32,
                    self.cg(),
                );

                // since randomness is a witness
                rsa_encryption_v1_5_gadget.check_randomness_compliance();
                let cipher_text_in_bytes = rsa_encryption_v1_5_gadget.get_output_wires().clone(); // in bytes

                // group every 8 bytes together
                let mut cipher_text = WireArray::new(cipher_text_in_bytes, self.cg().downgrade())
                    .pack_words_into_larger_words(8, 8);
                CircuitGenerator::make_output_array_with_str(
                    self.cg(),
                    &cipher_text,
                    "Output cipher text",
                );
                (
                    self.t.rsa_modulus,
                    self.t.input_message,
                    self.t.randomness,
                    self.t.cipher_text,
                    self.t.rsa_encryption_v1_5_gadget,
                ) = (
                    Some(rsa_modulus),
                    input_message,
                    randomness,
                    cipher_text,
                    Some(rsa_encryption_v1_5_gadget),
                );
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.input_message.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_message[i].as_ref().unwrap(),
                        CGTest::plain_text[i] as i64,
                    );
                }
                // let cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
                evaluator.set_wire_valuebi(
                    self.t.rsa_modulus.as_ref().unwrap(),
                    &self.t.rsa_modulus_value,
                    LongElement::CHUNK_BITWIDTH,
                );

                let priv_key = vec![0; 32]; //keyPair.getPrivate();

                // cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
                let tmp = vec![0; 32]; //cipher.do_final(CGTest::plain_text);
                let mut cipher_text_bytes = vec![0; self.t.rsa_key_length as usize / 8];
                cipher_text_bytes[0..self.t.rsa_key_length as usize / 8]
                    .clone_from_slice(&tmp[0..]);

                let mut cipher_text_padded = vec![0; cipher_text_bytes.len() + 1];

                cipher_text_padded[1..].clone_from_slice(&cipher_text_bytes);

                cipher_text_padded[0] = 0;

                let result = RSAUtil::extract_rsa_randomness1_5(&cipher_text_bytes, &priv_key);

                assert_eq!(
                    &result[0],
                    CGTest::plain_text,
                    "Randomness Extraction did not decrypt right"
                );

                let sample_randomness = &result[1];
                for i in 0..sample_randomness.len() {
                    evaluator.set_wire_valuei(
                        self.t.randomness[i].as_ref().unwrap(),
                        (sample_randomness[i] as i64 + 256) % 256,
                    );
                }

                // } catch (Exception e) {
                // 	System.err
                // 			.println("Error while generating sample input for circuit");
                // 	e.printStackTrace();
                // }
            }
        };

        // testing commonly used rsa key lengths

        // might need to increase memory heap to run this test on some platforms

        let key_size_array = vec![1024, 2048, 3072, 4096];

        for key_size in key_size_array {
            let cipher_text_bytes = vec![0; key_size / 8];
            // let random = SecureRandom::new();
            // let keyGen = KeyPairGenerator.getInstance("RSA");
            // keyGen.initialize(key_size, random);
            // let keyPair = keyGen.generateKeyPair();
            // let pubKey = keyPair.getPublic();
            let rsa_modulus_value = BigInteger::from(64); // (pubKey).getModulus();

            let rsa_key_length = key_size as i64;
            let plain_text_length = CGTest::plain_text.len();

            let t = CGTest {
                input_message: vec![],
                randomness: vec![],
                cipher_text: vec![],
                rsa_modulus: None,
                rsa_encryption_v1_5_gadget: None,
                rsa_modulus_value,
                rsa_key_length,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new(
                &format!("RSA{key_size}_Enc_TestEncryption"),
                t,
            );
            generator.generate_circuit();
            let evaluator = generator.eval_circuit().unwrap();

            // retrieve the ciphertext from the circuit, and verify that it matches the expected ciphertext and that it decrypts correctly (using the Java built-in RSA decryptor)
            let cipher_text_list = generator.get_out_wires();
            let mut t = BigInteger::ZERO;
            let mut i = 0;
            for w in &cipher_text_list {
                let val = evaluator.get_wire_value(w.as_ref().unwrap());
                t = t.add(val.shl(i * 64));
                i += 1;
            }

            // extract the bytes
            let (_, mut cipher_text_bytes_from_circuit) = t.to_bytes_be();

            // ignore the sign byte if any was added
            if t.bits() == key_size as u64
                && cipher_text_bytes_from_circuit.len() == key_size / 8 + 1
            {
                cipher_text_bytes_from_circuit = cipher_text_bytes_from_circuit[1..].to_vec();
            }

            for k in 0..cipher_text_bytes_from_circuit.len() {
                assert_eq!(cipher_text_bytes[k], cipher_text_bytes_from_circuit[k]);
            }

            // let cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
            // cipher.init(Cipher.DECRYPT_MODE, keyPair.getPrivate());
            let cipher_text_decrypted = vec![0; 32]; //cipher.do_final(cipher_text_bytes_from_circuit);
            assert_eq!(CGTest::plain_text, cipher_text_decrypted);
        }
    }
}
