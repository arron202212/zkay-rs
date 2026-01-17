// Declaration of arithmetic in the finite field F[p], for prime p of fixed length.
use crate::FieldTConfig;
use crate::PpConfig;
use crate::algebra::{
    field_utils::{
        BigInteger,
        algorithms::{PowerConfig, Powers, tonelli_shanks_sqrt},
        bigint::{GMP_NUMB_BITS, bigint},
        field_utils, fp_aux, {BigInt, algorithms},
    },
    fields::{
        field::{AdditiveGroup, Field},
        fpn_field::PrimeField,
        sqrt::SqrtPrecomputation,
    },
};
use crate::common::utils::bit_vector;
use num_traits::{One, Zero};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use educe::Educe;
use std::marker::PhantomData;
//  use crate::algebra::field_utils::bigint::bigint;

/**
 * Arithmetic in the finite field F[p], for prime p of fixed length.
 *
 * This pub struct implements Fp-arithmetic, for a large prime p, using a fixed number
 * of words. It is optimized for tight memory consumption, so the modulus p is
 * passed as a template parameter, to avoid per-element overheads.
 *
 * The implementation is mostly a wrapper around GMP's MPN (constant-size integers).
 * But for the integer sizes of interest for libff (3 to 5 limbs of 64 bits each),
 * we implement performance-critical routines, like addition and multiplication,
 * using hand-optimzied assembly code.
 */

pub trait Fp_modelConfig<const N: usize>:
    Send + Sync + 'static + Sized + Default + Clone + Copy + Eq + Debug
{
    // const num_limbs: usize = 42;
    const modulus: bigint<N> = bigint::<N>::one();
    const num_bits: usize = 42;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 42; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Fp_model<N, Self> = const_new_fp_model::<N, Self>(); // a quadratic nonresidue
    const nqr_to_t: Fp_model<N, Self> = const_new_fp_model::<N, Self>(); // nqr^t
    const multiplicative_generator: Fp_model<N, Self> = const_new_fp_model::<N, Self>(); // generator of Fp^*
    const root_of_unity: Fp_model<N, Self> = const_new_fp_model::<N, Self>(); // generator^((modulus-1)/2^s)
    const inv: u64 = 42; // modulus^(-1) mod W, where W = 2^(word size)
    const Rsquared: bigint<N> = bigint::<N>::one(); // R^2, where R = W^k, where k = ??
    const Rcubed: bigint<N> = bigint::<N>::one(); // R^3
}
pub const fn const_new_fp_model<const N: usize, T: Fp_modelConfig<N>>() -> Fp_model<N, T> {
    Fp_model::<N, T> {
        mont_repr: bigint::<N>::one(),
        t: PhantomData,
    }
}
#[derive(Educe)]
#[educe(Default, Clone, Debug, Hash, Copy, Eq)] // PartialEq,
pub struct Fp_model<const N: usize, T: Fp_modelConfig<N>> {
    pub mont_repr: bigint<N>,
    pub t: PhantomData<T>,
}

//      let num_limbs= n;
//      modulus:constexpr bigint<N>& mod =,
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//      i64 add_cnt;
//      i64 sub_cnt;
//      i64 mul_cnt;
//      i64 sqr_cnt;
//      i64 inv_cnt;
// //#endif
//      std::usize num_bits;
//      bigint<N> euler; // (modulus-1)/2
//      std::usize s; // modulus = 2^s * t + 1
//      bigint<N> t; // with t odd
//      bigint<N> t_minus_1_over_2; // (t-1)/2
//      Fp_model<n, modulus> nqr; // a quadratic nonresidue
//      Fp_model<n, modulus> nqr_to_t; // nqr^t
//      Fp_model<n, modulus> multiplicative_generator; // generator of Fp^*
//      Fp_model<n, modulus> root_of_unity; // generator^((modulus-1)/2^s)
//      mp_limb_t inv; // modulus^(-1) mod W, where W = 2^(word size)
//      bigint<N> Rsquared; // R^2, where R = W^k, where k = ??
//      bigint<N> Rcubed;   // R^3

