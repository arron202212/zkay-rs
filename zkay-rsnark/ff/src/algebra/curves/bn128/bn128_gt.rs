/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BN128_GT_HPP_
// #define BN128_GT_HPP_
//#include <iostream>

#include "depends/ate-pairing/include/bn.h"

use crate::algebra::field_utils::field_utils;
use crate::algebra::fields::prime_base::fp;

// namespace libff {

class bn128_GT;
std::ostream& operator<<(std::ostream &, const bn128_GT&);
std::istream& operator>>(std::istream &, bn128_GT&);

class bn128_GT {
public:
    static bn128_GT GT_one;
    bn::Fp12 elem;

    bn128_GT();
    bool operator==(const bn128_GT &other) const;
    bool operator!=(const bn128_GT &other) const;

    bn128_GT operator*(const bn128_GT &other) const;
    bn128_GT unitary_inverse() const;

    static bn128_GT one();

    void print() { std::cout << this->elem << "\n"; };

    friend std::ostream& operator<<(std::ostream &out, const bn128_GT &g);
    friend std::istream& operator>>(std::istream &in, bn128_GT &g);
};

template<mp_size_t m>
bn128_GT operator^(const bn128_GT &rhs, const bigint<m> &lhs)
{
    return power<bn128_GT, m>(rhs, lhs);
}


template<mp_size_t m, const bigint<m>& modulus_p>
bn128_GT operator^(const bn128_GT &rhs, const Fp_model<m,modulus_p> &lhs)
{
    return power<bn128_GT, m>(rhs, lhs.as_bigint());
}

// } // namespace libff
//#endif // BN128_GT_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::algebra::curves::bn128::bn128_gt;

// namespace libff {

bn128_GT bn128_GT::GT_one;
bn128_GT::bn128_GT()
{
    this->elem.clear();
}

bool bn128_GT::operator==(const bn128_GT &other) const
{
    return (this->elem == other.elem);
}

bool bn128_GT::operator!=(const bn128_GT& other) const
{
    return !(operator==(other));
}

bn128_GT bn128_GT::operator*(const bn128_GT &other) const
{
    bn128_GT result;
    bn::Fp12::mul(result.elem, this->elem, other.elem);
    return result;
}

bn128_GT bn128_GT::unitary_inverse() const
{
    bn128_GT result(*this);
    bn::Fp6::neg(result.elem.b_, result.elem.b_);
    return result;
}

bn128_GT bn128_GT::one()
{
    return GT_one;
}

std::ostream& operator<<(std::ostream &out, const bn128_GT &g)
{
//#ifndef BINARY_OUTPUT
    out << g.elem.a_ << OUTPUT_SEPARATOR << g.elem.b_;
#else
    out.write((char*) &g.elem.a_, sizeof(g.elem.a_));
    out.write((char*) &g.elem.b_, sizeof(g.elem.b_));
//#endif
    return out;
}

std::istream& operator>>(std::istream &in, bn128_GT &g)
{
//#ifndef BINARY_OUTPUT
    in >> g.elem.a_;
    consume_OUTPUT_SEPARATOR(in);
    in >> g.elem.b_;
#else
    in.read((char*) &g.elem.a_, sizeof(g.elem.a_));
    in.read((char*) &g.elem.b_, sizeof(g.elem.b_));
//#endif
    return in;
}
// } // namespace libff
