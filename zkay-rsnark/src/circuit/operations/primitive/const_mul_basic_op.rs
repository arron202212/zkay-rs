#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::{
    circuit::{
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

use num_bigint::Sign;
use zkay_derive::{ImplOpCodeConfig, ImplStructNameConfig};
#[derive(Debug, Clone, Hash, PartialEq, ImplOpCodeConfig, ImplStructNameConfig)]
pub struct ConstMulBasicOp {
    pub constInteger: BigInteger,
    pub inSign: bool,
}

pub fn new_const_mul(
    w: &WireType,
    out: &WireType,
    constInteger: &BigInteger,
    desc: String,
) -> Op<ConstMulBasicOp> {
    let inSign = constInteger.sign() == Sign::Minus;
    let constInteger = if !inSign {
        Util::modulo(constInteger, &Configs.field_prime)
    } else {
        let mut _constInteger = constInteger.neg();
        _constInteger = Util::modulo(&_constInteger, &Configs.field_prime);
        Configs.field_prime.clone().sub(_constInteger)
    };
    Op::<ConstMulBasicOp> {
        inputs: vec![Some(w.clone())],
        outputs: vec![Some(out.clone())],
        desc,
        t: ConstMulBasicOp {
            constInteger,
            inSign,
        },
    }
}
crate::impl_instruction_for!(Op<ConstMulBasicOp>);
// crate::impl_hash_code_for!(Op<ConstMulBasicOp>);
impl BasicOp for Op<ConstMulBasicOp> {
    fn getOpcode(&self) -> String {
        if !self.t.inSign {
            format!("const-mul-{:x}", self.t.constInteger)
        } else {
            format!(
                "const-mul-neg-{:x}",
                Configs.field_prime.clone().sub(self.t.constInteger.clone())
            )
        }
    }

    fn compute(&self, mut assignment: &mut Vec<Option<BigInteger>>) {
        let mut result = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap()
            .mul(self.t.constInteger.clone());
        if result.bits() >= Configs.log2_field_prime {
            result = result.rem(Configs.field_prime.clone());
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = Some(result);
    }

    fn getNumMulGates(&self) -> i32 {
        0
    }

    // fn hashCode(&self) -> u64 {
    //     let mut hasher = DefaultHasher::new();
    //     self.t.constInteger.hash(&mut hasher);
    //     let mut h = hasher.finish();
    //     for i in &self.inputs {
    //         h += i.as_ref().unwrap().hashCode();
    //     }
    //     h
    // }
}
impl Eq for Op<ConstMulBasicOp> {}
impl PartialEq for Op<ConstMulBasicOp> {
    fn eq(&self, other: &Self) -> bool {
        self.inputs[0].as_ref().unwrap() == other.inputs[0].as_ref().unwrap()
            && self.t.constInteger == other.t.constInteger
    }
}

impl Hash for Op<ConstMulBasicOp> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.constInteger.hash(state);
        let mut h = 0;
        for i in self.getInputs() {
            h += i.as_ref().unwrap().getWireId() as u64;
        }
        h.hash(state);
    }
}
