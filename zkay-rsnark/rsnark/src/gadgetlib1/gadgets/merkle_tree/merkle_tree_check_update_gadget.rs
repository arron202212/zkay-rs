/** @file
 *****************************************************************************

 Declaration of interfaces for the Merkle tree check read gadget.

 The gadget checks the following: given two roots R1 and R2, address A, two
 values V1 and V2, and authentication path P, check that
 - P is a valid authentication path for the value V1 as the A-th leaf in a Merkle tree with root R1, and
 - P is a valid authentication path for the value V2 as the A-th leaf in a Merkle tree with root R2.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_CHECK_UPDATE_GADGET_HPP_
// #define MERKLE_TREE_CHECK_UPDATE_GADGET_HPP_

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::hashes::crh_gadget;
use crate::gadgetlib1::gadgets::hashes::digest_selector_gadget;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable;




pub struct merkle_tree_check_update_gadget {//gadget<FieldT>


prev_hashers:    Vec<HashT>,
prev_hasher_inputs:    Vec<block_variable<FieldT> >,
prev_propagators:    Vec<digest_selector_gadget<FieldT> >,
prev_internal_output:    Vec<digest_variable<FieldT> >,

next_hashers:    Vec<HashT>,
next_hasher_inputs:    Vec<block_variable<FieldT> >,
next_propagators:    Vec<digest_selector_gadget<FieldT> >,
next_internal_output:    Vec<digest_variable<FieldT> >,

computed_next_root:    RcCell<digest_variable<FieldT> >,
check_next_root:    RcCell<bit_vector_copy_gadget<FieldT> >,



    digest_size:usize,
    tree_depth:usize,

address_bits:    pb_variable_array<FieldT>,
prev_leaf_digest:    digest_variable<FieldT>,
prev_root_digest:    digest_variable<FieldT>,
prev_path:    merkle_authentication_path_variable<FieldT, HashT>,
next_leaf_digest:    digest_variable<FieldT>,
next_root_digest:    digest_variable<FieldT>,
next_path:    merkle_authentication_path_variable<FieldT, HashT>,
update_successful:    pb_linear_combination<FieldT>,

    /* Note that while it is necessary to generate R1CS constraints
       for prev_path, it is not necessary to do so for next_path. See
       comment in the implementation of generate_r1cs_constraints() */

    
}


// pub fn  test_merkle_tree_check_update_gadget();



// use crate::gadgetlib1::gadgets::merkle_tree::merkle_tree_check_update_gadget;

//#endif // MERKLE_TREE_CHECK_UPDATE_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the Merkle tree check update gadget.

 See merkle_tree_check_update_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_CHECK_UPDATE_GADGET_TCC_
// #define MERKLE_TREE_CHECK_UPDATE_GADGET_TCC_


impl merkle_tree_check_update_gadget<FieldT, HashT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                                                tree_depth:usize,
                                                                                address_bits:&pb_variable_array<FieldT>,
                                                                                prev_leaf_digest:&digest_variable<FieldT>,
                                                                                prev_root_digest:&digest_variable<FieldT>,
                                                                                prev_path:&merkle_authentication_path_variable<FieldT, HashT>,
                                                                                next_leaf_digest:&digest_variable<FieldT>,
                                                                                next_root_digest:&digest_variable<FieldT>,
                                                                                next_path:&merkle_authentication_path_variable<FieldT, HashT>,
                                                                                update_successful:&pb_linear_combination<FieldT>,
                                                                                annotation_prefix:&String)->Self
    
