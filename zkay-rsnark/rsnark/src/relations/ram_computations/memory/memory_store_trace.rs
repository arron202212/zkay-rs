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
use super::memory_interface::{memory_base, memory_contents, memory_interface};

use std::collections::HashMap;
/**
 * A pair consisting of an address and a value.
 * It represents a memory store.
 */
pub type address_and_value = (usize, usize);

/**
 * A list in which each component consists of a timestamp and a memory store.
 */
#[derive(Default)]
pub struct memory_store_trace {
    //
    entries: HashMap<usize, address_and_value>,
    //
    //     memory_store_trace();
    //     address_and_value get_trace_entry(timestamp:usize,) const;
    //     HashMap<usize, address_and_value> get_all_trace_entries() const;
    //     pub fn  set_trace_entry(timestamp:usize, av:&address_and_value);

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
use std::ops::{Index, IndexMut};
impl Index<usize> for memory_store_trace {
    type Output = address_and_value;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.get(&index).unwrap()
    }
}

impl IndexMut<usize> for memory_store_trace {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.get_mut(&index).unwrap()
    }
}

impl memory_store_trace {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get_trace_entry(&self, timestamp: usize) -> address_and_value {
        if let Some(it) = self.entries.get(&timestamp) {
            *it
        } else {
            (0, 0)
        }
        // return if it != entries.end() {it.1} else{std::make_pair<usize, usize>(0, 0)};
    }

    pub fn get_all_trace_entries(&self) -> &HashMap<usize, address_and_value> {
        &self.entries
    }

    pub fn set_trace_entry(&mut self, timestamp: usize, av: address_and_value) {
        self.entries.insert(timestamp, av);
    }

    pub fn as_memory_contents(&self) -> memory_contents {
        self.entries.values().cloned().collect()
    }
}
