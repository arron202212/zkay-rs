#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::{
    InstanceOf,
    eval::circuit_evaluator::CircuitEvaluator,
    operations::{
        primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        wire_label_instruction::WireLabel,
    },
    structure::{
        circuit_generator::{
            CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
            get_active_circuit_generator,
        },
        wire_type::WireType,
    },
};
use dyn_clone::{DynClone, clone_trait_object};
use enum_dispatch::enum_dispatch;
use serde_closure::{Fn as Fns, traits::Fn as Fns};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Formatter},
    hash::DefaultHasher,
    hash::{Hash, Hasher},
};
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
    fn evaluate(&self, evaluator: &mut CircuitEvaluator) -> eyre::Result<()>;

    fn emit(&self, evaluator: &CircuitEvaluator) {}

    fn done_within_circuit(&self) -> bool {
        false
    }
    // fn get_num_mul_gates(&self) -> i32 {
    //     0
    // }
    // fn get_outputs(&self) -> Vec<Option<WireType>> {
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
    // fn hash_code(&self)->u64{
    //     let mut s = DefaultHasher::new();
    //     self.hash(&mut s);
    //     s.finish()
    // }
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
