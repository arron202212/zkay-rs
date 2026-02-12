



// #define EDWARDS_G2_HPP_
//#include <iostream>
//#include <vector>

use crate::algebra::curves::curve_utils;
use crate::algebra::curves::edwards::edwards_init;




pub struct edwards_G2 {

// #ifdef PROFILE_OP_COUNTS
    static i64 add_cnt;
    static i64 dbl_cnt;

    static Vec<std::usize> wnaf_window_table;
    static Vec<std::usize> fixed_base_exp_window_table;

    static edwards_G2 G2_zero;
    static edwards_G2 G2_one;
    static bool initialized;

    edwards_Fq3 X, Y, Z;
    edwards_G2();

    edwards_G2(X:edwards_Fq3&, Y:edwards_Fq3&, Z:&edwards_Fq3)->SelfX,Y,Z {};

    static edwards_Fq3 mul_by_a(elt:&edwards_Fq3);
    static edwards_Fq3 mul_by_d(elt:&edwards_Fq3);
    type base_field=edwards_Fq;
    type twist_field=edwards_Fq3;
    type scalar_field=edwards_Fr;

    // using inverted coordinates
    edwards_G2(X:edwards_Fq3&, Y:&edwards_Fq3)->Self X(Y), Y(X), Z(X*Y) {};

    pub fn  print() const;
    pub fn  print_coordinates() const;

    pub fn  to_affine_coordinates();
    pub fn  to_special();
    bool is_special() const;

    bool is_zero() const;

    bool operator==(other:&edwards_G2) const;
    bool operator!=(other:&edwards_G2) const;

    edwards_G2 operator+(other:&edwards_G2) const;
    edwards_G2 operator-() const;
    edwards_G2 operator-(other:&edwards_G2) const;

    edwards_G2 add(other:&edwards_G2) const;
    edwards_G2 mixed_add(other:&edwards_G2) const;
    edwards_G2 dbl() const;
    edwards_G2 mul_by_q() const;

    bool is_well_formed() const;

    static edwards_G2 zero();
    static edwards_G2 one();
    static edwards_G2 random_element();

    static std::usize size_in_bits() { return twist_field::ceil_size_in_bits() + 1; }
    static bigint<base_field::num_limbs> field_char() { return base_field::field_char(); }
    static bigint<scalar_field::num_limbs> order() { return scalar_field::field_char(); }

    friend std::ostream& operator<<(std::ostream &out, g:&edwards_G2);
    friend std::istream& operator>>(std::istream &in, edwards_G2 &g);

    static pub fn  batch_to_special_all_non_zeros(Vec<edwards_G2> &vec);
};


edwards_G2 operator*(lhs:&bigint<m>, rhs:&edwards_G2)
{
    return scalar_mul<edwards_G2, m>(rhs, lhs);
}


edwards_G2 operator*(lhs:&Fp_model<m, modulus_p>, rhs:&edwards_G2)
{
   return scalar_mul<edwards_G2, m>(rhs, lhs.as_bigint());
}






use crate::algebra::curves::edwards::edwards_g2;



// #ifdef PROFILE_OP_COUNTS
i64 edwards_G2::add_cnt = 0;
i64 edwards_G2::dbl_cnt = 0;


Vec<usize> edwards_G2::wnaf_window_table;
Vec<usize> edwards_G2::fixed_base_exp_window_table;

edwards_G2 edwards_G2::G2_zero = {};
edwards_G2 edwards_G2::G2_one = {};
bool edwards_G2::initialized = false;

pub fn new()
{
    if initialized
    {
        this->X = G2_zero.X;
        this->Y = G2_zero.Y;
        this->Z = G2_zero.Z;
    }
}

