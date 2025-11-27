// /** @file
//  *****************************************************************************
//  Declaration of arithmetic in the finite  field F[p^3].
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// //#ifndef FP3_HPP_
// // #define FP3_HPP_
// //#include <vector>

// use crate::algebra::fields::prime_base::fp;

// // namespace libff {

// /**
//  * Arithmetic in the field F[p^3].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  * Fp3 = Fp[U]/(U^3-non_residue), where non_residue is in Fp.
//  *
//  * ASSUMPTION: p = 1 (mod 6)
//  */
//
// pub struct Fp3_model {

//     type my_Fp=Fp_model<n, modulus>;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static i64 add_cnt;
//     static i64 sub_cnt;
//     static i64 mul_cnt;
//     static i64 sqr_cnt;
//     static i64 inv_cnt;
// //#endif

//     static bigint<3*n> euler; // (modulus^3-1)/2
//     static std::usize s;       // modulus^3 = 2^s * t + 1
//     static bigint<3*n> t;  // with t odd
//     static bigint<3*n> t_minus_1_over_2; // (t-1)/2
//     static my_Fp non_residue; // X^6-non_residue irreducible over Fp; used for constructing Fp3 = Fp[X] / (X^3 - non_residue)
//     static Fp3_model<n, modulus> nqr; // a quadratic nonresidue in Fp3
//     static Fp3_model<n, modulus> nqr_to_t; // nqr^t
//     static my_Fp Frobenius_coeffs_c1[3]; // non_residue^((modulus^i-1)/3)   for i=0,1,2
//     static my_Fp Frobenius_coeffs_c2[3]; // non_residue^((2*modulus^i-2)/3) for i=0,1,2

//     my_Fp c0, c1, c2;
//     Fp3_model() {};
//     Fp3_model(c0:my_Fp&, c1:my_Fp&, c2:&my_Fp)->Selfc0,c1,c2 {};

//     pub fn  clear() { c0.clear(); c1.clear(); c2.clear(); }
//     pub fn  print() const { print!("c0/c1/c2:\n"); c0.print(); c1.print(); c2.print(); }
//     pub fn  randomize();

//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     Vec<uint64_t> to_words() const;
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Returns true when the right-most bits of each element represent a value less than the modulus.
//      */
//     bool from_words(Vec<uint64_t> words);

//     bool is_zero() const { return c0.is_zero() && c1.is_zero() && c2.is_zero(); }
//     bool operator==(other:&Fp3_model) const;
//     bool operator!=(other:&Fp3_model) const;

//     Fp3_model& operator+=(other:&Fp3_model);
//     Fp3_model& operator-=(other:&Fp3_model);
//     Fp3_model& operator*=(other:&Fp3_model);
//     Fp3_model& operator^=(const u64 pow);
//
//     Fp3_model& operator^=(pow:&bigint<m>);

//     Fp3_model operator+(other:&Fp3_model) const;
//     Fp3_model operator-(other:&Fp3_model) const;
//     Fp3_model operator*(other:&Fp3_model) const;
//     Fp3_model operator^(const:u64 pow),
//
//     Fp3_model operator^(other:&bigint<m>) const;
//     Fp3_model operator-() const;

//     Fp3_model& square();
//     Fp3_model squared() const;
//     Fp3_model& invert();
//     Fp3_model inverse() const;
//     Fp3_model Frobenius_map(u64 power) const;
//     Fp3_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

//     static std::usize ceil_size_in_bits() { return 3 * my_Fp::ceil_size_in_bits(); }
//     static std::usize floor_size_in_bits() { return 3 * my_Fp::floor_size_in_bits(); }

//     static constexpr std::usize extension_degree() { return 3; }
//     static constexpr bigint<n> field_char() { return modulus; }

//     static Fp3_model<n, modulus> zero();
//     static Fp3_model<n, modulus> one();
//     static Fp3_model<n, modulus> random_element();

//     friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp3_model<n, modulus>);
//     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp3_model<n, modulus> &el);
// };

