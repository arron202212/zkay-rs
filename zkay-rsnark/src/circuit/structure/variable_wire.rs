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
use rccell::RcCell;

use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct VariableWire {
    pub bitWires: RcCell<Option<WireArray>>,
}
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
impl Wire<VariableWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self {
    //         bitWires: RcCell::new(None),
    //     }
    // }
    fn getBitWires(&self) -> Option<WireArray> {
        self.t.bitWires.borrow().clone()
    }

    fn setBits(&self, bitWires: Option<WireArray>) {
        *self.t.bitWires.borrow_mut() = bitWires;
    }
}