edwards_Fq3 edwards_G2::mul_by_a(elt:&edwards_Fq3)
{
	// should be
	//  edwards_Fq3(edwards_twist_mul_by_a_c0 * elt.c2, edwards_twist_mul_by_a_c1 * elt.c0, edwards_twist_mul_by_a_c2 * elt.c1)
	// but optimizing the fact that edwards_twist_mul_by_a_c1 = edwards_twist_mul_by_a_c2 = 1
    return edwards_Fq3(edwards_twist_mul_by_a_c0 * elt.c2, elt.c0, elt.c1);
}

edwards_Fq3 edwards_G2::mul_by_d(elt:&edwards_Fq3)
{
	return edwards_Fq3(edwards_twist_mul_by_d_c0 * elt.c2, edwards_twist_mul_by_d_c1 * elt.c0, edwards_twist_mul_by_d_c2 * elt.c1);
}

pub fn print() const
{
    if this->is_zero()
    {
        print!("O\n");
    }
    else
    {
        edwards_G2 copy(*this);
        copy.to_affine_coordinates();
        print!("(%Nd*z^2 + %Nd*z + %Nd , %Nd*z^2 + %Nd*z + %Nd)\n",
                   copy.X.c2.as_bigint().0.0, edwards_Fq::num_limbs,
                   copy.X.c1.as_bigint().0.0, edwards_Fq::num_limbs,
                   copy.X.c0.as_bigint().0.0, edwards_Fq::num_limbs,
                   copy.Y.c2.as_bigint().0.0, edwards_Fq::num_limbs,
                   copy.Y.c1.as_bigint().0.0, edwards_Fq::num_limbs,
                   copy.Y.c0.as_bigint().0.0, edwards_Fq::num_limbs);
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
        print!("(%Nd*z^2 + %Nd*z + %Nd : %Nd*z^2 + %Nd*z + %Nd : %Nd*z^2 + %Nd*z + %Nd)\n",
                   this->X.c2.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->X.c1.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->X.c0.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->Y.c2.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->Y.c1.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->Y.c0.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->Z.c2.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->Z.c1.as_bigint().0.0, edwards_Fq::num_limbs,
                   this->Z.c0.as_bigint().0.0, edwards_Fq::num_limbs);
    }
}

pub fn to_affine_coordinates()
{
    if this->is_zero()
    {
        this->X = edwards_Fq3::zero();
        this->Y = edwards_Fq3::one();
        this->Z = edwards_Fq3::one();
    }
    else
    {
        // go from inverted coordinates to projective coordinates
        edwards_Fq3 tX = this->Y * this->Z;
        edwards_Fq3 tY = this->X * this->Z;
        edwards_Fq3 tZ = this->X * this->Y;
        // go from projective coordinates to affine coordinates
        edwards_Fq3 tZ_inv = tZ.inverse();
        this->X = tX * tZ_inv;
        this->Y = tY * tZ_inv;
        this->Z = edwards_Fq3::one();
    }
}

pub fn to_special()
{
    if this->Z.is_zero()
    {
        return;
    }

// #ifdef DEBUG
    const edwards_G2 copy(*this);


    edwards_Fq3 Z_inv = this->Z.inverse();
    this->X = this->X * Z_inv;
    this->Y = this->Y * Z_inv;
    this->Z = edwards_Fq3::one();

// #ifdef DEBUG
    assert!((*this) == copy);

}

pub fn is_special()->bool
{
    return (this->is_zero() || this->Z == edwards_Fq3::one());
}

pub fn is_zero()->bool
{
    return (this->Y.is_zero() && this->Z.is_zero());
}

bool edwards_G2::operator==(other:&edwards_G2) const
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

    // X1/Z1 = X2/Z2 <=> X1*Z2 = X2*Z1
    if (this->X * other.Z) != (other.X * this->Z)
    {
        return false;
    }

    // Y1/Z1 = Y2/Z2 <=> Y1*Z2 = Y2*Z1
    if (this->Y * other.Z) != (other.Y * this->Z)
    {
        return false;
    }

    return true;
}

bool edwards_G2::operator!=(other:&edwards_G2) const
{
    return !(operator==(other));
}

