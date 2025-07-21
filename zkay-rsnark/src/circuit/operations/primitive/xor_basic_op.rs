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
    ops::{Add, BitXor, Mul, Neg, Rem, Sub},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct XorBasicOp;

pub fn new_xor(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<XorBasicOp> {
    // if w1.getWireId()==147444 ||  w2.getWireId()==147444
    // {
    //     panic!("===new_xor====w1.as_ref().unwrap().getWireId()========================{}",w1.getWireId());
    // }
    Op::<XorBasicOp> {
        inputs: vec![Some(w1.clone()), Some(w2.clone())],
        outputs: vec![Some(output.clone())],
        desc,
        t: XorBasicOp,
    }
}
crate::impl_instruction_for!(Op<XorBasicOp>);
crate::impl_hash_code_for!(Op<XorBasicOp>);
impl BasicOp for Op<XorBasicOp> {
    fn getOpcode(&self) -> String {
        "xor".to_owned()
    }

    fn checkInputs(&self, assignment: &Vec<Option<BigInteger>>) {
        // //super.checkInputs(assignment);
        self.super_checkInputs(assignment);
        let check = Util::isBinary(
            assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
                .as_ref()
                .unwrap(),
        ) && Util::isBinary(
            assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                .as_ref()
                .unwrap(),
        );
        assert!(
            check,
            "Error - Input(s) to XOR are not binary.{self:?} During Evaluation"
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) {
        if self.outputs[0].as_ref().unwrap().getWireId() == 349251 {
            println!(
                "==compute=====outputs=========={}===={}====",
                file!(),
                self.outputs[0].as_ref().unwrap().name()
            );
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = assignment
            [self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .as_ref()
            .map(|x| {
                x.bitxor(
                    assignment[self.inputs[1].as_ref().unwrap().getWireId() as usize]
                        .as_ref()
                        .unwrap(),
                )
            });
    }

    fn getNumMulGates(&self) -> i32 {
        1
    }
}

impl Eq for Op<XorBasicOp> {}
impl PartialEq for Op<XorBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        check1 || check2
    }
}
