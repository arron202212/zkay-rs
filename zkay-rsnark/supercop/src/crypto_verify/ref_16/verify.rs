// #include "crypto_verify_16.h"

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
//   F(8)
//   F(9)
//   F(10)
//   F(11)
//   F(12)
//   F(13)
//   F(14)
//   F(15)
//   return (1 & ((differentbits - 1) >> 8)) - 1;
// }
#[inline(always)]
pub fn crypto_verify_16(x: &[u8; 16], y: &[u8; 16]) -> i32 {
    crypto_verify_16_ref(x, y)
}
pub fn crypto_verify_16_ref(x: &[u8; 16], y: &[u8; 16]) -> i32 {
    let mut differentbits: u32 = 0;

    // 迭代 16 位元組並累加異或結果
    for i in 0..16 {
        differentbits |= (x[i] ^ y[i]) as u32;
    }

    // 恆定時間邏輯：
    // (differentbits - 1) >> 8 在 differentbits 為 0 時高位為 1
    let mask = (differentbits.wrapping_sub(1) >> 8) & 1;
    (mask as i32).wrapping_sub(1)
}

// use subtle::ConstantTimeEq;

// /// 對應 crypto_verify_16
// /// 傳入兩個 16 位元組數組，相等返回 0，不等返回 -1
// pub fn crypto_verify_16(x: &[u8; 16], y: &[u8; 16]) -> i32 {
//     // ct_eq 會生成不含分支的機器碼
//     if x.ct_eq(y).into() {
//         0
//     } else {
//         -1
//     }
// }

// pub fn crypto_verify_16(x: &[u8; 16], y: &[u8; 16]) -> i32 {
//     let mut differentbits: u32 = 0;

//     // 使用迭代器或循環，編譯器通常會自動展開 (Unroll)
//     for i in 0..16 {
//         differentbits |= (x[i] ^ y[i]) as u32;
//     }

//     // 恆定時間邏輯轉換：
//     // 若 differentbits == 0 -> 返回 0
//     // 若 differentbits != 0 -> 返回 -1
//     let mask = (differentbits.wrapping_sub(1) >> 8) & 1;
//     (mask as i32).wrapping_sub(1)
// }
