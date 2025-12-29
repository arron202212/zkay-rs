//  Declaration of arithmetic in the finite field F[(p^3)^2]

// use crate::algebra::fields::prime_base::fp;
// use crate::algebra::fields::prime_extension::fp2;
// use crate::algebra::fields::prime_extension::fp3;
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
use crate::Fp_modelConfig as FpmConfig;
use crate::Fp2_model;
use crate::Fp2_modelConfig;
use crate::Fp3_model;
use crate::Fp3_modelConfig;

use crate::scalar_multiplication::wnaf::find_wnaf;
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
pub trait Fp6_modelConfig<const N: usize>:
    'static + Send + Sync + Sized + Default + Clone + Copy
{
    type Fp_modelConfig: FpmConfig<N>;
    type Fp3_modelConfig: Fp3_modelConfig<N, Fp_modelConfig = Self::Fp_modelConfig>;
    type Fp2_modelConfig: Fp2_modelConfig<N, Fp_modelConfig = Self::Fp_modelConfig>;
    const non_residue: my_Fp_modelConfig<N, Self>;

    const nqr: (my_Fp_modelConfig<N, Self>, my_Fp_modelConfig<N, Self>);
    const nqr_to_t: (my_Fp_modelConfig<N, Self>, my_Fp_modelConfig<N, Self>);
    /// T::non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [my_Fp_modelConfig<N, Self>; 2];
}
type my_Fp_modelConfig<const N: usize, T> =
    Fp_model<N, <<T as Fp6_modelConfig<N>>::Fp3_modelConfig as Fp3_modelConfig<N>>::Fp_modelConfig>;
type my_Fp<const N: usize, T> = Fp_model<N, T>;
type my_Fp2<const N: usize, T> = Fp2_model<N, T>;
pub type my_Fp3<const N: usize, T> = Fp3_model<N, T>;
type my_Fpe<const N: usize, T> = my_Fp3<N, T>;

#[derive(Default, Clone, Copy)]
pub struct Fp6_2over3_model<const N: usize, T: Fp6_modelConfig<N>> {
    // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    // static i64 add_cnt;
    // static i64 sub_cnt;
    // static i64 mul_cnt;
    // static i64 sqr_cnt;
    // static i64 inv_cnt;
    //#endif

    // static bigint<6*n> euler; // (modulus^6-1)/2
    // static std::usize s; // modulus^6 = 2^s * t + 1
    // static bigint<6*n> t; // with t odd
    // static bigint<6*n> t_minus_1_over_2; // (t-1)/2
    // static Fp6_2over3_model<n, modulus> nqr; // a quadratic nonresidue in Fp6
    // static Fp6_2over3_model<n, modulus> nqr_to_t; // nqr^t
    // static my_Fp T::non_residue;
    // static my_Fp Frobenius_coeffs_c1[6]; // T::non_residue^((modulus^i-1)/6)   for i=0,1,2,3,4,5
    pub c0: my_Fp3<N, T::Fp3_modelConfig>,
    pub c1: my_Fp3<N, T::Fp3_modelConfig>,
    _t: PhantomData<T>,
    // Fp6_2over3_model() {};
    // Fp6_2over3_model(c0:my_Fp3<N,T::Fp3_modelConfig>, c1:my_Fp3<N,T::Fp3_modelConfig>)->Selfc0,c1 {};

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
    // bool operator==(other:&Fp6_2over3_model) const;
    // bool operator!=(other:&Fp6_2over3_model) const;

    // Fp6_2over3_model& operator+=(other:&Fp6_2over3_model);
    // Fp6_2over3_model& operator-=(other:&Fp6_2over3_model);
    // Fp6_2over3_model& operator*=(other:&Fp6_2over3_model);
    // Fp6_2over3_model& operator^=(const u64 pow);

    // Fp6_2over3_model& operator^=(pow:&bigint<m>);

