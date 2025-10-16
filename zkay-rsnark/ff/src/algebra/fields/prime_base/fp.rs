/** @file
 *****************************************************************************
 Declaration of arithmetic in the finite field F[p], for prime p of fixed length.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP_HPP_
// #define FP_HPP_

use crate::algebra::field_utils::algorithms;
use crate::algebra::field_utils::bigint::{bigint,GMP_NUMB_BITS};
use crate::algebra::fields::prime_base::fp::algorithms::tonelli_shanks_sqrt;
use crate::algebra::fields::prime_base::fp::algorithms::{Powers,PowerConfig};
 use std::ops::{AddAssign,Mul,MulAssign,SubAssign,Neg,BitXor,Sub,Add,BitXorAssign};
use crate::common::utils::bit_vector;
use num_traits::{One,Zero};
// namespace libff {

// 
// class Fp_model;

// 
// std::ostream& operator<<(std::ostream &, const Fp_model<n, modulus>&);

// 
// std::istream& operator>>(std::istream &, Fp_model<n, modulus> &);

/**
 * Arithmetic in the finite field F[p], for prime p of fixed length.
 *
 * This class implements Fp-arithmetic, for a large prime p, using a fixed number
 * of words. It is optimized for tight memory consumption, so the modulus p is
 * passed as a template parameter, to avoid per-element overheads.
 *
 * The implementation is mostly a wrapper around GMP's MPN (constant-size integers).
 * But for the integer sizes of interest for libff (3 to 5 limbs of 64 bits each),
 * we implement performance-critical routines, like addition and multiplication,
 * using hand-optimzied assembly code.
 */
// 
#[derive(Clone)]
pub struct Fp_model<const N:usize,const modulus:u128 >{
    pub mont_repr: bigint<N>,
}
//     static const mp_size_t num_limbs = n;
//     static const constexpr bigint<n>& mod = modulus;
// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static long long add_cnt;
//     static long long sub_cnt;
//     static long long mul_cnt;
//     static long long sqr_cnt;
//     static long long inv_cnt;
// //#endif
//     static std::size_t num_bits;
//     static bigint<N> euler; // (modulus-1)/2
//     static std::size_t s; // modulus = 2^s * t + 1
//     static bigint<N> t; // with t odd
//     static bigint<N> t_minus_1_over_2; // (t-1)/2
//     static Fp_model<n, modulus> nqr; // a quadratic nonresidue
//     static Fp_model<n, modulus> nqr_to_t; // nqr^t
//     static Fp_model<n, modulus> multiplicative_generator; // generator of Fp^*
//     static Fp_model<n, modulus> root_of_unity; // generator^((modulus-1)/2^s)
//     static mp_limb_t inv; // modulus^(-1) mod W, where W = 2^(word size)
//     static bigint<N> Rsquared; // R^2, where R = W^k, where k = ??
//     static bigint<N> Rcubed;   // R^3

//     Fp_model() {};
//     Fp_model(b:&bigint<n>);
//     Fp_model(const long x, const bool is_unsigned=false);

//     set_ulong(const unsigned long x);

//     /** Performs the operation montgomery_reduce(other * this.mont_repr). */
//     mul_reduce(const bigint<N> &other);

//     clear();
//     print();
//     randomize();

//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     std::vector<uint64_t> to_words();
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Returns true when the right-most bits represent a value less than the modulus.
//      *
//      * Precondition: the vector is large enough to contain ceil_size_in_bits() bits.
//      */
//     bool from_words(std::vector<uint64_t> words);

//     /* Return the standard (not Montgomery) representation of the
//        Field element's requivalence class. I.e. Fp(2).as_bigint()
//         would return bigint(2) */
//     bigint<N> as_bigint();
//     /* Return the last limb of the standard representation of the
//        field element. E.g. on 64-bit architectures Fp(123).as_ulong()
//        and Fp(2^64+123).as_ulong() would both return 123. */
//     unsigned long as_ulong();

//     bool operator==(const Fp_model& other);
//     bool operator!=(const Fp_model& other);
//     bool is_zero();

//     Fp_model& operator+=(const Fp_model& other);
//     Fp_model& operator-=(const Fp_model& other);
//     Fp_model& operator*=(const Fp_model& other);
//     Fp_model& operator^=(const unsigned long pow);
//     template<mp_size_t m>
//     Fp_model& operator^=(const bigint<m> &pow);

//     Fp_model operator+(const Fp_model& other);
//     Fp_model operator-(const Fp_model& other);
//     Fp_model operator*(const Fp_model& other);
//     Fp_model operator^(const unsigned long pow);
//     template<mp_size_t m>
//     Fp_model operator^(const bigint<m> &pow);
//     Fp_model operator-();

//     Fp_model& square();
//     Fp_model squared();
//     Fp_model& invert();
//     Fp_model inverse();
//     Fp_model Frobenius_map(unsigned long power);
//     Fp_model sqrt(); // HAS TO BE A SQUARE (else does not terminate)

//     static std::size_t ceil_size_in_bits() { return num_bits; }
//     static std::size_t floor_size_in_bits() { return num_bits - 1; }

//     static constexpr std::size_t extension_degree() { return 1; }
//     static constexpr bigint<N> field_char() { return modulus; }
//     static bool modulus_is_valid() { return modulus.data[n-1] != 0; } // mpn inverse assumes that highest limb is non-zero

