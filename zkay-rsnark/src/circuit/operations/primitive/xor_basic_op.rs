#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
        operations::primitive::basic_op::{BasicOp, BasicOpInOut, Op},
        structure::{wire::GetWireId, wire_type::WireType},
    },
    util::util::{BigInteger, Util},
};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::BitXor,
};
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct XorBasicOp;
impl XorBasicOp {
    pub fn new(w1: &WireType, w2: &WireType, output: &WireType, desc: String) -> Op<XorBasicOp> {
        // if w1.get_wire_id()==147444 ||  w2.get_wire_id()==147444
        // {
        //     panic!("===XorBasicOp::new====w1.as_ref().unwrap().get_wire_id()========================{}",w1.get_wire_id());
        // }
        Op::<XorBasicOp> {
            inputs: vec![Some(w1.clone()), Some(w2.clone())],
            outputs: vec![Some(output.clone())],
            desc,
            t: XorBasicOp,
        }
    }
}
crate::impl_instruction_for!(Op<XorBasicOp>);
crate::impl_hash_code_for!(Op<XorBasicOp>);
impl BasicOp for Op<XorBasicOp> {
    fn get_op_code(&self) -> String {
        "xor".to_owned()
    }

    fn check_inputs(&self, assignment: &Vec<Option<BigInteger>>) {
        // //super.check_inputs(assignment);
        self.super_check_inputs(assignment);
        let check = Util::is_binary(
            assignment[self.inputs[0].as_ref().unwrap().get_wire_id() as usize]
                .as_ref()
                .unwrap(),
        ) && Util::is_binary(
            assignment[self.inputs[1].as_ref().unwrap().get_wire_id() as usize]
                .as_ref()
                .unwrap(),
        );
        assert!(
            check,
            "Error - Input(s) to XOR are not binary.{self:?} During Evaluation"
        );
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let (in0_id, in1_id, out0_id) = (
            self.inputs[0].as_ref().unwrap().get_wire_id() as usize,
            self.inputs[1].as_ref().unwrap().get_wire_id() as usize,
            self.outputs[0].as_ref().unwrap().get_wire_id() as usize,
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
            .map(|x| x.bitxor(assignment[in1_id].as_ref().unwrap()));
        Ok(())
    }

    fn get_num_mul_gates(&self) -> i32 {
        1
    }
}

impl Eq for Op<XorBasicOp> {}
impl PartialEq for Op<XorBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        let check1 = self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[1].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        let check2 = self.inputs[1].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.inputs[0].as_ref().unwrap() == other.inputs[1].as_ref().unwrap();
        check1 || check2
    }
}
