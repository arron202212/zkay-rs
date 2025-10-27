/** @file
 *****************************************************************************

 Declaration of interfaces for a Merkle tree.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_HPP_
// #define MERKLE_TREE_HPP_

// use  <map>
// 

use ffec::common::utils;



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

type merkle_authentication_node=bit_vector ;
type merkle_authentication_path=Vec<merkle_authentication_node> ;
  type hash_value_type= HashT::hash_value_type ;
    type merkle_authentication_path_type= HashT::merkle_authentication_path_type ;
// 
pub struct merkle_tree<HashT> {
// 

  

// 

hash_defaults:    Vec<hash_value_type>,
values:    BTreeMap<usize, bit_vector>,
hashes:    BTreeMap<usize, hash_value_type>,

depth:    usize,
value_size:    usize,
digest_size:    usize,

    // merkle_tree(depth:usize, value_size:usize);
    // merkle_tree(depth:usize, value_size:usize, contents_as_vector:&Vec<bit_vector>);
    // merkle_tree(depth:usize, value_size:usize, contents:&BTreeMap<usize, bit_vector>);

    // bit_vector get_value(address:usize) const;
    // pub fn  set_value(address:usize, value:&bit_vector);

    // hash_value_type get_root() const;
    // merkle_authentication_path_type get_path(address:usize) const;

    // pub fn  dump() const;
}



// use crate::common::data_structures::merkle_tree;

//#endif // MERKLE_TREE_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for Merkle tree.

 See merkle_tree.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// #ifndef MERKLE_TREE_TCC
// #define MERKLE_TREE_TCC

// #include <algorithm>

use ffec::common::profiling;
// use ffec::common::utils;

// namespace libsnark {

pub fn  two_to_one_CRH<HashT>(l:& HashT::hash_value_type,
                                               r:& HashT::hash_value_type)->HashT::hash_value_type
{
    let mut  new_input=HashT::hash_value_type::new();
    new_input.insert(new_input.end(), l.begin(), l.end());
    new_input.insert(new_input.end(), r.begin(), r.end());

    let  digest_size = HashT::get_digest_len();
    assert(l.len() == digest_size);
    assert(r.len() == digest_size);

    return HashT::get_hash(new_input);
}

impl merkle_tree<HashT>{

pub fn new(depth:usize, value_size:usize) ->Self
    
{
    assert(depth < std::mem::size_of::<usize>() * 8);

    let digest_size = HashT::get_digest_len();
    assert(value_size <= digest_size);

    let  last=hash_value_type(digest_size);
    hash_defaults.reserve(depth+1);
    hash_defaults.push(last);
    for i in  0.. depth
    {
        last = two_to_one_CRH::<HashT>(last, last);
        hash_defaults.push(last);
    }

    hash_defaults.reverse();
    Self{depth, value_size}
}

// 
pub fn new2(depth:usize,
                                value_size:usize,
                                contents_as_vector:&Vec<bit_vector>) ->Self
    
{
    assert(log2(contents_as_vector.len()) <= depth);
    for  address in 0.. contents_as_vector.len()
    {
        let  idx = address + (1u64<<depth) - 1;
        values[idx] = contents_as_vector[address];
        hashes[idx] = contents_as_vector[address];
        hashes[idx].resize(digest_size);
    }

    let  idx_begin = (1u64<<depth) - 1;
    let idx_end = contents_as_vector.len() + ((1u64<<depth) - 1);

    for layer in (1..=depth).rev()
    {
        for  idx in  (idx_begin ..idx_end).step_by(2)
        {
            let  l = hashes[idx]; // this is sound, because idx_begin is always a left child
            let r = (if idx + 1 < idx_end  {hashes[idx+1]} else {hash_defaults[layer]});

            let h = two_to_one_CRH::<HashT>(l, r);
            hashes[(idx-1)/2] = h;
        }

        idx_begin = (idx_begin-1)/2;
        idx_end = (idx_end-1)/2;
    }
    Self::new(depth, value_size)
}

// 
pub fn new3(depth:usize,
                                value_size:usize,
                                contents:&BTreeMap<usize, bit_vector>) ->Self
    
{

    if (!contents.empty())
    {
        assert(contents.rbegin().first < 1u64<<depth);

        for it in &contents
        {
            let  address = it.first;
            let value = it.second;
            let idx = address + (1u64<<depth) - 1;

            values[address] = value;
            hashes[idx] = value;
            hashes[idx].resize(digest_size);
        }

        let last_it = hashes.end();

        for  layer in  (1..=depth).rev()
        {
            let  next_last_it = hashes.begin();
            let mut it=hashes.next();
            while it.is_some()
            {
                let  idx = it.first;
                let hash = it.second;

                if (idx % 2 == 0)
                {
                    // this is the right child of its parent and by invariant we are missing the left child
                    hashes[(idx-1)/2] = two_to_one_CRH::<HashT>(hash_defaults[layer], hash);
                }
                else
                {
                    if (std::next(it) == last_it || std::next(it).first != idx + 1)
                    {
                        // this is the left child of its parent and is missing its right child
                        hashes[(idx-1)/2] = two_to_one_CRH::<HashT>(hash, hash_defaults[layer]);
                    }
                    else
                    {
                        // typical case: this is the left child of the parent and adjacent to it there is a right child
                        hashes[(idx-1)/2] = two_to_one_CRH::<HashT>(hash, std::next(it).second);
                        it.next();
                    }
                }
            }

            last_it = next_last_it;
        }
    }
    Self::new(depth, value_size)
}


 pub fn get_value(address:usize) ->bit_vector
{
    assert(log2(address) <= depth);


    let  mut padded_result = (if let Some(it) = values.find(address) {it.1.clone()} else { bit_vecto::with_capacity(digest_size)});
    padded_result.resize(value_size,false);

    return padded_result;
}


pub fn set_value(address:usize,
                                   value:&bit_vector)
{
    assert(log2(address) <= depth);
    let  idx = address + (1u64<<depth) - 1;

    assert(value.len() == value_size);
    values[address] = value;
    hashes[idx] = value;
    hashes[idx].resize(digest_size);

    for idx in  (0..depth).rev()
    {
        idx = (idx-1)/2;

       
        let  l = (if let Some(it) = hashes.find(2*idx+1){it.1.clone()} else {hash_defaults[layer+1]} );

        let  r = (if let Some(it) = hashes.find(2*idx+2){it.1.clone()} else {hash_defaults[layer+1]} );

        let  h = two_to_one_CRH::<HashT>(l, r);
        hashes[idx] = h;
    }
}


  pub fn get_root() ->HashT::hash_value_type
{
if let Some(it) = hashes.find(0){it.1.clone()} else {hash_defaults[0]}
    // auto it = hashes.find(0);
    // return (it == hashes.end() ? hash_defaults[0] : it.1);
}


  pub fn get_path(address:usize) ->HashT::merkle_authentication_path_type
{
     let mut  result=HashT::merkle_authentication_path_type(depth);
    assert(log2(address) <= depth);
    let  idx = address + (1u64<<depth) - 1;

    for  layer in  (1..=depth).rev()
    {
        let  sibling_idx = ((idx + 1) ^ 1) - 1;
       
        if (layer == depth)
        {
            result[layer-1] = (if let Some(it2) =values.find(sibling_idx - ((1u64<<depth) - 1)){it2.1.clone()} else { bit_vector(value_size, false) });
            result[layer-1].resize(digest_size);
        }
        else
        {
            result[layer-1] = (if let Some(it) = hashes.find(sibling_idx){it.1.clone()} else {hash_defaults[layer] });
        }

        idx = (idx-1)/2;
    }

    return result;
}


pub fn dump() 
{
    for i in  0..1u64<<depth
    {
        printf("[{}] -> ", i);
        let  value = ( if let Some(it) = values.find(i){it.1.clone()}else{ bit_vector(value_size) });
        for &b in  &value
        {
            printf("{}", b  as u8);
        }
        printf("\n");
    }
    printf("\n");
}
}
// } // libsnark

// #endif // MERKLE_TREE_TCC
