// #![feature(asm_const)] 
use std::arch::asm;
pub fn mul_reduce_n3(a: &[u64; 3], b: &[u64; 3], modulus: &[u64; 3], inv: u64) -> [u64; 3] {
    let mut res = [0u64; 6];
    unsafe {
        
        
        comba_3_by_3_mul(&mut res, a, b);
        
        reduce_6_limb_product(&mut res, modulus, inv);

        
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
    
    

    let r_ptr = res_hi.as_mut_ptr();
    let m_ptr = modulus.as_ptr();

    core::arch::asm!(
        
        "mov {tmp}, [{m_ptr} + 16]",
        "cmp [{r_ptr} + 16], {tmp}",
        "jb 3f", 
        "ja 2f", 

        "mov {tmp}, [{m_ptr} + 8]",
        "cmp [{r_ptr} + 8], {tmp}",
        "jb 3f",
        "ja 2f",

        "mov {tmp}, [{m_ptr}]",
        "cmp [{r_ptr}], {tmp}",
        "jb 3f",

        
        "2:", 
        "mov {tmp}, [{m_ptr}]",
        "sub [{r_ptr}], {tmp}",

        "mov {tmp}, [{m_ptr} + 8]",
        "sbb [{r_ptr} + 8], {tmp}",

        "mov {tmp}, [{m_ptr} + 16]",
        "sbb [{r_ptr} + 16], {tmp}",

        "3:", 
        m_ptr = in(reg) m_ptr,
        r_ptr = in(reg) r_ptr,
        tmp = out(reg) _,     
        
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
        "mov r8, rdx",       

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
        "mov [{res} + 40], r12", 

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

//     for i in 0..3 {
//         core::arch::asm!(
//             // k = res[i] * inv mod 2^64
//             "mov rax, [{res} + {offset}]",
//             "mul {inv}",
//             "mov r8, rax", // r8 = k

//             // res[i..i+3] += k * modulus
//             "mul qword ptr [{m}]",
//             "add rax, [{res} + {offset}]", 
//             "mov r9, rdx",
//             "adc r9, 0", 

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
//             "add [{res} + {offset} + 24], rdx", 

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
        "add {c0}, rax", 
        "mov [ {res} + 8 ], {c0}",
        "adc {c1}, rdx", 
        "adc {c2}, 0",

        // --- Round 2: 2 * (a0 * a2) + a1 * a1 (register renaming c1, c2, c0) ---
        "mov rax, [ {A} + 0 ]",
        "xor {c0}, {c0}",
        "mul qword ptr [ {A} + 16 ]",
        "add {c1}, rax",
        "adc {c2}, rdx",
        "adc {c0}, 0",
        "add {c1}, rax", 
        "adc {c2}, rdx", 
        "adc {c0}, 0",

        "mov rax, [ {A} + 8 ]",
        "mul rax",       
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
        "add {c2}, rax", 
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
    let mut res = [0u64; 6]; 

    unsafe {
        
        
        core::arch::asm!(
            
            
            

            
            
            

            
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
            res_high = in(reg) &mut res[3], 
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
        
        reduce_6_limb_product(&mut res, modulus, inv);

        // --- 3. FINAL SUBTRACTION  ---
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
            
            let mut carry = 0u64;
            let mut res = [0u64; 4];

            
            for i in 0..4 {
                let (sum, c) = self.data[i].overflowing_add(other.data[i]);
                let (sum2, c2) = sum.overflowing_add(carry);
                res[i] = sum2;
                carry = (c as u64) + (c2 as u64);
            }

            
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
                
                "mov {rax}, [{b}]",
                "sub [{a}], {rax}",
                "mov {rax}, [{b} + 8]",
                "sbb [{a} + 8], {rax}",
                "mov {rax}, [{b} + 16]",
                "sbb [{a} + 16], {rax}",
                "mov {rax}, [{b} + 24]",
                "sbb [{a} + 24], {rax}",

                
                "jnc 2f",

                
                "mov {rax}, [{m}]",
                "add [{a}], {rax}",
                "mov {rax}, [{m} + 8]",
                "adc [{a} + 8], {rax}",
                "mov {rax}, [{m} + 16]",
                "adc [{a} + 16], {rax}",
                "mov {rax}, [{m} + 24]",
                "adc [{a} + 24], {rax}",

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

#[cfg(target_arch = "x86_64")]
pub unsafe fn mul_reduce_n4(a: &[u64; 4], b: &[u64; 4], modulus: &[u64; 4], inv: u64) -> [u64; 4] {
    let mut tmp = [0u64; 5]; 
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

#[cfg(target_arch = "x86_64")]
pub unsafe fn mul_reduce_n5(a: &[u64; 5], b: &[u64; 5], modulus: &[u64; 5], inv: u64) -> [u64; 5] {
    let mut tmp = [0u64; 6]; 
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

#[cfg(target_arch = "x86_64")]
pub unsafe fn mull_reduce_n3(a: &[u64; 3], b: &[u64; 3], modulus: &[u64; 3], inv: u64) -> [u64; 3] {
    let mut tmp = [0u64; 6]; 
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

    
    fn mul_reduce(this: &mut Self::Array, other: &Self::Array, modulus: &Self::Array, inv: u64);
}


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

    
    pub fn mul_assign(&mut self, other: &Self, modulus: &C::Array, inv: u64) {
        unsafe {
            C::mul_reduce(&mut self.data, &other.data, modulus, inv);
        }
    }
}
fn main() {
    
    let mut a = Fp::<N4>::new([1, 0, 0, 0]);
    let b = Fp::<N4>::new([2, 0, 0, 0]);
    let m = [0xFFFFFFFFFFFFFFFFu64; 4];
    let inv = 123456789u64; 

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

                
                "jc 2f",          
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

        
        "jc 2f",          
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

                
                "jnc 2f",

                
                "mov rax, [{m}]",
                "add [{a}], rax",
                "mov rax, [{m} + 8]",
                "adc [{a} + 8], rax",
                "mov rax, [{m} + 16]",
                "adc [{a} + 16], rax",

                "2:", 
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

        
        "jnc 2f",

        
        "mov rax, [{m}]",
        "add [{a}], rax",
        "mov rax, [{m} + 8]",
        "adc [{a} + 8], rax",
        "mov rax, [{m} + 16]",
        "adc [{a} + 16], rax",

        "2:", 
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


pub fn mul_reduce_portable<const N: usize>(
    a: &[u64; N],
    b: &[u64; N],
    modulus: &[u64; N],
    inv: u64,
) -> [u64; N] {
    
    
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

    
    for i in 0..N {
        let k = res[i].wrapping_mul(inv);
        let mut carry = 0u64;

        
        for j in 0..N {
            let prod = (k as u128) * (modulus[j] as u128) + (res[i + j] as u128) + (carry as u128);
            res[i + j] = prod as u64;
            carry = (prod >> 64) as u64;
        }

        
        let mut pos = i + N;
        let mut c_in = carry;
        while c_in > 0 && pos < 2 * N {
            let (val, overflow) = res[pos].overflowing_add(c_in);
            res[pos] = val;
            c_in = if overflow { 1 } else { 0 };
            pos += 1;
        }
    }

    
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
        } 
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
    
    let mut carry = 0u64;
    for i in 0..N {
        
        let (sum1, c1) = a[i].overflowing_add(b[i]);
        let (sum2, c2) = sum1.overflowing_add(carry);
        a[i] = sum2;
        carry = (c1 as u64) + (c2 as u64);
    }

    
    
    
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
                
                should_subtract = true;
            }
        }
    }

    
    if should_subtract {
        let mut borrow = 0u64;
        for i in 0..N {
            let (sub1, b1) = a[i].overflowing_sub(modulus[i]);
            let (sub2, b2) = sub1.overflowing_sub(borrow);
            a[i] = sub2;
            borrow = (b1 as u64) + (b2 as u64);
        }
        
        
    }
}
pub fn sub_assign_portable<const N: usize>(a: &mut [u64; N], b: &[u64; N], modulus: &[u64; N]) {
    
    let mut borrow = 0u64;
    for i in 0..N {
        let (sub1, b1) = a[i].overflowing_sub(b[i]);
        let (sub2, b2) = sub1.overflowing_sub(borrow);
        a[i] = sub2;
        borrow = (b1 as u64) + (b2 as u64);
    }

    
    
    if borrow > 0 {
        let mut carry = 0u64;
        for i in 0..N {
            let (sum1, c1) = a[i].overflowing_add(modulus[i]);
            let (sum2, c2) = sum1.overflowing_add(carry);
            a[i] = sum2;
            carry = (c1 as u64) + (c2 as u64);
        }
        
    }
}
