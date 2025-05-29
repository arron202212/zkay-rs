#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::bit_wire::BitWireConfig;
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct LinearCombinationBitWire;
crate::impl_hash_code_of_wire_for!(Wire<LinearCombinationBitWire>);
crate::impl_name_instance_of_wire_for!(Wire<LinearCombinationBitWire>);
pub fn new_linear_combination_bit(wireId: i32) -> Wire<LinearCombinationBitWire> {
    // super(wireId);
    Wire::<LinearCombinationBitWire> {
        wireId,
        t: LinearCombinationBitWire,
    }
}
impl setBitsConfig for LinearCombinationBitWire {}
impl setBitsConfig for Wire<LinearCombinationBitWire> {}
impl WireConfig for Wire<LinearCombinationBitWire> {
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::LinearCombinationBit(self.clone()))
    }
}
impl BitWireConfig for Wire<LinearCombinationBitWire> {}
impl Wire<LinearCombinationBitWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self
    // }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(vec![Some(WireType::LinearCombinationBit(self.clone()))])
    }
}
