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
    util::util::{BigInteger, Util},
};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Rem,
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct SplitBasicOp;
impl SplitBasicOp {
    pub fn new(w: &WireType, outs: Vec<Option<WireType>>, desc: String) -> Op<Self> {
        Op::<Self>::new(vec![Some(w.clone())], outs, desc, Self).unwrap()
    }
}
crate::impl_instruction_for!(Op<SplitBasicOp>);
crate::impl_hash_code_for!(Op<SplitBasicOp>);
impl BasicOp for Op<SplitBasicOp> {
    fn get_op_code(&self) -> String {
        "split".to_owned()
    }

    fn check_inputs(&self, assignment: &Vec<Option<BigInteger>>) {
        self.super_check_inputs(assignment);
        let bits_len = assignment[self.inputs[0].as_ref().unwrap().get_wire_id() as usize]
            .clone()
            .unwrap()
            .bits() as usize;
        assert!(
            self.outputs.len() >= bits_len,
            "Error in Split --- The number of bits does not fit -- Input: {:x},{self:?}\n\t{},{}",
            assignment[self.inputs[0].as_ref().unwrap().get_wire_id() as usize]
                .clone()
                .unwrap(),
            self.outputs.len(),
            bits_len
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let in0_id = self.inputs[0].as_ref().unwrap().get_wire_id() as usize;

        let mut in_val = assignment[in0_id].clone().unwrap();
        if in_val > CONFIGS.field_prime {
            in_val = in_val.rem(&CONFIGS.field_prime);
        }
        for i in 0..self.outputs.len() {
            let outi_id = self.outputs[i].as_ref().unwrap().get_wire_id() as usize;
            assignment[outi_id] = Some(if in_val.bit(i as u64) {
                Util::one()
            } else {
                BigInteger::ZERO
            });
        }
        Ok(())
    }

    fn get_num_mul_gates(&self) -> i32 {
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
