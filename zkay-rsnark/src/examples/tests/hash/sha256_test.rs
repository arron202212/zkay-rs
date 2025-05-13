
use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::hash::sha256_gadget;


/**
 * Tests SHA256 standard cases.
 * 
 */

pub struct SHA256_Test extends TestCase {

	@Test
	pub   testCase1() {

		String inputStr = "";
		String expectedDigest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

		CircuitGenerator generator = CircuitGenerator::new("SHA2_Test1") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {
				inputWires = createInputWireArray(inputStr.length());
				Vec<Wire> digest = SHA256Gadget::new(inputWires, 8, inputStr.length(), false, true, "").getOutputWires();
				makeOutputArray(digest);
			}

			
			pub   generateSampleInput(CircuitEvaluator e) {
				// no input needed
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();

		String outDigest = "";
		for w in generator.getOutWires() {
			outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
		}
		assertEquals(outDigest, expectedDigest);

	}

	@Test
	pub   testCase2() {

		String inputStr = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
		String expectedDigest = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";

		CircuitGenerator generator = CircuitGenerator::new("SHA2_Test2") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {
				inputWires = createInputWireArray(inputStr.length());
				Vec<Wire> digest = SHA256Gadget::new(inputWires, 8, inputStr.length(), false, true, "").getOutputWires();
				makeOutputArray(digest);
			}

			
			pub   generateSampleInput(CircuitEvaluator e) {
				for i in 0..inputStr.length() {
					e.setWireValue(inputWires[i], inputStr.charAt(i));
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();

		String outDigest = "";
		for w in generator.getOutWires() {
			outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
		}
		assertEquals(outDigest, expectedDigest);

	}

	@Test
	pub   testCase3() {

		String inputStr = "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
		String expectedDigest = "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1";

		CircuitGenerator generator = CircuitGenerator::new("SHA2_Test3") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {
				inputWires = createInputWireArray(inputStr.length());
				Vec<Wire> digest = SHA256Gadget::new(inputWires, 8, inputStr.length(), false, true, "").getOutputWires();
				makeOutputArray(digest);
			}

			
			pub   generateSampleInput(CircuitEvaluator e) {
				for i in 0..inputStr.length() {
					e.setWireValue(inputWires[i], inputStr.charAt(i));
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();

		String outDigest = "";
		for w in generator.getOutWires() {
			outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
		}
		assertEquals(outDigest, expectedDigest);

	}

	@Test
	pub   testCase4() {

		String inputStr = "abc";
		String expectedDigest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

		CircuitGenerator generator = CircuitGenerator::new("SHA2_Test4") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {
				inputWires = createInputWireArray(inputStr.length());
				Vec<Wire> digest = SHA256Gadget::new(inputWires, 8, inputStr.length(), false, true, "").getOutputWires();
				makeOutputArray(digest);
			}

			
			pub   generateSampleInput(CircuitEvaluator e) {
				for i in 0..inputStr.length() {
					e.setWireValue(inputWires[i], inputStr.charAt(i));
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();

		String outDigest = "";
		for w in generator.getOutWires() {
			outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
		}
		assertEquals(outDigest, expectedDigest);
	}
	
	
	
	@Test
	pub   testCase5() {

		String inputStr = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
		String expectedDigest = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";

		// Testing different settings of the bitWidthPerInputElement parameter
		// wordSize = # of bytes per input wire
		
		for wordSize in 1..=Config.LOG2_FIELD_PRIME / 8 - 1{
			
			i32 numBytesPerInputWire = wordSize;
			
			CircuitGenerator generator = CircuitGenerator::new("SHA2_Test5") {

				Vec<Wire> inputWires;
				
				  fn buildCircuit() {
					inputWires = createInputWireArray(inputStr.length()
							/ numBytesPerInputWire
							+ (if inputStr.length() % numBytesPerInputWire != 0  { 1 }else { 0}));
					Vec<Wire> digest = SHA256Gadget::new(inputWires, 8 * numBytesPerInputWire,
							inputStr.length(), false, true, "")
							.getOutputWires();
					makeOutputArray(digest);
				}

				
				pub   generateSampleInput(CircuitEvaluator e) {
					for i in 0..inputWires.length {
						BigInteger sum = BigInteger.ZERO;
						for  j in  i * numBytesPerInputWire.. j < inputStr.length().min((i + 1) * numBytesPerInputWire)
								 {
							BigInteger v = BigInteger.valueOf(inputStr
									.charAt(j));
							sum = sum.add(v.shiftLeft((j % numBytesPerInputWire) * 8));
						}
						e.setWireValue(inputWires[i], sum);
					}
				}
			};

			generator.generateCircuit();
			generator.evalCircuit();
			CircuitEvaluator evaluator = generator.getCircuitEvaluator();

			String outDigest = "";
			for w in generator.getOutWires() {
				outDigest += Util::padZeros(
						evaluator.getWireValue(w).toString(16), 8);
			}
			assertEquals(outDigest, expectedDigest);

		}

	}
}
