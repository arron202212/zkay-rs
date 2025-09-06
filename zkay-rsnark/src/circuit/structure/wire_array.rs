#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        InstanceOf,
        eval::instruction::Instruction,
        operations::primitive::{add_basic_op::AddBasicOp, pack_basic_op::PackBasicOp},
        structure::{
            circuit_generator::CreateConstantWire,
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            linear_combination_wire::LinearCombinationWire,
            wire::GeneratorConfig,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_type::WireType,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Index, IndexMut, Mul, Shl, Sub},
    sync::Arc,
};

use rccell::{RcCell, WeakCell};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct WireArray {
    pub array: Vec<Option<WireType>>,
    pub generator: WeakCell<CircuitGenerator>,
}

impl Index<usize> for WireArray {
    type Output = Option<WireType>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<usize> for WireArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl WireArray {
    pub fn newi(n: i32, generator: WeakCell<CircuitGenerator>) -> Self {
        Self::newic(n, generator)
    }

    pub fn newic(n: i32, generator: WeakCell<CircuitGenerator>) -> Self {
        WireArray::new(vec![None; n as usize], generator)
    }

    pub fn new(wireArray: Vec<Option<WireType>>, generator: WeakCell<CircuitGenerator>) -> Self {
        Self {
            array: wireArray,
            generator,
        }
    }

    pub fn get(&self, i: usize) -> &Option<WireType> {
        &self.array[i]
    }

    pub fn set(&mut self, i: usize, w: WireType) {
        self.array[i] = Some(w);
    }

    pub fn size(&self) -> usize {
        self.array.len()
    }

    pub fn as_array(&self) -> &Vec<Option<WireType>> {
        &self.array
    }
    pub fn generator(&self) -> RcCell<CircuitGenerator> {
        self.generator.clone().upgrade().unwrap()
    }
    pub fn mul_wire_array(
        &self,
        v: &WireArray,
        desired_length: usize,
        desc: &Option<String>,
    ) -> Self {
        let (ws1, ws2) = (
            self.adjust_length(Some(&self.array), desired_length),
            self.adjust_length(Some(&v.array), desired_length),
        );
        let (ws1, ws2) = (ws1.as_array(), ws2.as_array());
        let mut out = vec![None; desired_length];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().mulw(ws2[i].as_ref().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn sum_all_elements(&self, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let mut all_constant = true;
        let mut sum = BigInteger::ZERO;
        for w in &self.array {
            if w.as_ref().unwrap().instance_of("ConstantWire") {
                sum = sum.add(
                    w.as_ref()
                        .unwrap()
                        .try_as_constant_ref()
                        .unwrap()
                        .get_constant(),
                );
            } else {
                all_constant = false;
                break;
            }
        }
        if !all_constant {
            let output = WireType::LinearCombination(LinearCombinationWire::new(
                generator.get_current_wire_id(),
                None,
                generator.clone().downgrade(),
            ));
            generator.borrow_mut().current_wire_id += 1;
            let op = AddBasicOp::new(
                self.array.clone(),
                &output,
                desc.clone().unwrap_or(String::new()),
            );
            //			generator.add_to_evaluation_queue(Box::new(op));

            let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
            if let Some(cached_outputs) = cached_outputs {
                generator.borrow_mut().current_wire_id -= 1;
                //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());
                cached_outputs[0].clone().unwrap()
            } else {
                output
            }
        } else {
            generator.create_constant_wire(&sum, desc)
        }
    }

    pub fn add_wire_array(
        &self,
        v: &WireArray,
        desired_length: usize,
        desc: &Option<String>,
    ) -> Self {
        let (ws1, ws2) = (
            self.adjust_length(Some(&self.array), desired_length),
            self.adjust_length(Some(&v.array), desired_length),
        );
        let (ws1, ws2) = (ws1.as_array(), ws2.as_array());
        let mut out = vec![None; desired_length];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().addw(ws2[i].as_ref().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn xor_wire_array(
        &self,
        v: &WireArray,
        desired_length: usize,
        desc: &Option<String>,
    ) -> Self {
        let (ws1, ws2) = (
            self.adjust_length(Some(&self.array), desired_length),
            self.adjust_length(Some(&v.array), desired_length),
        );
        let (ws1, ws2) = (ws1.as_array(), ws2.as_array());
        let out: Vec<_> = ws1
            .iter()
            .zip(ws2)
            .map(|(w1, w2)| w1.as_ref().map(|x| x.xorw(w2.as_ref().unwrap(), desc)))
            .collect();

        WireArray::new(out, self.generator.clone())
    }

    pub fn xor_wire_arrayi(&self, v: &WireArray, desc: &Option<String>) -> Self {
        assert!(self.size() == v.size());
        let (ws1, ws2) = (&self.array, &v.array);

        let out: Vec<_> = ws1
            .iter()
            .zip(ws2)
            .map(|(w1, w2)| w1.as_ref().map(|x| x.xorw(w2.as_ref().unwrap(), desc)))
            .collect();

        WireArray::new(out, self.generator.clone())
    }

    pub fn and_wire_array(
        &self,
        v: &WireArray,
        desired_length: usize,
        desc: &Option<String>,
    ) -> Self {
        use std::time::Instant;
        let start = Instant::now();
        // let ws1 = ;
        // println!("End adjust_length  Time: == {} s", start.elapsed().as_secs());

        // let ws2 = ;
        // println!("End adjust_length  Time: == {} s", start.elapsed().as_secs());

        // let mut out = vec![None; desired_length];
        // for i in 0..out.len() {
        //     out[i] = ws1[i]
        //         .as_ref()
        //         .map(|x| x.clone().mulw(ws2[i].clone().unwrap(), desc));
        // }
        let out: Vec<_> = self
            .adjust_length(Some(&self.array), desired_length)
            .as_array()
            .iter()
            .zip(
                self.adjust_length(Some(&v.array), desired_length)
                    .as_array(),
            )
            .map(|(w1, w2)| w1.as_ref().map(|w1v| w1v.mulw(w2.as_ref().unwrap(), desc)))
            .collect();
        // println!(
        //     "End mulw  {desired_length} Time: == {} s",
        //     start.elapsed().as_secs()
        // );

        let v = WireArray::new(out, self.generator.clone());
        // println!("End WireArray  Time: == {} s", start.elapsed().as_secs());
        v
    }

    pub fn or_wire_array(
        &self,
        v: WireArray,
        desired_length: usize,
        desc: &Option<String>,
    ) -> Self {
        let out: Vec<_> = self
            .adjust_length(Some(&self.array), desired_length)
            .as_array()
            .iter()
            .zip(
                self.adjust_length(Some(&v.array), desired_length)
                    .as_array(),
            )
            .map(|(x, y)| x.as_ref().map(|x| x.orw(y.as_ref().unwrap(), desc)))
            .collect();

        WireArray::new(out, self.generator.clone())
    }

    pub fn inv_as_bits(&self, desired_bit_width: usize, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        let mut out = vec![None; desired_bit_width];
        for i in 0..desired_bit_width {
            out[i] = self.array.get(i).map_or_else(
                || generator.get_one_wire(),
                |v| v.as_ref().and_then(|x| x.inv_as_bit(desc)),
            );
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn adjust_length(&self, ws: Option<&Vec<Option<WireType>>>, desired_length: usize) -> Self {
        let mut generator = self.generator();

        let ws = ws.unwrap_or(&self.array);
        if ws.len() == desired_length {
            return WireArray::new(ws.clone(), self.generator.clone());
        }
        let zero_wire = generator.get_zero_wire();
        let mut new_ws = vec![zero_wire; desired_length];
        let len = ws.len().min(desired_length);
        new_ws[..len].clone_from_slice(&ws[..len]);

        WireArray::new(new_ws, self.generator.clone())
    }

    // pub fn adjustLengthi(&self, desired_length: usize) ->Self{
    //     if self.array.len() == desired_length {
    //         self.clone()
    //     }
    //     let mut new_ws = vec![None; desired_length];
    //     new_ws[..std::cmp::min(self.array.len(), desired_length)].clone_from_slice(&self.array);
    //     if self.array.len() < desired_length {
    //         for i in self.array.len()..desired_length {
    //             new_ws[i] = generator.get_zero_wire();
    //         }
    //     }
    //     WireArray::new(new_ws,self.generator.clone())
    // }

    // pub fn packAsBitsi(&self, n: usize, desc: &Option<String>) -> WireType{
    //     self.packAsBitsii(0, n, desc)
    // }

    // pub fn pack_as_bits(None,None,&self, desc: &Option<String>) -> WireType{
    //     self.packAsBitsi(self.array.len(), desc)
    // }

    pub fn check_if_constant_bits(&self, desc: &Option<String>) -> Option<BigInteger> {
        let mut all_constant = true;
        let mut sum = BigInteger::ZERO;
        for i in 0..self.array.len() {
            let w = self.array[i].clone();
            if w.as_ref().unwrap().instance_of("ConstantWire") {
                let cw = w;
                let v = cw
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .get_constant();
                if v == Util::one() {
                    sum = sum.add(v.shl(i));
                } else if !v == BigInteger::ZERO {
                    println!(
                        "Warning, one of the bit wires is constant but not binary : {}",
                        Util::getDesc(desc)
                    );
                }
            } else {
                all_constant = false;
            }
        }
        all_constant.then_some(sum)
    }

    pub fn pack_as_bits(
        &self,
        from: Option<usize>,
        to: Option<usize>,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator();

        let (from, to) = (from.unwrap_or(0), to.unwrap_or(self.array.len()));
        assert!(
            from <= to && to <= self.array.len(),
            "Invalid bounds: from > to"
        );

        let bits = self.array[from..to].to_vec();
        let mut all_constant = true;
        let mut sum = BigInteger::ZERO;
        for i in 0..bits.len() {
            let w = bits[i].clone();
            if w.as_ref().unwrap().instance_of("ConstantWire") {
                let cw = w;
                let v = cw
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .get_constant();
                if v == Util::one() {
                    sum = sum.add(v.shl(i));
                } else {
                    assert!(
                        v == BigInteger::ZERO,
                        "Trying to pack non-binary constant bits : {}",
                        Util::getDesc(desc)
                    );
                }
            } else {
                all_constant = false;
            }
        }
        if !all_constant {
            let out = WireType::LinearCombination(LinearCombinationWire::new(
                generator.get_current_wire_id(),
                None,
                generator.clone().downgrade(),
            ));
            generator.borrow_mut().current_wire_id += 1;
            out.set_bits(Some(WireArray::new(
                bits.clone(),
                generator.clone().downgrade(),
            )));
            let op = PackBasicOp::new(bits, &out, desc.clone().unwrap_or(String::new()));

            let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
            if let Some(cached_outputs) = cached_outputs {
                generator.borrow_mut().current_wire_id -= 1;
                //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());
                cached_outputs[0].clone().unwrap()
            } else {
                out
            }
        } else {
            generator.create_constant_wire(&sum, desc)
        }
    }

    pub fn rotate_left(&self, num_bits: usize, s: usize, desc: &Option<String>) -> Self {
        let bits = self.adjust_length(Some(&self.array), num_bits);
        let bits = bits.as_array();
        let rotated_bits: Vec<_> = (0..num_bits)
            .map(|i| bits[i + if i < s { num_bits } else { 0 } - s].clone())
            .collect();
        WireArray::new(rotated_bits, self.generator.clone())
    }

    pub fn rotate_right(&self, num_bits: usize, s: usize, desc: &Option<String>) -> Self {
        let bits = self.adjust_length(Some(&self.array), num_bits);
        let bits = bits.as_array();
        let rotated_bits: Vec<_> = (0..num_bits)
            .map(|i| bits[i + s - if i + s >= num_bits { num_bits } else { 0 }].clone())
            .collect();

        WireArray::new(rotated_bits, self.generator.clone())
    }

    pub fn shift_left(&self, num_bits: usize, s: usize, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        let bits = self.adjust_length(Some(&self.array), num_bits);
        let bits = bits.as_array();
        let zero_wire = generator.get_zero_wire();
        let mut shifted_bits = vec![zero_wire; num_bits];
        shifted_bits[s..num_bits].clone_from_slice(&bits[..num_bits - s]);
        WireArray::new(shifted_bits, self.generator.clone())
    }

    pub fn shift_right(&self, num_bits: usize, s: usize, desc: &Option<String>) -> Self {
        let mut generator = self.generator();
        let bits = self.adjust_length(Some(&self.array), num_bits);
        let bits = bits.as_array();
        let zero_wire = generator.get_zero_wire();
        let mut shifted_bits = vec![zero_wire; num_bits];
        shifted_bits[..num_bits.saturating_sub(s)].clone_from_slice(&bits[s..]);
        WireArray::new(shifted_bits, self.generator.clone())
    }

    pub fn pack_bits_into_words(
        &self,
        word_bitwidth: usize,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let num_words = (self.array.len() as f64 * 1.0 / word_bitwidth as f64).ceil() as usize;
        let padded = self.adjust_length(Some(&self.array), word_bitwidth * num_words);
        let padded = padded.as_array();
        let mut result = vec![None; num_words];
        for i in 0..num_words {
            result[i] = Some(
                WireArray::new(
                    padded[i * word_bitwidth..(i + 1) * word_bitwidth].to_vec(),
                    self.generator.clone(),
                )
                .pack_as_bits(None, None, &None),
            );
        }
        result
    }

    pub fn pack_words_into_larger_words(
        &self,
        word_bitwidth: i32,
        num_words_per_larger_word: i32,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut generator = self.generator();

        let num_larger_words =
            (self.array.len() as f64 * 1.0 / num_words_per_larger_word as f64).ceil() as usize;
        let mut result = vec![generator.get_zero_wire(); num_larger_words];
        for i in 0..self.array.len() {
            let subIndex = i % num_words_per_larger_word as usize;
            result[i / num_words_per_larger_word as usize] = result
                [i / num_words_per_larger_word as usize]
                .as_ref()
                .map(|x| {
                    x.clone().addw(
                        &self.array[i].clone().unwrap().mulb(
                            &BigInteger::from(2).pow(subIndex as u32 * word_bitwidth as u32),
                            &None,
                        ),
                        &None,
                    )
                });
        }
        result
    }

    pub fn get_bits(&self, bitwidth: usize, desc: &Option<String>) -> Self {
        let mut bits = vec![None; bitwidth * self.array.len()];
        let mut idx = 0;
        for i in 0..self.array.len() {
            let tmp = self.array[i]
                .as_ref()
                .unwrap()
                .get_bit_wiresi(bitwidth as u64, desc);
            let tmp = tmp.as_array();
            for j in 0..bitwidth {
                bits[idx] = tmp[j].clone();
                idx += 1;
            }
        }
        WireArray::new(bits, self.generator.clone())
    }
}
