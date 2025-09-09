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
        InstanceOf,
        {
            config::config::CONFIGS,
            eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
            structure::{
                circuit_generator::{
                    CGConfig, CGConfigFields, CircuitGenerator, add_to_evaluation_queue,
                    get_active_circuit_generator,
                },
                constant_wire,
                wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
                wire_array::WireArray,
                wire_ops::{AddWire, MulWire, SubWire},
                wire_type::WireType,
            },
        },
    },
    util::util::{
        ARcCell, {BigInteger, Util},
    },
};
use num_bigint::Sign;
use num_traits::Signed;
use rccell::{RcCell, WeakCell};
use serde::{Serialize, de::DeserializeOwned};
use serde_closure::{Fn, FnMut, FnOnce};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Rem, Shr, Sub},
    sync::Arc,
};
use zkay_derive::ImplStructNameConfig;

//  * An auxiliary class that handles the operations of long integers, such as the
//  * ones used in RSA operations. It applies some of the long integer
//  * optimizations from xjsnark (to appear). This is a preliminary version. More
//  * Other features and detailed tests will be added in the future.
//  *
//  * Usage examples exist in the RSA examples gadgets.

// pub type BigInteger = String;

#[derive(Debug, Clone, Hash)]
pub struct LongElement {
    pub array: Vec<Option<WireType>>,
    pub current_bitwidth: Vec<u64>,
    pub current_max_values: Vec<BigInteger>,
    pub bits: Option<WireArray>,
    pub generator: WeakCell<CircuitGenerator>,
    // pub generators: CircuitGenerator,
}
impl LongElement {
    // Should be declared as final, but left non-for testing purposes.
    // Don't change in the middle of circuit generation.
    // This represents the size of smaller chunks used to represent long
    // elements
    pub const CHUNK_BITWIDTH: i32 = 120;

    pub fn newa(mut bits: WireArray, generator: WeakCell<CircuitGenerator>) -> Self {
        let generators = generator.clone().upgrade().unwrap();
        let (array, current_max_values, current_bitwidth) = if Self::CHUNK_BITWIDTH
            >= bits.size() as i32
        {
            (
                vec![Some(bits.pack_as_bits_with_to(bits.size()))],
                vec![Util::compute_max_value(bits.size() as u64)],
                vec![bits.size() as u64],
            )
        } else {
            let mut max_chunk_val = Util::compute_max_value(Self::CHUNK_BITWIDTH as u64);
            let mut max_last_chunk_val = max_chunk_val.clone();
            let size = bits.size() as i32;
            if size % Self::CHUNK_BITWIDTH != 0 {
                bits = bits.adjust_length(
                    None,
                    (size + (Self::CHUNK_BITWIDTH - size % Self::CHUNK_BITWIDTH)) as usize,
                );
                max_last_chunk_val = Util::compute_max_value((size % Self::CHUNK_BITWIDTH) as u64);
            }
            let mut array = vec![None; bits.size() / Self::CHUNK_BITWIDTH as usize];
            let mut current_max_values = vec![BigInteger::default(); array.len()];
            let mut current_bitwidth = vec![0u64; array.len()];

            for i in 0..array.len() {
                array[i] = Some(
                    WireArray::new(
                        bits.as_array()[i * Self::CHUNK_BITWIDTH as usize
                            ..(i + 1) * Self::CHUNK_BITWIDTH as usize]
                            .to_vec(),
                        generator.clone(),
                    )
                    .pack_as_bits(),
                );
                if i == array.len() - 1 {
                    current_max_values[i] = max_last_chunk_val.clone();
                    current_bitwidth[i] = max_last_chunk_val.bits() as u64;
                } else {
                    current_max_values[i] = max_chunk_val.clone();
                    current_bitwidth[i] = max_chunk_val.bits() as u64;
                }
            }
            (array, current_max_values, current_bitwidth)
        };
        assert!(!current_bitwidth.get(0).is_some_and(|&v| v == 16), "newa");
        Self {
            array,
            current_max_values,
            current_bitwidth,
            bits: Some(bits),
            generator,
            // generators,
        }
    }

    pub fn new(
        w: Vec<Option<WireType>>,
        current_bitwidth: Vec<u64>,
        generator: WeakCell<CircuitGenerator>,
    ) -> Self {
        // let generators = generator.clone().upgrade().unwrap().borrow().clone();
        let mut current_max_values = vec![BigInteger::ZERO; w.len()];
        for i in 0..w.len() {
            current_max_values[i] = Util::compute_max_value(current_bitwidth[i]);
        }
        assert!(!current_bitwidth.get(0).is_some_and(|&v| v == 16), "new");
        Self {
            array: w,
            current_bitwidth,
            current_max_values,
            bits: None,
            generator,
            // generators,
        }
    }
    pub fn generator(&self) -> RcCell<CircuitGenerator> {
        // ARcCell<dyn CGConfig + Send + Sync>
        // get_active_circuit_generator().unwrap()
        // &self.generators
        self.generator.clone().upgrade().unwrap()
    }
    pub fn make_output(&mut self, desc: &Option<String>) {
        let mut generator = self.generator();
        // let generators = self.generator.clone().upgrade().unwrap();
        for w in self.get_array() {
            CircuitGenerator::make_output_with_option(generator.clone(), w.as_ref().unwrap(), desc);
        }
    }

