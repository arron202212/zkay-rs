//  Declaration of arithmetic in the finite field F[(p^2)^3]

// use crate::algebra::fields::prime_base::fp;
// use crate::algebra::fields::prime_extension::fp2;
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
use std::borrow::Borrow;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::Fp_model;
use crate::Fp_modelConfig;
use crate::Fp2_model;
use crate::Fp2_modelConfig;

// /**
//  * Arithmetic in the finite field F[(p^2)^3].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  *  Fp6 = Fp2[V]/(V^3-T::non_residue) where T::non_residue is in Fp.
//  *
//  * ASSUMPTION: p = 1 (mod 6)
//  */
//

pub trait Fp6_modelConfig<const N: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy
{
    type Fp_modelConfig: Fp_modelConfig<N>;
    type Fp2_modelConfig: Fp2_modelConfig<N, Fp_modelConfig = Self::Fp_modelConfig>;

    const non_residue: my_Fp<N, Self::Fp_modelConfig>;

    const nqr: (
        my_Fp<N, Self::Fp_modelConfig>,
        my_Fp<N, Self::Fp_modelConfig>,
    );
    const nqr_to_t: (
        my_Fp<N, Self::Fp_modelConfig>,
        my_Fp<N, Self::Fp_modelConfig>,
    );
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Self::Fp_modelConfig>; 2];
    const Frobenius_coeffs_c2: [my_Fp<N, Self::Fp_modelConfig>; 2];
}

type my_Fp<const N: usize, T> = Fp_model<N, T>;
type my_Fp2<const N: usize, T> = Fp2_model<N, T>;

#[derive(Default, Clone, Copy)]
pub struct Fp6_3over2_model<const N: usize, T: Fp6_modelConfig<N>> {
    // // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    //     static i64 add_cnt;
    //     static i64 sub_cnt;
    //     static i64 mul_cnt;
    //     static i64 sqr_cnt;
    //     static i64 inv_cnt;
    // //#endif

    //     static bigint<6*n> euler; // (modulus^6-1)/2
    //     static std::usize s; // modulus^6 = 2^s * t + 1
    //     static bigint<6*n> t; // with t odd
    //     static bigint<6*n> t_minus_1_over_2; // (t-1)/2
    //     static Fp6_3over2_model<n, modulus> nqr; // a quadratic nonresidue in Fp6
    //     static Fp6_3over2_model<n, modulus> nqr_to_t; // nqr^t
    //     static my_Fp2<N,T::Fp2_modelConfig> T::non_residue;
    //     static my_Fp2<N,T::Fp2_modelConfig> Frobenius_coeffs_c1[6]; // T::non_residue^((modulus^i-1)/3)   for i=0,1,2,3,4,5
    //     static my_Fp2<N,T::Fp2_modelConfig> Frobenius_coeffs_c2[6]; // T::non_residue^((2*modulus^i-2)/3) for i=0,1,2,3,4,5
    pub c0: my_Fp2<N, T::Fp2_modelConfig>,
    pub c1: my_Fp2<N, T::Fp2_modelConfig>,
    pub c2: my_Fp2<N, T::Fp2_modelConfig>,
    _t: PhantomData<T>,
    //     Fp6_3over2_model() {};
    //     Fp6_3over2_model(c0:my_Fp2<N,T::Fp2_modelConfig>, c1:my_Fp2<N,T::Fp2_modelConfig>, c2:my_Fp2<N,T::Fp2_modelConfig>)->Selfc0,c1,c2 {};

    //     pub fn  clear() { c0.clear(); c1.clear(); c2.clear(); }
    //     pub fn  print() const { print!("c0/c1/c2:\n"); c0.print(); c1.print(); c2.print(); }
    //     pub fn  randomize();

    //     /**
    //      * Returns the constituent bits in 64 bit words, in little-endian order.
    //      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
    //      */
    //     Vec<u64> to_words() const;
    //     /**
    //      * Sets the field element from the given bits in 64 bit words, in little-endian order.
    //      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
    //      * Returns true when the right-most bits of each element represent a value less than the modulus.
    //      */
    //     bool from_words(Vec<u64> words);

    //     bool is_zero() const { return c0.is_zero() && c1.is_zero() && c2.is_zero(); }
    //     bool operator==(other:&Fp6_3over2_model) const;
    //     bool operator!=(other:&Fp6_3over2_model) const;

    //     Fp6_3over2_model& operator+=(other:&Fp6_3over2_model);
    //     Fp6_3over2_model& operator-=(other:&Fp6_3over2_model);
    //     Fp6_3over2_model& operator*=(other:&Fp6_3over2_model);
    //     Fp6_3over2_model& operator^=(const u64 pow);
    //
    //     Fp6_3over2_model& operator^=(pow:&bigint<m>);

