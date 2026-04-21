//  Declaration of arithmetic in the finite field F[(p^3)^2]

use crate::{
    Fp_model, Fp_modelConfig as FpmConfig, Fp2_model, Fp2_modelConfig, Fp3_model, Fp3_modelConfig,
    PpConfig,
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

// /**
//  * Arithmetic in the finite field F[(p^3)^2].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  * Fp6 = Fp3[Y]/(Y^2-X) where Fp3 = Fp[X]/(X^3-T::non_residue) and T::non_residue is in Fp.
//  *
//  * ASSUMPTION: p = 1 (mod 6)
//  */
//

// type Fp_modelConfig<const N:usize,T>=
pub trait Fp6_modelConfig<const N: usize, const N2: usize, const N3: usize, const N6: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy + Eq + Debug
{
    type Fp_modelConfig: FpmConfig<N>;
    type Fp3_modelConfig: Fp3_modelConfig<N, N3, Fp_modelConfig = Self::Fp_modelConfig>;
    type Fp2_modelConfig: Fp2_modelConfig<N, N2, Fp_modelConfig = Self::Fp_modelConfig>;
    const euler: bigint<N6> = bigint::<N6>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N6> = bigint::<N6>::one(); // with t odd
    const t_minus_1_over_2: bigint<N6> = bigint::<N6>::one(); // (t-1)/2
    const non_residue: my_Fp_modelConfig<N, N2, N3, N6, Self> =
        Fp_model::<N, Self::Fp_modelConfig>::const_default();

    const nqr: Fp6_2over3_model<N, N2, N3, N6, Self> =
        Fp6_2over3_model::<N, N2, N3, N6, Self>::const_default();
    const nqr_to_t: Fp6_2over3_model<N, N2, N3, N6, Self> =
        Fp6_2over3_model::<N, N2, N3, N6, Self>::const_default();
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp_modelConfig<N, N2, N3, N6, Self>; 6] =
        [Fp_model::<N, Self::Fp_modelConfig>::const_default(); 6];
}
type my_Fp_modelConfig<const N: usize, const N2: usize,const N3: usize, const N6: usize, T> = Fp_model<
    N,
    <<T as Fp6_modelConfig<N, N2,N3,N6>>::Fp3_modelConfig as Fp3_modelConfig<N,N3>>::Fp_modelConfig,
>;
type my_Fp<const N: usize, T> = Fp_model<N, T>;
type my_Fp2<const N: usize, const N2: usize, T> = Fp2_model<N, N2, T>;
pub type my_Fp3<const N: usize, const N3: usize, T> = Fp3_model<N, N3, T>;
type my_Fpe<const N: usize, const N3: usize, T> = my_Fp3<N, N3, T>;

#[derive(Default, Clone, Debug, Copy, Eq)]
pub struct Fp6_2over3_model<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> {
    pub c0: my_Fp3<N, N3, T::Fp3_modelConfig>,
    pub c1: my_Fp3<N, N3, T::Fp3_modelConfig>,
    _t: PhantomData<T>,
}

impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> FPMConfig for Fp6_2over3_model<N, N2, N3, N6, T>
{
}
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> FieldTForPowersConfig<N6> for Fp6_2over3_model<N, N2, N3, N6, T>
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
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> Fp6_2over3_model<N, N2, N3, N6, T>
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
        2 * my_Fp3::<N, N3, T::Fp3_modelConfig>::ceil_size_in_bits()
    }
    pub fn floor_size_in_bits() -> usize {
        2 * my_Fp3::<N, N3, T::Fp3_modelConfig>::floor_size_in_bits()
    }
    pub fn extension_degree() -> usize {
        6
    }
    pub fn field_char() -> bigint<N> {
        T::Fp_modelConfig::modulus
    }
    pub fn new(
        c0: my_Fp3<N, N3, T::Fp3_modelConfig>,
        c1: my_Fp3<N, N3, T::Fp3_modelConfig>,
    ) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }
    pub const fn const_default() -> Self {
        Self {
            c0: my_Fp3::<N, N3, T::Fp3_modelConfig>::const_default(),
            c1: my_Fp3::<N, N3, T::Fp3_modelConfig>::const_default(),
            _t: PhantomData,
        }
    }
    pub fn mul_by_non_residue(
        elem: &Fp3_model<N, N3, T::Fp3_modelConfig>,
    ) -> Fp3_model<N, N3, T::Fp3_modelConfig> {
        Fp3_model::<N, N3, T::Fp3_modelConfig>::new(
            elem.c2 * T::non_residue.clone(),
            elem.c0,
            elem.c1,
        )
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp3::<N, N3, T::Fp3_modelConfig>::zero(),
            my_Fp3::<N, N3, T::Fp3_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp3::<N, N3, T::Fp3_modelConfig>::one(),
            my_Fp3::<N, N3, T::Fp3_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp3::<N, N3, T::Fp3_modelConfig>::random_element(),
            c1: my_Fp3::<N, N3, T::Fp3_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn mul_by_2345(&self, other: &Self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.mul_cnt++;

        //Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba)
        assert!(other.c0.c0.is_zero());
        assert!(other.c0.c1.is_zero());

        let (A, B) = (other.c0, other.c1);
        let (a, b) = (self.c0, self.c1);
        let aA = my_Fp3::<N, N3, T::Fp3_modelConfig>::new(
            a.c1 * A.c2 * T::non_residue,
            a.c2 * A.c2 * T::non_residue,
            a.c0 * A.c2,
        );
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
            self.c1.Frobenius_map(power) * T::Frobenius_coeffs_c1[power % 6].clone(),
        )
    }

    pub fn unitary_inverse(&self) -> Self {
        Self::new(self.c0, -self.c1)
    }

    pub fn cyclotomic_squared(&self) -> Self {
        let a = my_Fp2::<N, N2, T::Fp2_modelConfig>::new(self.c0.c0, self.c1.c1);

        let b = my_Fp2::<N, N2, T::Fp2_modelConfig>::new(self.c1.c0, self.c0.c2);

        let c = my_Fp2::<N, N2, T::Fp2_modelConfig>::new(self.c0.c1, self.c1.c2);

        let asq = a.squared();
        let bsq = b.squared();
        let csq = c.squared();

        let mut A_a = asq.c0 - a.c0;
        A_a = A_a + A_a + asq.c0;
        let mut A_b = asq.c1 + a.c1;
        A_b = A_b + A_b + asq.c1;

        let B_tmp = T::Fp3_modelConfig::non_residue * csq.c1;
        let mut B_a = B_tmp + b.c0;
        B_a = B_a + B_a + B_tmp;

        let mut B_b = csq.c0 - b.c1;
        B_b = B_b + B_b + csq.c0;

        let mut C_a = bsq.c0 - c.c0;
        C_a = C_a + C_a + bsq.c0;
        let mut C_b = bsq.c1 + c.c1;
        C_b = C_b + C_b + bsq.c1;

        Self::new(
            my_Fp3::<N, N3, T::Fp3_modelConfig>::new(A_a, C_a, B_b),
            my_Fp3::<N, N3, T::Fp3_modelConfig>::new(B_a, A_b, C_b),
        )
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

    pub fn from_words(&mut self, words: &[u64]) -> bool {
        let n = words.len() / 2;
        // Fp_model's from_words() takes care of asserts about vector length.
        self.c0.from_words(&words[0..n]) && self.c1.from_words(&words[n..])
    }
}

//
// bool Fp6_2over3_model<n,modulus>::operator==(other:&Fp6_2over3_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> PartialEq for Fp6_2over3_model<N, N2, N3, N6, T>
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.c0 == other.c0 && self.c1 == other.c1
    }
}

//
// bool Fp6_2over3_model<n,modulus>::operator!=(other:&Fp6_2over3_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator+(other:&Fp6_2over3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
//
//     Self::new(self.c0 + other.c0,
//                                 self.c1 + other.c1);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
    O: Borrow<Self>,
> Add<O> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Fp6_2over3_model<N, N2, N3, N6, T>;

    fn add(self, other: O) -> Self::Output {
        Self::new(self.c0 + other.borrow().c0, self.c1 + other.borrow().c1)
    }
}
//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator-(other:&Fp6_2over3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
//
//     Self::new(self.c0 - other.c0,
//                                 self.c1 - other.c1);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> Sub for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.c0 + other.c0, self.c1 + other.c1)
    }
}

//
// Fp6_2over3_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp6_2over3_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
//
//     Self::new(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
    TC: FpmConfig<N>,
> Mul<Fp_model<N, TC>> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Fp6_2over3_model<N, N2, N3, N6, T>;

    fn mul(self, rhs: Fp_model<N, TC>) -> Self::Output {
        Self::new(self.c0.clone() * rhs, self.c1.clone() * rhs)
    }
}

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator*(other:&Fp6_2over3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
//
//     //Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba)
//     B:&my_Fp3<N,T::Fp3_modelConfig> = other.c1, &A = other.c0,
//                  &b = self.c1, &a = self.c0;
//     let aA= a*A;
//     let bB= b*B;
//     let beta_bB= Fp6_2over3_model<n,modulus>::mul_by_non_residue(bB);

//     Self::new(aA + beta_bB,
//                                        (a+b)*(A+B) - aA  - bB);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
    O: Borrow<Self>,
> Mul<O> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Fp6_2over3_model<N, N2, N3, N6, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let (A, B) = (rhs.borrow().c0, rhs.borrow().c1);
        let (a, b) = (self.c0, self.c1);
        let aA = a * A;
        let bB = b * B;
        let beta_bB = Self::mul_by_non_residue(&bB);

        Self::new(aA + beta_bB, (a + b) * (A + B) - aA - bB)
    }
}
// impl<const N: usize, const N2: usize,const N3: usize, const N6: usize, T: Fp6_modelConfig<N, N2,N3,N6>> Mul<bigint<N>>
//     for Fp6_2over3_model<N, N2, N3,N6,T>
// {
//     type Output = Self;

