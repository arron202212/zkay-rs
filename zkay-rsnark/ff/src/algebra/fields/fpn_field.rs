
//  Declaration of common API for all finite fields in the prime_base/ and
//  prime_extension/ directories.

//  Currently NOT used by the fields in this library. This pub struct is not actually
//  the parent pub struct of any field. All APIs are enforced through tests instead.

//  The reason for this is to ensure high performance of all fields. This class
//  exists as documentation for common API between fields.

//  Includes fields Fp^n for specified n. All of the prime extension fields must
//  implement all functions declared in this class.


use ark_ff::{BigInteger };

use ark_std::{cmp::min, str::FromStr};
use num_bigint::BigUint;
use super::{field::Field,fft_friendly::FftField};

/// The interface for a prime field, i.e. the field of integers modulo a prime $p$.
/// In the following example we'll use the prime field underlying the BLS12-381 G1 curve.
/// ```rust
/// use ark_ff::{BigInteger, Field, PrimeField, Zero};
/// use ark_std::{test_rng, One, UniformRand};
/// use ark_test_curves::bls12_381::Fq as F;
///
/// let mut rng = test_rng();
/// let a = F::rand(&mut rng);
/// // We can access the prime modulus associated with `F`:
/// let modulus = <F as PrimeField>::MODULUS;
/// assert_eq!(a.pow(&modulus), a); // the Euler-Fermat theorem tells us: a^{p-1} = 1 mod p
///
/// // We can convert field elements to integers in the range [0, MODULUS - 1]:
/// let one: num_bigint::BigUint = F::one().into();
/// assert_eq!(one, num_bigint::BigUint::one());
///
/// // We can construct field elements from an arbitrary sequence of bytes:
/// let n = F::from_le_bytes_mod_order(&modulus.to_bytes_le());
/// assert_eq!(n, F::zero());
/// ```
// pub trait FpParameters: 'static + Send + Sync + Sized {
//     type BigInt: BigInteger;

//     /// The modulus of the field.
//     const MODULUS: Self::BigInt;

//     /// The number of bits needed to represent the `Self::MODULUS`.
//     const MODULUS_BITS: u32;

//     /// The number of bits that must be shaved from the beginning of
//     /// the representation when randomly sampling.
//     const REPR_SHAVE_BITS: u32;

//     /// Let `M` be the power of 2^64 nearest to `Self::MODULUS_BITS`. Then
//     /// `R = M % Self::MODULUS`.
//     const R: Self::BigInt;

//     /// R2 = R^2 % Self::MODULUS
//     const R2: Self::BigInt;

//     /// INV = -MODULUS^{-1} mod 2^64
//     const INV: u64;

//     /// A multiplicative generator of the field.
//     /// `Self::GENERATOR` is an element having multiplicative order
//     /// `Self::MODULUS - 1`.
//     const GENERATOR: Self::BigInt;

//     /// The number of bits that can be reliably stored.
//     /// (Should equal `SELF::MODULUS_BITS - 1`)
//     const CAPACITY: u32;

//     /// 2^s * t = MODULUS - 1 with t odd. This is the two-adicity of
//     /// `Self::MODULUS`.
//     const TWO_ADICITY: u32;

//     /// t for 2^s * t = MODULUS - 1
//     const T: Self::BigInt;

//     /// 2^s root of unity computed by GENERATOR^t
//     const ROOT_OF_UNITY: Self::BigInt;

//     /// (t - 1) / 2
//     const T_MINUS_ONE_DIV_TWO: Self::BigInt;

//     /// (Self::MODULUS - 1) / 2
//     const MODULUS_MINUS_ONE_DIV_TWO: Self::BigInt;
// }
// pub trait FpParameters: Send + Sync + 'static + Sized 
// {   
//      type BigInt: BigInteger;
//      const  num_limbs:usize;
//     const modulus:Self::BigInt;
//     const num_bits:usize;
//      const  euler:Self::BigInt; // (modulus-1)/2
//      const  s:usize; // modulus = 2^s * t + 1
//      const t:Self::BigInt; // with t odd
//      const t_minus_1_over_2:Self::BigInt; // (t-1)/2
//      const nqr:Self; // a quadratic nonresidue
//      const nqr_to_t:Self; // nqr^t
//      const multiplicative_generator:Self; // generator of Fp^*
//      const root_of_unity:Self; // generator^((modulus-1)/2^s)
//      const inv:u64; // modulus^(-1) mod W, where W = 2^(word size)
//      const Rsquared:Self::BigInt ; // R^2, where R = W^k, where k = ??
//      const Rcubed: Self::BigInt;   // R^3
// }

pub trait PrimeField:
    Field<BasePrimeField = Self>
    + FftField
    + FromStr
    + From<<Self as PrimeField>::BigInt>
    + Into<<Self as PrimeField>::BigInt>
    + From<BigUint>
    + Into<BigUint>
{
   
    /// A `BigInteger` type that can represent elements of this field.
    type BigInt: BigInteger;

    /// The modulus `p`.
    const MODULUS: Self::BigInt;

    /// The value `(p - 1)/ 2`.
    const MODULUS_MINUS_ONE_DIV_TWO: Self::BigInt;

    /// The size of the modulus in bits.
    const MODULUS_BIT_SIZE: u32;

    /// The trace of the field is defined as the smallest integer `t` such that by
    /// `2^s * t = p - 1`, and `t` is coprime to 2.
    const TRACE: Self::BigInt;
    /// The value `(t - 1)/ 2`.
    const TRACE_MINUS_ONE_DIV_TWO: Self::BigInt;

    /// Construct a prime field element from an integer in the range 0..(p - 1).
    fn from_bigint(repr: Self::BigInt) -> Option<Self>;

    /// Converts an element of the prime field into an integer in the range 0..(p - 1).
    fn into_bigint(self) -> Self::BigInt;

    /// Reads bytes in big-endian, and converts them to a field element.
    /// If the integer represented by `bytes` is larger than the modulus `p`, this method
    /// performs the appropriate reduction.
    fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        let mut bytes_copy = bytes.to_vec();
        bytes_copy.reverse();
        Self::from_le_bytes_mod_order(&bytes_copy)
    }

    /// Reads bytes in little-endian, and converts them to a field element.
    /// If the integer represented by `bytes` is larger than the modulus `p`, this method
    /// performs the appropriate reduction.
    fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        let num_modulus_bytes = ((Self::MODULUS_BIT_SIZE + 7) / 8) as usize;
        let num_bytes_to_directly_convert = min(num_modulus_bytes - 1, bytes.len());
        // Copy the leading little-endian bytes directly into a field element.
        // The number of bytes directly converted must be less than the
        // number of bytes needed to represent the modulus, as we must begin
        // modular reduction once the data is of the same number of bytes as the
        // modulus.
        let (bytes, bytes_to_directly_convert) =
            bytes.split_at(bytes.len() - num_bytes_to_directly_convert);
        // Guaranteed to not be None, as the input is less than the modulus size.
        let mut res = Self::from_random_bytes(bytes_to_directly_convert).unwrap();

        // Update the result, byte by byte.
        // We go through existing field arithmetic, which handles the reduction.
        // TODO: If we need higher speeds, parse more bytes at once, or implement
        // modular multiplication by a u64
        let window_size = Self::from(256u64);
        for byte in bytes.iter().rev() {
            res *= window_size;
            res += Self::from(*byte);
        }
        res
    }
}
