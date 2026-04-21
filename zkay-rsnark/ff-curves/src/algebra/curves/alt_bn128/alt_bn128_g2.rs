use crate::{
    FpmConfig, Fq2mConfig,
    algebra::curves::{
        alt_bn128::{
            alt_bn128_init::{
                alt_bn128_coeff_b, alt_bn128_twist_coeff_b, alt_bn128_twist_mul_by_b_c0,
                alt_bn128_twist_mul_by_b_c1, alt_bn128_twist_mul_by_q_X,
                alt_bn128_twist_mul_by_q_Y,
            },
            {
                alt_bn128_fields::{alt_bn128_Fq, alt_bn128_Fq2, alt_bn128_Fr},
                curves::Bn254,
            },
        },
        curve_utils::scalar_mul,
    },
    new_fq2,
};

use cfg_if::cfg_if;

use ffec::{
    BigInt, Fp_model, Fp_modelConfig, One, PpConfig, Zero,
    common::serialization::{OUTPUT_NEWLINE, OUTPUT_SEPARATOR, consume_output_separator},
    field_utils::{
        BigInt,
        bigint::{GMP_NUMB_BITS, bigint},
        field_utils::batch_invert,
    },
};
use num_bigint::BigUint;
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
// pub type alt_bn128_G2 = <Bn254 as Pairing>::G2;
#[derive(Default, Clone, Debug)]
pub struct alt_bn128_G2 {
    pub X: alt_bn128_Fq2,
    pub Y: alt_bn128_Fq2,
    pub Z: alt_bn128_Fq2,
}

//
pub trait alt_bn128_G2Config: Send + Sync + Sized + 'static {
    const wnaf_window_table: &'static [usize];
    const fixed_base_exp_window_table: &'static [usize];
}

type base_field = alt_bn128_Fq;
type twist_field = alt_bn128_Fq2;
type scalar_field = alt_bn128_Fr;

impl alt_bn128_G2Config for alt_bn128_G2 {
    const wnaf_window_table: &'static [usize] = &[5, 15, 39, 109];