//     Fp_model() {};
//     Fp_model(b:&bigint<N>);
//     Fp_model(x:long, is_unsigned:bool=false);

//     set_ulong(const u64 x);

//     /** Performs the operation montgomery_reduce(other * this.mont_repr). */
//     mul_reduce(other:&bigint<N>);

//     clear();
//     print();
//     randomize();

//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     Vec<uint64_t> to_words();
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Returns true when the right-most bits represent a value less than the modulus.
//      *
//      * Precondition: the vector is large enough to contain ceil_size_in_bits() bits.
//      */
//     bool from_words(Vec<uint64_t> words);

//     /* Return the standard (not Montgomery) representation of the
//        Field element's requivalence class. I.e. Fp(2).as_bigint()
//         would return bigint(2) */
//     bigint<N> as_bigint();
//     /* Return the last limb of the standard representation of the
//        field element. E.g. on 64-bit architectures Fp(123).as_ulong()
//        and Fp(2^64+123).as_ulong() would both return 123. */
//     u64 as_ulong();

//     bool operator==(other:&Fp_model);
//     bool operator!=(other:&Fp_model);
//     bool is_zero();

//     Fp_model& operator+=(other:&Fp_model);
//     Fp_model& operator-=(other:&Fp_model);
//     Fp_model& operator*=(other:&Fp_model);
//     Fp_model& operator^=(const u64 pow);
//
//     Fp_model& operator^=(pow:&bigint<m>);

//     Fp_model operator+(other:&Fp_model);
//     Fp_model operator-(other:&Fp_model);
//     Fp_model operator*(other:&Fp_model);
//     Fp_model operator^(const u64 pow);
//
//     Fp_model operator^(pow:&bigint<m>);
//     Fp_model operator-();

//     Fp_model& square();
//     Fp_model squared();
//     Fp_model& invert();
//     Fp_model inverse();
//     Fp_model Frobenius_map(u64 power);
//     Fp_model sqrt(); // HAS TO BE A SQUARE (else does not terminate)

//      std::usize ceil_size_in_bits() { return num_bits; }
//      std::usize floor_size_in_bits() { return num_bits - 1; }

//      constexpr std::usize extension_degree() { return 1; }
//      constexpr bigint<N> field_char() { return modulus; }
//      bool modulus_is_valid() { return modulus.0.0[n-1] != 0; } // mpn inverse assumes that highest limb is non-zero

//      Fp_model<n, modulus> zero();
//      Fp_model<n, modulus> one();
//      Fp_model<n, modulus> random_element();
//      Fp_model<n, modulus> geometric_generator(); // generator^k, for k = 1 to m, domain size m
//      Fp_model<n, modulus> arithmetic_generator();// generator++, for k = 1 to m, domain size m

//     friend std::ostream& operator<< <n,T>(std::ostream &out, p:&Fp_model<n, modulus>);
//     friend std::istream& operator>> <n,T>(std::istream &in, Fp_model<n, modulus> &p);

//
//     /** Returns a representation in bigint, depending on the MONTGOMERY_OUTPUT flag. */
//     bigint<N> bigint_repr();
// };

impl<const N: usize, T: Fp_modelConfig<N>> Fp_modelConfig<N> for Fp_model<N, T> {
    // const num_limbs: usize = 1;
    const modulus: bigint<N> = bigint::<N>::one();
    const num_bits: usize = 1;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Fp_model<N, Self> = Fp_model::<N, Self>::new(bigint::<N>::one()); // a quadratic nonresidue
    const nqr_to_t: Fp_model<N, Self> = Fp_model::<N, Self>::new(bigint::<N>::one()); // nqr^t
    const multiplicative_generator: Fp_model<N, Self> =
        Fp_model::<N, Self>::new(bigint::<N>::one()); // generator of Fp^*
    const root_of_unity: Fp_model<N, Self> = Fp_model::<N, Self>::new(bigint::<N>::one()); // generator^((modulus-1)/2^s)
    const inv: u64 = 1; // modulus^(-1) mod W, where W = 2^(word size)
    const Rsquared: bigint<N> = bigint::<N>::one(); // R^2, where R = W^k, where k = ??
    const Rcubed: bigint<N> = bigint::<N>::one(); // R^3
}

