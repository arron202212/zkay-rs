// /** @file
//  *****************************************************************************
//  Declaration of arithmetic in the finite field F[((p^2)^3)^2].
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef FP12_2OVER3OVER2_HPP_
// // #define FP12_2OVER3OVER2_HPP_
// //#include <vector>

// use crate::algebra::fields::prime_base::fp;
// use crate::algebra::fields::prime_extension::fp2;
// use crate::algebra::fields::prime_extension::fp6_3over2;

// // namespace libff {

// 
// pub struct Fp12_2over3over2_model;

// 
// std::ostream& operator<<(std::ostream &, const Fp12_2over3over2_model<n, modulus> &);

// 
// std::istream& operator>>(std::istream &, Fp12_2over3over2_model<n, modulus> &);

// /**
//  * Arithmetic in the finite field F[((p^2)^3)^2].
//  *
//  * Let p := modulus. This interface provides arithmetic for the extension field
//  * Fp12 = Fp6[W]/(W^2-V) where Fp6 = Fp2[V]/(V^3-non_residue) and non_residue is in Fp2
//  *
//  * ASSUMPTION: p = 1 (mod 6)
//  */
// 
// pub struct Fp12_2over3over2_model {

//     type my_Fp=Fp_model<n, modulus>;
//     type my_Fp2=Fp2_model<n, modulus>;
//     type my_Fp6=Fp6_3over2_model<n, modulus>;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static i64 add_cnt;
//     static i64 sub_cnt;
//     static i64 mul_cnt;
//     static i64 sqr_cnt;
//     static i64 inv_cnt;
// //#endif

//     static bigint<12*n> euler; // (modulus^12-1)/2
//     static std::usize s; // modulus^12 = 2^s * t + 1
//     static bigint<12*n> t; // with t odd
//     static bigint<12*n> t_minus_1_over_2; // (t-1)/2
//     static Fp12_2over3over2_model<n, modulus> nqr; // a quadratic nonresidue in Fp12
//     static Fp12_2over3over2_model<n, modulus> nqr_to_t; // nqr^t
//     static Fp2_model<n, modulus> non_residue;
//     static Fp2_model<n, modulus> Frobenius_coeffs_c1[12]; // non_residue^((modulus^i-1)/6) for i=0,...,11

//     my_Fp6 c0, c1;
//     Fp12_2over3over2_model() {};
//     Fp12_2over3over2_model(c0:my_Fp6&, c1:&my_Fp6)->Selfc0,c1 {};

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
//     bool operator==(other:&Fp12_2over3over2_model) const;
//     bool operator!=(other:&Fp12_2over3over2_model) const;

//     Fp12_2over3over2_model& operator+=(other:&Fp12_2over3over2_model);
//     Fp12_2over3over2_model& operator-=(other:&Fp12_2over3over2_model);
//     Fp12_2over3over2_model& operator*=(other:&Fp12_2over3over2_model);
//     Fp12_2over3over2_model& operator^=(const u64 pow);
//     
//     Fp12_2over3over2_model& operator^=(pow:&bigint<m>);

//     Fp12_2over3over2_model operator+(other:&Fp12_2over3over2_model) const;
//     Fp12_2over3over2_model operator-(other:&Fp12_2over3over2_model) const;
//     Fp12_2over3over2_model operator*(other:&Fp12_2over3over2_model) const;
//     Fp12_2over3over2_model operator^(const:u64 pow),
//     
//     Fp12_2over3over2_model operator^(exponent:&bigint<m>) const;
//     
//     Fp12_2over3over2_model operator^(exponent:&Fp_model<m, exp_modulus>) const;
//     Fp12_2over3over2_model operator-() const;

//     Fp12_2over3over2_model& square();
//     Fp12_2over3over2_model squared() const; // default is squared_complex
//     Fp12_2over3over2_model squared_karatsuba() const;
//     Fp12_2over3over2_model squared_complex() const;
//     Fp12_2over3over2_model& invert();
//     Fp12_2over3over2_model inverse() const;
//     Fp12_2over3over2_model Frobenius_map(u64 power) const;
//     Fp12_2over3over2_model unitary_inverse() const;
//     Fp12_2over3over2_model cyclotomic_squared() const;
//     Fp12_2over3over2_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