    //A long element representing a constant.

    pub fn newb(chunks: Vec<BigInteger>, generator: WeakCell<CircuitGenerator>) -> Self {
        let mut current_bitwidth = vec![0; chunks.len()];
        for i in 0..chunks.len() {
            current_bitwidth[i] = chunks[i].bits();
        }
        let generatorss = generator.clone().upgrade().unwrap();
        // let generators = generatorss.borrow().clone();

        assert!(!current_bitwidth.get(0).is_some_and(|&v| v == 16), "newb");
        Self {
            array: CircuitGenerator::create_constant_wire_array(
                generatorss.clone(),
                &chunks,
                &None,
            ),
            current_max_values: chunks,
            current_bitwidth,
            bits: None,
            generator,
            // generators,
        }
    }

    pub fn neww(
        w: Vec<Option<WireType>>,
        current_max_values: Vec<BigInteger>,
        generator: WeakCell<CircuitGenerator>,
    ) -> Self {
        let mut current_bitwidth = vec![0; w.len()];
        for i in 0..w.len() {
            current_bitwidth[i] = current_max_values[i].bits();
        }
        // assert!(!current_bitwidth.get(0).is_some_and(|&v|v==16),"neww");
        // let generators = generator.clone().upgrade().unwrap().borrow().clone();
        Self {
            array: w,
            current_max_values,
            current_bitwidth,
            bits: None,
            generator,
            // generators,
        }
    }

    pub fn add_overflow_check(&self, o: &Self) -> bool {
        self.current_max_values
            .iter()
            .zip(&o.current_max_values)
            .any(|(a, b)| a + b >= CONFIGS.field_prime)
    }

    pub fn mul_overflow_check(&self, o: &Self) -> bool {
        let length = self.array.len() + o.array.len() - 1;
        let mut overflow = false;
        let mut new_max_values = vec![BigInteger::ZERO; length];
        for i in 0..self.array.len() {
            for j in 0..o.array.len() {
                new_max_values[i + j] = new_max_values[i + j].clone().add(
                    self.current_max_values[i]
                        .clone()
                        .mul(&o.current_max_values[j]),
                );
            }
        }
        for i in 0..length {
            if new_max_values[i] >= CONFIGS.field_prime {
                overflow = true;
                break;
            }
        }
        overflow
    }

    fn is_constant(&self) -> bool {
        self.array
            .iter()
            .all(|v| v.as_ref().unwrap().instance_of("ConstantWire"))
    }

    pub fn get_size(&self) -> usize {
        self.array.len()
    }

    pub fn align(&self, total_num_chunks: usize) -> Self {
        let generator = self.generator();
        let mut new_array = self.array[..total_num_chunks].to_vec();
        let zero_wire = generator.get_zero_wire();
        for i in 0..new_array.len() {
            if new_array[i].is_none() {
                new_array[i] = zero_wire.clone();
            }
        }
        let mut new_max_values = vec![BigInteger::ZERO; total_num_chunks];
        new_max_values[..total_num_chunks.min(self.current_max_values.len())]
            .clone_from_slice(&self.current_max_values);
        let mut max_aligned_chunk_value = Util::compute_max_value(Self::CHUNK_BITWIDTH as u64);

        for i in 0..total_num_chunks {
            if new_max_values[i].bits() > Self::CHUNK_BITWIDTH as u64 {
                let chunk_bits = new_array[i]
                    .as_ref()
                    .unwrap()
                    .get_bit_wiresi(new_max_values[i].bits());
                let chunk_bits = chunk_bits.as_array();
                new_array[i] = Some(
                    WireArray::new(
                        chunk_bits[..Self::CHUNK_BITWIDTH as usize].to_vec(),
                        self.generator.clone(),
                    )
                    .pack_as_bits(),
                );
                let mut rem = WireArray::new(
                    chunk_bits[Self::CHUNK_BITWIDTH as usize..new_max_values[i].bits() as usize]
                        .to_vec(),
                    self.generator.clone(),
                )
                .pack_as_bits();
                if i != total_num_chunks - 1 {
                    new_max_values[i + 1] = new_max_values[i]
                        .clone()
                        .shr(Self::CHUNK_BITWIDTH)
                        .add(&new_max_values[i + 1]);
                    new_array[i + 1] = Some(rem.add(new_array[i + 1].as_ref().unwrap()));
                }
                new_max_values[i] = max_aligned_chunk_value.clone();
            }
        }
        LongElement::neww(new_array, new_max_values, self.generator.clone())
    }

    // This method extracts (some of) the bit wires corresponding to a long
    // element based on the total_bitwidth argument.
    // If total_bitwidth is -1, all bits are returned.
    // See restrict_bitwidth for restricting the bitwidth of all the long element
    // chunks

