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
    ops::BitOr,
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct OrBasicOp;
impl OrBasicOp {
    pub fn new(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<OrBasicOp> {
        use std::time::Instant;
        let _start = Instant::now();
        let op = Op::<OrBasicOp> {
            inputs: vec![Some(w1.clone()), Some(w2.clone())],
            outputs: vec![Some(output.clone())],
            desc,
            t: OrBasicOp,
        };
        // println!(
        //     "EndOrBasicOp::new 0 Time: == {:?} ",
        //     start.elapsed()
        // );
        op
    }
}
crate::impl_instruction_for!(Op<OrBasicOp>);
crate::impl_hash_code_for!(Op<OrBasicOp>);
impl BasicOp for Op<OrBasicOp> {
    fn getOpcode(&self) -> String {
        "or".to_owned()
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
        if !check {
            //println!("Error - Input(s) to OR are not binary.{self:?} ");
            panic!("Error During Evaluation");
        }
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let (in0_id, in1_id, out0_id) = (
            self.inputs[0].as_ref().unwrap().getWireId() as usize,
            self.inputs[1].as_ref().unwrap().getWireId() as usize,
            self.outputs[0].as_ref().unwrap().getWireId() as usize,
        );
        // if out0_id == 48124 || out0_id == 4{
        //     println!(
        //         "==compute=====outputs===={out0_id}======{}===={}====",
        //         file!(),
        //         self.outputs[0].as_ref().unwrap().name()
        //     );
        // }
        assignment[out0_id] = assignment[in0_id]
            .as_ref()
            .map(|x| x.clone().bitor(assignment[in1_id].as_ref().unwrap()));
        Ok(())
    }

    fn getNumMulGates(&self) -> i32 {
        1
    }
}
impl Eq for Op<OrBasicOp> {}
impl PartialEq for Op<OrBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        check1 || check2
    }
}
