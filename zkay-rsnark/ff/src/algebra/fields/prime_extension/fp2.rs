// Implementation of arithmetic in the finite field F[p^2].

use crate::{
    Fp_model, Fp_modelConfig as FpmConfig, Fp3_modelConfig, PpConfig,
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
    common::serialization::OUTPUT_SEPARATOR,
};

use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

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
type Fp_modelConfig<const N: usize, const N2: usize, T> =
    <T as Fp2_modelConfig<N, N2>>::Fp_modelConfig;

pub trait Fp2_modelConfig<const N: usize, const N2: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq + Debug
{
    type Fp_modelConfig: FpmConfig<N>;
    const euler: bigint<N2> = bigint::<N2>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N2> = bigint::<N2>::one(); // with t odd
    const t_minus_1_over_2: bigint<N2> = bigint::<N2>::one(); // (t-1)/2
    const non_residue: my_Fp<N, Fp_modelConfig<N, N2, Self>> =
        Fp_model::<N, Self::Fp_modelConfig>::const_default();

    const nqr: Fp2_model<N, N2, Self> = Fp2_model::<N, N2, Self>::const_default();
    const nqr_to_t: Fp2_model<N, N2, Self> = Fp2_model::<N, N2, Self>::const_default();
    /// non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp<N, Fp_modelConfig<N, N2, Self>>; 2] = [
        Fp_model::<N, Self::Fp_modelConfig>::const_default(),
        Fp_model::<N, Self::Fp_modelConfig>::const_default(),
    ];
}

type my_Fp<const N: usize, T> = Fp_model<N, T>;
#[derive(Default, Clone, Debug, Copy, Eq)]
pub struct Fp2_model<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> {
    pub c0: my_Fp<N, T::Fp_modelConfig>,
    pub c1: my_Fp<N, T::Fp_modelConfig>,
    _t: PhantomData<T>,
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> FPMConfig for Fp2_model<N, N2, T> {}
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> FieldTForPowersConfig<N2>
    for Fp2_model<N, N2, T>
{
    type FPM = Self;
    const num_limbs: usize = N;
    const s: usize = T::s; // modulus = 2^s * t + 1
    const t: bigint<N2> = T::t; // with t odd
    const t_minus_1_over_2: bigint<N2> = T::t_minus_1_over_2; // (t-1)/2
    const nqr: Self = T::nqr; // a quadratic nonresidue
    const nqr_to_t: Self = T::nqr_to_t; // nqr^t
    fn squared_(&self) -> Self {
        self.squared()
    }
}
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> Fp2_model<N, N2, T> {
    pub const non_residue: my_Fp<N, Fp_modelConfig<N, N2, T>> = T::non_residue;

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
        2 * my_Fp::<N, T::Fp_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        2 * my_Fp::<N, T::Fp_modelConfig>::floor_size_in_bits()
    }
    pub fn extension_degree() -> usize {
        2
    }
    pub fn field_char() -> bigint<N> {
        T::Fp_modelConfig::modulus
    }
    pub fn new(c0: my_Fp<N, T::Fp_modelConfig>, c1: my_Fp<N, T::Fp_modelConfig>) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }
    pub const fn const_new(
        c0: my_Fp<N, T::Fp_modelConfig>,
        c1: my_Fp<N, T::Fp_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }
    pub const fn const_newb(b: BigInt<N>) -> Self {
        Self {
            c0: my_Fp::<N, T::Fp_modelConfig>::const_new(b),
            c1: my_Fp::<N, T::Fp_modelConfig>::const_new(b),
            _t: PhantomData,
        }
    }
    pub const fn const_default() -> Self {
        Self {
            c0: my_Fp::<N, T::Fp_modelConfig>::const_new(BigInt::<N>::zero()),
            c1: my_Fp::<N, T::Fp_modelConfig>::const_new(BigInt::<N>::zero()),
            _t: PhantomData,
        }
    }
    pub const fn zero() -> Self {
        Self::const_new(
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
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba squaring) */
        let (a, b) = (self.c0, self.c1);
        let asq = a.squared();
        let bsq = b.squared();

        Self::new(asq + T::non_residue * bsq, (a + b).squared() - asq - bsq)
    }

    pub fn squared_complex(&self) -> Self {
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex squaring) */
        let (a, b) = (self.c0, self.c1);
        let ab = a * b;

        Self::new(
            (a + b) * (a + T::non_residue * b) - ab - T::non_residue * ab,
            ab + ab,
        )
    }

    pub fn inverse(&self) -> Self {
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
}

