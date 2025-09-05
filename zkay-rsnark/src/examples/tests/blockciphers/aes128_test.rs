#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{eval::circuit_evaluator::CircuitEvaluator,
    operations::gadget::{Gadget, GadgetConfig},
    structure::{
        circuit_generator::{
            CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
            addToEvaluationQueue, getActiveCircuitGenerator,
        },
        wire_type::WireType,
    }},
    examples::gadgets::blockciphers::{
        aes128_cipher_gadget::{AES128CipherGadget, SBoxOption, atomic_sbox_option},
        sbox::aes_s_box_gadget_optimized2::AESSBoxGadgetOptimized2,
    },
    util::util::{BigInteger, Util},
};

use std::sync::atomic::{self, AtomicU8, Ordering};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use zkay_derive::ImplStructNameConfig;
#[macro_export]
macro_rules! impl_cg_test {
    () => {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 16 bytes
            key: Vec<Option<WireType>>,        // 16 bytes
            ciphertext: Vec<Option<WireType>>, // 16 bytes
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGTest {
            pub fn new(name: &str) -> CircuitGeneratorExtend<Self> {
                CircuitGeneratorExtend::<Self>::new(
                    name,
                    Self {
                        plaintext: vec![],
                        key: vec![],
                        ciphertext: vec![],
                    },
                )
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn aes128_test_case0() {
        let a = Util::parse_big_int(
            "89228104670908091290687385480691397980782275631420279887247541550499959534759064731866521016916693902170178842167218244796073443825711414268411402820183",
        );
        let b = Util::parse_big_int(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617",
        );
        assert!(a % b == BigInteger::ZERO);
    }
    #[test]
    pub fn aes128_test_case1() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 16 bytes
            key: Vec<Option<WireType>>,        // 16 bytes
            ciphertext: Vec<Option<WireType>>, // 16 bytes
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let start = std::time::Instant::now();

                let plaintext = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let key = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let expandedKey = Gadget::<AES128CipherGadget>::expandKey(&key, &self.cg);
                // assert!(!plaintext.is_empty(),"plaintext.is_empty()");
                // println!("=====plaintext.len()======{}",plaintext.len());
                let ciphertext =
                    AES128CipherGadget::new(plaintext.clone(), expandedKey, &None, self.cg())
                        .getOutputWires()
                        .clone();
                CircuitGenerator::makeOutputArray(self.cg(), &ciphertext, &None);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let start = std::time::Instant::now();

                let keyV = Util::parse_big_int_x("2b7e151628aed2a6abf7158809cf4f3c");
                let msgV = Util::parse_big_int_x("ae2d8a571e03ac9c9eb76fac45af8e51");

                let (_, mut keyArray) = keyV.to_bytes_be();
                let (_, mut msgArray) = msgV.to_bytes_be();
                msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                for i in 0..self.t.plaintext.len() {
                    evaluator.setWireValuei(
                        self.t.plaintext[i].as_ref().unwrap(),
                        (msgArray[i] as i64 & 0xff),
                    );
                }
                for i in 0..self.t.key.len() {
                    evaluator.setWireValuei(
                        self.t.key[i].as_ref().unwrap(),
                        (keyArray[i] as i64 & 0xff),
                    );
                }
                println!(
                    "===generateSampleInput===start==elapsed== {:?} ",
                    start.elapsed()
                );
            }
        }
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "ae2d8a571e03ac9c9eb76fac45af8e51"
        // ciphertext: "f5d3d58503b9699de785895a96fdbaaf"

        // testing all available sBox implementations
        let start = std::time::Instant::now();
        for sboxOption in SBoxOption::iter() {
            atomic_sbox_option.store(sboxOption.clone().into(), Ordering::Relaxed);
            let t = CGTest {
                plaintext: vec![],  // 16 bytes
                key: vec![],        // 16 bytes
                ciphertext: vec![], // 16 bytes
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("AES128_Test1_{sboxOption}"), t);

            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();

            let cipherText = generator.get_out_wires();

            let result = Util::parse_big_int_x("f5d3d58503b9699de785895a96fdbaaf");

            let mut resultArray = result.to_bytes_be().1.clone();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assert_eq!(
                    evaluator.getWireValue(cipherText[i].as_ref().unwrap()),
                    BigInteger::from((resultArray[i] as i32 + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn aes128_test_case2() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 16 bytes
            key: Vec<Option<WireType>>,        // 16 bytes
            ciphertext: Vec<Option<WireType>>, // 16 bytes
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plaintext = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let key = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let expandedKey = Gadget::<AES128CipherGadget>::expandKey(&key, &self.cg);
                let ciphertext =
                    AES128CipherGadget::new(plaintext.clone(), expandedKey, &None, self.cg())
                        .getOutputWires()
                        .clone();
                CircuitGenerator::makeOutputArray(self.cg(), &ciphertext, &None);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let keyV = Util::parse_big_int_x("2b7e151628aed2a6abf7158809cf4f3c");
                let msgV = Util::parse_big_int_x("6bc1bee22e409f96e93d7e117393172a");

                let (_, mut keyArray) = keyV.to_bytes_be();
                let (_, mut msgArray) = msgV.to_bytes_be();
                msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                for i in 0..self.t.plaintext.len() {
                    evaluator.setWireValuei(
                        self.t.plaintext[i].as_ref().unwrap(),
                        (msgArray[i] as i64 & 0xff),
                    );
                }
                for i in 0..self.t.key.len() {
                    evaluator.setWireValuei(
                        self.t.key[i].as_ref().unwrap(),
                        (keyArray[i] as i64 & 0xff),
                    );
                }
            }
        }

        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "6bc1bee22e409f96e93d7e117393172a"
        // ciphertext: "3ad77bb40d7a3660a89ecaf32466ef97"

        // testing all available sBox implementations
        for sboxOption in SBoxOption::iter() {
            atomic_sbox_option.store(sboxOption.clone().into(), Ordering::Relaxed);
            let t = CGTest {
                plaintext: vec![],  // 16 bytes
                key: vec![],        // 16 bytes
                ciphertext: vec![], // 16 bytes
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("AES128_Test2_{sboxOption}"), t);

            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();

            let cipherText = generator.get_out_wires();

            let result = Util::parse_big_int_x("3ad77bb40d7a3660a89ecaf32466ef97");

            // expected output:0xf5d3d58503b9699de785895a96fdbaaf

            let (_, mut resultArray) = result.to_bytes_be();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assert_eq!(
                    evaluator.getWireValue(cipherText[i].as_ref().unwrap()),
                    BigInteger::from((resultArray[i] as i32 + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn aes128_test_case3() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 16 bytes
            key: Vec<Option<WireType>>,        // 16 bytes
            ciphertext: Vec<Option<WireType>>, // 16 bytes
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plaintext = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let key = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let expandedKey = Gadget::<AES128CipherGadget>::expandKey(&key, &self.cg);
                let ciphertext =
                    AES128CipherGadget::new(plaintext.clone(), expandedKey, &None, self.cg())
                        .getOutputWires()
                        .clone();
                CircuitGenerator::makeOutputArray(self.cg(), &ciphertext, &None);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let keyV = Util::parse_big_int_x("2b7e151628aed2a6abf7158809cf4f3c");
                let msgV = Util::parse_big_int_x("30c81c46a35ce411e5fbc1191a0a52ef");

                let (_, mut keyArray) = keyV.to_bytes_be();
                let (_, mut msgArray) = msgV.to_bytes_be();
                msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                for i in 0..self.t.plaintext.len() {
                    evaluator.setWireValuei(
                        self.t.plaintext[i].as_ref().unwrap(),
                        msgArray[i] as i64 & 0xff,
                    );
                }
                for i in 0..self.t.key.len() {
                    evaluator
                        .setWireValuei(self.t.key[i].as_ref().unwrap(), keyArray[i] as i64 & 0xff);
                }
            }
        }
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "6bc1bee22e409f96e93d7e117393172a"
        // ciphertext: "3ad77bb40d7a3660a89ecaf32466ef97"

        // testing all available sBox implementations
        for sboxOption in SBoxOption::iter() {
            atomic_sbox_option.store(sboxOption.clone().into(), Ordering::Relaxed);

            let t = CGTest {
                plaintext: vec![],  // 16 bytes
                key: vec![],        // 16 bytes
                ciphertext: vec![], // 16 bytes
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("AES128_Test3_{sboxOption}"), t);
            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();

            let cipherText = generator.get_out_wires();

            let result = Util::parse_big_int_x("43b1cd7f598ece23881b00e3ed030688");

            let (_, mut resultArray) = result.to_bytes_be();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assert_eq!(
                    evaluator.getWireValue(cipherText[i].as_ref().unwrap()),
                    BigInteger::from((resultArray[i] as i32 + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn aes128_test_case4() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 16 bytes
            key: Vec<Option<WireType>>,        // 16 bytes
            ciphertext: Vec<Option<WireType>>, // 16 bytes
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let plaintext = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let key = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let expandedKey = Gadget::<AES128CipherGadget>::expandKey(&key, &self.cg);
                let ciphertext =
                    AES128CipherGadget::new(plaintext.clone(), expandedKey, &None, self.cg())
                        .getOutputWires()
                        .clone();
                CircuitGenerator::makeOutputArray(self.cg(), &ciphertext, &None);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let keyV = Util::parse_big_int_x("2b7e151628aed2a6abf7158809cf4f3c");
                let msgV = Util::parse_big_int_x("f69f2445df4f9b17ad2b417be66c3710");

                let (_, mut keyArray) = keyV.to_bytes_be();
                let (_, mut msgArray) = msgV.to_bytes_be();
                msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                for i in 0..self.t.plaintext.len() {
                    evaluator.setWireValuei(
                        self.t.plaintext[i].as_ref().unwrap(),
                        (msgArray[i] as i64 & 0xff),
                    );
                }
                for i in 0..self.t.key.len() {
                    evaluator.setWireValuei(
                        self.t.key[i].as_ref().unwrap(),
                        (keyArray[i] as i64 & 0xff),
                    );
                }
            }
        }
        // key: "2b7e151628aed2a6abf7158809cf4f3c"
        // plaintext: "30c81c46a35ce411e5fbc1191a0a52ef"
        // ciphertext: "43b1cd7f598ece23881b00e3ed030688"

        // testing all available sBox implementations
        for sboxOption in SBoxOption::iter() {
            atomic_sbox_option.store(sboxOption.clone().into(), Ordering::Relaxed);

            let t = CGTest {
                plaintext: vec![],  // 16 bytes
                key: vec![],        // 16 bytes
                ciphertext: vec![], // 16 bytes
            };
            let mut generator =
                CircuitGeneratorExtend::<CGTest>::new(&format!("AES128_Test4_{sboxOption}"), t);

            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();

            let cipherText = generator.get_out_wires();

            let result = Util::parse_big_int_x("7b0c785e27e8ad3f8223207104725dd4");

            let (_, mut resultArray) = result.to_bytes_be();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assert_eq!(
                    evaluator.getWireValue(cipherText[i].as_ref().unwrap()),
                    BigInteger::from((resultArray[i] as i32 + 256) % 256),
                );
            }
        }
    }

    #[test]
    pub fn testCustomSboxImplementation() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            plaintext: Vec<Option<WireType>>,  // 16 bytes
            key: Vec<Option<WireType>>,        // 16 bytes
            ciphertext: Vec<Option<WireType>>, // 16 bytes
        }

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn buildCircuit(&mut self) {
                let start = std::time::Instant::now();
                let plaintext = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let key = CircuitGenerator::createInputWireArray(self.cg(), 16, &None);
                let expandedKey = Gadget::<AES128CipherGadget>::expandKey(&key, &self.cg);
                let ciphertext =
                    AES128CipherGadget::new(plaintext.clone(), expandedKey, &None, self.cg())
                        .getOutputWires()
                        .clone();
                CircuitGenerator::makeOutputArray(self.cg(), &ciphertext, &None);
                (self.t.plaintext, self.t.key, self.t.ciphertext) = (plaintext, key, ciphertext);
            }

            fn generateSampleInput(&self, evaluator: &mut CircuitEvaluator) {
                let start = std::time::Instant::now();
                let keyV = Util::parse_big_int_x("2b7e151628aed2a6abf7158809cf4f3c");
                let msgV = Util::parse_big_int_x("f69f2445df4f9b17ad2b417be66c3710");

                let (_, mut keyArray) = keyV.to_bytes_be();
                let (_, mut msgArray) = msgV.to_bytes_be();
                msgArray = msgArray[msgArray.len() - 16..msgArray.len()].to_vec();
                keyArray = keyArray[keyArray.len() - 16..keyArray.len()].to_vec();

                for i in 0..self.t.plaintext.len() {
                    evaluator.setWireValuei(
                        self.t.plaintext[i].as_ref().unwrap(),
                        (msgArray[i] as i64 & 0xff),
                    );
                }
                for i in 0..self.t.key.len() {
                    evaluator.setWireValuei(
                        self.t.key[i].as_ref().unwrap(),
                        (keyArray[i] as i64 & 0xff),
                    );
                }
                println!(
                    "===generateSampleInput===start==elapsed== {:?} ",
                    start.elapsed()
                );
            }
        };
        let start = std::time::Instant::now();
        atomic_sbox_option.store(SBoxOption::OPTIMIZED2.into(), Ordering::Relaxed);
        for b in 0..=15 {
            Gadget::<AESSBoxGadgetOptimized2>::set_bit_count(b);
            // AESSBoxGadgetOptimized2::solveLinearSystems();
            let t = CGTest {
                plaintext: vec![],  // 16 bytes
                key: vec![],        // 16 bytes
                ciphertext: vec![], // 16 bytes
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new(
                &format!("AES128_Test_SBox_Parametrization_{b}"),
                t,
            );
            generator.generateCircuit();
            let evaluator = generator.evalCircuit().unwrap();

            let cipherText = generator.get_out_wires();

            let result = Util::parse_big_int_x("7b0c785e27e8ad3f8223207104725dd4");

            let (_, mut resultArray) = result.to_bytes_be();
            resultArray = resultArray[resultArray.len() - 16..resultArray.len()].to_vec();

            for i in 0..16 {
                assert_eq!(
                    evaluator.getWireValue(cipherText[i].as_ref().unwrap()),
                    BigInteger::from((resultArray[i] as i32 + 256) % 256),
                );
            }
        }
    }
}
