// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
pub fn sc25519_slide(r: &mut [i8; 256], s: &sc25519, swindowsize: i32) {
    let (m, soplen) = ((1 << (swindowsize - 1)) - 1, 256);
    let mut sv0: u64 = s.v[0];
    let mut sv1: u64 = s.v[1];
    let mut sv2: u64 = s.v[2];
    let mut sv3: u64 = s.v[3];

    /* first put the binary expansion into r  */
    for i in 0..64 {
        r[i] = (sv0 & 1) as i8;
        r[i + 64] = (sv1 & 1) as i8;
        r[i + 128] = (sv2 & 1) as i8;
        r[i + 192] = (sv3 & 1) as i8;
        sv0 >>= 1;
        sv1 >>= 1;
        sv2 >>= 1;
        sv3 >>= 1;
    }

    /* Making it sliding window */
    for j in 0..soplen {
        if r[j] == 0 {
            continue;
        }
        for b in 1..=6 {
            if b >= soplen - j {
                break;
            }
            if r[j] + (r[j + b] << b) <= m {
                r[j] += r[j + b] << b;
                r[j + b] = 0;
            } else if r[j] - (r[j + b] << b) >= -m {
                r[j] -= r[j + b] << b;
                for k in j + b..soplen {
                    if r[k] == 0 {
                        r[k] = 1;
                        break;
                    }
                    r[k] = 0;
                }
            } else if r[j + b] != 0 {
                break;
            }
        }
    }
}

// impl Sc25519 {
//     /// 對應 sc25519_slide
//     /// 將標量轉化為滑動窗口表示，窗口大小通常為 5 或 7
//     pub fn to_slide_window(&self, swindowsize: i32) -> [i8; 256] {
//         let mut r = [0i8; 256];
//         let m = (1 << (swindowsize - 1)) - 1; // 窗口最大值限制

//         // 1. 展開為二進制 (0/1) 數組
//         for i in 0..64 {
//             r[i] = ((self.v[0] >> i) & 1) as i8;
//             r[i + 64] = ((self.v[1] >> i) & 1) as i8;
//             r[i + 128] = ((self.v[2] >> i) & 1) as i8;
//             r[i + 192] = ((self.v[3] >> i) & 1) as i8;
//         }

//         // 2. 執行滑動窗口變換 (Sliding Window Transform)
//         for j in 0..256 {
//             if r[j] != 0 {
//                 for b in 1..=6 {
//                     if j + b >= 256 {
//                         break;
//                     }

//                     let val = r[j] + (r[j + b] << b);
//                     if val <= m as i8 && val >= -(m as i8) {
//                         r[j] = val;
//                         r[j + b] = 0;
//                     } else if r[j] - (r[j + b] << b) >= -(m as i8) {
//                         r[j] -= r[j + b] << b;
//                         // 進位處理 (Propagate carry)
//                         for k in (j + b)..256 {
//                             if r[k] == 0 {
//                                 r[k] = 1;
//                                 break;
//                             }
//                             r[k] = 0;
//                         }
//                     } else if r[j + b] != 0 {
//                         break;
//                     }
//                 }
//             }
//         }
//         r
//     }
// }
