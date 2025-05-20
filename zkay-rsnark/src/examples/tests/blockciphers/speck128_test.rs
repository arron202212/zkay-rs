
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::blockciphers::speck128_cipher_gadget;

/**
 * Tests SPECK block cipher @ keysize = 128, blocksize = 128.
 * Test vector obtained from:  https://github.com/inmcm/Simon_Speck_Ciphers/blob/master/Python/SimonSpeckCiphers/tests/test_simonspeck.py			
 */

pub struct Speck128_Test  {

	
	pub   testCase1() {
		
		CircuitGenerator generator = CircuitGenerator::new("Speck128_Test") {

			Vec<Option<WireType>> plaintext; // 2 64-bit words
			Vec<Option<WireType>> key; // 2 64-bit words
			Vec<Option<WireType>> ciphertext; // 2 64-bit words
			
			
			  fn buildCircuit() {
				plaintext = createInputWireArray(2);
				key = createInputWireArray(2);
let expandedKey = Speck128CipherGadget.expandKey(key);
				ciphertext = Speck128CipherGadget::new(plaintext, expandedKey).getOutputWires();
				makeOutputArray(ciphertext);
			}

			
			pub  fn generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(key[0], BigInteger::new("0706050403020100", 16));
				evaluator.setWireValue(key[1], BigInteger::new("0f0e0d0c0b0a0908", 16));
				evaluator.setWireValue(plaintext[0], BigInteger::new("7469206564616d20", 16));
				evaluator.setWireValue(plaintext[1], BigInteger::new("6c61766975716520", 16));
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText= generator.getOutWires();
		assertEquals(evaluator.getWireValue(cipherText.get(0)), BigInteger::new("7860fedf5c570d18", 16));
		assertEquals(evaluator.getWireValue(cipherText.get(1)), BigInteger::new("a65d985179783265", 16));
	}
}