    // Fp6_2over3_model operator+(other:&Fp6_2over3_model) const;
    // Fp6_2over3_model operator-(other:&Fp6_2over3_model) const;
    // Fp6_2over3_model operator*(other:&Fp6_2over3_model) const;
    // Fp6_2over3_model mul_by_2345(other:&Fp6_2over3_model) const;
    // Fp6_2over3_model operator^(const:u64 pow),

    // Fp6_2over3_model operator^(exponent:&bigint<m>) const;

    // Fp6_2over3_model operator^(exponent:&Fp_model<m, exp_modulus>) const;
    // Fp6_2over3_model operator-() const;

    // Fp6_2over3_model& square();
    // Fp6_2over3_model squared() const;
    // Fp6_2over3_model& invert();
    // Fp6_2over3_model inverse() const;
    // Fp6_2over3_model Frobenius_map(u64 power) const;
    // Fp6_2over3_model unitary_inverse() const;
    // Fp6_2over3_model cyclotomic_squared() const;
    // Fp6_2over3_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

    // static my_Fp3<N,T::Fp3_modelConfig> mul_by_non_residue(elem:&my_Fp3<N,T::Fp3_modelConfig>);

    // Fp6_2over3_model cyclotomic_exp(exponent:&bigint<m>) const;

    // static std::usize ceil_size_in_bits() { return 2 * my_Fp3::<N,T::Fp3_modelConfig>::ceil_size_in_bits(); }
    // static std::usize floor_size_in_bits() { return 2 * my_Fp3::<N,T::Fp3_modelConfig>::floor_size_in_bits(); }

    // static constexpr std::usize extension_degree() { return 6; }
    // static constexpr bigint<n> field_char() { return modulus; }

    // static Fp6_2over3_model<n, modulus> zero();
    // static Fp6_2over3_model<n, modulus> one();
    // static Fp6_2over3_model<n, modulus> random_element();

    // friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp6_2over3_model<n, modulus>);
    // friend std::istream& operator>> <n, modulus>(std::istream &in, Fp6_2over3_model<n, modulus> &el);
}

// use crate::algebra::field_utils::field_utils;
// use crate::algebra::scalar_multiplication::wnaf;

impl<const N: usize, T: Fp6_modelConfig<N>> Fp6_2over3_model<N, T> {
    pub fn new(c0: my_Fp3<N, T::Fp3_modelConfig>, c1: my_Fp3<N, T::Fp3_modelConfig>) -> Self {
        Self {
            c0,
            c1,
            _t: PhantomData,
        }
    }

    pub fn mul_by_non_residue(
        elem: &Fp3_model<N, T::Fp3_modelConfig>,
    ) -> Fp3_model<N, T::Fp3_modelConfig> {
        Fp3_model::<N, T::Fp3_modelConfig>::new(elem.c2 * &T::non_residue, elem.c0, elem.c1)
    }

    pub fn zero() -> Self {
        Self::new(
            my_Fp3::<N, T::Fp3_modelConfig>::zero(),
            my_Fp3::<N, T::Fp3_modelConfig>::zero(),
        )
    }

    pub fn one() -> Self {
        Self::new(
            my_Fp3::<N, T::Fp3_modelConfig>::one(),
            my_Fp3::<N, T::Fp3_modelConfig>::zero(),
        )
    }

