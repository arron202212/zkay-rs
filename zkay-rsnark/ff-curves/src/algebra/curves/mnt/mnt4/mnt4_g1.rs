

// //  Declaration of interfaces for the MNT4 G1 group.


// 
// // #define MNT4_G1_HPP_

// //#include <vector>

// use crate::algebra::curves::curve_utils;
// use crate::algebra::curves::mnt::mnt4::mnt4_init;



// // pub struct mnt4_G1;
// // std::ostream& operator<<(std::ostream &, const mnt4_G1&);
// // std::istream& operator>>(std::istream &, mnt4_G1&);

// pub struct mnt4_G1 {
// //
// // // #ifdef PROFILE_OP_COUNTS
// //     static i64 add_cnt;
// //     static i64 dbl_cnt;
// // 
// //     static Vec<std::usize> wnaf_window_table;
// //     static Vec<std::usize> fixed_base_exp_window_table;
// //     static mnt4_G1 G1_zero;
// //     static mnt4_G1 G1_one;
// //     static bool initialized;
// //     static mnt4_Fq coeff_a;
// //     static mnt4_Fq coeff_b;

//     // type base_field=mnt4_Fq;
//     // type scalar_field=mnt4_Fr;

//     // Cofactor
//     // static let h_bitcount= 1;
//     // static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
//     // static bigint<h_limbs> h;

//      X:mnt4_Fq, Y:mnt4_Fq, Z:mnt4_Fq
// }

// //     // using projective coordinates
// //     mnt4_G1();
// //     mnt4_G1(X:mnt4_Fq&, Y:&mnt4_Fq)->SelfX,Y, Z(base_field::one()) {}
// //     mnt4_G1(X:mnt4_Fq&, Y:mnt4_Fq&, Z:&mnt4_Fq)->SelfX,Y,Z {}

// //     pub fn  print() const;
// //     pub fn  print_coordinates() const;

// //     pub fn  to_affine_coordinates();
// //     pub fn  to_special();
// //     bool is_special() const;

// //     bool is_zero() const;

// //     bool operator==(other:&mnt4_G1) const;
// //     bool operator!=(other:&mnt4_G1) const;

// //     mnt4_G1 operator+(other:&mnt4_G1) const;
// //     mnt4_G1 operator-() const;
// //     mnt4_G1 operator-(other:&mnt4_G1) const;

// //     mnt4_G1 add(other:&mnt4_G1) const;
// //     mnt4_G1 mixed_add(other:&mnt4_G1) const;
// //     mnt4_G1 dbl() const;
// //     mnt4_G1 mul_by_cofactor() const;

// //     bool is_well_formed() const;

// //     static mnt4_G1 zero();
// //     static mnt4_G1 one();
// //     static mnt4_G1 random_element();

// //     static std::usize size_in_bits() { return mnt4_Fq::ceil_size_in_bits() + 1; }
// //     static bigint<mnt4_Fq::num_limbs> field_char() { return mnt4_Fq::field_char(); }
// //     static bigint<mnt4_Fr::num_limbs> order() { return mnt4_Fr::field_char(); }

// //     friend std::ostream& operator<<(std::ostream &out, g:&mnt4_G1);
// //     friend std::istream& operator>>(std::istream &in, mnt4_G1 &g);

// //     static pub fn  batch_to_special_all_non_zeros(Vec<mnt4_G1> &vec);
// // };

// //
// // mnt4_G1 operator*(lhs:&bigint<m>, rhs:&mnt4_G1)
// // {
// //     return scalar_mul<mnt4_G1, m>(rhs, lhs);
// // }

// //
// // mnt4_G1 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&mnt4_G1)
// // {
// //     return scalar_mul<mnt4_G1, m>(rhs, lhs.as_bigint());
// // }

// // std::ostream& operator<<(std::ostream& out, v:&Vec<mnt4_G1>);
// // std::istream& operator>>(std::istream& in, Vec<mnt4_G1> &v);

// 

// 


//  Implementation of interfaces for the MNT4 G1 group.

//  See mnt4_g1.hpp .


// // use crate::algebra::curves::mnt::mnt4::mnt4_g1;



// // using std::usize;

// // #ifdef PROFILE_OP_COUNTS
// // i64 mnt4_G1::add_cnt = 0;
// // i64 mnt4_G1::dbl_cnt = 0;
// 

