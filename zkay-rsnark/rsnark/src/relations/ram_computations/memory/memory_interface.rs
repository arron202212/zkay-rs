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

// use  <cstddef>
// use  <map>
// use  <vector>



/**
 * A function from addresses to values that represents a memory's contents.
 */
type memory_contents=std::collectons::BTreeMap<size_t, size_t> ;

/**
 * A memory interface is a virtual class for specifying and maintaining a memory.
 *
 * A memory is parameterized by two quantities:
 * - num_addresses (which specifies the number of addresses); and
 * - value_size (which specifies the number of bits stored at each address).
 *
 * The methods get_val and set_val can be used to load and store values.
 */
pub struct memory_interface {
// 

    num_addresses:size_t,
    value_size:size_t,
}
impl memory_interface {
    pub fn new(num_addresses:size_t, value_size:size_t,) ->Self
    {
    Self{num_addresses,
        value_size}
}
    // memory_interface(const size_t num_addresses, const size_t value_size, const std::vector<size_t> &contents_as_vector);
    // memory_interface(const size_t num_addresses, const size_t value_size, const memory_contents &contents);

    // virtual size_t get_value(const size_t address) const = 0;
    // virtual void set_value(const size_t address, const size_t value) = 0;
}



//#endif // MEMORY_INTERFACE_HPP_

