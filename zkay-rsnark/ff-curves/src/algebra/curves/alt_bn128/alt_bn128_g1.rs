use crate::{
    FpmConfig,
    algebra::curves::{
        alt_bn128::{
            alt_bn128_fields::{alt_bn128_Fq, alt_bn128_Fq2, alt_bn128_Fr},
            alt_bn128_init::{
                alt_bn128_coeff_b, alt_bn128_twist_mul_by_b_c0, alt_bn128_twist_mul_by_b_c1,
            },
            curves::Bn254,
        },
        curve_utils::scalar_mul,
        pairing::Pairing,
    },
};

use cfg_if::cfg_if;
use ffec::{
    BigInt, FieldTConfig, Fp_model, Fp_modelConfig, One, PpConfig, Zero,
    common::serialization::{
        OUTPUT_NEWLINE, OUTPUT_SEPARATOR, consume_output_newline, consume_output_separator,
        read_line_as_usize,
    },
    field_utils::{
        BigInteger,
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

// pub type alt_bn128_G1 = <Bn254 as Pairing>::G1;

#[derive(Clone, Default, Debug)]
pub struct alt_bn128_G1 {
    pub X: alt_bn128_Fq,
    pub Y: alt_bn128_Fq,
    pub Z: alt_bn128_Fq,
}

pub trait alt_bn128_G1Config: Send + Sync + Sized + 'static {
    const wnaf_window_table: &'static [usize];
    const fixed_base_exp_window_table: &'static [usize];
}

pub type base_field = alt_bn128_Fq;
pub type scalar_field = alt_bn128_Fr;

impl From<BigUint> for alt_bn128_G1 {
    fn from(_: BigUint) -> Self {
        Default::default()
    }
}

impl PpConfig for alt_bn128_G1 {
    type BigIntT = bigint<1>;
    fn size_in_bits() -> usize {
        base_field::ceil_size_in_bits() + 1
    }
    // fn as_bigint(&self) -> bigint<N> {
    //     self.as_bigint()
    // }
    fn dbl(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt+=1;

        // handle point at infinity
        if self.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
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

        alt_bn128_G1::new(X3, Y3, Z3)
    }
    fn random_element() -> Self {
        Self::G1_one() * (scalar_field::random_element().as_bigint())
    }
    fn wnaf_window_table() -> Vec<usize> {
        <alt_bn128_G1 as alt_bn128_G1Config>::wnaf_window_table.to_vec()
    }
    fn fixed_base_exp_window_table() -> std::vec::Vec<usize> {
        <alt_bn128_G1 as alt_bn128_G1Config>::fixed_base_exp_window_table.to_vec()
    }
    fn batch_to_special_all_non_zeros(vec: &mut Vec<alt_bn128_G1>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z);
        }
        batch_invert::<alt_bn128_Fq>(&mut Z_vec);

        let one = alt_bn128_Fq::one();

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
    fn mixed_add(&self, other: &alt_bn128_G1) -> Self {
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

        let Z1Z1 = (self.Z).squared();

        let U1 = self.X;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = (self.Z) * Z1Z1;

        let S1 = (self.Y); // S1 = Y1 * Z2 * Z2Z2
        let S2 = (other.Y) * Z1_cubed; // S2 = Y2 * Z1 * Z1Z1

        // check for doubling case
        if U1 == U2 && S1 == S2 {
            // dbl case; nothing of above can be reused
            return self.dbl();
        }

        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt+=1;

        let H = U2 - (self.X); // H = U2-X1
        let HH = H.squared(); // HH = H^2
        let mut I = HH + HH; // I = 4*HH
        I = I + I;
        let J = H * I; // J = H*I
        let mut r = S2 - self.Y; // r = 2*(S2-Y1)
        r = r + r;
        let V = self.X * I; // V = X1*I
        let X3 = r.squared() - J - V - V; // X3 = r^2-J-2*V
        let mut Y3 = (self.Y) * J; // Y3 = r*(V-X3)-2*Y1*J
        Y3 = r * (V - X3) - Y3 - Y3;
        let Z3 = (self.Z + H).squared() - Z1Z1 - HH; // Z3 = (Z1+H)^2-Z1Z1-HH

        alt_bn128_G1::new(X3, Y3, Z3)
    }
    fn unitary_inverse(&self) -> Self {
        Default::default()
    }
    fn is_special(&self) -> bool {
        self.is_zero() || self.Z == alt_bn128_Fq::one()
    }
    fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone(); //alt_bn128_G1
            copy.to_affine_coordinates();
            print!(
                "({:N$} , {:N$})\n",
                copy.X.as_bigint().0,
                copy.Y.as_bigint().0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    fn is_well_formed(&self) -> bool {
        if self.is_zero() {
            return true;
        }

        // y^2 = x^3 + b

        // We are using Jacobian coordinates, so equation we need to check is actually

        // (y/z^3)^2 = (x/z^2)^3 + b
        // y^2 / z^6 = x^3 / z^6 + b
        // y^2 = x^3 + b z^6

        let X2 = self.X.squared();
        let Y2 = self.Y.squared();
        let Z2 = self.Z.squared();

        let X3 = self.X * X2;
        let Z3 = self.Z * Z2;
        let Z6 = Z3.squared();

        return (Y2 == X3 + alt_bn128_coeff_b * Z6);
    }

    fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = alt_bn128_Fq::zero();
            self.Y = alt_bn128_Fq::one();
            self.Z = alt_bn128_Fq::zero();
        } else {
            let Z_inv = self.Z.inverse();
            let Z2_inv = Z_inv.squared();
            let Z3_inv = Z2_inv * Z_inv;
            self.X = self.X * Z2_inv;
            self.Y = self.Y * Z3_inv;
            self.Z = alt_bn128_Fq::one();
        }
    }
}

