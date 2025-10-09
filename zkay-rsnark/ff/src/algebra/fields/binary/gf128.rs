/**@file
 *****************************************************************************
 Declaration of GF(2^128) finite field.
 *****************************************************************************
 * @author     This file is part of libff (see AUTHORS), migrated from libiop
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef LIBFF_ALGEBRA_GF128_HPP_
// #define LIBFF_ALGEBRA_GF128_HPP_

//#include <cstddef>
//#include <cstdint>
//#include <vector>
use crate::algebra::field_utils::bigint;

// namespace libff {

/* gf128 implements the field GF(2)/(x^128 + x^7 + x^2 + x + 1).
   Elements are represented internally with two uint64s */
class gf128 {

// #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    static long long add_cnt;
    static long long sub_cnt;
    static long long mul_cnt;
    static long long sqr_cnt;
    static long long inv_cnt;
//#endif
    // x^128 + x^7 + x^2 + x + 1
    static const constexpr uint64_t modulus_ = 0b10000111;
    static const constexpr uint64_t num_bits = 128;

    explicit gf128();
    /* we need a constructor that only initializes the low half of value_ to
       be able to do gf128(0) and gf128(1). */
    explicit gf128(const uint64_t value_low);
    explicit gf128(const uint64_t value_high, const uint64_t value_low);

    gf128& operator+=(const gf128 &other);
    gf128& operator-=(const gf128 &other);
    gf128& operator*=(const gf128 &other);
    gf128& operator^=(const unsigned long pow);
    template<mp_size_t m>
    gf128& operator^=(const bigint<m> &pow);
    
    gf128& square();
    gf128& invert();

    gf128 operator+(const gf128 &other) const;
    gf128 operator-(const gf128 &other) const;
    gf128 operator-() const;
    gf128 operator*(const gf128 &other) const;
    gf128 operator^(const unsigned long pow) const;
    template<mp_size_t m>
    gf128 operator^(const bigint<m> &pow) const;

    gf128 squared() const;
    gf128 inverse() const;
    gf128 sqrt() const;

    void randomize();
    void clear();

    bool operator==(const gf128 &other) const;
    bool operator!=(const gf128 &other) const;

    bool is_zero() const;

    void print() const;
    /**
     * Returns the constituent bits in 64 bit words, in little-endian order.
     * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
     */
    std::vector<uint64_t> to_words() const;
    /**
     * Sets the field element from the given bits in 64 bit words, in little-endian order.
     * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
     * Should always return true since the right-most bits are always valid.
     */
    bool from_words(std::vector<uint64_t> words);

    static gf128 random_element();

    static gf128 zero();
    static gf128 one();
    static gf128 multiplicative_generator; // generator of gf128^*

    static std::size_t ceil_size_in_bits() { return num_bits; }
    static std::size_t floor_size_in_bits() { return num_bits; }
    static constexpr std::size_t extension_degree() { return 128; }
    template<mp_size_t n>
    static constexpr bigint<n> field_char() { return bigint<n>(2); }

    friend std::ostream& operator<<(std::ostream &out, const gf128 &el);
    friend std::istream& operator>>(std::istream &in, gf128 &el);
private:
    /* little-endian */
    uint64_t value_[2];
};

// #ifdef PROFILE_OP_COUNTS
long long gf128::add_cnt = 0;
long long gf128::sub_cnt = 0;
long long gf128::mul_cnt = 0;
long long gf128::sqr_cnt = 0;
long long gf128::inv_cnt = 0;
//#endif

// } // namespace libff
use libff/algebra/fields/binary/gf128.tcc;

//#endif // namespace libff_ALGEBRA_GF128_HPP_
//#include <cstdio>

// #define __STDC_FORMAT_MACROS
//#include <inttypes.h>

//#include <sodium/randombytes.h>

#include "libff/algebra/field_utils/algorithms.hpp"
#include "libff/algebra/fields/binary/gf128.hpp"

// #ifdef USE_ASM
//#include <emmintrin.h>
//#include <immintrin.h>
//#include <smmintrin.h>
//#endif

