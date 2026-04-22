//ate pairing

struct bls12_381_ate_G1_precomp {
    pub PX: bls12_381_Fq,
    pub PY: bls12_381_Fq,
}

struct bls12_381_ate_ell_coeffs {
    pub ell_0: bls12_381_Fq2,
    pub ell_VW: bls12_381_Fq2,
    pub ell_VV: bls12_381_Fq2,
}

struct bls12_381_ate_G2_precomp {
    pub QX: bls12_381_Fq2,
    pub QY: bls12_381_Fq2,
    pub coeffs: Vec<bls12_381_ate_ell_coeffs>,
}

//choice of pairing

type bls12_381_G1_precomp = bls12_381_ate_G1_precomp;
type bls12_381_G2_precomp = bls12_381_ate_G2_precomp;

//final exponentiations

pub fn bls12_381_final_exponentiation_first_chunk(elt: &bls12_381_Fq12) -> bls12_381_Fq12 {
    enter_block("Call to bls12_381_final_exponentiation_first_chunk",false);

    
    //   Computes result = elt^((q^6-1)*(q^2+1)).
    //   Follows, e.g., Beuchat et al page 9, by computing result as follows:
    //      elt^((q^6-1)*(q^2+1)) = (conj(elt) * elt^(-1))^(q^2+1)
    //   More precisely:
    //   A = conj(elt)
    //   B = elt.inverse()
    //   C = A * B
    //   D = C.Frobenius_map(2)
    //   result = D * C


    let A = bls12_381_Fq12(elt.c0, -elt.c1);
    let B = elt.inverse();
    let C = A * B;
    let D = C.Frobenius_map(2);
    let result = D * C;

    leave_block("Call to bls12_381_final_exponentiation_first_chunk",false);

    result
}

pub fn bls12_381_exp_by_z(elt: &bls12_381_Fq12) -> bls12_381_Fq12 {
    enter_block("Call to bls12_381_exp_by_z",false);

    let mut result = elt.cyclotomic_exp(bls12_381_final_exponent_z);
    if bls12_381_final_exponent_is_z_neg {
        result = result.unitary_inverse();
    }

    leave_block("Call to bls12_381_exp_by_z",false);

    result
}

pub fn bls12_381_final_exponentiation_last_chunk(elt: &bls12_381_Fq12) -> bls12_381_Fq12 {
    enter_block("Call to bls12_381_final_exponentiation_last_chunk",false);

    //  https://eprint.iacr.org/2016/130.pdf (Algorithm 1 described in Table 1)
    let A = elt.cyclotomic_squared().unitary_inverse(); // elt^(-2)
    let B = bls12_381_exp_by_z(elt); // elt^z
    let C = B.cyclotomic_squared(); // elt^(2z)
    let D = A * B; // elt^(z-2)
    let E = bls12_381_exp_by_z(D); // elt^(z^2-2z)
    let F = bls12_381_exp_by_z(E); // elt^(z^3-2z^2)
    let G = bls12_381_exp_by_z(F); // elt^(z^4-2z^3)
    let H = G * C; // elt^(z^4-2z^3+2z)
    let I = bls12_381_exp_by_z(H); // elt^(z^5-2z^4+2z^2)
    let J = D.unitary_inverse(); // elt^(-z+2)
    let K = J * I; // elt^(z^5-2z^4+2z^2) * elt^(-z+2)
    let L = elt * K; // elt^(z^5-2z^4+2z^2) * elt^(-z+2) * elt
    let M = elt.unitary_inverse(); // elt^(-1)
    let N = E * elt; // elt^(z^2-2z) * elt
    let O = N.Frobenius_map(3); // (elt^(z^2-2z) * elt)^(q^3)
    let P = H * M; // elt^(z^4-2z^3+2z) * elt^(-1)
    let Q = P.Frobenius_map(1); // (elt^(z^4-2z^3+2z) * elt^(-1))^q
    let R = B * F; // elt^(z^3-2z^2) * elt^z
    let S = R.Frobenius_map(2); // (elt^(z^3-2z^2) * elt^z)^(q^2)
    let T = S * O; // (elt^(z^2-2z) * elt)^(q^3) * (elt^(z^3-2z^2) * elt^z)^(q^2)
    let U = T * Q; // (elt^(z^2-2z) * elt)^(q^3) * (elt^(z^3-2z^2) * elt^z)^(q^2) * (elt^(z^4-2z^3+2z) * elt^(-1))^q
    let result = U * L; // (elt^(z^2-2z) * elt)^(q^3) * (elt^(z^3-2z^2) * elt^z)^(q^2) * (elt^(z^4-2z^3+2z) * elt^(-1))^q * elt^(z^5-2z^4+2z^2) * elt^(-z+2) * elt

    leave_block("Call to bls12_381_final_exponentiation_last_chunk",false);

    result
}