//
// bool Fp2_model<n,modulus>::operator==(other:&Fp2_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> PartialEq for Fp2_model<N, N2, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.c0 == other.c0 && self.c1 == other.c1
    }
}

//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator+(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
//
//     return Fp2_model<n,modulus>(self.c0 + other.c0,
//                                 self.c1 + other.c1);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>, O: Borrow<Self>> Add<O>
    for Fp2_model<N, N2, T>
{
    type Output = Fp2_model<N, N2, T>;

    fn add(self, other: O) -> Self::Output {
        Self::new(self.c0 + other.borrow().c0, self.c1 + other.borrow().c1)
    }
}

//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
//
//     return Fp2_model<n,modulus>(self.c0 - other.c0,
//                                 self.c1 - other.c1);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> Sub for Fp2_model<N, N2, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.c0 - other.borrow().c0, self.c1 - other.borrow().c1)
    }
}

//
// Fp2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
//
//     return Fp2_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>, TC: FpmConfig<N>>
    Mul<Fp_model<N, TC>> for Fp2_model<N, N2, T>
{
    type Output = Fp2_model<N, N2, T>;

    fn mul(self, rhs: Fp_model<N, TC>) -> Self::Output {
        Fp2_model::<N, N2, T>::new(self.c0 * rhs, self.c1 * rhs)
    }
}

//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator*(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
//
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     const my_Fp<N,T::Fp2_modelConfig>
//         &A = other.c0, &B = other.c1,
//         &a = self.c0, &b = self.c1;
//     let aA= a * A;
//     let bB= b * B;

//     return Fp2_model<n,modulus>(aA + T::non_residue * bB,
//                                 (a + b)*(A+B) - aA - bB);
// }

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>, O: Borrow<Self>> Mul<O>
    for Fp2_model<N, N2, T>
{
    type Output = Fp2_model<N, N2, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let (a, b, c, d) = (self.c0, self.c1, rhs.borrow().c0, rhs.borrow().c1);
        let (ac, bd) = (a * c, b * d);
        Self::new(ac + T::non_residue * bd, (a + b) * (c + d) - ac - bd)
    }
}
//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-() const
// {
//     return Fp2_model<n,modulus>(-self.c0,
//                                 -self.c1);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> Neg for Fp2_model<N, N2, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.c0, -self.c1)
    }
}
//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp2_model<n, modulus>>(*this, pow);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> BitXor<u64>
    for Fp2_model<N, N2, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        Powers::power::<Fp2_model<N, N2, T>>(&self, rhs)
    }
}
//
//
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp2_model<n, modulus>, m>(*this, pow);
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> BitXor<bigint<N2>>
    for Fp2_model<N, N2, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N2>) -> Self::Output {
        Powers::power::<Fp2_model<N, N2, T>>(&self, rhs)
    }
}

//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator+=(const Fp2_model<n,modulus>& other)
// {
//     *self = *this + other;
//     return *self;
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>, O: Borrow<Self>> AddAssign<O>
    for Fp2_model<N, N2, T>
{
    fn add_assign(&mut self, other: O) {
        *self = *self + other.borrow();
    }
}

//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator-=(const Fp2_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>, O: Borrow<Self>> SubAssign<O>
    for Fp2_model<N, N2, T>
{
    fn sub_assign(&mut self, other: O) {
        *self = *self - *other.borrow();
    }
}
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator*=(const Fp2_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>, O: Borrow<Self>> MulAssign<O>
    for Fp2_model<N, N2, T>
{
    fn mul_assign(&mut self, other: O) {
        *self = *self * other.borrow();
    }
}
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> BitXorAssign<u64>
    for Fp2_model<N, N2, T>
{
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = *self ^ rhs;
    }
}
//
//
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> BitXorAssign<bigint<N2>>
    for Fp2_model<N, N2, T>
{
    fn bitxor_assign(&mut self, rhs: bigint<N2>) {
        *self = *self ^ rhs;
    }
}
// impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> Mul<bigint<N>>
//     for Fp2_model<N, N2, T>
// {
//     type Output = Self;

