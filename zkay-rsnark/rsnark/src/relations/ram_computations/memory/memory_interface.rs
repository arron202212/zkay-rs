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
// 



/**
 * A function from addresses to values that represents a memory's contents.
 */
type memory_contents=std::collectons::BTreeMap<usize, usize> ;

/**
 * A memory interface is a virtual pub struct for specifying and maintaining a memory.
 *
 * A memory is parameterized by two quantities:
 * - num_addresses (which specifies the number of addresses); and
 * - value_size (which specifies the number of bits stored at each address).
 *
 * The methods get_val and set_val can be used to load and store values.
 */
pub struct memory_interface {
// 

    num_addresses:usize,
    value_size:usize,
}
impl memory_interface {
    pub fn new(num_addresses:usize, value_size:usize,) ->Self
    {
    Self{num_addresses,
        value_size}
}
    // memory_interface(contents_as_vector:&usize num_addresses, value_size:usize, const Vec<usize>);
    // memory_interface(contents:&usize num_addresses, value_size:usize, const memory_contents);

    // virtual usize get_value(address:usize) 0:=,
    // virtual pub fn  set_value(address:usize, value:usize) = 0;
}



//#endif // MEMORY_INTERFACE_HPP_

