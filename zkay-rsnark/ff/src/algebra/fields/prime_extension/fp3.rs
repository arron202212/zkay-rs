/** @file
 *****************************************************************************
 Declaration of arithmetic in the finite  field F[p^3].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef FP3_HPP_
#define FP3_HPP_
#include <vector>

#include <libff/algebra/fields/prime_base/fp.hpp>

namespace libff {

template<mp_size_t n, const bigint<n>& modulus>
class Fp3_model;

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream &, const Fp3_model<n, modulus> &);

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream &, Fp3_model<n, modulus> &);

/**
 * Arithmetic in the field F[p^3].
 *
 * Let p := modulus. This interface provides arithmetic for the extension field
 * Fp3 = Fp[U]/(U^3-non_residue), where non_residue is in Fp.
 *
 * ASSUMPTION: p = 1 (mod 6)
 */
template<mp_size_t n, const bigint<n>& modulus>
class Fp3_model {
public:
    typedef Fp_model<n, modulus> my_Fp;
#ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    static long long add_cnt;
    static long long sub_cnt;
    static long long mul_cnt;
    static long long sqr_cnt;
    static long long inv_cnt;
#endif

    static bigint<3*n> euler; // (modulus^3-1)/2
    static std::size_t s;       // modulus^3 = 2^s * t + 1
    static bigint<3*n> t;  // with t odd
    static bigint<3*n> t_minus_1_over_2; // (t-1)/2
    static my_Fp non_residue; // X^6-non_residue irreducible over Fp; used for constructing Fp3 = Fp[X] / (X^3 - non_residue)
    static Fp3_model<n, modulus> nqr; // a quadratic nonresidue in Fp3
    static Fp3_model<n, modulus> nqr_to_t; // nqr^t
    static my_Fp Frobenius_coeffs_c1[3]; // non_residue^((modulus^i-1)/3)   for i=0,1,2
    static my_Fp Frobenius_coeffs_c2[3]; // non_residue^((2*modulus^i-2)/3) for i=0,1,2

    my_Fp c0, c1, c2;
    Fp3_model() {};
    Fp3_model(const my_Fp& c0, const my_Fp& c1, const my_Fp& c2) : c0(c0), c1(c1), c2(c2) {};

    void clear() { c0.clear(); c1.clear(); c2.clear(); }
    void print() const { printf("c0/c1/c2:\n"); c0.print(); c1.print(); c2.print(); }
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

    bool is_zero() const { return c0.is_zero() && c1.is_zero() && c2.is_zero(); }
    bool operator==(const Fp3_model &other) const;
    bool operator!=(const Fp3_model &other) const;

    Fp3_model& operator+=(const Fp3_model& other);
    Fp3_model& operator-=(const Fp3_model& other);
    Fp3_model& operator*=(const Fp3_model& other);
    Fp3_model& operator^=(const unsigned long pow);
    template<mp_size_t m>
    Fp3_model& operator^=(const bigint<m> &pow);

    Fp3_model operator+(const Fp3_model &other) const;
    Fp3_model operator-(const Fp3_model &other) const;
    Fp3_model operator*(const Fp3_model &other) const;
    Fp3_model operator^(const unsigned long pow) const;
    template<mp_size_t m>
    Fp3_model operator^(const bigint<m> &other) const;
    Fp3_model operator-() const;

    Fp3_model& square();
    Fp3_model squared() const;
    Fp3_model& invert();
    Fp3_model inverse() const;
    Fp3_model Frobenius_map(unsigned long power) const;
    Fp3_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

    static std::size_t ceil_size_in_bits() { return 3 * my_Fp::ceil_size_in_bits(); }
    static std::size_t floor_size_in_bits() { return 3 * my_Fp::floor_size_in_bits(); }

    static constexpr std::size_t extension_degree() { return 3; }
    static constexpr bigint<n> field_char() { return modulus; }

    static Fp3_model<n, modulus> zero();
    static Fp3_model<n, modulus> one();
    static Fp3_model<n, modulus> random_element();

    friend std::ostream& operator<< <n, modulus>(std::ostream &out, const Fp3_model<n, modulus> &el);
    friend std::istream& operator>> <n, modulus>(std::istream &in, Fp3_model<n, modulus> &el);
};

#ifdef PROFILE_OP_COUNTS
template<mp_size_t n, const bigint<n>& modulus>
long long Fp3_model<n, modulus>::add_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp3_model<n, modulus>::sub_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp3_model<n, modulus>::mul_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp3_model<n, modulus>::sqr_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp3_model<n, modulus>::inv_cnt = 0;
#endif

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream& out, const std::vector<Fp3_model<n, modulus> > &v);

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream& in, std::vector<Fp3_model<n, modulus> > &v);

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp3_model<n, modulus> &rhs);

