/** @file
 *****************************************************************************

 Declaration of interfaces for the Merkle tree check read gadget.

 The gadget checks the following: given a root R, address A, value V, and
 authentication path P, check that P is a valid authentication path for the
 value V as the A-th leaf in a Merkle tree with root R.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_CHECK_READ_GADGET_HPP_
// #define MERKLE_TREE_CHECK_READ_GADGET_HPP_

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::hashes::crh_gadget;
use crate::gadgetlib1::gadgets::hashes::digest_selector_gadget;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable;




pub struct merkle_tree_check_read_gadget {//gadget<FieldT>


hashers:    Vec<HashT>,
hasher_inputs:    Vec<block_variable<FieldT> >,
propagators:    Vec<digest_selector_gadget<FieldT> >,
internal_output:    Vec<digest_variable<FieldT> >,

computed_root:    RcCell<digest_variable<FieldT> >,
check_root:    RcCell<bit_vector_copy_gadget<FieldT> >,



    digest_size:usize,
    tree_depth:usize,
address_bits:    pb_linear_combination_array<FieldT>,
leaf:    digest_variable<FieldT>,
root:    digest_variable<FieldT>,
path:    merkle_authentication_path_variable<FieldT, HashT>,
read_successful:    pb_linear_combination<FieldT>,

   
}






// use crate::gadgetlib1::gadgets::merkle_tree::merkle_tree_check_read_gadget;

//#endif // MERKLE_TREE_CHECK_READ_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the Merkle tree check read.

 See merkle_tree_check_read_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_CHECK_READ_GADGET_TCC_
// #define MERKLE_TREE_CHECK_READ_GADGET_TCC_



impl merkle_tree_check_read_gadget<FieldT, HashT>{
pub fn new(pb:protoboard<FieldT>,
                                                                            tree_depth:usize,
                                                                            address_bits:&pb_linear_combination_array<FieldT>,
                                                                            leaf:&digest_variable<FieldT>,
                                                                            root:&digest_variable<FieldT>,
                                                                            path:&merkle_authentication_path_variable<FieldT, HashT>,
                                                                            read_successful:&pb_linear_combination<FieldT>,
                                                                            annotation_prefix:&String)->Self
   
{
    /*
       The tricky part here is ordering. For Merkle tree
       authentication paths, path[0] corresponds to one layer below
       the root (and path[tree_depth-1] corresponds to the layer
       containing the leaf), while address_bits has the reverse order:
       address_bits[0] is LSB, and corresponds to layer containing the
       leaf, and address_bits[tree_depth-1] is MSB, and corresponds to
       the subtree directly under the root.
    */
    assert!(tree_depth > 0);
    assert!(tree_depth == address_bits.len());

    for i in 0..tree_depth-1
    {
        internal_output.push(digest_variable::<FieldT>(pb, digest_size, FMT(self.annotation_prefix, " internal_output_{}", i)));
    }

    computed_root.reset(digest_variable::<FieldT>::new(pb, digest_size, FMT(self.annotation_prefix, " computed_root")));

    for i in 0..tree_depth
    {
        let mut inp= block_variable::<FieldT>::new(pb, path.left_digests[i], path.right_digests[i], FMT(self.annotation_prefix, " inp_{}", i));
        hasher_inputs.push(inp);
        hashers.push(HashT(pb, 2*digest_size, inp, if i == 0 {*computed_root} else{internal_output[i-1]},
                                   FMT(self.annotation_prefix, " load_hashers_{}", i)));
    }

    for i in 0..tree_depth
    {
        /*
          The propagators take a computed hash value (or leaf in the
          base case) and propagate it one layer up, either in the left
          or the right slot of authentication_path_variable.
        */
        propagators.push(digest_selector_gadget::<FieldT>::new(pb, digest_size, if i < tree_depth - 1 { internal_output[i]} else {leaf},
                                                                address_bits[tree_depth-1-i], path.left_digests[i], path.right_digests[i],
                                                                FMT(self.annotation_prefix, " digest_selector_{}", i)));
    }

    check_root.reset(bit_vector_copy_gadget::<FieldT>::new(pb, computed_root.bits, root.bits, read_successful, FieldT::capacity(), FMT(annotation_prefix, " check_root")));
    //  gadget<FieldT>(pb, annotation_prefix),
   Self{ digest_size:HashT::get_digest_len(),
   tree_depth,
   address_bits,
   leaf,
   root,
   path,
    read_successful}
}