impl alt_bn128_G1Config for alt_bn128_G1 {
    const wnaf_window_table: &'static [usize] = &[11, 24, 60, 127];

    // alt_bn128_G1::fixed_base_exp_window_table.resize(0);
    const fixed_base_exp_window_table: &'static [usize] = &[
        // window 1 is unbeaten in [-inf, 4.99]
        1,       // window 2 is unbeaten in [4.99, 10.99]
        5,       // window 3 is unbeaten in [10.99, 32.29]
        11,      // window 4 is unbeaten in [32.29, 55.23]
        32,      // window 5 is unbeaten in [55.23, 162.03]
        55,      // window 6 is unbeaten in [162.03, 360.15]
        162,     // window 7 is unbeaten in [360.15, 815.44]
        360,     // window 8 is unbeaten in [815.44, 2373.07]
        815,     // window 9 is unbeaten in [2373.07, 6977.75]
        2373,    // window 10 is unbeaten in [6977.75, 7122.23]
        6978,    // window 11 is unbeaten in [7122.23, 57818.46]
        7122,    // window 12 is never the best
        0,       // window 13 is unbeaten in [57818.46, 169679.14]
        57818,   // window 14 is never the best
        0,       // window 15 is unbeaten in [169679.14, 439758.91]
        169679,  // window 16 is unbeaten in [439758.91, 936073.41]
        439759,  // window 17 is unbeaten in [936073.41, 4666554.74]
        936073,  // window 18 is never the best
        0,       // window 19 is unbeaten in [4666554.74, 7580404.42]
        4666555, // window 20 is unbeaten in [7580404.42, 34552892.20]
        7580404, // window 21 is never the best
        0,       // window 22 is unbeaten in [34552892.20, inf]
        34552892,
    ];
}
impl alt_bn128_G1 {
    const h_bitcount: usize = 254;
    const h_limbs: usize = (Self::h_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
    const h: bigint<{ alt_bn128_G1::h_limbs }> = bigint::<{ alt_bn128_G1::h_limbs }>(BigInt!("1"));
    pub fn field_char() -> bigint<{ base_field::num_limbs }> {
        base_field::field_char()
    }
    pub fn order() -> bigint<{ scalar_field::num_limbs }> {
        scalar_field::field_char()
    }
    fn G1_zero() -> Self {
        Self::new(
            alt_bn128_Fq::zero(),
            alt_bn128_Fq::one(),
            alt_bn128_Fq::zero(),
        )
    }
    fn G1_one() -> Self {
        Self::new(
            alt_bn128_Fq::one(),
            alt_bn128_Fq::const_new(BigInt!("2")),
            alt_bn128_Fq::one(),
        )
    }
    pub fn new(X: alt_bn128_Fq, Y: alt_bn128_Fq, Z: alt_bn128_Fq) -> Self {
        Self { X, Y, Z }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$} : {:N$} : {:N$})\n",
                self.X.as_bigint().0,
                self.Y.as_bigint().0,
                self.Z.as_bigint().0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    pub fn is_zero(&self) -> bool {
        self.Z.is_zero()
    }

