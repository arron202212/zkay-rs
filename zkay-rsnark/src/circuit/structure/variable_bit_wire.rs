#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::bit_wire::BitWireConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;

use crate::circuit::structure::wire::GeneratorConfig;
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use rccell::{RcCell, WeakCell};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
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
    Wire::<VariableBitWire> {
        wireId,
        generator,
        t: VariableBitWire,
    }
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
