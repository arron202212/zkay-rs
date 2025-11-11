// /** @file
//  *****************************************************************************

//  Declaration of interfaces for the MNT6 G1 group.

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef MNT6_G1_HPP_
// // #define MNT6_G1_HPP_

// //#include <vector>

// use crate::algebra::curves::curve_utils;
// use crate::algebra::curves::mnt::mnt6::mnt6_init;

// // namespace libff {

// // pub struct mnt6_G1;
// // std::ostream& operator<<(std::ostream &, const mnt6_G1&);
// // std::istream& operator>>(std::istream &, mnt6_G1&);

// pub struct mnt6_G1 {

// // #ifdef PROFILE_OP_COUNTS
//     static i64 add_cnt;
//     static i64 dbl_cnt;
// //#endif
//     static Vec<std::usize> wnaf_window_table;
//     static Vec<std::usize> fixed_base_exp_window_table;
//     static mnt6_G1 G1_zero;
//     static mnt6_G1 G1_one;
//     static bool initialized;
//     static mnt6_Fq coeff_a;
//     static mnt6_Fq coeff_b;

//     type base_field=mnt6_Fq;
//     type scalar_field=mnt6_Fr;

//     // Cofactor
//     static let h_bitcount= 1;
//     static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
//     static bigint<h_limbs> h;

//     mnt6_Fq X, Y, Z;

//     // using projective coordinates
//     mnt6_G1();
//     mnt6_G1(X:mnt6_Fq&, Y:&mnt6_Fq)->SelfX,Y, Z(base_field::one()) {}
//     mnt6_G1(X:mnt6_Fq&, Y:mnt6_Fq&, Z:&mnt6_Fq)->SelfX,Y,Z {}

//     pub fn  print() const;
//     pub fn  print_coordinates() const;

//     pub fn  to_affine_coordinates();
//     pub fn  to_special();
//     bool is_special() const;

//     bool is_zero() const;

//     bool operator==(other:&mnt6_G1) const;
//     bool operator!=(other:&mnt6_G1) const;

//     mnt6_G1 operator+(other:&mnt6_G1) const;
//     mnt6_G1 operator-() const;
//     mnt6_G1 operator-(other:&mnt6_G1) const;

//     mnt6_G1 add(other:&mnt6_G1) const;
//     mnt6_G1 mixed_add(other:&mnt6_G1) const;
//     mnt6_G1 dbl() const;
//     mnt6_G1 mul_by_cofactor() const;

//     bool is_well_formed() const;

//     static mnt6_G1 zero();
//     static mnt6_G1 one();
//     static mnt6_G1 random_element();

//     static std::usize size_in_bits() { return base_field::ceil_size_in_bits() + 1; }
//     static bigint<base_field::num_limbs> field_char() { return base_field::field_char(); }
//     static bigint<scalar_field::num_limbs> order() { return scalar_field::field_char(); }

//     friend std::ostream& operator<<(std::ostream &out, g:&mnt6_G1);
//     friend std::istream& operator>>(std::istream &in, mnt6_G1 &g);

//     static pub fn  batch_to_special_all_non_zeros(Vec<mnt6_G1> &vec);
// };


// mnt6_G1 operator*(lhs:&bigint<m>, rhs:&mnt6_G1)
// {
//     return scalar_mul<mnt6_G1, m>(rhs, lhs);
// }


// mnt6_G1 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&mnt6_G1)
// {
//     return scalar_mul<mnt6_G1, m>(rhs, lhs.as_bigint());
// }

// std::ostream& operator<<(std::ostream& out, v:&Vec<mnt6_G1>);
// std::istream& operator>>(std::istream& in, Vec<mnt6_G1> &v);

// // } // namespace libff

// //#endif // MNT6_G1_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of interfaces for the MNT6 G1 group.

//  See mnt6_g1.hpp .

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use crate::algebra::curves::mnt::mnt6::mnt6_g1;

// // namespace libff {

// using std::usize;

// // #ifdef PROFILE_OP_COUNTS
// i64 mnt6_G1::add_cnt = 0;
// i64 mnt6_G1::dbl_cnt = 0;
// //#endif

