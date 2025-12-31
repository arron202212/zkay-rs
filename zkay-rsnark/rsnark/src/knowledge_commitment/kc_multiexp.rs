/*
  Split out from multiexp to prevent cyclical
  dependencies. I.e. previously multiexp dependend on
  knowledge_commitment, which dependend on sparse_vector, which
  dependend on multiexp (to do accumulate).

  Will probably go away in more general exp refactoring.
*/
use crate::knowledge_commitment::knowledge_commitment::{
    knowledge_commitment, knowledge_commitment_vector,
};
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::field_utils::bigint::bigint;
use ffec::scalar_multiplication::multiexp::{
    batch_to_special, inhibit_profiling_info, multi_exp, multi_exp_method, window_table,
    windowed_exp,
};
use ffec::scalar_multiplication::wnaf::opt_window_wnaf_exp;
use ffec::{One, Zero};

pub fn opt_window_wnaf_exps<T1: PpConfig, T2: PpConfig, FieldT, const N: usize>(
    base: &knowledge_commitment<T1, T2>,
    scalar: &bigint<N>,
    scalar_bits: usize,
) -> knowledge_commitment<T1, T2> {
    knowledge_commitment::<T1, T2>::new(
        opt_window_wnaf_exp(&base.g, scalar, scalar_bits),
        opt_window_wnaf_exp(&base.h, scalar, scalar_bits),
    )
}

pub fn kc_multi_exp_with_mixed_addition<
    T1: PpConfig,
    T2: PpConfig,
    FieldT: FieldTConfig,
    const Method: multi_exp_method,
>(
    vec: &knowledge_commitment_vector<T1, T2>,
    min_idx: usize,
    max_idx: usize,
    scalar: &[FieldT],
    chunks: usize,
) -> knowledge_commitment<T1, T2> {
    enter_block("Process scalar vector", false);
    let index_it = vec.indices.partition_point(|&i| i < min_idx);
    let offset = index_it;
    let value_it = &vec.values[offset];

    let zero = FieldT::zero();
    let one = FieldT::one();

    let mut p = vec![];
    let mut g = vec![];

    let mut acc = knowledge_commitment::<T1, T2>::zero();

    let mut num_skip = 0;
    let mut num_add = 0;
    let mut num_other = 0;

    let scalar_length = scalar.len();

    for (&index_it, v) in vec.indices.iter().zip(&vec.values).skip(offset) {
        if index_it >= max_idx {
            break;
        }

        let scalar_position = index_it - min_idx;
        assert!(scalar_position < scalar_length);

        let scalar = FieldT::from(scalar_position);

        if scalar == zero {
            // do nothing
            num_skip += 1;
        } else if scalar == one {
            // #ifdef USE_MIXED_ADDITION
            acc.g = acc.g.mixed_add(&value_it.g);
            acc.h = acc.h.mixed_add(&value_it.h);
            // #else
            // acc.g = acc.g + value_it->g;
            // acc.h = acc.h + value_it->h;
            //#endif
            num_add += 1;
        } else {
            p.push(scalar.clone());
            g.push(value_it.clone());
            num_other += 1;
        }

        // index_it+=1;
        // value_it+=1;
    }

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
    leave_block("Process scalar vector", false);

    acc + multi_exp::<knowledge_commitment<T1, T2>, FieldT, Method>(&g, &p, chunks)
}

pub fn kc_batch_exp_internal<T1: PpConfig, T2: PpConfig, FieldT: FieldTConfig, const NN: usize>(
    scalar_size: usize,
    T1_window: usize,
    T2_window: usize,
    T1_table: &window_table<T1>,
    T2_table: &window_table<T2>,
    T1_coeff: &FieldT,
    T2_coeff: &FieldT,
    v: &Vec<FieldT>,
    start_pos: usize,
    end_pos: usize,
    expected_size: usize,
) -> knowledge_commitment_vector<T1, T2> {
    let mut res = knowledge_commitment_vector::<T1, T2>::default();

    res.values.reserve(expected_size);
    res.indices.reserve(expected_size);

    for pos in (start_pos..end_pos) {
        if !v[pos].is_zero() {
            res.values.push(knowledge_commitment::<T1, T2>::new(
                windowed_exp::<T1, FieldT, NN>(
                    scalar_size,
                    T1_window,
                    T1_table,
                    &(v[pos].clone() * T1_coeff),
                ),
                windowed_exp::<T2, FieldT, NN>(
                    scalar_size,
                    T2_window,
                    T2_table,
                    &(v[pos].clone() * T2_coeff),
                ),
            ));
            res.indices.push(pos);
        }
    }

    res
}

pub fn kc_batch_exp<T1: PpConfig, T2: PpConfig, FieldT: FieldTConfig, const NN: usize>(
    scalar_size: usize,
    T1_window: usize,
    T2_window: usize,
    T1_table: &window_table<T1>,
    T2_table: &window_table<T2>,
    T1_coeff: &FieldT,
    T2_coeff: &FieldT,
    v: &Vec<FieldT>,
    suggested_num_chunks: usize,
) -> knowledge_commitment_vector<T1, T2> {
    let mut res = knowledge_commitment_vector::<T1, T2>::default();
    res.domain_size_ = v.len();

    let mut nonzero = 0;
    for i in 0..v.len() {
        nonzero += if v[i].is_zero() { 0 } else { 1 };
    }

    let num_chunks = 1usize.max(nonzero.min(suggested_num_chunks));

    if !inhibit_profiling_info {
        print_indent();
        print!(
            "Non-zero coordinate count: {}/{} {}\n",
            nonzero,
            v.len(),
            100 * nonzero / v.len()
        );
    }

    let mut tmp = vec![knowledge_commitment_vector::<T1, T2>::default(); num_chunks];
    let mut chunk_pos = vec![0; num_chunks + 1];

    let chunk_size = nonzero / num_chunks;
    let last_chunk = nonzero - chunk_size * (num_chunks - 1);

    chunk_pos[0] = 0;

    let mut cnt = 0;
    let mut chunkno = 1;

    for i in 0..v.len() {
        cnt += if v[i].is_zero() { 0 } else { 1 };
        if cnt == chunk_size && chunkno < num_chunks {
            chunk_pos[chunkno] = i;
            cnt = 0;
            chunkno += 1;
        }
    }

    chunk_pos[num_chunks] = v.len();

    // #ifdef MULTICORE
    //#pragma omp parallel for
    //#endif
    for i in 0..num_chunks {
        tmp[i] = kc_batch_exp_internal::<T1, T2, FieldT, NN>(
            scalar_size,
            T1_window,
            T2_window,
            T1_table,
            T2_table,
            T1_coeff,
            T2_coeff,
            v,
            chunk_pos[i],
            chunk_pos[i + 1],
            if i == num_chunks - 1 {
                last_chunk
            } else {
                chunk_size
            },
        );
        // #ifdef USE_MIXED_ADDITION
        batch_to_special::<knowledge_commitment<T1, T2>>(&mut tmp[i].values.clone());
        //#endif
    }

    if num_chunks == 1 {
        tmp[0].domain_size_ = v.len();
        return tmp[0].clone();
    }

    for i in 0..num_chunks {
        res.values.extend(tmp[i].values.clone());
        res.indices.extend(tmp[i].indices.clone());
    }
    res
}
