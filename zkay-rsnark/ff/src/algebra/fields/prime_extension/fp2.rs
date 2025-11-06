/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[p^2].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP2_HPP_
// #define FP2_HPP_
//#include <vector>

use crate::algebra::fields::{prime_base::fp::Fp_modelConfig};
// namespace libff {

// 
// pub struct Fp2_model;

// 
// std::ostream& operator<<(std::ostream &, const Fp2_model<n, modulus> &);

// 
// std::istream& operator>>(std::istream &, Fp2_model<n, modulus> &);

/**
 * Arithmetic in the field F[p^2].
 *
 * Let p := modulus. This interface provides arithmetic for the extension field
 * Fp2 = Fp[U]/(U^2-non_residue), where non_residue is in Fp.
 *
 * ASSUMPTION: p = 1 (mod 6)
 */
// 
pub trait Fp2_modelConfig: 'static + Send + Sync + Sized {
    type my_Fp:PrimeField;
    const non_residue: Self::my_Fp;

    const nqr: (Self::my_Fp, Self::my_Fp);
    const nqr_to_t: (Self::my_Fp, Self::my_Fp);
    /// non_residue^((modulus^i-1)/2)
    const Frobenius_coeffs_c1: [Self::my_Fp; 2];
}


// pub struct  Fp2_model {
// 
    // type my_Fp=Fp_model<n, modulus>;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static i64 add_cnt;
//     static i64 sub_cnt;
//     static i64 mul_cnt;
//     static i64 sqr_cnt;
//     static i64 inv_cnt;
// //#endif

//     static bigint<2*n> euler; // (modulus^2-1)/2
//     static std::usize s;       // modulus^2 = 2^s * t + 1
//     static bigint<2*n> t;  // with t odd
//     static bigint<2*n> t_minus_1_over_2; // (t-1)/2
//     static my_Fp non_residue; // X^4-non_residue irreducible over Fp; used for constructing Fp2 = Fp[X] / (X^2 - non_residue)
//     static Fp2_model<n, modulus> nqr; // a quadratic nonresidue in Fp2
//     static Fp2_model<n, modulus> nqr_to_t; // nqr^t
//     static my_Fp Frobenius_coeffs_c1[2]; // non_residue^((modulus^i-1)/2)

    //  c0:my_Fp, c1:my_Fp;
// }
//     Fp2_model() {};
//     Fp2_model(c0:my_Fp&, c1:&my_Fp)->Selfc0,c1 {};

//     pub fn  clear() { c0.clear(); c1.clear(); }
//     pub fn  print() const { print!("c0/c1:\n"); c0.print(); c1.print(); }
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
//     bool operator==(other:&Fp2_model) const;
//     bool operator!=(other:&Fp2_model) const;

//     Fp2_model& operator+=(other:&Fp2_model);
//     Fp2_model& operator-=(other:&Fp2_model);
//     Fp2_model& operator*=(other:&Fp2_model);
//     Fp2_model& operator^=(const u64 pow);
//     
//     Fp2_model& operator^=(pow:&bigint<m>);

//     Fp2_model operator+(other:&Fp2_model) const;
//     Fp2_model operator-(other:&Fp2_model) const;
//     Fp2_model operator*(other:&Fp2_model) const;
//     Fp2_model operator^(const:u64 pow),
//     
//     Fp2_model operator^(other:&bigint<m>) const;
//     Fp2_model operator-() const;

//     Fp2_model& square(); // default is squared_complex
//     Fp2_model squared() const; // default is squared_complex
//     Fp2_model& invert();
//     Fp2_model inverse() const;
//     Fp2_model Frobenius_map(u64 power) const;
//     Fp2_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)
//     Fp2_model squared_karatsuba() const;
//     Fp2_model squared_complex() const;

//     static std::usize ceil_size_in_bits() { return 2 * my_Fp::ceil_size_in_bits(); }
//     static std::usize floor_size_in_bits() { return 2 * my_Fp::floor_size_in_bits(); }

//     static constexpr std::usize extension_degree() { return 2; }
//     static constexpr bigint<n> field_char() { return modulus; }

//     static Fp2_model<n, modulus> zero();
//     static Fp2_model<n, modulus> one();
//     static Fp2_model<n, modulus> random_element();

//     friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp2_model<n, modulus>);
//     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp2_model<n, modulus> &el);
// };


//#endif // FP2_HPP_
/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[p^2].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP2_TCC_
// #define FP2_TCC_

