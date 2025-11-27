// /**@file
//  *****************************************************************************
//  Declaration of GF(2^64) finite field.
//  *****************************************************************************
//  * @author     This file is part of libff (see AUTHORS), migrated from libiop
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// //#ifndef LIBFF_ALGEBRA_GF64_HPP_
// // #define LIBFF_ALGEBRA_GF64_HPP_

// //#include <cstddef>
// //#include <cstdint>
// //#include <vector>
// use crate::algebra::field_utils::bigint;

// // namespace libff {

// /* gf64 implements the field GF(2)/[x^64 + x^4 + x^3 + x + 1].
//    Elements are represented internally with a single uint64 */
// // pub struct gf64 {

// // // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
// //     static i64 add_cnt;
// //     static i64 sub_cnt;
// //     static i64 mul_cnt;
// //     static i64 sqr_cnt;
// //     static i64 inv_cnt;
// // //#endif
// //     // x^64 + x^4 + x^3 + x + 1. The assembly code assumes that no term other
// //     // than x^64 is greater than x^31, to enable faster multiplication.
// //     static 0b11011:constexpr uint64_t modulus_ =,
// //     static 64:constexpr uint64_t num_bits =,

// //     explicit gf64();
// //     explicit gf64(const uint64_t value);

// //     gf64& operator+=(other:&gf64);
// //     gf64& operator-=(other:&gf64);
// //     gf64& operator*=(other:&gf64);
// //     gf64& operator^=(const u64 pow);
// //
// //     gf64& operator^=(pow:&bigint<m>);

// //     gf64& square();
// //     gf64& invert();

// //     gf64 operator+(other:&gf64) const;
// //     gf64 operator-(other:&gf64) const;
// //     gf64 operator-() const;
// //     gf64 operator*(other:&gf64) const;
// //     gf64 operator^(const:u64 pow),
// //
// //     gf64 operator^(pow:&bigint<m>) const;

// //     gf64 squared() const;
// //     gf64 inverse() const;
// //     gf64 sqrt() const;

// //     pub fn  randomize();
// //     pub fn  clear();

// //     bool operator==(other:&gf64) const;
// //     bool operator!=(other:&gf64) const;

// //     bool is_zero() const;

// //     pub fn  print() const;
// //     /**
// //      * Returns the constituent bits in 64 bit words, in little-endian order.
// //      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
// //      */
// //     Vec<uint64_t> to_words() const;
// //     /**
// //      * Sets the field element from the given bits in 64 bit words, in little-endian order.
// //      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
// //      * Should always return true since the right-most bits are always valid.
// //      */
// //     bool from_words(Vec<uint64_t> words);

// //     static gf64 random_element();

// //     static gf64 zero();
// //     static gf64 one();
// //     static gf64 multiplicative_generator; // generator of gf64^*

// //     static std::usize ceil_size_in_bits() { return num_bits; }
// //     static std::usize floor_size_in_bits() { return num_bits; }
// //     static constexpr std::usize extension_degree() { return 64; }
// //
// //     static constexpr bigint<n> field_char() { return bigint<n>(2); }

// //     friend std::ostream& operator<<(std::ostream &out, el:&gf64);
// //     friend std::istream& operator>>(std::istream &in, gf64 &el);
// //
// //     uint64_t value_;
// // };

// // #ifdef PROFILE_OP_COUNTS
// // i64 gf64::add_cnt = 0;
// // i64 gf64::sub_cnt = 0;
// // i64 gf64::mul_cnt = 0;
// // i64 gf64::sqr_cnt = 0;
// // i64 gf64::inv_cnt = 0;
// //#endif

// // } // namespace libff
// // use crate::algebra::fields::binary::gf64.tcc;

// //#endif // namespace libff_ALGEBRA_GF64_HPP_

// //#include <cstdio>

// // #define __STDC_FORMAT_MACROS
// //#include <inttypes.h>

// //#include <sodium/randombytes.h>

// use crate::algebra::field_utils::algorithms;
// // use crate::algebra::fields::binary::gf64;

