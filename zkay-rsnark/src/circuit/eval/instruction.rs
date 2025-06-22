#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::InstanceOf;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::primitive::basic_op::{BasicOp, Op};
use crate::circuit::operations::wire_label_instruction::WireLabel;
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::{
    CircuitGenerator, CircuitGeneratorExtend, CircuitGeneratorIQ, getActiveCircuitGenerator,
};
use crate::circuit::structure::wire_type::WireType;
use dyn_clone::{DynClone, clone_trait_object};
use enum_dispatch::enum_dispatch;
use serde_closure::{Fn as Fns, traits::Fn as Fns};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use zkay_derive::ImplStructNameConfig;
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
#[enum_dispatch]
pub trait Instruction: DynClone + DynHash + Debug + InstanceOf {
    fn evaluate(&self, evaluator: &mut CircuitEvaluator);

    fn emit(&self, evaluator: &CircuitEvaluator) {}

    fn doneWithinCircuit(&self) -> bool {
        false
    }
    // fn getNumMulGates(&self) -> i32 {
    //     0
    // }
    // fn getOutputs(&self) -> Vec<Option<WireType>> {
    //     vec![]
    // }
    // fn instance_of(&self, name: &str) -> bool {
    //     self.name() == name
    // }
    // fn name(&self) -> &str {
    //     ""
    // }
    fn basic_op(&self) -> Option<Box<dyn BasicOp>> {
        None
    }
    fn wire_label(&self) -> Option<Box<dyn WireLabel>> {
        None
    }
}
// dyn_clone::clone_trait_object!(Instruction);

impl Clone for Box<dyn Instruction> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

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

//dyn Fn<(&mut CircuitEvaluator,), Output = ()>
// pub type InstructionFunction<'a> =Box<dyn  Fns<(&'a mut CircuitEvaluator,), Output = ()>>;//fn(&mut CircuitEvaluator);

// #[enum_dispatch(Instruction)]
// #[derive(Clone,Hash,Debug,Eq,PartialEq,ImplStructNameConfig)]
// pub enum Box<dyn Instruction><'a>{
// Trait(Box<dyn Instruction>),
// Function(InstructionFunction<'a>),
// }
