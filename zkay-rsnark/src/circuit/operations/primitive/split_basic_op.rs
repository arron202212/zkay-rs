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
pub struct SplitBasicOp;
pub fn newSplitBasicOp(w: WireType, outs: Vec<WireType>, desc: Vec<String>) -> Op<SplitBasicOp> {
    Op::<SplitBasicOp> {
        inputs: vec![w],
        outputs: outs,
        desc: desc.get(0).unwrap_or(&String::new()).clone(),
        t: SplitBasicOp,
    }
}
impl BasicOp for Op<SplitBasicOp> {
    fn getOpcode(&self) -> String {
        return "split";
    }

    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        //super.checkInputs(assignment);
        assert!(
            self.outputs.len() >= assignment[self.inputs[0].getWireId()].bitLength(),
            "Error in Split --- The number of bits does not fit -- Input: {:x},{self:?}\n\t",
            assignment[self.inputs[0].getWireId()]
        );
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut inVal = assignment[self.inputs[0].getWireId()];
        if inVal.compareTo(Configs.get().unwrap().field_prime) > 0 {
            inVal = inVal.modulo(Configs.get().unwrap().field_prime);
        }
        for i in 0..self.outputs.length {
            assignment[self.outputs[i].getWireId()] = if inVal.testBit(i) {
                Util::one()
            } else {
                BigInteger::ZERO
            };
        }
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        self.inputs[0].equals(op.self.inputs[0]) && self.outputs.length == op.self.outputs.length
    }

    fn getNumMulGates(&self) -> i32 {
        self.outputs.len() as i32 + 1
    }
}
