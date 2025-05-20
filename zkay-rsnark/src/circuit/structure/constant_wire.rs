#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::const_mul_basic_op::ConstMulBasicOp;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array::WireArray;
 use crate::util::util::{Util,BigInteger};
use num_bigint::Sign;
use std::hash::{DefaultHasher, Hash, Hasher};
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash,PartialEq)]
pub struct ConstantWire {
pub constant: BigInteger,
}
impl ConstantWire {
    pub fn new(wireId: i32, value: BigInteger) -> Self {
        // super(wireId);
       Self {constant : value.modulo(Configs.get().unwrap().field_prime)}
    }

    pub fn getConstant(&self,) -> BigInteger {
        return self.constant.clone();
    }

    pub fn isBinary(&self,) -> bool {
        return self.constant.equals(Util::one()) || self.constant.equals(BigInteger::ZERO);
    }

    pub fn mul(&self,w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return self
                .generator
                .createConstantWire(self.constant.mul(w.constant), desc);
        } else {
            return w.mul(self.constant, desc);
        }
    }

    pub fn mulb(&self,b: BigInteger, desc: Vec<String>) -> WireType {
        let sign = b.sign() == Sign::Minus;
        let newConstant = self.constant.mul(b).modulo(Configs.get().unwrap().field_prime);

        let mut out = self.generator().knownConstantWires.get(newConstant);
        if out.is_some() {
            return out;
        }

        out = ConstantWire::new(
            self.generator().currentWireId,
            if !sign {
                newConstant
            } else {
                newConstant.sub(Configs.get().unwrap().field_prime)
            },
        );

        self.generator().currentWireId += 1;
        let op = ConstMulBasicOp::new(self, out, b, desc);
        let cachedOutputs = self.generator().addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            // self branch might not be needed
            self.generator().currentWireId -= 1;
            return cachedOutputs[0];
        }

        self.generator().knownConstantWires.put(newConstant, out);
        out
    }

    pub fn checkNonZero(&self,w: WireType, desc: Vec<String>) -> WireType {
        if self.constant.equals(BigInteger::ZERO) {
            return self.generator().zeroWire;
        } else {
            return self.generator().oneWire;
        }
    }

    pub fn invAsBit(&self,desc: Vec<String>) -> WireType {
        assert!(self.isBinary(), "Trying to invert a non-binary constant!");

        if self.constant.equals(BigInteger::ZERO) {
            self.generator().oneWire
        } else {
            self.generator().zeroWire
        }
    }

    pub fn or(&self,w: WireType, desc: Vec<String>) -> WireType {
        if  w.instance_of("ConstantWire") {
            let cw=w;
            assert!(
                self.isBinary() && cw.isBinary(),
                "Trying to OR two non-binary constants"
            );
            return if self.constant.equals(BigInteger::ZERO && cw.getConstant().equals(BigInteger::ZERO)) {
                self.generator().zeroWire
            } else {
                self.generator().oneWire
            };
        }
        if self.constant.equals(Util::one()) {
            self.generator().oneWire
        } else {
            w
        }
    }

    pub fn xor(&self,w: WireType, desc: Vec<String>) -> WireType {
         if  w.instance_of("ConstantWire") {
            let cw=w;
            assert!(
                isBinary() && cw.isBinary(),
                "Trying to XOR two non-binary constants"
            );
            return if self.constant.equals(cw.getConstant()) {
                self.generator().zeroWire
            } else {
                self.generator().oneWire
            };
        }
        if self.constant.equals(Util::one()) {
            w.invAsBit(desc)
        } else {
            w
        }
    }

    pub fn getBitWires(&self,bitwidth: u64, desc: Vec<String>) -> WireArray {
        assert!(
            self.constant.bitLength() <= bitwidth,
            "Trying to split a constant of {} bits into  {bitwidth} bits",
            self.constant.bitLength()
        );
        let mut bits = vec![ConstantWire::default(); bitwidth];
        for i in 0..bitwidth {
            bits[i] = if self.constant.testBit(i) {
                self.generator().oneWire
            } else {
                self.generator().zeroWire
            };
        }
        return WireArray::new(bits);
    }

    pub fn restrictBitLength(&self,bitwidth: u64, desc: Vec<String>) {
       self.getBitWires(bitwidth, desc);
    }

    fn pack(&self,desc: Vec<String>) {}
}