{
    assert!(tree_depth > 0);
    assert!(tree_depth == address_bits.len());

    for i in 0..tree_depth-1
    {
        prev_internal_output.push(digest_variable::<FieldT>(&pb, digest_size, FMT(self.annotation_prefix, " prev_internal_output_{}", i)));
        next_internal_output.push(digest_variable::<FieldT>(&pb, digest_size, FMT(self.annotation_prefix, " next_internal_output_{}", i)));
    }

    computed_next_root.reset(digest_variable::<FieldT>::new(pb, digest_size, FMT(self.annotation_prefix, " computed_root")));

    for i in 0..tree_depth
    {
        let mut  prev_inp=block_variable::<FieldT>::new(pb, prev_path.left_digests[i], prev_path.right_digests[i], FMT(self.annotation_prefix, " prev_inp_{}", i));
        prev_hasher_inputs.push(prev_inp);
        prev_hashers.push(HashT(&pb, 2*digest_size, prev_inp, if i == 0 {prev_root_digest} else{prev_internal_output[i-1]},
                                                                FMT(self.annotation_prefix, " prev_hashers_{}", i)));

        let mut  next_inp=block_variable::<FieldT>::new(pb, next_path.left_digests[i], next_path.right_digests[i], FMT(self.annotation_prefix, " next_inp_{}", i));
        next_hasher_inputs.push(next_inp);
        next_hashers.push(HashT(&pb, 2*digest_size, next_inp, if i == 0 {*computed_next_root} else{next_internal_output[i-1]},
                                                                FMT(self.annotation_prefix, " next_hashers_{}", i)));
    }

    for i in 0..tree_depth
    {
        prev_propagators.push(digest_selector_gadget::<FieldT>(&pb,  digest_size, if i < tree_depth -1 {prev_internal_output[i]} else {prev_leaf_digest},
                                                                     address_bits[tree_depth-1-i], prev_path.left_digests[i], prev_path.right_digests[i],
                                                                   FMT(self.annotation_prefix, " prev_propagators_{}", i)));
        next_propagators.push(digest_selector_gadget::<FieldT>(&pb, digest_size, if i < tree_depth -1 { next_internal_output[i]} else {next_leaf_digest},
                                                                     address_bits[tree_depth-1-i], next_path.left_digests[i], next_path.right_digests[i],
                                                                   FMT(self.annotation_prefix, " next_propagators_{}", i)));
    }

    check_next_root.reset(bit_vector_copy_gadget::<FieldT>::new(pb, computed_next_root.bits, next_root_digest.bits, update_successful, FieldT::capacity(), FMT(annotation_prefix, " check_next_root")));
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{digest_size:HashT::get_digest_len(),
   tree_depth,
   address_bits,
   prev_leaf_digest,
   prev_root_digest,
   prev_path,
   next_leaf_digest,
   next_root_digest,
   next_path,
    update_successful}
}


pub fn generate_r1cs_constraints()
{
    /* ensure correct hash computations */
    for i in 0..tree_depth
    {
        prev_hashers[i].generate_r1cs_constraints(false); // we check root outside and prev_left/prev_right above
        next_hashers[i].generate_r1cs_constraints(true); // however we must check right side hashes
    }

    /* ensure consistency of internal_left/internal_right with internal_output */
    for i in 0..tree_depth
    {
        prev_propagators[i].generate_r1cs_constraints();
        next_propagators[i].generate_r1cs_constraints();
    }

    /* ensure that prev auxiliary input and next auxiliary input match */
    for i in 0..tree_depth
    {
        for j in 0..digest_size
        {
            /*
              addr * (prev_left - next_left) + (1 - addr) * (prev_right - next_right) = 0
              addr * (prev_left - next_left - prev_right + next_right) = next_right - prev_right
            */
            self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(address_bits[tree_depth-1-i],
                                                                 prev_path.left_digests[i].bits[j] - next_path.left_digests[i].bits[j] - prev_path.right_digests[i].bits[j] + next_path.right_digests[i].bits[j],
                                                                 next_path.right_digests[i].bits[j] - prev_path.right_digests[i].bits[j]),
                                       FMT(self.annotation_prefix, " aux_check_{}_{}", i, j));
        }
    }

    /* Note that while it is necessary to generate R1CS constraints
       for prev_path, it is not necessary to do so for next_path.

       This holds, because { next_path.left_inputs[i],
       next_path.right_inputs[i] } is a pair { hash_output,
       auxiliary_input }. The bitness for hash_output is enforced
       above by next_hashers[i].generate_r1cs_constraints.

       Because auxiliary input is the same for prev_path and next_path
       (enforced above), we have that auxiliary_input part is also
       constrained to be boolean, because prev_path is *all*
       constrained to be all boolean. */

    check_next_root.generate_r1cs_constraints(false, false);
}


pub fn generate_r1cs_witness()
{
    /* do the hash computations bottom-up */
    for i in ( 0..=tree_depth-1).rev()
    {
        /* ensure consistency of prev_path and next_path */
        if self.pb.val(address_bits[tree_depth-1-i]) == FieldT::one()
        {
            next_path.left_digests[i].generate_r1cs_witness(prev_path.left_digests[i].get_digest());
        }
        else
        {
            next_path.right_digests[i].generate_r1cs_witness(prev_path.right_digests[i].get_digest());
        }

        /* propagate previous input */
        prev_propagators[i].generate_r1cs_witness();
        next_propagators[i].generate_r1cs_witness();

        /* compute hash */
        prev_hashers[i].generate_r1cs_witness();
        next_hashers[i].generate_r1cs_witness();
    }

    check_next_root.generate_r1cs_witness();
}


 pub fn root_size_in_bits()->usize
{
    return HashT::get_digest_len();
}


 pub fn expected_constraints(tree_depth:usize)->usize
{
    /* NB: this includes path constraints */
    let prev_hasher_constraints = tree_depth * HashT::expected_constraints(false);
    let next_hasher_constraints = tree_depth * HashT::expected_constraints(true);
    let prev_authentication_path_constraints = 2 * tree_depth * HashT::get_digest_len();
    let prev_propagator_constraints = tree_depth * HashT::get_digest_len();
    let next_propagator_constraints = tree_depth * HashT::get_digest_len();
    let check_next_root_constraints = 3 * ffec::div_ceil(HashT::get_digest_len(), FieldT::capacity());
    let aux_equality_constraints = tree_depth * HashT::get_digest_len();

    return (prev_hasher_constraints + next_hasher_constraints + prev_authentication_path_constraints +
            prev_propagator_constraints + next_propagator_constraints + check_next_root_constraints +
            aux_equality_constraints);
}

}

