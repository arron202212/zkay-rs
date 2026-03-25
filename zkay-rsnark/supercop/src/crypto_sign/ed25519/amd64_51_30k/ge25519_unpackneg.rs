// #include "fe25519.h"
// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519_neg::fe25519_neg;
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519_unpack::fe25519_unpack;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p3;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519::{fe25519, fe25519_mul, fe25519_square},
    fe25519_add::fe25519_add,
    fe25519_getparity::fe25519_getparity,
    fe25519_iseq::fe25519_iseq_vartime,
    fe25519_pow2523::fe25519_pow2523,
    fe25519_setint::fe25519_setint,
    fe25519_sub::fe25519_sub,
};
// /* d */
const ECD: fe25519 = fe25519 {
    v: [
        929955233495203,
        466365720129213,
        1662059464998953,
        2033849074728123,
        1442794654840575,
    ],
};
// /* sqrt(-1) */
const SQRTM1: fe25519 = fe25519 {
    v: [
        1718705420411056,
        234908883556509,
        2233514472574048,
        2117202627021982,
        765476049583133,
    ],
};

/* return 0 on success, -1 otherwise */
pub fn ge25519_unpackneg_vartime(r: &mut ge25519_p3, p: &[u8; 32]) -> i32 {
    let (mut t, mut chk, mut num, mut den, mut den2, mut den4, mut den6) = (
        fe25519::default(),
        fe25519::default(),
        fe25519::default(),
        fe25519::default(),
        fe25519::default(),
        fe25519::default(),
        fe25519::default(),
    );
    let par: u8 = p[31] >> 7;

    fe25519_setint(&mut r.z, 1);
    fe25519_unpack(&mut r.y, p);
    fe25519_square(&mut num, &r.y); /* x = y^2 */
    fe25519_mul(&mut den, &num, &ECD); /* den = dy^2 */
    let nums = num;
    fe25519_sub(&mut num, &nums, &r.z); /* x = y^2-1 */
    let dens = den;
    fe25519_add(&mut den, &r.z, &dens); /* den = dy^2+1 */
    /* Computation of sqrt(num/den)
       1.: computation of num^((p-5)/8)*den^((7p-35)/8) = (num*den^7)^((p-5)/8)
    */
    fe25519_square(&mut den2, &den);
    fe25519_square(&mut den4, &den2);
    fe25519_mul(&mut den6, &den4, &den2);
    fe25519_mul(&mut t, &den6, &num);
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &den);
    let tt = t.clone();
    fe25519_pow2523(&mut t, &tt);
    /* 2. computation of r.x = t * num * den^3
     */
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &num);
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &den);
    let tt = t.clone();
    fe25519_mul(&mut t, &tt, &den);
    fe25519_mul(&mut r.x, &t, &den);

    /* 3. Check whether sqrt computation gave correct result, multiply by sqrt(-1) if not:
     */
    fe25519_square(&mut chk, &r.x);
    let chks = chk.clone();
    fe25519_mul(&mut chk, &chks, &den);
    if fe25519_iseq_vartime(&chk, &num) == 0 {
        let rx = r.x.clone();
        fe25519_mul(&mut r.x, &rx, &SQRTM1);
    }

    /* 4. Now we have one of the two square roots, except if input was not a square
     */
    fe25519_square(&mut chk, &r.x);
    let chks = chk.clone();
    fe25519_mul(&mut chk, &chks, &den);
    if fe25519_iseq_vartime(&chk, &num) == 0 {
        return -1;
    }

    /* 5. Choose the desired square root according to parity:
     */
    if fe25519_getparity(&r.x) != (1 - par) {
        let rx = r.x.clone();
        fe25519_neg(&mut r.x, &rx);
    }

    fe25519_mul(&mut r.t, &r.x, &r.y);
    0
}

// impl Ge25519P3 {
//     /// 對應 ge25519_unpackneg_vartime
//     /// 從 32 字節恢復點，失敗返回 None
//     pub fn from_bytes_neg_vartime(p: &[u8; 32]) -> Option<Self> {
//         // 1. 提取 y 和符號位 (parity)
//         let par = p[31] >> 7;
//         let y = Fe25519::from_bytes(p);
//         let z = Fe25519::from(1);

//         // 2. 計算 num = y^2 - 1, den = dy^2 + 1
//         let y2 = y.square();
//         let num = y2.sub(&z);
//         let den = y2.mul(&ECD).add(&z);

//         // 3. 計算 sqrt(num/den) 的候選值
//         // 這裡的邏輯與 C 代碼中的 den^7 * num 加法鏈一致
//         let den2 = den.square();
//         let den4 = den2.square();
//         let den6 = den4.mul(&den2);
//         let mut t = den6.mul(&num).mul(&den);

//         t = t.pow2523(); // 之前實現的 x^((p-5)/8)

//         // r.x = t * num * den^3
//         let mut x = t.mul(&num).mul(&den).mul(&den).mul(&den);

//         // 4. 驗證並修正平方根 (乘以 sqrt(-1))
//         let mut chk = x.square().mul(&den);
//         if chk != num {
//             x = x.mul(&SQRTM1);
//             chk = x.square().mul(&den);
//         }

//         // 5. 如果仍然不符合方程，則失敗
//         if chk != num {
//             return None;
//         }

//         // 6. 根據符號位選擇正確的 x (Ed25519 標準：x 符號應與壓縮位相反)
//         // 註：C 代碼中使用 1-par 是因為它在解壓後通常直接用於後續的減法邏輯
//         if x.get_parity() != (1 - par) {
//             x = x.neg();
//         }

//         let t_coord = x.mul(&y);

//         Some(Ge25519P3 {
//             x,
//             y,
//             z,
//             t: t_coord,
//         })
//     }
// }
