
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::blockciphers::chaskey_lts128_cipher_gadget;


// test case from:  https://www.cryptolux.org/index.php/FELICS

pub struct Chaskey128_Test  {

	
	pub   testCase1() {

		CircuitGenerator generator = CircuitGenerator::new("Chaskey_Test1") {
	
			 Vec<Option<WireType>> plaintext; // 4 32-bit words
			 Vec<Option<WireType>> key; // 4 32-bit words
			 Vec<Option<WireType>> ciphertext; // 4 32-bit words

			
			  fn buildCircuit() {
				plaintext = createInputWireArray(4);
				key = createInputWireArray(4);
				ciphertext = ChaskeyLTS128CipherGadget::new(plaintext, key)
						.getOutputWires();
				makeOutputArray(ciphertext);
			}

			
			pub  fn generateSampleInput(CircuitEvaluator evaluator) {

				Vec<BigInteger> keyV = { BigInteger::from(0x68e90956L),
						BigInteger::from(0x29e3585fL),
						BigInteger::from(0x98ecec40L),
						BigInteger::from(0x2f9822c5L) };

				Vec<BigInteger> msgV = { BigInteger::from(0x262823b8L),
						BigInteger::from(0x5e405efdL),
						BigInteger::from(0xa901a369L),
						BigInteger::from(0xd87aea78L) };

				for i in 0..plaintext.len() {
					evaluator.setWireValue(plaintext[i], msgV[i]);
				}
				for i in 0..key.len() {
					evaluator.setWireValue(key[i], keyV[i]);
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText = generator.getOutWires();

		Vec<BigInteger> expeectedCiphertext = { BigInteger::from(0x4d8d60d5L),
				BigInteger::from(0x7b34bfa2L),
				BigInteger::from(0x2f77f8abL),
				BigInteger::from(0x07deeddfL) };

		for i in 0..4 {
			assertEquals(evaluator.getWireValue(cipherText.get(i)),
					expeectedCiphertext[i]);
		}

	}

}