    //     Fp6_3over2_model operator+(other:&Fp6_3over2_model) const;
    //     Fp6_3over2_model operator-(other:&Fp6_3over2_model) const;
    //     Fp6_3over2_model operator*(other:&Fp6_3over2_model) const;
    //     Fp6_3over2_model operator^(const:u64 pow),
    //
    //     Fp6_3over2_model operator^(other:&bigint<m>) const;
    //     Fp6_3over2_model operator-() const;

    //     Fp6_3over2_model& square();
    //     Fp6_3over2_model squared() const;
    //     Fp6_3over2_model& invert();
    //     Fp6_3over2_model inverse() const;
    //     Fp6_3over2_model Frobenius_map(u64 power) const;
    //     Fp6_3over2_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

    //     static my_Fp2<N,T::Fp2_modelConfig> mul_by_non_residue(elt:&my_Fp2<N,T::Fp2_modelConfig>);

    //     static std::usize ceil_size_in_bits() { return 3 * my_Fp2::<N,T::Fp2_modelConfig>::ceil_size_in_bits(); }
    //     static std::usize floor_size_in_bits() { return 3 * my_Fp2::<N,T::Fp2_modelConfig>::floor_size_in_bits(); }

    //     static constexpr std::usize extension_degree() { return 6; }
    //     static constexpr bigint<n> field_char() { return modulus; }

    //     static Fp6_3over2_model<n, modulus> zero();
    //     static Fp6_3over2_model<n, modulus> one();
    //     static Fp6_3over2_model<n, modulus> random_element();

    //     friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp6_3over2_model<n, modulus>);
    //     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp6_3over2_model<n, modulus> &el);
}

// use crate::algebra::field_utils::field_utils;
impl<const N: usize, T: Fp6_modelConfig<N>> Fp6_3over2_model<N, T> {
    pub fn new(
        c0: my_Fp2<N, T::Fp2_modelConfig>,
        c1: my_Fp2<N, T::Fp2_modelConfig>,
        c2: my_Fp2<N, T::Fp2_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            c2,
            _t: PhantomData,
        }
    }

    pub fn mul_by_non_residue(
        elt: &Fp2_model<N, T::Fp2_modelConfig>,
    ) -> Fp2_model<N, T::Fp2_modelConfig> {
        elt * &T::non_residue
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp2::<N, T::Fp2_modelConfig>::one(),
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp2::<N, T::Fp2_modelConfig>::random_element(),
            c1: my_Fp2::<N, T::Fp2_modelConfig>::random_element(),
            c2: my_Fp2::<N, T::Fp2_modelConfig>::random_element(),
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
            s0 + Self::mul_by_non_residue(&s3),
            s1 + Self::mul_by_non_residue(&s4),
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
        /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17 */
        let (a, b, c) = (self.c0, self.c1, self.c2);
        let t0 = a.squared();
        let t1 = b.squared();
        let t2 = c.squared();
        let t3 = a * b;
        let t4 = a * c;
        let t5 = b * c;
        let c0 = t0 - Self::mul_by_non_residue(&t5);
        let c1 = Self::mul_by_non_residue(&t2) - t3;
        let c2 = t1 - t4; // typo in paper referenced above. should be "-" as per, but is "*"
        let t6 = (a * c0 + Self::mul_by_non_residue(&(c * c1 + b * c2))).inverse();
        Self::new(t6 * c0, t6 * c1, t6 * c2)
    }

    pub fn invert(&mut self) -> &Self {
        *self = self.inverse();
        &*self
    }

    pub fn Frobenius_map(&self, power: usize) -> Self {
        Self::new(
            self.c0.Frobenius_map(power),
            &self.c1.Frobenius_map(power) * &T::Frobenius_coeffs_c1[power % 6],
            &self.c2.Frobenius_map(power) * &T::Frobenius_coeffs_c2[power % 6],
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
        self.c0.from_words(&words[0..n])
            && self.c1.from_words(&words[n..n * 2])
            && self.c2.from_words(&words[n * 2..])
    }
}

//
// bool Fp6_3over2_model<n,modulus>::operator==(other:&Fp6_3over2_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1 && self.c2 == other.c2);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> PartialEq for Fp6_3over2_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

//
// bool Fp6_3over2_model<n,modulus>::operator!=(other:&Fp6_3over2_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