//     fn mul(self, rhs: bigint<N>) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> PpConfig for Fp2_model<N, N2, T>
where
    <T as Fp2_modelConfig<N, N2>>::Fp_modelConfig: PpConfig,
{
    //type TT = bigint<N>;
    // type Fr=T::Fp_modelConfig;
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> One for Fp2_model<N, N2, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> Zero for Fp2_model<N, N2, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> From<usize>
    for Fp2_model<N, N2, T>
{
    fn from(b: usize) -> Self {
        let c = Fp_model::<N, T::Fp_modelConfig> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        };
        Fp2_model::<N, N2, T> {
            c0: c.clone(),
            c1: c.clone(),
            _t: PhantomData,
        }
    }
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> From<u32> for Fp2_model<N, N2, T> {
    fn from(b: u32) -> Self {
        let c = Fp_model::<N, T::Fp_modelConfig> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        };
        Fp2_model::<N, N2, T> {
            c0: c.clone(),
            c1: c.clone(),
            _t: PhantomData,
        }
    }
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> From<i32> for Fp2_model<N, N2, T> {
    fn from(b: i32) -> Self {
        let c = Fp_model::<N, T::Fp_modelConfig> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        };
        Fp2_model::<N, N2, T> {
            c0: c.clone(),
            c1: c.clone(),
            _t: PhantomData,
        }
    }
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> From<i64> for Fp2_model<N, N2, T> {
    fn from(b: i64) -> Self {
        let c = Fp_model::<N, T::Fp_modelConfig> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        };
        Fp2_model::<N, N2, T> {
            c0: c.clone(),
            c1: c.clone(),
            _t: PhantomData,
        }
    }
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> From<u64> for Fp2_model<N, N2, T> {
    fn from(b: u64) -> Self {
        let c = Fp_model::<N, T::Fp_modelConfig> {
            mont_repr: bigint::<N>::new(b),
            t: PhantomData,
        };
        Fp2_model::<N, N2, T> {
            c0: c.clone(),
            c1: c.clone(),
            _t: PhantomData,
        }
    }
}
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> From<&str>
    for Fp2_model<N, N2, T>
{
    fn from(b: &str) -> Self {
        let c = Fp_model::<N, T::Fp_modelConfig> {
            mont_repr: bigint::<N>::new_with_str(b).unwrap(),
            t: PhantomData,
        };
        Fp2_model::<N, N2, T> {
            c0: c.clone(),
            c1: c.clone(),
            _t: PhantomData,
        }
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

//
// std::ostream& operator<<(std::ostream &out, el:&Fp2_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> fmt::Display
    for Fp2_model<N, N2, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{OUTPUT_SEPARATOR}{}", self.c0, self.c1)
    }
}

use std::fmt;
use std::io::{self, Read};

// 对应: std::istream& operator>>(std::istream &in, Fp2_model<n, modulus> &el)
// Rust 中通常通过自定义函数或实现特定 Trait 来处理流输入
impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> Fp2_model<N, N2, T> {
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
        Ok(Fp2_model::<N, N2, T>::new(c0, c1))
    }
}

// 对应: std::ostream& operator<<(std::ostream& out, const std::vector<Fp2_model> &v)
pub fn write_vector<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>>(
    v: &[Fp2_model<N, N2, T>],
) -> String {
    let mut out = format!("{}\n", v.len());
    for el in v {
        // 对应 out << t << OUTPUT_NEWLINE;
        out.push_str(&format!("{}\n", el));
    }
    out
}

// 对应: std::istream& operator>>(std::istream& in, std::vector<Fp2_model> &v)
pub fn read_vector<R: io::BufRead, const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>>(
    reader: &mut R,
) -> io::Result<Vec<Fp2_model<N, N2, T>>> {
    let mut line = String::new();
    // 1. 读取大小 s
    reader.read_line(&mut line)?;
    let s: usize = line
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "size parse error"))?;

    // 2. 对应 v.reserve(s)
    let mut v = Vec::with_capacity(s);

    // 3. 循环读取并填充 (对应 for 循环)
    for _ in 0..s {
        // 对应 in >> el; v.emplace_back(el);
        let el = Fp2_model::<N, N2, T>::read(reader)?;
        v.push(el);
    }

    Ok(v)
}

impl<const N: usize, const N2: usize, T: Fp2_modelConfig<N, N2>> FromStr for Fp2_model<N, N2, T> {
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
