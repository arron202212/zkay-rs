/** @file
 *****************************************************************************

 Declaration of interfaces for functions to sample examples of memory contents.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef MEMORY_CONTENTS_EXAMPLES_HPP_
#define MEMORY_CONTENTS_EXAMPLES_HPP_

use  <libsnark/relations/ram_computations/memory/memory_interface.hpp>

namespace libsnark {

/**
 * Sample memory contents consisting of two blocks of random values;
 * the first block is located at the beginning of memory, while
 * the second block is located half-way through memory.
 */
memory_contents block_memory_contents(const size_t num_addresses,
                                      const size_t value_size,
                                      const size_t block1_size,
                                      const size_t block2_size);

/**
 * Sample memory contents having a given number of non-zero entries;
 * each non-zero entry is a random value at a random address (approximately).
 */
memory_contents random_memory_contents(const size_t num_addresses,
                                       const size_t value_size,
                                       const size_t num_filled);

} // libsnark

#endif // MEMORY_CONTENTS_EXAMPLES_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for functions to sample examples of memory contents.

 See memory_contents_examples.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <cstdlib>
use  <map>
use  <set>

use  <libsnark/relations/ram_computations/memory/examples/memory_contents_examples.hpp>

namespace libsnark {

memory_contents block_memory_contents(const size_t num_addresses,
                                      const size_t value_size,
                                      const size_t block1_size,
                                      const size_t block2_size)
{
    const size_t max_unit = 1ul<<value_size;

    memory_contents result;
    for (size_t i = 0; i < block1_size; ++i)
    {
        result[i] = std::rand() % max_unit;
    }

    for (size_t i = 0; i < block2_size; ++i)
    {
        result[num_addresses/2+i] = std::rand() % max_unit;
    }

    return result;
}

memory_contents random_memory_contents(const size_t num_addresses,
                                       const size_t value_size,
                                       const size_t num_filled)
{
    const size_t max_unit = 1ul<<value_size;

    std::set<size_t> unfilled;
    for (size_t i = 0; i < num_addresses; ++i)
    {
        unfilled.insert(i);
    }

    memory_contents result;
    for (size_t i = 0; i < num_filled; ++i)
    {
        auto it = unfilled.begin();
        std::advance(it, std::rand() % unfilled.size());
        result[*it] = std::rand() % max_unit;
        unfilled.erase(it);
    }

    return result;
}

} // libsnark
