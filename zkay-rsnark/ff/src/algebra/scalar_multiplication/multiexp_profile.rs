/**
 *****************************************************************************
 Implementation of interfaces for multi-exponentiation routines.

 See multiexp.hpp .
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#include <cstdio>
//#include <vector>

use crate::algebra::curves::bn128::bn128_pp;
use crate::algebra::scalar_multiplication::multiexp;
use crate::common::profiling;
use crate::common::rng;

using namespace libff;

using std::usize;

template <GroupT>
using run_result_t = std::pair<i64, Vec<GroupT> >;

template <T>
using test_instances_t = Vec<Vec<T> >;


test_instances_t<GroupT> generate_group_elements(usize count, usize size)
{
    // generating a random group element is expensive,
    // so for now we only generate a single one and repeat it
    test_instances_t<GroupT> result(count);

    for i in 0..count {
        GroupT x = GroupT::random_element();
        x.to_special(); // djb requires input to be in special form
        for j in 0..size {
            result[i].push_back(x);
            // result[i].push_back(GroupT::random_element());
        }
    }

    return result;
}


test_instances_t<FieldT> generate_scalars(usize count, usize size)
{
    // we use SHA512_rng because it is much faster than
    // FieldT::random_element()
    test_instances_t<FieldT> result(count);

    for i in 0..count {
        for j in 0..size {
            result[i].push_back(SHA512_rng<FieldT>(i * size + j));
        }
    }

    return result;
}


run_result_t<GroupT> profile_multiexp(
    test_instances_t<GroupT> group_elements,
    test_instances_t<FieldT> scalars)
{
    i64 start_time = get_nsec_time();

    Vec<GroupT> answers;
    for i in 0..group_elements.len() {
        answers.push_back(multi_exp<GroupT, FieldT, Method>(
            group_elements[i].cbegin(), group_elements[i].cend(),
            scalars[i].cbegin(), scalars[i].cend(),
            1));
    }

    i64 time_delta = get_nsec_time() - start_time;

    return run_result_t<GroupT>(time_delta, answers);
}


pub fn  print_performance_csv(
    usize expn_start,
    usize expn_end_fast,
    usize expn_end_naive,
    bool compare_answers)
{
    for expn in expn_start..=expn_end_fast {
        print!("%ld", expn); fflush(stdout);

        test_instances_t<GroupT> group_elements =
            generate_group_elements<GroupT>(10, 1 << expn);
        test_instances_t<FieldT> scalars =
            generate_scalars<FieldT>(10, 1 << expn);

        run_result_t<GroupT> result_bos_coster =
            profile_multiexp<GroupT, FieldT, multi_exp_method_bos_coster>(
                group_elements, scalars);
        print!("\t%lld", result_bos_coster.first); fflush(stdout);

        run_result_t<GroupT> result_djb =
            profile_multiexp<GroupT, FieldT, multi_exp_method_BDLO12>(
                group_elements, scalars);
        print!("\t%lld", result_djb.first); fflush(stdout);

        if compare_answers && (result_bos_coster.second != result_djb.second) {
            fprintf(stderr, "Answers NOT MATCHING (bos coster != djb)\n");
        }

        if expn <= expn_end_naive {
            run_result_t<GroupT> result_naive =
                profile_multiexp<GroupT, FieldT, multi_exp_method_naive>(
                    group_elements, scalars);
            print!("\t%lld", result_naive.first); fflush(stdout);

            if compare_answers && (result_bos_coster.second != result_naive.second) {
                fprintf(stderr, "Answers NOT MATCHING (bos coster != naive)\n");
            }
        }

        print!("\n");
    }
}

int main()
{
    print_compilation_info();

    print!("Profiling BN128_G1\n");
    bn128_pp::init_public_params();
    print_performance_csv<G1<bn128_pp>, Fr<bn128_pp> >(2, 20, 14, true);

    print!("Profiling BN128_G2\n");
    print_performance_csv<G2<bn128_pp>, Fr<bn128_pp> >(2, 20, 14, true);

    return 0;
}
