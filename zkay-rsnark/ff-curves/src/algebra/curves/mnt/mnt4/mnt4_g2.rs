//  Declaration of interfaces for the MNT4 G2 group.
use crate::algebra::curves::mnt::mnt4::mnt4_fields::{mnt4_Fq, mnt4_Fq2, mnt4_Fr};
use crate::algebra::curves::mnt::mnt4::mnt4_init::{
    mnt4_twist_coeff_a, mnt4_twist_coeff_b, mnt4_twist_mul_by_a_c0, mnt4_twist_mul_by_a_c1,
    mnt4_twist_mul_by_b_c0, mnt4_twist_mul_by_b_c1, mnt4_twist_mul_by_q_X, mnt4_twist_mul_by_q_Y,
};
use crate::{FpmConfig, Fq2mConfig};
use ffec::field_utils::field_utils::batch_invert;
use ffec::field_utils::{
    BigInt,
    bigint::{GMP_NUMB_BITS, bigint},
};
use ffec::{BigInt, Fp_model, Fp_modelConfig, One, PpConfig, Zero};
use num_bigint::BigUint;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

type base_field = mnt4_Fq;
type twist_field = mnt4_Fq2;
type scalar_field = mnt4_Fr;

#[derive(Clone, Debug, PartialEq)]
pub struct mnt4_G2 {
    pub X: mnt4_Fq2,
    pub Y: mnt4_Fq2,
    pub Z: mnt4_Fq2,
}
impl PpConfig for mnt4_G2 {
    type GType = Self;
}
impl FpmConfig for mnt4_G2 {
    type Fr = mnt4_Fq;
}
impl Fq2mConfig for mnt4_G2 {
    type Fr = Self;
}
impl mnt4_G2 {
    // Cofactor
    const h_bitcount: usize = 298;
    const h_limbs: usize = (Self::h_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
    const h: bigint<{ Self::h_limbs }> = bigint::<{ Self::h_limbs }>(BigInt!("1"));
    pub fn new(X: mnt4_Fq2, Y: mnt4_Fq2, Z: mnt4_Fq2) -> Self {
        Self { X, Y, Z }
    }
    pub fn size_in_bits() -> usize {
        return base_field::ceil_size_in_bits() + 1;
    }
    pub fn field_char() -> bigint<{ base_field::num_limbs }> {
        return base_field::field_char();
    }
    pub fn order() -> bigint<{ scalar_field::num_limbs }> {
        return scalar_field::field_char();
    }
}

impl Default for mnt4_G2 {
    fn default() -> Self {
        Self::G2_zero()
    }
}

impl mnt4_G2 {
    pub fn mul_by_a(elt: &mnt4_Fq2) -> mnt4_Fq2 {
        return mnt4_Fq2::new(
            mnt4_twist_mul_by_a_c0 * elt.c0,
            mnt4_twist_mul_by_a_c1 * elt.c1,
        );
    }

    pub fn mul_by_b(elt: &mnt4_Fq2) -> mnt4_Fq2 {
        return mnt4_Fq2::new(
            mnt4_twist_mul_by_b_c0 * elt.c1,
            mnt4_twist_mul_by_b_c1 * elt.c0,
        );
    }

