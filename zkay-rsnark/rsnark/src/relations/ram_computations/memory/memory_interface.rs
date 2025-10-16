/** @file
 *****************************************************************************

 Declaration of interfaces for a memory interface.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_INTERFACE_HPP_
// #define MEMORY_INTERFACE_HPP_

use  <cstddef>
use  <map>
use  <vector>



/**
 * A function from addresses to values that represents a memory's contents.
 */
type std::map<size_t, size_t> memory_contents;

/**
 * A memory interface is a virtual class for specifying and maintaining a memory.
 *
 * A memory is parameterized by two quantities:
 * - num_addresses (which specifies the number of addresses); and
 * - value_size (which specifies the number of bits stored at each address).
 *
 * The methods get_val and set_val can be used to load and store values.
 */
class memory_interface {
public:

    size_t num_addresses;
    size_t value_size;

    memory_interface(const size_t num_addresses, const size_t value_size) :
        num_addresses(num_addresses),
        value_size(value_size)
    {}
    memory_interface(const size_t num_addresses, const size_t value_size, const std::vector<size_t> &contents_as_vector);
    memory_interface(const size_t num_addresses, const size_t value_size, const memory_contents &contents);

    virtual size_t get_value(const size_t address) const = 0;
    virtual void set_value(const size_t address, const size_t value) = 0;
};



//#endif // MEMORY_INTERFACE_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a memory store trace.

 See memory_store_trace.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::relations::ram_computations/memory/memory_store_trace;



memory_store_trace::memory_store_trace()
{
}

address_and_value memory_store_trace::get_trace_entry(const size_t timestamp) const
{
    auto it = entries.find(timestamp);
    return (it != entries.end() ? it->second : std::make_pair<size_t, size_t>(0, 0));
}

std::map<size_t, address_and_value> memory_store_trace::get_all_trace_entries() const
{
    return entries;
}

void memory_store_trace::set_trace_entry(const size_t timestamp, const address_and_value &av)
{
    entries[timestamp] = av;
}

memory_contents memory_store_trace::as_memory_contents() const
{
    memory_contents result;

    for ts_and_addrval in &entries
    {
        result[ts_and_addrval.second.first] = ts_and_addrval.second.second;
    }

    return result;
}


