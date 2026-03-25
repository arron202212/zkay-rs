// #include "fe25519.h"
// #include "sc25519.h"
// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p2;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_slide::sc25519_slide;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519::{fe25519, fe25519_mul},
    fe25519_add::fe25519_add,
    fe25519_neg::fe25519_neg,
    fe25519_setint::fe25519_setint,
    fe25519_sub::fe25519_sub,
    ge25519::{
        ge25519, ge25519_dbl_p1p1, ge25519_niels, ge25519_nielsadd_p1p1, ge25519_p1p1,
        ge25519_p1p1_to_p2, ge25519_p1p1_to_p3, ge25519_p1p1_to_pniels, ge25519_p3, ge25519_pniels,
        ge25519_pnielsadd_p1p1,
    },
    sc25519::sc25519,
};
const S1_SWINDOWSIZE: u32 = 5;
const PRE1_SIZE: usize = 1 << (S1_SWINDOWSIZE - 2);

const S2_SWINDOWSIZE: u32 = 7;
const PRE2_SIZE: usize = 1 << (S2_SWINDOWSIZE - 2);

const PRE2: [ge25519_niels; PRE2_SIZE] = [ge25519_niels::const_default(); 32];
// {
// #include "ge25519_base_slide_multiples.data"
// };

const EC2D: fe25519 = fe25519 {
    v: [
        1859910466990425,
        932731440258426,
        1072319116312658,
        1815898335770999,
        633789495995903,
    ],
};

pub fn setneutral(r: &mut ge25519) {
    fe25519_setint(&mut r.x, 0);
    fe25519_setint(&mut r.y, 1);
    fe25519_setint(&mut r.z, 1);
    fe25519_setint(&mut r.t, 0);
}

/* computes [s1]p1 + [s2]p2 */
pub fn ge25519_double_scalarmult_vartime(
    r: &mut ge25519_p3,
    p1: &ge25519_p3,
    s1: &sc25519,
    s2: &sc25519,
) {
    let (mut slide1, mut slide2) = ([0i8; 256], [0i8; 256]);
    let mut pre1 = [ge25519_pniels::default(); PRE1_SIZE];
    let pre1p2 = [ge25519_p2::default(); PRE1_SIZE];
    let mut d1 = ge25519_p3::default();
    let mut t = ge25519_p1p1::default();
    let mut i: usize;

    sc25519_slide(&mut slide1, s1, S1_SWINDOWSIZE as i32);
    sc25519_slide(&mut slide2, s2, S2_SWINDOWSIZE as i32);

    /* precomputation */
    pre1[0] = p1.clone().into();
    ge25519_dbl_p1p1(&mut t, &pre1p2);
    ge25519_p1p1_to_p3(&mut d1, &t);
    /* Convert pre[0] to projective Niels representation */
    let mut d = pre1[0].ysubx;
    let (xaddy, ysubx) = (pre1[0].xaddy.clone(), &pre1[0].ysubx.clone());
    fe25519_sub(&mut pre1[0].ysubx, &xaddy, &ysubx);
    let xaddy = pre1[0].xaddy.clone();
    fe25519_add(&mut pre1[0].xaddy, &xaddy, &d);
    let t2d = pre1[0].t2d.clone();
    fe25519_mul(&mut pre1[0].t2d, &t2d, &EC2D);

    for i in 0..PRE1_SIZE - 1 {
        ge25519_pnielsadd_p1p1(&mut t, &d1, &pre1[i]);
        ge25519_p1p1_to_pniels(&mut pre1[i + 1], &t);
    }

    setneutral(r);
    i = 256;
    while i > 0 {
        if slide1[i - 1] != 0 || slide2[i - 1] != 0 {
            break;
        }
        i -= 1;
    }

    while i > 0 {
        ge25519_dbl_p1p1(&mut t, &[r.into()]); //(ge25519_p2 *)

        if slide1[i - 1] > 0 {
            ge25519_p1p1_to_p3(r, &t);
            ge25519_pnielsadd_p1p1(&mut t, r, &pre1[slide1[i - 1] as usize / 2]);
        } else if slide1[i - 1] < 0 {
            ge25519_p1p1_to_p3(r, &t);
            let mut neg = pre1[pre1.len() - slide1[i - 1] as usize / 2];
            d = neg.ysubx;
            neg.ysubx = neg.xaddy;
            neg.xaddy = d;
            let t2d = neg.t2d.clone();
            fe25519_neg(&mut neg.t2d, &t2d);
            ge25519_pnielsadd_p1p1(&mut t, r, &neg);
        }

        if slide2[i - 1] > 0 {
            ge25519_p1p1_to_p3(r, &t);
            ge25519_nielsadd_p1p1(&mut t, r, &PRE2[slide2[i - 1] as usize / 2]);
        } else if slide2[i - 1] < 0 {
            ge25519_p1p1_to_p3(r, &t);
            let mut nneg = PRE2[PRE2.len() - slide2[i - 1] as usize / 2];
            d = nneg.ysubx;
            nneg.ysubx = nneg.xaddy;
            nneg.xaddy = d;
            let t2d = nneg.t2d.clone();
            fe25519_neg(&mut nneg.t2d, &t2d);
            ge25519_nielsadd_p1p1(&mut t, r, &nneg);
        }

        ge25519_p1p1_to_p2(&mut (r.into()), &t); //(ge25519_p2 *)
        i -= 1;
    }
}