    pub fn get_bitsi(&mut self, total_bitwidth: i32) -> WireArray {
        if let Some(bits) = &self.bits {
            return bits.adjust_length(
                None,
                if total_bitwidth == -1 {
                    bits.size()
                } else {
                    total_bitwidth as usize
                },
            );
        }
        if self.array.len() == 1 {
            self.bits = Some(
                self.array[0]
                    .as_ref()
                    .unwrap()
                    .get_bit_wiresi(self.current_max_values[0].bits()),
            );
            return self.bits.as_ref().unwrap().adjust_length(
                None,
                if total_bitwidth == -1 {
                    self.bits.as_ref().unwrap().size()
                } else {
                    total_bitwidth as usize
                },
            );
        }
        if total_bitwidth <= Self::CHUNK_BITWIDTH && total_bitwidth >= 0 {
            let out = self.array[0]
                .as_ref()
                .unwrap()
                .get_bit_wiresi(self.current_max_values[0].bits());
            return out.adjust_length(None, total_bitwidth as usize);
        }

        let mut limit = total_bitwidth as usize;
        let max_val = self.get_max_val(Self::CHUNK_BITWIDTH);

        let mut bit_wires = if total_bitwidth != -1 {
            vec![None; total_bitwidth as usize]
        } else {
            limit = max_val.bits() as usize;
            vec![None; max_val.bits() as usize]
        };

        let new_length = (self.get_max_val(Self::CHUNK_BITWIDTH).bits() as f64 * 1.0
            / Self::CHUNK_BITWIDTH as f64)
            .ceil() as usize;
        let mut new_array = vec![None; new_length];
        let mut new_max_values = vec![BigInteger::ZERO; new_length];

        new_max_values[0..self.current_max_values.len()].clone_from_slice(&self.current_max_values);
        new_array[0..self.array.len()].clone_from_slice(&self.array);
        let mut idx = 0;
        let mut chunk_index = 0;
        while idx < limit && chunk_index < new_length {
            let (bits, bitwidth) = (
                new_max_values[chunk_index].bits(),
                Self::CHUNK_BITWIDTH as u64,
            );
            let chunk_bits = new_array[chunk_index]
                .as_ref()
                .unwrap()
                .get_bit_wiresi(bits.max(bitwidth));
            let chunk_bits = chunk_bits.as_array();
            let aligned_chunk_bits;
            if new_max_values[chunk_index].bits() > Self::CHUNK_BITWIDTH as u64 {
                aligned_chunk_bits = &chunk_bits[..Self::CHUNK_BITWIDTH as usize];
                let rem = WireArray::new(
                    chunk_bits[Self::CHUNK_BITWIDTH as usize
                        ..new_max_values[chunk_index].bits() as usize]
                        .to_vec(),
                    self.generator.clone(),
                )
                .pack_as_bits();

                if chunk_index != new_array.len() - 1 {
                    new_max_values[chunk_index + 1] = new_max_values[chunk_index]
                        .clone()
                        .shr(Self::CHUNK_BITWIDTH)
                        .add(new_max_values[chunk_index + 1].clone());
                    new_array[chunk_index + 1] =
                        Some(rem.add(new_array[chunk_index + 1].as_ref().unwrap()));
                }
            } else {
                aligned_chunk_bits = chunk_bits;
            }
            let len = aligned_chunk_bits.len().min(limit - idx);
            bit_wires[idx..idx + len].clone_from_slice(&aligned_chunk_bits[..len]);
            chunk_index += 1;
            idx += aligned_chunk_bits.len();
        }
        let out = WireArray::new(bit_wires, self.generator.clone());
        if limit >= max_val.bits() as usize {
            self.bits = Some(out.adjust_length(None, max_val.bits() as usize));
        }
        out
    }

    pub fn get_max_val(&self, bitwidth: i32) -> BigInteger {
        Util::group(&self.current_max_values, bitwidth)
    }

    fn multiply_polys(ai_vals: Vec<BigInteger>, bi_vals: Vec<BigInteger>) -> Vec<BigInteger> {
        let mut solution = vec![BigInteger::ZERO; ai_vals.len() + bi_vals.len() - 1];

        for i in 0..ai_vals.len() {
            for j in 0..bi_vals.len() {
                solution[i + j] = solution[i + j]
                    .clone()
                    .add(ai_vals[i].clone().mul(bi_vals[j].clone()))
                    .rem(&CONFIGS.field_prime);
            }
        }
        solution
    }

    pub fn mux_bit(&self, other: &Self, w: &WireType) -> Self {
        let length = self.array.len().max(other.array.len());
        let mut new_array = vec![None; length];
        let mut new_max_values = vec![BigInteger::ZERO; length];
        let zero_wire = self.generator().get_zero_wire().unwrap();
        for i in 0..length {
            let b1 = if i < self.array.len() {
                self.current_max_values[i].clone()
            } else {
                BigInteger::ZERO
            };
            let b2 = if i < other.array.len() {
                other.current_max_values[i].clone()
            } else {
                BigInteger::ZERO
            };
            new_max_values[i] = if b1 > b2 { b1 } else { b2 };

            let w1 = if i < self.array.len() {
                self.array[i].clone().unwrap()
            } else {
                zero_wire.clone()
            };
            let w2 = if i < other.array.len() {
                other.array[i].clone().unwrap()
            } else {
                zero_wire.clone()
            };
            new_array[i] = Some(w1.clone().add(w.clone().mul(w2.clone().sub(w1))));
            if new_array[i].as_ref().unwrap().instance_of("ConstantWire") {
                new_max_values[i] = new_array[i]
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .get_constant();
            }
        }
        LongElement::neww(new_array, new_max_values, self.generator.clone())
    }

