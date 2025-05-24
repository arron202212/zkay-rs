#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use rccell::RcCell;

use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct VariableWire {
    pub bitWires: RcCell<Option<WireArray>>,
}
impl setBitsConfig for VariableWire {}
impl VariableWire {
    pub fn new(wireId: i32) -> Self {
        // super(wireId);
        Self {
            bitWires: RcCell::new(None),
        }
    }
    fn getBitWires(&self) -> Option<WireArray> {
        self.bitWires.borrow().clone()
    }

    fn setBits(&self, bitWires: Option<WireArray>) {
        *self.bitWires.borrow_mut() = bitWires;
    }
}
