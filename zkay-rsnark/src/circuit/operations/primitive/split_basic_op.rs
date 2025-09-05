#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        config::config::Configs,
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
    pub fn new(w: &WireType, outs: Vec<Option<WireType>>, desc: String) -> Op<SplitBasicOp> {
        // assert!(outs.len()!=16,"==SplitBasicOp====outs==len=={}==",outs.len());
        Op::<SplitBasicOp> {
            inputs: vec![Some(w.clone())],
            outputs: outs,
            desc,
            t: SplitBasicOp,
        }
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
        let bits_len = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap()
            .bits() as usize;
        assert!(
            self.outputs.len() >= bits_len,
            "Error in Split --- The number of bits does not fit -- Input: {:x},{self:?}\n\t{},{}",
            assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
                .clone()
                .unwrap(),
            self.outputs.len(),
            bits_len
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let in0_id = self.inputs[0].as_ref().unwrap().getWireId() as usize;

        let mut inVal = assignment[in0_id].clone().unwrap();
        if inVal > Configs.field_prime {
            inVal = inVal.rem(&Configs.field_prime);
        }
        for i in 0..self.outputs.len() {
            let outi_id = self.outputs[i].as_ref().unwrap().getWireId() as usize;
            assignment[outi_id] = Some(if inVal.bit(i as u64) {
                Util::one()
            } else {
                BigInteger::ZERO
            });
            // if outi_id == 48124 || outi_id == 4{
            //         println!(
            //             "==compute=====outputs====={outi_id}===={}===={}==={}====))))===",
            //             file!(),
            //             self.outputs[i].as_ref().unwrap().name(),
            //             assignment[outi_id]
            //                 .as_ref()
            //                 .unwrap()
            //         );
            //     }
        }
        Ok(())
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
