// /** @file
//  *****************************************************************************
//  Declaration of arithmetic in the finite field F[(p^2)^3]
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef FP6_3OVER2_HPP_
// // #define FP6_3OVER2_HPP_
// //#include <vector>

// use crate::algebra::fields::prime_base::fp;
// use crate::algebra::fields::prime_extension::fp2;

// // namespace libff {

// 
// pub struct Fp6_3over2_model;

// 
// std::ostream& operator<<(std::ostream &, const Fp6_3over2_model<n, modulus> &);

// 
// std::istream& operator>>(std::istream &, Fp6_3over2_model<n, modulus> &);

// /**
//  * Arithmetic in the finite field F[(p^2)^3].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  *  Fp6 = Fp2[V]/(V^3-non_residue) where non_residue is in Fp.
//  *
//  * ASSUMPTION: p = 1 (mod 6)
//  */
// 
// pub struct Fp6_3over2_model {

//     type my_Fp=Fp_model<n, modulus>;
//     type my_Fp2=Fp2_model<n, modulus>;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static i64 add_cnt;
//     static i64 sub_cnt;
//     static i64 mul_cnt;
//     static i64 sqr_cnt;
//     static i64 inv_cnt;
// //#endif

//     static bigint<6*n> euler; // (modulus^6-1)/2
//     static std::usize s; // modulus^6 = 2^s * t + 1
//     static bigint<6*n> t; // with t odd
//     static bigint<6*n> t_minus_1_over_2; // (t-1)/2
//     static Fp6_3over2_model<n, modulus> nqr; // a quadratic nonresidue in Fp6
//     static Fp6_3over2_model<n, modulus> nqr_to_t; // nqr^t
//     static my_Fp2 non_residue;
//     static my_Fp2 Frobenius_coeffs_c1[6]; // non_residue^((modulus^i-1)/3)   for i=0,1,2,3,4,5
//     static my_Fp2 Frobenius_coeffs_c2[6]; // non_residue^((2*modulus^i-2)/3) for i=0,1,2,3,4,5

//     my_Fp2 c0, c1, c2;
//     Fp6_3over2_model() {};
//     Fp6_3over2_model(c0:my_Fp2&, c1:my_Fp2&, c2:&my_Fp2)->Selfc0,c1,c2 {};

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
//     bool operator==(other:&Fp6_3over2_model) const;
//     bool operator!=(other:&Fp6_3over2_model) const;

//     Fp6_3over2_model& operator+=(other:&Fp6_3over2_model);
//     Fp6_3over2_model& operator-=(other:&Fp6_3over2_model);
//     Fp6_3over2_model& operator*=(other:&Fp6_3over2_model);
//     Fp6_3over2_model& operator^=(const u64 pow);
//     
//     Fp6_3over2_model& operator^=(pow:&bigint<m>);

//     Fp6_3over2_model operator+(other:&Fp6_3over2_model) const;
//     Fp6_3over2_model operator-(other:&Fp6_3over2_model) const;
//     Fp6_3over2_model operator*(other:&Fp6_3over2_model) const;
//     Fp6_3over2_model operator^(const:u64 pow),
//     
//     Fp6_3over2_model operator^(other:&bigint<m>) const;
//     Fp6_3over2_model operator-() const;

//     Fp6_3over2_model& square();
//     Fp6_3over2_model squared() const;
//     Fp6_3over2_model& invert();
//     Fp6_3over2_model inverse() const;
//     Fp6_3over2_model Frobenius_map(u64 power) const;
//     Fp6_3over2_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

//     static my_Fp2 mul_by_non_residue(elt:&my_Fp2);

//     static std::usize ceil_size_in_bits() { return 3 * my_Fp2::ceil_size_in_bits(); }
//     static std::usize floor_size_in_bits() { return 3 * my_Fp2::floor_size_in_bits(); }

//     static constexpr std::usize extension_degree() { return 6; }
//     static constexpr bigint<n> field_char() { return modulus; }

//     static Fp6_3over2_model<n, modulus> zero();
//     static Fp6_3over2_model<n, modulus> one();
//     static Fp6_3over2_model<n, modulus> random_element();

//     friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp6_3over2_model<n, modulus>);
//     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp6_3over2_model<n, modulus> &el);
// };

// // #ifdef PROFILE_OP_COUNTS
// 
// i64 Fp6_3over2_model<n, modulus>::add_cnt = 0;

// 
// i64 Fp6_3over2_model<n, modulus>::sub_cnt = 0;

