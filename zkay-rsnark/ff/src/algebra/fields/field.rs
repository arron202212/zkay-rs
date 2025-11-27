/** @file
*****************************************************************************
Declaration of common API for all finite fields.

Currently NOT used by the fields in this library. This pub struct is not actually
the parent pub struct of any field. All APIs are enforced through tests instead.

The reason for this is to ensure high performance of all fields. This class
exists as documentation for common API between fields.

Includes two types of fields, F[p^n] for selected n and F[2^n] for a separate
range of n. All of these finite fields must implement all functions declared
in this class.
*****************************************************************************
* @author     This file is part of libff, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
use crate::algebra::field_utils::bigint;

/* The type parameter T is intended to be set to the child class
when this pub struct is extended. For example,
pub struct Fp_model : public Field<Fp_model> ... */
//
// pub trait Field<T> {

//      fn square()->T ;
//      fn invert()->T ;

//      fn squared()->T ;
//      fn inverse()->T ;
//     /** HAS TO BE A SQUARE (else does not terminate). */
//      fn sqrt()->T ;

//     //  T operator^(:u64 pow),

//     //  T operator^(pow:&bigint<m>) ;

//     // bool operator==(other:&T) ,
//     // bool operator!=(other:&T) ,
//     fn is_zero()->bool ;

//     fn  print() ;
//     /**
//      * Returns the ituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     fn to_words()->Vec<u64>;
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Returns true when the right-most bits represent a value less than the modulus.
//      */
//     fn from_words(words:Vec<u64>)->bool ;

//     fn  randomize() ;
//     fn  clear() ;

//     /* The  functions should be defined in field classes, but are  so they
//        can't be inherited. */
//      fn zero()->T;
//      fn one()->T;
//      fn random_element()->T;
//     /** Equals 1 for prime field Fp. */
//      fn  extension_degree()->usize;
//      fn ceil_size_in_bits()->usize;
//      fn floor_size_in_bits()->usize;

// }

// } // namespace libff

use super::fpn_field::PrimeField;
use super::sqrt::{LegendreSymbol, SqrtPrecomputation};
use crate::algebra::UniformRand;
use ark_serialize::{
    CanonicalDeserialize, CanonicalDeserializeWithFlags, CanonicalSerialize,
    CanonicalSerializeWithFlags, EmptyFlags, Flags,
};
use ark_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::*,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    vec::*,
};

// pub use ff_macros;
pub use num_traits::{One, Zero};
use zeroize::Zeroize;

// #[cfg(feature = "parallel")]
// use ark_std::cmp::max;
// #[cfg(feature = "parallel")]
// use rayon::prelude::*;

pub trait AdditiveGroup:
    Eq
    + 'static
    + Sized
    + CanonicalSerialize
    + CanonicalDeserialize
    + Copy
    + Clone
    + Default
    + Send
    + Sync
    + Hash
    + Debug
    + Display
    + UniformRand
    + Zeroize
    + Zero
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<<Self as AdditiveGroup>::Scalar, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<<Self as AdditiveGroup>::Scalar>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a <Self as AdditiveGroup>::Scalar, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a <Self as AdditiveGroup>::Scalar>
    + for<'a> Add<&'a mut Self, Output = Self>
    + for<'a> Sub<&'a mut Self, Output = Self>
    + for<'a> Mul<&'a mut <Self as AdditiveGroup>::Scalar, Output = Self>
    + for<'a> AddAssign<&'a mut Self>
    + for<'a> SubAssign<&'a mut Self>
    + for<'a> MulAssign<&'a mut <Self as AdditiveGroup>::Scalar>
    + ark_std::iter::Sum<Self>
    + for<'a> ark_std::iter::Sum<&'a Self>
{
    type Scalar: Field;

    /// The additive identity of the field.
    const ZERO: Self;

    /// Doubles `self`.
    #[must_use]
    fn double(&self) -> Self {
        let mut copy = *self;
        copy.double_in_place();
        copy
    }
    /// Doubles `self` in place.
    fn double_in_place(&mut self) -> &mut Self {
        *self += *self;
        self
    }

    /// Negates `self` in place.
    fn neg_in_place(&mut self) -> &mut Self {
        *self = -(*self);
        self
    }
}

