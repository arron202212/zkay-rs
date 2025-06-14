#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use super::wire_ops::{
    AddWire, AndBitwise, IsEqualTo, IsGreaterThan, IsGreaterThanOrEqual, IsLessThan,
    IsLessThanOrEqual, MulWire, OrBitwise, SubWire, XorBitwise,
};
use crate::circuit::InstanceOf;
use crate::circuit::StructNameConfig;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::non_zero_check_basic_op::{
    NonZeroCheckBasicOp, new_non_zero_check,
};
use crate::circuit::operations::primitive::or_basic_op::{OrBasicOp, new_or};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::operations::primitive::split_basic_op::{SplitBasicOp, new_split};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::structure::bit_wire::BitWire;
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::{CircuitGenerator, getActiveCircuitGenerator};
use crate::circuit::structure::constant_wire::ConstantWire;
use crate::circuit::structure::linear_combination_bit_wire::LinearCombinationBitWire;
use crate::circuit::structure::linear_combination_wire::{
    LinearCombinationWire, new_linear_combination,
};
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::{VariableWire, new_variable};
use crate::circuit::structure::wire::Base;
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::util::util::ARcCell;
use crate::util::util::BigInteger;
use crate::util::util::Util;
use enum_dispatch::enum_dispatch;
use rccell::RcCell;
use std::fmt;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Sub};
use std::sync::Arc;
use strum_macros::{EnumIs, EnumTryAs};
#[enum_dispatch(
    GetWireId,
    InstanceOf,
    StructNameConfig,
    WireConfig,
    setBitsConfig,
    InstanceOf
)]
#[derive(Debug, Clone, Hash, PartialEq, EnumIs, EnumTryAs)]
pub enum WireType {
    Wire(Wire<Base>),
    LinearCombinationBit(Wire<LinearCombinationBitWire>),
    LinearCombination(Wire<LinearCombinationWire>),
    Variable(Wire<VariableWire>),
    VariableBit(Wire<VariableBitWire>),
    Constant(Wire<ConstantWire>),
    Bit(Wire<BitWire>),
}
impl Default for WireType {
    fn default() -> Self {
        Self::Wire(Wire::<Base>::new(-1, Base).unwrap())
    }
}
impl WireType {
    // pub fn instance_of(&self, name: &str) -> bool {
    //     self.name() == name
    // }
    // fn name(&self) -> &str {
    //     ""
    // }

    // pub fn getConstant(&self) -> BigInteger {
    //     BigInteger::ZERO
    // }
    // pub fn isBinary(&self) -> bool {
    //     false
    // }
    // pub fn invAsBit(&self, desc: &Option<String>) -> Option<WireType> {
    //     None
    // }
    // pub fn getBitWiresIfExistAlready(&self) -> Option<WireArray> {
    //     self.getBitWires()
    // }
    // pub fn packIfNeeded(&self, desc: &Option<String>) {
    //     // if self.wireId == -1 {
    //     //     self.pack();
    //     // }
    // }
}
// impl setBitsConfig for WireType {}
// impl WireConfig for WireType {}
impl MulWire<BigInteger> for WireType {
    fn mul_wire(self, b: BigInteger, desc: &Option<String>) -> Self {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.packIfNeeded(desc);
        if b == Util::one() {
            return self.self_clone().unwrap();
        }
        if b == BigInteger::ZERO {
            return generator.zero_wire().clone().unwrap();
        }
        let out =
            WireType::LinearCombination(new_linear_combination(*generator.current_wire_id(), None));
        *generator.current_wire_id() += 1;
        let op = new_const_mul(
            self.self_clone().unwrap(),
            out.clone(),
            b,
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        //		generator.addToEvaluationQueue(Box::new(op));
        let cachedOutputs = generator.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *generator.current_wire_id() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }
}
impl MulWire<i64> for WireType {
    fn mul_wire(self, l: i64, desc: &Option<String>) -> WireType {
        self.mulb(BigInteger::from(l), desc)
    }
}
impl MulWire<(i64, i32)> for WireType {
    fn mul_wire(self, (base, exp): (i64, i32), desc: &Option<String>) -> WireType {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        self.mulb(b, desc)
    }
}
impl MulWire for WireType {
    fn mul_wire(self, w: Self, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        if w.instance_of("ConstantWire") {
            return self.mulb(w.try_as_constant_ref().unwrap().getConstant(), desc);
        }
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let output = WireType::Variable(new_variable(*generator.current_wire_id()));
        *generator.current_wire_id() += 1;
        let op = new_mul(
            self.self_clone().unwrap(),
            w,
            output.clone(),
            desc.as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
        );
        let cachedOutputs = generator.addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *generator.current_wire_id() -= 1;
            cachedOutputs[0].clone().unwrap()
        } else {
            output
        }
    }
}

