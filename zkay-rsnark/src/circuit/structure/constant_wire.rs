#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        InstanceOf,
        config::config::CONFIGS,
        eval::instruction::Instruction,
        operations::primitive::const_mul_basic_op::ConstMulBasicOp,
        structure::{
            circuit_generator::CreateConstantWire,
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire::GeneratorConfig,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
};

use num_bigint::Sign;
use rccell::{RcCell, WeakCell};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Neg, Rem, Sub},
    time::Instant,
};
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, Hash, PartialEq, ImplStructNameConfig)]
pub struct ConstantWire {
    pub constant: BigInteger,
}
crate::impl_name_instance_of_wire_g_for!(Wire<ConstantWire>);
impl ConstantWire {
    pub fn new(
        wire_id: i32,
        value: BigInteger,
        generator: WeakCell<CircuitGenerator>,
    ) -> Wire<ConstantWire> {
        Wire::<ConstantWire>::new(
            ConstantWire {
                constant: value.rem(&CONFIGS.field_prime),
            },
            wire_id,
            generator,
        )
        .unwrap()
    }
}
impl SetBitsConfig for ConstantWire {}
impl SetBitsConfig for Wire<ConstantWire> {}
impl Wire<ConstantWire> {
    pub fn get_constant(&self) -> BigInteger {
        self.t.constant.clone()
    }

    pub fn is_binary(&self) -> bool {
        self.t.constant == Util::one() || self.t.constant == BigInteger::ZERO
    }
}
impl WireConfig for Wire<ConstantWire> {
    fn self_clone(&self) -> Option<WireType> {
        Some(WireType::Constant(self.clone()))
    }
    fn mulw_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let start = Instant::now();
        let generator = self.generator();
        if w.instance_of("ConstantWire") {
            generator.create_constant_wire_with_option(
                &self
                    .t
                    .constant
                    .clone()
                    .mul(&w.try_as_constant_ref().unwrap().get_constant()),
                desc,
            )
        } else {
            w.mulb_with_option(&self.t.constant, desc)
        }
    }

    fn mulb_with_option(&self, b: &BigInteger, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();

        let sign = b.sign() == Sign::Minus;
        let new_constant = self.t.constant.clone().mul(b).rem(&CONFIGS.field_prime);
        let mut out: Option<WireType> = generator
            .get_known_constant_wires()
            .get(&new_constant)
            .cloned();
        if let Some(out) = out {
            return out.clone();
        }
        out = Some(WireType::Constant(ConstantWire::new(
            generator.get_current_wire_id(),
            if !sign {
                new_constant.clone()
            } else {
                new_constant.clone().sub(&CONFIGS.field_prime)
            },
            self.generator.clone(),
        )));
        generator.borrow_mut().current_wire_id += 1;
        let op = ConstMulBasicOp::new(
            &WireType::Constant(self.clone()),
            out.as_ref().unwrap(),
            b,
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            // self branch might not be needed
            generator.borrow_mut().current_wire_id -= 1;
            cached_outputs[0].clone().unwrap()
        } else {
            generator
                .borrow_mut()
                .known_constant_wires
                .insert(new_constant, out.clone().unwrap());
            out.clone().unwrap()
        }
    }

    fn check_non_zero_with_option(&self, desc: &Option<String>) -> WireType {
        let generator = self.generator();

        if self.t.constant == BigInteger::ZERO {
            generator.get_zero_wire().unwrap()
        } else {
            generator.get_one_wire().unwrap()
        }
    }

    fn inv_as_bit_with_option(&self, desc: &Option<String>) -> Option<WireType> {
        assert!(self.is_binary(), "Trying to invert a non-binary constant!");

        let generator = self.generator();

        if self.t.constant == BigInteger::ZERO {
            generator.get_one_wire()
        } else {
            generator.get_zero_wire()
        }
    }

    fn orw_with_option(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let generator = self.generator();

        if w.instance_of("ConstantWire") {
            let cw = w;
            assert!(
                self.is_binary() && cw.try_as_constant_ref().unwrap().is_binary(),
                "Trying to OR two non-binary constants"
            );
            return if self.t.constant == BigInteger::ZERO
                && cw.try_as_constant_ref().unwrap().get_constant() == BigInteger::ZERO
            {
                generator.get_zero_wire().unwrap()
            } else {
                generator.get_one_wire().unwrap()
            };
        }
        if self.t.constant == Util::one() {
            generator.get_one_wire().unwrap()
        } else {
            w.clone()
        }
    }

    fn xorw(&self, w: &WireType, desc: &Option<String>) -> WireType {
        let generator = self.generator();

        if w.instance_of("ConstantWire") {
            let cw = w;
            assert!(
                self.is_binary() && cw.try_as_constant_ref().unwrap().is_binary(),
                "Trying to XOR two non-binary constants"
            );
            return if self.t.constant == cw.try_as_constant_ref().unwrap().get_constant() {
                generator.get_zero_wire().unwrap()
            } else {
                generator.get_one_wire().unwrap()
            };
        }
        if self.t.constant == Util::one() {
            w.inv_as_bit_with_option(desc).unwrap()
        } else {
            w.clone()
        }
    }

    fn get_bit_wiresi_with_option(&self, bitwidth: u64, desc: &Option<String>) -> WireArray {
        assert!(
            self.t.constant.bits() <= bitwidth,
            "Trying to split a constant of {} bits into  {bitwidth} bits",
            self.t.constant.bits()
        );
        let generator = self.generator();

        let mut bits = vec![None; bitwidth as usize];
        for i in 0..bitwidth as usize {
            bits[i] = if self.t.constant.bit(i as u64) {
                generator.get_one_wire()
            } else {
                generator.get_zero_wire()
            };
        }
        WireArray::new(bits, self.generator.clone())
    }

    fn restrict_bit_length_with_option(&self, bitwidth: u64, desc: &Option<String>) {
        self.get_bit_wiresi_with_option(bitwidth, desc);
    }

    fn pack(&self, desc: &Option<String>) {}
}
