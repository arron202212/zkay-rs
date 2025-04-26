#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// from enum::IntEnum
// from typing::Optional, Any

use crate::types::{AddressValue, DataType};
use privacy::library_contracts::BN128_SCALAR_FIELD;
// use zkp_u256::{U256, Zero};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
pub fn __convert(val: DataType, nbits: Option<i32>, signed: bool) -> String {
    // if isinstance(val, IntEnum)
    //     val = val.value
    // elif isinstance(val, AddressValue):
    //     val = int.from_bytes(val.val, byteorder='big')
    let val = match val {
        DataType::Int(v) => v.to_string(),
        DataType::String(v) => v.clone(),
        DataType::AddressValue(v) => v.contents[0].clone(),
        _ => return val.to_string(),
    };
    let Some(val) = BigInt::parse_bytes(val.as_bytes(), 10) else {
        return val;
    };
    if let Some(nbits) = nbits {
        let mut trunc_val = val & ((BigInt::one() << nbits) - BigInt::one()); // # unsigned representation
        if signed && (trunc_val.clone() & (BigInt::one() << (nbits - 1))) != BigInt::zero() {
            trunc_val -= BigInt::one() << nbits; //# signed representation
        }
        trunc_val.to_string()
    } else {
        //# modulo field prime
        (val % BigInt::parse_bytes(BN128_SCALAR_FIELD.as_bytes(), 10).unwrap()).to_string()
    }
}
// type ConvertCallables = Box<fn(i32, Option<i32>) -> U256>;
// use std::collections::BTreeMap;
// use std::sync::OnceLock;
// static globalss: OnceLock<BTreeMap<String, ConvertCallables>> = OnceLock::new();
pub fn globals(t: &str, x: DataType) -> String {
    match t {
        "uint" => __convert(x, None, false),
        t if t.starts_with("uint") => {
            let i = t[4..].parse::<i32>().unwrap();
            assert!(i % 8 == 0, "illegal type {t}");
            __convert(x, Some(i), false)
        }
        t if t.starts_with("int") => {
            let i = t[4..].parse::<i32>().unwrap();
            assert!(i % 8 == 0, "illegal type {t}");
            __convert(x, Some(i), true)
        }
        _ => {
            unreachable!("{t}")
        }
    }
}
