// Implementation of arithmetic in the finite field F[p^2].
use crate::Fp_model;
use crate::Fp_modelConfig as FpmConfig;
use crate::Fp3_modelConfig;
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
use crate::const_new_fp_model;
use std::borrow::Borrow;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{One, Zero};
/**
 * Arithmetic in the field F[p^2].
 *
 * Let p := modulus. This interface provides arithmetic for the extension field
 * Fp2 = Fp[U]/(U^2-T::non_residue), where T::non_residue is in Fp.
 *
 * ASSUMPTION: p = 1 (mod 6)
 */
//
type Fp_modelConfig<const N: usize, T> = <T as Fp2_modelConfig<N>>::Fp_modelConfig;

pub trait Fp2_modelConfig<const N: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq
{
    type Fp_modelConfig: FpmConfig<N>;
    const non_residue: my_Fp<N, Fp_modelConfig<N, Self>> =
        const_new_fp_model::<N, Self::Fp_modelConfig>();

    const nqr: (
        my_Fp<N, Fp_modelConfig<N, Self>>,
        my_Fp<N, Fp_modelConfig<N, Self>>,
    ) = (
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
    );
    const nqr_to_t: (
        my_Fp<N, Fp_modelConfig<N, Self>>,
        my_Fp<N, Fp_modelConfig<N, Self>>,
    ) = (
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
    );
    /// non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Fp_modelConfig<N, Self>>; 2] = [
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
    ];
}

type my_Fp<const N: usize, T> = Fp_model<N, T>;
#[derive(Default, Clone, Copy, Eq)]
pub struct Fp2_model<const N: usize, T: Fp2_modelConfig<N>> {
    // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    // static i64 add_cnt;
    // static i64 sub_cnt;
    // static i64 mul_cnt;
    // static i64 sqr_cnt;
    // static i64 inv_cnt;
    //#endif

    // static bigint<2*n> euler; // (modulus^2-1)/2
    // static std::usize s;       // modulus^2 = 2^s * t + 1
    // static bigint<2*n> t;  // with t odd
    // static bigint<2*n> t_minus_1_over_2; // (t-1)/2
    // static my_Fp<N,T::Fp_modelConfig> non_residue; // X^4-non_residue irreducible over Fp; used for constructing Fp2 = Fp[X] / (X^2 - non_residue)
    // static Fp2_model<n, modulus> nqr; // a quadratic nonresidue in Fp2
    // static Fp2_model<n, modulus> nqr_to_t; // nqr^t
    // static my_Fp<N,T::Fp_modelConfig> T::Frobenius_coeffs_c1[2]; // non_residue^((modulus^i-1)/2)
    pub c0: my_Fp<N, T::Fp_modelConfig>,
    pub c1: my_Fp<N, T::Fp_modelConfig>,
    _t: PhantomData<T>,
}
// Fp2_model() {};
// Fp2_model(c0:my_Fp<N,T::Fp_modelConfig>&, c1:&my_Fp<N,T::Fp_modelConfig>)->Selfc0,c1 {};

// pub fn  clear() { c0.clear(); c1.clear(); }
// pub fn  print() const { print!("c0/c1:\n"); c0.print(); c1.print(); }
// pub fn  randomize();

// /**
//  * Returns the constituent bits in 64 bit words, in little-endian order.
//  * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//  */
// Vec<u64> to_words() const;
// /**
//  * Sets the field element from the given bits in 64 bit words, in little-endian order.
//  * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//  * Returns true when the right-most bits of each element represent a value less than the modulus.
//  */
// bool from_words(Vec<u64> words);

// bool is_zero() const { return c0.is_zero() && c1.is_zero(); }
// bool operator==(other:&Fp2_model) const;
// bool operator!=(other:&Fp2_model) const;

// Fp2_model& operator+=(other:&Fp2_model);
// Fp2_model& operator-=(other:&Fp2_model);
// Fp2_model& operator*=(other:&Fp2_model);
// Fp2_model& operator^=(const u64 pow);

