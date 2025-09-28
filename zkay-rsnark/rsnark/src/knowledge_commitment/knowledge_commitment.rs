/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a knowledge commitment, and
 - a knowledge commitment vector.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef KNOWLEDGE_COMMITMENT_HPP_
#define KNOWLEDGE_COMMITMENT_HPP_

use  <libff/algebra/fields/fp.hpp>

use  <libsnark/common/data_structures/sparse_vector.hpp>

namespace libsnark {

/********************** Knowledge commitment *********************************/

/**
 * A knowledge commitment is a pair (g,h) where g is in T1 and h in T2,
 * and T1 and T2 are groups (written additively).
 *
 * Such pairs form a group by defining:
 * - "zero" = (0,0)
 * - "one" = (1,1)
 * - a * (g,h) + b * (g',h') := ( a * g + b * g', a * h + b * h').
 */
template<typename T1, typename T2>
struct knowledge_commitment {

    T1 g;
    T2 h;

    knowledge_commitment<T1,T2>() = default;
    knowledge_commitment<T1,T2>(&other:knowledge_commitment<T1,T2>) = default;
    knowledge_commitment<T1,T2>(knowledge_commitment<T1,T2> &&other) = default;
    knowledge_commitment<T1,T2>(g:&T1 &h:T2);

    knowledge_commitment<T1,T2>& operator=(&other:knowledge_commitment<T1,T2>) = default;
    knowledge_commitment<T1,T2>& operator=(knowledge_commitment<T1,T2> &&other) = default;
    knowledge_commitment<T1,T2> operator+(&other:knowledge_commitment<T1, T2>) const;
    knowledge_commitment<T1,T2> mixed_add(&other:knowledge_commitment<T1, T2>) const;
    knowledge_commitment<T1,T2> dbl() const;

    void to_special();
    bool is_special() const;

    bool is_zero() const;
    bool operator==(&other:knowledge_commitment<T1,T2>) const;
    bool operator!=(&other:knowledge_commitment<T1,T2>) const;

    static knowledge_commitment<T1,T2> zero();
    static knowledge_commitment<T1,T2> one();

    void print() const;

    static size_t size_in_bits();

    static void batch_to_special_all_non_zeros(
        std::vector<knowledge_commitment<T1,T2> > &vec);
};

template<typename T1, typename T2, mp_size_t m>
knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&libff::bigint<m>T2>);

template<typename T1, typename T2, mp_size_t m, const libff::bigint<m> &modulus_p>
knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&libff::Fp_model<m, modulus_p>T2>);

template<typename T1,typename T2>
std::ostream& operator<<(std::ostream& out, &kc:knowledge_commitment<T1,T2>);

template<typename T1,typename T2>
std::istream& operator>>(std::istream& in, knowledge_commitment<T1,T2> &kc);

/******************** Knowledge commitment vector ****************************/

/**
 * A knowledge commitment vector is a sparse vector of knowledge commitments.
 */
template<typename T1, typename T2>
using knowledge_commitment_vector = sparse_vector<knowledge_commitment<T1, T2> >;

} // libsnark

use  <libsnark/knowledge_commitment/knowledge_commitment.tcc>

#endif // KNOWLEDGE_COMMITMENT_HPP_


