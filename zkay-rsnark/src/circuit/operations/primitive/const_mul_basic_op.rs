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
    ops::{Mul, Neg, Rem, Sub},
};

use num_bigint::Sign;
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct ConstMulBasicOp {
    pub const_integer: BigInteger,
    pub in_sign: bool,
}

impl ConstMulBasicOp {
    pub fn new(w: &WireType, out: &WireType, const_integer: &BigInteger, desc: String) -> Op<Self> {
        let in_sign = const_integer.sign() == Sign::Minus;
        let const_integer = if !in_sign {
            Util::modulo(const_integer, &CONFIGS.field_prime)
        } else {
            let mut _const_integer = const_integer.neg();
            _const_integer = Util::modulo(&_const_integer, &CONFIGS.field_prime);
            CONFIGS.field_prime.clone().sub(_const_integer)
        };
        Op::<Self>::new(
            vec![Some(w.clone())],
            vec![Some(out.clone())],
            desc,
            Self {
                const_integer,
                in_sign,
            },
        )
        .unwrap()
    }
}
crate::impl_instruction_for!(Op<ConstMulBasicOp>);
impl BasicOp for Op<ConstMulBasicOp> {
    fn get_op_code(&self) -> String {
        if !self.t.in_sign {
            format!("const-mul-{:x}", self.t.const_integer)
        } else {
            format!(
                "const-mul-neg-{:x}",
                CONFIGS
                    .field_prime
                    .clone()
                    .sub(self.t.const_integer.clone())
            )
        }
    }

    fn compute(&self, assignment: &mut Vec<Option<BigInteger>>) -> eyre::Result<()> {
        let (in0_id, out0_id) = (
            self.inputs[0].as_ref().unwrap().get_wire_id() as usize,
            self.outputs[0].as_ref().unwrap().get_wire_id() as usize,
        );

        let mut result = assignment[in0_id]
            .clone()
            .unwrap()
            .mul(&self.t.const_integer);
        if result.bits() >= CONFIGS.log2_field_prime {
            result = result.rem(&CONFIGS.field_prime);
        }

        assignment[out0_id] = Some(result);
        Ok(())
    }

    fn get_num_mul_gates(&self) -> i32 {
        0
    }
}
impl Eq for Op<ConstMulBasicOp> {}
impl PartialEq for Op<ConstMulBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.t.const_integer == other.t.const_integer
    }
}

impl Hash for Op<ConstMulBasicOp> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.const_integer.hash(state);
        let mut inputs = self.get_inputs();
        // inputs.sort_unstable_by_key(|x|x.as_ref().unwrap().get_wire_id());
        for i in inputs {
            i.as_ref().unwrap().get_wire_id().hash(state);
        }
    }
}
