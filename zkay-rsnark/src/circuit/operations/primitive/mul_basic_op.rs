#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        StructNameConfig,
        config::config::Configs,
        operations::primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        structure::{
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};
use num_bigint::Sign;
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct MulBasicOp;
pub fn new_mul(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<MulBasicOp> {
    Op::<MulBasicOp> {
        inputs: vec![Some(w1.clone()), Some(w2.clone())],
        outputs: vec![Some(output.clone())],
        desc,
        t: MulBasicOp,
    }
}
crate::impl_instruction_for!(Op<MulBasicOp>);
crate::impl_hash_code_for!(Op<MulBasicOp>);
impl BasicOp for Op<MulBasicOp> {
    fn getOpcode(&self) -> String {
        "mul".to_owned()
    }

    fn compute(&self, mut assignment: &mut Vec<Option<BigInteger>>) {
        if self.outputs[0].as_ref().unwrap().getWireId() == 349251 {
            println!(
                "==compute=====outputs=========={}===={}====",
                file!(),
                self.outputs[0].as_ref().unwrap().name()
            );
        }
        let mut result = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap()
            .mul(
                assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                    .as_ref()
                    .unwrap(),
            );
        if result.sign() == Sign::Minus {
            result = result.add(&Configs.field_prime).rem(&Configs.field_prime);
        }
        if result > Configs.field_prime {
            result = result.rem(&Configs.field_prime);
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = Some(result);
    }

    fn getNumMulGates(&self) -> i32 {
        1
    }
}
impl Eq for Op<MulBasicOp> {}
impl PartialEq for Op<MulBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        check1 || check2
    }
}