template<mp_size_t n, const bigint<n>& modulus>
bigint<3*n> Fp3_model<n, modulus>::euler;

template<mp_size_t n, const bigint<n>& modulus>
size_t Fp3_model<n, modulus>::s;

template<mp_size_t n, const bigint<n>& modulus>
bigint<3*n> Fp3_model<n, modulus>::t;

template<mp_size_t n, const bigint<n>& modulus>
bigint<3*n> Fp3_model<n, modulus>::t_minus_1_over_2;

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp3_model<n, modulus>::non_residue;

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n, modulus> Fp3_model<n, modulus>::nqr;

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n, modulus> Fp3_model<n, modulus>::nqr_to_t;

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp3_model<n, modulus>::Frobenius_coeffs_c1[3];

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp3_model<n, modulus>::Frobenius_coeffs_c2[3];

} // namespace libff
#include <libff/algebra/fields/prime_extension/fp3.tcc>

#endif // FP3_HPP_
/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[p^3].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef FP3_TCC_
#define FP3_TCC_

#include <libff/algebra/field_utils/field_utils.hpp>

namespace libff {

using std::size_t;

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::zero()
{
    return Fp3_model<n, modulus>(my_Fp::zero(), my_Fp::zero(), my_Fp::zero());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::one()
{
    return Fp3_model<n, modulus>(my_Fp::one(), my_Fp::zero(), my_Fp::zero());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::random_element()
{
    Fp3_model<n, modulus> r;
    r.c0 = my_Fp::random_element();
    r.c1 = my_Fp::random_element();
    r.c2 = my_Fp::random_element();

    return r;
}

template<mp_size_t n, const bigint<n>& modulus>
void Fp3_model<n,modulus>::randomize()
{
    (*this) = Fp3_model<n, modulus>::random_element();
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp3_model<n,modulus>::operator==(const Fp3_model<n,modulus> &other) const
{
    return (this->c0 == other.c0 && this->c1 == other.c1 && this->c2 == other.c2);
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp3_model<n,modulus>::operator!=(const Fp3_model<n,modulus> &other) const
{
    return !(operator==(other));
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::operator+(const Fp3_model<n,modulus> &other) const
{
#ifdef PROFILE_OP_COUNTS
    this->add_cnt++;
#endif
    return Fp3_model<n,modulus>(this->c0 + other.c0,
                                this->c1 + other.c1,
                                this->c2 + other.c2);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::operator-(const Fp3_model<n,modulus> &other) const
{
#ifdef PROFILE_OP_COUNTS
    this->sub_cnt++;
#endif
    return Fp3_model<n,modulus>(this->c0 - other.c0,
                                this->c1 - other.c1,
                                this->c2 - other.c2);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp3_model<n, modulus> &rhs)
{
#ifdef PROFILE_OP_COUNTS
    rhs.mul_cnt++;
#endif
    return Fp3_model<n,modulus>(lhs*rhs.c0,
                                lhs*rhs.c1,
                                lhs*rhs.c2);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::operator*(const Fp3_model<n,modulus> &other) const
{
#ifdef PROFILE_OP_COUNTS
    this->mul_cnt++;
#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (Karatsuba) */
    const my_Fp
        &A = other.c0, &B = other.c1, &C = other.c2,
        &a = this->c0, &b = this->c1, &c = this->c2;
    const my_Fp aA = a*A;
    const my_Fp bB = b*B;
    const my_Fp cC = c*C;

    return Fp3_model<n,modulus>(aA + non_residue*((b+c)*(B+C)-bB-cC),
                                (a+b)*(A+B)-aA-bB+non_residue*cC,
                                (a+c)*(A+C)-aA+bB-cC);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::operator-() const
{
    return Fp3_model<n,modulus>(-this->c0,
                                -this->c1,
                                -this->c2);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::operator^(const unsigned long pow) const
{
    return power<Fp3_model<n, modulus> >(*this, pow);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp3_model<n,modulus> Fp3_model<n,modulus>::operator^(const bigint<m> &pow) const
{
    return power<Fp3_model<n, modulus> >(*this, pow);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator+=(const Fp3_model<n,modulus>& other)
{
    (*this) = *this + other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator-=(const Fp3_model<n,modulus>& other)
{
    (*this) = *this - other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator*=(const Fp3_model<n,modulus>& other)
{
    (*this) = *this * other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator^=(const unsigned long pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::operator^=(const bigint<m> &pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::squared() const
{
#ifdef PROFILE_OP_COUNTS
    this->sqr_cnt++;
#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 4 (CH-SQR2) */
    const my_Fp
        &a = this->c0, &b = this->c1, &c = this->c2;
    const my_Fp s0 = a.squared();
    const my_Fp ab = a*b;
    const my_Fp s1 = ab + ab;
    const my_Fp s2 = (a - b + c).squared();
    const my_Fp bc = b*c;
    const my_Fp s3 = bc + bc;
    const my_Fp s4 = c.squared();

    return Fp3_model<n,modulus>(s0 + non_residue * s3,
                                s1 + non_residue * s4,
                                s1 + s2 + s3 - s0 - s4);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::square()
{
    (*this) = squared();
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::inverse() const
{
#ifdef PROFILE_OP_COUNTS
    this->inv_cnt++;
#endif
    const my_Fp
        &a = this->c0, &b = this->c1, &c = this->c2;

    /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 17 */
    const my_Fp t0 = a.squared();
    const my_Fp t1 = b.squared();
    const my_Fp t2 = c.squared();
    const my_Fp t3 = a*b;
    const my_Fp t4 = a*c;
    const my_Fp t5 = b*c;
    const my_Fp c0 = t0 - non_residue * t5;
    const my_Fp c1 = non_residue * t2 - t3;
    const my_Fp c2 = t1 - t4; // typo in paper referenced above. should be "-" as per Scott, but is "*"
    const my_Fp t6 = (a * c0 + non_residue * (c * c1 + b * c2)).inverse();
    return Fp3_model<n,modulus>(t6 * c0, t6 * c1, t6 * c2);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus>& Fp3_model<n,modulus>::invert()
{
    (*this) = inverse();
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::Frobenius_map(unsigned long power) const
{
    return Fp3_model<n,modulus>(c0,
                                Frobenius_coeffs_c1[power % 3] * c1,
                                Frobenius_coeffs_c2[power % 3] * c2);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp3_model<n,modulus>::sqrt() const
{
    return tonelli_shanks_sqrt(*this);
}

template<mp_size_t n, const bigint<n>& modulus>
std::vector<uint64_t> Fp3_model<n,modulus>::to_words() const
{
    std::vector<uint64_t> words = c0.to_words();
    std::vector<uint64_t> words1 = c1.to_words();
    std::vector<uint64_t> words2 = c2.to_words();
    words.insert(words.end(), words1.begin(), words1.end());
    words.insert(words.end(), words2.begin(), words2.end());
    return words;
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp3_model<n,modulus>::from_words(std::vector<uint64_t> words)
{
    std::vector<uint64_t>::const_iterator vec_start = words.begin();
    std::vector<uint64_t>::const_iterator vec_center1 = words.begin() + words.size() / 3;
    std::vector<uint64_t>::const_iterator vec_center2 = words.begin() + 2 * words.size() / 3;
    std::vector<uint64_t>::const_iterator vec_end = words.end();
    std::vector<uint64_t> words0(vec_start, vec_center1);
    std::vector<uint64_t> words1(vec_center1, vec_center2);
    std::vector<uint64_t> words2(vec_center2, vec_end);
    // Fp_model's from_words() takes care of asserts about vector length.
    return c0.from_words(words0) && c1.from_words(words1) && c2.from_words(words2);
}

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream &out, const Fp3_model<n, modulus> &el)
{
    out << el.c0 << OUTPUT_SEPARATOR << el.c1 << OUTPUT_SEPARATOR << el.c2;
    return out;
}

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream &in, Fp3_model<n, modulus> &el)
{
    in >> el.c0 >> el.c1 >> el.c2;
    return in;
}

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream& out, const std::vector<Fp3_model<n, modulus> > &v)
{
    out << v.size() << "\n";
    for (const Fp3_model<n, modulus>& t : v)
    {
        out << t << OUTPUT_NEWLINE;
    }

    return out;
}

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream& in, std::vector<Fp3_model<n, modulus> > &v)
{
    v.clear();

    size_t s;
    in >> s;

    char b;
    in.read(&b, 1);

    v.reserve(s);

    for (size_t i = 0; i < s; ++i)
    {
        Fp3_model<n, modulus> el;
        in >> el;
        v.emplace_back(el);
    }

    return in;
}

} // namespace libff
#endif // FP3_TCC_
