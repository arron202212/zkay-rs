#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// # modification of sapling_jubjub.py from https://github.com/zcash-hackworks/zcash-test-vectors
// # changed JubJub parameters to BabyJubJub parameters
// # (https://iden3-docs.readthedocs.io/en/latest/iden3_repos/research/publications/zkproof-standards-workshop-2/baby-jubjub/baby-jubjub.html)
// """
// The MIT License (MIT)

// Copyright (c) 2018-2019 Electric Coin Company

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
// """

const BASE_ORDER: &str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";

pub const CURVE_ORDER: &str =
    "2736030358979909402780800718157159386076813972158567259200215660948447373041";
use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, Num, One, Zero};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};
// class Fp<T> (object):
//     def __init__(self, t, s, modulus, strict=False):
//         if strict and not (0 <= s and s < modulus):
//             raise ValueError
//         self.t = t
//         self.s = s % modulus
//         self.m = modulus

//     def __neg__(self):
//         return self.t(-self.s)

//     def __add__(self, a):
//         return self.t(self.s + a.s)

//     def __sub__(self, a):
//         return self.t(self.s - a.s)

//     def __mul__(self, a):
//         return self.t(self.s * a.s)

//     def __truediv__(self, a):
//         assert a.s != 0
//         return self * a.inv()

//     def exp(self, e):
//         e = format(e, '0256b')
//         ret = self.t(1)
//         for c in e:
//             ret = ret * ret
//             if int(c):
//                 ret = ret * self
//         return ret

//     def inv(self):
//         return self.exp(self.m - 2)

//     def __eq__(self, a):
//         return self.s == a.s

pub trait FieldElement:
    Sized
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + Eq
    + Debug
{
    const M: &str;
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn squared(&self) -> Self {
        self.clone() * self.clone()
    }
    fn inv(self) -> Option<Self> {
        Some(self.exp(BigUint::parse_bytes(Self::M.as_bytes(), 10).unwrap() - 2u8))
    }
    fn exp(&self, e: BigUint) -> Self {
        let mut res = Self::one();

        for i in 0..e.bits() {
            res = res.squared();
            if e.bit(i) {
                res = self.clone() * res;
            }
        }

        res
    }
}
pub trait ModulusConstant: Sized + Clone + PartialEq + Eq + Debug {
    const M: &str;
    const NAME: &str;
}
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct FqConfig;
impl ModulusConstant for FqConfig {
    const M: &str = BASE_ORDER;
    const NAME: &str = "Fq";
}
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct FrConfig;
impl ModulusConstant for FrConfig {
    const M: &str = CURVE_ORDER;
    const NAME: &str = "Fr";
}

#[derive(Eq, Debug, Clone)]
pub struct Fp<T: ModulusConstant> {
    pub s: BigUint,
    m: BigUint,
    _t: PhantomData<T>,
}
impl<T: ModulusConstant> Fp<T> {
    pub fn new(s: &str) -> Self {
        Self {
            s: BigUint::parse_bytes(s.as_bytes(), 10).unwrap(),
            m: BigUint::parse_bytes(Self::M.as_bytes(), 10).unwrap(),
            _t: PhantomData,
        }
    }
    pub fn to_the_power_of(&self, exponent: BigUint) -> Self {
        let exp = exponent % (self.m.clone() - BigUint::one());
        let new_num = Self::mod_pow(&self.s, exp, &self.m);
        Self {
            s: new_num,
            m: self.m.clone(),
            _t: PhantomData,
        }
    }