    // alt_bn128_G2::fixed_base_exp_window_table.resize(0);
    const fixed_base_exp_window_table: &'static [usize] = &[
        // window 1 is unbeaten in [-inf, 5.10]
        1,       // window 2 is unbeaten in [5.10, 10.43]
        5,       // window 3 is unbeaten in [10.43, 25.28]
        10,      // window 4 is unbeaten in [25.28, 59.00]
        25,      // window 5 is unbeaten in [59.00, 154.03]
        59,      // window 6 is unbeaten in [154.03, 334.25]
        154,     // window 7 is unbeaten in [334.25, 742.58]
        334,     // window 8 is unbeaten in [742.58, 2034.40]
        743,     // window 9 is unbeaten in [2034.40, 4987.56]
        2034,    // window 10 is unbeaten in [4987.56, 8888.27]
        4988,    // window 11 is unbeaten in [8888.27, 26271.13]
        8888,    // window 12 is unbeaten in [26271.13, 39768.20]
        26271,   // window 13 is unbeaten in [39768.20, 106275.75]
        39768,   // window 14 is unbeaten in [106275.75, 141703.40]
        106276,  // window 15 is unbeaten in [141703.40, 462422.97]
        141703,  // window 16 is unbeaten in [462422.97, 926871.84]
        462423,  // window 17 is unbeaten in [926871.84, 4873049.17]
        926872,  // window 18 is never the best
        0,       // window 19 is unbeaten in [4873049.17, 5706707.88]
        4873049, // window 20 is unbeaten in [5706707.88, 31673814.95]
        5706708, // window 21 is never the best
        0,       // window 22 is unbeaten in [31673814.95, inf]
        31673815,
    ];
}
impl alt_bn128_G2 {
    const h_bitcount: usize = 256;
    const h_limbs: usize = (Self::h_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
    const h: bigint<{ Self::h_limbs }> = bigint::<{ Self::h_limbs }>(BigInt!("1"));

    pub fn field_char() -> Vec<u64> {
        base_field::field_char().as_ref().to_vec()
    }
    pub fn order() -> Vec<u64> {
        scalar_field::field_char().as_ref().to_vec()
    }

    pub fn new(X: alt_bn128_Fq2, Y: alt_bn128_Fq2, Z: alt_bn128_Fq2) -> Self {
        Self { X, Y, Z }
    }
    fn G2_zero() -> Self {
        Self::new(
            alt_bn128_Fq2::zero(),
            alt_bn128_Fq2::one(),
            alt_bn128_Fq2::zero(),
        )
    }
    fn G2_one() -> Self {
        Self::new(
            new_fq2!(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
                "11559732032986387107991004021392285783925812861821192530917403151452391805634"
            ),
            new_fq2!(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
                "4082367875863433681332203403145435568316851327593401208105741076214120093531"
            ),
            alt_bn128_Fq2::one(),
        )
    }
    pub fn initialize() {
        // if initialized
        // {
        //     self.X = G2_zero.X;
        //     self.Y = G2_zero.Y;
        //     self.Z = G2_zero.Z;
        // }
    }

    pub fn mul_by_b(elt: &alt_bn128_Fq2) -> alt_bn128_Fq2 {
        alt_bn128_Fq2::new(
            alt_bn128_twist_mul_by_b_c0() * elt.c0,
            alt_bn128_twist_mul_by_b_c1() * elt.c1,
        )
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$}*z + {:N$} : {:N$}*z + {:N$} : {:N$}*z + {:N$})\n",
                self.X.c1.as_bigint().0,
                self.X.c0.as_bigint().0,
                self.Y.c1.as_bigint().0,
                self.Y.c0.as_bigint().0,
                self.Z.c1.as_bigint().0,
                self.Z.c0.as_bigint().0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    pub fn is_zero(&self) -> bool {
        self.Z.is_zero()
    }

    pub fn add(&self, other: &alt_bn128_G2) -> Self {
        self.clone() + other
    }

    pub fn dbl(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt+=1;

        // handle point at infinity
        if self.is_zero() {
            return self.clone();
        }

        // NOTE: does not handle O and pts of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // using Jacobian coordinates according to
        // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l

        let A = self.X.squared(); // A = X1^2
        let B = self.Y.squared(); // B = Y1^2
        let C = B.squared(); // C = B^2
        let mut D = (self.X + B).squared() - A - C;
        D = D + D; // D = 2 * ((X1 + B)^2 - A - C)
        let E = A + A + A; // E = 3 * A
        let F = E.squared(); // F = E^2
        let X3 = F - (D + D); // X3 = F - 2 D
        let mut eightC = C + C;
        eightC = eightC + eightC;
        eightC = eightC + eightC;
        let Y3 = E * (D - X3) - eightC; // Y3 = E * (D - X3) - 8 * C
        let Y1Z1 = self.Y * self.Z;
        let Z3 = Y1Z1 + Y1Z1; // Z3 = 2 * Y1 * Z1

        Self::new(X3, Y3, Z3)
    }

    pub fn mul_by_q(&self) -> Self {
        Self::new(
            alt_bn128_twist_mul_by_q_X * self.X.Frobenius_map(1),
            alt_bn128_twist_mul_by_q_Y * self.Y.Frobenius_map(1),
            self.Z.Frobenius_map(1),
        )
    }

    pub fn mul_by_cofactor(&self) -> Self {
        self.clone() * Self::h
    }

    pub fn zero() -> Self {
        Self::G2_zero()
    }

    pub fn one() -> Self {
        Self::G2_one()
    }
}
impl FpmConfig for alt_bn128_G2 {
    type Fr = alt_bn128_Fq;
}