// 
// i64 Fp6_3over2_model<n, modulus>::mul_cnt = 0;

// 
// i64 Fp6_3over2_model<n, modulus>::sqr_cnt = 0;

// 
// i64 Fp6_3over2_model<n, modulus>::inv_cnt = 0;
// //#endif

// 
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp6_3over2_model<n, modulus> >);

// 
// std::istream& operator>>(std::istream& in, Vec<Fp6_3over2_model<n, modulus> > &v);

// 
// Fp6_3over2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp6_3over2_model<n, modulus>);

// 
// Fp6_3over2_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp6_3over2_model<n, modulus>);

// 
// bigint<6*n> Fp6_3over2_model<n, modulus>::euler;

// 
// usize Fp6_3over2_model<n, modulus>::s;

// 
// bigint<6*n> Fp6_3over2_model<n, modulus>::t;

// 
// bigint<6*n> Fp6_3over2_model<n, modulus>::t_minus_1_over_2;

// 
// Fp6_3over2_model<n, modulus> Fp6_3over2_model<n, modulus>::nqr;

// 
// Fp6_3over2_model<n, modulus> Fp6_3over2_model<n, modulus>::nqr_to_t;

// 
// Fp2_model<n, modulus> Fp6_3over2_model<n, modulus>::non_residue;

// 
// Fp2_model<n, modulus> Fp6_3over2_model<n, modulus>::Frobenius_coeffs_c1[6];

// 
// Fp2_model<n, modulus> Fp6_3over2_model<n, modulus>::Frobenius_coeffs_c2[6];

// // } // namespace libff
// use crate::algebra::fields::prime_extension::fp6_3over2.tcc;

// //#endif // FP6_3OVER2_HPP_
// /** @file
//  *****************************************************************************
//  Implementation of arithmetic in the finite field F[(p^2)^3].
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef FP6_3OVER2_TCC_
// // #define FP6_3OVER2_TCC_
// use crate::algebra::field_utils::field_utils;

// // namespace libff {

// using std::usize;

// 
// Fp2_model<n, modulus> Fp6_3over2_model<n,modulus>::mul_by_non_residue(elt:&Fp2_model<n, modulus>)
// {
//     return Fp2_model<n, modulus>(non_residue * elt);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::zero()
// {
//     return Fp6_3over2_model<n, modulus>(my_Fp2::zero(), my_Fp2::zero(), my_Fp2::zero());
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::one()
// {
//     return Fp6_3over2_model<n, modulus>(my_Fp2::one(), my_Fp2::zero(), my_Fp2::zero());
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::random_element()
// {
//     Fp6_3over2_model<n, modulus> r;
//     r.c0 = my_Fp2::random_element();
//     r.c1 = my_Fp2::random_element();
//     r.c2 = my_Fp2::random_element();

//     return r;
// }

// 
// pub fn randomize()
// {
//     (*this) = Fp6_3over2_model<n, modulus>::random_element();
// }

// 
// bool Fp6_3over2_model<n,modulus>::operator==(other:&Fp6_3over2_model<n,modulus>) const
// {
//     return (this->c0 == other.c0 && this->c1 == other.c1 && this->c2 == other.c2);
// }

// 
// bool Fp6_3over2_model<n,modulus>::operator!=(other:&Fp6_3over2_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator+(other:&Fp6_3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     return Fp6_3over2_model<n,modulus>(this->c0 + other.c0,
//                                        this->c1 + other.c1,
//                                        this->c2 + other.c2);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator-(other:&Fp6_3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     return Fp6_3over2_model<n,modulus>(this->c0 - other.c0,
//                                        this->c1 - other.c1,
//                                        this->c2 - other.c2);
// }

// 
// Fp6_3over2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp6_3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp6_3over2_model<n,modulus>(lhs*rhs.c0,
//                                        lhs*rhs.c1,
//                                        lhs*rhs.c2);
// }

// 
// Fp6_3over2_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp6_3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp6_3over2_model<n,modulus>(lhs*rhs.c0,
//                                        lhs*rhs.c1,
//                                        lhs*rhs.c2);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator*(other:&Fp6_3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (Karatsuba) */

//     A:&my_Fp2 = other.c0, &B = other.c1, &C = other.c2,
//                  &a = this->c0, &b = this->c1, &c = this->c2;
//     let aA= a*A;
//     let bB= b*B;
//     let cC= c*C;

