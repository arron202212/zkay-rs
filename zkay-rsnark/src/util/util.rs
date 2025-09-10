#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::{wire::GetWireId, wire_type::WireType};

use std::{
    collections::HashMap,
    ops::{Add, BitAnd, Rem, Shl, Shr, Sub},
    sync::Arc,
};

// use rand::distr::{Distribution, StandardUniform};
use num_bigint::{BigInt, RandomBits, Sign};
use num_traits::One;
use parking_lot::Mutex;
use rand::distributions::Distribution;

pub type BigInteger = BigInt;

pub type ARcCell<Typ> = Arc<Mutex<Typ>>;
#[macro_export]
macro_rules! arc_cell_new {
    ($exp:expr) => {{ std::sync::Arc::new(parking_lot::Mutex::new($exp)) }};
}

// let mut rng = rand::thread_rng();
pub struct Util {
    // seeded by 1 for testing purposes
    pub rand: RandomBits,
}
impl Default for Util {
    fn default() -> Self {
        Self {
            rand: RandomBits::new(256),
        }
    }
}
impl Util {
    #[inline]
    pub fn parse_big_int(s: &str) -> BigInteger {
        BigInteger::parse_bytes(s.as_bytes(), 10).unwrap()
    }
    #[inline]
    pub fn parse_big_int_x(s: &str) -> BigInteger {
        BigInteger::parse_bytes(s.as_bytes(), 16).unwrap()
    }
    #[inline]
    pub fn parse_bytes(s: &[u8]) -> BigInteger {
        BigInteger::parse_bytes(s, 10).unwrap()
    }
    #[inline]
    pub fn parse_bytes_x(s: &[u8]) -> BigInteger {
        BigInteger::parse_bytes(s, 16).unwrap()
    }
    #[inline]
    pub fn one() -> BigInteger {
        BigInteger::one()
    }
    pub fn split(x: &BigInteger, chunk_size: i32) -> Vec<BigInteger> {
        let num_chunks = 1i32.max((x.bits() as i32 + chunk_size - 1) / chunk_size); // ceil(x.bits() / chunk_size)
        Self::spliti(x, num_chunks, chunk_size)
    }

    pub fn spliti(x: &BigInteger, num_chunks: i32, chunk_size: i32) -> Vec<BigInteger> {
        let mask = Util::one().shl(chunk_size).sub(Util::one());
        (0..num_chunks)
            .map(|i| x.clone().shr(chunk_size * i).bitand(&mask))
            .collect()
    }

    pub fn combine(
        table: &Vec<Option<BigInteger>>,
        blocks: &Vec<Option<WireType>>,
        bitwidth: i32,
    ) -> BigInteger {
        let mut sum = BigInteger::ZERO;

        for i in 0..blocks.len() {
            if table[blocks[i].as_ref().unwrap().get_wire_id() as usize].is_none() {
                continue;
            }
            sum = sum.add(
                table[blocks[i].as_ref().unwrap().get_wire_id() as usize]
                    .as_ref()
                    .unwrap()
                    .shl(bitwidth as usize * i),
            );
        }
        sum
    }

    pub fn group(list: &Vec<BigInteger>, width: i32) -> BigInteger {
        let w = width as usize;
        list.iter()
            .enumerate()
            .fold(BigInteger::ZERO, |s, (i, x)| s.add(x.clone().shl(w * i)))
    }

    pub fn concati(a1: &Vec<i32>, a2: &Vec<i32>) -> Vec<i32> {
        a1.iter().chain(a2).cloned().collect()
    }

    pub fn concat(a1: &Vec<Option<WireType>>, a2: &Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        a1.iter().chain(a2).cloned().collect()
    }

    pub fn concata(w: &WireType, a: &Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let mut all = a.clone();
        all.insert(0, Some(w.clone()));
        all
    }

    pub fn concataa(arrays: &Vec<Vec<i32>>) -> Vec<i32> {
        arrays.iter().cloned().flatten().collect()
    }

    pub fn random_big_integer_array(num: u64, n: &BigInteger) -> Vec<BigInteger> {
        (0..num as usize)
            .map(|_| Self::next_random_big_integer(n))
            .collect()
    }

