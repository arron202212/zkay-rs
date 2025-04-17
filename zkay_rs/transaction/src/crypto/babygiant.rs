#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use ark_ed_on_bn254::{EdwardsAffine as BabyJubJub, EdwardsConfig, Fq, Fr};
use ark_ff::{BigInteger, BigInteger256, PrimeField};
// use ark_ec::twisted_edwards::{AffineCurve, ProjectiveCurve};
use ark_ec::CurveGroup;
use ark_ec::twisted_edwards::{Affine, Projective};
use hex;
use std::collections::HashMap;
use std::ops::Mul;
use std::str::FromStr;
pub fn baby_giant(
    max_bitwidth: u64,
    a: &Affine<EdwardsConfig>,
    b: &Projective<EdwardsConfig>,
) -> u64 {
    let m = 1u64 << (max_bitwidth / 2);
    println!("===m======{m},{a:?},{b:?}");
    let mut table = HashMap::new();
    for j in 0u64..m {
        // NOTE: equality and hashing (used for HashMap) does not perform as expected
        // for projective representation (because coordinates are ambiguous), so switching
        // to affine coordinates here
        let v = a.mul(Fr::new(BigInteger256::from(j))).into_affine();
        table.insert(v, j);
    }
    let am = a.mul(Fr::new(BigInteger256::from(m)));
    let mut gamma = b.clone();
    println!("==am====={am},{gamma:?}");
    for i in 0u64..m {
        if let Some(j) = table.get(&gamma.into_affine()) {
            return i * m + j;
        }
        gamma = gamma - &am;
        // println!("===,{gamma:?}");
    }

    panic!("No discrete log found");
}

pub fn compute_dlog(x: &str, y: &str) -> eyre::Result<String> {
    let res = do_compute_dlog(x, y);
    Ok(res.to_string())
}

fn parse_le_bytes_str(s: &str) -> BigInteger256 {
    let mut buffer = [0u8; 32]; // 32 bytes for 256 bits
    println!("===parse_le_bytes_str======s========={s}");
    let v = hex::decode(s).unwrap();
    println!("===parse_le_bytes_str======v========={v:?}");
    assert_eq!(v.len(), 32);
    let v = v.as_slice();
    for i in 0..32 {
        buffer[i] = v[i];
    }

    // let mut bi = BigInteger256::new([0; 4]);
    // bi.read_le(&mut buffer.as_ref()).unwrap();
    //  bi
    //    let v= BigInteger256::from_bits_le(&buffer.iter().flat_map(|b8| (0..8).rev().map(|i| (1<<i)&b8 != 0).collect::<Vec<_>>()).collect::<Vec<_>>())
    BigInteger256::try_from(num_bigint::BigUint::from_bytes_le(&buffer)).unwrap()
}

pub fn do_compute_dlog(x: &str, y: &str) -> u64 {
    // x and y are in little-endian hex string format

    let gx = Fq::from_str(
        "11904062828411472290643689191857696496057424932476499415469791423656658550213",
    )
    .unwrap();
    let gy = Fq::from_str(
        "9356450144216313082194365820021861619676443907964402770398322487858544118183",
    )
    .unwrap();
    let a = BabyJubJub::new(gx, gy);
    assert!(BabyJubJub::is_on_curve(&a));
    assert!(BabyJubJub::is_in_correct_subgroup_assuming_on_curve(&a));
    println!("==babygain===========do_compute_dlog====x====y===={x},================{y}");
    let bx = Fq::from_str(x).unwrap();
    let by = Fq::from_str(y).unwrap();

    let b = BabyJubJub::new(bx, by);
    assert!(BabyJubJub::is_on_curve(&b));
    assert!(BabyJubJub::is_in_correct_subgroup_assuming_on_curve(&b));
    let b = b.mul(Fr::new(BigInteger256::from(1u8)));

    baby_giant(32, &a, &b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_dlog0() {
        let dlog = do_compute_dlog(
            "c53d8d24e6767618b495ed560a0cb4fa3d86c5b86e0d9555ab4ef69cf675511a",
            "a7099eb9f4b811bbd4ea1643e449bd1551d732d9ebc81833e5e33a3c2890af14",
        );
        assert_eq!(1, dlog);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_compute_dlog1() {
//         let dlog = do_compute_dlog(
//             "0x05e712cbd0bee349ab612d42b81672d48546ab29a90798ad2b88f64585f0c805",
//             "0xbdb2d53146a7d643d6c6870319fe563a253f78c18a48e3fa45b6d7d9d3c310",
//             2,
//         );
//         assert_eq!(65545, dlog);
//     }

//     #[test]
//     fn test_compute_dlog2() {
//         let dlog = do_compute_dlog(
//             "0xf57b238724df2c542888b0df066af2e47f5a3b54efd22e0eeb63e830cdd3ca",
//             "0x0a7a0495c2be1431a515c4eb5480cec8328028598cbf23a60c8ad08363983b12",
//             2,
//         );
//         assert_eq!(4294967295, dlog);
//     }

//     #[test]
//     fn test_compute_dlog3() {
//         let dlog = do_compute_dlog(
//             "0x2f38eeff5a5e7c9cb7f297bebd43d488354a35867b67e4147620893c025985f7",
//             "0x011f455e2ad1c9ff8086a6f00fa560afc82f9b4dfb93db0c124edde66730dbda",
//             3,
//         );
//         assert_eq!(943594123598, dlog);
//     }

//     #[test]
//     fn test_compute_dlog4() {
//         let dlog = do_compute_dlog(
//             "0x084957e99aabdff4f3d79b0da6601dadbdbcaa864a97b50bf7230673262ed002",
//             "0x06b45565a8859505a8971e35d409d1fb33381589ac2fa4d7e59ce7c7d6619784",
//             3,
//         );
//         assert_eq!(1099511627775, dlog); // max value authorized (type(uint40).max)
//     }
// }
