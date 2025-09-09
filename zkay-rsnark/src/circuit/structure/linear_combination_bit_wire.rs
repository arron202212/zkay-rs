#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::{
    bit_wire::BitWireConfig,
    circuit_generator::CircuitGenerator,
    wire::{GeneratorConfig, GetWireId, SetBitsConfig, Wire, WireConfig},
    wire_array::WireArray,
    wire_type::WireType,
};

use rccell::{RcCell, WeakCell};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct LinearCombinationBitWire;
crate::impl_name_instance_of_wire_g_for!(Wire<LinearCombinationBitWire>);

impl LinearCombinationBitWire {
    pub fn new(
        wire_id: i32,
        generator: WeakCell<CircuitGenerator>,
    ) -> Wire<LinearCombinationBitWire> {
        Wire::<LinearCombinationBitWire>::new(LinearCombinationBitWire, wire_id, generator).unwrap()
    }
}
impl SetBitsConfig for LinearCombinationBitWire {}
impl SetBitsConfig for Wire<LinearCombinationBitWire> {}
impl WireConfig for Wire<LinearCombinationBitWire> {
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::LinearCombinationBit(self.clone()))
    }
}
impl BitWireConfig for Wire<LinearCombinationBitWire> {}
impl Wire<LinearCombinationBitWire> {
    pub fn get_bit_wires(&self) -> WireArray {
        WireArray::new(
            vec![Some(WireType::LinearCombinationBit(self.clone()))],
            self.generator.clone(),
        )
    }
}
