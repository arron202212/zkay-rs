
type base_field = bls12_381_Fq;
type twist_field = bls12_381_Fq2;
type scalar_field = bls12_381_Fr;

pub struct bls12_381_G2 {
    X: bls12_381_Fq2,
    Y: bls12_381_Fq2,
    Z: bls12_381_Fq2,
}
impl bls12_381_G2 {
    // using Jacobian coordinates

    pub fn new(X: bls12_381_Fq2, Y: bls12_381_Fq2, Z: bls12_381_Fq2) -> Self {
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

impl bls12_381_G2 {
    pub fn new() {
        self.X = G2_zero.X;
        self.Y = G2_zero.Y;
        self.Z = G2_zero.Z;
    }

    pub fn mul_by_b(elt: &bls12_381_Fq2) -> bls12_381_Fq2 {
        return bls12_381_Fq2(
            bls12_381_twist_mul_by_b_c0 * elt.c0,
            bls12_381_twist_mul_by_b_c1 * elt.c1,
        );
    }

    pub fn print() {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            print!(
                "(%Nd*z + %Nd , %Nd*z + %Nd)\n",
                copy.X.c1.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                copy.X.c0.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                copy.Y.c1.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                copy.Y.c0.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates() {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "(%Nd*z + %Nd : %Nd*z + %Nd : %Nd*z + %Nd)\n",
                self.X.c1.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.X.c0.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.Y.c1.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.Y.c0.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.Z.c1.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs,
                self.Z.c0.as_bigint().0.0[0],
                bls12_381_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates() {
        if self.is_zero() {
            self.X = bls12_381_Fq2::zero();
            self.Y = bls12_381_Fq2::one();
            self.Z = bls12_381_Fq2::zero();
        } else {
            let mut Z_inv = Z.inverse();
            let mut Z2_inv = Z_inv.squared();
            let mut Z3_inv = Z2_inv * Z_inv;
            self.X = self.X * Z2_inv;
            self.Y = self.Y * Z3_inv;
            self.Z = bls12_381_Fq2::one();
        }
    }

    pub fn to_special() {
        self.to_affine_coordinates();
    }

    pub fn is_special() -> bool {
        return (self.is_zero() || self.Z == bls12_381_Fq2::one());
    }

    pub fn is_zero() -> bool {
        return (self.Z.is_zero());
    }

    pub fn add(other: &bls12_381_G2) -> bls12_381_G2 {
        return self.clone() + other;
    }

    pub fn mixed_add(other: &bls12_381_G2) -> bls12_381_G2 {
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

        let U1: bls12_381_Fq2 = self.X;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = (self.Z) * Z1Z1;

        let S1: bls12_381_Fq2 = (self.Y); // S1 = Y1 * Z2 * Z2Z2
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
        let mut HH = H.squared(); // HH = H&2
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

        return bls12_381_G2(X3, Y3, Z3);
    }

    pub fn dbl() -> bls12_381_G2 {
        // #ifdef PROFILE_OP_COUNTS
        self.dbl_cnt += 1;

        // handle point at infinity
        if self.is_zero() {
            return self.clone();
        }

        // NOTE: does not handle O and pts of order 2,4
        // https://www.hyperelliptic.org/EFD/g1p/data/shortw/jacobian-0/doubling/dbl-2009-l

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

        return bls12_381_G2(X3, Y3, Z3);
    }

    pub fn mul_by_q() -> bls12_381_G2 {
        return bls12_381_G2(
            bls12_381_twist_mul_by_q_X * (self.X).Frobenius_map(1),
            bls12_381_twist_mul_by_q_Y * (self.Y).Frobenius_map(1),
            (self.Z).Frobenius_map(1),
        );
    }

    pub fn mul_by_cofactor() -> bls12_381_G2 {
        return bls12_381_G2::h * self.clone();
    }

    pub fn is_well_formed() -> bool {
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
        let mut X2 = self.X.squared();
        let mut Y2 = self.Y.squared();
        let mut Z2 = self.Z.squared();

        let mut X3 = self.X * X2;
        let mut Z3 = self.Z * Z2;
        let mut Z6 = Z3.squared();

        return (Y2 == X3 + bls12_381_twist_coeff_b * Z6);
    }

    pub fn zero() -> Self {
        return G2_zero;
    }

    pub fn one() -> Self {
        return G2_one;
    }

    pub fn random_element() -> Self {
        return (bls12_381_Fr::random_element().as_bigint()) * G2_one;
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<bls12_381_G2>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z.clone());
        }
        batch_invert::<bls12_381_Fq2>(Z_vec);

        let one = bls12_381_Fq2::one();

        for i in 0..vec.len() {
            let mut Z2 = Z_vec[i].squared();
            let mut Z3 = Z_vec[i] * Z2;

            vec[i].X = vec[i].X * Z2;
            vec[i].Y = vec[i].Y * Z3;
            vec[i].Z = one;
        }
    }
}

use std::io::{self, Read, Write};
use std::ops::{Add, Mul, Neg, Sub};

// G2 使用的是扩展域 Fq2
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct G2Projective {
    pub x: Fq2,
    pub y: Fq2,
    pub z: Fq2,
}

impl G2Projective {
    pub fn zero() -> Self { //...
    }
    pub fn is_zero(&self) -> bool {
        self.z.is_zero()
    }

    // 将 Jacobian 转换为仿射坐标 (x, y)
    pub fn to_affine(&self) -> (Fq2, Fq2, bool) {
        if self.is_zero() {
            return (Fq2::zero(), Fq2::zero(), true);
        }
        let z_inv = self.z.inverse().unwrap();
        let z_inv2 = z_inv.square();
        let z_inv3 = z_inv2 * &z_inv;
        (self.x * &z_inv2, self.y * &z_inv3, false)
    }

    pub fn dbl(&self) -> Self {
        // Jacobian 坐标下的倍点算法
        // ...
        todo!()
    }
}

// 1. 标量乘法 (lhs: Scalar * rhs: G2)
impl<'a> Mul<&'a G2Projective> for &'a BigInt {
    type Output = G2Projective;
    fn mul(self, rhs: &'a G2Projective) -> G2Projective {
        rhs.scalar_mul(self)
    }
}

// 2. 相等性比较 (Jacobian 交叉乘法)
impl PartialEq for G2Projective {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() {
            return other.is_zero();
        }
        if other.is_zero() {
            return false;
        }

        let z1_2 = self.z.square();
        let z2_2 = other.z.square();
        if self.x * &z2_2 != other.x * &z1_2 {
            return false;
        }

        let z1_3 = z1_2 * &self.z;
        let z2_3 = z2_2 * &other.z;
        self.y * &z2_3 == other.y * &z1_3
    }
}

// 3. 点加 (Jacobian Addition)
impl<'a> Add<&'a G2Projective> for &'a G2Projective {
    type Output = G2Projective;
    fn add(self, other: &'a G2Projective) -> G2Projective {
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
                return G2Projective::zero();
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

        G2Projective {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}

// 4. 取负与减法
impl Neg for G2Projective {
    type Output = Self;
    fn neg(self) -> Self {
        G2Projective {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl<'a> Sub<&'a G2Projective> for &'a G2Projective {
    type Output = G2Projective;
    fn sub(self, other: &'a G2Projective) -> G2Projective {
        self + &(-*other)
    }
}

// 5. 序列化 (带有 Fq2 压缩逻辑)
impl G2Projective {
    pub fn serialize<W: Write>(&self, mut writer: W, compress: bool) -> io::Result<()> {
        let (x, y, is_zero) = self.to_affine();
        writer.write_all(if is_zero { b"1" } else { b"0" })?;

        // 对应代码: out << copy.X << (copy.Y.c0.as_bigint() & 1)
        writer.write_all(&x.to_bytes())?; // 写入 Fq2 的 X 坐标

        if compress {
            // 在 G2 压缩中，通常根据 y 的 c0 分量的 LSB 来判断
            let y_c0_lsb = if y.c0.to_bigint().is_odd() {
                b"1"
            } else {
                b"0"
            };
            writer.write_all(y_c0_lsb)?;
        } else {
            writer.write_all(&y.to_bytes())?;
        }
        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut zero_buf = [0u8; 1];
        reader.read_exact(&mut zero_buf)?;
        let is_zero = zero_buf == b'1';

        if is_zero {
            return Ok(Self::zero());
        }

        let t_x = Fq2::read(&mut reader)?; // 读取 X (Fq2)

        let mut lsb_buf = [0u8; 1];
        reader.read_exact(&mut lsb_buf)?;
        let y_lsb = lsb_buf[0] - b'0';

        // 对应代码: y = sqrt(x^3 + twist_b)
        let t_x2 = t_x.square();
        let t_y2 = t_x2 * &t_x + &TWIST_COEFF_B;
        let mut t_y = t_y2
            .sqrt()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "No sqrt"))?;

        // 检查 c0 的 LSB
        if (t_y.c0.to_bigint().is_odd() as u8) != y_lsb {
            t_y = -t_y;
        }

        Ok(Self {
            x: t_x,
            y: t_y,
            z: Fq2::one(),
        })
    }
}
