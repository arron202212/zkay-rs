#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
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
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
// use crate::circuit::structure::circuit_generator::{
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
//     get_active_circuit_generator,
// };
// use crate::circuit::structure::wire_array;
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::rsa::rsa_encryption_oaep_gadget::RSAEncryptionOAEPGadget;
use crate::examples::generators::rsa::rsa_util::RSAUtil;
use zkay_derive::ImplStructNameConfig;
crate::impl_struct_name_for!(CircuitGeneratorExtend<RSAEncryptionOAEPCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionOAEPCircuitGenerator {
    pub rsaKeyLength: i32,
    pub plainTextLength: i32,
    pub inputMessage: Vec<Option<WireType>>,
    pub seed: Vec<Option<WireType>>,
    pub cipherText: Vec<Option<WireType>>,
    pub rsaModulus: Option<LongElement>,
    pub rsaEncryptionOAEPGadget: Option<Gadget<RSAEncryptionOAEPGadget>>,
}
impl RSAEncryptionOAEPCircuitGenerator {
    pub fn new(
        circuit_name: &str,
        rsaKeyLength: i32,
        plainTextLength: i32,
    ) -> CircuitGeneratorExtend<Self> {
        // constraints on the plaintext length will be checked by the gadget
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                rsaKeyLength,
                plainTextLength,
                inputMessage: vec![],
                seed: vec![],
                cipherText: vec![],
                rsaModulus: None,
                rsaEncryptionOAEPGadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<RSAEncryptionOAEPCircuitGenerator> {
    fn build_circuit(&mut self) {
        let mut inputMessage = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            self.t.plainTextLength as usize,
            &None,
        ); // in bytes
        for i in 0..self.t.plainTextLength as usize {
            inputMessage[i]
                .as_ref()
                .unwrap()
                .restrict_bit_length(8, &None);
        }

        let seed = CircuitGenerator::create_prover_witness_wire_array(
            self.cg(),
            RSAEncryptionOAEPGadget::SHA256_DIGEST_LENGTH as usize,
            &None,
        );
        // constraints on the seed are checked later.

        let rsaModulus =
            CircuitGenerator::create_long_element_input(self.cg(), self.t.rsaKeyLength, &None);

        // The modulus can also be hardcoded by changing the statement above to the following

        // rsaModulus = LongElement::new(Util::split(new
        // BigInteger("f0dac4df56945ec31a037c5b736b64192f14baf27f2036feb85dfe45dc99d8d3c024e226e6fd7cabb56f780f9289c000a873ce32c66f4c1b2970ae6b7a3ceb2d7167fbbfe41f7b0ed7a07e3c32f14c3940176d280ceb25ed0bf830745a9425e1518f27de822b17b2b599e0aea7d72a2a6efe37160e46bf7c78b0573c9014380ab7ec12ce272a83aaa464f814c08a0b0328e191538fefaadd236ae10ba9cbb525df89da59118c7a7b861ec1c05e09976742fc2d08bd806d3715e702d9faa3491a3e4cf76b5546f927e067b281c25ddc1a21b1fb12788d39b27ca0052144ab0aad7410dc316bd7e9d2fe5e0c7a1028102454be9c26c3c347dd93ee044b680c93cb",
        // 16), LongElement::CHUNK_BITWIDTH));

        // In case of hardcoding, comment the line that sets the modulus value in generate_sample_input()

        let rsaEncryptionOAEPGadget = RSAEncryptionOAEPGadget::new(
            rsaModulus.clone(),
            inputMessage.clone(),
            seed.clone(),
            self.t.rsaKeyLength,
            &None,
            self.cg(),
        );

        // since seed is a witness in this example, verify any needed constraints
        // If the key or the msg are witnesses, similar constraints are needed
        rsaEncryptionOAEPGadget.checkSeedCompliance();

        let cipherTextInBytes = rsaEncryptionOAEPGadget.get_output_wires(); // in bytes

        // do some grouping to reduce VK Size
        let cipherText = WireArray::new(cipherTextInBytes.clone(), self.cg().downgrade())
            .pack_words_into_larger_words(8, 30, &None);
        CircuitGenerator::make_output_array(
            self.cg(),
            &cipherText,
            &Some("Output cipher text".to_owned()),
        );
        (
            self.t.inputMessage,
            self.t.seed,
            self.t.cipherText,
            self.t.rsaModulus,
            self.t.rsaEncryptionOAEPGadget,
        ) = (
            inputMessage,
            seed,
            cipherText,
            Some(rsaModulus),
            Some(rsaEncryptionOAEPGadget),
        );
    }

    fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        let mut msg = String::new();
        for i in 0..self.t.inputMessage.len() {
            evaluator.set_wire_valuei(
                self.t.inputMessage[i].as_ref().unwrap(),
                (b'a' + i as u8) as i64,
            );
            msg.push((b'a' + i as u8) as char);
        }
        //println!("PlainText:{msg}");

        // to make sure that the implementation is working fine,
        // encrypt with the BouncyCastle RSA OAEP encryption in a sample run,
        // extract the seed (after decryption manually), then run the
        // circuit with the extracted seed

        // The BouncyCastle implementation is used at is supports SHA-256 for the MGF, while the native Java implementation uses SHA-1 by default.

        // Security.addProvider(BouncyCastleProvider::new());
        // let cipher = Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");

        // let random = SecureRandom::new();
        // let mut generator = KeyPairGenerator.getInstance("RSA");
        // generator.initialize(rsaKeyLength, random);
        // let pair = generator.generateKeyPair();
        // let pubKey = pair.getPublic();
        let modulus = 7i32; //(pubKey).getModulus();

        evaluator.set_wire_valuebi(
            self.t.rsaModulus.as_ref().unwrap(),
            &BigInteger::from(modulus),
            LongElement::CHUNK_BITWIDTH,
        );

        let privKey = vec![0; 32]; //pair.getPrivate();

        // cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
        let cipherText = vec![1, 2]; //cipher.doFinal(msg.getBytes());
        //			//println!("ciphertext : " + String::new(cipherText));
        let mut cipherTextPadded = vec![0; cipherText.len() + 1];
        cipherTextPadded[1..1 + cipherText.len()].clone_from_slice(&cipherText);
        cipherTextPadded[0] = 0;

        let result = RSAUtil::extractRSAOAEPSeed(&cipherText, &privKey);
        // result[0] contains the plaintext (after decryption)
        // result[1] contains the randomness

        assert_eq!(
            &result[0],
            msg.as_bytes(),
            "Randomness Extraction did not decrypt right"
        );

        let sampleRandomness = result[1].clone();
        for i in 0..sampleRandomness.len() {
            evaluator.set_wire_valuei(
                self.t.seed[i].as_ref().unwrap(),
                (sampleRandomness[i] as i64 + 256) % 256,
            );
        }

        // //println!("Error while generating sample input for circuit");
    }
}

pub fn main(args: Vec<String>) {
    let keyLength = 2048;
    let msgLength = 3;
    let mut generator = RSAEncryptionOAEPCircuitGenerator::new(
        &format!("rsa{keyLength}_oaep_encryption"),
        keyLength,
        msgLength,
    );
    generator.generate_circuit();
    let mut evaluator = generator.eval_circuit().ok();
    generator.prep_files(evaluator);
    generator.run_libsnark();
}
