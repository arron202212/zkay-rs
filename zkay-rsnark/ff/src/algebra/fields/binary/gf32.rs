
//  Declaration of GF(2^32) finite field.


// /* gf32 implements the field GF(2)/[x^32 + x^22 + x^2 + x^1 + 1].
//    Elements are represented internally with a single uint32 */
// // pub struct gf32 {
// //
// // // #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
// //     static i64 add_cnt;
// //     static i64 sub_cnt;
// //     static i64 mul_cnt;
// //     static i64 sqr_cnt;
// //     static i64 inv_cnt;
// // 
// //     // x^32 + x^22 + x^2 + x^1 + 1
// //     static 0b10000000000000000000111:constexpr uint64_t modulus_ =,
// //     static 32:constexpr uint64_t num_bits =,

// //     explicit gf32();
// //     explicit gf32(const uint32_t value);

// //     gf32& operator+=(other:&gf32);
// //     gf32& operator-=(other:&gf32);
// //     gf32& operator*=(other:&gf32);
// //     gf32& operator^=(const u64 pow);
// //
// //     gf32& operator^=(pow:&bigint<m>);

// //     gf32& square();
// //     gf32& invert();

// //     gf32 operator+(other:&gf32) const;
// //     gf32 operator-(other:&gf32) const;
// //     gf32 operator-() const;
// //     gf32 operator*(other:&gf32) const;
// //     gf32 operator^(const:u64 pow),
// //
// //     gf32 operator^(pow:&bigint<m>) const;

// //     gf32 squared() const;
// //     gf32 inverse() const;
// //     gf32 sqrt() const;

// //     pub fn  randomize();
// //     pub fn  clear();

// //     bool operator==(other:&gf32) const;
// //     bool operator!=(other:&gf32) const;

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

// //     static gf32 random_element();

// //     static gf32 zero();
// //     static gf32 one();
// //     static gf32 multiplicative_generator; // generator of gf32^*

// //     static std::usize ceil_size_in_bits() { return num_bits; }
// //     static std::usize floor_size_in_bits() { return num_bits; }
// //     static constexpr std::usize extension_degree() { return 32; }
// //
// //     static constexpr bigint<n> field_char() { return bigint<n>(2); }

// //     friend std::ostream& operator<<(std::ostream &out, el:&gf32);
// //     friend std::istream& operator>>(std::istream &in, gf32 &el);
// //
// //     uint32_t value_;
// // };

// // #ifdef PROFILE_OP_COUNTS
// // i64 gf32::add_cnt = 0;
// // i64 gf32::sub_cnt = 0;
// // i64 gf32::mul_cnt = 0;
// // i64 gf32::sqr_cnt = 0;
// // i64 gf32::inv_cnt = 0;
// 

// 
// // use crate::algebra::fields::binary::gf32.tcc;

// 
// //#include <cstdio>

// // #define __STDC_FORMAT_MACROS
// //#include <inttypes.h>

// //#include <sodium/randombytes.h>

// use crate::algebra::field_utils::algorithms;
// // use crate::algebra::fields::binary::gf32;

// // #ifdef USE_ASM
// //#include <emmintrin.h>
// //#include <immintrin.h>
// //#include <smmintrin.h>
// 



// // using std::usize;

// const uint64_t gf32::modulus_;
// gf32 gf32::multiplicative_generator = gf32(2);

// pub fn new()->Self value_(0)
// {
// }

// pub fn new(const uint32_t value)->Self value_(value)
// {
// }

// pub fn to_words()->Vec<uint64_t>
// {
//     return Vec<uint64_t>({uint64_t(this->value_)});
// }

// bool gf32::from_words(Vec<uint64_t> words)
// {
//     this->value_ = uint32_t(words[0]);
//     return true;
// }

// gf32& gf32::square()
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sqr_cnt++;
//     this->mul_cnt--;
// 
//     this->operator*=(*this);
//     return *this;
// }

// gf32& gf32::invert()
// {
//     (*this) = inverse();
//     return (*this);
// }

