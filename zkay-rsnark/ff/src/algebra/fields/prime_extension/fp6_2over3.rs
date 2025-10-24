/** @file
 *****************************************************************************
 Declaration of arithmetic in the finite field F[(p^3)^2]
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP6_2OVER3_HPP_
// #define FP6_2OVER3_HPP_
use crate::algebra::fields::prime_base::fp;
use crate::algebra::fields::prime_extension::fp2;
use crate::algebra::fields::prime_extension::fp3;

// namespace libff {

/**
 * Arithmetic in the finite field F[(p^3)^2].
 *
 * Let p := modulus. This interface provides arithmetic for the extension field
 * Fp6 = Fp3[Y]/(Y^2-X) where Fp3 = Fp[X]/(X^3-non_residue) and non_residue is in Fp.
 *
 * ASSUMPTION: p = 1 (mod 6)
 */
template<mp_size_t n, const bigint<n>& modulus>
class Fp6_2over3_model;

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream &, const Fp6_2over3_model<n, modulus> &);

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream &, Fp6_2over3_model<n, modulus> &);

template<mp_size_t n, const bigint<n>& modulus>
class Fp6_2over3_model {

    typedef Fp_model<n, modulus> my_Fp;
    typedef Fp2_model<n, modulus> my_Fp2;
    typedef Fp3_model<n, modulus> my_Fp3;
    typedef my_Fp3 my_Fpe;
// #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    static long long add_cnt;
    static long long sub_cnt;
    static long long mul_cnt;
    static long long sqr_cnt;
    static long long inv_cnt;
//#endif

    static bigint<6*n> euler; // (modulus^6-1)/2
    static std::size_t s; // modulus^6 = 2^s * t + 1
    static bigint<6*n> t; // with t odd
    static bigint<6*n> t_minus_1_over_2; // (t-1)/2
    static Fp6_2over3_model<n, modulus> nqr; // a quadratic nonresidue in Fp6
    static Fp6_2over3_model<n, modulus> nqr_to_t; // nqr^t
    static my_Fp non_residue;
    static my_Fp Frobenius_coeffs_c1[6]; // non_residue^((modulus^i-1)/6)   for i=0,1,2,3,4,5

    my_Fp3 c0, c1;
    Fp6_2over3_model() {};
    Fp6_2over3_model(const my_Fp3& c0, const my_Fp3& c1) : c0(c0), c1(c1) {};

    void print() const { print!("c0/c1:\n"); c0.print(); c1.print(); }
    void clear() { c0.clear(); c1.clear(); }
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
    bool operator==(const Fp6_2over3_model &other) const;
    bool operator!=(const Fp6_2over3_model &other) const;

    Fp6_2over3_model& operator+=(const Fp6_2over3_model& other);
    Fp6_2over3_model& operator-=(const Fp6_2over3_model& other);
    Fp6_2over3_model& operator*=(const Fp6_2over3_model& other);
    Fp6_2over3_model& operator^=(const unsigned long pow);
    template<mp_size_t m>
    Fp6_2over3_model& operator^=(const bigint<m> &pow);

    Fp6_2over3_model operator+(const Fp6_2over3_model &other) const;
    Fp6_2over3_model operator-(const Fp6_2over3_model &other) const;
    Fp6_2over3_model operator*(const Fp6_2over3_model &other) const;
    Fp6_2over3_model mul_by_2345(const Fp6_2over3_model &other) const;
    Fp6_2over3_model operator^(const unsigned long pow) const;
    template<mp_size_t m>
    Fp6_2over3_model operator^(const bigint<m> &exponent) const;
    template<mp_size_t m, const bigint<m>& exp_modulus>
    Fp6_2over3_model operator^(const Fp_model<m, exp_modulus> &exponent) const;
    Fp6_2over3_model operator-() const;

    Fp6_2over3_model& square();
    Fp6_2over3_model squared() const;
    Fp6_2over3_model& invert();
    Fp6_2over3_model inverse() const;
    Fp6_2over3_model Frobenius_map(unsigned long power) const;
    Fp6_2over3_model unitary_inverse() const;
    Fp6_2over3_model cyclotomic_squared() const;
    Fp6_2over3_model sqrt() const; // HAS TO BE A SQUARE (else does not terminate)

    static my_Fp3 mul_by_non_residue(const my_Fp3 &elem);

    template<mp_size_t m>
    Fp6_2over3_model cyclotomic_exp(const bigint<m> &exponent) const;

    static std::size_t ceil_size_in_bits() { return 2 * my_Fp3::ceil_size_in_bits(); }
    static std::size_t floor_size_in_bits() { return 2 * my_Fp3::floor_size_in_bits(); }

    static constexpr std::size_t extension_degree() { return 6; }
    static constexpr bigint<n> field_char() { return modulus; }

