/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SET_MEMBERSHIP_PROOF_VARIABLE_HPP_
// #define SET_MEMBERSHIP_PROOF_VARIABLE_HPP_

use crate::common::data_structures::set_commitment;
use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable;




pub struct set_membership_proof_variable<FieldT> {//gadget<FieldT>

address_bits:    pb_variable_array<FieldT>,
merkle_path:    RcCell<merkle_authentication_path_variable<FieldT, HashT> >,

    max_entries:usize,
    tree_depth:usize,

   

   
}



// use crate::gadgetlib1::gadgets::set_commitment::set_membership_proof_variable;

//#endif // SET_MEMBERSHIP_PROOF_VARIABLE_HPP
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SET_MEMBERSHIP_PROOF_VARIABLE_TCC_
// #define SET_MEMBERSHIP_PROOF_VARIABLE_TCC_



impl set_membership_proof_variable<FieldT, HashT>{
pub fn new(pb:protoboard<FieldT>,
                                                                            max_entries:usize,
                                                                            annotation_prefix:&String)->Self
    
{
    if tree_depth > 0
    {
        address_bits.allocate(pb, tree_depth, FMT(annotation_prefix, " address_bits"));
        merkle_path.reset(merkle_authentication_path_variable::<FieldT, HashT>::new(pb, tree_depth, FMT(annotation_prefix, " merkle_path")));
    }
    // gadget<FieldT>(pb, annotation_prefix),
   Self{max_entries,
    tree_depth:ffec::log2(max_entries)}
}


pub fn generate_r1cs_constraints()
{
    if tree_depth > 0
    {
        for i in 0..tree_depth
        {
            generate_boolean_r1cs_constraint::<FieldT>(self.pb, address_bits[i], FMT(self.annotation_prefix, " address_bits"));
        }
        merkle_path.generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness(proof:&set_membership_proof)
{
    if tree_depth > 0
    {
        address_bits.fill_with_bits_of_field_element(self.pb, FieldT(proof.address));
        merkle_path.generate_r1cs_witness(proof.address, proof.merkle_path);
    }
}


pub fn get_membership_proof() ->set_membership_proof
{
     let mut result=set_membership_proof::new();

    if tree_depth == 0
    {
       result.address =0;
    }
    else
    {
        result.address = address_bits.get_field_element_from_bits(self.pb).as_ulong();
        result.merkle_path = merkle_path.get_authentication_path(result.address);
    }

    result
}


pub fn as_r1cs_variable_assignment(proof:&set_membership_proof)->r1cs_variable_assignment<FieldT>
{
    let mut  pb=protoboard::<FieldT> ::new();
    let max_entries = (1u64 << (proof.merkle_path.len()));
    let mut  proof_variable=set_membership_proof_variable::<FieldT, HashT>::new(pb, max_entries, "proof_variable");
    proof_variable.generate_r1cs_witness(proof);

    return pb.full_variable_assignment();
}

}

//#endif // SET_MEMBERSHIP_PROOF_VARIABLE_TCC
