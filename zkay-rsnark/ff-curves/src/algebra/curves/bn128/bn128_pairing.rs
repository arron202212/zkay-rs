//  Declares functions for computing Ate pairings over the bn128 curves, split into a
//  offline and online stages.

use crate::algebra::curves::bn128::{
    bn128_fields::{Fp, Fp2, Fp6, bn128_Fq12},
    bn128_g1::bn128_G1,
    bn128_g2::bn128_G2,
    bn128_gt::bn128_GT,
};
use tracing::{Level, span};

#[derive(PartialEq, Clone, Default)]
pub struct bn128_ate_G1_precomp {
    pub P: [Fp; 3],
}

pub type bn128_ate_ell_coeffs = Fp6;

#[derive(PartialEq, Clone, Default)]
pub struct bn128_ate_G2_precomp {
    pub Q: [Fp2; 3],
    pub coeffs: Vec<bn128_ate_ell_coeffs>,
}

use std::fmt;
impl fmt::Display for bn128_ate_G1_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for bn128_ate_G2_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

pub fn bn128_ate_precompute_G1(P: &bn128_G1) -> bn128_ate_G1_precomp {
    let span = span!(Level::TRACE, "Call to bn128_ate_precompute_G1").entered();

    let mut result = bn128_ate_G1_precomp::default();
    let mut P_coord = [Fp::default(); 3];
    P.fill_coord(&mut P_coord);
    // ecop::NormalizeJac(result.P, P_coord);

    span.exit();
    result
}

pub fn bn128_ate_precompute_G2(Q: &bn128_G2) -> bn128_ate_G2_precomp {
    let span = span!(Level::TRACE, "Call to bn128_ate_precompute_G2").entered();

    let mut result = bn128_ate_G2_precomp::default();
    let mut Q_coord = [Fp2::default(); 3];
    Q.fill_coord(&mut Q_coord);
    // components::precomputeG2(result.coeffs, result.Q, Q_coord);

    span.exit();
    result
}

pub fn bn128_ate_miller_loop(
    prec_P: &bn128_ate_G1_precomp,
    prec_Q: &bn128_ate_G2_precomp,
) -> bn128_Fq12 {
    let mut f = bn128_Fq12::default();
    // components::millerLoop(f.elem, prec_Q.coeffs, prec_P.P);
    f
}

pub fn bn128_double_ate_miller_loop(
    prec_P1: &bn128_ate_G1_precomp,
    prec_Q1: &bn128_ate_G2_precomp,
    prec_P2: &bn128_ate_G1_precomp,
    prec_Q2: &bn128_ate_G2_precomp,
) -> bn128_Fq12 {
    let f = bn128_Fq12::default();
    // components::millerLoop2(f.elem, prec_Q1.coeffs, prec_P1.P, prec_Q2.coeffs, prec_P2.P);
    f
}

pub fn bn128_final_exponentiation(elt: &bn128_Fq12) -> bn128_GT {
    let span = span!(Level::TRACE, "Call to bn128_final_exponentiation").entered();
    let mut eltcopy: bn128_GT = elt.clone().into();
    eltcopy.elem.final_exp();
    span.exit();
    return eltcopy;
}

use ark_bn254::{Bn254, Fq12};
use ark_ec::pairing::Pairing;


pub fn bn128_final_exponentiationa(elt: &Fq12) -> Fq12 {
    
    

    
    
    
    let result = Bn254::final_exponentiation(ark_ec::pairing::MillerLoopOutput(*elt))
        .expect("最终幂次计算失败")
        .0; 

    
    result
}



// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Bn128AteG1Precomp {
//     pub p: [Fp; 3],
// }

// impl Bn128AteG1Precomp {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         for coord in &self.p {
//             writer.write_all(&coord.to_bytes())?;
//             writer.write_all(b"\n")?;
//         }
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
//         let mut p = [Fp::zero(); 3];
//         for i in 0..3 {
//             p[i] = Fp::read(&mut reader)?;
//         }
//         Ok(Self { p })
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Bn128AteEllCoeffs {
//     pub a: Fp2,
//     pub b: Fp2,
//     pub c: Fp2,
// }

// #[derive(Clone, Debug)]
// pub struct Bn128AteG2Precomp {
//     pub q: [Fp2; 3],
//     pub coeffs: Vec<Bn128AteEllCoeffs>,
// }

// impl PartialEq for Bn128AteG2Precomp {
//     fn eq(&self, other: &Self) -> bool {
//         if self.q != other.q || self.coeffs.len() != other.coeffs.len() {
//             return false;
//         }

//         self.coeffs
//             .iter()
//             .zip(other.coeffs.iter())
//             .all(|(c1, c2)| c1 == c2)
//     }
// }

// impl Eq for Bn128AteG2Precomp {}

// impl Bn128AteG2Precomp {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         for coord in &self.q {
//             writer.write_all(&coord.a.to_bytes())?;
//             writer.write_all(b"\n")?;
//             writer.write_all(&coord.b.to_bytes())?;
//             writer.write_all(b"\n")?;
//         }

//         writer.write_all(self.coeffs.len().to_string().as_bytes())?;
//         writer.write_all(b"\n")?;

//         for c in &self.coeffs {
//             let components = [&c.a.a, &c.a.b, &c.b.a, &c.b.b, &c.c.a, &c.c.b];
//             for comp in components {
//                 writer.write_all(&comp.to_bytes())?;
//                 writer.write_all(b"\n")?;
//             }
//         }
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
//         let mut q = [Fp2::zero(); 3];
//         for i in 0..3 {
//             q[i] = Fp2 {
//                 a: Fp::read(&mut reader)?,
//                 b: Fp::read(&mut reader)?,
//             };
//         }

//         let count: usize = 68;
//         let mut coeffs = Vec::with_capacity(count);
//         for _ in 0..count {
//             coeffs.push(Bn128AteEllCoeffs {
//                 a: Fp2 {
//                     a: Fp::read(&mut reader)?,
//                     b: Fp::read(&mut reader)?,
//                 },
//                 b: Fp2 {
//                     a: Fp::read(&mut reader)?,
//                     b: Fp::read(&mut reader)?,
//                 },
//                 c: Fp2 {
//                     a: Fp::read(&mut reader)?,
//                     b: Fp::read(&mut reader)?,
//                 },
//             });
//         }

//         Ok(Self { q, coeffs })
//     }
// }