// impl One for alt_bn128_G2 {
// fn one() -> Self { Self::G1_zero() }
// }
// impl BigInteger for alt_bn128_G2 {}
impl From<BigUint> for alt_bn128_G2 {
    fn from(_: BigUint) -> Self {
        Default::default()
    }
}

// impl AsRef<[u64]> for bigint<1> {
//     fn as_ref(&self) -> &[u64] {
//         &self.0
//     }
// }
impl PpConfig for alt_bn128_G2 {
    type BigIntT = bigint<1>;

    fn dbl(&self) -> Self {
        self.clone()
    }
    fn random_element() -> Self {
        Self::G2_one() * (alt_bn128_Fr::random_element().as_bigint())
    }
    fn wnaf_window_table() -> Vec<usize> {
        vec![]
    }
    fn fixed_base_exp_window_table() -> std::vec::Vec<usize> {
        vec![]
    }
    fn batch_to_special_all_non_zeros(vec: &mut Vec<alt_bn128_G2>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z);
        }
        batch_invert::<alt_bn128_Fq2>(&mut Z_vec);

        let one = alt_bn128_Fq2::one();

        for i in 0..vec.len() {
            let Z2 = Z_vec[i].squared();
            let Z3 = Z_vec[i] * Z2;

            vec[i].X = vec[i].X * Z2;
            vec[i].Y = vec[i].Y * Z3;
            vec[i].Z = one;
        }
    }
    fn to_special(&mut self) {
        self.to_affine_coordinates();
    }

    fn mixed_add(&self, other: &alt_bn128_G2) -> Self {
        // #ifdef DEBUG
        assert!(other.is_special());

        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // using Jacobian coordinates according to
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-madd-2007-bl
        // Note: (X1:Y1:Z1) = (X2:Y2:Z2)
        // iff
        // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
        // iff
        // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3
        // we know that Z2 = 1

        let Z1Z1 = self.Z.squared();

        let U1 = self.X;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = self.Z * Z1Z1;

        let S1 = self.Y; // S1 = Y1 * Z2 * Z2Z2
        let S2 = other.Y * Z1_cubed; // S2 = Y2 * Z1 * Z1Z1

        // check for doubling case
        if U1 == U2 && S1 == S2 {
            // dbl case; nothing of above can be reused
            return self.dbl();
        }

        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt+=1;

        let H = U2 - self.X; // H = U2-X1
        let HH = H.squared(); // HH = H^2
        let mut I = HH + HH; // I = 4*HH
        I = I + I;
        let J = H * I; // J = H*I
        let mut r = S2 - self.Y; // r = 2*(S2-Y1)
        r = r + r;
        let V = self.X * I; // V = X1*I
        let X3 = r.squared() - J - V - V; // X3 = r^2-J-2*V
        let mut Y3 = self.Y * J; // Y3 = r*(V-X3)-2*Y1*J
        Y3 = r * (V - X3) - Y3 - Y3;
        let Z3 = (self.Z + H).squared() - Z1Z1 - HH; // Z3 = (Z1+H)^2-Z1Z1-HH

        Self::new(X3, Y3, Z3)
    }

    fn unitary_inverse(&self) -> Self {
        Default::default()
    }
    fn is_special(&self) -> bool {
        self.is_zero() || self.Z == alt_bn128_Fq2::one()
    }

    fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            print!(
                "({:N$}*z + {:N$} , {:N$}*z + {:N$})\n",
                copy.X.c1.as_bigint().0,
                copy.X.c0.as_bigint().0,
                copy.Y.c1.as_bigint().0,
                copy.Y.c0.as_bigint().0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }
    fn size_in_bits() -> usize {
        twist_field::ceil_size_in_bits() + 1
    }

    fn is_well_formed(&self) -> bool {
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
        let X2 = self.X.squared();
        let Y2 = self.Y.squared();
        let Z2 = self.Z.squared();

        let X3 = self.X * X2;
        let Z3 = self.Z * Z2;
        let Z6 = Z3.squared();

        Y2 == X3 + alt_bn128_twist_coeff_b() * Z6
    }

    fn num_bits() -> usize {
        1
    }
    fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = alt_bn128_Fq2::zero();
            self.Y = alt_bn128_Fq2::one();
            self.Z = alt_bn128_Fq2::zero();
        } else {
            let Z_inv = self.Z.inverse();
            let Z2_inv = Z_inv.squared();
            let Z3_inv = Z2_inv * Z_inv;
            self.X = self.X * Z2_inv;
            self.Y = self.Y * Z3_inv;
            self.Z = alt_bn128_Fq2::one();
        }
    }
}
impl Fq2mConfig for alt_bn128_G2 {
    // //type TT = bigint<1>;
    type Fr = Self;
}