    pub fn random_element() -> Self {
        Self {
            c0: my_Fp3::<N, T::Fp3_modelConfig>::random_element(),
            c1: my_Fp3::<N, T::Fp3_modelConfig>::random_element(),
            _t: PhantomData,
        }
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn mul_by_2345(&self, other: &Self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.mul_cnt++;
        //#endif
        /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
        assert!(other.c0.c0.is_zero());
        assert!(other.c0.c1.is_zero());

        let (A, B) = (other.c0, other.c1);
        let (a, b) = (self.c0, self.c1);
        let aA = my_Fp3::<N, T::Fp3_modelConfig>::new(
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
            &self.c1.Frobenius_map(power) * &T::Frobenius_coeffs_c1[power % 6],
        )
    }

    pub fn unitary_inverse(&self) -> Self {
        Self::new(self.c0, -self.c1)
    }

    pub fn cyclotomic_squared(&self) -> Self {
        let a = my_Fp2::<N, T::Fp2_modelConfig>::new(self.c0.c0, self.c1.c1);
        //my_Fp a_a = c0.c0; // a = Fp2([c0[0],c1[1]])
        //my_Fp a_b = c1.c1;

        let b = my_Fp2::<N, T::Fp2_modelConfig>::new(self.c1.c0, self.c0.c2);
        //my_Fp b_a = c1.c0; // b = Fp2([c1[0],c0[2]])
        //my_Fp b_b = c0.c2;

        let c = my_Fp2::<N, T::Fp2_modelConfig>::new(self.c0.c1, self.c1.c2);
        //my_Fp c_a = c0.c1; // c = Fp2([c0[1],c1[2]])
        //my_Fp c_b = c1.c2;

        let asq = a.squared();
        let bsq = b.squared();
        let csq = c.squared();

        // A = vector(3*a^2 - 2*Fp2([vector(a)[0],-vector(a)[1]]))
        //my_Fp A_a = my_Fp(3l) * asq_a - my_Fp(2l) * a_a;
        let mut A_a = asq.c0 - a.c0;
        A_a = A_a + A_a + asq.c0;
        //my_Fp A_b = my_Fp(3l) * asq_b + my_Fp(2l) * a_b;
        let mut A_b = asq.c1 + a.c1;
        A_b = A_b + A_b + asq.c1;

        // B = vector(3*Fp2([T::non_residue*c2[1],c2[0]]) + 2*Fp2([vector(b)[0],-vector(b)[1]]))
        //my_Fp B_a = my_Fp(3l) * my_Fp3::<N,T::Fp3_modelConfig>::non_residue * csq_b + my_Fp(2l) * b_a;
        let B_tmp = T::Fp3_modelConfig::non_residue * csq.c1;
        let mut B_a = B_tmp + b.c0;
        B_a = B_a + B_a + B_tmp;

        //my_Fp B_b = my_Fp(3l) * csq_a - my_Fp(2l) * b_b;
        let mut B_b = csq.c0 - b.c1;
        B_b = B_b + B_b + csq.c0;

        // C = vector(3*b^2 - 2*Fp2([vector(c)[0],-vector(c)[1]]))
        //my_Fp C_a = my_Fp(3l) * bsq_a - my_Fp(2l) * c_a;
        let mut C_a = bsq.c0 - c.c0;
        C_a = C_a + C_a + bsq.c0;
        // my_Fp C_b = my_Fp(3l) * bsq_b + my_Fp(2l) * c_b;
        let mut C_b = bsq.c1 + c.c1;
        C_b = C_b + C_b + bsq.c1;

        // e0 = Fp3([A[0],C[0],B[1]])
        // e1 = Fp3([B[0],A[1],C[1]])
        // fin = Fp6e([e0,e1])
        // return fin

        Self::new(
            my_Fp3::<N, T::Fp3_modelConfig>::new(A_a, C_a, B_b),
            my_Fp3::<N, T::Fp3_modelConfig>::new(B_a, A_b, C_b),
        )
    }

    pub fn cyclotomic_exp(&self, exponent: &bigint<N>) -> Self {
        let mut res = Self::one();
        let this_inverse = self.unitary_inverse();

        let mut found_nonzero = false;
        let NAF = find_wnaf(1, exponent);

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
        self.c0.from_words(&words[0..n]) && self.c1.from_words(&words[n..])
    }
}

//
// bool Fp6_2over3_model<n,modulus>::operator==(other:&Fp6_2over3_model<n,modulus>) const
// {
//     return (self.c0 == other.c0 && self.c1 == other.c1);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> PartialEq for Fp6_2over3_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
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
// //#endif
//     Self::new(self.c0 + other.c0,
//                                 self.c1 + other.c1);
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp6_2over3_model<N, T> {
    type Output = Fp6_2over3_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}
//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator-(other:&Fp6_2over3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif
//     Self::new(self.c0 - other.c0,
//                                 self.c1 - other.c1);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> Sub for Fp6_2over3_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> <Fp6_2over3_model<N, T> as Sub>::Output {
        let mut r = self;
        r -= other;
        r
    }
}

