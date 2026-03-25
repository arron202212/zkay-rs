// #ifndef SC25519_H
// #define SC25519_H

// #define sc25519                  crypto_sign_ed25519_amd64_51_30k_batch_sc25519
// #define shortsc25519             crypto_sign_ed25519_amd64_51_30k_batch_shortsc25519
// #define sc25519_from32bytes      crypto_sign_ed25519_amd64_51_30k_batch_sc25519_from32bytes
// #define shortsc25519_from16bytes crypto_sign_ed25519_amd64_51_30k_batch_shortsc25519_from16bytes
// #define sc25519_from64bytes      crypto_sign_ed25519_amd64_51_30k_batch_sc25519_from64bytes
// #define sc25519_from_shortsc     crypto_sign_ed25519_amd64_51_30k_batch_sc25519_from_shortsc
// #define sc25519_to32bytes        crypto_sign_ed25519_amd64_51_30k_batch_sc25519_to32bytes
// #define sc25519_iszero_vartime   crypto_sign_ed25519_amd64_51_30k_batch_sc25519_iszero_vartime
// #define sc25519_isshort_vartime  crypto_sign_ed25519_amd64_51_30k_batch_sc25519_isshort_vartime
// #define sc25519_lt               crypto_sign_ed25519_amd64_51_30k_batch_sc25519_lt
// #define sc25519_add              crypto_sign_ed25519_amd64_51_30k_batch_sc25519_add
// #define sc25519_sub_nored        crypto_sign_ed25519_amd64_51_30k_batch_sc25519_sub_nored
// #define sc25519_mul              crypto_sign_ed25519_amd64_51_30k_batch_sc25519_mul
// #define sc25519_mul_shortsc      crypto_sign_ed25519_amd64_51_30k_batch_sc25519_mul_shortsc
// #define sc25519_window4          crypto_sign_ed25519_amd64_51_30k_batch_sc25519_window4
// #define sc25519_window5          crypto_sign_ed25519_amd64_51_30k_batch_sc25519_window5
// #define sc25519_slide           crypto_sign_ed25519_amd64_51_30k_batch_sc25519_slide
// #define sc25519_2interleave2     crypto_sign_ed25519_amd64_51_30k_batch_sc25519_2interleave2
// #define sc25519_barrett crypto_sign_ed25519_amd64_51_30k_batch_sc25519_barrett
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct sc25519 {
    pub v: [u64; 4],
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct shortsc25519 {
    pub v: [u64; 2],
}

// void sc25519_from32bytes(sc25519 *r, const unsigned char x[32]);

// void sc25519_from64bytes(sc25519 *r, const unsigned char x[64]);

// void sc25519_from_shortsc(sc25519 *r, const shortsc25519 *x);

// void sc25519_to32bytes(unsigned char r[32], const sc25519 *x);

// int sc25519_iszero_vartime(const sc25519 *x);

// int sc25519_lt(const sc25519 *x, const sc25519 *y);

// void sc25519_add(sc25519 *r, const sc25519 *x, const sc25519 *y);

// void sc25519_sub_nored(sc25519 *r, const sc25519 *x, const sc25519 *y);

// void sc25519_mul(sc25519 *r, const sc25519 *x, const sc25519 *y);

// void sc25519_mul_shortsc(sc25519 *r, const sc25519 *x, const shortsc25519 *y);

// /* Convert s into a representation of the form \sum_{i=0}^{63}r[i]2^(4*i)
//  * with r[i] in {-8,...,7}
//  */
// void sc25519_window4(signed char r[64], const sc25519 *s);

// void sc25519_window5(signed char r[51], const sc25519 *s);

// void sc25519_slide(signed char r[256], const sc25519 *s, int swindowsize);

// void sc25519_2interleave2(unsigned char r[127], const sc25519 *s1, const sc25519 *s2);

// void sc25519_barrett(sc25519 *r, unsigned long long x[8]);

// #endif

use std::arch::global_asm;

// 假设你有对应的汇编文件 sc25519_amd64.s
global_asm!(include_str!("sc25519_add.s"));
global_asm!(include_str!("sc25519_barrett.s"));
global_asm!(include_str!("sc25519_lt.s"));
global_asm!(include_str!("sc25519_sub_nored.s"));

pub fn sc25519_add(r: &mut sc25519, x: &sc25519, y: &sc25519) {}
pub fn sc25519_lt(x: *const sc25519, y: *const sc25519) -> i32 {
    0
}
pub fn sc25519_sub_nored(r: *mut sc25519, x: *const sc25519, y: *const sc25519) {}
unsafe extern "C" {
    // 转换与序列化
    pub fn sc25519_from32bytes(r: *mut sc25519, x: *const u8); // x: [u8; 32]
    pub fn sc25519_from64bytes(r: *mut sc25519, x: *const u8); // x: [u8; 64]
    pub fn sc25519_from_shortsc(r: *mut sc25519, x: *const shortsc25519);
    pub fn sc25519_to32bytes(r: *mut u8, x: *const sc25519); // r: [u8; 32]

    // 逻辑判定
    pub fn sc25519_iszero_vartime(x: *const sc25519) -> i32;
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_sc25519_lt(
        x: *const sc25519,
        y: *const sc25519,
    ) -> i32;

    // 基础算术
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_sc25519_add(
        r: *mut sc25519,
        x: *const sc25519,
        y: *const sc25519,
    );
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_sc25519_sub_nored(
        r: *mut sc25519,
        x: *const sc25519,
        y: *const sc25519,
    );
    pub fn sc25519_mul(r: *mut sc25519, x: *const sc25519, y: *const sc25519);
    pub fn sc25519_mul_shortsc(r: *mut sc25519, x: *const sc25519, y: *const shortsc25519);

    // 窗口化与预处理 (用于点乘加速)
    pub fn sc25519_window4(r: *mut i8, s: *const sc25519); // r: [i8; 64]
    pub fn sc25519_window5(r: *mut i8, s: *const sc25519); // r: [i8; 51]
    pub fn sc25519_slide(r: *mut i8, s: *const sc25519, swindowsize: i32); // r: [i8; 256]
    pub fn sc25519_2interleave2(r: *mut u8, s1: *const sc25519, s2: *const sc25519); // r: [u8; 127]

    // Barrett 归约 (处理 512 位输入)
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_sc25519_barrett(r: *mut sc25519, x: *const u64); // x: [u64; 8]

}
pub fn sc25519_barrett(r: &mut sc25519, x: &[u64; 8]) {
    // x:
}
// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
// pub struct sc25519 {
//     pub v: [u64; 4], // 4 * 64 = 256 bits
// }

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
// pub struct shortsc25519 {
//     pub v: [u64; 2], // 2 * 64 = 128 bits
// }
// impl sc25519 {
//     /// 对应 sc25519_from32bytes
//     pub fn from_32bytes(bytes: &[u8; 32]) -> Self {
//         let mut v = [0u64; 4];
//         for i in 0..4 {
//             v[i] = u64::from_le_bytes(bytes[i * 8..(i + 1) * 8].try_into().unwrap());
//         }
//         // 注意：实际实现中这里通常需要进行模 L 归约
//         Self { v }
//     }
//     pub fn from_64bytes(bytes: &[u8; 64]) -> Self {
//         // let mut v = [0u64; 4];
//         // 2. 準備接收結果的 sc25519 結構體
//         // 使用 MaybeUninit 以避免無謂的初始化，或直接用 zeroed
//         let mut r = unsafe { std::mem::zeroed::<sc25519>() };

//         unsafe {
//             // 3. 執行縮減運算
//             // x 指向 64 位元組數組，r 接收縮減後的結果
//             sc25519_from64bytes(&mut r, bytes.as_ptr());
//         }
//         r
//     }

//     /// 对应 sc25519_to32bytes
//     pub fn to_32bytes(&self) -> [u8; 32] {
//         let mut res = [0u8; 32];
//         for i in 0..4 {
//             res[i * 8..(i + 1) * 8].copy_from_slice(&self.v[i].to_le_bytes());
//         }
//         res
//     }

//     /// 对应 sc25519_iszero_vartime
//     pub fn is_zero_vartime(&self) -> bool {
//         self.v.iter().all(|&x| x == 0)
//     }
// }

// // 对应 sc25519_add
// impl std::ops::Add<&Self> for sc25519 {
//     type Output = Self;
//     fn add(self, rhs: &Self) -> Self {
//         // 这里应包含高精度加法逻辑及模 L 归约
//         todo!("实现模 L 加法")
//     }
// }
// impl std::ops::Mul<&Self> for sc25519 {
//     type Output = Self;
//     fn mul(self, rhs: &Self) -> Self {
//         // 这里应包含高精度加法逻辑及模 L 归约
//         todo!("实现模 L 加法")
//     }
// }
// impl sc25519 {
//     /// 对应 sc25519_window4
//     /// 将标量分解为 \sum r[i]2^(4*i)，r[i] 范围为 [-8, 7]
//     pub fn to_window4(&self) -> [i8; 64] {
//         let mut r = [0i8; 64];
//         // 具体的位移与符号位转换逻辑...
//         r
//     }
// }

// // use curve25519_dalek::scalar::Scalar;

// // let bytes = [0u8; 32];
// // let s = Scalar::from_bytes_mod_order(bytes); // 自动处理模归约
// // let sum = s + s; // 重载了运算符，等同于 sc25519_add
