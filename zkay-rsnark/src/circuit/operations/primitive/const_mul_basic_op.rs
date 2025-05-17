#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::operations::primitive::basic_op::Op;
 use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::util::util::{Util,BigInteger};

 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct ConstMulBasicOp {
    constInteger: BigInteger,
    inSign: bool,
}

pub fn newConstMulBasicOp(
    w: WireType,
    out: WireType,
    mut constInteger: BigInteger,
    desc: Vec<String>,
) -> Op<ConstMulBasicOp> {
    let inSign = constInteger.signum() == -1;
    if !inSign {
        constInteger = Util::modulo(constInteger, Configs.get().unwrap().field_prime);
    } else {
        let mut _constInteger = constInteger.negate();
        _constInteger = Util::modulo(_constInteger, Configs.get().unwrap().field_prime);
        constInteger = Configs.get().unwrap().field_prime.subtract(_constInteger);
    }
    Op::<ConstMulBasicOp> {
        inputs: vec![w],
        outputs: vec![out],
        desc: desc[0].clone(),
        t: ConstMulBasicOp {
            constInteger,
            inSign,
        },
    }
}
impl BasicOp for Op<ConstMulBasicOp> {
    fn getOpcode(&self) -> String {
        if !self.inSign {
            format!("const-mul-{:x}", self.constInteger)
        } else {
            format!(
                "const-mul-neg-{:x}",
                Configs.get().unwrap().field_prime.subtract(self.constInteger)
            )
        }
    }

    fn compute(&self, assignment: Vec<BigInteger>) {
        let mut result = assignment[self.inputs[0].getWireId()].multiply(self.t.constInteger);
        if result.bitLength() >= Configs.get().unwrap().log2_field_prime {
            result = result.modulo(Configs.get().unwrap().field_prime);
        }
        assignment[self.outputs[0].getWireId()] = result;
    }

    fn equals(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }
        let op = rhs;
        self.inputs[0].equals(op.inputs[0]) && self.t.constInteger.equals(op.t.constInteger)
    }

    fn getNumMulGates(&self) -> i32 {
        return 0;
    }

    fn hashCode(&self) -> i32 {
        let mut h = self.t.constInteger.hashCode();
        for i in self.inputs {
            h += i.hashCode();
        }
        h
    }
}
