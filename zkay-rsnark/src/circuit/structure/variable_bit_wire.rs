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
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct VariableBitWire;
pub fn new_variable_bit(wireId: i32) -> Wire<VariableBitWire> {
    // super(wireId);
    Wire::<VariableBitWire> {
        wireId,
        t: VariableBitWire,
    }
}
impl setBitsConfig for VariableBitWire {}
impl setBitsConfig for Wire<VariableBitWire> {}
impl Wire<VariableBitWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self
    // }

    pub fn getBitWires(&self) -> Option<WireArray> {
        Some(WireArray::new(vec![Some(WireType::VariableBit(
            self.clone(),
        ))]))
    }
}