// namespace libff {

using std::size_t;

const uint64_t gf128::modulus_;
gf128 gf128::multiplicative_generator = gf128(2);

gf128::gf128() : value_{0, 0}
{
}

gf128::gf128(const uint64_t value_low) : value_{value_low, 0}
{
}

gf128::gf128(const uint64_t value_high, const uint64_t value_low) :
    value_{value_low, value_high}
{
}

std::vector<uint64_t> gf128::to_words() const
{
    return std::vector<uint64_t>({this->value_[0], this->value_[1]});
}

bool gf128::from_words(std::vector<uint64_t> words)
{
    this->value_[0] = words[0];
    this->value_[1] = words[1];
    return true;
}

gf128& gf128::operator+=(const gf128 &other)
{
// #ifdef PROFILE_OP_COUNTS
    this->add_cnt++;
//#endif
    this->value_[0] ^= other.value_[0];
    this->value_[1] ^= other.value_[1];
    return (*this);
}

gf128& gf128::operator-=(const gf128 &other)
{
// #ifdef PROFILE_OP_COUNTS
    this->sub_cnt++;
//#endif
    this->value_[0] ^= other.value_[0];
    this->value_[1] ^= other.value_[1];
    return (*this);
}

gf128& gf128::operator*=(const gf128 &other)
{
// #ifdef PROFILE_OP_COUNTS
    this->mul_cnt++;
//#endif
    /* Does not require *this and other to be different, and therefore
       also works for squaring, implemented below. */
// #ifdef USE_ASM
    /* load the two operands and the modulus into 128-bit registers */
    const __m128i a = _mm_loadu_si128((const __m128i*) &(this->value_));
    const __m128i b = _mm_loadu_si128((const __m128i*) &(other.value_));
    const __m128i modulus = _mm_loadl_epi64((const __m128i*) &(libff::gf128::modulus_));

    /* compute the 256-bit result of a * b with the 64x64-bit multiplication
       intrinsic */
    __m128i mul256_high = _mm_clmulepi64_si128(a, b, 0x11); /* high of both */
    __m128i mul256_low = _mm_clmulepi64_si128(a, b, 0x00); /* low of both */

    __m128i mul256_mid1 = _mm_clmulepi64_si128(a, b, 0x01); /* low of a, high of b */
    __m128i mul256_mid2 = _mm_clmulepi64_si128(a, b, 0x10); /* high of a, low of b */

    /* Add the 4 terms together */
    __m128i mul256_mid = _mm_xor_si128(mul256_mid1, mul256_mid2);
    /* lower 64 bits of mid don't intersect with high, and upper 64 bits don't intersect with low */
    mul256_high = _mm_xor_si128(mul256_high, _mm_srli_si128(mul256_mid, 8));
    mul256_low = _mm_xor_si128(mul256_low, _mm_slli_si128(mul256_mid, 8));

    /* done computing mul256_low and mul256_high, time to reduce */

    /* reduce w.r.t. high half of mul256_high */
    __m128i tmp = _mm_clmulepi64_si128(mul256_high, modulus, 0x01);
    mul256_low = _mm_xor_si128(mul256_low, _mm_slli_si128(tmp, 8));
    mul256_high = _mm_xor_si128(mul256_high, _mm_srli_si128(tmp, 8));

    /* reduce w.r.t. low half of mul256_high */
    tmp = _mm_clmulepi64_si128(mul256_high, modulus, 0x00);
    mul256_low = _mm_xor_si128(mul256_low, tmp);

    _mm_storeu_si128((__m128i*) this->value_, mul256_low);

    return (*this);
#else
    /* Slow, but straight-forward */
    uint64_t shifted[2] = {this->value_[0], this->value_[1]};
    uint64_t result[2] = {0, 0};

    for i in 0..2
    {
        for j in 0..64
        {
            if other.value_[i] & (1ull << j)
            {
                result[0] ^= shifted[0];
                result[1] ^= shifted[1];
            }

            if shifted[1] & (1ull << 63)
            {
                shifted[1] = (shifted[1] << 1) | (shifted[0] >> 63);
                shifted[0] = (shifted[0] << 1) ^ this->modulus_;
            } else {
                shifted[1] = (shifted[1] << 1) | (shifted[0] >> 63);
                shifted[0] = shifted[0] << 1;
            }
        }

    }

    this->value_[0] = result[0];
    this->value_[1] = result[1];

    return (*this);
//#endif
}

