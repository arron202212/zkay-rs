#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        config::config::CONFIGS,
        eval::circuit_evaluator::CircuitEvaluator,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CGInstance, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire_type::WireType,
        },
    },
    examples::gadgets::hash::sha256_gadget::{Base, SHA256Gadget},
    util::util::{BigInteger, Util},
};
use std::ops::{Add, Mul, Shl, Sub};
use zkay_derive::ImplStructNameConfig;

//  * Tests SHA256 standard cases.

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn sha256_test_case1() {
        let expected_digest =
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_owned();

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const input_str: &[u8] = b"";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    CGTest::input_str.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                CircuitGenerator::make_output_array(self.cg(), &digest);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, _e: &mut CircuitEvaluator) {
                // no input needed
            }
        };
        let t = CGTest {
            input_wires: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test1", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let mut out_digest = String::new();
        for w in generator.get_out_wires() {
            out_digest += &Util::pad_zeros(
                &evaluator
                    .get_wire_value(w.as_ref().unwrap())
                    .to_str_radix(16),
                8,
            );
        }
        assert_eq!(out_digest, expected_digest);
    }

    #[test]
    pub fn sha256_test_case2() {
        let expected_digest = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const input_str: &[u8] = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    CGTest::input_str.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                CircuitGenerator::make_output_array(self.cg(), &digest);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..CGTest::input_str.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_wires[i].as_ref().unwrap(),
                        CGTest::input_str[i] as i64,
                    );
                }
            }
        };
        let t = CGTest {
            input_wires: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test2", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let mut out_digest = String::new();
        for w in generator.get_out_wires() {
            out_digest += &Util::pad_zeros(
                &evaluator
                    .get_wire_value(w.as_ref().unwrap())
                    .to_str_radix(16),
                8,
            );
        }
        assert_eq!(out_digest, expected_digest);
    }

    #[test]
    pub fn sha256_test_case3() {
        let expected_digest =
            "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1".to_owned();

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const input_str:&[u8] = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    CGTest::input_str.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                CircuitGenerator::make_output_array(self.cg(), &digest);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..CGTest::input_str.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_wires[i].as_ref().unwrap(),
                        CGTest::input_str[i] as i64,
                    );
                }
            }
        };
        let t = CGTest {
            input_wires: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test3", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let mut out_digest = String::new();
        for w in generator.get_out_wires() {
            out_digest += &Util::pad_zeros(
                &evaluator
                    .get_wire_value(w.as_ref().unwrap())
                    .to_str_radix(16),
                8,
            );
        }
        assert_eq!(out_digest, expected_digest);
    }

    #[test]
    pub fn sha256_test_case4() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
        }
        impl CGTest {
            const input_str: &[u8] = b"abc";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires =
                    CircuitGenerator::create_input_wire_array(self.cg(), CGTest::input_str.len());
                let digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    CGTest::input_str.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                CircuitGenerator::make_output_array(self.cg(), &digest);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..CGTest::input_str.len() {
                    evaluator.set_wire_valuei(
                        self.t.input_wires[i].as_ref().unwrap(),
                        CGTest::input_str[i] as i64,
                    );
                }
            }
        };
        let expected_digest =
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_owned();

        let t = CGTest {
            input_wires: vec![],
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test4", t);
        generator.generate_circuit();
        let evaluator = generator.eval_circuit().unwrap();

        let mut out_digest = String::new();
        for w in generator.get_out_wires() {
            out_digest += &Util::pad_zeros(
                &evaluator
                    .get_wire_value(w.as_ref().unwrap())
                    .to_str_radix(16),
                8,
            );
        }
        assert_eq!(out_digest, expected_digest);
    }

    #[test]
    pub fn sha256_test_case5() {
        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            input_wires: Vec<Option<WireType>>,
            num_bytes_per_input_wire: usize,
        }
        impl CGTest {
            const input_str: &[u8] = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        }
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let input_wires = CircuitGenerator::create_input_wire_array(
                    self.cg(),
                    CGTest::input_str.len() / self.t.num_bytes_per_input_wire
                        + if CGTest::input_str.len() % self.t.num_bytes_per_input_wire != 0 {
                            1
                        } else {
                            0
                        },
                );
                let digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8 * self.t.num_bytes_per_input_wire,
                    CGTest::input_str.len(),
                    false,
                    true,
                    &None,
                    self.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                CircuitGenerator::make_output_array(self.cg(), &digest);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for i in 0..self.t.input_wires.len() {
                    let mut sum = BigInteger::ZERO;
                    for j in i * self.t.num_bytes_per_input_wire
                        ..CGTest::input_str
                            .len()
                            .min((i + 1) * self.t.num_bytes_per_input_wire)
                    {
                        let v = BigInteger::from(CGTest::input_str[j]);
                        sum = sum.add(v.shl((j % self.t.num_bytes_per_input_wire) * 8));
                    }
                    evaluator.set_wire_value(self.t.input_wires[i].as_ref().unwrap(), &sum);
                }
            }
        }

        let expected_digest =
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1".to_owned();

        // Testing different settings of the bit_width_per_input_element parameter
        // word_size = # of bytes per input wire

        for word_size in 1..CONFIGS.log2_field_prime / 8 {
            let num_bytes_per_input_wire = word_size as usize;
            let t = CGTest {
                input_wires: vec![],
                num_bytes_per_input_wire,
            };
            let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test5", t);

            generator.generate_circuit();
            let evaluator = generator.eval_circuit().unwrap();

            let mut out_digest = generator
                .get_out_wires()
                .into_iter()
                .map(|w| {
                    Util::pad_zeros(
                        &evaluator
                            .get_wire_value(w.as_ref().unwrap())
                            .to_str_radix(16),
                        8,
                    )
                })
                .collect::<String>();
            assert_eq!(out_digest, expected_digest);
        }
    }
}