// Fp2_model& operator^=(pow:&bigint<m>);

// Fp2_model operator+(other:&Fp2_model) const;
// Fp2_model operator-(other:&Fp2_model) const;
// Fp2_model operator*(other:&Fp2_model) const;
// Fp2_model operator^(const:u64 pow),

// Fp2_model operator^(other:&bigint<m>) const;
// Fp2_model operator-() const;

// Fp2_model& square(); // default is squared_complex
// Fp2_model squared() const; // default is squared_complex
// Fp2_model& invert();
// Fp2_model inverse() const;
// Fp2_model Frobenius_map(u64 power) const;
// Fp2_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)
// Fp2_model squared_karatsuba() const;
// Fp2_model squared_complex() const;

// static std::usize ceil_size_in_bits() { return 2 * my_Fp::<N,T::Fp_modelConfig>::ceil_size_in_bits(); }
// static std::usize floor_size_in_bits() { return 2 * my_Fp::<N,T::Fp_modelConfig>::floor_size_in_bits(); }

// static constexpr std::usize extension_degree() { return 2; }
// static constexpr bigint<n> field_char() { return modulus; }

// static Fp2_model<n, modulus> zero();
// static Fp2_model<n, modulus> one();
// static Fp2_model<n, modulus> random_element();

// friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp2_model<n, modulus>);
// friend std::istream& operator>> <n, modulus>(std::istream &in, Fp2_model<n, modulus> &el);
// }

// use crate::algebra::field_utils::field_utils;

impl<const N: usize, T: Fp2_modelConfig<N>> Fp2_model<N, T> {
    pub fn ceil_size_in_bits() -> usize {
        2 * my_Fp::<N, T::Fp_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        2 * my_Fp::<N, T::Fp_modelConfig>::floor_size_in_bits()
    }

    pub const fn new(c0: my_Fp<N, T::Fp_modelConfig>, c1: my_Fp<N, T::Fp_modelConfig>) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp::<N, T::Fp_modelConfig>::zero(),
            my_Fp::<N, T::Fp_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp::<N, T::Fp_modelConfig>::one(),
            my_Fp::<N, T::Fp_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp::<N, T::Fp_modelConfig>::random_element(),
            c1: my_Fp::<N, T::Fp_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn squared(&self) -> Self {
        self.squared_complex()
    }

    pub fn square(&mut self) -> &Self {
        *self = self.squared();
        &*self
    }

    pub fn squared_karatsuba(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.sqr_cnt++;
        //#endif
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba squaring) */
        let (a, b) = (self.c0, self.c1);
        let asq = a.squared();
        let bsq = b.squared();

        Self::new(asq + T::non_residue * bsq, (a + b).squared() - asq - bsq)
    }

    pub fn squared_complex(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.sqr_cnt++;
        //#endif
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex squaring) */
        let (a, b) = (self.c0, self.c1);
        let ab = a * b;

        Self::new(
            (a + b) * (a + T::non_residue * b) - ab - T::non_residue * ab,
            ab + ab,
        )
    }

    pub fn inverse(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.inv_cnt++;
        //#endif
        let (a, b) = (self.c0, self.c1);

        /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */
        let t0 = a.squared();
        let t1 = b.squared();
        let t2 = t0 - T::non_residue * t1;
        let t3 = t2.inverse();
        let c0 = a * t3;
        let c1 = -(b * t3);

        Self::new(c0, c1)
    }

    pub fn invert(&mut self) -> &Self {
        *self = self.inverse();
        &*self
    }

    pub fn Frobenius_map(&self, power: usize) -> Self {
        Self::new(
            self.c0,
            T::Frobenius_coeffs_c1[power as usize % 2] * self.c1,
        )
    }

    pub fn sqrt(self) -> Self {
        tonelli_shanks_sqrt(&self)
    }

    pub fn to_words(&self) -> Vec<u64> {
        self.c0
            .to_words()
            .into_iter()
            .chain(self.c1.to_words())
            .collect()
    }

    pub fn from_words(&self, words: &[u64]) -> bool {
        let n = words.len() / 2;
        // Fp_model's from_words() takes care of asserts about vector length.
        self.c0.clone().from_words(&words[0..n]) && self.c1.clone().from_words(&words[n..])
    }
    pub fn clear(&mut self) {
        self.c0.clear();
        self.c1.clear();
    }
    pub fn print(&self) {
        print!("c0/c1:\n");
        self.c0.print();
        self.c1.print();
    }
    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }
    pub fn extension_degree() -> usize {
        2
    }
    pub fn field_char() -> bigint<N> {
        T::Fp_modelConfig::modulus
    }
}