    static Fp6_2over3_model<n, modulus> zero();
    static Fp6_2over3_model<n, modulus> one();
    static Fp6_2over3_model<n, modulus> random_element();

    friend std::ostream& operator<< <n, modulus>(std::ostream &out, const Fp6_2over3_model<n, modulus> &el);
    friend std::istream& operator>> <n, modulus>(std::istream &in, Fp6_2over3_model<n, modulus> &el);
};

// #ifdef PROFILE_OP_COUNTS
template<mp_size_t n, const bigint<n>& modulus>
long long Fp6_2over3_model<n, modulus>::add_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp6_2over3_model<n, modulus>::sub_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp6_2over3_model<n, modulus>::mul_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp6_2over3_model<n, modulus>::sqr_cnt = 0;

template<mp_size_t n, const bigint<n>& modulus>
long long Fp6_2over3_model<n, modulus>::inv_cnt = 0;
//#endif

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream& out, const std::vector<Fp6_2over3_model<n, modulus> > &v);

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream& in, std::vector<Fp6_2over3_model<n, modulus> > &v);

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp6_2over3_model<n, modulus> &rhs);

template<mp_size_t n, const bigint<n>& modulus>
bigint<6*n> Fp6_2over3_model<n, modulus>::euler;

template<mp_size_t n, const bigint<n>& modulus>
size_t Fp6_2over3_model<n, modulus>::s;

template<mp_size_t n, const bigint<n>& modulus>
bigint<6*n> Fp6_2over3_model<n, modulus>::t;

template<mp_size_t n, const bigint<n>& modulus>
bigint<6*n> Fp6_2over3_model<n, modulus>::t_minus_1_over_2;

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n, modulus>::nqr;

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n, modulus>::nqr_to_t;

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp6_2over3_model<n, modulus>::non_residue;

template<mp_size_t n, const bigint<n>& modulus>
Fp_model<n, modulus> Fp6_2over3_model<n, modulus>::Frobenius_coeffs_c1[6];

// } // namespace libff
use crate::algebra::fields::prime_extension::fp6_2over3.tcc;

