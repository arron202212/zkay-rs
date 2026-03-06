use crate::FpmConfig;
use crate::algebra::curves::bn128::bn128_fields::{Fp, Fp2, Fp6, Fp12};
use crate::algebra::curves::bn128::bn128_fields::{bn128_Fq, bn128_Fr};
use ffec::field_utils::{BigInt, bigint::{bigint,GMP_NUMB_BITS}};
use ffec::{BigInt, Fp_model, Fp_modelConfig, One, PpConfig, Zero};
use num_bigint::BigUint;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const bn128_Fq_s: usize = 1;
const bn128_Fq2_s: usize = 1;
const bn128_Fq_nqr_to_t: bn128_Fq = bn128_Fq::const_new(BigInt!("0"));
const bn128_Fq_t_minus_1_over_2: bn128_Fq = bn128_Fq::const_new(BigInt!("0"));
const bn128_coeff_b: bn128_Fq = bn128_Fq::const_new(BigInt!("0"));
const BN128_COEFF_B: bn128_Fq = bn128_Fq::const_new(BigInt!("0"));




type base_field = bn128_Fq;
type scalar_field = bn128_Fr;

#[derive(Clone, Debug, PartialEq)]
pub struct bn128_G1 {
    X: Fp,
    Y: Fp,
    Z: Fp,
}

