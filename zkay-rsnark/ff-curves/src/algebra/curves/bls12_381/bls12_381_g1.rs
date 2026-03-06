use ffec::field_utils::{bigint::{GMP_NUMB_BITS,bigint},field_utils::batch_invert};
use ffec::{Fp_modelConfig,PpConfig,
    Fp_model, Fp2_model, Fp2_modelConfig, Fp3_modelConfig, Fp6_3over2_model, Fp6_modelConfig,
    Fp12_2over3over2_model, Fp12_modelConfig,One, Zero
};

use std::borrow::Borrow;
use std::ops::{Add, Mul, Neg, Sub};

type base_field = bls12_381_Fq;
type scalar_field = bls12_381_Fr;

pub struct bls12_381_G1 {
    X: bls12_381_Fq,
    Y: bls12_381_Fq,
    Z: bls12_381_Fq,
}

impl bls12_381_G1 {
    pub fn new(X: bls12_381_Fq, Y: bls12_381_Fq, Z: bls12_381_Fq) -> Self {
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

impl bls12_381_G1 {
    pub fn new() {
        self.X = G1_zero.X;
        self.Y = G1_zero.Y;
        self.Z = G1_zero.Z;
    }

    pub fn print() {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            print!(
                "(%Nd , %Nd)\n",
                copy.X.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                copy.Y.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates() {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "(%Nd : %Nd : %Nd)\n",
                self.X.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.Y.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.Z.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates() {
        if self.is_zero() {
            self.X = bls12_381_Fq::zero();
            self.Y = bls12_381_Fq::one();
            self.Z = bls12_381_Fq::zero();
        } else {
            let Z_inv = Z.inverse();
            let Z2_inv = Z_inv.squared();
            let Z3_inv = Z2_inv * Z_inv;
            self.X = self.X * Z2_inv;
            self.Y = self.Y * Z3_inv;
            self.Z = bls12_381_Fq::one();
        }
    }

    pub fn to_special() {
        self.to_affine_coordinates();
    }

    pub fn is_special() -> bool {
        return (self.is_zero() || self.Z == bls12_381_Fq::one());
    }

    pub fn is_zero() -> bool {
        return (self.Z.is_zero());
    }

    pub fn add(other: &bls12_381_G1) -> bls12_381_G1 {
        return self.clone() + other;
    }

    pub fn mixed_add(other: &bls12_381_G1) -> bls12_381_G1 {
        // #ifdef DEBUG
        assert!(other.is_special());

        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone()
        }

        if other.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // check for doubling case

        // using Jacobian coordinates so:
        // (X1:Y1:Z1) = (X2:Y2:Z2)
        // iff
        // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
        // iff
        // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

        // we know that Z2 = 1

        let Z1Z1 = (self.Z).squared();

        let U1: bls12_381_Fq = self.X;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = (self.Z) * Z1Z1;

        let S1: bls12_381_Fq = (self.Y); // S1 = Y1 * Z2 * Z2Z2
        let S2 = (other.Y) * Z1_cubed; // S2 = Y2 * Z1 * Z1Z1

        if U1 == U2 && S1 == S2 {
            // dbl case; nothing of above can be reused
            return self.dbl();
        }

        // #ifdef PROFILE_OP_COUNTS
        self.add_cnt += 1;

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-madd-2007-bl
        let mut H = U2 - (self.X); // H = U2-X1
        let mut HH = H.squared(); // HH = H^2
        let mut I = HH + HH; // I = 4*HH
        I = I + I;
        let mut J = H * I; // J = H*I
        let mut r = S2 - (self.Y); // r = 2*(S2-Y1)
        r = r + r;
        let mut V = (self.X) * I; // V = X1*I
        let mut X3 = r.squared() - J - V - V; // X3 = r^2-J-2*V
        let mut Y3 = (self.Y) * J; // Y3 = r*(V-X3)-2*Y1*J
        Y3 = r * (V - X3) - Y3 - Y3;
        let mut Z3 = ((self.Z) + H).squared() - Z1Z1 - HH; // Z3 = (Z1+H)^2-Z1Z1-HH

        return bls12_381_G1(X3, Y3, Z3);
    }

    pub fn dbl() -> bls12_381_G1 {
        // #ifdef PROFILE_OP_COUNTS
        self.dbl_cnt += 1;

        // handle point at infinity
        if self.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // NOTE: does not handle O and pts of order 2,4
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l

        let mut A = (self.X).squared(); // A = X1^2
        let mut B = (self.Y).squared(); // B = Y1^2
        let mut C = B.squared(); // C = B^2
        let mut D = (self.X + B).squared() - A - C;
        D = D + D; // D = 2 * ((X1 + B)^2 - A - C)
        let mut E = A + A + A; // E = 3 * A
        let mut F = E.squared(); // F = E^2
        let mut X3 = F - (D + D); // X3 = F - 2 D
        let mut eightC = C + C;
        eightC = eightC + eightC;
        eightC = eightC + eightC;
        let mut Y3 = E * (D - X3) - eightC; // Y3 = E * (D - X3) - 8 * C
        let mut Y1Z1 = (self.Y) * (self.Z);
        let mut Z3 = Y1Z1 + Y1Z1; // Z3 = 2 * Y1 * Z1

        return bls12_381_G1(X3, Y3, Z3);
    }

    pub fn mul_by_cofactor() -> bls12_381_G1 {
        return bls12_381_G1::h * self.clone();
    }

    pub fn is_well_formed() -> bool {
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

        return (Y2 == X3 + bls12_381_coeff_b * Z6);
    }

    pub fn zero() -> Self {
        return G1_zero;
    }

    pub fn one() -> Self {
        return G1_one;
    }

    pub fn random_element() -> Self {
        return (scalar_field::random_element().as_bigint()) * G1_one;
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<bls12_381_G1>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z.clone());
        }
        batch_invert::<bls12_381_Fq>(Z_vec);

        let one = bls12_381_Fq::one();

        for i in 0..vec.len() {
            let Z2 = Z_vec[i].squared();
            let Z3 = Z_vec[i] * Z2;

            vec[i].X = vec[i].X * Z2;
            vec[i].Y = vec[i].Y * Z3;
            vec[i].Z = one;
        }
    }
}

use std::io::{self, Read, Write};
use std::ops::{Add, Mul, Neg, Sub};

// 假设已经定义了底层的 Fq (Base Field) 和 BigInt
// 这里映射代码中的 bls12_381_G1 结构
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct G1Projective {
    pub x: Fq,
    pub y: Fq,
    pub z: Fq,
}

impl G1Projective {
    pub fn zero() -> Self { /* ... */
    }
    pub fn is_zero(&self) -> bool {
        self.z.is_zero()
    }

    pub fn dbl(&self) -> Self {
        // 实现倍点逻辑
        // ...
    }

    // 对应代码中的 to_affine_coordinates 逻辑
    pub fn to_affine(&self) -> (Fq, Fq, bool) {
        if self.is_zero() {
            return (Fq::zero(), Fq::zero(), true);
        }
        let z_inv = self.z.inverse().unwrap();
        let z_inv2 = z_inv.square();
        let z_inv3 = z_inv2 * &z_inv;
        (self.x * &z_inv2, self.y * &z_inv3, false)
    }
}

// 1. 标量乘法 (lhs: &BigInt * rhs: &G1)
impl<'a> Mul<&'a G1Projective> for &'a BigInt {
    type Output = G1Projective;
    fn mul(self, rhs: &'a G1Projective) -> G1Projective {
        // 调用底层的标量乘法实现 (如二进制展开法)
        rhs.scalar_mul(self)
    }
}

// 2. 相等性判断 (Jacobian 比较)
impl PartialEq for G1Projective {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() {
            return other.is_zero();
        }
        if other.is_zero() {
            return false;
        }

