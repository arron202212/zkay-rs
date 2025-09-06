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
        gadgets::rsa::rsa_encryption_oaep_gadget::RSAEncryptionOAEPGadget,
        generators::rsa::rsa_util::RSAUtil,
    },
    util::util::BigInteger,
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
            inputMessage: Vec<Option<WireType>>,
            randomness: Vec<Option<WireType>>,
            cipherText: Vec<Option<WireType>>,
            rsaModulus: Option<LongElement>,
            rsaEncryptionV1_5_Gadget: Option<Gadget<RSAEncryptionV1_5_Gadget>>,
            rsaKeyLength: i64,
            rsaModulusValue: BigInteger,
        }
        impl CGTest {
            const plainText: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let plainTextLength = CGTest::plainText.len();
                let mut inputMessage = CircuitGenerator::create_prover_witness_wire_array(
                    self.cg(),
                    plainTextLength,
                    &None,
                ); // in bytes
                for i in 0..plainTextLength {
                    inputMessage[i]
                        .as_ref()
                        .unwrap()
                        .restrict_bit_length(8, &None);
                }

                let mut rsaModulus = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsaKeyLength as i32,
                    &None,
                );
                let mut randomness = CircuitGenerator::create_prover_witness_wire_array(
                    self.cg(),
                    Gadget::<RSAEncryptionV1_5_Gadget>::getExpectedRandomnessLength(
                        self.t.rsaKeyLength as i32,
                        plainTextLength as i32,
                    ) as usize,
                    &None,
                );
                let mut rsaEncryptionV1_5_Gadget = RSAEncryptionV1_5_Gadget::new(
                    rsaModulus.clone(),
                    inputMessage.clone(),
                    randomness.clone(),
                    self.t.rsaKeyLength as i32,
                    &None,
                    self.cg(),
                );

                // since randomness is a witness
                rsaEncryptionV1_5_Gadget.checkRandomnessCompliance();
                let cipherTextInBytes = rsaEncryptionV1_5_Gadget.get_output_wires().clone(); // in bytes

                // group every 8 bytes together
                let mut cipherText = WireArray::new(cipherTextInBytes, self.cg().downgrade())
                    .pack_words_into_larger_words(8, 8, &None);
                CircuitGenerator::make_output_array(
                    self.cg(),
                    &cipherText,
                    &Some("Output cipher text".to_owned()),
                );
                (
                    self.t.rsaModulus,
                    self.t.inputMessage,
                    self.t.randomness,
                    self.t.cipherText,
                    self.t.rsaEncryptionV1_5_Gadget,
                ) = (
                    Some(rsaModulus),
                    inputMessage,
                    randomness,
                    cipherText,
                    Some(rsaEncryptionV1_5_Gadget),
                );
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.inputMessage.len() {
                    evaluator.set_wire_valuei(
                        self.t.inputMessage[i].as_ref().unwrap(),
                        CGTest::plainText[i] as i64,
                    );
                }
                // try {
                // let cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
                evaluator.set_wire_valuebi(
                    self.t.rsaModulus.as_ref().unwrap(),
                    &self.t.rsaModulusValue,
                    LongElement::CHUNK_BITWIDTH,
                );

                let privKey = vec![0; 32]; //keyPair.getPrivate();

                // cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
                let tmp = vec![0; 32]; //cipher.doFinal(CGTest::plainText);
                let mut cipherTextBytes = vec![0; self.t.rsaKeyLength as usize / 8];
                cipherTextBytes[0..self.t.rsaKeyLength as usize / 8].clone_from_slice(&tmp[0..]);

                let mut cipherTextPadded = vec![0; cipherTextBytes.len() + 1];

                cipherTextPadded[1..].clone_from_slice(&cipherTextBytes);

                cipherTextPadded[0] = 0;

                let result = RSAUtil::extractRSARandomness1_5(&cipherTextBytes, &privKey);

                assert_eq!(
                    &result[0],
                    CGTest::plainText,
                    "Randomness Extraction did not decrypt right"
                );

                let sampleRandomness = &result[1];
                for i in 0..sampleRandomness.len() {
                    evaluator.set_wire_valuei(
                        self.t.randomness[i].as_ref().unwrap(),
                        (sampleRandomness[i] as i64 + 256) % 256,
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

        let keySizeArray = vec![1024, 2048, 3072, 4096];

        for keySize in keySizeArray {
            let cipherTextBytes = vec![0; keySize / 8];
            // let random = SecureRandom::new();
            // let keyGen = KeyPairGenerator.getInstance("RSA");
            // keyGen.initialize(keySize, random);
            // let keyPair = keyGen.generateKeyPair();
            // let pubKey = keyPair.getPublic();
            let rsaModulusValue = BigInteger::from(64); // (pubKey).getModulus();

            let rsaKeyLength = keySize as i64;
            let plainTextLength = CGTest::plainText.len();

            let t = CGTest {
                inputMessage: vec![],
                randomness: vec![],
                cipherText: vec![],
                rsaModulus: None,
                rsaEncryptionV1_5_Gadget: None,
                rsaModulusValue,
                rsaKeyLength,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new(
                &format!("RSA{keySize}_Enc_TestEncryption"),
                t,
            );
            generator.generate_circuit();
            let evaluator = generator.eval_circuit().unwrap();

            // retrieve the ciphertext from the circuit, and verify that it matches the expected ciphertext and that it decrypts correctly (using the Java built-in RSA decryptor)
            let cipherTextList = generator.get_out_wires();
            let mut t = BigInteger::ZERO;
            let mut i = 0;
            for w in &cipherTextList {
                let val = evaluator.get_wire_value(w.as_ref().unwrap());
                t = t.add(val.shl(i * 64));
                i += 1;
            }

            // extract the bytes
            let (_, mut cipherTextBytesFromCircuit) = t.to_bytes_be();

            // ignore the sign byte if any was added
            if t.bits() == keySize as u64 && cipherTextBytesFromCircuit.len() == keySize / 8 + 1 {
                cipherTextBytesFromCircuit = cipherTextBytesFromCircuit[1..].to_vec();
            }

            for k in 0..cipherTextBytesFromCircuit.len() {
                assert_eq!(cipherTextBytes[k], cipherTextBytesFromCircuit[k]);
            }

            // let cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
            // cipher.init(Cipher.DECRYPT_MODE, keyPair.getPrivate());
            let cipherTextDecrypted = vec![0; 32]; //cipher.doFinal(cipherTextBytesFromCircuit);
            assert_eq!(CGTest::plainText, cipherTextDecrypted);
        }
    }
}
