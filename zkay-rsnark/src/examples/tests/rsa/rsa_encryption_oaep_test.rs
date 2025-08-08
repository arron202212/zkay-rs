#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget::{Gadget, GadgetConfig};
use crate::circuit::structure::circuit_generator::{
    CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
    addToEvaluationQueue, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::rsa::rsa_encryption_oaep_gadget::RSAEncryptionOAEPGadget;
use crate::examples::generators::rsa::rsa_util::RSAUtil;
use crate::util::util::BigInteger;
use std::ops::{Add, Mul, Shl, Sub};
use zkay_derive::ImplStructNameConfig;
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn oaep_test_encryption_different_key_lengths() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputMessage: Vec<Option<WireType>>,
            seed: Vec<Option<WireType>>,
            cipherText: Vec<Option<WireType>>,
            rsaModulus: Option<LongElement>,
            rsaEncryptionOAEPGadget: Option<Gadget<RSAEncryptionOAEPGadget>>,
            rsaKeyLength: usize,
            rsaModulusValue: BigInteger,
        }
        impl CGTest {
            const plainText: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plainTextLength = CGTest::plainText.len();
                let inputMessage = self.createProverWitnessWireArray(plainTextLength, &None); // in bytes
                for i in 0..plainTextLength {
                    inputMessage[i]
                        .as_ref()
                        .unwrap()
                        .restrictBitLength(8, &None);
                }

                let rsaModulus = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let seed = self.createProverWitnessWireArray(
                    RSAEncryptionOAEPGadget::SHA256_DIGEST_LENGTH as usize,
                    &None,
                );
                let rsaEncryptionOAEPGadget = RSAEncryptionOAEPGadget::new(
                    rsaModulus.clone(),
                    inputMessage.clone(),
                    seed.clone(),
                    self.t.rsaKeyLength as i32,
                    &None,
                    self.cg(),
                );

                // since seed is a witness
                rsaEncryptionOAEPGadget.checkSeedCompliance();

                let cipherTextInBytes = rsaEncryptionOAEPGadget.getOutputWires().clone(); // in bytes

                // group every 8 bytes together
                let cipherText = WireArray::new(cipherTextInBytes, self.cg().downgrade())
                    .packWordsIntoLargerWords(8, 8, &None);
                self.makeOutputArray(&cipherText, &Some("Output cipher text".to_owned()));
                (
                    self.t.rsaModulus,
                    self.t.inputMessage,
                    self.t.seed,
                    self.t.cipherText,
                    self.t.rsaEncryptionOAEPGadget,
                ) = (
                    Some(rsaModulus),
                    inputMessage,
                    seed,
                    cipherText,
                    Some(rsaEncryptionOAEPGadget),
                );
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.inputMessage.len() {
                    evaluator.setWireValuei(
                        self.t.inputMessage[i].as_ref().unwrap(),
                        CGTest::plainText[i] as i64,
                    );
                }
                // try {

                // Security.addProvider(BouncyCastleProvider::new());
                let cipher = vec![0; 32]; //Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");

                evaluator.setWireValuebi(
                    self.t.rsaModulus.as_ref().unwrap(),
                    &self.t.rsaModulusValue,
                    LongElement::CHUNK_BITWIDTH,
                );

                let privKey = vec![0; 32]; //keyPair.getPrivate();

                // cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
                let tmp = vec![0; 32]; //cipher.doFinal(CGTest::plainText);
                let mut cipherTextBytes = vec![0; self.t.rsaKeyLength / 8];
                cipherTextBytes[0..self.t.rsaKeyLength / 8].clone_from_slice(&tmp[0..]);

                let mut cipherTextPadded = vec![0; cipherTextBytes.len() + 1];
                cipherTextPadded[1..].clone_from_slice(&cipherTextBytes);

                cipherTextPadded[0] = 0;

                let result = RSAUtil::extractRSAOAEPSeed(&cipherTextBytes, &privKey);

                assert_eq!(
                    result[0],
                    CGTest::plainText,
                    "Randomness Extraction did not decrypt right"
                );

                let sampleRandomness = &result[1];
                for i in 0..sampleRandomness.len() {
                    evaluator.setWireValuei(
                        self.t.seed[i].as_ref().unwrap(),
                        (sampleRandomness[i] as i64 + 256) % 256,
                    );
                }

                // } catch (Exception e) {
                // 	System.err
                // 			.println("Error while generating sample input for circuit");
                // 	e.printStackTrace();
                // }
            }
        }

        // testing commonly used rsa key lengths

        // might need to increase memory heap to run this test on some platforms

        let keySizeArray = vec![1024, 2048, 3072];

        for keySize in keySizeArray {
            let cipherTextBytes = vec![0; keySize / 8];

            // let random = SecureRandom::new();
            // let keyGen = KeyPairGenerator.getInstance("RSA");
            // keyGen.initialize(keySize, random);
            // let keyPair = keyGen.generateKeyPair();
            // let pubKey = keyPair.getPublic();
            let rsaModulusValue = BigInteger::from(64); //(pubKey).getModulus();

            let rsaKeyLength = keySize;
            let plainTextLength = CGTest::plainText.len();
            let t = CGTest {
                inputMessage: vec![],
                seed: vec![],
                cipherText: vec![],
                rsaModulus: None,
                rsaEncryptionOAEPGadget: None,
                rsaKeyLength,
                rsaModulusValue,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new(
                &format!("RSA{keySize}_OAEP_Enc_TestEncryption"),
                t,
            );

            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();

            // retrieve the ciphertext from the circuit, and verify that it
            // matches the expected ciphertext and that it decrypts correctly
            // (using the BouncyCastle RSA decryptor)
            let mut cipherTextList = generator.get_out_wires();
            let mut t = BigInteger::ZERO;
            let mut i = 0;
            for w in cipherTextList {
                let val = evaluator.getWireValue(w.as_ref().unwrap());
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
                assert_eq!(&cipherTextBytes[k], &cipherTextBytesFromCircuit[k]);
            }

            let cipher = vec![0; 32]; //Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");
            // cipher.init(Cipher.DECRYPT_MODE, keyPair.getPrivate());
            let cipherTextDecrypted = vec![0; 32]; //cipher.doFinal(cipherTextBytesFromCircuit);
            assert_eq!(CGTest::plainText, cipherTextDecrypted);
        }
    }
}