// Vec<usize> mnt6_G1::wnaf_window_table;
// Vec<usize> mnt6_G1::fixed_base_exp_window_table;
// mnt6_G1 mnt6_G1::G1_zero = {};
// mnt6_G1 mnt6_G1::G1_one = {};
// bool mnt6_G1::initialized = false;
// mnt6_Fq mnt6_G1::coeff_a;
// mnt6_Fq mnt6_G1::coeff_b;
// bigint<mnt6_G1::h_limbs> mnt6_G1::h;

// pub fn new()
// {
//     if mnt6_G1::initialized
//     {
//         this->X = G1_zero.X;
//         this->Y = G1_zero.Y;
//         this->Z = G1_zero.Z;
//     }
// }

// pub fn print() const
// {
//     if this->is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         mnt6_G1 copy(*this);
//         copy.to_affine_coordinates();
//         print!("(%Nd , %Nd)\n",
//                    copy.X.as_bigint().0.0, mnt6_Fq::num_limbs,
//                    copy.Y.as_bigint().0.0, mnt6_Fq::num_limbs);
//     }
// }

// pub fn print_coordinates() const
// {
//     if this->is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         print!("(%Nd : %Nd : %Nd)\n",
//                    this->X.as_bigint().0.0, mnt6_Fq::num_limbs,
//                    this->Y.as_bigint().0.0, mnt6_Fq::num_limbs,
//                    this->Z.as_bigint().0.0, mnt6_Fq::num_limbs);
//     }
// }

// pub fn to_affine_coordinates()
// {
//     if this->is_zero()
//     {
//         this->X = mnt6_Fq::zero();
//         this->Y = mnt6_Fq::one();
//         this->Z = mnt6_Fq::zero();
//     }
//     else
//     {
//         let Z_inv= Z.inverse();
//         this->X = this->X * Z_inv;
//         this->Y = this->Y * Z_inv;
//         this->Z = mnt6_Fq::one();
//     }
// }

// pub fn to_special()
// {
//     this->to_affine_coordinates();
// }

// pub fn is_special()->bool
// {
//     return (this->is_zero() || this->Z == mnt6_Fq::one());
// }

// pub fn is_zero()->bool
// {
//     return (this->X.is_zero() && this->Z.is_zero());
// }

// bool mnt6_G1::operator==(other:&mnt6_G1) const
// {
//     if this->is_zero()
//     {
//         return other.is_zero();
//     }

//     if other.is_zero()
//     {
//         return false;
//     }

//     /* now neither is O */

//     // X1/Z1 = X2/Z2 <=> X1*Z2 = X2*Z1
//     if (this->X * other.Z) != (other.X * this->Z)
//     {
//         return false;
//     }

//     // Y1/Z1 = Y2/Z2 <=> Y1*Z2 = Y2*Z1
//     if (this->Y * other.Z) != (other.Y * this->Z)
//     {
//         return false;
//     }

//     return true;
// }

// bool mnt6_G1::operator!=(other:&mnt6_G1) const
// {
//     return !(operator==(other));
// }

// mnt6_G1 mnt6_G1::operator+(other:&mnt6_G1) const
// {
//     // handle special cases having to do with O
//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return *this;
//     }

//     // no need to handle points of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // handle double case, and then all the rest
//     /*
//       The code below is equivalent to (but faster than) the snippet below:

//       if this->operator==(other)
//       {
//       return this->dbl();
//       }
//       else
//       {
//       return this->add(other);
//       }
//     */

//     let X1Z2= (this->X) * (other.Z);        // X1Z2 = X1*Z2
//     let X2Z1= (this->Z) * (other.X);        // X2Z1 = X2*Z1

//     // (used both in add and double checks)

//     let Y1Z2= (this->Y) * (other.Z);        // Y1Z2 = Y1*Z2
//     let Y2Z1= (this->Z) * (other.Y);        // Y2Z1 = Y2*Z1

