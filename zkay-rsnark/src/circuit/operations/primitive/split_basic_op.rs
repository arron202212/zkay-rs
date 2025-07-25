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
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct SplitBasicOp;
pub fn new_split(w: &WireType, outs: Vec<Option<WireType>>, desc: String) -> Op<SplitBasicOp> {
    Op::<SplitBasicOp> {
        inputs: vec![Some(w.clone())],
        outputs: outs,
        desc,
        t: SplitBasicOp,
    }
}
crate::impl_instruction_for!(Op<SplitBasicOp>);
crate::impl_hash_code_for!(Op<SplitBasicOp>);
impl BasicOp for Op<SplitBasicOp> {
    fn getOpcode(&self) -> String {
        "split".to_owned()
    }

    fn checkInputs(&self, assignment: &Vec<Option<BigInteger>>) {
        //super.checkInputs(assignment);
        self.super_checkInputs(assignment);
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

    fn compute(&self, mut assignment: &mut Vec<Option<BigInteger>>) {
        let mut inVal = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap();
        if inVal > Configs.field_prime {
            inVal = inVal.rem(&Configs.field_prime);
        }
        for i in 0..self.outputs.len() {
            if self.outputs[i].as_ref().unwrap().getWireId() == 349251 {
                println!(
                    "==compute=====outputs=========={}===={}====",
                    file!(),
                    self.outputs[i].as_ref().unwrap().name()
                );
            }
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
        self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.outputs.len() == other.outputs.len()
    }
}
