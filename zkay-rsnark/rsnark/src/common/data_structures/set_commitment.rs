// Declaration of interfaces for a Merkle tree based set commitment scheme.

use crate::common::data_structures::merkle_tree::HashTConfig;
use crate::common::data_structures::merkle_tree::merkle_authentication_path;
use crate::common::data_structures::merkle_tree::merkle_tree;
use crate::gadgetlib1::gadgets::hashes::hash_io; // TODO: the current structure is suboptimal
use ffec::common::utils::bit_vector;
use ffec::common::utils::log2;
use rccell::RcCell;
use std::collections::BTreeMap;

pub type set_commitment = bit_vector;

#[derive(Clone, Default)]
pub struct set_membership_proof {
    pub address: usize,
    pub merkle_path: merkle_authentication_path,
}

pub struct set_commitment_accumulator<HashT: HashTConfig> {
    pub tree: RcCell<merkle_tree<HashT>>,
    pub hash_to_pos: BTreeMap<bit_vector, usize>,
    pub depth: usize,
    pub digest_size: usize,
    pub value_size: usize,
}

// /* note that set_commitment has both .cpp, for implementation of
// non-templatized code (methods of set_membership_proof) and .tcc
// (implementation of set_commitment_accumulator<HashT> */
use ffec::common::serialization;
use std::mem;
impl set_membership_proof {
    pub fn size_in_bits(&self) -> usize {
        if self.merkle_path.is_empty() {
            return (8 * mem::size_of_val(&self.address));
        } else {
            return (8 * mem::size_of_val(&self.address)
                + self.merkle_path[0].len() * self.merkle_path.len());
        }
    }
}

impl<HashT: HashTConfig> set_commitment_accumulator<HashT> {
    pub fn new(max_entries: usize, value_size: usize) -> Self {
        let depth = log2(max_entries);
        let digest_size = HashT::get_digest_len();

        let tree = RcCell::new(merkle_tree::<HashT>::new(depth, digest_size));
        Self {
            value_size,
            tree,
            hash_to_pos: BTreeMap::new(),
            depth,
            digest_size,
        }
    }

    pub fn add(&mut self, value: &bit_vector) {
        assert!(self.value_size == 0 || value.len() == self.value_size);
        let hash = HashT::get_hash(&value);
        if !self.hash_to_pos.contains_key(&hash) {
            let pos = self.hash_to_pos.len();
            self.tree.borrow_mut().set_value(pos, &hash);
            self.hash_to_pos.insert(hash, pos);
        }
    }

    pub fn is_in_set(&self, value: &bit_vector) -> bool {
        assert!(self.value_size == 0 || value.len() == self.value_size);
        let hash = HashT::get_hash(&value);
        self.hash_to_pos.contains_key(&hash)
    }

    pub fn get_commitment(&self) -> set_commitment {
        return self.tree.borrow().get_root();
    }

    pub fn get_membership_proof(&self, value: &bit_vector) -> set_membership_proof {
        let hash = HashT::get_hash(value);
        let Some(&it) = self.hash_to_pos.get(&hash) else {
            panic!("the hash is not found ");
        };

        let mut proof = set_membership_proof::default();
        proof.address = it;
        proof.merkle_path = self.tree.borrow().get_path(it);

        return proof;
    }
}
