#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::structure::wire::{Wire,GetWireId, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, BitXor, Mul, Neg, Rem, Sub};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct XorBasicOp;

pub fn new_xor(w1: WireType, w2: WireType, output: WireType, desc: String) -> Op<XorBasicOp> {
    Op::<XorBasicOp> {
        inputs: vec![Some(w1), Some(w2)],
        outputs: vec![Some(output)],
        desc,
        t: XorBasicOp,
    }
}
crate::impl_instruction_for!(Op<XorBasicOp>);
crate::impl_hash_code_for!(Op<XorBasicOp>);
impl BasicOp for Op<XorBasicOp> {
    fn getOpcode(&self) -> String {
        return "xor".to_owned();
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
        assert!(
            check,
            "Error - Input(s) to XOR are not binary.{self:?} During Evaluation"
        );
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = assignment
            [self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .as_ref()
            .map(|x| {
                x.bitxor(
                    assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                        .clone()
                        .unwrap(),
                )
            });
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}

impl Eq for Op<XorBasicOp> {}
impl PartialEq for Op<XorBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        let check1 = self.inputs[0]
            .as_ref()
            .unwrap()
            ==other.inputs[0].as_ref().unwrap()
            && self.inputs[1]
                .as_ref()
                .unwrap()
                ==other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1]
            .as_ref()
            .unwrap()
            ==other.inputs[0].as_ref().unwrap()
            && self.inputs[0]
                .as_ref()
                .unwrap()
                ==other.inputs[1].as_ref().unwrap();
        check1 || check2
    }
}
