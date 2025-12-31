#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
// #![feature(adt_const_params)]

// Declaration of interfaces for multi-exponentiation routines.
use crate::PpConfig;
pub const inhibit_profiling_info: bool = false;
use std::io::Write;
use std::marker::ConstParamTy;
use std::ops::{Add, Mul};
#[derive(ConstParamTy, PartialEq, Eq)]
pub enum multi_exp_method {
    /**
     * Naive multi-exponentiation individually multiplies each base by the
     * corresponding scalar and adds up the results.
     * multi_exp_method_naive uses opt_window_wnaf_exp for exponentiation,
     * while multi_exp_method_plain uses operator *.
     */
    multi_exp_method_naive,
    multi_exp_method_naive_plain,
    /**
     * A variant of the Bos-Coster algorithm [1],
     * with implementation suggestions from [2].
     *
     * [1] = Bos and Coster, "Addition chain heuristics", CRYPTO '89
     * [2] = Bernstein, Duif, Lange, Schwabe, and Yang, "High-speed high-security signatures", CHES '11
     */
    multi_exp_method_bos_coster,
    /**
     * A special case of Pippenger's algorithm from Page 15 of
     * Bernstein, Doumen, Lange, Oosterwijk,
     * "Faster batch forgery identification", INDOCRYPT 2012
     * (https://eprint.iacr.org/2012/549.pdf)
     * When compiled with USE_MIXED_ADDITION, assumes input is in special form.
     * Requires that T implements .dbl() (and, if USE_MIXED_ADDITION is defined,
     * .to_special(), .mixed_add(), and batch_to_special()).
     */
    multi_exp_method_BDLO12,
}
//
// /**
//  * Computes the sum
//  * \sum_i scalar_start[i] * vec_start[i]
//  * using the selected method.
//  * Input is split into the given number of chunks, and, when compiled with
//  * MULTICORE, the chunks are processed in parallel.
//  */
//
// T multi_exp(Vec<T>::const_iterator vec_start,
// Vec<T>::const_iterator vec_end,
// Vec<FieldT>::const_iterator scalar_start,
// Vec<FieldT>::const_iterator scalar_end,
// const std::usize chunks);
//
//
// /**
//  * A variant of multi_exp that takes advantage of the method mixed_add (instead
//  * of the operator '+').
//  * Assumes input is in special form, and includes special pre-processing for
//  * scalars equal to 0 or 1.
//  */
//
// T multi_exp_with_mixed_addition(Vec<T>::const_iterator vec_start,
// Vec<T>::const_iterator vec_end,
// Vec<FieldT>::const_iterator scalar_start,
// Vec<FieldT>::const_iterator scalar_end,
// const std::usize chunks);
//
// /**
//  * A convenience function for calculating a pure inner product, where the
//  * more complicated methods are not required.
//  */
// template <T>
// T inner_product(Vec<T>::const_iterator a_start,
// Vec<T>::const_iterator a_end,
// Vec<T>::const_iterator b_start,
// Vec<T>::const_iterator b_end);
//
// /**
//  * A window table stores window sizes for different instance sizes for fixed-base multi-scalar multiplications.
//  */
//
pub type window_table<T> = Vec<Vec<T>>;
//
// /**
//  * Compute window size for the given number of scalars.
//  */
//
// std::usize get_exp_window_size(const std::usize num_scalars);
//
// /**
//  * Compute table of window sizes.
//  */
//
// window_table<T> get_window_table(scalar_size:std::usize,
//  window:std::usize,
//  g:&T);
//
//
// T windowed_exp(scalar_size:std::usize,
//    window:std::usize,
//    powers_of_g:&window_table<T>,
//    pow:&FieldT);
//
//
// Vec<T> batch_exp(scalar_size:std::usize,
//  window:std::usize,
//  table:&window_table<T>,
//  v:&Vec<FieldT>);
//
//
// Vec<T> batch_exp_with_coeff(scalar_size:std::usize,
// window:std::usize,
// table:&window_table<T>,
// coeff:&FieldT,
// v:&Vec<FieldT>);
//
//
// pub fn  batch_to_special(Vec<T> &vec);

