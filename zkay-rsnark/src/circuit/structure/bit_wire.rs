#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::add_basic_op::{AddBasicOp, new_add};
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::or_basic_op::{OrBasicOp, new_or};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::operations::wire_label_instruction::LabelType::output;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::linear_combination_bit_wire::{
    LinearCombinationBitWire, new_linear_combination_bit,
};
use crate::circuit::structure::linear_combination_wire::{
    LinearCombinationWire, new_linear_combination,
};
use crate::circuit::structure::variable_bit_wire::{VariableBitWire, new_variable_bit};
use crate::circuit::structure::variable_wire::{VariableWire, new_variable};
use crate::circuit::structure::wire::{Wire,GetWireId, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::InstanceOf;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct BitWire;
crate::impl_hash_code_of_wire_for!(Wire<BitWire>);
crate::impl_name_instance_of_wire_for!(Wire<BitWire>);
pub fn new_bit(wireId: i32) -> Wire<BitWire> {
    // super(wireId);
    Wire::<BitWire> { wireId, t: BitWire }
}
impl setBitsConfig for BitWire {}
impl setBitsConfig for Wire<BitWire> {}
impl WireConfig for Wire<BitWire> {}

impl BitWireConfig for Wire<BitWire> {}
pub trait BitWireConfig:WireConfig{
    // fn generator(&self) -> CircuitGenerator {
    //     CircuitGenerator::getActiveCircuitGenerator()
    //         .unwrap()
    //         .clone()
    // }
     fn mul(&self, w: WireType, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            return BitWireConfig::mulb(self,w.try_as_constant_ref().unwrap().getConstant(), desc);
        }
        let output1 = if w.instance_of("BitWire") {
            WireType::VariableBit(new_variable_bit(
                *self.generator().currentWireId.borrow_mut(),
            ))
        } else {
            WireType::Variable(new_variable(*self.generator().currentWireId.borrow_mut()))
        };
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_mul(
            self.self_clone().unwrap(),
            w,
            output1.clone(),
            desc.clone(),
        );
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        output1
    }

    fn  mulb(&self, b: BigInteger, desc: &String) -> WireType {
        if b == BigInteger::ZERO {
            return self.generator().zeroWire.borrow().clone().unwrap();
        } else if b == Util::one() {
            return self.self_clone().unwrap();
        }
        let out = WireType::LinearCombination(new_linear_combination(
            *self.generator().currentWireId.borrow_mut(),
            None,
        ));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_const_mul(self.self_clone().unwrap(), out.clone(), b, desc.clone());
        //			self.generator().addToEvaluationQueue(Box::new(op));
        //			return out;
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }

    fn  invAsBit(&self, desc: &String) -> Option<WireType> {
        //		WireType neg = WireType::new(*self.generator().currentWireId.borrow_mut()+=1);
        //		Instruction op = ConstMulBasicOp::new(self, neg, -1, desc);
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let neg = BitWireConfig::mulb(self,Util::one().neg(), desc);
        let out = WireType::LinearCombinationBit(new_linear_combination_bit(
            *self.generator().currentWireId.borrow_mut(),
        ));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_add(
            vec![self.generator().oneWire.clone(), Some(neg)],
            out.clone(),
            desc.clone(),
        );
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone();
        }
        Some(out)
    }

    fn  or(&self, w: WireType, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.orw(self.self_clone().unwrap(), desc);
        }
        if w.instance_of("BitWire") {
            let out = WireType::VariableBit(new_variable_bit(
                *self.generator().currentWireId.borrow_mut(),
            ));
            *self.generator().currentWireId.borrow_mut() += 1;
            let op = new_or(self.self_clone().unwrap(), w, out.clone(), desc.clone());
            let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                *self.generator().currentWireId.borrow_mut() -= 1;
                cachedOutputs[0].clone().unwrap()
            } else {
                out
            };
        }
        return self.orw(w, desc);
    }

    fn  xor(&self, w: WireType, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            return w.xorw(self.self_clone().unwrap(), desc);
        }
        if w.instance_of("BitWire") {
            let out = WireType::VariableBit(new_variable_bit(
                *self.generator().currentWireId.borrow_mut(),
            ));
            *self.generator().currentWireId.borrow_mut() += 1;
            let op = new_xor(self.self_clone().unwrap(), w, out.clone(), desc.clone());
            let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                *self.generator().currentWireId.borrow_mut() -= 1;
                cachedOutputs[0].clone().unwrap()
            } else {
                out
            };
        }
        self.xorw(w, desc)
    }

    fn  getBits(&self, w: WireType, bitwidth: i32, desc: &String) -> WireArray {
        return WireArray::new(vec![Some(self.self_clone().unwrap())])
            .adjustLength(None, bitwidth as usize);
    }
}
