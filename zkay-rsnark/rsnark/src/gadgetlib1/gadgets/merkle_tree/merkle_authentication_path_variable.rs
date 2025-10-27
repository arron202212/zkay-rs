/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_AUTHENTICATION_PATH_VARIABLE_HPP_
// #define MERKLE_AUTHENTICATION_PATH_VARIABLE_HPP_

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::hashes::hash_io;




pub struct merkle_authentication_path_variable {//gadget<FieldT>


    tree_depth:usize,
left_digests:    Vec<digest_variable<FieldT> >,
right_digests:    Vec<digest_variable<FieldT> >,

    
}



// use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable;

//#endif // MERKLE_AUTHENTICATION_PATH_VARIABLE_HPP
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_AUTHENTICATION_PATH_VARIABLE_TCC_
// #define MERKLE_AUTHENTICATION_PATH_VARIABLE_TCC_



impl merkle_authentication_path_variable<FieldT, HashT>{
pub fn new(pb:protoboard<FieldT>,
                                                                                        tree_depth:usize,
                                                                                        annotation_prefix:&String)->Self
   
{
    for i in 0..tree_depth
    {
        left_digests.push(digest_variable::<FieldT>(pb, HashT::get_digest_len(), FMT(annotation_prefix, " left_digests_{}", i)));
        right_digests.push(digest_variable::<FieldT>(pb, HashT::get_digest_len(), FMT(annotation_prefix, " right_digests_{}", i)));
    }
    //  gadget<FieldT>(pb, annotation_prefix),
    Self{tree_depth}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..tree_depth
    {
        left_digests[i].generate_r1cs_constraints();
        right_digests[i].generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness(address:&usize , path: merkle_authentication_path)
{
    assert!(path.len() == tree_depth);

    for i in 0..tree_depth
    {
        if address & (1u64 << (tree_depth-1-i))
        {
            left_digests[i].generate_r1cs_witness(path[i]);
        }
        else
        {
            right_digests[i].generate_r1cs_witness(path[i]);
        }
    }
}


 pub fn get_authentication_path(address:usize) ->merkle_authentication_path
{
     let mut result=merkle_authentication_path::new();
    for i in 0..tree_depth
    {
        if address & (1u64 << (tree_depth-1-i))
        {
            result.push(left_digests[i].get_digest());
        }
        else
        {
            result.push(right_digests[i].get_digest());
        }
    }

    return result;
}

}

//#endif // MERKLE_AUTHENTICATION_PATH_VARIABLE_TCC
