/** @file
*****************************************************************************

Functions to profile the algorithms that route on Benes and AS-Waksman networks.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// use  <algorithm>
use ffec::common::profiling;

use crate::common::routing_algorithms::as_waksman_routing_algorithm;
use crate::common::routing_algorithms::benes_routing_algorithm;

// pub fn  profile_benes_algorithm(n:usize)
// {
//     print!("* Size: {}\n", n);

//     assert!(n == 1u64<<ffec::log2(n));

//     ffec::enter_block("Generate permutation");
//     integer_permutation permutation(n);
//     permutation.random_shuffle();
//     ffec::leave_block("Generate permutation");

//     ffec::enter_block("Generate Benes routing assignment");
//     let routing= get_benes_routing(permutation);
//     ffec::leave_block("Generate Benes routing assignment");
// }

// pub fn  profile_as_waksman_algorithm(n:usize)
// {
//     print!("* Size: {}\n", n);

//     ffec::enter_block("Generate permutation");
//     integer_permutation permutation(n);
//     permutation.random_shuffle();
//     ffec::leave_block("Generate permutation");

//     ffec::enter_block("Generate AS-Waksman routing assignment");
//     let routing= get_as_waksman_routing(permutation);
//     ffec::leave_block("Generate AS-Waksman routing assignment");
// }

// int main()
// {
//     ffec::start_profiling();

//     for (usize n = 1u64<<10; n <= 1u64<<20; n <<= 1)
//     {
//         profile_benes_algorithm(n);
//     }

//     for (usize n = 1u64<<10; n <= 1u64<<20; n <<= 1)
//     {
//         profile_as_waksman_algorithm(n);
//     }
// }