use crate::algebra::field_utils::bigint::bigint;
use crate::algebra::scalar_multiplication::multiexp;
use crate::algebra::scalar_multiplication::wnaf::*;
use crate::common::profiling::{enter_block, leave_block};
use crate::common::utils::log2;

//
#[derive(Clone)]
pub struct ordered_exponent<const N: usize> {
    // to use std::push_heap and friends later
    pub idx: usize,
    pub r: bigint<N>,
}

impl<const N: usize> ordered_exponent<N> {
    pub fn new(idx: usize, r: bigint<N>) -> Self {
        Self { idx, r }
    }
}
use std::cmp::Ordering;

impl<const N: usize> PartialEq for ordered_exponent<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
impl<const N: usize> PartialOrd for ordered_exponent<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.r.0.0.cmp(&other.r.0.0))
    }
}

//     bool operator<(other:&ordered_exponent<n>) const
//     {
// // #if defined(__x86_64__) && defined(USE_ASM)
//         // if n == 3
//         // {
//             // long res;
//             // __asm__
//                 // ("// check for overflow           \n\t"
//                 //  "mov $0, %[res]                  \n\t"
//                 //  ADD_CMP(16)
//                 //  ADD_CMP(8)
//                 //  ADD_CMP(0)
//                 //  "jmp done%=                      \n\t"
//                 //  "subtract%=:                     \n\t"
//                 //  "mov $1, %[res]                  \n\t"
//                 //  "done%=:                         \n\t"
//                 //  : [res] "=&r" (res)
//                 //  : [A] "r" (other.r.0.0), [mod] "r" (this->r.0.0)
//                 //  : "cc", "%rax");
//             // return res;
//         // }
//         // else if n == 4
//         // {
//             // long res;
//             // __asm__
//                 // ("// check for overflow           \n\t"
//                 //  "mov $0, %[res]                  \n\t"
//                 //  ADD_CMP(24)
//                 //  ADD_CMP(16)
//                 //  ADD_CMP(8)
//                 //  ADD_CMP(0)
//                 //  "jmp done%=                      \n\t"
//                 //  "subtract%=:                     \n\t"
//                 //  "mov $1, %[res]                  \n\t"
//                 //  "done%=:                         \n\t"
//                 //  : [res] "=&r" (res)
//                 //  : [A] "r" (other.r.0.0), [mod] "r" (this->r.0.0)
//                 //  : "cc", "%rax");
//             // return res;
//         // }
//         // else if n == 5
//         // {
//             // long res;
//             // __asm__
//                 // ("// check for overflow           \n\t"
//                 //  "mov $0, %[res]                  \n\t"
//                 //  ADD_CMP(32)
//                 //  ADD_CMP(24)
//                 //  ADD_CMP(16)
//                 //  ADD_CMP(8)
//                 //  ADD_CMP(0)
//                 //  "jmp done%=                      \n\t"
//                 //  "subtract%=:                     \n\t"
//                 //  "mov $1, %[res]                  \n\t"
//                 //  "done%=:                         \n\t"
//                 //  : [res] "=&r" (res)
//                 //  : [A] "r" (other.r.0.0), [mod] "r" (this->r.0.0)
//                 //  : "cc", "%rax");
//             // return res;
//         // }
//         // else
// // //#endif
//         {
//             return (mpn_cmp(this->r.0.0, other.r.0.0, n) < 0);
//         }
//     }
// };

/**
 * multi_exp_inner<T, FieldT, Method>() implementes the specified
 * multiexponentiation method.
 * this implementation relies on some rather arcane template magic:
 * function templates cannot be partially specialized, so we cannot just write
 *     
 *     T multi_exp_inner<T, FieldT, multi_exp_method_naive>
 * thus we resort to using std::enable_if. the basic idea is that *overloading*
 * is what's actually happening here, it's just that, for any given value of
 * Method, only one of the templates will be valid, and thus the correct
 * implementation will be used.
 */

struct MultiExpInner<const Method: multi_exp_method>;

trait MultiExpInnerConfig {
    fn multi_exp_inner<T: PpConfig, FieldT: PpConfig, const NN: usize>(
        vec: &[T],
        scalar: &[FieldT],
    ) -> T;
}