//     return Fp6_3over2_model<n,modulus>(aA + Fp6_3over2_model<n,modulus>::mul_by_non_residue((b+c)*(B+C)-bB-cC),
//                                        (a+b)*(A+B)-aA-bB+Fp6_3over2_model<n,modulus>::mul_by_non_residue(cC),
//                                        (a+c)*(A+C)-aA+bB-cC);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator-() const
// {
//     return Fp6_3over2_model<n,modulus>(-this->c0,
//                                        -this->c1,
//                                        -this->c2);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp6_3over2_model<n, modulus> >(*this, pow);
// }

// 
// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::operator^(pow:&bigint<m>) const
// {
//     return power<Fp6_3over2_model<n, modulus>, m>(*this, pow);
// }

// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator+=(const Fp6_3over2_model<n,modulus>& other)
// {
//     (*this) = *this + other;
//     return (*this);
// }

// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator-=(const Fp6_3over2_model<n,modulus>& other)
// {
//     (*this) = *this - other;
//     return (*this);
// }

// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator*=(const Fp6_3over2_model<n,modulus>& other)
// {
//     (*this) = *this * other;
//     return (*this);
// }

// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// 
// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::squared() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (CH-SQR2) */

//     a:&my_Fp2 = this->c0, &b = this->c1, &c = this->c2;
//     let s0= a.squared();
//     let ab= a*b;
//     let s1= ab + ab;
//     let s2= (a - b + c).squared();
//     let bc= b*c;
//     let s3= bc + bc;
//     let s4= c.squared();

//     return Fp6_3over2_model<n,modulus>(s0 + Fp6_3over2_model<n,modulus>::mul_by_non_residue(s3),
//                                        s1 + Fp6_3over2_model<n,modulus>::mul_by_non_residue(s4),
//                                        s1 + s2 + s3 - s0 - s4);
// }

// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::square()
// {
//     (*this) = squared();
//     return (*this);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::inverse() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
// //#endif
//     /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17 */

//     a:&my_Fp2 = this->c0, &b = this->c1, &c = this->c2;
//     let t0= a.squared();
//     let t1= b.squared();
//     let t2= c.squared();
//     let t3= a*b;
//     let t4= a*c;
//     let t5= b*c;
//     let c0= t0 - Fp6_3over2_model<n,modulus>::mul_by_non_residue(t5);
//     let c1= Fp6_3over2_model<n,modulus>::mul_by_non_residue(t2) - t3;
//     Scott:my_Fp2 c2 = t1 - t4; // typo in paper referenced above. should be "-" as per, but is "*"
//     let t6= (a * c0 + Fp6_3over2_model<n,modulus>::mul_by_non_residue((c * c1 + b * c2))).inverse();
//     return Fp6_3over2_model<n,modulus>(t6 * c0, t6 * c1, t6 * c2);
// }

// 
// Fp6_3over2_model<n,modulus>& Fp6_3over2_model<n,modulus>::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::Frobenius_map(u64 power) const
// {
//     return Fp6_3over2_model<n,modulus>(c0.Frobenius_map(power),
//                                        Frobenius_coeffs_c1[power % 6] * c1.Frobenius_map(power),
//                                        Frobenius_coeffs_c2[power % 6] * c2.Frobenius_map(power));
// }

// 
// Fp6_3over2_model<n,modulus> Fp6_3over2_model<n,modulus>::sqrt() const
// {
//     return tonelli_shanks_sqrt(*this);
// }

// 
// Vec<uint64_t> Fp6_3over2_model<n,modulus>::to_words() const
// {
//     Vec<uint64_t> words = c0.to_words();
//     Vec<uint64_t> words1 = c1.to_words();
//     Vec<uint64_t> words2 = c2.to_words();
//     words.insert(words.end(), words1.begin(), words1.end());
//     words.insert(words.end(), words2.begin(), words2.end());
//     return words;
// }

// 
// bool Fp6_3over2_model<n,modulus>::from_words(Vec<uint64_t> words)
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
// std::ostream& operator<<(std::ostream &out, el:&Fp6_3over2_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1 << OUTPUT_SEPARATOR << el.c2;
//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, Fp6_3over2_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1 >> el.c2;
//     return in;
// }

// 
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp6_3over2_model<n, modulus> >)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// 
// std::istream& operator>>(std::istream& in, Vec<Fp6_3over2_model<n, modulus> > &v)
// {
//     v.clear();

//     usize s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     v.reserve(s);

//     for i in 0..s
//     {
//         Fp6_3over2_model<n, modulus> el;
//         in >> el;
//         v.emplace_back(el);
//     }

//     return in;
// }

// // } // namespace libff
// //#endif // FP6_3_OVER_2_TCC_
