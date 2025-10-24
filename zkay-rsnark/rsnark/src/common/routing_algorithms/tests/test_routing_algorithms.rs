/** @file
 *****************************************************************************

 Functions to test the algorithms that route on Benes and AS-Waksman networks.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <cassert>

use ffec::common::profiling;

use crate::common::routing_algorithms::as_waksman_routing_algorithm;
use crate::common::routing_algorithms::benes_routing_algorithm;



/**
 * Test Benes network routing for all permutations on 2^ffec::log2(N) elements.
 */
void test_benes(const size_t N)
{
    integer_permutation permutation(1u64 << ffec::log2(N));

    do {
        const benes_routing routing = get_benes_routing(permutation);
        assert!(valid_benes_routing(permutation, routing));
    } while (permutation.next_permutation());
}

/**
 * Test AS-Waksman network routing for all permutations on N elements.
 */
void test_as_waksman(const size_t N)
{
    integer_permutation permutation(N);

    do {
        const as_waksman_routing routing = get_as_waksman_routing(permutation);
        assert!(valid_as_waksman_routing(permutation, routing));
    } while (permutation.next_permutation());
}

int main(void)
{
    ffec::start_profiling();

    ffec::enter_block("Test routing algorithms");

    ffec::enter_block("Test Benes network routing algorithm");
    size_t bn_size = 8;
    ffec::print_indent(); print!("* for all permutations on {} elements\n", bn_size);
    test_benes(bn_size);
    ffec::leave_block("Test Benes network routing algorithm");


    ffec::enter_block("Test AS-Waksman network routing algorithm");
    size_t asw_max_size = 9;
    for i in 2..=asw_max_size
    {
        ffec::print_indent(); print!("* for all permutations on {} elements\n", i);
        test_as_waksman(i);
    }
    ffec::leave_block("Test AS-Waksman network routing algorithm");

    ffec::leave_block("Test routing algorithms");
}
