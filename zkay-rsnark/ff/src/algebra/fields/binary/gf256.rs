
//  Declaration of GF(2^256) finite field.


// /* x^256 + x^10 + x^5 + x^2 + 1 */
// /* gf256 implements the field GF(2)/(x^256 + x^10 + x^5 + x^2 + 1).
//    Elements are represented internally with four uint64s */
// pub struct gf256 {

// // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
//     static i64 add_cnt;
//     static i64 sub_cnt;
//     static i64 mul_cnt;
//     static i64 sqr_cnt;
//     static i64 inv_cnt;
// 
//     // x^256 + x^10 + x^5 + x^2 + 1
//     static 0b10000100101:constexpr uint64_t modulus_ =,
//     static 256:constexpr uint64_t num_bits =,

//     explicit gf256();
//     /* we need a constructor that only initializes the low 64 bits of value_ to
//        be able to do gf256(0) and gf256(1). */
//     explicit gf256(const uint64_t value_low);
//     explicit gf256(value_high:uint64_t, value_midh:uint64_t,
//                    value_midl:uint64_t, const uint64_t value_low);

//     gf256& operator+=(other:&gf256);
//     gf256& operator-=(other:&gf256);
//     gf256& operator*=(other:&gf256);
//     gf256& operator^=(const u64 pow);
//
//     gf256& operator^=(pow:&bigint<m>);

//     gf256& square();
//     gf256& invert();

//     gf256 operator+(other:&gf256) const;
//     gf256 operator-(other:&gf256) const;
//     gf256 operator-() const;
//     gf256 operator*(other:&gf256) const;
//     gf256 operator^(const:u64 pow),
//
//     gf256 operator^(pow:&bigint<m>) const;

//     gf256 squared() const;
//     gf256 inverse() const;
//     gf256 sqrt() const;

//     pub fn  randomize();
//     pub fn  clear();

//     bool operator==(other:&gf256) const;
//     bool operator!=(other:&gf256) const;

//     bool is_zero() const;

//     pub fn  print() const;
//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     Vec<uint64_t> to_words() const;
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Should always return true since the right-most bits are always valid.
//      */
//     bool from_words(Vec<uint64_t> words);

//     static gf256 random_element();

//     static gf256 zero();
//     static gf256 one();
//     static gf256 multiplicative_generator; // generator of gf256^*

//     static std::usize ceil_size_in_bits() { return num_bits; }
//     static std::usize floor_size_in_bits() { return num_bits; }
//     static constexpr std::usize extension_degree() { return 256; }
//
//     static constexpr bigint<n> field_char() { return bigint<n>(2); }

//     friend std::ostream& operator<<(std::ostream &out, el:&gf256);
//     friend std::istream& operator>>(std::istream &in, gf256 &el);
//
//     /* little-endian */
//     uint64_t value_[4];
// };

// // #ifdef PROFILE_OP_COUNTS
// i64 gf256::add_cnt = 0;
// i64 gf256::sub_cnt = 0;
// i64 gf256::mul_cnt = 0;
// i64 gf256::sqr_cnt = 0;
// i64 gf256::inv_cnt = 0;
// 

// 
// use crate::algebra::fields::binary::gf256.tcc;

// 
// //#include <cstdio>

// // #define __STDC_FORMAT_MACROS
// //#include <inttypes.h>

// //#include <sodium/randombytes.h>

// use crate::algebra::field_utils::algorithms;
// use crate::algebra::fields::binary::gf256;

// // #ifdef USE_ASM
// //#include <emmintrin.h>
// //#include <immintrin.h>
// //#include <smmintrin.h>
// 



// using std::usize;

// const uint64_t gf256::modulus_;
// gf256 gf256::multiplicative_generator = gf256(2);

// pub fn new()->Self value_{0, 0, 0, 0}
// {
// }

// pub fn new(0:uint64_t value_low)->Self value_{value_low,, 0, 0}
// {
// }