//     static Fp_model<n, modulus> zero();
//     static Fp_model<n, modulus> one();
//     static Fp_model<n, modulus> random_element();
//     static Fp_model<n, modulus> geometric_generator(); // generator^k, for k = 1 to m, domain size m
//     static Fp_model<n, modulus> arithmetic_generator();// generator++, for k = 1 to m, domain size m

//     friend std::ostream& operator<< <n,modulus>(std::ostream &out, const Fp_model<n, modulus> &p);
//     friend std::istream& operator>> <n,modulus>(std::istream &in, Fp_model<n, modulus> &p);

// private:
//     /** Returns a representation in bigint, depending on the MONTGOMERY_OUTPUT flag. */
//     bigint<N> bigint_repr();
// };

// #ifdef PROFILE_OP_COUNTS
// 
// long long Fp_model<n, modulus>::add_cnt = 0;

// 
// long long Fp_model<n, modulus>::sub_cnt = 0;

// 
// long long Fp_model<n, modulus>::mul_cnt = 0;

// 
// long long Fp_model<n, modulus>::sqr_cnt = 0;

// 
// long long Fp_model<n, modulus>::inv_cnt = 0;
//#endif

// 
// size_t Fp_model<n, modulus>::num_bits;

// 
// bigint<N> Fp_model<n, modulus>::euler;

// 
// size_t Fp_model<n, modulus>::s;

// 
// bigint<N> Fp_model<n, modulus>::t;

// 
// bigint<N> Fp_model<n, modulus>::t_minus_1_over_2;

// 
// Fp_model<n, modulus> Fp_model<n, modulus>::nqr;

// 
// Fp_model<n, modulus> Fp_model<n, modulus>::nqr_to_t;

// 
// Fp_model<n, modulus> Fp_model<n, modulus>::multiplicative_generator;

// 
// Fp_model<n, modulus> Fp_model<n, modulus>::root_of_unity;

// 
// mp_limb_t Fp_model<n, modulus>::inv;

// 
// bigint<N> Fp_model<n, modulus>::Rsquared;

// 
// bigint<N> Fp_model<n, modulus>::Rcubed;

// // } // namespace libff
// use crate::algebra::fields::prime_base::fp.tcc;

//#endif // FP_HPP_




/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[p], for prime p of fixed length.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP_TCC_
// #define FP_TCC_
//#include <cassert>
//#include <cmath>
//#include <cstdlib>
//#include <limits>
//#include <vector>

use crate::algebra::field_utils::field_utils;
use crate::algebra::field_utils::fp_aux;

// namespace libff {

// using std::size_t;

