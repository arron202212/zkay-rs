#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::structure::{
        circuit_generator::CircuitGenerator,
        wire::GeneratorConfig,
        wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
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
#[derive(Debug, Clone, ImplStructNameConfig, PartialEq)]
pub struct LinearCombinationWire {
    pub bit_wires: Option<WireArray>,
}
// impl Hash for LinearCombinationWire {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // self.bit_wires.hash(state);
//     }
// }
// impl PartialEq for LinearCombinationWire {
//     fn eq(&self, other: &Self) -> bool {
//         self.bit_wires == other.bit_wires
//     }
// }
//crate::impl_hash_code_of_wire_g_for!(Wire<LinearCombinationWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<LinearCombinationWire>);

impl LinearCombinationWire {
    pub fn new(
        wire_id: i32,
        bits: Option<WireArray>,
        generator: WeakCell<CircuitGenerator>,
    ) -> Wire<LinearCombinationWire> {
        //   if  wire_id>0 && wire_id<10000
        //     {
        //         println!("===LinearCombinationWire::new====={wire_id}==");
        //     }
        // //super(wire_id);
        // Wire::<LinearCombinationWire> {
        //     wire_id,
        //     generator,
        //     t: LinearCombinationWire { bit_wires: bits },
        // }
        Wire::<LinearCombinationWire>::new(
            LinearCombinationWire { bit_wires: bits },
            wire_id,
            generator,
        )
        .unwrap()
    }
}
impl SetBitsConfig for LinearCombinationWire {}
impl SetBitsConfig for Wire<LinearCombinationWire> {}
impl WireConfig for Wire<LinearCombinationWire> {
    fn get_bit_wires(&self) -> Option<WireArray> {
        self.t.bit_wires.clone()
    }
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::LinearCombination(self.clone()))
    }
}
impl Wire<LinearCombinationWire> {
    // pub fn new(wire_id: i32) -> Self {
    //     // //super(wire_id);
    //     Self {
    //         bit_wires: RcCell::new(None),
    //     }
    // }

    // pub fn newa(bits: WireArray) -> Self {
    //     // //super(bits);
    //     Self {
    //         bit_wires: RcCell::new(Some(bits)),
    //     }
    // }

    // fn get_bit_wires(&self) -> Option<WireArray> {
    //     self.t.bit_wires.borrow().clone()
    // }

    fn set_bits(&mut self, bit_wires: Option<WireArray>) {
        self.t.bit_wires = bit_wires;
    }
}
