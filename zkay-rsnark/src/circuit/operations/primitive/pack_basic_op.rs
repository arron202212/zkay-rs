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
    ops::{Add, Mul, Rem},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct PackBasicOp;

impl PackBasicOp {
    pub fn new(in_bits: Vec<Option<WireType>>, out: &WireType, desc: String) -> Op<Self> {
        Op::<Self>::new(in_bits, vec![Some(out.clone())], desc, Self).unwrap()
    }
}
crate::impl_instruction_for!(Op<PackBasicOp>);
crate::impl_hash_code_for!(Op<PackBasicOp>);
impl BasicOp for Op<PackBasicOp> {
    fn get_op_code(&self) -> String {
        "pack".to_owned()
    }

    fn check_inputs(&self, assignment: &Vec<Option<BigInteger>>) {
        self.super_check_inputs(assignment);
        assert!(
            (0..self.inputs.len()).all(|i| Util::is_binary(
                assignment[self.inputs[i].as_ref().unwrap().get_wire_id() as usize]
                    .as_ref()
                    .unwrap()
            )),
            "Error - Input(s) to Pack are not binary.{self:?} During Evaluation "
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let out0_id = self.outputs[0].as_ref().unwrap().get_wire_id() as usize;
        let sum = self
            .inputs
            .iter()
            .enumerate()
            .fold(BigInteger::ZERO, |sum, (i, w)| {
                sum.add(
                    assignment[w.as_ref().unwrap().get_wire_id() as usize]
                        .as_ref()
                        .unwrap()
                        .mul(BigInteger::from(2).pow(i as u32)),
                )
            });

        assignment[out0_id] = Some(sum.rem(&CONFIGS.field_prime));
        Ok(())
    }

    fn get_num_mul_gates(&self) -> i32 {
        0
    }
}
impl Eq for Op<PackBasicOp> {}
impl PartialEq for Op<PackBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        if other.inputs.len() != self.inputs.len() {
            return false;
        }

        let mut check = true;
        for i in 0..self.inputs.len() {
            check = check && self.inputs[i].as_ref().unwrap() == other.inputs[i].as_ref().unwrap();
        }
        check
    }
}
