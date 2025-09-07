#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use super::wire_ops::{
    AddWire, AndBitwise, IsEqualTo, IsGreaterThan, IsGreaterThanOrEqual, IsLessThan,
    IsLessThanOrEqual, MulWire, OrBitwise, SubWire, XorBitwise,
};
use crate::{
    circuit::{
        InstanceOf, StructNameConfig,
        eval::instruction::Instruction,
        operations::primitive::{
            basic_op::Op, const_mul_basic_op::ConstMulBasicOp, mul_basic_op::MulBasicOp,
            non_zero_check_basic_op::NonZeroCheckBasicOp, or_basic_op::OrBasicOp,
            pack_basic_op::PackBasicOp, split_basic_op::SplitBasicOp, xor_basic_op::XorBasicOp,
        },
        structure::{
            bit_wire::BitWire,
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                CreateConstantWire, add_to_evaluation_queue, get_active_circuit_generator,
            },
            constant_wire::ConstantWire,
            linear_combination_bit_wire::LinearCombinationBitWire,
            linear_combination_wire::LinearCombination_wire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{Base, GeneratorConfig, GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
        },
    },
    util::util::{ARcCell, BigInteger, Util},
};

use enum_dispatch::enum_dispatch;
use rccell::{RcCell, WeakCell};
use std::{
    fmt::{self, Debug},
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, BitAnd, BitOr, BitXor, Mul, Sub},
    sync::Arc,
};

use strum_macros::{EnumIs, EnumTryAs};
#[enum_dispatch(
    GetWireId,
    InstanceOf,
    StructNameConfig,
    GeneratorConfig,
    WireConfig,
    SetBitsConfig
)]
#[derive(Debug, EnumIs, EnumTryAs)]
pub enum WireType {
    Wire(Wire<Base>),
    LinearCombinationBit(Wire<LinearCombinationBitWire>),
    LinearCombination(Wire<LinearCombination_wire>),
    Variable(Wire<VariableWire>),
    VariableBit(Wire<VariableBitWire>),
    Constant(Wire<ConstantWire>),
    Bit(Wire<BitWire>),
}
// impl  Default for WireType{
//     fn default() -> Self {
//         Self::Wire(Wire::<Base>::new(-1, Base).unwrap())
//     }
// }
impl WireType {
    // pub fn instance_of(&self, name: &str) -> bool {
    //     self.name() == name
    // }
    // fn name(&self) -> &str {
    //     ""
    // }

