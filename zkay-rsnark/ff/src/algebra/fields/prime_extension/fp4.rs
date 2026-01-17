//  Declaration of interfaces for the (extension) field Fp4.

//  The field Fp4 equals Fp2[V]/(V^2-U) where Fp2 = Fp[U]/(U^2-T::non_residue) and T::non_residue is in Fp.

//  ASSUMPTION: the modulus p is 1 mod 6.

// use crate::algebra::fields::prime_base::fp;
// use crate::algebra::fields::prime_extension::fp2;
use crate::Fp_model;
use crate::Fp_modelConfig as FpmConfig;
use crate::Fp2_model;
use crate::Fp2_modelConfig;
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
use crate::scalar_multiplication::wnaf::find_wnaf;
use num_traits::{One, Zero};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};
type Fp_modelConfig<const N: usize, T> =
    <<T as Fp4_modelConfig<N>>::Fp2_modelConfig as Fp2_modelConfig<N>>::Fp_modelConfig;
pub trait Fp4_modelConfig<const N: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq + Debug
{
    type Fp2_modelConfig: Fp2_modelConfig<N>;

    const non_residue: my_Fp<N, Fp_modelConfig<N, Self>> =
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>();

    const nqr: (
        my_Fp<N, Fp_modelConfig<N, Self>>,
        my_Fp<N, Fp_modelConfig<N, Self>>,
    ) = (
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>(),
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>(),
    );
    const nqr_to_t: (
        my_Fp<N, Fp_modelConfig<N, Self>>,
        my_Fp<N, Fp_modelConfig<N, Self>>,
    ) = (
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>(),
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>(),
    );
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Fp_modelConfig<N, Self>>; 2] = [
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>(),
        const_new_fp_model::<N, Fp_modelConfig<N, Self>>(),
    ];
}
type my_Fp<const N: usize, T> = Fp_model<N, T>;
pub type my_Fp2<const N: usize, T> = Fp2_model<N, T>;
pub type my_Fpe<const N: usize, T> = my_Fp2<N, T>;

#[derive(Default, Clone, Debug, Copy, Eq)]
pub struct Fp4_model<const N: usize, T: Fp4_modelConfig<N>> {
    // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    // static i64 add_cnt;
    // static i64 sub_cnt;
    // static i64 mul_cnt;
    // static i64 sqr_cnt;
    // static i64 inv_cnt;
    //#endif

    // static bigint<4*n> euler; // (modulus^4-1)/2
    // static std::usize s; // modulus^4 = 2^s * t + 1
    // static bigint<4*n> t; // with t odd
    // static bigint<4*n> t_minus_1_over_2; // (t-1)/2
    // static Fp4_model<n, modulus> nqr; // a quadratic nonresidue in Fp4
    // static Fp4_model<n, modulus> nqr_to_t; // nqr^t
    // static my_Fp T::non_residue;
    // static my_Fp Frobenius_coeffs_c1[4]; // T::non_residue^((modulus^i-1)/4) for i=0,1,2,3
    pub c0: my_Fp2<N, T::Fp2_modelConfig>,
    pub c1: my_Fp2<N, T::Fp2_modelConfig>,
    _t: PhantomData<T>,
    // Fp4_model() {};
    // Fp4_model(c0:my_Fp2, c1:my_Fp2)->Selfc0,c1 {};

    // pub fn  print() const { print!("c0/c1:\n"); c0.print(); c1.print(); }
    // pub fn  clear() { c0.clear(); c1.clear(); }
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
    // bool operator==(other:&Fp4_model) const;
    // bool operator!=(other:&Fp4_model) const;

    // Fp4_model& operator+=(other:&Fp4_model);
    // Fp4_model& operator-=(other:&Fp4_model);
    // Fp4_model& operator*=(other:&Fp4_model);
    // Fp4_model& operator^=(const u64 pow);

    // Fp4_model& operator^=(pow:&bigint<m>);

    // Fp4_model operator+(other:&Fp4_model) const;
    // Fp4_model operator-(other:&Fp4_model) const;
    // Fp4_model operator*(other:&Fp4_model) const;
    // Fp4_model mul_by_023(other:&Fp4_model) const;
    // Fp4_model operator^(const:u64 pow),

    // Fp4_model operator^(exponent:&bigint<m>) const;

