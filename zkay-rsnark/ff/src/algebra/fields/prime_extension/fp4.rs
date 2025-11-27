// /** @file
//  *****************************************************************************

//  Declaration of interfaces for the (extension) field Fp4.

//  The field Fp4 equals Fp2[V]/(V^2-U) where Fp2 = Fp[U]/(U^2-non_residue) and non_residue is in Fp.

//  ASSUMPTION: the modulus p is 1 mod 6.

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// //#ifndef FP4_HPP_
// // #define FP4_HPP_

// use crate::algebra::fields::prime_base::fp;
// use crate::algebra::fields::prime_extension::fp2;

// // namespace libff {

//
// pub struct Fp4_model;

//
// std::ostream& operator<<(std::ostream &, const Fp4_model<n, modulus> &);

//
// std::istream& operator>>(std::istream &, Fp4_model<n, modulus> &);

//
// pub struct Fp4_model {
//
//     type my_Fp=Fp_model<n, modulus>;
//     type my_Fp2=Fp2_model<n, modulus>;
//     type my_Fpe=my_Fp2;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static i64 add_cnt;
//     static i64 sub_cnt;
//     static i64 mul_cnt;
//     static i64 sqr_cnt;
//     static i64 inv_cnt;
// //#endif

//     static bigint<4*n> euler; // (modulus^4-1)/2
//     static std::usize s; // modulus^4 = 2^s * t + 1
//     static bigint<4*n> t; // with t odd
//     static bigint<4*n> t_minus_1_over_2; // (t-1)/2
//     static Fp4_model<n, modulus> nqr; // a quadratic nonresidue in Fp4
//     static Fp4_model<n, modulus> nqr_to_t; // nqr^t
//     static my_Fp non_residue;
//     static my_Fp Frobenius_coeffs_c1[4]; // non_residue^((modulus^i-1)/4) for i=0,1,2,3

//     my_Fp2 c0, c1;
//     Fp4_model() {};
//     Fp4_model(c0:my_Fp2&, c1:&my_Fp2)->Selfc0,c1 {};

//     pub fn  print() const { print!("c0/c1:\n"); c0.print(); c1.print(); }
//     pub fn  clear() { c0.clear(); c1.clear(); }
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

//     bool is_zero() const { return c0.is_zero() && c1.is_zero(); }
//     bool operator==(other:&Fp4_model) const;
//     bool operator!=(other:&Fp4_model) const;

//     Fp4_model& operator+=(other:&Fp4_model);
//     Fp4_model& operator-=(other:&Fp4_model);
//     Fp4_model& operator*=(other:&Fp4_model);
//     Fp4_model& operator^=(const u64 pow);
//
//     Fp4_model& operator^=(pow:&bigint<m>);

//     Fp4_model operator+(other:&Fp4_model) const;
//     Fp4_model operator-(other:&Fp4_model) const;
//     Fp4_model operator*(other:&Fp4_model) const;
//     Fp4_model mul_by_023(other:&Fp4_model) const;
//     Fp4_model operator^(const:u64 pow),
//
//     Fp4_model operator^(exponent:&bigint<m>) const;
//
//     Fp4_model operator^(exponent:&Fp_model<m, modulus_p>) const;
//     Fp4_model operator-() const;

//     Fp4_model& square();
//     Fp4_model squared() const;
//     Fp4_model& invert();
//     Fp4_model inverse() const;
//     Fp4_model Frobenius_map(u64 power) const;
//     Fp4_model unitary_inverse() const;
//     Fp4_model cyclotomic_squared() const;
//     Fp4_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

//     static my_Fp2 mul_by_non_residue(elt:&my_Fp2);

//
//     Fp4_model cyclotomic_exp(exponent:&bigint<m>) const;

//     static std::usize ceil_size_in_bits() { return 2 * my_Fp2::ceil_size_in_bits(); }
//     static std::usize floor_size_in_bits() { return 2 * my_Fp2::floor_size_in_bits(); }

//     static constexpr std::usize extension_degree() { return 4; }
//     static constexpr bigint<n> field_char() { return modulus; }

//     static Fp4_model<n, modulus> zero();
//     static Fp4_model<n, modulus> one();
//     static Fp4_model<n, modulus> random_element();

//     friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp4_model<n, modulus>);
//     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp4_model<n, modulus> &el);
// };

// // #ifdef PROFILE_OP_COUNTS
//
// i64 Fp4_model<n, modulus>::add_cnt = 0;

//
// i64 Fp4_model<n, modulus>::sub_cnt = 0;

