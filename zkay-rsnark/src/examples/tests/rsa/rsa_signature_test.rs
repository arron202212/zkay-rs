#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget;
use crate::examples::gadgets::rsa::rsa_sig_verification_v1_5_gadget;

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
    pub fn testValidSignatureDifferentKeyLengths() {
        let inputStr = "abc";

        // testing commonly used rsa key lengths in addition to non-power of two
        // ones

        // might need to increase memory heap to run this test on some platforms

        let keySizeArray = vec![1024, 2048, 3072, 4096, 2047, 2049];

        for keySize in keySizeArray {
            let generator = CircuitGenerator::new("RSA" + keySize + "_SIG_TestValid");

            let rsaKeyLength = keySize;
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                inputMessage: Vec<Option<WireType>>,
                signature: LongElement,
                rsaModulus: LongElement,

                sha2Gadget: SHA256Gadget,
                rsaSigVerificationV1_5_Gadget: RSASigVerificationV1_5_Gadget,
            }

            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    inputMessage = createInputWireArray(inputStr.len());
                    sha2Gadget =
                        SHA256Gadget::new(inputMessage, 8, inputMessage.len(), false, true);
                    let digest = sha2Gadget.getOutputWires();
                    rsaModulus = createLongElementInput(rsaKeyLength);
                    signature = createLongElementInput(rsaKeyLength);
                    rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
                        rsaModulus,
                        digest,
                        signature,
                        rsaKeyLength,
                    );
                    makeOutput(rsaSigVerificationV1_5_Gadget.getOutputWires()[0]);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    for i in 0..inputMessage.len() {
                        evaluator.setWireValue(inputMessage[i], inputStr.charAt(i));
                    }
                    // try {
                    let keyGen = KeyPairGenerator.getInstance("RSA");
                    keyGen.initialize(rsaKeyLength, SecureRandom::new());
                    let keyPair = keyGen.generateKeyPair();
                    let signature = Signature.getInstance("SHA256withRSA");
                    signature.initSign(keyPair.getPrivate());

                    let message = inputStr.getBytes();
                    signature.update(message);

                    let sigBytes = signature.sign();

                    // pad an extra zero byte to avoid having a negative big
                    // integer
                    let signaturePadded = vec![0; sigBytes.len() + 1];
                    signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
                    signaturePadded[0] = 0;
                    let modulus = (keyPair.getPublic()).getModulus();
                    let sig = BigInteger::new(signaturePadded);

                    evaluator.setWireValue(self.rsaModulus, modulus, LongElement::CHUNK_BITWIDTH);
                    evaluator.setWireValue(self.signature, sig, LongElement::CHUNK_BITWIDTH);

                    // } catch (Exception e) {
                    // 	System.err
                    // 			.println("Error while generating sample input for circuit");
                    // 	e.printStackTrace();
                    // }
                }
            };

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            assertEquals(
                Util::one(),
                evaluator.getWireValue(generator.get_out_wires().get(0)),
            );
        }
    }

    #[test]
    pub fn testInvalidSignatureDifferentKeyLengths() {
        let inputStr = "abc";

        // testing commonly used rsa key lengths in addition to non-power of two
        // ones

        let keySizeArray = vec![1024, 2048, 3072, 4096, 2047, 2049];

        for keySize in keySizeArray {
            let generator = CircuitGenerator::new("RSA" + keySize + "_SIG_TestInvalid");

            let rsaKeyLength = keySize;
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                inputMessage: Vec<Option<WireType>>,
                signature: LongElement,
                rsaModulus: LongElement,

                sha2Gadget: SHA256Gadget,
                rsaSigVerificationV1_5_Gadget: RSASigVerificationV1_5_Gadget,
            }

            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    inputMessage = createInputWireArray(inputStr.len());
                    sha2Gadget =
                        SHA256Gadget::new(inputMessage, 8, inputMessage.len(), false, true);
                    let digest = sha2Gadget.getOutputWires();
                    rsaModulus = createLongElementInput(rsaKeyLength);
                    signature = createLongElementInput(rsaKeyLength);
                    rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
                        rsaModulus,
                        digest,
                        signature,
                        rsaKeyLength,
                    );
                    makeOutput(rsaSigVerificationV1_5_Gadget.getOutputWires()[0]);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    for i in 0..inputMessage.len() {
                        evaluator.setWireValue(inputMessage[i], inputStr.charAt(i));
                    }
                    // try {
                    let keyGen = KeyPairGenerator.getInstance("RSA");
                    keyGen.initialize(rsaKeyLength, SecureRandom::new());
                    let keyPair = keyGen.generateKeyPair();
                    let signature = Signature.getInstance("SHA256withRSA");
                    signature.initSign(keyPair.getPrivate());

                    let message = inputStr.getBytes();
                    signature.update(message);

                    let sigBytes = signature.sign();

                    // pad an extra zero byte to avoid having a negative big
                    // integer
                    let signaturePadded = vec![0; sigBytes.len() + 1];
                    signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
                    signaturePadded[0] = 0;
                    let modulus = (keyPair.getPublic()).getModulus();
                    let sig = BigInteger::new(signaturePadded);

                    evaluator.setWireValue(self.rsaModulus, modulus, LongElement::CHUNK_BITWIDTH);

                    // input the modulus itself instead of the signature
                    evaluator.setWireValue(
                        self.signature,
                        sig.sub(Util::one()),
                        LongElement::CHUNK_BITWIDTH,
                    );

                    // } catch (Exception e) {
                    // 	System.err
                    // 			.println("Error while generating sample input for circuit");
                    // 	e.printStackTrace();
                    // }
                }
            };

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            assertEquals(
                BigInteger::ZERO,
                evaluator.getWireValue(generator.get_out_wires().get(0)),
            );
        }
    }

    // This test checks the robustness of the code when the chunk bitwidth changes

    #[test]
    pub fn testValidSignatureDifferentChunkBitwidth() {
        let inputStr = "abc";

        let keySize = 1024;
        let defaultBitwidth = LongElement::CHUNK_BITWIDTH;

        let chunkBiwidthArray = vec![i32::default(); 106];
        for b in 16..chunkBiwidthArray.len() {
            LongElement::CHUNK_BITWIDTH = b;
            let generator = CircuitGenerator::new("RSA" + keySize + "_SIG_TestValid_ChunkB_" + b);

            let rsaKeyLength = keySize;
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                inputMessage: Vec<Option<WireType>>,
                signature: LongElement,
                rsaModulus: LongElement,

                sha2Gadget: SHA256Gadget,
                rsaSigVerificationV1_5_Gadget: RSASigVerificationV1_5_Gadget,
            }

            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    inputMessage = createInputWireArray(inputStr.len());
                    sha2Gadget =
                        SHA256Gadget::new(inputMessage, 8, inputMessage.len(), false, true);
                    let digest = sha2Gadget.getOutputWires();
                    rsaModulus = createLongElementInput(rsaKeyLength);
                    signature = createLongElementInput(rsaKeyLength);
                    rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
                        rsaModulus,
                        digest,
                        signature,
                        rsaKeyLength,
                    );
                    makeOutput(rsaSigVerificationV1_5_Gadget.getOutputWires()[0]);
                }

                fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    for i in 0..inputMessage.len() {
                        evaluator.setWireValue(inputMessage[i], inputStr.charAt(i));
                    }
                    // try {
                    let keyGen = KeyPairGenerator.getInstance("RSA");
                    keyGen.initialize(rsaKeyLength, SecureRandom::new());
                    let keyPair = keyGen.generateKeyPair();
                    let signature = Signature.getInstance("SHA256withRSA");
                    signature.initSign(keyPair.getPrivate());

                    let message = inputStr.getBytes();
                    signature.update(message);

                    let sigBytes = signature.sign();

                    // pad an extra zero byte to avoid having a negative big
                    // integer
                    let signaturePadded = vec![0; sigBytes.len() + 1];
                    signaturePadded[1..sigBytes.len()].clone_from_slice(&sigBytes[0..]);
                    signaturePadded[0] = 0;
                    let modulus = (keyPair.getPublic()).getModulus();
                    let sig = BigInteger::new(signaturePadded);

                    evaluator.setWireValue(self.rsaModulus, modulus, LongElement::CHUNK_BITWIDTH);
                    evaluator.setWireValue(self.signature, sig, LongElement::CHUNK_BITWIDTH);

                    // } catch (Exception e) {
                    // 	System.err
                    // 			.println("Error while generating sample input for circuit");
                    // 	e.printStackTrace();
                    // }
                }
            };

            generator.generateCircuit();
            generator.evalCircuit();
            let evaluator = generator.getCircuitEvaluator();
            assertEquals(
                Util::one(),
                evaluator.getWireValue(generator.get_out_wires().get(0)),
            );

            LongElement::CHUNK_BITWIDTH = defaultBitwidth; // needed for running all tests together
        }
    }
}
