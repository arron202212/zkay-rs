#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::operations::primitive::basic_op::{BasicOp, Op};

use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::BitOr;
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct OrBasicOp;

pub fn new_or(w1: WireType, w2: WireType, output: WireType, desc: String) -> Op<OrBasicOp> {
    Op::<OrBasicOp> {
        inputs: vec![Some(w1), Some(w2)],
        outputs: vec![Some(output)],
        desc,
        t: OrBasicOp,
    }
}

crate::impl_instruction_for!(Op<OrBasicOp>);
crate::impl_hash_code_for!(Op<OrBasicOp>);
impl BasicOp for Op<OrBasicOp> {
    fn getOpcode(&self) -> String {
        "or".to_owned()
    }

    fn checkInputs(&self, assignment: Vec<Option<BigInteger>>) {
        // //super.checkInputs(assignment);
        let check = Util::isBinary(
            assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
                .clone()
                .unwrap(),
        ) && Util::isBinary(
            assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                .clone()
                .unwrap(),
        );
        if !check {
            //println!("Error - Input(s) to OR are not binary.{self:?} ");
            panic!("Error During Evaluation");
        }
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = assignment
            [self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .as_ref()
            .map(|x| {
                x.clone().bitor(
                    assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                        .clone()
                        .unwrap(),
                )
            });
    }

    fn getNumMulGates(&self) -> i32 {
        1
    }
}
impl Eq for Op<OrBasicOp> {}
impl PartialEq for Op<OrBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        check1 || check2
    }
}
