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
use crate::circuit::structure::wire::WireConfig;
use std::ops::{Add,Sub,Mul,Rem};
use std::hash::{DefaultHasher, Hash, Hasher};
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash,PartialEq)]
pub struct AddBasicOp;

pub fn new_add(ws: Vec<Option<WireType>>, output: WireType, desc: Vec<String>) -> Op<AddBasicOp> {
    Op::<AddBasicOp> {
        inputs: ws,
        outputs: vec![Some(output)],
        desc:desc.get(0).map_or_else(||String::new(),|d|d.clone()),
        t: AddBasicOp,
    }
}
crate::impl_instruction_for!(Op<AddBasicOp>);
impl BasicOp for Op<AddBasicOp> {
    fn getOpcode(&self) -> String {
        return "add".to_owned();
    }

    fn compute(&self, assignment: Vec<Option<BigInteger>>) {
        let mut s = BigInteger::ZERO;
        for w in self.inputs {
            s = s.add(assignment[w.as_ref().unwrap().getWireId() as usize].clone().unwrap());
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = Some(s.rem(Configs.get().unwrap().field_prime.clone()));
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        if op.inputs.len() != self.inputs.len() {
            return false;
        }

        if self.inputs.len() == 2 {
            let check1 = self.inputs[0].as_ref().unwrap().equals(op.inputs[0].as_ref().unwrap()) && self.inputs[1].as_ref().unwrap().equals(op.inputs[1].as_ref().unwrap());
            let check2 = self.inputs[1].as_ref().unwrap().equals(op.inputs[0].as_ref().unwrap()) && self.inputs[0].as_ref().unwrap().equals(op.inputs[1].as_ref().unwrap());
            return check1 || check2;
        }

        self.inputs.iter().zip(&op.inputs).all(|(a, b)| a.as_ref().unwrap().equals(b.as_ref().unwrap()))
    }

    fn getNumMulGates(&self) -> i32 {
        0
    }
}
