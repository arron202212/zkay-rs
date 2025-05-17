#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::operations::primitive::basic_op::Op;
 use crate::circuit::operations::primitive::basic_op::BasicOp;
 use crate::util::util::{Util,BigInteger};
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct MulBasicOp;
pub fn newMulBasicOp(w1: WireType, w2: WireType, output: WireType, desc: Vec<String>) -> Op<MulBasicOp> {
    Op::<MulBasicOp> {
        inputs: vec![w1, w2],
        outputs: vec![output],
        desc: desc.get(0).unwrap_or(&String::new()).clone(),
        t: MulBasicOp,
    }
}
impl BasicOp for Op<MulBasicOp> {
    fn getOpcode(&self) -> String {
        return "mul";
    }

    fn compute(&self, mut assignment: Vec<BigInteger>) {
        let mut result =
            assignment[self.inputs[0].getWireId()].multiply(assignment[self.inputs[1].getWireId()]);
        if result.compareTo(Configs.get().unwrap().field_prime) > 0 {
            result = result.modulo(Configs.get().unwrap().field_prime);
        }
        assignment[self.outputs[0].getWireId()] = result;
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;

        let check1 = self.self.inputs[0].equals(op.self.inputs[0])
            && self.self.inputs[1].equals(op.self.inputs[1]);
        let check2 = self.self.inputs[1].equals(op.self.inputs[0])
            && self.self.inputs[0].equals(op.self.inputs[1]);
        check1 || check2
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
