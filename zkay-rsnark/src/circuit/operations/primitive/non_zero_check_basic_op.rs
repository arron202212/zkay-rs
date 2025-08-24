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
use num_bigint::Sign;
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct NonZeroCheckBasicOp;

impl NonZeroCheckBasicOp {
    pub fn new(
        w: &WireType,
        out1: &WireType,
        out2: &WireType,
        desc: String,
    ) -> Op<NonZeroCheckBasicOp> {
        Op::<NonZeroCheckBasicOp> {
            inputs: vec![Some(w.clone())],
            outputs: vec![Some(out1.clone()), Some(out2.clone())],
            desc,
            t: NonZeroCheckBasicOp,
        }
    }
}
crate::impl_instruction_for!(Op<NonZeroCheckBasicOp>);
crate::impl_hash_code_for!(Op<NonZeroCheckBasicOp>);
impl BasicOp for Op<NonZeroCheckBasicOp> {
    fn getOpcode(&self) -> String {
        "zerop".to_owned()
    }

    fn compute(&self, mut assignment: &mut Vec<Option<BigInteger>>) {
        let (in0_id, out0_id, out1_id) = (
            self.inputs[0].as_ref().unwrap().getWireId() as usize,
            self.outputs[0].as_ref().unwrap().getWireId() as usize,
            self.outputs[1].as_ref().unwrap().getWireId() as usize,
        );
        // if out0_id == 48124 || out0_id == 4{
        //     println!(
        //         "==compute=====outputs===={out0_id}======{}===={}====",
        //         file!(),
        //         self.outputs[0].as_ref().unwrap().name()
        //     );
        // }
        if assignment[in0_id].as_ref().unwrap().sign() == Sign::NoSign {
            assignment[out1_id] = Some(BigInteger::ZERO);
        } else {
            assignment[out1_id] = Some(Util::one());
        }
        assignment[out0_id] = Some(BigInteger::ZERO); // a dummy value
    }

    fn getNumMulGates(&self) -> i32 {
        2
    }
}
impl Eq for Op<NonZeroCheckBasicOp> {}
impl PartialEq for Op<NonZeroCheckBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
    }
}
