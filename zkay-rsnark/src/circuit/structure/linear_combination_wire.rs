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
pub struct LinearCombinationWire {
    pub bitWires: RcCell<Option<WireArray>>,
}
crate::impl_hash_code_of_wire_for!(Wire<LinearCombinationWire>);
crate::impl_name_instance_of_wire_for!(Wire<LinearCombinationWire>);
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
impl WireConfig for Wire<LinearCombinationWire> {
    fn getBitWires(&self) -> Option<WireArray> {
        self.t.bitWires.borrow().clone()
    }
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::LinearCombination(self.clone()))
    }
}
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

    // fn getBitWires(&self) -> Option<WireArray> {
    //     self.t.bitWires.borrow().clone()
    // }

    fn setBits(&self, bitWires: Option<WireArray>) {
        *self.t.bitWires.borrow_mut() = bitWires;
    }
}
