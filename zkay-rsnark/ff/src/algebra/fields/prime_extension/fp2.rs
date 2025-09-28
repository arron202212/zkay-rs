/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[p^2].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef FP2_HPP_
#define FP2_HPP_
#include <vector>

#include <libff/algebra/fields/prime_base/fp.hpp>

namespace libff {

template<mp_size_t n, const bigint<n>& modulus>
class Fp2_model;

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream &, const Fp2_model<n, modulus> &);

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream &, Fp2_model<n, modulus> &);

/**
 * Arithmetic in the field F[p^2].
 *
 * Let p := modulus. This interface provides arithmetic for the extension field
 * Fp2 = Fp[U]/(U^2-non_residue), where non_residue is in Fp.
 *
 * ASSUMPTION: p = 1 (mod 6)
 */
template<mp_size_t n, const bigint<n>& modulus>
class Fp2_model {
public:
    typedef Fp_model<n, modulus> my_Fp;
#ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    static long long add_cnt;
    static long long sub_cnt;
    static long long mul_cnt;
    static long long sqr_cnt;
    static long long inv_cnt;
#endif

    static bigint<2*n> euler; // (modulus^2-1)/2
    static std::size_t s;       // modulus^2 = 2^s * t + 1
    static bigint<2*n> t;  // with t odd
    static bigint<2*n> t_minus_1_over_2; // (t-1)/2
    static my_Fp non_residue; // X^4-non_residue irreducible over Fp; used for constructing Fp2 = Fp[X] / (X^2 - non_residue)
    static Fp2_model<n, modulus> nqr; // a quadratic nonresidue in Fp2
    static Fp2_model<n, modulus> nqr_to_t; // nqr^t
    static my_Fp Frobenius_coeffs_c1[2]; // non_residue^((modulus^i-1)/2) for i=0,1

    my_Fp c0, c1;
    Fp2_model() {};
    Fp2_model(const my_Fp& c0, const my_Fp& c1) : c0(c0), c1(c1) {};

    void clear() { c0.clear(); c1.clear(); }
    void print() const { printf("c0/c1:\n"); c0.print(); c1.print(); }
    void randomize();

    /**
     * Returns the constituent bits in 64 bit words, in little-endian order.
     * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
     */
    std::vector<uint64_t> to_words() const;
    /**
     * Sets the field element from the given bits in 64 bit words, in little-endian order.
     * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
     * Returns true when the right-most bits of each element represent a value less than the modulus.
     */
    bool from_words(std::vector<uint64_t> words);

    bool is_zero() const { return c0.is_zero() && c1.is_zero(); }
    bool operator==(const Fp2_model &other) const;
    bool operator!=(const Fp2_model &other) const;

    Fp2_model& operator+=(const Fp2_model& other);
    Fp2_model& operator-=(const Fp2_model& other);
    Fp2_model& operator*=(const Fp2_model& other);
    Fp2_model& operator^=(const unsigned long pow);
    template<mp_size_t m>
    Fp2_model& operator^=(const bigint<m> &pow);

    Fp2_model operator+(const Fp2_model &other) const;
    Fp2_model operator-(const Fp2_model &other) const;
    Fp2_model operator*(const Fp2_model &other) const;
    Fp2_model operator^(const unsigned long pow) const;
    template<mp_size_t m>
    Fp2_model operator^(const bigint<m> &other) const;
    Fp2_model operator-() const;

    Fp2_model& square(); // default is squared_complex
    Fp2_model squared() const; // default is squared_complex
    Fp2_model& invert();
    Fp2_model inverse() const;
    Fp2_model Frobenius_map(unsigned long power) const;
    Fp2_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)
    Fp2_model squared_karatsuba() const;
    Fp2_model squared_complex() const;

    static std::size_t ceil_size_in_bits() { return 2 * my_Fp::ceil_size_in_bits(); }
    static std::size_t floor_size_in_bits() { return 2 * my_Fp::floor_size_in_bits(); }

    static constexpr std::size_t extension_degree() { return 2; }
    static constexpr bigint<n> field_char() { return modulus; }

    static Fp2_model<n, modulus> zero();
    static Fp2_model<n, modulus> one();
    static Fp2_model<n, modulus> random_element();

    friend std::ostream& operator<< <n, modulus>(std::ostream &out, const Fp2_model<n, modulus> &el);
    friend std::istream& operator>> <n, modulus>(std::istream &in, Fp2_model<n, modulus> &el);
};

