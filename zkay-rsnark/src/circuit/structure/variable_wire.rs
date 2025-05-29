#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use rccell::RcCell;

use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct VariableWire {
    pub bitWires: RcCell<Option<WireArray>>,
}
crate::impl_hash_code_of_wire_for!(Wire<VariableWire>);
crate::impl_name_instance_of_wire_for!(Wire<VariableWire>);
pub fn new_variable(wireId: i32) -> Wire<VariableWire> {
    // super(wireId);
    Wire::<VariableWire> {
        wireId,
        t: VariableWire {
            bitWires: RcCell::new(None),
        },
    }
}
impl setBitsConfig for VariableWire {}
impl setBitsConfig for Wire<VariableWire> {}
impl WireConfig for Wire<VariableWire> {
    fn getBitWires(&self) -> Option<WireArray> {
        self.t.bitWires.borrow().clone()
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

    fn setBits(&self, bitWires: Option<WireArray>) {
        *self.t.bitWires.borrow_mut() = bitWires;
    }
}
