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
    examples::{
        gadgets::rsa::rsa_encryption_v1_5_gadget::RSAEncryptionV1_5_Gadget,
        generators::rsa::rsa_util::RSAUtil,
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use zkay_derive::ImplStructNameConfig;

// a demo for RSA Encryption PKCS #1, V1.5
crate::impl_struct_name_for!(CircuitGeneratorExtend<RSAEncryptionCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionCircuitGenerator {
    pub rsa_key_length: i32,
    pub plain_text_length: i32,
    pub input_message: Vec<Option<WireType>>,
    pub randomness: Vec<Option<WireType>>,
    pub cipher_text: Vec<Option<WireType>>,
    pub rsa_modulus: Option<LongElement>,
    pub rsa_encryption_v1_5_gadget: Option<Gadget<RSAEncryptionV1_5_Gadget>>,
}
impl RSAEncryptionCircuitGenerator {
    pub fn new(
        circuit_name: &str,
        rsa_key_length: i32,
        plain_text_length: i32,
    ) -> CircuitGeneratorExtend<Self> {
        // constraints on the plaintext length will be checked by the gadget
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                rsa_key_length,
                plain_text_length,
                input_message: vec![],
                randomness: vec![],
                cipher_text: vec![],
                rsa_modulus: None,
                rsa_encryption_v1_5_gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<RSAEncryptionCircuitGenerator> {
    fn build_circuit(&mut self) {
        let (rsa_key_length, plain_text_length) =
            (self.t.rsa_key_length, self.t.plain_text_length as usize);
        let mut input_message =
            CircuitGenerator::create_prover_witness_wire_array(self.cg(), plain_text_length, &None); // in bytes
        for i in 0..plain_text_length {
            input_message[i]
                .as_ref()
                .unwrap()
                .restrict_bit_length(8, &None);
        }

        let randomness = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            Gadget::<RSAEncryptionV1_5_Gadget>::get_expected_randomness_length(
                rsa_key_length,
                plain_text_length as i32,
            ) as usize,
            &None,
        );
        // constraints on the randomness vector are checked later.

        //  * Since an RSA modulus take many wires to present, it could increase
        //  * the size of verification key if we divide it into very small chunks,
        //  * e.g. 32-bits (which happens by default in this version to minimize
        //  * the number of gates later in the circuit). In case the verification
        //  * key size is important, e.g. going to be stored in a smart contract, a
        //  * possible workaround could be by either assuming the largest possible
        //  * bitwidths for the chunks, and then converting them into smaller
        //  * chunks, or let the prover provide the key as a witness to the
        //  * circuit, and compute its hash, which will be part of the statement.
        //  * This way of doing this increases the number of gates a bit, but
        //  * reduces the VK size when crucial.

        let rsa_modulus =
            CircuitGenerator::create_long_element_input(self.cg(), rsa_key_length, &None);

        // The modulus can also be hardcoded by changing the statement above to the following

        // rsa_modulus = LongElement::new(Util::split(new
        // BigInteger("f0dac4df56945ec31a037c5b736b64192f14baf27f2036feb85dfe45dc99d8d3c024e226e6fd7cabb56f780f9289c000a873ce32c66f4c1b2970ae6b7a3ceb2d7167fbbfe41f7b0ed7a07e3c32f14c3940176d280ceb25ed0bf830745a9425e1518f27de822b17b2b599e0aea7d72a2a6efe37160e46bf7c78b0573c9014380ab7ec12ce272a83aaa464f814c08a0b0328e191538fefaadd236ae10ba9cbb525df89da59118c7a7b861ec1c05e09976742fc2d08bd806d3715e702d9faa3491a3e4cf76b5546f927e067b281c25ddc1a21b1fb12788d39b27ca0052144ab0aad7410dc316bd7e9d2fe5e0c7a1028102454be9c26c3c347dd93ee044b680c93cb",
        // 16), LongElement::CHUNK_BITWIDTH));

        // In case of hardcoding the modulus, comment the line that sets the modulus value in generate_sample_input() to avoid an exception

        let rsa_encryption_v1_5_gadget = RSAEncryptionV1_5_Gadget::new(
            rsa_modulus.clone(),
            input_message.clone(),
            randomness.clone(),
            rsa_key_length,
            &None,
            self.cg(),
        );

        // since the randomness vector is a witness in this example, verify any needed constraints
        rsa_encryption_v1_5_gadget.check_randomness_compliance();

        let cipher_text_in_bytes = rsa_encryption_v1_5_gadget.get_output_wires(); // in bytes

        // do some grouping to reduce VK Size
        let cipher_text = WireArray::new(cipher_text_in_bytes.clone(), self.cg().downgrade())
            .pack_words_into_larger_words(8, 30, &None);
        CircuitGenerator::make_output_array(
            self.cg(),
            &cipher_text,
            &Some("Output cipher text".to_owned()),
        );
        (
            self.t.input_message,
            self.t.randomness,
            self.t.cipher_text,
            self.t.rsa_modulus,
            self.t.rsa_encryption_v1_5_gadget,
        ) = (
            input_message,
            randomness,
            cipher_text,
            Some(rsa_modulus),
            Some(rsa_encryption_v1_5_gadget),
        );
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        let mut msg = String::new();
        for i in 0..self.t.input_message.len() {
            evaluator.set_wire_valuei(
                self.t.input_message[i].as_ref().unwrap(),
                (b'a' + i as u8) as i64,
            );
            msg.push((b'a' + i as u8) as char);
        }
        //println!("PlainText: {msg}");

        // to make sure that the implementation is working fine,
        // encrypt with the underlying java implementation for RSA
        // Encryption in a sample run,
        // extract the randomness (after decryption manually), then run the
        // circuit with the extracted randomness

        // let random = SecureRandom::new();
        // let mut generator = KeyPairGenerator.getInstance("RSA");
        // generator.initialize(rsa_key_length, random);
        // let pair = generator.generateKeyPair();
        // let pubKey = pair.getPublic();
        let modulus = 1i32; //(pubKey).getModulus();

        // let cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
        evaluator.set_wire_valuebi(
            self.t.rsa_modulus.as_ref().unwrap(),
            &BigInteger::from(modulus),
            LongElement::CHUNK_BITWIDTH,
        );

        let priv_key = vec![0; 10]; //pair.getPrivate();

        // cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
        let cipher_text = vec![1, 2]; // cipher.do_final(msg.getBytes());
        //			//println!("ciphertext : " + String::new(cipher_text));
        let mut cipher_text_padded = vec![0; cipher_text.len() + 1];
        cipher_text_padded[1..cipher_text.len()].clone_from_slice(&cipher_text);
        cipher_text_padded[0] = 0;

        let result = RSAUtil::extract_rsa_randomness1_5(&cipher_text, &priv_key);
        // result[0] contains the plaintext (after decryption)
        // result[1] contains the randomness

        assert_eq!(
            &result[0],
            msg.as_bytes(),
            "Randomness Extraction did not decrypt right"
        );

        let sample_randomness = result[1].clone();
        for i in 0..sample_randomness.len() {
            evaluator.set_wire_valuei(
                self.t.randomness[i].as_ref().unwrap(),
                (sample_randomness[i] as i64 + 256) % 256,
            );
        }

        // //println!("Error while generating sample input for circuit");
    }
}

pub fn main(args: Vec<String>) {
    let key_length = 2048;
    let msg_length = 3;
    let mut generator = RSAEncryptionCircuitGenerator::new(
        &format!("rsa{key_length}_encryption"),
        key_length,
        msg_length,
    );
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