    pub fn mod_pow(base: &BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
        if modulus == &BigUint::one() {
            return BigUint::zero();
        }

        let mut result = BigUint::one();
        let mut base = base % modulus;
        while exp > BigUint::zero() {
            if exp.clone() % (BigUint::one() + BigUint::one()) == BigUint::one() {
                result = result * base.clone() % modulus;
            }
            exp = exp / (BigUint::one() + BigUint::one());
            base = base.clone() * base.clone() % modulus;
        }

        result
    }
    fn modulo(&self, b: &BigUint) -> BigUint {
        b % self.m.clone()
    }
}
impl<T: ModulusConstant> FieldElement for Fp<T> {
    const M: &str = T::M;
    fn zero() -> Self {
        Self {
            s: BigUint::zero(),
            m: BigUint::parse_bytes(Self::M.as_bytes(), 10).unwrap(),
            _t: PhantomData,
        }
    }
    fn one() -> Self {
        Self {
            s: BigUint::one(),
            m: BigUint::parse_bytes(Self::M.as_bytes(), 10).unwrap(),
            _t: PhantomData,
        }
    }
    fn is_zero(&self) -> bool {
        self.s.is_zero()
    }
}
impl<T: ModulusConstant> PartialEq for Fp<T> {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl<T: ModulusConstant> Neg for Fp<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            s: self.m.clone() - self.s.clone(),
            m: BigUint::parse_bytes(Self::M.as_bytes(), 10).unwrap(),
            _t: PhantomData,
        }
    }
}

impl<T: ModulusConstant> Add for Fp<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let s = self.modulo(&(self.s.clone() + rhs.s));
        Self {
            s,
            m: self.m.clone(),
            _t: PhantomData,
        }
    }
}

impl<T: ModulusConstant> Sub for Fp<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let difference = BigInt::from(self.s.clone()) - BigInt::from(rhs.s.clone());
        let big_prime = BigInt::from(self.m.clone());
        let remainder = difference % big_prime.clone();
        let new_num = remainder.clone()
            + if remainder < BigInt::zero() {
                big_prime
            } else {
                BigInt::zero()
            };
        Self {
            s: new_num.try_into().unwrap(),
            m: self.m.clone(),
            _t: PhantomData,
        }
    }
}

impl<T: ModulusConstant> Mul for Fp<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let s = self.modulo(&(self.s.clone() * rhs.s));
        Self {
            s,
            m: self.m.clone(),
            _t: PhantomData,
        }
    }
}

impl<T: ModulusConstant> Div for Fp<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // use Fermat's little theorem
        // self.num.pow(p-1) % p == 1
        // this means:
        // 1/n == pow(n, p-2, p) in Python
        let exp = rhs.m.clone() - (BigUint::one() + BigUint::one());
        let num_pow = rhs.to_the_power_of(exp);
        let result = self.s.clone() * num_pow.s;
        Self {
            s: result % self.m.clone(),
            m: self.m.clone(),
            _t: PhantomData,
        }
    }
}
use std::fmt;
impl<T: ModulusConstant> fmt::Display for Fp<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", T::NAME, self.s)
    }
}
pub type Fq = Fp<FqConfig>;
pub type Fr = Fp<FrConfig>;
// class Fq(Fp<T> ):

//     def __init__(self, s, strict=False):
//         Fp<T> .__init__(self, Fq, s, BASE_ORDER, strict=strict)

//     def __str__(self):
//         return 'Fq(%s)' % self.s

// class Fr(Fp<T> ):
//     def __init__(self, s, strict=False):
//         Fp<T> .__init__(self, Fr, s, CURVE_ORDER, strict=strict)

//     def __str__(self):
//         return 'Fr(%s)' % self.s

// Fq.ZERO = Fq(0)
// Fq.ONE = Fq(1)
// Fq.MINUS_ONE = Fq(-1)

// assert Fq.ZERO + Fq.ZERO == Fq.ZERO
// assert Fq.ZERO + Fq.ONE == Fq.ONE
// assert Fq.ONE + Fq.ZERO == Fq.ONE
// assert Fq.ZERO - Fq.ONE == Fq.MINUS_ONE
// assert Fq.ZERO * Fq.ONE == Fq.ZERO
// assert Fq.ONE * Fq.ZERO == Fq.ZERO

// #
// # Point arithmetic
// #

// BABYJUBJUB_A = Fq(1)
const BABYJUBJUB_D: &str =
    "9706598848417545097372247223557719406784115219466060233080913168975159366771";

// # an arbitrary generator
const BABYJUBJUB_GENERATOR_X: &str =
    "11904062828411472290643689191857696496057424932476499415469791423656658550213";
