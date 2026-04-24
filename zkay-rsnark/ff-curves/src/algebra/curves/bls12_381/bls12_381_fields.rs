

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



pub struct FrConfig;
impl MontConfig<4> for FrConfig {
    
    const MODULUS: BigInt<4> = field_new!(
        Fr,
        "52435875175126190479447740508185965837690552500527637822603658699938581184513"
    );

    
    const INVERSE: u64 = 0xfffffffeffffffff;

    
    
    const TWO_ADICITY: u32 = 32;

    
    const GENERATOR: BigInt<4> = field_new!(Fr, "7");
}



pub struct FqConfig;
impl MontConfig<6> for FqConfig {
    const MODULUS: BigInt<6> = field_new!(
        Fq,
        "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787"
    );

    const INVERSE: u64 = 0x89f3fffcfffcfffd;

    
    const TWO_ADICITY: u32 = 1;
}



pub struct Fq2Config;
impl Fq2Config {
    
    
    pub const NONRESIDUE: Fq = field_new!(Fq, "-1");

    
    pub const FROBENIUS_COEFF_C1: [Fq; 2] = [
        field_new!(Fq, "1"),
        field_new!(Fq, "-1"), 
    ];
}



pub struct Fq6Config;
impl Fq6Config {
    
    
    pub const NONRESIDUE: Fq2 = Fq2::new(Fq::ONE, Fq::ONE);
}
