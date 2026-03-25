// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::{
    fe25519, fe25519_mul, fe25519_nsquare, fe25519_square,
};

pub fn fe25519_invert(r: &mut fe25519, x: &fe25519) {
    let mut z2: fe25519 = fe25519::default();
    let mut z9: fe25519 = fe25519::default();
    let mut z11: fe25519 = fe25519::default();
    let mut z2_5_0: fe25519 = fe25519::default();
    let mut z2_10_0: fe25519 = fe25519::default();
    let mut z2_20_0: fe25519 = fe25519::default();
    let mut z2_50_0: fe25519 = fe25519::default();
    let mut z2_100_0: fe25519 = fe25519::default();
    let mut t: fe25519 = fe25519::default();

    /* 2 */
    fe25519_square(&mut z2, x);
    /* 4 */
    fe25519_square(&mut t, &z2);
    /* 8 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);
    /* 9 */
    fe25519_mul(&mut z9, &t, x);
    /* 11 */
    fe25519_mul(&mut z11, &z9, &z2);
    /* 22 */
    fe25519_square(&mut t, &z11);
    /* 2^5 - 2^0 = 31 */
    fe25519_mul(&mut z2_5_0, &t, &z9);

    /* 2^6 - 2^1 */
    fe25519_square(&mut t, &z2_5_0);
    /* 2^10 - 2^5 */
    fe25519_nsquare(&mut t, 4);
    /* 2^10 - 2^0 */
    fe25519_mul(&mut z2_10_0, &t, &z2_5_0);

    /* 2^11 - 2^1 */
    fe25519_square(&mut t, &z2_10_0);
    /* 2^20 - 2^10 */
    fe25519_nsquare(&mut t, 9);
    /* 2^20 - 2^0 */
    fe25519_mul(&mut z2_20_0, &t, &z2_10_0);

    /* 2^21 - 2^1 */
    fe25519_square(&mut t, &z2_20_0);
    /* 2^40 - 2^20 */
    fe25519_nsquare(&mut t, 19);
    /* 2^40 - 2^0 */
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &z2_20_0);

    /* 2^41 - 2^1 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);
    /* 2^50 - 2^10 */
    fe25519_nsquare(&mut t, 9);
    /* 2^50 - 2^0 */
    fe25519_mul(&mut z2_50_0, &t, &z2_10_0);

    /* 2^51 - 2^1 */
    fe25519_square(&mut t, &z2_50_0);
    /* 2^100 - 2^50 */
    fe25519_nsquare(&mut t, 49);
    /* 2^100 - 2^0 */
    fe25519_mul(&mut z2_100_0, &t, &z2_50_0);

    /* 2^101 - 2^1 */
    fe25519_square(&mut t, &z2_100_0);
    /* 2^200 - 2^100 */
    fe25519_nsquare(&mut t, 99);
    /* 2^200 - 2^0 */
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &z2_100_0);

    /* 2^201 - 2^1 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);
    /* 2^250 - 2^50 */
    fe25519_nsquare(&mut t, 49);
    /* 2^250 - 2^0 */
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &z2_50_0);

    /* 2^251 - 2^1 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);
    /* 2^252 - 2^2 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);
    /* 2^253 - 2^3 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);

    /* 2^254 - 2^4 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);

    /* 2^255 - 2^5 */
    let tt = t.clone();
    fe25519_square(&mut t, &tt);
    /* 2^255 - 21 */
    fe25519_mul(r, &t, &z11);
}

// impl Fe25519 {
//     /// 对应 fe25519_invert
//     pub fn invert(&self) -> Self {
//         let x = self;
//         let mut t: Fe25519;

//         // /* 2 */
//         let z2 = x.square();
//         // /* 8 */
//         t = z2.square().square();
//         // /* 9 */
//         let z9 = t.mul(x);
//         // /* 11 */
//         let z11 = z9.mul(&z2);
//         // /* 22 */
//         t = z11.square();
//         // /* 31 = 2^5 - 2^0 */
//         let z2_5_0 = t.mul(&z9);

//         // /* 2^10 - 2^0 */
//         t = z2_5_0.n_square(5); // 封装一个 n_square 辅助函数执行 5 次平方
//         let z2_10_0 = t.mul(&z2_5_0);

//         // /* 2^20 - 2^0 */
//         t = z2_10_0.n_square(10);
//         let z2_20_0 = t.mul(&z2_10_0);

//         // /* 2^40 - 2^0 */
//         t = z2_20_0.n_square(20);
//         t = t.mul(&z2_20_0);

//         // /* 2^50 - 2^0 */
//         t = t.n_square(10);
//         let z2_50_0 = t.mul(&z2_10_0);

//         // /* 2^100 - 2^0 */
//         t = z2_50_0.n_square(50);
//         let z2_100_0 = t.mul(&z2_50_0);

//         // /* 2^200 - 2^0 */
//         t = z2_100_0.n_square(100);
//         t = t.mul(&z2_100_0);

//         // /* 2^250 - 2^0 */
//         t = t.n_square(50);
//         t = t.mul(&z2_50_0);

//         // /* 2^255 - 21 */
//         t = t.n_square(5);
//         t.mul(&z11) // 返回结果 r
//     }

//     /// 辅助函数：执行 n 次平方
//     fn n_square(&self, n: usize) -> Self {
//         let mut res = *self;
//         for _ in 0..n {
//             res = res.square();
//         }
//         res
//     }
// }
