

use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;

use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::hash::sha256_gadget;
use examples::gadgets::math::field_division_gadget;

pub struct CachingTest  {

	
	pub   testCaching1() {

		let numIns = Config.LOG2_FIELD_PRIME;
		let inVals1 = Util::randomBigIntegerArray(numIns,
				Config.FIELD_PRIME);
		let inVals2 = Util::randomBigIntegerArray(numIns,
				Config.FIELD_PRIME);
		let inVals3 = Util::randomBigIntegerArray(numIns, 32);

		let shiftedRightVals = vec![BigInteger::default();numIns];
		let shiftedLeftVals = vec![BigInteger::default();numIns];
		let rotatedRightVals = vec![BigInteger::default();numIns];
		let rotatedLeftVals = vec![BigInteger::default();numIns];
		let xoredVals = vec![BigInteger::default();numIns];
		let oredVals = vec![BigInteger::default();numIns];
		let andedVals = vec![BigInteger::default();numIns];
		let invertedVals = vec![BigInteger::default();numIns];
		let multipliedVals = vec![BigInteger::default();numIns];
		let addedVals = vec![BigInteger::default();numIns];

		let mask = BigInteger::new("2").pow(Config.LOG2_FIELD_PRIME)
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

		let generator = CircuitGenerator::new("Caching_Test") {
			let inputs1;
			let inputs2;
			let inputs3; // 32-bit values

			
			  fn buildCircuit() {

				inputs1 = createInputWireArray(numIns);
				inputs2 = createInputWireArray(numIns);
				inputs3 = createInputWireArray(numIns);

				let shiftedRight = vec![Wire::default();numIns];
				let shiftedLeft = vec![Wire::default();numIns];
				let rotatedRight = vec![Wire::default();numIns];
				let rotatedLeft = vec![Wire::default();numIns];
				let xored = vec![Wire::default();numIns];
				let ored = vec![Wire::default();numIns];
				let anded = vec![Wire::default();numIns];
				let inverted = vec![Wire::default();numIns];

				let multiplied = vec![Wire::default();numIns];
				let added = vec![Wire::default();numIns];
				
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

				let currentCost = getNumOfConstraints();

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

			
			pub   generateSampleInput(let evaluator) {
				evaluator.setWireValue(inputs1, inVals1);
				evaluator.setWireValue(inputs2, inVals2);
				evaluator.setWireValue(inputs3, inVals3);
			}
		};
		generator.generateCircuit();
		let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();

		ArrayList<Wire> outWires = generator.getOutWires();
		let i, outputIndex = 0;
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

	
	pub   testAssertionCache() {

		// make sure we remove some of the clear duplicate assertions
		// and most importantly, no assertions are removed
		let generator = CircuitGenerator::new("assertions") {

			let in1;
			let in2;
			let witness1;
			let witness2;

			
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

			
			pub   generateSampleInput(let evaluator) {
				evaluator.setWireValue(in1, BigInteger.valueOf(5));
				evaluator.setWireValue(in2, BigInteger.valueOf(6));
				evaluator.setWireValue(witness1, BigInteger.valueOf(30));
				evaluator.setWireValue(witness2, BigInteger.valueOf(30));

			}
		};
		generator.generateCircuit();
		let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
	}

	
	pub   testMultiSHA256Calls() {

		// testing multiple unncessary calls to SHA256

		let inputStr = "abc";
		let expectedDigest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

		let generator = CircuitGenerator::new("SHA2_Test4") {

			let inputWires;

			
			  fn buildCircuit() {
				inputWires = createInputWireArray(inputStr.length());
				let digest = SHA256Gadget::new(inputWires, 8,
						inputStr.length(), false, true, "").getOutputWires();
				let numOfConstraintsBefore = getNumOfConstraints();
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
				let in2 = Arrays.copyOf(inputWires, inputWires.length);
				in2[0] = in2[1];
				SHA256Gadget::new(in2, 8, inputStr.length(), false, true, "")
						.getOutputWires();
				assertTrue(numOfConstraintsBefore < getNumOfConstraints());

				makeOutputArray(digest);
			}

			
			pub   generateSampleInput(let e) {
				for i in 0..inputStr.length() {
					e.setWireValue(inputWires[i], inputStr.charAt(i));
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		let evaluator = generator.getCircuitEvaluator();

		let outDigest = "";
		for w in generator.getOutWires() {
			outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16),
					8);
		}
		assertEquals(outDigest, expectedDigest);
	}

}