    // pub fn get_constant(&self) -> BigInteger {
    //     BigInteger::ZERO
    // }
    // pub fn is_binary(&self) -> bool {
    //     false
    // }
    // pub fn inv_as_bit(&self, desc: &Option<String>) -> Option<WireType> {
    //     None
    // }
    // pub fn get_bit_wires_if_exist_already(&self) -> Option<WireArray> {
    //     self.get_bit_wires()
    // }
    // pub fn pack_if_needed(&self, desc: &Option<String>) {
    //     // if self.wire_id == -1 {
    //     //     self.pack();
    //     // }
    // }
}
// impl SetBitsConfig for WireType{}
// impl WireConfig for WireType{}
impl MulWire<&BigInteger> for WireType {
    fn mul_wire(&self, b: &BigInteger, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        if b == &Util::one() {
            return self.self_clone().unwrap();
        }
        if b == &BigInteger::ZERO {
            return generator.get_zero_wire().unwrap();
        }
        let out = WireType::LinearCombination(LinearCombination_wire::new(
            generator.get_current_wire_id(),
            None,
            generator.clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = ConstMulBasicOp::new(
            &self.self_clone().unwrap(),
            &out,
            b,
            desc.clone().unwrap_or(String::new()),
        );
        //		generator.add_to_evaluation_queue(Box::new(op));

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            cached_outputs[0].clone().unwrap()
        } else {
            out
        }
    }
}
impl MulWire<i64> for WireType {
    fn mul_wire(&self, l: i64, desc: &Option<String>) -> Self {
        self.mulb(&BigInteger::from(l), desc)
    }
}
impl MulWire<(i64, i32)> for WireType {
    fn mul_wire(&self, (base, exp): (i64, i32), desc: &Option<String>) -> Self {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        self.mulb(&b, desc)
    }
}
impl MulWire<&WireType> for WireType {
    fn mul_wire(&self, w: &Self, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        if w.instance_of("ConstantWire") {
            return self.mulb(&w.try_as_constant_ref().unwrap().get_constant(), desc);
        }
        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let output = WireType::Variable(VariableWire::new(
            generator.get_current_wire_id(),
            generator.clone().downgrade(),
        ));
        generator.borrow_mut().current_wire_id += 1;
        let op = MulBasicOp::new(
            &self.self_clone().unwrap(),
            w,
            &output,
            desc.clone().unwrap_or(String::new()),
        );

        let cached_outputs = add_to_evaluation_queue(generator.clone(), Box::new(op));
        if let Some(cached_outputs) = cached_outputs {
            generator.borrow_mut().current_wire_id -= 1;
            cached_outputs[0].clone().unwrap()
        } else {
            output
        }
    }
}

impl AddWire<&WireType> for WireType {
    fn add_wire(&self, w: &Self, desc: &Option<String>) -> Self {
        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        WireArray::new(
            vec![Some(self.self_clone().unwrap()), Some(w.clone())],
            self.generator().clone().downgrade(),
        )
        .sum_all_elements(desc)
    }
}
impl AddWire<i64> for WireType {
    fn add_wire(&self, v: i64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.addw(&generator.create_constant_wire(v, desc), desc)
    }
}
impl AddWire<&BigInteger> for WireType {
    fn add_wire(&self, b: &BigInteger, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.addw(&generator.create_constant_wire(b, desc), desc)
    }
}
impl SubWire<&WireType> for WireType {
    fn sub_wire(&self, w: &Self, desc: &Option<String>) -> Self {
        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let neg = w.muli(-1, desc);
        self.addw(&neg, desc)
    }
}
impl SubWire<i64> for WireType {
    fn sub_wire(&self, v: i64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.subw(&generator.create_constant_wire(v as i64, desc), desc)
    }
}
impl SubWire<&BigInteger> for WireType {
    fn sub_wire(&self, b: &BigInteger, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.subw(&generator.create_constant_wire(b, desc), desc)
    }
}

impl XorBitwise<&WireType> for WireType {
    fn xor_bitwise(&self, w: &Self, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi(num_bits as u64, desc);
        let result = bits1.xor_wire_array(&bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            generator.create_constant_wire(&v, &None)
        } else {
            WireType::LinearCombination(LinearCombination_wire::new(
                -1,
                Some(result),
                generator.clone().downgrade(),
            ))
        }
    }
}

impl XorBitwise<i64> for WireType {
    fn xor_bitwise(&self, v: i64, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.xor_bitwises(&generator.create_constant_wire(v, desc), num_bits, desc)
    }
}

impl XorBitwise<&BigInteger> for WireType {
    fn xor_bitwise(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.xor_bitwises(&generator.create_constant_wire(b, desc), num_bits, desc)
    }
}

impl AndBitwise<&WireType> for WireType {
    fn and_bitwise(&self, w: &Self, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi(num_bits as u64, desc);
        let result = bits1.and_wire_array(&bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            generator.create_constant_wire(&v, &None)
        } else {
            WireType::LinearCombination(LinearCombination_wire::new(
                -1,
                Some(result),
                generator.clone().downgrade(),
            ))
        }
    }
}

impl AndBitwise<i64> for WireType {
    fn and_bitwise(&self, v: i64, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.and_bitwise(&generator.create_constant_wire(v, desc), num_bits, desc)
    }
}

impl AndBitwise<&BigInteger> for WireType {
    fn and_bitwise(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.and_bitwise(&generator.create_constant_wire(b, desc), num_bits, desc)
    }
}

impl OrBitwise<&WireType> for WireType {
    fn or_bitwise(&self, w: &Self, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        let bits1 = self.get_bit_wiresi(num_bits as u64, desc);
        let bits2 = w.get_bit_wiresi(num_bits as u64, desc);
        let result = bits1.or_wire_array(bits2, num_bits as usize, desc);
        let v = result.check_if_constant_bits(desc);
        if let Some(v) = v {
            generator.create_constant_wire(&v, &None)
        } else {
            WireType::LinearCombination(LinearCombination_wire::new(
                -1,
                Some(result),
                generator.clone().downgrade(),
            ))
        }
    }
}
impl OrBitwise<i64> for WireType {
    fn or_bitwise(&self, v: i64, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.or_bitwises(&generator.create_constant_wire(v, desc), num_bits, desc)
    }
}
impl OrBitwise<&BigInteger> for WireType {
    fn or_bitwise(&self, b: &BigInteger, num_bits: u64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.or_bitwises(&generator.create_constant_wire(b, desc), num_bits, desc)
    }
}
impl IsEqualTo<&WireType> for WireType {
    fn is_equal_to(&self, w: &Self, desc: &Option<String>) -> Self {
        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let s = self.subw(w, desc);
        s.check_non_zero(desc).inv_as_bit(desc).unwrap()
    }
}
impl IsEqualTo<&BigInteger> for WireType {
    fn is_equal_to(&self, b: &BigInteger, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_equal_tos(&generator.create_constant_wire(b, desc), &None)
    }
}
impl IsEqualTo<i64> for WireType {
    fn is_equal_to(&self, v: i64, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_equal_tos(&generator.create_constant_wire(v, desc), &None)
    }
}
impl IsLessThanOrEqual<&WireType> for WireType {
    fn is_less_than_or_equal(&self, w: &Self, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(w, desc).subw(&self.self_clone().unwrap(), desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize].clone().unwrap()
    }
}
impl IsLessThanOrEqual<i64> for WireType {
    fn is_less_than_or_equal(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_less_than_or_equals(&generator.create_constant_wire(v, desc), bitwidth, desc)
    }
}
impl IsLessThanOrEqual<&BigInteger> for WireType {
    fn is_less_than_or_equal(&self, b: &BigInteger, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_less_than_or_equals(&generator.create_constant_wire(b, desc), bitwidth, desc)
    }
}
impl IsLessThan<&WireType> for WireType {
    fn is_less_than(&self, w: &Self, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(&self.self_clone().unwrap(), desc).subw(w, desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize]
            .as_ref()
            .unwrap()
            .inv_as_bit(desc)
            .unwrap()
    }
}
impl IsLessThan<i64> for WireType {
    fn is_less_than(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_less_thans(&generator.create_constant_wire(v, desc), bitwidth, desc)
    }
}
impl IsLessThan<&BigInteger> for WireType {
    fn is_less_than(&self, b: &BigInteger, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_less_thans(&generator.create_constant_wire(b, desc), bitwidth, desc)
    }
}
impl IsGreaterThanOrEqual<&WireType> for WireType {
    fn is_greater_than_or_equal(&self, w: &Self, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(&self.self_clone().unwrap(), desc).subw(w, desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize].clone().unwrap()
    }
}
impl IsGreaterThanOrEqual<i64> for WireType {
    fn is_greater_than_or_equal(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_greater_than_or_equals(&generator.create_constant_wire(v, desc), bitwidth, desc)
    }
}
impl IsGreaterThanOrEqual<&BigInteger> for WireType {
    fn is_greater_than_or_equal(
        &self,
        b: &BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> Self {
        let mut generator = self.generator();

        self.is_greater_than_or_equals(&generator.create_constant_wire(b, desc), bitwidth, desc)
    }
}
impl IsGreaterThan<&WireType> for WireType {
    fn is_greater_than(&self, w: &Self, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.pack_if_needed(desc);
        w.pack_if_needed(desc);
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = generator.create_constant_wire(&p, desc);
        let sum = pWire.addw(w, desc).subw(&self.self_clone().unwrap(), desc);
        let bit_wires = sum.get_bit_wiresi(bitwidth as u64 + 1, desc);
        bit_wires[bitwidth as usize]
            .clone()
            .unwrap()
            .inv_as_bit(desc)
            .unwrap()
    }
}
impl IsGreaterThan<i64> for WireType {
    fn is_greater_than(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_greater_thans(&generator.create_constant_wire(v, desc), bitwidth, desc)
    }
}
impl IsGreaterThan<&BigInteger> for WireType {
    fn is_greater_than(&self, b: &BigInteger, bitwidth: i32, desc: &Option<String>) -> Self {
        let mut generator = self.generator();

        self.is_greater_thans(&generator.create_constant_wire(b, desc), bitwidth, desc)
    }
}

impl Add for WireType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_wire(&rhs, &None)
    }
}
impl Add<&WireType> for WireType {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self.add_wire(rhs, &None)
    }
}
impl Add<i64> for WireType {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        self.add_wire(rhs, &None)
    }
}