//
// Fp6_2over3_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp6_2over3_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     Self::new(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> Mul<&Fp_model<N, T::Fp_modelConfig>>
    for &Fp6_2over3_model<N, T>
{
    type Output = Fp6_2over3_model<N, T>;

    fn mul(self, rhs: &Fp_model<N, T::Fp_modelConfig>) -> Self::Output {
        Fp6_2over3_model::<N, T>::new(&self.c0 * rhs, &self.c1 * rhs)
    }
}

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator*(other:&Fp6_2over3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     B:&my_Fp3<N,T::Fp3_modelConfig> = other.c1, &A = other.c0,
//                  &b = self.c1, &a = self.c0;
//     let aA= a*A;
//     let bB= b*B;
//     let beta_bB= Fp6_2over3_model<n,modulus>::mul_by_non_residue(bB);

//     Self::new(aA + beta_bB,
//                                        (a+b)*(A+B) - aA  - bB);
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> Mul<O> for Fp6_2over3_model<N, T> {
    type Output = Fp6_2over3_model<N, T>;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= *rhs.borrow();
        r
    }
}

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator-() const
// {
//     Self::new(-self.c0,
//                                 -self.c1);
// }
impl<const N: usize, T: Fp6_modelConfig<N>> Neg for Fp6_2over3_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        // mpn_sub_n(r.mont_repr.0.0, modulus.0.0, self.mont_repr.0.0, n);
        r
    }
}

//
// Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp6_2over3_model<n, modulus> >(*this, pow);
// }

impl<const N: usize, T: Fp6_modelConfig<N>> BitXor<u64> for Fp6_2over3_model<N, T> {
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
// Fp6_2over3_model<n, modulus> Fp6_2over3_model<n,modulus>::operator^(exponent:&bigint<m>) const
// {
//     return power<Fp6_2over3_model<n, modulus>, m>(*this, exponent);
// }
impl<const N: usize, const M: usize, T: Fp6_modelConfig<N>> BitXor<&bigint<M>>
    for Fp6_2over3_model<N, T>
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
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> AddAssign<O>
    for Fp6_2over3_model<N, T>
{
    fn add_assign(&mut self, other: O) {}
}

//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator-=(const Fp6_2over3_model<n,modulus>& other)
// {
//     *self = *this - other;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> SubAssign<O>
    for Fp6_2over3_model<N, T>
{
    fn sub_assign(&mut self, other: O) {}
}
//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator*=(const Fp6_2over3_model<n,modulus>& other)
// {
//     *self = *this * other;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>, O: Borrow<Self>> MulAssign<O>
    for Fp6_2over3_model<N, T>
{
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
    }
}

//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator^=(const u64 pow)
// {
//     *self = *this ^ pow;
//     return *self;
// }
impl<const N: usize, T: Fp6_modelConfig<N>> BitXorAssign<u64> for Fp6_2over3_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        // *self = Powers::power::<Fp6_2over3_model<N, T>>(self, rhs);
    }
}
//
//
// Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     *self = *this ^ pow;
//     return *self;
// }

impl<const N: usize, const M: usize, T: Fp6_modelConfig<N>> BitXorAssign<&bigint<M>>
    for Fp6_2over3_model<N, T>
{
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        //*self = Powers::power::<Fp6_2over3_model<N, T>>(self, rhs);
    }
}

//
// std::ostream& operator<<(std::ostream &out, el:&Fp6_2over3_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

use std::fmt;
impl<const N: usize, T: Fp6_modelConfig<N>> fmt::Display for Fp6_2over3_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c0)
    }
}
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
use ark_std::Zero;
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
