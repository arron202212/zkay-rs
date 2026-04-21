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

pub const edwards_r_bitcount: usize = 181;
pub const edwards_q_bitcount: usize = 183;

pub const edwards_r_limbs: usize = (edwards_r_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
pub const edwards_q_limbs: usize = (edwards_q_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
const edwards_q_limbs2: usize = edwards_q_limbs * 2;
const edwards_q_limbs3: usize = edwards_q_limbs * 3;
const edwards_q_limbs6: usize = edwards_q_limbs * 6;
const edwards_q_limbs12: usize = edwards_q_limbs * 12;
pub type edwards_Fr = Fp_model<edwards_r_limbs, Backend>;
pub type edwards_Fq = Fp_model<edwards_q_limbs, Backend>;
pub type edwards_Fq3 = Fp3_model<edwards_q_limbs, edwards_q_limbs3, Backend>;
pub type edwards_Fq6 = Fp6_2over3_model<
    edwards_q_limbs,
    edwards_q_limbs2,
    edwards_q_limbs3,
    edwards_q_limbs6,
    Backend,
>;
pub type edwards_GT = edwards_Fq6;

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
    const num_limbs: usize = edwards_q_limbs;
    type GType = Self;
}
impl Fp_modelConfig<edwards_q_limbs> for Backend {}
impl Fp2_modelConfig<edwards_q_limbs, edwards_q_limbs2> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp3_modelConfig<edwards_q_limbs, edwards_q_limbs3> for Backend {
    type Fp_modelConfig = Self;
}
impl Fp6_modelConfig<edwards_q_limbs, edwards_q_limbs2, edwards_q_limbs6> for Backend {
    type Fp_modelConfig = Self;
    type Fp2_modelConfig = Self;
}
impl
    ffec::fp6_2over3::Fp6_modelConfig<
        edwards_q_limbs,
        edwards_q_limbs2,
        edwards_q_limbs3,
        edwards_q_limbs6,
    > for Backend
{
    type Fp_modelConfig = Self;
    type Fp2_modelConfig = Self;
    type Fp3_modelConfig = Self;
}
impl Fp12_modelConfig<edwards_q_limbs, edwards_q_limbs2, edwards_q_limbs6, edwards_q_limbs12>
    for Backend
{
    type Fp_modelConfig = Self;
    type Fp6_modelConfig = Self;
}

pub fn init_edwards_fields() {}

// use ark_ff::{BigInteger, Fp3, Fp6, MontgomeryConfig};

// pub struct FrConfig;
// impl MontgomeryConfig<3> for FrConfig {

//     const MODULUS: BigInt<3> = BigInt::new([//limbs]);

//     const INVERSE: u64 = 0xdde553277fffffff;

//     const RSQUARED: BigInt<3> = BigInt::new([//对应 Rsquared]);
// }

// pub struct FqConfig;
// impl MontgomeryConfig<3> for FqConfig {

//     const MODULUS: BigInt<3> = BigInt::new([//limbs]);
//     const INVERSE: u64 = 0x76eb690b7fffffff;

//     const TWO_ADICITY: u32 = 31;
//     const TRACE_MINUS_ONE_DIV_TWO: BigInt<3> = BigInt::new([//t_minus_1_over_2]);
// }

// pub type Fq3 = Fp3<Fq3Config>;
// pub struct Fq3Config;
// impl Fp3Config for Fq3Config {
//     type Fp = Fq;

//     const NONRESIDUE: Fq = field_new!(Fq, "61");

//     const FROBENIUS_COEFF_FP3_C1: [Fq; 3] = [
//         field_new!(Fq, "1"),
//         field_new!(
//             Fq,
//             "1073752683758513276629212192812154536507607213288832061"
//         ),
//         field_new!(
//             Fq,
//             "5136291436651207728317994048073823738016144056504959939"
//         ),
//     ];
// }

// pub type Fq6 = Fp6<Fq6Config>;
// pub struct Fq6Config;
// impl Fp6Config for Fq6Config {
//     type Fp3Config = Fq3Config;

//     const NONRESIDUE: Fq3 = Fq3::new(field_new!(Fq, "5"), Fq::ZERO, Fq::ZERO);
// }
