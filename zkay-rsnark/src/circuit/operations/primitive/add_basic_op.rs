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
    util::util::BigInteger,
};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Add, Rem},
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct AddBasicOp;
impl AddBasicOp {
    pub fn new(ws: Vec<Option<WireType>>, output: &WireType, desc: String) -> Op<AddBasicOp> {
        Op::<AddBasicOp> {
            inputs: ws,
            outputs: vec![Some(output.clone())],
            desc,
            t: AddBasicOp,
        }
    }
}
crate::impl_instruction_for!(Op<AddBasicOp>);
crate::impl_hash_code_for!(Op<AddBasicOp>);
impl BasicOp for Op<AddBasicOp> {
    // fn get_op_code(&self) -> String {
    //     return "add".to_owned();
    // }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let out0_id = self.outputs[0].as_ref().unwrap().get_wire_id() as usize;
        // if out0_id == 48124 || out0_id == 4{
        //     println!(
        //         "==compute=====outputs==={out0_id}======={}===={}====",
        //         file!(),
        //         self.outputs[0].as_ref().unwrap().name()
        //     );
        // }
        let s = self.inputs.iter().fold(BigInteger::ZERO, |s, w| {
            s.add(
                assignment[w.as_ref().unwrap().get_wire_id() as usize]
                    .as_ref()
                    .unwrap(),
            )
        });
        if out0_id == 48124 || out0_id == 4 {
            println!(
                "=={}={out0_id}==value=={s}",
                s.clone().rem(&Configs.field_prime)
            );
        }
        assignment[out0_id] = Some(s.rem(&Configs.field_prime));
        Ok(())
    }

    fn get_num_mul_gates(&self) -> i32 {
        0
    }
}

impl Eq for Op<AddBasicOp> {}
impl PartialEq for Op<AddBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        if other.inputs.len() != self.inputs.len() {
            return false;
        }

        if self.inputs.len() == 2 {
            let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
                && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
            let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
                && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
            return check1 || check2;
        }

        self.inputs
            .iter()
            .zip(&other.inputs)
            .all(|(a, b)| a.as_ref().unwrap() == b.as_ref().unwrap())
    }
}
