#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::util::util::{Util,BigInteger};
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct NonZeroCheckBasicOp;
fn newNonZeroCheckBasicOp(
    w: WireType,
    out1: WireType,
    out2: WireType,
    desc: Vec<String>,
) -> Op<NonZeroCheckBasicOp> {
    Op::<NonZeroCheckBasicOp> {
        inputs: vec![w],
        outputs: vec![out1, out2],
        desc: desc.get(0).unwrap_or(&String::new()).clone(),
        t: NonZeroCheckBasicOp,
    }
}
impl BasicOp for Op<NonZeroCheckBasicOp> {
    fn getOpcode(&self) -> String {
        return "zerop";
    }

    fn compute(&self, mut assignment: Vec<BigInteger>) {
        if assignment[self.inputs[0].getWireId()].signum() == 0 {
            assignment[self.outputs[1].getWireId()] = BigInteger::ZERO;
        } else {
            assignment[self.outputs[1].getWireId()] = Util::one();
        }
        assignment[self.outputs[0].getWireId()] = BigInteger::ZERO; // a dummy value
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        self.inputs[0].equals(op.self.inputs[0])
    }

    fn getNumMulGates(&self) -> i32 {
        return 2;
    }
}