    // Fp4_model operator^(exponent:&Fp_model<m, modulus_p>) const;
    // Fp4_model operator-() const;

    // Fp4_model& square();
    // Fp4_model squared() const;
    // Fp4_model& invert();
    // Fp4_model inverse() const;
    // Fp4_model Frobenius_map(u64 power) const;
    // Fp4_model unitary_inverse() const;
    // Fp4_model cyclotomic_squared() const;
    // Fp4_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

    // static my_Fp2 mul_by_non_residue(elt:&my_Fp2);

    // Fp4_model cyclotomic_exp(exponent:&bigint<m>) const;

    // static std::usize ceil_size_in_bits() { return 2 * my_Fp2::ceil_size_in_bits(); }
    // static std::usize floor_size_in_bits() { return 2 * my_Fp2::floor_size_in_bits(); }

    // static constexpr std::usize extension_degree() { return 4; }
    // static constexpr bigint<n> field_char() { return modulus; }

    // static Fp4_model<n, modulus> zero();
    // static Fp4_model<n, modulus> one();
    // static Fp4_model<n, modulus> random_element();

    // friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp4_model<n, modulus>);
    // friend std::istream& operator>> <n, modulus>(std::istream &in, Fp4_model<n, modulus> &el);
}

// use crate::algebra::field_utils::field_utils;
// use crate::algebra::scalar_multiplication::wnaf;

impl<const N: usize, T: Fp4_modelConfig<N>> Fp4_model<N, T> {
    pub fn ceil_size_in_bits() -> usize {
        2 * my_Fp2::<N, T::Fp2_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        2 * my_Fp2::<N, T::Fp2_modelConfig>::floor_size_in_bits()
    }
    pub fn new(c0: my_Fp2<N, T::Fp2_modelConfig>, c1: my_Fp2<N, T::Fp2_modelConfig>) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }
    pub fn mul_by_non_residue(
        elt: &my_Fp2<N, T::Fp2_modelConfig>,
    ) -> my_Fp2<N, T::Fp2_modelConfig> {
        my_Fp2::<N, T::Fp2_modelConfig>::new(T::non_residue * elt.c1, elt.c0)
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp2::<N, T::Fp2_modelConfig>::one(),
            my_Fp2::<N, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp2::<N, T::Fp2_modelConfig>::random_element(),
            c1: my_Fp2::<N, T::Fp2_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn mul_by_023(&self, other: &Self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.mul_cnt++;
        //#endif
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
        assert!(other.c0.c1.is_zero());

        let (A, B) = (other.c0, other.c1);
        let (a, b) = (self.c0, self.c1);
        let aA = my_Fp2::<N, T::Fp2_modelConfig>::new(a.c0 * A.c0, a.c1 * A.c0);
        let bB = b * B;

        let beta_bB = Self::mul_by_non_residue(&bB);
        Self::new(aA + beta_bB, (a + b) * (A + B) - aA - bB)
    }

    pub fn squared(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.sqr_cnt++;
        //#endif
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex) */
        let (a, b) = (self.c0, self.c1);
        let ab = a * b;

        Self::new(
            (a + b) * (a + Self::mul_by_non_residue(&b)) - ab - Self::mul_by_non_residue(&ab),
            ab + ab,
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
        /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */
        let (a, b) = (self.c0, self.c1);
        let t1 = b.squared();
        let t0 = a.squared() - Self::mul_by_non_residue(&t1);
        let new_t1 = t0.inverse();

        Self::new(a * new_t1, -(b * new_t1))
    }

    pub fn invert(&mut self) -> &Self {
        *self = self.inverse();
        &*self
    }

    pub fn Frobenius_map(&self, power: usize) -> Self {
        Self::new(
            self.c0.Frobenius_map(power),
            &self.c1.Frobenius_map(power) * &T::Frobenius_coeffs_c1[power % 4],
        )
    }

    pub fn unitary_inverse(&self) -> Self {
        Self::new(self.c0, -self.c1)
    }

    pub fn cyclotomic_squared(&self) -> Self {
        let A = self.c1.squared();
        let B = self.c1 + self.c0;
        let C = B.squared() - A;
        let D = Self::mul_by_non_residue(&A); // Fp2(A.c1 *, A.c0)
        let E = C - D;
        let F = D + D + my_Fp2::one();
        let G = E - my_Fp2::one();

        Self::new(F, G)
    }

    pub fn cyclotomic_exp(&self, exponent: &bigint<N>) -> Self {
        let mut res = Self::one();
        let this_inverse = self.unitary_inverse();

        let mut found_nonzero = false;
        let NAF = find_wnaf(1, &exponent.0);

        for i in (0..=(NAF.len() - 1)).rev() {
            if found_nonzero {
                res = res.cyclotomic_squared();
            }

            if NAF[i] != 0 {
                found_nonzero = true;

                if NAF[i] > 0 {
                    res = res * *self;
                } else {
                    res = res * this_inverse;
                }
            }
        }

        res
    }

    pub fn sqrt(&self) -> Self {
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
        self.c0.from_words(&words[..n]) && self.c1.from_words(&words[n..])
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
        <<T::Fp2_modelConfig as Fp2_modelConfig<N>>::Fp_modelConfig as FpmConfig<N>>::modulus
    }
}