    pub fn add(&self, other: &alt_bn128_G1) -> Self {
        self.clone() + other
    }

    pub fn mul_by_cofactor(&self) -> Self {
        // Cofactor = 1
        self.clone()
    }

    fn zero() -> Self {
        Self::G1_zero()
    }

    fn one() -> Self {
        Self::G1_one()
    }
}

impl PartialEq for alt_bn128_G1 {
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

        let Z1_cubed = (self.Z) * Z1_squared;
        let Z2_cubed = (other.Z) * Z2_squared;

        self.Y * Z2_cubed == other.Y * Z1_cubed
    }
}

// impl Add<i32> for alt_bn128_G1 {
//     type Output = alt_bn128_G1;

//     fn add(self, other: i32) -> Self::Output {

//     }
// }

impl<O: Borrow<Self>> Add<O> for alt_bn128_G1 {
    type Output = alt_bn128_G1;

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

        let Z1Z1 = self.Z.squared();
        let Z2Z2 = other.Z.squared();

        let U1 = self.X * Z2Z2;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = self.Z * Z1Z1;
        let Z2_cubed = other.Z * Z2Z2;

        let S1 = self.Y * Z2_cubed; // S1 = Y1 * Z2 * Z2Z2
        let S2 = other.Y * Z1_cubed; // S2 = Y2 * Z1 * Z1Z1

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

        alt_bn128_G1::new(X3, Y3, Z3)
    }
}

impl Sub for alt_bn128_G1 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl<const N: usize> Mul<bigint<N>> for alt_bn128_G1 {
    type Output = alt_bn128_G1;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        scalar_mul::<alt_bn128_G1, N>(&self, &rhs)
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for alt_bn128_G1 {
    type Output = alt_bn128_G1;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        scalar_mul::<alt_bn128_G1, N>(&self, &rhs.as_bigint())
    }
}

// impl Mul<i32> for alt_bn128_G1 {
//     type Output = alt_bn128_G1;

//     fn mul(self, other: i32) -> Self::Output {
//         let mut r = self;
//         // r += *other.borrow();
//         r
//     }
// }
impl<O: Borrow<Self>> Mul<O> for alt_bn128_G1 {
    type Output = alt_bn128_G1;

    fn mul(self, rhs: O) -> Self::Output {
        panic!("MYTODO");
        self
    }
}

impl Neg for alt_bn128_G1 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        alt_bn128_G1::new(self.X, -self.Y, self.Z)
    }
}

use std::fmt;
impl fmt::Display for alt_bn128_G1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for alt_bn128_G1 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for alt_bn128_G1 {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}
impl FpmConfig for alt_bn128_G1 {
    type Fr = alt_bn128_Fq;
}

use std::io::{self, Read, Write};

