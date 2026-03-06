use crate::algebra::curves::edwards::edwards_fields::{edwards_Fq, edwards_Fq3, edwards_Fr};

use crate::algebra::curves::edwards::edwards_init::{
    edwards_twist_mul_by_a_c0, edwards_twist_mul_by_d_c0, edwards_twist_mul_by_d_c1,
    edwards_twist_mul_by_d_c2, edwards_twist_mul_by_q_Y, edwards_twist_mul_by_q_Z,
};
use crate::{FpmConfig, Fq2mConfig};
use ffec::field_utils::field_utils::batch_invert;
use ffec::field_utils::{BigInt, bigint::bigint};
use ffec::{BigInt, Fp_model, Fp_modelConfig, One, PpConfig, Zero};
use num_bigint::BigUint;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct edwards_G2 {
    pub X: edwards_Fq3,
    pub Y: edwards_Fq3,
    pub Z: edwards_Fq3,
}
impl PpConfig for edwards_G2 {
    type TT = bigint<1>;
    // type Fr=Self;
}
impl FpmConfig for edwards_G2 {
    type Fr = edwards_Fq;
}
impl Fq2mConfig for edwards_G2 {
    type Fr = Self;
}
type base_field = edwards_Fq;
type twist_field = edwards_Fq3;
type scalar_field = edwards_Fr;
impl edwards_G2 {
    pub fn new(X: edwards_Fq3, Y: edwards_Fq3, Z: edwards_Fq3) -> Self {
        Self { X, Y, Z }
    }

