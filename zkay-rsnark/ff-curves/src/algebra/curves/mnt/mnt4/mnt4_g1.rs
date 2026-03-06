//  Declaration of interfaces for the MNT4 G1 group.

use crate::FpmConfig;
use crate::algebra::curves::mnt::mnt4::mnt4_fields::{mnt4_Fq, mnt4_Fr};
use ffec::field_utils::field_utils::batch_invert;
use ffec::field_utils::{BigInt, bigint::bigint};
use ffec::{BigInt, Fp_model, Fp_modelConfig, One, PpConfig, Zero};
use num_bigint::BigUint;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const mnt4_Fq_num_limbs: usize = 5;
const mnt4_Fr_num_limbs: usize = 5;
type base_field = mnt4_Fq;
type scalar_field = mnt4_Fr;

#[derive(Clone, Debug, PartialEq)]
pub struct mnt4_G1 {
    pub X: mnt4_Fq,
    pub Y: mnt4_Fq,
    pub Z: mnt4_Fq,
}
impl PpConfig for mnt4_G1 {
    type TT = bigint<1>;
    // type Fr=Self;
}
impl FpmConfig for mnt4_G1 {
    type Fr = mnt4_Fq;
}
impl mnt4_G1 {
    //     // using projective coordinates
    //     mnt4_G1::new();
    pub fn new_inv(X: mnt4_Fq, Y: mnt4_Fq) -> Self {
        Self {
            X,
            Y,
            Z: base_field::one(),
        }
    }
    pub fn new(X: mnt4_Fq, Y: mnt4_Fq, Z: mnt4_Fq) -> Self {
        Self { X, Y, Z }
    }

    pub fn size_in_bits() -> usize {
        return mnt4_Fq::ceil_size_in_bits() + 1;
    }
    pub fn field_char() -> bigint<{ base_field::num_limbs }> {
        return base_field::field_char();
    }
    pub fn order() -> bigint<{ scalar_field::num_limbs }> {
        return scalar_field::field_char();
    }
}