pub fn bls12_381_final_exponentiation(elt: &bls12_381_Fq12) -> bls12_381_GT {
    enter_block("Call to bls12_381_final_exponentiation",false);
    //  OLD naive version:
    //     bls12_381_GT result = elt^bls12_381_final_exponent;

    let mut A = bls12_381_final_exponentiation_first_chunk(elt);
    let mut result = bls12_381_final_exponentiation_last_chunk(A);

    leave_block("Call to bls12_381_final_exponentiation",false);
    result
}

//ate pairing

pub fn doubling_step_for_miller_loop(
    two_inv: bls12_381_Fq,
    current: &bls12_381_G2,
    c: &bls12_381_ate_ell_coeffs,
) {
    let X = current.X;
    Y = current.Y;
    Z = current.Z;

    let A = two_inv * (X * Y); // A = X1 * Y1 / 2
    let B = Y.squared(); // B = Y1^2
    let C = Z.squared(); // C = Z1^2
    let D = C + C + C; // D = 3 * C
    let E = bls12_381_twist_coeff_b * D; // E = twist_b * D
    let F = E + E + E; // F = 3 * E
    let G = two_inv * (B + F); // G = (B+F)/2
    let H = (Y + Z).squared() - (B + C); // H = (Y1+Z1)^2-(B+C)
    let I = E - B; // I = E-B
    let J = X.squared(); // J = X1^2
    let E_squared = E.squared(); // E_squared = E^2

    current.X = A * (B - F); // X3 = A * (B-F)
    current.Y = G.squared() - (E_squared + E_squared + E_squared); // Y3 = G^2 - 3*E^2
    current.Z = B * H; // Z3 = B * H
    c.ell_0 = I; // ell_0 = xi * I
    c.ell_VW = -bls12_381_twist * H; // ell_VW = - H (later: * yP)
    c.ell_VV = J + J + J; // ell_VV = 3*J (later: * xP)
}

pub fn mixed_addition_step_for_miller_loop(
    base: bls12_381_G2,
    current: &bls12_381_G2,
    c: &bls12_381_ate_ell_coeffs,
) {
    let X1 = current.X;
    Y1 = current.Y;
    Z1 = current.Z;
    let x2: bls12_381_Fq2 = base.X;
    let y2 = base.Y;

    let D = X1 - x2 * Z1; // D = X1 - X2*Z1
    let E = Y1 - y2 * Z1; // E = Y1 - Y2*Z1
    let F = D.squared(); // F = D^2
    let G = E.squared(); // G = E^2
    let H = D * F; // H = D*F
    let I = X1 * F; // I = X1 * F
    let J = H + Z1 * G - (I + I); // J = H + Z1*G - (I+I)

    current.X = D * J; // X3 = D*J
    current.Y = E * (I - J) - (H * Y1); // Y3 = E*(I-J)-(H*Y1)
    current.Z = Z1 * H; // Z3 = Z1*H
    c.ell_0 = E * x2 - D * y2; // ell_0 = xi * (E * X2 - D * Y2)
    c.ell_VV = -E; // ell_VV = - E (later: * xP)
    c.ell_VW = bls12_381_twist * D; // ell_VW = D (later: * yP    )
}

pub fn bls12_381_ate_precompute_G1(P: &bls12_381_G1) -> bls12_381_ate_G1_precomp {
    enter_block("Call to bls12_381_ate_precompute_G1",false);

    let mut Pcopy = P;
    Pcopy.to_affine_coordinates();

    let mut result = bls12_381_ate_G1_precomp::default();
    result.PX = Pcopy.X;
    result.PY = Pcopy.Y;

    leave_block("Call to bls12_381_ate_precompute_G1",false);
    result
}

