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
impl MulBasicOp {
    pub fn new(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<MulBasicOp> {
        Op::<MulBasicOp> {
            inputs: vec![Some(w1.clone()), Some(w2.clone())],
            outputs: vec![Some(output.clone())],
            desc,
            t: MulBasicOp,
        }
    }
}
crate::impl_instruction_for!(Op<MulBasicOp>);
crate::impl_hash_code_for!(Op<MulBasicOp>);
impl BasicOp for Op<MulBasicOp> {
    fn getOpcode(&self) -> String {
        "mul".to_owned()
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let (in0_id, in1_id, out0_id) = (
            self.inputs[0].as_ref().unwrap().getWireId() as usize,
            self.inputs[1].as_ref().unwrap().getWireId() as usize,
            self.outputs[0].as_ref().unwrap().getWireId() as usize,
        );
        let mut result = assignment[in0_id]
            .clone()
            .unwrap()
            .mul(assignment[in1_id].as_ref().unwrap());
        if result.sign() == Sign::Minus {
            if out0_id == 48124 || out0_id == 4 {
                println!(
                    "===result.sign() == Sign::Minus ========{out0_id}================{},{},{}",
                    result,
                    result.clone().add(&Configs.field_prime),
                    result
                        .clone()
                        .add(&Configs.field_prime)
                        .rem(&Configs.field_prime)
                );
            }
            result = result.add(&Configs.field_prime).rem(&Configs.field_prime);
        }
        if result > Configs.field_prime {
            if out0_id == 48124 || out0_id == 4 {
                println!(
                    "===result > Configs.field_prime ============{out0_id}============{},{}",
                    result,
                    result.clone().rem(&Configs.field_prime)
                );
            }
            result = result.rem(&Configs.field_prime);
        }
        // if self.outputs[0].as_ref().unwrap().getWireId() == 5 {
        // println!(
        //     "====result================={}======{},{}",
        //     self.outputs[0].as_ref().unwrap().getWireId(),
        //     result,
        //     result.clone().rem(&Configs.field_prime)
        // );
        // }
        assignment[out0_id] = Some(result);
        // if out0_id == 4 {
        //     println!(
        //         "==compute=====outputs==={}={}=={}===={}===={}=={}==",
        //         file!(),
        //         assignment[in0_id]
        //             .clone()
        //             .unwrap(),
        //         assignment[in1_id]
        //             .as_ref()
        //             .unwrap(),
        //         assignment[in0_id]
        //             .clone()
        //             .unwrap()
        //             .mul(
        //                 assignment[in1_id]
        //                     .as_ref()
        //                     .unwrap(),
        //             ),
        //         self.outputs[0].as_ref().unwrap().name(),
        //         assignment[out0_id]
        //             .as_ref()
        //             .unwrap()
        //     );
        // }
        Ok(())
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
