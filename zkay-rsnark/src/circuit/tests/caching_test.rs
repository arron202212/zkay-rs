#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::CGConfigFields;
use crate::circuit::structure::circuit_generator::put_active_circuit_generator;
use crate::circuit::structure::circuit_generator::{CircuitGenerator, getActiveCircuitGenerator};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget;
use crate::examples::gadgets::hash::sha256_gadget::SHA256Gadget;
use crate::examples::gadgets::math::field_division_gadget;
use crate::examples::gadgets::math::field_division_gadget::FieldDivisionGadget;

use crate::arc_cell_new;
use crate::util::util::ARcCell;
use crate::util::util::{BigInteger, Util};
use rccell::RcCell;

use std::collections::HashMap;
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Rem, Shl, Shr, Sub};
use zkay_derive::ImplStructNameConfig;
pub struct CachingTest;
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCaching1() {
        let mut numIns = Configs.log2_field_prime.clone();
        let mut inVals1 = Util::randomBigIntegerArray(numIns, Configs.field_prime.clone());
        let mut inVals2 = Util::randomBigIntegerArray(numIns, Configs.field_prime.clone());
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
        let mut multipliedVals = vec![BigInteger::default(); numIns];
        let mut addedVals = vec![BigInteger::default(); numIns];

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
            multipliedVals[i] = inVals1[i]
                .clone()
                .mul(inVals2[i].clone())
                .rem(Configs.field_prime.clone());
            addedVals[i] = inVals1[i]
                .clone()
                .add(inVals2[i].clone())
                .rem(Configs.field_prime.clone());
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

        //crate::impl_circuit_generator_config_fields_for!(CircuitGenerator<CGTest>);
        crate::impl_struct_name_for!(CircuitGenerator<CGTest>);
        impl CGConfig for CircuitGenerator<CGTest> {
            fn buildCircuit(&mut self) {
                println!("=====buildCircuit================={},{}", file!(), line!());
                let mut generator = getActiveCircuitGenerator().unwrap();
                let mut generator = generator.lock();
                //println!("=====buildCircuit================={},{}",file!(),line!());
                let numIns = self.t.numIns as usize;
                let mut inputs1 = generator.createInputWireArray(numIns, &None);
                let mut inputs2 = generator.createInputWireArray(numIns, &None);
                let mut inputs3 = generator.createInputWireArray(numIns, &None);
                //println!("=====buildCircuit================={},{}",file!(),line!());
                let mut shiftedRight = vec![None; numIns];
                let mut shiftedLeft = vec![None; numIns];
                let mut rotatedRight = vec![None; numIns];
                let mut rotatedLeft = vec![None; numIns];
                //println!("=====buildCircuit================={},{}",file!(),line!());
                let mut xored = vec![None; numIns];
                let mut ored = vec![None; numIns];
                let mut anded = vec![None; numIns];
                let mut inverted = vec![None; numIns];

                let mut multiplied = vec![None; numIns];
                let mut added = vec![None; numIns];
                println!(
                    "=====buildCircuit=========={numIns}======={},{}",
                    file!(),
                    line!()
                );
                for i in 0..numIns {
                    println!(
                        "=====buildCircuit=====i==={i}========={},{}",
                        file!(),
                        line!()
                    );
                    shiftedRight[i] = inputs1[i]
                        .clone()
                        .map(|x| x.shiftRight(Configs.log2_field_prime as usize, i, &None));
                    //println!("=====buildCircuit================={},{}",file!(),line!());

                    shiftedLeft[i] = inputs1[i]
                        .clone()
                        .map(|x| x.shiftLeft(Configs.log2_field_prime as usize, i, &None));
                    //println!("=====buildCircuit================={},{}",file!(),line!());

                    rotatedRight[i] = inputs3[i].clone().map(|x| x.rotateRight(32, i % 32, &None));
                    //println!("=====buildCircuit================={},{}",file!(),line!());

                    rotatedLeft[i] = inputs3[i].clone().map(|x| x.rotateLeft(32, i % 32, &None));
                    //println!("=====buildCircuit================={},{}",file!(),line!());

                    xored[i] = inputs1[i].clone().map(|x| {
                        x.xorBitwise(inputs2[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    //println!("=====buildCircuit=====*********************============{},{}",file!(),line!());

                    ored[i] = inputs1[i].clone().map(|x| {
                        x.orBitwise(inputs2[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    //println!("=====buildCircuit======*************============{},{}",file!(),line!());

                    anded[i] = inputs1[i].clone().map(|x| {
                        x.andBitwise(inputs2[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    //println!("=====buildCircuit======*************============{},{}",file!(),line!());

                    inverted[i] = inputs3[i].clone().map(|x| x.invBits(32, &None));
                    //println!("=====buildCircuit=======*************==========={},{}",file!(),line!());

                    multiplied[i] = inputs1[i]
                        .clone()
                        .map(|x| x.mul(inputs2[i].clone().unwrap()));
                    println!(
                        "=====buildCircuit======*************============{},{}",
                        file!(),
                        line!()
                    );

                    added[i] = inputs1[i]
                        .clone()
                        .map(|x| x.add(inputs2[i].clone().unwrap()));
                }
                println!(
                    "=====buildCircuit=====*************============={},{}",
                    file!(),
                    line!()
                );

                let mut currentCost = generator.get_num_of_constraints();
                println!(
                    "=====buildCircuit========188==********======={},{}",
                    file!(),
                    line!()
                );
                // repeat everything again, and verify that the number of
                // multiplication gates will not be affected
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
                        x.xorBitwise(inputs2[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    ored[i] = inputs1[i].clone().map(|x| {
                        x.orBitwise(inputs2[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    anded[i] = inputs1[i].clone().map(|x| {
                        x.andBitwise(inputs2[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    inverted[i] = inputs3[i].clone().map(|x| x.invBits(32, &None));
                    multiplied[i] = inputs1[i]
                        .clone()
                        .map(|x| x.mul(inputs2[i].clone().unwrap()));
                    added[i] = inputs1[i]
                        .clone()
                        .map(|x| x.add(inputs2[i].clone().unwrap()));
                }

                assert!(generator.get_num_of_constraints() == currentCost);
                println!(
                    "=====buildCircuit=========219*************========{},{}",
                    file!(),
                    line!()
                );
                // repeat binary operations again while changing the order of
                // the operands, and verify that the number of multiplication
                // gates will not be affected
                for i in 0..numIns {
                    xored[i] = inputs2[i].clone().map(|x| {
                        x.xorBitwise(inputs1[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    ored[i] = inputs2[i].clone().map(|x| {
                        x.orBitwise(inputs1[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    anded[i] = inputs2[i].clone().map(|x| {
                        x.andBitwise(inputs1[i].clone().unwrap(), Configs.log2_field_prime, &None)
                    });
                    multiplied[i] = inputs2[i]
                        .clone()
                        .map(|x| x.mul(inputs1[i].clone().unwrap()));
                    added[i] = inputs2[i]
                        .clone()
                        .map(|x| x.add(inputs1[i].clone().unwrap()));
                }

                assert!(generator.get_num_of_constraints() == currentCost);
                //println!("=====buildCircuit========*************=========={},{}",file!(),line!());
                generator.makeOutputArray(shiftedRight.clone(), &None);
                generator.makeOutputArray(shiftedLeft.clone(), &None);
                generator.makeOutputArray(rotatedRight.clone(), &None);
                generator.makeOutputArray(rotatedLeft.clone(), &None);
                generator.makeOutputArray(xored.clone(), &None);
                generator.makeOutputArray(ored.clone(), &None);
                generator.makeOutputArray(anded.clone(), &None);
                generator.makeOutputArray(inverted.clone(), &None);
                generator.makeOutputArray(multiplied.clone(), &None);
                generator.makeOutputArray(added.clone(), &None);

                currentCost = generator.get_num_of_constraints();
                println!(
                    "=====buildCircuit=========*************========={},{}",
                    file!(),
                    line!()
                );
                // repeat labeling as output (although not really meaningful)
                // and make sure no more constraints are added
                generator.makeOutputArray(shiftedRight, &None);
                generator.makeOutputArray(shiftedLeft, &None);
                generator.makeOutputArray(rotatedRight, &None);
                generator.makeOutputArray(rotatedLeft, &None);
                generator.makeOutputArray(xored, &None);
                generator.makeOutputArray(ored, &None);
                generator.makeOutputArray(anded, &None);
                generator.makeOutputArray(inverted, &None);
                generator.makeOutputArray(multiplied, &None);
                generator.makeOutputArray(added, &None);

                assert!(generator.get_num_of_constraints() == currentCost);
                println!(
                    "=====buildCircuit========*************=========={},{}",
                    file!(),
                    line!()
                );
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.inputs3 = inputs3;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValuea(self.t.inputs1.clone(), self.t.inVals1.clone());
                evaluator.setWireValuea(self.t.inputs2.clone(), self.t.inVals2.clone());
                evaluator.setWireValuea(self.t.inputs3.clone(), self.t.inVals3.clone());
            }
        }
        //println!("{}",line!());
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            inputs3: vec![],
            inVals1,
            inVals2,
            inVals3,
            numIns: numIns as u64,
        };
        let mut generator = CircuitGenerator::<CGTest>::new("Caching_Test", t);
        println!("{}", line!());
        let mut generator = arc_cell_new!(generator);
        put_active_circuit_generator("CGTest", generator.clone());
        let mut generator = generator.lock();
        println!("{}", line!());
        generator.generateCircuit();
        println!("{},{}", file!(), line!());
        let mut evaluator = CircuitEvaluator::new("CGTest");
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate();

        let mut outWires = generator.get_out_wires();
        let (mut i, mut outputIndex) = (0, 0);
        for i in 0..numIns {
            assert_eq!(
                shiftedRightVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                shiftedLeftVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                rotatedRightVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                rotatedLeftVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }
        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                xoredVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                oredVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                andedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }
        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                invertedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                multipliedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                addedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].clone().unwrap().clone())
            );
        }
    }

    #[test]
    pub fn testAssertionCache() {
        // make sure we remove some of the clear duplicate assertions
        // and most importantly, no assertions are removed

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub in1: Option<WireType>,
            pub in2: Option<WireType>,
            pub witness1: Option<WireType>,
            pub witness2: Option<WireType>,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGenerator<CGTest>);
        crate::impl_struct_name_for!(CircuitGenerator<CGTest>);
        impl CGConfig for CircuitGenerator<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = getActiveCircuitGenerator().unwrap();
                let mut generator = generator.lock();
                let mut in1 = generator.createInputWire(&None);
                let mut in2 = generator.createInputWire(&None);
                let mut witness1 = generator.createProverWitnessWire(&None);
                let mut witness2 = generator.createProverWitnessWire(&None);

                self.addAssertion(in1.clone(), in2.clone(), witness1.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 1);
                self.addAssertion(in1.clone(), in2.clone(), witness1.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 1);
                self.addAssertion(in2.clone(), in1.clone(), witness1.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 1);

                // since witness2 is another wire, the constraint should go
                // through
                self.addAssertion(in1.clone(), in2.clone(), witness2.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 2);
                self.addAssertion(in2.clone(), in1.clone(), witness2.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 2);

                self.addEqualityAssertion(witness1.clone(), witness2.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 3);
                self.addEqualityAssertion(witness2.clone(), witness1.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 4); // we don't detect
                // similarity here yet

                FieldDivisionGadget::new(in1.clone(), in2.clone(), &None);
                assert_eq!(generator.get_num_of_constraints(), 5);
                FieldDivisionGadget::new(in1.clone(), in2.clone(), &None);
                // since this operation is implemented externally, it's not easy
                // to filter it, because everytime a witness wire is introduced
                // by the gadget. To eliminate such similar operations, the
                // gadget itself needs to take care of it.
                assert_eq!(generator.get_num_of_constraints(), 6);
                self.t.in1 = Some(in1);
                self.t.in2 = Some(in2);
                self.t.witness1 = Some(witness1);
                self.t.witness2 = Some(witness2);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.setWireValue(self.t.in1.clone().unwrap(), BigInteger::from(5));
                evaluator.setWireValue(self.t.in2.clone().unwrap(), BigInteger::from(6));
                evaluator.setWireValue(self.t.witness1.clone().unwrap(), BigInteger::from(30));
                evaluator.setWireValue(self.t.witness2.clone().unwrap(), BigInteger::from(30));
            }
        }
        let t = CGTest {
            in1: None,
            in2: None,
            witness1: None,
            witness2: None,
        };
        let mut generator = CircuitGenerator::<CGTest>::new("assertions", t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest");
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate();
    }

    #[test]
    pub fn testMultiSHA256Calls() {
        // testing multiple unncessary calls to SHA256

        let mut inputStr = "abc";
        let mut expectedDigest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputWires: Vec<Option<WireType>>,
            pub inputStr: String,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGenerator<CGTest>);
        crate::impl_struct_name_for!(CircuitGenerator<CGTest>);
        impl CGConfig for CircuitGenerator<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = getActiveCircuitGenerator().unwrap();
                let mut generator = generator.lock();
                let inputStr = &self.t.inputStr;
                let mut inputWires = generator.createInputWireArray(inputStr.len(), &None);
                let mut digest =
                    SHA256Gadget::new(inputWires.clone(), 8, inputStr.len(), false, true, &None)
                        .getOutputWires();
                let mut numOfConstraintsBefore = generator.get_num_of_constraints();
                digest =
                    SHA256Gadget::new(inputWires.clone(), 8, inputStr.len(), false, true, &None)
                        .getOutputWires();
                digest =
                    SHA256Gadget::new(inputWires.clone(), 8, inputStr.len(), false, true, &None)
                        .getOutputWires();
                digest =
                    SHA256Gadget::new(inputWires.clone(), 8, inputStr.len(), false, true, &None)
                        .getOutputWires();
                digest =
                    SHA256Gadget::new(inputWires.clone(), 8, inputStr.len(), false, true, &None)
                        .getOutputWires();
                digest =
                    SHA256Gadget::new(inputWires.clone(), 8, inputStr.len(), false, true, &None)
                        .getOutputWires();

                // verify that the number of constraints match
                assert_eq!(numOfConstraintsBefore, generator.get_num_of_constraints());

                // do a small change and verify that number changes
                let mut in2 = inputWires.clone();
                in2[0] = in2[1].clone();
                SHA256Gadget::new(in2, 8, inputStr.len(), false, true, &None).getOutputWires();
                assert!(numOfConstraintsBefore < generator.get_num_of_constraints());

                generator.makeOutputArray(digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, e: &mut CircuitEvaluator) {
                for (i, c) in self.t.inputStr.bytes().enumerate() {
                    e.setWireValuei(self.t.inputWires[i].clone().unwrap(), c as i64);
                }
            }
        }
        let t = CGTest {
            inputWires: vec![],
            inputStr: inputStr.to_owned(),
        };
        let mut generator = CircuitGenerator::<CGTest>::new("SHA2_Test4", t);
        generator.generateCircuit();

        let mut evaluator = generator.evalCircuit();

        let mut outDigest = String::new();
        for w in generator.get_out_wires() {
            outDigest.push_str(&Util::padZeros(
                evaluator.getWireValue(w.clone().unwrap()).to_str_radix(16),
                8,
            ));
        }
        assert_eq!(outDigest, expectedDigest);
    }
}
