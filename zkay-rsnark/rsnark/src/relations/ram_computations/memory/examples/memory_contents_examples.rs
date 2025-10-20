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

use crate::relations::ram_computations::memory::memory_interface;



// /**
//  * Sample memory contents consisting of two blocks of random values;
//  * the first block is located at the beginning of memory, while
//  * the second block is located half-way through memory.
//  */
// memory_contents block_memory_contents(const size_t num_addresses,
//                                       const size_t value_size,
//                                       const size_t block1_size,
//                                       const size_t block2_size);

// /**
//  * Sample memory contents having a given number of non-zero entries;
//  * each non-zero entry is a random value at a random address (approximately).
//  */
// memory_contents random_memory_contents(const size_t num_addresses,
//                                        const size_t value_size,
//                                        const size_t num_filled);



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



pub fn  block_memory_contents(num_addresses:size_t,
                                      value_size:size_t,
                                      block1_size:size_t,
                                      block2_size:size_t)->memory_contents
{
    let  max_unit = 1u64<<value_size;

    let mut  result=memory_contents::new();
    for i in 0..block1_size
    {
        result[i] = std::rand() % max_unit;
    }

    for i in 0..block2_size
    {
        result[num_addresses/2+i] = std::rand() % max_unit;
    }

    return result;
}

pub fn  random_memory_contents(num_addresses:size_t,
                                       value_size:size_t,
                                       num_filled:size_t)->memory_contents
{
    let mut  max_unit = 1u64<<value_size;

    let mut  unfilled=BTreeSet::new();
    for i in 0..num_addresses
    {
        unfilled.insert(i);
    }
      use rand::Rng;
    let mut rng = rand::thread_rng();
     let mut  result=memory_contents::new();
    for i in 0..num_filled
    {
        let mut  it = unfilled.iter();
        it.advance_by(rng::r#gen::<usize>() % unfilled.size());
        result[it.next().unwrap()] = rng::r#gen::<usize>()% max_unit;
        unfilled.remove(it);
    }

    return result;
}


