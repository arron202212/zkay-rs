
use circuit::auxiliary::long_element;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::rsa::rsa_encryption_oaep_gadget;
use examples::generators::rsa::rsa_util;

public class RSAEncryptionOAEP_Test extends TestCase {

	@Test
	public void testEncryptionDifferentKeyLengths()  {

		String plainText = "abc";

		// testing commonly used rsa key lengths

		// might need to increase memory heap to run this test on some platforms

		int[] keySizeArray = new int[] { 1024, 2048, 3072 };

		for (int keySize : keySizeArray) {

			final byte[] cipherTextBytes = new byte[keySize / 8];

			SecureRandom random = new SecureRandom();
			KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
			keyGen.initialize(keySize, random);
			KeyPair keyPair = keyGen.generateKeyPair();
			Key pubKey = keyPair.getPublic();
			BigInteger rsaModulusValue = ((RSAPublicKey) pubKey).getModulus();

			CircuitGenerator generator = new CircuitGenerator("RSA" + keySize
					+ "_OAEP_Enc_TestEncryption") {

				int rsaKeyLength = keySize;
				int plainTextLength = plainText.length();
				Wire[] inputMessage;
				Wire[] seed;
				Wire[] cipherText;
				LongElement rsaModulus;

				RSAEncryptionOAEPGadget rsaEncryptionOAEPGadget;

				
				protected void buildCircuit() {
					inputMessage = createProverWitnessWireArray(plainTextLength); // in bytes
					for(int i = 0; i < plainTextLength;i+=1){
						inputMessage[i].restrictBitLength(8);
					}
					
					rsaModulus = createLongElementInput(rsaKeyLength);
					seed = createProverWitnessWireArray(RSAEncryptionOAEPGadget.SHA256_DIGEST_LENGTH);
					rsaEncryptionOAEPGadget = new RSAEncryptionOAEPGadget(
							rsaModulus, inputMessage, seed, rsaKeyLength);

					// since seed is a witness
					rsaEncryptionOAEPGadget.checkSeedCompliance();
					
					Wire[] cipherTextInBytes = rsaEncryptionOAEPGadget
							.getOutputWires(); // in bytes

					// group every 8 bytes together
					cipherText = new WireArray(cipherTextInBytes)
							.packWordsIntoLargerWords(8, 8);
					makeOutputArray(cipherText, "Output cipher text");
				}

				
				public void generateSampleInput(CircuitEvaluator evaluator) {

					for i in 0..inputMessage.length {
						evaluator.setWireValue(inputMessage[i],
								plainText.charAt(i));
					}
					try {

						Security.addProvider(new BouncyCastleProvider());
						Cipher cipher = Cipher.getInstance(
								"RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");

						evaluator
								.setWireValue(this.rsaModulus, rsaModulusValue,
										LongElement.CHUNK_BITWIDTH);

						Key privKey = keyPair.getPrivate();

						cipher.init(Cipher.ENCRYPT_MODE, pubKey, random);
						byte[] tmp = cipher.doFinal(plainText.getBytes());
						System.arraycopy(tmp, 0, cipherTextBytes, 0,
								keySize / 8);

						byte[] cipherTextPadded = new byte[cipherTextBytes.length + 1];
						System.arraycopy(cipherTextBytes, 0, cipherTextPadded,
								1, cipherTextBytes.length);
						cipherTextPadded[0] = 0;

						byte[][] result = RSAUtil.extractRSAOAEPSeed(
								cipherTextBytes, (RSAPrivateKey) privKey);

						boolean check = Arrays.equals(result[0],
								plainText.getBytes());
						if !check {
							panic!(
									"Randomness Extraction did not decrypt right");
						}

						byte[] sampleRandomness = result[1];
						for i in 0..sampleRandomness.length {
							evaluator.setWireValue(seed[i],
									(sampleRandomness[i] + 256) % 256);
						}

					} catch (Exception e) {
						System.err
								.println("Error while generating sample input for circuit");
						e.printStackTrace();
					}
				}
			};

			generator.generateCircuit();
			generator.evalCircuit();
			CircuitEvaluator evaluator = generator.getCircuitEvaluator();

			// retrieve the ciphertext from the circuit, and verify that it
			// matches the expected ciphertext and that it decrypts correctly
			// (using the BouncyCastle RSA decryptor)
			ArrayList<Wire> cipherTextList = generator.getOutWires();
			BigInteger t = BigInteger.ZERO;
			int i = 0;
			for (Wire w : cipherTextList) {
				BigInteger val = evaluator.getWireValue(w);
				t = t.add(val.shiftLeft(i * 64));
				i+=1;
			}

			// extract the bytes
			byte[] cipherTextBytesFromCircuit = t.toByteArray();

			// ignore the sign byte if any was added
			if t.bitLength( == keySize
					&& cipherTextBytesFromCircuit.length == keySize / 8 + 1) {
				cipherTextBytesFromCircuit = Arrays.copyOfRange(
						cipherTextBytesFromCircuit, 1,
						cipherTextBytesFromCircuit.length);
			}

			for k in 0..cipherTextBytesFromCircuit.length {
				assertEquals(cipherTextBytes[k], cipherTextBytesFromCircuit[k]);

			}

			Cipher cipher = Cipher.getInstance(
					"RSA/ECB/OAEPWithSHA-256AndMGF1Padding", "BC");
			cipher.init(Cipher.DECRYPT_MODE, keyPair.getPrivate());
			byte[] cipherTextDecrypted = cipher
					.doFinal(cipherTextBytesFromCircuit);
			assertTrue(Arrays.equals(plainText.getBytes(), cipherTextDecrypted));
		}

	}
}
