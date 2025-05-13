

use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;

use circuit::eval::instruction;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;

pub struct PrimitiveOpTest extends TestCase {

	@Test
	pub   testAddition() {

		i32 numIns = 100;
		Vec<BigInteger> inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		Vec<BigInteger> inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);

		ArrayList<BigInteger> result = new ArrayList<BigInteger>();
		result.add(inVals1[0].add(inVals1[1]).mod(Config.FIELD_PRIME));
		BigInteger s = BigInteger.ZERO;
		for i in 0..numIns {
			s = s.add(inVals1[i]);
		}
		result.add(s.mod(Config.FIELD_PRIME));
		for i in 0..numIns {
			result.add(inVals1[i].add(inVals2[i]).mod(Config.FIELD_PRIME));
		}

		CircuitGenerator generator = CircuitGenerator::new("addition") {
			WireArray inputs1;
			WireArray inputs2;

			
			  fn buildCircuit() {
				inputs1 = WireArray::new(createInputWireArray(numIns));
				inputs2 = WireArray::new(createInputWireArray(numIns));

				Wire result1 = inputs1.get(0).add(inputs1.get(1), "");
				Wire result2 = inputs1.sumAllElements();
				WireArray resultArray = inputs1.addWireArray(inputs2, inputs1.size());

				makeOutput(result1, "");
				makeOutput(result2, "");
				makeOutputArray(resultArray.asArray(), "");
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(inputs1.asArray(), inVals1);
				evaluator.setWireValue(inputs2.asArray(), inVals2);

			}
		};

		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();

