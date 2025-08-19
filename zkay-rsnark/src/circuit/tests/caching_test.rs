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
        StructNameConfig,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
                getActiveCircuitGenerator, put_active_circuit_generator,
            },
            circuit_generator::{CGConfigFields, CGInstance},
            wire::{GetWireId, WireConfig},
            wire_type::WireType,
        },
    },
    examples::gadgets::{
        hash::{
            sha256_gadget,
            sha256_gadget::{Base, SHA256Gadget},
        },
        math::{field_division_gadget, field_division_gadget::FieldDivisionGadget},
    },
    util::util::{ARcCell, BigInteger, Util},
};

use std::{
    collections::HashMap,
    ops::{Add, BitAnd, BitOr, BitXor, Mul, Rem, Shl, Shr, Sub},
};

use rccell::{RcCell, WeakCell};
use zkay_derive::ImplStructNameConfig;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCaching1() {
        unsafe { backtrace_on_stack_overflow::enable() };
        let mut numIns = Configs.log2_field_prime.clone();
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
        let mut multipliedVals = vec![BigInteger::default(); numIns];
        let mut addedVals = vec![BigInteger::default(); numIns];

        let mut mask = BigInteger::from(2)
            .pow(Configs.log2_field_prime as u32)
            .sub(Util::one());
        //4215603241   3977511079
        let s = BigInteger::from(3977511079u64);
        let t = BigInteger::from(
            s.to_str_radix(10)
                .parse::<u32>()
                .unwrap()
                .rotate_right((2 % 32) as u32)
                & 0x00000000ffffffffu32,
        );
        println!("==t===={}===s==={}==============", t, s);

        let s = BigInteger::from(3491785456u64);
        let t = BigInteger::from(
            s.to_str_radix(10)
                .parse::<u32>()
                .unwrap()
                .rotate_left((2 % 32) as u32)
                & 0x00000000ffffffff,
        );
        println!("==t===={}===s==={}======rotate_left========", t, s);
        //8181139172870928967080305625624286096015218543390575358010383571265581920620
        //8181139172870928967080305625624286096015218543390575358010383571265581920620===16362278345741857934160611251248572192030437086781150716020767142531163841240==
        for i in 0..numIns {
            shiftedRightVals[i] = inVals1[i].clone().shr(i).rem(&Configs.field_prime);
            // println!("=calc=={i}===shiftedRightVals[i]===={}==={}==",shiftedRightVals[i],inVals1[i]);
            shiftedLeftVals[i] = inVals1[i]
                .clone()
                .shl(i)
                .bitand(&mask)
                .rem(&Configs.field_prime);
            // println!("=******************=={i}===rotatedRightVals[i]===={}==={}==",rotatedRightVals[i],inVals3[i]);
            rotatedRightVals[i] = BigInteger::from(
                inVals3[i]
                    .to_str_radix(10)
                    .parse::<u32>()
                    .unwrap()
                    .rotate_right((i % 32) as u32)
                    & 0x00000000ffffffff,
            );

            // println!(
            //     "==rotatedRightVals[i]===={}===inVals3[i]====={}====={i}=========",
            //     rotatedRightVals[i], inVals3[i]
            // );
            rotatedLeftVals[i] = BigInteger::from(
                inVals3[i]
                    .to_str_radix(10)
                    .parse::<u32>()
                    .unwrap()
                    .rotate_left((i % 32) as u32)
                    & 0x00000000ffffffff,
            );
            // println!(
            //     "==rotatedLeftVals[i]===={}===inVals3[i]====={}====={i}=========",
            //     rotatedLeftVals[i], inVals3[i]
            // );
            xoredVals[i] = inVals1[i]
                .clone()
                .bitxor(&inVals2[i])
                .rem(&Configs.field_prime);
            oredVals[i] = inVals1[i]
                .clone()
                .bitor(&inVals2[i])
                .rem(&Configs.field_prime);
            andedVals[i] = inVals1[i]
                .clone()
                .bitand(&inVals2[i])
                .rem(&Configs.field_prime);
            invertedVals[i] = BigInteger::from(
                !inVals3[i].to_str_radix(10).parse::<u32>().unwrap() & 0x00000000ffffffff,
            );
            multipliedVals[i] = inVals1[i]
                .clone()
                .mul(&inVals2[i])
                .rem(&Configs.field_prime);
            addedVals[i] = inVals1[i]
                .clone()
                .add(&inVals2[i])
                .rem(&Configs.field_prime);
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

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                // println!("=====buildCircuit================={},{}", file!(), line!());
                let mut generator = &*self;

                //println!("=====buildCircuit================={},{}",file!(),line!());
                let numIns = self.t.numIns as usize;
                let mut inputs1 = CircuitGenerator::createInputWireArray(self.cg(), numIns, &None);
                let mut inputs2 = CircuitGenerator::createInputWireArray(self.cg(), numIns, &None);
                let mut inputs3 = CircuitGenerator::createInputWireArray(self.cg(), numIns, &None);
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
                // println!(
                //     "=====buildCircuit=========={numIns}======={},{}",
                //     file!(),
                //     line!()
                // );
                use std::time::Instant;
                let start = Instant::now();
                for i in 0..numIns {
                    shiftedRight[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shiftRight(Configs.log2_field_prime as usize, i, &None));
                    // println!(
                    //     "End shiftRight  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    shiftedLeft[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shiftLeft(Configs.log2_field_prime as usize, i, &None));
                    // println!(
                    //     "End shiftLeft  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    rotatedRight[i] = inputs3[i]
                        .as_ref()
                        .map(|x| x.rotateRight(32, i % 32, &None));
                    // println!(
                    //     "End rotateRight  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    rotatedLeft[i] = inputs3[i].as_ref().map(|x| x.rotateLeft(32, i % 32, &None));
                    // println!(
                    //     "End rotateLeft  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );

                    xored[i] = inputs1[i].as_ref().map(|x| {
                        x.xorBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // println!(
                    //     "End xorBitwise  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    ored[i] = inputs1[i].as_ref().map(|x| {
                        x.orBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // println!(
                    //     "End orBitwise  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    anded[i] = inputs1[i].as_ref().map(|x| {
                        x.andBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // println!(
                    //     "End andBitwise  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    inverted[i] = inputs3[i].as_ref().map(|x| x.invBits(32, &None));
                    // println!("End invBits  Time: {i}=== {} s", start.elapsed().as_secs());
                    multiplied[i] = inputs1[i]
                        .clone()
                        .map(|x| x.mul(inputs2[i].as_ref().unwrap()));
                    // println!("End mul  Time: {i}=== {} s", start.elapsed().as_secs());

                    added[i] = inputs1[i]
                        .clone()
                        .map(|x| x.add(inputs2[i].as_ref().unwrap()));
                    // println!("End  add  Time: {i}=== {} s", start.elapsed().as_secs());
                }
                // println!(
                //     "=====buildCircuit=====*************============={},{}",
                //     file!(),
                //     line!()
                // );

                let mut currentCost = generator.get_num_of_constraints();
                // println!(
                //     "=====buildCircuit========188==********======={},{}",
                //     file!(),
                //     line!()
                // );
                // repeat everything again, and verify that the number of
                // multiplication gates will not be affected
                for i in 0..numIns {
                    shiftedRight[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shiftRight(Configs.log2_field_prime as usize, i, &None));
                    shiftedLeft[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shiftLeft(Configs.log2_field_prime as usize, i, &None));
                    rotatedRight[i] = inputs3[i]
                        .as_ref()
                        .map(|x| x.rotateRight(32, i % 32, &None));
                    rotatedLeft[i] = inputs3[i].as_ref().map(|x| x.rotateLeft(32, i % 32, &None));
                    xored[i] = inputs1[i].as_ref().map(|x| {
                        x.xorBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    ored[i] = inputs1[i].as_ref().map(|x| {
                        x.orBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    anded[i] = inputs1[i].as_ref().map(|x| {
                        x.andBitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    inverted[i] = inputs3[i].as_ref().map(|x| x.invBits(32, &None));
                    multiplied[i] = inputs1[i]
                        .clone()
                        .map(|x| x.mul(inputs2[i].as_ref().unwrap()));
                    added[i] = inputs1[i]
                        .clone()
                        .map(|x| x.add(inputs2[i].as_ref().unwrap()));
                }

                assert_eq!(generator.get_num_of_constraints(), currentCost);
                println!(
                    "=====buildCircuit=====__________*****___________________________________________====*************========{},{}",
                    file!(),
                    line!()
                );
                // repeat binary operations again while changing the order of
                // the operands, and verify that the number of multiplication
                // gates will not be affected
                for i in 0..numIns {
                    xored[i] = inputs2[i].as_ref().map(|x| {
                        x.xorBitwise(
                            inputs1[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // assert_eq!(generator.get_num_of_constraints(), currentCost);
                    ored[i] = inputs2[i].as_ref().map(|x| {
                        x.orBitwise(
                            inputs1[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    anded[i] = inputs2[i].as_ref().map(|x| {
                        x.andBitwise(
                            inputs1[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    multiplied[i] = inputs2[i]
                        .clone()
                        .map(|x| x.mul(inputs1[i].as_ref().unwrap()));
                    added[i] = inputs2[i]
                        .clone()
                        .map(|x| x.add(inputs1[i].as_ref().unwrap()));
                }
                println!(
                    "=====buildCircuit====shiftedRight before====*************=========={},{},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                assert_eq!(generator.get_num_of_constraints(), currentCost);
                // println!(
                //     "=====buildCircuit========*************=========={},{}",
                //     file!(),
                //     line!()
                // );
                CircuitGenerator::makeOutputArray(self.cg(), &shiftedRight, &None);
                println!(
                    "=====buildCircuit===shiftedRight======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &shiftedLeft, &None);
                println!(
                    "=====buildCircuit==shiftedLeft==rotatedRight=before===*************======{},{},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &rotatedRight, &None);
                println!(
                    "=====buildCircuit===rotatedRight======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &rotatedLeft, &None);
                println!(
                    "=====buildCircuit===rotatedLeft======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &xored, &None);
                println!(
                    "=====buildCircuit===xored======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &ored, &None);
                println!(
                    "=====buildCircuit===ored======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &anded, &None);
                println!(
                    "=====buildCircuit===anded======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &inverted, &None);
                println!(
                    "=====buildCircuit===inverted======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &multiplied, &None);
                println!(
                    "=====buildCircuit===multiplied======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &added, &None);
                println!(
                    "=====buildCircuit===added======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                currentCost = generator.get_num_of_constraints();

                // repeat labeling as output (although not really meaningful)
                // and make sure no more constraints are added
                CircuitGenerator::makeOutputArray(self.cg(), &shiftedRight, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &shiftedLeft, &None);
                println!(
                    "=====buildCircuit===rotatedRight=====222222=*************========={},{}",
                    file!(),
                    line!()
                );
                CircuitGenerator::makeOutputArray(self.cg(), &rotatedRight, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &rotatedLeft, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &xored, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &ored, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &anded, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &inverted, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &multiplied, &None);
                CircuitGenerator::makeOutputArray(self.cg(), &added, &None);

                assert_eq!(generator.get_num_of_constraints(), currentCost);
                // println!(
                //     "=====buildCircuit========*************=========={},{}",
                //     file!(),
                //     line!()
                // );
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.inputs3 = inputs3;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                // println!("=====evaluator.getAssignment().len()============{}",evaluator.getAssignment().len());
                // println!("=====1======{}===={}===",&self.t.inputs1.len(), &self.t.inVals1.len());
                // println!("=====2======{}===={}===",&self.t.inputs2.len(), &self.t.inVals2.len());
                // println!("======3====={}===={}===",&self.t.inputs3.len(), &self.t.inVals3.len());
                evaluator.setWireValuea(&self.t.inputs1, &self.t.inVals1);
                evaluator.setWireValuea(&self.t.inputs2, &self.t.inVals2);
                evaluator.setWireValuea(&self.t.inputs3, &self.t.inVals3);
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
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Caching_Test", t);
        // println!("==================={}", line!());
        // let mut generator = arc_cell_new!(generator);
        // put_active_circuit_generator("CGTest", generator.cg());

        // println!("================{}", line!());
        generator.generateCircuit();
        // println!("================={},{}", file!(), line!());
        // let generator = RcCell::new(generator);
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);

        let mut outWires = generator.get_out_wires();
        let mut outputIndex = 0;
        for i in 0..numIns {
            // println!("=check=={i}===shiftedRightVals[i]===={}=={}===",shiftedRightVals[i],outWires[i + outputIndex].as_ref().unwrap().getWireId());
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
            // println!(
            //     "=====rotatedRightVals===+++======{i}==={}={}==",
            //     outWires[i + outputIndex].as_ref().unwrap().name(),
            //     outWires[i + outputIndex].as_ref().unwrap().getWireId()
            // );
            assert_eq!(
                rotatedRightVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        // println!("=349759=={}==",evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap()));
        for i in 0..numIns {
            // println!("={i}==={}==rotatedLeftVals======={}",rotatedLeftVals[i],outWires[i + outputIndex].as_ref().unwrap().getWireId());
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

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                multipliedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
            );
        }

        outputIndex += numIns;
        for i in 0..numIns {
            assert_eq!(
                addedVals[i],
                evaluator.getWireValue(outWires[i + outputIndex].as_ref().unwrap())
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

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = &*self;
                let mut in1 = CircuitGenerator::createInputWire(self.cg(), &None);
                let mut in2 = CircuitGenerator::createInputWire(self.cg(), &None);
                let mut witness1 = CircuitGenerator::createProverWitnessWire(self.cg(), &None);
                let mut witness2 = CircuitGenerator::createProverWitnessWire(self.cg(), &None);

                self.addAssertion(&in1, &in2, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 1);
                self.addAssertion(&in1, &in2, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 1);
                self.addAssertion(&in2, &in1, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 1);

                // since &witness2, is another wire, the constraint should go
                // through
                self.addAssertion(&in1, &in2, &witness2, &None);
                assert_eq!(generator.get_num_of_constraints(), 2);
                self.addAssertion(&in2, &in1, &witness2, &None);
                assert_eq!(generator.get_num_of_constraints(), 2);

                self.addEqualityAssertion(&witness1, &witness2, &None);
                assert_eq!(generator.get_num_of_constraints(), 3);
                self.addEqualityAssertion(&witness2, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 4); // we don't detect
                // similarity here yet

                FieldDivisionGadget::new(in1.clone(), in2.clone(), &None, self.cg());
                assert_eq!(generator.get_num_of_constraints(), 5);
                FieldDivisionGadget::new(in1.clone(), in2.clone(), &None, self.cg());
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
                evaluator.setWireValue(self.t.in1.as_ref().unwrap(), &BigInteger::from(5));
                evaluator.setWireValue(self.t.in2.as_ref().unwrap(), &BigInteger::from(6));
                evaluator.setWireValue(self.t.witness1.as_ref().unwrap(), &BigInteger::from(30));
                evaluator.setWireValue(self.t.witness2.as_ref().unwrap(), &BigInteger::from(30));
            }
        }
        let t = CGTest {
            in1: None,
            in2: None,
            witness1: None,
            witness2: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("assertions", t);
        generator.generateCircuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generateSampleInput(&mut evaluator);
        evaluator.evaluate(&generator.cg);
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

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let mut generator = &*self;
                let inputStr = &self.t.inputStr;
                let mut inputWires =
                    CircuitGenerator::createInputWireArray(self.cg(), inputStr.len(), &None);
                let mut digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires()
                .clone();
                let mut numOfConstraintsBefore = generator.get_num_of_constraints();
                digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires()
                .clone();
                digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires()
                .clone();
                digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires()
                .clone();
                digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires()
                .clone();
                digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires()
                .clone();

                // verify that the number of constraints match
                assert_eq!(numOfConstraintsBefore, generator.get_num_of_constraints());

                // do a small change and verify that number changes
                let mut in2 = inputWires.clone();
                in2[0] = in2[1].clone();
                SHA256Gadget::new(
                    in2,
                    8,
                    inputStr.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .getOutputWires();
                assert!(numOfConstraintsBefore < generator.get_num_of_constraints());

                CircuitGenerator::makeOutputArray(self.cg(), &digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for (i, c) in self.t.inputStr.bytes().enumerate() {
                    evaluator.setWireValuei(self.t.inputWires[i].as_ref().unwrap(), c as i64);
                }
            }
        }
        let t = CGTest {
            inputWires: vec![],
            inputStr: inputStr.to_owned(),
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test4", t);
        generator.generateCircuit();

        let mut evaluator = generator.evalCircuit().unwrap();

        let mut outDigest = String::new();
        for w in generator.get_out_wires() {
            outDigest.push_str(&Util::padZeros(
                &evaluator.getWireValue(w.as_ref().unwrap()).to_str_radix(16),
                8,
            ));
        }
        assert_eq!(outDigest, expectedDigest);
    }
}
