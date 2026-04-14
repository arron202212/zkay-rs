// Declaration of arithmetic in the finite field F[p], for prime p of fixed length.
use crate::{
    FieldTConfig, PpConfig,
    algebra::{
        field_utils::{
            BigInteger,
            algorithms::{
                FPMConfig, FieldTForPowersConfig, PowerConfig, Powers, tonelli_shanks_sqrt,
            },
            bigint::{GMP_NUMB_BITS, bigint},
            field_utils, fp_aux, {BigInt, algorithms},
        },
        fields::{
            field::{AdditiveGroup, Field},
            fpn_field::PrimeField,
            sqrt::SqrtPrecomputation,
        },
    },
    common::utils::bit_vector,
    fp_aux::{add_assign_portable, mul_reduce_portable, sub_assign_portable},
};
use cfg_if::cfg_if;
use educe::Educe;
use num_bigint::BigUint;
use num_traits::{Num, One, Signed, Zero};
use std::{
    borrow::Borrow,
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};
//  use crate::algebra::field_utils::bigint::bigint;

/**
 * Arithmetic in the finite field F[p], for prime p of fixed length.
 *
 * This pub struct implements Fp-arithmetic, for a large prime p, using a fixed number
 * of words. It is optimized for tight memory consumption, so the modulus p is
 * passed as a template parameter, to avoid per-element overheads.
 *
 * The implementation is mostly a wrapper around GMP's MPN (constant-size integers).
 * But for the integer sizes of interest for libff (3 to 5 limbs of 64 bits each),
 * we implement performance-critical routines, like addition and multiplication,
 * using hand-optimzied assembly code.
 */

pub trait Fp_modelConfig<const N: usize>:
    Send + Sync + 'static + Sized + Default + Clone + Copy + Eq + Debug
{
    // const num_limbs: usize = 4;
    const modulus: bigint<N> = bigint::<N>::one();
    const num_bits: usize = 254;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 42; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // a quadratic nonresidue
    const nqr_to_t: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // nqr^t
    const multiplicative_generator: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator of Fp^*
    const root_of_unity: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator^((modulus-1)/2^s)
    const inv: u64 = 0xc2e1f593efffffff; // modulus^(-1) mod W, where W = 2^(word size)
    const Rsquared: bigint<N> = bigint::<N>::one(); // R^2, where R = W^k, where k = ??
    const Rcubed: bigint<N> = bigint::<N>::one(); // R^3
}

#[derive(Educe)]
#[educe(Default, Clone, Debug, Hash, Copy, PartialOrd, Ord, Eq)] // PartialEq,
pub struct Fp_model<const N: usize, T: Fp_modelConfig<N>> {
    pub mont_repr: bigint<N>,
    pub t: PhantomData<T>,
}

impl<const N: usize, T: Fp_modelConfig<N>> Fp_modelConfig<N> for Fp_model<N, T> {
    // const num_limbs: usize = T::num_limbs;
    const modulus: bigint<N> = bigint::<N>::one();
    const num_bits: usize = 1;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // a quadratic nonresidue
    const nqr_to_t: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // nqr^t
    const multiplicative_generator: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator of Fp^*
    const root_of_unity: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator^((modulus-1)/2^s)
    const inv: u64 = 1; // modulus^(-1) mod W, where W = 2^(word size)
    const Rsquared: bigint<N> = bigint::<N>::one(); // R^2, where R = W^k, where k = ??
    const Rcubed: bigint<N> = bigint::<N>::one(); // R^3
}

// impl<const N: usize, T: Fp_modelConfig<N>> Borrow<Self> for Fp_model<N, T> {
//     fn borrow(&self)->Self{
//         *self
//     }
// }

impl<const N: usize, T: Fp_modelConfig<N>> FieldTConfig for Fp_model<N, T> {}

