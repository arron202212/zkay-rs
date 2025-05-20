#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire::Base;
 use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;
use crate::circuit::structure::linear_combination_bit_wire::LinearCombinationBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::constant_wire::ConstantWire;
use crate::circuit::structure::bit_wire::BitWire;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::eval::instruction::Instruction;
 use crate::circuit::structure::wire_array::WireArray;
use crate::util::util::BigInteger;
use std::ops::{Add,Sub,Mul};
use std::hash::{DefaultHasher, Hash, Hasher};
 use std::fmt::Debug;
 use std::fmt;
#[derive(Debug,Clone,Hash,PartialEq)]
pub enum WireType{
Wire(Base),
LinearCombinationBit(LinearCombinationBitWire),
LinearCombination(LinearCombinationWire),
Variable(VariableWire),
VariableBit(VariableBitWire),
Constant(ConstantWire),
Bit(BitWire),
}
impl Default for WireType{
fn default()->Self{
    Self::Wire(Base)
}}
impl  WireType{
    pub fn instance_of(&self,name:&str)->bool{
        self.name()==name
    }
    fn name(&self)->&str{
        ""
    }
 pub fn getConstant(&self,) -> BigInteger {
        return BigInteger::ZERO;
    }
    pub fn getBitWiresIfExistAlready(&self) -> Option<WireArray>  {
        return self.getBitWires();
    }
}
impl WireConfig for WireType{
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
                _=>""
            }
        )
    }
}


