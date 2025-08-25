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
pub struct PackBasicOp;

impl PackBasicOp {
    pub fn new(inBits: Vec<Option<WireType>>, out: &WireType, desc: String) -> Op<PackBasicOp> {
        Op::<PackBasicOp> {
            inputs: inBits,
            outputs: vec![Some(out.clone())],
            desc,
            t: PackBasicOp,
        }
    }
}
crate::impl_instruction_for!(Op<PackBasicOp>);
crate::impl_hash_code_for!(Op<PackBasicOp>);
impl BasicOp for Op<PackBasicOp> {
    fn getOpcode(&self) -> String {
        "pack".to_owned()
    }

    fn checkInputs(&self, assignment: &Vec<Option<BigInteger>>) {
        // //super.checkInputs(assignment);
        self.super_checkInputs(assignment);
        assert!(
            (0..self.inputs.len()).all(|i| Util::isBinary(
                assignment[self.inputs[i].as_ref().unwrap().getWireId() as usize]
                    .as_ref()
                    .unwrap()
            )),
            "Error - Input(s) to Pack are not binary.{self:?} During Evaluation "
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let out0_id = self.outputs[0].as_ref().unwrap().getWireId() as usize;
        // if out0_id == 48124 || out0_id == 4{
        //     println!(
        //         "==compute=====outputs===={out0_id}======{}===={}====",
        //         file!(),
        //         self.outputs[0].as_ref().unwrap().name()
        //     );
        // }
        let sum = self
            .inputs
            .iter()
            .enumerate()
            .fold(BigInteger::ZERO, |sum, (i, w)| {
                sum.add(
                    assignment[w.as_ref().unwrap().getWireId() as usize]
                        .as_ref()
                        .unwrap()
                        .mul(BigInteger::from(2).pow(i as u32)),
                )
            });

        assignment[out0_id] = Some(sum.rem(&Configs.field_prime));
        Ok(())
    }

    fn getNumMulGates(&self) -> i32 {
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