impl<const N:usize,const modulus:u128 > Fp_model<N,modulus>
{
// where for<'a> &'a Fp_model<N, modulus>: MulAssign<&'a Fp_model<N, modulus>>
// 
pub fn Rsquared()->bigint<N>{
   bigint::<N>::new(0)
}
pub fn Rcubed()->bigint<N>{
     bigint::<N>::new(0)
}
pub fn mul_reduce(other:&bigint<N>)
{
    /* stupid pre-processor tricks; beware */
// #if defined(__x86_64__) && defined(USE_ASM)
//     if n == 3
//     { // Use asm-optimized Comba multiplication and reduction
//         mp_limb_t res[2*n];
//         mp_limb_t c0, c1, c2;
//         COMBA_3_BY_3_MUL(c0, c1, c2, res, self.mont_repr.data, other.data);

//         mp_limb_t k;
//         mp_limb_t tmp1, tmp2, tmp3;
//         REDUCE_6_LIMB_PRODUCT(k, tmp1, tmp2, tmp3, inv, res, modulus.data);

//         /* subtract t > mod */
//         __asm__
//             ("/* check for overflow */        \n\t"
//              MONT_CMP(16)
//              MONT_CMP(8)
//              MONT_CMP(0)

//              "/* subtract mod if overflow */  \n\t"
//              "subtract%=:                     \n\t"
//              MONT_FIRSTSUB
//              MONT_NEXTSUB(8)
//              MONT_NEXTSUB(16)
//              "done%=:                         \n\t"
//              :
//              : [tmp] "r" (res+n), [M] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//         mpn_copyi(self.mont_repr.data, res+n, n);
//     }
//     else if n == 4
//     { // use asm-optimized "CIOS method"

//         mp_limb_t tmp[n+1];
//         mp_limb_t T0=0, T1=1, cy=2, u=3; // TODO: fix this

//         __asm__ (MONT_PRECOMPUTE
//                  MONT_FIRSTITER(1)
//                  MONT_FIRSTITER(2)
//                  MONT_FIRSTITER(3)
//                  MONT_FINALIZE(3)
//                  MONT_ITERFIRST(1)
//                  MONT_ITERITER(1, 1)
//                  MONT_ITERITER(1, 2)
//                  MONT_ITERITER(1, 3)
//                  MONT_FINALIZE(3)
//                  MONT_ITERFIRST(2)
//                  MONT_ITERITER(2, 1)
//                  MONT_ITERITER(2, 2)
//                  MONT_ITERITER(2, 3)
//                  MONT_FINALIZE(3)
//                  MONT_ITERFIRST(3)
//                  MONT_ITERITER(3, 1)
//                  MONT_ITERITER(3, 2)
//                  MONT_ITERITER(3, 3)
//                  MONT_FINALIZE(3)
//                  "/* check for overflow */        \n\t"
//                  MONT_CMP(24)
//                  MONT_CMP(16)
//                  MONT_CMP(8)
//                  MONT_CMP(0)

//                  "/* subtract mod if overflow */  \n\t"
//                  "subtract%=:                     \n\t"
//                  MONT_FIRSTSUB
//                  MONT_NEXTSUB(8)
//                  MONT_NEXTSUB(16)
//                  MONT_NEXTSUB(24)
//                  "done%=:                         \n\t"
//                  :
//                  : [tmp] "r" (tmp), [A] "r" (self.mont_repr.data), [B] "r" (other.data), [inv] "r" (inv), [M] "r" (modulus.data),
//                    [T0] "r" (T0), [T1] "r" (T1), [cy] "r" (cy), [u] "r" (u)
//                  : "cc", "memory", "%rax", "%rdx"
//         );
//         mpn_copyi(self.mont_repr.data, tmp, n);
//     }
//     else if n == 5
//     { // use asm-optimized "CIOS method"

//         mp_limb_t tmp[n+1];
//         mp_limb_t T0=0, T1=1, cy=2, u=3; // TODO: fix this

//         __asm__ (MONT_PRECOMPUTE
//                  MONT_FIRSTITER(1)
//                  MONT_FIRSTITER(2)
//                  MONT_FIRSTITER(3)
//                  MONT_FIRSTITER(4)
//                  MONT_FINALIZE(4)
//                  MONT_ITERFIRST(1)
//                  MONT_ITERITER(1, 1)
//                  MONT_ITERITER(1, 2)
//                  MONT_ITERITER(1, 3)
//                  MONT_ITERITER(1, 4)
//                  MONT_FINALIZE(4)
//                  MONT_ITERFIRST(2)
//                  MONT_ITERITER(2, 1)
//                  MONT_ITERITER(2, 2)
//                  MONT_ITERITER(2, 3)
//                  MONT_ITERITER(2, 4)
//                  MONT_FINALIZE(4)
//                  MONT_ITERFIRST(3)
//                  MONT_ITERITER(3, 1)
//                  MONT_ITERITER(3, 2)
//                  MONT_ITERITER(3, 3)
//                  MONT_ITERITER(3, 4)
//                  MONT_FINALIZE(4)
//                  MONT_ITERFIRST(4)
//                  MONT_ITERITER(4, 1)
//                  MONT_ITERITER(4, 2)
//                  MONT_ITERITER(4, 3)
//                  MONT_ITERITER(4, 4)
//                  MONT_FINALIZE(4)
//                  "/* check for overflow */        \n\t"
//                  MONT_CMP(32)
//                  MONT_CMP(24)
//                  MONT_CMP(16)
//                  MONT_CMP(8)
//                  MONT_CMP(0)

//                  "/* subtract mod if overflow */  \n\t"
//                  "subtract%=:                     \n\t"
//                  MONT_FIRSTSUB
//                  MONT_NEXTSUB(8)
//                  MONT_NEXTSUB(16)
//                  MONT_NEXTSUB(24)
//                  MONT_NEXTSUB(32)
//                  "done%=:                         \n\t"
//                  :
//                  : [tmp] "r" (tmp), [A] "r" (self.mont_repr.data), [B] "r" (other.data), [inv] "r" (inv), [M] "r" (modulus.data),
//                    [T0] "r" (T0), [T1] "r" (T1), [cy] "r" (cy), [u] "r" (u)
//                  : "cc", "memory", "%rax", "%rdx"
//         );
//         mpn_copyi(self.mont_repr.data, tmp, n);
//     }
//     else
// //#endif
//     {
//         mp_limb_t res[2*n];
//         mpn_mul_n(res, self.mont_repr.data, other.data, n);

//         /*
//           The Montgomery reduction here is based on Algorithm 14.32 in
//           Handbook of Applied Cryptography
//           <http://cacr.uwaterloo.ca/hac/about/chap14.pdf>.
//          */
//         for i in 0..n
//         {
//             mp_limb_t k = inv * res[i];
//             /* calculate res = res + k * mod * b^i */
//             mp_limb_t carryout = mpn_addmul_1(res+i, modulus.data, n, k);
//             carryout = mpn_add_1(res+n+i, res+n+i, n-i, carryout);
//             assert!(carryout == 0);
//         }

//         if mpn_cmp(res+n, modulus.data, n) >= 0
//         {
//             const mp_limb_t borrow = mpn_sub(res+n, res+n, n, modulus.data, n);
// //#ifndef NDEBUG
//             assert!(borrow == 0);
// #else
//             UNUSED(borrow);
// //#endif
//         }

        // mpn_copyi(self.mont_repr.data, res+n, n);
// Self{mont_repr:bigint::<N>::new(0)}
    }



pub fn new_bigint(b:&bigint<N>)->Self
{
    // mpn_copyi(self.mont_repr.data, Rsquared.data, n);
    Self::mul_reduce(b);
Self{mont_repr:bigint::<N>::new(0)}
}


pub fn new_i64(x:i64, is_unsigned:bool)->Self
{
    // assert!(std::numeric_limits<mp_limb_t>::max() >= std::numeric_limits<long>::max() as u64, "long won't fit in mp_limb_t");
    if is_unsigned || x >= 0
    {
        // self.mont_repr.data[0] = x;//(mp_limb_t)
    }
    else
    {
        // let  borrow = mpn_sub_1(self.mont_repr.data, modulus.data, n, (mp_limb_t)-x);
//#ifndef NDEBUG
//             assert!(borrow == 0);
// #else
//             UNUSED(borrow);
//#endif
    }

    
    // Self::mul_reduce(Self::Rsquared());
    Self{mont_repr:bigint::<N>::new(0)}
}


pub fn set_ulong(&mut self,x:u64)
{
    self.mont_repr.clear();
    self.mont_repr.data[0] = x;
    Self::mul_reduce(&Self::Rsquared());
}


pub fn clear(&mut self)
{
    self.mont_repr.clear();
}


pub fn randomize(&mut self)
{
   *self = Self::random_element();
}


pub fn as_bigint(&self) ->bigint<N> 
{
    let one = bigint::<N> ::one();
    let mut res=self.clone();
    Self::mul_reduce(&one);

    return (res.mont_repr);
}


