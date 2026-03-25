// #include "fe25519.h"
// #include "sc25519.h"
// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p2;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519::{fe25519, fe25519_mul},
    fe25519_add::fe25519_add,
    fe25519_setint::fe25519_setint,
    fe25519_sub::fe25519_sub,
    ge25519::{
        choose_t, ge25519_dbl_p1p1, ge25519_niels, ge25519_nielsadd2, ge25519_p1p1,
        ge25519_p1p1_to_p2, ge25519_p1p1_to_p3, ge25519_p3,
    },
    sc25519::sc25519,
    sc25519_window4::sc25519_window4,
};
// /* Multiples of the base point in Niels' representation */
const GE25519_BASE_MULTIPLES_NIELS: &[ge25519_niels] = &[];
//{
// #include "ge25519_base_niels_smalltables.data"
// };

// /* d */
const ECD: fe25519 = fe25519 {
    v: [
        929955233495203,
        466365720129213,
        1662059464998953,
        2033849074728123,
        1442794654840575,
    ],
};

pub fn ge25519_scalarmult_base(r: &mut ge25519_p3, s: &sc25519) {
    let mut b: [i8; 64] = [42; 64];
    let mut t = ge25519_niels::default();
    let mut d = fe25519::default();

    sc25519_window4(&mut b, s);

    let mut tp1p1 = ge25519_p1p1::default();
    choose_t(
        &mut (*r).into(),
        0,
        b[1] as i64,
        &GE25519_BASE_MULTIPLES_NIELS,
    );
    fe25519_sub(&mut d, &r.y, &r.x);
    let (y, x) = (r.y, r.x);
    fe25519_add(&mut r.y, &y, &x);
    r.x = d;
    r.t = r.z;
    fe25519_setint(&mut r.z, 2);
    for i in (3..64).step_by(2) {
        choose_t(
            &mut t,
            i as u64 / 2,
            b[i] as i64,
            GE25519_BASE_MULTIPLES_NIELS,
        );
        ge25519_nielsadd2(r, &t);
    }
    let rr: ge25519_p2 = (r).into();
    ge25519_dbl_p1p1(&mut tp1p1, &[rr]);
    let mut rr: ge25519_p2 = (r).into();
    ge25519_p1p1_to_p2(&mut rr, &tp1p1);
    let rr: ge25519_p2 = (r).into();
    ge25519_dbl_p1p1(&mut tp1p1, &[rr]);
    let mut rr: ge25519_p2 = (r).into();
    ge25519_p1p1_to_p2(&mut rr, &tp1p1);
    let rr: ge25519_p2 = (r).into();
    ge25519_dbl_p1p1(&mut tp1p1, &[rr]);
    let mut rr: ge25519_p2 = (r).into();
    ge25519_p1p1_to_p2(&mut rr, &tp1p1);
    let rr: ge25519_p2 = (r).into();
    ge25519_dbl_p1p1(&mut tp1p1, &[rr]);
    ge25519_p1p1_to_p3(r, &tp1p1);
    choose_t(&mut t, 0u64, b[0] as i64, GE25519_BASE_MULTIPLES_NIELS);
    let t2d = t.t2d;
    fe25519_mul(&mut t.t2d, &t2d, &ECD);
    ge25519_nielsadd2(r, &t);
    for i in (2..64).step_by(2) {
        choose_t(
            &mut t,
            i as u64 / 2,
            b[i] as i64,
            GE25519_BASE_MULTIPLES_NIELS,
        );
        ge25519_nielsadd2(r, &t);
    }
}

// use crate::crypto_sign::ed25519::amd64_51_30k::{
//     fe25519::Fe25519,
//     ge25519::{Ge25519Niels, Ge25519P3},
//     ge25519_double_scalarmult,
//     sc25519::Sc25519,
// };

// // #[derive(Debug, Clone, Copy)]
// // pub struct Fe25519(pub [u64; 5]);

