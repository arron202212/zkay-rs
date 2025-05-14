
use circuit::auxiliary::long_element;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::hash::sha256_gadget;
use examples::gadgets::rsa::rsa_sig_verification_v1_5_gadget;



//Tests RSA PKCS #1, V1.5 Signature

pub struct RSASignature_Test  {

	/*
	 * Note that these tests are for ensuring the basic functionality. To verify
	 * that the gadget cannot allow *any* invalid signatures to pass, this requires more than testing few cases, e.g. a
	 * careful review of the code  to ensure that there are no
	 * missing/incorrect constraints that a cheating prover could make use of.
	 */

	
	pub   testValidSignatureDifferentKeyLengths() {

let inputStr = "abc";

		// testing commonly used rsa key lengths in addition to non-power of two
		// ones

		// might need to increase memory heap to run this test on some platforms

let keySizeArray = vec![i32::default();] { 1024, 2048, 3072, 4096, 2047, 2049 };

		for keySize in keySizeArray {
			CircuitGenerator generator = CircuitGenerator::new("RSA" + keySize
					+ "_SIG_TestValid") {

let rsaKeyLength = keySize;
				Vec<Wire> inputMessage;
				LongElement signature;
				LongElement rsaModulus;

				SHA256Gadget sha2Gadget;
				RSASigVerificationV1_5_Gadget rsaSigVerificationV1_5_Gadget;

				
				  fn buildCircuit() {
					inputMessage = createInputWireArray(inputStr.length());
					sha2Gadget = SHA256Gadget::new(inputMessage, 8,
							inputMessage.length, false, true);
let digest = sha2Gadget.getOutputWires();
					rsaModulus = createLongElementInput(rsaKeyLength);
					signature = createLongElementInput(rsaKeyLength);
					rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
							rsaModulus, digest, signature, rsaKeyLength);
					makeOutput(rsaSigVerificationV1_5_Gadget.getOutputWires()[0]);
				}

				
				pub   generateSampleInput(CircuitEvaluator evaluator) {

					for i in 0..inputMessage.length {
						evaluator.setWireValue(inputMessage[i],
								inputStr.charAt(i));
					}
					try {
						KeyPairGenerator keyGen = KeyPairGenerator
								.getInstance("RSA");
						keyGen.initialize(rsaKeyLength, SecureRandom::new());
let keyPair = keyGen.generateKeyPair();
						Signature signature = Signature
								.getInstance("SHA256withRSA");
						signature.initSign(keyPair.getPrivate());

let message = inputStr.getBytes();
						signature.update(message);

let sigBytes = signature.sign();

						// pad an extra zero byte to avoid having a negative big
						// integer
let signaturePadded = vec![byte::default();sigBytes.length + 1];
						System.arraycopy(sigBytes, 0, signaturePadded, 1,
								sigBytes.length);
						signaturePadded[0] = 0;
						BigInteger modulus = ((RSAPublicKey) keyPair
								.getPublic()).getModulus();
let sig = BigInteger::new(signaturePadded);

						evaluator.setWireValue(self.rsaModulus, modulus,
								LongElement.CHUNK_BITWIDTH);
						evaluator.setWireValue(self.signature, sig,
								LongElement.CHUNK_BITWIDTH);

					} catch (Exception e) {
						System.err
								.println("Error while generating sample input for circuit");
						e.printStackTrace();
					}
				}
			};

			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
			assertEquals(BigInteger.ONE,
					evaluator.getWireValue(generator.getOutWires().get(0)));
		}

	}

	
	pub   testInvalidSignatureDifferentKeyLengths() {

		
let inputStr = "abc";

		// testing commonly used rsa key lengths in addition to non-power of two
		// ones


let keySizeArray = vec![i32::default();] { 1024, 2048, 3072, 4096, 2047, 2049 };

		for keySize in keySizeArray {
			CircuitGenerator generator = CircuitGenerator::new("RSA" + keySize
					+ "_SIG_TestInvalid") {

let rsaKeyLength = keySize;
				Vec<Wire> inputMessage;
				LongElement signature;
				LongElement rsaModulus;

				SHA256Gadget sha2Gadget;
				RSASigVerificationV1_5_Gadget rsaSigVerificationV1_5_Gadget;

				
				  fn buildCircuit() {
					inputMessage = createInputWireArray(inputStr.length());
					sha2Gadget = SHA256Gadget::new(inputMessage, 8,
							inputMessage.length, false, true);
let digest = sha2Gadget.getOutputWires();
					rsaModulus = createLongElementInput(rsaKeyLength);
					signature = createLongElementInput(rsaKeyLength);
					rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
							rsaModulus, digest, signature, rsaKeyLength);
					makeOutput(rsaSigVerificationV1_5_Gadget.getOutputWires()[0]);
				}

				
				pub   generateSampleInput(CircuitEvaluator evaluator) {

					for i in 0..inputMessage.length {
						evaluator.setWireValue(inputMessage[i],
								inputStr.charAt(i));
					}
					try {
						KeyPairGenerator keyGen = KeyPairGenerator
								.getInstance("RSA");
						keyGen.initialize(rsaKeyLength, SecureRandom::new());
let keyPair = keyGen.generateKeyPair();
						Signature signature = Signature
								.getInstance("SHA256withRSA");
						signature.initSign(keyPair.getPrivate());

let message = inputStr.getBytes();
						signature.update(message);

let sigBytes = signature.sign();

						// pad an extra zero byte to avoid having a negative big
						// integer
let signaturePadded = vec![byte::default();sigBytes.length + 1];
						System.arraycopy(sigBytes, 0, signaturePadded, 1,
								sigBytes.length);
						signaturePadded[0] = 0;
						BigInteger modulus = ((RSAPublicKey) keyPair
								.getPublic()).getModulus();
let sig = BigInteger::new(signaturePadded);

						evaluator.setWireValue(self.rsaModulus, modulus,
								LongElement.CHUNK_BITWIDTH);

						// input the modulus itself instead of the signature
						evaluator.setWireValue(self.signature, sig.subtract(BigInteger.ONE),
								LongElement.CHUNK_BITWIDTH);

					} catch (Exception e) {
						System.err
								.println("Error while generating sample input for circuit");
						e.printStackTrace();
					}
				}
			};

			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
			assertEquals(BigInteger.ZERO,
					evaluator.getWireValue(generator.getOutWires().get(0)));
		}

	}
	
	// This test checks the robustness of the code when the chunk bitwidth changes
	
	
	pub   testValidSignatureDifferentChunkBitwidth() {

let inputStr = "abc";

let keySize = 1024;
let defaultBitwidth = LongElement.CHUNK_BITWIDTH ;

let chunkBiwidthArray = vec![i32::default();106];
		for b in 16..chunkBiwidthArray.length{
			
			LongElement.CHUNK_BITWIDTH = b;
			CircuitGenerator generator = CircuitGenerator::new("RSA" + keySize
					+ "_SIG_TestValid_ChunkB_"+b) {

let rsaKeyLength = keySize;
				Vec<Wire> inputMessage;
				LongElement signature;
				LongElement rsaModulus;

				SHA256Gadget sha2Gadget;
				RSASigVerificationV1_5_Gadget rsaSigVerificationV1_5_Gadget;

				
				  fn buildCircuit() {
					inputMessage = createInputWireArray(inputStr.length());
					sha2Gadget = SHA256Gadget::new(inputMessage, 8,
							inputMessage.length, false, true);
let digest = sha2Gadget.getOutputWires();
					rsaModulus = createLongElementInput(rsaKeyLength);
					signature = createLongElementInput(rsaKeyLength);
					rsaSigVerificationV1_5_Gadget = RSASigVerificationV1_5_Gadget::new(
							rsaModulus, digest, signature, rsaKeyLength);
					makeOutput(rsaSigVerificationV1_5_Gadget.getOutputWires()[0]);
				}

				
				pub   generateSampleInput(CircuitEvaluator evaluator) {

					for i in 0..inputMessage.length {
						evaluator.setWireValue(inputMessage[i],
								inputStr.charAt(i));
					}
					try {
						KeyPairGenerator keyGen = KeyPairGenerator
								.getInstance("RSA");
						keyGen.initialize(rsaKeyLength, SecureRandom::new());
let keyPair = keyGen.generateKeyPair();
						Signature signature = Signature
								.getInstance("SHA256withRSA");
						signature.initSign(keyPair.getPrivate());

let message = inputStr.getBytes();
						signature.update(message);

let sigBytes = signature.sign();

						// pad an extra zero byte to avoid having a negative big
						// integer
let signaturePadded = vec![byte::default();sigBytes.length + 1];
						System.arraycopy(sigBytes, 0, signaturePadded, 1,
								sigBytes.length);
						signaturePadded[0] = 0;
						BigInteger modulus = ((RSAPublicKey) keyPair
								.getPublic()).getModulus();
let sig = BigInteger::new(signaturePadded);

						evaluator.setWireValue(self.rsaModulus, modulus,
								LongElement.CHUNK_BITWIDTH);
						evaluator.setWireValue(self.signature, sig,
								LongElement.CHUNK_BITWIDTH);

					} catch (Exception e) {
						System.err
								.println("Error while generating sample input for circuit");
						e.printStackTrace();
					}
				}
			};

			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
			assertEquals(BigInteger.ONE,
					evaluator.getWireValue(generator.getOutWires().get(0)));
			
			LongElement.CHUNK_BITWIDTH = defaultBitwidth; // needed for running all tests together
		}

	}

}