 pub fn as_ulong(&self) ->u64
{
    return self.as_bigint().as_ulong();
}


 pub fn is_zero(&self) ->bool
{
    return (self.mont_repr.is_zero()); // zero maps to zero
}


pub fn print(&self)
{
    let mut tmp=Self::zero();
    tmp.mont_repr.data[0] = 1;
    Self::mul_reduce(&self.mont_repr);

    tmp.mont_repr.print();
}


pub fn zero()->Self
{
    let mut res=Self::new_i64(0,false);
    // res.mont_repr.clear();
    return res;
}


 pub fn one()->Self
{
    let mut res=Self::new_i64(0,false);
    // res.mont_repr.data[0] = 1;
    Self::mul_reduce(&Self::Rsquared());
    return res;
}


pub fn geometric_generator()->Self
{
    let mut res=Self::new_i64(0,false);
    res.mont_repr.data[0] = 2;
    Self::mul_reduce(&Self::Rsquared());
    return res;
}


pub fn  arithmetic_generator()->Self
{
    let mut res=Self::new_i64(0,false);
    res.mont_repr.data[0] = 1;
    Self::mul_reduce(&Self::Rsquared());
    return res;
}


pub fn  squared(&self) ->Self
{
// #ifdef PROFILE_OP_COUNTS
    // self.sqr_cnt+=1;
    // self.mul_cnt-=1; // zero out the upcoming mul
//#endif
    /* stupid pre-processor tricks; beware */
// #if defined(__x86_64__) && defined(USE_ASM)
//     if n == 3
//     { // use asm-optimized Comba squaring
//         mp_limb_t res[2*n];
//         mp_limb_t c0, c1, c2;
//         COMBA_3_BY_3_SQR(c0, c1, c2, res, self.mont_repr.data);

//         mp_limb_t k;
//         mp_limb_t tmp1, tmp2, tmp3;
//         REDUCE_6_LIMB_PRODUCT(k, tmp1, tmp2, tmp3, inv, res, modulus.data);

//         /* subtract t > mod */
//         __asm__ volatile
//             ("/* check for overflow */        \n\t"
//              MONT_CMP(16)
//              MONT_CMP(8)
//              MONT_CMP(0)

//              "/* subtract mod if overflow */  \n\t"
//              "subtract%=:                     \n\t"
//              MONT_FIRSTSUB
//              MONT_NEXTSUB(8)
//              MONT_NEXTSUB(16)
//              "done%=:                         \n\t"
//              :
//              : [tmp] "r" (res+n), [M] "r" (modulus.data)
//              : "cc", "memory", "%rax");

//         Fp_model<n, modulus> r;
//         mpn_copyi(r.mont_repr.data, res+n, n);
//         return r;
//     }
//     else
// //#endif
    {
        let mut  r:Self=self.clone();
        r*=&r.clone();
        r
    }
}


pub fn square(&mut self)->&Self
{
    *self  = self.squared();
    self
}


 pub fn invert(&self)->&Self
{
// #ifdef PROFILE_OP_COUNTS
    // self.inv_cnt++;
//#endif

    assert!(!self.is_zero());

    let mut  g=bigint::<N>::new(0); /* gp should have room for vn = n limbs */

    let  s=vec![0;N+1]; //mp_limb_t/* sp should have room for vn+1 limbs */
    let  sn:i32=0;

    let  v = modulus; // both source operands are destroyed by mpn_gcdext

    /* computes gcd(u, v) = g = u*s + v*t, so s*u will be 1 (mod v) */
    // let  gn = mpn_gcdext(g.data, s, &sn, self.mont_repr.data, N, v.data, N);
//#ifndef NDEBUG
//     assert!(gn == 1 && g.data[0] == 1); /* inverse exists */
// #else
    // UNUSED(gn);
//#endif

    // let  q; /* division result fits into q, as sn <= n+1 */
    /* sn < 0 indicates negative sn; will fix up later */

    if sn.abs() >= N as i32
    {
        /* if sn could require modulus reduction, do it here */
        // mpn_tdiv_qr(&q, self.mont_repr.data, 0, s, sn.abs(), modulus.data, N);
    }
    else
    {
        /* otherwise just copy it over */
        // mpn_zero(self.mont_repr.data, N);
        // mpn_copyi(self.mont_repr.data, s, sn.abs());
    }

    /* fix up the negative sn */
    if sn < 0
    {
        // let  borrow = mpn_sub_n(self.mont_repr.data, modulus.data, self.mont_repr.data, N);
//#ifndef NDEBUG
//         assert!(borrow == 0);
// #else
//         UNUSED(borrow);
//#endif
    }

    Self::mul_reduce(&Self::Rcubed());
    return self;
}


pub fn  inverse(&self) ->Self
{
    let mut  r=self.clone();
    r.invert();
    r
}


pub fn  Frobenius_map(&self, power:u64)->Self
{
    // UNUSED(power); // only for API consistency
    // Fp_model<N,modulus> copy = *this;
    // return copy;
    self.clone()
}


pub fn  random_element() ->Self
{// / returns random element of Fp_model
    /* note that as Montgomery representation is a bijection then
       selecting a random element of {xR} is the same as selecting a
       random element of {x} */
    let mut  r=Self::new_i64(0,false);
    loop
    {
        r.mont_repr.randomize();

        /* clear all bits higher than MSB of modulus */
        let mut bitno = GMP_NUMB_BITS * N - 1;
        while modulus>>bitno&1!=0//.test_bit(bitno)
        {
           let part = bitno/GMP_NUMB_BITS;
            let bit = bitno - (GMP_NUMB_BITS*part);

            let  one = 1;
            r.mont_repr.data[part] &= !(one<<bit);

            bitno-=1;
        }
        // if r.mont_repr.data[..N]< modulus.data[..N]{
        //     break
        // }
    }
   /* if r.data is still >= modulus -- repeat (rejection sampling) */
    // while (mpn_cmp(r.mont_repr.data, modulus.data, N) >= 0);

    return r;
}


pub fn  sqrt(&self) ->Self
{
    // return tonelli_shanks_sqrt(self);
    self.clone()
}


pub fn to_words(&self)  ->Vec<u64>
{
    // TODO: implement for other bit architectures
    assert!(GMP_NUMB_BITS == 64, "Only 64-bit architectures are currently supported");
    let  repr = self.bigint_repr();
    repr.data.clone().try_into().unwrap()
}


