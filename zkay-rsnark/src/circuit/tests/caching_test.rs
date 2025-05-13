

use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;

use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::hash::sha256_gadget;
use examples::gadgets::math::field_division_gadget;

pub struct CachingTest extends TestCase {

	@Test
	pub   testCaching1() {

		i32 numIns = Config.LOG2_FIELD_PRIME;
		Vec<BigInteger> inVals1 = Util::randomBigIntegerArray(numIns,
				Config.FIELD_PRIME);
		Vec<BigInteger> inVals2 = Util::randomBigIntegerArray(numIns,
				Config.FIELD_PRIME);
		Vec<BigInteger> inVals3 = Util::randomBigIntegerArray(numIns, 32);

		Vec<BigInteger> shiftedRightVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> shiftedLeftVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> rotatedRightVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> rotatedLeftVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> xoredVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> oredVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> andedVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> invertedVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> multipliedVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> addedVals = vec![BigInteger::default();numIns];

		BigInteger mask = BigInteger::new("2").pow(Config.LOG2_FIELD_PRIME)
				.subtract(BigInteger.ONE);

		for i in 0..numIns {

			shiftedRightVals[i] = inVals1[i].shiftRight(i).mod(
					Config.FIELD_PRIME);
			shiftedLeftVals[i] = inVals1[i].shiftLeft(i).and(mask)
					.mod(Config.FIELD_PRIME);
			rotatedRightVals[i] = BigInteger.valueOf(Integer.rotateRight(
					inVals3[i].intValue(), i % 32) & 0x00000000ffffffffL);
			rotatedLeftVals[i] = BigInteger.valueOf(Integer.rotateLeft(
					inVals3[i].intValue(), i % 32) & 0x00000000ffffffffL);
			xoredVals[i] = inVals1[i].xor(inVals2[i]).mod(Config.FIELD_PRIME);
			oredVals[i] = inVals1[i].or(inVals2[i]).mod(Config.FIELD_PRIME);
			andedVals[i] = inVals1[i].and(inVals2[i]).mod(Config.FIELD_PRIME);
			invertedVals[i] = BigInteger
					.valueOf(~inVals3[i].intValue() & 0x00000000ffffffffL);
			multipliedVals[i] = inVals1[i].multiply(inVals2[i]).mod(
					Config.FIELD_PRIME);
			addedVals[i] = inVals1[i].add(inVals2[i]).mod(Config.FIELD_PRIME);

		}

		CircuitGenerator generator = CircuitGenerator::new("Caching_Test") {
			Vec<Wire> inputs1;
			Vec<Wire> inputs2;
			Vec<Wire> inputs3; // 32-bit values

			
			  fn buildCircuit() {

				inputs1 = createInputWireArray(numIns);
				inputs2 = createInputWireArray(numIns);
				inputs3 = createInputWireArray(numIns);

				Vec<Wire> shiftedRight = vec![Wire::default();numIns];
				Vec<Wire> shiftedLeft = vec![Wire::default();numIns];
				Vec<Wire> rotatedRight = vec![Wire::default();numIns];
				Vec<Wire> rotatedLeft = vec![Wire::default();numIns];
				Vec<Wire> xored = vec![Wire::default();numIns];
				Vec<Wire> ored = vec![Wire::default();numIns];
				Vec<Wire> anded = vec![Wire::default();numIns];
				Vec<Wire> inverted = vec![Wire::default();numIns];

				Vec<Wire> multiplied = vec![Wire::default();numIns];
				Vec<Wire> added = vec![Wire::default();numIns];
				
				for i in 0..numIns {
					shiftedRight[i] = inputs1[i].shiftRight(
							Config.LOG2_FIELD_PRIME, i);
					shiftedLeft[i] = inputs1[i].shiftLeft(
							Config.LOG2_FIELD_PRIME, i);
					rotatedRight[i] = inputs3[i].rotateRight(32, i % 32);
					rotatedLeft[i] = inputs3[i].rotateLeft(32, i % 32);
					xored[i] = inputs1[i].xorBitwise(inputs2[i],
							Config.LOG2_FIELD_PRIME);
					ored[i] = inputs1[i].orBitwise(inputs2[i],
							Config.LOG2_FIELD_PRIME);
					anded[i] = inputs1[i].andBitwise(inputs2[i],
							Config.LOG2_FIELD_PRIME);
					inverted[i] = inputs3[i].invBits(32);
					multiplied[i] = inputs1[i].mul(inputs2[i]);
					added[i] = inputs1[i].add(inputs2[i]);
				}

				i32 currentCost = getNumOfConstraints();

				// repeat everything again, and verify that the number of
				// multiplication gates will not be affected
				for i in 0..numIns {
					shiftedRight[i] = inputs1[i].shiftRight(
							Config.LOG2_FIELD_PRIME, i);
					shiftedLeft[i] = inputs1[i].shiftLeft(
							Config.LOG2_FIELD_PRIME, i);
					rotatedRight[i] = inputs3[i].rotateRight(32, i % 32);
					rotatedLeft[i] = inputs3[i].rotateLeft(32, i % 32);
					xored[i] = inputs1[i].xorBitwise(inputs2[i],
							Config.LOG2_FIELD_PRIME);
					ored[i] = inputs1[i].orBitwise(inputs2[i],
							Config.LOG2_FIELD_PRIME);
					anded[i] = inputs1[i].andBitwise(inputs2[i],
							Config.LOG2_FIELD_PRIME);
					inverted[i] = inputs3[i].invBits(32);
					multiplied[i] = inputs1[i].mul(inputs2[i]);
					added[i] = inputs1[i].add(inputs2[i]);
				}

				assertTrue(getNumOfConstraints() == currentCost);

				// repeat binary operations again while changing the order of
				// the operands, and verify that the number of multiplication
				// gates will not be affected
				for i in 0..numIns {
					xored[i] = inputs2[i].xorBitwise(inputs1[i],
							Config.LOG2_FIELD_PRIME);
					ored[i] = inputs2[i].orBitwise(inputs1[i],
							Config.LOG2_FIELD_PRIME);
					anded[i] = inputs2[i].andBitwise(inputs1[i],
							Config.LOG2_FIELD_PRIME);
					multiplied[i] = inputs2[i].mul(inputs1[i]);
					added[i] = inputs2[i].add(inputs1[i]);
				}

				assertTrue(getNumOfConstraints() == currentCost);

				makeOutputArray(shiftedRight);
				makeOutputArray(shiftedLeft);
				makeOutputArray(rotatedRight);
				makeOutputArray(rotatedLeft);
				makeOutputArray(xored);
				makeOutputArray(ored);
				makeOutputArray(anded);
				makeOutputArray(inverted);
				makeOutputArray(multiplied);
				makeOutputArray(added);

				currentCost = getNumOfConstraints();

				// repeat labeling as output (although not really meaningful)
				// and make sure no more constraints are added
				makeOutputArray(shiftedRight);
				makeOutputArray(shiftedLeft);
				makeOutputArray(rotatedRight);
				makeOutputArray(rotatedLeft);
				makeOutputArray(xored);
				makeOutputArray(ored);
				makeOutputArray(anded);
				makeOutputArray(inverted);
				makeOutputArray(multiplied);
				makeOutputArray(added);

				assertTrue(getNumOfConstraints() == currentCost);
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(inputs1, inVals1);
				evaluator.setWireValue(inputs2, inVals2);
				evaluator.setWireValue(inputs3, inVals3);
			}
		};
		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();

		ArrayList<Wire> outWires = generator.getOutWires();
		i32 i, outputIndex = 0;
		for i in 0..numIns
			assertEquals(shiftedRightVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(shiftedLeftVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(rotatedRightVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(rotatedLeftVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(xoredVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(oredVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(andedVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(invertedVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(multipliedVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(addedVals[i],
					evaluator.getWireValue(outWires.get(i + outputIndex)));

	}

	@Test
	pub   testAssertionCache() {

		// make sure we remove some of the clear duplicate assertions
		// and most importantly, no assertions are removed
		CircuitGenerator generator = CircuitGenerator::new("assertions") {

			Wire in1;
			Wire in2;
			Wire witness1;
			Wire witness2;

			
			  fn buildCircuit() {

				in1 = createInputWire();
				in2 = createInputWire();
				witness1 = createProverWitnessWire();
				witness2 = createProverWitnessWire();

				addAssertion(in1, in2, witness1);
				assertEquals(getNumOfConstraints(), 1);
				addAssertion(in1, in2, witness1);
				assertEquals(getNumOfConstraints(), 1);
				addAssertion(in2, in1, witness1);
				assertEquals(getNumOfConstraints(), 1);

				// since witness2 is another wire, the constraint should go
				// through
				addAssertion(in1, in2, witness2);
				assertEquals(getNumOfConstraints(), 2);
				addAssertion(in2, in1, witness2);
				assertEquals(getNumOfConstraints(), 2);

				addEqualityAssertion(witness1, witness2);
				assertEquals(getNumOfConstraints(), 3);
				addEqualityAssertion(witness2, witness1);
				assertEquals(getNumOfConstraints(), 4); // we don't detect
														// similarity here yet

				FieldDivisionGadget::new(in1, in2);
				assertEquals(getNumOfConstraints(), 5);
				FieldDivisionGadget::new(in1, in2);
				// since this operation is implemented externally, it's not easy
				// to filter it, because everytime a witness wire is introduced
				// by the gadget. To eliminate such similar operations, the
				// gadget itself needs to take care of it.
				assertEquals(getNumOfConstraints(), 6);
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(in1, BigInteger.valueOf(5));
				evaluator.setWireValue(in2, BigInteger.valueOf(6));
				evaluator.setWireValue(witness1, BigInteger.valueOf(30));
				evaluator.setWireValue(witness2, BigInteger.valueOf(30));

			}
		};
		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
	}

	@Test
	pub   testMultiSHA256Calls() {

		// testing multiple unncessary calls to SHA256

		String inputStr = "abc";
		String expectedDigest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

		CircuitGenerator generator = CircuitGenerator::new("SHA2_Test4") {

			Vec<Wire> inputWires;

			
			  fn buildCircuit() {
				inputWires = createInputWireArray(inputStr.length());
				Vec<Wire> digest = SHA256Gadget::new(inputWires, 8,
						inputStr.length(), false, true, "").getOutputWires();
				i32 numOfConstraintsBefore = getNumOfConstraints();
				digest = SHA256Gadget::new(inputWires, 8, inputStr.length(),
						false, true, "").getOutputWires();
				digest = SHA256Gadget::new(inputWires, 8, inputStr.length(),
						false, true, "").getOutputWires();
				digest = SHA256Gadget::new(inputWires, 8, inputStr.length(),
						false, true, "").getOutputWires();
				digest = SHA256Gadget::new(inputWires, 8, inputStr.length(),
						false, true, "").getOutputWires();
				digest = SHA256Gadget::new(inputWires, 8, inputStr.length(),
						false, true, "").getOutputWires();

				// verify that the number of constraints match
				assertEquals(numOfConstraintsBefore, getNumOfConstraints());

				// do a small change and verify that number changes
				Vec<Wire> in2 = Arrays.copyOf(inputWires, inputWires.length);
				in2[0] = in2[1];
				SHA256Gadget::new(in2, 8, inputStr.length(), false, true, "")
						.getOutputWires();
				assertTrue(numOfConstraintsBefore < getNumOfConstraints());

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
			outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16),
					8);
		}
		assertEquals(outDigest, expectedDigest);
	}

}