// pub fn new(value_high:uint64_t, value_midh:uint64_t,
//              value_midl:uint64_t, const uint64_t value_low)->Self
//     value_{value_low, value_midl, value_midh, value_high}
// {
// }

// pub fn to_words()->Vec<uint64_t>
// {
//     return Vec<uint64_t>({this->value_[0], this->value_[1], this->value_[2], this->value_[3]});
// }

// bool gf256::from_words(Vec<uint64_t> words)
// {
//     this->value_[0] = words[0];
//     this->value_[1] = words[1];
//     this->value_[2] = words[2];
//     this->value_[3] = words[3];
//     return true;
// }

// gf256& gf256::operator+=(other:&gf256)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// 
//     this->value_[0] ^= other.value_[0];
//     this->value_[1] ^= other.value_[1];
//     this->value_[2] ^= other.value_[2];
//     this->value_[3] ^= other.value_[3];
//     return (*this);
// }

// gf256& gf256::operator-=(other:&gf256)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// 
//     this->value_[0] ^= other.value_[0];
//     this->value_[1] ^= other.value_[1];
//     this->value_[2] ^= other.value_[2];
//     this->value_[3] ^= other.value_[3];
//     return (*this);
// }

// gf256& gf256::operator*=(other:&gf256)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// 
//     /* Does not require *this and other to be different, and therefore
//        also works for squaring, implemented below. */
// // #ifdef USE_ASM
//     /* depending on the manufacturer and generation of a CPU, the PCLMUL
//        instruction might take different amounts of time.
//        empirically, it appears that on recent Intel CPUs, PCLMUL is so fast that
//        a naive multiplicator that uses 16 PCLMULs is faster than anything more
//        complicated (because time spent doing non-PCLMUL operations dominates).
//        on AMD CPUs, however, more complicated multiplicators (e.g. Karatsuba,
//        which uses a total of 9 multiplications) can be faster.

//        thus we use a preprocessor flag to choose between a naive and a Karatsuba
//        multiplicator. */
// // #ifdef ASM_MINIMIZE_CLMULS
//     /* here we implement a Karatsuba-like approach for multiplying 4-limb numbers.

//        given
//          a = a0 + B * a1 + B^2 * a2 + B^3 * a3
//          b = b0 + B * b1 + B^2 * b2 + B^3 * b3
//        we can compute
//          c = a * b = c0 + ... + B^6 * c6
//        (where ai and bi are < B, but ci are < B^2)
//        with 9 multiplications as follows:
//          1. c0 = a0 * b0
//          2. c6 = a3 * b3
//          3. t  = a1 * b1
//          4. u  = a2 * b2
//          5. c1 = (a0 + a1) * (b0 + b1) - c0 - t
//          6. c2 = (a0 + a2) * (b0 + b2) - c0 + t - u
//          7. c5 = (a2 + a3) * (b2 + b3) - c6 - u
//          8. c4 = (a1 + a3) * (b1 + b3) - c6 + u - t
//          9. c3 = (a0 + a1 + a2 + a3) * (b0 + b1 + b2 + b3)
//                  - c0 - c1 - c2 - c4 - c5 - c6 */
//     /* load the two operands and the modulus into 128-bit registers.
//        we load corresponding limbs of both operands into a single register,
//        because it lets us implement Karatsuba with fewer 128-bit xors. */
//     let ab0= _mm_set_epi64x(this->value_[0], other.value_[0]);
//     let ab1= _mm_set_epi64x(this->value_[1], other.value_[1]);
//     let ab2= _mm_set_epi64x(this->value_[2], other.value_[2]);
//     let ab3= _mm_set_epi64x(this->value_[3], other.value_[3]);
//     let modulus= _mm_loadl_epi64((const __m128i*) &(this->modulus_));
//     __m128i c0 = _mm_clmulepi64_si128(ab0, ab0, 0x01); /* multiply low and high halves */
//     __m128i c6 = _mm_clmulepi64_si128(ab3, ab3, 0x01);