impl PartialEq for alt_bn128_G2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() {
            return other.is_zero();
        }

        if other.is_zero() {
            return false;
        }

        //now neither is O
        // using Jacobian coordinates so:
        // (X1:Y1:Z1) = (X2:Y2:Z2)
        // iff
        // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
        // iff
        // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

        let Z1_squared = self.Z.squared();
        let Z2_squared = other.Z.squared();

        if self.X * Z2_squared != other.X * Z1_squared {
            return false;
        }

        let Z1_cubed = self.Z * Z1_squared;
        let Z2_cubed = other.Z * Z2_squared;

        self.Y * Z2_cubed == other.Y * Z1_cubed
    }
}

impl Add<i32> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn add(self, other: O) -> Self::Output {
        let other = other.borrow().clone();
        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // using Jacobian coordinates according to
        // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
        // Note: (X1:Y1:Z1) = (X2:Y2:Z2)
        // iff
        // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
        // iff
        // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

        let Z1Z1 = (self.Z).squared();
        let Z2Z2 = (other.Z).squared();

        let U1 = self.X * Z2Z2;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = (self.Z) * Z1Z1;
        let Z2_cubed = (other.Z) * Z2Z2;

        let S1 = (self.Y) * Z2_cubed; // S1 = Y1 * Z2 * Z2Z2
        let S2 = (other.Y) * Z1_cubed; // S2 = Y2 * Z1 * Z1Z1

        // check for doubling case
        if U1 == U2 && S1 == S2 {
            // dbl case; nothing of above can be reused
            return self.dbl();
        }

        // rest of add case
        let H = U2 - U1; // H = U2-U1
        let S2_minus_S1 = S2 - S1;
        let I = (H + H).squared(); // I = (2 * H)^2
        let J = H * I; // J = H * I
        let r = S2_minus_S1 + S2_minus_S1; // r = 2 * (S2-S1)
        let V = U1 * I; // V = U1 * I
        let X3 = r.squared() - J - (V + V); // X3 = r^2 - J - 2 * V
        let S1_J = S1 * J;
        let Y3 = r * (V - X3) - (S1_J + S1_J); // Y3 = r * (V-X3)-2 S1 J
        let Z3 = ((self.Z + other.Z).squared() - Z1Z1 - Z2Z2) * H; // Z3 = ((Z1+Z2)^2-Z1Z1-Z2Z2) * H

        alt_bn128_G2::new(X3, Y3, Z3)
    }
}

impl Sub for alt_bn128_G2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl<const N: usize> Mul<bigint<N>> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        scalar_mul::<alt_bn128_G2, N>(&self, &rhs)
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        scalar_mul::<alt_bn128_G2, N>(&self, &rhs.as_bigint())
    }
}

// impl Mul<i32> for alt_bn128_G2 {
//     type Output = alt_bn128_G2;

//     fn mul(self, other: i32) -> Self::Output {
//         scalar_mul::<alt_bn128_G2, N>(rhs, lhs)
//     }
// }

impl<O: Borrow<Self>> Mul<O> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, rhs: O) -> Self::Output {
        panic!("MYTODO");
        self
    }
}

