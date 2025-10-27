/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BN128_G2_HPP_
// #define BN128_G2_HPP_
//#include <iostream>
//#include <vector>

#include "depends/ate-pairing/include/bn.h"

use crate::algebra::curves::bn128::bn128_init;
use crate::algebra::curves::curve_utils;

// namespace libff {

pub struct bn128_G2;
std::ostream& operator<<(std::ostream &, const bn128_G2&);
std::istream& operator>>(std::istream &, bn128_G2&);

pub struct bn128_G2 {

    static bn::Fp2 sqrt(el:&bn::Fp2);

// #ifdef PROFILE_OP_COUNTS
    static i64 add_cnt;
    static i64 dbl_cnt;
//#endif
    static Vec<std::usize> wnaf_window_table;
    static Vec<std::usize> fixed_base_exp_window_table;
    static bn128_G2 G2_zero;
    static bn128_G2 G2_one;
    static bool initialized;

    type base_field=bn128_Fq;
    type scalar_field=bn128_Fr;

    // Cofactor
    static let h_bitcount= 256;
    static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
    static bigint<h_limbs> h;

    bn::Fp2 X, Y, Z;
    pub fn Fp2 coord[3]) const { coord[0] = this->X; coord[1] = this->Y; coord[2] = this->Z; };

    bn128_G2();
    bn128_G2(bn::Fp2 coord[3])->Self X(coord[0]), Y(coord[1]), Z(coord[2]) {};

    pub fn  print() const;
    pub fn  print_coordinates() const;

    pub fn  to_affine_coordinates();
    pub fn  to_special();
    bool is_special() const;

    bool is_zero() const;

    bool operator==(other:&bn128_G2) const;
    bool operator!=(other:&bn128_G2) const;

    bn128_G2 operator+(other:&bn128_G2) const;
    bn128_G2 operator-() const;
    bn128_G2 operator-(other:&bn128_G2) const;

    bn128_G2 add(other:&bn128_G2) const;
    bn128_G2 mixed_add(other:&bn128_G2) const;
    bn128_G2 dbl() const;
    bn128_G2 mul_by_cofactor() const;

    bool is_well_formed() const;

    static bn128_G2 zero();
    static bn128_G2 one();
    static bn128_G2 random_element();

    static std::usize size_in_bits() { return 2*base_field::ceil_size_in_bits() + 1; }
    static bigint<base_field::num_limbs> field_char() { return base_field::field_char(); }
    static bigint<scalar_field::num_limbs> order() { return scalar_field::field_char(); }

    friend std::ostream& operator<<(std::ostream &out, g:&bn128_G2);
    friend std::istream& operator>>(std::istream &in, bn128_G2 &g);

    static pub fn  batch_to_special_all_non_zeros(Vec<bn128_G2> &vec);
};


bn128_G2 operator*(lhs:&bigint<m>, rhs:&bn128_G2)
{
    return scalar_mul<bn128_G2, m>(rhs, lhs);
}


bn128_G2 operator*(lhs:&Fp_model<m, modulus_p>, rhs:&bn128_G2)
{
    return scalar_mul<bn128_G2, m>(rhs, lhs.as_bigint());
}

// } // namespace libff
//#endif // BN128_G2_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::algebra::curves::bn128::bn128_g2;
use crate::algebra::curves::bn128::bn_utils;

// namespace libff {

using std::usize;

// #ifdef PROFILE_OP_COUNTS
i64 bn128_G2::add_cnt = 0;
i64 bn128_G2::dbl_cnt = 0;
//#endif

Vec<usize> bn128_G2::wnaf_window_table;
Vec<usize> bn128_G2::fixed_base_exp_window_table;
bn128_G2 bn128_G2::G2_zero = {};
bn128_G2 bn128_G2::G2_one = {};
bool bn128_G2::initialized = false;
bigint<bn128_G2::h_limbs> bn128_G2::h;

bn::Fp2 bn128_G2::sqrt(el:&bn::Fp2)
{
    usize v = bn128_Fq2_s;
    bn::Fp2 z = bn128_Fq2_nqr_to_t;
    bn::Fp2 w = mie::power(el, bn128_Fq2_t_minus_1_over_2);
    bn::Fp2 x = el * w;
    bn::Fp2 b = x * w;

#if DEBUG
    // check if square with Euler's criterion
    bn::Fp2 check = b;
    for i in 0..v-1
    {
        bn::Fp2::square(check, check);
    }

    assert!(check == bn::Fp2(bn::Fp(1), bn::Fp(0)));
//#endif

    // compute square root with Tonelli--Shanks
    // (does not terminate if not a square!)

    while (b != bn::Fp2(1))
    {
        usize m = 0;
        bn::Fp2 b2m = b;
        while (b2m != bn::Fp2(bn::Fp(1), bn::Fp(0)))
        {
            // invariant: b2m = b^(2^m) after entering this loop
            bn::Fp2::square(b2m, b2m);
            m += 1;
        }

        int j = (int) (v - m) - 1;
        w = z;
        while (j > 0)
        {
            bn::Fp2::square(w, w);
            --j;
        } // w = z^2^(v-m-1)

        z = w * w;
        b = b * z;
        x = x * w;
        v = m;
    }

    return x;
}

