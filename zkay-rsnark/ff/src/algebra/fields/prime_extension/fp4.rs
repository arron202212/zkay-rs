//  Declaration of interfaces for the (extension) field Fp4.

//  The field Fp4 equals Fp2[V]/(V^2-U) where Fp2 = Fp[U]/(U^2-T::non_residue) and T::non_residue is in Fp.

//  ASSUMPTION: the modulus p is 1 mod 6.

use crate::{
    Fp_model, Fp_modelConfig as FpmConfig, Fp2_model, Fp2_modelConfig, PpConfig,
    algebra::{
        field_utils::{
            BigInteger,
            algorithms::{
                FPMConfig, FieldTForPowersConfig, PowerConfig, Powers, tonelli_shanks_sqrt,
            },
            bigint::{GMP_NUMB_BITS, bigint},
            field_utils, fp_aux, {BigInt, algorithms},
        },
        fields::{
            field::{AdditiveGroup, Field},
            fpn_field::PrimeField,
            sqrt::SqrtPrecomputation,
        },
    },
    scalar_multiplication::wnaf::find_wnaf,
};
use num_traits::{One, Zero};
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

type Fp_modelConfig<const N: usize, const N2: usize, const N4: usize, T> =
    <<T as Fp4_modelConfig<N, N2, N4>>::Fp2_modelConfig as Fp2_modelConfig<N, N2>>::Fp_modelConfig;
pub trait Fp4_modelConfig<const N: usize, const N2: usize, const N4: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq + Debug
{
    type Fp2_modelConfig: Fp2_modelConfig<N, N2>;
    const euler: bigint<N4> = bigint::<N4>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N4> = bigint::<N4>::one(); // with t odd
    const t_minus_1_over_2: bigint<N4> = bigint::<N4>::one(); // (t-1)/2
    const non_residue: my_Fp<N, Fp_modelConfig<N, N2, N4, Self>> =
        Fp_model::<N, Fp_modelConfig<N, N2, N4, Self>>::const_default();

    const nqr: Fp4_model<N, N2, N4, Self> = Fp4_model::<N, N2, N4, Self>::const_default();
    const nqr_to_t: Fp4_model<N, N2, N4, Self> = Fp4_model::<N, N2, N4, Self>::const_default();
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Fp_modelConfig<N, N2, N4, Self>>; 4] =
        [Fp_model::<N, Fp_modelConfig<N, N2, N4, Self>>::const_default(); 4];
}
type my_Fp<const N: usize, T> = Fp_model<N, T>;
pub type my_Fp2<const N: usize, const N2: usize, T> = Fp2_model<N, N2, T>;
pub type my_Fpe<const N: usize, const N2: usize, T> = my_Fp2<N, N2, T>;

