

use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;

use circuit::eval::instruction;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;

pub struct PrimitiveOpTest  {

	
	pub   testAddition() {

		let numIns = 100;
let inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
let inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);

let result = new ArrayList<BigInteger>();
		result.add(inVals1[0].add(inVals1[1]).mod(Config.FIELD_PRIME));
let s = BigInteger.ZERO;
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

let result1 = inputs1.get(0).add(inputs1.get(1), "");
let result2 = inputs1.sumAllElements();
let resultArray = inputs1.addWireArray(inputs2, inputs1.size());

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
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();

let idx = 0;
		for output in generator.getOutWires() {
			assertEquals(evaluator.getWireValue(output), result.get(idx));
idx+=1;
		}
		assertEquals(generator.getNumOfConstraints(), numIns + 2);

	}

	
	pub   testMultiplication() {

let numIns = 100;
let inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
let inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);

let result = new ArrayList<BigInteger>();
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

let result1 = inputs1.get(0).mul(inputs1.get(1), "");
let resultArray = inputs1.mulWireArray(inputs2, numIns);

				makeOutput(result1, "");
				makeOutputArray(resultArray.asArray(), "");
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(inputs1.asArray(), inVals1);
				evaluator.setWireValue(inputs2.asArray(), inVals2);

			}
		};
		generator.generateCircuit();
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();
let idx = 0;
		for output in generator.getOutWires() {
			assertEquals(evaluator.getWireValue(output), result.get(idx));
            idx+=1;

		}
		assertEquals(generator.getNumOfConstraints(), numIns + 1);
	}

	
	pub   testComparison() {

let numIns = 10000;
let numBits = 10;
let inVals1 = Util::randomBigIntegerArray(numIns, numBits);
let inVals2 = Util::randomBigIntegerArray(numIns, numBits);

let result = new ArrayList<Integer>();
		for i in 0..numIns {
			result.add(inVals1[i].compareTo(inVals2[i]));
		}

let result1 = vec![Wire::default();numIns];
let result2 = vec![Wire::default();numIns];
let result3 = vec![Wire::default();numIns];
let result4 = vec![Wire::default();numIns];
let result5 = vec![Wire::default();numIns];

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
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
//		generator.printCircuit();
		evaluator.evaluate();
		for i in 0..numIns {
let r = result.get(i);
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

	
	pub   testBooleanOperations() {

let numIns = Config.LOG2_FIELD_PRIME;
let inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
let inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
let inVals3 = Util::randomBigIntegerArray(numIns, 32);

let shiftedRightVals = vec![BigInteger::default();numIns];
let shiftedLeftVals = vec![BigInteger::default();numIns];
let rotatedRightVals = vec![BigInteger::default();numIns];
let rotatedLeftVals = vec![BigInteger::default();numIns];
let xoredVals = vec![BigInteger::default();numIns];
let oredVals = vec![BigInteger::default();numIns];
let andedVals = vec![BigInteger::default();numIns];
let invertedVals = vec![BigInteger::default();numIns];

let mask = BigInteger::new("2").pow(Config.LOG2_FIELD_PRIME).subtract(BigInteger.ONE);
		
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

let shiftedRight = vec![Wire::default();numIns];
let shiftedLeft = vec![Wire::default();numIns];
let rotatedRight = vec![Wire::default();numIns];
let rotatedLeft = vec![Wire::default();numIns];
let xored = vec![Wire::default();numIns];
let ored = vec![Wire::default();numIns];
let anded = vec![Wire::default();numIns];
let inverted = vec![Wire::default();numIns];

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
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate();

let outWires = generator.getOutWires();
let i, outputIndex = 0;
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
	
	
	
	pub   testAssertion() {

let numIns = 100;
let inVals1 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
let inVals2 = Util::randomBigIntegerArray(numIns, Config.FIELD_PRIME);
		
		
let result = new ArrayList<BigInteger>();
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
						for i  in 0..numIns{
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
let evaluator = CircuitEvaluator::new(generator);
		generator.generateSampleInput(evaluator);
		evaluator.evaluate(); // no exception will be thrown
		assertEquals(generator.getNumOfConstraints(), numIns + 2);
	}
}
