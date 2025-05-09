
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::blockciphers::chaskey_lts128_cipher_gadget;


// test case from:  https://www.cryptolux.org/index.php/FELICS

public class Chaskey128_Test extends TestCase {

	@Test
	public void testCase1() {

		CircuitGenerator generator = new CircuitGenerator("Chaskey_Test1") {
	
			private Wire[] plaintext; // 4 32-bit words
			private Wire[] key; // 4 32-bit words
			private Wire[] ciphertext; // 4 32-bit words

			
			protected void buildCircuit() {
				plaintext = createInputWireArray(4);
				key = createInputWireArray(4);
				ciphertext = new ChaskeyLTS128CipherGadget(plaintext, key)
						.getOutputWires();
				makeOutputArray(ciphertext);
			}

			
			public void generateSampleInput(CircuitEvaluator evaluator) {

				BigInteger[] keyV = { BigInteger.valueOf(0x68e90956L),
						BigInteger.valueOf(0x29e3585fL),
						BigInteger.valueOf(0x98ecec40L),
						BigInteger.valueOf(0x2f9822c5L) };

				BigInteger[] msgV = { BigInteger.valueOf(0x262823b8L),
						BigInteger.valueOf(0x5e405efdL),
						BigInteger.valueOf(0xa901a369L),
						BigInteger.valueOf(0xd87aea78L) };

				for i in 0..plaintext.length {
					evaluator.setWireValue(plaintext[i], msgV[i]);
				}
				for i in 0..key.length {
					evaluator.setWireValue(key[i], keyV[i]);
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();
		ArrayList<Wire> cipherText = generator.getOutWires();

		BigInteger[] expeectedCiphertext = { BigInteger.valueOf(0x4d8d60d5L),
				BigInteger.valueOf(0x7b34bfa2L),
				BigInteger.valueOf(0x2f77f8abL),
				BigInteger.valueOf(0x07deeddfL) };

		for i in 0..4 {
			assertEquals(evaluator.getWireValue(cipherText.get(i)),
					expeectedCiphertext[i]);
		}

	}

}
