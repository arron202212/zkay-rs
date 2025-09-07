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
pub struct LinearCombination_wire {
    pub bit_wires: Option<WireArray>,
}
// impl Hash for LinearCombination_wire {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // self.bit_wires.hash(state);
//     }
// }
// impl PartialEq for LinearCombination_wire {
//     fn eq(&self, other: &Self) -> bool {
//         self.bit_wires == other.bit_wires
//     }
// }
//crate::impl_hash_code_of_wire_g_for!(Wire<LinearCombination_wire>);
crate::impl_name_instance_of_wire_g_for!(Wire<LinearCombination_wire>);

impl LinearCombination_wire {
    pub fn new(
        wire_id: i32,
        bits: Option<WireArray>,
        generator: WeakCell<CircuitGenerator>,
    ) -> Wire<LinearCombination_wire> {
        //   if  wire_id>0 && wire_id<10000
        //     {
        //         println!("===LinearCombination_wire::new====={wire_id}==");
        //     }
        // //super(wire_id);
        // Wire::<LinearCombination_wire> {
        //     wire_id,
        //     generator,
        //     t: LinearCombination_wire { bit_wires: bits },
        // }
        Wire::<LinearCombination_wire>::new(
            LinearCombination_wire { bit_wires: bits },
            wire_id,
            generator,
        )
        .unwrap()
    }
}
impl SetBitsConfig for LinearCombination_wire {}
impl SetBitsConfig for Wire<LinearCombination_wire> {}
impl WireConfig for Wire<LinearCombination_wire> {
    fn get_bit_wires(&self) -> Option<WireArray> {
        self.t.bit_wires.clone()
    }
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::LinearCombination(self.clone()))
    }
}
impl Wire<LinearCombination_wire> {
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