impl AddWire for WireType {
    fn add_wire(self, w: WireType, desc: &Option<String>) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        return WireArray::new(vec![Some(self.self_clone().unwrap()), Some(w)])
            .sumAllElements(desc);
    }
}
impl AddWire<i64> for WireType {
    fn add_wire(self, v: i64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.addw(generator.createConstantWirei(v, desc), desc)
    }
}
impl AddWire<BigInteger> for WireType {
    fn add_wire(self, b: BigInteger, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.addw(generator.createConstantWire(b, desc), desc)
    }
}
impl SubWire for WireType {
    fn sub_wire(self, w: WireType, desc: &Option<String>) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let neg = w.muli(-1, desc);
        self.addw(neg, desc)
    }
}
impl SubWire<i64> for WireType {
    fn sub_wire(self, v: i64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.subw(generator.createConstantWirei(v as i64, desc), desc);
    }
}
impl SubWire<BigInteger> for WireType {
    fn sub_wire(self, b: BigInteger, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.subw(generator.createConstantWire(b, desc), desc)
    }
}

impl XorBitwise for WireType {
    fn xor_bitwise(self, w: WireType, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.xorWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            generator.createConstantWire(v, &None)
        } else {
            WireType::LinearCombination(new_linear_combination(-1, Some(result)))
        }
    }
}

impl XorBitwise<i64> for WireType {
    fn xor_bitwise(self, v: i64, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.xorBitwise(generator.createConstantWirei(v, desc), numBits, desc);
    }
}

impl XorBitwise<BigInteger> for WireType {
    fn xor_bitwise(self, b: BigInteger, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.xorBitwise(generator.createConstantWire(b, desc), numBits, desc);
    }
}

impl AndBitwise for WireType {
    fn and_bitwise(self, w: WireType, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.andWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            generator.createConstantWire(v, &None)
        } else {
            WireType::LinearCombination(new_linear_combination(-1, Some(result)))
        }
    }
}

impl AndBitwise<i64> for WireType {
    fn and_bitwise(self, v: i64, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.andBitwise(generator.createConstantWirei(v, desc), numBits, desc);
    }
}

impl AndBitwise<BigInteger> for WireType {
    fn and_bitwise(self, b: BigInteger, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.andBitwise(generator.createConstantWire(b, desc), numBits, desc);
    }
}

