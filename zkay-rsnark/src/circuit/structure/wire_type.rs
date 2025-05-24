#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use super::wire_ops::{
    Add as AddWire, AndBitwise, IsEqualTo, IsGreaterThan, IsGreaterThanOrEqual, IsLessThan,
    IsLessThanOrEqual, Mul as MulWire, OrBitwise, Sub as SubWire, XorBitwise,
};
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::non_zero_check_basic_op::{
    NonZeroCheckBasicOp, new_non_zero_check,
};
use crate::circuit::operations::primitive::or_basic_op::{ORBasicOp, new_or};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::operations::primitive::split_basic_op::{SplitBasicOp, new_split};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::structure::bit_wire::BitWire;
use crate::circuit::structure::constant_wire::ConstantWire;
use crate::circuit::structure::linear_combination_bit_wire::LinearCombinationBitWire;
use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::wire::{Base, Wire};
use crate::circuit::structure::wire::{WireConfig, setBitsConfig};
use crate::circuit::structure::wire_array::WireArray;
use crate::util::util::BigInteger;
use crate::util::util::Util;
use std::fmt;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum WireType {
    Wire(Wire<Base>),
    LinearCombinationBit(LinearCombinationBitWire),
    LinearCombination(LinearCombinationWire),
    Variable(VariableWire),
    VariableBit(VariableBitWire),
    Constant(ConstantWire),
    Bit(BitWire),
}
impl Default for WireType {
    fn default() -> Self {
        Self::Wire(Wire::<Base>::new(-1, Base).unwrap())
    }
}
impl WireType {
    pub fn instance_of(&self, name: &str) -> bool {
        self.name() == name
    }
    fn name(&self) -> &str {
        ""
    }

    pub fn getConstant(&self) -> BigInteger {
        return BigInteger::ZERO;
    }
    pub fn isBinary(&self) -> bool {
        false
    }
    pub fn invAsBit(&self, desc: &String) -> Option<WireType> {
        None
    }
    pub fn getBitWiresIfExistAlready(&self) -> Option<WireArray> {
        return self.getBitWires();
    }
    // pub fn packIfNeeded(&self, desc: &String) {
    //     // if self.wireId == -1 {
    //     //     self.pack();
    //     // }
    // }
}
impl setBitsConfig for WireType {}
impl WireConfig for WireType {}
impl MulWire<BigInteger> for WireType {
    fn mul(self, b: BigInteger, desc: &String) -> Self {
        self.packIfNeeded(desc);
        if b == Util::one() {
            return self.self_clone().unwrap();
        }
        if b == BigInteger::ZERO {
            return self.generator().zeroWire.borrow().clone().unwrap();
        }
        let out = WireType::LinearCombination(LinearCombinationWire::new(
            *self.generator().currentWireId.borrow_mut(),
        ));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_const_mul(self.self_clone().unwrap(), out.clone(), b, desc.clone());
        //		self.generator().addToEvaluationQueue(Box::new(op));
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        out
    }
}
impl MulWire<i64> for WireType {
    fn mul(self, l: i64, desc: &String) -> WireType {
        return self.mulb(BigInteger::from(l), desc);
    }
}
impl MulWire<(i64, i32)> for WireType {
    fn mul(self, (base, exp): (i64, i32), desc: &String) -> WireType {
        let mut b = BigInteger::from(base);
        b = b.pow(exp as u32);
        return self.mulb(b, desc);
    }
}
impl MulWire for WireType {
    fn mul(self, w: Self, desc: &String) -> WireType {
        if w.instance_of("ConstantWire") {
            return self.mulb(w.getConstant(), desc);
        }
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let output = WireType::Variable(VariableWire::new(
            *self.generator().currentWireId.borrow_mut(),
        ));
        *self.generator().currentWireId.borrow_mut() += 1;
        let op = new_mul(self.self_clone().unwrap(), w, output.clone(), desc.clone());
        let cachedOutputs = self.generator().addToEvaluationQueue(Box::new(op));
        if let Some(cachedOutputs) = cachedOutputs {
            *self.generator().currentWireId.borrow_mut() -= 1;
            return cachedOutputs[0].clone().unwrap();
        }
        output
    }
}

