//  Declaration of arithmetic in the finite field F[(p^2)^3]

use crate::{
    Fp_model, Fp_modelConfig, Fp2_model, Fp2_modelConfig, PpConfig,
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
};

use num_traits::{One, Zero};
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

//  * Arithmetic in the finite field F[(p^2)^3].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  *  Fp6 = Fp2[V]/(V^3-T::non_residue) where T::non_residue is in Fp.
//  *
//  * ASSUMPTION: p = 1 (mod 6)

pub trait Fp6_modelConfig<const N: usize, const N2: usize, const N6: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq + Debug
{
    type Fp_modelConfig: Fp_modelConfig<N>;
    type Fp2_modelConfig: Fp2_modelConfig<N, N2, Fp_modelConfig = Self::Fp_modelConfig>;
    const euler: bigint<N6> = bigint::<N6>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N6> = bigint::<N6>::one(); // with t odd
    const t_minus_1_over_2: bigint<N6> = bigint::<N6>::one(); // (t-1)/2
    const non_residue: my_Fp2<N, N2, Self::Fp2_modelConfig> =
        Fp2_model::<N, N2, Self::Fp2_modelConfig>::const_default();

    const nqr: Fp6_3over2_model<N, N2, N6, Self> =
        Fp6_3over2_model::<N, N2, N6, Self>::const_default();
    const nqr_to_t: Fp6_3over2_model<N, N2, N6, Self> =
        Fp6_3over2_model::<N, N2, N6, Self>::const_default();
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp2<N, N2, Self::Fp2_modelConfig>; 6] =
        [Fp2_model::<N, N2, Self::Fp2_modelConfig>::const_default(); 6];
    const Frobenius_coeffs_c2: [my_Fp2<N, N2, Self::Fp2_modelConfig>; 6] =
        [Fp2_model::<N, N2, Self::Fp2_modelConfig>::const_default(); 6];
}

type my_Fp<const N: usize, T> = Fp_model<N, T>;
type my_Fp2<const N: usize, const N2: usize, T> = Fp2_model<N, N2, T>;

