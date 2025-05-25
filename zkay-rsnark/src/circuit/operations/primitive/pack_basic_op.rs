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
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct PackBasicOp;
pub fn new_pack(inBits: Vec<Option<WireType>>, out: WireType, desc: String) -> Op<PackBasicOp> {
    Op::<PackBasicOp> {
        inputs: inBits,
        outputs: vec![Some(out)],
        desc,
        t: PackBasicOp,
    }
}
crate::impl_instruction_for!(Op<PackBasicOp>);
impl BasicOp for Op<PackBasicOp> {
    fn getOpcode(&self) -> String {
        return "pack".to_owned();
    }

    fn checkInputs(&self, assignment: Vec<Option<BigInteger>>) {
        // //super.checkInputs(assignment);

        assert!(
            (0..self.inputs.len()).all(|i| Util::isBinary(
                assignment[self.inputs[i].as_ref().unwrap().getWireId() as usize]
                    .clone()
                    .unwrap()
            )),
            "Error - Input(s) to Pack are not binary.{self:?} During Evaluation "
        );
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        let mut sum = BigInteger::ZERO;
        for i in 0..self.inputs.len() {
            sum = sum.add(
                assignment[self.inputs[i].as_ref().unwrap().getWireId() as usize]
                    .clone()
                    .unwrap()
                    .mul(BigInteger::from(2).pow(i as u32)),
            );
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] =
            Some(sum.rem(Configs.get().unwrap().field_prime.clone()));
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        if op.inputs.len() != self.inputs.len() {
            return false;
        }

        let mut check = true;
        for i in 0..self.inputs.len() {
            check = check
                && self.inputs[i]
                    .as_ref()
                    .unwrap()
                    .equals(op.inputs[i].as_ref().unwrap());
        }
        check
    }

    fn getNumMulGates(&self) -> i32 {
        return 0;
    }
}