//     Fp12_2over3over2_model mul_by_024(ell_0:&my_Fp2, ell_VW:&my_Fp2, ell_VV:&my_Fp2) const;
//     Fp12_2over3over2_model mul_by_045(ell_0:&my_Fp2, ell_VW:&my_Fp2, ell_VV:&my_Fp2) const;

//     static my_Fp6 mul_by_non_residue(elt:&my_Fp6);

//     
//     Fp12_2over3over2_model cyclotomic_exp(exponent:&bigint<m>) const;

//     static std::usize ceil_size_in_bits() { return 2 * my_Fp6::ceil_size_in_bits(); }
//     static std::usize floor_size_in_bits() { return 2 * my_Fp6::floor_size_in_bits(); }

//     static constexpr std::usize extension_degree() { return 12; }
//     static constexpr bigint<n> field_char() { return modulus; }

//     static Fp12_2over3over2_model<n, modulus> zero();
//     static Fp12_2over3over2_model<n, modulus> one();
//     static Fp12_2over3over2_model<n, modulus> random_element();

//     friend std::ostream& operator<< <n, modulus>(std::ostream &out, el:&Fp12_2over3over2_model<n, modulus>);
//     friend std::istream& operator>> <n, modulus>(std::istream &in, Fp12_2over3over2_model<n, modulus> &el);
// };

// // #ifdef PROFILE_OP_COUNTS
// 
// i64 Fp12_2over3over2_model<n, modulus>::add_cnt = 0;

// 
// i64 Fp12_2over3over2_model<n, modulus>::sub_cnt = 0;

// 
// i64 Fp12_2over3over2_model<n, modulus>::mul_cnt = 0;

// 
// i64 Fp12_2over3over2_model<n, modulus>::sqr_cnt = 0;

// 
// i64 Fp12_2over3over2_model<n, modulus>::inv_cnt = 0;
// //#endif

// 
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp12_2over3over2_model<n, modulus> >);

// 
// std::istream& operator>>(std::istream& in, Vec<Fp12_2over3over2_model<n, modulus> > &v);

// 
// Fp12_2over3over2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp12_2over3over2_model<n, modulus>);

// 
// Fp12_2over3over2_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp12_2over3over2_model<n, modulus>);

// 
// Fp12_2over3over2_model<n, modulus> operator*(lhs:&Fp6_3over2_model<n, modulus>, rhs:&Fp12_2over3over2_model<n, modulus>);

// 
// bigint<12*n> Fp12_2over3over2_model<n, modulus>::euler;

// 
// usize Fp12_2over3over2_model<n, modulus>::s;

// 
// bigint<12*n> Fp12_2over3over2_model<n, modulus>::t;

// 
// bigint<12*n> Fp12_2over3over2_model<n, modulus>::t_minus_1_over_2;

// 
// Fp12_2over3over2_model<n, modulus> Fp12_2over3over2_model<n, modulus>::nqr;

// 
// Fp12_2over3over2_model<n, modulus> Fp12_2over3over2_model<n, modulus>::nqr_to_t;

// 
// Fp2_model<n, modulus> Fp12_2over3over2_model<n, modulus>::non_residue;

// 
// Fp2_model<n, modulus> Fp12_2over3over2_model<n, modulus>::Frobenius_coeffs_c1[12];

// // } // namespace libff
// use crate::algebra::fields::prime_extension::fp12_2over3over2.tcc;
// //#endif // FP12_2OVER3OVER2_HPP_
// /** @file
//  *****************************************************************************
//  Implementation of arithmetic in the finite field F[((p^2)^3)^2].
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef FP12_2OVER3OVER2_TCC_
// // #define FP12_2OVER3OVER2_TCC_

// // namespace libff {

// using std::usize;

// 
// Fp6_3over2_model<n, modulus> Fp12_2over3over2_model<n,modulus>::mul_by_non_residue(elt:&Fp6_3over2_model<n, modulus>)
// {
//     return Fp6_3over2_model<n, modulus>(non_residue * elt.c2, elt.c0, elt.c1);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::zero()
// {
//     return Fp12_2over3over2_model<n, modulus>(my_Fp6::zero(), my_Fp6::zero());
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::one()
// {
//     return Fp12_2over3over2_model<n, modulus>(my_Fp6::one(), my_Fp6::zero());
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::random_element()
// {
//     Fp12_2over3over2_model<n, modulus> r;
//     r.c0 = my_Fp6::random_element();
//     r.c1 = my_Fp6::random_element();