// use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::Fe25519;
// use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::Ge25519P3;
// use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::Sc25519;
// /// 对应 ge25519_niels 坐标 (y-x, y+x, 2d*xy)
// #[repr(C)]
// pub struct Ge25519Niels {
//     pub y_sub_x: Fe25519,
//     pub x_add_y: Fe25519,
//     pub t2d: Fe25519,
// }
// impl Ge25519Niels {
//     /// 对应 setneutral
//     pub fn identity() -> Self {
//         Self {
//             y_sub_x: Fe25519::from(0),
//             x_add_y: Fe25519::from(1),
//             t2d: Fe25519::from(1),
//         }
//     }
// }

// impl Ge25519P3 {
//     /// 对应 setneutral
//     pub fn identity() -> Self {
//         Self {
//             x: Fe25519::from(0),
//             y: Fe25519::from(1),
//             z: Fe25519::from(1),
//             t: Fe25519::from(0),
//         }
//     }

//     /// 对应 ge25519_double_scalarmult_vartime
//     /// 计算 s1*P1 + s2*BasePoint
//     pub fn double_scalarmult_vartime(p1: &Ge25519P3, s1: &Sc25519, s2: &Sc25519) -> Self {
//         // 1. 生成滑动窗口表示 (对应 sc25519_slide)
//         let slide1 = s1.to_slide_window(5);
//         let slide2 = s2.to_slide_window(7);

//         // 2. 预计算 P1 的倍数 (对应 pre1)
//         // 逻辑包含：P1, 3P1, 5P1... 转换为 PNiels 坐标
//         let pre1 = p1.build_precomputation_table_5();

//         // 3. 循环计算 (从最高位到最低位)
//         let mut r = Ge25519P3::identity();

//         for i in (0..256).rev() {
//             // 翻倍 (Double)
//             let mut t = r.double_to_p1p1();

//             // 处理 P1 的滑动窗口 (slide1)
//             if slide1[i] != 0 {
//                 r = t.to_p3();
//                 t = r.add_pniels(&pre1[slide1[i]]);
//             }

//             // 处理基点 P2 的滑动窗口 (slide2, 使用预定义的 PRE2 表)
//             if slide2[i] != 0 {
//                 r = t.to_p3();
//                 // t = r.add_niels(&PRE2[slide2[i]]);
//             }

//             r = t.to_p3();
//         }
//         r
//     }
// }
