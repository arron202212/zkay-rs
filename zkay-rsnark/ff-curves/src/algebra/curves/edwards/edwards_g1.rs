/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef EDWARDS_G1_HPP_
// #define EDWARDS_G1_HPP_
//#include <vector>

use crate::algebra::curves::curve_utils;
use crate::algebra::curves::edwards::edwards_init;

// namespace libff {



pub struct  edwards_G1 {

// #ifdef PROFILE_OP_COUNTS
add_cnt:     i64,
dbl_cnt:     i64,
//#endif
wnaf_window_table:     Vec<std::usize>,
fixed_base_exp_window_table:     Vec<std::usize>,
G1_zero:     edwards_G1,
G1_one:     edwards_G1,
initialized:     bool,

     X:edwards_Fq, Y:edwards_Fq, Z:edwards_Fq,

}
 type base_field=edwards_Fq;
    type scalar_field=edwards_Fr;
impl edwards_G1 {
    pub fn new(X:edwards_Fq, Y:edwards_Fq, Z:edwards_Fq) ->Self  {Self{X, Y, Z}}


   
    // using inverted coordinates
    pub fn new2(X:edwards_Fq, Y:edwards_Fq) ->Self  {let Z=X*Y;Self{X, Y, Z}}

    pub fn  size_in_bits()->usize { return edwards_Fq::ceil_size_in_bits() + 1; }
    pub fn   field_char()->bigint<base_field::num_limbs> { return base_field::field_char(); }
    pub fn   order()->bigint<scalar_field::num_limbs> { return scalar_field::field_char(); }


}

// 
// edwards_G1 operator*(lhs:&bigint<m>, rhs:&edwards_G1)
// {
//     return scalar_mul<edwards_G1, m>(rhs, lhs);
// }

// 
// edwards_G1 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&edwards_G1)
// {
//     return scalar_mul<edwards_G1, m>(rhs, lhs.as_bigint());
// }

// std::ostream& operator<<(std::ostream& out, v:&Vec<edwards_G1>);
// std::istream& operator>>(std::istream& in, Vec<edwards_G1> &v);

// } // namespace libff
//#endif // EDWARDS_G1_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use crate::algebra::curves::edwards::edwards_g1;

// namespace libff {

// using std::usize;

// #ifdef PROFILE_OP_COUNTS
// i64 edwards_G1::add_cnt = 0;
// i64 edwards_G1::dbl_cnt = 0;
//#endif

// Vec<usize> edwards_G1::wnaf_window_table;
// Vec<usize> edwards_G1::fixed_base_exp_window_table;
// edwards_G1 edwards_G1::G1_zero = {};
// edwards_G1 edwards_G1::G1_one = {};
// bool edwards_G1::initialized = false;

