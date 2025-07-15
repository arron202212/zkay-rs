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
        wire::GeneratorConfig,
        wire::{GetWireId, Wire, WireConfig, setBitsConfig},
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
pub struct LinearCombinationWire {
    pub bitWires: Option<WireArray>,
}
// impl Hash for LinearCombinationWire {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // self.bitWires.hash(state);
//     }
// }
impl PartialEq for LinearCombinationWire {
    fn eq(&self, other: &Self) -> bool {
        self.bitWires == other.bitWires
    }
}
//crate::impl_hash_code_of_wire_g_for!(Wire<LinearCombinationWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<LinearCombinationWire>);
pub fn new_linear_combination(
    wireId: i32,
    bits: Option<WireArray>,
    generator: WeakCell<CircuitGenerator>,
) -> Wire<LinearCombinationWire> {
    //   if  wireId>0 && wireId<10000
    //     {
    //         println!("===new_linear_combination====={wireId}==");
    //     }
    // super(wireId);
    // Wire::<LinearCombinationWire> {
    //     wireId,
    //     generator,
    //     t: LinearCombinationWire { bitWires: bits },
    // }
    Wire::<LinearCombinationWire>::new(LinearCombinationWire { bitWires: bits }, wireId, generator)
        .unwrap()
}
impl setBitsConfig for LinearCombinationWire {}
impl setBitsConfig for Wire<LinearCombinationWire> {}
impl WireConfig for Wire<LinearCombinationWire> {
    fn getBitWires(&self) -> Option<WireArray> {
        self.t.bitWires.clone()
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

    fn setBits(&mut self, bitWires: Option<WireArray>) {
        self.t.bitWires = bitWires;
    }
}
