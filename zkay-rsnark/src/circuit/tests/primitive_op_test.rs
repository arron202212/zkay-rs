#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, getActiveCircuitGenerator,
                put_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};

use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};

use rccell::{RcCell, WeakCell};
use zkay_derive::ImplStructNameConfig;
pub struct PrimitiveOpTest;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testAddition() {
        let mut numIns = 100;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);

        let mut result = vec![];
        result.push(
            inVals1[0]
                .clone()
                .add(&inVals1[1])
                .rem(&Configs.field_prime),
        );
        let mut s = BigInteger::ZERO;
        let numIns = numIns as usize;
        for i in 0..numIns {
            s = s.add(&inVals1[i]);
        }
        result.push(s.rem(&Configs.field_prime));
        for i in 0..numIns {
            result.push(
                inVals1[i]
                    .clone()
                    .add(&inVals2[i])
                    .rem(&Configs.field_prime),
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: WireArray,
            pub inputs2: WireArray,
            pub inVals1: Vec<BigInteger>,
            pub inVals2: Vec<BigInteger>,
            pub numIns: u64,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = self.cg.clone();

                let numIns = self.t.numIns as usize;
                let mut inputs1 = WireArray::new(
                    generator.createInputWireArray(numIns, &None),
                    generator.clone().downgrade(),
                );
                let mut inputs2 = WireArray::new(
                    generator.createInputWireArray(numIns, &None),
                    generator.clone().downgrade(),
                );

                let mut result1 = inputs1[0]
                    .clone()
                    .unwrap()
                    .add(inputs1[1].as_ref().unwrap());
                let mut result2 = inputs1.sumAllElements(&None);
                let mut resultArray = inputs1.addWireArray(&inputs2, inputs1.size(), &None);

                generator.makeOutput(&result1, &None);
                generator.makeOutput(&result2, &None);
                generator.makeOutputArray(resultArray.asArray(), &None);
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuea(self.t.inputs1.asArray(), &self.t.inVals1);
                evaluator.setWireValuea(self.t.inputs2.asArray(), &self.t.inVals2);
            }
        }
        let cg = CircuitGenerator::new("addition");
        let t = CGTest {
            inputs1: WireArray::newi(0, cg.clone().downgrade()),
            inputs2: WireArray::newi(0, cg.clone().downgrade()),
            inVals1,
            inVals2,
            numIns: numIns as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::newc(cg, t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);

        let mut idx = 0;
        for output in generator.get_out_wires() {
            assert_eq!(
                evaluator.getWireValue(output.as_ref().unwrap()),
                result[idx].clone()
            );
            idx += 1;
        }
        assert_eq!(generator.get_num_of_constraints(), numIns as i32 + 2);
    }

    #[test]
    pub fn testMultiplication() {
        let mut numIns = 100;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);

        let mut result = vec![];
        result.push(
            inVals1[0]
                .clone()
                .mul(&inVals1[1])
                .rem(&Configs.field_prime),
        );
        let numIns = numIns as usize;
        for i in 0..numIns {
            result.push(
                inVals1[i]
                    .clone()
                    .mul(&inVals2[i])
                    .rem(&Configs.field_prime),
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub inVals1: Vec<BigInteger>,
            pub inVals2: Vec<BigInteger>,
            pub numIns: u64,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = self.cg.clone();

                let numIns = self.t.numIns as usize;
                let mut inputs1 = WireArray::new(
                    generator.createInputWireArray(numIns, &None),
                    generator.clone().downgrade(),
                );
                let mut inputs2 = WireArray::new(
                    generator.createInputWireArray(numIns, &None),
                    generator.clone().downgrade(),
                );

                let mut result1 = inputs1[0]
                    .clone()
                    .unwrap()
                    .mul(inputs1[1].as_ref().unwrap());
                let mut resultArray = inputs1.mulWireArray(&inputs2, numIns, &None);

                generator.makeOutput(&result1, &None);
                generator.makeOutputArray(resultArray.asArray(), &None);
                self.t.inputs1 = inputs1.asArray().clone();
                self.t.inputs2 = inputs2.asArray().clone();
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuea(&self.t.inputs1, &self.t.inVals1);
                evaluator.setWireValuea(&self.t.inputs2, &self.t.inVals2);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            inVals1,
            inVals2,
            numIns: numIns as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("multiplication", t);

        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let mut idx = 0;
        for output in generator.get_out_wires() {
            assert_eq!(
                evaluator.getWireValue(output.as_ref().unwrap()),
                result[idx].clone()
            );
            idx += 1;
        }
        assert_eq!(generator.get_num_of_constraints(), numIns as i32 + 1);
    }

    #[test]
    pub fn testComparison() {
        let mut numIns = 10000;
        let mut numBits = 10;
        let mut inVals1 = Util::randomBigIntegerArrayi(numIns, numBits);
        let mut inVals2 = Util::randomBigIntegerArrayi(numIns, numBits);
        let numIns = numIns as usize;
        let mut result = vec![];
        for i in 0..numIns {
            let b = inVals1[i].clone() - inVals2[i].clone();
            result.push(if b == BigInteger::ZERO {
                0
            } else if b > BigInteger::ZERO {
                1
            } else {
                -1
            });
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub inVals1: Vec<BigInteger>,
            pub inVals2: Vec<BigInteger>,
            pub result1: Vec<Option<WireType>>,
            pub result2: Vec<Option<WireType>>,
            pub result3: Vec<Option<WireType>>,
            pub result4: Vec<Option<WireType>>,
            pub result5: Vec<Option<WireType>>,
            pub numIns: u64,
            pub numBits: i32,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = self.cg.clone();

                let numIns = self.t.numIns as usize;
                let numBits = self.t.numBits;
                let mut result1 = vec![None; numIns];
                let mut result2 = vec![None; numIns];
                let mut result3 = vec![None; numIns];
                let mut result4 = vec![None; numIns];
                let mut result5 = vec![None; numIns];
                let mut inputs1 = generator.createInputWireArray(numIns, &None);
                let mut inputs2 = generator.createInputWireArray(numIns, &None);

                for i in 0..numIns {
                    result1[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.isLessThan(inputs2[i].as_ref().unwrap(), numBits, &None));
                    result2[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.isLessThanOrEqual(inputs2[i].as_ref().unwrap(), numBits, &None));
                    result3[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.isGreaterThan(inputs2[i].as_ref().unwrap(), numBits, &None));
                    result4[i] = inputs1[i].as_ref().map(|x| {
                        x.isGreaterThanOrEqual(inputs2[i].as_ref().unwrap(), numBits, &None)
                    });
                    result5[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.isEqualTo(inputs2[i].as_ref().unwrap(), &None));
                }
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.result1 = result1;
                self.t.result2 = result2;
                self.t.result3 = result3;
                self.t.result4 = result4;
                self.t.result5 = result5;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuea(&self.t.inputs1, &self.t.inVals1);
                evaluator.setWireValuea(&self.t.inputs2, &self.t.inVals2);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            inVals1,
            inVals2,
            result1: vec![None; numIns as usize],
            result2: vec![None; numIns as usize],
            result3: vec![None; numIns as usize],
            result4: vec![None; numIns as usize],
            result5: vec![None; numIns as usize],
            numIns: numIns as u64,
            numBits,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("comparison", t.clone());
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        //		generator.printCircuit();
        evaluator.evaluate(&generator.cg);
        let mut result1 = t.result1.clone();
        let mut result2 = t.result2.clone();
        let mut result3 = t.result3.clone();
        let mut result4 = t.result4.clone();
        let mut result5 = t.result5.clone();
        for i in 0..numIns as usize {
            let mut r = result[i];
            if r == 0 {
                assert_eq!(
                    evaluator.getWireValue(result1[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.getWireValue(result2[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.getWireValue(result3[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.getWireValue(result4[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.getWireValue(result5[i].as_ref().unwrap()),
                    Util::one()
                );
            } else if r == 1 {
                assert_eq!(
                    evaluator.getWireValue(result1[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.getWireValue(result2[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.getWireValue(result3[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.getWireValue(result4[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.getWireValue(result5[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
            } else if r == -1 {
                assert_eq!(
                    evaluator.getWireValue(result1[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.getWireValue(result2[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.getWireValue(result3[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.getWireValue(result4[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.getWireValue(result5[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
            }
        }
    }

    #[test]
    pub fn testBooleanOperations() {
        let mut numIns = Configs.log2_field_prime;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);
        let mut inVals3 = Util::randomBigIntegerArrayi(numIns, 32);
        let numIns = numIns as usize;
        let mut shiftedRightVals = vec![BigInteger::default(); numIns];
        let mut shiftedLeftVals = vec![BigInteger::default(); numIns];
        let mut rotatedRightVals = vec![BigInteger::default(); numIns];
        let mut rotatedLeftVals = vec![BigInteger::default(); numIns];
        let mut xoredVals = vec![BigInteger::default(); numIns];
        let mut oredVals = vec![BigInteger::default(); numIns];
        let mut andedVals = vec![BigInteger::default(); numIns];
        let mut invertedVals = vec![BigInteger::default(); numIns];

        let mut mask = BigInteger::from(2)
            .pow(Configs.log2_field_prime as u32)
            .sub(Util::one());

        for i in 0..numIns {
            shiftedRightVals[i] = inVals1[i].clone().shr(i).rem(Configs.field_prime.clone());
            shiftedLeftVals[i] = inVals1[i]
                .clone()
                .shl(i)
                .bitand(mask.clone())
                .rem(Configs.field_prime.clone());
            rotatedRightVals[i] = BigInteger::from(
                inVals3[i]
                    .to_str_radix(10)
                    .parse::<i64>()
                    .unwrap()
                    .rotate_right((i % 32) as u32)
                    & 0x00000000ffffffff,
            );
            rotatedLeftVals[i] = BigInteger::from(
                inVals3[i]
                    .to_str_radix(10)
                    .parse::<i64>()
                    .unwrap()
                    .rotate_left((i % 32) as u32)
                    & 0x00000000ffffffff,
            );
            xoredVals[i] = inVals1[i]
                .clone()
                .bitxor(inVals2[i].clone())
                .rem(Configs.field_prime.clone());
            oredVals[i] = inVals1[i]
                .clone()
                .bitor(inVals2[i].clone())
                .rem(Configs.field_prime.clone());
            andedVals[i] = inVals1[i]
                .clone()
                .bitand(inVals2[i].clone())
                .rem(Configs.field_prime.clone());
            invertedVals[i] = BigInteger::from(
                !inVals3[i].to_str_radix(10).parse::<i64>().unwrap() & 0x00000000ffffffff,
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub inputs3: Vec<Option<WireType>>,
            pub inVals1: Vec<BigInteger>,
            pub inVals2: Vec<BigInteger>,
            pub inVals3: Vec<BigInteger>,
            pub numIns: u64,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = self.cg.clone();

                let numIns = self.t.numIns as usize;
                let mut inputs1 = generator.createInputWireArray(numIns, &None);
                let mut inputs2 = generator.createInputWireArray(numIns, &None);
                let mut inputs3 = generator.createInputWireArray(numIns, &None);

                let mut shiftedRight = vec![None; numIns];
                let mut shiftedLeft = vec![None; numIns];
                let mut rotatedRight = vec![None; numIns];
                let mut rotatedLeft = vec![None; numIns];
                let mut xored = vec![None; numIns];
                let mut ored = vec![None; numIns];
                let mut anded = vec![None; numIns];
                let mut inverted = vec![None; numIns];

                for i in 0..numIns {
                    shiftedRight[i] = inputs1[i]
                        .clone()
                        .map(|x| x.shiftRight(Configs.log2_field_prime as usize, i, &None));
                    shiftedLeft[i] = inputs1[i]
                        .clone()
                        .map(|x| x.shiftLeft(Configs.log2_field_prime as usize, i, &None));
                    rotatedRight[i] = inputs3[i].clone().map(|x| x.rotateRight(32, i % 32, &None));
                    rotatedLeft[i] = inputs3[i].clone().map(|x| x.rotateLeft(32, i % 32, &None));
                    xored[i] = inputs1[i].clone().map(|x| {
                        x.xorBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    ored[i] = inputs1[i].clone().map(|x| {
                        x.orBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    anded[i] = inputs1[i].clone().map(|x| {
                        x.andBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });

                    inverted[i] = inputs3[i].clone().map(|x| x.invBits(32, &None));
                }

                generator.makeOutputArray(&shiftedRight, &None);
                generator.makeOutputArray(&shiftedLeft, &None);
                generator.makeOutputArray(&rotatedRight, &None);
                generator.makeOutputArray(&rotatedLeft, &None);
                generator.makeOutputArray(&xored, &None);
                generator.makeOutputArray(&ored, &None);
                generator.makeOutputArray(&anded, &None);
                generator.makeOutputArray(&inverted, &None);
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.inputs3 = inputs3;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuea(&self.t.inputs1, &self.t.inVals1);
                evaluator.setWireValuea(&self.t.inputs2, &self.t.inVals2);
                evaluator.setWireValuea(&self.t.inputs3, &self.t.inVals3);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            inputs3: vec![],
            inVals1,
            inVals2,
            inVals3,
            numIns: numIns as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("boolean_operations", t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let numIns = numIns as usize;
        let mut outWires = generator.get_out_wires();
        let (mut i, mut outputIndex) = (0, 0);
        for i in 0..numIns {
            assert_eq!(
                shiftedRightVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                shiftedLeftVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                rotatedRightVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                rotatedLeftVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                xoredVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                oredVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                andedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                invertedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }
    }

    #[test]
    pub fn testAssertion() {
        let mut numIns = 100;
        let mut inVals1 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);
        let mut inVals2 = Util::randomBigIntegerArray(numIns, &Configs.field_prime);
        let numIns = numIns as usize;
        let mut result = vec![];
        result.push(
            inVals1[0]
                .clone()
                .mul(&inVals1[0])
                .rem(&Configs.field_prime),
        );
        for i in 0..numIns {
            result.push(
                inVals1[i]
                    .clone()
                    .mul(&inVals2[i])
                    .rem(&Configs.field_prime),
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub solutions: Vec<Option<WireType>>,
            pub inVals1: Vec<BigInteger>,
            pub inVals2: Vec<BigInteger>,
            pub numIns: u64,
            result: Vec<BigInteger>,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            // provide solutions as witnesses

            fn buildCircuit(&mut self) {
                let mut generator = self.cg.clone();

                let numIns = self.t.numIns as usize;
                let mut inputs1 = WireArray::new(
                    generator.createInputWireArray(numIns, &None),
                    generator.clone().downgrade(),
                );
                let mut inputs2 = WireArray::new(
                    generator.createInputWireArray(numIns, &None),
                    generator.clone().downgrade(),
                );
                let mut solutions = WireArray::new(
                    generator.createProverWitnessWireArray(numIns + 1, &None),
                    generator.clone().downgrade(),
                );
                let result = &self.t.result;
                let prover = crate::impl_prover!(
                                eval( result: Vec<BigInteger>,
                            solutions: WireArray,
                            numIns: usize)  {
                impl Instruction for Prover{
                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                                          evaluator.setWireValue(
                                    self.solutions[0].as_ref().unwrap(),
                                    self.result[0].clone(),
                                );
                                for i in 0..self.numIns {
                                    evaluator.setWireValue(
                                        self.solutions[i + 1].as_ref().unwrap(),
                                        self.result[i + 1].clone(),
                                    );
                                }
                }
                }
                            }
                        );
                generator.specifyProverWitnessComputation(prover);
                // generator.specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
                //     evaluator.setWireValue(solutions[0].as_ref().unwrap(), self.t.result[0].clone());
                //     for i in 0..numIns {
                //         evaluator.setWireValue(
                //             solutions[i + 1].as_ref().unwrap(),
                //             self.t.result[i + 1].clone(),
                //         );
                //     }
                // });
                // {
                //     use zkay_derive::ImplStructNameConfig;
                //     #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
                //     struct Prover {
                //         result: Vec<BigInteger>,
                //         solutions: WireArray,
                //         numIns: usize,
                //     }
                //     impl Instruction for Prover {
                //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                //             evaluator.setWireValue(
                //                 self.solutions[0].as_ref().unwrap(),
                //                 self.result[0].clone(),
                //             );
                //             for i in 0..self.numIns {
                //                 evaluator.setWireValue(
                //                     self.solutions[i + 1].as_ref().unwrap(),
                //                     self.result[i + 1].clone(),
                //                 );
                //             }
                //         }
                //     }
                //     Box::new(Prover {
                //         result: self.t.result.clone(),
                //         solutions: solutions.clone(),
                //         numIns,
                //     })
                // });

                self.addAssertion(
                    inputs1[0].as_ref().unwrap(),
                    inputs1[0].as_ref().unwrap(),
                    solutions[0].as_ref().unwrap(),
                    &None,
                );
                for i in 0..numIns {
                    self.addAssertion(
                        inputs1[i].as_ref().unwrap(),
                        inputs2[i].as_ref().unwrap(),
                        solutions[i + 1].as_ref().unwrap(),
                        &None,
                    );
                }
                let (zero_wire, one_wire) =
                    (self.get_zero_wire().unwrap(), self.get_one_wire().unwrap());
                // constant assertions will not add constraints
                self.addZeroAssertion(&zero_wire, &None);
                self.addOneAssertion(&one_wire, &None);
                self.addAssertion(&zero_wire, &one_wire, &zero_wire, &None);
                self.addAssertion(&one_wire, &one_wire, &one_wire, &None);
                self.addBinaryAssertion(&zero_wire, &None);
                self.addBinaryAssertion(&one_wire, &None);

                // won't add a constraint
                self.addEqualityAssertion(
                    inputs1[0].as_ref().unwrap(),
                    inputs1[0].as_ref().unwrap(),
                    &None,
                );

                // will add a constraint
                self.addEqualityAssertionb(inputs1[0].as_ref().unwrap(), &self.t.inVals1[0], &None);
                self.t.inputs1 = inputs1.asArray().clone();
                self.t.inputs2 = inputs2.asArray().clone();
                // self.t.inputs2=inputs2;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuea(&self.t.inputs1, &self.t.inVals1);
                evaluator.setWireValuea(&self.t.inputs2, &self.t.inVals2);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            solutions: vec![],
            inVals1,
            inVals2,
            numIns: numIns as u64,
            result: result.clone(),
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("assertions", t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg); // no exception will be thrown
        assert_eq!(generator.get_num_of_constraints(), numIns as i32 + 2);
    }
}