		i32 idx = 0;
		for output in generator.getOutWires() {
			assertEquals(evaluator.getWireValue(output), result.get(idx+=1));
		}
		assertEquals(generator.getNumOfConstraints(), numIns + 2);

	}

	@Test
	pub   testMultiplication() {

		i32 numIns = 100;
		Vec<BigInteger> inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		Vec<BigInteger> inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);

		ArrayList<BigInteger> result = new ArrayList<BigInteger>();
		result.add(inVals1[0].multiply(inVals1[1]).mod(Config.FIELD_PRIME));
		for i in 0..numIns {
			result.add(inVals1[i].multiply(inVals2[i]).mod(Config.FIELD_PRIME));
		}

		CircuitGenerator generator = CircuitGenerator::new("multiplication") {
			WireArray inputs1;
			WireArray inputs2;

			
			  fn buildCircuit() {
				inputs1 = WireArray::new(createInputWireArray(numIns));
				inputs2 = WireArray::new(createInputWireArray(numIns));

				Wire result1 = inputs1.get(0).mul(inputs1.get(1), "");
				WireArray resultArray = inputs1.mulWireArray(inputs2, numIns);

				makeOutput(result1, "");
				makeOutputArray(resultArray.asArray(), "");
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(inputs1.asArray(), inVals1);
				evaluator.setWireValue(inputs2.asArray(), inVals2);

			}
		};
		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
		i32 idx = 0;
		for output in generator.getOutWires() {
			assertEquals(evaluator.getWireValue(output), result.get(idx+=1));
		}
		assertEquals(generator.getNumOfConstraints(), numIns + 1);
	}

	@Test
	pub   testComparison() {

		i32 numIns = 10000;
		i32 numBits = 10;
		Vec<BigInteger> inVals1 = Util::randomBigIntegerArray(numIns, numBits);
		Vec<BigInteger> inVals2 = Util::randomBigIntegerArray(numIns, numBits);

		ArrayList<Integer> result = new ArrayList<Integer>();
		for i in 0..numIns {
			result.add(inVals1[i].compareTo(inVals2[i]));
		}

		Vec<Wire> result1 = vec![Wire::default();numIns];
		Vec<Wire> result2 = vec![Wire::default();numIns];
		Vec<Wire> result3 = vec![Wire::default();numIns];
		Vec<Wire> result4 = vec![Wire::default();numIns];
		Vec<Wire> result5 = vec![Wire::default();numIns];

		CircuitGenerator generator = CircuitGenerator::new("comparison") {

			Vec<Wire> inputs1;
			Vec<Wire> inputs2;

			
			  fn buildCircuit() {

				inputs1 = createInputWireArray(numIns);
				inputs2 = createInputWireArray(numIns);

				for i in 0..numIns {
					result1[i] = inputs1[i].isLessThan(inputs2[i], numBits);
					result2[i] = inputs1[i].isLessThanOrEqual(inputs2[i], numBits);
					result3[i] = inputs1[i].isGreaterThan(inputs2[i], numBits);
					result4[i] = inputs1[i].isGreaterThanOrEqual(inputs2[i], numBits);
					result5[i] = inputs1[i].isEqualTo(inputs2[i]);
				}
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(inputs1, inVals1);
				evaluator.setWireValue(inputs2, inVals2);

			}
		};
		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
//		generator.printCircuit();
		evaluator.evaluate();
		for i in 0..numIns {
			i32 r = result.get(i);
			if r == 0 {
				assertEquals(evaluator.getWireValue(result1[i]), BigInteger.ZERO);
				assertEquals(evaluator.getWireValue(result2[i]), BigInteger.ONE);
				assertEquals(evaluator.getWireValue(result3[i]), BigInteger.ZERO);
				assertEquals(evaluator.getWireValue(result4[i]), BigInteger.ONE);
				assertEquals(evaluator.getWireValue(result5[i]), BigInteger.ONE);
			} else if r == 1 {
				assertEquals(evaluator.getWireValue(result1[i]), BigInteger.ZERO);
				assertEquals(evaluator.getWireValue(result2[i]), BigInteger.ZERO);
				assertEquals(evaluator.getWireValue(result3[i]), BigInteger.ONE);
				assertEquals(evaluator.getWireValue(result4[i]), BigInteger.ONE);
				assertEquals(evaluator.getWireValue(result5[i]), BigInteger.ZERO);
			} else if r == -1 {
				assertEquals(evaluator.getWireValue(result1[i]), BigInteger.ONE);
				assertEquals(evaluator.getWireValue(result2[i]), BigInteger.ONE);
				assertEquals(evaluator.getWireValue(result3[i]), BigInteger.ZERO);
				assertEquals(evaluator.getWireValue(result4[i]), BigInteger.ZERO);
				assertEquals(evaluator.getWireValue(result5[i]), BigInteger.ZERO);
			}
		}
	}

	@Test
	pub   testBooleanOperations() {

		i32 numIns = Config.LOG2_FIELD_PRIME;
		Vec<BigInteger> inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		Vec<BigInteger> inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		Vec<BigInteger> inVals3 = Util::randomBigIntegerArray(numIns, 32);

		Vec<BigInteger> shiftedRightVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> shiftedLeftVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> rotatedRightVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> rotatedLeftVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> xoredVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> oredVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> andedVals = vec![BigInteger::default();numIns];
		Vec<BigInteger> invertedVals = vec![BigInteger::default();numIns];

		BigInteger mask = BigInteger::new("2").pow(Config.LOG2_FIELD_PRIME).subtract(BigInteger.ONE);
		
		for i in 0..numIns {
			shiftedRightVals[i] = inVals1[i].shiftRight(i).mod(Config.FIELD_PRIME);
			shiftedLeftVals[i] = inVals1[i].shiftLeft(i).and(mask).mod(Config.FIELD_PRIME);
			rotatedRightVals[i] = BigInteger.valueOf(Integer.rotateRight(inVals3[i].intValue(), i % 32) & 0x00000000ffffffffL);
			rotatedLeftVals[i] = BigInteger.valueOf(Integer.rotateLeft(inVals3[i].intValue(), i % 32) & 0x00000000ffffffffL );
			xoredVals[i] = inVals1[i].xor(inVals2[i]).mod(Config.FIELD_PRIME);
			oredVals[i] = inVals1[i].or(inVals2[i]).mod(Config.FIELD_PRIME);
			andedVals[i] = inVals1[i].and(inVals2[i]).mod(Config.FIELD_PRIME);
			invertedVals[i] = BigInteger.valueOf(~inVals3[i].intValue() & 0x00000000ffffffffL);
		}

		CircuitGenerator generator = CircuitGenerator::new("boolean_operations") {
			Vec<Wire> inputs1;
			Vec<Wire> inputs2;
			Vec<Wire> inputs3;

			
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

				for i in 0..numIns {
					shiftedRight[i] = inputs1[i].shiftRight(Config.LOG2_FIELD_PRIME, i);
					shiftedLeft[i] = inputs1[i].shiftLeft(Config.LOG2_FIELD_PRIME, i);
					rotatedRight[i] = inputs3[i].rotateRight(32, i % 32);
					rotatedLeft[i] = inputs3[i].rotateLeft(32, i % 32);
					xored[i] = inputs1[i].xorBitwise(inputs2[i], Config.LOG2_FIELD_PRIME);
					ored[i] = inputs1[i].orBitwise(inputs2[i], Config.LOG2_FIELD_PRIME);
					anded[i] = inputs1[i].andBitwise(inputs2[i], Config.LOG2_FIELD_PRIME);

					inverted[i] = inputs3[i].invBits(32);
				}

				makeOutputArray(shiftedRight);
				makeOutputArray(shiftedLeft);
				makeOutputArray(rotatedRight);
				makeOutputArray(rotatedLeft);
				makeOutputArray(xored);
				makeOutputArray(ored);
				makeOutputArray(anded);
				makeOutputArray(inverted);
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
			assertEquals(shiftedRightVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(shiftedLeftVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(rotatedRightVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));
		
		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(rotatedLeftVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(xoredVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(oredVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(andedVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

		outputIndex += numIns;
		for i in 0..numIns
			assertEquals(invertedVals[i], evaluator.getWireValue(outWires.get(i + outputIndex)));

	}
	
	
	@Test
	pub   testAssertion() {

		i32 numIns = 100;
		Vec<BigInteger> inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		Vec<BigInteger> inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		
		
		ArrayList<BigInteger> result = new ArrayList<BigInteger>();
		result.add(inVals1[0].multiply(inVals1[0]).mod(Config.FIELD_PRIME));
		for i in 0..numIns {
			result.add(inVals1[i].multiply(inVals2[i]).mod(Config.FIELD_PRIME));
		}

		CircuitGenerator generator = CircuitGenerator::new("assertions") {
			WireArray inputs1;
			WireArray inputs2;
			WireArray solutions; // provide solutions as witnesses

			
			  fn buildCircuit() {
				inputs1 = WireArray::new(createInputWireArray(numIns));
				inputs2 = WireArray::new(createInputWireArray(numIns));
				solutions = WireArray::new(createProverWitnessWireArray(numIns+1));

				specifyProverWitnessComputation(Instruction::new() {
					
					
					pub   evaluate(CircuitEvaluator evaluator) {
						evaluator.setWireValue(solutions.get(0),result.get(0));
						for(i32 i =0; i < numIns;i+=1){
							evaluator.setWireValue(solutions.get(i+1),result.get(i+1));
						}
					}
				});
				
				addAssertion(inputs1.get(0), inputs1.get(0), solutions.get(0));
				for i in 0..numIns{
					addAssertion(inputs1.get(i), inputs2.get(i), solutions.get(i+1));
				}
				
				// constant assertions will not add constraints
				addZeroAssertion(zeroWire);
				addOneAssertion(oneWire);
				addAssertion(zeroWire, oneWire, zeroWire);
				addAssertion(oneWire, oneWire, oneWire);
				addBinaryAssertion(zeroWire);
				addBinaryAssertion(oneWire);
				
				// won't add a constraint
				addEqualityAssertion(inputs1.get(0), inputs1.get(0));

				// will add a constraint
				addEqualityAssertion(inputs1.get(0), inVals1[0]);

				
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(inputs1.asArray(), inVals1);
				evaluator.setWireValue(inputs2.asArray(), inVals2);

			}
		};
		generator.generateCircuit();
		CircuitEvaluator evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate(); // no exception will be thrown
		assertEquals(generator.getNumOfConstraints(), numIns + 2);
	}
}
