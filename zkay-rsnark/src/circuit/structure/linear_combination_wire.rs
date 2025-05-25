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
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct LinearCombinationWire {
    pub bitWires: RcCell<Option<WireArray>>,
}
pub fn new_linear_combination(wireId: i32, bits: Option<WireArray>) -> Wire<LinearCombinationWire> {
    // super(wireId);
    Wire::<LinearCombinationWire> {
        wireId,
        t: LinearCombinationWire {
            bitWires: RcCell::new(bits),
        },
    }
}
impl setBitsConfig for LinearCombinationWire {}
impl setBitsConfig for Wire<LinearCombinationWire> {}
impl Wire<LinearCombinationWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // super(wireId);
    //     Self {
    //         bitWires: RcCell::new(None),
    //     }
    // }

    // pub fn newa(bits: WireArray) -> Self {
    //     // super(bits);
    //     Self {
    //         bitWires: RcCell::new(Some(bits)),
    //     }
    // }

    fn getBitWires(&self) -> Option<WireArray> {
        self.t.bitWires.borrow().clone()
    }

    fn setBits(&self, bitWires: Option<WireArray>) {
        *self.t.bitWires.borrow_mut() = bitWires;
    }
}
