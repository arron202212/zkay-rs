#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        InstanceOf, StructNameConfig,
        eval::instruction::Instruction,
        operations::{
            primitive::{
                add_basic_op::AddBasicOp, const_mul_basic_op::ConstMulBasicOp,
                mul_basic_op::MulBasicOp, or_basic_op::OrBasicOp, xor_basic_op::XorBasicOp,
            },
            wire_label_instruction::LabelType::output,
        },
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                addToEvaluationQueue, getActiveCircuitGenerator,
            },
            linear_combination_bit_wire::LinearCombinationBitWire,
            linear_combination_wire::LinearCombinationWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
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

use rccell::{RcCell, WeakCell};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct BitWire;
//crate::impl_hash_code_of_wire_g_for!(Wire<BitWire>);
crate::impl_name_instance_of_wire_g_for!(Wire<BitWire>);

impl BitWire {
    pub fn new(wireId: i32, generator: WeakCell<CircuitGenerator>) -> Wire<BitWire> {
        //   if wireId>0 && wireId<10000
        // {
        //     println!("=new_bit======={wireId}==");
        // }
        // //super(wireId);
        // Wire::<BitWire> {
        //     wireId,
        //     generator,
        //     t: BitWire,
        // }
        // crate::new_wire!(BitWire,wireId,generator)
        Wire::<BitWire>::new(BitWire, wireId, generator).unwrap()
    }
}
impl setBitsConfig for BitWire {}
impl setBitsConfig for Wire<BitWire> {}
impl WireConfig for Wire<BitWire> {
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::Bit(self.clone()))
    }
}

impl BitWireConfig for Wire<BitWire> {}
pub trait BitWireConfig: WireConfig {
    fn mul(&self, w: &WireType, desc: &Option<String>) -> WireType {
        println!(
            "===w.instance_of(ConstantWire)========={w:?}==={}====={}===={}=======",
            w.name(),
            line!(),
            file!()
        );

        if w.instance_of("ConstantWire") {
            return BitWireConfig::mulb(
                self,
                &w.try_as_constant_ref().unwrap().getConstant(),
                desc,
            );
        }
        let mut generator = self.generator();

        let output1 = if w.instance_of("BitWire") {
            WireType::VariableBit(VariableBitWire::new(
                generator.get_current_wire_id(),
                self.generator_weak(),
            ))
        } else {
            WireType::Variable(VariableWire::new(
                generator.get_current_wire_id(),
                self.generator_weak(),
            ))
        };
        generator.borrow_mut().current_wire_id += 1;
        let op = MulBasicOp::new(
            &self.self_clone().unwrap(),
            w,
            &output1,
            desc.clone().unwrap_or(String::new()),
        );

        let cachedOutputs = addToEvaluationQueue(generator.clone(), Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());

            cachedOutputs[0].clone().unwrap()
        } else {
            output1
        }
    }

    fn mulb(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        // println!("End Name Time: bbbbb {} s", line!());
        let mut generator = self.generator();

        if b == &BigInteger::ZERO {
            return generator.get_zero_wire().unwrap();
        } else if b == &Util::one() {
            return self.self_clone().unwrap();
        }
        let out = WireType::LinearCombination(LinearCombinationWire::new(
            generator.get_current_wire_id(),
            None,
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = ConstMulBasicOp::new(
            &self.self_clone().unwrap(),
            &out,
            b,
            desc.clone().unwrap_or(String::new()),
        );
        //			generator.addToEvaluationQueue(Box::new(op));
        //			return out;

        let cachedOutputs = addToEvaluationQueue(generator.clone(), Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());

            cachedOutputs[0].clone().unwrap()
        } else {
            out
        }
    }

    fn invAsBit(&self, desc: &Option<String>) -> Option<WireType> {
        //		WireTypeneg = WireType::new(generator.borrow_mut().current_wire_id+=1);
        //		Instruction op = ConstMulBasicOp::new(self, neg, -1, desc);
        //		generator.addToEvaluationQueue(Box::new(op));
        let mut generator = self.generator();

        let neg = BitWireConfig::mulb(self, &Util::one().neg(), desc);
        let out = WireType::LinearCombinationBit(LinearCombinationBitWire::new(
            generator.get_current_wire_id(),
            self.generator_weak(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = AddBasicOp::new(
            vec![generator.get_one_wire(), Some(neg)],
            &out,
            desc.clone().unwrap_or(String::new()),
        );
        //		generator.addToEvaluationQueue(Box::new(op));

        let cachedOutputs = addToEvaluationQueue(generator.clone(), Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            generator.borrow_mut().current_wire_id -= 1;
            //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());

            cachedOutputs[0].clone()
        } else {
            Some(out)
        }
    }

    fn or(&self, w: &WireType, desc: &Option<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            // println!(
            //     "===w.instance_of(ConstantWire)================={}===={}=======",
            //     line!(),
            //     file!()
            // );
            return w.orw(self.self_clone().as_ref().unwrap(), desc);
        }
        let mut generator = self.generator();

        if w.instance_of("BitWire") {
            let out = WireType::VariableBit(VariableBitWire::new(
                generator.get_current_wire_id(),
                self.generator_weak(),
            ));
            generator.borrow_mut().current_wire_id += 1;
            let op = OrBasicOp::new(
                &self.self_clone().unwrap(),
                w,
                &out,
                desc.clone().unwrap_or(String::new()),
            );

            let cachedOutputs = addToEvaluationQueue(generator.clone(), Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                generator.borrow_mut().current_wire_id -= 1;
                //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());

                cachedOutputs[0].clone().unwrap()
            } else {
                out
            };
        }
        self.orw(w, desc)
    }

    fn xor(&self, w: &WireType, desc: &Option<String>) -> WireType {
        if w.instance_of("ConstantWire") {
            // println!(
            //     "===w.instance_of(ConstantWire)================={}===={}=======",
            //     line!(),
            //     file!()
            // );
            return w.xorw(self.self_clone().as_ref().unwrap(), desc);
        }
        let mut generator = self.generator();

        if w.instance_of("BitWire") {
            let out = WireType::VariableBit(VariableBitWire::new(
                generator.get_current_wire_id(),
                self.generator_weak(),
            ));
            generator.borrow_mut().current_wire_id += 1;
            let op = XorBasicOp::new(
                &self.self_clone().unwrap(),
                w,
                &out,
                desc.clone().unwrap_or(String::new()),
            );

            let cachedOutputs = addToEvaluationQueue(generator.clone(), Box::new(op));
            if let Some(cachedOutputs) = cachedOutputs {
                generator.borrow_mut().current_wire_id -= 1;
                //println!("====generator.borrow_mut().current_wire_id======{}====={}{}",generator.borrow_mut().current_wire_id ,file!(),line!());
                cachedOutputs[0].clone().unwrap()
            } else {
                out
            }
        } else {
            self.xorw(w, desc)
        }
    }

    fn getBits(&self, w: &WireType, bitwidth: i32, desc: &Option<String>) -> WireArray {
        return WireArray::new(
            vec![Some(self.self_clone().unwrap())],
            self.generator_weak(),
        )
        .adjustLength(None, bitwidth as usize);
    }
}
