#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        config::config::CONFIGS,
        operations::primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        structure::{wire::GetWireId, wire_type::WireType},
    },
    util::util::BigInteger,
};
use num_bigint::Sign;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Add, Mul, Rem},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct MulBasicOp;
impl MulBasicOp {
    pub fn new(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<Self> {
        Op::<Self>::new(
            vec![Some(w1.clone()), Some(w2.clone())],
            vec![Some(output.clone())],
            desc,
            Self,
        )
        .unwrap()
    }
}
crate::impl_instruction_for!(Op<MulBasicOp>);
crate::impl_hash_code_for!(Op<MulBasicOp>);
impl BasicOp for Op<MulBasicOp> {
    fn get_op_code(&self) -> String {
        "mul".to_owned()
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let (in0_id, in1_id, out0_id) = (
            self.inputs[0].as_ref().unwrap().get_wire_id() as usize,
            self.inputs[1].as_ref().unwrap().get_wire_id() as usize,
            self.outputs[0].as_ref().unwrap().get_wire_id() as usize,
        );
        let mut result = assignment[in0_id]
            .clone()
            .unwrap()
            .mul(assignment[in1_id].as_ref().unwrap());
        if result.sign() == Sign::Minus {
            result = result.add(&CONFIGS.field_prime).rem(&CONFIGS.field_prime);
        }
        if result > CONFIGS.field_prime {
            result = result.rem(&CONFIGS.field_prime);
        }

        assignment[out0_id] = Some(result);

        Ok(())
    }

    fn get_num_mul_gates(&self) -> i32 {
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
