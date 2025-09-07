#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
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
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator, put_active_circuit_generator,
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
    pub fn test_caching1() {
        unsafe { backtrace_on_stack_overflow::enable() };
        let mut num_ins = Configs.log2_field_prime.clone();
        let mut in_vals1 = Util::random_big_integer_array(num_ins, &Configs.field_prime);
        let mut in_vals2 = Util::random_big_integer_array(num_ins, &Configs.field_prime);
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
        let mut multiplied_vals = vec![BigInteger::default(); num_ins];
        let mut added_vals = vec![BigInteger::default(); num_ins];

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
        for i in 0..num_ins {
            shifted_right_vals[i] = in_vals1[i].clone().shr(i).rem(&Configs.field_prime);
            // println!("=calc=={i}===shifted_right_vals[i]===={}==={}==",shifted_right_vals[i],in_vals1[i]);
            shifted_left_vals[i] = in_vals1[i]
                .clone()
                .shl(i)
                .bitand(&mask)
                .rem(&Configs.field_prime);
            // println!("=******************=={i}===rotated_right_vals[i]===={}==={}==",rotated_right_vals[i],in_vals3[i]);
            rotated_right_vals[i] = BigInteger::from(
                in_vals3[i]
                    .to_str_radix(10)
                    .parse::<u32>()
                    .unwrap()
                    .rotate_right((i % 32) as u32)
                    & 0x00000000ffffffff,
            );

            // println!(
            //     "==rotated_right_vals[i]===={}===in_vals3[i]====={}====={i}=========",
            //     rotated_right_vals[i], in_vals3[i]
            // );
            rotated_left_vals[i] = BigInteger::from(
                in_vals3[i]
                    .to_str_radix(10)
                    .parse::<u32>()
                    .unwrap()
                    .rotate_left((i % 32) as u32)
                    & 0x00000000ffffffff,
            );
            // println!(
            //     "==rotated_left_vals[i]===={}===in_vals3[i]====={}====={i}=========",
            //     rotated_left_vals[i], in_vals3[i]
            // );
            xored_vals[i] = in_vals1[i]
                .clone()
                .bitxor(&in_vals2[i])
                .rem(&Configs.field_prime);
            ored_vals[i] = in_vals1[i]
                .clone()
                .bitor(&in_vals2[i])
                .rem(&Configs.field_prime);
            anded_vals[i] = in_vals1[i]
                .clone()
                .bitand(&in_vals2[i])
                .rem(&Configs.field_prime);
            inverted_vals[i] = BigInteger::from(
                !in_vals3[i].to_str_radix(10).parse::<u32>().unwrap() & 0x00000000ffffffff,
            );
            multiplied_vals[i] = in_vals1[i]
                .clone()
                .mul(&in_vals2[i])
                .rem(&Configs.field_prime);
            added_vals[i] = in_vals1[i]
                .clone()
                .add(&in_vals2[i])
                .rem(&Configs.field_prime);
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

        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                // println!("=====build_circuit================={},{}", file!(), line!());
                let mut generator = &*self;

                //println!("=====build_circuit================={},{}",file!(),line!());
                let num_ins = self.t.num_ins as usize;
                let mut inputs1 =
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins, &None);
                let mut inputs2 =
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins, &None);
                let mut inputs3 =
                    CircuitGenerator::create_input_wire_array(self.cg(), num_ins, &None);
                //println!("=====build_circuit================={},{}",file!(),line!());
                let mut shifted_right = vec![None; num_ins];
                let mut shifted_left = vec![None; num_ins];
                let mut rotated_right = vec![None; num_ins];
                let mut rotated_left = vec![None; num_ins];
                //println!("=====build_circuit================={},{}",file!(),line!());
                let mut xored = vec![None; num_ins];
                let mut ored = vec![None; num_ins];
                let mut anded = vec![None; num_ins];
                let mut inverted = vec![None; num_ins];

                let mut multiplied = vec![None; num_ins];
                let mut added = vec![None; num_ins];
                // println!(
                //     "=====build_circuit=========={num_ins}======={},{}",
                //     file!(),
                //     line!()
                // );
                use std::time::Instant;
                let start = Instant::now();
                for i in 0..num_ins {
                    shifted_right[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shift_right(Configs.log2_field_prime as usize, i, &None));
                    // println!(
                    //     "End shift_right  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    shifted_left[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shift_left(Configs.log2_field_prime as usize, i, &None));
                    // println!(
                    //     "End shift_left  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    rotated_right[i] = inputs3[i]
                        .as_ref()
                        .map(|x| x.rotate_right(32, i % 32, &None));
                    // println!(
                    //     "End rotate_right  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    rotated_left[i] = inputs3[i]
                        .as_ref()
                        .map(|x| x.rotate_left(32, i % 32, &None));
                    // println!(
                    //     "End rotate_left  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );

                    xored[i] = inputs1[i].as_ref().map(|x| {
                        x.xor_bitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // println!(
                    //     "End xor_bitwise  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    ored[i] = inputs1[i].as_ref().map(|x| {
                        x.or_bitwises(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // println!(
                    //     "End or_bitwises  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    anded[i] = inputs1[i].as_ref().map(|x| {
                        x.and_bitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // println!(
                    //     "End and_bitwise  Time: {i}=== {} s",
                    //     start.elapsed().as_secs()
                    // );
                    inverted[i] = inputs3[i].as_ref().map(|x| x.inv_bits(32, &None));
                    // println!("End inv_bits  Time: {i}=== {} s", start.elapsed().as_secs());
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
                //     "=====build_circuit=====*************============={},{}",
                //     file!(),
                //     line!()
                // );

                let mut current_cost = generator.get_num_of_constraints();
                // println!(
                //     "=====build_circuit========188==********======={},{}",
                //     file!(),
                //     line!()
                // );
                // repeat everything again, and verify that the number of
                // multiplication gates will not be affected
                for i in 0..num_ins {
                    shifted_right[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shift_right(Configs.log2_field_prime as usize, i, &None));
                    shifted_left[i] = inputs1[i]
                        .as_ref()
                        .map(|x| x.shift_left(Configs.log2_field_prime as usize, i, &None));
                    rotated_right[i] = inputs3[i]
                        .as_ref()
                        .map(|x| x.rotate_right(32, i % 32, &None));
                    rotated_left[i] = inputs3[i]
                        .as_ref()
                        .map(|x| x.rotate_left(32, i % 32, &None));
                    xored[i] = inputs1[i].as_ref().map(|x| {
                        x.xor_bitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    ored[i] = inputs1[i].as_ref().map(|x| {
                        x.or_bitwises(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    anded[i] = inputs1[i].as_ref().map(|x| {
                        x.and_bitwise(
                            inputs2[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    inverted[i] = inputs3[i].as_ref().map(|x| x.inv_bits(32, &None));
                    multiplied[i] = inputs1[i]
                        .clone()
                        .map(|x| x.mul(inputs2[i].as_ref().unwrap()));
                    added[i] = inputs1[i]
                        .clone()
                        .map(|x| x.add(inputs2[i].as_ref().unwrap()));
                }

                assert_eq!(generator.get_num_of_constraints(), current_cost);
                println!(
                    "=====build_circuit=====__________*****___________________________________________====*************========{},{}",
                    file!(),
                    line!()
                );
                // repeat binary operations again while changing the order of
                // the operands, and verify that the number of multiplication
                // gates will not be affected
                for i in 0..num_ins {
                    xored[i] = inputs2[i].as_ref().map(|x| {
                        x.xor_bitwise(
                            inputs1[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    // assert_eq!(generator.get_num_of_constraints(), current_cost);
                    ored[i] = inputs2[i].as_ref().map(|x| {
                        x.or_bitwises(
                            inputs1[i].as_ref().unwrap(),
                            Configs.log2_field_prime,
                            &None,
                        )
                    });
                    anded[i] = inputs2[i].as_ref().map(|x| {
                        x.and_bitwise(
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
                    "=====build_circuit====shifted_right before====*************=========={},{},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                assert_eq!(generator.get_num_of_constraints(), current_cost);
                // println!(
                //     "=====build_circuit========*************=========={},{}",
                //     file!(),
                //     line!()
                // );
                CircuitGenerator::make_output_array(self.cg(), &shifted_right, &None);
                println!(
                    "=====build_circuit===shifted_right======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &shifted_left, &None);
                println!(
                    "=====build_circuit==shifted_left==rotated_right=before===*************======{},{},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &rotated_right, &None);
                println!(
                    "=====build_circuit===rotated_right======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &rotated_left, &None);
                println!(
                    "=====build_circuit===rotated_left======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &xored, &None);
                println!(
                    "=====build_circuit===xored======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &ored, &None);
                println!(
                    "=====build_circuit===ored======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &anded, &None);
                println!(
                    "=====build_circuit===anded======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &inverted, &None);
                println!(
                    "=====build_circuit===inverted======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &multiplied, &None);
                println!(
                    "=====build_circuit===multiplied======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &added, &None);
                println!(
                    "=====build_circuit===added======*************===={}====={},{}",
                    self.get_num_wires(),
                    file!(),
                    line!()
                );
                current_cost = generator.get_num_of_constraints();

                // repeat labeling as output (although not really meaningful)
                // and make sure no more constraints are added
                CircuitGenerator::make_output_array(self.cg(), &shifted_right, &None);
                CircuitGenerator::make_output_array(self.cg(), &shifted_left, &None);
                println!(
                    "=====build_circuit===rotated_right=====222222=*************========={},{}",
                    file!(),
                    line!()
                );
                CircuitGenerator::make_output_array(self.cg(), &rotated_right, &None);
                CircuitGenerator::make_output_array(self.cg(), &rotated_left, &None);
                CircuitGenerator::make_output_array(self.cg(), &xored, &None);
                CircuitGenerator::make_output_array(self.cg(), &ored, &None);
                CircuitGenerator::make_output_array(self.cg(), &anded, &None);
                CircuitGenerator::make_output_array(self.cg(), &inverted, &None);
                CircuitGenerator::make_output_array(self.cg(), &multiplied, &None);
                CircuitGenerator::make_output_array(self.cg(), &added, &None);

                assert_eq!(generator.get_num_of_constraints(), current_cost);
                // println!(
                //     "=====build_circuit========*************=========={},{}",
                //     file!(),
                //     line!()
                // );
                self.t.inputs1 = inputs1;
                self.t.inputs2 = inputs2;
                self.t.inputs3 = inputs3;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                // println!("=====evaluator.get_assignment().len()============{}",evaluator.get_assignment().len());
                // println!("=====1======{}===={}===",&self.t.inputs1.len(), &self.t.in_vals1.len());
                // println!("=====2======{}===={}===",&self.t.inputs2.len(), &self.t.in_vals2.len());
                // println!("======3====={}===={}===",&self.t.inputs3.len(), &self.t.in_vals3.len());
                evaluator.set_wire_valuea(&self.t.inputs1, &self.t.in_vals1);
                evaluator.set_wire_valuea(&self.t.inputs2, &self.t.in_vals2);
                evaluator.set_wire_valuea(&self.t.inputs3, &self.t.in_vals3);
            }
        }
        //println!("{}",line!());
        let t = CGTest {
            inputs1: vec![],
            inputs2: vec![],
            inputs3: vec![],
            in_vals1,
            in_vals2,
            in_vals3,
            num_ins: num_ins as u64,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("Caching_Test", t);
        // println!("==================={}", line!());
        // let mut generator = arc_cell_new!(generator);
        // put_active_circuit_generator("CGTest", generator.cg());

        // println!("================{}", line!());
        generator.generate_circuit();
        // println!("================={},{}", file!(), line!());
        // let generator = RcCell::new(generator);
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);

        let mut out_wires = generator.get_out_wires();
        let mut output_index = 0;
        for i in 0..num_ins {
            // println!("=check=={i}===shifted_right_vals[i]===={}=={}===",shifted_right_vals[i],out_wires[i + output_index].as_ref().unwrap().get_wire_id());
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
            // println!(
            //     "=====rotated_right_vals===+++======{i}==={}={}==",
            //     out_wires[i + output_index].as_ref().unwrap().name(),
            //     out_wires[i + output_index].as_ref().unwrap().get_wire_id()
            // );
            assert_eq!(
                rotated_right_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        // println!("=349759=={}==",evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap()));
        for i in 0..num_ins {
            // println!("={i}==={}==rotated_left_vals======={}",rotated_left_vals[i],out_wires[i + output_index].as_ref().unwrap().get_wire_id());
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

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                multiplied_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }

        output_index += num_ins;
        for i in 0..num_ins {
            assert_eq!(
                added_vals[i],
                evaluator.get_wire_value(out_wires[i + output_index].as_ref().unwrap())
            );
        }
    }

    #[test]
    pub fn test_assertion_cache() {
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
            fn build_circuit(&mut self) {
                let mut generator = &*self;
                let mut in1 = CircuitGenerator::create_input_wire(self.cg(), &None);
                let mut in2 = CircuitGenerator::create_input_wire(self.cg(), &None);
                let mut witness1 = CircuitGenerator::create_prover_witness_wire(self.cg(), &None);
                let mut witness2 = CircuitGenerator::create_prover_witness_wire(self.cg(), &None);

                CircuitGenerator::add_assertion(self.cg(), &in1, &in2, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 1);
                CircuitGenerator::add_assertion(self.cg(), &in1, &in2, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 1);
                CircuitGenerator::add_assertion(self.cg(), &in2, &in1, &witness1, &None);
                assert_eq!(generator.get_num_of_constraints(), 1);

                // since &witness2, is another wire, the constraint should go
                // through
                CircuitGenerator::add_assertion(self.cg(), &in1, &in2, &witness2, &None);
                assert_eq!(generator.get_num_of_constraints(), 2);
                CircuitGenerator::add_assertion(self.cg(), &in2, &in1, &witness2, &None);
                assert_eq!(generator.get_num_of_constraints(), 2);

                CircuitGenerator::add_equality_assertion(self.cg(), &witness1, &witness2, &None);
                assert_eq!(generator.get_num_of_constraints(), 3);
                CircuitGenerator::add_equality_assertion(self.cg(), &witness2, &witness1, &None);
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

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                evaluator.set_wire_value(self.t.in1.as_ref().unwrap(), &BigInteger::from(5));
                evaluator.set_wire_value(self.t.in2.as_ref().unwrap(), &BigInteger::from(6));
                evaluator.set_wire_value(self.t.witness1.as_ref().unwrap(), &BigInteger::from(30));
                evaluator.set_wire_value(self.t.witness2.as_ref().unwrap(), &BigInteger::from(30));
            }
        }
        let t = CGTest {
            in1: None,
            in2: None,
            witness1: None,
            witness2: None,
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("assertions", t);
        generator.generate_circuit();
        let mut evaluator = CircuitEvaluator::new("CGTest", &generator.cg);
        generator.generate_sample_input(&mut evaluator);
        evaluator.evaluate(&generator.cg);
    }

    #[test]
    pub fn test_multi_sha256_calls() {
        // testing multiple unncessary calls to SHA256

        let mut input_str = "abc";
        let mut expected_digest =
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

        #[derive(Debug, Clone, ImplStructNameConfig)]
        struct CGTest {
            pub input_wires: Vec<Option<WireType>>,
            pub input_str: String,
        }

        //crate::impl_circuit_generator_config_fields_for!(CircuitGeneratorExtend<CGTest>);
        crate::impl_struct_name_for!(CircuitGeneratorExtend<CGTest>);
        impl CGConfig for CircuitGeneratorExtend<CGTest> {
            fn build_circuit(&mut self) {
                let mut generator = &*self;
                let input_str = &self.t.input_str;
                let mut input_wires =
                    CircuitGenerator::create_input_wire_array(self.cg(), input_str.len(), &None);
                let mut digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                let mut num_of_constraints_before = generator.get_num_of_constraints();
                digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();
                digest = SHA256Gadget::new(
                    input_wires.clone(),
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires()
                .clone();

                // verify that the number of constraints match
                assert_eq!(
                    num_of_constraints_before,
                    generator.get_num_of_constraints()
                );

                // do a small change and verify that number changes
                let mut in2 = input_wires.clone();
                in2[0] = in2[1].clone();
                SHA256Gadget::new(
                    in2,
                    8,
                    input_str.len(),
                    false,
                    true,
                    &None,
                    generator.cg(),
                    Base,
                )
                .get_output_wires();
                assert!(num_of_constraints_before < generator.get_num_of_constraints());

                CircuitGenerator::make_output_array(self.cg(), &digest, &None);
                self.t.input_wires = input_wires;
            }

            fn generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
                for (i, c) in self.t.input_str.bytes().enumerate() {
                    evaluator.set_wire_valuei(self.t.input_wires[i].as_ref().unwrap(), c as i64);
                }
            }
        }
        let t = CGTest {
            input_wires: vec![],
            input_str: input_str.to_owned(),
        };
        let mut generator = CircuitGeneratorExtend::<CGTest>::new("SHA2_Test4", t);
        generator.generate_circuit();

        let mut evaluator = generator.eval_circuit().unwrap();

        let mut out_digest = String::new();
        for w in generator.get_out_wires() {
            out_digest.push_str(&Util::pad_zeros(
                &evaluator
                    .get_wire_value(w.as_ref().unwrap())
                    .to_str_radix(16),
                8,
            ));
        }
        assert_eq!(out_digest, expected_digest);
    }
}
