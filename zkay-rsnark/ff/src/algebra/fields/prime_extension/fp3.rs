//  Declaration of arithmetic in the finite  field F[p^3].

use crate::{
    Fp_model, Fp_modelConfig as FpmConfig, PpConfig,
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

use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

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
type Fp_modelConfig<const N: usize, const N3: usize, T> =
    <T as Fp3_modelConfig<N, N3>>::Fp_modelConfig;
pub trait Fp3_modelConfig<const N: usize, const N3: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq + Debug
{
    type Fp_modelConfig: FpmConfig<N>;
    const euler: bigint<N3> = bigint::<N3>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N3> = bigint::<N3>::one(); // with t odd
    const t_minus_1_over_2: bigint<N3> = bigint::<N3>::one(); // (t-1)/2
    const non_residue: my_Fp<N, Fp_modelConfig<N, N3, Self>> =
        Fp_model::<N, Self::Fp_modelConfig>::const_default();

    const nqr: Fp3_model<N, N3, Self> = Fp3_model::<N, N3, Self>::const_default();
    const nqr_to_t: Fp3_model<N, N3, Self> = Fp3_model::<N, N3, Self>::const_default();
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Fp_modelConfig<N, N3, Self>>; 3] =
        [Fp_model::<N, Self::Fp_modelConfig>::const_default(); 3];
    const Frobenius_coeffs_c2: [my_Fp<N, Fp_modelConfig<N, N3, Self>>; 3] =
        [Fp_model::<N, Self::Fp_modelConfig>::const_default(); 3];
}

type my_Fp<const N: usize, T> = Fp_model<N, T>;

