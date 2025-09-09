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
            wire_type::WireType,
        },
    },
    examples::gadgets::{
        hash::sha256_gadget::{Base, SHA256Gadget},
        rsa::rsa_sig_verification_v1_5_gadget::RSASigVerificationV1_5_Gadget,
    },
    util::util::{BigInteger, Util},
};
use std::ops::Sub;
use zkay_derive::ImplStructNameConfig;
//Tests RSA PKCS #1, V1.5 Signature

#[cfg(test)]
mod test {
    use super::*;

    //Note that these tests are for ensuring the basic functionality. To verify
    //that the gadget cannot allow *any* invalid signatures to pass, this requires more than testing few cases, e.g. a
    //careful review of the code  to ensure that there are no
    //missing/incorrect constraints that a cheating prover could make use of.

    #[test]
    pub fn test_valid_signature_different_key_lengths() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_message: Vec<Option<WireType>>,
            signature: Option<LongElement>,
            rsa_modulus: Option<LongElement>,
            sha2_gadget: Option<Gadget<SHA256Gadget<Base>>>,
            rsa_sig_verification_v1_5_gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
            rsa_key_length: usize,
        }
        impl CGTest {
            const input_str: &[u8] = b"abc";
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_message =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let sha2_gadget = SHA256Gadget::new(
                    input_message.clone(),
                    8,
                    input_message.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                );
                let digest = sha2_gadget.get_output_wires().clone();
                let rsa_modulus = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let signature = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let rsa_sig_verification_v1_5_gadget = RSASigVerificationV1_5_Gadget::new(
                    rsa_modulus.clone(),
                    digest.clone(),
                    signature.clone(),
                    self.t.rsa_key_length as i32,
                    &None,
                    self.cg(),
                );
                CircuitGenerator::make_output(
                    self.cg(),
                    rsa_sig_verification_v1_5_gadget.get_output_wires()[0]
                        .as_ref()
                        .unwrap(),
                );
                (
                    self.t.rsa_modulus,
                    self.t.input_message,
                    self.t.signature,
                    self.t.sha2_gadget,
                    self.t.rsa_sig_verification_v1_5_gadget,
                ) = (
                    Some(rsa_modulus),
                    input_message,
                    Some(signature),
                    Some(sha2_gadget),
                    Some(rsa_sig_verification_v1_5_gadget),
                );
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.input_message.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_message[i].as_ref().unwrap(),
                        CGTest::input_str[i] as i64,
                    );
                }
                // try {
                // let keyGen = KeyPairGenerator.getInstance("RSA");
                // keyGen.initialize(rsa_key_length, SecureRandom::new());
                // let keyPair = keyGen.generateKeyPair();
                // let signature = Signature.getInstance("SHA256withRSA");
                // signature.initSign(keyPair.getPrivate());

                let message = CGTest::input_str;
                // signature.update(message);

                let sig_bytes = vec![0; 32]; //signature.sign();

                // pad an extra zero byte to avoid having a negative big
                // integer
                let mut signature_padded = vec![0; sig_bytes.len() + 1];
                signature_padded[1..sig_bytes.len()].clone_from_slice(&sig_bytes[0..]);
                signature_padded[0] = 0;
                let modulus = BigInteger::from(1); //(keyPair.getPublic()).getModulus();
                let sig = Util::parse_big_int(&signature_padded);

                evaluator.set_wire_valuebi(
                    self.t.rsa_modulus.as_ref().unwrap(),
                    &modulus,
                    LongElement::CHUNK_BITWIDTH,
                );
                evaluator.set_wire_valuebi(
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

        let key_size_array = vec![1024, 2048, 3072, 4096, 2047, 2049];

        for key_size in key_size_array {
            let rsa_key_length = key_size;

            let t = CGTest {
                input_message: vec![],
                signature: None,
                rsa_modulus: None,
                sha2_gadget: None,
                rsa_sig_verification_v1_5_gadget: None,
                rsa_key_length,
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("RSA{key_size}_SIG_TestValid"), t);
            generator.generate_circuit();
            let evaluator = generator.eval_circuit().unwrap();

            assert_eq!(
                Util::one(),
                evaluator.get_wire_value(generator.get_out_wires()[0].as_ref().unwrap()),
            );
        }
    }

    #[test]
    pub fn test_invalid_signature_different_key_lengths() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_message: Vec<Option<WireType>>,
            signature: Option<LongElement>,
            rsa_modulus: Option<LongElement>,
            sha2_gadget: Option<Gadget<SHA256Gadget<Base>>>,
            rsa_sig_verification_v1_5_gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
            rsa_key_length: usize,
        }
        impl CGTest {
            const input_str: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_message =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let sha2_gadget = SHA256Gadget::new(
                    input_message.clone(),
                    8,
                    input_message.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                );
                let digest = sha2_gadget.get_output_wires().clone();
                let rsa_modulus = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let signature = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let rsa_sig_verification_v1_5_gadget = RSASigVerificationV1_5_Gadget::new(
                    rsa_modulus.clone(),
                    digest.clone(),
                    signature.clone(),
                    self.t.rsa_key_length as i32,
                    &None,
                    self.cg(),
                );
                CircuitGenerator::make_output(
                    self.cg(),
                    &rsa_sig_verification_v1_5_gadget.get_output_wires()[0]
                        .as_ref()
                        .unwrap(),
                );
                (
                    self.t.rsa_modulus,
                    self.t.input_message,
                    self.t.signature,
                    self.t.sha2_gadget,
                    self.t.rsa_sig_verification_v1_5_gadget,
                ) = (
                    Some(rsa_modulus),
                    input_message,
                    Some(signature),
                    Some(sha2_gadget),
                    Some(rsa_sig_verification_v1_5_gadget),
                );
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.input_message.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_message[i].as_ref().unwrap(),
                        CGTest::input_str[i] as i64,
                    );
                }
                // try {
                // let keyGen = KeyPairGenerator.getInstance("RSA");
                // keyGen.initialize(rsa_key_length, SecureRandom::new());
                // let keyPair = keyGen.generateKeyPair();
                // let signature = Signature.getInstance("SHA256withRSA");
                // signature.initSign(keyPair.getPrivate());

                let message = CGTest::input_str.clone();
                // signature.update(message);

                let sig_bytes = vec![0; 32]; //signature.sign();

                // pad an extra zero byte to avoid having a negative big
                // integer
                let mut signature_padded = vec![0; sig_bytes.len() + 1];
                signature_padded[1..sig_bytes.len()].clone_from_slice(&sig_bytes[0..]);
                signature_padded[0] = 0;
                let modulus = BigInteger::from(64); //(keyPair.getPublic()).getModulus();
                let sig = Util::parse_big_int(&signature_padded);

                evaluator.set_wire_valuebi(
                    self.t.rsa_modulus.as_ref().unwrap(),
                    &modulus,
                    LongElement::CHUNK_BITWIDTH,
                );

                // input the modulus itself instead of the signature
                evaluator.set_wire_valuebi(
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

        let key_size_array = vec![1024, 2048, 3072, 4096, 2047, 2049];

        for key_size in key_size_array {
            let rsa_key_length = key_size;

            let t = CGTest {
                input_message: vec![],
                signature: None,
                rsa_modulus: None,
                sha2_gadget: None,
                rsa_sig_verification_v1_5_gadget: None,
                rsa_key_length,
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("RSA{key_size}_SIG_TestInvalid"), t);
            generator.generate_circuit();
            let evaluator = generator.eval_circuit().unwrap();

            assert_eq!(
                BigInteger::ZERO,
                evaluator.get_wire_value(generator.get_out_wires()[0].as_ref().unwrap()),
            );
        }
    }

    // This test checks the robustness of the code when the chunk bitwidth changes

    #[test]
    pub fn test_valid_signature_different_chunk_bitwidth() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_message: Vec<Option<WireType>>,
            signature: Option<LongElement>,
            rsa_modulus: Option<LongElement>,
            sha2_gadget: Option<Gadget<SHA256Gadget<Base>>>,
            rsa_sig_verification_v1_5_gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
            rsa_key_length: usize,
        }
        impl CGTest {
            const input_str: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_message =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let sha2_gadget = SHA256Gadget::new(
                    input_message.clone(),
                    8,
                    input_message.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                );
                let digest = sha2_gadget.get_output_wires().clone();
                let rsa_modulus = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let signature = CircuitGenerator::create_long_element_input(
                    self.cg(),
                    self.t.rsa_key_length as i32,
                );
                let rsa_sig_verification_v1_5_gadget = RSASigVerificationV1_5_Gadget::new(
                    rsa_modulus.clone(),
                    digest.clone(),
                    signature.clone(),
                    self.t.rsa_key_length as i32,
                    &None,
                    self.cg(),
                );
                CircuitGenerator::make_output(
                    self.cg(),
                    rsa_sig_verification_v1_5_gadget.get_output_wires()[0]
                        .as_ref()
                        .unwrap(),
                );
                (
                    self.t.rsa_modulus,
                    self.t.input_message,
                    self.t.signature,
                    self.t.sha2_gadget,
                    self.t.rsa_sig_verification_v1_5_gadget,
                ) = (
                    Some(rsa_modulus),
                    input_message,
                    Some(signature),
                    Some(sha2_gadget),
                    Some(rsa_sig_verification_v1_5_gadget),
                );
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.input_message.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_message[i].as_ref().unwrap(),
                        CGTest::input_str[i] as i64,
                    );
                }
                // try {
                // let keyGen = KeyPairGenerator.getInstance("RSA");
                // keyGen.initialize(rsa_key_length, SecureRandom::new());
                // let keyPair = keyGen.generateKeyPair();
                // let signature = Signature.getInstance("SHA256withRSA");
                // signature.initSign(keyPair.getPrivate());

                let message = CGTest::input_str.clone();
                // signature.update(message);

                let sig_bytes = vec![0; 32]; //signature.sign();

                // pad an extra zero byte to avoid having a negative big
                // integer
                let mut signature_padded = vec![0; sig_bytes.len() + 1];
                signature_padded[1..sig_bytes.len()].clone_from_slice(&sig_bytes[0..]);
                signature_padded[0] = 0;
                let modulus = BigInteger::from(1); //(keyPair.getPublic()).getModulus();
                let sig = Util::parse_big_int(&signature_padded);

                evaluator.set_wire_valuebi(
                    self.t.rsa_modulus.as_ref().unwrap(),
                    &modulus,
                    LongElement::CHUNK_BITWIDTH,
                );
                evaluator.set_wire_valuebi(
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
        let key_size = 1024;
        let default_bitwidth = LongElement::CHUNK_BITWIDTH;

        let chunk_biwidth_array = vec![i32::default(); 106];
        for b in 16..chunk_biwidth_array.len() {
            // LongElement::CHUNK_BITWIDTH = b;
            let rsa_key_length = key_size;

            let t = CGTest {
                input_message: vec![],
                signature: None,
                rsa_modulus: None,
                sha2_gadget: None,
                rsa_sig_verification_v1_5_gadget: None,
                rsa_key_length,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new(
                &format!("RSA{key_size}_SIG_TestValid_ChunkB_{b}"),
                t,
            );
            generator.generate_circuit();
            let evaluator = generator.eval_circuit().unwrap();

            assert_eq!(
                Util::one(),
                evaluator.get_wire_value(generator.get_out_wires()[0].as_ref().unwrap()),
            );

            // LongElement::CHUNK_BITWIDTH = default_bitwidth; // needed for running all tests together
        }
    }
}