#[derive(Default, Clone, Debug, Copy, Eq)]
pub struct Fp4_model<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
> {
    pub c0: my_Fp2<N, N2, T::Fp2_modelConfig>,
    pub c1: my_Fp2<N, N2, T::Fp2_modelConfig>,
    _t: PhantomData<T>,
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> FPMConfig
    for Fp4_model<N, N2, N4, T>
{
}
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    FieldTForPowersConfig<N4> for Fp4_model<N, N2, N4, T>
{
    type FPM = Self;
    const num_limbs: usize = N;
    const s: usize = T::s; // modulus = 2^s * t + 1
    const t: bigint<N4> = T::t; // with t odd
    const t_minus_1_over_2: bigint<N4> = T::t_minus_1_over_2; // (t-1)/2
    const nqr: Self = T::nqr; // a quadratic nonresidue
    const nqr_to_t: Self = T::nqr_to_t; // nqr^t
    fn squared_(&self) -> Self {
        self.squared()
    }
}
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    Fp4_model<N, N2, N4, T>
{
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

    pub fn ceil_size_in_bits() -> usize {
        2 * my_Fp2::<N, N2, T::Fp2_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        2 * my_Fp2::<N, N2, T::Fp2_modelConfig>::floor_size_in_bits()
    }
    pub fn extension_degree() -> usize {
        4
    }
    pub fn field_char() -> bigint<N> {
        <<T::Fp2_modelConfig as Fp2_modelConfig<N, N2>>::Fp_modelConfig as FpmConfig<N>>::modulus
    }
    pub fn new(
        c0: my_Fp2<N, N2, T::Fp2_modelConfig>,
        c1: my_Fp2<N, N2, T::Fp2_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }
    pub const fn const_default() -> Self {
        Self {
            c0: my_Fp2::<N, N2, T::Fp2_modelConfig>::const_default(),
            c1: my_Fp2::<N, N2, T::Fp2_modelConfig>::const_default(),
            _t: PhantomData,
        }
    }
    pub fn mul_by_non_residue(
        elt: &my_Fp2<N, N2, T::Fp2_modelConfig>,
    ) -> my_Fp2<N, N2, T::Fp2_modelConfig> {
        my_Fp2::<N, N2, T::Fp2_modelConfig>::new(T::non_residue * elt.c1, elt.c0)
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp2::<N, N2, T::Fp2_modelConfig>::one(),
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp2::<N, N2, T::Fp2_modelConfig>::random_element(),
            c1: my_Fp2::<N, N2, T::Fp2_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn mul_by_023(&self, other: &Self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.mul_cnt++;

        //Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba)
        assert!(other.c0.c1.is_zero());

        let (A, B) = (other.c0, other.c1);
        let (a, b) = (self.c0, self.c1);
        let aA = my_Fp2::<N, N2, T::Fp2_modelConfig>::new(a.c0 * A.c0, a.c1 * A.c0);
        let bB = b * B;

        let beta_bB = Self::mul_by_non_residue(&bB);
        Self::new(aA + beta_bB, (a + b) * (A + B) - aA - bB)
    }

    pub fn squared(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.sqr_cnt++;

        //Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex)
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

        //From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8
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
            self.c1.Frobenius_map(power) * T::Frobenius_coeffs_c1[power % 4],
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

        for i in (0..NAF.len()).rev() {
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

    pub fn sqrt(&self) -> Option<Self> {
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
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> PartialEq
    for Fp4_model<N, N2, N4, T>
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.c0 == other.c0 && self.c1 == other.c1
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
    O: Borrow<Self>,
> AddAssign<O> for Fp4_model<N, N2, N4, T>
{
    fn add_assign(&mut self, other: O) {
        *self = *self + other.borrow();
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
    O: Borrow<Self>,
> SubAssign<O> for Fp4_model<N, N2, N4, T>
{
    fn sub_assign(&mut self, other: O) {
        *self = *self - *other.borrow();
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
    O: Borrow<Self>,
> MulAssign<O> for Fp4_model<N, N2, N4, T>
{
    fn mul_assign(&mut self, other: O) {
        *self = *self * other.borrow();
    }
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    BitXorAssign<u64> for Fp4_model<N, N2, N4, T>
{
    fn bitxor_assign(&mut self, other: u64) {
        *self = *self ^ other;
    }
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    BitXorAssign<bigint<N4>> for Fp4_model<N, N2, N4, T>
{
    fn bitxor_assign(&mut self, other: bigint<N4>) {
        *self = *self ^ other;
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
    O: Borrow<Self>,
> Add<O> for Fp4_model<N, N2, N4, T>
{
    type Output = Fp4_model<N, N2, N4, T>;

    fn add(self, other: O) -> Self::Output {
        Self::new(self.c0 + other.borrow().c0, self.c1 + other.borrow().c1)
    }
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> Sub
    for Fp4_model<N, N2, N4, T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.c0 - other.borrow().c0, self.c1 - other.borrow().c1)
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
    TC: FpmConfig<N>,
> Mul<Fp_model<N, TC>> for Fp4_model<N, N2, N4, T>
{
    type Output = Fp4_model<N, N2, N4, T>;

    fn mul(self, rhs: Fp_model<N, TC>) -> Self::Output {
        Self::new(self.c0 * rhs, self.c1 * rhs)
    }
}
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    Mul<Fp2_model<N, N2, <T as Fp4_modelConfig<N, N2, N4>>::Fp2_modelConfig>>
    for Fp4_model<N, N2, N4, T>
{
    type Output = Fp4_model<N, N2, N4, T>;

    fn mul(
        self,
        rhs: Fp2_model<N, N2, <T as Fp4_modelConfig<N, N2, N4>>::Fp2_modelConfig>,
    ) -> Self::Output {
        Self::new(self.c0 * rhs, self.c1 * rhs)
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N4: usize,
    T: Fp4_modelConfig<N, N2, N4>,
    O: Borrow<Self>,
> Mul<O> for Fp4_model<N, N2, N4, T>
{
    type Output = Fp4_model<N, N2, N4, T>;

    fn mul(self, other: O) -> Self::Output {
        let (A, B) = (other.borrow().c0, other.borrow().c1);
        let (a, b) = (self.c0, self.c1);
        let aA = a * A;
        let bB = b * B;

        let beta_bB = Self::mul_by_non_residue(&bB);
        Self::new(aA + beta_bB, (a + b) * (A + B) - aA - bB)
    }
}
// impl<const N: usize, const N2: usize,const N4: usize, T: Fp4_modelConfig<N, N2,N4>> Mul<bigint<N>>
//     for Fp4_model<N, N2, N4,T>
// {
//     type Output = Self;

//     fn mul(self, rhs: bigint<N>) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }
//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp4_model<n, modulus> >(*this, pow);
// }
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> BitXor<u64>
    for Fp4_model<N, N2, N4, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        Powers::power::<Fp4_model<N, N2, N4, T>>(&self, rhs)
    }
}
//
//
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(exponent:&bigint<m>) const
// {
//     return power<Fp4_model<n, modulus> >(*this, exponent);
// }

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    BitXor<bigint<N4>> for Fp4_model<N, N2, N4, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N4>) -> Self::Output {
        Powers::power::<Fp4_model<N, N2, N4, T>>(&self, rhs)
    }
}
//
//
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(exponent:&Fp4_model<m, modulus_p>) const
// {
//     return *self^(exponent.as_bigint());
// }
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> PpConfig
    for Fp4_model<N, N2, N4, T>
{
    
    type BigIntT = bigint<N>;
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> One
    for Fp4_model<N, N2, N4, T>
{
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> Zero
    for Fp4_model<N, N2, N4, T>
{
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}
//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-() const
// {
//     Self::new(-self.c0,
//                                 -self.c1);
// }

impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> Neg
    for Fp4_model<N, N2, N4, T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.c0, -self.c1)
    }
}

use std::fmt;
use std::io::{self, Read};

// 对应: std::ostream& operator<<(std::ostream &out, const Fp2_model<n, modulus> &el)
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> fmt::Display
    for Fp4_model<N, N2, N4, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // OUTPUT_SEPARATOR 在 Rust 中通常直接用空格或指定的 separator
        write!(f, "{} {}", self.c0, self.c1)
    }
}

// 对应: std::istream& operator>>(std::istream &in, Fp2_model<n, modulus> &el)
// Rust 中通常通过自定义函数或实现特定 Trait 来处理流输入
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>>
    Fp4_model<N, N2, N4, T>
{
    pub fn read<R: io::BufRead>(reader: &mut R) -> io::Result<Self> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        // 解析 c0 和 c1
        let c0 = parts[0]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "c0 parse error"))?;
        let c1 = parts[1]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "c1 parse error"))?;
        Ok(Fp4_model::<N, N2, N4, T>::new(c0, c1))
    }
}
impl<const N: usize, const N2: usize, const N4: usize, T: Fp4_modelConfig<N, N2, N4>> FromStr
    for Fp4_model<N, N2, N4, T>
{
    type Err = ();

    /// Interpret a string of numbers as a (congruent) prime field element.
    /// Does not accept unnecessary leading zeroes or a blank string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // use num_bigint::{BigInt, BigUint};
        // use num_traits::Signed;

        // let modulus = BigInt::from(P::MODULUS);
        // let mut a = BigInt::from_str(s).map_err(|_| ())? % &modulus;
        // if a.is_negative() {
        //     a += modulus
        // }
        // BigUint::try_from(a)
        //     .map_err(|_| ())
        //     .and_then(TryFrom::try_from)
        //     .ok()
        //     .and_then(Self::from_bigint)
        //     .ok_or(())
        Ok(Self::default())
    }
}

//
// std::ostream& operator<<(std::ostream &out, el:&Fp4_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }
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
