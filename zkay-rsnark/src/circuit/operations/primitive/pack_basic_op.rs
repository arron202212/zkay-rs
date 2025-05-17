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
pub struct PackBasicOp;
pub fn newPackBasicOp(inBits: Vec<WireType>, out: WireType, desc: Vec<String>) -> Op<PackBasicOp> {
    Op::<PackBasicOp> {
        inputs: inBits,
        outputs: vec![out],
        desc: desc.get(0).unwrap_or(&String::new()).clone(),
        t: PackBasicOp,
    }
}
impl BasicOp for Op<PackBasicOp> {
    fn getOpcode(&self) -> String {
        return "pack";
    }

    fn checkInputs(&self, assignment: Vec<BigInteger>) {
        // //super.checkInputs(assignment);

        assert!(
            (0..self.inputs.length).all(|i| Util::isBinary(assignment[self.inputs[i].getWireId()])),
            "Error - Input(s) to Pack are not binary.{self:?} During Evaluation "
        );
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut sum = BigInteger::ZERO;
        for i in 0..self.inputs.length {
            sum = sum
                .add(assignment[self.inputs[i].getWireId()].multiply(BigInteger::new("2").pow(i)));
        }
        assignment[self.outputs[0].getWireId()] = sum.modulo(Configs.get().unwrap().field_prime);
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        if op.inputs.length != self.inputs.length {
            return false;
        }

        let mut check = true;
        for i in 0..self.inputs.length {
            check = check && self.inputs[i].equals(op.inputs[i]);
        }
        check
    }

    fn getNumMulGates(&self) -> i32 {
        return 0;
    }
}
