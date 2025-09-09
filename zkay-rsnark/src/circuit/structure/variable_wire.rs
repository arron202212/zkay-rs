#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::structure::{
        circuit_generator::CircuitGenerator,
        wire::{GeneratorConfig, GetWireId, SetBitsConfig, Wire, WireConfig},
        wire_array::WireArray,
        wire_type::WireType,
    },
    util::util::ARcCell,
};
use rccell::{RcCell, WeakCell};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct VariableWire {
    pub bit_wires: Option<WireArray>,
}

impl PartialEq for VariableWire {
    fn eq(&self, other: &Self) -> bool {
        self.bit_wires == other.bit_wires
    }
}
crate::impl_name_instance_of_wire_g_for!(Wire<VariableWire>);
impl VariableWire {
    pub fn new(wire_id: i32, generator: WeakCell<CircuitGenerator>) -> Wire<VariableWire> {
        Wire::<VariableWire>::new(VariableWire { bit_wires: None }, wire_id, generator).unwrap()
    }
}
impl SetBitsConfig for VariableWire {}
impl SetBitsConfig for Wire<VariableWire> {}
impl WireConfig for Wire<VariableWire> {
    fn get_bit_wires(&self) -> Option<WireArray> {
        self.t.bit_wires.clone()
    }
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::Variable(self.clone()))
    }
}
impl Wire<VariableWire> {
    fn set_bits(&mut self, bit_wires: Option<WireArray>) {
        self.t.bit_wires = bit_wires;
    }
}