//     if X1Z2 == X2Z1 && Y1Z2 == Y2Z1
//     {
//         // perform dbl case
//         let XX= (this->X).squared();                   // XX  = X1^2
//         let ZZ= (this->Z).squared();                   // ZZ  = Z1^2
//         let w= mnt6_G1::coeff_a * ZZ + (XX + XX + XX); // w   = a*ZZ + 3*XX
//         let Y1Z1= (this->Y) * (this->Z);
//         let s= Y1Z1 + Y1Z1;                            // s   = 2*Y1*Z1
//         let ss= s.squared();                            // ss  = s^2
//         let sss= s * ss;                                 // sss = s*ss
//         let R= (this->Y) * s;                         // R   = Y1*s
//         let RR= R.squared();                            // RR  = R^2
//         let B= ((this->X)+R).squared()-XX-RR;         // B   = (X1+R)^2 - XX - RR
//         let h= w.squared() - (B+B);                    // h   = w^2 - 2*B
//         let X3= h * s;                                  // X3  = h*s
//         let Y3= w * (B-h)-(RR+RR);                      // Y3  = w*(B-h) - 2*RR
//         let Z3= sss;                                    // Z3  = sss

//         return mnt6_G1(X3, Y3, Z3);
//     }

//     // if we have arrived here we are in the add case
//     let Z1Z2= (this->Z) * (other.Z);      // Z1Z2 = Z1*Z2
//     let u= Y2Z1 - Y1Z2;                  // u    = Y2*Z1-Y1Z2
//     let uu= u.squared();                  // uu   = u^2
//     let v= X2Z1 - X1Z2;                  // v    = X2*Z1-X1Z2
//     let vv= v.squared();                  // vv   = v^2
//     let vvv= v * vv;                       // vvv  = v*vv
//     let R= vv * X1Z2;                    // R    = vv*X1Z2
//     let A= uu * Z1Z2 - (vvv + R + R);    // A    = uu*Z1Z2 - vvv - 2*R
//     let X3= v * A;                        // X3   = v*A
//     let Y3= u * (R-A) - vvv * Y1Z2;       // Y3   = u*(R-A) - vvv*Y1Z2
//     let Z3= vvv * Z1Z2;                   // Z3   = vvv*Z1Z2

//     return mnt6_G1(X3, Y3, Z3);
// }

// mnt6_G1 mnt6_G1::operator-() const
// {
//     return mnt6_G1(this->X, -(this->Y), this->Z);
// }


// mnt6_G1 mnt6_G1::operator-(other:&mnt6_G1) const
// {
//     return (*this) + (-other);
// }

// pub fn add(other:&mnt6_G1)->mnt6_G1
// {
//     // handle special cases having to do with O
//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return (*this);
//     }

//     // no need to handle points of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // handle double case
//     if this->operator==(other)
//     {
//         return this->dbl();
//     }

// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2

//     let Y1Z2= (this->Y) * (other.Z);        // Y1Z2 = Y1*Z2
//     let X1Z2= (this->X) * (other.Z);        // X1Z2 = X1*Z2
//     let Z1Z2= (this->Z) * (other.Z);        // Z1Z2 = Z1*Z2
//     let u= (other.Y) * (this->Z) - Y1Z2; // u    = Y2*Z1-Y1Z2
//     let uu= u.squared();                    // uu   = u^2
//     let v= (other.X) * (this->Z) - X1Z2; // v    = X2*Z1-X1Z2
//     let vv= v.squared();                    // vv   = v^2
//     let vvv= v * vv;                         // vvv  = v*vv
//     let R= vv * X1Z2;                      // R    = vv*X1Z2
//     let A= uu * Z1Z2 - (vvv + R + R);      // A    = uu*Z1Z2 - vvv - 2*R
//     let X3= v * A;                          // X3   = v*A
//     let Y3= u * (R-A) - vvv * Y1Z2;         // Y3   = u*(R-A) - vvv*Y1Z2
//     let Z3= vvv * Z1Z2;                     // Z3   = vvv*Z1Z2

//     return mnt6_G1(X3, Y3, Z3);
// }

// pub fn mixed_add(other:&mnt6_G1)->mnt6_G1
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2
//     //assert!(other.Z == mnt6_Fq::one());

//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return (*this);
//     }

// // #ifdef DEBUG
//     assert!(other.is_special());
// //#endif

//     X1Z2:&mnt6_Fq = (this->X);                    // X1Z2 = X1*Z2 (but other is special and not zero)
//     let X2Z1= (this->Z) * (other.X);        // X2Z1 = X2*Z1

//     // (used both in add and double checks)

//     Y1Z2:&mnt6_Fq = (this->Y);                    // Y1Z2 = Y1*Z2 (but other is special and not zero)
//     let Y2Z1= (this->Z) * (other.Y);        // Y2Z1 = Y2*Z1