//     return r;
// }

// 
// pub fn randomize()
// {
//     (*this) = Fp12_2over3over2_model<n, modulus>::random_element();
// }

// 
// bool Fp12_2over3over2_model<n,modulus>::operator==(other:&Fp12_2over3over2_model<n,modulus>) const
// {
//     return (this->c0 == other.c0 && this->c1 == other.c1);
// }

// 
// bool Fp12_2over3over2_model<n,modulus>::operator!=(other:&Fp12_2over3over2_model<n,modulus>) const
// {
//     return !(operator==(other));
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::operator+(other:&Fp12_2over3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     return Fp12_2over3over2_model<n,modulus>(this->c0 + other.c0,
//                                              this->c1 + other.c1);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::operator-(other:&Fp12_2over3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     return Fp12_2over3over2_model<n,modulus>(this->c0 - other.c0,
//                                              this->c1 - other.c1);
// }

// 
// Fp12_2over3over2_model<n, modulus> operator*(lhs:&Fp_model<n, modulus>, rhs:&Fp12_2over3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp12_2over3over2_model<n,modulus>(lhs*rhs.c0,
//                                              lhs*rhs.c1);
// }

// 
// Fp12_2over3over2_model<n, modulus> operator*(lhs:&Fp2_model<n, modulus>, rhs:&Fp12_2over3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp12_2over3over2_model<n,modulus>(lhs*rhs.c0,
//                                              lhs*rhs.c1);
// }

// 
// Fp12_2over3over2_model<n, modulus> operator*(lhs:&Fp6_3over2_model<n, modulus>, rhs:&Fp12_2over3over2_model<n, modulus>)
// {
// // #ifdef PROFILE_OP_COUNTS
//     rhs.mul_cnt++;
// //#endif
//     return Fp12_2over3over2_model<n,modulus>(lhs*rhs.c0,
//                                              lhs*rhs.c1);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::operator*(other:&Fp12_2over3over2_model<n,modulus>) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */

//     A:&my_Fp6 = other.c0, &B = other.c1,
//         &a = this->c0, &b = this->c1;
//     let aA= a * A;
//     let bB= b * B;

//     return Fp12_2over3over2_model<n,modulus>(aA + Fp12_2over3over2_model<n, modulus>::mul_by_non_residue(bB),
//                                              (a + b)*(A+B) - aA - bB);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::operator-() const
// {
//     return Fp12_2over3over2_model<n,modulus>(-this->c0,
//                                              -this->c1);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::operator^(const u64 pow) const
// {
//     return power<Fp12_2over3over2_model<n, modulus> >(*this, pow);
// }

// 
// 
// Fp12_2over3over2_model<n, modulus> Fp12_2over3over2_model<n,modulus>::operator^(exponent:&bigint<m>) const
// {
//     return power<Fp12_2over3over2_model<n, modulus> >(*this, exponent);
// }

// 
// 
// Fp12_2over3over2_model<n, modulus> Fp12_2over3over2_model<n,modulus>::operator^(exponent:&Fp_model<m, exp_modulus>) const
// {
//     return (*this)^(exponent.as_bigint());
// }

// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::operator+=(const Fp12_2over3over2_model<n,modulus>& other)
// {
//     (*this) = *this + other;
//     return (*this);
// }

// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::operator-=(const Fp12_2over3over2_model<n,modulus>& other)
// {
//     (*this) = *this - other;
//     return (*this);
// }

// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::operator*=(const Fp12_2over3over2_model<n,modulus>& other)
// {
//     (*this) = *this * other;
//     return (*this);
// }

// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// 
// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::squared() const
// {
//     return squared_complex();
// }

// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::square()
// {
//     (*this) = squared();
//     return (*this);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::squared_karatsuba() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba squaring) */

//     a:&my_Fp6 = this->c0, &b = this->c1;
//     let asq= a.squared();
//     let bsq= b.squared();

//     return Fp12_2over3over2_model<n,modulus>(asq + Fp12_2over3over2_model<n, modulus>::mul_by_non_residue(bsq),
//                                              (a + b).squared() - asq - bsq);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::squared_complex() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
// //#endif
//     /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex squaring) */

//     a:&my_Fp6 = this->c0, &b = this->c1;
//     let ab= a * b;

//     return Fp12_2over3over2_model<n,modulus>((a + b) * (a + Fp12_2over3over2_model<n, modulus>::mul_by_non_residue(b)) - ab - Fp12_2over3over2_model<n, modulus>::mul_by_non_residue(ab),
//                                              ab + ab);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::inverse() const
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
// //#endif
//     /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */

//     a:&my_Fp6 = this->c0, &b = this->c1;
//     let t0= a.squared();
//     let t1= b.squared();
//     let t2= t0 - Fp12_2over3over2_model<n, modulus>::mul_by_non_residue(t1);
//     let t3= t2.inverse();
//     let c0= a * t3;
//     let c1= - (b * t3);

//     return Fp12_2over3over2_model<n,modulus>(c0, c1);
// }

// 
// Fp12_2over3over2_model<n,modulus>& Fp12_2over3over2_model<n,modulus>::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::Frobenius_map(u64 power) const
// {
//     return Fp12_2over3over2_model<n,modulus>(c0.Frobenius_map(power),
//                                              Frobenius_coeffs_c1[power % 12] * c1.Frobenius_map(power));
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::unitary_inverse() const
// {
//     return Fp12_2over3over2_model<n,modulus>(this->c0,
//                                              -this->c1);
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::cyclotomic_squared() const
// {
//     /* OLD: naive implementation
//        return (*this).squared();
//     */
//     my_Fp2 z0 = this->c0.c0;
//     my_Fp2 z4 = this->c0.c1;
//     my_Fp2 z3 = this->c0.c2;
//     my_Fp2 z2 = this->c1.c0;
//     my_Fp2 z1 = this->c1.c1;
//     my_Fp2 z5 = this->c1.c2;

//     my_Fp2 t0, t1, t2, t3, t4, t5, tmp;

//     // t0 + t1*y = (z0 + z1*y)^2 = a^2
//     tmp = z0 * z1;
//     t0 = (z0 + z1) * (z0 + my_Fp6::non_residue * z1) - tmp - my_Fp6::non_residue * tmp;
//     t1 = tmp + tmp;
//     // t2 + t3*y = (z2 + z3*y)^2 = b^2
//     tmp = z2 * z3;
//     t2 = (z2 + z3) * (z2 + my_Fp6::non_residue * z3) - tmp - my_Fp6::non_residue * tmp;
//     t3 = tmp + tmp;
//     // t4 + t5*y = (z4 + z5*y)^2 = c^2
//     tmp = z4 * z5;
//     t4 = (z4 + z5) * (z4 + my_Fp6::non_residue * z5) - tmp - my_Fp6::non_residue * tmp;
//     t5 = tmp + tmp;

//     // for A

//     // z0 = 3 * t0 - 2 * z0
//     z0 = t0 - z0;
//     z0 = z0 + z0;
//     z0 = z0 + t0;
//     // z1 = 3 * t1 + 2 * z1
//     z1 = t1 + z1;
//     z1 = z1 + z1;
//     z1 = z1 + t1;

//     // for B

//     // z2 = 3 * (xi * t5) + 2 * z2
//     tmp = my_Fp6::non_residue * t5;
//     z2 = tmp + z2;
//     z2 = z2 + z2;
//     z2 = z2 + tmp;

//     // z3 = 3 * t4 - 2 * z3
//     z3 = t4 - z3;
//     z3 = z3 + z3;
//     z3 = z3 + t4;

//     // for C

//     // z4 = 3 * t2 - 2 * z4
//     z4 = t2 - z4;
//     z4 = z4 + z4;
//     z4 = z4 + t2;

//     // z5 = 3 * t3 + 2 * z5
//     z5 = t3 + z5;
//     z5 = z5 + z5;
//     z5 = z5 + t3;

//     return Fp12_2over3over2_model<n,modulus>(my_Fp6(z0,z4,z3),my_Fp6(z2,z1,z5));
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::mul_by_045(ell_0:&Fp2_model<n, modulus>,
//                                                                                 ell_VW:&Fp2_model<n, modulus>,
//                                                                                 ell_VV:&Fp2_model<n, modulus>) const
// {
//     /*
//     // OLD
//     Fp12_2over3over2_model<n,modulus> a(my_Fp6(ell_VW, my_Fp2::zero(), my_Fp2::zero()),
//                                         my_Fp6(my_Fp2::zero(), ell_0, ell_VV));

//     return (*this) * a;
//     */

//     my_Fp2 z0 = this->c0.c0;
//     my_Fp2 z1 = this->c0.c1;
//     my_Fp2 z2 = this->c0.c2;
//     my_Fp2 z3 = this->c1.c0;
//     my_Fp2 z4 = this->c1.c1;
//     my_Fp2 z5 = this->c1.c2;

//     my_Fp2 x0 = ell_VW;
//     my_Fp2 x4 = ell_0;
//     my_Fp2 x5 = ell_VV;

//     my_Fp2 t0, t1, t2, t3, t4, t5;
//     my_Fp2 tmp1, tmp2;

//     tmp1 = my_Fp6::non_residue * x4;
//     tmp2 = my_Fp6::non_residue * x5;

//     t0 = x0 * z0 + tmp1 * z4 + tmp2 * z3;
//     t1 = x0 * z1 + tmp1 * z5 + tmp2 * z4;
//     t2 = x0 * z2 + x4 * z3 + tmp2 * z5;
//     t3 = x0 * z3 + tmp1 * z2 + tmp2 * z1;
//     t4 = x0 * z4 + x4 * z0 + tmp2 * z2;
//     t5 = x0 * z5 + x4 * z1 + x5 * z0;

//     return Fp12_2over3over2_model<n,modulus>(my_Fp6(t0,t1,t2),my_Fp6(t3,t4,t5));
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::mul_by_024(ell_0:&Fp2_model<n, modulus>,
//                                                                                 ell_VW:&Fp2_model<n, modulus>,
//                                                                                 ell_VV:&Fp2_model<n, modulus>) const
// {
//     /* OLD: naive implementation
//        Fp12_2over3over2_model<n,modulus> a(my_Fp6(ell_0, my_Fp2::zero(), ell_VV),
//        my_Fp6(my_Fp2::zero(), ell_VW, my_Fp2::zero()));

//        return (*this) * a;
//     */
//     my_Fp2 z0 = this->c0.c0;
//     my_Fp2 z1 = this->c0.c1;
//     my_Fp2 z2 = this->c0.c2;
//     my_Fp2 z3 = this->c1.c0;
//     my_Fp2 z4 = this->c1.c1;
//     my_Fp2 z5 = this->c1.c2;

//     my_Fp2 x0 = ell_0;
//     my_Fp2 x2 = ell_VV;
//     my_Fp2 x4 = ell_VW;

//     my_Fp2 t0, t1, t2, s0, T3, T4, D0, D2, D4, S1;

//     D0 = z0 * x0;
//     D2 = z2 * x2;
//     D4 = z4 * x4;
//     t2 = z0 + z4;
//     t1 = z0 + z2;
//     s0 = z1 + z3 + z5;

//     // For z.a_.a_ = z0.
//     S1 = z1 * x2;
//     T3 = S1 + D4;
//     T4 = my_Fp6::non_residue * T3 + D0;
//     z0 = T4;

//     // For z.a_.b_ = z1
//     T3 = z5 * x4;
//     S1 = S1 + T3;
//     T3 = T3 + D2;
//     T4 = my_Fp6::non_residue * T3;
//     T3 = z1 * x0;
//     S1 = S1 + T3;
//     T4 = T4 + T3;
//     z1 = T4;

//     // For z.a_.c_ = z2
//     t0 = x0 + x2;
//     T3 = t1 * t0 - D0 - D2;
//     T4 = z3 * x4;
//     S1 = S1 + T4;
//     T3 = T3 + T4;

//     // For z.b_.a_ = z3 (z3 needs z2)
//     t0 = z2 + z4;
//     z2 = T3;
//     t1 = x2 + x4;
//     T3 = t0 * t1 - D2 - D4;
//     T4 = my_Fp6::non_residue * T3;
//     T3 = z3 * x0;
//     S1 = S1 + T3;
//     T4 = T4 + T3;
//     z3 = T4;

//     // For z.b_.b_ = z4
//     T3 = z5 * x2;
//     S1 = S1 + T3;
//     T4 = my_Fp6::non_residue * T3;
//     t0 = x0 + x4;
//     T3 = t2 * t0 - D0 - D4;
//     T4 = T4 + T3;
//     z4 = T4;

//     // For z.b_.c_ = z5.
//     t0 = x0 + x2 + x4;
//     T3 = s0 * t0 - S1;
//     z5 = T3;

//     return Fp12_2over3over2_model<n,modulus>(my_Fp6(z0,z1,z2),my_Fp6(z3,z4,z5));

// }

// 
// 
// Fp12_2over3over2_model<n, modulus> Fp12_2over3over2_model<n,modulus>::cyclotomic_exp(exponent:&bigint<m>) const
// {
//     Fp12_2over3over2_model<n,modulus> res = Fp12_2over3over2_model<n,modulus>::one();

//     bool found_one = false;
//     for i in ( 0..=m-1).rev()
//     {
//         for j in ( 0..=GMP_NUMB_BITS - 1).rev()
//         {
//             if found_one
//             {
//                 res = res.cyclotomic_squared();
//             }

//             static let one= 1;
//             if exponent.0.0[i] & (one<<j)
//             {
//                 found_one = true;
//                 res = res * (*this);
//             }
//         }
//     }

//     return res;
// }

// 
// Fp12_2over3over2_model<n,modulus> Fp12_2over3over2_model<n,modulus>::sqrt() const
// {
//     return tonelli_shanks_sqrt(*this);
// }

// 
// Vec<uint64_t> Fp12_2over3over2_model<n,modulus>::to_words() const
// {
//     Vec<uint64_t> words = c0.to_words();
//     Vec<uint64_t> words1 = c1.to_words();
//     words.insert(words.end(), words1.begin(), words1.end());
//     return words;
// }

// 
// bool Fp12_2over3over2_model<n,modulus>::from_words(Vec<uint64_t> words)
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
// std::ostream& operator<<(std::ostream &out, el:&Fp12_2over3over2_model<n, modulus>)
// {
//     out << el.c0 << OUTPUT_SEPARATOR << el.c1;
//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, Fp12_2over3over2_model<n, modulus> &el)
// {
//     in >> el.c0 >> el.c1;
//     return in;
// }

// 
// std::ostream& operator<<(std::ostream& out, v:&Vec<Fp12_2over3over2_model<n, modulus> >)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// 
// std::istream& operator>>(std::istream& in, Vec<Fp12_2over3over2_model<n, modulus> > &v)
// {
//     v.clear();

//     usize s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     v.reserve(s);

//     for i in 0..s
//     {
//         Fp12_2over3over2_model<n, modulus> el;
//         in >> el;
//         v.emplace_back(el);
//     }

//     return in;
// }

// // } // namespace libff
// //#endif // FP12_2OVER3OVER2_TCC_
 use educe::Educe;
use super::quadratic_extension::{QuadExtConfig, QuadExtField};
use crate::algebra::fields::{
    prime_extension::{
        fp6_3over2::{Fp6, Fp6Config},
        fp2::{Fp2, Fp2Config as Fp2ConfigTrait},
    },
    field::{Field,AdditiveGroup}, cyclotomic::CyclotomicMultSubgroup,
};
//  use crate::algebra::{fields::PrimeField, cyclotomic::CyclotomicMultSubgroup};
use ark_std::Zero;
use core::{marker::PhantomData, ops::Not};

type Fp2Config<P> = <<P as Fp12Config>::Fp6Config as Fp6Config>::Fp2Config;

pub trait Fp12Config: 'static + Send + Sync + Copy {
    type Fp6Config: Fp6Config;

    /// This *must* equal (0, 1, 0);
    /// see [[DESD06, Section 6.1]](https://eprint.iacr.org/2006/471.pdf).
    const NONRESIDUE: Fp6<Self::Fp6Config>;

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP12_C1: &'static [Fp2<Fp2Config<Self>>];

    /// Multiply by quadratic nonresidue v.
    #[inline(always)]
    fn mul_fp6_by_nonresidue_in_place(fe: &mut Fp6<Self::Fp6Config>) -> &mut Fp6<Self::Fp6Config> {
        // see [[DESD06, Section 6.1]](https://eprint.iacr.org/2006/471.pdf).
        let old_c1 = fe.c1;
        fe.c1 = fe.c0;
        fe.c0 = fe.c2;
        Self::Fp6Config::mul_fp2_by_nonresidue_in_place(&mut fe.c0);
        fe.c2 = old_c1;
        fe
    }
}

pub struct Fp12ConfigWrapper<P: Fp12Config>(PhantomData<P>);

impl<P: Fp12Config> QuadExtConfig for Fp12ConfigWrapper<P> {
    type BasePrimeField = <Fp2Config<P> as Fp2ConfigTrait>::Fp;
    type BaseField = Fp6<P::Fp6Config>;
    type FrobCoeff = Fp2<Fp2Config<P>>;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 12;

    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const FROBENIUS_COEFF_C1: &'static [Self::FrobCoeff] = P::FROBENIUS_COEFF_FP12_C1;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp6_by_nonresidue_in_place(fe)
    }

    fn mul_base_field_by_frob_coeff(fe: &mut Self::BaseField, power: usize) {
        fe.mul_assign_by_fp2(Self::FROBENIUS_COEFF_C1[power % Self::DEGREE_OVER_BASE_PRIME_FIELD]);
    }
}

pub type Fp12<P> = QuadExtField<Fp12ConfigWrapper<P>>;

impl<P: Fp12Config> Fp12<P> {
    pub fn mul_by_fp(&mut self, element: &<Self as Field>::BasePrimeField) {
        self.c0.mul_by_fp(element);
        self.c1.mul_by_fp(element);
    }

    pub fn mul_by_034(
        &mut self,
        c0: &Fp2<Fp2Config<P>>,
        c3: &Fp2<Fp2Config<P>>,
        c4: &Fp2<Fp2Config<P>>,
    ) {
        let a0 = self.c0.c0 * c0;
        let a1 = self.c0.c1 * c0;
        let a2 = self.c0.c2 * c0;
        let a = Fp6::new(a0, a1, a2);
        let mut b = self.c1;
        b.mul_by_01(c3, c4);

        let c0 = *c0 + c3;
        let c1 = c4;
        let mut e = self.c0 + &self.c1;
        e.mul_by_01(&c0, c1);
        self.c1 = e - &(a + &b);
        self.c0 = b;
        P::mul_fp6_by_nonresidue_in_place(&mut self.c0);
        self.c0 += &a;
    }

    pub fn mul_by_014(
        &mut self,
        c0: &Fp2<Fp2Config<P>>,
        c1: &Fp2<Fp2Config<P>>,
        c4: &Fp2<Fp2Config<P>>,
    ) {
        let mut aa = self.c0;
        aa.mul_by_01(c0, c1);
        let mut bb = self.c1;
        bb.mul_by_1(c4);
        let mut o = *c1;
        o += c4;
        self.c1 += &self.c0;
        self.c1.mul_by_01(c0, &o);
        self.c1 -= &aa;
        self.c1 -= &bb;
        self.c0 = bb;
        P::mul_fp6_by_nonresidue_in_place(&mut self.c0);
        self.c0 += &aa;
    }
}

pub const fn characteristic_square_mod_6_is_one(characteristic: &[u64]) -> bool {
    // char mod 6 = (a_0 + 2**64 * a_1 + ...) mod 6
    //            = a_0 mod 6 + (2**64 * a_1 mod 6) + (...) mod 6
    //            = a_0 mod 6 + (4 * a_1 mod 6) + (4 * ...) mod 6
    let mut char_mod_6 = 0u64;
    crate::const_for!((i in 0..(characteristic.len())) {
        char_mod_6 += if i == 0 {
            characteristic[i] % 6
        } else {
            (4 * (characteristic[i] % 6)) % 6
        };
    });
    (char_mod_6 * char_mod_6) % 6 == 1
}

impl<P: Fp12Config> CyclotomicMultSubgroup for Fp12<P> {
    const INVERSE_IS_FAST: bool = true;

    fn cyclotomic_inverse_in_place(&mut self) -> Option<&mut Self> {
        self.is_zero().not().then(|| self.conjugate_in_place())
    }

    fn cyclotomic_square_in_place(&mut self) -> &mut Self {
        // Faster Squaring in the Cyclotomic Subgroup of Sixth Degree Extensions
        // - Robert Granger and Michael Scott
        //
        if characteristic_square_mod_6_is_one(Self::characteristic()) {
            let fp2_nr = <P::Fp6Config as Fp6Config>::mul_fp2_by_nonresidue;

            let r0 = &self.c0.c0;
            let r4 = &self.c0.c1;
            let r3 = &self.c0.c2;
            let r2 = &self.c1.c0;
            let r1 = &self.c1.c1;
            let r5 = &self.c1.c2;

            // t0 + t1*y = (z0 + z1*y)^2 = a^2
            let mut tmp = *r0 * r1;
            let t0 = (*r0 + r1) * &(fp2_nr(*r1) + r0) - &tmp - &fp2_nr(tmp);
            let t1 = tmp.double();

            // t2 + t3*y = (z2 + z3*y)^2 = b^2
            tmp = *r2 * r3;
            let t2 = (*r2 + r3) * &(fp2_nr(*r3) + r2) - &tmp - &fp2_nr(tmp);
            let t3 = tmp.double();

            // t4 + t5*y = (z4 + z5*y)^2 = c^2
            tmp = *r4 * r5;
            let t4 = (*r4 + r5) * &(fp2_nr(*r5) + r4) - &tmp - &fp2_nr(tmp);
            let t5 = tmp.double();

            let z0 = &mut self.c0.c0;
            let z4 = &mut self.c0.c1;
            let z3 = &mut self.c0.c2;
            let z2 = &mut self.c1.c0;
            let z1 = &mut self.c1.c1;
            let z5 = &mut self.c1.c2;

            // for A

            // z0 = 3 * t0 - 2 * z0
            *z0 = t0 - &*z0;
            z0.double_in_place();
            *z0 += &t0;

            // z1 = 3 * t1 + 2 * z1
            *z1 = t1 + &*z1;
            z1.double_in_place();
            *z1 += &t1;

            // for B

            // z2 = 3 * (xi * t5) + 2 * z2
            tmp = fp2_nr(t5);
            *z2 += tmp;
            z2.double_in_place();
            *z2 += &tmp;

            // z3 = 3 * t4 - 2 * z3
            *z3 = t4 - &*z3;
            z3.double_in_place();
            *z3 += &t4;

            // for C

            // z4 = 3 * t2 - 2 * z4
            *z4 = t2 - &*z4;
            z4.double_in_place();
            *z4 += &t2;

            // z5 = 3 * t3 + 2 * z5
            *z5 += t3;
            z5.double_in_place();
            *z5 += &t3;
            self
        } else {
            self.square_in_place()
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_characteristic_square_mod_6_is_one() {
        use super::*;
        assert!(!characteristic_square_mod_6_is_one(&[36]));
        assert!(characteristic_square_mod_6_is_one(&[37]));
        assert!(!characteristic_square_mod_6_is_one(&[38]));
        assert!(!characteristic_square_mod_6_is_one(&[39]));
        assert!(!characteristic_square_mod_6_is_one(&[40]));
        assert!(characteristic_square_mod_6_is_one(&[41]));

        assert!(!characteristic_square_mod_6_is_one(&[36, 36]));
        assert!(!characteristic_square_mod_6_is_one(&[36, 37]));
        assert!(!characteristic_square_mod_6_is_one(&[36, 38]));
        assert!(!characteristic_square_mod_6_is_one(&[36, 39]));
        assert!(!characteristic_square_mod_6_is_one(&[36, 40]));
        assert!(!characteristic_square_mod_6_is_one(&[36, 41]));

        assert!(!characteristic_square_mod_6_is_one(&[36, 41]));
        assert!(!characteristic_square_mod_6_is_one(&[37, 41]));
        assert!(!characteristic_square_mod_6_is_one(&[38, 41]));
        assert!(characteristic_square_mod_6_is_one(&[39, 41]));
        assert!(!characteristic_square_mod_6_is_one(&[40, 41]));
        assert!(characteristic_square_mod_6_is_one(&[41, 41]));
        assert!(characteristic_square_mod_6_is_one(&[1, u64::MAX]));
    }
}