//     __m128i t = _mm_clmulepi64_si128(ab1, ab1, 0x01);
//     __m128i u = _mm_clmulepi64_si128(ab2, ab2, 0x01);

//     __m128i xor01 = _mm_xor_si128(ab0, ab1);
//     __m128i c1 = _mm_clmulepi64_si128(xor01, xor01, 0x01);
//     __m128i xor_c0_t = _mm_xor_si128(c0, t);
//     c1 = _mm_xor_si128(c1, xor_c0_t);

//     __m128i xor02 = _mm_xor_si128(ab0, ab2);
//     __m128i c2 = _mm_clmulepi64_si128(xor02, xor02, 0x01);
//     c2 = _mm_xor_si128(_mm_xor_si128(c2, xor_c0_t), u);

//     __m128i xor23 = _mm_xor_si128(ab2, ab3);
//     __m128i c5 = _mm_clmulepi64_si128(xor23, xor23, 0x01);
//     __m128i xor_c6_u = _mm_xor_si128(c6, u);
//     c5 = _mm_xor_si128(c5, xor_c6_u);

//     __m128i xor13 = _mm_xor_si128(ab1, ab3);
//     __m128i c4 = _mm_clmulepi64_si128(xor13, xor13, 0x01);
//     c4 = _mm_xor_si128(_mm_xor_si128(c4, xor_c6_u), t);

//     __m128i xor0123 = _mm_xor_si128(xor02, xor13);
//     __m128i c3 = _mm_clmulepi64_si128(xor0123, xor0123, 0x01);
//     c3 = _mm_xor_si128(_mm_xor_si128(_mm_xor_si128(
//          _mm_xor_si128(_mm_xor_si128(_mm_xor_si128(
//          c3, c0), c1), c2), c4), c5), c6);

// #else // ASM_MINIMIZE_CLMULS
//     /* here we compute the same c as in Karatsuba, but by just naively
//        multiplying all pairs of limbs of the operands and adding together
//        the results that correspond to the same shift. */
//     let a_low= _mm_loadu_si128((const __m128i*) &(this->value_[0]));
//     let a_high= _mm_loadu_si128((const __m128i*) &(this->value_[2]));
//     let b_low= _mm_loadu_si128((const __m128i*) &(other.value_[0]));
//     let b_high= _mm_loadu_si128((const __m128i*) &(other.value_[2]));
//     let modulus= _mm_loadl_epi64((const __m128i*) &(gf256::modulus_));

//     __m128i m00 = _mm_clmulepi64_si128(a_low, b_low, 0x00);
//     __m128i m01 = _mm_clmulepi64_si128(a_low, b_low, 0x10);
//     __m128i m10 = _mm_clmulepi64_si128(a_low, b_low, 0x01);
//     __m128i m11 = _mm_clmulepi64_si128(a_low, b_low, 0x11);
//     __m128i m20 = _mm_clmulepi64_si128(a_high, b_low, 0x00);
//     __m128i m21 = _mm_clmulepi64_si128(a_high, b_low, 0x10);
//     __m128i m30 = _mm_clmulepi64_si128(a_high, b_low, 0x01);
//     __m128i m31 = _mm_clmulepi64_si128(a_high, b_low, 0x11);
//     __m128i m02 = _mm_clmulepi64_si128(a_low, b_high, 0x00);
//     __m128i m03 = _mm_clmulepi64_si128(a_low, b_high, 0x10);
//     __m128i m12 = _mm_clmulepi64_si128(a_low, b_high, 0x01);
//     __m128i m13 = _mm_clmulepi64_si128(a_low, b_high, 0x11);
//     __m128i m22 = _mm_clmulepi64_si128(a_high, b_high, 0x00);
//     __m128i m23 = _mm_clmulepi64_si128(a_high, b_high, 0x10);
//     __m128i m32 = _mm_clmulepi64_si128(a_high, b_high, 0x01);
//     __m128i m33 = _mm_clmulepi64_si128(a_high, b_high, 0x11);