impl AddWire for WireType {
    fn add(self, w: WireType, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        return WireArray::new(vec![Some(self.self_clone().unwrap()), Some(w)])
            .sumAllElements(desc);
    }
}
impl AddWire<i64> for WireType {
    fn add(self, v: i64, desc: &String) -> WireType {
        return self.addw(self.generator().createConstantWirei(v, desc), desc);
    }
}
impl AddWire<BigInteger> for WireType {
    fn add(self, b: BigInteger, desc: &String) -> WireType {
        return self.addw(self.generator().createConstantWire(b, desc), desc);
    }
}
impl SubWire for WireType {
    fn sub(self, w: WireType, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let neg = w.muli(-1, desc);
        return self.addw(neg, desc);
    }
}
impl SubWire<i64> for WireType {
    fn sub(self, v: i64, desc: &String) -> WireType {
        return self.subw(self.generator().createConstantWirei(v, desc), desc);
    }
}
impl SubWire<BigInteger> for WireType {
    fn sub(self, b: BigInteger, desc: &String) -> WireType {
        return self.subw(self.generator().createConstantWire(b, desc), desc);
    }
}

impl XorBitwise for WireType {
    fn xor_bitwise(self, w: WireType, numBits: i32, desc: &String) -> WireType {
        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.xorWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator().createConstantWire(v, &String::new());
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }
}

impl XorBitwise<i64> for WireType {
    fn xor_bitwise(self, v: i64, numBits: i32, desc: &String) -> WireType {
        return self.xorBitwise(
            self.generator().createConstantWirei(v, desc),
            numBits,
            desc,
        );
    }
}

impl XorBitwise<BigInteger> for WireType {
    fn xor_bitwise(self, b: BigInteger, numBits: i32, desc: &String) -> WireType {
        return self.xorBitwise(
            self.generator().createConstantWire(b, desc),
            numBits,
            desc,
        );
    }
}

impl AndBitwise for WireType {
    fn and_bitwise(self, w: WireType, numBits: i32, desc: &String) -> WireType {
        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.andWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator().createConstantWire(v, &String::new());
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }
}

impl AndBitwise<i64> for WireType {
    fn and_bitwise(self, v: i64, numBits: i32, desc: &String) -> WireType {
        return self.andBitwise(
            self.generator().createConstantWirei(v, desc),
            numBits,
            desc,
        );
    }
}

impl AndBitwise<BigInteger> for WireType {
    fn and_bitwise(self, b: BigInteger, numBits: i32, desc: &String) -> WireType {
        return self.andBitwise(
            self.generator().createConstantWire(b, desc),
            numBits,
            desc,
        );
    }
}