edwards_G2 edwards_G2::operator+(other:&edwards_G2) const
{
    // handle special cases having to do with O
    if this->is_zero()
    {
        return other;
    }

    if other.is_zero()
    {
        return (*this);
    }

    return this->add(other);
}

edwards_G2 edwards_G2::operator-() const
{
    return edwards_G2(-(this->X), this->Y, this->Z);
}


edwards_G2 edwards_G2::operator-(other:&edwards_G2) const
{
    return (*this) + (-other);
}

pub fn add(other:&edwards_G2)->edwards_G2
{
// #ifdef PROFILE_OP_COUNTS
    this->add_cnt++;

    // NOTE: does not handle O and pts of order 2,4
    // http://www.hyperelliptic.org/EFD/g1p/auto-twisted-inverted.html#addition-add-2008-bbjlp

    let A= (this->Z) * (other.Z);                       // A = Z1*Z2
    let B= edwards_G2::mul_by_d(A.squared());           // B = d*A^2
    let C= (this->X) * (other.X);                       // C = X1*X2
    let D= (this->Y) * (other.Y);                       // D = Y1*Y2
    let E= C*D;                                         // E = C*D
    let H= C - edwards_G2::mul_by_a(D);                 // H = C-a*D
    let I= (this->X+this->Y)*(other.X+other.Y)-C-D;     // I = (X1+Y1)*(X2+Y2)-C-D
    let X3= (E+B)*H;                                    // X3 = (E+B)*H
    let Y3= (E-B)*I;                                    // Y3 = (E-B)*I
    let Z3= A*H*I;                                      // Z3 = A*H*I

    return edwards_G2(X3, Y3, Z3);
}

pub fn mixed_add(other:&edwards_G2)->edwards_G2
{
// #ifdef PROFILE_OP_COUNTS
    this->add_cnt++;

    // handle special cases having to do with O
    if this->is_zero()
    {
        return other;
    }

    if other.is_zero()
    {
        return *this;
    }

// #ifdef DEBUG
    assert!(other.is_special());


    // NOTE: does not handle O and pts of order 2,4
    // http://www.hyperelliptic.org/EFD/g1p/auto-edwards-inverted.html#addition-madd-2007-lb

    let A= this->Z;                                     // A = Z1*Z2
    let B= edwards_G2::mul_by_d(A.squared());           // B = d*A^2
    let C= (this->X) * (other.X);                       // C = X1*X2
    let D= (this->Y) * (other.Y);                       // D = Y1*Y2
    let E= C*D;                                         // E = C*D
    let H= C - edwards_G2::mul_by_a(D);                 // H = C-a*D
    let I= (this->X+this->Y)*(other.X+other.Y)-C-D;     // I = (X1+Y1)*(X2+Y2)-C-D
    let X3= (E+B)*H;                                    // X3 = (E+B)*H
    let Y3= (E-B)*I;                                    // Y3 = (E-B)*I
    let Z3= A*H*I;                                      // Z3 = A*H*I

    return edwards_G2(X3, Y3, Z3);
}

pub fn dbl()->edwards_G2
{
// #ifdef PROFILE_OP_COUNTS
    this->dbl_cnt++;

    if this->is_zero()
    {
        return (*this);
    }
    // NOTE: does not handle O and pts of order 2,4
    // http://www.hyperelliptic.org/EFD/g1p/auto-twisted-inverted.html#doubling-dbl-2008-bbjlp

    let A= (this->X).squared();                      // A = X1^2
    let B= (this->Y).squared();                      // B = Y1^2
    let U= edwards_G2::mul_by_a(B);                  // U = a*B
    let C= A+U;                                      // C = A+U
    let D= A-U;                                      // D = A-U
    let E= (this->X+this->Y).squared()-A-B;          // E = (X1+Y1)^2-A-B
    let X3= C*D;                                     // X3 = C*D
    let dZZ= edwards_G2::mul_by_d(this->Z.squared());
    let Y3= E*(C-dZZ-dZZ);                           // Y3 = E*(C-2*d*Z1^2)
    let Z3= D*E;                                     // Z3 = D*E

    return edwards_G2(X3, Y3, Z3);
}

