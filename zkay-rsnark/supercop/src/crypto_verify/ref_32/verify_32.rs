// #include "crypto_verify_32.h"
#[inline(always)]
pub fn crypto_verify_32(x: &[u8; 32], y: &[u8; 32]) -> i32 {
    crypto_verify_32_ref(x, y)
}
pub fn crypto_verify_32_ref(x: &[u8; 32], y: &[u8; 32]) -> i32 {
    let mut differentbits: u32 = 0;

    // 迭代 32 位元組並累加異或結果
    for i in 0..32 {
        differentbits |= (x[i] ^ y[i]) as u32;
    }

    // 恆定時間邏輯：
    // (differentbits - 1) >> 8 在 differentbits 為 0 時高位為 1
    let mask = (differentbits.wrapping_sub(1) >> 8) & 1;
    (mask as i32).wrapping_sub(1)
}

// use subtle::ConstantTimeEq;

// /// 對應 crypto_verify_32
// /// 相等返回 0，不等返回 -1
// pub fn crypto_verify_32(x: &[u8; 32], y: &[u8; 32]) -> i32 {
//     if x.ct_eq(y).into() {
//         0
//     } else {
//         -1
//     }
// }

// pub fn crypto_verify_32_ref(x: &[u8; 32], y: &[u8; 32]) -> i32 {
//     let mut differentbits: u32 = 0;

//     // 迭代 32 位元組並累加異或結果
//     for i in 0..32 {
//         differentbits |= (x[i] ^ y[i]) as u32;
//     }

//     // 恆定時間邏輯：
//     // (differentbits - 1) >> 8 在 differentbits 為 0 時高位為 1
//     let mask = (differentbits.wrapping_sub(1) >> 8) & 1;
//     (mask as i32).wrapping_sub(1)
// }