    pub fn check_non_zero(&self) -> WireType {
        let mut wire_non_zero = vec![None; self.array.len()];
        for i in 0..self.array.len() {
            wire_non_zero[i] = self.array[i].as_ref().map(|x| x.check_non_zero());
        }
        WireArray::new(wire_non_zero, self.generator.clone())
            .sum_all_elements()
            .check_non_zero()
    }

    pub fn get_array(&self) -> &Vec<Option<WireType>> {
        &self.array
    }

    pub fn get_current_bitwidth(&self) -> Vec<u64> {
        self.current_bitwidth.clone()
    }

    pub fn get_current_max_values(&self) -> Vec<BigInteger> {
        self.current_max_values.clone()
    }

    pub fn get_bits(&self) -> Option<WireArray> {
        self.bits.clone()
    }

    pub fn get_constant(&self, bitwidth_per_chunk: i32) -> Option<BigInteger> {
        if self
            .array
            .iter()
            .any(|v| v.as_ref().unwrap().instance_of("ConstantWire"))
        {
            return None;
        }
        let constants: Vec<_> = self
            .array
            .iter()
            .map(|v| {
                v.as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .get_constant()
            })
            .collect();
        Some(Util::group(&constants, bitwidth_per_chunk))
    }

    // This asserts that the current bitwidth conditions are satisfied
    pub fn restrict_bitwidth(&self) {
        if !self.is_aligned() {
            println!(
                "Warning [restrict_bitwidth()]: Might want to align before checking bitwidth constraints"
            );
            if CONFIGS.print_stack_trace_at_warnings {
                // Thread.dumpStack();
                //println!("Thread.dumpStack()");
            }
        }
        for i in 0..self.array.len() {
            self.array[i]
                .as_ref()
                .unwrap()
                .restrict_bit_length(self.current_bitwidth[i]);
        }
    }

    pub fn is_aligned(&self) -> bool {
        let mut check = true;
        for i in 0..self.array.len() {
            check &= self.current_bitwidth[i] <= Self::CHUNK_BITWIDTH as u64;
        }
        check
    }

    pub fn assert_equality_naive(&mut self, a: &mut Self) {
        let bits1 = a.get_bitsi(a.get_max_val(Self::CHUNK_BITWIDTH).bits() as i32);
        let bits2 = self.get_bitsi(self.get_max_val(Self::CHUNK_BITWIDTH).bits() as i32);
        let v1 = LongElement::newa(bits1, self.generator.clone());
        let v2 = LongElement::newa(bits2, self.generator.clone());
        let mut generator = self.generator();

        for i in 0..v1.array.len() {
            CircuitGenerator::add_equality_assertion(
                generator.clone(),
                v1.array[i].as_ref().unwrap(),
                v2.array[i].as_ref().unwrap(),
            );
        }
    }