#[derive(Default, Clone, Debug, Copy, Eq)]
pub struct Fp6_3over2_model<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
> {
    pub c0: my_Fp2<N, N2, T::Fp2_modelConfig>,
    pub c1: my_Fp2<N, N2, T::Fp2_modelConfig>,
    pub c2: my_Fp2<N, N2, T::Fp2_modelConfig>,
    _t: PhantomData<T>,
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> FPMConfig
    for Fp6_3over2_model<N, N2, N6, T>
{
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    FieldTForPowersConfig<N6> for Fp6_3over2_model<N, N2, N6, T>
{
    type FPM = Self;
    const num_limbs: usize = N;
    const s: usize = T::s; // modulus = 2^s * t + 1
    const t: bigint<N6> = T::t; // with t odd
    const t_minus_1_over_2: bigint<N6> = T::t_minus_1_over_2; // (t-1)/2
    const nqr: Self = T::nqr; // a quadratic nonresidue
    const nqr_to_t: Self = T::nqr_to_t; // nqr^t
    fn squared_(&self) -> Self {
        self.squared()
    }
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    Fp6_3over2_model<N, N2, N6, T>
{
    pub fn clear(&mut self) {
        self.c0.clear();
        self.c1.clear();
        self.c2.clear();
    }
    pub fn print(&self) {
        print!("c0/c1/c2:\n");
        self.c0.print();
        self.c1.print();
        self.c2.print();
    }
    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero() && self.c2.is_zero()
    }

    pub fn ceil_size_in_bits() -> usize {
        3 * my_Fp2::<N, N2, T::Fp2_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        3 * my_Fp2::<N, N2, T::Fp2_modelConfig>::floor_size_in_bits()
    }
    pub fn extension_degree() -> usize {
        6
    }
    pub fn field_char() -> bigint<N> {
        T::Fp_modelConfig::modulus
    }
    pub fn new(
        c0: my_Fp2<N, N2, T::Fp2_modelConfig>,
        c1: my_Fp2<N, N2, T::Fp2_modelConfig>,
        c2: my_Fp2<N, N2, T::Fp2_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            c2,
            _t: PhantomData,
        }
    }
    pub const fn const_new(
        c0: my_Fp2<N, N2, T::Fp2_modelConfig>,
        c1: my_Fp2<N, N2, T::Fp2_modelConfig>,
        c2: my_Fp2<N, N2, T::Fp2_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            c2,
            _t: PhantomData,
        }
    }
    pub const fn const_default() -> Self {
        Self {
            c0: my_Fp2::<N, N2, T::Fp2_modelConfig>::const_default(),
            c1: my_Fp2::<N, N2, T::Fp2_modelConfig>::const_default(),
            c2: my_Fp2::<N, N2, T::Fp2_modelConfig>::const_default(),
            _t: PhantomData,
        }
    }
    pub fn mul_by_non_residue(
        elt: &Fp2_model<N, N2, T::Fp2_modelConfig>,
    ) -> Fp2_model<N, N2, T::Fp2_modelConfig> {
        T::non_residue.clone() * elt
    }

    pub const fn zero() -> Self {
        Self::const_new(
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp2::<N, N2, T::Fp2_modelConfig>::one(),
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
            my_Fp2::<N, N2, T::Fp2_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp2::<N, N2, T::Fp2_modelConfig>::random_element(),
            c1: my_Fp2::<N, N2, T::Fp2_modelConfig>::random_element(),
            c2: my_Fp2::<N, N2, T::Fp2_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn squared(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.sqr_cnt++;

        //Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (CH-SQR2)
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

        //From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17
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
            self.c1.Frobenius_map(power) * T::Frobenius_coeffs_c1[power % 6].clone(),
            self.c2.Frobenius_map(power) * T::Frobenius_coeffs_c2[power % 6].clone(),
        )
    }

    pub fn sqrt(&self) -> Option<Self> {
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
        let n2 = n * 2;
        // Fp_model's from_words() takes care of asserts about vector length.
        self.c0.from_words(&words[..n])
            && self.c1.from_words(&words[n..n2])
            && self.c2.from_words(&words[n2..])
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> PartialEq
    for Fp6_3over2_model<N, N2, N6, T>
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.c0 == other.c0 && self.c1 == other.c1 && self.c2 == other.c2
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
    O: Borrow<Self>,
> Add<O> for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Fp6_3over2_model<N, N2, N6, T>;

    fn add(self, other: O) -> Self::Output {
        Self::new(
            self.c0 + other.borrow().c0,
            self.c1 + other.borrow().c1,
            self.c2 + other.borrow().c2,
        )
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> Sub
    for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.c0 - other.borrow().c0,
            self.c1 - other.borrow().c1,
            self.c2 - other.borrow().c2,
        )
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
    TC: Fp_modelConfig<N>,
> Mul<Fp_model<N, TC>> for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Fp6_3over2_model<N, N2, N6, T>;

    fn mul(self, rhs: Fp_model<N, TC>) -> Self::Output {
        Fp6_3over2_model::<N, N2, N6, T>::new(self.c0 * rhs, self.c1 * rhs, self.c2 * rhs)
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    Mul<Fp2_model<N, N2, T::Fp2_modelConfig>> for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Fp6_3over2_model<N, N2, N6, T>;

    fn mul(self, rhs: Fp2_model<N, N2, T::Fp2_modelConfig>) -> Self::Output {
        Fp6_3over2_model::<N, N2, N6, T>::new(self.c0 * rhs, self.c1 * rhs, self.c2 * rhs)
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
    O: Borrow<Self>,
> Mul<O> for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Fp6_3over2_model<N, N2, N6, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let (a, b, c, A, B, C) = (
            self.c0,
            self.c1,
            self.c2,
            rhs.borrow().c0,
            rhs.borrow().c1,
            rhs.borrow().c2,
        );
        let aA = a * A;
        let bB = b * B;
        let cC = c * C;

        Self::new(
            aA + Self::mul_by_non_residue(&((b + c) * (B + C) - bB - cC)),
            (a + b) * (A + B) - aA - bB + Self::mul_by_non_residue(&cC),
            (a + c) * (A + C) - aA + bB - cC,
        )
    }
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> Neg
    for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.c0, -self.c1, -self.c2)
    }
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> BitXor<u64>
    for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        Powers::power::<Fp6_3over2_model<N, N2, N6, T>>(&self, rhs)
    }
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    BitXor<bigint<N6>> for Fp6_3over2_model<N, N2, N6, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N6>) -> Self::Output {
        Powers::power::<Fp6_3over2_model<N, N2, N6, T>>(&self, rhs)
    }
}
impl<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
    O: Borrow<Self>,