impl<const N: usize, T: Fp_modelConfig<N>> PpConfig for Fp_model<N, T> {
    //type TT = bigint<N>;
    const num_limbs: usize = N;
    fn size_in_bits() -> usize {
        T::num_bits
    }
    // type Fr=Self;
}
impl<const N: usize, T: Fp_modelConfig<N>> AsMut<[u64]> for Fp_model<N, T> {
    fn as_mut(&mut self) -> &mut [u64] {
        &mut self.mont_repr.0.0
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<usize> for Fp_model<N, T> {
    fn from(b: usize) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<u32> for Fp_model<N, T> {
    fn from(b: u32) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<i32> for Fp_model<N, T> {
    fn from(b: i32) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<i64> for Fp_model<N, T> {
    fn from(b: i64) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<u64> for Fp_model<N, T> {
    fn from(b: u64) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b),
            t: PhantomData,
        }
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> From<&str> for Fp_model<N, T> {
    fn from(b: &str) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new_with_str(b).expect(b),
            t: PhantomData,
        }
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> From<BigUint> for Fp_model<N, T> {
    fn from(b: BigUint) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>(b.try_into().unwrap()),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> FPMConfig for Fp_model<N, T> {}
impl<const N: usize, T: Fp_modelConfig<N>> FieldTForPowersConfig<N> for Fp_model<N, T> {
    type FPM = Self;
    const num_limbs: usize = N;
    const s: usize = T::s; // modulus = 2^s * t + 1
    const t: bigint<N> = T::t; // with t odd
    const t_minus_1_over_2: bigint<N> = T::t_minus_1_over_2; // (t-1)/2
    const nqr: Self = T::nqr; // a quadratic nonresidue
    const nqr_to_t: Self = T::nqr_to_t; // nqr^t
    fn squared_(&self) -> Self {
        self.squared()
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    pub fn ceil_size_in_bits() -> usize {
        T::num_bits
    }
    pub fn floor_size_in_bits() -> usize {
        T::num_bits - 1
    }

    pub fn extension_degree() -> usize {
        1
    }
    pub fn field_char() -> bigint<N> {
        T::modulus
    }
    pub fn modulus_is_valid() -> bool {
        T::modulus.0.0[N - 1] != 0
    } // mpn inverse assumes that highest limb is non-zero

    pub const fn const_default() -> Fp_model<N, T> {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        }
    }
    pub const fn const_new(b: BigInt<N>) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>(b),
            t: PhantomData,
        }
    }

    pub fn mul_reduce(&mut self, other: &bigint<N>) {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {

               let data= match N {
                    3 => unsafe { mul_reduce_n3(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv) },
                    4 => unsafe { mul_reduce_n4(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv) },
                    5 => unsafe { mul_reduce_n5(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv) },
                    _ => {return }//self.sub_assign_portable(other, modulus), // 回退到普通实现
                };
                self.mont_repr.0.0.copy_from_slice(&data[N..N*2]);
            }else{
               let data= mul_reduce_portable::<N>(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv);
                self.mont_repr.0.0.copy_from_slice(&data[N..N*2]);
            }
        }
    }

    pub fn new(b: bigint<N>) -> Self {
        let mut _self = Self {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        };
        _self.mont_repr.0.0.copy_from_slice(&T::Rsquared.0.0[..N]);
        _self.mul_reduce(&b);
        _self
    }

    pub fn new_with_i64(x: i64, is_unsigned: bool) -> Self {
        // assert!(std::numeric_limits<mp_limb_t>::max() >= std::numeric_limits<long>::max() as u64, "long won't fit in mp_limb_t");
        let mut _self = Self {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        };
        if is_unsigned || x >= 0 {
            _self.mont_repr.0.0[0] = x as u64;
        } else {
            // 假设 this.mont_repr.data 是 [u64; N]
            // modulus.data 是 [u64; N]
            // x 是一个标量 (u64)

            let sub_val = (-(x as i64)) as u64; // 对应 (mp_limb_t)-x
            let mut borrow = 0u8;

            // 模拟 mpn_sub_1 的逻辑：从数组第一个元素减去标量，并传播借位
            let (res, b) = _self.mont_repr.0.0[0].overflowing_sub(sub_val);
            _self.mont_repr.0.0[0] = res;
            borrow = b as u8;

            // 如果 N > 1，需要传播借位（mpn_sub_1 会自动处理数组后续部分）
            for i in 1..N {
                if borrow == 0 {
                    break;
                }
                let (res, b) = _self.mont_repr.0.0[i].overflowing_sub(borrow as u64);
                _self.mont_repr.0.0[i] = res;
                borrow = b as u8;
            }

            // 对应 #ifndef NDEBUG assert(borrow == 0)
            debug_assert_eq!(borrow, 0, "Borrow must be zero in prime field subtraction");
        }

        _self.mul_reduce(&T::Rsquared);
        _self
    }
    pub fn set_ulong(&mut self, x: u64) {
        self.mont_repr.clear();
        self.mont_repr.0.0[0] = x;
        self.mul_reduce(&T::Rsquared);
    }

    pub const fn clear(&mut self) {
        self.mont_repr.clear();
    }

    pub fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub fn as_bigint(&self) -> bigint<N> {
        let mut one = bigint::<N>::one();
        let mut res = self.clone();
        res.mul_reduce(&one);

        res.mont_repr
    }

    pub fn as_ulong(&self) -> u64 {
        self.as_bigint().as_ulong()
    }

    pub fn is_zero(&self) -> bool {
        self.mont_repr.is_zero() // zero maps to zero
    }

    pub fn print(&self) {
        let mut tmp = Self::zero();
        tmp.mont_repr.0.0[0] = 1;
        tmp.mul_reduce(&self.mont_repr);

        tmp.mont_repr.print();
    }

    pub const fn zero() -> Self {
        let mut res = Self::const_default();
        res.mont_repr.clear();
        res
    }

    pub fn one() -> Self {
        let mut res = Self::default();
        res.mont_repr.0.0[0] = 1;
        res.mul_reduce(&T::Rsquared);
        res
    }

    pub fn geometric_generator() -> Self {
        let mut res = Self::default();
        res.mont_repr.0.0[0] = 2;
        res.mul_reduce(&T::Rsquared);
        res
    }

    pub fn arithmetic_generator() -> Self {
        let mut res = Self::default();
        res.mont_repr.0.0[0] = 1;
        res.mul_reduce(&T::Rsquared);
        res
    }

    pub fn squared(&self) -> Self {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {
               let out= squared_n3(&self.mont_repr.0.0,&T::modulus.0,T::inv);
                self.mont_repr.0.0.copy_from_slice(out);
            }else{

        let mut r: Self = self.clone();
        r *= &r.clone();
        r
            }
        }
    }

    pub fn square(&mut self) -> &Self {
        *self = self.squared();
        self
    }

    pub fn invert(&mut self) -> Self {
        // 1. 断言非零
        debug_assert!(!self.is_zero());

        // 2. 准备变量 (Rust 中通常使用 Vec 或固定长度数组)
        let mut v = T::modulus;
        let mut u = self.mont_repr;

        // 3. 计算 GCD (对应 mpn_gcdext)
        // Rust 的 BigInt 库通常返回 (gcd, s, t)
        let (g, s, _t) = u.extended_gcd(&v);

        // 4. 验证逆元存在 (gn == 1 && g == 1)
        debug_assert!(g.is_one(), "Inverse does not exist");

        // 5. 处理符号和模还原 (对应 if (sn < 0) 和 mpn_sub_n)
        let mut res = if num_bigint::BigInt::from(s).is_negative() {
            // 如果 s 是负数，加上模数: res = modulus - |s|
            let mut tmp = T::modulus;
            tmp.sub(s.abs()); //sub_noborrow
            tmp
        } else {
            // 如果 s 超过模数，进行取模 (对应 mpn_tdiv_qr)
            s % T::modulus
        };

        // 6. Montgomery 修正 (对应 mul_reduce(Rcubed))
        // 注意：在 Montgomery 空间求逆后，结果需要乘以 R^3 (或 R^2 再 reduce)
        res.mul_assign(&T::Rcubed);

        self.mont_repr = res;
        self.clone()
    }

    pub fn inverse(&self) -> Self {
        let mut r = self.clone();
        r.invert()
    }

    pub fn Frobenius_map(&self, _power: usize) -> Self {
        self.clone()
    }

    pub fn random_element() -> Self {
        // 1. 定义随机元素 r
        let mut r_data = [0u64; N];
        let mut rng = rand::thread_rng();
        loop {
            // 2. 随机填充所有位 (randomize)
            rng.fill(&mut r_data[..]);

            // 3. 清除模数最高位以上的无效位
            // 找到模数最高有效位（MSB）以上的 0 的个数
            let unused_bits = T::modulus[N - 1].leading_zeros();
            if unused_bits > 0 {
                // 创建掩码，例如 unused_bits 为 3，则掩码为 000111...1
                let mask = u64::MAX >> unused_bits;
                r_data[N - 1] &= mask;
            }

            // 4. 拒绝采样 (mpn_cmp)
            // 如果生成的数仍然大于等于模数，则重新循环
            if r_data < T::modulus.0.0 {
                break;
            }
        }

        // 5. 返回结果
        Fp_model::new(bigint::<N>(BigInt::<N>(r_data)))
    }

    pub fn sqrt(&self) -> Self {
        tonelli_shanks_sqrt(self)
    }

    pub fn to_words(&self) -> Vec<u64> {
        // TODO: implement for other bit architectures
        assert!(
            GMP_NUMB_BITS == 64,
            "Only 64-bit architectures are currently supported"
        );
        // 1. 对应 static_assert：确保当前平台的指针/字长是 64 位
        // Rust 中通常在编译期或运行期检查目标架构
        // #[cfg(not(target_pointer_width = "64"))]
        // compile_error!("Only 64-bit architectures are currently supported");

        // 2. 获取大整数表示 (对应 bigint_repr())
        // 假设返回的是 [u64; N] 或类似的结构
        let repr = self.bigint_repr();

        // 3. 转换为 Vec<u64> (对应 std::vector<uint64_t>)
        // 在 Rust 中，将数组或切片转换为向量非常直接且高效
        let words: Vec<u64> = repr.as_ref().to_vec();

        words
    }

    pub fn from_words(&mut self, words: &[u64]) -> bool {
        // // TODO: implement for other bit architectures
        // assert!(
        //     GMP_NUMB_BITS == 64,
        //     "Only 64-bit architectures are currently supported"
        // );

        // let start_bit = words.len() * 64; //- FieldT::ceil_size_in_bits();
        // assert!(start_bit >= 0); // Check the vector is big enough.
        // let start_word = start_bit / 64;
        // let bit_offset = start_bit % 64;

        // // Assumes mont_repr.0.0 is just the right size to fit ceil_size_in_bits().
        // // std::copy(words.begin() + start_word, words.end(),self.mont_repr.0.0);
        // self.mont_repr.0.0.clone_from_slice(&words[start_word..]);
        // // Zero out the left-most bit_offset bits.
        // self.mont_repr.0.0[N - 1] =
        //     ((self.mont_repr.0.0[N - 1] as u64) << bit_offset) >> bit_offset; //mp_limb_t

        // // return self.mont_repr < modulus;
        // false
        // 1. 架构断言 (static_assert)
        #[cfg(not(target_pointer_width = "64"))]
        compile_error!("Only 64-bit architectures are currently supported");

        // 2. 计算起始位与偏移量
        // FieldT::ceil_size_in_bits() 对应 T::MODULUS_BIT_SIZE
        let ceil_size_in_bits = Self::ceil_size_in_bits() as i64;
        let start_bit = (words.len() as i64 * 64) - ceil_size_in_bits;

        // 3. 检查向量长度是否足够 (assert)
        assert!(start_bit >= 0, "The vector is not big enough");

        let start_word = (start_bit / 64) as usize;
        let bit_offset = (start_bit % 64) as u32;

        // 4. 数据拷贝 (std::copy)
        // 将 words 中从 start_word 开始的部分拷贝到内部存储
        // 假设内部存储为 [u64; N]
        let mut data = [0u64; N];
        let copy_len = words.len() - start_word;
        data[..copy_len].copy_from_slice(&words[start_word..]);

        // 5. 清除左侧高位比特 (Zero out the left-most bits)
        // 逻辑是：先左移再逻辑右移，清除掉最高位的 bit_offset 个比特
        if bit_offset > 0 {
            data[N - 1] = (data[N - 1] << bit_offset) >> bit_offset;
        }

        // 6. 转换为 Montgomery 表示 (#ifndef MONTGOMERY_OUTPUT)
        // 如果输入的不是 Montgomery 表示，需要乘以 R^2 进行转换
        // 假设 cfg!(feature = "montgomery_output") 对应 MONTGOMERY_OUTPUT
        let mut result = Fp_model::<N, T>::new(bigint::<N>(BigInt::<N>(data)));
        // if !cfg!(feature = "montgomery_output") {
        //     result.mul_assign(&T::Rsquared); // 相当于乘 R 再取模，进入 Montgomery 域
        // }

        // 7. 返回范围检查结果 (return this->mont_repr < modulus)
        self.mont_repr < T::modulus // 内部逻辑通常是 self.mont_repr < T::modulus
    }

    pub fn bigint_repr(&self) -> bigint<N> {
        // 1. 根据 feature 决定返回值
        // 在 Rust 中，#[cfg(feature = "montgomery_output")] 对应 #ifdef MONTGOMERY_OUTPUT
        cfg_if! {
        if  #[cfg(feature = "montgomery_output")]
         {
             // 如果定义了 montgomery_output，直接返回内部存储格式
             return self.mont_repr;
         }
         else

         {
             // 否则，返回转换为标准表示的大整数（即进行 Montgomery 约减）
             return self.as_bigint();
         }
          }
    }
}
use rand::Rng;
use rand::distributions::{Distribution, Standard};

impl<const N: usize, T: Fp_modelConfig<N>> Distribution<Fp_model<N, T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fp_model<N, T> {
        let mut r_data = [0u64; N];
        let modulus = &T::modulus.0.0; // 假设存储为 [u64; N]

        loop {
            // 1. 填充随机位 (对应 r.mont_repr.randomize())
            for limb in r_data.iter_mut() {
                *limb = rng.next_u64();
            }

            // 2. 清除高位无效比特 (对应 C++ 中的 while(test_bit) 逻辑)
            // 找到模数最高有效位上方的 0 的数量
            let unused_bits = modulus[N - 1].leading_zeros();
            if unused_bits > 0 {
                let mask = u64::MAX >> unused_bits;
                r_data[N - 1] &= mask;
            }

            // 3. 拒绝采样 (对应 while(mpn_cmp >= 0))
            // Rust 的数组原生支持按大整数逻辑比较 (Lexicographical order)
            if &r_data < modulus {
                break;
            }
        }

        Fp_model::new(bigint::<N>(BigInt::<N>(r_data)))
    }
}
// let mut rng = rand::thread_rng();

// // 方式 A：显式采样
// let r: Fp384 = rng.sample(Standard);

// // 方式 B：使用 gen (编译器会自动推导类型)
// let r: Fp384 = rng.gen();

impl<const N: usize, T: Fp_modelConfig<N>> PartialEq for Fp_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mont_repr == other.mont_repr
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> AddAssign<O> for Fp_model<N, T> {
    fn add_assign(&mut self, other: O) {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {

                match N {
                    3 => unsafe { add_assign_n3(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    4 => unsafe { add_assign_n4(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    5 => unsafe { add_assign_n5(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    _ => {return }//self.sub_assign_portable(other, modulus), // 回退到普通实现
                };
            }else{
               add_assign_portable(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0);
            }
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> SubAssign<O> for Fp_model<N, T> {
    fn sub_assign(&mut self, other: O) {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {

                match N {
                    3 => unsafe { sub_assign_n3(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    4 => unsafe { sub_assign_n4(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    5 => unsafe { sub_assign_n5(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    _ => {return }
                };
            }else{
               sub_assign_portable(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0);
            }
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> MulAssign<O> for Fp_model<N, T> {
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
        self.mul_reduce(&rhs.mont_repr);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXorAssign<u64> for Fp_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = Powers::power::<Fp_model<N, T>>(self, rhs);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXorAssign<bigint<N>> for Fp_model<N, T> {
    fn bitxor_assign(&mut self, rhs: bigint<N>) {
        *self = Powers::power::<Fp_model<N, T>>(self, rhs);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Add<i32> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Sub<i32> for Fp_model<N, T> {
    type Output = Self;

    fn sub(self, other: i32) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Sub for Fp_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        r -= other;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<bigint<N>> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<BigUint> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: BigUint) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<i32> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Mul<i64> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: i64) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>, OT: Fp_modelConfig<N>> Mul<Fp_model<N, OT>>
    for Fp_model<N, T>
{
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: Fp_model<N, OT>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
// impl<const N: usize, T: Fp_modelConfig<N>> Mul for Fp_model<N, T> {
//     type Output = Fp_model<N, T>;

//     fn mul(self, rhs: Fp_model<N, T> ) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }

impl<const N: usize, T: Fp_modelConfig<N>> BitXor<u64> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXor<usize> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> BitXor<bigint<N>> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}
/// 模擬 mpn_sub_n：計算 a - b，並將結果存入 res
/// 返回最終的借位 (borrow)
#[inline]
pub fn sub_n(res: &mut [u64], a: &[u64], b: &[u64]) -> u64 {
    let mut borrow = 0u64;

    // 遍歷所有 limb 進行減法
    for i in 0..a.len() {
        // t = a[i] - b[i] - borrow
        let (v1, b1) = a[i].overflowing_sub(b[i]);
        let (v2, b2) = v1.overflowing_sub(borrow);

        res[i] = v2;
        borrow = (b1 | b2) as u64; // 只要任何一步產生借位，borrow 就為 1
    }

    borrow
}
macro_rules! sub_n {
    ($res:expr, $a:expr, $b:expr, $n:expr) => {{
        let mut borrow = 0u64;
        let mut i = 0;

        // 使用 loop 配合 const N，編譯器會自動進行循環展開 (Unrolling)
        while i < $n {
            let (v1, b1) = $a[i].overflowing_sub($b[i]);
            let (v2, b2) = v1.overflowing_sub(borrow);
            $res[i] = v2;
            // 只要兩次減法中任何一次產生借位，borrow 即為 1
            borrow = (b1 | b2) as u64;
            i += 1;
        }
        borrow
    }};
}
impl<const N: usize, T: Fp_modelConfig<N>> Neg for Fp_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // 1. 檢查是否為零
        if self.is_zero() {
            return self;
        }
        let mut res_data = [0u64; N];
        let a = T::modulus; // 假設 modulus 存儲在 BigInt(pub [u64; N])
        let b = self.mont_repr;

        // 調用宏
        sub_n!(res_data, a, b, N);

        // 3. 返回新元素
        Self::const_new(BigInt::<N>(res_data))
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> One for Fp_model<N, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Zero for Fp_model<N, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

use std::fmt;
impl<const N: usize, T: Fp_modelConfig<N>> fmt::Display for Fp_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bigint_repr(),)
    }
}
// use std::io::{self, Read};

impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    // 模擬輸入流讀取邏輯
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        // 1. 讀取原始數據（假設 mont_repr 有從流讀取的方法）
        let mut mont_repr = bigint::read(&mut reader)?;

        // 2. 構建初步模型
        let mut p = Self::new(mont_repr);

        // 3. 處理 Montgomery 轉換 (#ifndef MONTGOMERY_OUTPUT)
        // #[cfg(not(feature = "montgomery_output"))]
        // {
        //     // 如果輸入的是普通數值，乘 R^2 並 reduce 進入 Montgomery 域
        //     p.mul_assign(&T::Rsquared);
        // }

        Ok(p)
    }
}
use std::io::{self, Read, Write};

impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    /// 从电路输入文件中读取一个域元素
    pub fn read_from_circuit<R: Read>(mut reader: R) -> io::Result<Self> {
        // 1. 直接读取原始 BigInt 字节 (对应 in >> p.mont_repr)
        let mut repr = [0u64; N];
        for limb in repr.iter_mut() {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            *limb = u64::from_le_bytes(buf); // 假设电路文件是小端序
        }

        let mut p = Self::const_new(BigInt::<N>(repr));

        // 2. 根据编译配置处理 Montgomery 转换
        // 如果电路文件存的是“人读数值”，必须转进 Montgomery 域才能计算
        // #[cfg(not(feature = "montgomery_output"))]
        // {
        //     p.mul_assign(T::Rsquared);
        // }

        // 3. 验证读取到的数据是否在合法域范围内
        // if p.is_valid() {
        Ok(p)
        // } else {
        //     Err(io::Error::new(
        //         io::ErrorKind::InvalidData,
        //         "Out of field range",
        //     ))
        // }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    /// 将域元素写入输出文件，供电路验证或结果检查
    pub fn write_to_circuit<W: Write>(&self, mut writer: W) -> io::Result<()> {
        // 1. 获取要输出的数据 (对应 p.bigint_repr())
        let repr = if cfg!(feature = "montgomery_output") {
            self.mont_repr
        } else {
            self.as_bigint() // 移除 Montgomery 因子 R
        };

        // 2. 写入字节流
        for limb in repr.0.0.iter() {
            writer.write_all(&limb.to_le_bytes())?;
        }
        Ok(())
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> FromStr for Fp_model<N, T> {
    type Err = ();

    /// Interpret a string of numbers as a (congruent) prime field element.
    /// Does not accept unnecessary leading zeroes or a blank string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // use num_bigint::{BigInt, BigUint};
        // use num_traits::Signed;

        // let modulus = BigInt::from(P::MODULUS);
        // let mut a = BigInt::from_str(s).map_err(|_| ())? % &modulus;
        // if a.is_negative() {
        //     a += modulus
        // }
        // BigUint::try_from(a)
        //     .map_err(|_| ())
        //     .and_then(TryFrom::try_from)
        //     .ok()
        //     .and_then(Self::from_bigint)
        //     .ok_or(())
        Ok(Self::default())
    }
}

/// A trait that specifies the configuration of a prime field.
/// Also specifies how to perform arithmetic on field elements.
pub trait FpConfig<const N: usize>: Send + Sync + 'static + Sized {
    /// The modulus of the field.
    const MODULUS: BigInt<N>;

    /// A multiplicative generator of the field.
    /// `Self::GENERATOR` is an element having multiplicative order
    /// `Self::modulus - 1`.
    const GENERATOR: Fp<Self, N>;

    /// Additive identity of the field, i.e. the element `e`
    /// such that, for all elements `f` of the field, `e + f = f`.
    const ZERO: Fp<Self, N>;

    /// Multiplicative identity of the field, i.e. the element `e`
    /// such that, for all elements `f` of the field, `e * f = f`.
    const ONE: Fp<Self, N>;

    /// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    const TWO_ADICITY: u32;

    /// 2^s root of unity computed by GENERATOR^t
    const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N>;

    /// An integer `b` such that there exists a multiplicative subgroup
    /// of size `b^k` for some integer `k`.
    const SMALL_SUBGROUP_BASE: Option<u32> = None;

    /// The integer `k` such that there exists a multiplicative subgroup
    /// of size `Self::SMALL_SUBGROUP_BASE^k`.
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32> = None;

    /// GENERATOR^((modulus-1) / (2^s *
    /// SMALL_SUBGROUP_BASE^SMALL_SUBGROUP_BASE_ADICITY)) Used for mixed-radix
    /// FFT.
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<Fp<Self, N>> = None;

    /// Precomputed material for use when computing square roots.
    /// Currently uses the generic Tonelli-Shanks,
    /// which works for every modulus.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp<Self, N>>>;

    /// Set a += b.
    fn add_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Set a -= b.
    fn sub_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Set a = a + a.
    fn double_in_place(a: &mut Fp<Self, N>);

    /// Set a = -a;
    fn neg_in_place(a: &mut Fp<Self, N>);

    /// Set a *= b.
    fn mul_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Compute the inner product `<a, b>`.
    fn sum_of_products<const T: usize>(a: &[Fp<Self, N>; T], b: &[Fp<Self, N>; T]) -> Fp<Self, N>;

    /// Set a *= a.
    fn square_in_place(a: &mut Fp<Self, N>);

    /// Compute a^{-1} if `a` is not zero.
    fn inverse(a: &Fp<Self, N>) -> Option<Fp<Self, N>>;

    /// Construct a field element from an integer in the range
    /// `0..(Self::modulus - 1)`. Returns `None` if the integer is outside
    /// this range.
    fn from_bigint(other: BigInt<N>) -> Option<Fp<Self, N>>;

    /// Convert a field element to an integer in the range `0..(Self::modulus -
    /// 1)`.
    fn into_bigint(other: Fp<Self, N>) -> BigInt<N>;
}
/// Represents an element of the prime field F_p, where `p == P::modulus`.
/// This type can represent elements in any field of size at most N * 64 bits.
#[derive(Educe)]
#[educe(Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Fp<P: FpConfig<N>, const N: usize>(
    /// Contains the element in Montgomery form for efficient multiplication.
    /// To convert an element to a [`BigInt`](struct@BigInt), use `into_bigint` or `into`.
    #[doc(hidden)]
    pub BigInt<N>,
    #[doc(hidden)] pub PhantomData<P>,
);

pub type Fp64<P> = Fp<P, 1>;
pub type Fp128<P> = Fp<P, 2>;
pub type Fp192<P> = Fp<P, 3>;
pub type Fp256<P> = Fp<P, 4>;
pub type Fp320<P> = Fp<P, 5>;
pub type Fp384<P> = Fp<P, 6>;
pub type Fp448<P> = Fp<P, 7>;
pub type Fp512<P> = Fp<P, 8>;
pub type Fp576<P> = Fp<P, 9>;
pub type Fp640<P> = Fp<P, 10>;
pub type Fp704<P> = Fp<P, 11>;
pub type Fp768<P> = Fp<P, 12>;
pub type Fp832<P> = Fp<P, 13>;
