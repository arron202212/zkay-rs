#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct Gadget<T> {
    generator: CircuitGenerator,
    description: String,
    t: T,
}
pub fn newGadget(desc: Vec<String>) -> (CircuitGenerator, String) {
    (
        CircuitGenerator::getActiveCircuitGenerator(),
        desc.get(0).unwrap_or(&String::new()),
    )
}

pub trait GadgetConfig {
    fn getOutputWires() -> Vec<WireType>;

    fn toString(&self) -> String {
        "getClass().getSimpleName()".to_owned() + " " + &self.description
    }

    fn debugStr(&self, s: String) -> String {
        format!("{self:?}:{s}")
    }
}
