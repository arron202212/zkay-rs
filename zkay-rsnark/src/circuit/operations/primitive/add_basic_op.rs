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
pub struct AddBasicOp;

pub fn NewAddBasicOp(ws: Vec<WireType>, output: WireType, desc: Vec<String>) -> Op<AddBasicOp> {
    Op::<AddBasicOp> {
        inputs: ws,
        outputs: vec![output],
        desc,
        t: AddBasicOp,
    }
}

impl BasicOp for Op<AddBasicOp> {
    fn getOpcode(&self) -> String {
        return "add";
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut s = BigInteger::ZERO;
        for w in self.inputs {
            s = s.add(assignment[w.getWireId()]);
        }
        assignment[self.outputs[0].getWireId()] = s.modulo(Configs.get().unwrap().field_prime);
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
            let check1 = self.inputs[0].equals(op.inputs[0]) && self.inputs[1].equals(op.inputs[1]);
            let check2 = self.inputs[1].equals(op.inputs[0]) && self.inputs[0].equals(op.inputs[1]);
            return check1 || check2;
        }

        self.inputs.iter().zip(&op.inputs).all(|(a, b)| a.equals(b))
    }

    fn getNumMulGates(&self) -> i32 {
        0
    }
}
