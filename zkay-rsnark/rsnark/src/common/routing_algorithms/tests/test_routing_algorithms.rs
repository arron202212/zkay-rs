//  Functions to test the algorithms that route on Benes and AS-Waksman networks.

use crate::common::data_structures::integer_permutation::integer_permutation;
use crate::common::routing_algorithms::as_waksman_routing_algorithm::{
    get_as_waksman_routing, valid_as_waksman_routing,
};
use crate::common::routing_algorithms::benes_routing_algorithm::{
    get_benes_routing, valid_benes_routing,
};
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use ffec::log2;
use tracing::{Level, span};

// /**
//  * Test Benes network routing for all permutations on 2^log2(N) elements.
//  */
pub fn test_benes(N: usize) {
    let mut permutation = integer_permutation::new(1usize << log2(N));

    loop {
        let routing = get_benes_routing(&permutation);
        assert!(valid_benes_routing(&permutation, &routing));
        if !permutation.next_permutation() {
            break;
        }
    }
}

// /**
//  * Test AS-Waksman network routing for all permutations on N elements.
//  */
pub fn test_as_waksman(N: usize) {
    let mut permutation = integer_permutation::new(N);

    loop {
        let routing = get_as_waksman_routing(&permutation);
        assert!(valid_as_waksman_routing(&permutation, &routing));
        if !permutation.next_permutation() {
            break;
        }
    }
}

pub fn main() -> i32 {
    start_profiling();

    let span0 = span!(Level::TRACE, "Test routing algorithms");
    let _ = span0.enter();

    let span1 = span!(Level::TRACE, "Test Benes network routing algorithm").entered();
    let bn_size = 8;
    print_indent();
    print!("* for all permutations on {} elements\n", bn_size);
    test_benes(bn_size);
    span1.exit();

    let span = span!(Level::TRACE, "Test AS-Waksman network routing algorithm").entered();
    let asw_max_size = 9;
    for i in 2..=asw_max_size {
        print_indent();
        print!("* for all permutations on {} elements\n", i);
        test_as_waksman(i);
    }
    span.exit();

    0
}