// // 对应你提供的常量 d
// const ECD: Fe25519 = Fe25519 {
//     v: [
//         929955233495203,
//         466365720129213,
//         1662059464998953,
//         2033849074728123,
//         1442794654840575,
//     ],
// };

// // /// 預計算表結構，對應 GE25519_BASE_MULTIPLES_NIELS
// // pub struct BasePrecomputedTable {
// //     // 64 個窗口，每個窗口存儲 8 個預計算好的 Niels 點 (1G, 2G...8G 的變體)
// //     table: [[Ge25519Niels; 8]; 64],
// // }

// impl Ge25519P3 {
//     /// 對應 ge25519_scalarmult_base
//     pub fn scalarmult_base(s: &Sc25519) -> Self {
//         // 1. 將標量 s 切分為 4-bit 窗口 (對應 sc25519_window4)
//         // b 數組長度 64，每個元素範圍 [-8, 8]
//         let b = s.to_window4();

//         let mut r = Ge25519P3::identity();
//         let mut t_niels = Ge25519Niels::identity();

//         // 2. 處理奇數索引窗口 (i=1, 3, 5...63)
//         // 這裡對應 C 代碼的第一個循環，執行 Niels 點的累加
//         // r.lookup_and_add_niels(&b[1], 0); // 對應第一次 choose_t 和轉換

//         for i in (3..64).step_by(2) {
//             // t_niels = lookup_niels(i / 2, b[i]); // 恆定時間查表 (choose_t)
//             // r.niels_add(&t_niels);               // 對應 ge25519_nielsadd2
//         }

//         // 3. 執行 4 次翻倍 (Double)
//         // 因為每個窗口跨度是 4-bit，處理完一組窗口後需要位移
//         for _ in 0..4 {
//             // r = r.double();
//         }

//         // 4. 處理偶數索引窗口 (i=0, 2, 4...62)
//         // 這裡處理剩下的一半窗口
//         for i in (0..64).step_by(2) {
//             // t_niels = lookup_niels(i / 2, b[i]);
//             // 對於 i=0，C 代碼特別處理了與 ECD (d 參數) 的乘法
//             if i == 0 {
//                 t_niels.t2d = t_niels.t2d.mul(&ECD);
//             }
//             // r.niels_add(&t_niels);
//         }

//         r
//     }
// }

// use curve25519_dalek::edwards::EdwardsPoint;
// use curve25519_dalek::traits::Identity;
// use curve25519_dalek::field::FieldElement;
// use curve25519_dalek::constants;
// use subtle::{Choice, ConditionallySelectable};

// // 模擬彙編中的 51-bit 分量結構 (這裡使用 dalek 內置的 FieldElement)
// #[derive(Copy, Clone, Debug)]
// pub struct AffinePoint {
//     pub y_plus_x: FieldElement,  // 對應 txaddy
//     pub y_minus_x: FieldElement, // 對應 tysubx
//     pub xy2d: FieldElement,      // 對應 tt2d
// }

// impl ConditionallySelectable for AffinePoint {
//     fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
//         AffinePoint {
//             y_plus_x: FieldElement::conditional_select(&a.y_plus_x, &b.y_plus_x, choice),
//             y_minus_x: FieldElement::conditional_select(&a.y_minus_x, &b.y_minus_x, choice),
//             xy2d: FieldElement::conditional_select(&a.xy2d, &b.xy2d, choice),
//         }
//     }
// }

// /// 模仿彙編函數的 Rust 實現
// pub fn batch_choose_t(
//     table: &[AffinePoint], // 預計算表基地址 basep
//     pos: usize,            // 偏移量 pos
//     index: i8,             // 選擇索引 b
// ) -> AffinePoint {
//     // 1. 計算絕對值 u = |index|
//     let mask = index >> 7;
//     let u = ((index ^ mask).wrapping_sub(mask)) as u8;

//     // 2. 初始化結果 (對應彙編初始化 tysubx0=1, txaddy0=1)
//     let mut result = AffinePoint {
//         y_plus_x: FieldElement::ONE,
//         y_minus_x: FieldElement::ONE,
//         xy2d: FieldElement::ZERO,
//     };

