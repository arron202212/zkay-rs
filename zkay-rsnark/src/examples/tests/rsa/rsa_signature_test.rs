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
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget};
use crate::examples::gadgets::rsa::rsa_sig_verification_v1_5_gadget::RSASigVerificationV1_5_Gadget;
use crate::util::util::BigInteger;
use crate::util::util::Util;
use std::ops::Sub;
use zkay_derive::ImplStructNameConfig;
//Tests RSA PKCS #1, V1.5 Signature

#[cfg(test)]
mod test {
    use super::*;

    /*
     * Note that these tests are for ensuring the basic functionality. To verify
     * that the gadget cannot allow *any* invalid signatures to pass, this requires more than testing few cases, e.g. a
     * careful review of the code  to ensure that there are no
     * missing/incorrect constraints that a cheating prover could make use of.
     */

    #[test]
    pub fn test_valid_signature_different_key_lengths() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputMessage: Vec<Option<WireType>>,
            signature: Option<LongElement>,
            rsaModulus: Option<LongElement>,
            sha2Gadget: Option<Gadget<SHA256Gadget<Base>>>,
            rsaSigVerificationV1_5_Gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
            rsaKeyLength: usize,
        }
        impl CGTest {
            const inputStr: &[u8] = b"abc";
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputMessage = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let sha2Gadget = SHA256Gadget::new(
                    inputMessage.clone(),
                    8,
                    inputMessage.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                );
                let digest = sha2Gadget.getOutputWires().clone();
                let rsaModulus = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let signature = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
                    rsaModulus.clone(),
                    digest.clone(),
                    signature.clone(),
                    self.t.rsaKeyLength as i32,
                    &None,
                    self.cg(),
                );
                self.makeOutput(
                    rsaSigVerificationV1_5_Gadget.getOutputWires()[0]
                        .as_ref()
                        .unwrap(),
                    &None,
                );
                (
                    self.t.rsaModulus,
                    self.t.inputMessage,
                    self.t.signature,
                    self.t.sha2Gadget,
                    self.t.rsaSigVerificationV1_5_Gadget,
                ) = (
                    Some(rsaModulus),
                    inputMessage,
                    Some(signature),
                    Some(sha2Gadget),
                    Some(rsaSigVerificationV1_5_Gadget),
                );
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.inputMessage.len() {
                    evaluator.setWireValuei(
                        self.t.inputMessage[i].as_ref().unwrap(),
                        CGTest::inputStr[i] as i64,
                    );
                }
                // try {
                // let keyGen = KeyPairGenerator.getInstance("RSA");
                // keyGen.initialize(rsaKeyLength, SecureRandom::new());
                // let keyPair = keyGen.generateKeyPair();
                // let signature = Signature.getInstance("SHA256withRSA");
                // signature.initSign(keyPair.getPrivate());

                let message = CGTest::inputStr;
                // signature.update(message);

                let sigBytes = vec![0; 32]; //signature.sign();

                // pad an extra zero byte to avoid having a negative big
                // integer
                let mut signaturePadded = vec![0; sigBytes.len() + 1];
                signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
                signaturePadded[0] = 0;
                let modulus = BigInteger::from(1); //(keyPair.getPublic()).getModulus();
                let sig = BigInteger::parse_bytes(&signaturePadded, 10).unwrap();

                evaluator.setWireValuebi(
                    self.t.rsaModulus.as_ref().unwrap(),
                    &modulus,
                    LongElement::CHUNK_BITWIDTH,
                );
                evaluator.setWireValuebi(
                    self.t.signature.as_ref().unwrap(),
                    &sig,
                    LongElement::CHUNK_BITWIDTH,
                );

                // } catch (Exception e) {
                // 	System.err
                // 			.println("Error while generating sample input for circuit");
                // 	e.printStackTrace();
                // }
            }
        };

        // testing commonly used rsa key lengths in addition to non-power of two
        // ones

        // might need to increase memory heap to run this test on some platforms

        let keySizeArray = vec![1024, 2048, 3072, 4096, 2047, 2049];

        for keySize in keySizeArray {
            let rsaKeyLength = keySize;

            let t = CGTest {
                inputMessage: vec![],
                signature: None,
                rsaModulus: None,
                sha2Gadget: None,
                rsaSigVerificationV1_5_Gadget: None,
                rsaKeyLength,
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("RSA{keySize}_SIG_TestValid"), t);
            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();
            // let evaluator = generator.getCircuitEvaluator();
            assert_eq!(
                Util::one(),
                evaluator.getWireValue(generator.get_out_wires()[0].as_ref().unwrap()),
            );
        }
    }

    #[test]
    pub fn test_invalid_signature_different_key_lengths() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputMessage: Vec<Option<WireType>>,
            signature: Option<LongElement>,
            rsaModulus: Option<LongElement>,
            sha2Gadget: Option<Gadget<SHA256Gadget<Base>>>,
            rsaSigVerificationV1_5_Gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
            rsaKeyLength: usize,
        }
        impl CGTest {
            const inputStr: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputMessage = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let sha2Gadget = SHA256Gadget::new(
                    inputMessage.clone(),
                    8,
                    inputMessage.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                );
                let digest = sha2Gadget.getOutputWires().clone();
                let rsaModulus = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let signature = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
                    rsaModulus.clone(),
                    digest.clone(),
                    signature.clone(),
                    self.t.rsaKeyLength as i32,
                    &None,
                    self.cg(),
                );
                self.makeOutput(
                    &rsaSigVerificationV1_5_Gadget.getOutputWires()[0]
                        .as_ref()
                        .unwrap(),
                    &None,
                );
                (
                    self.t.rsaModulus,
                    self.t.inputMessage,
                    self.t.signature,
                    self.t.sha2Gadget,
                    self.t.rsaSigVerificationV1_5_Gadget,
                ) = (
                    Some(rsaModulus),
                    inputMessage,
                    Some(signature),
                    Some(sha2Gadget),
                    Some(rsaSigVerificationV1_5_Gadget),
                );
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.inputMessage.len() {
                    evaluator.setWireValuei(
                        self.t.inputMessage[i].as_ref().unwrap(),
                        CGTest::inputStr[i] as i64,
                    );
                }
                // try {
                // let keyGen = KeyPairGenerator.getInstance("RSA");
                // keyGen.initialize(rsaKeyLength, SecureRandom::new());
                // let keyPair = keyGen.generateKeyPair();
                // let signature = Signature.getInstance("SHA256withRSA");
                // signature.initSign(keyPair.getPrivate());

                let message = CGTest::inputStr.clone();
                // signature.update(message);

                let sigBytes = vec![0; 32]; //signature.sign();

                // pad an extra zero byte to avoid having a negative big
                // integer
                let mut signaturePadded = vec![0; sigBytes.len() + 1];
                signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
                signaturePadded[0] = 0;
                let modulus = BigInteger::from(64); //(keyPair.getPublic()).getModulus();
                let sig = BigInteger::parse_bytes(&signaturePadded, 10).unwrap();

                evaluator.setWireValuebi(
                    self.t.rsaModulus.as_ref().unwrap(),
                    &modulus,
                    LongElement::CHUNK_BITWIDTH,
                );

                // input the modulus itself instead of the signature
                evaluator.setWireValuebi(
                    self.t.signature.as_ref().unwrap(),
                    &sig.clone().sub(Util::one()),
                    LongElement::CHUNK_BITWIDTH,
                );

                // } catch (Exception e) {
                // 	System.err
                // 			.println("Error while generating sample input for circuit");
                // 	e.printStackTrace();
                // }
            }
        };

        // testing commonly used rsa key lengths in addition to non-power of two
        // ones

        let keySizeArray = vec![1024, 2048, 3072, 4096, 2047, 2049];

        for keySize in keySizeArray {
            let rsaKeyLength = keySize;

            let t = CGTest {
                inputMessage: vec![],
                signature: None,
                rsaModulus: None,
                sha2Gadget: None,
                rsaSigVerificationV1_5_Gadget: None,
                rsaKeyLength,
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("RSA{keySize}_SIG_TestInvalid"), t);
            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();
            // let evaluator = generator.getCircuitEvaluator();
            assert_eq!(
                BigInteger::ZERO,
                evaluator.getWireValue(generator.get_out_wires()[0].as_ref().unwrap()),
            );
        }
    }

    // This test checks the robustness of the code when the chunk bitwidth changes

    #[test]
    pub fn test_valid_signature_different_chunk_bitwidth() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputMessage: Vec<Option<WireType>>,
            signature: Option<LongElement>,
            rsaModulus: Option<LongElement>,
            sha2Gadget: Option<Gadget<SHA256Gadget<Base>>>,
            rsaSigVerificationV1_5_Gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
            rsaKeyLength: usize,
        }
        impl CGTest {
            const inputStr: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputMessage = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let sha2Gadget = SHA256Gadget::new(
                    inputMessage.clone(),
                    8,
                    inputMessage.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                );
                let digest = sha2Gadget.getOutputWires().clone();
                let rsaModulus = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let signature = self.createLongElementInput(self.t.rsaKeyLength as i32, &None);
                let rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
                    rsaModulus.clone(),
                    digest.clone(),
                    signature.clone(),
                    self.t.rsaKeyLength as i32,
                    &None,
                    self.cg(),
                );
                self.makeOutput(
                    rsaSigVerificationV1_5_Gadget.getOutputWires()[0]
                        .as_ref()
                        .unwrap(),
                    &None,
                );
                (
                    self.t.rsaModulus,
                    self.t.inputMessage,
                    self.t.signature,
                    self.t.sha2Gadget,
                    self.t.rsaSigVerificationV1_5_Gadget,
                ) = (
                    Some(rsaModulus),
                    inputMessage,
                    Some(signature),
                    Some(sha2Gadget),
                    Some(rsaSigVerificationV1_5_Gadget),
                );
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.inputMessage.len() {
                    evaluator.setWireValuei(
                        self.t.inputMessage[i].as_ref().unwrap(),
                        CGTest::inputStr[i] as i64,
                    );
                }
                // try {
                // let keyGen = KeyPairGenerator.getInstance("RSA");
                // keyGen.initialize(rsaKeyLength, SecureRandom::new());
                // let keyPair = keyGen.generateKeyPair();
                // let signature = Signature.getInstance("SHA256withRSA");
                // signature.initSign(keyPair.getPrivate());

                let message = CGTest::inputStr.clone();
                // signature.update(message);

                let sigBytes = vec![0; 32]; //signature.sign();

                // pad an extra zero byte to avoid having a negative big
                // integer
                let mut signaturePadded = vec![0; sigBytes.len() + 1];
                signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
                signaturePadded[0] = 0;
                let modulus = BigInteger::from(1); //(keyPair.getPublic()).getModulus();
                let sig = BigInteger::parse_bytes(&signaturePadded, 10).unwrap();

                evaluator.setWireValuebi(
                    self.t.rsaModulus.as_ref().unwrap(),
                    &modulus,
                    LongElement::CHUNK_BITWIDTH,
                );
                evaluator.setWireValuebi(
                    self.t.signature.as_ref().unwrap(),
                    &sig,
                    LongElement::CHUNK_BITWIDTH,
                );

                // } catch (Exception e) {
                // 	System.err
                // 			.println("Error while generating sample input for circuit");
                // 	e.printStackTrace();
                // }
            }
        };
        let keySize = 1024;
        let defaultBitwidth = LongElement::CHUNK_BITWIDTH;

        let chunkBiwidthArray = vec![i32::default(); 106];
        for b in 16..chunkBiwidthArray.len() {
            // LongElement::CHUNK_BITWIDTH = b;
            let rsaKeyLength = keySize;

            let t = CGTest {
                inputMessage: vec![],
                signature: None,
                rsaModulus: None,
                sha2Gadget: None,
                rsaSigVerificationV1_5_Gadget: None,
                rsaKeyLength,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new(
                &format!("RSA{keySize}_SIG_TestValid_ChunkB_{b}"),
                t,
            );
            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();
            // let evaluator = generator.getCircuitEvaluator();
            assert_eq!(
                Util::one(),
                evaluator.getWireValue(generator.get_out_wires()[0].as_ref().unwrap()),
            );

            // LongElement::CHUNK_BITWIDTH = defaultBitwidth; // needed for running all tests together
        }
    }
}