impl Add<&BigInteger> for WireType {
    type Output = Self;

    fn add(self, rhs: &BigInteger) -> Self::Output {
        self.add_wire(rhs, &None)
    }
}
impl Sub<&WireType> for WireType {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        self.sub_wire(rhs, &None)
    }
}

impl Sub for WireType {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_wire(&rhs, &None)
    }
}

impl Sub<i64> for WireType {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        self.sub_wire(rhs, &None)
    }
}
impl Sub<&BigInteger> for WireType {
    type Output = Self;

    fn sub(self, rhs: &BigInteger) -> Self::Output {
        self.sub_wire(rhs, &None)
    }
}

impl Mul<i64> for WireType {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        self.mul_wire(rhs, &None)
    }
}

impl Mul<&BigInteger> for WireType {
    type Output = Self;

    fn mul(self, rhs: &BigInteger) -> Self::Output {
        self.mul_wire(rhs, &None)
    }
}

impl Mul<&WireType> for WireType {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        self.mul_wire(rhs, &None)
    }
}
impl Mul for WireType {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_wire(&rhs, &None)
    }
}

impl BitAnd for WireType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and_bitwise(&rhs, 32, &None)
    }
}

impl BitAnd<&WireType> for WireType {
    type Output = Self;

    fn bitand(self, rhs: &Self) -> Self::Output {
        self.and_bitwise(rhs, 32, &None)
    }
}

