#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::util::util::{BigInteger, Util};

use crate::circuit::eval::instruction::Instruction;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;

pub struct PrimitiveOpTest;

#[cfg(test)]
mod test {

    #[test]
    pub fn testAddition() {
        let mut numIns = 100;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);

        let mut result = vec![];
        result.add(
            inVals1[0]
                .add(inVals1[1])
                .rem(Configs.get().unwrap().field_prime.clone()),
        );
        let mut s = BigInteger::ZERO;
        for i in 0..numIns {
            s = s.add(inVals1[i]);
        }
        result.add(s.rem(Configs.get().unwrap().field_prime.clone()));
        for i in 0..numIns {
            result.add(
                inVals1[i]
                    .add(inVals2[i])
                    .rem(Configs.get().unwrap().field_prime.clone()),
            );
        }

        let mut generator = CircuitGenerator::new("addition");
        {
            let mut inputs1;
            let mut inputs2;

            fn buildCircuit() {
                inputs1 = WireArray::new(generator.createInputWireArray(numIns));
                inputs2 = WireArray::new(generator.createInputWireArray(numIns));

                let mut result1 = inputs1.get(0).add(inputs1.get(1), "");
                let mut result2 = inputs1.sumAllElements();
                let mut resultArray = inputs1.addWireArray(inputs2, inputs1.size());

                generator.makeOutput(result1, "");
                generator.makeOutput(result2, "");
                generator
                    .generator
                    .makeOutputArray(resultArray.asArray(), "");
            }

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(inputs1.asArray(), inVals1);
                evaluator.setWireValue(inputs2.asArray(), inVals2);
            }
        };

        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator.clone());
        generator.generateSampleInput(evaluator.clone());
        evaluator.evaluate();

        let mut idx = 0;
        for output in generator.getOutWires() {
            assert_eq!(evaluator.getWireValue(output), result.get(idx));
            idx += 1;
        }
        assert_eq!(generator.getNumOfConstraints(), numIns + 2);
    }

    #[test]
    pub fn testMultiplication() {
        let mut numIns = 100;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);

        let mut result = vec![];
        result.add(
            inVals1[0]
                .mul(inVals1[1])
                .rem(Configs.get().unwrap().field_prime.clone()),
        );
        for i in 0..numIns {
            result.add(
                inVals1[i]
                    .mul(inVals2[i])
                    .rem(Configs.get().unwrap().field_prime.clone()),
            );
        }

        let mut generator = CircuitGenerator::new("multiplication");
        {
            let mut inputs1;
            let mut inputs2;

            fn buildCircuit() {
                inputs1 = WireArray::new(generator.createInputWireArray(numIns));
                inputs2 = WireArray::new(generator.createInputWireArray(numIns));

                let mut result1 = inputs1.get(0).mul(inputs1.get(1), "");
                let mut resultArray = inputs1.mulWireArray(inputs2, numIns);

                generator.makeOutput(result1, "");
                generator
                    .generator
                    .makeOutputArray(resultArray.asArray(), "");
            }

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(inputs1.asArray(), inVals1);
                evaluator.setWireValue(inputs2.asArray(), inVals2);
            }
        };
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator.clone());
        generator.generateSampleInput(evaluator.clone());
        evaluator.evaluate();
        let mut idx = 0;
        for output in generator.getOutWires() {
            assert_eq!(evaluator.getWireValue(output), result.get(idx));
            idx += 1;
        }
        assert_eq!(generator.getNumOfConstraints(), numIns + 1);
    }

    #[test]
    pub fn testComparison() {
        let mut numIns = 10000;
        let mut numBits = 10;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, numBits);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, numBits);

        let mut result = vec![];
        for i in 0..numIns {
            let b = inVals1[i] - inVals2[i];
            result.add(if b == 0 {
                0
            } else if b > 0 {
                1
            } else {
                -1
            });
        }

        let mut result1 = vec![None; numIns];
        let mut result2 = vec![None; numIns];
        let mut result3 = vec![None; numIns];
        let mut result4 = vec![None; numIns];
        let mut result5 = vec![None; numIns];

        let mut generator = CircuitGenerator::new("comparison");
        {
            let mut inputs1;
            let mut inputs2;

            fn buildCircuit() {
                inputs1 = generator.createInputWireArray(numIns);
                inputs2 = generator.createInputWireArray(numIns);

                for i in 0..numIns {
                    result1[i] = inputs1[i].isLessThan(inputs2[i], numBits);
                    result2[i] = inputs1[i].isLessThanOrEqual(inputs2[i], numBits);
                    result3[i] = inputs1[i].isGreaterThan(inputs2[i], numBits);
                    result4[i] = inputs1[i].isGreaterThanOrEqual(inputs2[i], numBits);
                    result5[i] = inputs1[i].isEqualTo(inputs2[i]);
                }
            }

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(inputs1, inVals1);
                evaluator.setWireValue(inputs2, inVals2);
            }
        };
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator);
        generator.generateSampleInput(evaluator);
        //		generator.printCircuit();
        evaluator.evaluate();
        for i in 0..numIns {
            let mut r = result.get(i);
            if r == 0 {
                assert_eq!(evaluator.getWireValue(result1[i]), BigInteger::ZERO);
                assert_eq!(evaluator.getWireValue(result2[i]), Util::one());
                assert_eq!(evaluator.getWireValue(result3[i]), BigInteger::ZERO);
                assert_eq!(evaluator.getWireValue(result4[i]), Util::one());
                assert_eq!(evaluator.getWireValue(result5[i]), Util::one());
            } else if r == 1 {
                assert_eq!(evaluator.getWireValue(result1[i]), BigInteger::ZERO);
                assert_eq!(evaluator.getWireValue(result2[i]), BigInteger::ZERO);
                assert_eq!(evaluator.getWireValue(result3[i]), Util::one());
                assert_eq!(evaluator.getWireValue(result4[i]), Util::one());
                assert_eq!(evaluator.getWireValue(result5[i]), BigInteger::ZERO);
            } else if r == -1 {
                assert_eq!(evaluator.getWireValue(result1[i]), Util::one());
                assert_eq!(evaluator.getWireValue(result2[i]), Util::one());
                assert_eq!(evaluator.getWireValue(result3[i]), BigInteger::ZERO);
                assert_eq!(evaluator.getWireValue(result4[i]), BigInteger::ZERO);
                assert_eq!(evaluator.getWireValue(result5[i]), BigInteger::ZERO);
            }
        }
    }

    #[test]
    pub fn testBooleanOperations() {
        let mut numIns = Config.log2_field_prime;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);
        let mut inVals3 = Util::randomBigIntegerArray(numIns, 32);

        let mut shiftedRightVals = vec![BigInteger::default(); numIns];
        let mut shiftedLeftVals = vec![BigInteger::default(); numIns];
        let mut rotatedRightVals = vec![BigInteger::default(); numIns];
        let mut rotatedLeftVals = vec![BigInteger::default(); numIns];
        let mut xoredVals = vec![BigInteger::default(); numIns];
        let mut oredVals = vec![BigInteger::default(); numIns];
        let mut andedVals = vec![BigInteger::default(); numIns];
        let mut invertedVals = vec![BigInteger::default(); numIns];

        let mut mask = BigInteger::from(2)
            .pow(Config.log2_field_prime as u32)
            .sub(Util::one());

        for i in 0..numIns {
            shiftedRightVals[i] = inVals1[i]
                .shiftRight(i)
                .rem(Configs.get().unwrap().field_prime.clone());
            shiftedLeftVals[i] = inVals1[i]
                .shl(i)
                .and(mask)
                .rem(Configs.get().unwrap().field_prime.clone());
            rotatedRightVals[i] = BigInteger::from(
                inVals3[i]
                    .to_str_radix(10)
                    .parse::<i64>()
                    .unwrap()
                    .rotateRight(i % 32)
                    & 0x00000000ffffffff,
            );
            rotatedLeftVals[i] = BigInteger::from(
                inVals3[i]
                    .to_str_radix(10)
                    .parse::<i64>()
                    .unwrap()
                    .rotateLeft(i % 32)
                    & 0x00000000ffffffff,
            );
            xoredVals[i] = inVals1[i]
                .xor(inVals2[i])
                .rem(Configs.get().unwrap().field_prime.clone());
            oredVals[i] = inVals1[i]
                .or(inVals2[i])
                .rem(Configs.get().unwrap().field_prime.clone());
            andedVals[i] = inVals1[i]
                .and(inVals2[i])
                .rem(Configs.get().unwrap().field_prime.clone());
            invertedVals[i] = BigInteger::from(
                !inVals3[i].to_str_radix(10).parse::<i64>().unwrap() & 0x00000000ffffffff,
            );
        }

        let mut generator = CircuitGenerator::new("boolean_operations");
        {
            let mut inputs1;
            let mut inputs2;
            let mut inputs3;

            fn buildCircuit() {
                inputs1 = generator.createInputWireArray(numIns);
                inputs2 = generator.createInputWireArray(numIns);
                inputs3 = generator.createInputWireArray(numIns);

                let mut shiftedRight = vec![None; numIns];
                let mut shiftedLeft = vec![None; numIns];
                let mut rotatedRight = vec![None; numIns];
                let mut rotatedLeft = vec![None; numIns];
                let mut xored = vec![None; numIns];
                let mut ored = vec![None; numIns];
                let mut anded = vec![None; numIns];
                let mut inverted = vec![None; numIns];

                for i in 0..numIns {
                    shiftedRight[i] = inputs1[i].shiftRight(Config.log2_field_prime, i);
                    shiftedLeft[i] = inputs1[i].shl(Config.log2_field_prime, i);
                    rotatedRight[i] = inputs3[i].rotateRight(32, i % 32);
                    rotatedLeft[i] = inputs3[i].rotateLeft(32, i % 32);
                    xored[i] = inputs1[i].xorBitwise(inputs2[i], Config.log2_field_prime);
                    ored[i] = inputs1[i].orBitwise(inputs2[i], Config.log2_field_prime);
                    anded[i] = inputs1[i].andBitwise(inputs2[i], Config.log2_field_prime);

                    inverted[i] = inputs3[i].invBits(32);
                }

                generator.makeOutputArray(shiftedRight);
                generator.makeOutputArray(shiftedLeft);
                generator.makeOutputArray(rotatedRight);
                generator.makeOutputArray(rotatedLeft);
                generator.makeOutputArray(xored);
                generator.makeOutputArray(ored);
                generator.makeOutputArray(anded);
                generator.makeOutputArray(inverted);
            }

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(inputs1, inVals1);
                evaluator.setWireValue(inputs2, inVals2);
                evaluator.setWireValue(inputs3, inVals3);
            }
        };
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator.clone());
        generator.generateSampleInput(evaluator.clone());
        evaluator.evaluate();

        let mut outWires = generator.getOutWires();
        let (mut i, mut outputIndex) = (0, 0);
        for i in 0..numIns {
            assert_eq!(
                shiftedRightVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                shiftedLeftVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                rotatedRightVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                rotatedLeftVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                xoredVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                oredVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                andedVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                invertedVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }
    }

    #[test]
    pub fn testAssertion() {
        let mut numIns = 100;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, Configs.get().unwrap().field_prime);

        let mut result = vec![];
        result.add(
            inVals1[0]
                .mul(inVals1[0])
                .rem(Configs.get().unwrap().field_prime.clone()),
        );
        for i in 0..numIns {
            result.add(
                inVals1[i]
                    .mul(inVals2[i])
                    .rem(Configs.get().unwrap().field_prime.clone()),
            );
        }

        let mut generator = CircuitGenerator::new("assertions");
        {
            let mut inputs1;
            let mut inputs2;
            let mut solutions; // provide solutions as witnesses

            fn buildCircuit() {
                inputs1 = WireArray::new(generator.createInputWireArray(numIns));
                inputs2 = WireArray::new(generator.createInputWireArray(numIns));
                solutions = WireArray::new(generator.createProverWitnessWireArray(numIns + 1));

                specifyProverWitnessComputation(&{
                    #[derive(Hash, Clone, Debug)]
                    struct Prover {
                        result: Vec<BigInteger>,
                        solutions: WireArray,
                    }
                    impl Instruction for Prover {
                        fn evaluate(evaluator: CircuitEvaluator) {
                            evaluator.setWireValue(solutions.get(0), result.get(0));
                            for i in 0..numIns {
                                evaluator.setWireValue(solutions.get(i + 1), result.get(i + 1));
                            }
                        }
                    }
                    Box::new(Prover {
                        result: result.clone(),
                        solutions: solutions.clone(),
                    })
                });

                addAssertion(inputs1.get(0), inputs1.get(0), solutions.get(0));
                for i in 0..numIns {
                    addAssertion(inputs1.get(i), inputs2.get(i), solutions.get(i + 1));
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

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(inputs1.asArray(), inVals1);
                evaluator.setWireValue(inputs2.asArray(), inVals2);
            }
        };
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator.clone());
        generator.generateSampleInput(evaluator.clone());
        evaluator.evaluate(); // no exception will be thrown
        assert_eq!(generator.getNumOfConstraints(), numIns + 2);
    }
}
