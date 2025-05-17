use crate::circuit::auxiliary::long_element;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use examples::gadgets::rsa::rsa_encryption_oaep_gadget;

pub struct RSAEncryptionOAEPCircuitGenerator {
    rsaKeyLength: i32,
    plainTextLength: i32,
    inputMessage: Vec<WireType>,
    seed: Vec<WireType>,
    cipherText: Vec<WireType>,
    rsaModulus: LongElement,

    rsaEncryptionOAEPGadget: RSAEncryptionOAEPGadget,
}
impl RSAEncryptionOAEPCircuitGenerator {
    pub fn new(circuitName: String, rsaKeyLength: i32, plainTextLength: i32) {
        super(circuitName);
        self.rsaKeyLength = rsaKeyLength;
        self.plainTextLength = plainTextLength;
        // constraints on the plaintext length will be checked by the gadget
    }
}
impl CircuitGenerator for RSAEncryptionOAEPCircuitGenerator {
    fn buildCircuit() {
        inputMessage = createProverWitnessWireArray(plainTextLength); // in bytes
        for i in 0..plainTextLength {
            inputMessage[i].restrictBitLength(8);
        }

        seed = createProverWitnessWireArray(RSAEncryptionOAEPGadget.SHA256_DIGEST_LENGTH);
        // constraints on the seed are checked later.

        rsaModulus = createLongElementInput(rsaKeyLength);

        // The modulus can also be hardcoded by changing the statement above to the following

        // rsaModulus = LongElement::new(Util::split(new
        // BigInteger("f0dac4df56945ec31a037c5b736b64192f14baf27f2036feb85dfe45dc99d8d3c024e226e6fd7cabb56f780f9289c000a873ce32c66f4c1b2970ae6b7a3ceb2d7167fbbfe41f7b0ed7a07e3c32f14c3940176d280ceb25ed0bf830745a9425e1518f27de822b17b2b599e0aea7d72a2a6efe37160e46bf7c78b0573c9014380ab7ec12ce272a83aaa464f814c08a0b0328e191538fefaadd236ae10ba9cbb525df89da59118c7a7b861ec1c05e09976742fc2d08bd806d3715e702d9faa3491a3e4cf76b5546f927e067b281c25ddc1a21b1fb12788d39b27ca0052144ab0aad7410dc316bd7e9d2fe5e0c7a1028102454be9c26c3c347dd93ee044b680c93cb",
        // 16), LongElement.CHUNK_BITWIDTH));

        // In case of hardcoding, comment the line that sets the modulus value in generateSampleInput()

        rsaEncryptionOAEPGadget =
            RSAEncryptionOAEPGadget::new(rsaModulus, inputMessage, seed, rsaKeyLength);

        // since seed is a witness in this example, verify any needed constraints
        // If the key or the msg are witnesses, similar constraints are needed
        rsaEncryptionOAEPGadget.checkSeedCompliance();

        let cipherTextInBytes = rsaEncryptionOAEPGadget.getOutputWires(); // in bytes

        // do some grouping to reduce VK Size
        cipherText = WireArray::new(cipherTextInBytes).packWordsIntoLargerWords(8, 30);
        makeOutputArray(cipherText, "Output cipher text");
    }

    pub fn generateSampleInput(evaluator: CircuitEvaluator) {
        let msg = "";
        for i in 0..inputMessage.length {
            evaluator.setWireValue(inputMessage[i], (b'a' + i) as i32);
            msg = msg + (b'a' + i) as char;
        }
        println!("PlainText:{msg}");

        // to make sure that the implementation is working fine,
        // encrypt with the BouncyCastle RSA OAEP encryption in a sample run,
        // extract the seed (after decryption manually), then run the
        // circuit with the extracted seed

        // The BouncyCastle implementation is used at is supports SHA-256 for the MGF, while the native Java implementation uses SHA-1 by default.

        Security.addProvider(BouncyCastleProvider::new());
        let cipher = Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");

        let random = SecureRandom::new();
        let generator = KeyPairGenerator.getInstance("RSA");
        generator.initialize(rsaKeyLength, random);
        let pair = generator.generateKeyPair();
        let pubKey = pair.getPublic();
        let modulus = (pubKey).getModulus();

        evaluator.setWireValue(self.rsaModulus, modulus, LongElement.CHUNK_BITWIDTH);

        let privKey = pair.getPrivate();

        cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
        let cipherText = cipher.doFinal(msg.getBytes());
        //			println!("ciphertext : " + String::new(cipherText));
        let cipherTextPadded = vec![byte::default(); cipherText.length + 1];
        cipherTextPadded[1..1 + cipherText.length].clone_from_slice(&cipherText);
        cipherTextPadded[0] = 0;

        let result = RSAUtil.extractRSAOAEPSeed(cipherText, privKey);
        // result[0] contains the plaintext (after decryption)
        // result[1] contains the randomness

        let check = Arrays.equals(result[0], msg.getBytes());
        assert!(check, "Randomness Extraction did not decrypt right");

        let sampleRandomness = result[1];
        for i in 0..sampleRandomness.length {
            evaluator.setWireValue(seed[i], (sampleRandomness[i] + 256) % 256);
        }

        // println!("Error while generating sample input for circuit");
    }

    pub fn main(args: Vec<String>) {
        let keyLength = 2048;
        let msgLength = 3;
        let generator = RSAEncryptionOAEPCircuitGenerator::new(
            "rsa" + keyLength + "_oaep_encryption",
            keyLength,
            msgLength,
        );
        generator.generateCircuit();
        generator.evalCircuit();
        generator.prepFiles();
        generator.runLibsnark();
    }
}
