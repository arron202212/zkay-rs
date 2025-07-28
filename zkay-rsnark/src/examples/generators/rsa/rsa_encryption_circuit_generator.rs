#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
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
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            },
            constant_wire::{ConstantWire, new_constant},
            variable_bit_wire::VariableBitWire,
            variable_wire::{VariableWire, new_variable},
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
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
//     CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
//     getActiveCircuitGenerator,
// };
// use crate::circuit::structure::wire_array;
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::rsa::rsa_encryption_v1_5_gadget::RSAEncryptionV1_5_Gadget;
use crate::examples::generators::rsa::rsa_util::RSAUtil;
use zkay_derive::ImplStructNameConfig;
// a demo for RSA Encryption PKCS #1, V1.5
crate::impl_struct_name_for!(CircuitGeneratorExtend<RSAEncryptionCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSAEncryptionCircuitGenerator {
    rsaKeyLength: i32,
    plainTextLength: i32,
    inputMessage: Vec<Option<WireType>>,
    randomness: Vec<Option<WireType>>,
    cipherText: Vec<Option<WireType>>,
    rsaModulus: Option<LongElement>,

    rsaEncryptionV1_5_Gadget: Option<Gadget<RSAEncryptionV1_5_Gadget>>,
}
impl RSAEncryptionCircuitGenerator {
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
                randomness: vec![],
                cipherText: vec![],
                rsaModulus: None,
                rsaEncryptionV1_5_Gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<RSAEncryptionCircuitGenerator> {
    fn buildCircuit(&mut self) {
        let (rsaKeyLength, plainTextLength) =
            (self.t.rsaKeyLength, self.t.plainTextLength as usize);
        let mut inputMessage = self.createProverWitnessWireArray(plainTextLength, &None); // in bytes
        for i in 0..plainTextLength {
            inputMessage[i]
                .as_ref()
                .unwrap()
                .restrictBitLength(8, &None);
        }

        let randomness = self.createProverWitnessWireArray(
            Gadget::<RSAEncryptionV1_5_Gadget>::getExpectedRandomnessLength(
                rsaKeyLength,
                plainTextLength as i32,
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

        let rsaModulus = self.createLongElementInput(rsaKeyLength, &None);

        // The modulus can also be hardcoded by changing the statement above to the following

        // rsaModulus = LongElement::new(Util::split(new
        // BigInteger("f0dac4df56945ec31a037c5b736b64192f14baf27f2036feb85dfe45dc99d8d3c024e226e6fd7cabb56f780f9289c000a873ce32c66f4c1b2970ae6b7a3ceb2d7167fbbfe41f7b0ed7a07e3c32f14c3940176d280ceb25ed0bf830745a9425e1518f27de822b17b2b599e0aea7d72a2a6efe37160e46bf7c78b0573c9014380ab7ec12ce272a83aaa464f814c08a0b0328e191538fefaadd236ae10ba9cbb525df89da59118c7a7b861ec1c05e09976742fc2d08bd806d3715e702d9faa3491a3e4cf76b5546f927e067b281c25ddc1a21b1fb12788d39b27ca0052144ab0aad7410dc316bd7e9d2fe5e0c7a1028102454be9c26c3c347dd93ee044b680c93cb",
        // 16), LongElement::CHUNK_BITWIDTH));

        // In case of hardcoding the modulus, comment the line that sets the modulus value in generateSampleInput() to avoid an exception

        let rsaEncryptionV1_5_Gadget = RSAEncryptionV1_5_Gadget::new(
            rsaModulus.clone(),
            inputMessage.clone(),
            randomness.clone(),
            rsaKeyLength,
            &None,
            self.cg(),
        );

        // since the randomness vector is a witness in this example, verify any needed constraints
        rsaEncryptionV1_5_Gadget.checkRandomnessCompliance();

        let cipherTextInBytes = rsaEncryptionV1_5_Gadget.getOutputWires(); // in bytes

        // do some grouping to reduce VK Size
        let cipherText = WireArray::new(cipherTextInBytes.clone(), self.cg().downgrade())
            .packWordsIntoLargerWords(8, 30, &None);
        self.makeOutputArray(&cipherText, &Some("Output cipher text".to_owned()));
        (
            self.t.inputMessage,
            self.t.randomness,
            self.t.cipherText,
            self.t.rsaModulus,
            self.t.rsaEncryptionV1_5_Gadget,
        ) = (
            inputMessage,
            randomness,
            cipherText,
            Some(rsaModulus),
            Some(rsaEncryptionV1_5_Gadget),
        );
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        let mut msg = String::new();
        for i in 0..self.t.inputMessage.len() {
            evaluator.setWireValuei(
                self.t.inputMessage[i].as_ref().unwrap(),
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
        // generator.initialize(rsaKeyLength, random);
        // let pair = generator.generateKeyPair();
        // let pubKey = pair.getPublic();
        let modulus = 1i32; //(pubKey).getModulus();

        // let cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
        evaluator.setWireValuebi(
            self.t.rsaModulus.as_ref().unwrap(),
            &BigInteger::from(modulus),
            LongElement::CHUNK_BITWIDTH,
        );

        let privKey = vec![0; 10]; //pair.getPrivate();

        // cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
        let cipherText = vec![1, 2]; // cipher.doFinal(msg.getBytes());
        //			//println!("ciphertext : " + String::new(cipherText));
        let mut cipherTextPadded = vec![0; cipherText.len() + 1];
        cipherTextPadded[1..cipherText.len()].clone_from_slice(&cipherText);
        cipherTextPadded[0] = 0;

        let result = RSAUtil::extractRSARandomness1_5(&cipherText, &privKey);
        // result[0] contains the plaintext (after decryption)
        // result[1] contains the randomness

        assert_eq!(
            &result[0],
            msg.as_bytes(),
            "Randomness Extraction did not decrypt right"
        );

        let sampleRandomness = result[1].clone();
        for i in 0..sampleRandomness.len() {
            evaluator.setWireValuei(
                self.t.randomness[i].as_ref().unwrap(),
                (sampleRandomness[i] as i64 + 256) % 256,
            );
        }

        // //println!("Error while generating sample input for circuit");
    }
}

pub fn main(args: Vec<String>) {
    let keyLength = 2048;
    let msgLength = 3;
    let mut generator = RSAEncryptionCircuitGenerator::new(
        &format!("rsa{keyLength}_encryption"),
        keyLength,
        msgLength,
    );
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit().ok();
    generator.prepFiles(evaluator);
    generator.runLibsnark();
}
