// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
// /*Arithmetic modulo the group order n = 2^252 + 27742317777372353535851937790883648493
//  *                                    = 7237005577332262213973186563042994240857116359379907606001950938285454250989
//  */
// /* Contains order, 2*order, 4*order, 8*order, each represented in 4 consecutive unsigned long long */
// static const unsigned long long order[16] = {0x5812631A5CF5D3EDULL, 0x14DEF9DEA2F79CD6ULL,
//                                              0x0000000000000000ULL, 0x1000000000000000ULL,
//                                              0xB024C634B9EBA7DAULL, 0x29BDF3BD45EF39ACULL,
//                                              0x0000000000000000ULL, 0x2000000000000000ULL,
//                                              0x60498C6973D74FB4ULL, 0x537BE77A8BDE7359ULL,
//                                              0x0000000000000000ULL, 0x4000000000000000ULL,
//                                              0xC09318D2E7AE9F68ULL, 0xA6F7CEF517BCE6B2ULL,
//                                              0x0000000000000000ULL, 0x8000000000000000ULL};

// 曲線階 L 的倍數表 (對應 static const unsigned long long order[16])
const ORDER: [u64; 16] = [
    0x5812631A5CF5D3ED,
    0x14DEF9DEA2F79CD6,
    0x0000000000000000,
    0x1000000000000000, // 1*L
    0xB024C634B9EBA7DA,
    0x29BDF3BD45EF39AC,
    0x0000000000000000,
    0x2000000000000000, // 2*L
    0x60498C6973D74FB4,
    0x537BE77A8BDE7359,
    0x0000000000000000,
    0x4000000000000000, // 4*L
    0xC09318D2E7AE9F68,
    0xA6F7CEF517BCE6B2,
    0x0000000000000000,
    0x8000000000000000, // 8*L
];

pub const fn smaller(a: u64, b: u64) -> u64 {
    let atop: u64 = a >> 32;
    let abot: u64 = a & 4294967295;
    let btop: u64 = b >> 32;
    let bbot: u64 = b & 4294967295;
    let atopbelowbtop: u64 = (atop - btop) >> 63;
    let atopeqbtop: u64 = ((atop ^ btop) - 1) >> 63;
    let abotbelowbbot: u64 = (abot - bbot) >> 63;
    atopbelowbtop | (atopeqbtop & abotbelowbbot)
}

pub fn sc25519_from32bytes(r: &mut sc25519, x: &[u8; 32]) {
    let mut t = [0u64; 4];
    let mut b: u64;
    let mut mask: u64;

    /* assuming little-endian */
    for i in 0..4 {
        r.v[i] = u64::from_le_bytes(x[i * 8..(i + 1) * 8].try_into().unwrap());
    }

    for j in (0..=3).rev() {
        b = 0;
        for i in 0..4 {
            b += ORDER[4 * j + i]; /* no overflow for this particular order */
            t[i] = r.v[i] - b;
            b = smaller(r.v[i], b);
        }
        mask = b - 1;
        for i in 0..4 {
            r.v[i] ^= mask & (r.v[i] ^ t[i]);
        }
    }
}

// #[derive(Clone, Copy, Debug, Default)]
// pub struct Sc25519 {
//     pub v: [u64; 4],
// }

// impl Sc25519 {
//     /// 恆定時間比較 a < b (對應 smaller)
//     fn is_smaller(a: u64, b: u64) -> u64 {
//         // 使用 wrapping_sub 並取符號位
//         (a.wrapping_sub(b) >> 63)
//     }

//     /// 對應 sc25519_from32bytes
//     pub fn from_32bytes(bytes: &[u8; 32]) -> Self {
//         let mut r = [0u64; 4];
//         // 讀取小端字節 (Rust 標準庫方法)
//         for i in 0..4 {
//             r[i] = u64::from_le_bytes(bytes[i * 8..(i + 1) * 8].try_into().unwrap());
//         }

//         // 條件減法歸約
//         for j in (0..4).rev() {
//             let mut t = [0u64; 4];
//             let mut borrow: u64 = 0;

//             // 計算 r - (j*L)
//             for i in 0..4 {
//                 let val_to_sub = ORDER[j * 4 + i].wrapping_add(borrow);
//                 t[i] = r[i].wrapping_sub(val_to_sub);
//                 borrow = Self::is_smaller(r[i], val_to_sub);
//             }

//             // 如果借位為 1 (即 r < j*L)，mask 為 0；否則 mask 為全 1
//             let mask = borrow.wrapping_sub(1);

//             // 恆定時間選擇：r = if r >= j*L { t } else { r }
//             for i in 0..4 {
//                 r[i] ^= mask & (r[i] ^ t[i]);
//             }
//         }

//         Sc25519 { v: r }
//     }
// }