    // an improved equality assertion algorithm from xjsnark
    pub fn assert_equality(&self, e: &Self) {
        let generator = self.generator();
        let (mut a1, mut a2) = (self.array.clone(), e.array.clone());
        let (mut bounds1, mut bounds2) = (
            self.current_max_values.clone(),
            e.current_max_values.clone(),
        );

        let limit = a1.len().max(a2.len());
        // padding
        if e.array.len() != limit {
            a2 = WireArray::new(a2, self.generator.clone())
                .adjust_length(None, limit)
                .as_array()
                .clone();
            bounds2 = vec![BigInteger::ZERO; limit];
            bounds2[..e.current_max_values.len()].clone_from_slice(&e.current_max_values);
        }
        if self.array.len() != limit {
            a1 = WireArray::new(a1, self.generator.clone())
                .adjust_length(None, limit)
                .as_array()
                .clone();
            bounds1 = vec![BigInteger::ZERO; limit];
            bounds1[..self.current_max_values.len()].clone_from_slice(&self.current_max_values);
        }

        // simpl e equality assertion cases
        if a1.len() == a2.len() && a1.len() == 1 {
            CircuitGenerator::add_equality_assertion_with_str(
                generator.clone(),
                a1[0].as_ref().unwrap(),
                a2[0].as_ref().unwrap(),
                "Equality assertion of long elements | case 1",
            );
            return;
        } else if self.is_aligned() && e.is_aligned() {
            for i in 0..limit {
                CircuitGenerator::add_equality_assertion_with_option(
                    generator.clone(),
                    a1[i].as_ref().unwrap(),
                    a2[i].as_ref().unwrap(),
                    &Some(format! {"Equality assertion of long elements | case 2 | index {i}"}),
                );
            }
            return;
        }

        // To make the equality check more efficient, group the chunks together
        // while ensuring that there are no overflows.

        let mut group1 = vec![];
        let mut group1_bounds = vec![];
        let mut group2 = vec![];
        let mut group2_bounds = vec![];

        // This self.array will store how many chunks were grouped together for every
        // wire in group1 or group2
        // The grouping needs to happen in the same way for the two operands, so
        // it's one steps self.array
        let mut steps = vec![];

        let shift = BigInteger::from(2).pow(Self::CHUNK_BITWIDTH as u32);
        let mut i = 0;
        while i < limit {
            let mut step = 1;
            let mut w1 = a1[i].clone().unwrap();
            let mut w2 = a2[i].clone().unwrap();
            let mut b1 = bounds1[i].clone();
            let mut b2 = bounds2[i].clone();
            while i + step <= limit - 1 {
                let delta = shift.pow(step as u32);
                if b1.clone().add(bounds1[i + step].clone().mul(&delta)).bits()
                    < CONFIGS.log2_field_prime - 2
                    && b2.clone().add(bounds2[i + step].clone().mul(&delta)).bits()
                        < CONFIGS.log2_field_prime - 2
                {
                    w1 = w1.add(a1[i + step].as_ref().unwrap().mulb(&delta));
                    w2 = w2.add(a2[i + step].as_ref().unwrap().mulb(&delta));
                    b1 = b1.add(bounds1[i + step].clone().mul(&delta));
                    b2 = b2.add(bounds2[i + step].clone().mul(&delta));
                    step += 1;
                } else {
                    break;
                }
            }
            group1.push(Some(w1));
            group1_bounds.push(b1);
            group2.push(Some(w2));
            group2_bounds.push(b2);
            steps.push(step);
            i += step;
        }

        let num_of_grouped_chunks = group1.len();

        // After grouping, subtraction will be needed to compare the grouped
        // chunks and propagate carries.
        // To avoid dealing with cases where the first operand in the
        // subtraction is less than the second operand,
        // we introduce an auxiliary constant computed based on the bounds of
        // the second operand. The chunks
        // of this aux_constant will be added to the chunks of the first operand
        // before subtraction.

        let mut aux_constant = BigInteger::ZERO;
        let mut aux_constant_chunks = vec![BigInteger::ZERO; num_of_grouped_chunks];

        let mut carries = CircuitGenerator::create_prover_witness_wire_array(
            self.generator.clone().upgrade().unwrap(),
            num_of_grouped_chunks - 1,
        );
        let mut carries_bitwidth_bounds = vec![0; carries.len()];

        // computing the aux_constant_chunks, and the total aux_constant
        let mut accum_step = 0;
        for j in 0..aux_constant_chunks.len() - 1 {
            aux_constant_chunks[j] = BigInteger::from(2).pow(group2_bounds[j].bits() as u32);
            aux_constant =
                aux_constant.add(aux_constant_chunks[j].clone().mul(shift.pow(accum_step)));
            accum_step += steps[j] as u32;
            carries_bitwidth_bounds[j] = aux_constant_chunks[j].bits().max(group1_bounds[j].bits())
                - steps[j] as u64 * Self::CHUNK_BITWIDTH as u64
                + 1;
        }

        // since the two elements should be equal, we should not need any aux
        // chunk in the last step
        //aux_constant_chunks.last_mut().unwrap() = BigInteger::ZERO;

        // Note: the previous aux_constant_chunks are not aligned. We compute an
        // aligned version as follows.

        // First split the aux constant into small chunks based on
        // Self::CHUNK_BITWIDTH
        let aligned_aux_constant_small_chunks = Util::split(&aux_constant, Self::CHUNK_BITWIDTH);

        // second, group the small aux chunks based on the steps self.array computed
        // earlier to get the aligned_aux_constant_chunks
        // aligned_aux_constant_chunks is the grouped version of
        // aligned_aux_constant_small_chunks

        let mut aligned_aux_constant_chunks = vec![BigInteger::ZERO; num_of_grouped_chunks];

        let mut idx = 0;
        'loop1: for j in 0..num_of_grouped_chunks {
            for k in 0..steps[j] {
                aligned_aux_constant_chunks[j] = aligned_aux_constant_chunks[j].clone().add(
                    aligned_aux_constant_small_chunks[idx]
                        .clone()
                        .mul(shift.pow(k as u32)),
                );
                idx += 1;
                if idx == aligned_aux_constant_small_chunks.len() {
                    break 'loop1;
                }
            }
        }
        if idx != aligned_aux_constant_small_chunks.len() {
            if idx == aligned_aux_constant_small_chunks.len() - 1 {
                aligned_aux_constant_chunks[num_of_grouped_chunks - 1] =
                    aligned_aux_constant_chunks[num_of_grouped_chunks - 1]
                        .clone()
                        .add(
                            aligned_aux_constant_small_chunks[idx]
                                .clone()
                                .mul(shift.pow(steps[num_of_grouped_chunks - 1] as u32)),
                        );
            } else {
                panic!("Case not expected. Please report.");
            }
        }
        let steps: Vec<_> = steps.iter().map(|&i| i as i32).collect();
        // specify how the values of carries are obtained during runtime
        let prover = crate::impl_prover!(
                                        eval( carries: Vec<Option<WireType>>,
                                        group1: Vec<Option<WireType>>,
                                        group2: Vec<Option<WireType>>,
                                        steps: Vec<i32>,
                                        aux_constant_chunks: Vec<BigInteger>,
                                        aligned_aux_constant_chunks: Vec<BigInteger>)  {
                        impl Instruction for Prover{
                         fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                            let mut prev_carry = BigInteger::ZERO;
                                            for i in 0..self.carries.len() {
                                                let a = evaluator.get_wire_value(self.group1[i].as_ref().unwrap());
                                                let b = evaluator.get_wire_value(self.group2[i].as_ref().unwrap());
                                                let mut carry_value = self.aux_constant_chunks[i]
                                                    .clone()
                                                    .add(a)
                                                    .sub(b)
                                                    .sub(&self.aligned_aux_constant_chunks[i])
                                                    .add(prev_carry);
                                                carry_value = carry_value.shr(self.steps[i] * LongElement::CHUNK_BITWIDTH);
                                                evaluator
                                                    .set_wire_value(self.carries[i].as_ref().unwrap(), &carry_value);
                                                prev_carry = carry_value;
                                            }
        Ok(())
                        }
                        }
                                    }
                                );
        CircuitGenerator::specify_prover_witness_computation(generator.clone(), prover);

        // We must make sure that the carries values are bounded.

        for j in 0..carries.len() {
            carries[j]
                .as_ref()
                .unwrap()
                .restrict_bit_length(carries_bitwidth_bounds[j]);

            // Note: in this context restrict_bit_length and get_bit_wires will be
            // the same, but it's safer to use restrict_bit_length
            // when enforcing constraints.
        }

        // Now apply the main constraints
        let zero_wire = generator.get_zero_wire();
        let mut prev_carry = zero_wire.clone().unwrap();
        let mut prev_bound = BigInteger::ZERO;

        // recall carries.len() = num_of_grouped_chunks - 1
        for j in 0..carries.len() + 1 {
            let aux_constant_chunk_wire =
                CircuitGenerator::create_constant_wire(generator.clone(), &aux_constant_chunks[j]);
            let aligned_aux_constant_chunk_wire = CircuitGenerator::create_constant_wire(
                generator.clone(),
                &aligned_aux_constant_chunks[j],
            );

            // the last carry value must be zero
            let current_carry = if j == carries.len() {
                zero_wire.clone()
            } else {
                carries[j].clone()
            };

            // overflow check for safety
            if aux_constant_chunks[j]
                .clone()
                .add(group1_bounds[j].clone())
                .add(BigInteger::from((prev_bound >= CONFIGS.field_prime) as u8))
                != BigInteger::ZERO
            {

                //println!("Overflow possibility @ ForceEqual()");
            }

            let w1 = aux_constant_chunk_wire
                .add(group1[j].clone().unwrap().sub(group2[j].as_ref().unwrap()))
                .add(prev_carry);
            let w2 = aligned_aux_constant_chunk_wire.add(
                current_carry
                    .clone()
                    .unwrap()
                    .mulb(&shift.pow(steps[j] as u32)),
            );

            // enforce w1 = w2
            // note: in the last iteration, both aux_constant_chunk_wire and
            // current_carry will be zero,
            // i.e., there will be no more values to be checked.

            CircuitGenerator::add_equality_assertion_with_option(
                generator.clone(),
                &w1,
                &w2,
                &Some(format!(
                    "Equality assertion of long elements | case 3 | index {j}"
                )),
            );

            prev_carry = current_carry.clone().unwrap();
            if j != carries.len() {
                prev_bound = Util::compute_max_value(carries_bitwidth_bounds[j]);
            }
        }
    }

    // applies an improved technique to assert comparison
    pub fn assert_less_than(&self, other: &Self) {
        // first verify that both elements are aligned
        assert!(
            self.is_aligned() && other.is_aligned(),
            "input chunks are not aligned"
        );

        let a1 = self.get_array();
        let a2 = other.get_array();
        let length = a1.len().max(a2.len());
        let mut generator = self.generator();

        let zero_wire = generator.get_zero_wire().unwrap();
        let padded_a1 = Util::pad_wire_array(&a1, length, &zero_wire);
        let padded_a2 = Util::pad_wire_array(&a2, length, &zero_wire);

        //Instead of doing the comparison naively (which will involve all the
        //bits) let the prover help us by pointing to the first chunk in the
        //other element that is more than the corresponding chunk in this
        //element.

        let helper_bits = CircuitGenerator::create_prover_witness_wire_array(
            self.generator.clone().upgrade().unwrap(),
            length,
        );
        // set the value of the helper_bits outside the circuits
        let prover = crate::impl_prover!(
                        eval(
                                 length: usize,
                                padded_a1: Vec<Option<WireType>>,
                                padded_a2: Vec<Option<WireType>>,
                                helper_bits: Vec<Option<WireType>>)  {
                                impl Instruction for Prover{
                                    fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{

                                        let mut found = false;
                                        for i in (0..self.length).rev() {
                                            let v1 = evaluator.get_wire_value(self.padded_a1[i].as_ref().unwrap());
                                            let v2 = evaluator.get_wire_value(self.padded_a2[i].as_ref().unwrap());

                                            let check = v2 > v1 && !found;
                                            evaluator.set_wire_value(
                                                self.helper_bits[i].as_ref().unwrap(),
                                                &(if check { Util::one() } else { BigInteger::ZERO }),
                                            );
                                            if check {
                                                found = true;
                                            }
                                        }
        Ok(())
                                    }
                                }
                    }
                );
        CircuitGenerator::specify_prover_witness_computation(generator.clone(), prover);

        // verify constraints about helper bits.
        for w in &helper_bits {
            CircuitGenerator::add_binary_assertion(generator.clone(), w.as_ref().unwrap());
        }
        // Only one bit should be set.
        CircuitGenerator::add_one_assertion(
            generator.clone(),
            &WireArray::new(helper_bits.clone(), self.generator.clone()).sum_all_elements(),
        );

        // verify "the greater than condition" for the specified chunk
        let mut chunk1 = zero_wire.clone();
        let mut chunk2 = zero_wire.clone();

        for i in 0..helper_bits.len() {
            chunk1 = chunk1.add(
                padded_a1[i]
                    .clone()
                    .unwrap()
                    .mul(helper_bits[i].as_ref().unwrap()),
            );
            chunk2 = chunk2.add(
                padded_a2[i]
                    .clone()
                    .unwrap()
                    .mul(helper_bits[i].as_ref().unwrap()),
            );
        }
        CircuitGenerator::add_one_assertion(
            generator.clone(),
            &chunk1.is_less_thans(&chunk2, Self::CHUNK_BITWIDTH),
        );

        // check that the other more significant chunks are equal
        let mut helper_bits2: Vec<Option<WireType>> = vec![None; helper_bits.len()];
        helper_bits2[0] = generator.get_zero_wire();
        for i in 1..helper_bits.len() {
            helper_bits2[i] = helper_bits2[i - 1]
                .as_ref()
                .map(|x| x.clone().add(helper_bits[i - 1].as_ref().unwrap()));

            CircuitGenerator::add_assertion(
                generator.clone(),
                helper_bits2[i].as_ref().unwrap(),
                &padded_a1[i]
                    .clone()
                    .unwrap()
                    .sub(padded_a2[i].as_ref().unwrap()),
                &zero_wire,
            );
        }

        // no checks needed for the less significant chunks
    }
}