pub fn generate_r1cs_constraints()
{
    /* ensure correct hash computations */
    for i in 0..tree_depth
    {
        // Note that we check root outside and have enforced booleanity of path.left_digests/path.right_digests outside in path.generate_r1cs_constraints
        hashers[i].generate_r1cs_constraints(false);
    }

    /* ensure consistency of path.left_digests/path.right_digests with internal_output */
    for i in 0..tree_depth
    {
        propagators[i].generate_r1cs_constraints();
    }

    check_root.generate_r1cs_constraints(false, false);
}


pub fn generate_r1cs_witness()
{
    /* do the hash computations bottom-up */
    for i in ( 0..=tree_depth-1).rev()
    {
        /* propagate previous input */
        propagators[i].generate_r1cs_witness();

        /* compute hash */
        hashers[i].generate_r1cs_witness();
    }

    check_root.generate_r1cs_witness();
}


pub fn root_size_in_bits()->usize
{
    return HashT::get_digest_len();
}


pub fn expected_constraints(tree_depth:usize)->usize
{
    /* NB: this includes path constraints */
    let hasher_constraints = tree_depth * HashT::expected_constraints(false);
    let propagator_constraints = tree_depth * HashT::get_digest_len();
    let authentication_path_constraints = 2 * tree_depth * HashT::get_digest_len();
    let check_root_constraints = 3 * ffec::div_ceil(HashT::get_digest_len(), FieldT::capacity());

    return hasher_constraints + propagator_constraints + authentication_path_constraints + check_root_constraints;
}

}

pub fn  test_merkle_tree_check_read_gadget()
{
    /* prepare test */
    let digest_len = HashT::get_digest_len();
    let tree_depth = 16;
   let mut  path=vec![merkle_authentication_node;tree_depth];

    let mut  prev_hash:Vec<_>=(0..digest_len).map(|_|std::rand() % 2).collect();
     let mut  leaf = prev_hash;

     let mut  address_bits;

     let mut  address = 0;
    for level in ( 0..=tree_depth-1).rev()
    {
        let mut computed_is_right = (std::rand() % 2);
        address |= if computed_is_right {1u64 << (tree_depth-1-level)} else{0};
        address_bits.push_back(computed_is_right);
         let mut  other:Vec<_>=(0..digest_len).map(|_|std::rand() % 2).collect();

         let mut  block = prev_hash;
        block.insert(if computed_is_right  {block.begin() }else  {block.end()}, other.begin(), other.end());
         let mut  h = HashT::get_hash(block);

        path[level] = other;

        prev_hash = h;
    }
     let mut  root = prev_hash;

    /* execute test */
    let mut  pb=protoboard::<FieldT> ::new();
    let mut  address_bits_va=pb_variable_array::<FieldT>::new();
    address_bits_va.allocate(pb, tree_depth, "address_bits");
    let mut  leaf_digest=digest_variable::<FieldT>::new(pb, digest_len, "input_block");
    let mut  root_digest=digest_variable::<FieldT>::new(pb, digest_len, "output_digest");
    let mut  path_var=merkle_authentication_path_variable::<FieldT, HashT>::new(pb, tree_depth, "path_var");
    let mut  ml=merkle_tree_check_read_gadget::<FieldT, HashT>::new(pb, tree_depth, address_bits_va, leaf_digest, root_digest, path_var, ONE, "ml");

    path_var.generate_r1cs_constraints();
    ml.generate_r1cs_constraints();

    address_bits_va.fill_with_bits(pb, address_bits);
    assert!(address_bits_va.get_field_element_from_bits(pb).as_ulong() == address);
    leaf_digest.generate_r1cs_witness(leaf);
    path_var.generate_r1cs_witness(address, path);
    ml.generate_r1cs_witness();

    /* make sure that read checker didn't accidentally overwrite anything */
    address_bits_va.fill_with_bits(pb, address_bits);
    leaf_digest.generate_r1cs_witness(leaf);
    root_digest.generate_r1cs_witness(root);
    assert!(pb.is_satisfied());

    let num_constraints = pb.num_constraints();
    let expected_constraints = merkle_tree_check_read_gadget::<FieldT, HashT>::expected_constraints(tree_depth);
    assert!(num_constraints == expected_constraints);
}


//#endif // MERKLE_TREE_CHECK_READ_GADGET_TCC_