impl edwards_G1{
pub fn new()
{
    if initialized
    {
        self.X = G1_zero.X;
        self.Y = G1_zero.Y;
        self.Z = G1_zero.Z;
    }
}

pub fn print() const
{
    if self.is_zero()
    {
        print!("O\n");
    }
    else
    {
        let  copy=self.clone();
        copy.to_affine_coordinates();
        print!("({:N$} , {:N$}\n",
                   copy.X.as_bigint().0.0, 
                   copy.Y.as_bigint().0.0, N=edwards_Fq::num_limbs);
    }
}

pub fn print_coordinates() 
{
    if self.is_zero()
    {
        print!("O\n");
    }
    else
    {
        print!("({:N$} : {:N$} : {:N$})\n",
                   self.X.as_bigint().0.0, 
                   self.Y.as_bigint().0.0, 
                   self.Z.as_bigint().0.0, N=edwards_Fq::num_limbs);
    }
}

pub fn to_affine_coordinates()
{
    if self.is_zero()
    {
        self.X = edwards_Fq::zero();
        self.Y = edwards_Fq::one();
        self.Z = edwards_Fq::one();
    }
    else
    {
        // go from inverted coordinates to projective coordinates
        let tX = self.Y * self.Z;
        let tY = self.X * self.Z;
        let tZ = self.X * self.Y;
        // go from projective coordinates to affine coordinates
        let tZ_inv = tZ.inverse();
        self.X = tX * tZ_inv;
        self.Y = tY * tZ_inv;
        self.Z = edwards_Fq::one();
    }
}

pub fn to_special()
{
    if self.Z.is_zero()
    {
        return;
    }

// #ifdef DEBUG
    // const edwards_G1 copy(self.clone());
//#endif

    let  Z_inv = self.Z.inverse();
    self.X = self.X * Z_inv;
    self.Y = self.Y * Z_inv;
    self.Z = edwards_Fq::one();

// #ifdef DEBUG
    // assert!((self.clone()) == copy);
//#endif
}

pub fn is_special()->bool
{
    return (self.is_zero() || self.Z == edwards_Fq::one());
}

pub fn is_zero()->bool
{
    return (self.Y.is_zero() && self.Z.is_zero());
}



pub fn add(other:&edwards_G1)->edwards_G1
{
// #ifdef PROFILE_OP_COUNTS
    self.add_cnt+=1;
//#endif
    // NOTE: does not handle O and pts of order 2,4
    // http://www.hyperelliptic.org/EFD/g1p/auto-edwards-inverted.html#addition-add-2007-bl

    let A = (self.Z) * (other.Z);                   // A = Z1*Z2
    let B = edwards_coeff_d * A.squared();           // B = d*A^2
    let C = (self.X) * (other.X);                   // C = X1*X2
    let D = (self.Y) * (other.Y);                   // D = Y1*Y2
    let E = C * D;                                   // E = C*D
    let H = C - D;                                   // H = C-D
    let I = (self.X+self.Y)*(other.X+other.Y)-C-D; // I = (X1+Y1)*(X2+Y2)-C-D
    let X3 = (E+B)*H;                                // X3 = c*(E+B)*H
    let Y3 = (E-B)*I;                                // Y3 = c*(E-B)*I
    let Z3 = A*H*I;                                  // Z3 = A*H*I

    return edwards_G1::new(X3, Y3, Z3);
}

pub fn mixed_add(other:&edwards_G1)->edwards_G1
{
// #ifdef PROFILE_OP_COUNTS
    self.add_cnt+=1;
//#endif
    // handle special cases having to do with O
    if self.is_zero()
    {
        return other;
    }

    if other.is_zero()
    {
        return self.clone();
    }

// #ifdef DEBUG
    assert!(other.is_special());
//#endif

    // NOTE: does not handle O and pts of order 2,4
    // http://www.hyperelliptic.org/EFD/g1p/auto-edwards-inverted.html#addition-madd-2007-lb

    let A = self.Z;                                 // A = Z1
    let B = edwards_coeff_d * A.squared();           // B = d*A^2
    let C = (self.X) * (other.X);                   // C = X1*X2
    let D = (self.Y) * (other.Y);                   // D = Y1*Y2
    let E = C * D;                                   // E = C*D
    let H = C - D;                                   // H = C-D
    let I = (self.X+self.Y)*(other.X+other.Y)-C-D; // I = (X1+Y1)*(X2+Y2)-C-D
    let X3 = (E+B)*H;                                // X3 = c*(E+B)*H
    let Y3 = (E-B)*I;                                // Y3 = c*(E-B)*I
    let Z3 = A*H*I;                                  // Z3 = A*H*I

    return edwards_G1::new(X3, Y3, Z3);
}

pub fn dbl()->edwards_G1
{
// #ifdef PROFILE_OP_COUNTS
    self.dbl_cnt+=1;
//#endif
    if self.is_zero()
    {
        return (self.clone());
    }
    // NOTE: does not handle O and pts of order 2,4
    // http://www.hyperelliptic.org/EFD/g1p/auto-edwards-inverted.html#doubling-dbl-2007-bl

    let  A = (self.X).squared();                      // A = X1^2
    let  B = (self.Y).squared();                      // B = Y1^2
    let  C = A+B;                                      // C = A+B
    let  D = A-B;                                      // D = A-B
    let  E = (self.X+self.Y).squared()-C;            // E = (X1+Y1)^2-C
    let  X3 = C*D;                                     // X3 = C*D
    let  dZZ = edwards_coeff_d * self.Z.squared();
    let  Y3 = E*(C-dZZ-dZZ);                           // Y3 = E*(C-2*d*Z1^2)
    let  Z3 = D*E;                                     // Z3 = D*E

    return edwards_G1::new(X3, Y3, Z3);
}

pub fn is_well_formed()->bool
{
    /* Note that point at infinity is the only special case we must check as
       inverted representation does no cover points (0, +-c) and (+-c, 0). */
    if self.is_zero()
    {
        return true;
    }
    /*
        a x^2 + y^2 = 1 + d x^2 y^2

        We are using inverted, so equation we need to check is actually

        a (z/x)^2 + (z/y)^2 = 1 + d z^4 / (x^2 * y^2)
        z^2 (a y^2 + x^2 - dz^2) = x^2 y^2
    */
    let  X2 = self.X.squared();
    let Y2 = self.Y.squared();
    let Z2 = self.Z.squared();

    // for G1 a = 1
    return (Z2 * (Y2 + X2 - edwards_coeff_d * Z2) == X2 * Y2);
}

pub fn zero()->edwards_G1
{
    return G1_zero;
}

pub fn one()->edwards_G1
{
    return G1_one;
}

pub fn random_element()->edwards_G1
{
    return edwards_Fr::random_element().as_bigint() * G1_one;
}

pub fn batch_to_special_all_non_zeros(Vec<edwards_G1> &vec)
{
    let mut  Z_vec=Vec::with_capacity(vec.len());

    for el in &vec
    {
        Z_vec.push(el.Z);
    }
    batch_invert::<edwards_Fq>(Z_vec);

    let  one = edwards_Fq::one();

    for i in 0..vec.len()
    {
        vec[i].X = vec[i].X * Z_vec[i];
        vec[i].Y = vec[i].Y * Z_vec[i];
        vec[i].Z = one;
    }
}
}
// } // namespace libff