impl Default for mnt4_G1 {
    fn default() -> Self {
        Self::G1_zero()
    }
}
impl mnt4_G1 {
    pub fn G1_zero() -> Self {
        Self {
            X: Default::default(),
            Y: Default::default(),
            Z: Default::default(),
        }
    }
    pub fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            print!(
                "({:N$} , {:N$})\n",
                copy.X.as_bigint().0.0[0],
                copy.Y.as_bigint().0.0[0],
                N = mnt4_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$}: {:N$}: {:N$})\n",
                self.X.as_bigint().0.0[0],
                self.Y.as_bigint().0.0[0],
                self.Z.as_bigint().0.0[0],
                N = mnt4_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = mnt4_Fq::zero();
            self.Y = mnt4_Fq::one();
            self.Z = mnt4_Fq::zero();
        } else {
            let Z_inv = self.Z.inverse();
            self.X = self.X.clone() * Z_inv;
            self.Y = self.Y.clone() * Z_inv;
            self.Z = mnt4_Fq::one();
        }
    }

    pub fn to_special(&mut self) {
        self.to_affine_coordinates();
    }

    pub fn is_special(&self) -> bool {
        return (self.is_zero() || self.Z == mnt4_Fq::one());
    }

    pub fn is_zero(&self) -> bool {
        return (self.X.is_zero() && self.Z.is_zero());
    }

    pub fn add(&self, other: &Self) -> mnt4_G1 {
        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone()
        }

        if other.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // handle double case
        if self == (other) {
            return self.dbl();
        }

        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt+=1;

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2

        let Y1Z2 = (self.Y) * (other.Z); // Y1Z2 = Y1*Z2
        let X1Z2 = (self.X) * (other.Z); // X1Z2 = X1*Z2
        let Z1Z2 = (self.Z) * (other.Z); // Z1Z2 = Z1*Z2
        let u = (other.Y) * (self.Z) - Y1Z2; // u    = Y2*Z1-Y1Z2
        let uu = u.squared(); // uu   = u^2
        let v = (other.X) * (self.Z) - X1Z2; // v    = X2*Z1-X1Z2
        let vv = v.squared(); // vv   = v^2
        let vvv = v * vv; // vvv  = v*vv
        let R = vv * X1Z2; // R    = vv*X1Z2
        let A = uu * Z1Z2 - (vvv + R + R); // A    = uu*Z1Z2 - vvv - 2*R
        let X3 = v * A; // X3   = v*A
        let Y3 = u * (R - A) - vvv * Y1Z2; // Y3   = u*(R-A) - vvv*Y1Z2
        let Z3 = vvv * Z1Z2; // Z3   = vvv*Z1Z2

        return mnt4_G1::new(X3, Y3, Z3);
    }

    pub fn mixed_add(&self, other: &mnt4_G1) -> mnt4_G1 {
        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt+=1;

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2
        //assert!(other.Z == mnt4_Fq::one());

        if self.is_zero() {
            return other.clone()
        }

        if other.is_zero() {
            return self.clone();
        }

        // #ifdef DEBUG
        assert!(other.is_special());

        let X1Z2: mnt4_Fq = (self.X); // X1Z2 = X1*Z2 (but other is special and not zero)
        let X2Z1 = (self.Z) * (other.X); // X2Z1 = X2*Z1

        // (used both in add and double checks)

        let Y1Z2: mnt4_Fq = (self.Y); // Y1Z2 = Y1*Z2 (but other is special and not zero)
        let Y2Z1 = (self.Z) * (other.Y); // Y2Z1 = Y2*Z1

        if X1Z2 == X2Z1 && Y1Z2 == Y2Z1 {
            return self.dbl();
        }

        let u = Y2Z1 - self.Y; // u = Y2*Z1-Y1
        let uu = u.squared(); // uu = u2
        let v = X2Z1 - self.X; // v = X2*Z1-X1
        let vv = v.squared(); // vv = v2
        let vvv = v * vv; // vvv = v*vv
        let R = vv * self.X; // R = vv*X1
        let A = uu * self.Z - vvv - R - R; // A = uu*Z1-vvv-2*R
        let X3 = v * A; // X3 = v*A
        let Y3 = u * (R - A) - vvv * self.Y; // Y3 = u*(R-A)-vvv*Y1
        let Z3 = vvv * self.Z; // Z3 = vvv*Z1

        return mnt4_G1::new(X3, Y3, Z3);
    }

    pub fn dbl(&self) -> mnt4_G1 {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt+=1;

        if self.is_zero() {
            return self.clone();
        }
        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#doubling-dbl-2007-bl

        let XX = (self.X).squared(); // XX  = X1^2
        let ZZ = (self.Z).squared(); // ZZ  = Z1^2
        let w :mnt4_Fq=   ZZ*mnt4_G1::coeff_a + (XX + XX + XX); // w   = a*ZZ + 3*XX
        let Y1Z1 = (self.Y) * (self.Z);
        let s = Y1Z1 + Y1Z1; // s   = 2*Y1*Z1
        let ss = s.squared(); // ss  = s^2
        let sss = s * ss; // sss = s*ss
        let R = (self.Y) * s; // R   = Y1*s
        let RR = R.squared(); // RR  = R^2
        let B = ((self.X) + R).squared() - XX - RR; // B   = (X1+R)^2 - XX - RR
        let h = w.squared() - (B + B); // h   = w^2 - 2*B
        let X3 = h * s; // X3  = h*s
        let Y3 = w * (B - h) - (RR + RR); // Y3  = w*(B-h) - 2*RR
        let Z3 = sss; // Z3  = sss

        return mnt4_G1::new(X3, Y3, Z3);
    }

    pub fn mul_by_cofactor(&self) -> mnt4_G1 {
        // Cofactor = 1
        return self.clone();
    }

    pub fn is_well_formed(&self) -> bool {
        if self.is_zero() {
            return true;
        }
        /*
            y^2 = x^3 + ax + b

            We are using projective, so equation we need to check is actually

            (y/z)^2 = (x/z)^3 + a (x/z) + b
            z y^2 = x^3  + a z^2 x + b z^3

            z (y^2 - b z^2) = x ( x^2 + a z^2)
        */
        let X2 = self.X.squared();
        let Y2 = self.Y.squared();
        let Z2 = self.Z.squared();

        (self.Z * (Y2 -  Z2*mnt4_G1::coeff_b ) == self.X * (X2 +   Z2*mnt4_G1::coeff_a))
    }

    pub fn zero() -> mnt4_G1 {
        return Self::G1_zero();
    }

    pub fn one() -> mnt4_G1 {
        return Self::G1_one();
    }

    pub fn G1_one() -> mnt4_G1 {
        Self::default()
    }
    pub fn random_element() -> mnt4_G1 {
        Self::G1_one()*scalar_field::random_element().as_bigint()
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<mnt4_G1>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z.clone());
        }
        batch_invert::<mnt4_Fq>(&mut Z_vec);

        let mut one = mnt4_Fq::one();

        for i in 0..vec.len() {
            vec[i] = mnt4_G1::new(vec[i].X * Z_vec[i], vec[i].Y * Z_vec[i], one);
        }
    }
}

