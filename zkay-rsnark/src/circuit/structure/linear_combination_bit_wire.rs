#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::{
    bit_wire::BitWireConfig,
    circuit_generator::CircuitGenerator,
    wire::{GeneratorConfig, GetWireId, Wire, WireConfig, setBitsConfig},
    wire_array::WireArray,
    wire_type::WireType,
};

use rccell::{RcCell, WeakCell};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct LinearCombinationBitWire;
//crate::impl_hash_code_of_wire_g_for!(Wire<LinearCombinationBitWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<LinearCombinationBitWire>);

impl LinearCombinationBitWire {
    pub fn new(
        wireId: i32,
        generator: WeakCell<CircuitGenerator>,
    ) -> Wire<LinearCombinationBitWire> {
        //   if wireId>0 && wireId<10000
        //     {
        //         println!("==LinearCombinationBitWire::new======{wireId}==");
        //     }
        // //super(wireId);
        // Wire::<LinearCombinationBitWire> {
        //     wireId,
        //     generator,
        //     t: LinearCombinationBitWire,
        // }
        // crate::new_wire!(LinearCombinationBitWire,wireId,generator)
        Wire::<LinearCombinationBitWire>::new(LinearCombinationBitWire, wireId, generator).unwrap()
    }
}
impl setBitsConfig for LinearCombinationBitWire {}
impl setBitsConfig for Wire<LinearCombinationBitWire> {}
impl WireConfig for Wire<LinearCombinationBitWire> {
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::LinearCombinationBit(self.clone()))
    }
}
impl BitWireConfig for Wire<LinearCombinationBitWire> {}
impl Wire<LinearCombinationBitWire> {
    // pub fn new(wireId: i32) -> Self {
    //     // //super(wireId);
    //     Self
    // }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(
            vec![Some(WireType::LinearCombinationBit(self.clone()))],
            self.generator.clone(),
        )
    }
}