// pub fn operator==(other:&edwards_G1)->bool
// {
//     if self.is_zero()
//     {
//         return other.is_zero();
//     }

//     if other.is_zero()
//     {
//         return false;
//     }

//     /* now neither is O */

//     // X1/Z1 = X2/Z2 <=> X1*Z2 = X2*Z1
//     if (self.X * other.Z) != (other.X * self.Z)
//     {
//         return false;
//     }

//     // Y1/Z1 = Y2/Z2 <=> Y1*Z2 = Y2*Z1
//     if (self.Y * other.Z) != (other.Y * self.Z)
//     {
//         return false;
//     }

//     return true;
// }


// std::ostream& operator<<(std::ostream &out, g:&edwards_G1)
// {
//     edwards_G1 copy(g);
//     copy.to_affine_coordinates();
// // #ifdef NO_PT_COMPRESSION
//     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// #else
//     /* storing LSB of Y */
//     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.as_bigint().0.0[0] & 1);
// //#endif

//     return out;
// }


// bool edwards_G1::operator!=(other:&edwards_G1) const
// {
//     return !(operator==(other));
// }

// edwards_G1 edwards_G1::operator+(other:&edwards_G1) const
// {
//     // handle special cases having to do with O
//     if self.is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return (self.clone());
//     }

//     return self.add(other);
// }

// edwards_G1 edwards_G1::operator-() const
// {
//     return edwards_G1(-(self.X), self.Y, self.Z);
// }


// edwards_G1 edwards_G1::operator-(other:&edwards_G1) const
// {
//     return (self.clone()) + (-other);
// }

// std::istream& operator>>(std::istream &in, edwards_G1 &g)
// {
//     edwards_Fq tX, tY;

// // #ifdef NO_PT_COMPRESSION
//     in >> tX;
//     consume_OUTPUT_SEPARATOR(in);
//     in >> tY;
// #else
//     /*
//       a x^2 + y^2 = 1 + d x^2 y^2
//       y = sqrt((1-ax^2)/(1-dx^2))
//     */
//     unsigned char Y_lsb;
//     in >> tX;

//     consume_OUTPUT_SEPARATOR(in);
//     in.read((char*)&Y_lsb, 1);
//     Y_lsb -= '0';

//     edwards_Fq tX2 = tX.squared();
//     edwards_Fq tY2 = (edwards_Fq::one() - tX2) * // a = 1 for G1 (not a twist)
//         (edwards_Fq::one() - edwards_coeff_d * tX2).inverse();
//     tY = tY2.sqrt();

//     if (tY.as_bigint().0.0[0] & 1) != Y_lsb
//     {
//         tY = -tY;
//     }
// //#endif

//     // using inverted coordinates
//     g.X = tY;
//     g.Y = tX;
//     g.Z = tX * tY;

// // #ifdef USE_MIXED_ADDITION
//     g.to_special();
// //#endif

//     return in;
// }

// std::ostream& operator<<(std::ostream& out, v:&Vec<edwards_G1>)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// std::istream& operator>>(std::istream& in, Vec<edwards_G1> &v)
// {
//     v.clear();

//     usize s;
//     in >> s;
//     v.reserve(s);
//     consume_newline(in);

//     for i in 0..s
//     {
//         edwards_G1 g;
//         in >> g;
//         v.emplace_back(g);
//         consume_OUTPUT_NEWLINE(in);
//     }

//     return in;
// }
