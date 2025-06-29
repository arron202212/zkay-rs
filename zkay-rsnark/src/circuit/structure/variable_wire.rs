#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::structure::{
        circuit_generator::CircuitGenerator,
        wire::{GeneratorConfig, GetWireId, Wire, WireConfig, setBitsConfig},
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
    pub bitWires: Option<WireArray>,
}
impl Hash for VariableWire {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bitWires.hash(state);
    }
}
impl PartialEq for VariableWire {
    fn eq(&self, other: &Self) -> bool {
        self.bitWires == other.bitWires
    }
}
crate::impl_hash_code_of_wire_g_for!(Wire<VariableWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<VariableWire>);
pub fn new_variable(wireId: i32, generator: WeakCell<CircuitGenerator>) -> Wire<VariableWire> {
    // super(wireId);
    Wire::<VariableWire> {
        wireId,
        generator,
        t: VariableWire { bitWires: None },
    }
}
impl setBitsConfig for VariableWire {}
impl setBitsConfig for Wire<VariableWire> {}
impl WireConfig for Wire<VariableWire> {
    fn getBitWires(&self) -> Option<WireArray> {
        self.t.bitWires.clone()
    }
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::Variable(self.clone()))
    }
}
impl Wire<VariableWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self {
    //         bitWires: RcCell::new(None),
    //     }
    // }
    // fn getBitWires(&self) -> Option<WireArray> {
    //     self.t.bitWires.borrow().clone()
    // }

    fn setBits(&mut self, bitWires: Option<WireArray>) {
        self.t.bitWires = bitWires;
    }
}