//     fn mul(self, rhs: bigint<N>) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator-() const
// {
//     Self::new(-self.c0,
//                                 -self.c1);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> Neg for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.c0, -self.c1)
    }
}

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp6_2over3_model<n, modulus> >(*this, pow);
// }

impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> BitXor<u64> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        Powers::power::<Fp6_2over3_model<N, N2, N3, N6, T>>(&self, rhs)
    }
}

//
//
// Fp6_2over3_model<n, modulus> Fp6_2over3_model<n,modulus>::operator^(exponent:&bigint<m>) const
// {
//     return power<Fp6_2over3_model<n, modulus>, m>(*this, exponent);
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> BitXor<bigint<N6>> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N6>) -> Self::Output {
        Powers::power::<Fp6_2over3_model<N, N2, N3, N6, T>>(&self, rhs)
    }
}

//
//
// Fp6_2over3_model<n, modulus> Fp6_2over3_model<n,modulus>::operator^(exponent:&Fp6_2over3_model<m, exp_modulus>) const
// {
//     return *self^(exponent.as_bigint());
// }

//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator+=(const Fp6_2over3_model<n,modulus>& other)
// {
//     *self = *this + other;
//     return *self;
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
    O: Borrow<Self>,
> AddAssign<O> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn add_assign(&mut self, other: O) {
        *self = *self + other.borrow();
    }
}

//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator-=(const Fp6_2over3_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
    O: Borrow<Self>,
> SubAssign<O> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn sub_assign(&mut self, other: O) {
        *self = *self - *other.borrow();
    }
}
//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator*=(const Fp6_2over3_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
    O: Borrow<Self>,
> MulAssign<O> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn mul_assign(&mut self, rhs: O) {
        *self = *self * rhs.borrow();
    }
}

//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> BitXorAssign<u64> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = *self ^ rhs;
    }
}
//
//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }

impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> BitXorAssign<bigint<N6>> for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn bitxor_assign(&mut self, rhs: bigint<N6>) {
        *self = *self ^ rhs;
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> PpConfig for Fp6_2over3_model<N, N2, N3, N6, T>
{
    type GType = Self;
}

impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> One for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn one() -> Self {
        Self::one()
    }
}

impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> Zero for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

use std::fmt;
use std::io::{self, Read};

// 对应: std::ostream& operator<<(std::ostream &out, const Fp2_model<n, modulus> &el)
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> fmt::Display for Fp6_2over3_model<N, N2, N3, N6, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // OUTPUT_SEPARATOR 在 Rust 中通常直接用空格或指定的 separator
        write!(f, "{} {}", self.c0, self.c1)
    }
}

// 对应: std::istream& operator>>(std::istream &in, Fp2_model<n, modulus> &el)
// Rust 中通常通过自定义函数或实现特定 Trait 来处理流输入
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> Fp6_2over3_model<N, N2, N3, N6, T>
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
        Ok(Fp6_2over3_model::<N, N2, N3, N6, T>::new(c0, c1))
    }
}
impl<
    const N: usize,
    const N2: usize,
    const N3: usize,
    const N6: usize,
    T: Fp6_modelConfig<N, N2, N3, N6>,
> FromStr for Fp6_2over3_model<N, N2, N3, N6, T>
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
// std::ostream& operator<<(std::ostream &out, el:&Fp6_2over3_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

//
// std::istream& operator>>(std::istream &in, Fp6_2over3_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1;
//     return in;
// }

use super::quadratic_extension::{QuadExtConfig, QuadExtField};
use crate::algebra::fields::{
    cyclotomic::CyclotomicMultSubgroup,
    prime_extension::fp3::{Fp3, Fp3Config},
};
//  use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};

use core::{marker::PhantomData, ops::Not};

