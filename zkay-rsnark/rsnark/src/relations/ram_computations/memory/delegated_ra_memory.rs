// Declaration of interfaces for a delegated random-access memory.



use crate::common::data_structures::merkle_tree::{
    HashTConfig, hash_value_type, merkle_authentication_path_type, merkle_tree,
};
use crate::relations::ram_computations::memory::memory_interface::{
    memory_base, memory_contents, memory_interface,
};
use ffec::common::utils::bit_vector;
use ffec::common::utils::log2;


use rccell::RcCell;
use std::collections::BTreeMap;

#[derive(Clone, Default)]
pub struct delegated_ra_memory<HashT: HashTConfig> {
    // : public memory_interface
    // bit_vector int_to_tree_elem(i:&usize) const;
    // usize int_from_tree_elem(v:&bit_vector) const;
    contents: RcCell<merkle_tree<HashT>>,
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

// use ffec::common::profiling;
// use ffec::common::utils;
pub type delegated_ra_memorys<HashT>=memory_base<delegated_ra_memory<HashT>> ;
impl<HashT: HashTConfig> delegated_ra_memory<HashT> {
    pub fn new(num_addresses: usize, value_size: usize) -> memory_base<Self> {
        //memory_interface(num_addresses, value_size)
        let contents = merkle_tree::<HashT>::new(log2(num_addresses), value_size);
        memory_base::new(
            num_addresses,
            value_size,
            Self {
                contents: RcCell::new(contents),
            },
        )
    }

    pub fn new2(
        num_addresses: usize,
        value_size: usize,
        contents_as_vector: Vec<usize>,
    ) -> memory_base<Self> {
        //memory_interface(num_addresses, value_size)
        let mut contents_as_bit_vector_vector: Vec<_> = contents_as_vector
            .iter()
            .map(|&value| Self::int_to_tree_elem(value, value_size))
            .collect();
        // std::transform(contents_as_vector.begin(), contents_as_vector.end(), contents_as_bit_vector_vector, [this](usize value) { return int_to_tree_elem(value); });

        let mut contents = merkle_tree::<HashT>::new2(
            log2(num_addresses),
            value_size,
            contents_as_bit_vector_vector,
        );
        memory_base::new(
            num_addresses,
            value_size,
            Self {
                contents: RcCell::new(contents),
            },
        )
    }

    pub fn new3(
        num_addresses: usize,
        value_size: usize,
        contents_as_map: BTreeMap<usize, usize>,
    ) -> memory_base<Self> {
        //memory_interface(num_addresses, value_size)
        let contents_as_bit_vector_map: BTreeMap<_, _> = contents_as_map
            .iter()
            .map(|(&k, &v)| (k, Self::int_to_tree_elem(v, value_size)))
            .collect();

        let contents =
            merkle_tree::<HashT>::new3(log2(num_addresses), value_size, contents_as_bit_vector_map);
        memory_base::new(
            num_addresses,
            value_size,
            Self {
                contents: RcCell::new(contents),
            },
        )
    }
    pub fn int_to_tree_elem(i: usize, value_size: usize) -> bit_vector {
        let mut v = vec![false; value_size];
        for k in 0..value_size {
            v[k] = ((i & (1usize << k)) != 0);
        }
        return v;
    }

    pub fn int_from_tree_elem(v: &bit_vector, value_size: usize) -> usize {
        let mut result = 0;
        for i in 0..value_size {
            result |= (if v[i] { 1usize } else { 0usize }) << i;
        }

        return result;
    }
}

impl<HashT: HashTConfig> memory_base<delegated_ra_memory<HashT>> {
    pub fn get_root(&self) -> hash_value_type {
         self.t.contents.borrow().get_root()
    }

    pub fn get_path(&self, address: usize) -> merkle_authentication_path_type {
         self.t.contents.borrow().get_path(address)
    }

    pub fn dump(&self) {
        self.t.contents.borrow().dump();
    }
}

impl<HashT: HashTConfig> memory_interface for memory_base<delegated_ra_memory<HashT>> {
    fn get_value(&self, address: usize) -> usize {
        delegated_ra_memory::<HashT>::int_from_tree_elem(
            &self.t.contents.borrow().get_value(address),
            self.value_size,
        )
    }

    fn set_value(&mut self, address: usize, value: usize) {
        self.t.contents.borrow_mut().set_value(
            address,
            &delegated_ra_memory::<HashT>::int_to_tree_elem(value, self.value_size),
        );
    }
}

