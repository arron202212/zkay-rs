pub fn init_bn128_params() {}

// use ark_bn254::{Config, Fq, Fq2, Fr, G1Projective, G2Projective};
// use ark_ff::{BigInteger, Field, Fp2, PrimeField};

// pub struct Bn128Config;

// impl Config for Bn128Config {
//     type BaseField = Fq;
//     type ScalarField = Fr;

//     const COEFF_B: Fq = field_new!(Fq, "3");

//     const G1_GENERATOR: G1Projective = G1Projective::new(Fq::ONE, field_new!(Fq, "2"), Fq::ONE);

//     const G2_COEFF_B: Fq2 = Fq2::new(
//         field_new!(
//             Fq,
//             "19485874751759354771024239261021720505790618469301721065564631296452457478373"
//         ),
//         field_new!(
//             Fq,
//             "266929791119991161246907387137283842545076965332900288569378510910307636690"
//         ),
//     );
// }

// pub const G1_WNAF_WINDOW_TABLE: [usize; 4] = [10, 24, 40, 132];

// pub const G1_FIXED_BASE_WINDOW_TABLE: [usize; 22] = [
//     1, 4, 10, 25, 62, 158, 362, 807, 2090, 4460, 9280, 43303, 0, 0, 210999, 506869, 930023, 0, 0,
//     8350812, 21708139, 29482996,
// ];

// pub const G2_GENERATOR: G2Projective = G2Projective::new(
//     Fq2::new(
//         field_new!(
//             Fq,
//             "15267802884793550383558706039165621050290089775961208824303765753922461897946"
//         ),
//         field_new!(
//             Fq,
//             "9034493566019742339402378670461897774509967669562610788113215988055021632533"
//         ),
//     ),
//     Fq2::new(
//         field_new!(
//             Fq,
//             "644888581738283025171396578091639672120333224302184904896215738366765861164"
//         ),
//         field_new!(
//             Fq,
//             "20532875081203448695448744255224543661959516361327385779878476709582931298750"
//         ),
//     ),
//     Fq2::ONE,
// );

// pub const G2_COFACTOR: [u64; 4] = [0x321549];
