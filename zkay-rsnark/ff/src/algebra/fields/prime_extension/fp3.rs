//  Declaration of arithmetic in the finite  field F[p^3].

// use crate::algebra::fields::prime_base::fp;
use crate::Fp_model;
use crate::Fp_modelConfig as FpmConfig;
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
// /**
//  * Arithmetic in the field F[p^3].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  * Fp3 = Fp[U]/(U^3-T::non_residue), where T::non_residue is in Fp.
//  *
//  * ASSUMPTION: p = 1 (mod 6)
//  */
//
type Fp_modelConfig<const N: usize, T> = <T as Fp3_modelConfig<N>>::Fp_modelConfig;
pub trait Fp3_modelConfig<const N: usize>:
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
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Fp_modelConfig<N, Self>>; 2] = [
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
    ];
    const Frobenius_coeffs_c2: [my_Fp<N, Fp_modelConfig<N, Self>>; 2] = [
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
        const_new_fp_model::<N, Self::Fp_modelConfig>(),
    ];
}

type my_Fp<const N: usize, T> = Fp_model<N, T>;

#[derive(Default, Clone, Copy, Eq)]
pub struct Fp3_model<const N: usize, T: Fp3_modelConfig<N>> {
    // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    // static i64 add_cnt;
    // static i64 sub_cnt;
    // static i64 mul_cnt;
    // static i64 sqr_cnt;
    // static i64 inv_cnt;
    //#endif

    // static bigint<3*n> euler; // (modulus^3-1)/2
    // static std::usize s;       // modulus^3 = 2^s * t + 1
    // static bigint<3*n> t;  // with t odd
    // static bigint<3*n> t_minus_1_over_2; // (t-1)/2
    // static my_Fp<N,T::Fp_modelConfig> T::non_residue; // X^6-T::non_residue irreducible over Fp; used for constructing Fp3 = Fp[X] / (X^3 - T::non_residue)
    // static Fp3_model<n, modulus> nqr; // a quadratic nonresidue in Fp3
    // static Fp3_model<n, modulus> nqr_to_t; // nqr^t
    // static my_Fp<N,T::Fp_modelConfig> Frobenius_coeffs_c1[3]; // T::non_residue^((modulus^i-1)/3)   for i=0,1,2
    // static my_Fp<N,T::Fp_modelConfig> Frobenius_coeffs_c2[3]; // T::non_residue^((2*modulus^i-2)/3) for i=0,1,2
    pub c0: my_Fp<N, T::Fp_modelConfig>,
    pub c1: my_Fp<N, T::Fp_modelConfig>,
    pub c2: my_Fp<N, T::Fp_modelConfig>,
    _t: PhantomData<T>,
    // Fp3_model() {};
    // Fp3_model(c0:my_Fp<N,T::Fp_modelConfig>, c1:my_Fp<N,T::Fp_modelConfig>, c2:my_Fp<N,T::Fp_modelConfig>)->Selfc0,c1,c2 {};

    // pub fn  clear() { c0.clear(); c1.clear(); c2.clear(); }
    // pub fn  print() const { print!("c0/c1/c2:\n"); c0.print(); c1.print(); c2.print(); }
    // pub fn  randomize();

    // /**
    //  * Returns the constituent bits in 64 bit words, in little-endian order.
    //  * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
    //  */
    // Vec<uint64_t> to_words() const;
    // /**
    //  * Sets the field element from the given bits in 64 bit words, in little-endian order.
    //  * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
    //  * Returns true when the right-most bits of each element represent a value less than the modulus.
    //  */
    // bool from_words(Vec<uint64_t> words);

    // bool is_zero() const { return c0.is_zero() && c1.is_zero() && c2.is_zero(); }
    // bool operator==(other:&Fp3_model) const;
    // bool operator!=(other:&Fp3_model) const;