        let z1_2 = self.z.square();
        let z2_2 = other.z.square();

        // X1 * Z2^2 == X2 * Z1^2
        if self.x * &z2_2 != other.x * &z1_2 {
            return false;
        }

        let z1_3 = z1_2 * &self.z;
        let z2_3 = z2_2 * &other.z;

        // Y1 * Z2^3 == Y2 * Z1^3
        self.y * &z2_3 == other.y * &z1_3
    }
}

// 3. 点加 (Jacobian Addition)
impl<'a> Add<&'a G1Projective> for &'a G1Projective {
    type Output = G1Projective;
    fn add(self, other: &'a G1Projective) -> G1Projective {
        if self.is_zero() {
            return *other;
        }
        if other.is_zero() {
            return *self;
        }

        let z1z1 = self.z.square();
        let z2z2 = other.z.square();

        let u1 = self.x * &z2z2;
        let u2 = other.x * &z1z1;

        let s1 = self.y * &(z2z2 * &other.z);
        let s2 = other.y * &(z1z1 * &self.z);

        if u1 == u2 {
            if s1 == s2 {
                return self.dbl();
            } else {
                return G1Projective::zero();
            }
        }

        let h = u2 - &u1;
        let r = (s2 - &s1).double();
        let i = h.double().square();
        let j = h * &i;
        let v = u1 * &i;

        let x3 = r.square() - &j - &v.double();
        let y3 = r * &(v - &x3) - &(s1 * &j).double();
        let z3 = ((self.z + &other.z).square() - &z1z1 - &z2z2) * &h;

        G1Projective {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}

// 4. 取负与减法
impl Neg for G1Projective {
    type Output = Self;
    fn neg(self) -> Self {
        G1Projective {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl<'a> Sub<&'a G1Projective> for &'a G1Projective {
    type Output = G1Projective;
    fn sub(self, other: &'a G1Projective) -> G1Projective {
        self + &(-*other)
    }
}

// 5. 序列化 (带有压缩逻辑)
impl G1Projective {
    pub fn serialize<W: Write>(&self, mut writer: W, compress: bool) -> io::Result<()> {
        let (x, y, is_zero) = self.to_affine();
        writer.write_all(if is_zero { b"1" } else { b"0" })?;

        if compress {
            // 写入 X 和 Y 的最低有效位
            writer.write_all(&x.to_bytes())?;
            let y_lsb = if y.to_bigint().is_odd() { b"1" } else { b"0" };
            writer.write_all(y_lsb)?;
        } else {
            writer.write_all(&x.to_bytes())?;
            writer.write_all(&y.to_bytes())?;
        }
        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let is_zero = buf[0] == b'1';

        if is_zero {
            return Ok(Self::zero());
        }

        // 解析 X 和根据 LSB 恢复 Y 的逻辑...
        // let x = Fq::read(&mut reader)?;
        // ... sqrt 恢复 ...

        Ok(Self {
            x: todo!(),
            y: todo!(),
            z: Fq::one(),
        })
    }
}
