// #include <stdio.h>
// #include <assert.h>
// #include "randombytes.h"

pub fn randombytes(r: &mut [u8], l: u64) {
    // FILE *fp = fopen("/dev/urandom", "r");  //TODO Remove hard-coded use of /dev/urandom.
    let bytes_read = r.len(); //fread(r, 1, l, fp);
    getrandom(r).expect("get random failed");
    assert!(bytes_read == l as usize);
    // fclose(fp);
}

use getrandom::getrandom;

// /// 對應 randombytes 的 Rust 實現
// pub fn randombytes(dest: &mut [u8]) {
//     // getrandom 會自動處理系統級隨機數源
//     // 如果失敗（極罕見），通常代表系統環境有重大問題
//     getrandom(dest).expect("隨機數生成失敗");
// }
// use rand::RngCore;

// pub fn randombytesc(dest: &mut [u8]) {
//     // 使用 ThreadRng，它內部緩存了從系統獲取的種子，性能更高
//     rand::thread_rng().fill_bytes(dest);
// }