pub fn new()
{
    if bn128_G2::initialized
    {
        this->X = G2_zero.X;
        this->Y = G2_zero.Y;
        this->Z = G2_zero.Z;
    }
}

pub fn print() const
{
    if this->is_zero()
    {
        print!("O\n");
    }
    else
    {
        bn128_G2 copy(*this);
        copy.to_affine_coordinates();
        std::cout << "(" << copy.X.toString(10) << " : " << copy.Y.toString(10) << " : " << copy.Z.toString(10) << ")\n";
    }
}

pub fn print_coordinates() const
{
    if this->is_zero()
    {
        print!("O\n");
    }
    else
    {
        std::cout << "(" << X.toString(10) << " : " << Y.toString(10) << " : " << Z.toString(10) << ")\n";
    }
}

pub fn to_affine_coordinates()
{
    if this->is_zero()
    {
        X = 0;
        Y = 1;
        Z = 0;
    }
    else
    {
        bn::Fp2 r;
        r = Z;
        r.inverse();
        bn::Fp2::square(Z, r);
        X *= Z;
        r *= Z;
        Y *= r;
        Z = 1;
    }
}

pub fn to_special()
{
    this->to_affine_coordinates();
}

pub fn is_special()->bool
{
    return (this->is_zero() || this->Z == 1);
}

pub fn is_zero()->bool
{
    return Z.isZero();
}

bool bn128_G2::operator==(other:&bn128_G2) const
{
    if this->is_zero()
    {
        return other.is_zero();
    }

    if other.is_zero()
    {
        return false;
    }

    /* now neither is O */

    bn::Fp2 Z1sq, Z2sq, lhs, rhs;
    bn::Fp2::square(Z1sq, this->Z);
    bn::Fp2::square(Z2sq, other.Z);
    bn::Fp2::mul(lhs, Z2sq, this->X);
    bn::Fp2::mul(rhs, Z1sq, other.X);

    if lhs != rhs
    {
        return false;
    }

    bn::Fp2 Z1cubed, Z2cubed;
    bn::Fp2::mul(Z1cubed, Z1sq, this->Z);
    bn::Fp2::mul(Z2cubed, Z2sq, other.Z);
    bn::Fp2::mul(lhs, Z2cubed, this->Y);
    bn::Fp2::mul(rhs, Z1cubed, other.Y);

    return (lhs == rhs);
}

bool bn128_G2::operator!=(other:&bn128_G2) const
{
    return !(operator==(other));
}

bn128_G2 bn128_G2::operator+(other:&bn128_G2) const
{
    // handle special cases having to do with O
    if this->is_zero()
    {
        return other;
    }

    if other.is_zero()
    {
        return *this;
    }

    // no need to handle points of order 2,4
    // (they cannot exist in a prime-order subgroup)

    // handle double case, and then all the rest
    if this->operator==(other)
    {
        return this->dbl();
    }
    return this->add(other);
}

bn128_G2 bn128_G2::operator-() const
{
    bn128_G2 result(*this);
    bn::Fp2::neg(result.Y, result.Y);
    return result;
}

bn128_G2 bn128_G2::operator-(other:&bn128_G2) const
{
    return (*this) + (-other);
}

pub fn add(other:&bn128_G2)->bn128_G2
{
// #ifdef PROFILE_OP_COUNTS
    this->add_cnt++;
//#endif

    bn::Fp2 this_coord[3], other_coord[3], result_coord[3];
    this->fill_coord(this_coord);
    other.fill_coord(other_coord);
    bn::ecop::ECAdd(result_coord, this_coord, other_coord);

    bn128_G2 result(result_coord);
    return result;
}

