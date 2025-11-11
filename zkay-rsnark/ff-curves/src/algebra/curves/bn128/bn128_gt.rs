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

use ffec::algebra::field_utils::field_utils;
use ffec::algebra::fields::prime_base::fp;

// namespace libff {

pub struct bn128_GT;
std::ostream& operator<<(std::ostream &, const bn128_GT&);
std::istream& operator>>(std::istream &, bn128_GT&);

pub struct bn128_GT {

    static bn128_GT GT_one;
    bn::Fp12 elem;

    bn128_GT();
    bool operator==(other:&bn128_GT) const;
    bool operator!=(other:&bn128_GT) const;

    bn128_GT operator*(other:&bn128_GT) const;
    bn128_GT unitary_inverse() const;

    static bn128_GT one();

    pub fn cout << this->elem << "\n"; };

    friend std::ostream& operator<<(std::ostream &out, g:&bn128_GT);
    friend std::istream& operator>>(std::istream &in, bn128_GT &g);
};


bn128_GT operator^(rhs:&bn128_GT, lhs:&bigint<m>)
{
    return power<bn128_GT, m>(rhs, lhs);
}



bn128_GT operator^(rhs:&bn128_GT, lhs:&Fp_model<m,modulus_p>)
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
pub fn new()
{
    this->elem.clear();
}

bool bn128_GT::operator==(other:&bn128_GT) const
{
    return (this->elem == other.elem);
}

bool bn128_GT::operator!=(other:&bn128_GT) const
{
    return !(operator==(other));
}

bn128_GT bn128_GT::operator*(other:&bn128_GT) const
{
    bn128_GT result;
    bn::Fp12::mul(result.elem, this->elem, other.elem);
    return result;
}

pub fn unitary_inverse()->bn128_GT
{
    bn128_GT result(*this);
    bn::Fp6::neg(result.elem.b_, result.elem.b_);
    return result;
}

bn128_GT bn128_GT::one()
{
    return GT_one;
}

std::ostream& operator<<(std::ostream &out, g:&bn128_GT)
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