#[derive(Default, Clone, Debug, Copy, Eq)]
pub struct Fp3_model<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> {
    pub c0: my_Fp<N, T::Fp_modelConfig>,
    pub c1: my_Fp<N, T::Fp_modelConfig>,
    pub c2: my_Fp<N, T::Fp_modelConfig>,
    _t: PhantomData<T>,
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> FPMConfig for Fp3_model<N, N3, T> {}
impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> FieldTForPowersConfig<N3>
    for Fp3_model<N, N3, T>
{
    type FPM = Self;
    const num_limbs: usize = N;
    const s: usize = T::s; // modulus = 2^s * t + 1
    const t: bigint<N3> = T::t; // with t odd
    const t_minus_1_over_2: bigint<N3> = T::t_minus_1_over_2; // (t-1)/2
    const nqr: Self = T::nqr; // a quadratic nonresidue
    const nqr_to_t: Self = T::nqr_to_t; // nqr^t
    fn squared_(&self) -> Self {
        self.squared()
    }
}
// use crate::algebra::field_utils::field_utils;
impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> Fp3_model<N, N3, T> {
    pub const fn const_new(b: BigInt<N>) -> Self {
        Self {
            c0: my_Fp::<N, T::Fp_modelConfig>::const_new(b),
            c1: my_Fp::<N, T::Fp_modelConfig>::const_new(b),
            c2: my_Fp::<N, T::Fp_modelConfig>::const_new(b),
            _t: PhantomData,
        }
    }
    pub const fn const_default() -> Self {
        Self {
            c0: my_Fp::<N, T::Fp_modelConfig>::const_new(BigInt::<N>::zero()),
            c1: my_Fp::<N, T::Fp_modelConfig>::const_new(BigInt::<N>::zero()),
            c2: my_Fp::<N, T::Fp_modelConfig>::const_new(BigInt::<N>::zero()),
            _t: PhantomData,
        }
    }
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
        3 * my_Fp::<N, T::Fp_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        3 * my_Fp::<N, T::Fp_modelConfig>::floor_size_in_bits()
    }
    pub fn extension_degree() -> usize {
        3
    }
    pub fn field_char() -> bigint<N> {
        T::Fp_modelConfig::modulus
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

        //Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (CH-SQR2)

        let (a, b, c) = (self.c0, self.c1, self.c2);
        let s0: my_Fp<N, T::Fp_modelConfig> = a.squared();
        let ab: my_Fp<N, T::Fp_modelConfig> = a * b;
        let s1: my_Fp<N, T::Fp_modelConfig> = ab + ab;
        let s2: my_Fp<N, T::Fp_modelConfig> = (a - b + c).squared();
        let bc: my_Fp<N, T::Fp_modelConfig> = b * c;
        let s3: my_Fp<N, T::Fp_modelConfig> = bc + bc;
        let s4: my_Fp<N, T::Fp_modelConfig> = c.squared();

        Self::new(
            T::non_residue * s3 + s0,
            T::non_residue * s4 + s1,
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

        let (a, b, c) = (self.c0, self.c1, self.c2);

        //From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17
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

    pub fn from_words(&mut self, words: &[u64]) -> bool {
        let n = words.len() / 3;
        let n2 = n * 2;
        // Fp_model's from_words() takes care of asserts about vector length.
        self.c0.from_words(&words[0..n])
            && self.c1.from_words(&words[n..n2])
            && self.c2.from_words(&words[n2..])
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> PartialEq for Fp3_model<N, N3, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.c0 == other.c0 && self.c1 == other.c1 && self.c2 == other.c2
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, O: Borrow<Self>> AddAssign<O>
    for Fp3_model<N, N3, T>
{
    fn add_assign(&mut self, other: O) {
        *self = *self + other.borrow();
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, O: Borrow<Self>> SubAssign<O>
    for Fp3_model<N, N3, T>
{
    fn sub_assign(&mut self, other: O) {
        *self = *self - *other.borrow();
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, O: Borrow<Self>> MulAssign<O>
    for Fp3_model<N, N3, T>
{
    fn mul_assign(&mut self, other: O) {
        *self = *self * other.borrow();
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> BitXorAssign<u64>
    for Fp3_model<N, N3, T>
{
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = *self ^ rhs;
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> BitXorAssign<bigint<N3>>
    for Fp3_model<N, N3, T>
{
    fn bitxor_assign(&mut self, rhs: bigint<N3>) {
        *self = *self ^ rhs;
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, O: Borrow<Self>> Add<O>
    for Fp3_model<N, N3, T>
{
    type Output = Fp3_model<N, N3, T>;

    fn add(self, other: O) -> Self::Output {
        Self::new(
            self.c0 + other.borrow().c0,
            self.c1 + other.borrow().c1,
            self.c2 + other.borrow().c2,
        )
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, TC: FpmConfig<N>>
    Add<Fp_model<N, TC>> for Fp3_model<N, N3, T>
{
    type Output = Fp3_model<N, N3, T>;

    fn add(self, rhs: Fp_model<N, TC>) -> Self::Output {
        Fp3_model::<N, N3, T>::new(
            self.c0.clone() * rhs.clone(),
            self.c1.clone() * rhs.clone(),
            self.c2.clone() * rhs.clone(),
        )
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> Sub for Fp3_model<N, N3, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.c0 - other.c0, self.c1 - other.c1, self.c2 - other.c2)
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, TC: FpmConfig<N>>
    Mul<Fp_model<N, TC>> for Fp3_model<N, N3, T>
{
    type Output = Self;

    fn mul(self, rhs: Fp_model<N, TC>) -> Self::Output {
        Self::new(
            self.c0.clone() * rhs.clone(),
            self.c1.clone() * rhs.clone(),
            self.c2.clone() * rhs.clone(),
        )
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>, O: Borrow<Self>> Mul<O>
    for Fp3_model<N, N3, T>
{
    type Output = Fp3_model<N, N3, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let (a, b, c, d, e, f) = (
            self.c0,
            self.c1,
            self.c2,
            rhs.borrow().c0,
            rhs.borrow().c1,
            rhs.borrow().c2,
        );
        let (ad, be, cf) = (a * d, b * e, c * f);
        Self::new(
            ad + T::non_residue * ((b + c) * (e * f) - be - cf),
            (a + b) * (d + e) - ad - be + T::non_residue * cf,
            (a + c) * (d + f) - ad + be - cf,
        )
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> BitXor<u64>
    for Fp3_model<N, N3, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        Powers::power::<Fp3_model<N, N3, T>>(&self, rhs)
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> BitXor<bigint<N3>>
    for Fp3_model<N, N3, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N3>) -> Self::Output {
        Powers::power::<Fp3_model<N, N3, T>>(&self, rhs)
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> Neg for Fp3_model<N, N3, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.c0, -self.c1, -self.c2)
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> PpConfig for Fp3_model<N, N3, T> {
    type GType = Self;
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> Mul<bigint<N>>
    for Fp3_model<N, N3, T>
{
    type Output = Self;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> One for Fp3_model<N, N3, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> Zero for Fp3_model<N, N3, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

use std::fmt;
use std::io::{self, BufRead};

// 1. 对应: operator<<(std::ostream &out, const Fp3_model &el)
impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> fmt::Display
    for Fp3_model<N, N3, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用空格作为 OUTPUT_SEPARATOR
        write!(f, "{} {} {}", self.c0, self.c1, self.c2)
    }
}

// 2. 对应: operator>>(std::istream &in, Fp3_model &el)
impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> Fp3_model<N, N3, T> {
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

        Ok(Fp3_model::<N, N3, T>::new(c0, c1, c2))
    }
}

// 3. 对应: operator<<(std::ostream& out, const std::vector<Fp3_model> &v)
pub fn write_vector<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>>(
    v: &[Fp3_model<N, N3, T>],
) -> String {
    let mut out = format!("{}\n", v.len()); // 写入长度
    for el in v {
        out.push_str(&format!("{}\n", el)); // 逐个写入元素并换行
    }
    out
}

// 4. 对应: operator>>(std::istream& in, std::vector<Fp3_model> &v)
pub fn read_vector<R: BufRead, const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>>(
    reader: &mut R,
) -> io::Result<Vec<Fp3_model<N, N3, T>>> {
    // v.clear() 在 Rust 中通过新建 Vec 或传入 &mut Vec 处理
    let mut line = String::new();

    // 读取大小 s
    reader.read_line(&mut line)?;
    let s: usize = line
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "长度解析错误"))?;

    // 对应 v.reserve(s)
    let mut v = Vec::with_capacity(s);

    // 对应 for (size_t i = 0; i < s; ++i)
    for _ in 0..s {
        // 对应 in >> el; v.emplace_back(el);
        let el = Fp3_model::<N, N3, T>::read(reader)?;
        v.push(el);
    }

    Ok(v)
}

impl<const N: usize, const N3: usize, T: Fp3_modelConfig<N, N3>> FromStr for Fp3_model<N, N3, T> {
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