/// The interface for a generic field.
/// Types implementing [`Field`] support common field operations such as addition, subtraction, multiplication, and inverses.
///
/// ## Defining your own field
/// To demonstrate the various field operations, we can first define a prime ordered field $\mathbb{F}_{p}$ with $p = 17$. When defining a field $\mathbb{F}_p$, we need to provide the modulus(the $p$ in $\mathbb{F}_p$) and a generator. Recall that a generator $g \in \mathbb{F}_p$ is a field element whose powers comprise the entire field: $\mathbb{F}_p =\\{g, g^1, \ldots, g^{p-1}\\}$.
/// We can then manually construct the field element associated with an integer with `Fp::from` and perform field addition, subtraction, multiplication, and inversion on it.
/// ```rust
/// use ark_ff::{AdditiveGroup, fields::{Field, Fp64, MontBackend, MontConfig}};
///
/// #[derive(MontConfig)]
/// #[modulus = "17"]
/// #[generator = "3"]
/// pub struct FqConfig;
/// pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
///
/// # fn main() {
/// let a = Fq::from(9);
/// let b = Fq::from(10);
///
/// assert_eq!(a, Fq::from(26));          // 26 =  9 mod 17
/// assert_eq!(a - b, Fq::from(16));      // -1 = 16 mod 17
/// assert_eq!(a + b, Fq::from(2));       // 19 =  2 mod 17
/// assert_eq!(a * b, Fq::from(5));       // 90 =  5 mod 17
/// assert_eq!(a.square(), Fq::from(13)); // 81 = 13 mod 17
/// assert_eq!(b.double(), Fq::from(3));  // 20 =  3 mod 17
/// assert_eq!(a / b, a * b.inverse().unwrap()); // need to unwrap since `b` could be 0 which is not invertible
/// # }
/// ```
///
/// ## Using pre-defined fields
/// In the following example, we’ll use the field associated with the BLS12-381 pairing-friendly group.
/// ```rust
/// use ark_ff::{AdditiveGroup, Field};
/// use ark_test_curves::bls12_381::Fq as F;
/// use ark_std::{One, UniformRand, test_rng};
///
/// let mut rng = test_rng();
/// // Let's sample uniformly random field elements:
/// let a = F::rand(&mut rng);
/// let b = F::rand(&mut rng);
///
/// let c = a + b;
/// let d = a - b;
/// assert_eq!(c + d, a.double());
///
/// let e = c * d;
/// assert_eq!(e, a.square() - b.square());         // (a + b)(a - b) = a^2 - b^2
/// assert_eq!(a.inverse().unwrap() * a, F::one()); // Euler-Fermat theorem tells us: a * a^{-1} = 1 mod p
/// ```
pub trait Field:
    'static
    + Copy
    + Clone
    + Debug
    + Display
    + Default
    + Send
    + Sync
    + Eq
    + Zero
    + One
    + Ord
    + Neg<Output = Self>
    + UniformRand
    + Zeroize
    + Sized
    + Hash
    + CanonicalSerialize
    + CanonicalSerializeWithFlags
    + CanonicalDeserialize
    + CanonicalDeserializeWithFlags
    + AdditiveGroup<Scalar = Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
    + for<'a> Div<&'a mut Self, Output = Self>
    + for<'a> DivAssign<&'a mut Self>
    + for<'a> core::iter::Product<&'a Self>
    + From<u128>
    + From<u64>
    + From<u32>
    + From<u16>
    + From<u8>
    + From<i128>
    + From<i64>
    + From<i32>
    + From<i16>
    + From<i8>
    + From<bool>
    + Product<Self>
{
    type BasePrimeField: PrimeField;

    /// Determines the algorithm for computing square roots.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Self>>;

    /// The multiplicative identity of the field.
    const ONE: Self;

    /// Returns the characteristic of the field,
    /// in little-endian representation.
    fn characteristic() -> &'static [u64] {
        Self::BasePrimeField::characteristic()
    }

    /// Returns the extension degree of this field with respect
    /// to `Self::BasePrimeField`.
    fn extension_degree() -> u64;

    fn to_base_prime_field_elements(&self) -> impl Iterator<Item = Self::BasePrimeField>;

    /// Convert a slice of base prime field elements into a field element.
    /// If the slice length != Self::extension_degree(), must return None.
    fn from_base_prime_field_elems(
        elems: impl IntoIterator<Item = Self::BasePrimeField>,
    ) -> Option<Self>;

    /// Constructs a field element from a single base prime field elements.
    /// ```
    /// # use ark_ff::Field;
    /// # use ark_test_curves::bls12_381::Fq as F;
    /// # use ark_test_curves::bls12_381::Fq2 as F2;
    /// # use ark_std::One;
    /// assert_eq!(F2::from_base_prime_field(F::one()), F2::one());
    /// ```
    fn from_base_prime_field(elem: Self::BasePrimeField) -> Self;

    /// Attempt to deserialize a field element. Returns `None` if the
    /// deserialization fails.
    ///
    /// This function is primarily intended for sampling random field elements
    /// from a hash-function or RNG output.
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_random_bytes_with_flags::<EmptyFlags>(bytes).map(|f| f.0)
    }

    /// Attempt to deserialize a field element, splitting the bitflags metadata
    /// according to `F` specification. Returns `None` if the deserialization
    /// fails.
    ///
    /// This function is primarily intended for sampling random field elements
    /// from a hash-function or RNG output.
    fn from_random_bytes_with_flags<F: Flags>(bytes: &[u8]) -> Option<(Self, F)>;

    /// Returns a `LegendreSymbol`, which indicates whether this field element
    /// is  1 : a quadratic residue
    ///  0 : equal to 0
    /// -1 : a quadratic non-residue
    fn legendre(&self) -> LegendreSymbol;

    /// Returns the square root of self, if it exists.
    #[must_use]
    fn sqrt(&self) -> Option<Self> {
        match Self::SQRT_PRECOMP {
            Some(tv) => tv.sqrt(self),
            None => unimplemented!(),
        }
    }

    /// Sets `self` to be the square root of `self`, if it exists.
    fn sqrt_in_place(&mut self) -> Option<&mut Self> {
        (*self).sqrt().map(|sqrt| {
            *self = sqrt;
            self
        })
    }

    /// Returns `self * self`.
    #[must_use]
    fn square(&self) -> Self;

    /// Squares `self` in place.
    fn square_in_place(&mut self) -> &mut Self;

    /// Computes the multiplicative inverse of `self` if `self` is nonzero.
    #[must_use]
    fn inverse(&self) -> Option<Self>;

    /// If `self.inverse().is_none()`, this just returns `None`. Otherwise, it sets
    /// `self` to `self.inverse().unwrap()`.
    fn inverse_in_place(&mut self) -> Option<&mut Self>;

    /// Returns `sum([a_i * b_i])`.
    #[inline]
    fn sum_of_products<const T: usize>(a: &[Self; T], b: &[Self; T]) -> Self {
        let mut sum = Self::zero();
        for i in 0..a.len() {
            sum += a[i] * b[i];
        }
        sum
    }

    /// Sets `self` to `self^s`, where `s = Self::BasePrimeField::MODULUS^power`.
    /// This is also called the Frobenius automorphism.
    fn frobenius_map_in_place(&mut self, power: usize);

    /// Returns `self^s`, where `s = Self::BasePrimeField::MODULUS^power`.
    /// This is also called the Frobenius automorphism.
    #[must_use]
    fn frobenius_map(&self, power: usize) -> Self {
        let mut this = *self;
        this.frobenius_map_in_place(power);
        this
    }

    /// Returns `self^exp`, where `exp` is an integer represented with `u64` limbs,
    /// least significant limb first.
    #[must_use]
    fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        let mut res = Self::one();

        for i in crate::algebra::bits::BitIteratorBE::without_leading_zeros(exp) {
            res.square_in_place();

            if i {
                res *= self;
            }
        }
        res
    }

    /// Exponentiates a field element `f` by a number represented with `u64`
    /// limbs, using a precomputed table containing as many powers of 2 of
    /// `f` as the 1 + the floor of log2 of the exponent `exp`, starting
    /// from the 1st power. That is, `powers_of_2` should equal `&[p, p^2,
    /// p^4, ..., p^(2^n)]` when `exp` has at most `n` bits.
    ///
    /// This returns `None` when a power is missing from the table.
    #[inline]
    fn pow_with_table<S: AsRef<[u64]>>(powers_of_2: &[Self], exp: S) -> Option<Self> {
        let mut res = Self::one();
        for (pow, bit) in
            crate::algebra::bits::BitIteratorLE::without_trailing_zeros(exp).enumerate()
        {
            if bit {
                res *= powers_of_2.get(pow)?;
            }
        }
        Some(res)
    }

    fn mul_by_base_prime_field(&self, elem: &Self::BasePrimeField) -> Self;
}