//#endif // FP6_2OVER3_HPP_
/** @file
 *****************************************************************************
 Implementation of arithmetic in the finite field F[(p^3)^2].
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP6_2OVER3_TCC_
// #define FP6_2OVER3_TCC_
use crate::algebra::field_utils::field_utils;
use crate::algebra::scalar_multiplication::wnaf;

// namespace libff {

template<mp_size_t n, const bigint<n>& modulus>
Fp3_model<n,modulus> Fp6_2over3_model<n, modulus>::mul_by_non_residue(const Fp3_model<n,modulus> &elem)
{
    return Fp3_model<n, modulus>(non_residue * elem.c2, elem.c0, elem.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n, modulus>::zero()
{
    return Fp6_2over3_model<n,modulus>(my_Fp3::zero(),
                                my_Fp3::zero());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n, modulus>::one()
{
    return Fp6_2over3_model<n,modulus>(my_Fp3::one(),
                                my_Fp3::zero());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::random_element()
{
    Fp6_2over3_model<n, modulus> r;
    r.c0 = my_Fp3::random_element();
    r.c1 = my_Fp3::random_element();

    return r;
}

template<mp_size_t n, const bigint<n>& modulus>
void Fp6_2over3_model<n,modulus>::randomize()
{
    (*this) = Fp6_2over3_model<n, modulus>::random_element();
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp6_2over3_model<n,modulus>::operator==(const Fp6_2over3_model<n,modulus> &other) const
{
    return (this->c0 == other.c0 && this->c1 == other.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp6_2over3_model<n,modulus>::operator!=(const Fp6_2over3_model<n,modulus> &other) const
{
    return !(operator==(other));
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator+(const Fp6_2over3_model<n,modulus> &other) const
{
// #ifdef PROFILE_OP_COUNTS
    this->add_cnt++;
//#endif
    return Fp6_2over3_model<n,modulus>(this->c0 + other.c0,
                                this->c1 + other.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator-(const Fp6_2over3_model<n,modulus> &other) const
{
// #ifdef PROFILE_OP_COUNTS
    this->sub_cnt++;
//#endif
    return Fp6_2over3_model<n,modulus>(this->c0 - other.c0,
                                this->c1 - other.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n, modulus> operator*(const Fp_model<n, modulus> &lhs, const Fp6_2over3_model<n, modulus> &rhs)
{
// #ifdef PROFILE_OP_COUNTS
    rhs.mul_cnt++;
//#endif
    return Fp6_2over3_model<n,modulus>(lhs*rhs.c0,
                                lhs*rhs.c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator*(const Fp6_2over3_model<n,modulus> &other) const
{
// #ifdef PROFILE_OP_COUNTS
    this->mul_cnt++;
//#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */

    const my_Fp3 &B = other.c1, &A = other.c0,
                 &b = this->c1, &a = this->c0;
    const my_Fp3 aA = a*A;
    const my_Fp3 bB = b*B;
    const my_Fp3 beta_bB = Fp6_2over3_model<n,modulus>::mul_by_non_residue(bB);

    return Fp6_2over3_model<n,modulus>(aA + beta_bB,
                                       (a+b)*(A+B) - aA  - bB);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::mul_by_2345(const Fp6_2over3_model<n,modulus> &other) const
{
// #ifdef PROFILE_OP_COUNTS
    this->mul_cnt++;
//#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Karatsuba) */
    assert!(other.c0.c0.is_zero());
    assert!(other.c0.c1.is_zero());

    const my_Fp3 &B = other.c1, &A = other.c0,
                 &b = this->c1, &a = this->c0;
    const my_Fp3 aA = my_Fp3(a.c1 * A.c2 * non_residue, a.c2 * A.c2 * non_residue, a.c0 * A.c2);
    const my_Fp3 bB = b*B;
    const my_Fp3 beta_bB = Fp6_2over3_model<n,modulus>::mul_by_non_residue(bB);

    return Fp6_2over3_model<n,modulus>(aA + beta_bB,
                                       (a+b)*(A+B) - aA  - bB);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator-() const
{
    return Fp6_2over3_model<n,modulus>(-this->c0,
                                -this->c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::operator^(const unsigned long pow) const
{
    return power<Fp6_2over3_model<n, modulus> >(*this, pow);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n,modulus>::operator^(const bigint<m> &exponent) const
{
    return power<Fp6_2over3_model<n, modulus>, m>(*this, exponent);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m, const bigint<m>& exp_modulus>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n,modulus>::operator^(const Fp_model<m, exp_modulus> &exponent) const
{
    return (*this)^(exponent.as_bigint());
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator+=(const Fp6_2over3_model<n,modulus>& other)
{
    (*this) = *this + other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator-=(const Fp6_2over3_model<n,modulus>& other)
{
    (*this) = *this - other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator*=(const Fp6_2over3_model<n,modulus>& other)
{
    (*this) = *this * other;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator^=(const unsigned long pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::operator^=(const bigint<m> &pow)
{
    (*this) = *this ^ pow;
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::squared() const
{
// #ifdef PROFILE_OP_COUNTS
    this->sqr_cnt++;
//#endif
    /* Devegili OhEig Scott Dahab --- Multiplication and Squaring on Pairing-Friendly Fields.pdf; Section 3 (Complex) */
    const my_Fp3 &b = this->c1, &a = this->c0;
    const my_Fp3 ab = a * b;

    return Fp6_2over3_model<n,modulus>((a+b)*(a+Fp6_2over3_model<n,modulus>::mul_by_non_residue(b))-ab-Fp6_2over3_model<n,modulus>::mul_by_non_residue(ab),
                                ab + ab);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::square()
{
    (*this) = squared();
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::inverse() const
{
// #ifdef PROFILE_OP_COUNTS
    this->inv_cnt++;
//#endif
    /* From "High-Speed Software Implementation of the Optimal Ate Pairing over Barreto-Naehrig Curves"; Algorithm 8 */

    const my_Fp3 &b = this->c1, &a = this->c0;
    const my_Fp3 t1 = b.squared();
    const my_Fp3 t0 = a.squared() - Fp6_2over3_model<n,modulus>::mul_by_non_residue(t1);
    const my_Fp3 new_t1 = t0.inverse();

    return Fp6_2over3_model<n,modulus>(a * new_t1,
                                       - (b * new_t1));
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus>& Fp6_2over3_model<n,modulus>::invert()
{
    (*this) = inverse();
    return (*this);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::Frobenius_map(unsigned long power) const
{
    return Fp6_2over3_model<n,modulus>(c0.Frobenius_map(power),
                                       Frobenius_coeffs_c1[power % 6] * c1.Frobenius_map(power));
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::unitary_inverse() const
{
    return Fp6_2over3_model<n,modulus>(this->c0,
                                -this->c1);
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::cyclotomic_squared() const
{
    my_Fp2 a = my_Fp2(c0.c0, c1.c1);
    //my_Fp a_a = c0.c0; // a = Fp2([c0[0],c1[1]])
    //my_Fp a_b = c1.c1;

    my_Fp2 b = my_Fp2(c1.c0, c0.c2);
    //my_Fp b_a = c1.c0; // b = Fp2([c1[0],c0[2]])
    //my_Fp b_b = c0.c2;

    my_Fp2 c = my_Fp2(c0.c1, c1.c2);
    //my_Fp c_a = c0.c1; // c = Fp2([c0[1],c1[2]])
    //my_Fp c_b = c1.c2;

    my_Fp2 asq = a.squared();
    my_Fp2 bsq = b.squared();
    my_Fp2 csq = c.squared();

    // A = vector(3*a^2 - 2*Fp2([vector(a)[0],-vector(a)[1]]))
    //my_Fp A_a = my_Fp(3l) * asq_a - my_Fp(2l) * a_a;
    my_Fp A_a = asq.c0 - a.c0;
    A_a = A_a + A_a + asq.c0;
    //my_Fp A_b = my_Fp(3l) * asq_b + my_Fp(2l) * a_b;
    my_Fp A_b = asq.c1 + a.c1;
    A_b = A_b + A_b + asq.c1;

    // B = vector(3*Fp2([non_residue*c2[1],c2[0]]) + 2*Fp2([vector(b)[0],-vector(b)[1]]))
    //my_Fp B_a = my_Fp(3l) * my_Fp3::non_residue * csq_b + my_Fp(2l) * b_a;
    my_Fp B_tmp = my_Fp3::non_residue * csq.c1;
    my_Fp B_a = B_tmp + b.c0;
    B_a = B_a + B_a + B_tmp;

    //my_Fp B_b = my_Fp(3l) * csq_a - my_Fp(2l) * b_b;
    my_Fp B_b = csq.c0 - b.c1;
    B_b = B_b + B_b + csq.c0;

    // C = vector(3*b^2 - 2*Fp2([vector(c)[0],-vector(c)[1]]))
    //my_Fp C_a = my_Fp(3l) * bsq_a - my_Fp(2l) * c_a;
    my_Fp C_a = bsq.c0 - c.c0;
    C_a = C_a + C_a + bsq.c0;
    // my_Fp C_b = my_Fp(3l) * bsq_b + my_Fp(2l) * c_b;
    my_Fp C_b = bsq.c1 + c.c1;
    C_b = C_b + C_b + bsq.c1;

    // e0 = Fp3([A[0],C[0],B[1]])
    // e1 = Fp3([B[0],A[1],C[1]])
    // fin = Fp6e([e0,e1])
    // return fin

    return Fp6_2over3_model<n, modulus>(my_Fp3(A_a, C_a, B_b),
                                        my_Fp3(B_a, A_b, C_b));
}

template<mp_size_t n, const bigint<n>& modulus>
template<mp_size_t m>
Fp6_2over3_model<n, modulus> Fp6_2over3_model<n,modulus>::cyclotomic_exp(const bigint<m> &exponent) const
{
    Fp6_2over3_model<n,modulus> res = Fp6_2over3_model<n,modulus>::one();
    Fp6_2over3_model<n,modulus> this_inverse = this->unitary_inverse();

    bool found_nonzero = false;
    std::vector<long> NAF = find_wnaf(1, exponent);

    for i in ( 0..=static_cast<long>(NAF.len() - 1)).rev()
    {
        if found_nonzero
        {
            res = res.cyclotomic_squared();
        }

        if NAF[i] != 0
        {
            found_nonzero = true;

            if NAF[i] > 0
            {
                res = res * (*this);
            }
            else
            {
                res = res * this_inverse;
            }
        }
    }

    return res;
}

template<mp_size_t n, const bigint<n>& modulus>
Fp6_2over3_model<n,modulus> Fp6_2over3_model<n,modulus>::sqrt() const
{
    return tonelli_shanks_sqrt(*this);
}

template<mp_size_t n, const bigint<n>& modulus>
std::vector<uint64_t> Fp6_2over3_model<n,modulus>::to_words() const
{
    std::vector<uint64_t> words = c0.to_words();
    std::vector<uint64_t> words1 = c1.to_words();
    words.insert(words.end(), words1.begin(), words1.end());
    return words;
}

template<mp_size_t n, const bigint<n>& modulus>
bool Fp6_2over3_model<n,modulus>::from_words(std::vector<uint64_t> words)
{
    std::vector<uint64_t>::const_iterator vec_start = words.begin();
    std::vector<uint64_t>::const_iterator vec_center = words.begin() + words.len() / 2;
    std::vector<uint64_t>::const_iterator vec_end = words.end();
    std::vector<uint64_t> words0(vec_start, vec_center);
    std::vector<uint64_t> words1(vec_center, vec_end);
    // Fp_model's from_words() takes care of asserts about vector length.
    return c0.from_words(words0) && c1.from_words(words1);
}

template<mp_size_t n, const bigint<n>& modulus>
std::ostream& operator<<(std::ostream &out, const Fp6_2over3_model<n, modulus> &el)
{
    out << el.c0 << OUTPUT_SEPARATOR << el.c1;
    return out;
}

template<mp_size_t n, const bigint<n>& modulus>
std::istream& operator>>(std::istream &in, Fp6_2over3_model<n, modulus> &el)
{
    in >> el.c0 >> el.c1;
    return in;
}

// } // namespace libff
//#endif // FP6_2OVER3_TCC_