//
// std::ostream& operator<<(std::ostream &out, el:&Fp4_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

//
// bool Fp4_model<n,modulus>::operator==(other:&Fp4_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1);
// }

//
// bool Fp4_model<n,modulus>::operator!=(other:&Fp4_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

impl<const N: usize, T: Fp4_modelConfig<N>> PartialEq for Fp4_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator+=(const Fp4_model<n,modulus>& other)
// {
//     *self = *this + other;
//     return *self;
// }

impl<const N: usize, T: Fp4_modelConfig<N>, O: Borrow<Self>> AddAssign<O> for Fp4_model<N, T> {
    fn add_assign(&mut self, other: O) {}
}
//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator-=(const Fp4_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }

impl<const N: usize, T: Fp4_modelConfig<N>, O: Borrow<Self>> SubAssign<O> for Fp4_model<N, T> {
    fn sub_assign(&mut self, other: O) {}
}
//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator*=(const Fp4_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }
impl<const N: usize, T: Fp4_modelConfig<N>, O: Borrow<Self>> MulAssign<O> for Fp4_model<N, T> {
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
    }
}

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, T: Fp4_modelConfig<N>> BitXorAssign<u64> for Fp4_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        // *self = Powers::power::<Fp4_model<N, T>>(self, rhs);
    }
}
//
//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }

impl<const N: usize, const M: usize, T: Fp4_modelConfig<N>> BitXorAssign<&bigint<M>>
    for Fp4_model<N, T>
{
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        //*self = Powers::power::<Fp4_model<N, T>>(self, rhs);
    }
}

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator+(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
// //#endif
//     Self::new(self.c0 + other.c0,
//                                 self.c1 + other.c1);
// }

impl<const N: usize, T: Fp4_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp4_model<N, T> {
    type Output = Fp4_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}
//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif
//     Self::new(self.c0 - other.c0,
//                                 self.c1 - other.c1);
// }
impl<const N: usize, T: Fp4_modelConfig<N>> Sub for Fp4_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        r -= other;
        r
    }
}

//
// Fp4_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp4_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     Self::new(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator*(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     B:&my_Fp2 = other.c1, &A = other.c0,
//         &b = self.c1, &a = self.c0;
//     let aA= a*A;
//     let bB= b*B;

//     let beta_bB= Fp4_model<n,modulus>::mul_by_non_residue(bB);
//     Self::new(aA + beta_bB,
//                                 (a+b)*(A+B) - aA  - bB);
// }

//
// Fp4_model<n, modulus> operator*(lhs:&Fp4_model<n, modulus>, rhs:&Fp4_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     Self::new(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

impl<const N: usize, T: Fp4_modelConfig<N>, O: Borrow<Self>> Mul<O> for Fp4_model<N, T> {
    type Output = Fp4_model<N, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= *rhs.borrow();
        r
    }
}
//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp4_model<n, modulus> >(*this, pow);
// }
impl<const N: usize, T: Fp4_modelConfig<N>> BitXor<u64> for Fp4_model<N, T> {
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
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(exponent:&bigint<m>) const
// {
//     return power<Fp4_model<n, modulus> >(*this, exponent);
// }

impl<const N: usize, const M: usize, T: Fp4_modelConfig<N>> BitXor<&bigint<M>> for Fp4_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<M>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}
//
//
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(exponent:&Fp4_model<m, modulus_p>) const
// {
//     return *self^(exponent.as_bigint());
// }
impl<const N: usize, T: Fp4_modelConfig<N>> PpConfig for Fp4_model<N, T>
where
    <<T as Fp4_modelConfig<N>>::Fp2_modelConfig as Fp2_modelConfig<N>>::Fp_modelConfig: PpConfig,
{
    type TT = bigint<N>;
    //  type Fr=<T::Fp2_modelConfig as Fp2_modelConfig<N>>::Fp_modelConfig;
}