// Given a vector of field elements {v_i}, compute the vector {v_i^(-1)}
pub fn batch_inversion<F: Field>(v: &mut [F]) {
    batch_inversion_and_mul(v, &F::one());
}

#[cfg(not(feature = "parallel"))]
// Given a vector of field elements {v_i}, compute the vector {coeff * v_i^(-1)}
pub fn batch_inversion_and_mul<F: Field>(v: &mut [F], coeff: &F) {
    serial_batch_inversion_and_mul(v, coeff);
}

#[cfg(feature = "parallel")]
// Given a vector of field elements {v_i}, compute the vector {coeff * v_i^(-1)}
pub fn batch_inversion_and_mul<F: Field>(v: &mut [F], coeff: &F) {
    // Divide the vector v evenly between all available cores
    let min_elements_per_thread = 1;
    let num_cpus_available = rayon::current_num_threads();
    let num_elems = v.len();
    let num_elem_per_thread = max(num_elems / num_cpus_available, min_elements_per_thread);

    // Batch invert in parallel, without copying the vector
    v.par_chunks_mut(num_elem_per_thread).for_each(|mut chunk| {
        serial_batch_inversion_and_mul(&mut chunk, coeff);
    });
}

/// Given a vector of field elements {v_i}, compute the vector {coeff * v_i^(-1)}.
/// This method is explicitly single-threaded.
fn serial_batch_inversion_and_mul<F: Field>(v: &mut [F], coeff: &F) {
    // Montgomery’s Trick and Fast Implementation of Masked AES
    // Genelle, Prouff and Quisquater
    // Section 3.2
    // but with an optimization to multiply every element in the returned vector by
    // coeff

    // First pass: compute [a, ab, abc, ...]
    let mut prod = Vec::with_capacity(v.len());
    let mut tmp = F::one();
    for f in v.iter().filter(|f| !f.is_zero()) {
        tmp *= f;
        prod.push(tmp);
    }

    // Invert `tmp`.
    tmp = tmp.inverse().unwrap(); // Guaranteed to be nonzero.

    // Multiply product by coeff, so all inverses will be scaled by coeff
    tmp *= coeff;

    // Second pass: iterate backwards to compute inverses
    for (f, s) in v
        .iter_mut()
        // Backwards
        .rev()
        // Ignore normalized elements
        .filter(|f| !f.is_zero())
        // Backwards, skip last element, fill in one for last term.
        .zip(prod.into_iter().rev().skip(1).chain(Some(F::one())))
    {
        // tmp := tmp * f; f := tmp * s = 1/f
        let new_tmp = tmp * *f;
        *f = tmp * &s;
        tmp = new_tmp;
    }
}