impl BitAnd<i64> for WireType {
    type Output = Self;

    fn bitand(self, rhs: i64) -> Self::Output {
        self.and_bitwise(rhs, 32, &None)
    }
}

impl BitAnd<&BigInteger> for WireType {
    type Output = Self;

    fn bitand(self, rhs: &BigInteger) -> Self::Output {
        self.and_bitwise(rhs, 32, &None)
    }
}
impl BitOr for WireType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or_bitwise(&rhs, 32, &None)
    }
}

impl BitOr<i64> for WireType {
    type Output = Self;

    fn bitor(self, rhs: i64) -> Self::Output {
        self.or_bitwise(rhs, 32, &None)
    }
}
impl BitOr<&BigInteger> for WireType {
    type Output = Self;

    fn bitor(self, rhs: &BigInteger) -> Self::Output {
        self.or_bitwise(rhs, 32, &None)
    }
}

impl BitXor for WireType {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor_bitwise(&rhs, 32, &None)
    }
}

impl BitXor<i64> for WireType {
    type Output = Self;

    fn bitxor(self, rhs: i64) -> Self::Output {
        self.xor_bitwise(rhs, 32, &None)
    }
}
impl BitXor<&BigInteger> for WireType {
    type Output = Self;

    fn bitxor(self, rhs: &BigInteger) -> Self::Output {
        self.xor_bitwise(rhs, 32, &None)
    }
}

impl std::fmt::Display for WireType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wire(w) => w.to_string(),
                Self::LinearCombinationBit(w) => w.to_string(),
                Self::LinearCombination(w) => w.to_string(),
                Self::Variable(w) => w.to_string(),
                Self::VariableBit(w) => w.to_string(),
                Self::Constant(w) => w.to_string(),
                Self::Bit(w) => w.to_string(),
            }
        )
    }
}

impl Hash for WireType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Wire(w) => w.hash(state),
            Self::LinearCombinationBit(w) => w.hash(state),
            Self::LinearCombination(w) => w.hash(state),
            Self::Variable(w) => w.hash(state),
            Self::VariableBit(w) => w.hash(state),
            Self::Constant(w) => w.hash(state),
            Self::Bit(w) => w.hash(state),
        }
    }
}
impl Eq for WireType {}
impl PartialEq for WireType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Wire(w), Self::Wire(o)) => w.eq(o),
            (Self::LinearCombinationBit(w), Self::LinearCombinationBit(o)) => w.eq(o),
            (Self::LinearCombination(w), Self::LinearCombination(o)) => w.eq(o),
            (Self::Variable(w), Self::Variable(o)) => w.eq(o),
            (Self::VariableBit(w), Self::VariableBit(o)) => w.eq(o),
            (Self::Constant(w), Self::Constant(o)) => w.eq(o),
            (Self::Bit(w), Self::Bit(o)) => w.eq(o),
            _ => false,
        }
    }
}

impl Clone for WireType {
    fn clone(&self) -> Self {
        match self {
            Self::Wire(w) => Self::Wire(w.clone()),
            Self::LinearCombinationBit(w) => Self::LinearCombinationBit(w.clone()),
            Self::LinearCombination(w) => Self::LinearCombination(w.clone()),
            Self::Variable(w) => Self::Variable(w.clone()),
            Self::VariableBit(w) => Self::VariableBit(w.clone()),
            Self::Constant(w) => Self::Constant(w.clone()),
            Self::Bit(w) => Self::Bit(w.clone()),
        }
    }
}