 pub fn from_words(&mut self,words:&Vec<u64>)->bool
{
    // TODO: implement for other bit architectures
    assert!(GMP_NUMB_BITS == 64, "Only 64-bit architectures are currently supported");

    // type FieldT=Fp_model<N, modulus> ; // Without the typedef C++ doesn't compile.
    let start_bit = words.len() * 64 ;//- FieldT::ceil_size_in_bits();
    assert!(start_bit >= 0); // Check the vector is big enough.
    let start_word = start_bit / 64;
    let bit_offset = start_bit % 64;

    // Assumes mont_repr.data is just the right size to fit ceil_size_in_bits().
    // std::copy(words.begin() + start_word, words.end(), self.mont_repr.data);
    self.mont_repr.data.clone_from_slice(&words[start_word..]);
    // Zero out the left-most bit_offset bits.
    self.mont_repr.data[N - 1] = ((self.mont_repr.data[N - 1] as u64) << bit_offset) >> bit_offset;//mp_limb_t
//#ifndef MONTGOMERY_OUTPUT
    Self::mul_reduce(&Self::Rsquared());
//#endif
    // return self.mont_repr < modulus;
    false
}


 pub fn bigint_repr(&self) ->bigint<N>
{
    // If the flag is defined, serialization and words output use the montgomery representation
    // instead of the human-readable value.
// #ifdef MONTGOMERY_OUTPUT
    return self.mont_repr.clone();
// #else
    // return self.as_bigint();
//#endif
}
}
// } // namespace libff
//#endif // FP_TCC_

impl<const N:usize,const modulus:u128> PartialEq for Fp_model<N,modulus>  {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
       self.mont_repr == other.mont_repr
    }
}

// 
// bool pub fn operator==(const Fp_model& other)
// {
//     return (self.mont_repr == other.mont_repr);
// }

// 
// bool pub fn operator!=(const Fp_model& other)
// {
//     return (self.mont_repr != other.mont_repr);
// }

impl<const N:usize,const modulus:u128> AddAssign for Fp_model<N,modulus>  {
    fn add_assign(&mut self, other: Self) {
// #ifdef PROFILE_OP_COUNTS
// self.add_cnt++;
//#endif
// #if defined(__x86_64__) && defined(USE_ASM)
//     if N == 3
//     {
//         __asm__
//             ("/* perform bignum addition */   \n\t"
//              ADD_FIRSTADD
//              ADD_NEXTADD(8)
//              ADD_NEXTADD(16)
//              "/* if overflow: subtract     */ \n\t"
//              "/* (tricky point: if A and B are in the range we do not need to do anything special for the possible carry flag) */ \n\t"
//              "jc      subtract%=              \n\t"

//              "/* check for overflow */        \n\t"
//              ADD_CMP(16)
//              ADD_CMP(8)
//              ADD_CMP(0)

//              "/* subtract mod if overflow */  \n\t"
//              "subtract%=:                     \n\t"
//              ADD_FIRSTSUB
//              ADD_NEXTSUB(8)
//              ADD_NEXTSUB(16)
//              "done%=:                         \n\t"
//              :
//              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//     }
//     else if n == 4
//     {
//         __asm__
//             ("/* perform bignum addition */   \n\t"
//              ADD_FIRSTADD
//              ADD_NEXTADD(8)
//              ADD_NEXTADD(16)
//              ADD_NEXTADD(24)
//              "/* if overflow: subtract     */ \n\t"
//              "/* (tricky point: if A and B are in the range we do not need to do anything special for the possible carry flag) */ \n\t"
//              "jc      subtract%=              \n\t"

//              "/* check for overflow */        \n\t"
//              ADD_CMP(24)
//              ADD_CMP(16)
//              ADD_CMP(8)
//              ADD_CMP(0)

//              "/* subtract mod if overflow */  \n\t"
//              "subtract%=:                     \n\t"
//              ADD_FIRSTSUB
//              ADD_NEXTSUB(8)
//              ADD_NEXTSUB(16)
//              ADD_NEXTSUB(24)
//              "done%=:                         \n\t"
//              :
//              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//     }
//     else if n == 5
//     {
//         __asm__
//             ("/* perform bignum addition */   \n\t"
//              ADD_FIRSTADD
//              ADD_NEXTADD(8)
//              ADD_NEXTADD(16)
//              ADD_NEXTADD(24)
//              ADD_NEXTADD(32)
//              "/* if overflow: subtract     */ \n\t"
//              "/* (tricky point: if A and B are in the range we do not need to do anything special for the possible carry flag) */ \n\t"
//              "jc      subtract%=              \n\t"

//              "/* check for overflow */        \n\t"
//              ADD_CMP(32)
//              ADD_CMP(24)
//              ADD_CMP(16)
//              ADD_CMP(8)
//              ADD_CMP(0)

//              "/* subtract mod if overflow */  \n\t"
//              "subtract%=:                     \n\t"
//              ADD_FIRSTSUB
//              ADD_NEXTSUB(8)
//              ADD_NEXTSUB(16)
//              ADD_NEXTSUB(24)
//              ADD_NEXTSUB(32)
//              "done%=:                         \n\t"
//              :
//              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//     }
//     else
// //#endif
//     {
//         mp_limb_t scratch[n+1];
//         const mp_limb_t carry = mpn_add_n(scratch, self.mont_repr.data, other.mont_repr.data, n);
//         scratch[n] = carry;

//         if carry || mpn_cmp(scratch, modulus.data, n) >= 0
//         {
//             const mp_limb_t borrow = mpn_sub(scratch, scratch, n+1, modulus.data, n);
// //#ifndef NDEBUG
//             assert!(borrow == 0);
// #else
//             UNUSED(borrow);
// //#endif
//         }

        // mpn_copyi(self.mont_repr.data, scratch, n);
    // }

    // return *this;
    }
}

