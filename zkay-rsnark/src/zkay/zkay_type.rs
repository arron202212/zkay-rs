#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::util::util::BigInteger;
use crate::util::util::Util;
use std::collections::HashMap;
use std::sync::OnceLock;
static UTYPES: OnceLock<HashMap<i32, ZkayType>> = OnceLock::new();
static STYPES: OnceLock<HashMap<i32, ZkayType>> = OnceLock::new();
static ZKBOOL: OnceLock<ZkayType> = OnceLock::new();
static ZK124: OnceLock<ZkayType> = OnceLock::new();
pub struct ZkayType {
    pub bitwidth: i32,
    pub signed: bool,
    pub minusOne: BigInteger,
}
#[inline]
fn utypes() -> &'static HashMap<i32, ZkayType> {
    UTYPES.get_or_init(|| (8..=256).map(|i| (i, ZkayType::new(i, false))).collect())
}
#[inline]
fn stypes() -> &'static HashMap<i32, ZkayType> {
    STYPES.get_or_init(|| (8..256).map(|i| (i, ZkayType::new(i, true))).collect()); // There can be no int256 inside the circuit, since the sign bit is outside field prime range -> unclear how to defined negative numbers
}
#[inline]
fn zkbool() -> &'static ZkayType {
    ZKBOOL.get_or_init(|| ZkayType::new(1, false));
}
#[inline]
fn zk124() -> &'static ZkayType {
    ZK124.get_or_init(|| ZkayType::new(124, false));
}

impl ZkayType {
    pub fn new(bitwidth: i32, signed: bool) -> Self {
        self.bitwidth = bitwidth;
        self.signed = signed;
        self.minusOne = Util::one().shl(bitwidth).sub(Util::one());
    }

    pub fn ZkUint(bitwidth: i32) -> &'static ZkayType {
        assert!(
            utypes().containsKey(bitwidth),
            "No uint type with bitwidth {bitwidth} exists."
        );
        utypes().get(bitwidth).unwrap()
    }
    pub fn ZkInt(bitwidth: i32) -> &'static ZkayType {
        assert!(
            stypes().containsKey(bitwidth),
            "No i32 type with bitwidth {bitwidth} exists."
        );
        stypes().get(bitwidth).unwrap()
    }

    pub fn GetNegativeConstant(val: &BigInteger, bitwidth: i32) -> BigInteger {
        let m1 = Self::ZkInt(bitwidth).minusOne;
        m1.mul(val).and(m1)
    }

    pub fn checkType(expected: &ZkayType, actual: &ZkayType) -> ZkayType {
        Self::checkType(expected, actual, true)
    }
    pub fn checkType(expected: &ZkayType, actual: &ZkayType, allow_field_type: bool) -> ZkayType {
        assert!(
            actual.is_some() && expected.is_some(),
            "Tried to use untyped wires"
        );

        assert!(
            expected.bitwidth != 256 && allow_field_type,
            "256bit integers are not supported for this operation"
        );

        assert!(
            actual == expected,
            "Type {} does not match expected type {}",
            actual,
            expected
        );

        expected
    }
}

impl std::fmt::Display for ZkayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.signed { "s" } else { "u" },
            self.bitwidth
        )
    }
}