// // Vec<usize> mnt4_G1::wnaf_window_table;
// // Vec<usize> mnt4_G1::fixed_base_exp_window_table;
// // mnt4_G1 mnt4_G1::G1_zero = {};
// // mnt4_G1 mnt4_G1::G1_one = {};
// // bool mnt4_G1::initialized = false;
// // mnt4_Fq mnt4_G1::coeff_a;
// // mnt4_Fq mnt4_G1::coeff_b;
// // bigint<mnt4_G1::h_limbs> mnt4_G1::h;

// impl mnt4_G1{
// pub fn new()->Self
// {
//     // if Self::initialized
//     // {
//     //     self.X = G1_zero.X;
//     //     self.Y = G1_zero.Y;
//     //     self.Z = G1_zero.Z;
//     // }
//     // Self{X,Y,Z}
//     G1_zero
// }

// pub fn print(&self)
// {
//     if self.is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         let mut  copy=self.clone();
//         copy.to_affine_coordinates();
//         print!("({:N$} , {:N$})\n",
//                    copy.X.as_bigint().0.0,
//                    copy.Y.as_bigint().0.0, N=mnt4_Fq::num_limbs);
//     }
// }

// pub fn print_coordinates(&self)
// {
//     if self.is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         print!("({:N$}: {:N$}: {:N$})\n",
//                    self.X.as_bigint().0.0,
//                    self.Y.as_bigint().0.0,
//                    self.Z.as_bigint().0.0, N=mnt4_Fq::num_limbs);
//     }
// }

// pub fn to_affine_coordinates(&mut self)
// {
//     if self.is_zero()
//     {
//         self.X = mnt4_Fq::zero();
//         self.Y = mnt4_Fq::one();
//         self.Z = mnt4_Fq::zero();
//     }
//     else
//     {
//         let  Z_inv = Z.inverse();
//         self.X = self.X.clone()* Z_inv;
//         self.Y = self.Y.clone() * Z_inv;
//         self.Z = mnt4_Fq::one();
//     }
// }

// pub fn to_special(&mut self)
// {
//     self.to_affine_coordinates();
// }

// pub fn is_special()->bool
// {
//     return (self.is_zero() || self.Z == mnt4_Fq::one());
// }

// pub fn is_zero()->bool
// {
//     return (self.X.is_zero() && self.Z.is_zero());
// }

// pub fn add( other:&Self) ->&mnt4_G1
// {
//     // handle special cases having to do with O
//     if self.is_zero()
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
//     if self.operator==(other)
//     {
//         return self.dbl();
//     }

// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt+=1;
// 
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2

//     // let Y1Z2= (self.Y) * (other.Z);        // Y1Z2 = Y1*Z2
//     // let X1Z2= (self.X) * (other.Z);        // X1Z2 = X1*Z2
//     // let Z1Z2= (self.Z) * (other.Z);        // Z1Z2 = Z1*Z2
//     // let u= (other.Y) * (self.Z) - Y1Z2; // u    = Y2*Z1-Y1Z2
//     // let uu= u.squared();                    // uu   = u^2
//     // let v= (other.X) * (self.Z) - X1Z2; // v    = X2*Z1-X1Z2
//     // let vv= v.squared();                    // vv   = v^2
//     // let vvv= v * vv;                         // vvv  = v*vv
//     // let R= vv * X1Z2;                      // R    = vv*X1Z2
//     // let A= uu * Z1Z2 - (vvv + R + R);      // A    = uu*Z1Z2 - vvv - 2*R
//     // let X3= v * A;                          // X3   = v*A
//     // let Y3= u * (R-A) - vvv * Y1Z2;         // Y3   = u*(R-A) - vvv*Y1Z2
//     // let Z3= vvv * Z1Z2;                     // Z3   = vvv*Z1Z2

//     return mnt4_G1(X3, Y3, Z3);
// }

// pub fn mixed_add(other:&mnt4_G1)->mnt4_G1
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt+=1;
// 
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2
//     //assert!(other.Z == mnt4_Fq::one());

//     if self.is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return (*this);
//     }

// // #ifdef DEBUG
//     assert!(other.is_special());
// 

//     // X1Z2:&mnt4_Fq = (self.X);                    // X1Z2 = X1*Z2 (but other is special and not zero)
//     // let X2Z1= (self.Z) * (other.X);        // X2Z1 = X2*Z1

