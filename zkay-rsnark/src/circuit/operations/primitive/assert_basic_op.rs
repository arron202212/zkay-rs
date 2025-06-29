#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        config::config::Configs,
        operations::primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        structure::{
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Rem, Sub},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct AssertBasicOp;
pub fn new_assert(w1: WireType, w2: WireType, output: WireType, desc: String) -> Op<AssertBasicOp> {
    Op::<AssertBasicOp> {
        inputs: vec![Some(w1), Some(w2)],
        outputs: vec![Some(output)],
        desc,
        t: AssertBasicOp,
    }
}
crate::impl_instruction_for!(Op<AssertBasicOp>);
crate::impl_hash_code_for!(Op<AssertBasicOp>);
// impl crate::circuit::eval::instruction::Instruction for Op<AssertBasicOp>{
//      fn basic_op(&self) -> Option<Box<dyn BasicOp>> {
//         Box::new(self.clone())
//     }
// }
impl BasicOp for Op<AssertBasicOp> {
    fn compute(&self, assignment: Vec<Option<BigInteger>>) {
        let leftSide = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap()
            .mul(
                assignment[self.inputs[1].clone().unwrap().getWireId() as usize]
                    .clone()
                    .unwrap(),
            )
            .rem(Configs.field_prime.clone());
        let rightSide = assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap();
        let check = leftSide == rightSide;
        if !check {
            //println!("Error - Assertion Failed {self:?}");
            println!(
                "{} * {} != {}",
                assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                    .as_ref()
                    .unwrap(),
                assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
                    .as_ref()
                    .unwrap(),
                assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize]
                    .as_ref()
                    .unwrap()
            );
            panic!("Error During Evaluation");
        }
    }

    fn checkOutputs(&self, assignment: Vec<Option<BigInteger>>) {
        // do nothing
    }

    fn getOpcode(&self) -> String {
        "assert".to_owned()
    }

    fn getNumMulGates(&self) -> i32 {
        1
    }
}

impl Eq for Op<AssertBasicOp> {}
impl PartialEq for Op<AssertBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        return (check1 || check2)
            && self.outputs[0].as_ref().unwrap() == other.outputs[0].as_ref().unwrap();
    }
}
