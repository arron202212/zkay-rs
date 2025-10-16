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

// template<mp_size_t n, const bigint<n>& modulus>
// class Fp4_model;

// template<mp_size_t n, const bigint<n>& modulus>
// std::ostream& operator<<(std::ostream &, const Fp4_model<n, modulus> &);

// template<mp_size_t n, const bigint<n>& modulus>
// std::istream& operator>>(std::istream &, Fp4_model<n, modulus> &);

// template<mp_size_t n, const bigint<n>& modulus>
// class Fp4_model {
// public:
//     typedef Fp_model<n, modulus> my_Fp;
//     typedef Fp2_model<n, modulus> my_Fp2;
//     typedef my_Fp2 my_Fpe;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static long long add_cnt;
//     static long long sub_cnt;
//     static long long mul_cnt;
//     static long long sqr_cnt;
//     static long long inv_cnt;
// //#endif

//     static bigint<4*n> euler; // (modulus^4-1)/2
//     static std::size_t s; // modulus^4 = 2^s * t + 1
//     static bigint<4*n> t; // with t odd
//     static bigint<4*n> t_minus_1_over_2; // (t-1)/2
//     static Fp4_model<n, modulus> nqr; // a quadratic nonresidue in Fp4
//     static Fp4_model<n, modulus> nqr_to_t; // nqr^t
//     static my_Fp non_residue;
//     static my_Fp Frobenius_coeffs_c1[4]; // non_residue^((modulus^i-1)/4) for i=0,1,2,3

//     my_Fp2 c0, c1;
//     Fp4_model() {};
//     Fp4_model(const my_Fp2& c0, const my_Fp2& c1) : c0(c0), c1(c1) {};

//     void print() const { print!("c0/c1:\n"); c0.print(); c1.print(); }
//     void clear() { c0.clear(); c1.clear(); }
//     void randomize();

//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     std::vector<uint64_t> to_words() const;
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Returns true when the right-most bits of each element represent a value less than the modulus.
//      */
//     bool from_words(std::vector<uint64_t> words);

//     bool is_zero() const { return c0.is_zero() && c1.is_zero(); }
//     bool operator==(const Fp4_model &other) const;
//     bool operator!=(const Fp4_model &other) const;

//     Fp4_model& operator+=(const Fp4_model& other);
//     Fp4_model& operator-=(const Fp4_model& other);
//     Fp4_model& operator*=(const Fp4_model& other);
//     Fp4_model& operator^=(const unsigned long pow);
//     template<mp_size_t m>
//     Fp4_model& operator^=(const bigint<m> &pow);

//     Fp4_model operator+(const Fp4_model &other) const;
//     Fp4_model operator-(const Fp4_model &other) const;
//     Fp4_model operator*(const Fp4_model &other) const;
//     Fp4_model mul_by_023(const Fp4_model &other) const;
//     Fp4_model operator^(const unsigned long pow) const;
//     template<mp_size_t m>
//     Fp4_model operator^(const bigint<m> &exponent) const;
//     template<mp_size_t m, const bigint<m>& modulus_p>
//     Fp4_model operator^(const Fp_model<m, modulus_p> &exponent) const;
//     Fp4_model operator-() const;

//     Fp4_model& square();
//     Fp4_model squared() const;
//     Fp4_model& invert();
//     Fp4_model inverse() const;
//     Fp4_model Frobenius_map(unsigned long power) const;
//     Fp4_model unitary_inverse() const;
//     Fp4_model cyclotomic_squared() const;
//     Fp4_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

//     static my_Fp2 mul_by_non_residue(const my_Fp2 &elt);

//     template<mp_size_t m>
//     Fp4_model cyclotomic_exp(const bigint<m> &exponent) const;

//     static std::size_t ceil_size_in_bits() { return 2 * my_Fp2::ceil_size_in_bits(); }
//     static std::size_t floor_size_in_bits() { return 2 * my_Fp2::floor_size_in_bits(); }

//     static constexpr std::size_t extension_degree() { return 4; }
//     static constexpr bigint<n> field_char() { return modulus; }

//     static Fp4_model<n, modulus> zero();
//     static Fp4_model<n, modulus> one();
//     static Fp4_model<n, modulus> random_element();

//     friend std::ostream& operator<< <n, modulus>(std::ostream &out, const Fp4_model<n, modulus> &el);
//     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp4_model<n, modulus> &el);
// };

// // #ifdef PROFILE_OP_COUNTS
// template<mp_size_t n, const bigint<n>& modulus>
// long long Fp4_model<n, modulus>::add_cnt = 0;

// template<mp_size_t n, const bigint<n>& modulus>
// long long Fp4_model<n, modulus>::sub_cnt = 0;