//     __m128i c0 = m00;
//     __m128i c1 = _mm_xor_si128(m01, m10);
//     __m128i c2 = _mm_xor_si128(_mm_xor_si128(m02, m11), m20);
//     __m128i c3 = _mm_xor_si128(_mm_xor_si128(_mm_xor_si128(m03, m12), m21), m30);
//     __m128i c4 = _mm_xor_si128(_mm_xor_si128(m13, m22), m31);
//     __m128i c5 = _mm_xor_si128(m23, m32);
//     __m128i c6 = m33;

// 

//     /* this part is common to both multiplication algorithms:
//        given the 6 overlapping 128-bit limbs such that
//        a * b = c0 + (c1 << 64) + (c2 << 128) + (c3 << 192) + ... (c6 << 384)
//        merge them into non-overlapping 128-bit limbs
//        a * b = d0 + (d1 << 128) + (d2 << 256) + (d3 << 384) */
//     __m128i d0 = _mm_xor_si128(c0, _mm_slli_si128(c1, 8));
//     __m128i d1 = _mm_xor_si128(_mm_xor_si128(c2, _mm_srli_si128(c1, 8)), _mm_slli_si128(c3, 8));
//     __m128i d2 = _mm_xor_si128(_mm_xor_si128(c4, _mm_srli_si128(c3, 8)), _mm_slli_si128(c5, 8));
//     __m128i d3 = _mm_xor_si128(c6, _mm_srli_si128(c5, 8));

//     /* done with the multiplication, time to reduce */
//     /* reduce w.r.t. high half of d3 */
//     __m128i tmp = _mm_clmulepi64_si128(d3, modulus, 0x01);
//     d2 = _mm_xor_si128(d2, _mm_srli_si128(tmp, 8));
//     d1 = _mm_xor_si128(d1, _mm_slli_si128(tmp, 8));

//     /* reduce w.r.t. low half of d3 */
//     tmp = _mm_clmulepi64_si128(d3, modulus, 0x00);
//     d1 = _mm_xor_si128(d1, tmp);

//     /* reduce w.r.t. high half of d2 */
//     tmp = _mm_clmulepi64_si128(d2, modulus, 0x01);
//     d1 = _mm_xor_si128(d1, _mm_srli_si128(tmp, 8));
//     d0 = _mm_xor_si128(d0, _mm_slli_si128(tmp, 8));

//     /* reduce w.r.t. low half of d2 */
//     tmp = _mm_clmulepi64_si128(d2, modulus, 0x00);
//     d0 = _mm_xor_si128(d0, tmp);

//     /* done, now just store everything back into this->value_ */
//     _mm_storeu_si128((__m128i*) &this->value_[0], d0);
//     _mm_storeu_si128((__m128i*) &this->value_[2], d1);

//     return (*this);
// #else
//     /* Slow, but straight-forward */
//     uint64_t shifted[4] = {this->value_[0], this->value_[1],
//                            this->value_[2], this->value_[3]};
//     uint64_t result[4] = {0, 0, 0, 0};

//     for i in 0..4
//     {
//         for j in 0..64
//         {
//             if other.value_[i] & (1ull << j)
//             {
//                 result[0] ^= shifted[0];
//                 result[1] ^= shifted[1];
//                 result[2] ^= shifted[2];
//                 result[3] ^= shifted[3];
//             }

//             bool reduce = (shifted[3] & (1ull << 63));

//             shifted[3] = (shifted[3] << 1) | (shifted[2] >> 63);
//             shifted[2] = (shifted[2] << 1) | (shifted[1] >> 63);
//             shifted[1] = (shifted[1] << 1) | (shifted[0] >> 63);
//             shifted[0] = shifted[0] << 1;

//             if reduce
//             {
//                 shifted[0] ^= this->modulus_;
//             }
//         }

//     }

//     this->value_[0] = result[0];
//     this->value_[1] = result[1];
//     this->value_[2] = result[2];
//     this->value_[3] = result[3];
// 

//     return (*this);
// }

