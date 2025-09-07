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
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                CreateConstantWire, add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};

use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
};

use rccell::{RcCell, WeakCell};

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Base;
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct SHA256Gadget<T> {
    pub unpadded_inputs: Vec<Option<WireType>>,
    pub bit_width_per_input_element: usize,
    pub total_length_in_bytes: usize,
    pub num_blocks: usize,
    pub binary_output: bool,
    pub padding_required: bool,
    pub prepared_input_bits: Vec<Option<WireType>>,
    pub output: Vec<Option<WireType>>,
    pub t: T,
}
impl<T> SHA256Gadget<T> {
    pub fn new(
        ins: Vec<Option<WireType>>,
        bit_width_per_input_element: usize,
        total_length_in_bytes: usize,
        binary_output: bool,
        padding_required: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
        t: T,
    ) -> Gadget<Self> {
        assert!(
            total_length_in_bytes * 8 <= ins.len() * bit_width_per_input_element
                && total_length_in_bytes * 8
                    >= (ins.len().saturating_sub(1)) * bit_width_per_input_element,
            "Inconsistent Length Information {},{},{}",
            total_length_in_bytes,
            ins.len(),
            bit_width_per_input_element
        );

        assert!(
            padding_required
                || total_length_in_bytes % 64 == 0
                || ins.len() * bit_width_per_input_element == total_length_in_bytes,
            "When padding is not forced, total_length_in_bytes % 64 must be zero."
        );
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                unpadded_inputs: ins,
                bit_width_per_input_element,
                total_length_in_bytes,
                num_blocks: 0,
                binary_output,
                padding_required,
                prepared_input_bits: vec![],
                output: vec![],
                t,
            },
        );
        _self.build_circuit();
        _self
    }
}
impl<T> Gadget<SHA256Gadget<T>> {
    const H: [i64; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    const K: [i64; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    fn build_circuit(&mut self) {
        let mut generator = self.generator.clone();

        // pad if needed
        self.prepare();

        let mut out_digest = vec![None; 8];
        let mut h_wires = vec![None; Self::H.len()];
        for i in 0..Self::H.len() {
            h_wires[i] = Some(generator.create_constant_wire(Self::H[i], &None));
        }

        for block_num in 0..self.t.num_blocks {
            let mut ws_splitted = vec![vec![]; 64];
            let mut w = vec![None; 64];

            for i in 0..64 {
                if i < 16 {
                    ws_splitted[i] = Util::reverse_bytes(
                        &self.t.prepared_input_bits
                            [block_num * 512 + i * 32..block_num * 512 + (i + 1) * 32]
                            .to_vec(),
                    );

                    w[i] = Some(
                        WireArray::new(ws_splitted[i].clone(), generator.clone().downgrade())
                            .pack_as_bits(None, Some(32), &None),
                    );
                } else {
                    let t1 = w[i - 15].as_ref().unwrap().rotate_right(32, 7, &None);
                    let t2 = w[i - 15].as_ref().unwrap().rotate_right(32, 18, &None);
                    let t3 = w[i - 15].as_ref().unwrap().shift_right(32, 3, &None);
                    let mut s0 = t1.xor_bitwises(&t2, 32, &None);
                    s0 = s0.xor_bitwises(&t3, 32, &None);

                    let t4 = w[i - 2].as_ref().unwrap().rotate_right(32, 17, &None);
                    let t5 = w[i - 2].as_ref().unwrap().rotate_right(32, 19, &None);
                    let t6 = w[i - 2].as_ref().unwrap().shift_right(32, 10, &None);
                    let mut s1 = t4.xor_bitwises(&t5, 32, &None);
                    s1 = s1.xor_bitwises(&t6, 32, &None);

                    w[i] = w[i - 16]
                        .as_ref()
                        .map(|x| x.clone().add(w[i - 7].as_ref().unwrap()));
                    w[i] = w[i].as_ref().map(|x| x.clone().add(s0).add(s1));
                    w[i] = w[i].as_ref().map(|x| x.clone().trim_bits(34, 32, &None));
                }
            }

            let mut a = h_wires[0].clone().unwrap();
            let mut b = h_wires[1].clone().unwrap();
            let mut c = h_wires[2].clone().unwrap();
            let mut d = h_wires[3].clone().unwrap();
            let mut e = h_wires[4].clone().unwrap();
            let mut f = h_wires[5].clone().unwrap();
            let mut g = h_wires[6].clone().unwrap();
            let mut h = h_wires[7].clone().unwrap();

            for i in 0..64 {
                let t1 = e.rotate_right(32, 6, &None);
                let t2 = e.rotate_right(32, 11, &None);
                let t3 = e.rotate_right(32, 25, &None);
                let mut s1 = t1.xor_bitwises(&t2, 32, &None);
                s1 = s1.xor_bitwises(&t3, 32, &None);

                let ch = self.compute_ch(&e, &f, &g, 32);

                let t4 = a.rotate_right(32, 2, &None);
                let t5 = a.rotate_right(32, 13, &None);
                let t6 = a.rotate_right(32, 22, &None);
                let mut s0 = t4.xor_bitwises(&t5, 32, &None);
                s0 = s0.xor_bitwises(&t6, 32, &None);

                // since after each iteration, SHA256 does c = b; and b = a;, we can make use of that to save multiplications in maj computation.
                // To do this, we make use of the caching feature, by just changing the order of wires sent to maj(). Caching will take care of the rest.
                let maj = if i % 2 == 1 {
                    self.compute_maj(&c, &b, &a, 32)
                } else {
                    self.compute_maj(&a, &b, &c, 32)
                };

                let temp1 = w[i]
                    .clone()
                    .unwrap()
                    .addi(Self::K[i], &None)
                    .add(s1)
                    .add(&h)
                    .add(ch);

                let temp2 = maj.add(s0);

                h = g;
                g = f;
                f = e;
                e = temp1.clone().add(d);
                e = e.trim_bits(35, 32, &None);

                d = c;
                c = b;
                b = a;
                a = temp2.add(temp1);
                a = a.trim_bits(35, 32, &None);
            }

            h_wires[0] = h_wires[0]
                .clone()
                .map(|x| x.add(a.clone()).trim_bits(33, 32, &None));
            h_wires[1] = h_wires[1]
                .clone()
                .map(|x| x.add(b.clone()).trim_bits(33, 32, &None));
            h_wires[2] = h_wires[2]
                .clone()
                .map(|x| x.add(c.clone()).trim_bits(33, 32, &None));
            h_wires[3] = h_wires[3]
                .clone()
                .map(|x| x.add(d.clone()).trim_bits(33, 32, &None));
            h_wires[4] = h_wires[4]
                .clone()
                .map(|x| x.add(e.clone()).trim_bits(33, 32, &None));
            h_wires[5] = h_wires[5]
                .clone()
                .map(|x| x.add(f.clone()).trim_bits(33, 32, &None));
            h_wires[6] = h_wires[6]
                .clone()
                .map(|x| x.add(g.clone()).trim_bits(33, 32, &None));
            h_wires[7] = h_wires[7]
                .clone()
                .map(|x| x.add(h.clone()).trim_bits(33, 32, &None));
        }

        out_digest[0] = h_wires[0].clone();
        out_digest[1] = h_wires[1].clone();
        out_digest[2] = h_wires[2].clone();
        out_digest[3] = h_wires[3].clone();
        out_digest[4] = h_wires[4].clone();
        out_digest[5] = h_wires[5].clone();
        out_digest[6] = h_wires[6].clone();
        out_digest[7] = h_wires[7].clone();

        if !self.t.binary_output {
            self.t.output = out_digest;
            return;
        }
        self.t.output = vec![None; 8 * 32];
        for i in 0..8 {
            let bits = out_digest[i].as_ref().unwrap().get_bit_wiresi(32, &None);
            let bits = bits.as_array();
            for j in 0..32 {
                self.t.output[j + i * 32] = bits[j].clone();
            }
        }
    }

    fn compute_maj(&self, a: &WireType, b: &WireType, c: &WireType, num_bits: usize) -> WireType {
        let mut result = vec![None; num_bits];
        let (a_bits, b_bits, c_bits) = (
            a.get_bit_wiresi(num_bits as u64, &None),
            b.get_bit_wiresi(num_bits as u64, &None),
            c.get_bit_wiresi(num_bits as u64, &None),
        );
        let (a_bits, b_bits, c_bits) = (a_bits.as_array(), b_bits.as_array(), c_bits.as_array());

        for i in 0..num_bits {
            let t1 = a_bits[i].clone().unwrap().mul(b_bits[i].clone().unwrap());
            let t2 = a_bits[i]
                .clone()
                .unwrap()
                .add(b_bits[i].clone().unwrap())
                .add(t1.muli(-2, &None));
            result[i] = Some(t1.add(c_bits[i].clone().unwrap().mul(t2)));
        }
        WireArray::new(result, self.generator.clone().downgrade()).pack_as_bits(None, None, &None)
    }

    fn compute_ch(&self, a: &WireType, b: &WireType, c: &WireType, num_bits: usize) -> WireType {
        let mut result = vec![None; num_bits];

        let (a_bits, b_bits, c_bits) = (
            a.get_bit_wiresi(num_bits as u64, &None),
            b.get_bit_wiresi(num_bits as u64, &None),
            c.get_bit_wiresi(num_bits as u64, &None),
        );
        let (a_bits, b_bits, c_bits) = (a_bits.as_array(), b_bits.as_array(), c_bits.as_array());

        for i in 0..num_bits {
            let t1 = b_bits[i].clone().unwrap().sub(c_bits[i].as_ref().unwrap());
            let t2 = t1.mul(a_bits[i].as_ref().unwrap());
            result[i] = Some(t2.add(c_bits[i].as_ref().unwrap()));
        }
        WireArray::new(result, self.generator.clone().downgrade()).pack_as_bits(None, None, &None)
    }

    fn prepare(&mut self) {
        let mut generator = &self.generator;

        self.t.num_blocks = (self.t.total_length_in_bytes as f64 / 64.0).ceil() as usize;
        let bits = WireArray::new(
            self.t.unpadded_inputs.clone(),
            self.generator.clone().downgrade(),
        )
        .get_bits(self.t.bit_width_per_input_element, &None);
        let bits = bits.as_array();
        let tail_length = self.t.total_length_in_bytes % 64;
        if self.t.padding_required {
            let mut pad;
            if 64 - tail_length >= 9 {
                pad = vec![None; 64 - tail_length];
            } else {
                pad = vec![None; 128 - tail_length];
            }
            self.t.num_blocks = (self.t.total_length_in_bytes + pad.len()) / 64;
            pad[0] = Some(generator.create_constant_wire(0x80, &None));
            for i in 1..pad.len() - 8 {
                pad[i] = generator.get_zero_wire();
            }
            let length_in_bits = self.t.total_length_in_bytes * 8;
            let mut length_bits = vec![None; 64];
            let pn = pad.len();
            for i in 0..8 {
                pad[pn - 1 - i] = Some(
                    generator
                        .create_constant_wire(((length_in_bits >> (8 * i)) & 0xFF) as i64, &None),
                );
                let tmp = pad[pn - 1 - i].as_ref().unwrap().get_bit_wiresi(8, &None);
                let tmp = tmp.as_array();
                length_bits[(7 - i) * 8..(7 - i + 1) * 8].clone_from_slice(&tmp);
            }
            let total_number_of_bits = self.t.num_blocks * 512;
            let mut prepared_input_bits = vec![generator.get_zero_wire(); total_number_of_bits];
            let len = self.t.total_length_in_bytes * 8;
            prepared_input_bits[..len].clone_from_slice(&bits[..len]);
            prepared_input_bits[len + 7] = generator.get_one_wire();
            let n = prepared_input_bits.len();
            prepared_input_bits[n - 64..].clone_from_slice(&length_bits[..64]);
            self.t.prepared_input_bits = prepared_input_bits;
        } else {
            self.t.prepared_input_bits = bits.clone();
        }
    }
    pub fn super_get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}

impl GadgetConfig for Gadget<SHA256Gadget<Base>> {
    //outputs digest as 32-bit words

    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.output
    }
}