    // Fp3_model& operator+=(other:&Fp3_model);
    // Fp3_model& operator-=(other:&Fp3_model);
    // Fp3_model& operator*=(other:&Fp3_model);
    // Fp3_model& operator^=(const u64 pow);

    // Fp3_model& operator^=(pow:&bigint<m>);

    // Fp3_model operator+(other:&Fp3_model) const;
    // Fp3_model operator-(other:&Fp3_model) const;
    // Fp3_model operator*(other:&Fp3_model) const;
    // Fp3_model operator^(const:u64 pow),

    // Fp3_model operator^(other:&bigint<m>) const;
    // Fp3_model operator-() const;

    // Fp3_model& square();
    // Fp3_model squared() const;
    // Fp3_model& invert();
    // Fp3_model inverse() const;
    // Fp3_model Frobenius_map(u64 power) const;
    // Fp3_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

    // static std::usize ceil_size_in_bits() { return 3 * my_Fp::<N,T::Fp_modelConfig>::ceil_size_in_bits(); }
    // static std::usize floor_size_in_bits() { return 3 * my_Fp::<N,T::Fp_modelConfig>::floor_size_in_bits(); }

    // static constexpr std::usize extension_degree() { return 3; }
    // static constexpr bigint<n> field_char() { return modulus; }

    // static Fp3_model<n, modulus> zero();
    // static Fp3_model<n, modulus> one();
    // static Fp3_model<n, modulus> random_element();

    // friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp3_model<n, modulus>);
    // friend std::istream& operator>> <n, modulus>(std::istream &in, Fp3_model<n, modulus> &el);
}

// use crate::algebra::field_utils::field_utils;
impl<const N: usize, T: Fp3_modelConfig<N>> Fp3_model<N, T> {
    pub fn ceil_size_in_bits() -> usize {
        3 * my_Fp::<N, T::Fp_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        3 * my_Fp::<N, T::Fp_modelConfig>::floor_size_in_bits()
    }
    pub fn new(
        c0: my_Fp<N, T::Fp_modelConfig>,
        c1: my_Fp<N, T::Fp_modelConfig>,
        c2: my_Fp<N, T::Fp_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            c2,
            _t: PhantomData,
        }
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp::<N, T::Fp_modelConfig>::zero(),
            my_Fp::<N, T::Fp_modelConfig>::zero(),
            my_Fp::<N, T::Fp_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp::<N, T::Fp_modelConfig>::one(),
            my_Fp::<N, T::Fp_modelConfig>::zero(),
            my_Fp::<N, T::Fp_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp::<N, T::Fp_modelConfig>::random_element(),
            c1: my_Fp::<N, T::Fp_modelConfig>::random_element(),
            c2: my_Fp::<N, T::Fp_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn squared(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.sqr_cnt++;
        //#endif
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (CH-SQR2) */

        let (a, b, c) = (self.c0, self.c1, self.c2);
        let s0 = a.squared();
        let ab = a * b;
        let s1 = ab + ab;
        let s2 = (a - b + c).squared();
        let bc = b * c;
        let s3 = bc + bc;
        let s4 = c.squared();

        Self::new(
            s0 + T::non_residue * s3,
            s1 + T::non_residue * s4,
            s1 + s2 + s3 - s0 - s4,
        )
    }

    pub fn square(&mut self) -> &Self {
        *self = self.squared();
        &*self
    }

    pub fn inverse(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.inv_cnt++;
        //#endif
        let (a, b, c) = (self.c0, self.c1, self.c2);

        /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17 */
        let t0 = a.squared();
        let t1 = b.squared();
        let t2 = c.squared();
        let t3 = a * b;
        let t4 = a * c;
        let t5 = b * c;
        let c0 = t0 - T::non_residue * t5;
        let c1 = T::non_residue * t2 - t3;
        let c2 = t1 - t4; // typo in paper referenced above. should be "-" as per, but is "*"
        let t6 = (a * c0 + T::non_residue * (c * c1 + b * c2)).inverse();
        Self::new(t6 * c0, t6 * c1, t6 * c2)
    }