//
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator+(other:&Fp6_3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
// //#endif
//     Self::new(self.c0 + other.c0,
//                                        self.c1 + other.c1,
//                                        self.c2 + other.c2);
// }

impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp6_3over2_model<N, T> {
    type Output = Fp6_3over2_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}

//
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator-(other:&Fp6_3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif
//     Self::new(self.c0 - other.c0,
//                                        self.c1 - other.c1,
//                                        self.c2 - other.c2);
// }

impl<const N: usize, T: Fp6_modelConfig<N>> Sub for Fp6_3over2_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> <Fp6_3over2_model<N, T> as Sub>::Output {
        let mut r = self;
        r -= other;
        r
    }
}

//
// Fp6_3over2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp6_3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     Self::new(lhs*rhs.c0,
//                                        lhs*rhs.c1,
//                                        lhs*rhs.c2);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> Mul<&Fp_model<N, T::Fp_modelConfig>>
    for &Fp6_3over2_model<N, T>
{
    type Output = Fp6_3over2_model<N, T>;

    fn mul(self, rhs: &Fp_model<N, T::Fp_modelConfig>) -> Self::Output {
        Fp6_3over2_model::<N, T>::new(&self.c0 * rhs, &self.c1 * rhs, &self.c2 * rhs)
    }
}

//
// Fp6_3over2_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp6_3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     Self::new(lhs*rhs.c0,
//                                        lhs*rhs.c1,
//                                        lhs*rhs.c2);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> Mul<&Fp2_model<N, T::Fp2_modelConfig>>
    for &Fp6_3over2_model<N, T>
{
    type Output = Fp6_3over2_model<N, T>;

    fn mul(self, rhs: &Fp2_model<N, T::Fp2_modelConfig>) -> Self::Output {
        Fp6_3over2_model::<N, T>::new(self.c0 * rhs, self.c1 * rhs, self.c2 * rhs)
    }
}

//
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator*(other:&Fp6_3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (Karatsuba) */
//     A:&my_Fp2<N,T::Fp2_modelConfig> = other.c0, &B = other.c1, &C = other.c2,
//                  &a = self.c0, &b = self.c1, &c = self.c2;
//     let aA= a*A;
//     let bB= b*B;
//     let cC= c*C;

//     Self::new(aA + Fp6_3over2_model<n,modulus>::mul_by_non_residue((b+c)*(B+C)-bB-cC),
//                                        (a+b)*(A+B)-aA-bB+Fp6_3over2_model<n,modulus>::mul_by_non_residue(cC),
//                                        (a+c)*(A+C)-aA+bB-cC);
// }

impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> Mul<O> for Fp6_3over2_model<N, T> {
    type Output = Fp6_3over2_model<N, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= *rhs.borrow();
        r
    }
}

//
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator-() const
// {
//     Self::new(-self.c0,
//                                        -self.c1,
//                                        -self.c2);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> Neg for Fp6_3over2_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        // mpn_sub_n(r.mont_repr.0.0, modulus.0.0, self.mont_repr.0.0, n);
        r
    }
}
//
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp6_3over2_model<n, modulus> >(*this, pow);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> BitXor<u64> for Fp6_3over2_model<N, T> {
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
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp6_3over2_model<n, modulus>, m>(*this, pow);
// }
impl<const N: usize, const M: usize, T: Fp6_modelConfig<N>> BitXor<&bigint<M>>
    for Fp6_3over2_model<N, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<M>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}
//
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator+=(const Fp6_3over2_model<n,modulus>& other)
// {
//     *self = *this + other;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> AddAssign<O>
    for Fp6_3over2_model<N, T>
{
    fn add_assign(&mut self, other: O) {}
}

//
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator-=(const Fp6_3over2_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> SubAssign<O>
    for Fp6_3over2_model<N, T>
{
    fn sub_assign(&mut self, other: O) {}
}
//
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator*=(const Fp6_3over2_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> MulAssign<O>
    for Fp6_3over2_model<N, T>
{
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
    }
}
//
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>> BitXorAssign<u64> for Fp6_3over2_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        // *self = Powers::power::<Fp6_3over2_model<N, T>>(self, rhs);
    }
}

//
//
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, const M: usize, T: Fp6_modelConfig<N>> BitXorAssign<&bigint<M>>
    for Fp6_3over2_model<N, T>
{
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        //*self = Powers::power::<Fp6_3over2_model<N, T>>(self, rhs);
    }
}

//
// std::ostream& operator<<(std::ostream &out, el:&Fp6_3over2_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1 << OUTPUT_SEPARATOR << el.c2;
//     return out;
// }

//
// std::istream& operator>>(std::istream &in, Fp6_3over2_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1 >> el.c2;
//     return in;
// }

//
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp6_3over2_model<n, modulus> >)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

use std::fmt;
impl<const N: usize, T: Fp6_modelConfig<N>> fmt::Display for Fp6_3over2_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c0)
    }
}
//
// std::istream& operator>>(std::istream& in, Vec<Fp6_3over2_model<n, modulus> > &v)
// {
//     v.clear();

//     usize s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     v.reserve(s);

//     for i in 0..s
//     {
//         Fp6_3over2_model<n, modulus> el;
//         in >> el;
//         v.emplace_back(el);
//     }