//     // // (used both in add and double checks)

//     // Y1Z2:&mnt4_Fq = (self.Y);                    // Y1Z2 = Y1*Z2 (but other is special and not zero)
//     // let Y2Z1= (self.Z) * (other.Y);        // Y2Z1 = Y2*Z1

//     // if X1Z2 == X2Z1 && Y1Z2 == Y2Z1
//     // {
//     //     return self.dbl();
//     // }

//     // let u= Y2Z1 - self.Y;              // u = Y2*Z1-Y1
//     // let uu= u.squared();                 // uu = u2
//     // let v= X2Z1 - self.X;              // v = X2*Z1-X1
//     // let vv= v.squared();                 // vv = v2
//     // let vvv= v*vv;                       // vvv = v*vv
//     // let R= vv * self.X;                // R = vv*X1
//     // let A= uu * self.Z - vvv - R - R;  // A = uu*Z1-vvv-2*R
//     // let X3= v * A;                       // X3 = v*A
//     // let Y3= u*(R-A) - vvv * self.Y;    // Y3 = u*(R-A)-vvv*Y1
//     // let Z3= vvv * self.Z;              // Z3 = vvv*Z1

//     return mnt4_G1(X3, Y3, Z3);
// }

// pub fn dbl()->mnt4_G1
// {
// // #ifdef PROFILE_OP_COUNTS
//     self.dbl_cnt+=1;
// 
//     if self.is_zero()
//     {
//         return (*this);
//     }
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#doubling-dbl-2007-bl

//     // let XX= (self.X).squared();                   // XX  = X1^2
//     // let ZZ= (self.Z).squared();                   // ZZ  = Z1^2
//     // let w= mnt4_G1::coeff_a * ZZ + (XX + XX + XX); // w   = a*ZZ + 3*XX
//     // let Y1Z1= (self.Y) * (self.Z);
//     // let s= Y1Z1 + Y1Z1;                            // s   = 2*Y1*Z1
//     // let ss= s.squared();                            // ss  = s^2
//     // let sss= s * ss;                                 // sss = s*ss
//     // let R= (self.Y) * s;                         // R   = Y1*s
//     // let RR= R.squared();                            // RR  = R^2
//     // let B= ((self.X)+R).squared()-XX-RR;         // B   = (X1+R)^2 - XX - RR
//     // let h= w.squared() - (B+B);                    // h   = w^2 - 2*B
//     // let X3= h * s;                                  // X3  = h*s
//     // let Y3= w * (B-h)-(RR+RR);                      // Y3  = w*(B-h) - 2*RR
//     // let Z3= sss;                                    // Z3  = sss

//     return mnt4_G1(X3, Y3, Z3);
// }

// pub fn mul_by_cofactor()->mnt4_G1
// {
//     // Cofactor = 1
//     return (*this);
// }

// pub fn is_well_formed()->bool
// {
//     if self.is_zero()
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
//     // let X2= self.X.squared();
//     // let Y2= self.Y.squared();
//     // let Z2= self.Z.squared();

//     // return (self.Z * (Y2 - mnt4_G1::coeff_b * Z2) == self.X * (X2 + mnt4_G1::coeff_a * Z2));
//     false
// }

// pub fn zero()->mnt4_G1
// {
//     return G1_zero;
// }

// pub fn one()->mnt4_G1
// {
//     return G1_one;
// }

// pub fn random_element()->mnt4_G1
// {
//     return (scalar_field::random_element().as_bigint()) * G1_one;
// }

// pub fn batch_to_special_all_non_zeros(vec:&Vec<mnt4_G1>)
// {
//     let  Z_vec=Vec::with_capacity(vec.len());

//     for el in &vec
//     {
//         Z_vec.emplace_back(el.Z);
//     }
//     batch_invert::<mnt4_Fq>(Z_vec);

//     let mut one = mnt4_Fq::one();

//     for i in 0..vec.len()
//     {
//         vec[i] = mnt4_G1::from(vec[i].X * Z_vec[i], vec[i].Y * Z_vec[i], one);
//     }
// }
// }
// 

// // bool mnt4_G1::operator==(other:&mnt4_G1) const
// // {
// //     if self.is_zero()
// //     {
// //         return other.is_zero();
// //     }

// //     if other.is_zero()
// //     {
// //         return false;
// //     }

