/** @file
 *****************************************************************************

 Declaration of interfaces for the the R1CS ppzkSNARK verifier gadget.

 The gadget r1cs_ppzksnark_verifier_gadget verifiers correct computation of r1cs_ppzksnark_verifier_strong_IC.
 The gadget is built from two main sub-gadgets:
 - r1cs_ppzksnark_verifier_process_vk_gadget, which verifies correct computation of r1cs_ppzksnark_verifier_process_vk, and
 - r1cs_ppzksnark_online_verifier_gadget, which verifies correct computation of r1cs_ppzksnark_online_verifier_strong_IC.
 See r1cs_ppzksnark.hpp for description of the aforementioned functions.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_PPZKSNARK_VERIFIER_GADGET_HPP_
// #define R1CS_PPZKSNARK_VERIFIER_GADGET_HPP_

use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget;
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_checks;
use crate::gadgetlib1::gadgets::pairing::pairing_params;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;




pub struct r1cs_ppzksnark_proof_variable<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;

g_A_g:    RcCell<G1_variable<ppT> >,
g_A_h:    RcCell<G1_variable<ppT> >,
g_B_g:    RcCell<G2_variable<ppT> >,
g_B_h:    RcCell<G1_variable<ppT> >,
g_C_g:    RcCell<G1_variable<ppT> >,
g_C_h:    RcCell<G1_variable<ppT> >,
g_H:    RcCell<G1_variable<ppT> >,
g_K:    RcCell<G1_variable<ppT> >,

all_G1_vars:    Vec<RcCell<G1_variable<ppT> > >,
all_G2_vars:    Vec<RcCell<G2_variable<ppT> > >,

all_G1_checkers:    Vec<RcCell<G1_checker_gadget<ppT> > >,
G2_checker:    RcCell<G2_checker_gadget<ppT> >,

proof_contents:    pb_variable_array<FieldT>,

}


pub struct r1cs_ppzksnark_verification_key_variable<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;

alphaA_g2:    RcCell<G2_variable<ppT> >,
alphaB_g1:    RcCell<G1_variable<ppT> >,
alphaC_g2:    RcCell<G2_variable<ppT> >,
gamma_g2:    RcCell<G2_variable<ppT> >,
gamma_beta_g1:    RcCell<G1_variable<ppT> >,
gamma_beta_g2:    RcCell<G2_variable<ppT> >,
rC_Z_g2:    RcCell<G2_variable<ppT> >,
encoded_IC_base:    RcCell<G1_variable<ppT> >,
encoded_IC_query:    Vec<RcCell<G1_variable<ppT> > >,

all_bits:    pb_variable_array<FieldT>,
all_vars:    pb_linear_combination_array<FieldT>,
input_size:    usize,

all_G1_vars:    Vec<RcCell<G1_variable<ppT> > >,
all_G2_vars:    Vec<RcCell<G2_variable<ppT> > >,

packer:    RcCell<multipacking_gadget<FieldT> >,

    // Unfortunately, g++ 4.9 and g++ 5.0 have a bug related to
    // incorrect inlining of small functions:
    // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=65307, which
    // produces wrong assembly even at -O1. The test case at the bug
    // report is directly derived from this code here. As a temporary
    // work-around we mark the key functions noinline to hint compiler
    // that inlining should not be performed.

    // TODO: remove later, when g++ developers fix the bug.

}


pub struct r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> {

    // type FieldT=ffec::Fr<ppT>;

encoded_IC_base:    RcCell<G1_variable<ppT> >,
encoded_IC_query:    Vec<RcCell<G1_variable<ppT> > >,

vk_alphaB_g1_precomp:    RcCell<G1_precomputation<ppT> >,
vk_gamma_beta_g1_precomp:    RcCell<G1_precomputation<ppT> >,

pp_G2_one_precomp:    RcCell<G2_precomputation<ppT> >,
vk_alphaA_g2_precomp:    RcCell<G2_precomputation<ppT> >,
vk_alphaC_g2_precomp:    RcCell<G2_precomputation<ppT> >,
vk_gamma_beta_g2_precomp:    RcCell<G2_precomputation<ppT> >,
vk_gamma_g2_precomp:    RcCell<G2_precomputation<ppT> >,
vk_rC_Z_g2_precomp:    RcCell<G2_precomputation<ppT> >,

}


pub struct r1cs_ppzksnark_verifier_process_vk_gadget<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;

compute_vk_alphaB_g1_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_vk_gamma_beta_g1_precomp:    RcCell<precompute_G1_gadget<ppT> >,

compute_vk_alphaA_g2_precomp:    RcCell<precompute_G2_gadget<ppT> >,
compute_vk_alphaC_g2_precomp:    RcCell<precompute_G2_gadget<ppT> >,
compute_vk_gamma_beta_g2_precomp:    RcCell<precompute_G2_gadget<ppT> >,
compute_vk_gamma_g2_precomp:    RcCell<precompute_G2_gadget<ppT> >,
compute_vk_rC_Z_g2_precomp:    RcCell<precompute_G2_gadget<ppT> >,

vk:    r1cs_ppzksnark_verification_key_variable<ppT>,
   pvk: r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> , // important to have a reference here

   
}


pub struct r1cs_ppzksnark_online_verifier_gadget<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;

pvk:    r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT>,

input:    pb_variable_array<FieldT>,
elt_size:    usize,
proof:    r1cs_ppzksnark_proof_variable<ppT>,
result:    pb_variable<FieldT>,
    input_len:usize,

acc:    RcCell<G1_variable<ppT> >,
accumulate_input:    RcCell<G1_multiscalar_mul_gadget<ppT> >,

proof_g_A_g_acc:    RcCell<G1_variable<ppT> >,
compute_proof_g_A_g_acc:    RcCell<G1_add_gadget<ppT> >,
proof_g_A_g_acc_C:    RcCell<G1_variable<ppT> >,
compute_proof_g_A_g_acc_C:    RcCell<G1_add_gadget<ppT> >,

proof_g_A_h_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_A_g_acc_C_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_A_g_acc_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_A_g_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_B_h_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_C_h_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_C_g_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_K_precomp:    RcCell<G1_precomputation<ppT> >,
proof_g_H_precomp:    RcCell<G1_precomputation<ppT> >,

proof_g_B_g_precomp:    RcCell<G2_precomputation<ppT> >,

compute_proof_g_A_h_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_A_g_acc_C_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_A_g_acc_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_A_g_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_B_h_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_C_h_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_C_g_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_K_precomp:    RcCell<precompute_G1_gadget<ppT> >,
compute_proof_g_H_precomp:    RcCell<precompute_G1_gadget<ppT> >,

compute_proof_g_B_g_precomp:    RcCell<precompute_G2_gadget<ppT> >,

check_kc_A_valid:    RcCell<check_e_equals_e_gadget<ppT> >,
check_kc_B_valid:    RcCell<check_e_equals_e_gadget<ppT> >,
check_kc_C_valid:    RcCell<check_e_equals_e_gadget<ppT> >,
check_QAP_valid:    RcCell<check_e_equals_ee_gadget<ppT> >,
check_CC_valid:    RcCell<check_e_equals_ee_gadget<ppT> >,

kc_A_valid:    pb_variable<FieldT>,
kc_B_valid:    pb_variable<FieldT>,
kc_C_valid:    pb_variable<FieldT>,
QAP_valid:    pb_variable<FieldT>,
CC_valid:    pb_variable<FieldT>,

all_test_results:    pb_variable_array<FieldT>,
all_tests_pass:    RcCell<conjunction_gadget<FieldT> >,

}


pub struct r1cs_ppzksnark_verifier_gadget<ppT> {//gadget<ffec::Fr<ppT> >

    // type FieldT=ffec::Fr<ppT>;

pvk:    RcCell<r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> >,
compute_pvk:    RcCell<r1cs_ppzksnark_verifier_process_vk_gadget<ppT> >,
online_verifier:    RcCell<r1cs_ppzksnark_online_verifier_gadget<ppT> >,

}



// use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget;

//#endif // R1CS_PPZKSNARK_VERIFIER_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the the R1CS ppzkSNARK verifier gadget.

 See r1cs_ppzksnark_verifier_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_PPZKSNARK_VERIFIER_GADGET_TCC_
// #define R1CS_PPZKSNARK_VERIFIER_GADGET_TCC_

use crate::gadgetlib1::constraint_profiling;



impl r1cs_ppzksnark_proof_variable<ppT> {
pub fn new(pb:protoboard<FieldT>,
                                                                  annotation_prefix:&String)->Self
    
{
    let num_G1 = 7;
    let num_G2 = 1;

    g_A_g.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_A_g")));
    g_A_h.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_A_h")));
    g_B_g.reset(G2_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_B_g")));
    g_B_h.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_B_h")));
    g_C_g.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_C_g")));
    g_C_h.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_C_h")));
    g_H.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_H")));
    g_K.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " g_K")));

    all_G1_vars =  vec![g_A_g, g_A_h, g_B_h, g_C_g, g_C_h, g_H,g_K ];
    all_G2_vars =  vec![g_B_g] ;

    all_G1_checkers.resize(all_G1_vars.len());

    for i in 0..all_G1_vars.len()
    {
        all_G1_checkers[i].reset(G1_checker_gadget::<ppT>::new(pb, *all_G1_vars[i], FMT(annotation_prefix, " all_G1_checkers_{}", i)));
    }
    G2_checker.reset(G2_checker_gadget::<ppT>::new(pb, *g_B_g, FMT(annotation_prefix, " G2_checker")));

    assert!(all_G1_vars.len() == num_G1);
    assert!(all_G2_vars.len() == num_G2);
    // gadget<FieldT>(pb, annotation_prefix)
    Self{}
}


pub fn generate_r1cs_constraints()
{
    for G1_checker in &all_G1_checkers
    {
        G1_checker.generate_r1cs_constraints();
    }

    G2_checker.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness(proof:&r1cs_ppzksnark_proof<other_curve::<ppT> >)
{
    let G1_elems = vec![ proof.g_A.g, proof.g_A.h, proof.g_B.h, proof.g_C.g, proof.g_C.h, proof.g_H, proof.g_K] ;
    let G2_elems =  vec![proof.g_B.g ];

    assert!(G1_elems.len() == all_G1_vars.len());
    assert!(G2_elems.len() == all_G2_vars.len());

    for i in 0..G1_elems.len()
    {
        all_G1_vars[i].generate_r1cs_witness(G1_elems[i]);
    }

    for i in 0..G2_elems.len()
    {
        all_G2_vars[i].generate_r1cs_witness(G2_elems[i]);
    }

    for G1_checker in &all_G1_checkers
    {
        G1_checker.generate_r1cs_witness();
    }

    G2_checker.generate_r1cs_witness();
}


pub fn size()->usize
{
    let num_G1 = 7;
    let num_G2 = 1;
    return (num_G1 * G1_variable::<ppT>::num_field_elems + num_G2 * G2_variable::<ppT>::num_field_elems);
}
}

impl r1cs_ppzksnark_verification_key_variable<ppT> {
pub fn new(pb:protoboard<FieldT>,
                                                                                        all_bits:&pb_variable_array<FieldT>,
                                                                                        input_size:usize,
                                                                                        annotation_prefix:&String)->Self
   
{
    let num_G1 = 2 + (input_size + 1);
    let num_G2 = 5;

    assert!(all_bits.len() == (G1_variable::<ppT>::size_in_bits() * num_G1 + G2_variable::<ppT>::size_in_bits() * num_G2));

    self.alphaA_g2.reset(G2_variable::<ppT>::new(pb, FMT(annotation_prefix, " alphaA_g2")));
    self.alphaB_g1.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " alphaB_g1")));
    self.alphaC_g2.reset(G2_variable::<ppT>::new(pb, FMT(annotation_prefix, " alphaC_g2")));
    self.gamma_g2.reset(G2_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma_g2")));
    self.gamma_beta_g1.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma_beta_g1")));
    self.gamma_beta_g2.reset(G2_variable::<ppT>::new(pb, FMT(annotation_prefix, " gamma_beta_g2")));
    self.rC_Z_g2.reset(G2_variable::<ppT>::new(pb, FMT(annotation_prefix, " rC_Z_g2")));

    all_G1_vars =  vec![self.alphaB_g1, self.gamma_beta_g1 ];
    all_G2_vars =  vec![self.alphaA_g2, self.alphaC_g2, self.gamma_g2, self.gamma_beta_g2, self.rC_Z_g2] ;

    self.encoded_IC_query.resize(input_size);
    self.encoded_IC_base.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " encoded_IC_base")));
    self.all_G1_vars.push(self.encoded_IC_base);

    for i in 0..input_size
    {
        self.encoded_IC_query[i].reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " encoded_IC_query_{}", i)));
        all_G1_vars.push(self.encoded_IC_query[i]);
    }

    for G1_var in &all_G1_vars
    {
        all_vars.insert(all_vars.end(), G1_var.all_vars.begin(), G1_var.all_vars.end());
    }

    for G2_var in &all_G2_vars
    {
        all_vars.insert(all_vars.end(), G2_var.all_vars.begin(), G2_var.all_vars.end());
    }

    assert!(all_G1_vars.len() == num_G1);
    assert!(all_G2_vars.len() == num_G2);
    assert!(all_vars.len() == (num_G1 * G1_variable::<ppT>::num_variables() + num_G2 * G2_variable::<ppT>::num_variables()));

    packer.reset(multipacking_gadget::<FieldT>::new(pb, all_bits, all_vars, FieldT::size_in_bits(), FMT(annotation_prefix, " packer")));
    //  gadget<FieldT>(pb, annotation_prefix),
   Self{all_bits,
    input_size}
}


pub fn generate_r1cs_constraints(enforce_bitness:bool)
{
    packer.generate_r1cs_constraints(enforce_bitness);
}


pub fn generate_r1cs_witness(vk:&r1cs_ppzksnark_verification_key<other_curve::<ppT> >)
{


    let mut G1_elems =  vec![vk.alphaB_g1, vk.gamma_beta_g1] ;
    let mut G2_elems =  vec![vk.alphaA_g2, vk.alphaC_g2, vk.gamma_g2, vk.gamma_beta_g2, vk.rC_Z_g2] ;

    assert!(vk.encoded_IC_query.rest.indices.len() == input_size);
    G1_elems.push(vk.encoded_IC_query.first);
    for i in 0..input_size
    {
        assert!(vk.encoded_IC_query.rest.indices[i] == i);
        G1_elems.push(vk.encoded_IC_query.rest.values[i]);
    }

    assert!(G1_elems.len() == all_G1_vars.len());
    assert!(G2_elems.len() == all_G2_vars.len());

    for i in 0..G1_elems.len()
    {
        all_G1_vars[i].generate_r1cs_witness(G1_elems[i]);
    }

    for i in 0..G2_elems.len()
    {
        all_G2_vars[i].generate_r1cs_witness(G2_elems[i]);
    }

    packer.generate_r1cs_witness_from_packed();
}


pub fn generate_r1cs_witness(vk_bits:&bit_vector)
{
    all_bits.fill_with_bits(self.pb, vk_bits);
    packer.generate_r1cs_witness_from_bits();
}


pub fn get_bits()->bit_vector
{
    return all_bits.get_bits(self.pb);
}


pub fn size_in_bits(input_size:usize)->usize
{
    let num_G1 = 2 + (input_size + 1);
    let num_G2 = 5;
    let result = G1_variable::<ppT>::size_in_bits() * num_G1 + G2_variable::<ppT>::size_in_bits() * num_G2;
    print!("G1_size_in_bits = {}, G2_size_in_bits = {}\n", G1_variable::<ppT>::size_in_bits(), G2_variable::<ppT>::size_in_bits());
    print!("r1cs_ppzksnark_verification_key_variable::<ppT>::size_in_bits({}) = {}\n", input_size, result);
    return result;
}




pub fn get_verification_key_bits(r1cs_vk:&r1cs_ppzksnark_verification_key<other_curve::<ppT> >)->bit_vector 
{
    type FieldT=ffec::Fr<ppT>;

    let  input_size_in_elts = r1cs_vk.encoded_IC_query.rest.indices.len(); // this might be approximate for bound verification keys, however they are not supported by r1cs_ppzksnark_verification_key_variable
    let vk_size_in_bits = r1cs_ppzksnark_verification_key_variable::<ppT>::size_in_bits(input_size_in_elts);

    let mut  pb=protoboard::<FieldT> ::new();
    let mut  vk_bits=pb_variable_array::<FieldT>::new();
    vk_bits.allocate(pb, vk_size_in_bits, "vk_bits");
    let mut  vk=r1cs_ppzksnark_verification_key_variable::<ppT>::new(pb, vk_bits, input_size_in_elts, "translation_step_vk");
    vk.generate_r1cs_witness(r1cs_vk);

    return vk.get_bits();
}
}

impl r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT>{

// pub fn new()
// {
//     // will be allocated outside
// }


pub fn new(pb:protoboard<FieldT>,
                                                                                                                                                r1cs_vk:&r1cs_ppzksnark_verification_key<other_curve::<ppT> >,
                                                                                                                                                annotation_prefix:&String)
{
    encoded_IC_base.reset(G1_variable::<ppT>::new(pb, r1cs_vk.encoded_IC_query.first, FMT(annotation_prefix, " encoded_IC_base")));
    encoded_IC_query.resize(r1cs_vk.encoded_IC_query.rest.indices.len());
    for i in 0..r1cs_vk.encoded_IC_query.rest.indices.len()
    {
        assert!(r1cs_vk.encoded_IC_query.rest.indices[i] == i);
        encoded_IC_query[i].reset(G1_variable::<ppT>::new(pb, r1cs_vk.encoded_IC_query.rest.values[i], FMT(annotation_prefix, " encoded_IC_query")));
    }

    vk_alphaB_g1_precomp.reset(G1_precomputation::<ppT>::new(pb, r1cs_vk.alphaB_g1, FMT(annotation_prefix, " vk_alphaB_g1_precomp")));
    vk_gamma_beta_g1_precomp.reset(G1_precomputation::<ppT>::new(pb, r1cs_vk.gamma_beta_g1, FMT(annotation_prefix, " vk_gamma_beta_g1_precomp")));

    pp_G2_one_precomp.reset(G2_precomputation::<ppT>::new(pb, ffec::G2::<other_curve::<ppT> >::one(), FMT(annotation_prefix, " pp_G2_one_precomp")));
    vk_alphaA_g2_precomp.reset(G2_precomputation::<ppT>::new(pb, r1cs_vk.alphaA_g2, FMT(annotation_prefix, " vk_alphaA_g2_precomp")));
    vk_alphaC_g2_precomp.reset(G2_precomputation::<ppT>::new(pb, r1cs_vk.alphaC_g2, FMT(annotation_prefix, " vk_alphaC_g2_precomp")));
    vk_gamma_beta_g2_precomp.reset(G2_precomputation::<ppT>::new(pb, r1cs_vk.gamma_beta_g2, FMT(annotation_prefix, " vk_gamma_beta_g2_precomp")));
    vk_gamma_g2_precomp.reset(G2_precomputation::<ppT>::new(pb, r1cs_vk.gamma_g2, FMT(annotation_prefix, " vk_gamma_g2_precomp")));
    vk_rC_Z_g2_precomp.reset(G2_precomputation::<ppT>::new(pb, r1cs_vk.rC_Z_g2, FMT(annotation_prefix, " vk_rC_Z_g2_precomp")));
}
}
impl r1cs_ppzksnark_verifier_process_vk_gadget<ppT> {

pub fn new(pb:protoboard<FieldT>,
                                                                                          vk:&r1cs_ppzksnark_verification_key_variable<ppT>,
                                                                                          pvk:&r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> ,
                                                                                          annotation_prefix:&String)->Self
  
{
    pvk.encoded_IC_base = vk.encoded_IC_base;
    pvk.encoded_IC_query = vk.encoded_IC_query;

    pvk.vk_alphaB_g1_precomp.reset(G1_precomputation::<ppT>::new());
    pvk.vk_gamma_beta_g1_precomp.reset(G1_precomputation::<ppT>::new());

    pvk.pp_G2_one_precomp.reset(G2_precomputation::<ppT>::new());
    pvk.vk_alphaA_g2_precomp.reset(G2_precomputation::<ppT>::new());
    pvk.vk_alphaC_g2_precomp.reset(G2_precomputation::<ppT>::new());
    pvk.vk_gamma_beta_g2_precomp.reset(G2_precomputation::<ppT>::new());
    pvk.vk_gamma_g2_precomp.reset(G2_precomputation::<ppT>::new());
    pvk.vk_rC_Z_g2_precomp.reset(G2_precomputation::<ppT>::new());

    compute_vk_alphaB_g1_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *vk.alphaB_g1, *pvk.vk_alphaB_g1_precomp, FMT(annotation_prefix, " compute_vk_alphaB_g1_precomp")));
    compute_vk_gamma_beta_g1_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *vk.gamma_beta_g1, *pvk.vk_gamma_beta_g1_precomp, FMT(annotation_prefix, " compute_vk_gamma_beta_g1_precomp")));

    pvk.pp_G2_one_precomp.reset(G2_precomputation::<ppT>::new(pb, ffec::G2::<other_curve::<ppT> >::one(), FMT(annotation_prefix, " pp_G2_one_precomp")));
    compute_vk_alphaA_g2_precomp.reset(precompute_G2_gadget::<ppT>::new(pb, *vk.alphaA_g2, *pvk.vk_alphaA_g2_precomp, FMT(annotation_prefix, " compute_vk_alphaA_g2_precomp")));
    compute_vk_alphaC_g2_precomp.reset(precompute_G2_gadget::<ppT>::new(pb, *vk.alphaC_g2, *pvk.vk_alphaC_g2_precomp, FMT(annotation_prefix, " compute_vk_alphaC_g2_precomp")));
    compute_vk_gamma_beta_g2_precomp.reset(precompute_G2_gadget::<ppT>::new(pb, *vk.gamma_beta_g2, *pvk.vk_gamma_beta_g2_precomp, FMT(annotation_prefix, " compute_vk_gamma_beta_g2_precomp")));
    compute_vk_gamma_g2_precomp.reset(precompute_G2_gadget::<ppT>::new(pb, *vk.gamma_g2, *pvk.vk_gamma_g2_precomp, FMT(annotation_prefix, " compute_vk_gamma_g2_precomp")));
    compute_vk_rC_Z_g2_precomp.reset(precompute_G2_gadget::<ppT>::new(pb, *vk.rC_Z_g2, *pvk.vk_rC_Z_g2_precomp, FMT(annotation_prefix, " compute_vk_rC_Z_g2_precomp")));
    //   gadget<FieldT>(pb, annotation_prefix),
   Self{vk,
    pvk}
}


pub fn generate_r1cs_constraints()
{
    compute_vk_alphaB_g1_precomp.generate_r1cs_constraints();
    compute_vk_gamma_beta_g1_precomp.generate_r1cs_constraints();

    compute_vk_alphaA_g2_precomp.generate_r1cs_constraints();
    compute_vk_alphaC_g2_precomp.generate_r1cs_constraints();
    compute_vk_gamma_beta_g2_precomp.generate_r1cs_constraints();
    compute_vk_gamma_g2_precomp.generate_r1cs_constraints();
    compute_vk_rC_Z_g2_precomp.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_vk_alphaB_g1_precomp.generate_r1cs_witness();
    compute_vk_gamma_beta_g1_precomp.generate_r1cs_witness();

    compute_vk_alphaA_g2_precomp.generate_r1cs_witness();
    compute_vk_alphaC_g2_precomp.generate_r1cs_witness();
    compute_vk_gamma_beta_g2_precomp.generate_r1cs_witness();
    compute_vk_gamma_g2_precomp.generate_r1cs_witness();
    compute_vk_rC_Z_g2_precomp.generate_r1cs_witness();
}
}

impl  r1cs_ppzksnark_online_verifier_gadget<ppT>{
pub fn new(pb:protoboard<FieldT>,
                                                                                  pvk:&r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT>,
                                                                                  input:&pb_variable_array<FieldT>,
                                                                                  elt_size:usize,
                                                                                  proof:&r1cs_ppzksnark_proof_variable<ppT>,
                                                                                  result:&pb_variable<FieldT>,
                                                                                  annotation_prefix:&String)->Self
    
{
    // accumulate input and store base in acc
    acc.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " acc")));
    let mut  IC_terms=vec![];
    for i in 0..pvk.encoded_IC_query.len()
    {
        IC_terms.push(*(pvk.encoded_IC_query[i]));
    }
    accumulate_input.reset(G1_multiscalar_mul_gadget::<ppT>::new(pb, *(pvk.encoded_IC_base), input, elt_size, IC_terms, *acc, FMT(annotation_prefix, " accumulate_input")));

    // allocate results for precomputation
    proof_g_A_h_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_A_g_acc_C_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_A_g_acc_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_A_g_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_B_h_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_C_h_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_C_g_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_K_precomp.reset(G1_precomputation::<ppT>::new());
    proof_g_H_precomp.reset(G1_precomputation::<ppT>::new());

    proof_g_B_g_precomp.reset(G2_precomputation::<ppT>::new());

    // do the necessary precomputations
    // compute things not available in plain from proof/vk
    proof_g_A_g_acc.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " proof_g_A_g_acc")));
    compute_proof_g_A_g_acc.reset(G1_add_gadget::<ppT>::new(pb, *(proof.g_A_g), *acc , *proof_g_A_g_acc, FMT(annotation_prefix, " compute_proof_g_A_g_acc")));
    proof_g_A_g_acc_C.reset(G1_variable::<ppT>::new(pb, FMT(annotation_prefix, " proof_g_A_g_acc_C")));
    compute_proof_g_A_g_acc_C.reset(G1_add_gadget::<ppT>::new(pb, *proof_g_A_g_acc, *(proof.g_C_g) , *proof_g_A_g_acc_C, FMT(annotation_prefix, " compute_proof_g_A_g_acc_C")));

    compute_proof_g_A_g_acc_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *proof_g_A_g_acc, *proof_g_A_g_acc_precomp, FMT(annotation_prefix, " compute_proof_g_A_g_acc_precomp")));
    compute_proof_g_A_g_acc_C_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *proof_g_A_g_acc_C, *proof_g_A_g_acc_C_precomp, FMT(annotation_prefix, " compute_proof_g_A_g_acc_C_precomp")));

    // do other precomputations
    compute_proof_g_A_h_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_A_h), *proof_g_A_h_precomp, FMT(annotation_prefix, " compute_proof_g_A_h_precomp")));
    compute_proof_g_A_g_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_A_g), *proof_g_A_g_precomp, FMT(annotation_prefix, " compute_proof_g_A_g_precomp")));
    compute_proof_g_B_h_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_B_h), *proof_g_B_h_precomp, FMT(annotation_prefix, " compute_proof_g_B_h_precomp")));
    compute_proof_g_C_h_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_C_h), *proof_g_C_h_precomp, FMT(annotation_prefix, " compute_proof_g_C_h_precomp")));
    compute_proof_g_C_g_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_C_g), *proof_g_C_g_precomp, FMT(annotation_prefix, " compute_proof_g_C_g_precomp")));
    compute_proof_g_H_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_H), *proof_g_H_precomp, FMT(annotation_prefix, " compute_proof_g_H_precomp")));
    compute_proof_g_K_precomp.reset(precompute_G1_gadget::<ppT>::new(pb, *(proof.g_K), *proof_g_K_precomp, FMT(annotation_prefix, " compute_proof_g_K_precomp")));
    compute_proof_g_B_g_precomp.reset(precompute_G2_gadget::<ppT>::new(pb, *(proof.g_B_g), *proof_g_B_g_precomp, FMT(annotation_prefix, " compute_proof_g_B_g_precomp")));

    // check validity of A knowledge commitment
    kc_A_valid.allocate(pb, FMT(annotation_prefix, " kc_A_valid"));
    check_kc_A_valid.reset(check_e_equals_e_gadget::<ppT>::new(pb, *proof_g_A_g_precomp, *(pvk.vk_alphaA_g2_precomp), *proof_g_A_h_precomp, *(pvk.pp_G2_one_precomp), kc_A_valid, FMT(annotation_prefix, " check_kc_A_valid")));

    // check validity of B knowledge commitment
    kc_B_valid.allocate(pb, FMT(annotation_prefix, " kc_B_valid"));
    check_kc_B_valid.reset(check_e_equals_e_gadget::<ppT>::new(pb, *(pvk.vk_alphaB_g1_precomp), *proof_g_B_g_precomp, *proof_g_B_h_precomp, *(pvk.pp_G2_one_precomp), kc_B_valid, FMT(annotation_prefix, " check_kc_B_valid")));

    // check validity of C knowledge commitment
    kc_C_valid.allocate(pb, FMT(annotation_prefix, " kc_C_valid"));
    check_kc_C_valid.reset(check_e_equals_e_gadget::<ppT>::new(pb, *proof_g_C_g_precomp, *(pvk.vk_alphaC_g2_precomp), *proof_g_C_h_precomp, *(pvk.pp_G2_one_precomp), kc_C_valid, FMT(annotation_prefix, " check_kc_C_valid")));

    // check QAP divisibility
    QAP_valid.allocate(pb, FMT(annotation_prefix, " QAP_valid"));
    check_QAP_valid.reset(check_e_equals_ee_gadget::<ppT>::new(pb, *proof_g_A_g_acc_precomp, *proof_g_B_g_precomp, *proof_g_H_precomp, *(pvk.vk_rC_Z_g2_precomp), *proof_g_C_g_precomp, *(pvk.pp_G2_one_precomp), QAP_valid, FMT(annotation_prefix, " check_QAP_valid")));

    // check coefficients
    CC_valid.allocate(pb, FMT(annotation_prefix, " CC_valid"));
    check_CC_valid.reset(check_e_equals_ee_gadget::<ppT>::new(pb, *proof_g_K_precomp, *(pvk.vk_gamma_g2_precomp), *proof_g_A_g_acc_C_precomp, *(pvk.vk_gamma_beta_g2_precomp), *(pvk.vk_gamma_beta_g1_precomp), *proof_g_B_g_precomp, CC_valid, FMT(annotation_prefix, " check_CC_valid")));

    // final constraint
    all_test_results.push(kc_A_valid);
    all_test_results.push(kc_B_valid);
    all_test_results.push(kc_C_valid);
    all_test_results.push(QAP_valid);
    all_test_results.push(CC_valid);

    all_tests_pass.reset(conjunction_gadget::<FieldT>::new(pb, all_test_results, result, FMT(annotation_prefix, " all_tests_pass")));
    // gadget<FieldT>(pb, annotation_prefix),
   Self{pvk,
   input,
   elt_size,
   proof,
   result,
    input_len:input.len()}
}


pub fn generate_r1cs_constraints()
{
    PROFILE_CONSTRAINTS(self.pb, "accumulate verifier input");
    {
        ffec::print_indent(); print!("* Number of bits as an input to verifier gadget: {}\n", input.len());
        accumulate_input.generate_r1cs_constraints();
    }

    PROFILE_CONSTRAINTS(self.pb, "rest of the verifier");
    {
        compute_proof_g_A_g_acc.generate_r1cs_constraints();
        compute_proof_g_A_g_acc_C.generate_r1cs_constraints();

        compute_proof_g_A_g_acc_precomp.generate_r1cs_constraints();
        compute_proof_g_A_g_acc_C_precomp.generate_r1cs_constraints();

        compute_proof_g_A_h_precomp.generate_r1cs_constraints();
        compute_proof_g_A_g_precomp.generate_r1cs_constraints();
        compute_proof_g_B_h_precomp.generate_r1cs_constraints();
        compute_proof_g_C_h_precomp.generate_r1cs_constraints();
        compute_proof_g_C_g_precomp.generate_r1cs_constraints();
        compute_proof_g_H_precomp.generate_r1cs_constraints();
        compute_proof_g_K_precomp.generate_r1cs_constraints();
        compute_proof_g_B_g_precomp.generate_r1cs_constraints();

        check_kc_A_valid.generate_r1cs_constraints();
        check_kc_B_valid.generate_r1cs_constraints();
        check_kc_C_valid.generate_r1cs_constraints();
        check_QAP_valid.generate_r1cs_constraints();
        check_CC_valid.generate_r1cs_constraints();

        all_tests_pass.generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    accumulate_input.generate_r1cs_witness();

    compute_proof_g_A_g_acc.generate_r1cs_witness();
    compute_proof_g_A_g_acc_C.generate_r1cs_witness();

    compute_proof_g_A_g_acc_precomp.generate_r1cs_witness();
    compute_proof_g_A_g_acc_C_precomp.generate_r1cs_witness();

    compute_proof_g_A_h_precomp.generate_r1cs_witness();
    compute_proof_g_A_g_precomp.generate_r1cs_witness();
    compute_proof_g_B_h_precomp.generate_r1cs_witness();
    compute_proof_g_C_h_precomp.generate_r1cs_witness();
    compute_proof_g_C_g_precomp.generate_r1cs_witness();
    compute_proof_g_H_precomp.generate_r1cs_witness();
    compute_proof_g_K_precomp.generate_r1cs_witness();
    compute_proof_g_B_g_precomp.generate_r1cs_witness();

    check_kc_A_valid.generate_r1cs_witness();
    check_kc_B_valid.generate_r1cs_witness();
    check_kc_C_valid.generate_r1cs_witness();
    check_QAP_valid.generate_r1cs_witness();
    check_CC_valid.generate_r1cs_witness();

    all_tests_pass.generate_r1cs_witness();
}
}

impl  r1cs_ppzksnark_verifier_gadget<ppT> {


pub fn new(pb:protoboard<FieldT>,
                                                                    vk:&r1cs_ppzksnark_verification_key_variable<ppT>,
                                                                    input:&pb_variable_array<FieldT>,
                                                                    elt_size:usize,
                                                                    proof:&r1cs_ppzksnark_proof_variable<ppT>,
                                                                    result:&pb_variable<FieldT>,
                                                                    annotation_prefix:&String)->Self
    
{
    pvk.reset(r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable::<ppT>::new());
    compute_pvk.reset(r1cs_ppzksnark_verifier_process_vk_gadget::<ppT>::new(pb, vk, *pvk, FMT(annotation_prefix, " compute_pvk")));
    online_verifier.reset(r1cs_ppzksnark_online_verifier_gadget::<ppT>::new(pb, *pvk, input, elt_size, proof, result, FMT(annotation_prefix, " online_verifier")));
    // gadget<FieldT>(pb, annotation_prefix)
    Self{}
}


pub fn generate_r1cs_constraints()
{
    PROFILE_CONSTRAINTS(self.pb, "precompute pvk");
    {
        compute_pvk.generate_r1cs_constraints();
    }

    PROFILE_CONSTRAINTS(self.pb, "online verifier");
    {
        online_verifier.generate_r1cs_constraints();
    }
}


pub fn generate_r1cs_witness()
{
    compute_pvk.generate_r1cs_witness();
    online_verifier.generate_r1cs_witness();
}

}

//#endif // R1CS_PPZKSNARK_VERIFIER_GADGET_TCC_