// 模拟 C++ 的输出流操作符 <<
pub fn write_alt_bn128_g1<W: Write>(mut out: W, g: &alt_bn128_G1) -> io::Result<()> {
    let mut copy = g.clone();
    copy.to_affine_coordinates();

    // 输出是否为零点的标志 (1 为零点, 0 为非零)
    let is_zero = if copy.is_zero() { b'1' } else { b'0' };
    out.write_all(&[is_zero])?;
    out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
    cfg_if! {
       if #[cfg(feature = "no_pt_compression")]
        {
            // 非压缩模式：输出 X 和 Y
            write!(out, "{}{}{}", copy.X, OUTPUT_SEPARATOR, copy.Y)?;
        }
       else
        {
            // 压缩模式：输出 X 和 Y 的最低有效位 (LSB)
            let y_lsb = (copy.Y.as_bigint().0.0[0] & 1) as u8 + b'0';
            write!(out, "{}", copy.X)?;
            out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
            out.write_all(&[y_lsb])?;
        }
    }
    Ok(())
}

// 模拟 C++ 的输入流操作符 >>
pub fn read_alt_bn128_g1<R: Read>(mut input: R) -> io::Result<alt_bn128_G1> {
    let mut is_zero_buf = [0u8; 1];
    input.read_exact(&mut is_zero_buf)?;
    let is_zero = is_zero_buf[0] - b'0';

    cfg_if! { if #[cfg(feature = "no_pt_compression")]
        {
            consume_output_separator(&mut input)?;
            let tx: alt_bn128_Fq = alt_bn128_Fq::read(&mut input)?;
            consume_output_separator(&mut input)?;
            let ty: alt_bn128_Fq = alt_bn128_Fq::read(&mut input)?;

            if is_zero == 0 {
                Ok(alt_bn128_G1 { X: tx, Y: ty, Z: alt_bn128_Fq::one() })
            } else {
                Ok(alt_bn128_G1::zero())
            }
        }
    else

        {
            consume_output_separator(&mut input)?;
            let tx: alt_bn128_Fq = alt_bn128_Fq::read(&mut input)?;
            consume_output_separator(&mut input)?;

            let mut y_lsb_buf = [0u8; 1];
            input.read_exact(&mut y_lsb_buf)?;
            let y_lsb = y_lsb_buf[0] - b'0';

            if is_zero == 0 {
                // y = +/- sqrt(x^3 + b)
                let tx2 = tx.squared();
                let ty2 = tx2 * tx + alt_bn128_coeff_b;
                let mut ty = ty2.sqrt().ok_or(io::Error::new(io::ErrorKind::InvalidData, "No sqrt"))?;

                // 检查 LSB 是否匹配，不匹配则取相反数
                if (ty.as_bigint().0.0[0] & 1) as u8 != y_lsb {
                    ty = -ty;
                }
                Ok(alt_bn128_G1 { X: tx, Y: ty, Z: alt_bn128_Fq::one() })
            } else {
                Ok(alt_bn128_G1::zero())
            }
        }}
}

// 向量序列化
pub fn write_vector_g1<W: Write>(mut out: W, v: &[alt_bn128_G1]) -> io::Result<()> {
    writeln!(out, "{}", v.len())?;
    for g in v {
        write_alt_bn128_g1(&mut out, g)?;
        out.write_all(OUTPUT_NEWLINE.as_bytes())?;
    }
    Ok(())
}

// 向量反序列化
pub fn read_vector_g1<R: Read>(mut input: R) -> io::Result<Vec<alt_bn128_G1>> {
    let mut s_str = String::new();
    // 简化处理：读取一行作为 size
    let s: usize = read_line_as_usize(&mut input)?;
    let mut v = Vec::with_capacity(s);
    for _ in 0..s {
        let g = read_alt_bn128_g1(&mut input)?;
        consume_output_newline(&mut input)?;
        v.push(g);
    }
    Ok(v)
}
