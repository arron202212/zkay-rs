#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Gadget<T> {
    pub description: String,
    pub t: T,
}
pub fn newGadget(desc: &String) -> (CircuitGenerator, String) {
    (CircuitGenerator::getActiveCircuitGenerator().unwrap(), desc.clone())
}

pub trait GadgetConfig: Debug {
    fn getOutputWires(&self) -> Vec<Option<WireType>>;

    fn toString(&self) -> String {
        "getClass().getSimpleName()".to_owned() + " " + &self.description()
    }
    fn description(&self) -> String {
        String::new()
    }
    fn debugStr(&self, s: String) -> String {
        format!("{self:?}:{s}")
    }
}