// gf256& gf256::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// gf256& gf256::square()
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
//     this->mul_cnt--;
// 
//     this->operator*=(*this);
//     return *this;
// }

// gf256& gf256::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// gf256 gf256::operator+(other:&gf256) const
// {
//     gf256 result(*this);
//     return (result+=(other));
// }

// gf256 gf256::operator-(other:&gf256) const
// {
//     gf256 result(*this);
//     return (result-=(other));
// }

// gf256 gf256::operator-() const
// {
//     return gf256(*this);
// }

// gf256 gf256::operator*(other:&gf256) const
// {
//     gf256 result(*this);
//     return (result*=(other));
// }

// gf256 gf256::operator^(const u64 pow) const
// {
//     return power<gf256>(*this, pow);
// }

// pub fn squared()->gf256
// {
//     gf256 result(*this);
//     result.square();
//     return result;
// }

// /* calculate el^{-1} as el^{2^{256}-2}. the addition chain below
//    requires 270 mul/sqr operations total. */
// pub fn inverse()->gf256
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
//     this->mul_cnt -= 15;
//     this->sqr_cnt -= 255;
// 
//     assert!(!this->is_zero());
//     gf256 a(*this);

//     gf256 result(0);
//     for i in 0..=7
//     {
//         /* entering the loop a = el^{2^{2^i}-1} */
//         gf256 b = a;
//         for j in 0..(1UL<<i)
//         {
//             b.square();
//         }
//         /* after the loop b = a^{2^i} = el^{2^{2^i}*(2^{2^i}-1)} */
//         a *= b;
//         /* now a = el^{2^{2^{i+1}}-1} */
//         if i == 0
//         {
//             result = b;
//         }
//         else
//         {
//             result *= b;
//         }
//     }
//     /* now result = el^{2^256-2} */
//     return result;
// }

// pub fn sqrt()->gf256
// {
//     return (*this)^bigint<4>("57896044618658097711785492504343953926634992332820282019728792003956564819968"); // 2^255
// }

// pub fn randomize()
// {
//     randombytes_buf(&this->value_, 256/8);
// }

// pub fn clear()
// {
//     this->value_[0] = 0;
//     this->value_[1] = 0;
//     this->value_[2] = 0;
//     this->value_[3] = 0;
// }

// bool gf256::operator==(other:&gf256) const
// {
//     return ((this->value_[0] == other.value_[0]) &&
//             (this->value_[1] == other.value_[1]) &&
//             (this->value_[2] == other.value_[2]) &&
//             (this->value_[3] == other.value_[3]));
// }

// bool gf256::operator!=(other:&gf256) const
// {
//     return !(this->operator==(other));
// }

// pub fn is_zero()->bool
// {
//     return (this->value_[0] == 0) && (this->value_[1] == 0) &&
//            (this->value_[2] == 0) && (this->value_[3] == 0);
// }

// pub fn print() const
// {
//     print!("%016" PRIx64 "%016" PRIx64 "%016" PRIx64 "%016" PRIx64 "\n",
//            this->value_[3], this->value_[2],
//            this->value_[1], this->value_[0]);
// }

// gf256 gf256::random_element()
// {
//     gf256 result;
//     result.randomize();
//     return result;
// }

// gf256 gf256::zero()
// {
//     return gf256(0);
// }

// gf256 gf256::one()
// {
//     return gf256(1);
// }

// std::ostream& operator<<(std::ostream &out, el:&gf256)
// {
//     out << el.value_[0] << " " << el.value_[1] << " " << el.value_[2] << " " << el.value_[3];
//     return out;
// }

// std::istream& operator>>(std::istream &in, gf256 &el)
// {
//     in >> el.value_[0] >> el.value_[1] >> el.value_[2] >> el.value_[3];
//     return in;
// }

// 
// use crate::algebra::field_utils::algorithms;



//
// gf256& gf256::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
// gf256 gf256::operator^(pow:&bigint<m>) const
// {
//     return power<gf256>(*this, pow);
// }

// 