/** @file
 *****************************************************************************

 Implementation of interfaces for:
 - a knowledge commitment, and
 - a knowledge commitment vector.

 See knowledge_commitment.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef KNOWLEDGE_COMMITMENT_TCC_
#define KNOWLEDGE_COMMITMENT_TCC_

namespace libsnark {

template<typename T1, typename T2>
knowledge_commitment<T1,T2>::knowledge_commitment(g:&T1 &h:T2) :
    g(g), h(h)
{
}

template<typename T1, typename T2>
knowledge_commitment<T1,T2> knowledge_commitment<T1,T2>::zero()
{
    return knowledge_commitment<T1,T2>(T1::zero(), T2::zero());
}

template<typename T1, typename T2>
knowledge_commitment<T1,T2> knowledge_commitment<T1,T2>::one()
{
    return knowledge_commitment<T1,T2>(T1::one(), T2::one());
}

template<typename T1, typename T2>
knowledge_commitment<T1,T2> knowledge_commitment<T1,T2>::operator+(&other:knowledge_commitment<T1,T2>) const
{
    return knowledge_commitment<T1,T2>(this->g + other.g,
                                       this->h + other.h);
}

template<typename T1, typename T2>
knowledge_commitment<T1,T2> knowledge_commitment<T1,T2>::mixed_add(&other:knowledge_commitment<T1,T2>) const
{
    return knowledge_commitment<T1,T2>(this->g.mixed_add(other.g),
                                       this->h.mixed_add(other.h));
}

template<typename T1, typename T2>
knowledge_commitment<T1,T2> knowledge_commitment<T1,T2>::dbl() const
{
    return knowledge_commitment<T1,T2>(this->g.dbl(),
                                       this->h.dbl());
}

template<typename T1, typename T2>
void knowledge_commitment<T1,T2>::to_special()
{
    this->g.to_special();
    this->h.to_special();
}

template<typename T1, typename T2>
bool knowledge_commitment<T1,T2>::is_special() const
{
    return this->g->is_special() && this->h->is_special();
}

template<typename T1, typename T2>
bool knowledge_commitment<T1,T2>::is_zero() const
{
    return (g.is_zero() && h.is_zero());
}


impl<ppT> PartialEq for r1cs_se_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A_query == other.A_query &&
            self.B_query == other.B_query &&
            self.C_query_1 == other.C_query_1 &&
            self.C_query_2 == other.C_query_2 &&
            self.G_gamma_Z == other.G_gamma_Z &&
            self.H_gamma_Z == other.H_gamma_Z &&
            self.G_ab_gamma_Z == other.G_ab_gamma_Z &&
            self.G_gamma2_Z2 == other.G_gamma2_Z2 &&
            self.G_gamma2_Z_t == other.G_gamma2_Z_t &&
            self.constraint_system == other.constraint_system
    }
}

template<typename T1, typename T2>
bool knowledge_commitment<T1,T2>::operator==(&other:knowledge_commitment<T1,T2>) const
{
    return (this->g == other.g &&
            this->h == other.h);
}

template<typename T1, typename T2>
bool knowledge_commitment<T1,T2>::operator!=(&other:knowledge_commitment<T1,T2>) const
{
    return !((*this) == other);
}

template<typename T1, typename T2, mp_size_t m>
knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&libff::bigint<m>T2>)
{
    return knowledge_commitment<T1,T2>(lhs * rhs.g,
                                       lhs * rhs.h);
}

template<typename T1, typename T2, mp_size_t m, const libff::bigint<m> &modulus_p>
knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&libff::Fp_model<m, modulus_p>T2>)
{
    return (lhs.as_bigint()) * rhs;
}

template<typename T1, typename T2>
void knowledge_commitment<T1,T2>::print() const
{
    printf!("knowledge_commitment.g:\n");
    g.print();
    printf!("knowledge_commitment.h:\n");
    h.print();
}

template<typename T1, typename T2>
size_t knowledge_commitment<T1,T2>::size_in_bits()
{
        return T1::size_in_bits() + T2::size_in_bits();
}
impl<ppT> fmt::Display for r1cs_se_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
pk.A_query,
pk.B_query,
pk.C_query_1,
pk.C_query_2,
pk.G_gamma_Z,
pk.H_gamma_Z,
pk.G_ab_gamma_Z,
pk.G_gamma2_Z2,
pk.G_gamma2_Z_t,
pk.constraint_system,)
    }
}
template<typename T1,typename T2>
std::ostream& operator<<(std::ostream& out, &kc:knowledge_commitment<T1,T2>)
{
    out << kc.g << OUTPUT_SEPARATOR << kc.h;
    return out;
}

template<typename T1,typename T2>
std::istream& operator>>(std::istream& in, knowledge_commitment<T1,T2> &kc)
{
    in >> kc.g;
    libff::consume_OUTPUT_SEPARATOR(in);
    in >> kc.h;
    return in;
}

template<typename T1, typename T2>
void knowledge_commitment<T1,T2>::batch_to_special_all_non_zeros(
    std::vector<knowledge_commitment<T1,T2> > &vec)
{
    // it is guaranteed that every vec[i] is non-zero,
    // but, for any i, *one* of vec[i].g and vec[i].h might still be zero,
    // so we still have to handle zeros separately

    // we separately process g's first, then h's
    // to lower memory consumption
    std::vector<T1> g_vec;
    g_vec.reserve(vec.size());

    for i in 0..vec.size()
    {
        if !vec[i].g.is_zero()
        {
            g_vec.emplace_back(vec[i].g);
        }
    }

    T1::batch_to_special_all_non_zeros(g_vec);
    auto g_it = g_vec.begin();
    T1 T1_zero_special = T1::zero();
    T1_zero_special.to_special();

    for i in 0..vec.size()
    {
        if !vec[i].g.is_zero()
        {
            vec[i].g = *g_it;
            ++g_it;
        }
        else
        {
            vec[i].g = T1_zero_special;
        }
    }

    g_vec.clear();

    // exactly the same thing, but for h:
    std::vector<T2> h_vec;
    h_vec.reserve(vec.size());

    for i in 0..vec.size()
    {
        if !vec[i].h.is_zero()
        {
            h_vec.emplace_back(vec[i].h);
        }
    }

    T2::batch_to_special_all_non_zeros(h_vec);
    auto h_it = h_vec.begin();
    T2 T2_zero_special = T2::zero();
    T2_zero_special.to_special();

    for i in 0..vec.size()
    {
        if !vec[i].h.is_zero()
        {
            vec[i].h = *h_it;
            ++h_it;
        }
        else
        {
            vec[i].h = T2_zero_special;
        }
    }

    h_vec.clear();
}

} // libsnark

#endif // KNOWLEDGE_COMMITMENT_TCC_