//     return in;
// }

use super::cubic_extension::{CubicExtConfig, CubicExtField};
use crate::algebra::fields::{
    cyclotomic::CyclotomicMultSubgroup,
    prime_extension::fp2::{Fp2, Fp2Config},
};
//  use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use ark_std::Zero;
use core::marker::PhantomData;

pub trait Fp6Config: 'static + Send + Sync + Copy {
    type Fp2Config: Fp2Config;

    const NONRESIDUE: Fp2<Self::Fp2Config>;

    /// Determines the algorithm for computing square roots.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp6<Self>>> = None;

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP6_C1: &'static [Fp2<Self::Fp2Config>];
    const FROBENIUS_COEFF_FP6_C2: &'static [Fp2<Self::Fp2Config>];

    #[inline(always)]
    fn mul_fp2_by_nonresidue_in_place(fe: &mut Fp2<Self::Fp2Config>) -> &mut Fp2<Self::Fp2Config> {
        *fe *= &Self::NONRESIDUE;
        fe
    }
    #[inline(always)]
    fn mul_fp2_by_nonresidue(mut fe: Fp2<Self::Fp2Config>) -> Fp2<Self::Fp2Config> {
        Self::mul_fp2_by_nonresidue_in_place(&mut fe);
        fe
    }
}

pub struct Fp6ConfigWrapper<P: Fp6Config>(PhantomData<P>);

impl<P: Fp6Config> CubicExtConfig for Fp6ConfigWrapper<P> {
    type BasePrimeField = <P::Fp2Config as Fp2Config>::Fp;
    type BaseField = Fp2<P::Fp2Config>;
    type FrobCoeff = Fp2<P::Fp2Config>;

    const SQRT_PRECOMP: Option<SqrtPrecomputation<CubicExtField<Self>>> = P::SQRT_PRECOMP;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 6;

    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const FROBENIUS_COEFF_C1: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP6_C1;
    const FROBENIUS_COEFF_C2: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP6_C2;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp2_by_nonresidue_in_place(fe)
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

pub type Fp6<P> = CubicExtField<Fp6ConfigWrapper<P>>;

impl<P: Fp6Config> Fp6<P> {
    pub fn mul_assign_by_fp2(&mut self, other: Fp2<P::Fp2Config>) {
        self.c0 *= &other;
        self.c1 *= &other;
        self.c2 *= &other;
    }

    pub fn mul_by_fp(&mut self, element: &<P::Fp2Config as Fp2Config>::Fp) {
        self.c0.mul_assign_by_fp(element);
        self.c1.mul_assign_by_fp(element);
        self.c2.mul_assign_by_fp(element);
    }

    pub fn mul_by_fp2(&mut self, element: &Fp2<P::Fp2Config>) {
        self.c0 *= element;
        self.c1 *= element;
        self.c2 *= element;
    }

    pub fn mul_by_1(&mut self, c1: &Fp2<P::Fp2Config>) {
        let mut b_b = self.c1;
        b_b *= c1;

        let mut t1 = *c1;
        {
            let mut tmp = self.c1;
            tmp += self.c2;

            t1 *= &tmp;
            t1 -= &b_b;
            P::mul_fp2_by_nonresidue_in_place(&mut t1);
        }

        let mut t2 = *c1;
        {
            let mut tmp = self.c0;
            tmp += &self.c1;

            t2 *= &tmp;
            t2 -= &b_b;
        }

        self.c0 = t1;
        self.c1 = t2;
        self.c2 = b_b;
    }

    pub fn mul_by_01(&mut self, c0: &Fp2<P::Fp2Config>, c1: &Fp2<P::Fp2Config>) {
        let mut a_a = self.c0;
        let mut b_b = self.c1;
        a_a *= c0;
        b_b *= c1;

        let mut t1 = *c1;
        {
            let mut tmp = self.c1;
            tmp += self.c2;

            t1 *= &tmp;
            t1 -= &b_b;
            P::mul_fp2_by_nonresidue_in_place(&mut t1);
            t1 += &a_a;
        }

        let mut t3 = *c0;
        {
            let mut tmp = self.c0;
            tmp += self.c2;

            t3 *= &tmp;
            t3 -= &a_a;
            t3 += &b_b;
        }

        let mut t2 = *c0;
        t2 += c1;
        {
            let mut tmp = self.c0;
            tmp += &self.c1;

            t2 *= &tmp;
            t2 -= &a_a;
            t2 -= &b_b;
        }

        self.c0 = t1;
        self.c1 = t2;
        self.c2 = t3;
    }
}

// We just use the default algorithms; there don't seem to be any faster ones.
impl<P: Fp6Config> CyclotomicMultSubgroup for Fp6<P> {}
