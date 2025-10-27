/** @file
 *****************************************************************************

 Declaration of interfaces for a Merkle tree based set commitment scheme.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SET_COMMITMENT_HPP_
// #define SET_COMMITMENT_HPP_

use ffec::common::utils;

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadgets::hashes::hash_io; // TODO: the current structure is suboptimal



type set_commitment=bit_vector ;

struct set_membership_proof {
address:    usize,
merkle_path:    merkle_authentication_path,

    // bool operator==(other:&set_membership_proof) const;
    // usize size_in_bits() const;
    // friend std::ostream& operator<<(std::ostream &out, other:&set_membership_proof);
    // friend std::istream& operator>>(std::istream &in, set_membership_proof &other);
}

// 
pub struct set_commitment_accumulator<HashT> {
// 
tree:    RcCell<merkle_tree<HashT> >,
hash_to_pos:    BTreeMap<bit_vector, usize>,
// 

depth:    usize,
digest_size:    usize,
value_size:    usize,


}



/* note that set_commitment has both .cpp, for implementation of
   non-templatized code (methods of set_membership_proof) and .tcc
   (implementation of set_commitment_accumulator<HashT> */
// use crate::common::data_structures::set_commitment;

//#endif // SET_COMMITMENT_HPP_
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use ffec::common::serialization;
use std::mem;
// use crate::common::data_structures::set_commitment;
impl set_membership_proof{
 pub fn size_in_bits() ->usize
{
    if merkle_path.empty()
    {
        return (8 * mem::size_of_val(address));
    }
    else
    {
        return (8 * mem::size_of_val(address) + merkle_path[0].len() * merkle_path.len());
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


/** @file
 *****************************************************************************

 Implementation of a Merkle tree based set commitment scheme.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SET_COMMITMENT_TCC_
// #define SET_COMMITMENT_TCC_


impl set_commitment_accumulator<HashT>{

pub fn new(max_entries:usize, value_size:usize) ->Self
    
{
    let depth = ffec::log2(max_entries);
    let digest_size = HashT::get_digest_len();

    tree.reset( merkle_tree::<HashT>::new(depth, digest_size));
    Self{value_size}
}


pub fn add(value:&bit_vector)
{
    assert!(value_size == 0 || value.len() == value_size);
    let  hash = HashT::get_hash(value);
    if !hash_to_pos.contains(hash) 
    {
        let  pos = hash_to_pos.len();
        tree.set_value(pos, hash);
        hash_to_pos[hash] = pos;
    }
}


 pub fn is_in_set(value:&bit_vector) ->bool
{
    assert!(value_size == 0 || value.len() == value_size);
    let  hash = HashT::get_hash(value);
    hash_to_pos.contains(hash)
}


pub fn get_commitment() ->set_commitment 
{
    return tree.get_root();
}


 pub fn get_membership_proof(value:&bit_vector) ->set_membership_proof
{
   let  hash = HashT::get_hash(value);
    let Some(it) = hash_to_pos.find(hash) else{
        painc!("the hash is not found ");
    };
   

    let mut  proof=set_membership_proof::new();
    proof.address = itsecond;
    proof.merkle_path = tree.get_path(it.second);

    return proof;
}

}

//#endif // SET_COMMITMENT_TCC_
