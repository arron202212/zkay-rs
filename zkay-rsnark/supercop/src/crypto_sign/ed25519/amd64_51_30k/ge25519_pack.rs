// #include "fe25519.h"
// #include "sc25519.h"
// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::fe25519;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p3;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519::fe25519_mul, fe25519_getparity::fe25519_getparity, fe25519_invert::fe25519_invert,
    fe25519_pack::fe25519_pack,
};
pub fn ge25519_pack(r: &mut [u8; 32], p: &ge25519_p3) {
    let (mut tx, mut ty, mut zi) = (fe25519::default(), fe25519::default(), fe25519::default());
    fe25519_invert(&mut zi, &p.z);
    fe25519_mul(&mut tx, &p.x, &zi);
    fe25519_mul(&mut ty, &p.y, &zi);
    fe25519_pack(r, &ty);
    r[31] ^= fe25519_getparity(&tx) << 7;
}

// impl Ge25519P3 {
//     /// 對應 ge25519_pack
//     pub fn to_bytes(&self) -> [u8; 32] {
//         // 1. 計算 Z 的逆元 (對應 fe25519_invert)
//         let zi = self.z.invert();

//         // 2. 計算仿射座標 x = X * Z^-1, y = Y * Z^-1 (對應 fe25519_mul)
//         let tx = self.x.mul(&zi);
//         let ty = self.y.mul(&zi);

//         // 3. 序列化 y 座標 (對應 fe25519_pack)
//         let mut r = ty.to_bytes();

//         // 4. 將 x 的奇偶性放入最高位 (對應 fe25519_getparity)
//         // r[31] 的最高位原本應為 0（因為 y < 2^255-19），現在存入符號位
//         r[31] |= self.x_parity(&tx) << 7;

//         r
//     }

//     /// 輔助函數：獲取 x 的奇偶性
//     fn x_parity(&self, tx: &Fe25519) -> u8 {
//         tx.get_parity() & 1
//     }
// }
