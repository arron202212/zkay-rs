/** @file
*****************************************************************************

Declaration of interfaces for functions to sample examples of memory contents.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef MEMORY_CONTENTS_EXAMPLES_HPP_
// #define MEMORY_CONTENTS_EXAMPLES_HPP_
use crate::relations::ram_computations::memory::memory_interface::memory_contents;
use std::collections::BTreeSet;
// /**
//  * Sample memory contents consisting of two blocks of random values;
//  * the first block is located at the beginning of memory, while
//  * the second block is located half-way through memory.
//  */
// memory_contents block_memory_contents(num_addresses:usize,
//                                       value_size:usize,
//                                       block1_size:usize,
//                                       block2_size:usize);

// /**
//  * Sample memory contents having a given number of non-zero entries;
//  * each non-zero entry is a random value at a random address (approximately).
//  */
// memory_contents random_memory_contents(num_addresses:usize,
//                                        value_size:usize,
//                                        num_filled:usize);

//#endif // MEMORY_CONTENTS_EXAMPLES_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for functions to sample examples of memory contents.

See memory_contents_examples.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// use  <cstdlib>
// use  <map>
// use  <set>

// use crate::relations::ram_computations::memory::examples::memory_contents_examples;

pub fn block_memory_contents(
    num_addresses: usize,
    value_size: usize,
    block1_size: usize,
    block2_size: usize,
) -> memory_contents {
    let max_unit = 1usize << value_size;

    let mut result = memory_contents::new();
    for i in 0..block1_size {
        result.insert(i, rand::random::<usize>() % max_unit);
    }

    for i in 0..block2_size {
        result.insert(num_addresses / 2 + i, rand::random::<usize>() % max_unit);
    }

    return result;
}

pub fn random_memory_contents(
    num_addresses: usize,
    value_size: usize,
    num_filled: usize,
) -> memory_contents {
    let mut max_unit = 1usize << value_size;

    let mut unfilled = BTreeSet::new();
    for i in 0..num_addresses {
        unfilled.insert(i);
    }

    let mut result = memory_contents::new();
    for i in 0..num_filled {
        let mut it = unfilled.iter();
        // it.advance_by(rand::random::<usize>() % unfilled.len());
        let j = *it.next().unwrap();
        result.insert(j, rand::random::<usize>() % max_unit);
        unfilled.remove(&j);
    }

    return result;
}
