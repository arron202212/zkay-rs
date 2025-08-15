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
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget};
use crate::examples::gadgets::rsa::rsa_sig_verification_v1_5_gadget::RSASigVerificationV1_5_Gadget;
use zkay_derive::ImplStructNameConfig;
//a demo for RSA Signatures PKCS #1, V1.5
crate::impl_struct_name_for!(CircuitGeneratorExtend<RSASigVerCircuitGenerator>);
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct RSASigVerCircuitGenerator {
    pub rsaKeyLength: i32,
    pub inputMessage: Vec<Option<WireType>>,
    pub signature: Option<LongElement>,
    pub rsaModulus: Option<LongElement>,
    pub sha2Gadget: Option<Gadget<SHA256Gadget<Base>>>,
    pub rsaSigVerificationV1_5_Gadget: Option<Gadget<RSASigVerificationV1_5_Gadget>>,
}
impl RSASigVerCircuitGenerator {
    pub fn new(circuit_name: &str, rsaKeyLength: i32) -> CircuitGeneratorExtend<Self> {
        CircuitGeneratorExtend::new(
            circuit_name,
            Self {
                rsaKeyLength,
                inputMessage: vec![],
                signature: None,
                rsaModulus: None,
                sha2Gadget: None,
                rsaSigVerificationV1_5_Gadget: None,
            },
        )
    }
}
impl CGConfig for CircuitGeneratorExtend<RSASigVerCircuitGenerator> {
    fn buildCircuit(&mut self) {
        // a sample input message of 3 byte
        let inputMessage = self.createInputWireArray(3, &None);
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
        let digest = sha2Gadget.getOutputWires();

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

        let rsaModulus = self.createLongElementInput(self.t.rsaKeyLength, &None);

        // The modulus can also be hardcoded by changing the statement above to the following

        // rsaModulus = LongElement::new(Util::split(new
        // BigInteger("f0dac4df56945ec31a037c5b736b64192f14baf27f2036feb85dfe45dc99d8d3c024e226e6fd7cabb56f780f9289c000a873ce32c66f4c1b2970ae6b7a3ceb2d7167fbbfe41f7b0ed7a07e3c32f14c3940176d280ceb25ed0bf830745a9425e1518f27de822b17b2b599e0aea7d72a2a6efe37160e46bf7c78b0573c9014380ab7ec12ce272a83aaa464f814c08a0b0328e191538fefaadd236ae10ba9cbb525df89da59118c7a7b861ec1c05e09976742fc2d08bd806d3715e702d9faa3491a3e4cf76b5546f927e067b281c25ddc1a21b1fb12788d39b27ca0052144ab0aad7410dc316bd7e9d2fe5e0c7a1028102454be9c26c3c347dd93ee044b680c93cb",
        // 16), LongElement::CHUNK_BITWIDTH));

        // In case of hardcoding the modulus, comment the line that sets the modulus value in generateSampleInput() to avoid an exception

        let signature = self.createLongElementProverWitness(self.t.rsaKeyLength, &None);

        // since the signature is provided as a witness, verify some properties
        // about it
        signature.restrictBitwidth();
        signature.assertLessThan(&rsaModulus); // might not be really necessary in that
        // case

        let rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
            rsaModulus.clone(),
            digest.clone(),
            signature.clone(),
            self.t.rsaKeyLength,
            &None,
            self.cg(),
        );
        self.makeOutput(
            rsaSigVerificationV1_5_Gadget.getOutputWires()[0]
                .as_ref()
                .unwrap(),
            &Some("Is Signature valid?".to_owned()),
        );
        (
            self.t.inputMessage,
            self.t.signature,
            self.t.rsaModulus,
            self.t.sha2Gadget,
            self.t.rsaSigVerificationV1_5_Gadget,
        ) = (
            inputMessage,
            Some(signature),
            Some(rsaModulus),
            Some(sha2Gadget),
            Some(rsaSigVerificationV1_5_Gadget),
        );
    }

    fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
        let inputStr = b"abc";
        for i in 0..self.t.inputMessage.len() {
            evaluator.setWireValuei(self.t.inputMessage[i].as_ref().unwrap(), inputStr[i] as i64);
        }

        // let keyGen = KeyPairGenerator.getInstance("RSA");
        // keyGen.initialize(rsaKeyLength, SecureRandom::new());
        // let keyPair = keyGen.generateKeyPair();

        // let signature = Signature.getInstance("SHA256withRSA");
        // signature.initSign(keyPair.getPrivate());

        let message = inputStr;
        // signature.update(message);

        let sigBytes = vec![0u8; 32]; //signature.sign();
        let mut signaturePadded = vec![0; sigBytes.len() + 1];
        signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
        signaturePadded[0] = 0;
        let modulus = BigInteger::from(7); //(keyPair.getPublic()).getModulus();
        //			//println!(modulus.toString(16));
        let sig = BigInteger::from_signed_bytes_be(&signaturePadded);

        // if !minimizeVerificationKey {
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
        // } else {
        // evaluator.setWireValue(self.rsaModulusWires,
        // Util::split(modulus, Configs.log2_field_prime - 1));
        // evaluator.setWireValue(self.signatureWires,
        // Util::split(sig, Configs.log2_field_prime - 1));
        // }

        // //println!("Error while generating sample input for circuit");
    }
}

pub fn main(args: Vec<String>) {
    let keyLength = 2048;
    let mut generator =
        RSASigVerCircuitGenerator::new(&format! {"rsa{keyLength}_sha256_sig_verify"}, keyLength);
    generator.generateCircuit();
    let mut evaluator = generator.evalCircuit().ok();
    generator.prepFiles(evaluator);
    generator.runLibsnark();
}