// impl<const N: usize, T: Fp_modelConfig<N>> Borrow<Self> for Fp_model<N, T> {
//     fn borrow(&self)->Self{
//         *self
//     }
// }

impl<const N: usize, T: Fp_modelConfig<N>> FieldTConfig for Fp_model<N, T> {}

impl<const N: usize, T: Fp_modelConfig<N>> PpConfig for Fp_model<N, T> {
    type TT = bigint<N>;
    // type Fr=Self;
}
impl<const N: usize, T: Fp_modelConfig<N>> From<usize> for Fp_model<N, T> {
    fn from(b: usize) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<u32> for Fp_model<N, T> {
    fn from(b: u32) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<i32> for Fp_model<N, T> {
    fn from(b: i32) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<i64> for Fp_model<N, T> {
    fn from(b: i64) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    pub const fn const_new(b: BigInt<N>) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>(b),
            t: PhantomData,
        }
    }

    pub fn mul_reduce(&mut self, other: &bigint<N>) {}

    pub const fn new(b: bigint<N>) -> Self {
        // mpn_copyi(self.mont_repr.data, Rsquared.data, n);
        let mut _self = Self {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        };
        // _self.mul_reduce(b);
        _self
    }

    pub fn new_with_i64(x: i64, is_unsigned: bool) -> Self {
        // assert!(std::numeric_limits<mp_limb_t>::max() >= std::numeric_limits<long>::max() as u64, "long won't fit in mp_limb_t");
        if is_unsigned || x >= 0 {
            // self.mont_repr.data[0] = x;//(mp_limb_t)
        } else {
            // let  borrow = mpn_sub_1(self.mont_repr.data, modulus.data, n, (mp_limb_t)-x);
            //#ifndef NDEBUG
            //             assert!(borrow == 0);
            // #else
            //             UNUSED(borrow);
            //#endif
        }

        // Self::mul_reduce(T::Rsquared);
        Self {
            mont_repr: bigint::<N>::new(0),
            t: PhantomData,
        }
    }
    pub fn set_ulong(&mut self, x: u64) {
        self.mont_repr.clear();
        self.mont_repr.0.0[0] = x;
        self.mul_reduce(&T::Rsquared);
    }

    pub fn clear(&mut self) {
        self.mont_repr.clear();
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn as_bigint(&self) -> bigint<N> {
        let mut one = bigint::<N>::one();
        let mut res = self.clone();
        res.mul_reduce(&one);

        res.mont_repr
    }

    pub fn as_ulong(&self) -> u64 {
        self.as_bigint().as_ulong()
    }

    pub fn is_zero(&self) -> bool {
        self.mont_repr.is_zero() // zero maps to zero
    }

    pub fn print(&self) {
        let mut tmp = Self::zero();
        tmp.mont_repr.0.0[0] = 1;
        tmp.mul_reduce(&self.mont_repr);

        tmp.mont_repr.print();
    }
    pub fn ceil_size_in_bits() -> usize {
        T::num_bits
    }
    pub fn floor_size_in_bits() -> usize {
        T::num_bits - 1
    }

    pub fn extension_degree() -> usize {
        1
    }
    pub fn field_char() -> bigint<N> {
        T::modulus
    }
    pub fn modulus_is_valid() -> bool {
        T::modulus.0.0[N - 1] != 0
    } // mpn inverse assumes that highest limb is non-zero

    pub fn zero() -> Self {
        let mut res = Self::new_with_i64(0, false);
        // res.mont_repr.clear();
        return res;
    }

    pub fn one() -> Self {
        let mut res = Self::new_with_i64(0, false);
        // res.mont_repr.0.0[0] = 1;
        res.mul_reduce(&T::Rsquared);
        return res;
    }

    pub fn geometric_generator() -> Self {
        let mut res = Self::new_with_i64(0, false);
        res.mont_repr.0.0[0] = 2;
        res.mul_reduce(&T::Rsquared);
        res
    }

    pub fn arithmetic_generator() -> Self {
        let mut res = Self::new_with_i64(0, false);
        res.mont_repr.0.0[0] = 1;
        res.mul_reduce(&T::Rsquared);
        res
    }

    pub fn squared(&self) -> Self {
        let mut r: Self = self.clone();
        r *= &r.clone();
        r
    }

    pub fn square(&mut self) -> &Self {
        *self = self.squared();
        self
    }

    pub fn invert(&self) -> &Self {
        self
    }

    pub fn inverse(&self) -> Self {
        let mut r = self.clone();
        r.invert();
        r
    }

    pub fn Frobenius_map(&self, power: usize) -> Self {
        *self
    }

    pub fn random_element() -> Self {
        Self::one()
    }

    pub fn sqrt(&self) -> Self {
        *self
    }

    pub fn to_words(&self) -> Vec<u64> {
        // TODO: implement for other bit architectures
        assert!(
            GMP_NUMB_BITS == 64,
            "Only 64-bit architectures are currently supported"
        );
        let repr = self.bigint_repr();
        repr.0.0.clone().try_into().unwrap()
    }

    pub fn from_words(&mut self, words: &[u64]) -> bool {
        // TODO: implement for other bit architectures
        assert!(
            GMP_NUMB_BITS == 64,
            "Only 64-bit architectures are currently supported"
        );

        let start_bit = words.len() * 64; //- FieldT::ceil_size_in_bits();
        assert!(start_bit >= 0); // Check the vector is big enough.
        let start_word = start_bit / 64;
        let bit_offset = start_bit % 64;

        // Assumes mont_repr.0.0 is just the right size to fit ceil_size_in_bits().
        // std::copy(words.begin() + start_word, words.end(), self.mont_repr.0.0);
        self.mont_repr.0.0.clone_from_slice(&words[start_word..]);
        // Zero out the left-most bit_offset bits.
        self.mont_repr.0.0[N - 1] =
            ((self.mont_repr.0.0[N - 1] as u64) << bit_offset) >> bit_offset; //mp_limb_t

        // return self.mont_repr < modulus;
        false
    }

    pub fn bigint_repr(&self) -> bigint<N> {
        self.as_bigint()
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> PartialEq for Fp_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mont_repr == other.mont_repr
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> AddAssign<O> for Fp_model<N, T> {
    fn add_assign(&mut self, other: O) {}
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> SubAssign<O> for Fp_model<N, T> {
    fn sub_assign(&mut self, other: O) {}
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> MulAssign<O> for Fp_model<N, T> {
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
        self.mul_reduce(&rhs.mont_repr);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXorAssign<u64> for Fp_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = Powers::power::<Fp_model<N, T>>(self, rhs);
    }
}

impl<const N: usize, const M: usize, T: Fp_modelConfig<N>> BitXorAssign<&bigint<M>>
    for Fp_model<N, T>
{
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        *self = Powers::power::<Fp_model<N, T>>(self, rhs);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Add<i32> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Sub<i32> for Fp_model<N, T> {
    type Output = Self;

    fn sub(self, other: i32) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Sub for Fp_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        r -= other;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<bigint<N>> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<i32> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> Mul<O> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXor<u64> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXor<usize> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}
impl<const N: usize, const M: usize, T: Fp_modelConfig<N>> BitXor<&bigint<M>> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<M>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Neg for Fp_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = Self::new_with_i64(0, false);
        // mpn_sub_n(r.mont_repr.0.0, modulus.0.0, self.mont_repr.0.0, n);
        r
    }
}

use std::fmt;
impl<const N: usize, T: Fp_modelConfig<N>> fmt::Display for Fp_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bigint_repr(),)
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> One for Fp_model<N, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Zero for Fp_model<N, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

/// A trait that specifies the configuration of a prime field.
/// Also specifies how to perform arithmetic on field elements.
pub trait FpConfig<const N: usize>: Send + Sync + 'static + Sized {
    /// The modulus of the field.
    const MODULUS: BigInt<N>;

    /// A multiplicative generator of the field.
    /// `Self::GENERATOR` is an element having multiplicative order
    /// `Self::MODULUS - 1`.
    const GENERATOR: Fp<Self, N>;

    /// Additive identity of the field, i.e. the element `e`
    /// such that, for all elements `f` of the field, `e + f = f`.
    const ZERO: Fp<Self, N>;

    /// Multiplicative identity of the field, i.e. the element `e`
    /// such that, for all elements `f` of the field, `e * f = f`.
    const ONE: Fp<Self, N>;

    /// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    const TWO_ADICITY: u32;

    /// 2^s root of unity computed by GENERATOR^t
    const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N>;

    /// An integer `b` such that there exists a multiplicative subgroup
    /// of size `b^k` for some integer `k`.
    const SMALL_SUBGROUP_BASE: Option<u32> = None;

    /// The integer `k` such that there exists a multiplicative subgroup
    /// of size `Self::SMALL_SUBGROUP_BASE^k`.
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32> = None;

    /// GENERATOR^((MODULUS-1) / (2^s *
    /// SMALL_SUBGROUP_BASE^SMALL_SUBGROUP_BASE_ADICITY)) Used for mixed-radix
    /// FFT.
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<Fp<Self, N>> = None;

    /// Precomputed material for use when computing square roots.
    /// Currently uses the generic Tonelli-Shanks,
    /// which works for every modulus.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp<Self, N>>>;

    /// Set a += b.
    fn add_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Set a -= b.
    fn sub_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Set a = a + a.
    fn double_in_place(a: &mut Fp<Self, N>);

    /// Set a = -a;
    fn neg_in_place(a: &mut Fp<Self, N>);

    /// Set a *= b.
    fn mul_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Compute the inner product `<a, b>`.
    fn sum_of_products<const T: usize>(a: &[Fp<Self, N>; T], b: &[Fp<Self, N>; T]) -> Fp<Self, N>;

    /// Set a *= a.
    fn square_in_place(a: &mut Fp<Self, N>);

    /// Compute a^{-1} if `a` is not zero.
    fn inverse(a: &Fp<Self, N>) -> Option<Fp<Self, N>>;

    /// Construct a field element from an integer in the range
    /// `0..(Self::MODULUS - 1)`. Returns `None` if the integer is outside
    /// this range.
    fn from_bigint(other: BigInt<N>) -> Option<Fp<Self, N>>;

    /// Convert a field element to an integer in the range `0..(Self::MODULUS -
    /// 1)`.
    fn into_bigint(other: Fp<Self, N>) -> BigInt<N>;
}
/// Represents an element of the prime field F_p, where `p == P::MODULUS`.
/// This type can represent elements in any field of size at most N * 64 bits.
#[derive(Educe)]
#[educe(Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Fp<P: FpConfig<N>, const N: usize>(
    /// Contains the element in Montgomery form for efficient multiplication.
    /// To convert an element to a [`BigInt`](struct@BigInt), use `into_bigint` or `into`.
    #[doc(hidden)]
    pub BigInt<N>,
    #[doc(hidden)] pub PhantomData<P>,
);

pub type Fp64<P> = Fp<P, 1>;
pub type Fp128<P> = Fp<P, 2>;
pub type Fp192<P> = Fp<P, 3>;
pub type Fp256<P> = Fp<P, 4>;
pub type Fp320<P> = Fp<P, 5>;
pub type Fp384<P> = Fp<P, 6>;
pub type Fp448<P> = Fp<P, 7>;
pub type Fp512<P> = Fp<P, 8>;
pub type Fp576<P> = Fp<P, 9>;
pub type Fp640<P> = Fp<P, 10>;
pub type Fp704<P> = Fp<P, 11>;
pub type Fp768<P> = Fp<P, 12>;
pub type Fp832<P> = Fp<P, 13>;
