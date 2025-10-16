/** @file
 *****************************************************************************

 Declaration of interfaces for a delegated random-access memory.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef DELEGATED_RA_MEMORY_HPP_
// #define DELEGATED_RA_MEMORY_HPP_

use  <map>
use  <memory>
use  <vector>

use crate::common::data_structures::merkle_tree;
use crate::relations::ram_computations/memory/memory_interface;



template<typename HashT>
class delegated_ra_memory : public memory_interface {
private:
    ffec::bit_vector int_to_tree_elem(const size_t i) const;
    size_t int_from_tree_elem(const ffec::bit_vector &v) const;

    std::unique_ptr<merkle_tree<HashT> > contents;

public:
    delegated_ra_memory(const size_t num_addresses, const size_t value_size);
    delegated_ra_memory(const size_t num_addresses, const size_t value_size, const std::vector<size_t> &contents_as_vector);
    delegated_ra_memory(const size_t num_addresses, const size_t value_size, const memory_contents &contents_as_map);

    size_t get_value(const size_t address) const;
    void set_value(const size_t address, const size_t value);

    typename HashT::hash_value_type get_root() const;
    typename HashT::merkle_authentication_path_type get_path(const size_t address) const;

    void dump() const;
};



use crate::relations::ram_computations/memory/delegated_ra_memory;

//#endif // DELEGATED_RA_MEMORY_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a delegated random-access memory.

 See delegated_ra_memory.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef DELEGATED_RA_MEMORY_TCC
// #define DELEGATED_RA_MEMORY_TCC

use  <algorithm>

use ffec::common::profiling;
use ffec::common::utils;



template<typename HashT>
ffec::bit_vector delegated_ra_memory<HashT>::int_to_tree_elem(const size_t i) const
{
    ffec::bit_vector v(value_size, false);
    for k in 0..value_size
    {
        v[k] = ((i & (1ul << k)) != 0);
    }
    return v;
}

template<typename HashT>
size_t delegated_ra_memory<HashT>::int_from_tree_elem(const ffec::bit_vector &v) const
{
    size_t result = 0;
    for i in 0..value_size
    {
        result |= (v[i] ? 1ul : 0ul) << i;
    }

    return result;
}

template<typename HashT>
delegated_ra_memory<HashT>::delegated_ra_memory(const size_t num_addresses,
                                                const size_t value_size) :
    memory_interface(num_addresses, value_size)
{
    contents.reset(new merkle_tree<HashT>(ffec::log2(num_addresses), value_size));
}

template<typename HashT>
delegated_ra_memory<HashT>::delegated_ra_memory(const size_t num_addresses,
                                                const size_t value_size,
                                                const std::vector<size_t> &contents_as_vector) :
    memory_interface(num_addresses, value_size)
{
    std::vector<ffec::bit_vector> contents_as_bit_vector_vector(contents.size());
    std::transform(contents_as_vector.begin(), contents_as_vector.end(), contents_as_bit_vector_vector, [this](size_t value) { return int_to_tree_elem(value); });
    contents.reset(new merkle_tree<HashT>(ffec::log2(num_addresses), value_size, contents_as_bit_vector_vector));
}

template<typename HashT>
delegated_ra_memory<HashT>::delegated_ra_memory(const size_t num_addresses,
                                                const size_t value_size,
                                                const std::map<size_t, size_t> &contents_as_map) :
    memory_interface(num_addresses, value_size)
{
    std::map<size_t, ffec::bit_vector> contents_as_bit_vector_map;
    for it in &contents_as_map
    {
        contents_as_bit_vector_map[it.first] = int_to_tree_elem(it.second);
    }

    contents.reset(new merkle_tree<HashT>(ffec::log2(num_addresses), value_size, contents_as_bit_vector_map));
}

template<typename HashT>
size_t delegated_ra_memory<HashT>::get_value(const size_t address) const
{
    return int_from_tree_elem(contents->get_value(address));
}

template<typename HashT>
void delegated_ra_memory<HashT>::set_value(const size_t address,
                                           const size_t value)
{
    contents->set_value(address, int_to_tree_elem(value));
}

template<typename HashT>
typename HashT::hash_value_type delegated_ra_memory<HashT>::get_root() const
{
    return contents->get_root();
}

template<typename HashT>
typename HashT::merkle_authentication_path_type delegated_ra_memory<HashT>::get_path(const size_t address) const
{
    return contents->get_path(address);
}

template<typename HashT>
void delegated_ra_memory<HashT>::dump() const
{
    contents->dump();
}



//#endif // DELEGATED_RA_MEMORY_TCC