pub trait Fp6Config: 'static + Send + Sync {
    type Fp3Config: Fp3Config;

    const NONRESIDUE: Fp3<Self::Fp3Config>;

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP6_C1: &'static [<Self::Fp3Config as Fp3Config>::Fp];

    #[inline(always)]
    fn mul_fp3_by_nonresidue_in_place(fe: &mut Fp3<Self::Fp3Config>) -> &mut Fp3<Self::Fp3Config> {
        let old_c1 = fe.c1;
        fe.c1 = fe.c0;
        fe.c0 = fe.c2;
        <Self::Fp3Config as Fp3Config>::mul_fp_by_nonresidue_in_place(&mut fe.c0);
        fe.c2 = old_c1;
        fe
    }
}

pub struct Fp6ConfigWrapper<P: Fp6Config>(PhantomData<P>);

impl<P: Fp6Config> QuadExtConfig for Fp6ConfigWrapper<P> {
    type BasePrimeField = <P::Fp3Config as Fp3Config>::Fp;
    type BaseField = Fp3<P::Fp3Config>;
    type FrobCoeff = Self::BasePrimeField;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 6;

    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const FROBENIUS_COEFF_C1: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP6_C1;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp3_by_nonresidue_in_place(fe);
        fe
    }

    fn mul_base_field_by_frob_coeff(fe: &mut Self::BaseField, power: usize) {
        fe.mul_assign_by_fp(&Self::FROBENIUS_COEFF_C1[power % Self::DEGREE_OVER_BASE_PRIME_FIELD]);
    }
}

pub type Fp6<P> = QuadExtField<Fp6ConfigWrapper<P>>;

impl<P: Fp6Config> Fp6<P> {
    pub fn mul_by_034(
        &mut self,
        c0: &<P::Fp3Config as Fp3Config>::Fp,
        c3: &<P::Fp3Config as Fp3Config>::Fp,
        c4: &<P::Fp3Config as Fp3Config>::Fp,
    ) {
        let z0 = self.c0.c0;
        let z1 = self.c0.c1;
        let z2 = self.c0.c2;
        let z3 = self.c1.c0;
        let z4 = self.c1.c1;
        let z5 = self.c1.c2;

        let x0 = *c0;
        let x3 = *c3;
        let x4 = *c4;

        let mut tmp1 = x3;
        tmp1 *= &<P::Fp3Config as Fp3Config>::NONRESIDUE;
        let mut tmp2 = x4;
        tmp2 *= &<P::Fp3Config as Fp3Config>::NONRESIDUE;

        self.c0.c0 = x0 * &z0 + &(tmp1 * &z5) + &(tmp2 * &z4);
        self.c0.c1 = x0 * &z1 + &(x3 * &z3) + &(tmp2 * &z5);
        self.c0.c2 = x0 * &z2 + &(x3 * &z4) + &(x4 * &z3);
        self.c1.c0 = x0 * &z3 + &(x3 * &z0) + &(tmp2 * &z2);
        self.c1.c1 = x0 * &z4 + &(x3 * &z1) + &(x4 * &z0);
        self.c1.c2 = x0 * &z5 + &(x3 * &z2) + &(x4 * &z1);
    }

    pub fn mul_by_014(
        &mut self,
        c0: &<P::Fp3Config as Fp3Config>::Fp,
        c1: &<P::Fp3Config as Fp3Config>::Fp,
        c4: &<P::Fp3Config as Fp3Config>::Fp,
    ) {
        let z0 = self.c0.c0;
        let z1 = self.c0.c1;
        let z2 = self.c0.c2;
        let z3 = self.c1.c0;
        let z4 = self.c1.c1;
        let z5 = self.c1.c2;

        let x0 = *c0;
        let x1 = *c1;
        let x4 = *c4;

        let mut tmp1 = x1;
        tmp1 *= &<P::Fp3Config as Fp3Config>::NONRESIDUE;
        let mut tmp2 = x4;
        tmp2 *= &<P::Fp3Config as Fp3Config>::NONRESIDUE;

        self.c0.c0 = x0 * &z0 + &(tmp1 * &z2) + &(tmp2 * &z4);
        self.c0.c1 = x0 * &z1 + &(x1 * &z0) + &(tmp2 * &z5);
        self.c0.c2 = x0 * &z2 + &(x1 * &z1) + &(x4 * &z3);
        self.c1.c0 = x0 * &z3 + &(tmp1 * &z5) + &(tmp2 * &z2);
        self.c1.c1 = x0 * &z4 + &(x1 * &z3) + &(x4 * &z0);
        self.c1.c2 = x0 * &z5 + &(x1 * &z4) + &(x4 * &z1);
    }
}

impl<P: Fp6Config> CyclotomicMultSubgroup for Fp6<P> {
    const INVERSE_IS_FAST: bool = true;
    fn cyclotomic_inverse_in_place(&mut self) -> Option<&mut Self> {
        self.is_zero().not().then(|| {
            self.conjugate_in_place();
            self
        })
    }
}