#ifdef PROFILE_OP_COUNTS
template<mp_size_t n, const bigint<n>& modulus>
long long Fp2_model<n, modulus>::add_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp2_model<n, modulus>::sub_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp2_model<n, modulus>::mul_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp2_model<n, modulus>::sqr_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp2_model<n, modulus>::inv_cnt = 0;
#endif

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream& out, const std::vector<Fp2_model<n, modulus> > &v);

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream& in, std::vector<Fp2_model<n, modulus> > &v);

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp2_model<n, modulus> &rhs);

template<mp_size_t n, const bigint<n>& modulus>
bigint<2*n> Fp2_model<n, modulus>::euler;

template<mp_size_t n, const bigint<n>& modulus>
size_t Fp2_model<n, modulus>::s;

template<mp_size_t n, const bigint<n>& modulus>
bigint<2*n> Fp2_model<n, modulus>::t;

template<mp_size_t n, const bigint<n>& modulus>
bigint<2*n> Fp2_model<n, modulus>::t_minus_1_over_2;

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp2_model<n, modulus>::non_residue;

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n, modulus> Fp2_model<n, modulus>::nqr;

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n, modulus> Fp2_model<n, modulus>::nqr_to_t;

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp2_model<n, modulus>::Frobenius_coeffs_c1[2];

} // namespace libff
#include <libff/algebra/fields/prime_extension/fp2.tcc>

#endif // FP2_HPP_
/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[p^2].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef FP2_TCC_
#define FP2_TCC_

#include <libff/algebra/field_utils/field_utils.hpp>