// 
// Fp_model<n,modulus>& pub fn operator+=(const Fp_model<n,modulus>& other)
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
// //#endif
// // #if defined(__x86_64__) && defined(USE_ASM)
// //     if n == 3
// //     {
// //         __asm__
// //             ("/* perform bignum addition */   \n\t"
// //              ADD_FIRSTADD
// //              ADD_NEXTADD(8)
// //              ADD_NEXTADD(16)
// //              "/* if overflow: subtract     */ \n\t"
// //              "/* (tricky point: if A and B are in the range we do not need to do anything special for the possible carry flag) */ \n\t"
// //              "jc      subtract%=              \n\t"

// //              "/* check for overflow */        \n\t"
// //              ADD_CMP(16)
// //              ADD_CMP(8)
// //              ADD_CMP(0)

// //              "/* subtract mod if overflow */  \n\t"
// //              "subtract%=:                     \n\t"
// //              ADD_FIRSTSUB
// //              ADD_NEXTSUB(8)
// //              ADD_NEXTSUB(16)
// //              "done%=:                         \n\t"
// //              :
// //              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
// //              : "cc", "memory", "%rax");
// //     }
// //     else if n == 4
// //     {
// //         __asm__
// //             ("/* perform bignum addition */   \n\t"
// //              ADD_FIRSTADD
// //              ADD_NEXTADD(8)
// //              ADD_NEXTADD(16)
// //              ADD_NEXTADD(24)
// //              "/* if overflow: subtract     */ \n\t"
// //              "/* (tricky point: if A and B are in the range we do not need to do anything special for the possible carry flag) */ \n\t"
// //              "jc      subtract%=              \n\t"

// //              "/* check for overflow */        \n\t"
// //              ADD_CMP(24)
// //              ADD_CMP(16)
// //              ADD_CMP(8)
// //              ADD_CMP(0)

// //              "/* subtract mod if overflow */  \n\t"
// //              "subtract%=:                     \n\t"
// //              ADD_FIRSTSUB
// //              ADD_NEXTSUB(8)
// //              ADD_NEXTSUB(16)
// //              ADD_NEXTSUB(24)
// //              "done%=:                         \n\t"
// //              :
// //              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
// //              : "cc", "memory", "%rax");
// //     }
// //     else if n == 5
// //     {
// //         __asm__
// //             ("/* perform bignum addition */   \n\t"
// //              ADD_FIRSTADD
// //              ADD_NEXTADD(8)
// //              ADD_NEXTADD(16)
// //              ADD_NEXTADD(24)
// //              ADD_NEXTADD(32)
// //              "/* if overflow: subtract     */ \n\t"
// //              "/* (tricky point: if A and B are in the range we do not need to do anything special for the possible carry flag) */ \n\t"
// //              "jc      subtract%=              \n\t"

// //              "/* check for overflow */        \n\t"
// //              ADD_CMP(32)
// //              ADD_CMP(24)
// //              ADD_CMP(16)
// //              ADD_CMP(8)
// //              ADD_CMP(0)

// //              "/* subtract mod if overflow */  \n\t"
// //              "subtract%=:                     \n\t"
// //              ADD_FIRSTSUB
// //              ADD_NEXTSUB(8)
// //              ADD_NEXTSUB(16)
// //              ADD_NEXTSUB(24)
// //              ADD_NEXTSUB(32)
// //              "done%=:                         \n\t"
// //              :
// //              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
// //              : "cc", "memory", "%rax");
// //     }
// //     else
// // //#endif
// //     {
// //         mp_limb_t scratch[n+1];
// //         const mp_limb_t carry = mpn_add_n(scratch, self.mont_repr.data, other.mont_repr.data, n);
// //         scratch[n] = carry;