impl Add<u64> for LongElement {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self.add(BigInteger::from(rhs))
    }
}

impl Add<BigInteger> for LongElement {
    type Output = Self;

    fn add(self, rhs: BigInteger) -> Self::Output {
        if rhs.sign() == Sign::NoSign {
            return self;
        }
        if rhs.sign() == Sign::Minus {
            return self.sub(rhs.neg());
        }
        let generator = self.generator.clone();
        self.add(&LongElement::newb(
            Util::split(&rhs, Self::CHUNK_BITWIDTH),
            generator,
        ))
    }
}

impl Add<&Self> for LongElement {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let generator = self.generator();
        if self.add_overflow_check(rhs) {
            //println!("Warning: Addition overflow could happen");
        }

        let length = self.array.len().max(rhs.array.len());
        let w1 =
            WireArray::new(self.array.clone(), self.generator.clone()).adjust_length(None, length);
        let w1 = w1.as_array();
        let w2 =
            WireArray::new(rhs.array.clone(), self.generator.clone()).adjust_length(None, length);
        let w2 = w2.as_array();
        let mut result = vec![None; length];
        let mut new_max_values = vec![BigInteger::ZERO; length];
        for i in 0..length {
            result[i] = w1[i].clone().map(|x| x.add(w2[i].as_ref().unwrap()));
            let max1 = if i < self.array.len() {
                self.current_max_values[i].clone()
            } else {
                BigInteger::ZERO
            };
            let max2 = if i < rhs.array.len() {
                &rhs.current_max_values[i]
            } else {
                &BigInteger::ZERO
            };

            new_max_values[i] = max1.add(max2);
        }
        LongElement::neww(result, new_max_values, self.generator.clone())
    }
}
impl Sub<u64> for LongElement {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::sub(self, BigInteger::from(rhs))
    }
}
impl Sub<BigInteger> for LongElement {
    type Output = Self;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        if rhs.sign() == Sign::NoSign {
            return self;
        }
        if rhs.sign() == Sign::Minus {
            return self.add(rhs.neg());
        }
        let generator = self.generator.clone();
        self.sub(&LongElement::newb(
            Util::split(&rhs, Self::CHUNK_BITWIDTH),
            generator,
        ))
    }
}
impl Sub<&Self> for LongElement {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        assert!(
            self.is_aligned() && rhs.is_aligned(),
            "Subtraction arguments must be properly aligned"
        );
        let mut generator = self.generator();