namespace libff {

using std::size_t;

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::zero()
{
    return Fp2_model<n, modulus>(my_Fp::zero(), my_Fp::zero());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::one()
{
    return Fp2_model<n, modulus>(my_Fp::one(), my_Fp::zero());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::random_element()
{
    Fp2_model<n, modulus> r;
    r.c0 = my_Fp::random_element();
    r.c1 = my_Fp::random_element();

    return r;
}

template<mp_size_t n, const bigint<n>& modulus>
void Fp2_model<n,modulus>::randomize()
{
    (*this) = Fp2_model<n, modulus>::random_element();
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp2_model<n,modulus>::operator==(const Fp2_model<n,modulus> &other) const
{
    return (this->c0 == other.c0 && this->c1 == other.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp2_model<n,modulus>::operator!=(const Fp2_model<n,modulus> &other) const
{
    return !(operator==(other));
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::operator+(const Fp2_model<n,modulus> &other) const
{
#ifdef PROFILE_OP_COUNTS
    this->add_cnt++;
#endif
    return Fp2_model<n,modulus>(this->c0 + other.c0,
                                this->c1 + other.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-(const Fp2_model<n,modulus> &other) const
{
#ifdef PROFILE_OP_COUNTS
    this->sub_cnt++;
#endif
    return Fp2_model<n,modulus>(this->c0 - other.c0,
                                this->c1 - other.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp2_model<n, modulus> &rhs)
{
#ifdef PROFILE_OP_COUNTS
    rhs.mul_cnt++;
#endif
    return Fp2_model<n,modulus>(lhs*rhs.c0,
                                lhs*rhs.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::operator*(const Fp2_model<n,modulus> &other) const
{
#ifdef PROFILE_OP_COUNTS
    this->mul_cnt++;
#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
    const my_Fp
        &A = other.c0, &B = other.c1,
        &a = this->c0, &b = this->c1;
    const my_Fp aA = a * A;
    const my_Fp bB = b * B;

    return Fp2_model<n,modulus>(aA + non_residue * bB,
                                (a + b)*(A+B) - aA - bB);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::operator-() const
{
    return Fp2_model<n,modulus>(-this->c0,
                                -this->c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(const unsigned long pow) const
{
    return power<Fp2_model<n, modulus>>(*this, pow);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp2_model<n,modulus> Fp2_model<n,modulus>::operator^(const bigint<m> &pow) const
{
    return power<Fp2_model<n, modulus>, m>(*this, pow);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator+=(const Fp2_model<n,modulus>& other)
{
    (*this) = *this + other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator-=(const Fp2_model<n,modulus>& other)
{
    (*this) = *this - other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator*=(const Fp2_model<n,modulus>& other)
{
    (*this) = *this * other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(const unsigned long pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::operator^=(const bigint<m> &pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::squared() const
{
    return squared_complex();
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::square()
{
    (*this) = squared();
    return (*this);
}


template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::squared_karatsuba() const
{
#ifdef PROFILE_OP_COUNTS
    this->sqr_cnt++;
#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba squaring) */
    const my_Fp &a = this->c0, &b = this->c1;
    const my_Fp asq = a.squared();
    const my_Fp bsq = b.squared();

    return Fp2_model<n,modulus>(asq + non_residue * bsq,
                                (a + b).squared() - asq - bsq);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::squared_complex() const
{
#ifdef PROFILE_OP_COUNTS
    this->sqr_cnt++;
#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex squaring) */
    const my_Fp &a = this->c0, &b = this->c1;
    const my_Fp ab = a * b;

    return Fp2_model<n,modulus>((a + b) * (a + non_residue * b) - ab - non_residue * ab,
                                ab + ab);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::inverse() const
{
#ifdef PROFILE_OP_COUNTS
    this->inv_cnt++;
#endif
    const my_Fp &a = this->c0, &b = this->c1;

    /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */
    const my_Fp t0 = a.squared();
    const my_Fp t1 = b.squared();
    const my_Fp t2 = t0 - non_residue * t1;
    const my_Fp t3 = t2.inverse();
    const my_Fp c0 = a * t3;
    const my_Fp c1 = - (b * t3);

    return Fp2_model<n,modulus>(c0, c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus>& Fp2_model<n,modulus>::invert()
{
    (*this) = inverse();
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::Frobenius_map(unsigned long power) const
{
    return Fp2_model<n,modulus>(c0,
                                Frobenius_coeffs_c1[power % 2] * c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp2_model<n,modulus> Fp2_model<n,modulus>::sqrt() const
{
    return tonelli_shanks_sqrt(*this);
}

template<mp_size_t n, const bigint<n>& modulus>
std::vector<uint64_t> Fp2_model<n,modulus>::to_words() const
{
    std::vector<uint64_t> words = c0.to_words();
    std::vector<uint64_t> words1 = c1.to_words();
    words.insert(words.end(), words1.begin(), words1.end());
    return words;
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp2_model<n,modulus>::from_words(std::vector<uint64_t> words)
{
    std::vector<uint64_t>::const_iterator vec_start = words.begin();
    std::vector<uint64_t>::const_iterator vec_center = words.begin() + words.size() / 2;
    std::vector<uint64_t>::const_iterator vec_end = words.end();
    std::vector<uint64_t> words0(vec_start, vec_center);
    std::vector<uint64_t> words1(vec_center, vec_end);
    // Fp_model's from_words() takes care of asserts about vector length.
    return c0.from_words(words0) && c1.from_words(words1);
}

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream &out, const Fp2_model<n, modulus> &el)
{
    out << el.c0 << OUTPUT_SEPARATOR << el.c1;
    return out;
}

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream &in, Fp2_model<n, modulus> &el)
{
    in >> el.c0 >> el.c1;
    return in;
}

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream& out, const std::vector<Fp2_model<n, modulus> > &v)
{
    out << v.size() << "\n";
    for (const Fp2_model<n, modulus>& t : v)
    {
        out << t << OUTPUT_NEWLINE;
    }

    return out;
}

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream& in, std::vector<Fp2_model<n, modulus> > &v)
{
    v.clear();

    size_t s;
    in >> s;

    char b;
    in.read(&b, 1);

    v.reserve(s);

    for (size_t i = 0; i < s; ++i)
    {
        Fp2_model<n, modulus> el;
        in >> el;
        v.emplace_back(el);
    }

    return in;
}

} // namespace libff
#endif // FP2_TCC_