    pub fn invert(&mut self) -> &Self {
        *self = self.inverse();
        &*self
    }

    pub fn Frobenius_map(&self, power: usize) -> Self {
        Self::new(
            self.c0,
            T::Frobenius_coeffs_c1[power % 3] * self.c1,
            T::Frobenius_coeffs_c2[power % 3] * self.c2,
        )
    }

    pub fn sqrt(&self) -> Self {
        tonelli_shanks_sqrt(&self)
    }

    pub fn to_words(&self) -> Vec<u64> {
        self.c0
            .to_words()
            .into_iter()
            .chain(self.c1.to_words())
            .chain(self.c2.to_words())
            .collect()
    }

    pub fn from_words(&self, words: &[u64]) -> bool {
        let n = words.len() / 3;
        // Fp_model's from_words() takes care of asserts about vector length.
        self.c0.clone().from_words(&words[0..n])
            && self.c1.clone().from_words(&words[n..n * 2])
            && self.c2.clone().from_words(&words[n * 2..])
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
// std::ostream& operator<<(std::ostream &out, el:&Fp3_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1 << OUTPUT_SEPARATOR << el.c2;
//     return out;
// }

//
// bool Fp3_model<n,modulus>::operator==(other:&Fp3_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1 && self.c2 == other.c2);
// }

//
// bool Fp3_model<n,modulus>::operator!=(other:&Fp3_model<n,modulus>) const
// {
//     return !(operator==(other));
// }
impl<const N: usize, T: Fp3_modelConfig<N>> PartialEq for Fp3_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // false
        false
    }
}
//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator+=(const Fp3_model<n,modulus>& other)
// {
//     *self = *this + other;
//     return *self;
// }

impl<const N: usize, T: Fp3_modelConfig<N>, O: Borrow<Self>> AddAssign<O> for Fp3_model<N, T> {
    fn add_assign(&mut self, other: O) {}
}
//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator-=(const Fp3_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }

impl<const N: usize, T: Fp3_modelConfig<N>, O: Borrow<Self>> SubAssign<O> for Fp3_model<N, T> {
    fn sub_assign(&mut self, other: O) {}
}
//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator*=(const Fp3_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }

impl<const N: usize, T: Fp3_modelConfig<N>, O: Borrow<Self>> MulAssign<O> for Fp3_model<N, T> {
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
    }
}
//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, T: Fp3_modelConfig<N>> BitXorAssign<u64> for Fp3_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        // *self = Powers::power::<Fp3_model<N, T>>(self, rhs);
    }
}
//
//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }

impl<const N: usize, const M: usize, T: Fp3_modelConfig<N>> BitXorAssign<&bigint<M>>
    for Fp3_model<N, T>
{
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        ////*self = Powers::power::<Fp3_model<N, T>>(self, rhs);
    }
}

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator+(other:&Fp3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
// //#endif
//     return Fp3_model<n,modulus>(self.c0 + other.c0,
//                                 self.c1 + other.c1,
//                                 self.c2 + other.c2);
// }

impl<const N: usize, T: Fp3_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp3_model<N, T> {
    type Output = Fp3_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}
//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator-(other:&Fp3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif
//     return Fp3_model<n,modulus>(self.c0 - other.c0,
//                                 self.c1 - other.c1,
//                                 self.c2 - other.c2);
// }

impl<const N: usize, T: Fp3_modelConfig<N>> Sub for Fp3_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        r -= other;
        r
    }
}
//
// Fp3_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp3_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp3_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1,
//                                 lhs*rhs.c2);
// }

