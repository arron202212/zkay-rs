// Declaration of interfaces for a random-access memory.

use crate::relations::ram_computations::memory::memory_interface::{
    memory_base, memory_contents, memory_interface,
};

// /**
//  * A random-access memory maintains the memory's contents via a map (from addresses to values).
//  */
#[derive(Clone, Default)]
pub struct ra_memory {
    contents: memory_contents,
}


impl ra_memory {
    pub fn new(num_addresses: usize, value_size: usize) -> memory_base<Self> {
        memory_base::new(
            num_addresses,
            value_size,
            Self {
                contents: memory_contents::new(),
            },
        )
    }

    pub fn new2(
        num_addresses: usize,
        value_size: usize,
        contents_as_vector: Vec<usize>,
    ) -> memory_base<Self> {
        //copy Vec into BTreeMap
        memory_base::new(
            num_addresses,
            value_size,
            Self {
                contents: contents_as_vector.into_iter().enumerate().collect(),
            },
        )
    }

    pub fn new3(
        num_addresses: usize,
        value_size: usize,
        contents: memory_contents,
    ) -> memory_base<Self> {
        memory_base::new(num_addresses, value_size, Self { contents })
    }
}
impl memory_interface for memory_base<ra_memory> {
    fn get_value(&self, address: usize) -> usize {
        assert!(address < self.num_addresses);
        if let Some(&it) = self.t.contents.get(&address) {
            it
        } else {
            0
        }
    }

    fn set_value(&mut self, address: usize, value: usize) {
        self.t.contents.insert(address, value);
    }
}
