//  Declaration of interfaces for initializing MNT4.

use crate::algebra::curves::mnt::{
    mnt4::mnt4_init::{mnt4_twist_mul_by_a_c0, mnt4_twist_mul_by_a_c1},
    mnt46_common::{mnt46_A_bitcount, mnt46_A_limbs, mnt46_B_bitcount, mnt46_B_limbs},
};

use ffec::{
    field_utils::{
        bigint::{GMP_NUMB_BITS, bigint},
        field_utils::batch_invert,
    },
    {
        Fp_model, Fp_modelConfig, Fp2_model, Fp2_modelConfig, Fp3_modelConfig, Fp4_model,
        Fp4_modelConfig, Fp6_3over2_model, Fp6_modelConfig, Fp12_2over3over2_model,
        Fp12_modelConfig, One, PpConfig, Zero,
    },
};

use std::{
    borrow::Borrow,
    ops::{Add, Mul, Neg, Sub},
};

pub const mnt4_r_bitcount: usize = mnt46_A_bitcount;
pub const mnt4_q_bitcount: usize = mnt46_B_bitcount;

pub const mnt4_r_limbs: usize = mnt46_A_limbs;
pub const mnt4_q_limbs: usize = mnt46_B_limbs;
const mnt4_q_limbs2: usize = mnt4_q_limbs * 2;
const mnt4_q_limbs3: usize = mnt4_q_limbs * 3;
const mnt4_q_limbs4: usize = mnt4_q_limbs * 4;
const mnt4_q_limbs6: usize = mnt4_q_limbs * 6;
const mnt4_q_limbs12: usize = mnt4_q_limbs * 12;
pub type mnt4_Fr = Fp_model<mnt4_r_limbs, Backend>;
pub type mnt4_Fq = Fp_model<mnt4_q_limbs, Backend>;
pub type mnt4_Fq2 = Fp2_model<mnt4_q_limbs, mnt4_q_limbs2, Backend>;
pub type mnt4_Fq4 = Fp4_model<mnt4_q_limbs, mnt4_q_limbs2, mnt4_q_limbs4, Backend>;
pub type mnt4_GT = mnt4_Fq4;

pub type mp_limb_t = u64;
pub type bigint_r = bigint<mnt4_r_limbs>;
pub type bigint_q = bigint<mnt4_q_limbs>;

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
impl Fp_modelConfig<mnt4_q_limbs> for Backend {}
impl Fp2_modelConfig<mnt4_q_limbs, mnt4_q_limbs2> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp3_modelConfig<mnt4_q_limbs, mnt4_q_limbs3> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp4_modelConfig<mnt4_q_limbs, mnt4_q_limbs2, mnt4_q_limbs4> for Backend {
    type Fp2_modelConfig = Self;
}
impl Fp6_modelConfig<mnt4_q_limbs, mnt4_q_limbs2, mnt4_q_limbs6> for Backend {
    type Fp_modelConfig = Self;
    type Fp2_modelConfig = Self;
}
impl Fp12_modelConfig<mnt4_q_limbs, mnt4_q_limbs2, mnt4_q_limbs6, mnt4_q_limbs12> for Backend {
    type Fp_modelConfig = Self;
    type Fp6_modelConfig = Self;
}

pub fn init_mnt4_fields() {}

// use ark_ff::{BigInteger, MontConfig, field_new, Fp2, Fp2Config, Fp4, Fp4Config};

// pub struct FrConfig;
// impl MontConfig<5> for FrConfig {

//     const MODULUS: BigInt<5> = field_new!(Fr, "475922286169261325753349249653048451545124878552823515553267735739164647307408490559963137");

//     const INVERSE: u64 = 0xbb4334a3ffffffff;

//     const TWO_ADICITY: u32 = 34;
//     const GENERATOR: BigInt<5> = field_new!(Fr, "10");
// }

// pub struct FqConfig;
// impl MontConfig<5> for FqConfig {
//     const MODULUS: BigInt<5> = field_new!(Fq, "475922286169261325753349249653048451545124879242694725395555128576210262817955800483758081");
//     const INVERSE: u64 = 0xb071a1b67165ffff;

//     const TWO_ADICITY: u32 = 17;
// }

// pub struct Fq2Config;
// impl Fp2Config for Fq2Config {
//     type Fp = Fq;
//     const NONRESIDUE: Fq = field_new!(Fq, "17");

//     const FROBENIUS_COEFF_C1: [Fq; 2] = [
//         field_new!(Fq, "1"),
//         field_new!(Fq, "475922286169261325753349249653048451545124879242694725395555128576210262817955800483758080"),
//     ];
// }

// pub type Fq4 = Fp4<Fq4Config>;
// pub struct Fq4Config;
// impl Fp4Config for Fq4Config {
//     type Fp2Config = Fq2Config;
//     const NONRESIDUE: Fq2 = Fq2::new(field_new!(Fq, "17"), Fq::ZERO);
// }
