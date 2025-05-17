#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::operations::primitive::basic_op::{BasicOp,Op};
 use crate::util::util::{Util,BigInteger};
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct AssertBasicOp;
pub fn newAssertBasicOp(w1: WireType, w2: WireType, output: WireType, desc: Vec<String>) -> Op<AssertBasicOp> {
    Op::<AssertBasicOp> {
        inputs: vec![w1, w2],
        outputs: vec![output],
        desc,
        t: AssertBasicOp,
    }
}

impl BasicOp for AssertBasicOp {
    fn compute(&self, assignment: Vec<BigInteger>) {
        let leftSide = assignment[self.inputs[0].getWireId()]
            .multiply(assignment[self.inputs[1].getWireId()])
            .modulo(Configs.get().unwrap().field_prime);
        let rightSide = assignment[self.outputs[0].getWireId()];
        let check = leftSide.equals(rightSide);
        if !check {
            println!("Error - Assertion Failed {self:?}");
            println!(
                "{} * {} != {}",
                assignment[self.inputs[1].getWireId()],
                assignment[self.inputs[0].getWireId()],
                assignment[self.outputs[0].getWireId()]
            );
            panic!("Error During Evaluation");
        }
    }

    fn checkOutputs(&self,assignment: Vec<BigInteger>) {
        // do nothing
    }

    fn getOpcode(&self) -> String {
        return "assert";
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;

        let check1 =
            self.inputs[0].equals(op.self.inputs[0]) && self.inputs[1].equals(op.self.inputs[1]);
        let check2 =
            self.inputs[1].equals(op.self.inputs[0]) && self.inputs[0].equals(op.self.inputs[1]);
        return (check1 || check2) && self.outputs[0].equals(op.self.outputs[0]);
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
