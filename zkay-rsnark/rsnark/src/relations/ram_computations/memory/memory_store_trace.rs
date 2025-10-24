/** @file
 *****************************************************************************

 Declaration of interfaces for a memory store trace.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_STORE_TRACE_HPP_
// #define MEMORY_STORE_TRACE_HPP_

use crate::relations::ram_computations::memory::memory_interface;


use std::collections::HashMap;
/**
 * A pair consisting of an address and a value.
 * It represents a memory store.
 */
type address_and_value=(size_t, size_t) ;

/**
 * A list in which each component consists of a timestamp and a memory store.
 */
pub struct  memory_store_trace {
// private:
     entries:HashMap<size_t, address_and_value>,

// 
//     memory_store_trace();
//     address_and_value get_trace_entry(timestamp:size_t,) const;
//     HashMap<size_t, address_and_value> get_all_trace_entries() const;
//     void set_trace_entry(timestamp:size_t, av:&address_and_value);

//     memory_contents as_memory_contents() const;
}



//#endif // MEMORY_STORE_TRACE_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a memory store trace.

 See memory_store_trace.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use crate::relations::ram_computations::memory::memory_store_trace;


impl memory_store_trace{
pub fn new()->Self
{  
    Self{}
}

 pub fn get_trace_entry(timestamp:size_t,) ->address_and_value
{
    if let  Some(it) = entries.find(timestamp){
        it.1
    }else{
        (0,0)
    }
    // return if it != entries.end() {it->second} else{std::make_pair<size_t, size_t>(0, 0)};
}

 pub fn get_all_trace_entries() ->HashMap<size_t, address_and_value>
{
    return entries;
}

 pub fn set_trace_entry(timestamp:size_t, av:&address_and_value)
{
    entries[timestamp] = av;
}

 pub fn as_memory_contents() ->memory_contents
{
    let mut  result=memory_contents::new();

    for ts_and_addrval in &entries
    {
        result[ts_and_addrval.1.0] = ts_and_addrval.1.1;
    }

    return result;
}

}