    pub fn next_random_big_integer(n: &BigInteger) -> BigInteger {
        let rand = RandomBits::new(n.bits());
        let mut result: BigInteger = rand.sample(&mut rand::thread_rng());
        while result.sign() == Sign::Minus || &result >= n {
            result = rand.sample(&mut rand::thread_rng());
        }
        result
    }

    pub fn random_big_integer_arrayi(num: u64, num_bits: i32) -> Vec<BigInteger> {
        (0..num as usize)
            .map(|_| Self::next_random_big_integeri(num_bits as u64))
            .collect()
    }

    pub fn next_random_big_integeri(num_bits: u64) -> BigInteger {
        let rand = RandomBits::new(num_bits);
        let mut result: BigInteger = rand.sample(&mut rand::thread_rng());
        while result.sign() == Sign::Minus {
            result = rand.sample(&mut rand::thread_rng());
        }
        result
    }

    pub fn get_desc(desc: &Option<String>) -> String {
        desc.clone().unwrap_or(String::new())
    }

    pub fn parse_sequence_lists(s: String) -> Vec<i32> {
        s.split(",")
            .filter_map(|c| {
                (!c.is_empty()).then(|| {
                    let r: Vec<_> = c.split(":").filter_map(|v| v.parse::<i32>().ok()).collect();
                    (r[0]..=r[1]).collect::<Vec<_>>()
                })
            })
            .flatten()
            .collect()
    }

    pub fn reverse_bytes(in_bit_wires: &Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let mut outs = in_bit_wires.clone();
        let num_bytes = in_bit_wires.len() / 8;
        for i in 0..num_bytes / 2 {
            let other = num_bytes - i - 1;
            for j in 0..8 {
                let temp = outs[i * 8 + j].clone();
                outs[i * 8 + j] = outs[other * 8 + j].clone();
                outs[other * 8 + j] = temp;
            }
        }
        outs
    }

    pub fn array_to_stringi(a: &Vec<i32>, separator: &String) -> String {
        let mut s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].to_string());
            s.push_str(separator);
        }
        s.push_str(&a[a.len() - 1].to_string());
        s
    }

    pub fn array_to_string(a: &Vec<Option<WireType>>, separator: &String) -> String {
        let mut s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].as_ref().unwrap().to_string());
            s.push_str(separator);
        }
        s.push_str(&a[a.len() - 1].as_ref().unwrap().to_string());
        s
    }

    pub fn is_binary(v: &BigInteger) -> bool {
        v == &BigInteger::ZERO || v == &Util::one()
    }

    pub fn pad_zeros(s: &String, l: usize) -> String {
        format!("{s:0>l$}")
    }

    // Computation is cheap, keeping lots of BigIntegers in memory likely isn't, so use a weak hash map

    pub fn compute_max_value(num_bits: u64) -> BigInteger {
        let mut max_value_cache = HashMap::new();
        max_value_cache
            .entry(num_bits)
            .or_insert_with_key(|i| Util::one().shl(i).sub(Util::one()))
            .clone()
    }

    pub fn compute_bound(num_bits: i32) -> BigInteger {
        let mut bound_cache = HashMap::new();
        return bound_cache
            .entry(num_bits)
            .or_insert_with(|| Util::one().shl(num_bits))
            .clone();
    }

    pub fn pad_wire_array(
        a: &Vec<Option<WireType>>,
        length: usize,
        p: &WireType,
    ) -> Vec<Option<WireType>> {
        if a.len() >= length {
            if a.len() > length {
                println!("No padding needed!");
            }
            a.clone()
        } else {
            let mut new_array = vec![None; length];
            new_array[..a.len()].clone_from_slice(a);
            new_array[a.len()..length].fill(Some(p.clone()));
            new_array
        }
    }

    pub fn modulo(x: &BigInteger, m: &BigInteger) -> BigInteger {
        if x.sign() != Sign::Minus && x < m {
            x.clone() // In range, 'mod' is no-op, but creates new BigInteger
        } else {
            x.rem(m)
        }
    }
}
