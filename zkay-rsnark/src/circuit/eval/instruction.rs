#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use dyn_clone::{clone_trait_object, DynClone};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};


trait DynHash {
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T: Hash> DynHash for T {
    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state)
    }
}

impl Hash for dyn DynHash + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

pub trait Instruction:DynClone+DynHash {
    fn evaluate(&self,evaluator: CircuitEvaluator);

    fn emit(&self,evaluator: CircuitEvaluator) {}

    fn doneWithinCircuit(&self) -> bool {
        false
    }
    fn instance_of(&self,name:&str)->bool{
        self.name()==name
    }
    fn name(&self)->&str{
        ""
    }
}
dyn_clone::clone_trait_object!(Instruction);

// impl Debug for dyn Instruction {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.my_fmt(f)
//     }
// }
impl Hash for dyn Instruction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

impl PartialEq for dyn Instruction {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Eq for dyn Instruction {}