// //     /* now neither is O */
// //     // X1/Z1 = X2/Z2 <=> X1*Z2 = X2*Z1
// //     if (self.X * other.Z) != (other.X * self.Z)
// //     {
// //         return false;
// //     }

// //     // Y1/Z1 = Y2/Z2 <=> Y1*Z2 = Y2*Z1
// //     if (self.Y * other.Z) != (other.Y * self.Z)
// //     {
// //         return false;
// //     }

// //     return true;
// // }

// // bool mnt4_G1::operator!=(other:&mnt4_G1) const
// // {
// //     return !(operator==(other));
// // }

// // mnt4_G1 mnt4_G1::operator+(other:&mnt4_G1) const
// // {
// //     // handle special cases having to do with O
// //     if self.is_zero()
// //     {
// //         return other;
// //     }

// //     if other.is_zero()
// //     {
// //         return *this;
// //     }

// //     // no need to handle points of order 2,4
// //     // (they cannot exist in a prime-order subgroup)

// //     // handle double case, and then all the rest
// //     /*
// //       The code below is equivalent to (but faster than) the snippet below:

// //       if self.operator==(other)
// //       {
// //       return self.dbl();
// //       }
// //       else
// //       {
// //       return self.add(other);
// //       }
// //     */
// //     let X1Z2= (self.X) * (other.Z);        // X1Z2 = X1*Z2
// //     let X2Z1= (self.Z) * (other.X);        // X2Z1 = X2*Z1

// //     // (used both in add and double checks)

// //     let Y1Z2= (self.Y) * (other.Z);        // Y1Z2 = Y1*Z2
// //     let Y2Z1= (self.Z) * (other.Y);        // Y2Z1 = Y2*Z1

// //     if X1Z2 == X2Z1 && Y1Z2 == Y2Z1
// //     {
// //         // perform dbl case
// //         let XX= (self.X).squared();                   // XX  = X1^2
// //         let ZZ= (self.Z).squared();                   // ZZ  = Z1^2
// //         let w= mnt4_G1::coeff_a * ZZ + (XX + XX + XX); // w   = a*ZZ + 3*XX
// //         let Y1Z1= (self.Y) * (self.Z);
// //         let s= Y1Z1 + Y1Z1;                            // s   = 2*Y1*Z1
// //         let ss= s.squared();                            // ss  = s^2
// //         let sss= s * ss;                                 // sss = s*ss
// //         let R= (self.Y) * s;                         // R   = Y1*s
// //         let RR= R.squared();                            // RR  = R^2
// //         let B= ((self.X)+R).squared()-XX-RR;         // B   = (X1+R)^2 - XX - RR
// //         let h= w.squared() - (B+B);                    // h   = w^2 - 2*B
// //         let X3= h * s;                                  // X3  = h*s
// //         let Y3= w * (B-h)-(RR+RR);                      // Y3  = w*(B-h) - 2*RR
// //         let Z3= sss;                                    // Z3  = sss

// //         return mnt4_G1(X3, Y3, Z3);
// //     }

// //     // if we have arrived here we are in the add case
// //     let Z1Z2= (self.Z) * (other.Z);        // Z1Z2 = Z1*Z2
// //     let u= Y2Z1 - Y1Z2; // u    = Y2*Z1-Y1Z2
// //     let uu= u.squared();                  // uu   = u^2
// //     let v= X2Z1 - X1Z2; // v    = X2*Z1-X1Z2
// //     let vv= v.squared();                  // vv   = v^2
// //     let vvv= v * vv;                       // vvv  = v*vv
// //     let R= vv * X1Z2;                    // R    = vv*X1Z2
// //     let A= uu * Z1Z2 - (vvv + R + R);    // A    = uu*Z1Z2 - vvv - 2*R
// //     let X3= v * A;                        // X3   = v*A
// //     let Y3= u * (R-A) - vvv * Y1Z2;       // Y3   = u*(R-A) - vvv*Y1Z2
// //     let Z3= vvv * Z1Z2;                   // Z3   = vvv*Z1Z2

// //     return mnt4_G1(X3, Y3, Z3);
// // }

// // mnt4_G1 mnt4_G1::operator-() const
// // {
// //     return mnt4_G1(self.X, -(self.Y), self.Z);
// // }

// // mnt4_G1 mnt4_G1::operator-(other:&mnt4_G1) const
// // {
// //     return (*this) + (-other);
// // }