//     // 3. 常數時間遍歷 (對應彙編中的 cmp + cmove)
//     // 假設每個 window 包含 8 個預計算點
//     for i in 1..=8 {
//         let is_equal = Choice::from(if u == i as u8 { 1 } else { 0 });
//         // 這裡底層會編譯成 CMOV 或位運算掩碼，保證常數時間
//         result.conditional_select(&table[pos + i - 1], is_equal);
//     }

//     // 4. 處理符號 (如果 index < 0)
//     let is_negative = Choice::from((index < 0) as u8);
//     let flipped = AffinePoint {
//         y_plus_x: result.y_minus_x,
//         y_minus_x: result.y_plus_x,
//         xy2d: result.xy2d, // 實際實現中 xy2d 可能需要取反
//     };

//     AffinePoint::conditional_select(&result, &flipped, is_negative)
// }

// fn main() {
//     // 示例：從預計算表中提取一個點
//     // 在實際 Ed25519 中，這些點是預先計算好的曲線基點倍數
//     let mock_table = vec![/* ... 填充預計算點 ... */];

//     let chosen_point = batch_choose_t(&mock_table, 0, -3);
//     println!("Chosen point ready for scalar multiplication.");
// }

// 這是一個非常專業的需求。要將這段 .s 彙編檔案直接整合到 Rust 項目中，我們需要通過 cc crate 進行編譯，並在 Rust 中聲明對應的 extern "C" 函數。
// 以下是完整的實施步驟：
// 1. 準備彙編檔案
// 將你之前的彙編代碼保存為項目根目錄下的 src/asm/choose_t.s。請確保檔案末尾有正確的返回指令（通常是 ret），因為你提供的代碼片段在 cmove 處截斷了。
// 2. 配置 Cargo.toml
// 你需要 cc 庫來編譯彙編代碼：
// toml
// [package]
// name = "ed25519_asm_link"
// version = "0.1.0"
// edition = "2021"

// [build-dependencies]
// cc = "1.0"
//

// 3. 編寫 build.rs
// 在項目根目錄（與 Cargo.toml 同級）創建 build.rs，告訴 Rust 如何編譯這個 .s 檔案：
// rust
// fn main() {
//     // 只有在 x86_64 架構下才編譯此彙編
//     if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64" {
//         cc::Build::new()
//             .file("src/asm/choose_t.s")
//             .compile("choose_t_asm"); // 編譯成 libchoose_t_asm.a
//     }
// }
//

