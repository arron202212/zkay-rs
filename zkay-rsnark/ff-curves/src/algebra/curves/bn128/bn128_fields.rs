use ffec::field_utils::{
    bigint::{GMP_NUMB_BITS, bigint},
    field_utils::batch_invert,
};
use ffec::{
    Fp_model, Fp_modelConfig, Fp2_model, Fp2_modelConfig, Fp3_modelConfig, Fp6_3over2_model,
    Fp6_modelConfig, Fp12_2over3over2_model, Fp12_modelConfig, One, PpConfig, Zero,
};

use ffec::{BigInt, MontFp};
use std::borrow::Borrow;
use std::ops::{Add, Mul, Neg, Sub};

const bn128_r_bitcount: usize = 254;
const bn128_q_bitcount: usize = 254;

const bn128_r_limbs: usize = (bn128_r_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
const bn128_q_limbs: usize = (bn128_q_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
const bn128_q_limbs2: usize = bn128_q_limbs * 2;
const bn128_q_limbs3: usize = bn128_q_limbs * 3;
const bn128_q_limbs6: usize = bn128_q_limbs * 6;
const bn128_q_limbs12: usize = bn128_q_limbs * 12;
pub type bn128_Fr = Fp_model<bn128_r_limbs, Backend>;
pub type bn128_Fq = Fp_model<bn128_q_limbs, Backend>;
pub type bn128_Fq2 = Fp2_model<bn128_q_limbs, bn128_q_limbs2, Backend>;
pub type bn128_Fq6 = Fp6_3over2_model<bn128_q_limbs, bn128_q_limbs2, bn128_q_limbs6, Backend>;
pub type bn128_Fq12 =
    Fp12_2over3over2_model<bn128_q_limbs, bn128_q_limbs2, bn128_q_limbs6, bn128_q_limbs12, Backend>;

pub type Fp = bn128_Fq;
pub type Fp2 = bn128_Fq2;
pub type Fp6 = bn128_Fq6;
pub type Fp12 = bn128_Fq12;

#[derive(Default, Clone, Debug, Copy, PartialEq, Eq)]
pub struct Backend;

impl<O: Borrow<Self>> Add<O> for Backend {
    type Output = Backend;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for Backend {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for Backend {
    type Output = Backend;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for Backend {
    type Output = Backend;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<O: Borrow<Self>> Mul<O> for Backend {
    type Output = Backend;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for Backend {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for Backend {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for Backend {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

impl PpConfig for Backend {
    type BigIntT = bigint<1>;
}
impl Fp_modelConfig<bn128_q_limbs> for Backend {}
impl Fp2_modelConfig<bn128_q_limbs, bn128_q_limbs2> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp3_modelConfig<bn128_q_limbs, bn128_q_limbs3> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp6_modelConfig<bn128_q_limbs, bn128_q_limbs2, bn128_q_limbs6> for Backend {
    type Fp_modelConfig = Self;
    type Fp2_modelConfig = Self;
}
impl Fp12_modelConfig<bn128_q_limbs, bn128_q_limbs2, bn128_q_limbs6, bn128_q_limbs12> for Backend {
    type Fp_modelConfig = Self;
    type Fp6_modelConfig = Self;
}

pub fn init_bn128_fields() {}

// use ark_ff::{BigInteger, MontConfig, field_new};

// pub struct FrConfig;
// impl MontConfig<4> for FrConfig {

//     const MODULUS: BigInt<4> = field_new!(
//         Fr,
//         "21888242871839275222246405745257275088548364400416034343698204186575808495617"
//     );

//     const INVERSE: u64 = 0xc2e1f593efffffff;

//     const TWO_ADICITY: u32 = 28;
//     const GENERATOR: BigInt<4> = field_new!(Fr, "5");

//     const ROOT_OF_UNITY: BigInt<4> = field_new!(
//         Fr,
//         "19103219067921713944291392827692070036145651957329286315305642004821462161904"
//     );
// }

// pub struct FqConfig;
// impl MontConfig<4> for FqConfig {

//     const MODULUS: BigInt<4> = field_new!(
//         Fq,
//         "21888242871839275222246405745257275088696311157297823662689037894645226208583"
//     );

//     const INVERSE: u64 = 0x87d20782e4866389;

//     const TWO_ADICITY: u32 = 1;
//     const TRACE_MINUS_ONE_DIV_TWO: BigInt<4> = field_new!(
//         Fq,
//         "5472060717959818805561601436314318772174077789324455915672259473661306552145"
//     );
// }
