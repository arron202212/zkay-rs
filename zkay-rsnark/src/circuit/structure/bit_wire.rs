#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::add_basic_op::{AddBasicOp, new_add};
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::or_basic_op::{ORBasicOp, new_or};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::operations::wire_label_instruction::LabelType::output;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::linear_combination_bit_wire::LinearCombinationBitWire;
use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct BitWire;

impl setBitsConfig for BitWire {}
impl BitWire {
    pub fn BitWire(wireId: i32) -> Self {
        // super(wireId);
        Self
    }
    pub fn generator(&self) -> CircuitGenerator {
        CircuitGenerator::getActiveCircuitGenerator()
            .unwrap()
            .clone()
    }

    pub fn mul(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return self.mulb(w.getConstant(), desc);
        }
        let output1 = if w.instance_of("BitWire") {
            WireType::VariableBit(VariableBitWire::new(self.generator().currentWireId))
        } else {
            WireType::Variable(VariableWire::new(self.generator().currentWireId))
        };
        self.generator().currentWireId += 1;
        let op = new_mul(WireType::Bit(self.clone()), w, output1, desc);
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        output1
    }

    pub fn mulb(&self, b: BigInteger, desc: Vec<String>) -> WireType {
        if b == BigInteger::ZERO {
            return self.generator().zeroWire.clone().unwrap();
        } else if b == Util::one() {
            return WireType::Bit(self.clone());
        }
        let out =
            WireType::LinearCombination(LinearCombinationWire::new(self.generator().currentWireId));
        self.generator().currentWireId += 1;
        let op = new_const_mul(WireType::Bit(self.clone()), out, b, desc);
        //			self.generator().addToEvaluationQueue(Box::new(op));
        //			return out;
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    pub fn invAsBit(&self, desc: Vec<String>) -> WireType {
        //		WireType neg = WireType::new(self.generator().currentWireId+=1);
        //		Instruction op = ConstMulBasicOp::new(self, neg, -1, desc);
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let neg = self.mulb(Util::one().neg(), desc);
        let out = WireType::LinearCombinationBit(LinearCombinationBitWire::new(
            self.generator().currentWireId,
        ));
        self.generator().currentWireId += 1;
        let op = new_add(vec![self.generator().oneWire.clone(), Some(neg)], out, desc);
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    pub fn or(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.or(WireType::Bit(self.clone()), desc);
        }
        if w.instance_of("BitWire") {
            let out = WireType::VariableBit(VariableBitWire::new(self.generator().currentWireId));
            self.generator().currentWireId += 1;
            let op = new_or(WireType::Bit(self.clone()), w, out, desc);
            let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                self.generator().currentWireId -= 1;
                cachedOutputs[0].clone().unwrap()
            } else {
                out
            };
        }
        return WireConfig::or(&WireType::Bit(self.clone()), w, desc);
    }

    pub fn xor(&self, w: WireType, desc: Vec<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.xor(WireType::Bit(self.clone()), desc);
        }
        if w.instance_of("BitWire") {
            let out = WireType::VariableBit(VariableBitWire::new(self.generator().currentWireId));
            self.generator().currentWireId += 1;
            let op = new_xor(WireType::Bit(self.clone()), w, out, desc);
            let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                self.generator().currentWireId -= 1;
                cachedOutputs[0].clone().unwrap()
            } else {
                out
            };
        }
        WireConfig::xor(&WireType::Bit(self.clone()), w, desc)
    }

    pub fn getBits(&self, w: WireType, bitwidth: i32, desc: Vec<String>) -> WireArray {
        return WireArray::new(vec![Some(WireType::Bit(self.clone()))])
            .adjustLengthi(bitwidth as usize);
    }
}