const fn check(method: multi_exp_method) -> u8 {
    method as _
}

// fn main() {
//     dbg!(Item::<{ check(0) }>::foo(),   // A
//          Item::<{ check(1) }>::foo(),   // B
//          Item::<{ check(-1) }>::foo()); // C
// }

// pub trait AsBigint {
//     const num_limbs: i32 = 0;
//     fn as_bigint<const N: usize>(&self) -> bigint<N>;
//     fn dbl(&self) -> Self;
//     fn fixed_base_exp_window_table() -> Vec<usize>;
//     fn batch_to_special_all_non_zeros<T>(t: Vec<T>);
//     fn to_special(&self);
// }

// template<T, FieldT, multi_exp_method Method,
//     std::enable_if<(Method == multi_exp_method_naive), int>::type = 0>
// T multi_exp_inner(
//     Vec<T>::const_iterator vec_start,
//     Vec<T>::const_iterator vec_end,
//     Vec<FieldT>::const_iterator scalar_start,
//     Vec<FieldT>::const_iterator scalar_end)
impl MultiExpInnerConfig for MultiExpInner<{ multi_exp_method::multi_exp_method_naive }> {
    fn multi_exp_inner<T: PpConfig, FieldT: PpConfig, const NN: usize>(
        vec: &[T],
        scalar: &[FieldT],
    ) -> T {
        // const NN:usize=const {FieldT::num_limbs};
        assert!(vec.len() == scalar.len());
        let mut result = T::zero();
        for (v, s) in vec.iter().zip(scalar) {
            let scalar_bigint = s.as_bigint::<{ NN }>();
            result = result + opt_window_wnaf_exp(v, &scalar_bigint, scalar_bigint.num_bits());
        }
        // assert!(scalar_it == scalar_end);

        return result;
    }
}

// template<T, FieldT, multi_exp_method Method,
//     std::enable_if<(Method == multi_exp_method_naive_plain), int>::type = 0>
// T multi_exp_inner(
//     Vec<T>::const_iterator vec_start,
//     Vec<T>::const_iterator vec_end,
//     Vec<FieldT>::const_iterator scalar_start,
//     Vec<FieldT>::const_iterator scalar_end)
impl MultiExpInnerConfig for MultiExpInner<{ multi_exp_method::multi_exp_method_naive_plain }> {
    fn multi_exp_inner<T: PpConfig, FieldT: PpConfig, const NN: usize>(
        vec: &[T],
        scalar: &[FieldT],
    ) -> T {
        assert!(vec.len() == scalar.len());
        let mut result = T::zero();
        for (v, s) in vec.iter().zip(scalar) {
            result = result; //+ s * v;
        }
        // assert!(scalar_it == scalar_end);

        return result;
    }
}