impl<const N: usize, T: Fp3_modelConfig<N>> Mul<&Fp_model<N, T::Fp_modelConfig>>
    for &Fp3_model<N, T>
{
    type Output = Fp3_model<N, T>;

    fn mul(self, rhs: &Fp_model<N, T::Fp_modelConfig>) -> Self::Output {
        let rhs = *rhs;
        Fp3_model::<N, T>::new(self.c0 * rhs, self.c1 * rhs, self.c2 * rhs)
    }
}

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator*(other:&Fp3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (Karatsuba) */
//     const my_Fp<N,T::Fp3_modelConfig>
//         &A = other.c0, &B = other.c1, &C = other.c2,
//         &a = self.c0, &b = self.c1, &c = self.c2;
//     let aA= a*A;
//     let bB= b*B;
//     let cC= c*C;

//     return Fp3_model<n,modulus>(aA + T::non_residue*((b+c)*(B+C)-bB-cC),
//                                 (a+b)*(A+B)-aA-bB+T::non_residue*cC,
//                                 (a+c)*(A+C)-aA+bB-cC);
// }

impl<const N: usize, T: Fp3_modelConfig<N>, O: Borrow<Self>> Mul<O> for Fp3_model<N, T> {
    type Output = Fp3_model<N, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= *rhs.borrow();
        r
    }
}
//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp3_model<n, modulus> >(*this, pow);
// }

impl<const N: usize, T: Fp3_modelConfig<N>> BitXor<u64> for Fp3_model<N, T> {
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
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp3_model<n, modulus> >(*this, pow);
// }

impl<const N: usize, const M: usize, T: Fp3_modelConfig<N>> BitXor<&bigint<M>> for Fp3_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<M>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}
//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator-() const
// {
//     return Fp3_model<n,modulus>(-self.c0,
//                                 -self.c1,
//                                 -self.c2);
// }

impl<const N: usize, T: Fp3_modelConfig<N>> Neg for Fp3_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        // mpn_sub_n(r.mont_repr.0.0, modulus.0.0, self.mont_repr.0.0, n);
        r
    }
}

use std::fmt;
impl<const N: usize, T: Fp3_modelConfig<N>> fmt::Display for Fp3_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c0)
    }
}
//
// std::istream& operator>>(std::istream &in, Fp3_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1 >> el.c2;
//     return in;
// }

//
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp3_model<n, modulus> >)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }
impl<const N: usize, T: Fp3_modelConfig<N>> PpConfig for Fp3_model<N, T> where <T as Fp3_modelConfig<N>>::Fp_modelConfig: PpConfig{
    type TT = bigint<N>;
    //  type Fr=T::Fp_modelConfig;
}

impl<const N: usize, T: Fp3_modelConfig<N>> Mul<bigint<N>> for Fp3_model<N, T> {
    type Output = Self;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp3_modelConfig<N>> One for Fp3_model<N, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, T: Fp3_modelConfig<N>> Zero for Fp3_model<N, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
//
// std::istream& operator>>(std::istream& in, Vec<Fp3_model<n, modulus> > &v)
// {
//     v.clear();

//     usize s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     v.reserve(s);

//     for i in 0..s
//     {
//         Fp3_model<n, modulus> el;
//         in >> el;
//         v.emplace_back(el);
//     }

//     return in;
// }

use super::cubic_extension::{CubicExtConfig, CubicExtField};
use crate::algebra::fields::cyclotomic::CyclotomicMultSubgroup;
// use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use core::marker::PhantomData;

/// Trait that specifies constants and methods for defining degree-three extension fields.
pub trait Fp3Config: 'static + Send + Sync + Sized {
    /// Base prime field underlying this extension.
    type Fp: PrimeField;
    /// Cubic non-residue in `Self::Fp` used to construct the extension
    /// field. That is, `NONRESIDUE` is such that the cubic polynomial
    /// `f(X) = X^3 - Self::NONRESIDUE` in Fp\[X\] is irreducible in `Self::Fp`.
    const NONRESIDUE: Self::Fp;