pub fn mixed_add(other:&bn128_G2)->bn128_G2
{
    if this->is_zero()
    {
        return other;
    }

    if other.is_zero()
    {
        return *this;
    }

    // no need to handle points of order 2,4
    // (they cannot exist in a prime-order subgroup)

// #ifdef DEBUG
    assert!(other.is_special());
//#endif

    // check for doubling case

    // using Jacobian coordinates so:
    // (X1:Y1:Z1) = (X2:Y2:Z2)
    // iff
    // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
    // iff
    // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

    // we know that Z2 = 1

    bn::Fp2 Z1Z1;
    bn::Fp2::square(Z1Z1, this->Z);
    U1:&bn::Fp2 = this->X;
    bn::Fp2 U2;
    bn::Fp2::mul(U2, other.X, Z1Z1);
    bn::Fp2 Z1_cubed;
    bn::Fp2::mul(Z1_cubed, this->Z, Z1Z1);

    S1:&bn::Fp2 = this->Y;
    bn::Fp2 S2;
    bn::Fp2::mul(S2, other.Y, Z1_cubed); // S2 = Y2*Z1*Z1Z1

    if U1 == U2 && S1 == S2
    {
        // dbl case; nothing of above can be reused
        return this->dbl();
    }

// #ifdef PROFILE_OP_COUNTS
    this->add_cnt++;
//#endif

    bn128_G2 result;
    bn::Fp2 H, HH, I, J, r, V, tmp;
    // H = U2-X1
    bn::Fp2::sub(H, U2, this->X);
    // HH = H^2
    bn::Fp2::square(HH, H);
    // I = 4*HH
    bn::Fp2::add(tmp, HH, HH);
    bn::Fp2::add(I, tmp, tmp);
    // J = H*I
    bn::Fp2::mul(J, H, I);
    // r = 2*(S2-Y1)
    bn::Fp2::sub(tmp, S2, this->Y);
    bn::Fp2::add(r, tmp, tmp);
    // V = X1*I
    bn::Fp2::mul(V, this->X, I);
    // X3 = r^2-J-2*V
    bn::Fp2::square(result.X, r);
    bn::Fp2::sub(result.X, result.X, J);
    bn::Fp2::sub(result.X, result.X, V);
    bn::Fp2::sub(result.X, result.X, V);
    // Y3 = r*(V-X3)-2*Y1*J
    bn::Fp2::sub(tmp, V, result.X);
    bn::Fp2::mul(result.Y, r, tmp);
    bn::Fp2::mul(tmp, this->Y, J);
    bn::Fp2::sub(result.Y, result.Y, tmp);
    bn::Fp2::sub(result.Y, result.Y, tmp);
    // Z3 = (Z1+H)^2-Z1Z1-HH
    bn::Fp2::add(tmp, this->Z, H);
    bn::Fp2::square(result.Z, tmp);
    bn::Fp2::sub(result.Z, result.Z, Z1Z1);
    bn::Fp2::sub(result.Z, result.Z, HH);
    return result;
}

pub fn dbl()->bn128_G2
{
// #ifdef PROFILE_OP_COUNTS
    this->dbl_cnt++;
//#endif

    bn::Fp2 this_coord[3], result_coord[3];
    this->fill_coord(this_coord);
    bn::ecop::ECDouble(result_coord, this_coord);

    bn128_G2 result(result_coord);
    return result;
}

pub fn mul_by_cofactor()->bn128_G2
{
    return bn128_G2::h * (*this);
}

pub fn is_well_formed()->bool
{
    if this->is_zero()
    {
        return true;
    }
    /*
        y^2 = x^3 + b

        We are using Jacobian coordinates, so equation we need to check is actually

        (y/z^3)^2 = (x/z^2)^3 + b
        y^2 / z^6 = x^3 / z^6 + b
        y^2 = x^3 + b z^6
    */
    bn::Fp2 X2, Y2, Z2;
    bn::Fp2::square(X2, this->X);
    bn::Fp2::square(Y2, this->Y);
    bn::Fp2::square(Z2, this->Z);

    bn::Fp2 X3, Z3, Z6;
    bn::Fp2::mul(X3, X2, this->X);
    bn::Fp2::mul(Z3, Z2, this->Z);
    bn::Fp2::square(Z6, Z3);

    return (Y2 == X3 + bn128_twist_coeff_b * Z6);
}

bn128_G2 bn128_G2::zero()
{
    return G2_zero;
}

bn128_G2 bn128_G2::one()
{
    return G2_one;
}

bn128_G2 bn128_G2::random_element()
{
    return bn128_Fr::random_element().as_bigint() * G2_one;
}

