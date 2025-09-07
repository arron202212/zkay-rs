#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator, put_active_circuit_generator,
            },
            circuit_generator::{CGConfigFields, CGInstance},
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
    pub fn test_addition() {
        let mut num_ins = 100;
        let mut in_vals1 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);
        let mut in_vals2 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);

        let mut result = vec![];
        result.push(
            in_vals1[0]
                .clone()
                .add(&in_vals1[1])
                .rem(&CONFIGS.field_prime),
        );
        let mut s = BigInteger::ZERO;
        let num_ins = num_ins as usize;
        for i in 0..num_ins {
            s = s.add(&in_vals1[i]);
        }
        result.push(s.rem(&CONFIGS.field_prime));
        for i in 0..num_ins {
            result.push(
                in_vals1[i]
                    .clone()
                    .add(&in_vals2[i])
                    .rem(&CONFIGS.field_prime),
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: WireArray,
            pub inputs2: WireArray,
            pub in_vals1: Vec<BigInteger>,
            pub in_vals2: Vec<BigInteger>,
            pub num_ins: u64,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let mut generator = &*self;

                let num_ins = self.t.num_ins as usize;
                let mut inputs1 = WireArray::new(
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins),
                    generator.cg_weak(),
                );
                let mut inputs2 = WireArray::new(
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins),
                    generator.cg_weak(),
                );

                let mut result1 = inputs1[0]
                    .clone()
                    .unwrap()
                    .add(inputs1[1].as_ref().unwrap());
                let mut result2 = inputs1.sum_all_elements(&None);
                let mut result_array = inputs1.add_wire_array(&inputs2, inputs1.size(), &None);

                CircuitGenerator::make_output(self.cg(), &result1, &None);
                CircuitGenerator::make_output(self.cg(), &result2, &None);
                CircuitGenerator::make_output_array(self.cg(), result_array.as_array(), &None);
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuea(self.t.inputs1.as_array(), &self.t.in_vals1);
                evaluator.set_wire_valuea(self.t.inputs2.as_array(), &self.t.in_vals2);
            }
        }
        let cg = CircuitGenerator::new("addition");
        let t = CGTest {
            inputs1: WireArray::newi(0, cg.cg_weak()),
            inputs2: WireArray::newi(0, cg.cg_weak()),
            in_vals1,
            in_vals2,
            num_ins: num_ins as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::newc(cg, t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);

        let mut idx = 0;
        for output in generator.get_out_wires() {
            assert_eq!(
                evaluator.get_wire_value(output.as_ref().unwrap()),
                result[idx].clone()
            );
            idx += 1;
        }
        assert_eq!(generator.get_num_of_constraints(), num_ins as i32 + 2);
    }

    #[test]
    pub fn test_multiplication() {
        let mut num_ins = 100;
        let mut in_vals1 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);
        let mut in_vals2 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);

        let mut result = vec![];
        result.push(
            in_vals1[0]
                .clone()
                .mul(&in_vals1[1])
                .rem(&CONFIGS.field_prime),
        );
        let num_ins = num_ins as usize;
        for i in 0..num_ins {
            result.push(
                in_vals1[i]
                    .clone()
                    .mul(&in_vals2[i])
                    .rem(&CONFIGS.field_prime),
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub in_vals1: Vec<BigInteger>,
            pub in_vals2: Vec<BigInteger>,
            pub num_ins: u64,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let mut generator = &*self;

                let num_ins = self.t.num_ins as usize;
                let mut inputs1 = WireArray::new(
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins),
                    generator.cg_weak(),
                );
                let mut inputs2 = WireArray::new(
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins),
                    generator.cg_weak(),
                );

                let mut result1 = inputs1[0]
                    .clone()
                    .unwrap()
                    .mul(inputs1[1].as_ref().unwrap());
                let mut result_array = inputs1.mul_wire_array(&inputs2, num_ins, &None);

                CircuitGenerator::make_output(self.cg(), &result1, &None);
                CircuitGenerator::make_output_array(self.cg(), result_array.as_array(), &None);
                self.t.inputs1 = inputs1.as_array().clone();
                self.t.inputs2 = inputs2.as_array().clone();
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuea(&self.t.inputs1, &self.t.in_vals1);
                evaluator.set_wire_valuea(&self.t.inputs2, &self.t.in_vals2);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            in_vals1,
            in_vals2,
            num_ins: num_ins as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("multiplication", t);

        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let mut idx = 0;
        for output in generator.get_out_wires() {
            assert_eq!(
                evaluator.get_wire_value(output.as_ref().unwrap()),
                result[idx].clone()
            );
            idx += 1;
        }
        assert_eq!(generator.get_num_of_constraints(), num_ins as i32 + 1);
    }

    #[test]
    pub fn test_comparison() {
        let mut num_ins = 10000;
        let mut num_bits = 10;
        let mut in_vals1 = Util::random_big_integer_arrayi(num_ins, num_bits);
        let mut in_vals2 = Util::random_big_integer_arrayi(num_ins, num_bits);
        let num_ins = num_ins as usize;
        let mut result = vec![];
        for i in 0..num_ins {
            let b = in_vals1[i].clone() - in_vals2[i].clone();
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
            pub in_vals1: Vec<BigInteger>,
            pub in_vals2: Vec<BigInteger>,
            pub result1: Vec<Option<WireType>>,
            pub result2: Vec<Option<WireType>>,
            pub result3: Vec<Option<WireType>>,
            pub result4: Vec<Option<WireType>>,
            pub result5: Vec<Option<WireType>>,
            pub num_ins: u64,
            pub num_bits: i32,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let mut generator = &*self;

                let num_ins = self.t.num_ins as usize;
                let num_bits = self.t.num_bits;
                let mut result1 = vec![None; num_ins];
                let mut result2 = vec![None; num_ins];
                let mut result3 = vec![None; num_ins];
                let mut result4 = vec![None; num_ins];
                let mut result5 = vec![None; num_ins];
                let mut inputs1 = CircuitGenerator::create_input_wire_array(self.cg(), num_ins);
                let mut inputs2 = CircuitGenerator::create_input_wire_array(self.cg(), num_ins);

                for i in 0..num_ins {
                    result1[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.is_less_thans(inputs2[i].as_ref().unwrap(), num_bits, &None));

                    result2[i] = inputs1[i].as_ref().map(|x| {
                        x.is_less_than_or_equals(inputs2[i].as_ref().unwrap(), num_bits, &None)
                    });
                    result3[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.is_greater_thans(inputs2[i].as_ref().unwrap(), num_bits, &None));
                    result4[i] = inputs1[i].as_ref().map(|x| {
                        x.is_greater_than_or_equals(inputs2[i].as_ref().unwrap(), num_bits, &None)
                    });
                    result5[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.is_equal_tos(inputs2[i].as_ref().unwrap(), &None));
                }
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.result1 = result1;
                self.t.result2 = result2;
                self.t.result3 = result3;
                self.t.result4 = result4;
                self.t.result5 = result5;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuea(&self.t.inputs1, &self.t.in_vals1);
                evaluator.set_wire_valuea(&self.t.inputs2, &self.t.in_vals2);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            in_vals1,
            in_vals2,
            result1: vec![None; num_ins as usize],
            result2: vec![None; num_ins as usize],
            result3: vec![None; num_ins as usize],
            result4: vec![None; num_ins as usize],
            result5: vec![None; num_ins as usize],
            num_ins: num_ins as u64,
            num_bits,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("comparison", t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        //		generator.print_circuit();
        evaluator.evaluate(&generator.cg);
        let mut result1 = generator.t.result1.clone();
        let mut result2 = generator.t.result2.clone();
        let mut result3 = generator.t.result3.clone();
        let mut result4 = generator.t.result4.clone();
        let mut result5 = generator.t.result5.clone();
        for i in 0..num_ins as usize {
            let mut r = result[i];
            if r == 0 {
                assert_eq!(
                    evaluator.get_wire_value(result1[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.get_wire_value(result2[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.get_wire_value(result3[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.get_wire_value(result4[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.get_wire_value(result5[i].as_ref().unwrap()),
                    Util::one()
                );
            } else if r == 1 {
                assert_eq!(
                    evaluator.get_wire_value(result1[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.get_wire_value(result2[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.get_wire_value(result3[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.get_wire_value(result4[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.get_wire_value(result5[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
            } else if r == -1 {
                assert_eq!(
                    evaluator.get_wire_value(result1[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.get_wire_value(result2[i].as_ref().unwrap()),
                    Util::one()
                );
                assert_eq!(
                    evaluator.get_wire_value(result3[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.get_wire_value(result4[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
                assert_eq!(
                    evaluator.get_wire_value(result5[i].as_ref().unwrap()),
                    BigInteger::ZERO
                );
            }
        }
    }

    #[test]
    pub fn test_boolean_operations() {
        let mut num_ins = CONFIGS.log2_field_prime;
        let mut in_vals1 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);
        let mut in_vals2 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);
        let mut in_vals3 = Util::random_big_integer_arrayi(num_ins, 32);
        let num_ins = num_ins as usize;
        let mut shifted_right_vals = vec![BigInteger::default(); num_ins];
        let mut shifted_left_vals = vec![BigInteger::default(); num_ins];
        let mut rotated_right_vals = vec![BigInteger::default(); num_ins];
        let mut rotated_left_vals = vec![BigInteger::default(); num_ins];
        let mut xored_vals = vec![BigInteger::default(); num_ins];
        let mut ored_vals = vec![BigInteger::default(); num_ins];
        let mut anded_vals = vec![BigInteger::default(); num_ins];
        let mut inverted_vals = vec![BigInteger::default(); num_ins];

        let mut mask = BigInteger::from(2)
            .pow(CONFIGS.log2_field_prime as u32)
            .sub(Util::one());

        for i in 0..num_ins {
            shifted_right_vals[i] = in_vals1[i].clone().shr(i).rem(&CONFIGS.field_prime);
            shifted_left_vals[i] = in_vals1[i]
                .clone()
                .shl(i)
                .bitand(mask.clone())
                .rem(&CONFIGS.field_prime);
            rotated_right_vals[i] = BigInteger::from(
                in_vals3[i]
                    .to_str_radix(10)
                    .parse::<u32>()
                    .unwrap()
                    .rotate_right((i % 32) as u32)
                    & 0x00000000ffffffff,
            );
            rotated_left_vals[i] = BigInteger::from(
                in_vals3[i]
                    .to_str_radix(10)
                    .parse::<u32>()
                    .unwrap()
                    .rotate_left((i % 32) as u32)
                    & 0x00000000ffffffff,
            );
            xored_vals[i] = in_vals1[i]
                .clone()
                .bitxor(in_vals2[i].clone())
                .rem(&CONFIGS.field_prime);
            ored_vals[i] = in_vals1[i]
                .clone()
                .bitor(in_vals2[i].clone())
                .rem(&CONFIGS.field_prime);
            anded_vals[i] = in_vals1[i]
                .clone()
                .bitand(in_vals2[i].clone())
                .rem(&CONFIGS.field_prime);
            inverted_vals[i] = BigInteger::from(
                !in_vals3[i].to_str_radix(10).parse::<u32>().unwrap() & 0x00000000ffffffff,
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub inputs3: Vec<Option<WireType>>,
            pub in_vals1: Vec<BigInteger>,
            pub in_vals2: Vec<BigInteger>,
            pub in_vals3: Vec<BigInteger>,
            pub num_ins: u64,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let mut generator = &*self;

                let num_ins = self.t.num_ins as usize;
                let mut inputs1 = CircuitGenerator::create_input_wire_array(self.cg(), num_ins);
                let mut inputs2 = CircuitGenerator::create_input_wire_array(self.cg(), num_ins);
                let mut inputs3 = CircuitGenerator::create_input_wire_array(self.cg(), num_ins);

                let mut shifted_right = vec![None; num_ins];
                let mut shifted_left = vec![None; num_ins];
                let mut rotated_right = vec![None; num_ins];
                let mut rotated_left = vec![None; num_ins];
                let mut xored = vec![None; num_ins];
                let mut ored = vec![None; num_ins];
                let mut anded = vec![None; num_ins];
                let mut inverted = vec![None; num_ins];

                for i in 0..num_ins {
                    shifted_right[i] = inputs1[i]
                        .clone()
                        .map(|x| x.shift_right(CONFIGS.log2_field_prime as usize, i, &None));
                    shifted_left[i] = inputs1[i]
                        .clone()
                        .map(|x| x.shift_left(CONFIGS.log2_field_prime as usize, i, &None));
                    rotated_right[i] = inputs3[i]
                        .clone()
                        .map(|x| x.rotate_right(32, i % 32, &None));
                    rotated_left[i] = inputs3[i].clone().map(|x| x.rotate_left(32, i % 32, &None));
                    xored[i] = inputs1[i].clone().map(|x| {
                        x.xor_bitwise(
                            inputs2[i].as_ref().unwrap(),
                            CONFIGS.log2_field_prime,
                            &None,
                        )
                    });
                    ored[i] = inputs1[i].clone().map(|x| {
                        x.or_bitwises(
                            inputs2[i].as_ref().unwrap(),
                            CONFIGS.log2_field_prime,
                            &None,
                        )
                    });
                    anded[i] = inputs1[i].clone().map(|x| {
                        x.and_bitwise(
                            inputs2[i].as_ref().unwrap(),
                            CONFIGS.log2_field_prime,
                            &None,
                        )
                    });

                    inverted[i] = inputs3[i].clone().map(|x| x.inv_bits(32, &None));
                }

                CircuitGenerator::make_output_array(self.cg(), &shifted_right, &None);
                CircuitGenerator::make_output_array(self.cg(), &shifted_left, &None);
                CircuitGenerator::make_output_array(self.cg(), &rotated_right, &None);
                CircuitGenerator::make_output_array(self.cg(), &rotated_left, &None);
                CircuitGenerator::make_output_array(self.cg(), &xored, &None);
                CircuitGenerator::make_output_array(self.cg(), &ored, &None);
                CircuitGenerator::make_output_array(self.cg(), &anded, &None);
                CircuitGenerator::make_output_array(self.cg(), &inverted, &None);
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.inputs3 = inputs3;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuea(&self.t.inputs1, &self.t.in_vals1);
                evaluator.set_wire_valuea(&self.t.inputs2, &self.t.in_vals2);
                evaluator.set_wire_valuea(&self.t.inputs3, &self.t.in_vals3);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            inputs3: vec![],
            in_vals1,
            in_vals2,
            in_vals3,
            num_ins: num_ins as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("boolean_operations", t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);
        let num_ins = num_ins as usize;
        let mut out_wires = generator.get_out_wires();
        let (mut i, mut output_index) = (0, 0);
        for i in 0..num_ins {
            assert_eq!(
                shifted_right_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                shifted_left_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                rotated_right_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                rotated_left_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                xored_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                ored_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                anded_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                inverted_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }
    }

    #[test]
    pub fn test_assertions() {
        let mut num_ins = 100;
        let mut in_vals1 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);
        let mut in_vals2 = Util::random_big_integer_array(num_ins, &CONFIGS.field_prime);
        let num_ins = num_ins as usize;
        let mut result = vec![];
        result.push(
            in_vals1[0]
                .clone()
                .mul(&in_vals1[0])
                .rem(&CONFIGS.field_prime),
        );
        for i in 0..num_ins {
            result.push(
                in_vals1[i]
                    .clone()
                    .mul(&in_vals2[i])
                    .rem(&CONFIGS.field_prime),
            );
        }

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub inputs1: Vec<Option<WireType>>,
            pub inputs2: Vec<Option<WireType>>,
            pub solutions: Vec<Option<WireType>>,
            pub in_vals1: Vec<BigInteger>,
            pub in_vals2: Vec<BigInteger>,
            pub num_ins: u64,
            result: Vec<BigInteger>,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            // provide solutions as witnesses

            fn build_circuit(&mut self) {
                let mut generator = &*self;

                let num_ins = self.t.num_ins as usize;
                let mut inputs1 = WireArray::new(
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins),
                    generator.cg_weak(),
                );
                let mut inputs2 = WireArray::new(
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins),
                    generator.cg_weak(),
                );
                let mut solutions = WireArray::new(
                    CircuitGenerator::create_prover_witness_wire_array(
                        self.cg.clone(),
                        num_ins + 1,
                        &None,
                    ),
                    generator.cg_weak(),
                );
                let result = &self.t.result;
                let prover = crate::impl_prover!(
                                                eval( result: Vec<BigInteger>,
                                            solutions: WireArray,
                                            num_ins: usize)  {
                                impl Instruction for Prover{
                                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                                          evaluator.set_wire_value(
                                                    self.solutions[0].as_ref().unwrap(),
                                                    &self.result[0],
                                                );
                                                for i in 0..self.num_ins {
                                                    evaluator.set_wire_value(
                                                        self.solutions[i + 1].as_ref().unwrap(),
                                                        &self.result[i + 1],
                                                    );
                                                }
                Ok(())
                                }
                                }
                                            }
                                        );
                CircuitGenerator::specify_prover_witness_computation(self.cg(), prover);
                // CircuitGenerator::specify_prover_witness_computation(self.cg(),&|evaluator: &mut CircuitEvaluator| {
                //     evaluator.set_wire_value(solutions[0].as_ref().unwrap(), self.t.result[0].clone());
                //     for i in 0..num_ins {
                //         evaluator.set_wire_value(
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
                //         num_ins: usize,
                //     }
                //     impl Instruction for Prover {
                //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                //             evaluator.set_wire_value(
                //                 self.solutions[0].as_ref().unwrap(),
                //                 self.result[0].clone(),
                //             );
                //             for i in 0..self.num_ins {
                //                 evaluator.set_wire_value(
                //                     self.solutions[i + 1].as_ref().unwrap(),
                //                     self.result[i + 1].clone(),
                //                 );
                //             }
                //         }
                //     }
                //     Box::new(Prover {
                //         result: self.t.result.clone(),
                //         solutions: solutions.clone(),
                //         num_ins,
                //     })
                // });

                CircuitGenerator::add_assertion(
                    self.cg(),
                    inputs1[0].as_ref().unwrap(),
                    inputs1[0].as_ref().unwrap(),
                    solutions[0].as_ref().unwrap(),
                    &None,
                );
                for i in 0..num_ins {
                    CircuitGenerator::add_assertion(
                        self.cg(),
                        inputs1[i].as_ref().unwrap(),
                        inputs2[i].as_ref().unwrap(),
                        solutions[i + 1].as_ref().unwrap(),
                        &None,
                    );
                }
                let (zero_wire, one_wire) =
                    (self.get_zero_wire().unwrap(), self.get_one_wire().unwrap());
                // constant assertions will not add constraints
                CircuitGenerator::add_zero_assertion(self.cg(), &zero_wire, &None);
                CircuitGenerator::add_one_assertion(self.cg(), &one_wire, &None);
                CircuitGenerator::add_assertion(
                    self.cg(),
                    &zero_wire,
                    &one_wire,
                    &zero_wire,
                    &None,
                );
                CircuitGenerator::add_assertion(self.cg(), &one_wire, &one_wire, &one_wire, &None);
                CircuitGenerator::add_binary_assertion(self.cg(), &zero_wire, &None);
                CircuitGenerator::add_binary_assertion(self.cg(), &one_wire, &None);

                // won't add a constraint
                CircuitGenerator::add_equality_assertion(
                    self.cg(),
                    inputs1[0].as_ref().unwrap(),
                    inputs1[0].as_ref().unwrap(),
                    &None,
                );

                // will add a constraint
                CircuitGenerator::add_equality_assertionb(
                    self.cg(),
                    inputs1[0].as_ref().unwrap(),
                    &self.t.in_vals1[0],
                    &None,
                );
                self.t.inputs1 = inputs1.as_array().clone();
                self.t.inputs2 = inputs2.as_array().clone();
                // self.t.inputs2=inputs2;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_valuea(&self.t.inputs1, &self.t.in_vals1);
                evaluator.set_wire_valuea(&self.t.inputs2, &self.t.in_vals2);
            }
        }
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            solutions: vec![],
            in_vals1,
            in_vals2,
            num_ins: num_ins as u64,
            result: result.clone(),
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("assertions", t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg); // no exception will be thrown
        assert_eq!(generator.get_num_of_constraints(), num_ins as i32 + 2);
    }
}
