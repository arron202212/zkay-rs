use crate::algebra::curves::bn128::bn128_fields::{Fp, Fp2, Fp6, Fp12};
use ffec::field_utils::{BigInt, bigint::bigint};
use ffec::{BigInt, Fp_model, Fp_modelConfig, One, PpConfig, Zero};
use num_bigint::BigUint;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct bn128_GT {
    pub elem: Fp12,
}
impl PpConfig for bn128_GT {
    type TT = bigint<1>;
    // type Fr=Self;
}
impl Default for bn128_GT {
    fn default() -> Self {
        Self {
            elem: Fp12::default(),
        }
    }
}
impl From<Fp12> for bn128_GT {
    fn from(elem:Fp12) -> Self {
        Self {
            elem
        }
    }
}
impl bn128_GT {
    pub fn unitary_inverse(&self) -> bn128_GT {
        let result = self.clone();
        // result.elem.b_=-result.elem.b_.clone();
        result
    }
    pub fn GT_zero() -> bn128_GT {
        Self {
            elem: Fp12::default(),
        }
    }
    pub fn GT_one() -> bn128_GT {
        Self {
            elem: Fp12::default(),
        }
    }
    pub fn one() -> bn128_GT {
        return Self::GT_one();
    }
}

impl Add<i32> for bn128_GT {
    type Output = bn128_GT;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for bn128_GT {
    type Output = bn128_GT;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for bn128_GT {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for bn128_GT {
    type Output = bn128_GT;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for bn128_GT {
    type Output = bn128_GT;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Mul<i32> for bn128_GT {
    type Output = bn128_GT;

    fn mul(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<O: Borrow<Self>> Mul<O> for bn128_GT {
    type Output = bn128_GT;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for bn128_GT {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for bn128_GT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}
impl One for bn128_GT {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for bn128_GT {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
// use std::io::{self, Read, Write};
// use std::ops::{Mul, MulAssign};

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Bn128GT {
//     pub elem: Fp12,
// }

// impl Bn128GT {
//     pub fn one() -> Self {
//         Self { elem: Fp12::one() }
//     }

//     pub fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self {
//         Self {
//             elem: self.elem.pow(exp),
//         }
//     }
// }

// impl<'a> Mul<&'a Bn128GT> for &'a Bn128GT {
//     type Output = Bn128GT;
//     fn mul(self, other: &'a Bn128GT) -> Bn128GT {
//         Bn128GT {
//             elem: self.elem * &other.elem,
//         }
//     }
// }

// impl<'a> Mul<&'a BigInt> for &'a Bn128GT {
//     type Output = Bn128GT;
//     fn mul(self, rhs: &'a BigInt) -> Bn128GT {
//         self.pow(rhs.to_u64_slice())
//     }
// }

// impl Bn128GT {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         writer.write_all(&self.elem.a.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.elem.b.to_bytes())?;
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
//         let a = Fp6::read(&mut reader)?;

//         let b = Fp6::read(&mut reader)?;
//         Ok(Self {
//             elem: Fp12 { a, b },
//         })
//     }
// }