impl Add<i32> for mnt4_G1 {
    type Output = mnt4_G1;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for mnt4_G1 {
    type Output = mnt4_G1;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for mnt4_G1 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for mnt4_G1 {
    type Output = mnt4_G1;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for mnt4_G1 {
    type Output = mnt4_G1;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Mul<i32> for mnt4_G1 {
    type Output = mnt4_G1;

    fn mul(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<O: Borrow<Self>> Mul<O> for mnt4_G1 {
    type Output = mnt4_G1;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for mnt4_G1 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for mnt4_G1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for mnt4_G1 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for mnt4_G1 {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
// use std::io::{self, Read, Write};
// use std::ops::{Add, Mul, Neg, Sub};

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Mnt4G1 {
//     pub x: Fq,
//     pub y: Fq,
//     pub z: Fq,
// }

// impl Mnt4G1 {
//     pub fn zero() -> Self {
//         Self {
//             x: Fq::zero(),
//             y: Fq::one(),
//             z: Fq::zero(),
//         }
//     }

//     pub fn is_zero(&self) -> bool {
//         self.z.is_zero()
//     }

//     pub fn to_affine(&self) -> (Fq, Fq, bool) {
//         if self.is_zero() {
//             return (Fq::zero(), Fq::zero(), true);
//         }
//         let z_inv = self.z.inverse().unwrap();
//         (self.x * &z_inv, self.y * &z_inv, false)
//     }
// }

// impl PartialEq for Mnt4G1 {
//     fn eq(&self, other: &Self) -> bool {
//         if self.is_zero() {
//             return other.is_zero();
//         }
//         if other.is_zero() {
//             return false;
//         }

//         (self.x * &other.z == other.x * &self.z) && (self.y * &other.z == other.y * &self.z)
//     }
// }

// impl<'a> Add<&'a Mnt4G1> for &'a Mnt4G1 {
//     type Output = Mnt4G1;
//     fn add(self, other: &'a Mnt4G1) -> Mnt4G1 {
//         if self.is_zero() {
//             return *other;
//         }
//         if other.is_zero() {
//             return *self;
//         }

//         let x1z2 = self.x * &other.z;
//         let x2z1 = self.z * &other.x;
//         let y1z2 = self.y * &other.z;
//         let y2z1 = self.z * &other.y;

//         if x1z2 == x2z1 && y1z2 == y2z1 {
//             let xx = self.x.square();
//             let zz = self.z.square();
//             let w = (COEFF_A * &zz) + &xx.triple();
//             let y1z1 = self.y * &self.z;
//             let s = y1z1.double();
//             let ss = s.square();
//             let sss = s * &ss;
//             let r = self.y * &s;
//             let rr = r.square();
//             let b = (self.x + &r).square() - &xx - &rr;
//             let h = w.square() - &b.double();

//             return Mnt4G1 {
//                 x: h * &s,
//                 y: w * &(b - &h) - &rr.double(),
//                 z: sss,
//             };
//         }

//         let z1z2 = self.z * &other.z;
//         let u = y2z1 - &y1z2;
//         let v = x2z1 - &x1z2;
//         let vv = v.square();
//         let vvv = v * &vv;
//         let r = vv * &x1z2;
//         let a = u.square() * &z1z2 - &vvv - &r.double();

//         Mnt4G1 {
//             x: v * &a,
//             y: u * &(r - &a) - &(vvv * &y1z2),
//             z: vvv * &z1z2,
//         }
//     }
// }

// impl<'a> Mul<&'a Mnt4G1> for &'a BigInt {
//     type Output = Mnt4G1;
//     fn mul(self, rhs: &'a Mnt4G1) -> Mnt4G1 {
//         rhs.scalar_mul(self)
//     }
// }

// impl Neg for Mnt4G1 {
//     type Output = Self;
//     fn neg(mut self) -> Self {
//         self.y = -self.y;
//         self
//     }
// }

// impl<'a> Sub<&'a Mnt4G1> for &'a Mnt4G1 {
//     type Output = Mnt4G1;
//     fn sub(self, other: &'a Mnt4G1) -> Mnt4G1 {
//         self + &(-*other)
//     }
// }

// impl Mnt4G1 {
//     pub fn serialize<W: Write>(&self, mut writer: W, compress: bool) -> io::Result<()> {
//         let (x, y, is_zero) = self.to_affine();
//         writer.write_all(if is_zero { b"1" } else { b"0" })?;
//         writer.write_all(b" ")?;

//         writer.write_all(&x.to_bytes())?;
//         writer.write_all(b" ")?;
//         if compress {
//             let y_lsb = if y.to_bigint().is_odd() { b"1" } else { b"0" };
//             writer.write_all(y_lsb)?;
//         } else {
//             writer.write_all(&y.to_bytes())?;
//         }
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R, compress: bool) -> io::Result<Self> {
//         let mut zero_buf = [0u8; 1];
//         reader.read_exact(&mut zero_buf)?;
//         let is_zero = zero_buf == b'1';

//         if is_zero {
//             return Ok(Self::zero());
//         }

//         let t_x = Fq::read(&mut reader)?;
//         let t_y = if !compress {
//             Fq::read(&mut reader)?
//         } else {
//             let x2 = t_x.square();
//             let y2 = (x2 + &COEFF_A) * &t_x + &COEFF_B;
//             let mut y = y2
//                 .sqrt()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "No sqrt"))?;

//             let mut lsb_buf = [0u8; 1];
//             reader.read_exact(&mut lsb_buf)?;
//             if y.to_bigint().is_odd() != (lsb_buf == b'1') {
//                 y = -y;
//             }
//             y
//         };

//         Ok(Self {
//             x: t_x,
//             y: t_y,
//             z: Fq::one(),
//         })
//     }
// }

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