// //         if carry || mpn_cmp(scratch, modulus.data, n) >= 0
// //         {
// //             const mp_limb_t borrow = mpn_sub(scratch, scratch, n+1, modulus.data, n);
// // //#ifndef NDEBUG
// //             assert!(borrow == 0);
// // #else
// //             UNUSED(borrow);
// // //#endif
// //         }

//         // mpn_copyi(self.mont_repr.data, scratch, n);
//     // }

//     return *this;
// }
impl<const N:usize,const modulus:u128> SubAssign for Fp_model<N,modulus>  {
    fn sub_assign(&mut self, other: Self) {
        // #ifdef PROFILE_OP_COUNTS
        // self.sub_cnt++;
        //#endif
// #if defined(__x86_64__) && defined(USE_ASM)
//     if n == 3
//     {
//         __asm__
//             (SUB_FIRSTSUB
//              SUB_NEXTSUB(8)
//              SUB_NEXTSUB(16)

//              "jnc     done%=\n\t"

//              SUB_FIRSTADD
//              SUB_NEXTADD(8)
//              SUB_NEXTADD(16)

//              "done%=:\n\t"
//              :
//              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//     }
//     else if n == 4
//     {
//         __asm__
//             (SUB_FIRSTSUB
//              SUB_NEXTSUB(8)
//              SUB_NEXTSUB(16)
//              SUB_NEXTSUB(24)

//              "jnc     done%=\n\t"

//              SUB_FIRSTADD
//              SUB_NEXTADD(8)
//              SUB_NEXTADD(16)
//              SUB_NEXTADD(24)

//              "done%=:\n\t"
//              :
//              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//     }
//     else if n == 5
//     {
//         __asm__
//             (SUB_FIRSTSUB
//              SUB_NEXTSUB(8)
//              SUB_NEXTSUB(16)
//              SUB_NEXTSUB(24)
//              SUB_NEXTSUB(32)

//              "jnc     done%=\n\t"

//              SUB_FIRSTADD
//              SUB_NEXTADD(8)
//              SUB_NEXTADD(16)
//              SUB_NEXTADD(24)
//              SUB_NEXTADD(32)

//              "done%=:\n\t"
//              :
//              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
//              : "cc", "memory", "%rax");
//     }
//     else
// //#endif
    {
        // mp_limb_t scratch[n+1];
        // if mpn_cmp(self.mont_repr.data, other.mont_repr.data, n) < 0
        // {
        //     const mp_limb_t carry = mpn_add_n(scratch, self.mont_repr.data, modulus.data, n);
        //     scratch[n] = carry;
        // }
        // else
        // {
        //     mpn_copyi(scratch, self.mont_repr.data, n);
        //     scratch[n] = 0;
        // }

        // const mp_limb_t borrow = mpn_sub(scratch, scratch, n+1, other.mont_repr.data, n);
//#ifndef NDEBUG
//         assert!(borrow == 0);
// #else
//         UNUSED(borrow);
//#endif

        // mpn_copyi(self.mont_repr.data, scratch, n);
    }
    // return *this;
    }
}
// 
// Fp_model<n,modulus>& pub fn operator-=(const Fp_model<n,modulus>& other)
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif
// // #if defined(__x86_64__) && defined(USE_ASM)
// //     if n == 3
// //     {
// //         __asm__
// //             (SUB_FIRSTSUB
// //              SUB_NEXTSUB(8)
// //              SUB_NEXTSUB(16)

// //              "jnc     done%=\n\t"

// //              SUB_FIRSTADD
// //              SUB_NEXTADD(8)
// //              SUB_NEXTADD(16)

// //              "done%=:\n\t"
// //              :
// //              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
// //              : "cc", "memory", "%rax");
// //     }
// //     else if n == 4
// //     {
// //         __asm__
// //             (SUB_FIRSTSUB
// //              SUB_NEXTSUB(8)
// //              SUB_NEXTSUB(16)
// //              SUB_NEXTSUB(24)

// //              "jnc     done%=\n\t"

// //              SUB_FIRSTADD
// //              SUB_NEXTADD(8)
// //              SUB_NEXTADD(16)
// //              SUB_NEXTADD(24)

// //              "done%=:\n\t"
// //              :
// //              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
// //              : "cc", "memory", "%rax");
// //     }
// //     else if n == 5
// //     {
// //         __asm__
// //             (SUB_FIRSTSUB
// //              SUB_NEXTSUB(8)
// //              SUB_NEXTSUB(16)
// //              SUB_NEXTSUB(24)
// //              SUB_NEXTSUB(32)

// //              "jnc     done%=\n\t"

// //              SUB_FIRSTADD
// //              SUB_NEXTADD(8)
// //              SUB_NEXTADD(16)
// //              SUB_NEXTADD(24)
// //              SUB_NEXTADD(32)

// //              "done%=:\n\t"
// //              :
// //              : [A] "r" (self.mont_repr.data), [B] "r" (other.mont_repr.data), [mod] "r" (modulus.data)
// //              : "cc", "memory", "%rax");
// //     }
// //     else
// // //#endif
//     {
//         mp_limb_t scratch[n+1];
//         if mpn_cmp(self.mont_repr.data, other.mont_repr.data, n) < 0
//         {
//             const mp_limb_t carry = mpn_add_n(scratch, self.mont_repr.data, modulus.data, n);
//             scratch[n] = carry;
//         }
//         else
//         {
//             mpn_copyi(scratch, self.mont_repr.data, n);
//             scratch[n] = 0;
//         }

