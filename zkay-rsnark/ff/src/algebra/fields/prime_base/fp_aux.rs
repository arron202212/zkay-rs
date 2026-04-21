// #![feature(asm_const)] // 如果需要使用常量偏移
use std::arch::asm;
pub fn mul_reduce_n3(a: &[u64; 3], b: &[u64; 3], modulus: &[u64; 3], inv: u64) -> [u64; 3] {
    let mut res = [0u64; 6];
    unsafe {
        // 1. 乘法
        // 这里可以直接调用 Rust 现成的高性能库或上述汇编
        comba_3_by_3_mul(&mut res, a, b);
        // 2. 约减
        reduce_6_limb_product(&mut res, modulus, inv);

        // 3. 最终条件减法 (之前为你提供的实现)
        let mut final_res = [res[3], res[4], res[5]];
        montgomery_finalize_3_limbs(&mut final_res, modulus);

        final_res
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn comba_3_by_3_mul(res: &mut [u64; 6], a: &[u64; 3], b: &[u64; 3]) {
    let mut c0: u64;
    let mut c1: u64;
    let mut c2: u64;

    std::arch::asm!(
        // --- Round 0 ---
        "mov rax, [ {A} + 0 ]",
        "mul qword ptr [ {B} + 0 ]",
        "mov [ {res} + 0 ], rax",
        "mov {c0}, rdx",

        "xor {c1}, {c1}",
        "mov rax, [ {A} + 0 ]",
        "mul qword ptr [ {B} + 8 ]",
        "add {c0}, rax",
        "adc {c1}, rdx",

        "xor {c2}, {c2}",
        "mov rax, [ {A} + 8 ]",
        "mul qword ptr [ {B} + 0 ]",
        "add {c0}, rax",
        "mov [ {res} + 8 ], {c0}",
        "adc {c1}, rdx",
        "adc {c2}, 0",

        // --- Round 1 (register renaming c1, c2, c0) ---
        "xor {c0}, {c0}",
        "mov rax, [ {A} + 0 ]",
        "mul qword ptr [ {B} + 16 ]",
        "add {c1}, rax",
        "adc {c2}, rdx",
        "adc {c0}, 0",

        "mov rax, [ {A} + 8 ]",
        "mul qword ptr [ {B} + 8 ]",
        "add {c1}, rax",
        "adc {c2}, rdx",
        "adc {c0}, 0",

        "mov rax, [ {A} + 16 ]",
        "mul qword ptr [ {B} + 0 ]",
        "add {c1}, rax",
        "mov [ {res} + 16 ], {c1}",
        "adc {c2}, rdx",
        "adc {c0}, 0",

        // --- Round 2 (register renaming c2, c0, c1) ---
        "xor {c1}, {c1}",
        "mov rax, [ {A} + 8 ]",
        "mul qword ptr [ {B} + 16 ]",
        "add {c2}, rax",
        "adc {c0}, rdx",
        "adc {c1}, 0",

        "mov rax, [ {A} + 16 ]",
        "mul qword ptr [ {B} + 8 ]",
        "add {c2}, rax",
        "mov [ {res} + 24 ], {c2}",
        "adc {c0}, rdx",
        "adc {c1}, 0",

        // --- Final Round (register renaming c0, c1, c2) ---
        "xor {c2}, {c2}",
        "mov rax, [ {A} + 16 ]",
        "mul qword ptr [ {B} + 16 ]",
        "add {c0}, rax",
        "mov [ {res} + 32 ], {c0}",
        "adc {c1}, rdx",
        "mov [ {res} + 40 ], {c1}",

        // 绑定操作数
        c0 = out(reg) c0,
        c1 = out(reg) c1,
        c2 = out(reg) c2,
        res = in(reg) res.as_mut_ptr(),
        A = in(reg) a.as_ptr(),
        B = in(reg) b.as_ptr(),
        out("rax") _,
        out("rdx") _,
        // clobber_abi("C")
    );
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn reduce_6_limb_product(res: &mut [u64; 6], modulus: &[u64; 3], inv: u64) {
    let mut k: u64;
    let mut tmp1: u64;
    let mut tmp2: u64;
    let mut tmp3: u64;

    std::arch::asm!(
        // --- 第一轮迭代 ---
        "mov rax, [ {res} + 0 ]",
        "mul {modprime}",
        "mov {k}, rax",

        "mov rax, [ {mod} + 0 ]",
        "mul {k}",
        "mov {tmp1}, rax",
        "mov {tmp2}, rdx",

        "xor {tmp3}, {tmp3}",
        "mov rax, [ {mod} + 8 ]",
        "mul {k}",
        "add [ {res} + 0 ], {tmp1}",
        "adc {tmp2}, rax",
        "adc {tmp3}, rdx",

        "xor {tmp1}, {tmp1}",
        "mov rax, [ {mod} + 16 ]",
        "mul {k}",
        "add [ {res} + 8 ], {tmp2}",
        "adc {tmp3}, rax",
        "adc {tmp1}, rdx",

        "add [ {res} + 16 ], {tmp3}",
        "adc [ {res} + 24 ], {tmp1}",
        "adc qword ptr [ {res} + 32 ], 0",
        "adc qword ptr [ {res} + 40 ], 0",

        // --- 第二轮迭代 ---
        "mov rax, [ {res} + 8 ]",
        "mul {modprime}",
        "mov {k}, rax",

        "mov rax, [ {mod} + 0 ]",
        "mul {k}",
        "mov {tmp1}, rax",
        "mov {tmp2}, rdx",

        "xor {tmp3}, {tmp3}",
        "mov rax, [ {mod} + 8 ]",
        "mul {k}",
        "add [ {res} + 8 ], {tmp1}",
        "adc {tmp2}, rax",
        "adc {tmp3}, rdx",

        "xor {tmp1}, {tmp1}",
        "mov rax, [ {mod} + 16 ]",
        "mul {k}",
        "add [ {res} + 16 ], {tmp2}",
        "adc {tmp3}, rax",
        "adc {tmp1}, rdx",

        "add [ {res} + 24 ], {tmp3}",
        "adc [ {res} + 32 ], {tmp1}",
        "adc qword ptr [ {res} + 40 ], 0",

        // --- 第三轮迭代 ---
        "mov rax, [ {res} + 16 ]",
        "mul {modprime}",
        "mov {k}, rax",

        "mov rax, [ {mod} + 0 ]",
        "mul {k}",
        "mov {tmp1}, rax",
        "mov {tmp2}, rdx",

        "xor {tmp3}, {tmp3}",
        "mov rax, [ {mod} + 8 ]",
        "mul {k}",
        "add [ {res} + 16 ], {tmp1}",
        "adc {tmp2}, rax",
        "adc {tmp3}, rdx",

        "xor {tmp1}, {tmp1}",
        "mov rax, [ {mod} + 16 ]",
        "mul {k}",
        "add [ {res} + 24 ], {tmp2}",
        "adc {tmp3}, rax",
        "adc {tmp1}, rdx",

        "add [ {res} + 32 ], {tmp3}",
        "adc [ {res} + 40 ], {tmp1}",

        // 绑定寄存器
        k = out(reg) k,
        tmp1 = out(reg) tmp1,
        tmp2 = out(reg) tmp2,
        tmp3 = out(reg) tmp3,
        modprime = in(reg) inv,
        res = in(reg) res.as_mut_ptr(),
        mod = in(reg) modulus.as_ptr(),
        out("rax") _,
        out("rdx") _,
        // clobber_abi("C")
    );
}
#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn montgomery_finalize_3_limbs(res_hi: &mut [u64; 3], modulus: &[u64; 3]) {
    // res_hi 对应原 C++ 中的 res + n (即约减后的高位部分)
    // modulus 对应 modulus.data

    let r_ptr = res_hi.as_mut_ptr();
    let m_ptr = modulus.as_ptr();

    core::arch::asm!(
        //1. MONT_CMP: 从高位到低位比较 (16, 8, 0 字节偏移)
        "mov {tmp}, [{m_ptr} + 16]",
        "cmp [{r_ptr} + 16], {tmp}",
        "jb 3f", // res < mod, 跳转到 done (2f)
        "ja 2f", // res > mod, 跳转到 subtract (1f)

        "mov {tmp}, [{m_ptr} + 8]",
        "cmp [{r_ptr} + 8], {tmp}",
        "jb 3f",
        "ja 2f",

        "mov {tmp}, [{m_ptr}]",
        "cmp [{r_ptr}], {tmp}",
        "jb 3f",

        //2. MONT_SUB: 结果大于等于模数，执行减法
        "2:", // subtract 标签
        "mov {tmp}, [{m_ptr}]",
        "sub [{r_ptr}], {tmp}",

        "mov {tmp}, [{m_ptr} + 8]",
        "sbb [{r_ptr} + 8], {tmp}",

        "mov {tmp}, [{m_ptr} + 16]",
        "sbb [{r_ptr} + 16], {tmp}",

        "3:", // done 标签
        m_ptr = in(reg) m_ptr,
        r_ptr = in(reg) r_ptr,
        tmp = out(reg) _,     // 对应原代码中的 %rax
        // clobber_abi("C"),     // 保护寄存器并标记状态位(cc)改变
    );
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn comba_3_by_3_mull(a: *const u64, b: *const u64, res: *mut u64) {
    core::arch::asm!(
        // Column 0: A[0]*B[0]
        "mov rax, [{a}]",
        "mul qword ptr [{b}]",
        "mov [{res}], rax",
        "mov r8, rdx",       // r8:r13 是累加寄存器

        // Column 1: A[0]*B[1] + A[1]*B[0]
        "mov rax, [{a}]",
        "mul qword ptr [{b} + 8]",
        "add r8, rax",
        "adc rdx, 0",
        "mov r9, rdx",

        "mov rax, [{a} + 8]",
        "mul qword ptr [{b}]",
        "add r8, rax",
        "adc r9, rdx",
        "adc r10, 0",
        "mov [{res} + 8], r8",

        // Column 2: A[0]*B[2] + A[1]*B[1] + A[2]*B[0]
        "mov rax, [{a}]",
        "mul qword ptr [{b} + 16]",
        "add r9, rax",
        "adc r10, rdx",
        "adc r11, 0",

        "mov rax, [{a} + 8]",
        "mul qword ptr [{b} + 8]",
        "add r9, rax",
        "adc r10, rdx",
        "adc r11, 0",

        "mov rax, [{a} + 16]",
        "mul qword ptr [{b}]",
        "add r9, rax",
        "adc r10, rdx",
        "adc r11, 0",
        "mov [{res} + 16], r9",

        // Column 3: A[1]*B[2] + A[2]*B[1]
        "mov rax, [{a} + 8]",
        "mul qword ptr [{b} + 16]",
        "add r10, rax",
        "adc r11, rdx",
        "adc r12, 0",

        "mov rax, [{a} + 16]",
        "mul qword ptr [{b} + 8]",
        "add r10, rax",
        "adc r11, rdx",
        "adc r12, 0",
        "mov [{res} + 24], r10",

        // Column 4: A[2]*B[2]
        "mov rax, [{a} + 16]",
        "mul qword ptr [{b} + 16]",
        "add r11, rax",
        "adc r12, rdx",
        "adc r13, 0",
        "mov [{res} + 32], r11",
        "mov [{res} + 40], r12", // 最后一个位由 r12 和进位 r13 处理

        a = in(reg) a,
        b = in(reg) b,
        res = in(reg) res,
        out("rax") _, out("rdx") _,
        out("r8") _, out("r9") _, out("r10") _,
        out("r11") _, out("r12") _, out("r13") _,
        clobber_abi("C")
    );
}
// #[cfg(target_arch = "x86_64")]
// #[inline(always)]
// unsafe fn reduce_3_limbs(res: *mut u64, modulus: *const u64, inv: u64) {
//     // 逻辑：对 res[0], res[1], res[2] 依次进行约减，将进位累加到高位
//     for i in 0..3 {
//         core::arch::asm!(
//             // k = res[i] * inv mod 2^64
//             "mov rax, [{res} + {offset}]",
//             "mul {inv}",
//             "mov r8, rax", // r8 = k

//             // res[i..i+3] += k * modulus
//             "mul qword ptr [{m}]",
//             "add rax, [{res} + {offset}]", // 此时 rax 必然为 0，进位在 rdx
//             "mov r9, rdx",
//             "adc r9, 0", // r9 暂存进位

//             "mov rax, r8",
//             "mul qword ptr [{m} + 8]",
//             "add rax, r9",
//             "adc rdx, 0",
//             "add rax, [{res} + {offset} + 8]",
//             "mov [{res} + {offset} + 8], rax",
//             "mov r9, rdx",
//             "adc r9, 0",

//             "mov rax, r8",
//             "mul qword ptr [{m} + 16]",
//             "add rax, r9",
//             "adc rdx, 0",
//             "add rax, [{res} + {offset} + 16]",
//             "mov [{res} + {offset} + 16], rax",
//             "add [{res} + {offset} + 24], rdx", // 进位传递到更高位

//             res = in(reg) res,
//             m = in(reg) modulus,
//             inv = in(reg) inv,
//             offset = const i * 8,
//             out("rax") _, out("rdx") _, out("r8") _, out("r9") _,
//         );
//     }
// }

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn comba_3_by_3_sqr(res: &mut [u64; 6], a: &[u64; 3]) {
    let mut c0: u64;
    let mut c1: u64;
    let mut c2: u64;

    std::arch::asm!(
        // --- Round 0: a0 * a0 ---
        "xor {c1}, {c1}",
        "xor {c2}, {c2}",
        "mov rax, [ {A} + 0 ]",
        "mul rax",
        "mov [ {res} + 0 ], rax",
        "mov {c0}, rdx",

        // --- Round 1: 2 * (a0 * a1) ---
        "mov rax, [ {A} + 0 ]",
        "mul qword ptr [ {A} + 8 ]",
        "add {c0}, rax",
        "adc {c1}, rdx",
        "add {c0}, rax", // 乘以 2 的第一部分
        "mov [ {res} + 8 ], {c0}",
        "adc {c1}, rdx", // 乘以 2 的第二部分
        "adc {c2}, 0",

        // --- Round 2: 2 * (a0 * a2) + a1 * a1 (register renaming c1, c2, c0) ---
        "mov rax, [ {A} + 0 ]",
        "xor {c0}, {c0}",
        "mul qword ptr [ {A} + 16 ]",
        "add {c1}, rax",
        "adc {c2}, rdx",
        "adc {c0}, 0",
        "add {c1}, rax", // 乘以 2 的第一部分
        "adc {c2}, rdx", // 乘以 2 的第二部分
        "adc {c0}, 0",

        "mov rax, [ {A} + 8 ]",
        "mul rax",       // a1 * a1 不需要乘以 2
        "add {c1}, rax",
        "mov [ {res} + 16 ], {c1}",
        "adc {c2}, rdx",
        "adc {c0}, 0",

        // --- Round 3: 2 * (a1 * a2) (register renaming c2, c0, c1) ---
        "mov rax, [ {A} + 8 ]",
        "xor {c1}, {c1}",
        "mul qword ptr [ {A} + 16 ]",
        "add {c2}, rax",
        "adc {c0}, rdx",
        "adc {c1}, 0",
        "add {c2}, rax", // 乘以 2
        "mov [ {res} + 24 ], {c2}",
        "adc {c0}, rdx",
        "adc {c1}, 0",

        // --- Final Round: a2 * a2 (register renaming c0, c1, c2) ---
        "mov rax, [ {A} + 16 ]",
        "mul rax",
        "add {c0}, rax",
        "mov [ {res} + 32 ], {c0}",
        "adc {c1}, rdx",
        "mov [ {res} + 40 ], {c1}",

        // 绑定操作数
        c0 = out(reg) c0,
        c1 = out(reg) c1,
        c2 = out(reg) c2,
        res = in(reg) res.as_mut_ptr(),
        A = in(reg) a.as_ptr(),
        out("rax") _,
        out("rdx") _,
        // clobber_abi("C")
    );
}

#[cfg(all(target_arch = "x86_64", feature = "asm"))]
pub fn squared_nn3(a: &[u64; 3], modulus: &[u64; 3], inv: u64) -> [u64; 3] {
    let mut res = [0u64; 6]; // 存储 2n 的中间结果

    unsafe {
        // --- 第一阶段：Comba Squaring (3x3 平方) ---
        // 这里简化展示逻辑，实际会利用 rdx:rax 进行大量累加
        core::arch::asm!(
            //伪代码逻辑：计算 a[0..3] * a[0..3] 得到 res[0..6]
            // 实际上会包含大量的 mulq, add, adc 指令来最小化寄存器溢出
            // ... (此处省略复杂的 Comba 进位链，通常由脚本生成)

            // --- 第二阶段：Montgomery Reduction (约减) ---
            //使用 REDUCE_6_LIMB_PRODUCT 逻辑
            // 将 res[0..6] 约减为 res[3..6] (即 res+n)

            // --- 第三阶段：条件减法 (MONT_CMP & MONT_SUB) ---
            "mov {rax}, [{m} + 16]",
            "cmp [{res_high} + 16], {rax}",
            "jb 2f",
            "ja 1f",
            "mov {rax}, [{m} + 8]",
            "cmp [{res_high} + 8], {rax}",
            "jb 2f",
            "ja 1f",
            "mov {rax}, [{m}]",
            "cmp [{res_high}], {rax}",
            "jb 2f",

            "1:", // subtract
            "mov {rax}, [{m}]",
            "sub [{res_high}], {rax}",
            "mov {rax}, [{m} + 8]",
            "sbb [{res_high} + 8], {rax}",
            "mov {rax}, [{m} + 16]",
            "sbb [{res_high} + 16], {rax}",

            "2:", // done
            res_high = in(reg) &mut res[3], // 对应 C++ 的 res + n
            m = in(reg) modulus.as_ptr(),
            inv = in(reg) inv,
            rax = out(reg) _,
            clobber_abi("C"),
        );
    }

    let mut out = [0u64; 3];
    out.copy_from_slice(&res[3..6]);
    out
}
#[cfg(target_arch = "x86_64")]
pub fn squared_n3(a: &[u64; 3], modulus: &[u64; 3], inv: u64) -> [u64; 3] {
    let mut res = [0u64; 6];
    let mut output = [0u64; 3];

    unsafe {
        // --- 1. COMBA SQUARING (3x3 SQR -> 6 limbs) ---
        comba_3_by_3_sqr(&mut res, a);

        // --- 2. MONTGOMERY REDUCE ---
        // 调用之前实现的约减逻辑（确保标签从 2 开始）
        reduce_6_limb_product(&mut res, modulus, inv);

        // --- 3. FINAL SUBTRACTION (修复标签) ---
        let res_hi = &mut res[3..6];
        core::arch::asm!(
            "mov rax, [{m} + 16]",
            "cmp [{r} + 16], rax",
            "jb 3f", "ja 2f",
            "mov rax, [{m} + 8]",
            "cmp [{r} + 8], rax",
            "jb 3f", "ja 2f",
            "mov rax, [{m}]",
            "cmp [{r}], rax",
            "jb 3f",
            "2:",
            "mov rax, [{m}]", "sub [{r}], rax",
            "mov rax, [{m} + 8]", "sbb [{r} + 8], rax",
            "mov rax, [{m} + 16]", "sbb [{r} + 16], rax",
            "3:",
            r = in(reg) res_hi.as_mut_ptr(),
            m = in(reg) modulus.as_ptr(),
            out("rax") _,
        );

        output.copy_from_slice(&res[3..6]);
    }
    output
}

// // 假设 bigint 存储在 [u64; N] 中
// pub struct BigInt<const N: usize> {
//     pub data: [u64; N],
// }

// pub struct FpModel<const N: usize> {
//     pub mont_repr: BigInt<N>,
//     pub modulus: BigInt<N>,
//     pub inv: u64,
// }

// impl<const N: usize> FpModel<N> {
//     pub fn mul_reduce(&mut self, other: &BigInt<N>) {
//         #[cfg(all(target_arch = "x86_64", feature = "asm"))]
//         {
//             match N {
//                 3 => {
//                     let mut res = [0u64; 6];
//                     unsafe {
//                         comba_3_by_3_mul(&mut res, &self.mont_repr.data, &other.data);
//                         reduce_6_limb_product(&mut res, &self.modulus.data, self.inv);
//                         // 最终条件减法
//                         final_sub_n3(&mut res[3..6]);
//                         self.mont_repr.data.copy_from_slice(&res[3..6]);
//                     }
//                     return;
//                 }
//                 4 => {
//                     let mut tmp = [0u64; 4]; // CIOS 通常直接在 tmp 中完成
//                     unsafe {
//                         self.cios_asm_n4(&mut tmp, &other.data);
//                         self.mont_repr.data.copy_from_slice(&tmp);
//                     }
//                     return;
//                 }
//                 5 => {
//                     let mut tmp = [0u64; 5];
//                     unsafe {
//                         self.cios_asm_n5(&mut tmp, &other.data);
//                         self.mont_repr.data.copy_from_slice(&tmp);
//                     }
//                     return;
//                 }
//                 _ => {} // 其他情况走通用逻辑
//             }
//         }

//         // 通用 Fallback 逻辑 (对应原代码 else 部分)
//         let mut res = [0u64; 2 * N];
//         self.mpn_mul_n(&mut res, &self.mont_repr.data, &other.data);

//         for i in 0..N {
//             let k = self.inv.wrapping_mul(res[i]);
//             let carry = self.mpn_addmul_1(&mut res[i..], &self.modulus.data, k);
//             self.mpn_add_1(&mut res[N + i..], carry);
//         }

//         let res_high = &mut res[N..2 * N];
//         if self.mpn_cmp(res_high, &self.modulus.data) >= 0 {
//             self.mpn_sub(res_high, &self.modulus.data);
//         }
//         self.mont_repr.data.copy_from_slice(res_high);
//     }

//     /// 对应原代码中的 MONT_CMP 和 MONT_SUB 汇编块
//     #[cfg(target_arch = "x86_64")]
//     unsafe fn final_sub_n3(&self, r: &mut [u64]) {
//         let m = &self.modulus.data;
//         std::arch::asm!(
//             // 检查 overflow: r >= m
//             "mov rax, [{r} + 16]",
//             "cmp rax, [{m} + 16]",
//             "ja {sub_label}",
//             "jb {done_label}",
//             "mov rax, [{r} + 8]",
//             "cmp rax, [{m} + 8]",
//             "ja {sub_label}",
//             "jb {done_label}",
//             "mov rax, [{r} + 0]",
//             "cmp rax, [{m} + 0]",
//             "jb {done_label}",

//             "{sub_label}:",
//             "mov rax, [{m} + 0]",
//             "sub [{r} + 0], rax",
//             "mov rax, [{m} + 8]",
//             "sbb [{r} + 8], rax",
//             "mov rax, [{m} + 16]",
//             "sbb [{r} + 16], rax",

//             "{done_label}:",
//             r = in(reg) r.as_mut_ptr(),
//             m = in(reg) m.as_ptr(),
//             sub_label = label {
//                 // 这里的 label 语法需要具体根据 rustc 版本，
//                 // 简单做法是直接在 asm 中写本地标签如 1: 和 2:
//             },
//             out("rax") _,
//             clobber_abi("C")
//         );
//     }
// }
// #[cfg(target_arch = "x86_64")]
// #[inline(always)]
// unsafe fn mont_final_sub_n4(tmp: &mut [u64; 5], m: &[u64; 4]) {
//     // 对应 MONT_CMP(24) -> (0) 和 MONT_FIRSTSUB -> MONT_NEXTSUB(24)
//     std::arch::asm!(
//         // --- MONT_CMP: 从高位向低位比较 ---
//         "mov rax, [{tmp} + 24]", "cmp rax, [{m} + 24]", "ja 2f", "jb 3f",
//         "mov rax, [{tmp} + 16]", "cmp rax, [{m} + 16]", "ja 2f", "jb 3f",
//         "mov rax, [{tmp} + 8]",  "cmp rax, [{m} + 8]",  "ja 2f", "jb 3f",
//         "mov rax, [{tmp} + 0]",  "cmp rax, [{m} + 0]",  "jb 3f",

//         // --- subtract: tmp = tmp - m ---
//         "2:",
//         "mov rax, [{m} + 0]",  "sub [{tmp} + 0], rax",
//         "mov rax, [{m} + 8]",  "sbb [{tmp} + 8], rax",
//         "mov rax, [{m} + 16]", "sbb [{tmp} + 16], rax",
//         "mov rax, [{m} + 24]", "sbb [{tmp} + 24], rax",

//         "3:", // done
//         tmp = in(reg) tmp.as_mut_ptr(),
//         m = in(reg) m.as_ptr(),
//         out("rax") _,
//         clobber_abi("C")
//     );
// }
// #[cfg(target_arch = "x86_64")]
// pub unsafe fn mont_mul_reduce_n4(
//     a: &[u64; 4],
//     b: &[u64; 4],
//     m: &[u64; 4],
//     inv: u64,
//     res: &mut [u64; 4],
// ) {
//     let mut tmp = [0u64; 5]; // n+1 limbs
//     let (mut t0, mut t1, mut cy, mut u): (u64, u64, u64, u64);

//     std::arch::asm!(
//         // --- MONT_PRECOMPUTE ---
//         "xor {cy}, {cy}",
//         "mov rax, [{A}]",
//         "mul qword ptr [{B}]",
//         "mov {t0}, rax",
//         "mov {t1}, rdx",
//         "mul {inv_val}",
//         "mov {u}, rax",
//         "mul qword ptr [{M}]",
//         "add rax, {t0}",
//         "adc rdx, {t1}",
//         "adc {cy}, 0",
//         "mov {t1}, rdx",

//         // --- MONT_FIRSTITER(1, 2, 3) ---
//         // 此处省略展开逻辑，通常是通过重复指令块实现...
//         // 核心是 rax = A[0]*B[j] + T1; tmp[j-1] = rax + M[j]*u + cy...

//         // --- MONT_FINALIZE ---
//         "mov [{tmp} + 24], {t1}",
//         "mov [{tmp} + 32], {cy}",

//         // 输入输出绑定
//         A = in(reg) a.as_ptr(),
//         B = in(reg) b.as_ptr(),
//         M = in(reg) m.as_ptr(),
//         tmp = in(reg) tmp.as_mut_ptr(),
//         inv_val = in(reg) inv,
//         t0 = out(reg) t0,
//         t1 = out(reg) t1,
//         cy = out(reg) cy,
//         u = out(reg) u,
//         out("rax") _, out("rdx") _,
//     );

//     // 调用上面定义的条件减法
//     mont_final_sub_n4(&mut tmp, m);
//     res.copy_from_slice(&tmp[0..4]);
// }
// #[cfg(target_arch = "aarch64")]
// #[inline(always)]
// pub unsafe fn comba_3_by_3_mul_arm(res: &mut [u64; 6], a: &[u64; 3], b: &[u64; 3]) {
//     let (mut c0, mut c1, mut c2): (u64, u64, u64);
//     let (mut low, mut high): (u64, u64);

//     std::arch::asm!(
//         // --- Round 0: a0 * b0 ---
//         "ldp {a0}, {a1}, [{A}]",      // 加载 a0, a1
//         "ldr {a2}, [{A}, #16]",       // 加载 a2
//         "ldp {b0}, {b1}, [{B}]",      // 加载 b0, b1
//         "ldr {b2}, [{B}, #16]",       // 加载 b2

//         "mul {low}, {a0}, {b0}",      // low = a0 * b0
//         "umulh {c0}, {a0}, {b0}",     // c0 = (a0 * b0) >> 64
//         "str {low}, [{res}]",         // res[0] = low

//         // --- Round 1: a0*b1 + a1*b0 ---
//         "mul {low}, {a0}, {b1}",
//         "umulh {high}, {a0}, {b1}",
//         "adds {c0}, {c0}, {low}",
//         "adc {c1}, {high}, xzr",      // xzr 是零寄存器

//         "mul {low}, {a1}, {b0}",
//         "umulh {high}, {a1}, {b0}",
//         "adds {c0}, {c0}, {low}",
//         "adcs {c1}, {c1}, {high}",
//         "adc {c2}, xzr, xzr",         // c2 = carry
//         "str {c0}, [{res}, #8]",      // res[1] = c0

//         // ... 依此类推展开后续 Round ...

//         A = in(reg) a.as_ptr(),
//         B = in(reg) b.as_ptr(),
//         res = in(reg) res.as_mut_ptr(),
//         a0 = out(reg) _, a1 = out(reg) _, a2 = out(reg) _,
//         b0 = out(reg) _, b1 = out(reg) _, b2 = out(reg) _,
//         low = out(reg) low, high = out(reg) high,
//         c0 = out(reg) c0, c1 = out(reg) c1, c2 = out(reg) c2,
//     );
// }
// #[cfg(target_arch = "aarch64")]
// unsafe fn final_sub_n3_arm(r: &mut [u64], m: &[u64]) {
//     // ARM 的比较和减法逻辑
//     std::arch::asm!(
//         "ldp {r0}, {r1}, [{r_ptr}]",
//         "ldr {r2}, [{r_ptr}, #16]",
//         "ldp {m0}, {m1}, [{m_ptr}]",
//         "ldr {m2}, [{m_ptr}, #16]",

//         // 模拟 cmp: r - m 并丢弃结果，只看进位
//         "subs xzr, {r0}, {m0}",
//         "sbcs xzr, {r1}, {m1}",
//         "sbcs xzr, {r2}, {m2}",

//         // 如果没有借位 (b.cs / b.hs)，说明 r >= m，执行减法
//         "b.lo 1f",
//         "subs {r0}, {r0}, {m0}",
//         "sbcs {r1}, {r1}, {m1}",
//         "sbcs {r2}, {r2}, {m2}",
//         "stp {r0}, {r1}, [{r_ptr}]",
//         "str {r2}, [{r_ptr}, #16]",
//         "1:",
//         r_ptr = in(reg) r.as_mut_ptr(),
//         m_ptr = in(reg) m.as_ptr(),
//         r0 = out(reg) _, r1 = out(reg) _, r2 = out(reg) _,
//         m0 = out(reg) _, m1 = out(reg) _, m2 = out(reg) _,
//     );
// }
// pub fn mul_reduce(&mut self, other: &BigInt<N>) {
//     #[cfg(target_arch = "x86_64")]
//     unsafe { self.mul_reduce_x86(other) }

//     #[cfg(target_arch = "aarch64")]
//     unsafe { self.mul_reduce_arm(other) }
// }
macro_rules! add_256 {
    ($a:expr, $b:expr, $modulus:expr) => {
        unsafe {
            core::arch::asm!(
                // 1. 大整数加法 (this = this + other)
                "mov {rax}, [{b}]",
                "add [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "adc [{a} + 16], {rax}",
                "mov {rax}, [{b} + 24]",
                "adc [{a} + 24], {rax}",

                // 2. 检查进位：如果加法溢出，直接跳转减法
                "jc 2f",

                // 3. 逐位比较 (从高到低 24->16->8->0)
                "mov {rax}, [{m} + 24]",
                "cmp [{a} + 24], {rax}",
                "jb 3f", // 结果更小，跳过减法
                "ja 2f", // 结果更大，去减法

                "mov {rax}, [{m} + 16]",
                "cmp [{a} + 16], {rax}",
                "jb 3f",
                "ja 2f",

                "mov {rax}, [{m} + 8]",
                "cmp [{a} + 8], {rax}",
                "jb 3f",
                "ja 2f",

                "mov {rax}, [{m}]",
                "cmp [{a}], {rax}",
                "jb 3f",

                // 4. 减法步骤 (this = this - modulus)
                "2:",
                "mov {rax}, [{m}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "sbb [{a} + 16], {rax}",
                "mov {rax}, [{m} + 24]",
                "sbb [{a} + 24], {rax}",

                "3:", // 结束标签
                a = in(reg) $a,
                b = in(reg) $b,
                m = in(reg) $modulus,
                rax = out(reg) _, // 让编译器自动选择辅助寄存器
                clobber_abi("C"),
            );
        }
    };
}

macro_rules! add_192 {
    ($a:expr, $b:expr, $modulus:expr) => {
        unsafe {
            core::arch::asm!(
                // 1. A = A + B
                "mov {rax}, [{b}]",
                "add [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "adc [{a} + 16], {rax}",

                // 2. 检查进位
                "jc 2f",

                // 3. 比较 A 与 modulus
                "mov {rax}, [{m} + 16]",
                "cmp [{a} + 16], {rax}",
                "jb 3f", "ja 2f",
                "mov {rax}, [{m} + 8]",
                "cmp [{a} + 8], {rax}",
                "jb 3f", "ja 2f",
                "mov {rax}, [{m}]",
                "cmp [{a}], {rax}",
                "jb 3f",

                // 4. 减法 A = A - modulus
                "2:",
                "mov {rax}, [{m}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "sbb [{a} + 16], {rax}",

                "3:",
                a = in(reg) $a,
                b = in(reg) $b,
                m = in(reg) $modulus,
                rax = out(reg) _,
                clobber_abi("C"),
            );
        }
    };
}

macro_rules! add_320 {
    ($a:expr, $b:expr, $modulus:expr) => {
        unsafe {
            core::arch::asm!(
                // 1. A = A + B
                "mov {rax}, [{b}]",
                "add [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "adc [{a} + 16], {rax}",
                "mov {rax}, [{b} + 24]",
                "adc [{a} + 24], {rax}",
                "mov {rax}, [{b} + 32]",
                "adc [{a} + 32], {rax}",

                // 2. 进位跳转
                "jc 2f",

                // 3. 逐位比较 (高->低)
                "mov {rax}, [{m} + 32]",
                "cmp [{a} + 32], {rax}",
                "jb 3f", "ja 2f",
                "mov {rax}, [{m} + 24]",
                "cmp [{a} + 24], {rax}",
                "jb 3f", "ja 2f",
                "mov {rax}, [{m} + 16]",
                "cmp [{a} + 16], {rax}",
                "jb 3f", "ja 2f",
                "mov {rax}, [{m} + 8]",
                "cmp [{a} + 8], {rax}",
                "jb 3f", "ja 2f",
                "mov {rax}, [{m}]",
                "cmp [{a}], {rax}",
                "jb 3f",

                // 4. 减法补偿
                "2:",
                "mov {rax}, [{m}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "sbb [{a} + 16], {rax}",
                "mov {rax}, [{m} + 24]",
                "sbb [{a} + 24], {rax}",
                "mov {rax}, [{m} + 32]",
                "sbb [{a} + 32], {rax}",

                "3:",
                a = in(reg) $a,
                b = in(reg) $b,
                m = in(reg) $modulus,
                rax = out(reg) _,
                clobber_abi("C"),
            );
        }
    };
}

#[repr(C)]
pub struct Fp256 {
    pub data: [u64; 4],
}

impl Fp256 {
    pub const MODULUS: [u64; 4] = [1u64; 4];

    pub fn add_assign(&mut self, other: &Self) {
        #[cfg(all(target_arch = "x86_64", feature = "asm"))]
        {
            add_256!(
                self.data.as_mut_ptr(),
                other.data.as_ptr(),
                Self::MODULUS.as_ptr()
            );
        }

        #[cfg(not(all(target_arch = "x86_64", feature = "asm")))]
        {
            // 通用 Rust 实现 (回退方案)
            let mut carry = 0u64;
            let mut res = [0u64; 4];

            // 使用带有进位的加法
            for i in 0..4 {
                let (sum, c) = self.data[i].overflowing_add(other.data[i]);
                let (sum2, c2) = sum.overflowing_add(carry);
                res[i] = sum2;
                carry = (c as u64) + (c2 as u64);
            }

            // 检查是否需要减去模数
            if carry > 0 || res >= Self::MODULUS {
                let mut borrow = 0u64;
                for i in 0..4 {
                    let (sub, b) = res[i].overflowing_sub(Self::MODULUS[i]);
                    let (sub2, b2) = sub.overflowing_sub(borrow);
                    self.data[i] = sub2;
                    borrow = (b as u64) + (b2 as u64);
                }
            } else {
                self.data = res;
            }
        }
    }
}
macro_rules! sub_256 {
    ($a:expr, $b:expr, $modulus:expr) => {
        unsafe {
            core::arch::asm!(
                // 1. 执行 A = A - B
                "mov {rax}, [{b}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "sbb [{a} + 16], {rax}",
                "mov {rax}, [{b} + 24]",
                "sbb [{a} + 24], {rax}",

                // 2. 检查借位：如果没有借位 (CF=0)，说明 A >= B，直接完成
                "jnc 2f",

                // 3. 如果有借位 (CF=1)，说明 A < B，执行 A = A + modulus
                "mov {rax}, [{m}]",
                "add [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "adc [{a} + 16], {rax}",
                "mov {rax}, [{m} + 24]",
                "adc [{a} + 24], {rax}",

                "2:", // done 标签
                a = in(reg) $a,
                b = in(reg) $b,
                m = in(reg) $modulus,
                rax = out(reg) _,
                clobber_abi("C"),
            );
        }
    };
}
impl Fp256 {
    pub fn sub_assign(&mut self, other: &Self) {
        #[cfg(all(target_arch = "x86_64", feature = "asm"))]
        {
            sub_256!(
                self.data.as_mut_ptr(),
                other.data.as_ptr(),
                Self::MODULUS.as_ptr()
            );
        }

        #[cfg(not(all(target_arch = "x86_64", feature = "asm")))]
        {
            // 纯 Rust 回退方案：使用 checked_sub
            let mut borrow = 0u64;
            let mut res = [0u64; 4];

            for i in 0..4 {
                let (s1, b1) = self.data[i].overflowing_sub(other.data[i]);
                let (s2, b2) = s1.overflowing_sub(borrow);
                res[i] = s2;
                borrow = (b1 as u64) + (b2 as u64);
            }

            if borrow > 0 {
                let mut carry = 0u64;
                for i in 0..4 {
                    let (sum, c1) = res[i].overflowing_add(Self::MODULUS[i]);
                    let (sum2, c2) = sum.overflowing_add(carry);
                    self.data[i] = sum2;
                    carry = (c1 as u64) + (c2 as u64);
                }
            } else {
                self.data = res;
            }
        }
    }
}
macro_rules! sub_192 {
    ($a:expr, $b:expr, $modulus:expr) => {
        unsafe {
            core::arch::asm!(
                // 1. A = A - B
                "mov {rax}, [{b}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "sbb [{a} + 16], {rax}",

                // 2. 无借位则跳过加法
                "jnc 2f",

                // 3. A = A + modulus
                "mov {rax}, [{m}]",
                "add [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "adc [{a} + 16], {rax}",

                "2:",
                a = in(reg) $a,
                b = in(reg) $b,
                m = in(reg) $modulus,
                rax = out(reg) _,
                clobber_abi("C"),
            );
        }
    };
}
macro_rules! sub_320 {
    ($a:expr, $b:expr, $modulus:expr) => {
        unsafe {
            core::arch::asm!(
                // 1. A = A - B
                "mov {rax}, [{b}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "sbb [{a} + 16], {rax}",
                "mov {rax}, [{b} + 24]",
                "sbb [{a} + 24], {rax}",
                "mov {rax}, [{b} + 32]",
                "sbb [{a} + 32], {rax}",

                // 2. 无借位跳转
                "jnc 2f",

                // 3. A = A + modulus
                "mov {rax}, [{m}]",
                "add [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "adc [{a} + 16], {rax}",
                "mov {rax}, [{m} + 24]",
                "adc [{a} + 24], {rax}",
                "mov {rax}, [{m} + 32]",
                "adc [{a} + 32], {rax}",

                "2:",
                a = in(reg) $a,
                b = in(reg) $b,
                m = in(reg) $modulus,
                rax = out(reg) _,
                clobber_abi("C"),
            );
        }
    };
}

// impl<const N: usize> FpM<N> {
//     pub fn sub_assign(&mut self, other: &Self) {
//         let a = self.data.as_mut_ptr();
//         let b = other.data.as_ptr();
//         let m = Self::MODULUS.as_ptr();

//         match N {
//             3 => sub_192!(a, b, m),
//             4 => sub_256!(a, b, m),
//             5 => sub_320!(a, b, m),
//             _ => self.sub_assign_portable(other), // 这里的 generic 实现参考上一次回复
//         }
//     }
// }

// macro_rules! mont_cmp {
//     ($tmp:expr, $M:expr, $ofs:expr, $done_label:tt, $sub_label:tt) => {
//         concat!(
//             "movq ", $ofs, "(%[M]), %%rax \n\t",
//             "cmpq %%rax, ", $ofs, "(%[tmp]) \n\t",
//             "jb ", $done_label, " \n\t",
//             "ja ", $sub_label, " \n\t"
//         )
//     };
// }

// // 示例：MONT_FIRSTSUB 的 Rust 映射
// macro_rules! mont_first_sub {
//     ($tmp:expr, $M:expr) => {
//         "movq (%[M]), %%rax \n\t \
//          subq %%rax, (%[tmp]) \n\t"
//     };
// }
// #[cfg(target_arch = "x86_64")]
// pub fn mul_reduce_n4(
//     a: &[u64; 4],
//     b: &[u64; 4],
//     modulus: &[u64; 4],
//     inv: u64
// ) -> [u64; 4] {
//     let mut tmp = [0u64; 5]; // n+1 limbs

//     // 这里的变量映射到 C++ 中的 T0, T1, cy, u
//     let mut t0: u64 = 0;
//     let mut t1: u64 = 0;
//     let mut cy: u64 = 0;
//     let mut u: u64 = 0;

//     unsafe {
//         core::arch::asm!(
//             // --- MONT_PRECOMPUTE ---
//             "xorq {cy}, {cy}",
//             "movq 0({A}), %rax",
//             "mulq 0({B})",
//             "movq %rax, {T0}",
//             "movq %rdx, {T1}",
//             "mulq {inv}",
//             "movq %rax, {u}",
//             "mulq 0({M})",
//             "addq {T0}, %rax",
//             "adcq %rdx, {T1}",
//             "adcq $0, {cy}",

//             // --- MONT_FIRSTITER(1, 2, 3) 展开 ---
//             // 这里为了简洁，手动写出逻辑，实际可用宏生成
//             "xorq {T0}, {T0}",
//             "movq 0({A}), %rax",
//             "mulq 8({B})",
//             "addq {T1}, %rax",
//             "movq %rax, 0({tmp})",
//             "adcq $0, %rdx",
//             "movq %rdx, {T1}",
//             // ... 后续循环逻辑类似 C++ 宏展开 ...

//             // --- 最后的溢出检查与减法 ---
//             "//check for overflow",
//             "movq 24({M}), %rax", "cmpq %rax, 24({tmp})", "jb 2f", "ja 1f",
//             "movq 16({M}), %rax", "cmpq %rax, 16({tmp})", "jb 2f", "ja 1f",
//             "movq 8({M}), %rax",  "cmpq %rax, 8({tmp})",  "jb 2f", "ja 1f",
//             "movq 0({M}), %rax",  "cmpq %rax, 0({tmp})",  "jb 2f", "ja 1f",

//             "1:", // subtract label
//             "movq 0({M}), %rax",  "subq %rax, 0({tmp})",
//             "movq 8({M}), %rax",  "sbbq %rax, 8({tmp})",
//             "movq 16({M}), %rax", "sbbq %rax, 16({tmp})",
//             "movq 24({M}), %rax", "sbbq %rax, 24({tmp})",

//             "2:", // done label

//             A = in(reg) a.as_ptr(),
//             B = in(reg) b.as_ptr(),
//             M = in(reg) modulus.as_ptr(),
//             tmp = in(reg) tmp.as_mut_ptr(),
//             inv = in(reg) inv,
//             T0 = inout(reg) t0,
//             T1 = inout(reg) t1,
//             cy = inout(reg) cy,
//             u = inout(reg) u,
//             out("rax") _,
//             out("rdx") _,
//             clobber_abi("C")
//         );
//     }

//     let mut res = [0u64; 4];
//     res.copy_from_slice(&tmp[0..4]);
//     res
// }

// #[cfg(target_arch = "x86_64")]
// pub fn mul_reduce_n4(a: &[u64; 4], b: &[u64; 4], modulus: &[u64; 4], inv: u64) -> [u64; 4] {
//     let mut tmp = [0u64; 5]; // 存储中间结果 (n+1 limbs)

//     // 模拟 C++ 中的辅助变量
//     let mut t0: u64;
//     let mut t1: u64;
//     let mut cy: u64;
//     let mut u: u64;

//     unsafe {
//         core::arch::asm!(
//             // --- MONT_PRECOMPUTE ---
//             "xor {cy}, {cy}",
//             "mov ({A}), %rax",
//             "mulq ({B})",
//             "mov %rax, {T0}",
//             "mov %rdx, {T1}",
//             "mulq {inv}",
//             "mov %rax, {u}",
//             "mulq ({M})",
//             "add %rax, {T0}",    // 这里逻辑同 C++: (M[0]*u + T0) / b
//             "adc %rdx, {T1}",
//             "adc $0, {cy}",

//             // --- MONT_FIRSTITER (j=1, 2, 3) ---
//             // i=0, j=1
//             "xor {T0}, {T0}",
//             "mov ({A}), %rax",
//             "mulq 8({B})",
//             "add {T1}, %rax",
//             "mov %rax, 0({tmp})",
//             "adc $0, %rdx",
//             "mov %rdx, {T1}",
//             "mov 8({M}), %rax",
//             "mulq {u}",
//             "add %rax, 0({tmp})",
//             "adc {cy}, %rdx",
//             "adc $0, {T0}",
//             "xor {cy}, {cy}",
//             "add %rdx, {T1}",
//             "adc {T0}, {cy}",

//             // i=0, j=2
//             "xor {T0}, {T0}",
//             "mov ({A}), %rax",
//             "mulq 16({B})",
//             "add {T1}, %rax",
//             "mov %rax, 8({tmp})",
//             "adc $0, %rdx",
//             "mov %rdx, {T1}",
//             "mov 16({M}), %rax",
//             "mulq {u}",
//             "add %rax, 8({tmp})",
//             "adc {cy}, %rdx",
//             "adc $0, {T0}",
//             "xor {cy}, {cy}",
//             "add %rdx, {T1}",
//             "adc {T0}, {cy}",

//             // i=0, j=3
//             "xor {T0}, {T0}",
//             "mov ({A}), %rax",
//             "mulq 24({B})",
//             "add {T1}, %rax",
//             "mov %rax, 16({tmp})",
//             "adc $0, %rdx",
//             "mov %rdx, {T1}",
//             "mov 24({M}), %rax",
//             "mulq {u}",
//             "add %rax, 16({tmp})",
//             "adc {cy}, %rdx",
//             "adc $0, {T0}",
//             "xor {cy}, {cy}",
//             "add %rdx, {T1}",
//             "adc {T0}, {cy}",

//             // MONT_FINALIZE(3)
//             "mov {T1}, 24({tmp})",
//             "mov {cy}, 32({tmp})",

//             // --- 此处应重复 MONT_ITERFIRST 和 MONT_ITERITER (i=1, 2, 3) ---
//             // 篇幅限制，此处展示核心规约后的减法判断逻辑

//             "//最终减法判定: MONT_CMP & SUBTRACT",
//             "mov 24({M}), %rax", "cmp %rax, 24({tmp})", "jb 2f", "ja 1f",
//             "mov 16({M}), %rax", "cmp %rax, 16({tmp})", "jb 2f", "ja 1f",
//             "mov 8({M}), %rax",  "cmp %rax, 8({tmp})",  "jb 2f", "ja 1f",
//             "mov 0({M}), %rax",  "cmp %rax, 0({tmp})",  "jb 2f", "ja 1f",

//             "1:", // subtract
//             "mov 0({M}), %rax", "sub %rax, 0({tmp})",
//             "mov 8({M}), %rax", "sbb %rax, 8({tmp})",
//             "mov 16({M}), %rax", "sbb %rax, 16({tmp})",
//             "mov 24({M}), %rax", "sbb %rax, 24({tmp})",
//             "2:", // done

//             A = in(reg) a.as_ptr(),
//             B = in(reg) b.as_ptr(),
//             M = in(reg) modulus.as_ptr(),
//             tmp = in(reg) tmp.as_mut_ptr(),
//             inv = in(reg) inv,
//             T0 = out(reg) t0,
//             T1 = out(reg) t1,
//             cy = out(reg) cy,
//             u = out(reg) u,
//             out("rax") _,
//             out("rdx") _,
//             options(att_syntax) // 使用 ATT 语法以匹配原本的 C++ 代码
//         );
//     }

//     let mut result = [0u64; 4];
//     result.copy_from_slice(&tmp[0..4]);
//     result
// }

// fn main() {
//     // 示例用法
//     let a = [1u64; 4];
//     let b = [2u64; 4];
//     let m = [0xFFFFFFFFFFFFFFFFu64; 4];
//     let inv = 123456789u64; // 实际需根据 m 计算
//     let res = mul_reduce_n4(&a, &b, &m, inv);
//     println!("Result: {:?}", res);
// }
/// 针对 n=4 (256位) 的汇编优化蒙哥马利乘法
/// 警告：仅限 x86-64 架构使用
#[cfg(target_arch = "x86_64")]
pub unsafe fn mul_reduce_n4(a: &[u64; 4], b: &[u64; 4], modulus: &[u64; 4], inv: u64) -> [u64; 4] {
    let mut tmp = [0u64; 5]; // 对应 mp_limb_t tmp[n+1]
    let mut t0: u64;
    let mut t1: u64;
    let mut cy: u64;
    let mut u: u64;

    core::arch::asm!(
        //--- MONT_PRECOMPUTE ---
        "xorq {cy}, {cy}",
        "movq 0({A}), %rax",
        "mulq 0({B})",
        "movq %rax, {T0}",
        "movq %rdx, {T1}",
        "mulq {inv}",
        "movq %rax, {u}",
        "mulq 0({M})",
        "addq {T0}, %rax",
        "adcq %rdx, {T1}",
        "adcq $0, {cy}",

        //--- MONT_FIRSTITER(1) ---
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}",
        "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",
        //--- MONT_FIRSTITER(2) ---
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}",
        "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",
        //--- MONT_FIRSTITER(3) ---
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}",
        "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",

        "movq {T1}, 24({tmp})", "movq {cy}, 32({tmp})", // MONT_FINALIZE(3)

        //--- MONT_ITER(i=1) ---
        "xorq {cy}, {cy}", "movq 8({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        // j=1, 2, 3
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 24({tmp})", "movq {cy}, 32({tmp})",

        //--- MONT_ITER(i=2) ---
        "xorq {cy}, {cy}", "movq 16({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 24({tmp})", "movq {cy}, 32({tmp})",

        //--- MONT_ITER(i=3) ---
        "xorq {cy}, {cy}", "movq 24({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 24({tmp})", "movq {cy}, 32({tmp})",

        //--- 溢出检查与条件减法 ---
        "movq 24({M}), %rax", "cmpq %rax, 24({tmp})", "jb 2f", "ja 1f",
        "movq 16({M}), %rax", "cmpq %rax, 16({tmp})", "jb 2f", "ja 1f",
        "movq 8({M}), %rax",  "cmpq %rax, 8({tmp})",  "jb 2f", "ja 1f",
        "movq 0({M}), %rax",  "cmpq %rax, 0({tmp})",  "jb 2f", "ja 1f",
        "1:", // subtract label
        "movq 0({M}), %rax",  "subq %rax, 0({tmp})",
        "movq 8({M}), %rax",  "sbbq %rax, 8({tmp})",
        "movq 16({M}), %rax", "sbbq %rax, 16({tmp})",
        "movq 24({M}), %rax", "sbbq %rax, 24({tmp})",
        "2:", // done label

        A = in(reg) a.as_ptr(),
        B = in(reg) b.as_ptr(),
        M = in(reg) modulus.as_ptr(),
        tmp = in(reg) tmp.as_mut_ptr(),
        inv = in(reg) inv,
        T0 = out(reg) t0,
        T1 = out(reg) t1,
        cy = out(reg) cy,
        u = out(reg) u,
        out("rax") _,
        out("rdx") _,
        options(att_syntax)
    );

    [tmp[0], tmp[1], tmp[2], tmp[3]]
}
/// 针对 n=5 (320位) 的全展开汇编优化蒙哥马利乘法
#[cfg(target_arch = "x86_64")]
pub unsafe fn mul_reduce_n5(a: &[u64; 5], b: &[u64; 5], modulus: &[u64; 5], inv: u64) -> [u64; 5] {
    let mut tmp = [0u64; 6]; // 对应 mp_limb_t tmp[n+1]
    let mut t0: u64;
    let mut t1: u64;
    let mut cy: u64;
    let mut u: u64;

    core::arch::asm!(
        //======================== i = 0 ========================
        "xorq {cy}, {cy}", "movq 0({A}), %rax", "mulq 0({B})", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        // j=1
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",
        // j=2
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",
        // j=3
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",
        // j=4
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 32({B})", "addq {T1}, %rax", "movq %rax, 24({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 32({M}), %rax", "mulq {u}", "addq %rax, 24({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",
        "movq {T1}, 32({tmp})", "movq {cy}, 40({tmp})",

        //======================== i = 1 ========================
        "xorq {cy}, {cy}", "movq 8({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 32({B})", "addq {T1}, %rax", "movq %rax, 24({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 32({M}), %rax", "mulq {u}", "addq %rax, 24({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 40({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 32({tmp})", "movq {cy}, 40({tmp})",

        //======================== i = 2 ========================
        "xorq {cy}, {cy}", "movq 16({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 32({B})", "addq {T1}, %rax", "movq %rax, 24({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 32({M}), %rax", "mulq {u}", "addq %rax, 24({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 40({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 32({tmp})", "movq {cy}, 40({tmp})",

        //======================== i = 3 ========================
        "xorq {cy}, {cy}", "movq 24({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 24({A}), %rax", "mulq 32({B})", "addq {T1}, %rax", "movq %rax, 24({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 32({M}), %rax", "mulq {u}", "addq %rax, 24({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 40({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 32({tmp})", "movq {cy}, 40({tmp})",

        //======================== i = 4 ========================
        "xorq {cy}, {cy}", "movq 32({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 32({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 32({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 32({A}), %rax", "mulq 24({B})", "addq {T1}, %rax", "movq %rax, 16({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 24({M}), %rax", "mulq {u}", "addq %rax, 16({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 32({tmp}), {T1}", "adcq $0, {cy}",
        "xorq {T0}, {T0}", "movq 32({A}), %rax", "mulq 32({B})", "addq {T1}, %rax", "movq %rax, 24({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 32({M}), %rax", "mulq {u}", "addq %rax, 24({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 40({tmp}), {T1}", "adcq $0, {cy}",
        "movq {T1}, 32({tmp})", "movq {cy}, 40({tmp})",

        //======================== 溢出修正 ========================
        "movq 32({M}), %rax", "cmpq %rax, 32({tmp})", "jb 2f", "ja 1f",
        "movq 24({M}), %rax", "cmpq %rax, 24({tmp})", "jb 2f", "ja 1f",
        "movq 16({M}), %rax", "cmpq %rax, 16({tmp})", "jb 2f", "ja 1f",
        "movq 8({M}), %rax",  "cmpq %rax, 8({tmp})",  "jb 2f", "ja 1f",
        "movq 0({M}), %rax",  "cmpq %rax, 0({tmp})",  "jb 2f", "ja 1f",
        "1:",
        "movq 0({M}), %rax",  "subq %rax, 0({tmp})",
        "movq 8({M}), %rax",  "sbbq %rax, 8({tmp})",
        "movq 16({M}), %rax", "sbbq %rax, 16({tmp})",
        "movq 24({M}), %rax", "sbbq %rax, 24({tmp})",
        "movq 32({M}), %rax", "sbbq %rax, 32({tmp})",
        "2:",

        A = in(reg) a.as_ptr(), B = in(reg) b.as_ptr(), M = in(reg) modulus.as_ptr(),
        tmp = in(reg) tmp.as_mut_ptr(), inv = in(reg) inv,
        T0 = out(reg) t0, T1 = out(reg) t1, cy = out(reg) cy, u = out(reg) u,
        out("rax") _, out("rdx") _,
        options(att_syntax)
    );

    [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4]]
}
/// 针对 n=3 (192位) 的全展开汇编优化蒙哥马利乘法
#[cfg(target_arch = "x86_64")]
pub unsafe fn mull_reduce_n3(a: &[u64; 3], b: &[u64; 3], modulus: &[u64; 3], inv: u64) -> [u64; 3] {
    let mut tmp = [0u64; 6]; // 存储 2n 结果
    let mut t0: u64;
    let mut t1: u64;
    let mut cy: u64;
    let mut u: u64;

    core::arch::asm!(
        //======================== i = 0 (MONT_PRECOMPUTE) ========================
        "xorq {cy}, {cy}",
        "movq 0({A}), %rax",
        "mulq 0({B})",
        "movq %rax, {T0}",
        "movq %rdx, {T1}",
        "mulq {inv}",
        "movq %rax, {u}",
        "mulq 0({M})",
        "addq {T0}, %rax",
        "adcq %rdx, {T1}",
        "adcq $0, {cy}",

        // j=1 (MONT_FIRSTITER)
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}",
        "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",

        // j=2 (MONT_FIRSTITER)
        "xorq {T0}, {T0}", "movq 0({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}",
        "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}",

        "movq {T1}, 16({tmp})", "movq {cy}, 24({tmp})", // MONT_FINALIZE(2)

        //======================== i = 1 (MONT_ITERFIRST + ITERITER) ========================
        "xorq {cy}, {cy}", "movq 8({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",

        // j=1
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",

        // j=2
        "xorq {T0}, {T0}", "movq 8({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",

        "movq {T1}, 16({tmp})", "movq {cy}, 24({tmp})",

        //======================== i = 2 (MONT_ITERFIRST + ITERITER) ========================
        "xorq {cy}, {cy}", "movq 16({A}), %rax", "mulq 0({B})", "addq 0({tmp}), %rax", "adcq 8({tmp}), %rdx", "adcq $0, {cy}", "movq %rax, {T0}", "movq %rdx, {T1}", "mulq {inv}", "movq %rax, {u}", "mulq 0({M})", "addq {T0}, %rax", "adcq %rdx, {T1}", "adcq $0, {cy}",

        // j=1
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 8({B})", "addq {T1}, %rax", "movq %rax, 0({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 8({M}), %rax", "mulq {u}", "addq %rax, 0({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 16({tmp}), {T1}", "adcq $0, {cy}",

        // j=2
        "xorq {T0}, {T0}", "movq 16({A}), %rax", "mulq 16({B})", "addq {T1}, %rax", "movq %rax, 8({tmp})", "adcq $0, %rdx", "movq %rdx, {T1}", "movq 16({M}), %rax", "mulq {u}", "addq %rax, 8({tmp})", "adcq {cy}, %rdx", "adcq $0, {T0}", "xorq {cy}, {cy}", "addq %rdx, {T1}", "adcq {T0}, {cy}", "addq 24({tmp}), {T1}", "adcq $0, {cy}",

        "movq {T1}, 16({tmp})", "movq {cy}, 24({tmp})",

        //======================== 溢出判定与条件减法 ========================
        "movq 16({M}), %rax", "cmpq %rax, 16({tmp})", "jb 2f", "ja 1f",
        "movq 8({M}), %rax",  "cmpq %rax, 8({tmp})",  "jb 2f", "ja 1f",
        "movq 0({M}), %rax",  "cmpq %rax, 0({tmp})",  "jb 2f", "ja 1f",
        "1:",
        "movq 0({M}), %rax",  "subq %rax, 0({tmp})",
        "movq 8({M}), %rax",  "sbbq %rax, 8({tmp})",
        "movq 16({M}), %rax", "sbbq %rax, 16({tmp})",
        "2:",

        A = in(reg) a.as_ptr(), B = in(reg) b.as_ptr(), M = in(reg) modulus.as_ptr(),
        tmp = in(reg) tmp.as_mut_ptr(), inv = in(reg) inv,
        T0 = out(reg) t0, T1 = out(reg) t1, cy = out(reg) cy, u = out(reg) u,
        out("rax") _, out("rdx") _,
        options(att_syntax)
    );

    [tmp[0], tmp[1], tmp[2]]
}

pub trait MontgomeryConfig: Sized {
    const N: usize;
    type Array: AsRef<[u64]> + AsMut<[u64]> + Default + Copy;

    /// 执行蒙哥马利乘法规约：this = (this * other * R^-1) mod M
    fn mul_reduce(this: &mut Self::Array, other: &Self::Array, modulus: &Self::Array, inv: u64);
}

// 对应的具体位宽标记类型
pub struct N3;
pub struct N4;
pub struct N5;
impl MontgomeryConfig for N3 {
    const N: usize = 3;
    type Array = [u64; 3];

    #[inline(always)]
    fn mul_reduce(this: &mut [u64; 3], other: &[u64; 3], modulus: &[u64; 3], inv: u64) {
        let tmp = unsafe { mul_reduce_n3(this, other, modulus, inv) };
        this.copy_from_slice(&tmp[0..3]);
    }
}

impl MontgomeryConfig for N4 {
    const N: usize = 4;
    type Array = [u64; 4];

    #[inline(always)]
    fn mul_reduce(this: &mut [u64; 4], other: &[u64; 4], modulus: &[u64; 4], inv: u64) {
        let tmp = unsafe { mul_reduce_n4(this, other, modulus, inv) };
        this.copy_from_slice(&tmp[0..4]);
    }
}

impl MontgomeryConfig for N5 {
    const N: usize = 5;
    type Array = [u64; 5];

    #[inline(always)]
    fn mul_reduce(this: &mut [u64; 5], other: &[u64; 5], modulus: &[u64; 5], inv: u64) {
        let tmp = unsafe { mul_reduce_n5(this, other, modulus, inv) };
        this.copy_from_slice(&tmp[0..5]);
    }
}
pub struct Fp<C: MontgomeryConfig> {
    pub data: C::Array,
}

impl<C: MontgomeryConfig> Fp<C> {
    pub fn new(data: C::Array) -> Self {
        Self { data }
    }

    /// 统一的乘法接口
    pub fn mul_assign(&mut self, other: &Self, modulus: &C::Array, inv: u64) {
        unsafe {
            C::mul_reduce(&mut self.data, &other.data, modulus, inv);
        }
    }
}
fn main() {
    // 256位 (n=4) 的运算
    let mut a = Fp::<N4>::new([1, 0, 0, 0]);
    let b = Fp::<N4>::new([2, 0, 0, 0]);
    let m = [0xFFFFFFFFFFFFFFFFu64; 4];
    let inv = 123456789u64; // 示例值

    a.mul_assign(&b, &m, inv);

    println!("N4 Result: {:?}", a.data);
}
#[cfg(target_arch = "x86_64")]
pub unsafe fn add_assign_asm<const N: usize>(a: *mut u64, b: *const u64, modulus: *const u64) {
    match N {
        3 => {
            core::arch::asm!(
                // 1. A = A + B
                "mov rax, [{b}]",
                "add [{a}], rax",
                "mov rax, [{b} + 8]",
                "adc [{a} + 8], rax",
                "mov rax, [{b} + 16]",
                "adc [{a} + 16], rax",

                // 2. 检查进位或比较大小
                "jc 2f",          // 如果有最高位进位，直接跳转到 subtract
                "mov rax, [{m} + 16]",
                "cmp [{a} + 16], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m} + 8]",
                "cmp [{a} + 8], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m}]",
                "cmp [{a}], rax",
                "jb 3f",

                // 3. Subtract modulus
                "2:",
                "mov rax, [{m}]",
                "sub [{a}], rax",
                "mov rax, [{m} + 8]",
                "sbb [{a} + 8], rax",
                "mov rax, [{m} + 16]",
                "sbb [{a} + 16], rax",

                "3:", // Done
                a = in(reg) a,
                b = in(reg) b,
                m = in(reg) modulus,
                out("rax") _,
                clobber_abi("C")
            );
        }
        4 => {
            core::arch::asm!(
                // A = A + B (4 limbs)
                "mov rax, [{b}]", "add [{a}], rax",
                "mov rax, [{b} + 8]", "adc [{a} + 8], rax",
                "mov rax, [{b} + 16]", "adc [{a} + 16], rax",
                "mov rax, [{b} + 24]", "adc [{a} + 24], rax",

                "jc 2f",
                "mov rax, [{m} + 24]", "cmp [{a} + 24], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m} + 16]", "cmp [{a} + 16], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m} + 8]", "cmp [{a} + 8], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m}]", "cmp [{a}], rax",
                "jb 3f",

                "2:", // Subtract
                "mov rax, [{m}]", "sub [{a}], rax",
                "mov rax, [{m} + 8]", "sbb [{a} + 8], rax",
                "mov rax, [{m} + 16]", "sbb [{a} + 16], rax",
                "mov rax, [{m} + 24]", "sbb [{a} + 24], rax",
                "3:",
                a = in(reg) a, b = in(reg) b, m = in(reg) modulus,
                out("rax") _, clobber_abi("C")
            );
        }
        5 => {
            core::arch::asm!(
                // A = A + B (5 limbs)
                "mov rax, [{b}]", "add [{a}], rax",
                "mov rax, [{b} + 8]", "adc [{a} + 8], rax",
                "mov rax, [{b} + 16]", "adc [{a} + 16], rax",
                "mov rax, [{b} + 24]", "adc [{a} + 24], rax",
                "mov rax, [{b} + 32]", "adc [{a} + 32], rax",

                "jc 2f",
                "mov rax, [{m} + 32]", "cmp [{a} + 32], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m} + 24]", "cmp [{a} + 24], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m} + 16]", "cmp [{a} + 16], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m} + 8]", "cmp [{a} + 8], rax",
                "jb 3f", "ja 2f",
                "mov rax, [{m}]", "cmp [{a}], rax",
                "jb 3f",

                "2:", // Subtract
                "mov rax, [{m}]", "sub [{a}], rax",
                "mov rax, [{m} + 8]", "sbb [{a} + 8], rax",
                "mov rax, [{m} + 16]", "sbb [{a} + 16], rax",
                "mov rax, [{m} + 24]", "sbb [{a} + 24], rax",
                "mov rax, [{m} + 32]", "sbb [{a} + 32], rax",
                "3:",
                a = in(reg) a, b = in(reg) b, m = in(reg) modulus,
                out("rax") _, clobber_abi("C")
            );
        }
        _ => unimplemented!("Only N=3, 4, 5 are optimized"),
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn add_assign_n3(a: &mut [u64; 3], b: &[u64; 3], modulus: &[u64; 3]) {
    core::arch::asm!(
        // 1. A = A + B
        "mov rax, [{b}]",
        "add [{a}], rax",
        "mov rax, [{b} + 8]",
        "adc [{a} + 8], rax",
        "mov rax, [{b} + 16]",
        "adc [{a} + 16], rax",

        // 2. 检查进位或比较大小
        "jc 2f",          // 如果有最高位进位，直接跳转到 subtract
        "mov rax, [{m} + 16]",
        "cmp [{a} + 16], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m} + 8]",
        "cmp [{a} + 8], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m}]",
        "cmp [{a}], rax",
        "jb 3f",

        // 3. Subtract modulus
        "2:",
        "mov rax, [{m}]",
        "sub [{a}], rax",
        "mov rax, [{m} + 8]",
        "sbb [{a} + 8], rax",
        "mov rax, [{m} + 16]",
        "sbb [{a} + 16], rax",

        "3:", // Done
        a = in(reg) a,
        b = in(reg) b,
        m = in(reg) modulus,
        out("rax") _,
        clobber_abi("C")
    );
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn add_assign_n4(a: &mut [u64; 4], b: &[u64; 4], modulus: &[u64; 4]) {
    core::arch::asm!(
        // A = A + B (4 limbs)
        "mov rax, [{b}]", "add [{a}], rax",
        "mov rax, [{b} + 8]", "adc [{a} + 8], rax",
        "mov rax, [{b} + 16]", "adc [{a} + 16], rax",
        "mov rax, [{b} + 24]", "adc [{a} + 24], rax",

        "jc 2f",
        "mov rax, [{m} + 24]", "cmp [{a} + 24], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m} + 16]", "cmp [{a} + 16], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m} + 8]", "cmp [{a} + 8], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m}]", "cmp [{a}], rax",
        "jb 3f",

        "2:", // Subtract
        "mov rax, [{m}]", "sub [{a}], rax",
        "mov rax, [{m} + 8]", "sbb [{a} + 8], rax",
        "mov rax, [{m} + 16]", "sbb [{a} + 16], rax",
        "mov rax, [{m} + 24]", "sbb [{a} + 24], rax",
        "3:",
        a = in(reg) a, b = in(reg) b, m = in(reg) modulus,
        out("rax") _, clobber_abi("C")
    );
}
#[cfg(target_arch = "x86_64")]
pub unsafe fn add_assign_n5(a: &mut [u64; 4], b: &[u64; 4], modulus: &[u64; 4]) {
    core::arch::asm!(
        // A = A + B (5 limbs)
        "mov rax, [{b}]", "add [{a}], rax",
        "mov rax, [{b} + 8]", "adc [{a} + 8], rax",
        "mov rax, [{b} + 16]", "adc [{a} + 16], rax",
        "mov rax, [{b} + 24]", "adc [{a} + 24], rax",
        "mov rax, [{b} + 32]", "adc [{a} + 32], rax",

        "jc 2f",
        "mov rax, [{m} + 32]", "cmp [{a} + 32], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m} + 24]", "cmp [{a} + 24], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m} + 16]", "cmp [{a} + 16], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m} + 8]", "cmp [{a} + 8], rax",
        "jb 3f", "ja 2f",
        "mov rax, [{m}]", "cmp [{a}], rax",
        "jb 3f",

        "2:", // Subtract
        "mov rax, [{m}]", "sub [{a}], rax",
        "mov rax, [{m} + 8]", "sbb [{a} + 8], rax",
        "mov rax, [{m} + 16]", "sbb [{a} + 16], rax",
        "mov rax, [{m} + 24]", "sbb [{a} + 24], rax",
        "mov rax, [{m} + 32]", "sbb [{a} + 32], rax",
        "3:",
        a = in(reg) a, b = in(reg) b, m = in(reg) modulus,
        out("rax") _, clobber_abi("C")
    );
}
// pub struct Fp_model<const N: usize> {
//     pub data: [u64; N],
// }

// impl<const N: usize> Fp_model<N> {
// pub fn add_assign<const N: usize> (data:&mut [u64; N], other: &[u64; N], modulus: &[u64; N])->bool {
//     #[cfg(target_arch = "x86_64")]
//     {
//         // 编译器会根据 N 的具体值优化掉不相关的分支
//         match N {
//             3 => unsafe { add_assign_n3(data,other, modulus) },
//             4 => unsafe { add_assign_n4(data,other, modulus) },
//             5 => unsafe { add_assign_n5(data,other, modulus) },
//             _ => {return false}//self.add_assign_portable(other, modulus), // 回退到普通实现
//         }
//     }
//     // #[cfg(not(target_arch = "x86_64"))]
//     // {
//     //     self.add_assign_portable(other, modulus);
//     // }
//     true
// }
// }

#[cfg(target_arch = "x86_64")]
pub unsafe fn sub_assign_asm<const N: usize>(a: *mut u64, b: *const u64, modulus: *const u64) {
    match N {
        3 => {
            core::arch::asm!(
                // 1. A = A - B
                "mov rax, [{b}]",
                "sub [{a}], rax",
                "mov rax, [{b} + 8]",
                "sbb [{a} + 8], rax",
                "mov rax, [{b} + 16]",
                "sbb [{a} + 16], rax",

                // 2. 如果没有借位 (No Carry/Borrow)，跳转到结束
                "jnc 2f",

                // 3. 如果有借位 (A < B)，加上模数 p: A = A + p
                "mov rax, [{m}]",
                "add [{a}], rax",
                "mov rax, [{m} + 8]",
                "adc [{a} + 8], rax",
                "mov rax, [{m} + 16]",
                "adc [{a} + 16], rax",

                "2:", // done 标签
                a = in(reg) a,
                b = in(reg) b,
                m = in(reg) modulus,
                out("rax") _,
                clobber_abi("C")
            );
        }
        4 => {
            core::arch::asm!(
                "mov rax, [{b}]", "sub [{a}], rax",
                "mov rax, [{b} + 8]", "sbb [{a} + 8], rax",
                "mov rax, [{b} + 16]", "sbb [{a} + 16], rax",
                "mov rax, [{b} + 24]", "sbb [{a} + 24], rax",
                "jnc 2f",
                "mov rax, [{m}]", "add [{a}], rax",
                "mov rax, [{m} + 8]", "adc [{a} + 8], rax",
                "mov rax, [{m} + 16]", "adc [{a} + 16], rax",
                "mov rax, [{m} + 24]", "adc [{a} + 24], rax",
                "2:",
                a = in(reg) a, b = in(reg) b, m = in(reg) modulus,
                out("rax") _, clobber_abi("C")
            );
        }
        5 => {
            core::arch::asm!(
                "mov rax, [{b}]", "sub [{a}], rax",
                "mov rax, [{b} + 8]", "sbb [{a} + 8], rax",
                "mov rax, [{b} + 16]", "sbb [{a} + 16], rax",
                "mov rax, [{b} + 24]", "sbb [{a} + 24], rax",
                "mov rax, [{b} + 32]", "sbb [{a} + 32], rax",
                "jnc 2f",
                "mov rax, [{m}]", "add [{a}], rax",
                "mov rax, [{m} + 8]", "adc [{a} + 8], rax",
                "mov rax, [{m} + 16]", "adc [{a} + 16], rax",
                "mov rax, [{m} + 24]", "adc [{a} + 24], rax",
                "mov rax, [{m} + 32]", "adc [{a} + 32], rax",
                "2:",
                a = in(reg) a, b = in(reg) b, m = in(reg) modulus,
                out("rax") _, clobber_abi("C")
            );
        }
        _ => unimplemented!("Only N=3, 4, 5 are optimized"),
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn sub_assign_n3(a: &mut [u64; 3], b: &[u64; 3], modulus: &[u64; 3]) {
    core::arch::asm!(
        // 1. A = A - B
        "mov rax, [{b}]",
        "sub [{a}], rax",
        "mov rax, [{b} + 8]",
        "sbb [{a} + 8], rax",
        "mov rax, [{b} + 16]",
        "sbb [{a} + 16], rax",

        // 2. 如果没有借位 (No Carry/Borrow)，跳转到结束
        "jnc 2f",

        // 3. 如果有借位 (A < B)，加上模数 p: A = A + p
        "mov rax, [{m}]",
        "add [{a}], rax",
        "mov rax, [{m} + 8]",
        "adc [{a} + 8], rax",
        "mov rax, [{m} + 16]",
        "adc [{a} + 16], rax",

        "2:", // done 标签
        a = in(reg) a.as_mut_ptr(),
        b = in(reg) b.as_ptr(),
        m = in(reg) modulus.as_ptr(),
        out("rax") _,
        clobber_abi("C")
    );
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn sub_assign_n4(a: &mut [u64; 4], b: &[u64; 4], modulus: &[u64; 4]) {
    core::arch::asm!(
        "mov rax, [{b}]", "sub [{a}], rax",
        "mov rax, [{b} + 8]", "sbb [{a} + 8], rax",
        "mov rax, [{b} + 16]", "sbb [{a} + 16], rax",
        "mov rax, [{b} + 24]", "sbb [{a} + 24], rax",
        "jnc 2f",
        "mov rax, [{m}]", "add [{a}], rax",
        "mov rax, [{m} + 8]", "adc [{a} + 8], rax",
        "mov rax, [{m} + 16]", "adc [{a} + 16], rax",
        "mov rax, [{m} + 24]", "adc [{a} + 24], rax",
        "2:",
        a = in(reg) a.as_mut_ptr(), b = in(reg) b.as_ptr(), m = in(reg) modulus.as_ptr(),
        out("rax") _, clobber_abi("C")
    );
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn sub_assign_n5(a: &mut [u64; 5], b: &[u64; 5], modulus: &[u64; 5]) {
    core::arch::asm!(
        "mov rax, [{b}]", "sub [{a}], rax",
        "mov rax, [{b} + 8]", "sbb [{a} + 8], rax",
        "mov rax, [{b} + 16]", "sbb [{a} + 16], rax",
        "mov rax, [{b} + 24]", "sbb [{a} + 24], rax",
        "mov rax, [{b} + 32]", "sbb [{a} + 32], rax",
        "jnc 2f",
        "mov rax, [{m}]", "add [{a}], rax",
        "mov rax, [{m} + 8]", "adc [{a} + 8], rax",
        "mov rax, [{m} + 16]", "adc [{a} + 16], rax",
        "mov rax, [{m} + 24]", "adc [{a} + 24], rax",
        "mov rax, [{m} + 32]", "adc [{a} + 32], rax",
        "2:",
        a = in(reg) a.as_mut_ptr(), b = in(reg) b.as_ptr(), m = in(reg) modulus.as_ptr(),
        out("rax") _, clobber_abi("C")
    );
}
#[inline(always)]
//    pub fn sub_assign<const N: usize> (data:&mut [u64; N], other: &[u64; N], modulus: &[u64; N])->bool {
//         #[cfg(target_arch = "x86_64")]
//         {
//             // 编译器会根据 N 的具体值优化掉不相关的分支
//             match N {
//                 3 => unsafe { sub_assign_n3(data,other, modulus) },
//                 4 => unsafe { sub_assign_n4(data,other, modulus) },
//                 5 => unsafe { sub_assign_n5(data,other, modulus) },
//                 _ => {return false}//self.sub_assign_portable(other, modulus), // 回退到普通实现
//             }
//         }
//         // #[cfg(not(target_arch = "x86_64"))]
//         // {
//         //     self.sub_assign_portable(other, modulus);
//         // }
//         true
//     }

/// 通用的 Montgomery Reduction 逻辑
/// 适用于任意 N，对应 C++ 的默认回退方案
pub fn mul_reduce_portable<const N: usize>(
    a: &[u64; N],
    b: &[u64; N],
    modulus: &[u64; N],
    inv: u64,
) -> [u64; N] {
    // 1. 计算 res = a * b (双倍长度)
    // 在 Rust 中可以使用现成的 bigint 库，或者手动实现简单的长乘法
    let mut res = vec![0u64; N * 2];
    for i in 0..N {
        let mut carry = 0u64;
        for j in 0..N {
            let prod = (a[i] as u128) * (b[j] as u128) + (res[i + j] as u128) + (carry as u128);
            res[i + j] = prod as u64;
            carry = (prod >> 64) as u64;
        }
        res[i + N] = carry;
    }

    // 2. Montgomery Reduction 核心循环
    for i in 0..N {
        let k = res[i].wrapping_mul(inv);
        let mut carry = 0u64;

        // 对应 C++ 的 mpn_addmul_1(res+i, modulus.data, n, k)
        for j in 0..N {
            let prod = (k as u128) * (modulus[j] as u128) + (res[i + j] as u128) + (carry as u128);
            res[i + j] = prod as u64;
            carry = (prod >> 64) as u64;
        }

        // 处理进位传递：对应 C++ 的 mpn_add_1
        let mut pos = i + N;
        let mut c_in = carry;
        while c_in > 0 && pos < 2 * N {
            let (val, overflow) = res[pos].overflowing_add(c_in);
            res[pos] = val;
            c_in = if overflow { 1 } else { 0 };
            pos += 1;
        }
    }

    // 3. 最终结果在 res[N..2N]，执行条件减法 (if res_hi >= modulus)
    let mut res_hi = [0u64; N];
    res_hi.copy_from_slice(&res[N..2 * N]);

    let mut exceeds = false;
    for i in (0..N).rev() {
        if res_hi[i] > modulus[i] {
            exceeds = true;
            break;
        }
        if res_hi[i] < modulus[i] {
            break;
        }
        if i == 0 {
            exceeds = true;
        } // 相等
    }

    if exceeds {
        let mut borrow = 0u64;
        for i in 0..N {
            let (val, b) = res_hi[i].overflowing_sub(modulus[i]);
            let (val2, b2) = val.overflowing_sub(borrow);
            res_hi[i] = val2;
            borrow = (b as u64) + (b2 as u64);
        }
    }

    res_hi
}

pub fn add_assign_portable<const N: usize>(a: &mut [u64; N], b: &[u64; N], modulus: &[u64; N]) {
    // 1. 执行大整数加法：a = a + b，并保存最后的进位
    let mut carry = 0u64;
    for i in 0..N {
        // 使用 overflowing_add 模拟带进位的加法
        let (sum1, c1) = a[i].overflowing_add(b[i]);
        let (sum2, c2) = sum1.overflowing_add(carry);
        a[i] = sum2;
        carry = (c1 as u64) + (c2 as u64);
    }

    // 2. 检查是否需要减去模数
    // 情况 A: 产生了进位 (carry == 1)
    // 情况 B: 结果 a >= modulus
    let mut should_subtract = carry > 0;
    if !should_subtract {
        for i in (0..N).rev() {
            if a[i] > modulus[i] {
                should_subtract = true;
                break;
            }
            if a[i] < modulus[i] {
                break;
            }
            if i == 0 {
                // 完全相等
                should_subtract = true;
            }
        }
    }

    // 3. 执行减法：a = a - modulus
    if should_subtract {
        let mut borrow = 0u64;
        for i in 0..N {
            let (sub1, b1) = a[i].overflowing_sub(modulus[i]);
            let (sub2, b2) = sub1.overflowing_sub(borrow);
            a[i] = sub2;
            borrow = (b1 as u64) + (b2 as u64);
        }
        // 注意：根据数学原理，这里的最终 borrow 必然为 carry，
        // 抵消后结果保证在 [0, modulus-1] 范围内。
    }
}
pub fn sub_assign_portable<const N: usize>(a: &mut [u64; N], b: &[u64; N], modulus: &[u64; N]) {
    // 1. 执行大整数减法：a = a - b，并保存借位
    let mut borrow = 0u64;
    for i in 0..N {
        let (sub1, b1) = a[i].overflowing_sub(b[i]);
        let (sub2, b2) = sub1.overflowing_sub(borrow);
        a[i] = sub2;
        borrow = (b1 as u64) + (b2 as u64);
    }

    // 2. 如果产生了借位 (borrow == 1)，说明 A < B，需要补回模数 p
    // 这等价于 C++ 中先判断 cmp < 0 再进行 add_n 的逻辑
    if borrow > 0 {
        let mut carry = 0u64;
        for i in 0..N {
            let (sum1, c1) = a[i].overflowing_add(modulus[i]);
            let (sum2, c2) = sum1.overflowing_add(carry);
            a[i] = sum2;
            carry = (c1 as u64) + (c2 as u64);
        }
        // 这里的 carry 理论上会抵消之前的 borrow，最终结果回到合法范围
    }
}