gf128& gf128::operator^=(const unsigned long pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

gf128& gf128::square()
{
// #ifdef PROFILE_OP_COUNTS
    this->sqr_cnt++;
    this->mul_cnt--;
//#endif
    this->operator*=(*this);
    return *this;
}

gf128& gf128::invert()
{
    (*this) = inverse();
    return (*this);
}

gf128 gf128::operator+(const gf128 &other) const
{
    gf128 result(*this);
    return (result+=(other));
}

gf128 gf128::operator-(const gf128 &other) const
{
    gf128 result(*this);
    return (result-=(other));
}

gf128 gf128::operator-() const
{
    return gf128(this->value_[1], this->value_[0]);
}

gf128 gf128::operator*(const gf128 &other) const
{
    gf128 result(*this);
    return (result*=(other));
}

gf128 gf128::operator^(const unsigned long pow) const
{
    return power<gf128>(*this, pow);
}

gf128 gf128::squared() const
{
    gf128 result(*this);
    result.square();
    return result;
}

/* calculate el^{-1} as el^{2^{128}-2}. the addition chain below
   requires 142 mul/sqr operations total. */
gf128 gf128::inverse() const
{
// #ifdef PROFILE_OP_COUNTS
    this->inv_cnt++;
    this->mul_cnt -= 13;
    this->sqr_cnt -= 127;
//#endif
    assert!(!this->is_zero());
    gf128 a(*this);

    gf128 result(0);
    for i in 0..=6
    {
        /* entering the loop a = el^{2^{2^i}-1} */
        gf128 b = a;
        for j in 0..(1UL<<i)
        {
            b.square();
        }
        /* after the loop b = a^{2^i} = el^{2^{2^i}*(2^{2^i}-1)} */
        a *= b;
        /* now a = el^{2^{2^{i+1}}-1} */

        if i == 0
        {
            result = b;
        }
        else
        {
            result *= b;
        }
    }
    /* now result = el^{2^128-2} */
    return result;
}

gf128 gf128::sqrt() const
{
    return (*this)^bigint<2>("170141183460469231731687303715884105728"); // 2^127
}

void gf128::randomize()
{
    randombytes_buf(&this->value_, 128/8);
}

void gf128::clear()
{
    this->value_[0] = 0;
    this->value_[1] = 0;
}

bool gf128::operator==(const gf128 &other) const
{
    return (this->value_[0] == other.value_[0]) && ((this->value_[1] == other.value_[1]));
}

bool gf128::operator!=(const gf128 &other) const
{
    return !(this->operator==(other));
}

bool gf128::is_zero() const
{
    return (this->value_[0] == 0) && (this->value_[1] == 0);
}

void gf128::print() const
{
    print!("%016" PRIx64 "%016" PRIx64 "\n", this->value_[1], this->value_[0]);
}

gf128 gf128::random_element()
{
    gf128 result;
    result.randomize();
    return result;
}

gf128 gf128::zero()
{
    return gf128(0);
}

gf128 gf128::one()
{
    return gf128(1);
}

std::ostream& operator<<(std::ostream &out, const gf128 &el)
{
    out << el.value_[0] << " " << el.value_[1];
    return out;
}

std::istream& operator>>(std::istream &in, gf128 &el)
{
    in >> el.value_[0] >> el.value_[1];
    return in;
}

// } // namespace libff
#include "libff/algebra/field_utils/algorithms.hpp"

// namespace libff {

template<mp_size_t m>
gf128& gf128::operator^=(const bigint<m> &pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t m>
gf128 gf128::operator^(const bigint<m> &pow) const
{
    return power<gf128>(*this, pow);
}

// } // namespace libff