        let result = CircuitGenerator::create_long_element_prover_witness(
            self.generator.clone().upgrade().unwrap(),
            self.get_max_val(Self::CHUNK_BITWIDTH).bits() as i32,
            &None,
        );
        let other = rhs;
        let long_element = &self;
        let prover = crate::impl_prover!(
                                eval( long_element:LongElement,
                                        other:LongElement,
                                        result:LongElement)  {
                                        impl Instruction for Prover{
                                            fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                               let my_value = evaluator
                                                .get_wire_valuei(&self.long_element, LongElement::CHUNK_BITWIDTH);
                                            let other_value =
                                                evaluator.get_wire_valuei(&self.other, LongElement::CHUNK_BITWIDTH);
                                            let result_value = my_value.sub(other_value);
                                            assert!(
                                                result_value.sign() != Sign::Minus,
                                                "Result of subtraction is negative!"
                                            );
                                            evaluator.set_wire_valuebi(
                                                &self.result,
                                                &result_value,
                                                LongElement::CHUNK_BITWIDTH,
                                            );
        Ok(())
                                            }
                                        }
                            }
                        );
        CircuitGenerator::specify_prover_witness_computation(generator.clone(), prover);

        result.restrict_bitwidth();
        self.assert_equality(&result.clone().add(rhs));
        result
    }
}

