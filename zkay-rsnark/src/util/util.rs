#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;

use rand::Rng;
use std::collections::HashMap;
use std::ops::{Add, BitAnd, Mul, Rem, Shl, Shr, Sub};
use std::sync::Arc;
// use rand::distr::{Distribution, StandardUniform};
use num_bigint::{BigInt, RandBigInt, RandomBits, Sign, ToBigInt};
use num_traits::{One, sign::Signed};
use parking_lot::Mutex;
use rand::distributions::Distribution;

pub type BigInteger = BigInt;

pub type ARcCell<typ> = Arc<Mutex<typ>>;
#[macro_export]
macro_rules! arc_cell_new {
    ($exp:expr) => {{ std::sync::Arc::new(parking_lot::Mutex::new($exp)) }};
}

// let mut rng = rand::thread_rng();
pub struct Util {
    // seeded by 1 for testing purposes
    rand: RandomBits,
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
    pub fn one() -> BigInteger {
        BigInteger::one()
    }
    pub fn split(x: &BigInteger, chunkSize: i32) -> Vec<BigInteger> {
        let numChunks = std::cmp::max(1, (x.bits() + chunkSize as u64 - 1) / chunkSize as u64); // ceil(x.bits() / chunkSize)
        Self::spliti(x, numChunks as i32, chunkSize)
    }

    pub fn spliti(x: &BigInteger, numChunks: i32, chunkSize: i32) -> Vec<BigInteger> {
        let mask = Util::one().shl(chunkSize).sub(Util::one());
        (0..numChunks)
            .map(|i| x.clone().shr(chunkSize * i).bitand(&mask))
            .collect()
    }

    pub fn combine(
        table: &Vec<Option<BigInteger>>,
        blocks: &Vec<Option<WireType>>,
        bitwidth: i32,
    ) -> BigInteger {
        let mut sum = BigInteger::ZERO;

        for i in 0..blocks.len() {
            if table[blocks[i].as_ref().unwrap().getWireId() as usize].is_none() {
                continue;
            }
            sum = sum.add(
                table[blocks[i].as_ref().unwrap().getWireId() as usize]
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

    pub fn randomBigIntegerArray(num: u64, n: &BigInteger) -> Vec<BigInteger> {
        (0..num as usize)
            .map(|_| Self::nextRandomBigInteger(n))
            .collect()
    }

    pub fn nextRandomBigInteger(n: &BigInteger) -> BigInteger {
        let rand = RandomBits::new(n.bits());
        let mut result: BigInteger = rand.sample(&mut rand::thread_rng());
        while result.sign() == Sign::Minus || &result >= n {
            result = rand.sample(&mut rand::thread_rng());
        }
        result
    }

    pub fn randomBigIntegerArrayi(num: u64, numBits: i32) -> Vec<BigInteger> {
        (0..num as usize)
            .map(|_| Self::nextRandomBigIntegeri(numBits as u64))
            .collect()
    }

    pub fn nextRandomBigIntegeri(numBits: u64) -> BigInteger {
        let rand = RandomBits::new(numBits);
        let mut result: BigInteger = rand.sample(&mut rand::thread_rng());
        while result.sign() == Sign::Minus {
            result = rand.sample(&mut rand::thread_rng());
        }
        result
    }

    pub fn getDesc(desc: &Option<String>) -> String {
        desc.as_ref()
            .map_or_else(|| String::new(), |d| d.to_owned())
    }

    pub fn parseSequenceLists(s: String) -> Vec<i32> {
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

    pub fn reverseBytes(inBitWires: &Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let mut outs = inBitWires.clone();
        let numBytes = inBitWires.len() / 8;
        for i in 0..numBytes / 2 {
            let other = numBytes - i - 1;
            for j in 0..8 {
                let temp = outs[i * 8 + j].clone();
                outs[i * 8 + j] = outs[other * 8 + j].clone();
                outs[other * 8 + j] = temp;
            }
        }
        outs
    }

    pub fn arrayToStringi(a: &Vec<i32>, separator: &String) -> String {
        let mut s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].to_string());
            s.push_str(separator);
        }
        s.push_str(&a[a.len() - 1].to_string());
        s
    }

    pub fn arrayToString(a: &Vec<Option<WireType>>, separator: &String) -> String {
        let mut s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].as_ref().unwrap().to_string());
            s.push_str(separator);
        }
        s.push_str(&a[a.len() - 1].as_ref().unwrap().to_string());
        s
    }

    pub fn isBinary(v: &BigInteger) -> bool {
        v == &BigInteger::ZERO || v == &Util::one()
    }

    pub fn padZeros(s: &String, l: usize) -> String {
        format!("{s:0>l$}")
    }

    // Computation is cheap, keeping lots of BigIntegers in memory likely isn't, so use a weak hash map

    pub fn computeMaxValue(numBits: u64) -> BigInteger {
        let mut maxValueCache = HashMap::new();
        return maxValueCache
            .entry(numBits)
            .or_insert_with_key(|i| Util::one().shl(i).sub(Util::one()))
            .clone();
    }

    pub fn computeBound(numBits: i32) -> BigInteger {
        let mut boundCache = HashMap::new();
        return boundCache
            .entry(numBits)
            .or_insert_with(|| Util::one().shl(numBits))
            .clone();
    }

    pub fn padWireArray(
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
            let mut newArray = vec![None; length];
            newArray[..a.len()].clone_from_slice(a);
            newArray[a.len()..length].fill(Some(p.clone()));
            newArray
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