impl<const N: usize, T: Fp4_modelConfig<N>> Mul<bigint<N>> for Fp4_model<N, T> {
    type Output = Self;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp4_modelConfig<N>> One for Fp4_model<N, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, T: Fp4_modelConfig<N>> Zero for Fp4_model<N, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-() const
// {
//     Self::new(-self.c0,
//                                 -self.c1);
// }

impl<const N: usize, T: Fp4_modelConfig<N>> Neg for Fp4_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        // mpn_sub_n(r.mont_repr.0.0, modulus.0.0, self.mont_repr.0.0, n);
        r
    }
}

use std::fmt;
impl<const N: usize, T: Fp4_modelConfig<N>> fmt::Display for Fp4_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c0)
    }
}
//
// std::istream& operator>>(std::istream &in, Fp4_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1;
//     return in;
// }

use super::quadratic_extension::{QuadExtConfig, QuadExtField};
use crate::algebra::fields::{
    cyclotomic::CyclotomicMultSubgroup,
    prime_extension::fp2::{Fp2, Fp2Config},
};
// use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use core::{marker::PhantomData, ops::Not};

pub trait Fp4Config: 'static + Send + Sync {
    type Fp2Config: Fp2Config;

    /// This *must* equal (0, 1);
    /// see [[DESD06, Section 5.1]](https://eprint.iacr.org/2006/471.pdf).
    const NONRESIDUE: Fp2<Self::Fp2Config>;

    /// Coefficients for the Frobenius automorphism.
    /// T::non_residue^((modulus^i-1)/4) for i=0,1,2,3
    const FROBENIUS_COEFF_FP4_C1: &'static [<Self::Fp2Config as Fp2Config>::Fp];

    #[inline(always)]
    fn mul_fp2_by_nonresidue_in_place(fe: &mut Fp2<Self::Fp2Config>) -> &mut Fp2<Self::Fp2Config> {
        // see [[DESD06, Section 5.1]](https://eprint.iacr.org/2006/471.pdf).
        let new_c1 = fe.c0;
        Self::Fp2Config::mul_fp_by_nonresidue_in_place(&mut fe.c1);
        fe.c0 = fe.c1;
        fe.c1 = new_c1;
        fe
    }
}

pub struct Fp4ConfigWrapper<P: Fp4Config>(PhantomData<P>);

impl<P: Fp4Config> QuadExtConfig for Fp4ConfigWrapper<P> {
    type BasePrimeField = <P::Fp2Config as Fp2Config>::Fp;
    type BaseField = Fp2<P::Fp2Config>;
    type FrobCoeff = Self::BasePrimeField;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 4;

    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const FROBENIUS_COEFF_C1: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP4_C1;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp2_by_nonresidue_in_place(fe)
    }

    fn mul_base_field_by_frob_coeff(fe: &mut Self::BaseField, power: usize) {
        fe.mul_assign_by_fp(&Self::FROBENIUS_COEFF_C1[power % Self::DEGREE_OVER_BASE_PRIME_FIELD]);
    }
}

pub type Fp4<P> = QuadExtField<Fp4ConfigWrapper<P>>;

impl<P: Fp4Config> Fp4<P> {
    pub fn mul_by_fp(&mut self, element: &<P::Fp2Config as Fp2Config>::Fp) {
        self.c0.mul_assign_by_fp(element);
        self.c1.mul_assign_by_fp(element);
    }

    pub fn mul_by_fp2(&mut self, element: &Fp2<P::Fp2Config>) {
        self.c0 *= element;
        self.c1 *= element;
    }
}

impl<P: Fp4Config> CyclotomicMultSubgroup for Fp4<P> {
    const INVERSE_IS_FAST: bool = true;
    fn cyclotomic_inverse_in_place(&mut self) -> Option<&mut Self> {
        self.is_zero().not().then(|| {
            self.conjugate_in_place();
            self
        })
    }
}