> AddAssign<O> for Fp6_3over2_model<N, N2, N6, T>
{
    fn add_assign(&mut self, other: O) {
        *self = *self + other.borrow();
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
    O: Borrow<Self>,
> SubAssign<O> for Fp6_3over2_model<N, N2, N6, T>
{
    fn sub_assign(&mut self, other: O) {
        *self = *self - *other.borrow();
    }
}
impl<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
    O: Borrow<Self>,
> MulAssign<O> for Fp6_3over2_model<N, N2, N6, T>
{
    fn mul_assign(&mut self, other: O) {
        *self = *self * other.borrow();
    }
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    BitXorAssign<u64> for Fp6_3over2_model<N, N2, N6, T>
{
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = *self ^ rhs;
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    BitXorAssign<bigint<N6>> for Fp6_3over2_model<N, N2, N6, T>
{
    fn bitxor_assign(&mut self, rhs: bigint<N6>) {
        *self = *self ^ rhs;
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> PpConfig
    for Fp6_3over2_model<N, N2, N6, T>
{
    type BigIntT = bigint<N>;
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> One
    for Fp6_3over2_model<N, N2, N6, T>
{
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> Zero
    for Fp6_3over2_model<N, N2, N6, T>
{
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

use std::fmt;
use std::io::{self, BufRead};

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> fmt::Display
    for Fp6_3over2_model<N, N2, N6, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.c0, self.c1, self.c2)
    }
}

impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>>
    Fp6_3over2_model<N, N2, N6, T>
{
    pub fn read<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "数据不足"));
        }

        // 解析 c0, c1, c2
        let c0 = parts[0]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "c0解析失败"))?;
        let c1 = parts[1]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "c1解析失败"))?;
        let c2 = parts[2]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "c2解析失败"))?;

        Ok(Fp6_3over2_model::<N, N2, N6, T>::new(c0, c1, c2))
    }
}

pub fn write_vector<
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
>(
    v: &[Fp6_3over2_model<N, N2, N6, T>],
) -> String {
    let mut out = format!("{}\n", v.len());
    for el in v {
        out.push_str(&format!("{}\n", el));
    }
    out
}

pub fn read_vector<
    R: BufRead,
    const N: usize,
    const N2: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N6>,
>(
    reader: &mut R,
) -> io::Result<Vec<Fp6_3over2_model<N, N2, N6, T>>> {
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let s: usize = line
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "长度解析错误"))?;

    let mut v = Vec::with_capacity(s);

    for _ in 0..s {
        let el = Fp6_3over2_model::<N, N2, N6, T>::read(reader)?;
        v.push(el);
    }

    Ok(v)
}
impl<const N: usize, const N2: usize, const N6: usize, T: Fp6_modelConfig<N, N2, N6>> FromStr
    for Fp6_3over2_model<N, N2, N6, T>
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

use super::cubic_extension::{CubicExtConfig, CubicExtField};
use crate::algebra::fields::{
    cyclotomic::CyclotomicMultSubgroup,
    prime_extension::fp2::{Fp2, Fp2Config},
};
//  use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};

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