impl PpConfig for bn128_G1 {
    type TT = bigint<1>;
    // type Fr=Self;
}
impl FpmConfig for bn128_G1 {
    type Fr = bn128_Fq;
}
impl bn128_G1 {
    const  h_bitcount:usize = 1;
     const  h_limbs:usize = (Self::h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
    const  h:bigint<{Self::h_limbs}>=bigint::<{Self::h_limbs}>(BigInt!("1"));
    pub fn fill_coord(&self, coord: &mut [Fp; 3]) {
        coord[0] = self.X;
        coord[1] = self.Y;
        coord[2] = self.Z;
    }

    pub fn new(coord: [Fp; 3]) -> Self {
        Self {
            X: coord[0],
            Y: coord[1],
            Z: coord[2],
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
impl Default for bn128_G1 {
    fn default() -> Self {
        Self::G1_zero()
    }
}

impl bn128_G1 {
    pub fn G1_zero() -> Self {
        Self {
            X: Default::default(),
            Y: Default::default(),
            Z: Default::default(),
        }
    }
    pub fn G1_one() -> Self {
        Self {
            X: Default::default(),
            Y: Default::default(),
            Z: Default::default(),
        }
    }
    pub fn sqrt(el: &Fp) -> Fp {
        let mut v = bn128_Fq_s;
        let mut z = bn128_Fq_nqr_to_t;
        let mut w = el.clone(); //power(el, bn128_Fq_t_minus_1_over_2);
        let mut x = el.clone() * w;
        let mut b = x * w;

        // check if square with Euler's criterion
        let mut check = b;
        for i in 0..v - 1 {
            check.square();
        }

        assert!(check == Fp::from(1));

        // compute square root with Tonelli--Shanks
        // (does not terminate if not a square!)

        while (b != Fp::from(1)) {
            let mut m = 0;
            let mut b2m = b;
            while (b2m != Fp::from(1)) {
                // invariant: b2m = b^(2^m) after entering this loop
                b2m.square();
                m += 1;
            }

            let mut j = (v - m) - 1;
            w = z.clone();
            while (j > 0) {
                w.square();
                j -= 1;
            } // w = z^2^(v-m-1)

            z = w.clone() * w;
            b = b * z;
            x = x * w;
            v = m;
        }

        return x;
    }

    pub fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            // std::cout << "(" << copy.X.toString(10) << " : " << copy.Y.toString(10) << " : " << copy.Z.toString(10) << ")\n";
        }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            // std::cout << "(" << X.toString(10) << " : " << Y.toString(10) << " : " << Z.toString(10) << ")\n";
        }
    }

    pub fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = 0.into();
            self.Y = 1.into();
            self.Z = 0.into();
        } else {
            let mut r = Fp::default();
            r = self.Z;
            r.inverse();
            self.Z = r.squared();
            self.X *= self.Z;
            r *= self.Z;
            self.Y *= r;
            self.Z = 1.into();
        }
    }

    pub fn to_special(&mut self) {
        self.to_affine_coordinates();
    }

    pub fn is_special(&self) -> bool {
        return (self.is_zero() || self.Z == 1.into());
    }

    pub fn is_zero(&self) -> bool {
        return self.Z.is_zero();
    }

    pub fn add(&mut self, other: &bn128_G1) -> bn128_G1 {
        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt += 1;

        let (mut this_coord, mut other_coord, mut result_coord) =
            ([Fp::default(); 3], [Fp::default(); 3], [Fp::default(); 3]);
        self.fill_coord(&mut this_coord);
        other.fill_coord(&mut other_coord);
        // ecop::ECAdd(result_coord, this_coord, other_coord);

        let result = bn128_G1::new(result_coord);
        result
    }

    pub fn mixed_add(&self, other: &bn128_G1) -> bn128_G1 {
        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // #ifdef DEBUG
        assert!(other.is_special());

        // check for doubling case

        // using Jacobian coordinates so:
        // (X1:Y1:Z1) = (X2:Y2:Z2)
        // iff
        // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
        // iff
        // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

        // we know that Z2 = 1

        let mut Z1Z1 = Fp::default();
        Z1Z1 = self.Z.squared();
        let mut  U1: Fp = self.X.clone();
        let mut U2 = Fp::default();
        U2 = other.X.clone() * Z1Z1;
        let mut Z1_cubed = Fp::default();
        Z1_cubed = self.Z.clone() * Z1Z1;

        let mut S1: Fp = self.Y;
        let mut S2 = Fp::default();
        S2 = other.Y.clone() * Z1_cubed; // S2 = Y2*Z1*Z1Z1

        if U1 == U2 && S1 == S2 {
            // dbl case; nothing of above can be reused
            return self.dbl();
        }

        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt += 1;

        let mut result = bn128_G1::default();
        let (mut H,mut  HH,mut  I,mut  J,mut  r,mut  V,mut  tmp) = (
            Fp::default(),
            Fp::default(),
            Fp::default(),
            Fp::default(),
            Fp::default(),
            Fp::default(),
            Fp::default(),
        );
        // H = U2-X1
        H = U2.clone() - self.X.clone();
        // HH = H^2
        HH = H.squared();
        // I = 4*HH
        tmp = HH.clone() + &HH;
        I = tmp.clone() + &tmp;
        // J = H*I
        J = H.clone() * I.clone();
        // r = 2*(S2-Y1)
        tmp = S2.clone() - self.Y.clone();
        r = tmp.clone() + &tmp;
        // V = X1*I
        V = self.X.clone() * I;
        // X3 = r^2-J-2*V
        result.X = r.squared();
        result.X = result.X.clone() - J.clone();
        result.X = result.X.clone() - V.clone();
        result.X = result.X.clone() - V.clone();
        // Y3 = r*(V-X3)-2*Y1*J
        tmp = V.clone() - result.X.clone();
        result.Y = r.clone() * tmp.clone();
        tmp = self.Y.clone() * J.clone();
        result.Y = result.Y.clone() - tmp.clone();
        result.Y = result.Y.clone() - tmp.clone();
        // Z3 = (Z1+H)^2-Z1Z1-HH
        tmp = self.Z.clone() + &H;
        result.Z = tmp.squared();
        result.Z = result.Z.clone() - Z1Z1.clone();
        result.Z = result.Z.clone() - HH.clone();
        result
    }

    pub fn dbl(&self) -> bn128_G1 {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt += 1;

        let (mut this_coord, mut result_coord) = ([Fp::default(); 3], [Fp::default(); 3]);
        self.fill_coord(&mut this_coord);
        // ecop::ECDouble(result_coord, this_coord);

        let mut result = bn128_G1::new(result_coord);
        result
    }

    pub fn mul_by_cofactor(&self) -> bn128_G1 {
        // Cofactor = 1
        return self.clone();
    }

    pub fn zero() -> bn128_G1 {
        return Self::G1_zero();
    }

    pub fn one() -> bn128_G1 {
        return Self::G1_one();
    }

    pub fn random_element() -> bn128_G1 {
        Self::G1_one() * bn128_Fr::random_element().as_bigint()
    }

    pub fn is_well_formed(&self) -> bool {
        if self.is_zero() {
            return true;
        }
        /*
            y^2 = x^3 + b

            We are using Jacobian coordinates, so equation we need to check is actually

            (y/z^3)^2 = (x/z^2)^3 + b
            y^2 / z^6 = x^3 / z^6 + b
            y^2 = x^3 + b z^6
        */
        let (mut X2, mut Y2, mut Z2) = (Fp::default(), Fp::default(), Fp::default());
        X2 = self.X.clone().squared();
        Y2 = self.Y.clone().squared();
        Z2 = self.Z.clone().squared();

        let (mut X3, mut Z3, mut Z6) = (Fp::default(), Fp::default(), Fp::default());
        X3 = X2.clone() * self.X.clone();
        Z3 = Z2.clone() * self.Z.clone();
        Z6 = Z3.squared();

        return (Y2 == X3 + bn128_coeff_b * Z6);
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<bn128_G1>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z);
        }
        // bn_batch_invert::<Fp>(Z_vec);

        let one: Fp = 1.into();

        for i in 0..vec.len() {
            let (mut Z2, mut Z3) = (Fp::default(), Fp::default());
            Z2 = Z_vec[i].squared();
            Z3 = Z2.clone() * Z_vec[i].clone();

            vec[i].X = vec[i].X.clone() * Z2.clone();
            vec[i].Y = vec[i].Y.clone() * Z3.clone();
            vec[i].Z = one;
        }
    }
}

