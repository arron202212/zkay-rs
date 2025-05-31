#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::{CircuitGenerator, getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Gadget<T> {
    pub description: String,
    pub t: T,
}
pub fn newGadget(desc: &Option<String>) -> (Box<dyn CGConfig + Send + Sync>, String) {
    (
        getActiveCircuitGenerator("CGBase").unwrap(),
        desc.as_ref()
            .map_or_else(|| String::new(), |d| d.to_owned()),
    )
}

pub trait GadgetConfig: Debug {
    fn getOutputWires(&self) -> Vec<Option<WireType>>;

    // fn toString(&self) -> String {
    //     "getClass().getSimpleName()".to_owned() + " " + &self.description()
    // }
    fn description(&self) -> String {
        String::new()
    }
    fn debugStr(&self, s: &str) -> Option<String> {
        Some(format!("{self:?}:{s}"))
    }
}

#[macro_export]
macro_rules! impl_display_of_gadget_for {
    ($impl_type:ty) => {
        impl std::fmt::Display for $impl_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.getSimpleName(), self.description(),)
            }
        }
    };
}