const BABYJUBJUB_GENERATOR_Y: &str =
    "9356450144216313082194365820021861619676443907964402770398322487858544118183";
// #[macro_export]
// macro_rules! fq {
//         ($val: expr) => {
//             Fq::new($val)
//         };
// }

// class Point(object):
//     def __init__(self, u, v):
//         self.u = u
//         self.v = v

//     def __add__(self, a):
//         (u1, v1) = (self.u, self.v)
//         (u2, v2) = (a.u, a.v)
//         u3 = (u1*v2 + v1*u2) / (Fq.ONE + BABYJUBJUB_D * u1 * u2 * v1 * v2)
//         v3 = (v1 * v2 - BABYJUBJUB_A * u1 * u2) / (Fq.ONE - BABYJUBJUB_D * u1 * u2 * v1 * v2)
//         return Point(u3, v3)

//     def double(self):
//         return self + self

//     def negate(self):
//         return Point(-self.u, self.v)

//     def __mul__(self, s):
//         s = format(s.s, '0256b')
//         ret = self.ZERO
//         for c in s:
//             ret = ret.double()
//             if int(c):
//                 ret = ret + self
//         return ret

//     def __eq__(self, a):
//         return self.u == a.u and self.v == a.v

//     def __str__(self):
//         return 'Point(%s, %s)' % (self.u, self.v)

// Point.ZERO = Point(Fq.ZERO, Fq.ONE)
// Point.GENERATOR = Point(Fq(BABYJUBJUB_GENERATOR_X), Fq(BABYJUBJUB_GENERATOR_Y))

// assert Point.ZERO + Point.ZERO == Point.ZERO

#[derive(Eq, Debug, Clone)]
pub struct Point {
    pub u: Fq,
    pub v: Fq,
}
impl Point {
    pub fn new(u: Fq, v: Fq) -> Self {
        Self { u, v }
    }
    pub fn double(self) -> Self {
        self.clone() + self
    }
    pub fn negate(self) -> Self {
        Self::new(-self.u, self.v)
    }
    pub fn zero() -> Self {
        Self::new(Fq::zero(), Fq::one())
    }
    pub fn generator() -> Self {
        Self::new(
            Fq::new(BABYJUBJUB_GENERATOR_X),
            Fq::new(BABYJUBJUB_GENERATOR_Y),
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (u1, v1) = (self.u.clone(), self.v.clone());
        let (u2, v2) = (rhs.u.clone(), rhs.v.clone());
        let u3 = (u1.clone() * v2.clone() + v1.clone() * u2.clone())
            / (Fq::one()
                + Fq::new(BABYJUBJUB_D) * u1.clone() * u2.clone() * v1.clone() * v2.clone());
        let v3 = (v1.clone() * v2.clone() - Fq::one() * u1.clone() * u2.clone())
            / (Fq::one() - Fq::new(BABYJUBJUB_D) * u1 * u2 * v1 * v2);
        Self::new(u3, v3)
    }
}
impl Mul<Fr> for Point {
    type Output = Self;
    fn mul(self, rhs: Fr) -> Self::Output {
        let s = rhs.s.bits();
        let mut ret = Self::zero();
        for i in 0..s {
            ret = ret.double();
            if rhs.s.bit(i) {
                ret = ret + self.clone();
            }
        }
        ret
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({},{})", self.u, self.v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fq() {
        let minus_one = -Fq::one();
        assert_eq!(Fq::zero() + Fq::zero(), Fq::zero());
        assert_eq!(Fq::zero() + Fq::one(), Fq::one());
        assert_eq!(Fq::one() + Fq::zero(), Fq::one());
        assert_eq!(Fq::zero() - Fq::one(), minus_one);
        assert_eq!(Fq::zero() * Fq::one(), Fq::zero());
        assert_eq!(Fq::one() * Fq::zero(), Fq::zero());
    }

    #[test]
    fn test_point() {
        assert_eq!(Point::zero() + Point::zero(), Point::zero());
    }
}
