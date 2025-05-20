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
use crate::circuit::structure::wire::WireConfig;
 use crate::util::util::{Util,BigInteger};
 use std::ops::{Mul,Add,Sub,Rem};
use std::hash::{DefaultHasher, Hash, Hasher};
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash,PartialEq)]
pub struct AssertBasicOp;
pub fn newAssertBasicOp(w1: WireType, w2: WireType, output: WireType, desc: Vec<String>) -> Op<AssertBasicOp> {
    Op::<AssertBasicOp> {
        inputs: vec![Some(w1),Some( w2)],
        outputs: vec![Some(output)],
        desc:desc.get(0).map_or_else(||String::new(),|d|d.clone()),
        t: AssertBasicOp,
    }
}
crate::impl_instruction_for!(Op<AssertBasicOp>);
impl BasicOp for Op<AssertBasicOp> {
    fn compute(&self, assignment: Vec<Option<BigInteger>>) {
        let leftSide = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize].clone().unwrap()
            .mul(assignment[self.inputs[1].clone().unwrap().getWireId()  as usize].clone().unwrap())
            .rem(Configs.get().unwrap().field_prime.clone());
        let rightSide = assignment[self.outputs[0].as_ref().unwrap().getWireId()  as usize].clone().unwrap();
        let check = leftSide==rightSide;
        if !check {
            println!("Error - Assertion Failed {self:?}");
            println!(
                "{} * {} != {}",
                assignment[self.inputs[1].as_ref().unwrap().getWireId()  as usize].as_ref().unwrap(),
                assignment[self.inputs[0].as_ref().unwrap().getWireId()  as usize].as_ref().unwrap(),
                assignment[self.outputs[0].as_ref().unwrap().getWireId()  as usize].as_ref().unwrap()
            );
            panic!("Error During Evaluation");
        }
    }

    fn checkOutputs(&self,assignment: Vec<Option<BigInteger>>) {
        // do nothing
    }

    fn getOpcode(&self) -> String {
        return "assert".to_owned();
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;

        let check1 =
            self.inputs[0].as_ref().unwrap().equals(op.inputs[0].as_ref().unwrap()) && self.inputs[1].as_ref().unwrap().equals(op.inputs[1].as_ref().unwrap());
        let check2 =
            self.inputs[1].as_ref().unwrap().equals(op.inputs[0].as_ref().unwrap()) && self.inputs[0].as_ref().unwrap().equals(op.inputs[1].as_ref().unwrap());
        return (check1 || check2) && self.outputs[0].as_ref().unwrap().equals(op.outputs[0].as_ref().unwrap());
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