//
// bool Fp2_model<n,modulus>::operator==(other:&Fp2_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1);
// }
impl<const N: usize, T: Fp2_modelConfig<N>> PartialEq for Fp2_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

//
// bool Fp2_model<n,modulus>::operator!=(other:&Fp2_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator+(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
// //#endif
//     return Fp2_model<n,modulus>(self.c0 + other.c0,
//                                 self.c1 + other.c1);
// }
impl<const N: usize, T: Fp2_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp2_model<N, T> {
    type Output = Fp2_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}

//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif
//     return Fp2_model<n,modulus>(self.c0 - other.c0,
//                                 self.c1 - other.c1);
// }
impl<const N: usize, T: Fp2_modelConfig<N>> Sub for Fp2_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> <Fp2_model<N, T> as Sub>::Output {
        let mut r = self;
        r -= other;
        r
    }
}

//
// Fp2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp2_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }
impl<const N: usize, T: Fp2_modelConfig<N>> Mul<&Fp_model<N, T::Fp_modelConfig>>
    for &Fp2_model<N, T>
{
    type Output = Fp2_model<N, T>;

    fn mul(self, rhs: &Fp_model<N, T::Fp_modelConfig>) -> Self::Output {
        let rhs = *rhs;
        Fp2_model::<N, T>::new(self.c0 * rhs, self.c1 * rhs)
    }
}

//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator*(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     const my_Fp<N,T::Fp2_modelConfig>
//         &A = other.c0, &B = other.c1,
//         &a = self.c0, &b = self.c1;
//     let aA= a * A;
//     let bB= b * B;

//     return Fp2_model<n,modulus>(aA + T::non_residue * bB,
//                                 (a + b)*(A+B) - aA - bB);
// }

impl<const N: usize, T: Fp2_modelConfig<N>, O: Borrow<Self>> Mul<O> for Fp2_model<N, T> {
    type Output = Fp2_model<N, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= *rhs.borrow();
        r
    }
}
//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-() const
// {
//     return Fp2_model<n,modulus>(-self.c0,
//                                 -self.c1);
// }
impl<const N: usize, T: Fp2_modelConfig<N>> Neg for Fp2_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        // mpn_sub_n(r.mont_repr.0.0, modulus.0.0, self.mont_repr.0.0, n);
        r
    }
}
//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp2_model<n, modulus>>(*this, pow);
// }
impl<const N: usize, T: Fp2_modelConfig<N>> BitXor<u64> for Fp2_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}
//
//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp2_model<n, modulus>, m>(*this, pow);
// }
impl<const N: usize, const M: usize, T: Fp2_modelConfig<N>> BitXor<&bigint<M>> for Fp2_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<M>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator+=(const Fp2_model<n,modulus>& other)
// {
//     *self = *this + other;
//     return *self;
// }
impl<const N: usize, T: Fp2_modelConfig<N>, O: Borrow<Self>> AddAssign<O> for Fp2_model<N, T> {
    fn add_assign(&mut self, other: O) {}
}