//
// i64 Fp4_model<n, modulus>::mul_cnt = 0;

//
// i64 Fp4_model<n, modulus>::sqr_cnt = 0;

//
// i64 Fp4_model<n, modulus>::inv_cnt = 0;
// //#endif

//
// Fp4_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp4_model<n, modulus>);

//
// Fp4_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp4_model<n, modulus>);

//
// bigint<4*n> Fp4_model<n, modulus>::euler;

//
// usize Fp4_model<n, modulus>::s;

//
// bigint<4*n> Fp4_model<n, modulus>::t;

//
// bigint<4*n> Fp4_model<n, modulus>::t_minus_1_over_2;

//
// Fp4_model<n, modulus> Fp4_model<n, modulus>::nqr;

//
// Fp4_model<n, modulus> Fp4_model<n, modulus>::nqr_to_t;

//
// Fp_model<n, modulus> Fp4_model<n, modulus>::non_residue;

//
// Fp_model<n, modulus> Fp4_model<n, modulus>::Frobenius_coeffs_c1[4];

// // } // namespace libff

// use crate::algebra::fields::prime_extension::fp4.tcc;

// //#endif // FP4_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of interfaces for the (extension) field Fp4.

//  See fp4.hpp .

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// //#ifndef FP4_TCC_
// // #define FP4_TCC_

// use crate::algebra::field_utils::field_utils;
// use crate::algebra::scalar_multiplication::wnaf;

// // namespace libff {

//
// Fp2_model<n, modulus> Fp4_model<n, modulus>::mul_by_non_residue(elt:&Fp2_model<n, modulus>)
// {
//     return Fp2_model<n, modulus>(non_residue * elt.c1, elt.c0);
// }

//
// Fp4_model<n, modulus> Fp4_model<n, modulus>::zero()
// {
//     return Fp4_model<n,modulus>(my_Fp2::zero(),
//                                 my_Fp2::zero());
// }

//
// Fp4_model<n, modulus> Fp4_model<n, modulus>::one()
// {
//     return Fp4_model<n,modulus>(my_Fp2::one(),
//                                 my_Fp2::zero());
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::random_element()
// {
//     Fp4_model<n, modulus> r;
//     r.c0 = my_Fp2::random_element();
//     r.c1 = my_Fp2::random_element();

//     return r;
// }

//
// pub fn randomize()
// {
//     (*this) = Fp4_model<n, modulus>::random_element();
// }

//
// bool Fp4_model<n,modulus>::operator==(other:&Fp4_model<n,modulus>) const
// {
//     return (this->c0 == other.c0 && this->c1 == other.c1);
// }

//
// bool Fp4_model<n,modulus>::operator!=(other:&Fp4_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator+(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(this->c0 + other.c0,
//                                 this->c1 + other.c1);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(this->c0 - other.c0,
//                                 this->c1 - other.c1);
// }

//
// Fp4_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp4_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

//
// Fp4_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp4_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator*(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     B:&my_Fp2 = other.c1, &A = other.c0,
//         &b = this->c1, &a = this->c0;
//     let aA= a*A;
//     let bB= b*B;

//     let beta_bB= Fp4_model<n,modulus>::mul_by_non_residue(bB);
//     return Fp4_model<n,modulus>(aA + beta_bB,
//                                 (a+b)*(A+B) - aA  - bB);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::mul_by_023(other:&Fp4_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     assert!(other.c0.c1.is_zero());

//     B:&my_Fp2 = other.c1, &A = other.c0,
//         &b = this->c1, &a = this->c0;
//     let aA= my_Fp2(a.c0 * A.c0, a.c1 * A.c0);
//     let bB= b*B;

//     let beta_bB= Fp4_model<n,modulus>::mul_by_non_residue(bB);
//     return Fp4_model<n,modulus>(aA + beta_bB,
//                                 (a+b)*(A+B) - aA  - bB);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-() const
// {
//     return Fp4_model<n,modulus>(-this->c0,
//                                 -this->c1);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp4_model<n, modulus> >(*this, pow);
// }

//
//
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(exponent:&bigint<m>) const
// {
//     return power<Fp4_model<n, modulus> >(*this, exponent);
// }

//
//
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(exponent:&Fp_model<m, modulus_p>) const
// {
//     return (*this)^(exponent.as_bigint());
// }

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator+=(const Fp4_model<n,modulus>& other)
// {
//     (*this) = *this + other;
//     return (*this);
// }

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator-=(const Fp4_model<n,modulus>& other)
// {
//     (*this) = *this - other;
//     return (*this);
// }

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator*=(const Fp4_model<n,modulus>& other)
// {
//     (*this) = *this * other;
//     return (*this);
// }

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::squared() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex) */
//     b:&my_Fp2 = this->c1, &a = this->c0;
//     let ab= a * b;

