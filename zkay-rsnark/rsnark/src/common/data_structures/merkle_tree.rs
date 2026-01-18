// Declaration of interfaces for a Merkle tree.

use ffec::FieldTConfig;
use ffec::common::profiling;
use ffec::common::utils;
use ffec::common::utils::bit_vector;
use ffec::common::utils::log2;
use rccell::RcCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;

/**
 * A Merkle tree is maintained as two maps:
 * - a map from addresses to values, and
 * - a map from addresses to hashes.
 *
 * The second map maintains the intermediate hashes of a Merkle tree
 * built atop the values currently stored in the tree (the
 * implementation admits a very efficient support for sparse
 * trees). Besides offering methods to load and store values, the
 * pub struct offers methods to retrieve the root of the Merkle tree and to
 * obtain the authentication paths for (the value at) a given address.
 */

pub type merkle_authentication_node = bit_vector;
pub type merkle_authentication_path = Vec<merkle_authentication_node>;
pub type hash_value_type = bit_vector; //hash_value_type ;
pub type merkle_authentication_path_type = merkle_authentication_path; //merkle_authentication_path_type ;
//
pub struct merkle_tree<HashT: HashTConfig> {
    hash_defaults: Vec<hash_value_type>,
    values: Vec<bit_vector>,
    hashes: Vec<hash_value_type>,

    depth: usize,
    value_size: usize,
    digest_size: usize,

    // merkle_tree(depth:usize, value_size:usize);
    // merkle_tree(depth:usize, value_size:usize, contents_as_vector:&Vec<bit_vector>);
    // merkle_tree(depth:usize, value_size:usize, contents:&Vec<usize, bit_vector>);

    // bit_vector get_value(address:usize) const;
    // pub fn  set_value(address:usize, value:&bit_vector);

    // hash_value_type get_root() const;
    // merkle_authentication_path_type get_path(address:usize) const;

    // pub fn  dump() const;
    _t: PhantomData<HashT>,
}

pub trait VecConfig {
    fn len(&self) -> usize;
    fn with_capacity(len: usize) -> Self;
}
pub trait GConfig: Default + Clone {}
pub trait PConfig: Default + Clone {}
pub trait HashTConfig: Default + Clone {
    // type hash_value_type=hash_value_type;
    // type merkle_authentication_path_type=merkle_authentication_path_type;
    fn new5<P: PConfig, G: GConfig, G2: GConfig>(
        pb: RcCell<P>,
        sz: usize,
        g: G,
        a: G2,
        s: String,
    ) -> Self {
        Default::default()
    }
    fn get_digest_len() -> usize {
        0
    }
    fn get_block_len() -> usize {
        0
    }
    fn get_hash(input: &bit_vector) -> bit_vector {
        input.clone()
    }
    fn get_hash_for_field<FieldT: FieldTConfig>(input: &bit_vector) -> Vec<FieldT> {
        vec![]
    }
    fn generate_r1cs_constraints(&self, b: bool) {}
    fn generate_r1cs_witness(&self) {}
    fn expected_constraints(b: bool) -> usize {
        0
    }
}

pub fn two_to_one_CRH<HashT: HashTConfig>(
    l: &hash_value_type,
    r: &hash_value_type,
) -> hash_value_type {
    let mut new_input = hash_value_type::new();
    new_input.extend(l);
    new_input.extend(r);

    let digest_size = HashT::get_digest_len();
    assert!(l.len() == digest_size);
    assert!(r.len() == digest_size);

    HashT::get_hash(&new_input)
}

impl<HashT: HashTConfig> merkle_tree<HashT> {
    pub fn new(depth: usize, value_size: usize) -> Self {
        assert!(depth < std::mem::size_of::<usize>() * 8);

        let digest_size = HashT::get_digest_len();
        assert!(value_size <= digest_size);

        let mut last = hash_value_type::with_capacity(digest_size);
        let mut hash_defaults = Vec::with_capacity(depth + 1);
        hash_defaults.push(last.clone());
        for i in 0..depth {
            last = two_to_one_CRH::<HashT>(&last, &last);
            hash_defaults.push(last.clone());
        }

        // hash_defaults.reverse();
        Self {
            depth,
            value_size,
            digest_size,
            hash_defaults,
            hashes: Vec::new(),
            values: Vec::new(),
            _t: PhantomData,
        }
    }

    //
    pub fn new2(depth: usize, value_size: usize, contents_as_vector: Vec<bit_vector>) -> Self {
        assert!(log2(contents_as_vector.len()) <= depth);
        let mut hash_defaults = Vec::with_capacity(depth + 1);
        let n = contents_as_vector.len();
        let (mut values, mut hashes) = (Vec::new(), Vec::new());
        for address in 0..contents_as_vector.len() {
            let idx = address + (1usize << depth) - 1;
            values[idx] = contents_as_vector[address].clone();
            hashes[idx] = contents_as_vector[address].clone();
            // hashes[idx].resize(digest_size);
        }

        let mut idx_begin = (1usize << depth) - 1;
        let mut idx_end = contents_as_vector.len() + ((1usize << depth) - 1);

        for layer in (1..=depth).rev() {
            for idx in (idx_begin..idx_end).step_by(2) {
                let l = &hashes[idx]; // this is sound, because idx_begin is always a left child
                let r = (if idx + 1 < idx_end {
                    &hashes[(idx + 1)]
                } else {
                    &hash_defaults[layer]
                });

                let h = two_to_one_CRH::<HashT>(l, r);
                hashes[(idx - 1) / 2] = h;
            }

            idx_begin = (idx_begin - 1) / 2;
            idx_end = (idx_end - 1) / 2;
        }
        Self::new(depth, value_size)
    }

