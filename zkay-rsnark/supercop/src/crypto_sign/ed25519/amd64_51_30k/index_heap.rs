// #include "sc25519.h"
// #include "index_heap.h"
// #ifndef INDEX_HEAP_H
// #define INDEX_HEAP_H
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::{sc25519, sc25519_lt};
// #include "sc25519.h"

// #define heap_init                crypto_sign_ed25519_amd64_51_30k_batch_heap_init
// #define heap_extend              crypto_sign_ed25519_amd64_51_30k_batch_heap_extend
// #define heap_pop                 crypto_sign_ed25519_amd64_51_30k_batch_heap_pop
// #define heap_push                crypto_sign_ed25519_amd64_51_30k_batch_heap_push
// #define heap_get2max             crypto_sign_ed25519_amd64_51_30k_batch_heap_get2max
// #define heap_rootreplaced        crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced
// #define heap_rootreplaced_3limbs crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced_3limbs
// #define heap_rootreplaced_2limbs crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced_2limbs
// #define heap_rootreplaced_1limb  crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced_1limb

// void heap_init(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);

// void heap_extend(unsigned long long *h, unsigned long long oldlen, unsigned long long newlen, sc25519 *scalars);

// unsigned long long heap_pop(unsigned long long *h, unsigned long long *hlen, sc25519 *scalars);

// void heap_push(unsigned long long *h, unsigned long long *hlen, unsigned long long elem, sc25519 *scalars);

// void heap_get2max(unsigned long long *h, unsigned long long *max1, unsigned long long *max2, sc25519 *scalars);

// void heap_rootreplaced(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
// void heap_rootreplaced_3limbs(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
// void heap_rootreplaced_2limbs(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
// void heap_rootreplaced_1limb(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);

// #endif

pub fn heap_rootreplaced(h: &mut u64, hlen: u64, scalars: &sc25519) {}
pub fn heap_rootreplaced_3limbs(h: &mut u64, hlen: u64, scalars: &sc25519) {}
pub fn heap_rootreplaced_2limbs(h: &mut u64, hlen: u64, scalars: &sc25519) {}
pub fn heap_rootreplaced_1limb(h: &mut u64, hlen: u64, scalars: &sc25519) {}
unsafe extern "C" {
    // heap_init(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
    // pub fn heap_init(h: *mut u64, hlen: u64, scalars: *const sc25519);
    /// 扩展堆的大小并重新调整堆结构
    /// h: 堆数组指针
    /// oldlen: 当前堆的长度
    /// newlen: 扩展后的新长度
    /// scalars: 指向标量数组的指针
    // pub fn heap_extend(h: *mut u64, oldlen: u64, newlen: u64, scalars: *const sc25519);
    // heap_pop(unsigned long long *h, unsigned long long *hlen, sc25519 *scalars);
    pub fn heap_pop(h: *mut u64, hlen: *mut u64, scalars: *const sc25519) -> u64;

    // heap_push(unsigned long long *h, unsigned long long *hlen, unsigned long long elem, sc25519 *scalars);
    // pub fn heap_push(h: *mut u64, hlen: *mut u64, elem: u64, scalars: *const sc25519);
    /// 获取堆中最大的两个元素的索引
    /// h: 堆数组指针
    /// max1: 用于接收最大元素索引的指针 (unsigned long long *)
    /// max2: 用于接收次大元素索引的指针 (unsigned long long *)
    /// scalars: 指向标量数组的指针，用于比较权重
    // pub fn heap_get2max(h: *mut u64, max1: *mut u64, max2: *mut u64, scalars: *const sc25519);
    // heap_rootreplaced(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced(
        h: *mut u64,
        hlen: u64,
        scalars: *const sc25519,
    );

    // 针对不同 limb 的优化版本
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced_3limbs(
        h: *mut u64,
        hlen: u64,
        scalars: *const sc25519,
    );
    /// 针对 2 个 limb（通常指 128 位）优化的堆顶元素替换函数
    /// h: 堆数组指针
    /// hlen: 当前堆的长度
    /// scalars: 权重/标量数组指针
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced_2limbs(
        h: *mut u64,
        hlen: u64,
        scalars: *const sc25519,
    );
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_heap_rootreplaced_1limb(
        h: *mut u64,
        hlen: u64,
        scalars: *const sc25519,
    );
}