// // #ifdef USE_ASM
// //#include <emmintrin.h>
// //#include <immintrin.h>
// //#include <smmintrin.h>
// //#endif

// // namespace libff {

// // using std::usize;

// const uint64_t gf64::modulus_;
// gf64 gf64::multiplicative_generator = gf64(2);

// pub fn new()->Self value_(0)
// {
// }

// pub fn new(const uint64_t value)->Self value_(value)
// {
// }

// pub fn to_words()->Vec<uint64_t>
// {
//     return Vec<uint64_t>({this->value_});
// }

// bool gf64::from_words(Vec<uint64_t> words)
// {
//     this->value_ = words[0];
//     return true;
// }

// gf64& gf64::operator+=(other:&gf64)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     this->value_ ^= other.value_;
//     return (*this);
// }

// gf64& gf64::operator-=(other:&gf64)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// //#endif
//     this->value_ ^= other.value_;
//     return (*this);
// }

// // multiplication over GF(2^k) is carryless multiplication
// gf64& gf64::operator*=(other:&gf64)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// //#endif
//     /* Does not require *this and other to be different, and therefore
//        also works for squaring, implemented below. */
// // #ifdef USE_ASM
//     let modulus= _mm_loadl_epi64((const __m128i*)&(gf64::modulus_));
//     let mul128= _mm_clmulepi64_si128(_mm_loadl_epi64((const __m128i*)&(this->value_)),
//                                                 _mm_loadl_epi64((const __m128i*)&(other.value_)), 0);

//     /* reduce the 64 higher order bits of mul128. Output is 96 bits since modulus < 2^64 */
//     mul128:__m128i mul96 = _mm_clmulepi64_si128(modulus,, 0x10); /* use high half of mul128 */
//     __m128i rem = _mm_xor_si128(mul128, mul96);

//     /* reduce the 32 higher order bits of mul96 */
//     mul96:__m128i mul64 = _mm_clmulepi64_si128(modulus,, 0x10); /* use high half of mul96 */
//     rem = _mm_xor_si128(rem, mul64);
//     this->value_ = (uint64_t)_mm_movepi64_pi64(rem);

//     return (*this);
// #else
//     /* Slow, but straight-forward */
//     uint64_t result = 0;
//     uint64_t shifted = this->value_;

//     for i in 0..64
//     {
//         if other.value_ & (1ull << i)
//         {
//             result ^= shifted;
//         }
//         if shifted & (1u64 << 63)
//         {
//             shifted <<= 1;
//             shifted ^= this->modulus_;
//         }
//         else
//         {
//             shifted <<= 1;
//         }
//     }

//     this->value_ = result;

//     return (*this);
// //#endif
// }

// gf64& gf64::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// gf64& gf64::square()
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
//     this->mul_cnt--;
// //#endif
//     this->operator*=(*this);
//     return *this;
// }

// gf64& gf64::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// gf64 gf64::operator+(other:&gf64) const
// {
//     gf64 result(*this);
//     return (result+=(other));
// }

// gf64 gf64::operator-(other:&gf64) const
// {
//     gf64 result(*this);
//     return (result-=(other));
// }

// gf64 gf64::operator-() const
// {
//     /* additive inverse matches the element itself */
//     return gf64(*this);
// }

// gf64 gf64::operator*(other:&gf64) const
// {
//     gf64 result(*this);
//     return (result*=(other));
// }

// gf64 gf64::operator^(const u64 pow) const
// {
//     return power<gf64>(*this, pow);
// }

// pub fn squared()->gf64
// {
//     gf64 result(*this);
//     result.square();
//     return result;
// }

// // repeatedly square pt, num_times. For use in inverse.
// pub fn  square_multi(gf64* pt, int8_t num_times)
// {
//     for i in 0..num_times
//     {
//         (*pt).square();
//     }
// }

