#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::add_basic_op::AddBasicOp;
use crate::circuit::operations::primitive::const_mul_basic_op::ConstMulBasicOp;
use crate::circuit::operations::primitive::mul_basic_op::MulBasicOp;
use crate::circuit::operations::primitive::or_basic_op::ORBasicOp;
use crate::circuit::operations::primitive::xor_basic_op::XorBasicOp;
 use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;
use crate::circuit::structure::linear_combination_bit_wire::LinearCombinationBitWire;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::operations::wire_label_instruction::LabelType::output;
 use crate::util::util::{Util,BigInteger};

 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct BitWire;

impl BitWire {
    // pub  BitWire(wireId:i32 ) {
    // 	super(wireId);
    // }

    pub fn mul(&self,w: WireType, desc: String) -> WireType {
        if w.instanceof("ConstantWire") {
            return self.mul(w.getConstant(), desc);
        }
        let output = if w.instanceof(BitWire) {
            VariableBitWire::new(self.generator().currentWireId)
        } else {
            output = VariableWire::new(self.generator().currentWireId)
        };
        self.generator().currentWireId += 1;
        let op = MulBasicOp::new(self, w, output, desc);
        let cachedOutputs = self.generator().addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0];
        }
        output
    }

    pub fn mulb(&self,b: BigInteger, desc: Vec<String>) -> WireType {
        if b.equals(BigInteger::ZERO) {
            return self.generator().zeroWire;
        } else if b.equals(Util::one()) {
            return self;
        }
        let out = LinearCombinationWire::new(self.generator().currentWireId += 1);
        let op = ConstMulBasicOp::new(self, out, b, desc);
        //			self.generator().addToEvaluationQueue(op);
        //			return out;
        let cachedOutputs = self.generator().addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn invAsBit(&self,desc: Vec<String>) -> WireType {
        //		WireType neg = WireType::new(self.generator().currentWireId+=1);
        //		Instruction op = ConstMulBasicOp::new(self, neg, -1, desc);
        //		self.generator().addToEvaluationQueue(op);
        let neg = self.mul(-1, desc);
        let out = LinearCombinationBitWire::new(self.generator().currentWireId);
        self.generator().currentWireId += 1;
        let op = AddBasicOp::new(vec![self.generator().oneWire, neg], out, desc);
        //		self.generator().addToEvaluationQueue(op);
        let cachedOutputs = self.generator().addToEvaluationQueue(op);
        if let Some(cachedOutputs) = cachedOutputs {
            self.generator().currentWireId -= 1;
            return cachedOutputs[0].clone();
        }
        out
    }

    pub fn or(&self,w: WireType, desc: Vec<String>) -> WireType {
        if w.instanceof("ConstantWire") {
            return w.or(self, desc);
        }
        if w.instanceof(BitWire) {
            let out = VariableBitWire::new(self.generator().currentWireId);
            self.generator().currentWireId += 1;
            let op = ORBasicOp::new(self, w, out, desc);
            let cachedOutputs = self.generator().addToEvaluationQueue(op);
            return if let Some(cachedOutputs) = cachedOutputs {
                self.generator().currentWireId -= 1;
                cachedOutputs[0].clone()
            } else {
                out
            };
        }
        return WireConfig::or(&self,w, desc);
    }

    pub fn xor(&self,w: WireType, desc: Vec<String>) -> WireType {
        if w.instanceof("ConstantWire") {
            return w.xor(self, desc);
        }
        if w.instanceof("BitWire") {
            let out = VariableBitWire::new(self.generator().currentWireId);
            self.generator().currentWireId += 1;
            let op = XorBasicOp::new(self, w, out, desc);
            let cachedOutputs = self.generator().addToEvaluationQueue(op);
            return if let Some(cachedOutputs) = cachedOutputs {
                self.generator().currentWireId -= 1;
                cachedOutputs[0].clone()
            } else {
                out
            };
        }
        WireConfig::xor(&self,w, desc)
    }

    pub fn getBits(&self,w: WireType, bitwidth: i32, desc: Vec<String>) -> WireArray {
        return WireArray::new(vec![self]).adjustLength(bitwidth);
    }
}