impl OrBitwise for WireType {
    fn or_bitwise(self, w: WireType, numBits: i32, desc: &String) -> WireType {
        let bits1 = self.getBitWiresi(numBits as u64, desc);
        let bits2 = w.getBitWiresi(numBits as u64, desc);
        let result = bits1.orWireArray(bits2, numBits as usize, desc);
        let v = result.checkIfConstantBits(desc);
        if let Some(v) = v {
            return self.generator().createConstantWire(v, &String::new());
        }
        WireType::LinearCombination(LinearCombinationWire::newa(result))
    }
}
impl OrBitwise<i64> for WireType {
    fn or_bitwise(self, v: i64, numBits: i32, desc: &String) -> WireType {
        return self.orBitwise(
            self.generator().createConstantWirei(v, desc),
            numBits,
            desc,
        );
    }
}
impl OrBitwise<BigInteger> for WireType {
    fn or_bitwise(self, b: BigInteger, numBits: i32, desc: &String) -> WireType {
        return self.orBitwise(
            self.generator().createConstantWire(b, desc),
            numBits,
            desc,
        );
    }
}
impl IsEqualTo for WireType {
    fn is_equal_to(&self, w: WireType, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let s = self.subw(w, desc);
        return s.checkNonZero(desc).invAsBit(desc).unwrap();
    }
}
impl IsEqualTo<BigInteger> for WireType {
    fn is_equal_to(&self, b: BigInteger, desc: &String) -> WireType {
        return self.isEqualTo(self.generator().createConstantWire(b, desc), &String::new());
    }
}
impl IsEqualTo<i64> for WireType {
    fn is_equal_to(&self, v: i64, desc: &String) -> WireType {
        return self.isEqualTo(self.generator().createConstantWirei(v, desc), &String::new());
    }
}
impl IsLessThanOrEqual for WireType {
    fn is_less_than_or_equal(&self, w: WireType, bitwidth: i32, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc);
        let sum = pWire.addw(w, desc).subw(self.self_clone().unwrap(), desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires.get(bitwidth as usize);
    }
}
impl IsLessThanOrEqual<i64> for WireType {
    fn is_less_than_or_equal(&self, v: i64, bitwidth: i32, desc: &String) -> WireType {
        return self.isLessThanOrEqual(
            self.generator().createConstantWirei(v, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsLessThanOrEqual<BigInteger> for WireType {
    fn is_less_than_or_equal(&self, b: BigInteger, bitwidth: i32, desc: &String) -> WireType {
        return self.isLessThanOrEqual(
            self.generator().createConstantWire(b, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsLessThan for WireType {
    fn is_less_than(&self, w: WireType, bitwidth: i32, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc);
        let sum = pWire.addw(self.self_clone().unwrap(), desc).subw(w, desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires
            .get(bitwidth as usize)
            .invAsBit(desc)
            .unwrap();
    }
}
impl IsLessThan<i64> for WireType {
    fn is_less_than(&self, v: i64, bitwidth: i32, desc: &String) -> WireType {
        return self.isLessThan(
            self.generator().createConstantWirei(v, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsLessThan<BigInteger> for WireType {
    fn is_less_than(&self, b: BigInteger, bitwidth: i32, desc: &String) -> WireType {
        return self.isLessThan(
            self.generator().createConstantWire(b, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsGreaterThanOrEqual for WireType {
    fn is_greater_than_or_equal(&self, w: WireType, bitwidth: i32, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2u8).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc);
        let sum = pWire.addw(self.self_clone().unwrap(), desc).subw(w, desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires.get(bitwidth as usize);
    }
}
impl IsGreaterThanOrEqual<i64> for WireType {
    fn is_greater_than_or_equal(&self, v: i64, bitwidth: i32, desc: &String) -> WireType {
        return self.isGreaterThanOrEqual(
            self.generator().createConstantWirei(v, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsGreaterThanOrEqual<BigInteger> for WireType {
    fn is_greater_than_or_equal(&self, b: BigInteger, bitwidth: i32, desc: &String) -> WireType {
        return self.isGreaterThanOrEqual(
            self.generator().createConstantWire(b, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsGreaterThan for WireType {
    fn is_greater_than(&self, w: WireType, bitwidth: i32, desc: &String) -> WireType {
        self.packIfNeeded(desc);
        w.packIfNeeded(desc);
        let p = BigInteger::from(2).pow(bitwidth as u32);
        let pWire = self.generator().createConstantWire(p, desc);
        let sum = pWire.addw(w, desc).subw(self.self_clone().unwrap(), desc);
        let bitWires = sum.getBitWiresi(bitwidth as u64 + 1, desc);
        return bitWires
            .get(bitwidth as usize)
            .invAsBit(desc)
            .unwrap();
    }
}
impl IsGreaterThan<i64> for WireType {
    fn is_greater_than(&self, v: i64, bitwidth: i32, desc: &String) -> WireType {
        return self.isGreaterThan(
            self.generator().createConstantWirei(v, desc),
            bitwidth,
            desc,
        );
    }
}
impl IsGreaterThan<BigInteger> for WireType {
    fn is_greater_than(&self, b: BigInteger, bitwidth: i32, desc: &String) -> WireType {
        return self.isGreaterThan(
            self.generator().createConstantWire(b, desc),
            bitwidth,
            desc,
        );
    }
}

impl Add for WireType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self
    }
}

impl Add<u64> for WireType {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self
    }
}

impl Add<BigInteger> for WireType {
    type Output = Self;

    fn add(self, rhs: BigInteger) -> Self::Output {
        self
    }
}
impl Sub for WireType {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self
    }
}

impl Sub<u64> for WireType {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self
    }
}
impl Sub<BigInteger> for WireType {
    type Output = Self;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        self
    }
}

impl Mul for WireType {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self
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