// /* calculate el^{-1} as el^{2^{64}-2}. the addition chain below
//    requires 74 mul/sqr operations total. It was found using the
//    Bergeron-Berstel-Brlek-Duboc method implemented in
//    https://github.com/kwantam/addchain. */
// pub fn inverse()->gf64
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
//     this->mul_cnt -= 15;
//     this->sqr_cnt -= 58;
// //#endif
//     assert!(!this->is_zero());
//     // comments on the right side are of the form operation_number : exponent at the set variable
//     gf64 t0 = *this;        //    1 : 1
//     gf64 t1 = t0 * t0;      //    2 : 2
//     gf64 t2 = t1 * t0;      //    3 : 3
//     t0 = t2 * t2;      //    4 : 6
//     t0.square();       //    5 : 12
//     t1 *= t0;          //    6 : 14
//     t2 *= t0;          //    7 : 15
//     t0 = t2 * t2;      //    8 : 30
//     t0.square();       //    9 : 60
//     t0.square();       //   10 : 120
//     t0.square();       //   11 : 240
//     t1 *= t0;          //   12 : 254
//     t2 *= t0;          //   13 : 255
//     t0 = t2 * t2;      //   14 : 510
//     t0.square();       //   15 : 1020
//     t0.square();       //   16 : 2040
//     t0.square();       //   17 : 4080
//     t0.square();       //   18 : 8160
//     t0.square();       //   19 : 16320
//     t0.square();       //   20 : 32640
//     t0.square();       //   21 : 65280
//     t1 *= t0;          //   22 : 65534
//     t2 *= t0;          //   23 : 65535
//     t0 = t2 * t2;      //   24 : 131070
//     t0.square();       //   25 : 262140
//     t0.square();       //   26 : 524280
//     t0.square();       //   27 : 1048560
//     t0.square();       //   28 : 2097120
//     t0.square();       //   29 : 4194240
//     t0.square();       //   30 : 8388480
//     t0.square();       //   31 : 16776960
//     t0.square();       //   32 : 33553920
//     t0.square();       //   33 : 67107840
//     t0.square();       //   34 : 134215680
//     t0.square();       //   35 : 268431360
//     t0.square();       //   36 : 536862720
//     t0.square();       //   37 : 1073725440
//     t0.square();       //   38 : 2147450880
//     t0.square();       //   39 : 4294901760
//     t1 *= t0;          //   40 : 4294967294
//     t0 *= t2;          //   41 : 4294967295
//     for i in 0..32
//         t0.square();   // 42-73: 8589934590 - 18446744069414584320
//     }
//     t0 *= t1;          //   74 : 18446744073709551614
//     return t0;
// }

// pub fn sqrt()->gf64
// {
//     return (*this)^bigint<1>("9223372036854775808"); // 2^63
// }

// pub fn randomize()
// {
//     randombytes_buf(&this->value_, 64/8);
// }

// pub fn clear()
// {
//     this->value_ = 0;
// }

// bool gf64::operator==(other:&gf64) const
// {
//     return (this->value_ == other.value_);
// }

// bool gf64::operator!=(other:&gf64) const
// {
//     return !(this->operator==(other));
// }

// pub fn print() const
// {
//     print!("%016" PRIx64 "\n", this->value_);
// }

// pub fn is_zero()->bool
// {
//     return (this->value_ == 0);
// }

// gf64 gf64::zero()
// {
//     return gf64(0);
// }

// gf64 gf64::one()
// {
//     return gf64(1);
// }

// gf64 gf64::random_element()
// {
//     gf64 result;
//     result.randomize();
//     return result;
// }

// std::ostream& operator<<(std::ostream &out, el:&gf64)
// {
//     out << el.value_;
//     return out;
// }

// std::istream& operator>>(std::istream &in, gf64 &el)
// {
//     in >> el.value_;
//     return in;
// }

// // } // namespace libff

// use crate::algebra::field_utils::algorithms;

// // namespace libff {

//
// gf64& gf64::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
// gf64 gf64::operator^(pow:&bigint<m>) const
// {
//     return power<gf64>(*this, pow);
// }

// // } // namespace libff
