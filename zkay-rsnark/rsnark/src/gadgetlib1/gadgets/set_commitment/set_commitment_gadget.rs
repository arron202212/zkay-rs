/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef SET_COMMITMENT_GADGET_HPP_
// #define SET_COMMITMENT_GADGET_HPP_

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets::merkle_tree::merkle_tree_check_read_gadget;
use crate::gadgetlib1::gadgets::set_commitment::set_membership_proof_variable;




type set_commitment_variable = digest_variable<FieldT>;


pub struct set_commitment_gadget<FieldT> {//gadget<FieldT>

element_block:    RcCell<block_variable<FieldT> >,
element_digest:    RcCell<digest_variable<FieldT> >,
hash_element:    RcCell<HashT>,
check_membership:    RcCell<merkle_tree_check_read_gadget<FieldT, HashT> >,


tree_depth:    usize,
element_bits:    pb_variable_array<FieldT>,
root_digest:    set_commitment_variable<FieldT, HashT>,
proof:    set_membership_proof_variable<FieldT, HashT>,
check_successful:    pb_linear_combination<FieldT>,

}






// use crate::gadgetlib1::gadgets::set_commitment::set_commitment_gadget;

//#endif // SET_COMMITMENT_GADGET_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef SET_COMMITMENT_GADGET_TCC_
// #define SET_COMMITMENT_GADGET_TCC_

use crate::common::data_structures::set_commitment;



impl set_commitment_gadget<FieldT, HashT>{
pub fn new(pb:protoboard<FieldT>,
                                                            max_entries:usize,
                                                            element_bits:&pb_variable_array<FieldT>,
                                                            root_digest:&set_commitment_variable<FieldT, HashT>,
                                                            proof:&set_membership_proof_variable<FieldT, HashT>,
                                                            check_successful:&pb_linear_combination<FieldT>,
                                                            annotation_prefix:&String)->Self
   
{
    element_block.reset(block_variable::<FieldT>::new(pb, { element_bits }, FMT(annotation_prefix, " element_block")));

    if tree_depth == 0
    {
        hash_element.reset(HashT::new(pb, element_bits.len(), *element_block, root_digest, FMT(annotation_prefix, " hash_element")));
    }
    else
    {
        element_digest.reset(digest_variable::<FieldT>::new(pb, HashT::get_digest_len(),
                                                         FMT(annotation_prefix, " element_digest")));
        hash_element.reset(HashT::new(pb, element_bits.len(), *element_block, *element_digest, FMT(annotation_prefix, " hash_element")));
        check_membership.reset(merkle_tree_check_read_gadget::<FieldT, HashT>::new(pb,
                                                                                tree_depth,
                                                                                proof.address_bits,
                                                                                *element_digest,
                                                                                root_digest,
                                                                                *proof.merkle_path,
                                                                                check_successful,
                                                                                FMT(annotation_prefix, " check_membership")));
    }
    //  gadget<FieldT>(pb, annotation_prefix), 
    Self{tree_depth:ffec::log2(max_entries),element_bits,
   root_digest,proof,check_successful}
}


pub fn generate_r1cs_constraints()
{
    hash_element.generate_r1cs_constraints();

    if tree_depth > 0
    {
        check_membership.generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    hash_element.generate_r1cs_witness();

    if tree_depth > 0
    {
        check_membership.generate_r1cs_witness();
    }
}


 pub fn root_size_in_bits()->usize
{
    return merkle_tree_check_read_gadget::<FieldT, HashT>::root_size_in_bits();
}
}

pub fn  test_set_commitment_gadget()
{
    let digest_len = HashT::get_digest_len();
    let max_set_size = 16;
    let value_size =  (if HashT::get_block_len() > 0 {HashT::get_block_len()} else{10});

    let mut accumulator=set_commitment_accumulator::<HashT> ::new(max_set_size, value_size);

    let mut set_elems=vec![];
    for i in 0..max_set_size
    {
        let mut  elem:Vec<_>=(0..value_size).map(|_|std::rand() % 2);
        
        set_elems.push(elem);
        accumulator.add(elem);
        assert!(accumulator.is_in_set(elem));
    }

    let mut  pb=protoboard::<FieldT> ::new();
    let mut  element_bits=pb_variable_array::<FieldT>::new();
    element_bits.allocate(pb, value_size, "element_bits");
    let mut  root_digest=set_commitment_variable::<FieldT, HashT>::new(pb, digest_len, "root_digest");

   let mut check_succesful= pb_variable::<FieldT>::new();
    check_succesful.allocate(pb, "check_succesful");

    let mut proof= set_membership_proof_variable::<FieldT, HashT>::new(pb, max_set_size, "proof");

    let mut  sc=set_commitment_gadget::<FieldT, HashT>::new(pb, max_set_size, element_bits, root_digest, proof, check_succesful, "sc");
    sc.generate_r1cs_constraints();

    /* test all elements from set */
    for i in 0..max_set_size
    {
        element_bits.fill_with_bits(pb, set_elems[i]);
        pb.val(check_succesful) = FieldT::one();
        proof.generate_r1cs_witness(accumulator.get_membership_proof(set_elems[i]));
        sc.generate_r1cs_witness();
        root_digest.generate_r1cs_witness(accumulator.get_commitment());
        assert!(pb.is_satisfied());
    }
    print!("membership tests OK\n");

    /* test an element not in set */
    for i in 0..value_size
    {
        pb.val(element_bits[i]) = FieldT(std::rand() % 2);
    }

    pb.val(check_succesful) = FieldT::zero(); /* do not require the check result to be successful */
    proof.generate_r1cs_witness(accumulator.get_membership_proof(set_elems[0])); /* try it with invalid proof */
    sc.generate_r1cs_witness();
    root_digest.generate_r1cs_witness(accumulator.get_commitment());
    assert!(pb.is_satisfied());

    pb.val(check_succesful) = FieldT::one(); /* now require the check result to be succesful */
    proof.generate_r1cs_witness(accumulator.get_membership_proof(set_elems[0])); /* try it with invalid proof */
    sc.generate_r1cs_witness();
    root_digest.generate_r1cs_witness(accumulator.get_commitment());
    assert!(!pb.is_satisfied()); /* the protoboard should be unsatisfied */
    print!("non-membership test OK\n");
}



//#endif // SET_COMMITMENT_GADGET_TCC_
