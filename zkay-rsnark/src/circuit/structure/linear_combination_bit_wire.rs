#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::wire::{Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct LinearCombinationBitWire;
pub fn new_linear_combination_bit(wireId: i32) -> Wire<LinearCombinationBitWire> {
    // super(wireId);
    Wire::<LinearCombinationBitWire> {
        wireId,
        t: LinearCombinationBitWire,
    }
}
impl setBitsConfig for LinearCombinationBitWire {}
impl setBitsConfig for Wire<LinearCombinationBitWire> {}
impl Wire<LinearCombinationBitWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self
    // }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(vec![Some(WireType::LinearCombinationBit(self.clone()))])
    }
}