    pub fn G2_zero() -> Self {
        Self {
            X: Default::default(),
            Y: Default::default(),
            Z: Default::default(),
        }
    }
    pub fn G2_one() -> Self {
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
                "({:N$}*z + {:N$} , {:N$}*z + {:N$})\n",
                copy.X.c1.as_bigint().0.0[0],
                copy.X.c0.as_bigint().0.0[0],
                copy.Y.c1.as_bigint().0.0[0],
                copy.Y.c0.as_bigint().0.0[0],
                N = mnt4_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$}*z + {:N$} : {:N$}*z + {:N$} : {:N$}*z + {:N$})\n",
                self.X.c1.as_bigint().0.0[0],
                self.X.c0.as_bigint().0.0[0],
                self.Y.c1.as_bigint().0.0[0],
                self.Y.c0.as_bigint().0.0[0],
                self.Z.c1.as_bigint().0.0[0],
                self.Z.c0.as_bigint().0.0[0],
                N = mnt4_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = mnt4_Fq2::zero();
            self.Y = mnt4_Fq2::one();
            self.Z = mnt4_Fq2::zero();
        } else {
            let Z_inv = self.Z.inverse();
            self.X = self.X * Z_inv;
            self.Y = self.Y * Z_inv;
            self.Z = mnt4_Fq2::one();
        }
    }

    pub fn to_special(&mut self) {
        self.to_affine_coordinates();
    }

    pub fn is_special(&self) -> bool {
        return (self.is_zero() || self.Z == mnt4_Fq2::one());
    }

    pub fn is_zero(&self) -> bool {
        return (self.X.is_zero() && self.Z.is_zero());
    }

    pub fn add(&self, other: &mnt4_G2) -> mnt4_G2 {
        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone();
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

        return mnt4_G2::new(X3, Y3, Z3);
    }

    pub fn mixed_add(&self, other: &mnt4_G2) -> mnt4_G2 {
        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt+=1;

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2
        //assert!(other.Z == mnt4_Fq2::one());

        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        // #ifdef DEBUG
        assert!(other.is_special());

        let X1Z2: mnt4_Fq2 = (self.X); // X1Z2 = X1*Z2 (but other is special and not zero)
        let X2Z1 = (self.Z) * (other.X); // X2Z1 = X2*Z1

        // (used both in add and double checks)

        let Y1Z2: mnt4_Fq2 = (self.Y); // Y1Z2 = Y1*Z2 (but other is special and not zero)
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

        return mnt4_G2::new(X3, Y3, Z3);
    }

    pub fn dbl(&self) -> mnt4_G2 {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt+=1;

        if self.is_zero() {
            return self.clone();
        }
        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#doubling-dbl-2007-bl

        let XX = (self.X).squared(); // XX  = X1^2
        let ZZ = (self.Z).squared(); // ZZ  = Z1^2
        let w = mnt4_G2::mul_by_a(&ZZ) + (XX + XX + XX); // w   = a*ZZ + 3*XX
        let Y1Z1 = (self.Y) * (self.Z);
        let s = Y1Z1 + Y1Z1; // s   = 2*Y1*Z1
        let ss = s.squared(); // ss  = s^2
        let sss = s * ss; // sss = s*ss
        let R = (self.Y) * s; // R   = Y1*s
        let RR = R.squared(); // RR  = R^2
        let B = ((self.X) + R).squared() - XX - RR; // B   = (X1+R)^2 - XX - RR
        let h = w.squared() - (B + B); // h   = w^2-2*B
        let X3 = h * s; // X3  = h*s
        let Y3 = w * (B - h) - (RR + RR); // Y3  = w*(B-h) - 2*RR
        let Z3 = sss; // Z3  = sss

        return mnt4_G2::new(X3, Y3, Z3);
    }

    pub fn mul_by_q(&self) -> mnt4_G2 {
        return mnt4_G2::new(
            (self.X).Frobenius_map(1) * mnt4_twist_mul_by_q_X,
            (self.Y).Frobenius_map(1) * mnt4_twist_mul_by_q_Y,
            (self.Z).Frobenius_map(1),
        );
    }

    pub fn mul_by_cofactor(&self) -> mnt4_G2 {
        self.clone() * mnt4_G2::h
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
        let aZ2 = mnt4_twist_coeff_a * Z2;

        return (self.Z * (Y2 - mnt4_twist_coeff_b * Z2) == self.X * (X2 + aZ2));
    }

    pub fn zero() -> Self {
        return Self::G2_zero();
    }

    pub fn one() -> Self {
        return Self::G2_one();
    }

    pub fn random_element() -> Self {
        Self::G2_one() * mnt4_Fr::random_element().as_bigint()
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<mnt4_G2>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z.clone());
        }
        batch_invert::<mnt4_Fq2>(&mut Z_vec);

        let one = mnt4_Fq2::one();

        for i in 0..vec.len() {
            vec[i] = mnt4_G2::new(vec[i].X * Z_vec[i], vec[i].Y * Z_vec[i], one);
        }
    }
}

