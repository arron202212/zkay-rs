#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::{
    bit_wire::BitWireConfig,
    circuit_generator::CircuitGenerator,
    wire::{GeneratorConfig, GetWireId, Wire, WireConfig, setBitsConfig},
    wire_array::WireArray,
    wire_type::WireType,
};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

use rccell::{RcCell, WeakCell};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct VariableBitWire;
crate::impl_hash_code_of_wire_g_for!(Wire<VariableBitWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<VariableBitWire>);
pub fn new_variable_bit(
    wireId: i32,
    generator: WeakCell<CircuitGenerator>,
) -> Wire<VariableBitWire> {
    // super(wireId);
    // Wire::<VariableBitWire> {
    //     wireId,
    //     generator,
    //     t: VariableBitWire,
    // }
    // crate::new_wire!(VariableBitWire,wireId,generator)
    Wire::<VariableBitWire>::new(VariableBitWire, wireId, generator).unwrap()
}
impl setBitsConfig for VariableBitWire {}
impl setBitsConfig for Wire<VariableBitWire> {}
impl WireConfig for Wire<VariableBitWire> {
    fn getBitWires(&self) -> Option<WireArray> {
        Some(WireArray::new(
            vec![Some(WireType::VariableBit(self.clone()))],
            self.generator.clone(),
        ))
    }
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::VariableBit(self.clone()))
    }
}
impl BitWireConfig for Wire<VariableBitWire> {}
impl Wire<VariableBitWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self
    // }
}
