//  Declaration of PublicParams for Fp field arithmetic

use ffec::common::default_types::ec_pp;
use std::marker::PhantomData;
use std::ops::{AddAssign, BitXorAssign, MulAssign, Neg, SubAssign};
// /*******************                        R1P World                           ******************/
/* curve-specific public parameters */
#[derive(Default, Clone, PartialEq)]
pub struct default_ec_pp;

#[derive(Default, Clone, PartialEq)]
pub struct bigint;
impl bigint {
    pub fn test_bit(&self, i: u32) -> bool {
        false
    }
}
pub trait FrConfig: Default + Clone {
    fn size_in_bits() -> usize {
        0
    }
}
impl FrConfig for default_ec_pp {}

#[derive(Default, Clone, PartialEq)]
pub struct Fr<T: FrConfig>(PhantomData<T>);
pub type Fp = Fr<default_ec_pp>;
impl Fp {
    pub fn as_ulong(&self) -> u64 {
        0
    }
    pub fn as_bigint(&self) -> bigint {
        bigint::default()
    }
}
use crate::gadgetlib2::variable::FElemInterface;
impl FElemInterface for Fp {}

impl BitXorAssign<u64> for Fp {
    #[inline]
    fn bitxor_assign(&mut self, other: u64) {}
}

impl AddAssign<i64> for Fp {
    #[inline]
    fn add_assign(&mut self, other: i64) {}
}

impl SubAssign<i64> for Fp {
    #[inline]
    fn sub_assign(&mut self, other: i64) {}
}

impl MulAssign<i64> for Fp {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: i64) {}
}

impl AddAssign<u64> for Fp {
    #[inline]
    fn add_assign(&mut self, other: u64) {}
}

impl SubAssign<u64> for Fp {
    #[inline]
    fn sub_assign(&mut self, other: u64) {}
}

impl MulAssign<u64> for Fp {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: u64) {}
}

impl AddAssign<&Self> for Fp {
    #[inline]
    fn add_assign(&mut self, other: &Self) {}
}

impl SubAssign<&Self> for Fp {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {}
}

impl MulAssign<&Self> for Fp {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &Self) {}
}

impl From<u64> for Fp {
    fn from(rhs: u64) -> Self {
        Self::default()
    }
}
impl From<i64> for Fp {
    fn from(rhs: i64) -> Self {
        Self::default()
    }
}

type FpVector = Vec<Fp>;

pub struct PublicParams {
    pub log_p: usize,
}

impl PublicParams {
    pub fn new(log_p: usize) -> Self {
        Self { log_p }
    }

    pub fn getFp(x: i64) -> Fp {
        return Fp::from(x);
    }
}

pub fn initPublicParamsFromDefaultPp<Fr: FrConfig>() -> PublicParams {
    // default_ec_pp::init_public_params();
    let log_p = Fr::size_in_bits();
    PublicParams::new(log_p)
}