//         const mp_limb_t borrow = mpn_sub(scratch, scratch, n+1, other.mont_repr.data, n);
// //#ifndef NDEBUG
//         assert!(borrow == 0);
// #else
//         UNUSED(borrow);
// //#endif

//         mpn_copyi(self.mont_repr.data, scratch, n);
//     }
//     return *this;
// }
impl<const N:usize,const modulus:u128> MulAssign<&Self>for Fp_model<N,modulus>  {
    fn mul_assign(&mut self, rhs: &Self) {
        // // #ifdef PROFILE_OP_COUNTS
        //     self.mul_cnt++;
        // //#endif
        Self::mul_reduce(&rhs.mont_repr);
        // *self
    }
}
// 
// Fp_model<n,modulus>& pub fn operator*=(const Fp_model<n,modulus>& other)
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.mul_cnt++;
// //#endif

//     Self::mul_reduce(other.mont_repr);
//     return *this;
// }

impl<const N:usize,const modulus:u128> BitXorAssign<u64> for Fp_model<N,modulus>  {
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = Powers::power::<Fp_model::<N, modulus> >(self, rhs);
    }
}

// 
// Fp_model<n,modulus>& pub fn operator^=(const unsigned long pow)
// {
//     (*this) = power<Fp_model<n, modulus> >(*this, pow);
//     return (*this);
// }
impl<const N:usize,const M:usize,const modulus:u128> BitXorAssign<&bigint<M>> for Fp_model<N,modulus>  {
    fn bitxor_assign(&mut self, rhs: &bigint<M>) {
        *self = Powers::power::<Fp_model::<N, modulus>>(self, rhs);
    }
}
// 
// template<mp_size_t m>
// Fp_model<n,modulus>& pub fn operator^=(const bigint<m> &pow)
// {
//     (*this) = power<Fp_model<n, modulus>, m>(*this, pow);
//     return (*this);
// }
impl<const N:usize,const modulus:u128> Add for Fp_model<N,modulus>  {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r=self;
        r+=other;
        r
    }
}
// 
// pub fn  operator+(const Fp_model<n,modulus>& other)
// {
//     Fp_model<n, modulus> r(*this);
//     return (r += other);
// }
impl<const N:usize,const modulus:u128> Sub for  Fp_model<N,modulus>  {
    type Output = Self;

    fn sub(self, other: Self) -> <Fp_model<N, modulus> as Sub>::Output  {
        let mut r=self;
        r-=other;
        r
    }
}
// 
// pub fn  operator-(const Fp_model<n,modulus>& other)
// {
//     Fp_model<n, modulus> r(*this);
//     return (r -= other);
// }
impl<const N:usize,const modulus:u128> Mul for  Fp_model<N,modulus> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
         let mut r=self;
        r *= &rhs;
        r
    }
}

// 
// pub fn  operator*(const Fp_model<n,modulus>& other)
// {
//     Fp_model<n, modulus> r(*this);
//     return (r *= other);
// }
impl<const N:usize,const modulus:u128> BitXor<u64> for Fp_model<N, modulus> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output  {
            let mut r=self;
            r ^= rhs;
            r
    }
}
// 
// pub fn  operator^(const unsigned long pow)
// {
//     Fp_model<n, modulus> r(*this);
//     return (r ^= pow);
// }
impl<const N:usize,const M:usize,const modulus:u128> BitXor<&bigint<M>> for Fp_model<N,modulus>{
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<M>) -> Self::Output {
            let mut r=self;
            r ^= rhs;
            r
    }
}
// 
// template<mp_size_t m>
// pub fn  operator^(const bigint<m> &pow)
// {
//     Fp_model<n, modulus> r(*this);
//     return (r ^= pow);
// }
impl<const N:usize,const modulus:u128>  Neg for Fp_model<N,modulus> {
    type Output = Self;

    fn neg(self) -> Self::Output {
            // #ifdef PROFILE_OP_COUNTS
                // self.sub_cnt++;
            //#endif

            // if self.is_zero()
            // {
            //     return self;
            // }
            // else
            // {
                let mut  r=Self::new_i64(0,false);
                // mpn_sub_n(r.mont_repr.data, modulus.data, self.mont_repr.data, n);
                return r;
            // }
    }
}
// 
// pub fn  operator-()
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.sub_cnt++;
// //#endif

//     if self.is_zero()
//     {
//         return (*this);
//     }
//     else
//     {
//         Fp_model<n, modulus> r;
//         mpn_sub_n(r.mont_repr.data, modulus.data, self.mont_repr.data, n);
//         return r;
//     }
// }
use std::fmt;
impl<const N:usize,const modulus:u128> fmt::Display for Fp_model<N, modulus>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  
    self.bigint_repr() ,
)
    }
}

// 
// std::ostream& operator<<(std::ostream &out, const Fp_model<n, modulus> &p)
// {
//     out << p.bigint_repr();
//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, Fp_model<n, modulus> &p)
// {
//     in >> p.mont_repr;
// //#ifndef MONTGOMERY_OUTPUT
//     p.mul_reduce(Self::Rsquared());
// //#endif
//     return in;
// }
impl<const N:usize,const modulus:u128> One for Fp_model<N, modulus> {
    fn one()->Self{
        Self::one()
    }
}

impl<const N:usize,const modulus:u128> Zero for Fp_model<N, modulus> {
    fn zero()->Self{
        Self::zero()
    }
    fn is_zero(&self)->bool{
    false
    }
}