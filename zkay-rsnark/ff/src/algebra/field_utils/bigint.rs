/** @file
 *****************************************************************************
 Declaration of bigint wrapper class around GMP's MPZ long integers.

 Notice that this class has no arithmetic operators. This is deliberate. All
 bigints should either be hardcoded or operated on the bit level to ensure
 high performance.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef BIGINT_HPP_
#define BIGINT_HPP_
#include <cstddef>
#include <iostream>

#include <gmp.h>

#include <libff/common/serialization.hpp>

namespace libff {

template<mp_size_t n> class bigint;
template<mp_size_t n> std::ostream& operator<<(std::ostream &, const bigint<n>&);
template<mp_size_t n> std::istream& operator>>(std::istream &, bigint<n>&);

/**
 * Wrapper class around GMP's MPZ long integers. It supports arithmetic operations,
 * serialization and randomization. Serialization is fragile, see common/serialization.hpp.
 */

template<mp_size_t n>
class bigint {

    static const mp_size_t N = n;

    mp_limb_t data[n] = {0};

    bigint() = default;
    bigint(const unsigned long x); /// Initalize from a small integer
    bigint(const char* s); /// Initialize from a string containing an integer in decimal notation
    bigint(const mpz_t r); /// Initialize from MPZ element

    static bigint one();

    void print() const;
    void print_hex() const;
    bool operator==(const bigint<n>& other) const;
    bool operator!=(const bigint<n>& other) const;
    bool operator<(const bigint<n>& other) const;
    void clear();
    bool is_zero() const;
    bool is_even() const;
    std::size_t max_bits() const { return n * GMP_NUMB_BITS; } /// Returns the number of bits representable by this bigint type
    std::size_t num_bits() const; /// Returns the number of bits in this specific bigint value, i.e., position of the most-significant 1

    unsigned long as_ulong() const; /// Return the last limb of the integer
    void to_mpz(mpz_t r) const;
    bool test_bit(const std::size_t bitno) const;

    bigint& randomize();

    friend std::ostream& operator<< <n>(std::ostream &out, const bigint<n> &b);
    friend std::istream& operator>> <n>(std::istream &in, bigint<n> &b);
};

} // namespace libff
#include <libff/algebra/field_utils/bigint.tcc>
#endif


/** @file
 *****************************************************************************
 Implementation of bigint wrapper class around GMP's MPZ long integers.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef BIGINT_TCC_
#define BIGINT_TCC_
#include <cassert>
#include <cstring>
#include <random>

namespace libff {

using std::size_t;

template<mp_size_t n>
bigint<n>::bigint(const unsigned long x) /// Initalize from a small integer
{
    assert(8*sizeof(x) <= GMP_NUMB_BITS);
    this->data[0] = x;
}

template<mp_size_t n>
bigint<n>::bigint(const char* s) /// Initialize from a string containing an integer in decimal notation
{
    size_t l = strlen(s);
    unsigned char* s_copy = new unsigned char[l];

    for i in 0..l
    {
        assert(s[i] >= '0' && s[i] <= '9');
        s_copy[i] = s[i] - '0';
    }

    mp_size_t limbs_written = mpn_set_str(this->data, s_copy, l, 10);
#ifndef NDEBUG
    assert(limbs_written <= n);
#else
    UNUSED(limbs_written);
#endif

    delete[] s_copy;
}

template<mp_size_t n>
bigint<n>::bigint(const mpz_t r) /// Initialize from MPZ element
{
    mpz_t k;
    mpz_init_set(k, r);

    for i in 0..n
    {
        data[i] = mpz_get_ui(k);
        mpz_fdiv_q_2exp(k, k, GMP_NUMB_BITS);
    }

    assert(mpz_sgn(k) == 0);
    mpz_clear(k);
}


template<mp_size_t n>
bigint<n> bigint<n>::one()
{
    bigint<n> one;
    one.data[0] = 1;
    return one;
}

template<mp_size_t n>
void bigint<n>::print() const
{
    gmp_printf("%Nd\n", this->data, n);
}

template<mp_size_t n>
void bigint<n>::print_hex() const
{
    gmp_printf("%Nx\n", this->data, n);
}

template<mp_size_t n>
bool bigint<n>::operator==(const bigint<n>& other) const
{
    return (mpn_cmp(this->data, other.data, n) == 0);
}

template<mp_size_t n>
bool bigint<n>::operator!=(const bigint<n>& other) const
{
    return !(operator==(other));
}

template<mp_size_t n>
bool bigint<n>::operator<(const bigint<n>& other) const
{
    return (mpn_cmp(this->data, other.data, n) < 0);
}

template<mp_size_t n>
void bigint<n>::clear()
{
    mpn_zero(this->data, n);
}

template<mp_size_t n>
bool bigint<n>::is_zero() const
{
    for i in 0..n
    {
        if this->data[i]
        {
            return false;
        }
    }

    return true;
}

template<mp_size_t n>
bool bigint<n>::is_even() const
{
    return (data[0] & 1) == 0;
}

template<mp_size_t n>
size_t bigint<n>::num_bits() const
{
/*
    for (long i = max_bits(); i >= 0; --i)
    {
        if this->test_bit(i)
        {
            return i+1;
        }
    }

    return 0;
*/
    for (long i = n-1; i >= 0; --i)
    {
        mp_limb_t x = this->data[i];
        if x == 0
        {
            continue;
        }
        else
        {
            return ((i+1) * GMP_NUMB_BITS) - __builtin_clzl(x);
        }
    }
    return 0;
}

