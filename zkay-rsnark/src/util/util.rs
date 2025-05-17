#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire_type::WireType;
use std::collections::HashMap;
use std::ops::Rem;
use std::ops::Add;
use rand::Rng;
// use rand::distr::{Distribution, StandardUniform};
 use rand::distributions::Distribution;
use num_traits::{sign::Signed,One};
use num_bigint::{BigInt,RandomBits,ToBigInt, RandBigInt,Sign};
pub type BigInteger = BigInt;
// let mut rng = rand::thread_rng();
pub struct Util{
    // seeded by 1 for testing purposes
    rand: RandomBits,
}
impl  Default for Util {
    fn default() -> Self {
        Self {
            rand: RandomBits::new(256),
        }
    }
}
impl Util {
    pub fn one()->BigInteger{
        BigInteger::one()
    }
    pub fn split(x: BigInteger, chunkSize: i32) -> Vec<BigInteger> {
        let numChunks = std::cmp::max(1, (x.bitLength() + chunkSize - 1) / chunkSize); // ceil(x.bitLength() / chunkSize)
        return Self::split(x, numChunks, chunkSize);
    }

    pub fn spliti(x: BigInteger, numChunks: i32, chunkSize: i32) -> Vec<BigInteger> {
        let chunks = vec![BigInteger::default(); numChunks as usize];
        let mask = Util::one().shiftLeft(chunkSize).subtract(Util::one());
        for i in 0..numChunks {
            chunks[i as usize] = x.shiftRight(chunkSize * i).and(mask);
        }
        return chunks;
    }

    pub fn combine(table: Vec<BigInteger>, blocks: Vec<WireType>, bitwidth: i32) -> BigInteger {
        let sum = BigInteger::ZERO;
        for i in 0..blocks.len() {
            if table[blocks[i].getWireId()] == None {
                continue;
            }
            sum = sum.add(table[blocks[i].getWireId()].shiftLeft(bitwidth * i));
        }
        return sum;
    }

    pub fn group(list: Vec<BigInteger>, width: i32) -> BigInteger {
        let x = BigInteger::ZERO;
        for i in 0..list.len() {
            x = x.add(list[i].shiftLeft(width * i));
        }
        return x;
    }

    pub fn concati(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
        let all = vec![i32::default(); a1.len() + a2.len()];
        for i in 0..all.len() {
            all[i] = if i < a1.len() {
                a1[i]
            } else {
                a2[i - a1.len()]
            };
        }
        return all;
    }

    pub fn concat(a1: Vec<WireType>, a2: Vec<WireType>) -> Vec<WireType> {
        let all = vec![WireType::default(); a1.len() + a2.len()];
        for i in 0..all.len() {
            all[i] = if i < a1.len() {
                a1[i]
            } else {
                a2[i - a1.len()]
            };
        }
        return all;
    }

    pub fn concat(w: WireType, a: Vec<WireType>) -> Vec<WireType> {
        let all = vec![WireType::default(); 1 + a.len()];
        for i in 0..all.len() {
            all[i] = if i < 1 { w } else { a[i - 1] };
        }
        return all;
    }

    pub fn concat(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let sum = 0;
        for array in arrays {
            sum += array.len();
        }
        let all = vec![i32::default(); sum];
        let idx = 0;
        for array in arrays {
            for a in array {
                all[idx] = a;
                idx += 1;
            }
        }
        return all;
    }

    pub fn randomBigIntegerArray(num: i32, n: BigInteger) -> Vec<BigInteger> {
        let result = vec![BigInteger::default(); num as usize];
        for i in 0..num {
            result[i as usize] = nextRandomBigInteger(n);
        }
        return result;
    }

    pub fn nextRandomBigInteger(&self,n: BigInteger) -> BigInteger {
        let rand=RandomBits::new(n.bitLength());
        let result = rand.sample(rand::thread_rng());
        while result.compareTo(n) >= 0 {
            result =  rand.sample(rand::thread_rng());
        }
        return result;
    }

    pub fn randomBigIntegerArrayi(&self,num: i32, numBits: i32) -> Vec<BigInteger> {
        let result = vec![BigInteger::default(); num as usize];
        for i in 0..num {
            result[i as usize] = self.nextRandomBigInteger(BigInteger::from(numBits as u32));
        }
        return result;
    }

    pub fn nextRandomBigInteger(&self,numBits: i32) -> BigInteger {
        return  RandomBits::new(numBits as u64).sample(rand::thread_rng());
    }

    pub fn getDesc(desc: Vec<String>) -> String {
        if desc.len() == 0 {
            return "".to_owned();
        } else {
            return desc[0].clone();
        }
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

    pub fn reverseBytes(inBitWires: Vec<WireType>) -> Vec<WireType> {
        let outs = inBitWires.clone();
        let numBytes = inBitWires.len() / 8;
        for i in 0..numBytes / 2 {
            let other = numBytes - i - 1;
            for j in 0..8 {
                let temp = outs[i * 8 + j];
                outs[i * 8 + j] = outs[other * 8 + j];
                outs[other * 8 + j] = temp;
            }
        }
        return outs;
    }

    pub fn arrayToStringi(a: Vec<i32>, separator: String) -> String {
        let s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(&a[i].to_string());
            s.push_str(&separator);
        }
        s.push_str(&a[a.len() - 1].to_string());
        return s;
    }

    pub fn arrayToString(a: Vec<WireType>, separator: String) -> String {
        let s = String::new();
        for i in 0..a.len() - 1 {
            s.push_str(a[i].to_string());
            s.push_str(&separator);
        }
        s.push_str(a[a.len() - 1].to_string());
        return s;
    }

    pub fn isBinary(v: BigInteger) -> bool {
        return v==BigInteger::ZERO || v==Util::one();
    }

    pub fn padZeros(s: String, l: usize) -> String {
        return format!("{s:0>l$}");
    }

    // Computation is cheap, keeping lots of BigIntegers in memory likely isn't, so use a weak hash map

    pub fn computeMaxValue(numBits: i32) -> BigInteger {
        let maxValueCache = HashMap::new();
        return maxValueCache.entry(numBits).or_insert_with_key(|i| Util::one().shiftLeft(i).subtract(Util::one()));
    }

    pub fn computeBound(numBits: i32) -> BigInteger {
        let boundCache = HashMap::new();
        return boundCache.entry(numBits).or_insert_with(|| Util::one().shiftLeft(numBits));
    }

    pub fn padWireArray(a: Vec<WireType>, length: usize, p: WireType) -> Vec<WireType> {
        if a.len() == length {
            return a;
        } else if a.len() > length {
            println!("No padding needed!");
            return a;
        } else {
            let newArray = vec![WireType::default(); length];
            newArray[..a.len()].clone_from_slice(&a);
            for k in a.len()..length {
                newArray[k] = p;
            }
            return newArray;
        }
    }

    pub fn modulo(x: BigInteger, m: BigInteger) -> BigInteger {
        if x.sign() != Sign::Minus && x<m {
            return x; // In range, 'mod' is no-op, but creates new BigInteger
        } else {
            return x.rem(m);
        }
    }
}