impl Add<i32> for mnt4_G2 {
    type Output = mnt4_G2;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for mnt4_G2 {
    type Output = mnt4_G2;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for mnt4_G2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for mnt4_G2 {
    type Output = mnt4_G2;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for mnt4_G2 {
    type Output = mnt4_G2;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Mul<i32> for mnt4_G2 {
    type Output = mnt4_G2;

    fn mul(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<O: Borrow<Self>> Mul<O> for mnt4_G2 {
    type Output = mnt4_G2;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for mnt4_G2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for mnt4_G2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for mnt4_G2 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for mnt4_G2 {
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
// pub struct Mnt4G2 {
//     pub x: Fq2,
//     pub y: Fq2,
//     pub z: Fq2,
// }

// impl Mnt4G2 {
//     pub fn zero() -> Self {
//         Self {
//             x: Fq2::zero(),
//             y: Fq2::one(),
//             z: Fq2::zero(),
//         }
//     }

//     pub fn is_zero(&self) -> bool {
//         self.z.is_zero()
//     }

//     pub fn to_affine(&self) -> (Fq2, Fq2, bool) {
//         if self.is_zero() {
//             return (Fq2::zero(), Fq2::zero(), true);
//         }
//         let z_inv = self.z.inverse().unwrap();
//         (self.x * z_inv, self.y*z_inv, false)
//     }

//     fn mul_by_a(val: &Fq2) -> Fq2 {
//         val * &TWIST_COEFF_A
//     }
// }

// impl PartialEq for Mnt4G2 {
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

// impl<'a> Add<&'a Mnt4G2> for &'a Mnt4G2 {
//     type Output = Mnt4G2;
//     fn add(self, other: &'a Mnt4G2) -> Mnt4G2 {
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
//             let w = Mnt4G2::mul_by_a(&zz) + &xx.triple();
//             let y1z1 = self.y * &self.z;
//             let s = y1z1.double();
//             let ss = s.square();
//             let sss = s * &ss;
//             let r = self.y * &s;
//             let rr = r.square();
//             let b = (self.x + &r).square() - &xx - &rr;
//             let h = w.square() - &b.double();

//             return Mnt4G2 {
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

//         Mnt4G2 {
//             x: v * &a,
//             y: u * (r - &a) - &(vvv*y1z2),
//             z: vvv * &z1z2,
//         }
//     }
// }

// impl<'a> Mul<&'a Mnt4G2> for &'a BigInt {
//     type Output = Mnt4G2;
//     fn mul(self, rhs: &'a Mnt4G2) -> Mnt4G2 {
//         rhs.scalar_mul(self)
//     }
// }

// impl Neg for Mnt4G2 {
//     type Output = Self;
//     fn neg(mut self) -> Self {
//         self.y = -self.y;
//         self
//     }
// }

// impl<'a> Sub<&'a Mnt4G2> for &'a Mnt4G2 {
//     type Output = Mnt4G2;
//     fn sub(self, other: &'a Mnt4G2) -> Mnt4G2 {
//         self + &(-*other)
//     }
// }

// impl Mnt4G2 {
//     pub fn serialize<W: Write>(&self, mut writer: W, compress: bool) -> io::Result<()> {
//         let (x, y, is_zero) = self.to_affine();
//         writer.write_all(if is_zero { b"1" } else { b"0" })?;
//         writer.write_all(b" ")?;

//         writer.write_all(&x.to_bytes())?;
//         writer.write_all(b" ")?;

//         if compress {
//             let y_lsb = if y.c0.to_bigint().is_odd() {
//                 b"1"
//             } else {
//                 b"0"
//             };
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

//         let t_x = Fq2::read(&mut reader)?;
//         let t_y = if !compress {
//             Fq2::read(&mut reader)?
//         } else {
//             let x2 = t_x.square();
//             let y2 = (x2 + &TWIST_COEFF_A) * &t_x + &TWIST_COEFF_B;
//             let mut y = y2
//                 .sqrt()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "No sqrt"))?;

//             let mut lsb_buf = [0u8; 1];
//             reader.read_exact(&mut lsb_buf)?;

//             if y.c0.to_bigint().is_odd() != (lsb_buf == b'1') {
//                 y = -y;
//             }
//             y
//         };

//         Ok(Self {
//             x: t_x,
//             y: t_y,
//             z: Fq2::one(),
//         })
//     }
// }

use crate::algebra::curves::{
    AffineRepr, CurveGroup,
    mnt::mnt4::{MNT4, MNT4Config},
    short_weierstrass::{Affine, Projective},
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{vec, vec::*};
use educe::Educe;
use ffec::algebra::fields::{field::Field, prime_extension::fp2::Fp2};

pub type G2Affine<P> = Affine<<P as MNT4Config>::G2Config>;
pub type G2Projective<P> = Projective<<P as MNT4Config>::G2Config>;

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Clone, Debug, PartialEq, Eq)]
pub struct G2Prepared<P: MNT4Config> {
    pub x: Fp2<P::Fp2Config>,
    pub y: Fp2<P::Fp2Config>,
    pub x_over_twist: Fp2<P::Fp2Config>,
    pub y_over_twist: Fp2<P::Fp2Config>,
    pub double_coefficients: Vec<AteDoubleCoefficients<P>>,
    pub addition_coefficients: Vec<AteAdditionCoefficients<P>>,
}

impl<P: MNT4Config> Default for G2Prepared<P> {
    fn default() -> Self {
        Self::from(G2Affine::<P>::generator())
    }
}

impl<P: MNT4Config> From<G2Affine<P>> for G2Prepared<P> {
    fn from(g: G2Affine<P>) -> Self {
        let twist_inv = P::TWIST.inverse().unwrap();

        let mut g_prep = G2Prepared {
            x: g.x,
            y: g.y,
            x_over_twist: g.x * &twist_inv,
            y_over_twist: g.y * &twist_inv,
            double_coefficients: vec![],
            addition_coefficients: vec![],
        };

        let mut r = G2ProjectiveExtended {
            x: g.x,
            y: g.y,
            z: <Fp2<P::Fp2Config>>::one(),
            t: <Fp2<P::Fp2Config>>::one(),
        };

        let neg_g = -g;
        for bit in P::ATE_LOOP_COUNT.iter().skip(1) {
            let (r2, coeff) = MNT4::doubling_for_flipped_miller_loop(&r);
            g_prep.double_coefficients.push(coeff);
            r = r2;

            let (r_temp, add_coeff) = match bit {
                1 => MNT4::mixed_addition_for_flipped_miller_loop(&g.x, &g.y, &r),
                -1 => MNT4::mixed_addition_for_flipped_miller_loop(&neg_g.x, &neg_g.y, &r),
                0 => continue,
                _ => unreachable!(),
            };
            g_prep.addition_coefficients.push(add_coeff);
            r = r_temp;
        }

        if P::ATE_IS_LOOP_COUNT_NEG {
            let rz_inv = r.z.inverse().unwrap();
            let rz2_inv = rz_inv.square();
            let rz3_inv = rz_inv * &rz2_inv;

            let minus_r_affine_x = r.x * &rz2_inv;
            let minus_r_affine_y = -r.y * &rz3_inv;

            let add_result = MNT4::mixed_addition_for_flipped_miller_loop(
                &minus_r_affine_x,
                &minus_r_affine_y,
                &r,
            );
            g_prep.addition_coefficients.push(add_result.1);
        }

        g_prep
    }
}

impl<'a, P: MNT4Config> From<&'a G2Affine<P>> for G2Prepared<P> {
    fn from(g2: &'a G2Affine<P>) -> Self {
        (*g2).into()
    }
}

impl<P: MNT4Config> From<G2Projective<P>> for G2Prepared<P> {
    fn from(g2: G2Projective<P>) -> Self {
        g2.into_affine().into()
    }
}
impl<'a, P: MNT4Config> From<&'a G2Projective<P>> for G2Prepared<P> {
    fn from(g2: &'a G2Projective<P>) -> Self {
        (*g2).into()
    }
}

pub struct G2ProjectiveExtended<P: MNT4Config> {
    pub x: Fp2<P::Fp2Config>,
    pub y: Fp2<P::Fp2Config>,
    pub z: Fp2<P::Fp2Config>,
    pub t: Fp2<P::Fp2Config>,
}

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Clone, Debug, PartialEq, Eq)]
pub struct AteDoubleCoefficients<P: MNT4Config> {
    pub c_h: Fp2<P::Fp2Config>,
    pub c_4c: Fp2<P::Fp2Config>,
    pub c_j: Fp2<P::Fp2Config>,
    pub c_l: Fp2<P::Fp2Config>,
}

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Clone, Debug, PartialEq, Eq)]
pub struct AteAdditionCoefficients<P: MNT4Config> {
    pub c_l1: Fp2<P::Fp2Config>,
    pub c_rz: Fp2<P::Fp2Config>,
}
