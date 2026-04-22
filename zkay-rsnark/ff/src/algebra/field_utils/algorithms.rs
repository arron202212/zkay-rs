// #![feature(generic_const_exprs)]

// Declaration of interfaces for (square-and-multiply) exponentiation and
// Tonelli-Shanks square root.

use crate::{algebra::field_utils::bigint::BigIntegerT, bigint};
use num_traits::{One, Zero};
use std::{fmt::Debug, ops::BitXor};

pub trait FPMConfig: Zero + One {}
// #![feature(generic_const_exprs)]
pub trait FieldTForPowersConfig<const N: usize>:
    Send
    + Sync
    + 'static
    + Sized
    + Default
    + Clone
    + Copy
    + Eq
    + Debug
    + One
    + Zero
    + for<'a> std::ops::MulAssign<&'a Self>
    + BitXor<crate::algebra::field_utils::bigint::bigint<N>, Output = Self>
{
    type FPM: FPMConfig;
    const num_limbs: usize = 254;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 42; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Self::FPM; // a quadratic nonresidue
    const nqr_to_t: Self::FPM; // nqr^t
    fn squared_(&self) -> Self {
        Default::default()
    }
}

////Repeated squaring.
//
// FieldT power(base:&FieldT, exponent:&bigint<m>);

////Repeated squaring.
//
// FieldT power(base:&FieldT, const u64 exponent);

//
//  * The u64 long versions exist because libiop tends to use usize instead
//  * of u64, and usize may be the same size as ul or ull.
//
//
// FieldT power(base:&FieldT, const u64  exponent);

//
// FieldT power(base:&FieldT, const Vec<u64> exponent);

//
//  * Tonelli-Shanks square root with given s, t, and quadratic non-residue.
//  * Only terminates if there is a square root. Only works if required parameters
//  * are set in the field class.
//
//
// FieldT tonelli_shanks_sqrt<(value:&FieldT);

pub struct Powers;

pub trait PowerConfig<const N: usize, T = Self> {
    fn power<FieldT: FieldTForPowersConfig<N>>(base: &FieldT, exponent: T) -> FieldT;
}

impl<const N: usize> PowerConfig<N, bigint<N>> for Powers {
    fn power<FieldT: FieldTForPowersConfig<N>>(base: &FieldT, exponent: bigint<N>) -> FieldT {
        let mut result = FieldT::one();
        let mut found_one = false;

        for i in (0..exponent.max_bits()).rev() {
            if found_one {
                result = result * result;
            }

            if exponent.test_bit(i) {
                found_one = true;
                result = result * base.clone();
            }
        }

        result
    }
}

//
// FieldT power(base:&FieldT, const u64 exponent)
impl<const N: usize> PowerConfig<N, u64> for Powers {
    fn power<FieldT: FieldTForPowersConfig<N>>(base: &FieldT, exponent: u64) -> FieldT {
        Self::power::<FieldT>(base, bigint::<N>::new(exponent))
    }
}

//
// FieldT power(base:&FieldT, const u64 exponent)
impl<const N: usize> PowerConfig<N, u128> for Powers {
    fn power<FieldT: FieldTForPowersConfig<N>>(base: &FieldT, exponent: u128) -> FieldT {
        let mut result = FieldT::one();

        let mut found_one = false;

        for i in (0..8 * std::mem::size_of_val(&exponent)).rev() {
            if found_one {
                result = result.squared_();
            }

            if exponent & (1 << i) != 0 {
                found_one = true;
                result *= base;
            }
        }

        result
    }
}

// FieldT power(base:&FieldT, const Vec<u64 long> exponent)
impl<const N: usize> PowerConfig<N, Vec<u128>> for Powers {
    fn power<FieldT: FieldTForPowersConfig<N>>(base: &FieldT, exponent: Vec<u128>) -> FieldT {
        let mut result = FieldT::one();

        let mut found_one = false;

        for j in 0..exponent.len() {
            let mut cur_exp = exponent[j];
            for i in (0..8 * std::mem::size_of_val(&cur_exp)).rev() {
                if found_one {
                    result = result.squared_();
                }

                if cur_exp & (1 << i) != 0 {
                    found_one = true;
                    result *= base;
                }
            }
        }

        result
    }
}

pub fn tonelli_shanks_sqrt<const N: usize, FieldT: FieldTForPowersConfig<N, FPM = FieldT>>(
    value: &FieldT,
) -> Option<FieldT> {
    // A few assertions to make sure s, t, and nqr are initialized.
    assert!(FieldT::s != 0);
    assert!(!FieldT::t.is_even()); // Check that t is odd.
    assert!(!FieldT::nqr.is_zero());

    if value.is_zero() {
        return Some(FieldT::zero());
    }

    let mut one = FieldT::one();

    let mut v = FieldT::s;
    let mut z = FieldT::nqr_to_t;
    let mut w = value.clone() ^ FieldT::t_minus_1_over_2;
    let mut x = value.clone() * w;
    let mut b = x * w; // b = value^t

    // #if DEBUG
    // check if square with euler's criterion
    let mut check = b.clone();
    for i in 0..v - 1 {
        check = check.squared_();
    }
    assert!(check == one);

    // compute square root with Tonelli--Shanks
    // (does not terminate if not a square!)

    while b != one {
        let mut m = 0;
        let mut b2m = b;
        while (b2m != one) {
            //invariant: b2m = b^(2^m) after entering this loop
            b2m = b2m.squared_();
            m += 1;
        }

        let mut j = v - m - 1;
        w = z;
        while j > 0 {
            w = w.squared_();
            j -= 1;
        } // w = z^2^(v-m-1)

        z = w.squared_();
        b = b * z;
        x = x * w;
        v = m;
    }

    Some(x)
}
