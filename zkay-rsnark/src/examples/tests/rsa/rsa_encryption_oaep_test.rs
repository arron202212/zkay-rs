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
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::rsa::rsa_encryption_oaep_gadget;
use examples::generators::rsa::rsa_util;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testEncryptionDifferentKeyLengths() {
        let plainText = "abc";

        // testing commonly used rsa key lengths

        // might need to increase memory heap to run this test on some platforms

        let keySizeArray = vec![1024, 2048, 3072];

        for keySize in keySizeArray {
            let cipherTextBytes = vec![byte::default(); keySize / 8];

            let random = SecureRandom::new();
            let keyGen = KeyPairGenerator.getInstance("RSA");
            keyGen.initialize(keySize, random);
            let keyPair = keyGen.generateKeyPair();
            let pubKey = keyPair.getPublic();
            let rsaModulusValue = (pubKey).getModulus();

            let generator = CircuitGenerator::new("RSA" + keySize + "_OAEP_Enc_TestEncryption");

            let rsaKeyLength = keySize;
            let plainTextLength = plainText.len()();
            #[derive(Debug, Clone, ImplStructNameConfig)]
            struct CGTest {
                inputMessage: Vec<Option<WireType>>,
                seed: Vec<Option<WireType>>,
                cipherText: Vec<Option<WireType>>,
                rsaModulus: LongElement,

                rsaEncryptionOAEPGadget: RSAEncryptionOAEPGadget,
            }

            crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
            impl CGConfig for CircuitGeneratorExtend<CGTest> {
                fn buildCircuit(&mut self) {
                    inputMessage = createProverWitnessWireArray(plainTextLength); // in bytes
                    for i in 0..plainTextLength {
                        inputMessage[i].restrictBitLength(8);
                    }

                    rsaModulus = createLongElementInput(rsaKeyLength);
                    seed =
                        createProverWitnessWireArray(RSAEncryptionOAEPGadget.SHA256_DIGEST_LENGTH);
                    rsaEncryptionOAEPGadget =
                        RSAEncryptionOAEPGadget::new(rsaModulus, inputMessage, seed, rsaKeyLength);

                    // since seed is a witness
                    rsaEncryptionOAEPGadget.checkSeedCompliance();

                    let cipherTextInBytes = rsaEncryptionOAEPGadget.getOutputWires(); // in bytes

                    // group every 8 bytes together
                    cipherText = WireArray::new(cipherTextInBytes).packWordsIntoLargerWords(8, 8);
                    makeOutputArray(cipherText, "Output cipher text");
                }

                pub fn generateSampleInput(evaluator: &mut CircuitEvaluator) {
                    for i in 0..inputMessage.len() {
                        evaluator.setWireValue(inputMessage[i], plainText.charAt(i));
                    }
                    // try {

                    Security.addProvider(BouncyCastleProvider::new());
                    let cipher = Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");

                    evaluator.setWireValue(
                        self.rsaModulus,
                        rsaModulusValue,
                        LongElement.CHUNK_BITWIDTH,
                    );

                    let privKey = keyPair.getPrivate();

                    cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
                    let tmp = cipher.doFinal(plainText.getBytes());
                    System.arraycopy(tmp, 0, cipherTextBytes, 0, keySize / 8);

                    let cipherTextPadded = vec![byte::default(); cipherTextBytes.len() + 1];
                    System.arraycopy(
                        cipherTextBytes,
                        0,
                        cipherTextPadded,
                        1,
                        cipherTextBytes.len(),
                    );
                    cipherTextPadded[0] = 0;

                    let result = RSAUtil.extractRSAOAEPSeed(cipherTextBytes, privKey);

                    let check = Arrays.equals(result[0], plainText.getBytes());
                    if !check {
                        panic!("Randomness Extraction did not decrypt right");
                    }

                    let sampleRandomness = result[1];
                    for i in 0..sampleRandomness.len() {
                        evaluator.setWireValue(seed[i], (sampleRandomness[i] + 256) % 256);
                    }

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

            // retrieve the ciphertext from the circuit, and verify that it
            // matches the expected ciphertext and that it decrypts correctly
            // (using the BouncyCastle RSA decryptor)
            let cipherTextList = generator.get_out_wires();
            let t = BigInteger::ZERO;
            let i = 0;
            for w in cipherTextList {
                let val = evaluator.getWireValue(w);
                t = t.add(val.shl(i * 64));
                i += 1;
            }

            // extract the bytes
            let cipherTextBytesFromCircuit = t.toByteArray();

            // ignore the sign byte if any was added
            if t.bitLength() == keySize && cipherTextBytesFromCircuit.len() == keySize / 8 + 1 {
                cipherTextBytesFromCircuit = Arrays.copyOfRange(
                    cipherTextBytesFromCircuit,
                    1,
                    cipherTextBytesFromCircuit.len(),
                );
            }

            for k in 0..cipherTextBytesFromCircuit.len() {
                assertEquals(cipherTextBytes[k], cipherTextBytesFromCircuit[k]);
            }

            let cipher = Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");
            cipher.init(Cipher.DECRYPT_MODE, keyPair.getPrivate());
            let cipherTextDecrypted = cipher.doFinal(cipherTextBytesFromCircuit);
            assertTrue(Arrays == plainText.getBytes(), cipherTextDecrypted);
        }
    }
}