    const FROBENIUS_COEFF_FP3_C1: &'static [Self::Fp];
    const FROBENIUS_COEFF_FP3_C2: &'static [Self::Fp];

    /// p^3 - 1 = 2^s * t, where t is odd.
    const TWO_ADICITY: u32;
    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64];
    /// t-th power of a quadratic nonresidue in Fp3.
    const QUADRATIC_NONRESIDUE_TO_T: Fp3<Self>;

    /// Return `fe * Self::NONRESIDUE`.
    /// The default implementation can be specialized if [`Self::NONRESIDUE`] has a special
    /// structure that can speed up multiplication
    #[inline(always)]
    fn mul_fp_by_nonresidue_in_place(fe: &mut Self::Fp) -> &mut Self::Fp {
        *fe *= Self::NONRESIDUE;
        fe
    }
}

/// Wrapper for [`Fp3Config`], allowing combination of the [`Fp3Config`] and [`CubicExtConfig`] traits.
pub struct Fp3ConfigWrapper<P: Fp3Config>(PhantomData<P>);

impl<P: Fp3Config> CubicExtConfig for Fp3ConfigWrapper<P> {
    type BasePrimeField = P::Fp;
    type BaseField = P::Fp;
    type FrobCoeff = P::Fp;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 3;
    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const SQRT_PRECOMP: Option<SqrtPrecomputation<CubicExtField<Self>>> =
        Some(SqrtPrecomputation::TonelliShanks {
            two_adicity: P::TWO_ADICITY,
            quadratic_nonresidue_to_trace: P::QUADRATIC_NONRESIDUE_TO_T,
            trace_of_modulus_minus_one_div_two: P::TRACE_MINUS_ONE_DIV_TWO,
        });

    const FROBENIUS_COEFF_C1: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP3_C1;
    const FROBENIUS_COEFF_C2: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP3_C2;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp_by_nonresidue_in_place(fe)
    }

    fn mul_base_field_by_frob_coeff(
        c1: &mut Self::BaseField,
        c2: &mut Self::BaseField,
        power: usize,
    ) {
        *c1 *= &Self::FROBENIUS_COEFF_C1[power % Self::DEGREE_OVER_BASE_PRIME_FIELD];
        *c2 *= &Self::FROBENIUS_COEFF_C2[power % Self::DEGREE_OVER_BASE_PRIME_FIELD];
    }
}

pub type Fp3<P> = CubicExtField<Fp3ConfigWrapper<P>>;

impl<P: Fp3Config> Fp3<P> {
    /// In-place multiply all coefficients `c0`, `c1`, and `c2` of `self`
    /// by an element from [`Fp`](`Fp3Config::Fp`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ark_std::test_rng;
    /// # use ark_std::UniformRand;
    /// # use ark_test_curves::mnt6_753 as ark_mnt6_753;
    /// use ark_mnt6_753::{Fq as Fp, Fq3 as Fp3};
    /// let c0: Fp = Fp::rand(&mut test_rng());
    /// let c1: Fp = Fp::rand(&mut test_rng());
    /// let c2: Fp = Fp::rand(&mut test_rng());
    /// let mut ext_element: Fp3 = Fp3::new(c0, c1, c2);
    ///
    /// let base_field_element: Fp = Fp::rand(&mut test_rng());
    /// ext_element.mul_assign_by_fp(&base_field_element);
    ///
    /// assert_eq!(ext_element.c0, c0 * base_field_element);
    /// assert_eq!(ext_element.c1, c1 * base_field_element);
    /// assert_eq!(ext_element.c2, c2 * base_field_element);
    /// ```
    pub fn mul_assign_by_fp(&mut self, value: &P::Fp) {
        self.c0 *= value;
        self.c1 *= value;
        self.c2 *= value;
    }
}

// We just use the default algorithms; there don't seem to be any faster ones.
impl<P: Fp3Config> CyclotomicMultSubgroup for Fp3<P> {}