//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator-=(const Fp2_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }
impl<const N: usize, T: Fp2_modelConfig<N>, O: Borrow<Self>> SubAssign<O> for Fp2_model<N, T> {
    fn sub_assign(&mut self, other: O) {}
}
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator*=(const Fp2_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }
impl<const N: usize, T: Fp2_modelConfig<N>, O: Borrow<Self>> MulAssign<O> for Fp2_model<N, T> {
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
    }
}
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, T: Fp2_modelConfig<N>> BitXorAssign<u64> for Fp2_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        // *self = Powers::power::<Fp2_model<N, T>>(self, rhs);
    }
}
//
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, const M: usize, T: Fp2_modelConfig<N>> BitXorAssign<&bigint<M>>
    for Fp2_model<N, T>
{
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        // *self = Powers::power::<Fp2_model<N, T>>(self, rhs);
    }
}

//
// std::ostream& operator<<(std::ostream &out, el:&Fp2_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

use std::fmt;
impl<const N: usize, T: Fp2_modelConfig<N>> fmt::Display for Fp2_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c0)
    }
}

impl<const N: usize, T: Fp2_modelConfig<N>> Mul<bigint<N>> for Fp2_model<N, T> {
    type Output = Self;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp2_modelConfig<N>> PpConfig for Fp2_model<N, T> {
    type T = bigint<N>;
}

impl<const N: usize, T: Fp2_modelConfig<N>> One for Fp2_model<N, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, T: Fp2_modelConfig<N>> Zero for Fp2_model<N, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
//
// std::istream& operator>>(std::istream &in, Fp2_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1;
//     return in;
// }

//
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp2_model<n, modulus> >)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

//
// std::istream& operator>>(std::istream& in, Vec<Fp2_model<n, modulus> > &v)
// {
//     v.clear();

//     usize s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     v.reserve(s);

//     for i in 0..s
//     {
//         Fp2_model<n, modulus> el;
//         in >> el;
//         v.emplace_back(el);
//     }

//     return in;
// }

use super::quadratic_extension::{QuadExtConfig, QuadExtField};
use crate::algebra::fields::cyclotomic::CyclotomicMultSubgroup;
use core::{marker::PhantomData, ops::Not};

/// Trait that specifies constants and methods for defining degree-two extension fields.
pub trait Fp2Config: 'static + Send + Sync + Sized {
    /// Base prime field underlying this extension.
    type Fp: PrimeField;

    /// Quadratic non-residue in [`Self::Fp`] used to construct the extension
    /// field. That is, `NONRESIDUE` is such that the quadratic polynomial
    /// `f(X) = X^2 - Self::NONRESIDUE` in Fp\[X\] is irreducible in `Self::Fp`.
    const NONRESIDUE: Self::Fp;

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP2_C1: &'static [Self::Fp];

    /// Return `fe * Self::NONRESIDUE`.
    /// Intended for specialization when [`Self::NONRESIDUE`] has a special
    /// structure that can speed up multiplication
    #[inline(always)]
    fn mul_fp_by_nonresidue_in_place(fe: &mut Self::Fp) -> &mut Self::Fp {
        *fe *= Self::NONRESIDUE;
        fe
    }

    /// A specializable method for setting `y = x + NONRESIDUE * y`.
    /// This allows for optimizations when the non-residue is
    /// canonically negative in the field.
    #[inline(always)]
    fn mul_fp_by_nonresidue_and_add(y: &mut Self::Fp, x: &Self::Fp) {
        Self::mul_fp_by_nonresidue_in_place(y);
        *y += x;
    }

    /// A specializable method for computing x + mul_fp_by_nonresidue(y) + y
    /// This allows for optimizations when the non-residue is not -1.
    #[inline(always)]
    fn mul_fp_by_nonresidue_plus_one_and_add(y: &mut Self::Fp, x: &Self::Fp) {
        let old_y = *y;
        Self::mul_fp_by_nonresidue_and_add(y, x);
        *y += old_y;
    }

    /// A specializable method for computing x - mul_fp_by_nonresidue(y)
    /// This allows for optimizations when the non-residue is
    /// canonically negative in the field.
    #[inline(always)]
    fn sub_and_mul_fp_by_nonresidue(y: &mut Self::Fp, x: &Self::Fp) {
        *y = *x - Self::mul_fp_by_nonresidue_in_place(y);
    }
}

