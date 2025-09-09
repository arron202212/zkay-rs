#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::{self, LongElement},
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            gadget::GadgetConfig,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            },
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::{
        hash::sha256_gadget::{Base, SHA256Gadget},
        rsa::rsa_sig_verification_v1_5_gadget::RSASigVerificationV1_5_Gadget,
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use zkay_derive::ImplStructNameConfig;

//a demo for RSA Signatures PKCS #1, V1.5
crate::impl_struct_name_for!(CircuitGeneratorExtend<RSASigVerCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSASigVerCircuitGenerator {
    pub rsa_key_length: i32,
    pub input_message: Vec<Option<WireType>>,
    pub signature: Option<LongElement>,
    pub rsa_modulus: Option<LongElement>,
    pub sha2_gadget: Option<Gadget<SHA256Gadget<Base>>>,
    pub rsa_sig_verification_v1_5_gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
}
impl RSASigVerCircuitGenerator {
    pub fn new(circuit_name: &str, rsa_key_length: i32) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                rsa_key_length,
                input_message: vec![],
                signature: None,
                rsa_modulus: None,
                sha2_gadget: None,
                rsa_sig_verification_v1_5_gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<RSASigVerCircuitGenerator> {
    fn build_circuit(&mut self) {
        // a sample input message of 3 byte
        let input_message = CircuitGenerator::create_input_wire_array(self.cg(), 3);
        let sha2_gadget = SHA256Gadget::new(
            input_message.clone(),
            8,
            input_message.len(),
            false,
            true,
            self.cg(),
            Base,
        );
        let digest = sha2_gadget.get_output_wires();

        //  * Since an RSA modulus take many wires to present, it could increase
        //  * the size of verification key if we divide it into very small chunks,
        //  * e.g. 32-bits (which happens by default in this version to minimize
        //  * the number of gates later in the circuit). In case the verification
        //  * key size is important, e.g. going to be stored in a smart contract,
        //  * there is a workaround, by first assuming the largest possible
        //  * bitwidths for the chunks, and then converting them into smaller
        //  * chunks. Even better, let the prover provide the key as a witness to
        //  * the circuit, and compute their hash, which will be part of the
        //  * statement. This way of doing this increases the number of gates a
        //  * bit, but reduces the VK size when needed.

        let rsa_modulus =
            CircuitGenerator::create_long_element_input(self.cg(), self.t.rsa_key_length);

        // The modulus can also be hardcoded by changing the statement above to the following

        // rsa_modulus = LongElement::new(Util::split(new
        // BigInteger("f0dac4df56945ec31a037c5b736b64192f14baf27f2036feb85dfe45dc99d8d3c024e226e6fd7cabb56f780f9289c000a873ce32c66f4c1b2970ae6b7a3ceb2d7167fbbfe41f7b0ed7a07e3c32f14c3940176d280ceb25ed0bf830745a9425e1518f27de822b17b2b599e0aea7d72a2a6efe37160e46bf7c78b0573c9014380ab7ec12ce272a83aaa464f814c08a0b0328e191538fefaadd236ae10ba9cbb525df89da59118c7a7b861ec1c05e09976742fc2d08bd806d3715e702d9faa3491a3e4cf76b5546f927e067b281c25ddc1a21b1fb12788d39b27ca0052144ab0aad7410dc316bd7e9d2fe5e0c7a1028102454be9c26c3c347dd93ee044b680c93cb",
        // 16), LongElement::CHUNK_BITWIDTH));

        // In case of hardcoding the modulus, comment the line that sets the modulus value in generate_sample_input() to avoid an exception

        let signature = CircuitGenerator::create_long_element_prover_witness(
            self.cg(),
            self.t.rsa_key_length,
            &None,
        );

        // since the signature is provided as a witness, verify some properties
        // about it
        signature.restrict_bitwidth();
        signature.assert_less_than(&rsa_modulus); // might not be really necessary in that
        // case

        let rsa_sig_verification_v1_5_gadget = RSASigVerificationV1_5_Gadget::new(
            rsa_modulus.clone(),
            digest.clone(),
            signature.clone(),
            self.t.rsa_key_length,
            self.cg(),
        );
        CircuitGenerator::make_output_with_str(
            self.cg(),
            rsa_sig_verification_v1_5_gadget.get_output_wires()[0]
                .as_ref()
                .unwrap(),
            "Is Signature valid?",
        );
        (
            self.t.input_message,
            self.t.signature,
            self.t.rsa_modulus,
            self.t.sha2_gadget,
            self.t.rsa_sig_verification_v1_5_gadget,
        ) = (
            input_message,
            Some(signature),
            Some(rsa_modulus),
            Some(sha2_gadget),
            Some(rsa_sig_verification_v1_5_gadget),
        );
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        let input_str = b"abc";
        for i in 0..self.t.input_message.len() {
            evaluator.set_wire_valuei(
                self.t.input_message[i].as_ref().unwrap(),
                input_str[i] as i64,
            );
        }

        // let keyGen = KeyPairGenerator.getInstance("RSA");
        // keyGen.initialize(rsa_key_length, SecureRandom::new());
        // let keyPair = keyGen.generateKeyPair();

        // let signature = Signature.getInstance("SHA256withRSA");
        // signature.initSign(keyPair.getPrivate());

        let message = input_str;
        // signature.update(message);

        let sig_bytes = vec![0u8; 32]; //signature.sign();
        let mut signature_padded = vec![0; sig_bytes.len() + 1];
        signature_padded[1..sig_bytes.len()].clone_from_slice(&sig_bytes[0..]);
        signature_padded[0] = 0;
        let modulus = BigInteger::from(7); //(keyPair.getPublic()).getModulus();
        //			//println!(modulus.toString(16));
        let sig = BigInteger::from_signed_bytes_be(&signature_padded);

        // if !minimizeVerificationKey {
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
        // } else {
        // evaluator.set_wire_value(self.rsaModulusWires,
        // Util::split(modulus, CONFIGS.log2_field_prime - 1));
        // evaluator.set_wire_value(self.signatureWires,
        // Util::split(sig, CONFIGS.log2_field_prime - 1));
        // }

        // //println!("Error while generating sample input for circuit");
    }
}

pub fn main(args: Vec<String>) {
    let key_length = 2048;
    let mut generator =
        RSASigVerCircuitGenerator::new(&format! {"rsa{key_length}_sha256_sig_verify"}, key_length);
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
