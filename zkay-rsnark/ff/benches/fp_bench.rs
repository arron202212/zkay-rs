use criterion::{Criterion, black_box, criterion_group, criterion_main};

// --- 纯 Rust 实现 (模拟编译器生成的常规代码) ---
fn add_portable<const N: usize>(a: &mut [u64; N], b: &[u64; N], m: &[u64; N]) {
    let mut carry = 0u64;
    for i in 0..N {
        let (res, c) = a[i].overflowing_add(b[i]);
        let (res2, c2) = res.overflowing_add(carry);
        a[i] = res2;
        carry = (c as u64) + (c2 as u64);
    }

    // 检查是否大于模数
    let mut exceeds = carry > 0;
    if !exceeds {
        for i in (0..N).rev() {
            if a[i] > m[i] {
                exceeds = true;
                break;
            }
            if a[i] < m[i] {
                break;
            }
            if i == 0 {
                exceeds = true;
            } // 相等
        }
    }

    if exceeds {
        let mut borrow = 0u64;
        for i in 0..N {
            let (res, b_out) = a[i].overflowing_sub(m[i]);
            let (res2, b_out2) = res.overflowing_sub(borrow);
            a[i] = res2;
            borrow = (b_out as u64) + (b_out2 as u64);
        }
    }
}

// --- 调用我们的汇编实现 ---
// 假设你之前的函数叫 sub_assign_asm, add_assign_asm ...

fn bench_fp_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fp_Addition_N3");
    let mut a = [0x1111111111111111u64; 3];
    let b = [0x2222222222222222u64; 3];
    let m = [0x3333333333333333u64; 3]; // 简化模数

    // 测试 Portable
    group.bench_function("Portable_Add", |bencher| {
        bencher.iter(|| {
            add_portable(black_box(&mut a), black_box(&b), black_box(&m));
        })
    });

    // 测试 ASM
    group.bench_function("ASM_Add", |bencher| {
        bencher.iter(|| unsafe {
            // 这里调用你写的汇编函数
            // add_assign_asm::<3>(a.as_mut_ptr(), b.as_ptr(), m.as_ptr());
            black_box(&mut a); // 模拟
        })
    });

    group.finish();
}

criterion_group!(benches, bench_fp_ops);
criterion_main!(benches);