template<mp_size_t n>
unsigned long bigint<n>::as_ulong() const
{
    return this->data[0];
}

template<mp_size_t n>
void bigint<n>::to_mpz(mpz_t r) const
{
    mpz_set_ui(r, 0);

    for (int i = n-1; i >= 0; --i)
    {
        mpz_mul_2exp(r, r, GMP_NUMB_BITS);
        mpz_add_ui(r, r, this->data[i]);
    }
}

template<mp_size_t n>
bool bigint<n>::test_bit(const std::size_t bitno) const
{
    if bitno >= n * GMP_NUMB_BITS
    {
        return false;
    }
    else
    {
        const std::size_t part = bitno/GMP_NUMB_BITS;
        const std::size_t bit = bitno - (GMP_NUMB_BITS*part);
        const mp_limb_t one = 1;
        return (this->data[part] & (one<<bit)) != 0;
    }
}

template<mp_size_t n>
bigint<n>& bigint<n>::randomize()
{
    static_assert(GMP_NUMB_BITS == sizeof(mp_limb_t) * 8, "Wrong GMP_NUMB_BITS value");
	std::random_device rd;
	constexpr size_t num_random_words = sizeof(mp_limb_t) * n / sizeof(std::random_device::result_type);
	auto random_words = reinterpret_cast<std::random_device::result_type*>(this->data);
	for i in 0..num_random_words
	{
		random_words[i] = rd();
	}

    return (*this);
}


template<mp_size_t n>
std::ostream& operator<<(std::ostream &out, const bigint<n> &b)
{
#ifdef BINARY_OUTPUT
    out.write((char*)b.data, sizeof(b.data[0]) * n);
#else
    mpz_t t;
    mpz_init(t);
    b.to_mpz(t);

    out << t;

    mpz_clear(t);
#endif
    return out;
}

template<mp_size_t n>
std::istream& operator>>(std::istream &in, bigint<n> &b)
{
#ifdef BINARY_OUTPUT
    in.read((char*)b.data, sizeof(b.data[0]) * n);
#else
    std::string s;
    in >> s;

    size_t l = s.size();
    unsigned char* s_copy = new unsigned char[l];

    for i in 0..l
    {
        assert(s[i] >= '0' && s[i] <= '9');
        s_copy[i] = s[i] - '0';
    }

    mp_size_t limbs_written = mpn_set_str(b.data, s_copy, l, 10);
    assert(limbs_written <= n);
    if limbs_written < n
    {
      memset(b.data + limbs_written, 0, (n - limbs_written) * sizeof(mp_limb_t));
    }

    delete[] s_copy;
#endif
    return in;
}

} // namespace libff
#endif // BIGINT_TCC_
