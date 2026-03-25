// #ifndef GE25519_H
// #define GE25519_H
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::fe25519;
// /*
//  * Arithmetic on the twisted Edwards curve -x^2 + y^2 = 1 + dx^2y^2
//  * with d = -(121665/121666) =
//  * 37095705934669439343138083508754565189542113879843219016388785533085940283555
//  * Base point:
//  * (15112221349535400772501151409588531511454012693041857206046113283949847762202,46316835694926478169428394003475163141307993866256225615783033603165251855960);
//  */
// #include "fe25519.h"
// #include "sc25519.h"

// #define ge25519                           crypto_sign_ed25519_amd64_51_30k_batch_ge25519
// #define ge25519_base                      crypto_sign_ed25519_amd64_51_30k_batch_ge25519_base
// #define ge25519_unpackneg_vartime         crypto_sign_ed25519_amd64_51_30k_batch_unpackneg_vartime
// #define ge25519_pack                      crypto_sign_ed25519_amd64_51_30k_batch_pack
// #define ge25519_isneutral_vartime         crypto_sign_ed25519_amd64_51_30k_batch_isneutral_vartime
// #define ge25519_add                       crypto_sign_ed25519_amd64_51_30k_batch_ge25519_add
// #define ge25519_double                    crypto_sign_ed25519_amd64_51_30k_batch_ge25519_double
// #define ge25519_double_scalarmult_vartime crypto_sign_ed25519_amd64_51_30k_batch_double_scalarmult_vartime
// #define ge25519_multi_scalarmult_vartime  crypto_sign_ed25519_amd64_51_30k_batch_ge25519_multi_scalarmult_vartime
// #define ge25519_scalarmult_base           crypto_sign_ed25519_amd64_51_30k_batch_scalarmult_base
// #define ge25519_p1p1_to_p2                crypto_sign_ed25519_amd64_51_30k_batch_ge25519_p1p1_to_p2
// #define ge25519_p1p1_to_p3                crypto_sign_ed25519_amd64_51_30k_batch_ge25519_p1p1_to_p3
// #define ge25519_p1p1_to_pniels            crypto_sign_ed25519_amd64_51_30k_batch_ge25519_p1p1_to_pniels
// #define ge25519_add_p1p1                  crypto_sign_ed25519_amd64_51_30k_batch_ge25519_add_p1p1
// #define ge25519_dbl_p1p1                  crypto_sign_ed25519_amd64_51_30k_batch_ge25519_dbl_p1p1
// #define choose_t                          crypto_sign_ed25519_amd64_51_30k_batch_choose_t
// #define choose_t_smultq                   crypto_sign_ed25519_amd64_51_30k_batch_choose_t_smultq
// #define ge25519_nielsadd2                 crypto_sign_ed25519_amd64_51_30k_batch_ge25519_nielsadd2
// #define ge25519_nielsadd_p1p1             crypto_sign_ed25519_amd64_51_30k_batch_ge25519_nielsadd_p1p1
// #define ge25519_pnielsadd_p1p1            crypto_sign_ed25519_amd64_51_30k_batch_ge25519_pnielsadd_p1p1