// // std::ostream& operator<<(std::ostream &out, g:&mnt4_G1)
// // {
// //     mnt4_G1 copy(g);
// //     copy.to_affine_coordinates();

// //     out << if copy.is_zero() {1} else{0} << OUTPUT_SEPARATOR;
// // // #ifdef NO_PT_COMPRESSION
// //     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// // #else
// //     /* storing LSB of Y */
// //     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.as_bigint().0.0[0] & 1);
// // 

// //     return out;
// // }

// // std::istream& operator>>(std::istream &in, mnt4_G1 &g)
// // {
// //     char is_zero;
// //     mnt4_Fq tX, tY;

// // // #ifdef NO_PT_COMPRESSION
// //     in >> is_zero >> tX >> tY;
// //     is_zero -= '0';
// // #else
// //     in.read((char*)&is_zero, 1);
// //     is_zero -= '0';
// //     consume_OUTPUT_SEPARATOR(in);

// //     unsigned char Y_lsb;
// //     in >> tX;
// //     consume_OUTPUT_SEPARATOR(in);
// //     in.read((char*)&Y_lsb, 1);
// //     Y_lsb -= '0';

// //     // y = +/- sqrt(x^3 + a*x + b)
// //     if is_zero == 0
// //     {
// //         mnt4_Fq tX2 = tX.squared();
// //         mnt4_Fq tY2 = (tX2 + mnt4_G1::coeff_a) * tX + mnt4_G1::coeff_b;
// //         tY = tY2.sqrt();

// //         if (tY.as_bigint().0.0[0] & 1) != Y_lsb
// //         {
// //             tY = -tY;
// //         }
// //     }
// // 
// //     // using projective coordinates
// //     if is_zero == 0
// //     {
// //         g.X = tX;
// //         g.Y = tY;
// //         g.Z = mnt4_Fq::one();
// //     }
// //     else
// //     {
// //         g = mnt4_G1::zero();
// //     }

// //     return in;
// // }

// // std::ostream& operator<<(std::ostream& out, v:&Vec<mnt4_G1>)
// // {
// //     out << v.len() << "\n";
// //     for t in &v
// //     {
// //         out << t << OUTPUT_NEWLINE;
// //     }

// //     return out;
// // }

// // std::istream& operator>>(std::istream& in, Vec<mnt4_G1> &v)
// // {
// //     v.clear();

// //     usize s;
// //     in >> s;

// //     consume_newline(in);

// //     v.reserve(s);

// //     for i in 0..s
// //     {
// //         mnt4_G1 g;
// //         in >> g;
// //         consume_OUTPUT_NEWLINE(in);
// //         v.emplace_back(g);
// //     }

// //     return in;
// // }

use crate::algebra::curves::{
    AffineRepr, CurveGroup,
    mnt::mnt4::{MNT4, MNT4Config},
    short_weierstrass::{Affine, Projective},
};
use ffec::algebra::fields::{field::Field, prime_extension::fp2::Fp2};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::*;
use educe::Educe;

pub type G1Affine<P> = Affine<<P as MNT4Config>::G1Config>;
pub type G1Projective<P> = Projective<<P as MNT4Config>::G1Config>;

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Copy, Clone, Debug, PartialEq, Eq)]
pub struct G1Prepared<P: MNT4Config> {
    pub x: P::Fp,
    pub y: P::Fp,
    pub x_twist: Fp2<P::Fp2Config>,
    pub y_twist: Fp2<P::Fp2Config>,
}

impl<P: MNT4Config> From<G1Affine<P>> for G1Prepared<P> {
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

impl<'a, P: MNT4Config> From<&'a G1Affine<P>> for G1Prepared<P> {
    fn from(g1: &'a G1Affine<P>) -> Self {
        (*g1).into()
    }
}

impl<P: MNT4Config> From<G1Projective<P>> for G1Prepared<P> {
    fn from(g1: G1Projective<P>) -> Self {
        g1.into_affine().into()
    }
}
impl<'a, P: MNT4Config> From<&'a G1Projective<P>> for G1Prepared<P> {
    fn from(g1: &'a G1Projective<P>) -> Self {
        (*g1).into()
    }
}

impl<P: MNT4Config> Default for G1Prepared<P> {
    fn default() -> Self {
        Self::from(G1Affine::<P>::generator())
    }
}
