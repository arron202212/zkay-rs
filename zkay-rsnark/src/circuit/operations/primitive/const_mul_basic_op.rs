#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use num_bigint::Sign;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct ConstMulBasicOp {
    pub constInteger: BigInteger,
    pub inSign: bool,
}

pub fn new_const_mul(
    w: WireType,
    out: WireType,
    mut constInteger: BigInteger,
    desc: Vec<String>,
) -> Op<ConstMulBasicOp> {
    let inSign = constInteger.sign() == Sign::Minus;
    if !inSign {
        constInteger = Util::modulo(constInteger, Configs.get().unwrap().field_prime.clone());
    } else {
        let mut _constInteger = constInteger.neg();
        _constInteger = Util::modulo(_constInteger, Configs.get().unwrap().field_prime.clone());
        constInteger = Configs.get().unwrap().field_prime.clone().sub(_constInteger);
    }
    Op::<ConstMulBasicOp> {
        inputs: vec![Some(w)],
        outputs: vec![Some(out)],
        desc: desc[0].clone(),
        t: ConstMulBasicOp {
            constInteger,
            inSign,
        },
    }
}
crate::impl_instruction_for!(Op<ConstMulBasicOp>);
impl BasicOp for Op<ConstMulBasicOp> {
    fn getOpcode(&self) -> String {
        if !self.t.inSign {
            format!("const-mul-{:x}", self.t.constInteger)
        } else {
            format!(
                "const-mul-neg-{:x}",
                Configs.get().unwrap().field_prime.clone().sub(self.t.constInteger.clone())
            )
        }
    }

    fn compute(&self, mut assignment: Vec<Option<BigInteger>>) {
        let mut result = assignment[self.inputs[0].as_ref().unwrap().getWireId() as usize]
            .clone()
            .unwrap()
            .mul(self.t.constInteger.clone());
        if result.bits() >= Configs.get().unwrap().log2_field_prime {
            result = result.rem(Configs.get().unwrap().field_prime.clone());
        }
        assignment[self.outputs[0].as_ref().unwrap().getWireId() as usize] = Some(result);
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }
        let op = rhs;
        self.inputs[0]
            .as_ref()
            .unwrap()
            .equals(op.inputs[0].as_ref().unwrap())
            && self.t.constInteger == op.t.constInteger
    }

    fn getNumMulGates(&self) -> i32 {
        return 0;
    }

    fn hashCode(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.t.constInteger.hash(&mut hasher);
        let mut h = hasher.finish();
        for i in &self.inputs {
            h += i.as_ref().unwrap().hashCode();
        }
        h
    }
}
