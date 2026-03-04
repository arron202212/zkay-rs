//  Declaration of interfaces for initializing MNT6.

use crate::algebra::curves::mnt::mnt46_common::{
    mnt46_A_bitcount, mnt46_A_limbs, mnt46_B_bitcount, mnt46_B_limbs,
};
use ffec::field_utils::{
    bigint::{GMP_NUMB_BITS, bigint},
    field_utils::batch_invert,
};
use ffec::{
    Fp_model, Fp_modelConfig, Fp2_model, Fp2_modelConfig, Fp3_model, Fp3_modelConfig,
    Fp6_3over2_model, Fp6_modelConfig, Fp12_2over3over2_model, Fp12_modelConfig, One, PpConfig,
    Zero, fp6_2over3::Fp6_2over3_model,
};

use std::borrow::Borrow;
use std::ops::{Add, Mul, Neg, Sub};

pub const mnt6_r_bitcount: usize = mnt46_B_bitcount;
pub const mnt6_q_bitcount: usize = mnt46_A_bitcount;

pub const mnt6_r_limbs: usize = mnt46_B_limbs;
pub const mnt6_q_limbs: usize = mnt46_A_limbs;

pub type mnt6_Fr = Fp_model<mnt6_r_limbs, Backend>;
pub type mnt6_Fq = Fp_model<mnt6_q_limbs, Backend>;
pub type mnt6_Fq3 = Fp3_model<mnt6_q_limbs, Backend>;
pub type mnt6_Fq6 = Fp6_2over3_model<mnt6_q_limbs, Backend>;
pub type mnt6_GT = mnt6_Fq6;

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
    type TT = bigint<mnt6_q_limbs>;
    // type Fr=Self;
}
impl Fp_modelConfig<mnt6_q_limbs> for Backend {}
impl Fp2_modelConfig<mnt6_q_limbs> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp3_modelConfig<mnt6_q_limbs> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp6_modelConfig<mnt6_q_limbs> for Backend {
    type Fp_modelConfig = Self;
    type Fp2_modelConfig = Self;
}
impl ffec::fp6_2over3::Fp6_modelConfig<mnt6_q_limbs> for Backend {
    type Fp_modelConfig = Self;
    type Fp2_modelConfig = Self;
    type Fp3_modelConfig = Self;
}
impl Fp12_modelConfig<mnt6_q_limbs> for Backend {
    type Fp_modelConfig = Self;
    type Fp6_modelConfig = Self;
}

pub fn init_mnt6_fields() {}

// use ark_ff::{BigInteger, MontConfig, field_new, Fp3, Fp3Config, Fp6, Fp6Config};

// pub struct FrConfig;
// impl MontConfig<5> for FrConfig {
//     const MODULUS: BigInt<5> = field_new!(Fr, "475922286169261325753349249653048451545124879242694725395555128576210262817955800483758081");
//     const INVERSE: u64 = 0xb071a1b67165ffff;

//     const TWO_ADICITY: u32 = 17;
//     const GENERATOR: BigInt<5> = field_new!(Fr, "17");
// }

// pub struct FqConfig;
// impl MontConfig<5> for FqConfig {
//     const MODULUS: BigInt<5> = field_new!(Fq, "475922286169261325753349249653048451545124878552823515553267735739164647307408490559963137");
//     const INVERSE: u64 = 0xbb4334a3ffffffff;

//     const TWO_ADICITY: u32 = 34;
// }

// pub struct Fq3Config;
// impl Fp3Config for Fq3Config {
//     type Fp = Fq;
//     const NONRESIDUE: Fq = field_new!(Fq, "5");

//     const FROBENIUS_COEFF_FP3_C1: [Fq; 3] = [
//         field_new!(Fq, "1"),
//         field_new!(Fq, "471738898967521029133040851318449165997304108729558973770077319830005517129946578866686956"),
//         field_new!(Fq, "4183387201740296620308398334599285547820769823264541783190415909159130177461911693276180"),
//     ];
// }

// pub type Fq6 = Fp6<Fq6Config>;
// pub struct Fq6Config;
// impl Fp6Config for Fq6Config {
//     type Fp3Config = Fq3Config;
//     const NONRESIDUE: Fq3 = Fq3::new(field_new!(Fq, "2"), Fq::ZERO, Fq::ZERO);
// }
