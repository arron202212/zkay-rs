pub fn init_bls12_381_params() {
    init_bls12_381_fields();
}

use ark_bls12_381::{Config, Fq, Fq2, G1Projective, G2Projective};
use ark_ff::{Field, field_new};

// --- BLS12-381 曲線配置 ---
pub struct Bls12_381Config;

impl Config for Bls12_381Config {
    // 曲線方程 y^2 = x^3 + 4
    const COEFF_B: Fq = field_new!(Fq, "4");

    // G1 產生元 (G1_one)
    const G1_GENERATOR: G1Projective = G1Projective::new(
        field_new!(
            Fq,
            "3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507"
        ),
        field_new!(
            Fq,
            "1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569"
        ),
        Fq::ONE,
    );

    // G2 扭曲係數 (Twist Coeff B = COEFF_B * (1 + i))
    const G2_COEFF_B: Fq2 = Fq2::new(field_new!(Fq, "4"), field_new!(Fq, "4"));
}

// --- 預計算與優化參數 ---

// G1 餘因子 (Cofactor h)
pub const G1_COFACTOR: [u64; 1] = [76329603384216526031706109802092473003];

// G1 w-NAF 窗口表
pub const G1_WNAF_WINDOW_TABLE: [usize; 4] = [11, 24, 60, 127];

// G2 產生元 (G2_one)
pub const G2_GENERATOR: G2Projective = G2Projective::new(
    Fq2::new(
        field_new!(
            Fq,
            "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160"
        ),
        field_new!(
            Fq,
            "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758"
        ),
    ),
    Fq2::new(
        field_new!(
            Fq,
            "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905"
        ),
        field_new!(
            Fq,
            "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582"
        ),
    ),
    Fq2::ONE,
);
