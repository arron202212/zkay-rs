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
    // bool operator==(other:&set_membership_proof) const;
    // usize size_in_bits() const;
    // friend std::ostream& operator<<(std::ostream &out, other:&set_membership_proof);
    // friend std::istream& operator>>(std::istream &in, set_membership_proof &other);
}

//
pub struct set_commitment_accumulator<HashT: HashTConfig> {
    pub tree: RcCell<merkle_tree<HashT>>,
    pub hash_to_pos: BTreeMap<bit_vector, usize>,
    pub depth: usize,
    pub digest_size: usize,
    pub value_size: usize,
}

/* note that set_commitment has both .cpp, for implementation of
non-templatized code (methods of set_membership_proof) and .tcc
(implementation of set_commitment_accumulator<HashT> */

use ffec::common::serialization;
use std::mem;
// use crate::common::data_structures::set_commitment;
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
// bool set_membership_proof::operator==(other:&set_membership_proof) const
// {
//     return (self.address == other.address &&
//             self.merkle_path == other.merkle_path);
// }

// std::ostream& operator<<(std::ostream &out, proof:&set_membership_proof)
// {
//     out << proof.address << "\n";
//     out << proof.merkle_path.len() << "\n";
//     for i in 0..proof.merkle_path.len()
//     {
//         ffec::output_bool_vector(out, proof.merkle_path[i]);
//     }

//     return out;
// }

// std::istream& operator>>(std::istream &in, set_membership_proof &proof)
// {
//     in >> proof.address;
//     ffec::consume_newline(in);
//     usize tree_depth;
//     in >> tree_depth;
//     ffec::consume_newline(in);
//     proof.merkle_path.resize(tree_depth);

//     for i in 0..tree_depth
//     {
//         ffec::input_bool_vector(in, proof.merkle_path[i]);
//     }

//     return in;
// }

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
