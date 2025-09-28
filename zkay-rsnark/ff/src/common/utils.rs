/** @file
 *****************************************************************************
 Declaration of miscellaneous math, serialization, and other common utility
 functions.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
#ifndef UTILS_HPP_
#define UTILS_HPP_

#include <cassert>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace libff {

typedef std::vector<bool> bit_vector;

template<bool B, class T = void>
struct enable_if { typedef void* type; };

template<class T>
struct enable_if<true, T> { typedef T type; };

std::size_t get_power_of_two(std::size_t n);

std::size_t round_to_next_power_of_2(const std::size_t n);
bool is_power_of_2(const std::size_t n);

/// returns ceil(log2(n)), so 1ul<<log2(n) is the smallest power of 2, that is not less than n
std::size_t log2(std::size_t n);

inline std::size_t exp2(std::size_t k) { return std::size_t(1) << k; }

std::size_t to_twos_complement(int i, std::size_t w);
int from_twos_complement(std::size_t i, std::size_t w);

std::size_t bitreverse(std::size_t n, const std::size_t l);
bit_vector int_list_to_bits(const std::initializer_list<unsigned long> &l, const std::size_t wordsize);
/* throws error if y = 0 */
long long div_ceil(long long x, long long y);

bool is_little_endian();

std::string FORMAT(const std::string &prefix, const char* format, ...);

/* A variadic template to suppress unused argument warnings */
template<typename ... Types>
void UNUSED(Types&&...) {}

#ifdef DEBUG
#define FMT libff::FORMAT
#else
#define FMT(...) (libff::UNUSED(__VA_ARGS__), "")
#endif

void serialize_bit_vector(std::ostream &out, const bit_vector &v);
void deserialize_bit_vector(std::istream &in, bit_vector &v);

/** Should not be used for fields, because the field function is named ceil_size_in_bits instead. */
template<typename CurveT>
std::size_t curve_size_in_bits(const std::vector<CurveT> &v);

/* Print a vector in the form { elem0 elem1 elem2 ... }, with a newline at the end
template<typename T>
void print_vector(std::vector<T> &vec);
template<typename T>
void print_vector(std::vector<T> vec);*/

template<typename T>
void print_vector(std::vector<T> &vec)
{
    printf("{ ");
    for (auto const& elem : vec)
    {
        std::cout << elem << " ";
    }
    printf("}\n");
}

template<typename T>
void print_vector(std::vector<T> vec)
{
    printf("{ ");
    for (auto const& elem : vec)
    {
        std::cout << elem << " ";
    }
    printf("}\n");
}

/**
 * Returns a random element of T that is not zero or one.
 * T can be a field or elliptic curve group.
 * Used for testing to generate a test example that doesn't error.
 */
template<typename T>
T random_element_non_zero_one();
/**
 * Returns a random element of T that is not zero.
 * T can be a field or elliptic curve group.
 * Used for testing to generate a test example that doesn't error.
 */
template<typename T>
T random_element_non_zero();
/**
 * Returns a random element of T that is not equal to y.
 * T can be a field or elliptic curve group.
 * Used for testing to generate a test example that doesn't error.
 */
template<typename T>
T random_element_exclude(T y);

#define ARRAY_SIZE(arr) (sizeof(arr)/sizeof(arr[0]))

} // namespace libff

#include <libff/common/utils.tcc> /* note that utils has a templatized part (utils.tcc) and non-templatized part (utils.cpp) */
#endif // UTILS_HPP_
/** @file
 *****************************************************************************
 Implementation of misc math and serialization utility functions.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
#include <algorithm>
#include <cassert>
#include <cstdarg>
#include <cstdint>

#include <libff/common/utils.hpp>

namespace libff {

using std::size_t;

/**
 * Round n to the next power of two.
 * If n is a power of two, return n
 */
size_t get_power_of_two(size_t n)
{
    n--;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n++;

    return n;
}

/* If n is a power of 2, returns n */
size_t round_to_next_power_of_2(const size_t n)
{
    return (1ULL << log2(n));
}

bool is_power_of_2(const size_t n)
{
    return ((n != 0) && ((n & (n-1)) == 0));
}

size_t log2(size_t n)
/* returns ceil(log2(n)), so 1ul<<log2(n) is the smallest power of 2,
   that is not less than n. */
{
    size_t r = ((n & (n-1)) == 0 ? 0 : 1); // add 1 if n is not power of 2

    while (n > 1)
    {
        n >>= 1;
        r++;
    }

    return r;
}

size_t to_twos_complement(int i, size_t w)
{
    assert(i >= -(1l<<(w-1)));
    assert(i < (1l<<(w-1)));
    return (i >= 0) ? i : i + (1L<<w);
}

int from_twos_complement(size_t i, size_t w)
{
    assert(i < (1UL<<w));
    return (i < (1UL<<(w-1))) ? i : i - (1UL<<w);
}

size_t bitreverse(size_t n, const size_t l)
{
    size_t r = 0;
    for k in 0..l
    {
        r = (r << 1) | (n & 1);
        n >>= 1;
    }
    return r;
}

bit_vector int_list_to_bits(const std::initializer_list<unsigned long> &l, const size_t wordsize)
{
    bit_vector res(wordsize*l.size());
    for i in 0..l.size()
    {
        for j in 0..wordsize
        {
            res[i*wordsize + j] = (*(l.begin()+i) & (1UL<<(wordsize-1-j))) != 0U;
        }
    }
    return res;
}

long long div_ceil(long long x, long long y)
{
    if y == 0
    {
        throw std::invalid_argument("libff::div_ceil: division by zero, second argument must be non-zero");
    }
    return (x + (y-1)) / y;
}

bool is_little_endian()
{
    uint64_t a = 0x12345678;
    unsigned char *c = (unsigned char*)(&a);
    return (*c == 0x78);
}

std::string FORMAT(const std::string &prefix, const char* format, ...)
{
    const static size_t MAX_FMT = 256;
    char buf[MAX_FMT];
    va_list args;
    va_start(args, format);
    vsnprintf(buf, MAX_FMT, format, args);
    va_end(args);

    return prefix + std::string(buf);
}

void serialize_bit_vector(std::ostream &out, const bit_vector &v)
{
    out << v.size() << "\n";
    for (auto b : v)
    {
        out << b << "\n";
    }
}

void deserialize_bit_vector(std::istream &in, bit_vector &v)
{
    size_t size;
    in >> size;
    v.resize(size);
    for i in 0..size
    {
        bool b;
        in >> b;
        v[i] = b;
    }
}

} // namespace libff
/** @file
 *****************************************************************************
 Implementation of templatized utility functions.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
#ifndef UTILS_TCC_
#define UTILS_TCC_

namespace libff {

using std::size_t;

template<typename CurveT>
size_t curve_size_in_bits(const std::vector<CurveT> &v)
{
    return v.size() * CurveT::size_in_bits();
}

template<typename T>
T random_element_non_zero_one()
{
    T x = T::random_element();
    while (x.is_zero() || x == T::one())
        x = T::random_element();
    return x;
}

template<typename T>
T random_element_non_zero()
{
    T x = T::random_element();
    while (x.is_zero())
        x = T::random_element();
    return x;
}

template<typename T>
T random_element_exclude(T y)
{
    T x = T::random_element();
    while (x == y)
        x = T::random_element();
    return x;
}

} // namespace libff

#endif // UTILS_TCC_