pub fn  test_merkle_tree_check_update_gadget()
{
    /* prepare test */
    let digest_len = HashT::get_digest_len();

    let tree_depth = 16;
    let mut prev_path=vec![merkle_authentication_node;tree_depth];

    let  prev_load_hash:Vec<_>=(0..digest_len).map(|_| rand::random() % 2).collect();
    let  prev_store_hash:Vec<_>=(0..digest_len).map(|_| rand::random() % 2).collect();

    let  loaded_leaf = prev_load_hash;
    let stored_leaf = prev_store_hash;

    let mut address_bits;

    let mut  address = 0;
    for level in ( 0..=tree_depth-1).rev()
    {
        let mut computed_is_right = (rand::random() % 2);
        address |= if computed_is_right {1u64 << (tree_depth-1-level)} else{0};
        address_bits.push_back(computed_is_right);
        let mut  other:Vec<_>=(0..digest_len).map(|_| rand::random() % 2).collect();

         let mut  load_block = prev_load_hash;
        load_block.insert( if computed_is_right  {load_block.begin()} else {load_block.end()}, other.begin(), other.end());
        let mut store_block = prev_store_hash;
        store_block.insert(if computed_is_right  {store_block.begin() }else {store_block.end()}, other.begin(), other.end());

         let mut  load_h = HashT::get_hash(load_block);
         let mut  store_h = HashT::get_hash(store_block);

        prev_path[level] = other;

        prev_load_hash = load_h;
        prev_store_hash = store_h;
    }

     let mut  load_root = prev_load_hash;
    let mut store_root = prev_store_hash;

    /* execute the test */
    let mut  pb=protoboard::<FieldT> ::new();
     let mut  address_bits_va=pb_variable_array::<FieldT>::new();
    address_bits_va.allocate(&pb, tree_depth, "address_bits");
    let mut  prev_leaf_digest=digest_variable::<FieldT>::new(pb, digest_len, "prev_leaf_digest");
    let mut prev_root_digest=digest_variable::<FieldT>::new(pb, digest_len, "prev_root_digest");
    let mut prev_path_var= merkle_authentication_path_variable::<FieldT, HashT>::new(pb, tree_depth, "prev_path_var");
    let mut  next_leaf_digest=digest_variable::<FieldT>::new(pb, digest_len, "next_leaf_digest");
    let mut  next_root_digest=digest_variable::<FieldT>::new(pb, digest_len, "next_root_digest");
    let mut  next_path_var= merkle_authentication_path_variable::<FieldT, HashT>::new(pb, tree_depth, "next_path_var");
    let mut mls=merkle_tree_check_update_gadget::<FieldT, HashT> ::new(pb, tree_depth, address_bits_va,
                                                       prev_leaf_digest, prev_root_digest, prev_path_var,
                                                       next_leaf_digest, next_root_digest, next_path_var, ONE, "mls");

    prev_path_var.generate_r1cs_constraints();
    mls.generate_r1cs_constraints();

    address_bits_va.fill_with_bits(&pb, address_bits);
    assert!(address_bits_va.get_field_element_from_bits(pb).as_ulong() == address);
    prev_leaf_digest.generate_r1cs_witness(loaded_leaf);
    prev_path_var.generate_r1cs_witness(address, prev_path);
    next_leaf_digest.generate_r1cs_witness(stored_leaf);
    address_bits_va.fill_with_bits(&pb, address_bits);
    mls.generate_r1cs_witness();

    /* make sure that update check will check for the right things */
    prev_leaf_digest.generate_r1cs_witness(loaded_leaf);
    next_leaf_digest.generate_r1cs_witness(stored_leaf);
    prev_root_digest.generate_r1cs_witness(load_root);
    next_root_digest.generate_r1cs_witness(store_root);
    address_bits_va.fill_with_bits(&pb, address_bits);
    assert!(pb.is_satisfied());

    let num_constraints = pb.num_constraints();
    let expected_constraints = merkle_tree_check_update_gadget::<FieldT, HashT>::expected_constraints(tree_depth);
    assert!(num_constraints == expected_constraints);
}



//#endif // MERKLE_TREE_CHECK_UPDATE_GADGET_TCC_