// template<T, FieldT, multi_exp_method Method,
//     std::enable_if<(Method == multi_exp_method_bos_coster), int>::type = 0>
// T multi_exp_inner(
//     Vec<T>::const_iterator vec_start,
//     Vec<T>::const_iterator vec_end,
//     Vec<FieldT>::const_iterator scalar_start,
//     Vec<FieldT>::const_iterator scalar_end)
impl MultiExpInnerConfig for MultiExpInner<{ multi_exp_method::multi_exp_method_bos_coster }> {
    fn multi_exp_inner<T: PpConfig, FieldT: PpConfig, const NN: usize>(
        vec: &[T],
        scalar: &[FieldT],
    ) -> T {
        const n: usize = 0; //FieldT::num_limbs;//MYTODO

        if vec.is_empty() {
            return T::zero();
        }

        // if vec.len()==1
        // {
        //     return scalar[0].clone()*vec[0].clone();
        // }

        let vec_len = scalar.len();
        let odd_vec_len = vec_len + 1 - vec_len % 2;
        let mut opt_q = Vec::with_capacity(odd_vec_len);
        let mut g = Vec::with_capacity(odd_vec_len);

        assert!(vec.len() == scalar.len());
        for (i, (v, s)) in vec.iter().zip(scalar).enumerate() {
            g.push(v.clone());
            opt_q.push(ordered_exponent::<n>::new(i, s.as_bigint()));
        }
        // std::make_heap(opt_q.begin(),opt_q.end());

        if vec_len != odd_vec_len {
            g.push(T::zero());
            opt_q.push(ordered_exponent::<n>::new(
                odd_vec_len - 1,
                bigint::<n>::new(0u64),
            ));
        }
        assert!(g.len() % 2 == 1);
        assert!(opt_q.len() == g.len());

        let mut opt_result = T::zero();

        //     loop
        //     {
        //         let mut a =  opt_q[0].clone();
        //         let  b =  if opt_q[1] < opt_q[2] { &opt_q[2]} else {&opt_q[1]};

        //         let  abits:usize = a.r.num_bits();

        //         if b.r.is_zero()
        //         {
        //             // opt_result = opt_result + (a.r * g[a.idx]);
        //             opt_result = opt_result + opt_window_wnaf_exp(&g[a.idx], &a.r, abits);
        //             break;
        //         }

        //         let  bbits = b.r.num_bits();
        //         let  limit = 20usize.min(abits-bbits);

        //         if bbits < 1usize<<limit
        //         {
        //             /*
        //               In this case, exponentiating to the power of a is cheaper than
        //               subtracting b from a multiple times, so let's do it directly
        //             */
        //             // opt_result = opt_result + (a.r * g[a.idx]);
        //             opt_result = opt_result + opt_window_wnaf_exp(&g[a.idx], &a.r, abits);
        // // // #ifdef DEBUG
        //             // print!("Skipping the following pair ({} bit number vs {} bit):\n", abits, bbits);
        //             // a.r.print();
        //             // b.r.print();
        // // //#endif
        //             a.r.clear();
        //         }
        //         else
        //         {
        //             // x A + y B => (x-y) A + y (B+A)
        //             a.r.0.0[..n].iter_mut().zip(& b.r.0.0[..n]).for_each(|(ar    ,&br)|{
        //                 *ar-=br;
        //             });
        //             g[b.idx] = g[b.idx].clone() + g[a.idx].clone();
        //         }

        //         // regardless of whether a was cleared or subtracted from we push it down, then take back up

        //         /* heapify A down */
        //         let mut  a_pos = 0;
        //         while 2*a_pos + 2< odd_vec_len
        //         {
        //             // this is a max-heap so to maintain a heap property we swap with the largest of the two
        //             if opt_q[2*a_pos+1] < opt_q[2*a_pos+2]
        //             {
        //                 opt_q.swap(a_pos,2*a_pos+2);
        //                 a_pos = 2*a_pos+2;
        //             }
        //             else
        //             {
        //                 opt_q.swap(a_pos,2*a_pos+1);
        //                 a_pos = 2*a_pos+1;
        //             }
        //         }

        //         /* now heapify A up appropriate amount of times */
        //         while a_pos > 0 && opt_q[(a_pos-1)/2] < opt_q[a_pos]
        //         {
        //             opt_q.swap(a_pos,(a_pos-1)/2);
        //             a_pos = (a_pos-1) / 2;
        //         }
        //     }

        return opt_result;
    }
}

