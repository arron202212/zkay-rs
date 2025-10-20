/** @file
 *****************************************************************************

 Declaration of interfaces for a random-access memory.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RA_MEMORY_HPP_
// #define RA_MEMORY_HPP_

use crate::relations::ram_computations::memory::memory_interface;



/**
 * A random-access memory maintains the memory's contents via a map (from addresses to values).
 */
pub struct ra_memory {
// public://: public memory_interface 

     contents:memory_contents,
}
//     ra_memory(num_addresses:size_t, value_size:size_t);
//     ra_memory(num_addresses:size_t, value_size:size_t, contents_as_vector:&std::vector<size_t>);
//     ra_memory(num_addresses:size_t, value_size:size_t, contents:&memory_contents);

//     size_t get_value(address:size_t) const;
//     void set_value(address:size_t, value:size_t);

// };



//#endif // RA_MEMORY_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a random-access memory.

 See ra_memory.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  <cassert>

// use crate::relations::ram_computations/memory/ra_memory;


impl ra_memory{
pub fn new(num_addresses:size_t, value_size:size_t) ->Self
{
// memory_interface(num_addresses, value_size)
    Self{}
}

pub fn new2(num_addresses:size_t,
                     value_size:size_t,
                     contents_as_vector:&std::vector<size_t>)  ->Self
    
{
// memory_interface(num_addresses, value_size)
    /* copy std::vector into std::map */
    for i in 0..contents_as_vector.size()
    {
        contents[i] = contents_as_vector[i];
    }
}


pub fn new3(num_addresses:size_t,
                     value_size:size_t,
                     contents:&memory_contents) ->Self
    
{
// memory_interface(num_addresses, value_size), contents(contents)
}

pub fn get_value(address:size_t) ->usize
{
    assert!(address < num_addresses);
    if let Some( it) = contents.find(address){
    it.1}else{0}
    // return if it == contents.end() {0} else{it->second};
}

pub fn set_value(address:size_t, value:size_t)
{
    contents[address] = value;
}


}