//     if X1Z2 == X2Z1 && Y1Z2 == Y2Z1
//     {
//         return this->dbl();
//     }

//     mnt6_Fq u = Y2Z1 - this->Y;             // u = Y2*Z1-Y1
//     mnt6_Fq uu = u.squared();                // uu = u2
//     mnt6_Fq v = X2Z1 - this->X;             // v = X2*Z1-X1
//     mnt6_Fq vv = v.squared();                // vv = v2
//     mnt6_Fq vvv = v*vv;                      // vvv = v*vv
//     mnt6_Fq R = vv * this->X;               // R = vv*X1
//     mnt6_Fq A = uu * this->Z - vvv - R - R; // A = uu*Z1-vvv-2*R
//     mnt6_Fq X3 = v * A;                      // X3 = v*A
//     mnt6_Fq Y3 = u*(R-A) - vvv * this->Y;   // Y3 = u*(R-A)-vvv*Y1
//     mnt6_Fq Z3 = vvv * this->Z;             // Z3 = vvv*Z1

//     return mnt6_G1(X3, Y3, Z3);
// }

// pub fn dbl()->mnt6_G1
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->dbl_cnt++;
// //#endif
//     if this->is_zero()
//     {
//         return (*this);
//     }
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#doubling-dbl-2007-bl

//     let XX= (this->X).squared();                   // XX  = X1^2
//     let ZZ= (this->Z).squared();                   // ZZ  = Z1^2
//     let w= mnt6_G1::coeff_a * ZZ + (XX + XX + XX); // w   = a*ZZ + 3*XX
//     let Y1Z1= (this->Y) * (this->Z);
//     let s= Y1Z1 + Y1Z1;                            // s   = 2*Y1*Z1
//     let ss= s.squared();                            // ss  = s^2
//     let sss= s * ss;                                 // sss = s*ss
//     let R= (this->Y) * s;                         // R   = Y1*s
//     let RR= R.squared();                            // RR  = R^2
//     let B= ((this->X)+R).squared()-XX-RR;         // B   = (X1+R)^2 - XX - RR
//     let h= w.squared() - (B+B);                    // h   = w^2 - 2*B
//     let X3= h * s;                                  // X3  = h*s
//     let Y3= w * (B-h)-(RR+RR);                      // Y3  = w*(B-h) - 2*RR
//     let Z3= sss;                                    // Z3  = sss

//     return mnt6_G1(X3, Y3, Z3);
// }

// pub fn mul_by_cofactor()->mnt6_G1
// {
//     // Cofactor = 1
//     return (*this);
// }

// pub fn is_well_formed()->bool
// {
//     if this->is_zero()
//     {
//         return true;
//     }
//     /*
//         y^2 = x^3 + ax + b

//         We are using projective, so equation we need to check is actually

//         (y/z)^2 = (x/z)^3 + a (x/z) + b
//         z y^2 = x^3  + a z^2 x + b z^3

//         z (y^2 - b z^2) = x ( x^2 + a z^2)
//     */
//     let X2= this->X.squared();
//     let Y2= this->Y.squared();
//     let Z2= this->Z.squared();

//     return (this->Z * (Y2 - mnt6_G1::coeff_b * Z2) == this->X * (X2 + mnt6_G1::coeff_a * Z2));
// }

// mnt6_G1 mnt6_G1::zero()
// {
//     return G1_zero;
// }

// mnt6_G1 mnt6_G1::one()
// {
//     return G1_one;
// }

// mnt6_G1 mnt6_G1::random_element()
// {
//     return (scalar_field::random_element().as_bigint()) * G1_one;
// }

// std::ostream& operator<<(std::ostream &out, g:&mnt6_G1)
// {
//     mnt6_G1 copy(g);
//     copy.to_affine_coordinates();

//     out << if copy.is_zero() {1} else{0} << OUTPUT_SEPARATOR;
// // #ifdef NO_PT_COMPRESSION
//     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// #else
//     /* storing LSB of Y */
//     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.as_bigint().0.0[0] & 1);
// //#endif

//     return out;
// }

// std::istream& operator>>(std::istream &in, mnt6_G1 &g)
// {
//     char is_zero;
//     mnt6_Fq tX, tY;

