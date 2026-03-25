// #include "crypto_verify_8.h"

// int crypto_verify(const unsigned char *x,const unsigned char *y)
// {
//   unsigned int differentbits = 0;
// #define F(i) differentbits |= x[i] ^ y[i];
//   F(0)
//   F(1)
//   F(2)
//   F(3)
//   F(4)
//   F(5)
//   F(6)
//   F(7)
//   return (1 & ((differentbits - 1) >> 8)) - 1;
// }

#[inline(always)]
pub fn crypto_verify_8(x: &[u8; 8], y: &[u8; 8]) -> i32 {
    crypto_verify_8_ref(x, y)
}
pub fn crypto_verify_8_ref(x: &[u8; 8], y: &[u8; 8]) -> i32 {
    let mut differentbits: u32 = 0;

    // 迭代 8 位元組並累加異或結果
    for i in 0..8 {
        differentbits |= (x[i] ^ y[i]) as u32;
    }

    // 恆定時間邏輯：
    // (differentbits - 1) >> 8 在 differentbits 為 0 時高位為 1
    let mask = (differentbits.wrapping_sub(1) >> 8) & 1;
    (mask as i32).wrapping_sub(1)
}

// use subtle::ConstantTimeEq;

// /// 對應 crypto_verify_8
// pub fn crypto_verify_8(x: &[u8; 8], y: &[u8; 8]) -> i32 {
//     // ct_eq 返回一個 Choice 對象，1 代表相等，0 代表不等
//     if x.ct_eq(y).into() {
//         0
//     } else {
//         -1
//     }
// }

// pub fn crypto_verify_8(x: &[u8; 8], y: &[u8; 8]) -> i32 {
//     let mut differentbits: u32 = 0;

//     for i in 0..8 {
//         differentbits |= (x[i] ^ y[i]) as u32;
//     }

//     // 這裡的邏輯與 C 代碼一致：
//     // 如果 differentbits 是 0，返回 0；否則返回 -1
//     ((1u32 & (differentbits.wrapping_sub(1) >> 8)) as i32).wrapping_sub(1)
// }