use crate::algebra::field_utils::field_utils;

// namespace libff {

// using std::usize;

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::zero()
// {
//     return Fp2_model<n, modulus>(my_Fp::zero(), my_Fp::zero());
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::one()
// {
//     return Fp2_model<n, modulus>(my_Fp::one(), my_Fp::zero());
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::random_element()
// {
//     Fp2_model<n, modulus> r;
//     r.c0 = my_Fp::random_element();
//     r.c1 = my_Fp::random_element();

//     return r;
// }

// 
// pub fn randomize()
// {
//     (*this) = Fp2_model<n, modulus>::random_element();
// }

// 
// bool Fp2_model<n,modulus>::operator==(other:&Fp2_model<n,modulus>) const
// {
//     return (this->c0 == other.c0 && this->c1 == other.c1);
// }

// 
// bool Fp2_model<n,modulus>::operator!=(other:&Fp2_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator+(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     return Fp2_model<n,modulus>(this->c0 + other.c0,
//                                 this->c1 + other.c1);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     return Fp2_model<n,modulus>(this->c0 - other.c0,
//                                 this->c1 - other.c1);
// }

// 
// Fp2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp2_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator*(other:&Fp2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     const my_Fp
//         &A = other.c0, &B = other.c1,
//         &a = this->c0, &b = this->c1;
//     let aA= a * A;
//     let bB= b * B;

//     return Fp2_model<n,modulus>(aA + non_residue * bB,
//                                 (a + b)*(A+B) - aA - bB);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-() const
// {
//     return Fp2_model<n,modulus>(-this->c0,
//                                 -this->c1);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp2_model<n, modulus>>(*this, pow);
// }

// 
// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp2_model<n, modulus>, m>(*this, pow);
// }

// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator+=(const Fp2_model<n,modulus>& other)
// {
//     (*this) = *this + other;
//     return (*this);
// }

// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator-=(const Fp2_model<n,modulus>& other)
// {
//     (*this) = *this - other;
//     return (*this);
// }

// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator*=(const Fp2_model<n,modulus>& other)
// {
//     (*this) = *this * other;
//     return (*this);
// }

// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// 
// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::squared() const
// {
//     return squared_complex();
// }

// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::square()
// {
//     (*this) = squared();
//     return (*this);
// }


// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::squared_karatsuba() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba squaring) */
//     a:&my_Fp = this->c0, &b = this->c1;
//     let asq= a.squared();
//     let bsq= b.squared();

//     return Fp2_model<n,modulus>(asq + non_residue * bsq,
//                                 (a + b).squared() - asq - bsq);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::squared_complex() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex squaring) */
//     a:&my_Fp = this->c0, &b = this->c1;
//     let ab= a * b;

//     return Fp2_model<n,modulus>((a + b) * (a + non_residue * b) - ab - non_residue * ab,
//                                 ab + ab);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::inverse() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
// //#endif
//     a:&my_Fp = this->c0, &b = this->c1;

//     /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */
//     let t0= a.squared();
//     let t1= b.squared();
//     let t2= t0 - non_residue * t1;
//     let t3= t2.inverse();
//     let c0= a * t3;
//     let c1= - (b * t3);

//     return Fp2_model<n,modulus>(c0, c1);
// }

// 
// Fp2_model<n,modulus>& Fp2_model<n,modulus>::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::Frobenius_map(u64 power) const
// {
//     return Fp2_model<n,modulus>(c0,
//                                 Frobenius_coeffs_c1[power % 2] * c1);
// }

// 
// Fp2_model<n,modulus> Fp2_model<n,modulus>::sqrt() const
// {
//     return tonelli_shanks_sqrt(*this);
// }

// 
// Vec<uint64_t> Fp2_model<n,modulus>::to_words() const
// {
//     Vec<uint64_t> words = c0.to_words();
//     Vec<uint64_t> words1 = c1.to_words();
//     words.insert(words.end(), words1.begin(), words1.end());
//     return words;
// }

// 
// bool Fp2_model<n,modulus>::from_words(Vec<uint64_t> words)
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
// std::ostream& operator<<(std::ostream &out, el:&Fp2_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

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

// } // namespace libff
//#endif // FP2_TCC_


use super::quadratic_extension::{QuadExtConfig, QuadExtField};
use crate::algebra::fields::{fpn_field::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use ark_std::Zero;
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