pub type ge25519_p3 = ge25519;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ge25519 {
    pub x: fe25519,
    pub y: fe25519,
    pub z: fe25519,
    pub t: fe25519,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ge25519_p1p1 {
    pub x: fe25519,
    pub z: fe25519,
    pub y: fe25519,
    pub t: fe25519,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ge25519_p2 {
    pub x: fe25519,
    pub y: fe25519,
    pub z: fe25519,
}
impl From<&ge25519> for ge25519_p2 {
    fn from(g: &ge25519) -> Self {
        Self {
            x: g.x,
            y: g.y,
            z: g.z,
        }
    }
}
impl From<&mut ge25519> for ge25519_p2 {
    fn from(g: &mut ge25519) -> Self {
        Self {
            x: g.x,
            y: g.y,
            z: g.z,
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ge25519_niels {
    pub ysubx: fe25519,
    pub xaddy: fe25519,
    pub t2d: fe25519,
}
impl ge25519_niels {
    pub const fn const_default() -> Self {
        ge25519_niels {
            ysubx: fe25519 { v: [0; 5] },
            xaddy: fe25519 { v: [0; 5] },
            t2d: fe25519 { v: [0; 5] },
        }
    }
}
impl From<ge25519> for ge25519_niels {
    fn from(g: ge25519) -> Self {
        Self {
            ysubx: g.x,
            xaddy: g.y,
            t2d: g.t,
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ge25519_pniels {
    pub ysubx: fe25519,
    pub xaddy: fe25519,
    pub z: fe25519,
    pub t2d: fe25519,
}
impl From<ge25519> for ge25519_pniels {
    fn from(g: ge25519) -> Self {
        Self {
            ysubx: g.x,
            xaddy: g.y,
            z: g.z,
            t2d: g.t,
        }
    }
}
// extern void ge25519_p1p1_to_p2(ge25519_p2 *r, const ge25519_p1p1 *p);
// extern void ge25519_p1p1_to_p3(ge25519_p3 *r, const ge25519_p1p1 *p);
// extern void ge25519_p1p1_to_pniels(ge25519_pniels *r, const ge25519_p1p1 *p);
// extern void ge25519_add_p1p1(ge25519_p1p1 *r, const ge25519_p3 *p, const ge25519_p3 *q);
// extern void ge25519_dbl_p1p1(ge25519_p1p1 *r, const ge25519_p2 *p);
// extern void choose_t(ge25519_niels *t, unsigned long long pos, signed long long b, const ge25519_niels *base_multiples);
// extern void choose_t_smultq(ge25519_pniels *t, signed long long b, const ge25519_pniels *pre);
// extern void ge25519_nielsadd2(ge25519_p3 *r, const ge25519_niels *q);
// extern void ge25519_nielsadd_p1p1(ge25519_p1p1 *r, const ge25519_p3 *p, const ge25519_niels *q);
// extern void ge25519_pnielsadd_p1p1(ge25519_p1p1 *r, const ge25519_p3 *p, const ge25519_pniels *q);

// extern const ge25519 ge25519_base;

// extern int ge25519_unpackneg_vartime(ge25519 *r, const unsigned char p[32]);

// extern void ge25519_pack(unsigned char r[32], const ge25519 *p);

// extern int ge25519_isneutral_vartime(const ge25519 *p);

// extern void ge25519_add(ge25519 *r, const ge25519 *p, const ge25519 *q);

// extern void ge25519_double(ge25519 *r, const ge25519 *p);

// /* computes [s1]p1 + [s2]ge25519_base */
// extern void ge25519_double_scalarmult_vartime(ge25519 *r, const ge25519 *p1, const sc25519 *s1, const sc25519 *s2);

// extern void ge25519_multi_scalarmult_vartime(ge25519 *r, ge25519 *p, sc25519 *s, const unsigned long long npoints);

// extern void ge25519_scalarmult_base(ge25519 *r, const sc25519 *s);

// #endif
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
use std::arch::global_asm;

// 假设你有对应的汇编文件 sc25519_amd64.s
global_asm!(include_str!("ge25519_add_p1p1.s"));
global_asm!(include_str!("ge25519_p1p1_to_pniels.s"));
global_asm!(include_str!("ge25519_dbl_p1p1.s"));
global_asm!(include_str!("ge25519_nielsadd_p1p1.s"));
global_asm!(include_str!("ge25519_nielsadd2.s"));
global_asm!(include_str!("ge25519_p1p1_to_p2.s"));
global_asm!(include_str!("ge25519_p1p1_to_p3.s"));
global_asm!(include_str!("ge25519_pnielsadd_p1p1.s"));
pub fn ge25519_dbl_p1p1(r: &mut ge25519_p1p1, p: &[ge25519_p2]) {}
pub fn ge25519_p1p1_to_p2(r: &mut ge25519_p2, p: &ge25519_p1p1) {}
pub fn choose_t(t: &mut ge25519_niels, pos: u64, b: i64, base_multiples: &[ge25519_niels]) {}
pub fn ge25519_p1p1_to_p3(r: &mut ge25519_p3, p: &ge25519_p1p1) {}
pub fn ge25519_add_p1p1(r: &mut ge25519_p1p1, p: &ge25519_p3, q: &ge25519_p3) {}
pub fn ge25519_pnielsadd_p1p1(r: &mut ge25519_p1p1, p: &ge25519_p3, q: &ge25519_pniels) {}
pub fn ge25519_p1p1_to_pniels(r: &mut ge25519_pniels, p: &ge25519_p1p1) {}
pub fn ge25519_nielsadd_p1p1(r: &mut ge25519_p1p1, p: &ge25519_p3, q: &ge25519_niels) {}
pub fn ge25519_nielsadd2(r: &mut ge25519_p3, q: &ge25519_niels) {}
unsafe extern "C" {
    // 坐标转换
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_p1p1_to_p2(
        r: *mut ge25519_p2,
        p: *const ge25519_p1p1,
    );
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_p1p1_to_p3(
        r: *mut ge25519_p3,
        p: *const ge25519_p1p1,
    );
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_p1p1_to_pniels(
        r: *mut ge25519_pniels,
        p: *const ge25519_p1p1,
    );
    // 点加与点倍
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_add_p1p1(
        r: *mut ge25519_p1p1,
        p: *const ge25519_p3,
        q: *const ge25519_p3,
    );
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_dbl_p1p1(
        r: *mut ge25519_p1p1,
        p: *const ge25519_p2,
    );

    // 预计算与查找表 (Table Lookup)
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_choose_t(
        t: *mut ge25519_niels,
        pos: u64,
        b: i64,
        base_multiples: *const ge25519_niels,
    );
    pub fn choose_t_smultq(t: *mut ge25519_pniels, b: i64, pre: *const ge25519_pniels);
    /// 将 niels 格式的点 q 加到 p3 格式的点 r 上，结果更新在 r 中
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_nielsadd2(
        r: *mut ge25519_p3,
        q: *const ge25519_niels,
    );

    /// 将 p3 格式的点 p 与 niels 格式的点 q 相加，结果存入 p1p1 格式的 r 中
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_nielsadd_p1p1(
        r: *mut ge25519_p1p1,
        p: *const ge25519_p3,
        q: *const ge25519_niels,
    );
    /// 使用 pniels 格式进行点加，输出到 p1p1
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ge25519_pnielsadd_p1p1(
        r: *mut ge25519_p1p1,
        p: *const ge25519_p3,
        q: *const ge25519_pniels,
    );
    // 基础点 (生成元)
    // pub static ge25519_base: ge25519;
    // 序列化 (打包/解包)
    pub fn ge25519_unpackneg_vartime(r: *mut ge25519, p: *const u8) -> i32;

    pub fn ge25519_pack(r: *mut u8, p: *const ge25519); // [u8; 32]
    /// 检查点是否为单位元（非常数时间实现）
    /// 返回值：1 表示是单位元，0 表示不是
    pub fn ge25519_isneutral_vartime(p: *const ge25519) -> i32;
    // 将点 p 和点 q 相加，结果存入 r
    // 注意：r, p, q 通常都是指向 ge25519_p3 结构体的指针
    // pub fn ge25519_add(r: *mut ge25519, p: *const ge25519, q: *const ge25519);
    /// 执行倍点运算：r = 2 * p
    pub fn ge25519_double(r: *mut ge25519, p: *const ge25519);
    // 标量乘法 (Scalar Multiplication)

    // 计算 [s1]P1 + [s2]Base (用于验签)
    pub fn ge25519_double_scalarmult_vartime(
        r: *mut ge25519,
        p1: *const ge25519,
        s1: *const sc25519,
        s2: *const sc25519,
    );
    /// 执行批量标量乘法：R = s[0]*P[0] + s[1]*P[1] + ... + s[n-1]*P[n-1]
    /// 注意：这是一个非常数时间（vartime）实现，仅限用于签名校验等公开操作。
    pub fn ge25519_multi_scalarmult_vartime(
        r: *mut ge25519,
        p: *const ge25519, // 点数组首地址
        s: *const sc25519, // 标量数组首地址
        npoints: u64,      // 对应 C 的 unsigned long long
    );
    // 计算 [s]Base
    pub fn ge25519_scalarmult_base(r: *mut ge25519, s: *const sc25519);

}

// use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::Fe25519;

// /// 擴展座標 (Projective / Extended Coordinates): x = X/Z, y = Y/Z, xy = T/Z
// /// 對應 typedef struct { fe25519 x, y, z, t; } ge25519;
// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default)]
// pub struct ge25519_p3 {
//     pub x: Fe25519,
//     pub y: Fe25519,
//     pub z: Fe25519,
//     pub t: Fe25519,
// }
// pub type ge25519 = ge25519_p3;
// /// 中間座標 (Completed Coordinates): x = X/Z, y = Y/T
// /// 對應 ge25519_p1p1
// pub struct ge25519_p1p1 {
//     pub x: Fe25519,
//     pub y: Fe25519,
//     pub z: Fe25519,
//     pub t: Fe25519,
// }

// /// 射影座標 (Projective Coordinates): x = X/Z, y = Y/Z
// /// 對應 ge25519_p2
// pub struct ge25519_p2 {
//     pub x: Fe25519,
//     pub y: Fe25519,
//     pub z: Fe25519,
// }

// /// Niels 座標: 用於預計算表，加速點加法
// /// 對應 ge25519_niels { ysubx, xaddy, t2d }
// pub struct ge25519_niels {
//     pub y_sub_x: Fe25519,
//     pub x_add_y: Fe25519,
//     pub t2d: Fe25519, // 2d * t
// }

// /// Projective Niels 座標: 帶有 Z 分量的 Niels 座標
// /// 對應 ge25519_pniels
// pub struct Ge25519PNiels {
//     pub y_sub_x: Fe25519,
//     pub x_add_y: Fe25519,
//     pub z: Fe25519,
//     pub t2d: Fe25519,
// }
// impl ge25519_p3 {
//     pub fn from_bytes_neg_vartime(public_key: &[u8; 32]) -> Self {
//         // 2. 準備接收解壓縮結果的結構體
//         let mut neg_a = unsafe { std::mem::zeroed::<ge25519>() };

//         // 3. 執行解壓縮
//         let result = unsafe { ge25519_unpackneg_vartime(&mut neg_a, public_key.as_ptr()) };

//         if result == 0 {
//             // 成功！此時 neg_a 儲存的是 -A
//             println!("Successfully unpacked and negated the point.");
//         } else {
//             // 失敗：輸入的數據不是有效的曲線點
//             println!("Invalid point encoding.");
//         }
//         neg_a
//     }
//     // pub fn double_scalarmult_vartime(neg_a:&ge25519,k:&sc25519,s:&sc25519)->Self{
//     // // // 假設我們有：
//     // // // 1. 公鑰的負值 -A (來自 ge25519_unpackneg_vartime)
//     // // let neg_a = unsafe { std::mem::zeroed::<ge25519>() };
//     // // // 2. 標量 k (來自 sc25519_from64bytes)
//     // // let k = unsafe { std::mem::zeroed::<sc25519>() };
//     // // // 3. 標量 s (來自簽名的後半部)
//     // // let s = unsafe { std::mem::zeroed::<sc25519>() };

//     // // 準備輸出結果 R'
//     // let mut r_prime = unsafe { std::mem::zeroed::<ge25519>() };

//     // unsafe {
//     //     // 計算 R' = [k](-A) + [s]B
//     //     // 對應參數：r, p1=neg_a, s1=k, s2=s
//     //     ge25519_double_scalarmult_vartime(
//     //         &mut r_prime,
//     //         neg_a,
//     //         k,
//     //         s
//     //     );
//     // }

//     // // 最後步驟通常是將 r_prime pack 之後與簽名中的 R 進行比較
//     //     r_prime
//     // }
//     pub fn to_bytes(&self) -> [u8; 32] {
//         let mut v = [0u8; 32];
//         unsafe {
//             ge25519_pack(v.as_mut_ptr(), self);
//         }
//         v
//     }
// }