// template<T, FieldT, multi_exp_method Method,
//     std::enable_if<(Method == multi_exp_method_BDLO12), int>::type = 0>
// T multi_exp_inner(
//     Vec<T>::const_iterator bases,
//     Vec<T>::const_iterator bases_end,
//     Vec<FieldT>::const_iterator exponents,
//     Vec<FieldT>::const_iterator exponents_end)
impl MultiExpInnerConfig for MultiExpInner<{ multi_exp_method::multi_exp_method_BDLO12 }> {
    fn multi_exp_inner<T: PpConfig, FieldT: PpConfig, const NN: usize>(
        bases: &[T],
        exponents: &[FieldT],
    ) -> T {
        // UNUSED(exponents_end);
        let length = bases.len();

        // empirically, this seems to be a decent estimate of the optimal value of c
        let log2_length = log2(length);
        let c = log2_length - (log2_length / 3 - 2);

        // let  exp_num_limbs =FieldT::num_limbs;
        let mut bn_exponents = Vec::<bigint<{ NN }>>::with_capacity(length);
        let mut num_bits = 0;

        for i in 0..length {
            bn_exponents[i] = exponents[i].as_bigint::<{ NN }>();
            num_bits = std::cmp::max(num_bits, bn_exponents[i].num_bits());
        }

        let num_groups = (num_bits + c - 1) / c;

        let mut result = T::zero();
        let mut result_nonzero = false;

        for k in num_groups..=num_groups {
            if result_nonzero {
                for i in 0..c {
                    result = result.dbl();
                }
            }

            let mut buckets = Vec::with_capacity(1 << c);
            let mut bucket_nonzero = Vec::with_capacity(1 << c);

            for i in 0..length {
                let mut id = 0;
                for j in 0..c {
                    if bn_exponents[i].test_bit(k * c + j) {
                        id |= 1 << j;
                    }
                }

                if id == 0 {
                    continue;
                }

                if bucket_nonzero[id] {
                    // // #ifdef USE_MIXED_ADDITION
                    // buckets[id] = buckets[id].mixed_add(bases[i]);
                    // #else
                    // buckets[id] = buckets[id] + bases[i];
                    // //#endif
                } else {
                    buckets[id] = bases[i].clone();
                    bucket_nonzero[id] = true;
                }
            }

            // // #ifdef USE_MIXED_ADDITION
            // batch_to_special(buckets);
            // //#endif

            let mut running_sum = T::zero();
            let mut running_sum_nonzero = false;

            for i in (0..1usize << c).rev() {
                if bucket_nonzero[i] {
                    if running_sum_nonzero {
                        // // #ifdef USE_MIXED_ADDITION
                        // running_sum = running_sum.mixed_add(buckets[i]);
                        // #else
                        // running_sum = running_sum + buckets[i];
                        // //#endif
                    } else {
                        running_sum = buckets[i].clone();
                        running_sum_nonzero = true;
                    }
                }

                if running_sum_nonzero {
                    if result_nonzero {
                        result = result + running_sum.clone();
                    } else {
                        result = running_sum.clone();
                        result_nonzero = true;
                    }
                }
            }
        }

        return result;
    }
}
//
pub fn multi_exp<T: PpConfig, FieldT: PpConfig, const Method: multi_exp_method>(
    vec: &[T],
    scalar: &[FieldT],
    chunks: usize,
) -> T {
    let total = vec.len();
    if total < chunks || chunks == 1 {
        // no need to split into "chunks", can call implementation directly
        return MultiExpInner::<{ multi_exp_method::multi_exp_method_BDLO12 }>::multi_exp_inner::<
            T,
            FieldT,
            0,
        >(vec, scalar);
    }

    let one = total / chunks;

    let mut partial = vec![T::zero(); chunks];

    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    for i in 0..chunks {
        partial[i] =
            MultiExpInner::<{ multi_exp_method::multi_exp_method_BDLO12 }>::multi_exp_inner::<
                T,
                FieldT,
                0,
            >(
                &vec[i * one..vec.len().min((i + 1) * one)],
                &scalar[i * one..scalar.len().min((i + 1) * one)],
            );
        //      multi_exp_inner<T, FieldT, Method>(
        //  vec_start + i*one,
        //  if i == chunks-1 {vec_end} else{vec_start + (i+1)*one},
        //  scalar_start + i*one,
        //  if i == chunks-1 {scalar_end} else{scalar_start + (i+1)*one)};
    }

    let mut finals = T::zero();

    for i in 0..chunks {
        finals = finals + partial[i].clone();
    }

    return finals;
}
use crate::common::profiling::print_indent;
pub fn multi_exp_with_mixed_addition<
    T: PpConfig,
    FieldT: PpConfig,
    const Method: multi_exp_method,
