#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct LinearCombinationBitWire;
impl setBitsConfig for LinearCombinationBitWire {}
impl LinearCombinationBitWire {
    pub fn new(wireId: i32) -> Self {
        // super(wireId);
        Self
    }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(vec![Some(WireType::LinearCombinationBit(self.clone()))])
    }
}
