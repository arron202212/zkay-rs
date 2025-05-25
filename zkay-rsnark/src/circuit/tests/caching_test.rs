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

use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget;
use crate::examples::gadgets::math::field_division_gadget;

pub struct CachingTest;
#[cfg(test)]
mod test {

    #[test]
    pub fn testCaching1() {
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
        let mut multipliedVals = vec![BigInteger::default(); numIns];
        let mut addedVals = vec![BigInteger::default(); numIns];

        let mut mask = BigInteger::from(2)
            .pow(Config.log2_field_prime as u32)
            .sub(Util::one());

        for i in 0..numIns {
            shiftedRightVals[i] = inVals1[i]
                .shiftRight(i)
                .rem(Configs.get().unwrap().field_prime);
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
                    ..to_str_radix(10).parse::<i64>().unwrap().rotateLeft(i % 32)
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
            invertedVals[i] = BigInteger
                .valueOf(!inVals3[i].to_str_radix(10).parse::<i64>().unwrap() & 0x00000000ffffffff);
            multipliedVals[i] = inVals1[i]
                .mul(inVals2[i])
                .rem(Configs.get().unwrap().field_prime.clone());
            addedVals[i] = inVals1[i]
                .add(inVals2[i])
                .rem(Configs.get().unwrap().field_prime.clone());
        }

        let mut generator = CircuitGenerator::new("Caching_Test");
        {
            let mut inputs1;
            let mut inputs2;
            let mut inputs3; // 32-bit values

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

                let mut multiplied = vec![None; numIns];
                let mut added = vec![None; numIns];

                for i in 0..numIns {
                    shiftedRight[i] = inputs1[i].shiftRight(Config.log2_field_prime, i);
                    shiftedLeft[i] = inputs1[i].shl(Config.log2_field_prime, i);
                    rotatedRight[i] = inputs3[i].rotateRight(32, i % 32);
                    rotatedLeft[i] = inputs3[i].rotateLeft(32, i % 32);
                    xored[i] = inputs1[i].xorBitwise(inputs2[i], Config.log2_field_prime);
                    ored[i] = inputs1[i].orBitwise(inputs2[i], Config.log2_field_prime);
                    anded[i] = inputs1[i].andBitwise(inputs2[i], Config.log2_field_prime);
                    inverted[i] = inputs3[i].invBits(32);
                    multiplied[i] = inputs1[i].mul(inputs2[i]);
                    added[i] = inputs1[i].add(inputs2[i]);
                }

                let mut currentCost = generator.getNumOfConstraints();

                // repeat everything again, and verify that the number of
                // multiplication gates will not be affected
                for i in 0..numIns {
                    shiftedRight[i] = inputs1[i].shiftRight(Config.log2_field_prime, i);
                    shiftedLeft[i] = inputs1[i].shl(Config.log2_field_prime, i);
                    rotatedRight[i] = inputs3[i].rotateRight(32, i % 32);
                    rotatedLeft[i] = inputs3[i].rotateLeft(32, i % 32);
                    xored[i] = inputs1[i].xorBitwise(inputs2[i], Config.log2_field_prime);
                    ored[i] = inputs1[i].orBitwise(inputs2[i], Config.log2_field_prime);
                    anded[i] = inputs1[i].andBitwise(inputs2[i], Config.log2_field_prime);
                    inverted[i] = inputs3[i].invBits(32);
                    multiplied[i] = inputs1[i].mul(inputs2[i]);
                    added[i] = inputs1[i].add(inputs2[i]);
                }

                assert!(generator.getNumOfConstraints() == currentCost);

                // repeat binary operations again while changing the order of
                // the operands, and verify that the number of multiplication
                // gates will not be affected
                for i in 0..numIns {
                    xored[i] = inputs2[i].xorBitwise(inputs1[i], Config.log2_field_prime);
                    ored[i] = inputs2[i].orBitwise(inputs1[i], Config.log2_field_prime);
                    anded[i] = inputs2[i].andBitwise(inputs1[i], Config.log2_field_prime);
                    multiplied[i] = inputs2[i].mul(inputs1[i]);
                    added[i] = inputs2[i].add(inputs1[i]);
                }

                assert!(generator.getNumOfConstraints() == currentCost);

                generator.makeOutputArray(shiftedRight);
                generator.makeOutputArray(shiftedLeft);
                generator.makeOutputArray(rotatedRight);
                generator.makeOutputArray(rotatedLeft);
                generator.makeOutputArray(xored);
                generator.makeOutputArray(ored);
                generator.makeOutputArray(anded);
                generator.makeOutputArray(inverted);
                generator.makeOutputArray(multiplied);
                generator.makeOutputArray(added);

                currentCost = generator.getNumOfConstraints();

                // repeat labeling as output (although not really meaningful)
                // and make sure no more constraints are added
                generator.makeOutputArray(shiftedRight);
                generator.makeOutputArray(shiftedLeft);
                generator.makeOutputArray(rotatedRight);
                generator.makeOutputArray(rotatedLeft);
                generator.makeOutputArray(xored);
                generator.makeOutputArray(ored);
                generator.makeOutputArray(anded);
                generator.makeOutputArray(inverted);
                generator.makeOutputArray(multiplied);
                generator.makeOutputArray(added);

                assert!(generator.getNumOfConstraints() == currentCost);
            }

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(inputs1, inVals1);
                evaluator.setWireValue(inputs2, inVals2);
                evaluator.setWireValue(inputs3, inVals3);
            }
        };
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator);
        generator.generateSampleInput(evaluator);
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

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                multipliedVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                addedVals[i],
                evaluator.getWireValue(outWires.get(i + outputIndex))
            );
        }
    }

    #[test]
    pub fn testAssertionCache() {
        // make sure we remove some of the clear duplicate assertions
        // and most importantly, no assertions are removed
        let mut generator = CircuitGenerator::new("assertions");
        {
            let mut in1;
            let mut in2;
            let mut witness1;
            let mut witness2;

            fn buildCircuit() {
                in1 = generator.createInputWire();
                in2 = generator.createInputWire();
                witness1 = generator.createProverWitnessWire();
                witness2 = generator.createProverWitnessWire();

                addAssertion(in1, in2, witness1);
                assert_eq!(generator.getNumOfConstraints(), 1);
                addAssertion(in1, in2, witness1);
                assert_eq!(generator.getNumOfConstraints(), 1);
                addAssertion(in2, in1, witness1);
                assert_eq!(generator.getNumOfConstraints(), 1);

                // since witness2 is another wire, the constraint should go
                // through
                addAssertion(in1, in2, witness2);
                assert_eq!(generator.getNumOfConstraints(), 2);
                addAssertion(in2, in1, witness2);
                assert_eq!(generator.getNumOfConstraints(), 2);

                addEqualityAssertion(witness1, witness2);
                assert_eq!(generator.getNumOfConstraints(), 3);
                addEqualityAssertion(witness2, witness1);
                assert_eq!(generator.getNumOfConstraints(), 4); // we don't detect
                // similarity here yet

                FieldDivisionGadget::new(in1, in2);
                assert_eq!(generator.getNumOfConstraints(), 5);
                FieldDivisionGadget::new(in1, in2);
                // since this operation is implemented externally, it's not easy
                // to filter it, because everytime a witness wire is introduced
                // by the gadget. To eliminate such similar operations, the
                // gadget itself needs to take care of it.
                assert_eq!(generator.getNumOfConstraints(), 6);
            }

            pub fn generateSampleInput(evaluator: CircuitEvaluator) {
                evaluator.setWireValue(in1, BigInteger::from(5));
                evaluator.setWireValue(in2, BigInteger::from(6));
                evaluator.setWireValue(witness1, BigInteger::from(30));
                evaluator.setWireValue(witness2, BigInteger::from(30));
            }
        };
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new(generator.clone());
        generator.generateSampleInput(evaluator.clone());
        evaluator.evaluate();
    }

    #[test]
    pub fn testMultiSHA256Calls() {
        // testing multiple unncessary calls to SHA256

        let mut inputStr = "abc";
        let mut expectedDigest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

        let mut generator = CircuitGenerator::new("SHA2_Test4");
        {
            let mut inputWires;

            fn buildCircuit() {
                inputWires = generator.createInputWireArray(inputStr.len()());
                let mut digest =
                    SHA256Gadget::new(inputWires, 8, inputStr.len()(), false, true, "")
                        .getOutputWires();
                let mut numOfConstraintsBefore = generator.getNumOfConstraints();
                digest = SHA256Gadget::new(inputWires, 8, inputStr.len()(), false, true, "")
                    .getOutputWires();
                digest = SHA256Gadget::new(inputWires, 8, inputStr.len()(), false, true, "")
                    .getOutputWires();
                digest = SHA256Gadget::new(inputWires, 8, inputStr.len()(), false, true, "")
                    .getOutputWires();
                digest = SHA256Gadget::new(inputWires, 8, inputStr.len()(), false, true, "")
                    .getOutputWires();
                digest = SHA256Gadget::new(inputWires, 8, inputStr.len()(), false, true, "")
                    .getOutputWires();

                // verify that the number of constraints match
                assert_eq!(numOfConstraintsBefore, generator.getNumOfConstraints());

                // do a small change and verify that number changes
                let mut in2 = Arrays.copyOf(inputWires, inputWires.len());
                in2[0] = in2[1];
                SHA256Gadget::new(in2, 8, inputStr.len()(), false, true, "").getOutputWires();
                assert!(numOfConstraintsBefore < generator.getNumOfConstraints());

                generator.makeOutputArray(digest);
            }

            pub fn generateSampleInput(e: CircuitEvaluator) {
                for i in 0..inputStr.len()() {
                    e.setWireValue(inputWires[i], inputStr.charAt(i));
                }
            }
        };

        generator.generateCircuit();
        generator.evalCircuit();
        let mut evaluator = generator.getCircuitEvaluator();

        let mut outDigest = "";
        for w in generator.getOutWires() {
            outDigest += Util::padZeros(evaluator.getWireValue(w).toString(16), 8);
        }
        assert_eq!(outDigest, expectedDigest);
    }
}
