#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        config::config::Configs,
        operations::primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        structure::{
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Rem, Sub},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct AddBasicOp;

pub fn new_add(ws: Vec<Option<WireType>>, output: WireType, desc: String) -> Op<AddBasicOp> {
    Op::<AddBasicOp> {
        inputs: ws,
        outputs: vec![Some(output)],
        desc,
        t: AddBasicOp,
    }
}
crate::impl_instruction_for!(Op<AddBasicOp>);
crate::impl_hash_code_for!(Op<AddBasicOp>);
impl BasicOp for Op<AddBasicOp> {
    // fn getOpcode(&self) -> String {
    //     return "add".to_owned();
    // }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        let mut s = BigInteger::ZERO;
        for w in &self.inputs {
            s = s.add(
                assignment[w.as_ref().unwrap().getWireId() as usize]
                    .clone()
                    .unwrap(),
            );
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] =
            Some(s.rem(Configs.field_prime.clone()));
    }

    fn getNumMulGates(&self) -> i32 {
        0
    }
}

impl Eq for Op<AddBasicOp> {}
impl PartialEq for Op<AddBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        if other.inputs.len() != self.inputs.len() {
            return false;
        }

        if self.inputs.len() == 2 {
            let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
                && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
            let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
                && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
            return check1 || check2;
        }

        self.inputs
            .iter()
            .zip(&other.inputs)
            .all(|(a, b)| a.as_ref().unwrap() == b.as_ref().unwrap())
    }
}