// template<mp_size_t n, const bigint<n>& modulus>
// long long Fp4_model<n, modulus>::mul_cnt = 0;

// template<mp_size_t n, const bigint<n>& modulus>
// long long Fp4_model<n, modulus>::sqr_cnt = 0;

// template<mp_size_t n, const bigint<n>& modulus>
// long long Fp4_model<n, modulus>::inv_cnt = 0;
// //#endif

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp4_model<n, modulus> &rhs);

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> operator*(const Fp2_model<n, modulus> &lhs, const Fp4_model<n, modulus> &rhs);

// template<mp_size_t n, const bigint<n>& modulus>
// bigint<4*n> Fp4_model<n, modulus>::euler;

// template<mp_size_t n, const bigint<n>& modulus>
// size_t Fp4_model<n, modulus>::s;

// template<mp_size_t n, const bigint<n>& modulus>
// bigint<4*n> Fp4_model<n, modulus>::t;

// template<mp_size_t n, const bigint<n>& modulus>
// bigint<4*n> Fp4_model<n, modulus>::t_minus_1_over_2;

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> Fp4_model<n, modulus>::nqr;

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> Fp4_model<n, modulus>::nqr_to_t;

// template<mp_size_t n, const bigint<n>& modulus>
// Fp_model<n, modulus> Fp4_model<n, modulus>::non_residue;

// template<mp_size_t n, const bigint<n>& modulus>
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

// template<mp_size_t n, const bigint<n>& modulus>
// Fp2_model<n, modulus> Fp4_model<n, modulus>::mul_by_non_residue(const Fp2_model<n, modulus> &elt)
// {
//     return Fp2_model<n, modulus>(non_residue * elt.c1, elt.c0);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> Fp4_model<n, modulus>::zero()
// {
//     return Fp4_model<n,modulus>(my_Fp2::zero(),
//                                 my_Fp2::zero());
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> Fp4_model<n, modulus>::one()
// {
//     return Fp4_model<n,modulus>(my_Fp2::one(),
//                                 my_Fp2::zero());
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::random_element()
// {
//     Fp4_model<n, modulus> r;
//     r.c0 = my_Fp2::random_element();
//     r.c1 = my_Fp2::random_element();

//     return r;
// }

// template<mp_size_t n, const bigint<n>& modulus>
// void Fp4_model<n,modulus>::randomize()
// {
//     (*this) = Fp4_model<n, modulus>::random_element();
// }