>(
    vec: &[T],
    scalar: &[FieldT],
    chunks: usize,
) -> T {
    // //#ifndef NDEBUG
    // assert!(std::distance(vec_start, vec_end) == std::distance(scalar_start, scalar_end));
    // #else
    // UNUSED(vec_end);
    // //#endif
    enter_block("Process scalar vector", false);
    // auto value_it = vec_start;
    // auto scalar_it = scalar_start;

    let zero = FieldT::zero();
    let one = FieldT::one();
    let mut p: Vec<FieldT> = vec![];
    let mut g = vec![];

    let acc = T::zero();

    let mut num_skip = 0;
    let mut num_add = 0;
    let mut num_other = 0;

    for (v, s) in vec.iter().zip(scalar) {
        if *s == zero {
            // do nothing
            num_skip += 1;
        } else if *s == one {
            // // #ifdef USE_MIXED_ADDITION
            // acc = acc.mixed_add(*value_it);
            // #else
            // acc = acc + (*value_it);
            // //#endif
            num_add += 1;
        } else {
            p.push(s.clone());
            g.push(v.clone());
            num_other += 1;
        }
    }

    if !inhibit_profiling_info {
        print_indent();
        print!(
            "* Elements of w skipped: {} {}\n",
            num_skip,
            100 * num_skip / (num_skip + num_add + num_other)
        );
        print_indent();
        print!(
            "* Elements of w processed with special addition: {} {}\n",
            num_add,
            100 * num_add / (num_skip + num_add + num_other)
        );
        print_indent();
        print!(
            "* Elements of w remaining: {} {}\n",
            num_other,
            100 * num_other / (num_skip + num_add + num_other)
        );
    }

    leave_block("Process scalar vector", false);

    return acc + multi_exp::<T, FieldT, Method>(&g, &p, chunks);
}

pub fn inner_product<T: PpConfig>(a: &[T], b: &[T]) -> T {
    return multi_exp::<T, T, { multi_exp_method::multi_exp_method_naive_plain }>(a, b, 1);
}

pub fn get_exp_window_size<T: PpConfig>(num_scalars: usize) -> usize {
    if T::fixed_base_exp_window_table().is_empty() {
        // // #ifdef LOWMEM
        // return 14;
        // #else
        // return 17;
        // //#endif
    }
    let mut window = 1;
    for i in (0..T::fixed_base_exp_window_table().len()).rev() {
        // // #ifdef DEBUG
        // if !inhibit_profiling_info
        // {
        // print!("%ld {} {}\n", i, num_scalars, T::fixed_base_exp_window_table[i]);
        // }
        // //#endif
        if T::fixed_base_exp_window_table()[i] != 0
            && num_scalars >= T::fixed_base_exp_window_table()[i]
        {
            window = i + 1;
            break;
        }
    }

    if !inhibit_profiling_info {
        print_indent();
        print!(
            "Choosing window size {} for {} elements\n",
            window, num_scalars
        );
    }

    // // #ifdef LOWMEM
    // window = std::min((usize)14, window);
    // //#endif
    return window;
}

pub fn get_window_table<T: PpConfig>(scalar_size: usize, window: usize, g: &T) -> window_table<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    let mut in_window = 1usize << window;
    let mut outerc = (scalar_size + window - 1) / window;
    let mut last_in_window = 1usize << (scalar_size - (outerc - 1) * window);
    // // #ifdef DEBUG
    // if !inhibit_profiling_info
    // {
    // print_indent(); print!("* scalar_size={}; window={}; in_window={}; outerc={}\n", scalar_size, window, in_window, outerc);
    // }
    // //#endif

    let mut powers_of_g = vec![vec![T::zero(); in_window]; outerc];

    let mut gouter = g.clone();

    for outer in 0..outerc {
        let mut ginner = T::zero();
        let mut cur_in_window = if outer == outerc - 1 {
            last_in_window
        } else {
            in_window
        };
        for inner in 0..cur_in_window as usize {
            powers_of_g[outer][inner] = ginner.clone();
            ginner = ginner + gouter.clone();
        }

        for i in 0..window {
            gouter = gouter.clone() + gouter;
        }
    }

    return powers_of_g;
}

pub fn windowed_exp<T: PpConfig, FieldT: PpConfig, const NN: usize>(
    scalar_size: usize,
    window: usize,
    powers_of_g: &window_table<T>,
    pow: &FieldT,
) -> T {
    let mut outerc = (scalar_size + window - 1) / window;
    let pow_val = pow.as_bigint::<{ NN }>();

    /* exp */
    let mut res = powers_of_g[0][0].clone();

    for outer in 0..outerc {
        let mut inner = 0;
        for i in 0..window {
            if pow_val.test_bit(outer * window + i) {
                inner |= 1u32 << i;
            }
        }

        res = res + powers_of_g[outer][inner as usize].clone();
    }

    return res;
}

