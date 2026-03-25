// #include "fe25519.h"
// #include "sc25519.h"
// #include "ge25519.h"
// #include "index_heap.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_iszero::sc25519_iszero_vartime;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519_setint::fe25519_setint,
    ge25519::{ge25519, ge25519_p3},
    ge25519_add::ge25519_add,
    ge25519_double::ge25519_double,
    index_heap::{
        heap_extend, heap_get2max, heap_init, heap_rootreplaced, heap_rootreplaced_1limb,
        heap_rootreplaced_2limbs, heap_rootreplaced_3limbs,
    },
    sc25519::{sc25519, sc25519_sub_nored},
};
pub fn setneutral(r: &mut ge25519) {
    fe25519_setint(&mut r.x, 0);
    fe25519_setint(&mut r.y, 1);
    fe25519_setint(&mut r.z, 1);
    fe25519_setint(&mut r.t, 0);
}

pub fn ge25519_scalarmult_vartime_2limbs(r: &mut ge25519, p: &mut ge25519, s: &mut sc25519) {
    if s.v[1] == 0 && s.v[0] == 1 {
        /* This will happen most of the time after Bos-Coster */

        *r = p.clone();
    } else if s.v[1] == 0 && s.v[0] == 0 {
        /* This won't ever happen, except for all scalars == 0 in Bos-Coster */
        setneutral(r);
    } else {
        let mut mask = 1u64 << 63;
        let mut i: usize = 1;
        while (mask & s.v[1]) == 0 && mask != 0 {
            mask >>= 1;
        }
        if mask == 0 {
            mask = 1u64 << 63;
            i = 0;
            while (mask & s.v[0]) == 0 && mask != 0 {
                mask >>= 1;
            }
        }
        let mut d = p.clone();
        mask >>= 1;
        while mask != 0 {
            let dd = d.clone();
            ge25519_double(&mut d, &dd);
            if (s.v[i] & mask) != 0 {
                let dd = d.clone();
                ge25519_add(&mut d, &dd, p);
            }
            mask >>= 1;
        }
        if i == 1 {
            mask = 1u64 << 63;
            while mask != 0 {
                let dd = d.clone();
                ge25519_double(&mut d, &dd);
                if s.v[0] & mask != 0 {
                    let dd = d.clone();
                    ge25519_add(&mut d, &dd, p);
                }
                mask >>= 1;
            }
        }
        *r = d.clone();
    }
}

// /* caller's responsibility to ensure npoints >= 5 */
pub fn ge25519_multi_scalarmult_vartime(
    r: &mut ge25519_p3,
    p: &mut [ge25519_p3],
    s: &mut [sc25519],
    npoints: u64,
) {
    let mut pos = vec![0u64; npoints as usize];
    let mut hlen = ((npoints + 1) / 2) | 1;
    let (mut max1, mut max2) = (0u64, 0u64);

    heap_init(&mut pos, hlen, &s[..]);

    loop {
        heap_get2max(&mut pos, &mut max1, &mut max2, s);
        if s[max1 as usize].v[3] == 0 || sc25519_iszero_vartime(&s[max2 as usize]) != 0 {
            break;
        }
        sc25519_sub_nored(&mut s[max1 as usize], &s[max1 as usize], &s[max2 as usize]);
        let (pmx2, pmx1) = (p[max2 as usize].clone(), p[max1 as usize].clone());
        ge25519_add(&mut p[max2 as usize], &pmx2, &pmx1);
        heap_rootreplaced(&mut pos[0], hlen, &s[0]);
    }
    loop {
        heap_get2max(&mut pos, &mut max1, &mut max2, s);
        if s[max1 as usize].v[2] == 0 || sc25519_iszero_vartime(&s[max2 as usize]) != 0 {
            break;
        }
        sc25519_sub_nored(&mut s[max1 as usize], &s[max1 as usize], &s[max2 as usize]);
        let (pmx2, pmx1) = (p[max2 as usize].clone(), p[max1 as usize].clone());
        ge25519_add(&mut p[max2 as usize], &pmx2, &pmx1);
        heap_rootreplaced_3limbs(&mut pos[0], hlen, &s[0]);
    }
    /* We know that (npoints-1)/2 scalars are only 128-bit scalars */
    heap_extend(&mut pos, hlen, npoints, s);
    hlen = npoints;
    loop {
        heap_get2max(&mut pos, &mut max1, &mut max2, s);
        if s[max1 as usize].v[1] == 0 || sc25519_iszero_vartime(&s[max2 as usize]) != 0 {
            break;
        }
        sc25519_sub_nored(&mut s[max1 as usize], &s[max1 as usize], &s[max2 as usize]);
        let (pmx2, pmx1) = (p[max2 as usize].clone(), p[max1 as usize].clone());
        ge25519_add(&mut p[max2 as usize], &pmx2, &pmx1);
        heap_rootreplaced_2limbs(&mut pos[0], hlen, &s[0]);
    }
    loop {
        heap_get2max(&mut pos, &mut max1, &mut max2, s);
        if sc25519_iszero_vartime(&s[max2 as usize]) != 0 {
            break;
        }
        sc25519_sub_nored(&mut s[max1 as usize], &s[max1 as usize], &s[max2 as usize]);
        let (pmx2, pmx1) = (p[max2 as usize].clone(), p[max1 as usize].clone());
        ge25519_add(&mut p[max2 as usize], &pmx2, &pmx1);
        heap_rootreplaced_1limb(&mut pos[0], hlen, &s[0]);
    }

    ge25519_scalarmult_vartime_2limbs(r, &mut p[max1 as usize], &mut s[max1 as usize]);
}

// impl Ge25519P3 {
//     /// 對應 ge25519_scalarmult_vartime_2limbs
//     /// 處理只有低 128 位（2 limbs）的標量
//     pub fn scalarmult_vartime_2limbs(&mut self, p: &Ge25519P3, s: &Sc25519) {
//         if s.v[1] == 0 && s.v[0] == 1 {
//             *self = *p;
//             return;
//         }
//         if s.is_zero_vartime() {
//             *self = Ge25519P3::identity();
//             return;
//         }

//         let mut d = *p;
//         // 找到最高有效位 (MSB)
//         let (mut i, mut mask) = if s.v[1] != 0 {
//             (1, 1u64 << (63 - s.v[1].leading_zeros()))
//         } else {
//             (0, 1u64 << (63 - s.v[0].leading_zeros()))
//         };

//         mask >>= 1; // 跳過最高位

//         // 雙循環處理兩個 limb
//         while i >= 0 {
//             while mask != 0 {
//                 d = d.double();
//                 if (s.v[i as usize] & mask) != 0 {
//                     d = &d + p;
//                 }
//                 mask >>= 1;
//             }
//             i -= 1;
//             mask = 1u64 << 63;
//         }
//         *self = d;
//     }
// }
