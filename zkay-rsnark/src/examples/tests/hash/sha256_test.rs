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
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
    addToEvaluationQueue, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget::SHA256Gadget;
use crate::util::util::{BigInteger, Util};
use std::ops::{Add, Mul, Shl, Sub};
use zkay_derive::ImplStructNameConfig;
/**
 * Tests SHA256 standard cases.
 *
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn testCase1() {
        let expectedDigest =
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_owned();

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const inputStr: &[u8] = b"";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    CGTest::inputStr.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                )
                .getOutputWires()
                .clone();
                self.makeOutputArray(&digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, _e: &mut CircuitEvaluator) {
                // no input needed
            }
        };
        let t = CGTest { inputWires: vec![] };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test1", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();
        // let evaluator = generator.getCircuitEvaluator();

        let mut outDigest = String::new();
        for w in generator.get_out_wires() {
            outDigest += &Util::padZeros(
                &evaluator.getWireValue(w.as_ref().unwrap()).to_str_radix(16),
                8,
            );
        }
        assert_eq!(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase2() {
        let expectedDigest = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const inputStr: &[u8] = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    CGTest::inputStr.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                )
                .getOutputWires()
                .clone();
                self.makeOutputArray(&digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..CGTest::inputStr.len() {
                    evaluator.setWireValuei(
                        self.t.inputWires[i].as_ref().unwrap(),
                        CGTest::inputStr[i] as i64,
                    );
                }
            }
        };
        let t = CGTest { inputWires: vec![] };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test2", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();
        // let evaluator = generator.getCircuitEvaluator();

        let mut outDigest = String::new();
        for w in generator.get_out_wires() {
            outDigest += &Util::padZeros(
                &evaluator.getWireValue(w.as_ref().unwrap()).to_str_radix(16),
                8,
            );
        }
        assert_eq!(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase3() {
        let expectedDigest =
            "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1".to_owned();

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const inputStr:&[u8] = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    CGTest::inputStr.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                )
                .getOutputWires()
                .clone();
                self.makeOutputArray(&digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..CGTest::inputStr.len() {
                    evaluator.setWireValuei(
                        self.t.inputWires[i].as_ref().unwrap(),
                        CGTest::inputStr[i] as i64,
                    );
                }
            }
        };
        let t = CGTest { inputWires: vec![] };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test3", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();
        // let evaluator = generator.getCircuitEvaluator();

        let mut outDigest = String::new();
        for w in generator.get_out_wires() {
            outDigest += &Util::padZeros(
                &evaluator.getWireValue(w.as_ref().unwrap()).to_str_radix(16),
                8,
            );
        }
        assert_eq!(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase4() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const inputStr: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = self.createInputWireArray(CGTest::inputStr.len(), &None);
                let digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8,
                    CGTest::inputStr.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                )
                .getOutputWires()
                .clone();
                self.makeOutputArray(&digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..CGTest::inputStr.len() {
                    evaluator.setWireValuei(
                        self.t.inputWires[i].as_ref().unwrap(),
                        CGTest::inputStr[i] as i64,
                    );
                }
            }
        };
        let expectedDigest =
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_owned();

        let t = CGTest { inputWires: vec![] };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test4", t);
        generator.generateCircuit();
        let evaluator = generator.evalCircuit().unwrap();
        // let evaluator = generator.getCircuitEvaluator();

        let mut outDigest = String::new();
        for w in generator.get_out_wires() {
            outDigest += &Util::padZeros(
                &evaluator.getWireValue(w.as_ref().unwrap()).to_str_radix(16),
                8,
            );
        }
        assert_eq!(outDigest, expectedDigest);
    }

    #[test]
    pub fn testCase5() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            inputWires: Vec<Option<WireType>>,
            numBytesPerInputWire: usize,
        }
        impl CGTest {
            const inputStr: &[u8] = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let inputWires = self.createInputWireArray(
                    CGTest::inputStr.len() / self.t.numBytesPerInputWire
                        + if CGTest::inputStr.len() % self.t.numBytesPerInputWire != 0 {
                            1
                        } else {
                            0
                        },
                    &None,
                );
                let digest = SHA256Gadget::new(
                    inputWires.clone(),
                    8 * self.t.numBytesPerInputWire,
                    CGTest::inputStr.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                )
                .getOutputWires()
                .clone();
                self.makeOutputArray(&digest, &None);
                self.t.inputWires = inputWires;
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.inputWires.len() {
                    let mut sum = BigInteger::ZERO;
                    for j in i * self.t.numBytesPerInputWire
                        ..CGTest::inputStr
                            .len()
                            .min((i + 1) * self.t.numBytesPerInputWire)
                    {
                        let v = BigInteger::from(CGTest::inputStr[j]);
                        sum = sum.add(v.shl((j % self.t.numBytesPerInputWire) * 8));
                    }
                    evaluator.setWireValue(self.t.inputWires[i].as_ref().unwrap(), &sum);
                }
            }
        };

        let expectedDigest =
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1".to_owned();

        // Testing different settings of the bitWidthPerInputElement parameter
        // wordSize = # of bytes per input wire

        for wordSize in 1..=Configs.log2_field_prime / 8 - 1 {
            let numBytesPerInputWire = wordSize as usize;
            let t = CGTest {
                inputWires: vec![],
                numBytesPerInputWire,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test5", t);

            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();
            // let evaluator = generator.getCircuitEvaluator();

            let mut outDigest = String::new();
            for w in generator.get_out_wires() {
                outDigest += &Util::padZeros(
                    &evaluator.getWireValue(w.as_ref().unwrap()).to_str_radix(16),
                    8,
                );
            }
            assert_eq!(outDigest, expectedDigest);
        }
    }
}