pub fn batch_exp<T: PpConfig, FieldT: PpConfig, const NN: usize>(
    scalar_size: usize,
    window: usize,
    table: &window_table<T>,
    v: &Vec<FieldT>,
) -> Vec<T> {
    if !inhibit_profiling_info {
        print_indent();
    }
    let mut res = vec![table[0][0].clone(); v.len()];

    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    for i in 0..v.len() {
        res[i] = windowed_exp::<_, _, { NN }>(scalar_size, window, table, &v[i]);

        if !inhibit_profiling_info && (i % 10000 == 0) {
            print!(".");
            // fflush(stdout);
        }
    }

    if !inhibit_profiling_info {
        print!(" DONE!\n");
    }

    return res;
}

pub fn batch_exp_with_coeff<T: PpConfig, FieldT: PpConfig, const NN: usize>(
    scalar_size: usize,
    window: usize,
    table: &window_table<T>,
    coeff: &FieldT,
    v: &Vec<FieldT>,
) -> Vec<T> {
    if !inhibit_profiling_info {
        print_indent();
    }
    let mut res = vec![table[0][0].clone(); v.len()];

    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    for i in 0..v.len() {
        res[i] = windowed_exp::<_, _, { NN }>(scalar_size, window, table, &(v[i].clone() * coeff));

        if !inhibit_profiling_info && (i % 10000 == 0) {
            print!(".");
            // fflush(stdout);
        }
    }

    if !inhibit_profiling_info {
        print!(" DONE!\n");
    }

    return res;
}

pub fn batch_to_special<T: PpConfig>(vec: &mut Vec<T>) {
    enter_block("Batch-convert elements to special form", false);

    let mut non_zero_vec = vec![];
    for i in 0..vec.len() {
        if !vec[i].is_zero() {
            non_zero_vec.push(vec[i].clone());
        }
    }

    T::batch_to_special_all_non_zeros(non_zero_vec.clone());
    let mut it = non_zero_vec.iter();
    let mut zero_special = T::zero();
    zero_special.to_special();

    for i in 0..vec.len() {
        if !vec[i].is_zero() {
            vec[i] = it.next().unwrap().clone();
        } else {
            vec[i] = zero_special.clone();
        }
    }
    leave_block("Batch-convert elements to special form", false);
}

// struct Item<const I: u8>;
// #[derive(Debug)] struct A;
// #[derive(Debug)] struct B;
// #[derive(Debug)] struct C;

// impl Item<0> { fn foo() -> A { A } }
// impl Item<1> { fn foo() -> B { B } }
// impl Item<2> { fn foo() -> C { C } }

// const fn check(i: i32) -> u8 {
//     match i {
//         0   => 0,
//         1.. => 1,
//         _   => 2,
//     }
// }

// fn main() {
//     dbg!(Item::<{ check(0) }>::foo(),   // A
//          Item::<{ check(1) }>::foo(),   // B
//          Item::<{ check(-1) }>::foo()); // C
// }

// struct Guard<const U: bool>;
// trait Protect {}
// impl Protect for Guard<true> {}

// fn main() {
//    f::<0>()
// }

// fn f<const N: usize>()
// where
//    Guard<{
//        const fn _f_guard<const N: usize>() -> bool {
//            if !N > 0 {
//                panic!("guard evaluated to false")
//            }
//            true
//        }
//        _f_guard::<N>()
//    }>: Protect,
// {
//    todo!()
// }

// #![feature(adt_const_params)]

// #[derive(ConstParamTy, PartialEq, Eq)]
// enum MyEnum {
//     VariantA,
//     VariantB,
// }

// struct MyStruct<const V: MyEnum> {
//     // ... fields
// }

// fn main() {
//     let _instance_a = MyStruct::<{ MyEnum::VariantA }>;
//     let _instance_b = MyStruct::<{ MyEnum::VariantB }>;
// }