std::ostream& operator<<(std::ostream &out, g:&bn128_G2)
{
    bn128_G2 gcopy(g);
    gcopy.to_affine_coordinates();

    out << if gcopy.is_zero() {'1'} else{'0'} << OUTPUT_SEPARATOR;

// #ifdef NO_PT_COMPRESSION
    /* no point compression case */
//#ifndef BINARY_OUTPUT
    out << gcopy.X.a_ << OUTPUT_SEPARATOR << gcopy.X.b_ << OUTPUT_SEPARATOR;
    out << gcopy.Y.a_ << OUTPUT_SEPARATOR << gcopy.Y.b_;
#else
    out.write((char*) &gcopy.X.a_, sizeof(gcopy.X.a_));
    out.write((char*) &gcopy.X.b_, sizeof(gcopy.X.b_));
    out.write((char*) &gcopy.Y.a_, sizeof(gcopy.Y.a_));
    out.write((char*) &gcopy.Y.b_, sizeof(gcopy.Y.b_));
//#endif

#else
    /* point compression case */
//#ifndef BINARY_OUTPUT
    out << gcopy.X.a_ << OUTPUT_SEPARATOR << gcopy.X.b_;
#else
    out.write((char*) &gcopy.X.a_, sizeof(gcopy.X.a_));
    out.write((char*) &gcopy.X.b_, sizeof(gcopy.X.b_));
//#endif
    out << OUTPUT_SEPARATOR << if (((unsigned char*)&gcopy.Y.a_)[0] & 1) != 0 {'1'} else{'0'};
//#endif

    return out;
}

std::istream& operator>>(std::istream &in, bn128_G2 &g)
{
    char is_zero;
    in.read((char*)&is_zero, 1); // this reads is_zero;
    is_zero -= '0';
    consume_OUTPUT_SEPARATOR(in);

// #ifdef NO_PT_COMPRESSION
    /* no point compression case */
//#ifndef BINARY_OUTPUT
    in >> g.X.a_;
    consume_OUTPUT_SEPARATOR(in);
    in >> g.X.b_;
    consume_OUTPUT_SEPARATOR(in);
    in >> g.Y.a_;
    consume_OUTPUT_SEPARATOR(in);
    in >> g.Y.b_;
#else
    in.read((char*) &g.X.a_, sizeof(g.X.a_));
    in.read((char*) &g.X.b_, sizeof(g.X.b_));
    in.read((char*) &g.Y.a_, sizeof(g.Y.a_));
    in.read((char*) &g.Y.b_, sizeof(g.Y.b_));
//#endif

#else
    /* point compression case */
    bn::Fp2 tX;
//#ifndef BINARY_OUTPUT
    in >> tX.a_;
    consume_OUTPUT_SEPARATOR(in);
    in >> tX.b_;
#else
    in.read((char*)&tX.a_, sizeof(tX.a_));
    in.read((char*)&tX.b_, sizeof(tX.b_));
//#endif
    consume_OUTPUT_SEPARATOR(in);
    unsigned char Y_lsb;
    in.read((char*)&Y_lsb, 1);
    Y_lsb -= '0';

    // y = +/- sqrt(x^3 + b)
    if is_zero == 0
    {
        g.X = tX;
        bn::Fp2 tX2, tY2;
        bn::Fp2::square(tX2, tX);
        bn::Fp2::mul(tY2, tX2, tX);
        bn::Fp2::add(tY2, tY2, bn128_twist_coeff_b);

        g.Y = bn128_G2::sqrt(tY2);
        if (((unsigned char*)&g.Y.a_)[0] & 1) != Y_lsb
        {
            bn::Fp2::neg(g.Y, g.Y);
        }
    }
//#endif

    /* finalize */
    if is_zero == 0
    {
        g.Z = bn::Fp2(bn::Fp(1), bn::Fp(0));
    }
    else
    {
        g = bn128_G2::zero();
    }

    return in;
}

pub fn batch_to_special_all_non_zeros(Vec<bn128_G2> &vec)
{
    Vec<bn::Fp2> Z_vec;
    Z_vec.reserve(vec.len());

    for el in &vec
    {
        Z_vec.emplace_back(el.Z);
    }
    bn_batch_invert<bn::Fp2>(Z_vec);

    1:bn::Fp2 one =,

    for i in 0..vec.len()
    {
        bn::Fp2 Z2, Z3;
        bn::Fp2::square(Z2, Z_vec[i]);
        bn::Fp2::mul(Z3, Z2, Z_vec[i]);

        bn::Fp2::mul(vec[i].X, vec[i].X, Z2);
        bn::Fp2::mul(vec[i].Y, vec[i].Y, Z3);
        vec[i].Z = one;
    }
}

// } // namespace libff
