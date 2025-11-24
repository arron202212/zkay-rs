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
pub type memory_contents = std::collections::BTreeMap<usize, usize>;

/**
 * A memory interface is a virtual pub struct for specifying and maintaining a memory.
 *
 * A memory is parameterized by two quantities:
 * - num_addresses (which specifies the number of addresses); and
 * - value_size (which specifies the number of bits stored at each address).
 *
 * The methods get_val and set_val can be used to load and store values.
 */

pub struct memory_base<T> {
    pub num_addresses: usize,
    pub value_size: usize,
    pub t: T,
}
impl<T> memory_base<T> {
    pub fn new(num_addresses: usize, value_size: usize, t: T) -> Self {
        Self {
            num_addresses,
            value_size,
            t,
        }
    }
    // memory_interface(num_addresses:usize , value_size:usize,  contents_as_vector:Vec<usize>);
    // memory_interface(num_addresses:usize, value_size:usize, contents: memory_contents);
}
pub trait memory_interface {
    fn get_value(&self, address: usize) -> usize;
    fn set_value(&mut self, address: usize, value: usize);
}

//#endif // MEMORY_INTERFACE_HPP_
