#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// from enum::IntEnum
// from typing::Optional, Any

use crate::types::AddressValue;
use privacy::library_contracts::BN128_SCALAR_FIELDS;
use zkp_u256::{U256, Zero};
fn __convert(val: i32, nbits: Option<i32>, signed: bool) -> U256 {
    // if isinstance(val, IntEnum)
    //     val = val.value
    // elif isinstance(val, AddressValue):
    //     val = int.from_bytes(val.val, byteorder='big')

    if let Some(nbits) = nbits {
        let mut trunc_val = val & ((1 << nbits) - 1); // # unsigned representation
        if signed && trunc_val & (1 << (nbits - 1)) != 0 {
            trunc_val -= 1 << nbits; //# signed representation
        }
        trunc_val.into()
    } else {
        //# modulo field prime
        U256::from(val) % BN128_SCALAR_FIELDS.clone()
    }
}
// type ConvertCallables = Box<fn(i32, Option<i32>) -> U256>;
// use std::collections::BTreeMap;
// use std::sync::OnceLock;
// static globalss: OnceLock<BTreeMap<String, ConvertCallables>> = OnceLock::new();
fn globals(t: &str, x: i32) -> U256 {
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