    //
    pub fn new3(depth: usize, value_size: usize, contents: BTreeMap<usize, bit_vector>) -> Self {
        let mut hash_defaults: Vec<_> = Vec::<Self>::with_capacity(depth + 1);
        let (mut values, mut hashes) = (Vec::new(), Vec::new());
        if (!contents.is_empty()) {
            assert!(contents.iter().last().unwrap().1.len() < 1usize << depth);

            for (address, value) in contents {
                let idx = address + (1usize << depth) - 1;

                values[address] = value.clone();
                hashes[idx] = value.clone();
                // hashes[idx].resize(digest_size);
            }

            let mut last_it = hashes.iter();

            for layer in (1..=depth).rev() {
                let next_last_it = hashes.iter();
                let mut it = hashes.iter();
                let mut idx = 0;
                while let Some(hash) = it.next() {
                    if (idx % 2 == 0) {
                        // this is the right child of its parent and by invariant we are missing the left child
                        // hashes[(idx-1)/2]= two_to_one_CRH::<HashT>(hash_defaults[layer], hash);
                    } else {
                        // if (it.next() == last_it || it.next().unwrap().0 != idx + 1)
                        // {
                        //     // this is the left child of its parent and is missing its right child
                        //     // hashes[(idx-1)/2]= two_to_one_CRH::<HashT>(hash, hash_defaults[layer]);
                        // }
                        // else
                        // {
                        //     // typical case: this is the left child of the parent and adjacent to it there is a right child
                        //     // hashes[(idx-1)/2]= two_to_one_CRH::<HashT>(hash, it.next().unwrap().1);
                        //     it.next();
                        // }
                    }
                    idx += 1;
                }

                last_it = next_last_it;
            }
        }
        Self::new(depth, value_size)
    }

    pub fn get_value(&self, address: usize) -> bit_vector {
        assert!(log2(address) <= self.depth);

        let mut padded_result = (if let Some(it) = self.values.get(address) {
            it.clone()
        } else {
            bit_vector::with_capacity(self.digest_size)
        });
        padded_result.resize(self.value_size, false);

        return padded_result;
    }

    pub fn set_value(&mut self, address: usize, value: &bit_vector) {
        assert!(log2(address) <= self.depth);
        let mut idx = address + (1usize << self.depth) - 1;

        assert!(value.len() == self.value_size);
        self.values[address] = value.clone();
        self.hashes[idx] = value.clone();
        self.hashes[idx].resize(self.digest_size, false);

        for layer in (0..self.depth).rev() {
            idx = (idx - 1) / 2;

            let l = (if let Some(it) = self.hashes.get(2 * idx + 1) {
                it
            } else {
                &self.hash_defaults[layer + 1]
            });

            let r = (if let Some(it) = self.hashes.get(2 * idx + 2) {
                it
            } else {
                &self.hash_defaults[layer + 1]
            });

            let h = two_to_one_CRH::<HashT>(l, r);
            self.hashes[idx] = h;
        }
    }

    pub fn get_root(&self) -> hash_value_type {
        if let Some(it) = self.hashes.get(0) {
            it.to_vec()
        } else {
            self.hash_defaults[0].clone()
        }
        // auto it = hashes.get(0);
        // return (it == hashes.end() ? hash_defaults[0] : it[1]);
    }

    pub fn get_path(&self, address: usize) -> merkle_authentication_path_type {
        let depth = self.depth as usize;
        let mut result = merkle_authentication_path_type::with_capacity(depth);
        assert!(log2(address) <= depth);
        let mut idx = address + (1usize << depth) - 1;

        for layer in (1..=depth).rev() {
            let sibling_idx = ((idx + 1) ^ 1) - 1;

            if (layer == depth) {
                result[layer - 1] =
                    (if let Some(it2) = self.values.get((sibling_idx - ((1usize << depth) - 1))) {
                        it2.clone()
                    } else {
                        vec![false; self.value_size]
                    });
                result[layer - 1].resize(self.digest_size, false);
            } else {
                result[layer - 1] = (if let Some(it) = self.hashes.get(sibling_idx) {
                    it.clone()
                } else {
                    self.hash_defaults[layer].clone()
                });
            }

            idx = (idx - 1) / 2;
        }

        result
    }

    pub fn dump(&self) {
        for i in 0..1usize << self.depth {
            print!("[{}] -> ", i);
            let value = (if let Some(it) = self.values.get(i) {
                it.clone()
            } else {
                bit_vector::with_capacity(self.value_size)
            });
            for &b in &value {
                print!("{}", b as u8);
            }
            print!("\n");
        }
        print!("\n");
    }
}
// } // libsnark

// #endif // MERKLE_TREE_TCC
