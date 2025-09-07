#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        StructNameConfig,
        config::config::CONFIGS,
        operations::primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        structure::{
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
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
pub struct AssertBasicOp;
impl AssertBasicOp {
    pub fn new(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<AssertBasicOp> {
        // if w1.get_wire_id()==4 && w2.get_wire_id()==0 && output.get_wire_id()==48124{
        //     panic!("{},{},{}",w1.name(),w2.name(),output.name());
        // }
        let start = std::time::Instant::now();

        let w1 = Some(w1.clone());

        let w2 = Some(w2.clone());

        let output = Some(output.clone());

        let t = AssertBasicOp;

        let inputs = vec![w1, w2];

        let outputs = vec![output];

        Op::<AssertBasicOp> {
            inputs,
            outputs,
            desc,
            t,
        }
    }
}
crate::impl_instruction_for!(Op<AssertBasicOp>);
crate::impl_hash_code_for!(Op<AssertBasicOp>);
// impl crate::circuit::eval::instruction::Instruction for Op<AssertBasicOp>{
//      fn basic_op(&self) -> Option<Box<dyn BasicOp>> {
//         Box::new(self.clone())
//     }
// }
impl BasicOp for Op<AssertBasicOp> {
    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let (in0_id, in1_id, out0_id) = (
            self.inputs[0].as_ref().unwrap().get_wire_id() as usize,
            self.inputs[1].as_ref().unwrap().get_wire_id() as usize,
            self.outputs[0].as_ref().unwrap().get_wire_id() as usize,
        );

        let left_side = assignment[in0_id]
            .clone()
            .unwrap()
            .mul(assignment[in1_id].as_ref().unwrap())
            .rem(&CONFIGS.field_prime);
        let right_side = assignment[out0_id].clone().unwrap();

        eyre::ensure!(
            left_side == right_side,
            "Error During Evaluation    {} * {} != {}  in {} * {} != {}",
            assignment[in0_id].as_ref().unwrap(),
            assignment[in1_id].as_ref().unwrap(),
            assignment[out0_id].as_ref().unwrap(),
            in0_id,
            in1_id,
            out0_id
        );
        Ok(())
    }

    fn check_outputs(&self, assignment: &Vec<Option<BigInteger>>) {
        // do nothing
    }

    fn get_op_code(&self) -> String {
        "assert".to_owned()
    }

    fn get_num_mul_gates(&self) -> i32 {
        1
    }
}

impl Eq for Op<AssertBasicOp> {}
impl PartialEq for Op<AssertBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        return (check1 || check2)
            && self.outputs[0].as_ref().unwrap() == other.outputs[0].as_ref().unwrap();
    }
}