    // using inverted coordinates
    pub fn new_inv(X: edwards_Fq3, Y: edwards_Fq3) -> Self {
        Self {
            X: Y.clone(),
            Y: X.clone(),
            Z: X * Y,
        }
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

impl Default for edwards_G2 {
    fn default() -> Self {
        Self::G2_zero()
    }
}
impl edwards_G2 {
    pub fn mul_by_a(elt: &edwards_Fq3) -> edwards_Fq3 {
        // should be
        //  edwards_Fq3(edwards_twist_mul_by_a_c0 * elt.c2, edwards_twist_mul_by_a_c1 * elt.c0, edwards_twist_mul_by_a_c2 * elt.c1)
        // but optimizing the fact that edwards_twist_mul_by_a_c1 = edwards_twist_mul_by_a_c2 = 1
        return edwards_Fq3::new(edwards_twist_mul_by_a_c0 * elt.c2, elt.c0, elt.c1);
    }

    pub fn mul_by_d(elt: &edwards_Fq3) -> edwards_Fq3 {
        return edwards_Fq3::new(
            edwards_twist_mul_by_d_c0 * elt.c2,
            edwards_twist_mul_by_d_c1 * elt.c0,
            edwards_twist_mul_by_d_c2 * elt.c1,
        );
    }

    pub fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            print!(
                "({:N$}*z^2 + {:N$}*z + {:N$} , {:N$}*z^2 + {:N$}*z + {:N$})\n",
                copy.X.c2.as_bigint().0.0[0],
                copy.X.c1.as_bigint().0.0[0],
                copy.X.c0.as_bigint().0.0[0],
                copy.Y.c2.as_bigint().0.0[0],
                copy.Y.c1.as_bigint().0.0[0],
                copy.Y.c0.as_bigint().0.0[0],
                N = edwards_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$}*z^2 + {:N$}*z + {:N$} : {:N$}*z^2 + {:N$}*z + {:N$} : {:N$}*z^2 + {:N$}*z + {:N$})\n",
                self.X.c2.as_bigint().0.0[0],
                self.X.c1.as_bigint().0.0[0],
                self.X.c0.as_bigint().0.0[0],
                self.Y.c2.as_bigint().0.0[0],
                self.Y.c1.as_bigint().0.0[0],
                self.Y.c0.as_bigint().0.0[0],
                self.Z.c2.as_bigint().0.0[0],
                self.Z.c1.as_bigint().0.0[0],
                self.Z.c0.as_bigint().0.0[0],
                N = edwards_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = edwards_Fq3::zero();
            self.Y = edwards_Fq3::one();
            self.Z = edwards_Fq3::one();
        } else {
            // go from inverted coordinates to projective coordinates
            let tX = self.Y * self.Z;
            let tY = self.X * self.Z;
            let tZ = self.X * self.Y;
            // go from projective coordinates to affine coordinates
            let tZ_inv = tZ.inverse();
            self.X = tX * tZ_inv;
            self.Y = tY * tZ_inv;
            self.Z = edwards_Fq3::one();
        }
    }

    pub fn to_special(&mut self) {
        if self.Z.is_zero() {
            return;
        }

        // #ifdef DEBUG
        let copy = self.clone();

        let Z_inv = self.Z.inverse();
        self.X = self.X * Z_inv;
        self.Y = self.Y * Z_inv;
        self.Z = edwards_Fq3::one();

        // #ifdef DEBUG
        assert!(self.clone() == copy);
    }

    pub fn is_special(&self) -> bool {
        return (self.is_zero() || self.Z == edwards_Fq3::one());
    }

    pub fn is_zero(&self) -> bool {
        return (self.Y.is_zero() && self.Z.is_zero());
    }

    pub fn add(&mut self, other: &edwards_G2) -> edwards_G2 {
        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt += 1;

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-twisted-inverted.html#addition-add-2008-bbjlp

        let A = (self.Z) * (other.Z); // A = Z1*Z2
        let B = edwards_G2::mul_by_d(&A.squared()); // B = d*A^2
        let C = (self.X) * (other.X); // C = X1*X2
        let D = (self.Y) * (other.Y); // D = Y1*Y2
        let E = C * D; // E = C*D
        let H = C - edwards_G2::mul_by_a(&D); // H = C-a*D
        let I = (self.X + self.Y) * (other.X + other.Y) - C - D; // I = (X1+Y1)*(X2+Y2)-C-D
        let X3 = (E + B) * H; // X3 = (E+B)*H
        let Y3 = (E - B) * I; // Y3 = (E-B)*I
        let Z3 = A * H * I; // Z3 = A*H*I

        return edwards_G2::new(X3, Y3, Z3);
    }

    pub fn mixed_add(&self, other: &edwards_G2) -> edwards_G2 {
        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt += 1;

        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        // #ifdef DEBUG
        assert!(other.is_special());

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-edwards-inverted.html#addition-madd-2007-lb

        let A = self.Z; // A = Z1*Z2
        let B = edwards_G2::mul_by_d(&A.squared()); // B = d*A^2
        let C = (self.X) * (other.X); // C = X1*X2
        let D = (self.Y) * (other.Y); // D = Y1*Y2
        let E = C * D; // E = C*D
        let H = C - edwards_G2::mul_by_a(&D); // H = C-a*D
        let I = (self.X + self.Y) * (other.X + other.Y) - C - D; // I = (X1+Y1)*(X2+Y2)-C-D
        let X3 = (E + B) * H; // X3 = (E+B)*H
        let Y3 = (E - B) * I; // Y3 = (E-B)*I
        let Z3 = A * H * I; // Z3 = A*H*I

        return edwards_G2::new(X3, Y3, Z3);
    }

    pub fn dbl(&self) -> edwards_G2 {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt += 1;

        if self.is_zero() {
            return self.clone();
        }
        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-twisted-inverted.html#doubling-dbl-2008-bbjlp

        let A = (self.X).squared(); // A = X1^2
        let B = (self.Y).squared(); // B = Y1^2
        let U = edwards_G2::mul_by_a(&B); // U = a*B
        let C = A + U; // C = A+U
        let D = A - U; // D = A-U
        let E = (self.X + self.Y).squared() - A - B; // E = (X1+Y1)^2-A-B
        let X3 = C * D; // X3 = C*D
        let dZZ = edwards_G2::mul_by_d(&self.Z.squared());
        let Y3 = E * (C - dZZ - dZZ); // Y3 = E*(C-2*d*Z1^2)
        let Z3 = D * E; // Z3 = D*E

        return edwards_G2::new(X3, Y3, Z3);
    }

    pub fn mul_by_q(&self) -> edwards_G2 {
        return edwards_G2::new(
            (self.X).Frobenius_map(1),
(self.Y).Frobenius_map(1)*            edwards_twist_mul_by_q_Y.clone(),
(self.Z).Frobenius_map(1)*            edwards_twist_mul_by_q_Z.clone(),
        );
    }

    pub fn is_well_formed(&self) -> bool {
        /* Note that point at infinity is the only special case we must check as
        inverted representation does no cover points (0, +-c) and (+-c, 0). */
        if self.is_zero() {
            return true;
        }
        /*
            a x^2 + y^2 = 1 + d x^2 y^2

            We are using inverted, so equation we need to check is actually

            a (z/x)^2 + (z/y)^2 = 1 + d z^4 / (x^2 * y^2)
            z^2 (a y^2 + x^2 - dz^2) = x^2 y^2
        */
        let X2 = self.X.squared();
        let Y2 = self.Y.squared();
        let Z2 = self.Z.squared();
        let aY2 = edwards_G2::mul_by_a(&Y2);
        let dZ2 = edwards_G2::mul_by_d(&Z2);
        return (Z2 * (aY2 + X2 - dZ2) == X2 * Y2);
    }

    pub fn zero() -> Self {
        return Self::G2_zero();
    }

    pub fn one() -> Self {
        return Self::G2_one();
    }
    pub fn G2_one() -> Self {
        Self {
            X: Default::default(),
            Y: Default::default(),
            Z: Default::default(),
        }
    }
    pub fn G2_zero() -> Self {
        Self {
            X: Default::default(),
            Y: Default::default(),
            Z: Default::default(),
        }
    }
    pub fn random_element() -> Self {
        Self::G2_one() * edwards_Fr::random_element().as_bigint()
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<edwards_G2>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z.clone());
        }
        batch_invert::<edwards_Fq3>(&mut Z_vec);

        let one = edwards_Fq3::one();

        for i in 0..vec.len() {
            vec[i].X = vec[i].X * Z_vec[i];
            vec[i].Y = vec[i].Y * Z_vec[i];
            vec[i].Z = one;
        }
    }
}