// pub fn squared()->gf32
// {
//     gf32 result(*this);
//     result.square();
//     return result;
// }

// // repeatedly square pt, num_times. For use in inverse.
// pub fn  square_multi(gf32* pt, int8_t num_times)
// {
//     for i in 0..num_times
//     {
//         (*pt).square();
//     }
// }

// /* calculate el^{-1} as el^{2^{32}-2}. */
// pub fn inverse()->gf32
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->inv_cnt++;
//     this->mul_cnt -= 9;
//     this->sqr_cnt -= 31;
// 
//     assert!(!this->is_zero());
//     gf32 a(*this);

//     gf32 result(0);
//     for i in 0..=4
//     {
//         /* entering the loop a = el^{2^{2^i}-1} */
//         gf32 b = a;
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
//     /* now result = el^{2^32-2} */
//     return result;
// }

// pub fn sqrt()->gf32
// {
//     return (*this)^bigint<1>("2147483648"); // 2^31
// }

// pub fn randomize()
// {
//     randombytes_buf(&this->value_, 32/8);
// }

// pub fn clear()
// {
//     this->value_ = 0;
// }

// pub fn print() const
// {
//     print!("%u\n", this->value_);
// }

// pub fn is_zero()->bool
// {
//     return (this->value_ == 0);
// }

// gf32 gf32::zero()
// {
//     return gf32(0);
// }

// gf32 gf32::one()
// {
//     return gf32(1);
// }

// gf32 gf32::random_element()
// {
//     gf32 result;
//     result.randomize();
//     return result;
// }

// gf32& gf32::operator+=(other:&gf32)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// 
//     this->value_ ^= other.value_;
//     return (*this);
// }

// gf32& gf32::operator-=(other:&gf32)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->sub_cnt++;
// 
//     this->value_ ^= other.value_;
//     return (*this);
// }

// // multiplication over GF(2^k) is carryless multiplication
// gf32& gf32::operator*=(other:&gf32)
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->mul_cnt++;
// 
//     /* Does not require *this and other to be different, and therefore
//        also works for squaring, implemented below. */
//     /* Slow, but straight-forward */
//     uint32_t result = 0;
//     uint32_t shifted = this->value_;

//     for i in 0..32
//     {
//         if (other.value_ & (1ULL << i)) != 0U
//         {
//             result ^= shifted;
//         }
//         if (shifted & (1UL << 31)) != 0U
//         {
//             shifted <<= 1;
//             shifted ^= gf32::modulus_;
//         }
//         else
//         {
//             shifted <<= 1;
//         }
//     }

//     this->value_ = result;

//     return (*this);
// }

// gf32& gf32::operator^=(const u64 pow)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

// gf32 gf32::operator+(other:&gf32) const
// {
//     gf32 result(*this);
//     return (result+=(other));
// }

// gf32 gf32::operator-(other:&gf32) const
// {
//     gf32 result(*this);
//     return (result-=(other));
// }

// gf32 gf32::operator-() const
// {
//     /* additive inverse matches the element itself */
//     return gf32(*this);
// }

// gf32 gf32::operator*(other:&gf32) const
// {
//     gf32 result(*this);
//     return (result*=(other));
// }

// gf32 gf32::operator^(const u64 pow) const
// {
//     return power<gf32>(*this, pow);
// }

// bool gf32::operator==(other:&gf32) const
// {
//     return (this->value_ == other.value_);
// }

// bool gf32::operator!=(other:&gf32) const
// {
//     return !(this->operator==(other));
// }

// std::ostream& operator<<(std::ostream &out, el:&gf32)
// {
//     out << el.value_;
//     return out;
// }

// std::istream& operator>>(std::istream &in, gf32 &el)
// {
//     in >> el.value_;
//     return in;
// }

// 
// use crate::algebra::field_utils::algorithms;



//
// gf32& gf32::operator^=(pow:&bigint<m>)
// {
//     (*this) = *this ^ pow;
//     return (*this);
// }

//
// gf32 gf32::operator^(pow:&bigint<m>) const
// {
//     return power<gf32>(*this, pow);
// }

// 