impl Neg for alt_bn128_G2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.X, -self.Y, self.Z)
    }
}

use std::fmt;
impl fmt::Display for alt_bn128_G2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for alt_bn128_G2 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for alt_bn128_G2 {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

use std::io::{self, Read, Write};

// 假设已经定义了 Fq2 及其相关运算
// Fq2 通常由 c0 和 c1 两个 Fq 组成

impl alt_bn128_G2 {
    /// 翻译 C++: operator<<
    pub fn write<W: Write>(&self, mut out: W) -> io::Result<()> {
        let mut copy = self.clone();
        copy.to_affine_coordinates();

        // out << (copy.is_zero() ? 1 : 0) << OUTPUT_SEPARATOR;
        let is_zero_byte = if copy.is_zero() { b'1' } else { b'0' };
        out.write_all(&[is_zero_byte])?;
        out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
        cfg_if! {
               if  #[cfg(feature = "no_pt_compression")]
                {
                    // out << copy.X << OUTPUT_SEPARATOR << copy.Y;
                    write!(out, "{}{}{}", copy.X, OUTPUT_SEPARATOR, copy.Y)?;
                }
        else

                {
                    //storing LSB of Y (注意：G2 通常取 c0 的 LSB)
                    // out << copy.X << OUTPUT_SEPARATOR << (copy.Y.c0.as_bigint().data[0] & 1);
                    let y_lsb = (copy.Y.c0.as_bigint().0.0[0] & 1) as u8 + b'0';
                    write!(out, "{}", copy.X)?;
                    out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
                    out.write_all(&[y_lsb])?;
                }
        }
        Ok(())
    }

    /// 翻译 C++: operator>>
    pub fn read<R: Read + std::io::BufRead>(mut input: R) -> io::Result<Self> {
        let mut is_zero_raw = [0u8; 1];
        let mut tx: alt_bn128_Fq2;
        let ty: alt_bn128_Fq2;
        cfg_if! {  if #[cfg(feature = "no_pt_compression")]
         {
             // in >> is_zero >> tX >> tY;
             input.read_exact(&mut is_zero_raw)?;
             let is_zero = is_zero_raw[0] - b'0';
             tx = alt_bn128_Fq2::read(&mut input)?;
             ty = alt_bn128_Fq2::read(&mut input)?;

             if is_zero != 0 {
                 return Ok(Self::zero());
             }
         }

        else
         {
             // in.read((char*)&is_zero, 1);
             input.read_exact(&mut is_zero_raw)?;
             let is_zero = is_zero_raw[0] - b'0';
             consume_output_separator(&mut input)?;

             // in >> tX;
             tx = alt_bn128_Fq2::read(&mut input)?;
             consume_output_separator(&mut input)?;

             // in.read((char*)&Y_lsb, 1);
             let mut y_lsb_raw = [0u8; 1];
             input.read_exact(&mut y_lsb_raw)?;
             let y_lsb = y_lsb_raw[0] - b'0';

             if is_zero == 0 {
                 // y = +/- sqrt(x^3 + b)
                 let tx2 = tx.squared();
                 let ty2 = tx2 * tx + alt_bn128_twist_coeff_b();
                 let mut ty_sqrt = ty2.sqrt().ok_or(io::Error::new(io::ErrorKind::InvalidData, "No sqrt"))?;

                 // if ((tY.c0.as_bigint().data[0] & 1) != Y_lsb)
                 if (ty_sqrt.c0.as_bigint().0.0[0] & 1) as u8 != y_lsb {
                     ty_sqrt = -ty_sqrt;
                 }
                 ty = ty_sqrt;
             } else {
                 return Ok(Self::zero());
             }
         }}

        // using projective coordinates (还原为 Jacobian/Projective 坐标)
        Ok(Self {
            X: tx,
            Y: ty,
            Z: alt_bn128_Fq2::one(),
        })
    }
}