pub fn mul_by_q()->edwards_G2
{
    return edwards_G2((this->X).Frobenius_map(1),
                      edwards_twist_mul_by_q_Y * (this->Y).Frobenius_map(1),
                      edwards_twist_mul_by_q_Z * (this->Z).Frobenius_map(1));
}

pub fn is_well_formed()->bool
{
    /* Note that point at infinity is the only special case we must check as
       inverted representation does no cover points (0, +-c) and (+-c, 0). */
    if this->is_zero()
    {
        return true;
    }
    /*
        a x^2 + y^2 = 1 + d x^2 y^2

        We are using inverted, so equation we need to check is actually

        a (z/x)^2 + (z/y)^2 = 1 + d z^4 / (x^2 * y^2)
        z^2 (a y^2 + x^2 - dz^2) = x^2 y^2
    */
    edwards_Fq3 X2 = this->X.squared();
    edwards_Fq3 Y2 = this->Y.squared();
    edwards_Fq3 Z2 = this->Z.squared();
    edwards_Fq3 aY2 = edwards_G2::mul_by_a(Y2);
    edwards_Fq3 dZ2 = edwards_G2::mul_by_d(Z2);
    return (Z2 * (aY2 + X2 - dZ2) == X2 * Y2);
}

edwards_G2 edwards_G2::zero()
{
    return G2_zero;
}

edwards_G2 edwards_G2::one()
{
    return G2_one;
}

edwards_G2 edwards_G2::random_element()
{
    return edwards_Fr::random_element().as_bigint() * G2_one;
}

std::ostream& operator<<(std::ostream &out, g:&edwards_G2)
{
    edwards_G2 copy(g);
    copy.to_affine_coordinates();
// #ifdef NO_PT_COMPRESSION
    out << copy.X << OUTPUT_SEPARATOR << copy.Y;
#else
    /* storing LSB of Y */
    out << copy.X << OUTPUT_SEPARATOR << (copy.Y.c0.as_bigint().0.0[0] & 1);

    return out;
}

std::istream& operator>>(std::istream &in, edwards_G2 &g)
{
    edwards_Fq3 tX, tY;

// #ifdef NO_PT_COMPRESSION
    in >> tX;
    consume_OUTPUT_SEPARATOR(in);
    in >> tY;
#else
    /*
      a x^2 + y^2 = 1 + d x^2 y^2
      y = sqrt((1-ax^2)/(1-dx^2))
    */
    unsigned char Y_lsb;
    in >> tX;
    consume_OUTPUT_SEPARATOR(in);

    in.read((char*)&Y_lsb, 1);
    Y_lsb -= '0';

    edwards_Fq3 tX2 = tX.squared();
    let tY2=
        (edwards_Fq3::one() - edwards_G2::mul_by_a(tX2)) *
        (edwards_Fq3::one() - edwards_G2::mul_by_d(tX2)).inverse();
    tY = tY2.sqrt();

    if (tY.c0.as_bigint().0.0[0] & 1) != Y_lsb
    {
        tY = -tY;
    }


    // using inverted coordinates
    g.X = tY;
    g.Y = tX;
    g.Z = tX * tY;

// #ifdef USE_MIXED_ADDITION
    g.to_special();


    return in;
}

pub fn batch_to_special_all_non_zeros(Vec<edwards_G2> &vec)
{
    Vec<edwards_Fq3> Z_vec;
    Z_vec.reserve(vec.len());

    for el in &vec
    {
        Z_vec.emplace_back(el.Z);
    }
    batch_invert<edwards_Fq3>(Z_vec);

    let one= edwards_Fq3::one();

    for i in 0..vec.len()
    {
        vec[i].X = vec[i].X * Z_vec[i];
        vec[i].Y = vec[i].Y * Z_vec[i];
        vec[i].Z = one;
    }
}


