#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::structure::wire::{Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use num_bigint::Sign;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct NonZeroCheckBasicOp;
pub fn new_non_zero_check(
    w: WireType,
    out1: WireType,
    out2: WireType,
    desc: String,
) -> Op<NonZeroCheckBasicOp> {
    Op::<NonZeroCheckBasicOp> {
        inputs: vec![Some(w)],
        outputs: vec![Some(out1), Some(out2)],
        desc,
        t: NonZeroCheckBasicOp,
    }
}
crate::impl_instruction_for!(Op<NonZeroCheckBasicOp>);
impl BasicOp for Op<NonZeroCheckBasicOp> {
    fn getOpcode(&self) -> String {
        return "zerop".to_owned();
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        if assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .as_ref()
            .unwrap()
            .sign()
            == Sign::NoSign
        {
            assignment[self.outputs[1].as_ref().unwrap().getWireId() as usize] =
                Some(BigInteger::ZERO);
        } else {
            assignment[self.outputs[1].as_ref().unwrap().getWireId() as usize] = Some(Util::one());
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = Some(BigInteger::ZERO); // a dummy value
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        let op = rhs;
        self.inputs[0]
            .as_ref()
            .unwrap()
            .equals(op.inputs[0].as_ref().unwrap())
    }

    fn getNumMulGates(&self) -> i32 {
        return 2;
    }
}