impl Add<i32> for bn128_G1 {
    type Output = bn128_G1;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for bn128_G1 {
    type Output = bn128_G1;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for bn128_G1 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for bn128_G1 {
    type Output = bn128_G1;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for bn128_G1 {
    type Output = bn128_G1;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Mul<i32> for bn128_G1 {
    type Output = bn128_G1;

    fn mul(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<O: Borrow<Self>> Mul<O> for bn128_G1 {
    type Output = bn128_G1;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for bn128_G1 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for bn128_G1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for bn128_G1 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for bn128_G1 {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
// use ffec::field_utils::BigInt;
// use num_bigint::BigInt;
// use std::io::{self, Read, Write};
// use std::ops::{Add, Mul, Neg, Sub};

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Bn128G1 {
//     pub x: Fp,
//     pub y: Fp,
//     pub z: Fp,
// }

// impl Bn128G1 {
//     pub fn zero() -> Self {}
//     pub fn is_zero(&self) -> bool {
//         self.z.is_zero()
//     }

//     pub fn to_affine(&self) -> (Fp, Fp, bool) {
//         if self.is_zero() {
//             return (Fp::zero(), Fp::zero(), true);
//         }
//         let z_inv = self.z.inverse().unwrap();
//         let z_inv2 = z_inv.square();
//         let z_inv3 = z_inv2 * &z_inv;
//         (self.x * &z_inv2, self.y * &z_inv3, false)
//     }

//     pub fn dbl(&self) -> Self {
//         todo!()
//     }
//     pub fn add_nodeck(&self, other: &Self) -> Self {
//         todo!()
//     }
// }

// impl<'a, const N: usize> Mul<&'a BigInt<N>> for &'a Bn128G1 {
//     type Output = Bn128G1;
//     fn mul(self, rhs: &'a BigInt<N>) -> Bn128G1 {
//         rhs.scalar_mul(self)
//     }
// }

// impl PartialEq for Bn128G1 {
//     fn eq(&self, other: &Self) -> bool {
//         if self.is_zero() {
//             return other.is_zero();
//         }
//         if other.is_zero() {
//             return false;
//         }

//         let z1sq = self.z.square();
//         let z2sq = other.z.square();

//         if (self.x * &z2sq) != (other.x * &z1sq) {
//             return false;
//         }

//         let z1cubed = z1sq * &self.z;
//         let z2cubed = z2sq * &other.z;

//         (self.y * &z2cubed) == (other.y * &z1cubed)
//     }
// }

// impl<'a> Add<&'a Bn128G1> for &'a Bn128G1 {
//     type Output = Bn128G1;
//     fn add(self, other: &'a Bn128G1) -> Bn128G1 {
//         if self.is_zero() {
//             return *other;
//         }
//         if other.is_zero() {
//             return *self;
//         }

//         if self == other {
//             return self.dbl();
//         }
//         self.add_nodeck(other)
//     }
// }

// impl Neg for Bn128G1 {
//     type Output = Self;
//     fn neg(mut self) -> Self {
//         self.y = -self.y;
//         self
//     }
// }

// impl<'a> Sub<&'a Bn128G1> for &'a Bn128G1 {
//     type Output = Bn128G1;
//     fn sub(self, other: &'a Bn128G1) -> Bn128G1 {
//         self + &(-*other)
//     }
// }

// impl Bn128G1 {
//     pub fn serialize<W: Write>(&self, mut writer: W, compress: bool) -> io::Result<()> {
//         let (x, y, is_zero) = self.to_affine();
//         writer.write_all(if is_zero { b"1" } else { b"0" })?;
//         writer.write_all(b" ")?;

//         if !compress {
//             writer.write_all(&x.to_bytes())?;
//             writer.write_all(b" ")?;
//             writer.write_all(&y.to_bytes())?;
//         } else {
//             writer.write_all(&x.to_bytes())?;
//             writer.write_all(b" ")?;

//             let y_lsb = if y.to_bigint().is_odd() { b"1" } else { b"0" };
//             writer.write_all(y_lsb)?;
//         }
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R, compress: bool) -> io::Result<Self> {
//         let mut zero_buf = [0u8; 1];
//         reader.read_exact(&mut zero_buf)?;
//         let is_zero = zero_buf[0] == b'1';

//         if is_zero {
//             return Ok(Self::zero());
//         }

//         let t_x = Fp::read(&mut reader)?;
//         let mut t_y: Fp;

//         if !compress {
//             t_y = Fp::read(&mut reader)?;
//         } else {
//             let t_y2 = t_x.square() * &t_x + &BN128_COEFF_B;
//             t_y = t_y2
//                 .sqrt()
//                 .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid X"))?;

//             let mut lsb_buf = [0u8; 1];
//             reader.read_exact(&mut lsb_buf)?;
//             let y_lsb = lsb_buf[0] == b'1';

//             if t_y.to_bigint().is_odd() != y_lsb {
//                 t_y = -t_y;
//             }
//         }

//         Ok(Self {
//             x: t_x,
//             y: t_y,
//             z: Fp::one(),
//         })
//     }
// }