impl OrBitwise for WireType {
    fn or_bitwise(self, w: WireType, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.orWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            generator.createConstantWire(v, &None)
        } else {
            WireType::LinearCombination(new_linear_combination(-1, Some(result)))
        }
    }
}
impl OrBitwise<i64> for WireType {
    fn or_bitwise(self, v: i64, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.orBitwise(generator.createConstantWirei(v, desc), numBits, desc);
    }
}
impl OrBitwise<BigInteger> for WireType {
    fn or_bitwise(self, b: BigInteger, numBits: u64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.orBitwise(generator.createConstantWire(b, desc), numBits, desc);
    }
}
impl IsEqualTo for WireType {
    fn is_equal_to(&self, w: WireType, desc: &Option<String>) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let s = self.subw(w, desc);
        s.checkNonZero(desc).invAsBit(desc).unwrap()
    }
}
impl IsEqualTo<BigInteger> for WireType {
    fn is_equal_to(&self, b: BigInteger, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.isEqualTo(generator.createConstantWire(b, desc), &None)
    }
}
impl IsEqualTo<i64> for WireType {
    fn is_equal_to(&self, v: i64, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.isEqualTo(generator.createConstantWirei(v, desc), &None)
    }
}
impl IsLessThanOrEqual for WireType {
    fn is_less_than_or_equal(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.createConstantWire(p, desc);
        let sum = pWire.addw(w, desc).subw(self.self_clone().unwrap(), desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        bitWires[bitwidth as usize].clone().unwrap()
    }
}
impl IsLessThanOrEqual<i64> for WireType {
    fn is_less_than_or_equal(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.isLessThanOrEqual(generator.createConstantWirei(v, desc), bitwidth, desc);
    }
}
impl IsLessThanOrEqual<BigInteger> for WireType {
    fn is_less_than_or_equal(
        &self,
        b: BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.isLessThanOrEqual(generator.createConstantWire(b, desc), bitwidth, desc);
    }
}
impl IsLessThan for WireType {
    fn is_less_than(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.createConstantWire(p, desc);
        let sum = pWire.addw(self.self_clone().unwrap(), desc).subw(w, desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires[bitwidth as usize]
            .as_ref()
            .unwrap()
            .invAsBit(desc)
            .unwrap();
    }
}
impl IsLessThan<i64> for WireType {
    fn is_less_than(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.isLessThan(generator.createConstantWirei(v, desc), bitwidth, desc);
    }
}
impl IsLessThan<BigInteger> for WireType {
    fn is_less_than(&self, b: BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.isLessThan(generator.createConstantWire(b, desc), bitwidth, desc);
    }
}
impl IsGreaterThanOrEqual for WireType {
    fn is_greater_than_or_equal(
        &self,
        w: WireType,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = generator.createConstantWire(p, desc);
        let sum = pWire.addw(self.self_clone().unwrap(), desc).subw(w, desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        bitWires[bitwidth as usize].clone().unwrap()
    }
}
impl IsGreaterThanOrEqual<i64> for WireType {
    fn is_greater_than_or_equal(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.isGreaterThanOrEqual(generator.createConstantWirei(v, desc), bitwidth, desc);
    }
}
impl IsGreaterThanOrEqual<BigInteger> for WireType {
    fn is_greater_than_or_equal(
        &self,
        b: BigInteger,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        return self.isGreaterThanOrEqual(generator.createConstantWire(b, desc), bitwidth, desc);
    }
}
impl IsGreaterThan for WireType {
    fn is_greater_than(&self, w: WireType, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = generator.createConstantWire(p, desc);
        let sum = pWire.addw(w, desc).subw(self.self_clone().unwrap(), desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires[bitwidth as usize]
            .clone()
            .unwrap()
            .invAsBit(desc)
            .unwrap();
    }
}
impl IsGreaterThan<i64> for WireType {
    fn is_greater_than(&self, v: i64, bitwidth: i32, desc: &Option<String>) -> WireType {
        let mut generator = self.generator();
        let mut generator = generator.lock();
        self.isGreaterThan(generator.createConstantWirei(v, desc), bitwidth, desc)
    }
}
impl IsGreaterThan<BigInteger> for WireType {
    fn is_greater_than(&self, b: BigInteger, bitwidth: i32, desc: &Option<String>) -> WireType {
        let generator = self.generator();
        let mut generator = generator.lock();
        self.isGreaterThan(generator.createConstantWire(b, desc), bitwidth, desc)
    }
}

impl Add for WireType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_wire(rhs, &None)
    }
}

impl Add<i64> for WireType {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        self.add_wire(rhs, &None)
    }
}

impl Add<BigInteger> for WireType {
    type Output = Self;

    fn add(self, rhs: BigInteger) -> Self::Output {
        self.add_wire(rhs, &None)
    }
}
impl Sub for WireType {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_wire(rhs, &None)
    }
}

impl Sub<i64> for WireType {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        self.sub_wire(rhs, &None)
    }
}
impl Sub<BigInteger> for WireType {
    type Output = Self;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        self.sub_wire(rhs, &None)
    }
}

impl Mul for WireType {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_wire(rhs, &None)
    }
}

impl BitAnd for WireType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and_bitwise(rhs, 32, &None)
    }
}

impl BitAnd<i64> for WireType {
    type Output = Self;

    fn bitand(self, rhs: i64) -> Self::Output {
        self.and_bitwise(rhs, 32, &None)
    }
}

impl BitAnd<BigInteger> for WireType {
    type Output = Self;

    fn bitand(self, rhs: BigInteger) -> Self::Output {
        self.and_bitwise(rhs, 32, &None)
    }
}
impl BitOr for WireType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or_bitwise(rhs, 32, &None)
    }
}

impl BitOr<i64> for WireType {
    type Output = Self;

    fn bitor(self, rhs: i64) -> Self::Output {
        self.or_bitwise(rhs, 32, &None)
    }
}
impl BitOr<BigInteger> for WireType {
    type Output = Self;

    fn bitor(self, rhs: BigInteger) -> Self::Output {
        self.or_bitwise(rhs, 32, &None)
    }
}

impl BitXor for WireType {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor_bitwise(rhs, 32, &None)
    }
}

impl BitXor<i64> for WireType {
    type Output = Self;

    fn bitxor(self, rhs: i64) -> Self::Output {
        self.xor_bitwise(rhs, 32, &None)
    }
}
impl BitXor<BigInteger> for WireType {
    type Output = Self;

    fn bitxor(self, rhs: BigInteger) -> Self::Output {
        self.xor_bitwise(rhs, 32, &None)
    }
}

impl std::fmt::Display for WireType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                _ => "",
            }
        )
    }
}

impl StructNameConfig for WireType {
    fn name(&self) -> String {
        String::new()
    }
}

impl InstanceOf for WireType {
    fn instance_of(&self, name: &str) -> bool {
        self.name() == name
    }
}
