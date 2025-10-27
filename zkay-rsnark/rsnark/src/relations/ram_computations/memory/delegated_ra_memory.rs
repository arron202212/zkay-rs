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

// use  <map>
// 
// 

use crate::common::data_structures::merkle_tree;
use crate::relations::ram_computations::memory::memory_interface;



// 
pub struct delegated_ra_memory <HashT>{
// : public memory_interface 
    // bit_vector int_to_tree_elem(i:&usize) const;
    // usize int_from_tree_elem(v:&bit_vector) const;

     contents:RcCell<merkle_tree<HashT> >,
}

// 
//     delegated_ra_memory(num_addresses:&usize, value_size:&usize);
//     delegated_ra_memory(num_addresses:&usize, value_size:&usize, contents_as_vector:&Vec<usize>);
//     delegated_ra_memory(num_addresses:&usize, value_size:&usize, contents_as_map:&memory_contents);

//     usize get_value(address:&usize) const;
//     pub fn  set_value(address:&usize, value:&usize);

//     HashT::hash_value_type get_root() const;
//     HashT::merkle_authentication_path_type get_path(address:&usize) const;

//     pub fn  dump() const;
// };



// use crate::relations::ram_computations::memory::delegated_ra_memory;

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

// use  <algorithm>

use ffec::common::profiling;
use ffec::common::utils;


impl delegated_ra_memory<HashT>{
 pub fn int_to_tree_elem(i:&usize) ->bit_vector
{
     let mut v=vec![false;value_size];
    for k in 0..value_size
    {
        v[k] = ((i & (1u64 << k)) != 0);
    }
    return v;
}

pub fn int_from_tree_elem(v:&bit_vector) ->usize
{
    let mut  result = 0;
    for i in 0..value_size
    {
        result |= ( if v[i]  {1u64} else {0u64}) << i;
    }

    return result;
}


pub fn new(num_addresses:&usize,
                                                value_size:&usize) ->Self
    
{
//memory_interface(num_addresses, value_size)
    contents.reset( merkle_tree::<HashT>::new(log2(num_addresses), value_size));
}


pub fn new2(num_addresses:&usize,
                                                value_size:&usize,
                                                contents_as_vector:&Vec<usize>) ->Self
    
{
    //memory_interface(num_addresses, value_size)
    let mut  contents_as_bit_vector_vector:Vec<_>=contents_as_vector.iter().map(|value| int_to_tree_elem(value)).collect();
    // std::transform(contents_as_vector.begin(), contents_as_vector.end(), contents_as_bit_vector_vector, [this](usize value) { return int_to_tree_elem(value); });
    contents.reset(merkle_tree::<HashT>::new(log2(num_addresses), value_size, contents_as_bit_vector_vector));
}


pub fn new3(num_addresses:&usize,
                                                value_size:&usize,
                                                contents_as_map:&BTreeMap<usize, usize>) ->Self
    
{
    //memory_interface(num_addresses, value_size)
    let  contents_as_bit_vector_map:BTreeMap<_,_>=contents_as_map.iter().map(|(k,v)| (k,int_to_tree_elem(v))).collect();

    contents.reset(merkle_tree::<HashT>::new(log2(num_addresses), value_size, contents_as_bit_vector_map));
}

 pub fn get_value(address:&usize) ->usize
{
    return int_from_tree_elem(contents.get_value(address));
}


pub fn set_value(address:&usize,
                                           value:&usize)
{
    contents.set_value(address, int_to_tree_elem(value));
}

 pub fn get_root() ->HashT::hash_value_type
{
    return contents.get_root();
}

 pub fn get_path(address:&usize) ->HashT::merkle_authentication_path_type
{
    return contents.get_path(address);
}


pub fn dump() 
{
    contents.dump();
}

}

//#endif // DELEGATED_RA_MEMORY_TCC