impl Mul<&Self> for LongElement {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        // Implements the improved long integer multiplication from xjsnark

        if self.mul_overflow_check(rhs) {
            //println!("Warning: Mul overflow could happen");
        }
        let length = self.array.len() + rhs.array.len() - 1;
        let mut result: Vec<Option<WireType>>;
        let mut generator = self.generator();
        // check if we can just apply the simpl e non-costly multiplication
        if rhs.array.len() == 1 || self.array.len() == 1 || self.is_constant() || rhs.is_constant()
        {
            let zero = generator.get_zero_wire();
            result = vec![zero; length];

            // O(n*m) multiplication. Fine to apply if any of the operands has
            // dim 1
            // or any of them is constant
            for i in 0..self.array.len() {
                for j in 0..rhs.array.len() {
                    result[i + j] = result[i + j].clone().map(|x| {
                        x.add(
                            self.array[i]
                                .as_ref()
                                .unwrap()
                                .clone()
                                .mul(rhs.array[j].as_ref().unwrap().clone()),
                        )
                    });
                }
            }
        } else {
            // impl ement the optimization

            result = CircuitGenerator::create_prover_witness_wire_array(
                self.generator.clone().upgrade().unwrap(),
                length,
            );
            // for safety
            let (array1, array2) = (&self.array, &rhs.array);
            let prover = crate::impl_prover!(
                                    eval(result: Vec<Option<WireType>>,
                                    array1: Vec<Option<WireType>>,
                                    array2: Vec<Option<WireType>>)  {
                                    impl Instruction for Prover{
                                     fn evaluate(&self, evaluator: &mut CircuitEvaluator) ->eyre::Result<()>{
                                                               let a = evaluator.get_wires_values(&self.array1);
                                                            let b = evaluator.get_wires_values(&self.array2);
                                                            let result_vals = LongElement::multiply_polys(a, b);
                                                            evaluator.set_wire_valuea(&self.result, &result_vals);
            Ok(())
                                    }
                                    }
                                                }
                                            );
            CircuitGenerator::specify_prover_witness_computation(generator.clone(), prover);

            for k in 0..length {
                let constant = BigInteger::from(k as u64 + 1);
                let mut coeff = Util::one();

                let mut vector1 = vec![None; self.array.len()];
                let mut vector2 = vec![None; rhs.array.len()];
                let mut vector3 = vec![None; length];
                for i in 0..length {
                    if i < self.array.len() {
                        vector1[i] = self.array[i].as_ref().map(|x| x.clone().mulb(&coeff));
                    }
                    if i < rhs.array.len() {
                        vector2[i] = rhs.array[i].as_ref().map(|x| x.clone().mulb(&coeff));
                    }
                    vector3[i] = result[i].clone().map(|x| x.mulb(&coeff));
                    coeff = Util::modulo(&coeff.mul(&constant), &CONFIGS.field_prime);
                }

                let v1 = WireArray::new(vector1, self.generator.clone()).sum_all_elements();
                let v2 = WireArray::new(vector2, self.generator.clone()).sum_all_elements();
                let v3 = WireArray::new(vector3, self.generator.clone()).sum_all_elements();
                CircuitGenerator::add_assertion(generator.clone(), &v1, &v2, &v3);
            }
        }

        let mut new_max_values = vec![BigInteger::ZERO; length];
        for i in 0..self.array.len() {
            for j in 0..rhs.array.len() {
                new_max_values[i + j] = new_max_values[i + j].clone().add(
                    self.current_max_values[i]
                        .clone()
                        .mul(rhs.current_max_values[j].clone()),
                );
            }
        }
        LongElement::neww(result, new_max_values, self.generator.clone())
    }
}

impl Eq for LongElement {}
impl PartialEq for LongElement {
    fn eq(&self, other: &Self) -> bool {
        // if o == None || !(o instance_of LongElement) {
        // 	return false;
        // }
        // LongElement v = (LongElement) o;
        if other.array.len() != self.array.len() {
            return false;
        }
        // let mut  check = true;
        // for i in 0.. self.array.len() {
        // 	if !v.array[i]==self.array[i] {
        // 		check = false;
        // 		break;
        // 	}
        // }
        // return check;
        self.array.iter().zip(&other.array).all(|(a, b)| a == b)
    }
}