// // #ifdef PROFILE_OP_COUNTS
//
// i64 Fp3_model<n, modulus>::add_cnt = 0;

//
// i64 Fp3_model<n, modulus>::sub_cnt = 0;

//
// i64 Fp3_model<n, modulus>::mul_cnt = 0;

//
// i64 Fp3_model<n, modulus>::sqr_cnt = 0;

//
// i64 Fp3_model<n, modulus>::inv_cnt = 0;
// //#endif

//
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp3_model<n, modulus> >);

//
// std::istream& operator>>(std::istream& in, Vec<Fp3_model<n, modulus> > &v);

//
// Fp3_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp3_model<n, modulus>);

//
// bigint<3*n> Fp3_model<n, modulus>::euler;

//
// usize Fp3_model<n, modulus>::s;

//
// bigint<3*n> Fp3_model<n, modulus>::t;

//
// bigint<3*n> Fp3_model<n, modulus>::t_minus_1_over_2;

//
// Fp_model<n, modulus> Fp3_model<n, modulus>::non_residue;

//
// Fp3_model<n, modulus> Fp3_model<n, modulus>::nqr;

//
// Fp3_model<n, modulus> Fp3_model<n, modulus>::nqr_to_t;

//
// Fp_model<n, modulus> Fp3_model<n, modulus>::Frobenius_coeffs_c1[3];

//
// Fp_model<n, modulus> Fp3_model<n, modulus>::Frobenius_coeffs_c2[3];

// // } // namespace libff
// use crate::algebra::fields::prime_extension::fp3.tcc;

// //#endif // FP3_HPP_
// /** @file
//  *****************************************************************************
//  Implementation of arithmetic in the finite field F[p^3].
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// //#ifndef FP3_TCC_
// // #define FP3_TCC_

// use crate::algebra::field_utils::field_utils;

// // namespace libff {

// using std::usize;

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::zero()
// {
//     return Fp3_model<n, modulus>(my_Fp::zero(), my_Fp::zero(), my_Fp::zero());
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::one()
// {
//     return Fp3_model<n, modulus>(my_Fp::one(), my_Fp::zero(), my_Fp::zero());
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::random_element()
// {
//     Fp3_model<n, modulus> r;
//     r.c0 = my_Fp::random_element();
//     r.c1 = my_Fp::random_element();
//     r.c2 = my_Fp::random_element();

//     return r;
// }

//
// pub fn randomize()
// {
//     (*this) = Fp3_model<n, modulus>::random_element();
// }

//
// bool Fp3_model<n,modulus>::operator==(other:&Fp3_model<n,modulus>) const
// {
//     return (this->c0 == other.c0 && this->c1 == other.c1 && this->c2 == other.c2);
// }

