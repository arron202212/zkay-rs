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
pub struct XorBasicOp;

fn new_xor_basic_op(w1: WireType, w2: WireType, output: WireType, desc: Vec<String>) -> Op<XorBasicOp> {
    Op::<XorBasicOp> {
        inputs: vec![w1, w2],
        outputs: output,
        desc: desc.get(0).unwrap_or(&String::new()).clone(),
        t: XorBasicOp,
    }
}

impl BasicOp for XorBasicOp {
    fn getOpcode(&self) -> String {
        return "xor";
    }

    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        // //super.checkInputs(assignment);
        let check = Util::isBinary(assignment[self.self.inputs[0].getWireId()])
            && Util::isBinary(assignment[self.self.inputs[1].getWireId()]);
        assert!(
            check,
            "Error - Input(s) to XOR are not binary.{self:?} During Evaluation"
        );
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        assignment[self.outputs[0].getWireId()] =
            assignment[self.inputs[0].getWireId()].xor(assignment[self.inputs[1].getWireId()]);
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
        check1 || check2
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
