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

use crate::algebra::fields::prime_base::fp;

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
pub struct  Fp2_model {
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
//     static my_Fp Frobenius_coeffs_c1[2]; // non_residue^((modulus^i-1)/2) for i=0,1

    //  c0:my_Fp, c1:my_Fp;
}
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

// #ifdef PROFILE_OP_COUNTS
// 
// i64 Fp2_model<n, modulus>::add_cnt = 0;

// 
// i64 Fp2_model<n, modulus>::sub_cnt = 0;

// 
// i64 Fp2_model<n, modulus>::mul_cnt = 0;

// 
// i64 Fp2_model<n, modulus>::sqr_cnt = 0;

// 
// i64 Fp2_model<n, modulus>::inv_cnt = 0;
// //#endif

// 
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp2_model<n, modulus> >);

// 
// std::istream& operator>>(std::istream& in, Vec<Fp2_model<n, modulus> > &v);

// 
// Fp2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp2_model<n, modulus>);

// 
// bigint<2*n> Fp2_model<n, modulus>::euler;

// 
// usize Fp2_model<n, modulus>::s;

// 
// bigint<2*n> Fp2_model<n, modulus>::t;

// 
// bigint<2*n> Fp2_model<n, modulus>::t_minus_1_over_2;

// 
// Fp_model<n, modulus> Fp2_model<n, modulus>::non_residue;

// 
// Fp2_model<n, modulus> Fp2_model<n, modulus>::nqr;

// 
// Fp2_model<n, modulus> Fp2_model<n, modulus>::nqr_to_t;

// 
// Fp_model<n, modulus> Fp2_model<n, modulus>::Frobenius_coeffs_c1[2];

// } // namespace libff
// use crate::algebra::fields::prime_extension::fp2.tcc;

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