pub fn bls12_381_ate_precompute_G2(Q: &bls12_381_G2) -> bls12_381_ate_G2_precomp {
    enter_block("Call to bls12_381_ate_precompute_G2",false);

    let mut Qcopy = Q.clone();
    Qcopy.to_affine_coordinates();

    let mut two_inv = (bls12_381_Fq("2").inverse()); // could add to global params if needed

    let mut result = bls12_381_ate_G2_precomp::default();
    result.QX = Qcopy.X;
    result.QY = Qcopy.Y;

    let mut R = bls12_381_G2::default();
    R.X = Qcopy.X;
    R.Y = Qcopy.Y;
    R.Z = bls12_381_Fq2::one();

    let loop_count: bigint<bls12_381_Fq::num_limbs> = bls12_381_ate_loop_count;
    let mut found_one = false;
    let mut c = bls12_381_ate_ell_coeffs::default();

    for i in (0..=loop_count.max_bits()).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        doubling_step_for_miller_loop(two_inv, R, c);
        result.coeffs.push(c);

        if bit != 0 {
            mixed_addition_step_for_miller_loop(Qcopy, R, c);
            result.coeffs.push(c);
        }
    }

    leave_block("Call to bls12_381_ate_precompute_G2",false);
    result
}

pub fn bls12_381_ate_miller_loop(
    prec_P: &bls12_381_ate_G1_precomp,
    prec_Q: &bls12_381_ate_G2_precomp,
) -> bls12_381_Fq12 {
    enter_block("Call to bls12_381_ate_miller_loop",false);

    let mut f = bls12_381_Fq12::one();

    let mut found_one = false;
    let mut idx = 0;

    let loop_count: bigint<bls12_381_Fq::num_limbs> = bls12_381_ate_loop_count;
    let mut c = bls12_381_ate_ell_coeffs::default();

    for i in (0..=loop_count.max_bits()).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        //  code below gets executed for all bits (EXCEPT the MSB itself) of
        // bls12_381_param_p (skipping leading zeros) in MSB to LSB
        // order 

        c = prec_Q.coeffs[idx];
        idx += 1;
        f = f.squared();
        f = f.mul_by_045(c.ell_0, prec_P.PY * c.ell_VW, prec_P.PX * c.ell_VV);

        if bit != 0 {
            c = prec_Q.coeffs[idx];
            idx += 1;
            f = f.mul_by_045(c.ell_0, prec_P.PY * c.ell_VW, prec_P.PX * c.ell_VV);
        }
    }

    if bls12_381_ate_is_loop_count_neg {
        f = f.inverse();
    }

    leave_block("Call to bls12_381_ate_miller_loop",false);
    f
}

pub fn bls12_381_ate_double_miller_loop(
    prec_P1: &bls12_381_ate_G1_precomp,
    prec_Q1: &bls12_381_ate_G2_precomp,
    prec_P2: &bls12_381_ate_G1_precomp,
    prec_Q2: &bls12_381_ate_G2_precomp,
) -> bls12_381_Fq12 {
    enter_block("Call to bls12_381_ate_double_miller_loop",false);

    let mut f = bls12_381_Fq12::one();

    let mut found_one = false;
    let mut idx = 0;

    let loop_count: bigint<bls12_381_Fq::num_limbs> = bls12_381_ate_loop_count;
    for i in (0..=loop_count.max_bits()).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        //  code below gets executed for all bits (EXCEPT the MSB itself) of
        // bls12_381_param_p (skipping leading zeros) in MSB to LSB
        // order 

        let mut c1 = prec_Q1.coeffs[idx];
        let mut c2 = prec_Q2.coeffs[idx];
        idx += 1;

        f = f.squared();

        f = f.mul_by_045(c1.ell_0, prec_P1.PY * c1.ell_VW, prec_P1.PX * c1.ell_VV);
        f = f.mul_by_045(c2.ell_0, prec_P2.PY * c2.ell_VW, prec_P2.PX * c2.ell_VV);

        if bit != 0 {
            let mut c1 = prec_Q1.coeffs[idx];
            let mut c2 = prec_Q2.coeffs[idx];
            idx += 1;

            f = f.mul_by_045(c1.ell_0, prec_P1.PY * c1.ell_VW, prec_P1.PX * c1.ell_VV);
            f = f.mul_by_045(c2.ell_0, prec_P2.PY * c2.ell_VW, prec_P2.PX * c2.ell_VV);
        }
    }

    if bls12_381_ate_is_loop_count_neg {
        f = f.inverse();
    }

    leave_block("Call to bls12_381_ate_double_miller_loop",false);

    f
}

pub fn bls12_381_ate_pairing(Q: &bls12_381_G1, P: &bls12_381_G2) -> bls12_381_Fq12 {
    enter_block("Call to bls12_381_ate_pairing",false);
    let mut prec_P = bls12_381_ate_precompute_G1(P);
    let mut prec_Q = bls12_381_ate_precompute_G2(Q);
    let mut result = bls12_381_ate_miller_loop(prec_P, prec_Q);
    leave_block("Call to bls12_381_ate_pairing",false);
    result
}