/// Wrapper for [`Fp2Config`], allowing combination of the [`Fp2Config`] and [`QuadExtConfig`] traits.
pub struct Fp2ConfigWrapper<P: Fp2Config>(PhantomData<P>);

impl<P: Fp2Config> QuadExtConfig for Fp2ConfigWrapper<P> {
    type BasePrimeField = P::Fp;
    type BaseField = P::Fp;
    type FrobCoeff = P::Fp;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 2;

    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const FROBENIUS_COEFF_C1: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP2_C1;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp_by_nonresidue_in_place(fe)
    }

    #[inline(always)]
    fn mul_base_field_by_nonresidue_and_add(y: &mut Self::BaseField, x: &Self::BaseField) {
        P::mul_fp_by_nonresidue_and_add(y, x)
    }

    #[inline(always)]
    fn mul_base_field_by_nonresidue_plus_one_and_add(y: &mut Self::BaseField, x: &Self::BaseField) {
        P::mul_fp_by_nonresidue_plus_one_and_add(y, x)
    }

    #[inline(always)]
    fn sub_and_mul_base_field_by_nonresidue(y: &mut Self::BaseField, x: &Self::BaseField) {
        P::sub_and_mul_fp_by_nonresidue(y, x)
    }

    fn mul_base_field_by_frob_coeff(fe: &mut Self::BaseField, power: usize) {
        *fe *= &Self::FROBENIUS_COEFF_C1[power % Self::DEGREE_OVER_BASE_PRIME_FIELD];
    }
}

/// Alias for instances of quadratic extension fields. Helpful for omitting verbose
/// instantiations involving `Fp2ConfigWrapper`.
pub type Fp2<P> = QuadExtField<Fp2ConfigWrapper<P>>;

impl<P: Fp2Config> Fp2<P> {
    /// In-place multiply both coefficients `c0` and `c1` of `self`
    /// by an element from [`Fp`](`Fp2Config::Fp`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ark_std::test_rng;
    /// # use ark_test_curves::bls12_381::{Fq as Fp, Fq2 as Fp2};
    /// # use ark_std::UniformRand;
    /// let c0: Fp = Fp::rand(&mut test_rng());
    /// let c1: Fp = Fp::rand(&mut test_rng());
    /// let mut ext_element: Fp2 = Fp2::new(c0, c1);
    ///
    /// let base_field_element: Fp = Fp::rand(&mut test_rng());
    /// ext_element.mul_assign_by_fp(&base_field_element);
    ///
    /// assert_eq!(ext_element.c0, c0 * base_field_element);
    /// assert_eq!(ext_element.c1, c1 * base_field_element);
    /// ```
    pub fn mul_assign_by_fp(&mut self, other: &P::Fp) {
        self.c0 *= other;
        self.c1 *= other;
    }
}

impl<P: Fp2Config> CyclotomicMultSubgroup for Fp2<P> {
    const INVERSE_IS_FAST: bool = true;
    fn cyclotomic_inverse_in_place(&mut self) -> Option<&mut Self> {
        // As the multiplicative subgroup is of order p^2 - 1, the
        // only non-trivial cyclotomic subgroup is of order p+1
        // Therefore, for any element in the cyclotomic subgroup, we have that `x^(p+1) = 1`.
        // Recall that `x^(p+1)` in a quadratic extension field is equal
        // to the norm in the base field, so we have that
        // `x * x.conjugate() = 1`. By uniqueness of inverses,
        // for this subgroup, x.inverse() = x.conjugate()

        self.is_zero().not().then(|| {
            self.conjugate_in_place();
            self
        })
    }
}
