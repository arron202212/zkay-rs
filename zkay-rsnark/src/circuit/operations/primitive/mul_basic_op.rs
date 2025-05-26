#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::structure::wire::{Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct MulBasicOp;
pub fn new_mul(w1: WireType, w2: WireType, output: WireType, desc: String) -> Op<MulBasicOp> {
    Op::<MulBasicOp> {
        inputs: vec![Some(w1), Some(w2)],
        outputs: vec![Some(output)],
        desc,
        t: MulBasicOp,
    }
}
crate::impl_instruction_for!(Op<MulBasicOp>);
impl BasicOp for Op<MulBasicOp> {
    fn getOpcode(&self) -> String {
        return "mul".to_owned();
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        let mut result = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap()
            .mul(
                assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                    .clone()
                    .unwrap(),
            );
        if result > Configs.get().unwrap().field_prime {
            result = result.rem(Configs.get().unwrap().field_prime.clone());
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = Some(result);
    }

    fn getNumMulGates(&self) -> i32 {
        return 1;
    }
}
impl Eq for Op<MulBasicOp> {}
impl PartialEq for Op<MulBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        let check1 = self.inputs[0]
            .as_ref()
            .unwrap()
            .equals(other.inputs[0].as_ref().unwrap())
            && self.inputs[1]
                .as_ref()
                .unwrap()
                .equals(other.inputs[1].as_ref().unwrap());
        let check2 = self.inputs[1]
            .as_ref()
            .unwrap()
            .equals(other.inputs[0].as_ref().unwrap())
            && self.inputs[0]
                .as_ref()
                .unwrap()
                .equals(other.inputs[1].as_ref().unwrap());
        check1 || check2
    }
}
