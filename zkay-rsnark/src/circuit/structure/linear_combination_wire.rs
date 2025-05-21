#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct LinearCombinationWire {
    pub bitWires: Option<WireArray>,
}
impl setBitsConfig for LinearCombinationWire {}
impl LinearCombinationWire {
    pub fn new(wireId: i32) -> Self {
        // super(wireId);
        Self { bitWires: None }
    }

    // pub  LinearCombinationWire(WireArray bits) {
    // 	super(bits);
    // }

    fn getBitWires(&self) -> Option<WireArray> {
        self.bitWires.clone()
    }

    fn setBits(&mut self, bitWires: Option<WireArray>) {
        self.bitWires = bitWires;
    }
}