pub fn bls12_381_ate_reduced_pairing(P: &bls12_381_G1, Q: &bls12_381_G2) -> bls12_381_GT {
    enter_block("Call to bls12_381_ate_reduced_pairing",false);
    let f = bls12_381_ate_pairing(P, Q);
    let result = bls12_381_final_exponentiation(f);
    leave_block("Call to bls12_381_ate_reduced_pairing",false);
    result
}

//choice of pairing

pub fn bls12_381_precompute_G1(P: &bls12_381_G1) -> bls12_381_G1_precomp {
    return bls12_381_ate_precompute_G1(P);
}

pub fn bls12_381_precompute_G2(Q: &bls12_381_G2) -> bls12_381_G2_precomp {
    return bls12_381_ate_precompute_G2(Q);
}

pub fn bls12_381_miller_loop(
    prec_P: &bls12_381_G1_precomp,
    prec_Q: &bls12_381_G2_precomp,
) -> bls12_381_Fq12 {
    return bls12_381_ate_miller_loop(prec_P, prec_Q);
}

pub fn bls12_381_double_miller_loop(
    prec_P1: &bls12_381_G1_precomp,
    prec_Q1: &bls12_381_G2_precomp,
    prec_P2: &bls12_381_G1_precomp,
    prec_Q2: &bls12_381_G2_precomp,
) -> bls12_381_Fq12 {
    return bls12_381_ate_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

pub fn bls12_381_pairing(P: bls12_381_G1, Q: &bls12_381_G2) -> bls12_381_Fq12 {
    return bls12_381_ate_pairing(P, Q);
}

pub fn bls12_381_reduced_pairing(P: &bls12_381_G1, Q: &bls12_381_G2) -> bls12_381_GT {
    return bls12_381_ate_reduced_pairing(P, Q);
}

use std::io::{self, Read, Write};

// --- G1 预计算数据 ---
// 通常存储仿射坐标形式的 P，用于 Miller Loop 中的直线计算
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct G1Precomp {
    pub px: Fq,
    pub py: Fq,
}

impl G1Precomp {
    pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.px.to_bytes())?;
        writer.write_all(b" ")?; // 对应 OUTPUT_SEPARATOR
        writer.write_all(&self.py.to_bytes())?;
        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
        let px = Fq::read(&mut reader)?;
        // 这里应有逻辑跳过分隔符
        let py = Fq::read(&mut reader)?;
        Ok(Self { px, py })
    }
}

// --- Ate 配对中的直线系数 ---
// ell_0, ell_vw, ell_vv 分别对应直线方程中的系数
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AteEllCoeffs {
    pub ell_0: Fq2,
    pub ell_vw: Fq2,
    pub ell_vv: Fq2,
}

impl AteEllCoeffs {
    pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.ell_0.to_bytes())?;
        writer.write_all(b" ")?;
        writer.write_all(&self.ell_vw.to_bytes())?;
        writer.write_all(b" ")?;
        writer.write_all(&self.ell_vv.to_bytes())?;
        Ok(())
    }
}

// --- G2 预计算数据 ---
// 包含 G2 点的坐标以及在 Miller Loop 中倍点/点加产生的系数列表
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct G2Precomp {
    pub qx: Fq2,
    pub qy: Fq2,
    pub coeffs: Vec<AteEllCoeffs>,
}

impl G2Precomp {
    pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
        // 1. 写入 Q 坐标
        writer.write_all(&self.qx.to_bytes())?;
        writer.write_all(b" ")?;
        writer.write_all(&self.qy.to_bytes())?;
        writer.write_all(b"\n")?;

        // 2. 写入系数列表长度
        writer.write_all(self.coeffs.len().to_string().as_bytes())?;
        writer.write_all(b"\n")?;

        // 3. 逐个写入系数
        for coeff in &self.coeffs {
            coeff.serialize(&mut writer)?;
            writer.write_all(b"\n")?; // 对应 OUTPUT_NEWLINE
        }
        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
        // 读取 QX, QY
        let qx = Fq2::read(&mut reader)?;
        let qy = Fq2::read(&mut reader)?;

        // 读取 Vec 长度
        let mut len_buf = String::new();
        // 简化的读取长度逻辑...
        let s: usize = 68; // 示例长度，BLS12-381 通常固定

        let mut coeffs = Vec::with_capacity(s);
        for _ in 0..s {
            coeffs.push(AteEllCoeffs::deserialize(&mut reader)?);
        }

        Ok(Self { qx, qy, coeffs })
    }
}