// // #ifdef NO_PT_COMPRESSION
//     in >> is_zero >> tX >> tY;
//     is_zero -= '0';
// #else
//     in.read((char*)&is_zero, 1); // this reads is_zero;
//     is_zero -= '0';
//     consume_OUTPUT_SEPARATOR(in);

//     unsigned char Y_lsb;
//     in >> tX;
//     consume_OUTPUT_SEPARATOR(in);
//     in.read((char*)&Y_lsb, 1);
//     Y_lsb -= '0';

//     // y = +/- sqrt(x^3 + a*x + b)
//     if is_zero == 0
//     {
//         mnt6_Fq tX2 = tX.squared();
//         mnt6_Fq tY2 = (tX2 + mnt6_G1::coeff_a) * tX + mnt6_G1::coeff_b;
//         tY = tY2.sqrt();

//         if (tY.as_bigint().0.0[0] & 1) != Y_lsb
//         {
//             tY = -tY;
//         }
//     }
// //#endif
//     // using projective coordinates
//     if is_zero == 0
//     {
//         g.X = tX;
//         g.Y = tY;
//         g.Z = mnt6_Fq::one();
//     }
//     else
//     {
//         g = mnt6_G1::zero();
//     }

//     return in;
// }

// std::ostream& operator<<(std::ostream& out, v:&Vec<mnt6_G1>)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// std::istream& operator>>(std::istream& in, Vec<mnt6_G1> &v)
// {
//     v.clear();

//     usize s;
//     in >> s;
//     consume_newline(in);

//     v.reserve(s);

//     for i in 0..s
//     {
//         mnt6_G1 g;
//         in >> g;
//         consume_OUTPUT_NEWLINE(in);
//         v.emplace_back(g);
//     }

//     return in;
// }

// pub fn batch_to_special_all_non_zeros(Vec<mnt6_G1> &vec)
// {
//     Vec<mnt6_Fq> Z_vec;
//     Z_vec.reserve(vec.len());

//     for el in &vec
//     {
//         Z_vec.emplace_back(el.Z);
//     }
//     batch_invert<mnt6_Fq>(Z_vec);

//     let one= mnt6_Fq::one();

//     for i in 0..vec.len()
//     {
//         vec[i] = mnt6_G1(vec[i].X * Z_vec[i], vec[i].Y * Z_vec[i], one);
//     }
// }

// // } // namespace libff

use crate::algebra::curves::{
    mnt::mnt6::{MNT6,MNT6Config},
    short_weierstrass::{Affine, Projective},
    AffineRepr, CurveGroup,
};
use ffec::algebra::fields::{field::Field, prime_extension::fp3::Fp3};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::*;
use educe::Educe;

pub type G1Affine<P> = Affine<<P as MNT6Config>::G1Config>;
pub type G1Projective<P> = Projective<<P as MNT6Config>::G1Config>;

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Copy, Clone, Debug, PartialEq, Eq)]
pub struct G1Prepared<P: MNT6Config> {
    pub x: P::Fp,
    pub y: P::Fp,
    pub x_twist: Fp3<P::Fp3Config>,
    pub y_twist: Fp3<P::Fp3Config>,
}

impl<P: MNT6Config> From<G1Affine<P>> for G1Prepared<P> {
    fn from(g1: G1Affine<P>) -> Self {
        let mut x_twist = P::TWIST;
        x_twist.mul_assign_by_fp(&g1.x);

        let mut y_twist = P::TWIST;
        y_twist.mul_assign_by_fp(&g1.y);

        Self {
            x: g1.x,
            y: g1.y,
            x_twist,
            y_twist,
        }
    }
}

impl<'a, P: MNT6Config> From<&'a G1Affine<P>> for G1Prepared<P> {
    fn from(g1: &'a G1Affine<P>) -> Self {
        (*g1).into()
    }
}

impl<P: MNT6Config> From<G1Projective<P>> for G1Prepared<P> {
    fn from(g1: G1Projective<P>) -> Self {
        g1.into_affine().into()
    }
}
impl<'a, P: MNT6Config> From<&'a G1Projective<P>> for G1Prepared<P> {
    fn from(g1: &'a G1Projective<P>) -> Self {
        (*g1).into()
    }
}

impl<P: MNT6Config> Default for G1Prepared<P> {
    fn default() -> Self {
        Self::from(G1Affine::<P>::generator())
    }
}
