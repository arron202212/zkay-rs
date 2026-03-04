// use ffec::algebra::fields::prime_base::fp;
// use ffec::algebra::fields::prime_extension::fp2;
// use ffec::algebra::fields::prime_extension::fp6_3over2;
// use ffec::algebra::fields::prime_extension::fp12_2over3over2;

const bls12_381_r_bitcount: usize = 255;
const bls12_381_q_bitcount: usize = 381;

const bls12_381_r_limbs: usize = (bls12_381_r_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
const bls12_381_q_limbs: usize = (bls12_381_q_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;

type bls12_381_Fr = Fp_model<bls12_381_r_limbs, bls12_381_modulus_r>;
type bls12_381_Fq = Fp_model<bls12_381_q_limbs, bls12_381_modulus_q>;
type bls12_381_Fq2 = Fp2_model<bls12_381_q_limbs, bls12_381_modulus_q>;
type bls12_381_Fq6 = Fp6_3over2_model<bls12_381_q_limbs, bls12_381_modulus_q>;
type bls12_381_Fq12 = Fp12_2over3over2_model<bls12_381_q_limbs, bls12_381_modulus_q>;
type bls12_381_GT = bls12_381_Fq12;

pub fn init_bls12_381_fields() {}

use ark_ff::{BigInteger, MontConfig, field_new};

// --- 標量域 Fr (Scalar Field) 參數 ---
// 用於處理標量乘法、多項式與 ZK 證明
pub struct FrConfig;
impl MontConfig<4> for FrConfig {
    // 模數 r = 52435875175126190479447740508185965837690552500527637822603658699938581184513
    const MODULUS: BigInt<4> = field_new!(
        Fr,
        "52435875175126190479447740508185965837690552500527637822603658699938581184513"
    );

    // 蒙哥馬利逆元素 (-1/modulus mod 2^64)
    const INVERSE: u64 = 0xfffffffeffffffff;

    // 2-Adicity (s = 32): r-1 可以被 2^32 整除
    // 這使得 BLS12-381 非常適合高效 FFT 運算
    const TWO_ADICITY: u32 = 32;

    // 乘法群生成元 (Multiplicative Generator = 7)
    const GENERATOR: BigInt<4> = field_new!(Fr, "7");
}

// --- 基域 Fq (Base Field) 參數 ---
// 用於 G1/G2 點座標運算 (381 bits 需要 6 個 u64 limbs)
pub struct FqConfig;
impl MontConfig<6> for FqConfig {
    const MODULUS: BigInt<6> = field_new!(
        Fq,
        "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787"
    );

    const INVERSE: u64 = 0x89f3fffcfffcfffd;

    // Fq 的 s = 1, 說明其不具備高效 FFT 性質，僅用於曲線算術
    const TWO_ADICITY: u32 = 1;
}

// --- 擴展域 Fq2 (Twist Field) ---
// 用於 G2 群運算，定義為 Fq[u] / (u^2 + 1)
pub struct Fq2Config;
impl Fq2Config {
    // 非剩餘元素 (Non-residue = -1)
    // 原代碼中 non_residue = bls12_381_Fq::root_of_unity (即 modulus - 1)
    pub const NONRESIDUE: Fq = field_new!(Fq, "-1");

    // Frobenius 係數用於快速計算 p-th 次冪
    pub const FROBENIUS_COEFF_C1: [Fq; 2] = [
        field_new!(Fq, "1"),
        field_new!(Fq, "-1"), // 對應原代碼中 c1[1] = modulus - 1
    ];
}

// --- 擴展域 Fq6 (Intermediate Field) ---
// 定義為 Fq2[v] / (v^3 - (u+1))
pub struct Fq6Config;
impl Fq6Config {
    // 非剩餘元素 (Non-residue = u + 1)
    // 對應原代碼 nqr = Fq2(1, 1)
    pub const NONRESIDUE: Fq2 = Fq2::new(Fq::ONE, Fq::ONE);
}
