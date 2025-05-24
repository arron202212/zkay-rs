#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use rand::Rng;
use std::collections::HashMap;
use std::ops::{Add, BitAnd, Mul, Rem, Shl, Shr, Sub};
// use rand::distr::{Distribution, StandardUniform};
use num_bigint::{BigInt, RandBigInt, RandomBits, Sign, ToBigInt};
use num_traits::{One, sign::Signed};
use rand::distributions::Distribution;
pub type BigInteger = BigInt;
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
    pub fn one() -> BigInteger {
        BigInteger::one()
    }
    pub fn split(x: BigInteger, chunkSize: i32) -> Vec<BigInteger> {
        let numChunks = std::cmp::max(1, (x.bits() + chunkSize as u64 - 1) / chunkSize as u64); // ceil(x.bits() / chunkSize)
        return Self::spliti(x, numChunks as i32, chunkSize);
    }

    pub fn spliti(x: BigInteger, numChunks: i32, chunkSize: i32) -> Vec<BigInteger> {
        let mut chunks = vec![BigInteger::default(); numChunks as usize];
        let mask = Util::one().shl(chunkSize).sub(Util::one());
        for i in 0..numChunks {
            chunks[i as usize] = x.clone().shr(chunkSize * i).bitand(mask.clone());
        }
        return chunks;
    }

    pub fn combine(
        table: Vec<Option<BigInteger>>,
        blocks: Vec<Option<WireType>>,
        bitwidth: i32,
    ) -> BigInteger {
        let mut sum = BigInteger::ZERO;
        for i in 0..blocks.len() {
            if table[blocks[i].as_ref().unwrap().getWireId() as usize] == None {
                continue;
            }
            sum = sum.add(
                table[blocks[i].as_ref().unwrap().getWireId() as usize]
                    .as_ref()
                    .unwrap()
                    .shl(bitwidth as usize * i),
            );
        }
        return sum;
    }

    pub fn group(list: Vec<BigInteger>, width: i32) -> BigInteger {
        let mut x = BigInteger::ZERO;
        for i in 0..list.len() {
            x = x.add(list[i].clone().shl(width as usize * i));
        }
        return x;
    }

    pub fn concati(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
        let mut all = vec![i32::default(); a1.len() + a2.len()];
        for i in 0..all.len() {
            all[i] = if i < a1.len() {
                a1[i]
            } else {
                a2[i - a1.len()]
            };
        }
        return all;
    }

    pub fn concat(a1: Vec<Option<WireType>>, a2: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let mut all = vec![None; a1.len() + a2.len()];
        for i in 0..all.len() {
            all[i] = if i < a1.len() {
                a1[i].clone()
            } else {
                a2[i - a1.len()].clone()
            };
        }
        return all;
    }

    pub fn concata(w: WireType, a: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let mut all = vec![None; 1 + a.len()];
        for i in 0..all.len() {
            all[i] = if i < 1 {
                Some(w.clone())
            } else {
                a[i - 1].clone()
            };
        }
        return all;
    }

    pub fn concataa(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = 0;
        for array in &arrays {
            sum += array.len();
        }
        let mut all = vec![i32::default(); sum];
        let mut idx = 0;
        for array in &arrays {
            for &a in array {
                all[idx] = a;
                idx += 1;
            }
        }
        return all;
    }

    pub fn randomBigIntegerArray(&self, num: i32, n: BigInteger) -> Vec<BigInteger> {
        let mut result = vec![BigInteger::default(); num as usize];
        for i in 0..num {
            result[i as usize] = self.nextRandomBigInteger(n.clone());
        }
        return result;
    }

    pub fn nextRandomBigInteger(&self, n: BigInteger) -> BigInteger {
        let rand = RandomBits::new(n.bits());
        let mut result = rand.sample(&mut rand::thread_rng());
        while result >= n {
            result = rand.sample(&mut rand::thread_rng());
        }
        return result;
    }

    pub fn randomBigIntegerArrayi(&self, num: i32, numBits: i32) -> Vec<BigInteger> {
        let mut result = vec![BigInteger::default(); num as usize];
        for i in 0..num {
            result[i as usize] = self.nextRandomBigInteger(BigInteger::from(numBits as u32));
        }
        return result;
    }

    pub fn nextRandomBigIntegeri(&self, numBits: i32) -> BigInteger {
        return RandomBits::new(numBits as u64).sample(&mut rand::thread_rng());
    }

    pub fn getDesc(desc: &String) -> String {
        desc.clone()
    }

    pub fn parseSequenceLists(s: String) -> Vec<i32> {
        let mut list = Vec::new();
        let chunks = s.split(",");
        for chunk in chunks {
            if chunk.is_empty() {
                continue;
            }
            let c: Vec<_> = chunk.split(":").collect();
            let lower = c[0].parse::<i32>().unwrap();
            let upper = c[1].parse::<i32>().unwrap();
            for i in lower..=upper {
                list.push(i);
            }
        }
        return list;
    }

    pub fn reverseBytes(inBitWires: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
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
        return outs;
    }

    pub fn arrayToStringi(a: Vec<i32>, separator: String) -> String {
        let mut s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].to_string());
            s.push_str(&separator);
        }
        s.push_str(&a[a.len() - 1].to_string());
        return s;
    }

    pub fn arrayToString(a: Vec<Option<WireType>>, separator: String) -> String {
        let mut s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].as_ref().unwrap().to_string());
            s.push_str(&separator);
        }
        s.push_str(&a[a.len() - 1].as_ref().unwrap().to_string());
        return s;
    }

    pub fn isBinary(v: BigInteger) -> bool {
        return v == BigInteger::ZERO || v == Util::one();
    }

    pub fn padZeros(s: String, l: usize) -> String {
        return format!("{s:0>l$}");
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
        a: Vec<Option<WireType>>,
        length: usize,
        p: WireType,
    ) -> Vec<Option<WireType>> {
        if a.len() == length {
            return a;
        } else if a.len() > length {
            println!("No padding needed!");
            return a;
        } else {
            let mut newArray = vec![None; length];
            newArray[..a.len()].clone_from_slice(&a);
            newArray[a.len()..length].fill(Some(p.clone()));
            return newArray;
        }
    }

    pub fn modulo(x: BigInteger, m: BigInteger) -> BigInteger {
        if x.sign() != Sign::Minus && x < m {
            return x; // In range, 'mod' is no-op, but creates new BigInteger
        } else {
            return x.rem(m);
        }
    }
}