impl Add<i32> for edwards_G2 {
    type Output = edwards_G2;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for edwards_G2 {
    type Output = edwards_G2;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for edwards_G2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for edwards_G2 {
    type Output = edwards_G2;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for edwards_G2 {
    type Output = edwards_G2;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Mul<i32> for edwards_G2 {
    type Output = edwards_G2;

    fn mul(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<O: Borrow<Self>> Mul<O> for edwards_G2 {
    type Output = edwards_G2;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for edwards_G2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for edwards_G2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}
impl One for edwards_G2 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for edwards_G2 {
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
// pub struct EdwardsG2 {
//     pub x: Fq3,
//     pub y: Fq3,
//     pub z: Fq3,
// }

// impl EdwardsG2 {
//     pub fn zero() -> Self {

//         Self {
//             x: Fq3::zero(),
//             y: Fq3::one(),
//             z: Fq3::one(),
//         }
//     }

//     pub fn is_zero(&self) -> bool {
//         self.x.is_zero() && (self.y == self.z)
//     }

//     pub fn to_affine(&self) -> (Fq3, Fq3) {
//         let z_inv = self.z.inverse().expect("Division by zero");
//         (self.x * &z_inv, self.y * &z_inv)
//     }

//     fn mul_by_a(val: Fq3) -> Fq3 {
//
//         todo!()
//     }
//     fn mul_by_d(val: Fq3) -> Fq3 {
//
//         todo!()
//     }

//     pub fn add_logic(&self, other: &Self) -> Self {
//
//         todo!()
//     }
// }

// impl<'a> Mul<&'a EdwardsG2> for &'a BigInt {
//     type Output = EdwardsG2;
//     fn mul(self, rhs: &'a EdwardsG2) -> EdwardsG2 {
//         rhs.scalar_mul(self)
//     }
// }

// impl PartialEq for EdwardsG2 {
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

// impl<'a> Add<&'a EdwardsG2> for &'a EdwardsG2 {
//     type Output = EdwardsG2;
//     fn add(self, other: &'a EdwardsG2) -> EdwardsG2 {
//         if self.is_zero() {
//             return *other;
//         }
//         if other.is_zero() {
//             return *self;
//         }
//         self.add_logic(other)
//     }
// }

// impl Neg for EdwardsG2 {
//     type Output = Self;
//     fn neg(mut self) -> Self {
//         self.x = -self.x;
//         self
//     }
// }

// impl<'a> Sub<&'a EdwardsG2> for &'a EdwardsG2 {
//     type Output = EdwardsG2;
//     fn sub(self, other: &'a EdwardsG2) -> EdwardsG2 {
//         self + &(-*other)
//     }
// }

// impl EdwardsG2 {
//     pub fn serialize<W: Write>(&self, mut writer: W, compress: bool) -> io::Result<()> {
//         let (x, y) = self.to_affine();

//         if !compress {
//             writer.write_all(&x.to_bytes())?;
//             writer.write_all(b" ")?;
//             writer.write_all(&y.to_bytes())?;
//         } else {
//             writer.write_all(&x.to_bytes())?;
//             writer.write_all(b" ")?;

//             let y_lsb = if y.c0.to_bigint().is_odd() {
//                 b"1"
//             } else {
//                 b"0"
//             };
//             writer.write_all(y_lsb)?;
//         }
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R, compress: bool) -> io::Result<Self> {
//         if !compress {
//             let x = Fq3::read(&mut reader)?;
//             let y = Fq3::read(&mut reader)?;
//             Ok(Self {
//                 x,
//                 y,
//                 z: Fq3::one(),
//             })
//         } else {
//             let t_x = Fq3::read(&mut reader)?;
//             let mut lsb_buf = [0u8; 1];
//             reader.read_exact(&mut lsb_buf)?;
//             let y_lsb = lsb_buf == b'1';

//             let x2 = t_x.square();
//             let num = Fq3::one() - &Self::mul_by_a(x2);
//             let den = (Fq3::one() - &Self::mul_by_d(x2))
//                 .inverse()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid X"))?;

//             let mut t_y = (num * &den)
//                 .sqrt()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "No sqrt"))?;

//             if t_y.c0.to_bigint().is_odd() != y_lsb {
//                 t_y = -t_y;
//             }

//             Ok(Self {
//                 x: t_y,
//                 y: t_x,
//                 z: t_x * &t_y,
//             })
//         }
//     }
// }