// 4. 在 Rust 中進行 FFI 綁定與調用
// 根據彙編中的 qhasm 定義，該函數接收 4 個 int64（在 Rust 中為 u64 或 *mut u64）。
// rust
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct Fe51([u64; 5]); // Radix 2^51 的場元素

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct PrecomputedPoint {
//     pub tysubx: Fe51, // tysubx0-4
//     pub txaddy: Fe51, // txaddy0-4
//     pub tt2d: Fe51,   // tt2d0-4
// }

// extern "C" {
//     // 根據彙編函數名定義：_crypto_sign_ed25519_amd64_51_30k_batch_choose_t
//     // 參數順序通常對應 rdi, rsi, rdx, rcx
//     fn crypto_sign_ed25519_amd64_51_30k_batch_choose_t(
//         tp: *mut PrecomputedPoint, // rdi
//         pos: u64,                 // rsi
//         b: i64,                  // rdx
//         basep: *const u64,       // rcx
//     );
// }

// fn main() {
//     let mut result = std::mem::MaybeUninit::<PrecomputedPoint>::uninit();
//     let base_table = vec![0u64; 4096]; // 模擬預計算表
//     let pos = 0;
//     let b = 1;

//     unsafe {
//         // 調用彙編函數
//         crypto_sign_ed25519_amd64_51_30k_batch_choose_t(
//             result.as_mut_ptr(),
//             pos,
//             b,
//             base_table.as_ptr(),
//         );

//         let point = result.assume_init();
//         println!("從彙編獲取的點數據: {:?}", point);
//     }
// }
// //

// // 關鍵注意事項：
// // 函數名稱修飾：在 macOS 上，彙編符號通常帶有底線（如 _crypto_sign...），而 Linux 則沒有。cc crate 通常會處理這種差異，但在 extern "C" 聲明時需注意對齊。
// // 堆棧對齊：這段彙編開頭有 sub %r11, %rsp 進行手動堆棧對齊，這符合 AMD64 ABI 要求。
// // 寄存器保護：該代碼手動保存了 rbx, rbp, r12-r15 到棧上，這意味着它是一個「合規」的函數調用，不會損壞 Rust 的運行環境。

// fn main() {
//     // 1. 初始化一個足夠大的預計算表
//     // 彙編中 imulq $960，說明每組數據佔 960 字节
//     // 每個 PrecomputedPoint 佔 120 字节 (15 * 8)
//     // 假設每個 pos 包含 8 個點 (960 / 120 = 8)
//     let points_per_pos = 8;
//     let total_pos = 2;
//     let mut mock_table = vec![PrecomputedPoint::default(); points_per_pos * total_pos];

//     // 2. 填充一些特徵數據以便驗證
//     // 假設我們要找 pos=1, index=3 的點
//     let target_pos = 1;
//     let target_index = 3;
//     let table_idx = (target_pos * points_per_pos) + (target_index - 1);

//     mock_table[table_idx].y_plus_x.0[0] = 0x12345678;
//     mock_table[table_idx].y_minus_x.0[0] = 0x87654321;

//     // 3. 準備接收結果的變量
//     let mut result = PrecomputedPoint::default();

//     // 4. 調用彙編
//     unsafe {
//         crypto_sign_ed25519_amd64_51_30k_batch_choose_t(
//             &mut result as *mut PrecomputedPoint,
//             target_pos as u64,
//             target_index as i64,
//             mock_table.as_ptr() as *const u64,
//         );
//     }

//     // 5. 驗證結果
//     println!("提取結果 (Y+X)[0]: 0x{:x}", result.y_plus_x.0[0]);
//     println!("提取結果 (Y-X)[0]: 0x{:x}", result.y_minus_x.0[0]);

//     assert_eq!(result.y_plus_x.0[0], 0x12345678);
// }

// fn main() {
//     let mut result = PrecomputedPoint::default();
//     let mut mock_table = vec![PrecomputedPoint::default(); 16];

//     // 設置索引為 3 的原始數據
//     let target_idx = 2; // 0-based index for b=3
//     mock_table[target_idx].y_plus_x.0[0] = 0xAAAA_AAAA;
//     mock_table[target_idx].y_minus_x.0[0] = 0xBBBB_BBBB;

//     unsafe {
//         // 調用 b = -3
//         crypto_sign_ed25519_amd64_51_30k_batch_choose_t(
//             &mut result,
//             0,  // pos
//             -3, // b (負數索引)
//             mock_table.as_ptr() as *const u64,
//         );
//     }

//     // 驗證是否發生了交換：
//     // 原本 y_plus_x 是 AAAA，因為 b 為負，現在 result.y_plus_x 應該是 BBBB
//     println!("b = -3 時的 Y+X (應為 0xbbbb...): 0x{:x}", result.y_plus_x.0[0]);
//     println!("b = -3 時的 Y-X (應為 0xaaaa...): 0x{:x}", result.y_minus_x.0[0]);

//     assert_eq!(result.y_plus_x.0[0], 0xBBBB_BBBB);
// }

// 這些數據是 amd64-51-30k 預計算表中的一組常數。每一組包含三個分量（對應 y_plus_x, y_minus_x, xy2d），每個分量由 5 個 u64 組成，採用 Radix
//  表示法。
// 要在 Rust 中使用這些數據進行測試，我們需要將其映射到之前定義的 PrecomputedPoint 結構中。
// 1. 將常數數據映射到 Rust 結構
// 你可以將這些數據放入一個靜態數組中，以便 batch_choose_t 函數調用：
// rust
// // 每個點由 3 個 FieldElement 組成，每個 FieldElement 有 5 個 u64
// const BASE_TABLE: [[[u64; 5]; 3]; 26] = [
//     [
//         [0x00003905d740913e, 0x0000ba2817d673a2, 0x00023e2827f4e67c, 0x000133d2e0c21a34, 0x00044fd2f9298f81], // y_plus_x
//         [0x000493c6f58c3b85, 0x0000df7181c325f7, 0x0000f50b0b3e4cb7, 0x0005329385a44c32, 0x00007cf9d3a33d4b], // y_minus_x
//         [0x000515674b6fbb59, 0x00001dd454bd5b77, 0x00055f1be90784fc, 0x00066566ea4e8e64, 0x0004f0ebe1faf16e], // xy2d
//     ],
//     // ... 依此類推放入你提供的所有數據 ...
// ];
//

// 2. 數據佈局分析
// 你提供的數據格式如下：
// 外層結構：{{{...}, {...}, {...}}}, ...
// 單個點 (PrecomputedPoint)：
// 第一組 {...}：

//  (對應匯編中的 txaddy)
// 第二組 {...}：

//  (對應匯編中的 tysubx)
// 第三組 {...}：
//  (對應匯編中的 tt2d)
// 數值含義：例如 0x00003905d740913e 是分量 0 (低 51 位)，0x00044fd2f9298f81 是分量 4 (最高位部分)。
// 3. Rust 調用與驗證範例
// 如果你想驗證匯編函數是否正確提取了第一組數據（當

//  時）：
// rust
// fn main() {
//     let mut result = PrecomputedPoint::default();

//     // 將數據轉為扁平的 u64 數組以符合匯編接口 (basep)
//     // 匯編中 pos * 960 音譯為每組 pos 包含 8 個點 (8 * 120 bytes = 960)
//     let mut flat_table = vec![0u64; 960 / 8 * 8 * 2]; // 預留空間

//     // 將你提供的第一組數據填入 table[0]
//     let first_point = [
//         0x00003905d740913e, 0x0000ba2817d673a2, 0x00023e2827f4e67c, 0x000133d2e0c21a34, 0x00044fd2f9298f81, // y_plus_x
//         0x000493c6f58c3b85, 0x0000df7181c325f7, 0x0000f50b0b3e4cb7, 0x0005329385a44c32, 0x00007cf9d3a33d4b, // y_minus_x
//         0x000515674b6fbb59, 0x00001dd454bd5b77, 0x00055f1be90784fc, 0x00066566ea4e8e64, 0x0004f0ebe1faf16e, // xy2d
//     ];
//     flat_table[0..15].copy_from_slice(&first_point);

//     unsafe {
//         crypto_sign_ed25519_amd64_51_30k_batch_choose_t(
//             &mut result,
//             0, // pos = 0
//             1, // b = 1 (提取第一個點)
//             flat_table.as_ptr(),
//         );
//     }

//     // 驗證 tysubx (y_minus_x) 是否正確提取
//     // 注意：匯編中 tysubx 偏移是 0，txaddy 偏移是 40
//     println!("提取的 y_minus_x[0]: 0x{:x}", result.y_minus_x.0[0]);
//     assert_eq!(result.y_minus_x.0[0], 0x000493c6f58c3b85);
// }
//

// 注意事項
// 分量順序：請仔細核對你的匯編代碼中 0(%rdi) 寫入的是哪個分量。根據你之前的代碼 tysubx0 = t if = 且 t = *(basep + 0 + pos)，說明 y_minus_x 位於結構體的起始位置（偏移 0）。
// 負數索引測試：如果你調用

// ，結果中的 y_plus_x 應該會變成 0x000493c6f58c3b85（即原本的
