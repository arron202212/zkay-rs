#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire::{Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use num_bigint::Sign;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct ConstantWire {
    pub constant: BigInteger,
}
pub fn new_constant(wireId: i32, value: BigInteger) -> Wire<ConstantWire> {
    // super(wireId);
    Wire::<ConstantWire> {
        wireId,
        t: ConstantWire {
            constant: value.rem(Configs.get().unwrap().field_prime.clone()),
        },
    }
}

impl setBitsConfig for ConstantWire {}
impl setBitsConfig for Wire<ConstantWire> {}
impl Wire<ConstantWire> {
    // pub fn new(wireId: i32, value: BigInteger) -> Self {
    //     // super(wireId);
    //     Self {
    //         constant: value.rem(Configs.get().unwrap().field_prime.clone()),
    //     }
    // }
    pub fn generator(&self) -> CircuitGenerator {
        CircuitGenerator::getActiveCircuitGenerator()
            .unwrap()
            .clone()
    }
    pub fn getConstant(&self) -> BigInteger {
        return self.t.constant.clone();
    }

    pub fn isBinary(&self) -> bool {
        return self.t.constant == Util::one() || self.t.constant == BigInteger::ZERO;
    }

    pub fn mul(&self, w: WireType, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            return self
                .generator()
                .createConstantWire(self.t.constant.clone().mul(w.getConstant().clone()), desc);
        } else {
            return w.mulb(self.t.constant.clone(), desc);
        }
    }

    pub fn mulb(&self, b: BigInteger, desc: &String) -> WireType {
        let sign = b.sign() == Sign::Minus;
        let newConstant = self
            .t
            .constant
            .clone()
            .mul(b.clone())
            .rem(Configs.get().unwrap().field_prime.clone());

        let mut out: Option<WireType> = self
            .generator()
            .knownConstantWires
            .borrow()
            .get(&newConstant)
            .cloned();
        if let Some(out) = out {
            return out.clone();
        }

        out = Some(WireType::Constant(new_constant(
            *self.generator().currentWireId.borrow_mut(),
            if !sign {
                newConstant.clone()
            } else {
                newConstant
                    .clone()
                    .sub(Configs.get().unwrap().field_prime.clone())
            },
        )));

        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_const_mul(
            WireType::Constant(self.clone()),
            out.clone().unwrap(),
            b.clone(),
            desc.clone(),
        );
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            // self branch might not be needed
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }

        self.generator()
            .knownConstantWires
            .borrow_mut()
            .insert(newConstant, out.clone().unwrap());
        out.clone().unwrap()
    }

    pub fn checkNonZero(&self, w: WireType, desc: &String) -> WireType {
        if self.t.constant == BigInteger::ZERO {
            return self.generator().zeroWire.borrow().clone().unwrap();
        } else {
            return self.generator().oneWire.clone().unwrap();
        }
    }

    pub fn invAsBit(&self, desc: &String) -> WireType {
        assert!(self.isBinary(), "Trying to invert a non-binary constant!");

        if self.t.constant == BigInteger::ZERO {
            self.generator().oneWire.clone().unwrap()
        } else {
            self.generator().zeroWire.borrow().clone().unwrap()
        }
    }

    pub fn or(&self, w: WireType, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            let cw = w;
            assert!(
                self.isBinary() && cw.isBinary(),
                "Trying to OR two non-binary constants"
            );
            return if self.t.constant == BigInteger::ZERO && cw.getConstant() == BigInteger::ZERO {
                self.generator().zeroWire.borrow().clone().unwrap()
            } else {
                self.generator().oneWire.clone().unwrap()
            };
        }
        if self.t.constant == Util::one() {
            self.generator().oneWire.clone().unwrap()
        } else {
            w
        }
    }

    pub fn xor(&self, w: WireType, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            let cw = w;
            assert!(
                self.isBinary() && cw.isBinary(),
                "Trying to XOR two non-binary constants"
            );
            return if self.t.constant == cw.getConstant() {
                self.generator().zeroWire.borrow().clone().unwrap()
            } else {
                self.generator().oneWire.clone().unwrap()
            };
        }
        if self.t.constant == Util::one() {
            w.invAsBit(desc).unwrap()
        } else {
            w
        }
    }

    pub fn getBitWires(&self, bitwidth: u64, desc: &String) -> WireArray {
        assert!(
            self.t.constant.bits() <= bitwidth,
            "Trying to split a constant of {} bits into  {bitwidth} bits",
            self.t.constant.bits()
        );
        let mut bits = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            bits[i] = if self.t.constant.bit(i as u64) {
                self.generator().oneWire.clone()
            } else {
                self.generator().zeroWire.borrow().clone()
            };
        }
        return WireArray::new(bits);
    }

    pub fn restrictBitLength(&self, bitwidth: u64, desc: &String) {
        self.getBitWires(bitwidth, desc);
    }

    fn pack(&self, desc: &String) {}
}