// template<mp_size_t n, const bigint<n>& modulus>
// bool Fp4_model<n,modulus>::operator==(const Fp4_model<n,modulus> &other) const
// {
//     return (this->c0 == other.c0 && this->c1 == other.c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// bool Fp4_model<n,modulus>::operator!=(const Fp4_model<n,modulus> &other) const
// {
//     return !(operator==(other));
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator+(const Fp4_model<n,modulus> &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(this->c0 + other.c0,
//                                 this->c1 + other.c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-(const Fp4_model<n,modulus> &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(this->c0 - other.c0,
//                                 this->c1 - other.c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp4_model<n, modulus> &rhs)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n, modulus> operator*(const Fp2_model<n, modulus> &lhs, const Fp4_model<n, modulus> &rhs)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp4_model<n,modulus>(lhs*rhs.c0,
//                                 lhs*rhs.c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator*(const Fp4_model<n,modulus> &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */

//     const my_Fp2 &B = other.c1, &A = other.c0,
//         &b = this->c1, &a = this->c0;
//     const my_Fp2 aA = a*A;
//     const my_Fp2 bB = b*B;

//     const my_Fp2 beta_bB = Fp4_model<n,modulus>::mul_by_non_residue(bB);
//     return Fp4_model<n,modulus>(aA + beta_bB,
//                                 (a+b)*(A+B) - aA  - bB);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::mul_by_023(const Fp4_model<n,modulus> &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
//     assert!(other.c0.c1.is_zero());

//     const my_Fp2 &B = other.c1, &A = other.c0,
//         &b = this->c1, &a = this->c0;
//     const my_Fp2 aA = my_Fp2(a.c0 * A.c0, a.c1 * A.c0);
//     const my_Fp2 bB = b*B;

//     const my_Fp2 beta_bB = Fp4_model<n,modulus>::mul_by_non_residue(bB);
//     return Fp4_model<n,modulus>(aA + beta_bB,
//                                 (a+b)*(A+B) - aA  - bB);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator-() const
// {
//     return Fp4_model<n,modulus>(-this->c0,
//                                 -this->c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::operator^(const unsigned long pow) const
// {
//     return power<Fp4_model<n, modulus> >(*this, pow);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// template<mp_size_t m>
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(const bigint<m> &exponent) const
// {
//     return power<Fp4_model<n, modulus> >(*this, exponent);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// template<mp_size_t m, const bigint<m>& modulus_p>
// Fp4_model<n, modulus> Fp4_model<n,modulus>::operator^(const Fp_model<m, modulus_p> &exponent) const
// {
//     return (*this)^(exponent.as_bigint());
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator+=(const Fp4_model<n,modulus>& other)
// {
//     (*this) = *this + other;
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator-=(const Fp4_model<n,modulus>& other)
// {
//     (*this) = *this - other;
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator*=(const Fp4_model<n,modulus>& other)
// {
//     (*this) = *this * other;
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator^=(const unsigned long pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// template<mp_size_t m>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::operator^=(const bigint<m> &pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::squared() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex) */

//     const my_Fp2 &b = this->c1, &a = this->c0;
//     const my_Fp2 ab = a * b;

//     return Fp4_model<n,modulus>((a+b)*(a+Fp4_model<n,modulus>::mul_by_non_residue(b))-ab-Fp4_model<n,modulus>::mul_by_non_residue(ab),
//                                 ab + ab);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::square()
// {
//     (*this) = squared();
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::inverse() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
// //#endif
//     /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */
//     const my_Fp2 &b = this->c1, &a = this->c0;
//     const my_Fp2 t1 = b.squared();
//     const my_Fp2 t0 = a.squared() - Fp4_model<n,modulus>::mul_by_non_residue(t1);
//     const my_Fp2 new_t1 = t0.inverse();

//     return Fp4_model<n,modulus>(a * new_t1, - (b * new_t1));
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus>& Fp4_model<n,modulus>::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::Frobenius_map(unsigned long power) const
// {
//     return Fp4_model<n,modulus>(c0.Frobenius_map(power),
//                                 Frobenius_coeffs_c1[power % 4] * c1.Frobenius_map(power));
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::unitary_inverse() const
// {
//     return Fp4_model<n,modulus>(this->c0,
//                                 -this->c1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::cyclotomic_squared() const
// {
//     const my_Fp2 A = this->c1.squared();
//     const my_Fp2 B = this->c1 + this->c0;
//     const my_Fp2 C = B.squared() - A;
//     const my_Fp2 D = Fp4_model<n,modulus>::mul_by_non_residue(A); // Fp2(A.c1 * non_residue, A.c0)
//     const my_Fp2 E = C - D;
//     const my_Fp2 F = D + D + my_Fp2::one();
//     const my_Fp2 G = E - my_Fp2::one();

//     return Fp4_model<n,modulus>(F, G);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// template<mp_size_t m>
// Fp4_model<n, modulus> Fp4_model<n,modulus>::cyclotomic_exp(const bigint<m> &exponent) const
// {
//     Fp4_model<n,modulus> res = Fp4_model<n,modulus>::one();
//     Fp4_model<n,modulus> this_inverse = this->unitary_inverse();

//     bool found_nonzero = false;
//     std::vector<long> NAF = find_wnaf(1, exponent);

//     for i in ( 0..=static_cast<long>(NAF.size() - 1)).rev()
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

// template<mp_size_t n, const bigint<n>& modulus>
// Fp4_model<n,modulus> Fp4_model<n,modulus>::sqrt() const
// {
//     return tonelli_shanks_sqrt(*this);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// std::vector<uint64_t> Fp4_model<n,modulus>::to_words() const
// {
//     std::vector<uint64_t> words = c0.to_words();
//     std::vector<uint64_t> words1 = c1.to_words();
//     words.insert(words.end(), words1.begin(), words1.end());
//     return words;
// }

// template<mp_size_t n, const bigint<n>& modulus>
// bool Fp4_model<n,modulus>::from_words(std::vector<uint64_t> words)
// {
//     std::vector<uint64_t>::const_iterator vec_start = words.begin();
//     std::vector<uint64_t>::const_iterator vec_center = words.begin() + words.size() / 2;
//     std::vector<uint64_t>::const_iterator vec_end = words.end();
//     std::vector<uint64_t> words0(vec_start, vec_center);
//     std::vector<uint64_t> words1(vec_center, vec_end);
//     // Fp_model's from_words() takes care of asserts about vector length.
//     return c0.from_words(words0) && c1.from_words(words1);
// }

// template<mp_size_t n, const bigint<n>& modulus>
// std::ostream& operator<<(std::ostream &out, const Fp4_model<n, modulus> &el)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

// template<mp_size_t n, const bigint<n>& modulus>
// std::istream& operator>>(std::istream &in, Fp4_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1;
//     return in;
// }

// // } // namespace libff

// //#endif // FP4_TCC_
