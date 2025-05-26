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
pub struct SplitBasicOp;
pub fn new_split(w: WireType, outs: Vec<Option<WireType>>, desc: String) -> Op<SplitBasicOp> {
    Op::<SplitBasicOp> {
        inputs: vec![Some(w)],
        outputs: outs,
        desc,
        t: SplitBasicOp,
    }
}
crate::impl_instruction_for!(Op<SplitBasicOp>);
impl BasicOp for Op<SplitBasicOp> {
    fn getOpcode(&self) -> String {
        return "split".to_owned();
    }

    fn checkInputs(&self, assignment: Vec<Option<BigInteger>>) {
        //super.checkInputs(assignment);
        assert!(
            self.outputs.len()
                >= assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
                    .clone()
                    .unwrap()
                    .bits() as usize,
            "Error in Split --- The number of bits does not fit -- Input: {:x},{self:?}\n\t",
            assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
                .clone()
                .unwrap()
        );
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        let mut inVal = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap();
        if inVal > Configs.get().unwrap().field_prime {
            inVal = inVal.rem(Configs.get().unwrap().field_prime.clone());
        }
        for i in 0..self.outputs.len() {
            assignment[self.outputs[i].as_ref().unwrap().getWireId() as usize] =
                Some(if inVal.bit(i as u64) {
                    Util::one()
                } else {
                    BigInteger::ZERO
                });
        }
    }

    fn getNumMulGates(&self) -> i32 {
        self.outputs.len() as i32 + 1
    }
}
impl Eq for Op<SplitBasicOp> {}
impl PartialEq for Op<SplitBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        self.inputs[0]
            .as_ref()
            .unwrap()
            .equals(other.inputs[0].as_ref().unwrap())
            && self.outputs.len() == other.outputs.len()
    }
}