// /* caller's responsibility to ensure hlen>=3 */
pub fn heap_init(h: &mut Vec<u64>, hlen: u64, scalars: &[sc25519]) {
    h[0] = 0;
    let mut i: u64 = 1;
    while i < hlen {
        let ii = i;
        heap_push(h, &mut i, ii, scalars);
    }
}

pub fn heap_extend(h: &mut Vec<u64>, oldlen: u64, newlen: u64, scalars: &[sc25519]) {
    let mut i = oldlen;
    while i < newlen {
        let ii = i;
        heap_push(h, &mut i, ii, scalars);
    }
}

pub fn heap_push(h: &mut Vec<u64>, hlen: &mut u64, elem: u64, scalars: &[sc25519]) {
    /* Move up towards the root */
    /* XXX: Check size of hlen, whether cast to signed value is ok */
    let mut pos: i64 = (*hlen) as i64;
    let mut ppos: i64 = (pos - 1) / 2;
    let mut t: u64;
    h[*hlen as usize] = elem;
    while pos > 0 {
        /* if(sc25519_lt_vartime(&scalars[h[ppos]], &scalars[h[pos]])) */
        if sc25519_lt(
            &scalars[h[ppos as usize] as usize],
            &scalars[h[pos as usize] as usize],
        ) != 0
        {
            t = h[ppos as usize];
            h[ppos as usize] = h[pos as usize];
            h[pos as usize] = t;
            pos = ppos;
            ppos = (pos - 1) / 2;
        } else {
            break;
        }
    }
    (*hlen) += 1;
}

// /* Put the largest value in the heap in max1, the second largest in max2 */
pub fn heap_get2max(h: &mut Vec<u64>, max1: &mut u64, max2: &mut u64, scalars: &[sc25519]) {
    *max1 = h[0];
    *max2 = h[1];
    if sc25519_lt(&scalars[h[1] as usize], &scalars[h[2] as usize]) != 0 {
        *max2 = h[2];
    }
}

// /* After the root has been replaced, restore heap property */
// /* extern void heap_rootreplaced(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
// */
// /* extern void heap_rootreplaced_shortscalars(unsigned long long *h, unsigned long long hlen, sc25519 *scalars);
// */
// /// 索引堆管理器
// pub struct IndexMaxHeap<'a> {
//     pub h: Vec<usize>,          // 存儲索引
//     pub scalars: &'a [Sc25519], // 指向標量數組的引用，用於比較
// }

// impl<'a> IndexMaxHeap<'a> {
//     /// 對應 heap_init 和 heap_push
//     pub fn new(num_elements: usize, scalars: &'a [Sc25519]) -> Self {
//         let mut heap = Self {
//             h: Vec::with_capacity(num_elements),
//             scalars,
//         };
//         for i in 0..num_elements {
//             heap.push(i);
//         }
//         heap
//     }

//     /// 對應 heap_push
//     pub fn push(&mut self, elem_idx: usize) {
//         let mut pos = self.h.len();
//         self.h.push(elem_idx);

//         // Sift Up (向上調整)
//         while pos > 0 {
//             let ppos = (pos - 1) / 2;
//             // 比較標量值: if scalars[h[ppos]] < scalars[h[pos]]
//             if self.scalars[self.h[ppos]] < self.scalars[self.h[pos]] {
//                 self.h.swap(pos, ppos);
//                 pos = ppos;
//             } else {
//                 break;
//             }
//         }
//     }

//     /// 對應 heap_get2max
//     /// 獲取標量值最大的兩個索引
//     pub fn get_2_max(&self) -> (usize, usize) {
//         let max1 = self.h[0];
//         // 在最大堆中，第二大值必然是根節點的左右子節點之一
//         let left = self.h[1];
//         let right = self.h[2];

//         let max2 = if self.scalars[left] < self.scalars[right] {
//             right
//         } else {
//             left
//         };
//         (max1, max2)
//     }

//     /// 對應 heap_extend
//     pub fn extend(&mut self, new_len: usize) {
//         let old_len = self.h.len();
//         for i in old_len..new_len {
//             self.push(i);
//         }
//     }
// }
