#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::structure::{
        circuit_generator::{
            CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
            getActiveCircuitGenerator,
        },
        wire_type::WireType,
    },
    util::util::ARcCell,
};
use rccell::{RcCell, WeakCell};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Gadget<T> {
    pub generator: RcCell<CircuitGenerator>,
    pub description: String,
    pub t: T,
}

pub trait GadgetConfig: Debug {
    fn getOutputWires(&self) -> &Vec<Option<WireType>>;

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
                write!(f, "{} {}", self.name(), self.description(),)
            }
        }
    };
}