//
// bool Fp3_model<n,modulus>::operator!=(other:&Fp3_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator+(other:&Fp3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     return Fp3_model<n,modulus>(this->c0 + other.c0,
//                                 this->c1 + other.c1,
//                                 this->c2 + other.c2);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator-(other:&Fp3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     return Fp3_model<n,modulus>(this->c0 - other.c0,
//                                 this->c1 - other.c1,
//                                 this->c2 - other.c2);
// }

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

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator*(other:&Fp3_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (Karatsuba) */
//     const my_Fp
//         &A = other.c0, &B = other.c1, &C = other.c2,
//         &a = this->c0, &b = this->c1, &c = this->c2;
//     let aA= a*A;
//     let bB= b*B;
//     let cC= c*C;

//     return Fp3_model<n,modulus>(aA + non_residue*((b+c)*(B+C)-bB-cC),
//                                 (a+b)*(A+B)-aA-bB+non_residue*cC,
//                                 (a+c)*(A+C)-aA+bB-cC);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator-() const
// {
//     return Fp3_model<n,modulus>(-this->c0,
//                                 -this->c1,
//                                 -this->c2);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp3_model<n, modulus> >(*this, pow);
// }

//
//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp3_model<n, modulus> >(*this, pow);
// }

//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator+=(const Fp3_model<n,modulus>& other)
// {
//     (*this) = *this + other;
//     return (*this);
// }

//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator-=(const Fp3_model<n,modulus>& other)
// {
//     (*this) = *this - other;
//     return (*this);
// }

//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator*=(const Fp3_model<n,modulus>& other)
// {
//     (*this) = *this * other;
//     return (*this);
// }

//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::squared() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (CH-SQR2) */
//     const my_Fp
//         &a = this->c0, &b = this->c1, &c = this->c2;
//     let s0= a.squared();
//     let ab= a*b;
//     let s1= ab + ab;
//     let s2= (a - b + c).squared();
//     let bc= b*c;
//     let s3= bc + bc;
//     let s4= c.squared();

//     return Fp3_model<n,modulus>(s0 + non_residue * s3,
//                                 s1 + non_residue * s4,
//                                 s1 + s2 + s3 - s0 - s4);
// }

//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::square()
// {
//     (*this) = squared();
//     return (*this);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::inverse() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
// //#endif
//     const my_Fp
//         &a = this->c0, &b = this->c1, &c = this->c2;

//     /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17 */
//     let t0= a.squared();
//     let t1= b.squared();
//     let t2= c.squared();
//     let t3= a*b;
//     let t4= a*c;
//     let t5= b*c;
//     let c0= t0 - non_residue * t5;
//     let c1= non_residue * t2 - t3;
//     Scott:my_Fp c2 = t1 - t4; // typo in paper referenced above. should be "-" as per, but is "*"
//     let t6= (a * c0 + non_residue * (c * c1 + b * c2)).inverse();
//     return Fp3_model<n,modulus>(t6 * c0, t6 * c1, t6 * c2);
// }

//
// Fp3_model<n,modulus>& Fp3_model<n,modulus>::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::Frobenius_map(u64 power) const
// {
//     return Fp3_model<n,modulus>(c0,
//                                 Frobenius_coeffs_c1[power % 3] * c1,
//                                 Frobenius_coeffs_c2[power % 3] * c2);
// }

//
// Fp3_model<n,modulus> Fp3_model<n,modulus>::sqrt() const
// {
//     return tonelli_shanks_sqrt(*this);
// }

//
// Vec<uint64_t> Fp3_model<n,modulus>::to_words() const
// {
//     Vec<uint64_t> words = c0.to_words();
//     Vec<uint64_t> words1 = c1.to_words();
//     Vec<uint64_t> words2 = c2.to_words();
//     words.insert(words.end(), words1.begin(), words1.end());
//     words.insert(words.end(), words2.begin(), words2.end());
//     return words;
// }

//
// bool Fp3_model<n,modulus>::from_words(Vec<uint64_t> words)
// {
//     Vec<uint64_t>::const_iterator vec_start = words.begin();
//     Vec<uint64_t>::const_iterator vec_center1 = words.begin() + words.len() / 3;
//     Vec<uint64_t>::const_iterator vec_center2 = words.begin() + 2 * words.len() / 3;
//     Vec<uint64_t>::const_iterator vec_end = words.end();
//     Vec<uint64_t> words0(vec_start, vec_center1);
//     Vec<uint64_t> words1(vec_center1, vec_center2);
//     Vec<uint64_t> words2(vec_center2, vec_end);
//     // Fp_model's from_words() takes care of asserts about vector length.
//     return c0.from_words(words0) && c1.from_words(words1) && c2.from_words(words2);
// }

//
// std::ostream& operator<<(std::ostream &out, el:&Fp3_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1 << OUTPUT_SEPARATOR << el.c2;
//     return out;
// }

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

// // } // namespace libff
// //#endif // FP3_TCC_
use super::cubic_extension::{CubicExtConfig, CubicExtField};
use crate::algebra::fields::{
    cyclotomic::CyclotomicMultSubgroup, fpn_field::PrimeField, sqrt::SqrtPrecomputation,
};
// use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use ark_std::Zero;
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
