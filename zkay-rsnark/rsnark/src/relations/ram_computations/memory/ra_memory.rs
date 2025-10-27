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
// //: public memory_interface 

     contents:memory_contents,
}
//     ra_memory(num_addresses:usize, value_size:usize);
//     ra_memory(num_addresses:usize, value_size:usize, contents_as_vector:&Vec<usize>);
//     ra_memory(num_addresses:usize, value_size:usize, contents:&memory_contents);

//     usize get_value(address:usize) const;
//     pub fn  set_value(address:usize, value:usize);

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

// use crate::relations::ram_computations::memory::ra_memory;


impl ra_memory{
pub fn new(num_addresses:usize, value_size:usize) ->Self
{
// memory_interface(num_addresses, value_size)
    Self{}
}

pub fn new2(num_addresses:usize,
                     value_size:usize,
                     contents_as_vector:&Vec<usize>)  ->Self
    
{
// memory_interface(num_addresses, value_size)
    /* copy Vec into BTreeMap */
    for i in 0..contents_as_vector.len()
    {
        contents[i] = contents_as_vector[i];
    }
}


pub fn new3(num_addresses:usize,
                     value_size:usize,
                     contents:&memory_contents) ->Self
    
{
// memory_interface(num_addresses, value_size),contents
}

pub fn get_value(address:usize) ->usize
{
    assert!(address < num_addresses);
    if let Some( it) = contents.find(address){
    it.1}else{0}
    // return if it == contents.end() {0} else{it.1};
}

pub fn set_value(address:usize, value:usize)
{
    contents[address] = value;
}


}