//     return Fp4_model<n,modulus>((a+b)*(a+Fp4_model<n,modulus>::mul_by_non_residue(b))-ab-Fp4_model<n,modulus>::mul_by_non_residue(ab),
//                                 ab + ab);
// }

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::square()
// {
//     (*this) = squared();
//     return (*this);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::inverse() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
// //#endif
//     /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */
//     b:&my_Fp2 = this->c1, &a = this->c0;
//     let t1= b.squared();
//     let t0= a.squared() - Fp4_model<n,modulus>::mul_by_non_residue(t1);
//     let new_t1= t0.inverse();

//     return Fp4_model<n,modulus>(a * new_t1, - (b * new_t1));
// }

//
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::Frobenius_map(u64 power) const
// {
//     return Fp4_model<n,modulus>(c0.Frobenius_map(power),
//                                 Frobenius_coeffs_c1[power % 4] * c1.Frobenius_map(power));
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::unitary_inverse() const
// {
//     return Fp4_model<n,modulus>(this->c0,
//                                 -this->c1);
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::cyclotomic_squared() const
// {
//     let A= this->c1.squared();
//     let B= this->c1 + this->c0;
//     let C= B.squared() - A;
//     non_residue:my_Fp2 D = Fp4_model<n,modulus>::mul_by_non_residue(A); // Fp2(A.c1 *, A.c0)
//     let E= C - D;
//     let F= D + D + my_Fp2::one();
//     let G= E - my_Fp2::one();

//     return Fp4_model<n,modulus>(F, G);
// }

//
//
// Fp4_model<n, modulus> Fp4_model<n,modulus>::cyclotomic_exp(exponent:&bigint<m>) const
// {
//     Fp4_model<n,modulus> res = Fp4_model<n,modulus>::one();
//     Fp4_model<n,modulus> this_inverse = this->unitary_inverse();

//     bool found_nonzero = false;
//     Vec<long> NAF = find_wnaf(1, exponent);

//     for i in ( 0..=static_cast<long>(NAF.len() - 1)).rev()
//     {
//         if found_nonzero
//         {
//             res = res.cyclotomic_squared();
//         }

//         if NAF[i] != 0
//         {
//             found_nonzero = true;

//             if NAF[i] > 0
//             {
//                 res = res * (*this);
//             }
//             else
//             {
//                 res = res * this_inverse;
//             }
//         }
//     }

//     return res;
// }

//
// Fp4_model<n,modulus> Fp4_model<n,modulus>::sqrt() const
// {
//     return tonelli_shanks_sqrt(*this);
// }

//
// Vec<uint64_t> Fp4_model<n,modulus>::to_words() const
// {
//     Vec<uint64_t> words = c0.to_words();
//     Vec<uint64_t> words1 = c1.to_words();
//     words.insert(words.end(), words1.begin(), words1.end());
//     return words;
// }

//
// bool Fp4_model<n,modulus>::from_words(Vec<uint64_t> words)
// {
//     Vec<uint64_t>::const_iterator vec_start = words.begin();
//     Vec<uint64_t>::const_iterator vec_center = words.begin() + words.len() / 2;
//     Vec<uint64_t>::const_iterator vec_end = words.end();
//     Vec<uint64_t> words0(vec_start, vec_center);
//     Vec<uint64_t> words1(vec_center, vec_end);
//     // Fp_model's from_words() takes care of asserts about vector length.
//     return c0.from_words(words0) && c1.from_words(words1);
// }

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

// // } // namespace libff

// //#endif // FP4_TCC_
use super::quadratic_extension::{QuadExtConfig, QuadExtField};
use crate::algebra::fields::{
    cyclotomic::CyclotomicMultSubgroup,
    prime_extension::fp2::{Fp2, Fp2Config},
};
// use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use ark_std::Zero;
use core::{marker::PhantomData, ops::Not};

pub trait Fp4Config: 'static + Send + Sync {
    type Fp2Config: Fp2Config;

    /// This *must* equal (0, 1);
    /// see [[DESD06, Section 5.1]](https://eprint.iacr.org/2006/471.pdf).
    const NONRESIDUE: Fp2<Self::Fp2Config>;

    /// Coefficients for the Frobenius automorphism.
    /// non_residue^((modulus^i-1)/4) for i=0,1,2,3
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
