/** @file
 *****************************************************************************

 Declaration of interfaces for a *single-predicate* ppzkPCD for R1CS.

 This includes:
 - pub struct for proving key
 - pub struct for verification key
 - pub struct for processed verification key
 - pub struct for key pair (proving key & verification key)
 - pub struct for proof
 - generator algorithm
 - prover algorithm
 - verifier algorithm
 - online verifier algorithm

 The implementation follows, extends, and optimizes the approach described
 in \[BCTV14]. Thus, PCD is constructed from two "matched" ppzkSNARKs for R1CS.

 Acronyms:

 "R1CS" = "Rank-1 Constraint Systems"
 "ppzkSNARK" = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
 "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

 References:

 \[BCTV14]:
 "Scalable Zero Knowledge via Cycles of Elliptic Curves",
 Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
 CRYPTO 2014,
 <http://eprint.iacr.org/2014/595>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_SP_PPZKPCD_HPP_
// #define R1CS_SP_PPZKPCD_HPP_

// 

use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd_params;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;



/******************************** Proving key ********************************/


/**
 * A proving key for the R1CS (single-predicate) ppzkPCD.
 */
// 
   type A_pp= PCD_ppT::curve_A_pp;
    type B_pp= PCD_ppT::curve_B_pp;
pub struct r1cs_sp_ppzkpcd_proving_key<PCD_ppT> {

 

compliance_predicate:    r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,

compliance_step_r1cs_pk:    r1cs_ppzksnark_proving_key<A_pp>,
translation_step_r1cs_pk:    r1cs_ppzksnark_proving_key<B_pp>,

compliance_step_r1cs_vk:    r1cs_ppzksnark_verification_key<A_pp>,
translation_step_r1cs_vk:    r1cs_ppzksnark_verification_key<B_pp>,

}
impl r1cs_sp_ppzkpcd_proving_key<PCD_ppT> {
    pub fn new(compliance_predicate:r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,
compliance_step_r1cs_pk:                                r1cs_ppzksnark_proving_key<A_pp>,
translation_step_r1cs_pk:                                r1cs_ppzksnark_proving_key<B_pp>,
                                compliance_step_r1cs_vk:r1cs_ppzksnark_verification_key<A_pp>,
                                translation_step_r1cs_vk:r1cs_ppzksnark_verification_key<B_pp>) ->Self
       
    {
    Self{ compliance_predicate,
        compliance_step_r1cs_pk,
        translation_step_r1cs_pk,
        compliance_step_r1cs_vk,
        translation_step_r1cs_vk
    }}

}


/******************************* Verification key ****************************/


/**
 * A verification key for the R1CS (single-predicate) ppzkPCD.
 */
//    type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
pub struct r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {

 

compliance_step_r1cs_vk:    r1cs_ppzksnark_verification_key<A_pp>,
translation_step_r1cs_vk:    r1cs_ppzksnark_verification_key<B_pp>,
}
impl r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {
    pub fn new(compliance_step_r1cs_vk:r1cs_ppzksnark_verification_key<A_pp>,
                                     translation_step_r1cs_vk:r1cs_ppzksnark_verification_key<B_pp>) ->Self
       
    {
    Self{ compliance_step_r1cs_vk,
        translation_step_r1cs_vk}}

     pub fn size_in_bits() ->usize
    {
        return (compliance_step_r1cs_vk.size_in_bits()
                + translation_step_r1cs_vk.size_in_bits());
    }

   
}


/************************ Processed verification key *************************/


/**
 * A processed verification key for the R1CS (single-predicate) ppzkPCD.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
//  type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
pub struct r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> {

   

compliance_step_r1cs_pvk:    r1cs_ppzksnark_processed_verification_key<A_pp>,
translation_step_r1cs_pvk:    r1cs_ppzksnark_processed_verification_key<B_pp>,
translation_step_r1cs_vk_bits:    bit_vector,
}
impl r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> {
   
    pub fn new(
compliance_step_r1cs_pvk:r1cs_ppzksnark_processed_verification_key<A_pp>,
translation_step_r1cs_pvk:                                               r1cs_ppzksnark_processed_verification_key<B_pp>,
                                               translation_step_r1cs_vk_bits:bit_vector) ->Self
       
    {
    Self{
 compliance_step_r1cs_pvk,
        translation_step_r1cs_pvk,
        translation_step_r1cs_vk_bits
    }}


     pub fn size_in_bits() ->usize
    {
        return (compliance_step_r1cs_pvk.size_in_bits() +
                translation_step_r1cs_pvk.size_in_bits() +
                translation_step_r1cs_vk_bits.len());
    }

   }


/********************************* Key pair **********************************/

/**
 * A key pair for the R1CS (single-predicate) ppzkPC, which consists of a proving key and a verification key.
 */
//    type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
pub struct  r1cs_sp_ppzkpcd_keypair<PCD_ppT> {

 

pk:    r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
vk:    r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
}
impl r1cs_sp_ppzkpcd_keypair<PCD_ppT> {
    
    pub fn new(
pk:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
vk:                            r1cs_sp_ppzkpcd_verification_key<PCD_ppT>) ->Self
       
    {
        Self{ pk,
        vk
    }}
    pub fn new2(
kp_A:r1cs_ppzksnark_keypair<A_pp>,
kp_B:                            r1cs_ppzksnark_keypair<B_pp>) ->Self
       
    {
    Self{
     pk:r1cs_sp_ppzkpcd_proving_key::<PCD_ppT>::new(kp_A.pk,kp_B.pk),
        vk:r1cs_sp_ppzkpcd_verification_key::<PCD_ppT>::new(kp_A.vk,kp_B.vk)
    }}
}


/*********************************** Proof ***********************************/

/**
 * A proof for the R1CS (single-predicate) ppzkPCD.
 */
// 
// using r1cs_sp_ppzkpcd_proof = r1cs_ppzksnark_proof< PCD_ppT::curve_B_pp>;


// /***************************** Main algorithms *******************************/

// /**
//  * A generator algorithm for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Given a compliance predicate, this algorithm produces proving and verification keys for the predicate.
//  */
// 
// r1cs_sp_ppzkpcd_keypair<PCD_ppT> r1cs_sp_ppzkpcd_generator(compliance_predicate:r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>);

// /**
//  * A prover algorithm for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Given a proving key, inputs for the compliance predicate, and proofs for
//  * the predicate's input messages, this algorithm produces a proof (of knowledge)
//  * that attests to the compliance of the output message.
//  */
// 
// r1cs_sp_ppzkpcd_proof<PCD_ppT> r1cs_sp_ppzkpcd_prover(pk:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
//                                                       primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
//                                                       auxiliary_input:r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT>,
//                                                       incoming_proofs:Vec<r1cs_sp_ppzkpcd_proof<PCD_ppT> >);

// /*
//  Below are two variants of verifier algorithm for the R1CS (single-predicate) ppzkPCD.

//  These are the two cases that arise from whether the verifier accepts a
//  (non-processed) verification key or, instead, a processed verification key.
//  In the latter case, we call the algorithm an "online verifier".
//  */

// /**
//  * A verifier algorithm for the R1CS (single-predicate) ppzkPCD that
//  * accepts a non-processed verification key.
//  */
// 
// bool r1cs_sp_ppzkpcd_verifier(vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
//                               primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
//                               proof:r1cs_sp_ppzkpcd_proof<PCD_ppT>);

// /**
//  * Convert a (non-processed) verification key into a processed verification key.
//  */
// 
// r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> r1cs_sp_ppzkpcd_process_vk(vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>);

// /**
//  * A verifier algorithm for the R1CS (single-predicate) ppzkPCD that
//  * accepts a processed verification key.
//  */
// 
// bool r1cs_sp_ppzkpcd_online_verifier(pvk:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>,
//                                      primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
//                                      proof:r1cs_sp_ppzkpcd_proof<PCD_ppT>);



// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd;

//#endif // R1CS_SP_PPZKPCD_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a *single-predicate* ppzkPCD for R1CS.

 See r1cs_sp_ppzkpcd.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_SP_PPZKPCD_TCC_
// #define R1CS_SP_PPZKPCD_TCC_

// use  <algorithm>
// use  <cassert>
// use  <iostream>

use ffec::common::profiling;
use ffec::common::utils;

use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::sp_pcd_circuits;



// 
// bool r1cs_sp_ppzkpcd_proving_key<PCD_ppT>::operator==(other:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>) const
// {
//     return (self.compliance_predicate == other.compliance_predicate &&
//             self.compliance_step_r1cs_pk == other.compliance_step_r1cs_pk &&
//             self.translation_step_r1cs_pk == other.translation_step_r1cs_pk &&
//             self.compliance_step_r1cs_vk == other.compliance_step_r1cs_vk &&
//             self.translation_step_r1cs_vk == other.translation_step_r1cs_vk);
// }

// 
// std::ostream& operator<<(std::ostream &out, pk:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>)
// {
//     out << pk.compliance_predicate;
//     out << pk.compliance_step_r1cs_pk;
//     out << pk.translation_step_r1cs_pk;
//     out << pk.compliance_step_r1cs_vk;
//     out << pk.translation_step_r1cs_vk;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_sp_ppzkpcd_proving_key<PCD_ppT> &pk)
// {
//     in >> pk.compliance_predicate;
//     in >> pk.compliance_step_r1cs_pk;
//     in >> pk.translation_step_r1cs_pk;
//     in >> pk.compliance_step_r1cs_vk;
//     in >> pk.translation_step_r1cs_vk;

//     return in;
// }

// 
// bool r1cs_sp_ppzkpcd_verification_key<PCD_ppT>::operator==(other:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>) const
// {
//     return (self.compliance_step_r1cs_vk == other.compliance_step_r1cs_vk &&
//             self.translation_step_r1cs_vk == other.translation_step_r1cs_vk);
// }

// 
// std::ostream& operator<<(std::ostream &out, vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>)
// {
//     out << vk.compliance_step_r1cs_vk;
//     out << vk.translation_step_r1cs_vk;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_sp_ppzkpcd_verification_key<PCD_ppT> &vk)
// {
//     in >> vk.compliance_step_r1cs_vk;
//     in >> vk.translation_step_r1cs_vk;

//     return in;
// }
impl r1cs_sp_ppzkpcd_verification_key<PCD_ppT>{

pub fn dummy_verification_key()->r1cs_sp_ppzkpcd_verification_key<PCD_ppT> 
{
    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    let mut result=r1cs_sp_ppzkpcd_verification_key::<PCD_ppT> ::new();
    result.compliance_step_r1cs_vk = r1cs_ppzksnark_verification_key::< PCD_ppT::curve_A_pp>::dummy_verification_key(sp_compliance_step_pcd_circuit_maker::<curve_A_pp>::input_size_in_elts());
    result.translation_step_r1cs_vk = r1cs_ppzksnark_verification_key::< PCD_ppT::curve_B_pp>::dummy_verification_key(sp_translation_step_pcd_circuit_maker::<curve_B_pp>::input_size_in_elts());

    return result;
}}

// 
// bool r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>::operator==(other:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>) const
// {
//     return (self.compliance_step_r1cs_pvk == other.compliance_step_r1cs_pvk &&
//             self.translation_step_r1cs_pvk == other.translation_step_r1cs_pvk &&
//             self.translation_step_r1cs_vk_bits == other.translation_step_r1cs_vk_bits);
// }

// 
// std::ostream& operator<<(std::ostream &out, pvk:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>)
// {
//     out << pvk.compliance_step_r1cs_pvk;
//     out << pvk.translation_step_r1cs_pvk;
//     ffec::serialize_bit_vector(out, pvk.translation_step_r1cs_vk_bits);

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk)
// {
//     in >> pvk.compliance_step_r1cs_pvk;
//     in >> pvk.translation_step_r1cs_pvk;
//     ffec::deserialize_bit_vector(in, pvk.translation_step_r1cs_vk_bits);

//     return in;
// }


 pub fn r1cs_sp_ppzkpcd_generator<PCD_ppT>(compliance_predicate:r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>)->r1cs_sp_ppzkpcd_keypair<PCD_ppT>
{
    assert!(ffec::Fr::< PCD_ppT::curve_A_pp>::modulo == ffec::Fq::< PCD_ppT::curve_B_pp>::modulo);
    assert!(ffec::Fq::< PCD_ppT::curve_A_pp>::modulo == ffec::Fr::< PCD_ppT::curve_B_pp>::modulo);

    type FieldT_A=ffec::Fr::< PCD_ppT::curve_A_pp>;
    type FieldT_B=ffec::Fr::< PCD_ppT::curve_B_pp>;

    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    ffec::enter_block("Call to r1cs_sp_ppzkpcd_generator");

    assert!(compliance_predicate.is_well_formed());

    ffec::enter_block("Construct compliance step PCD circuit");
    let mut  compliance_step_pcd_circuit=sp_compliance_step_pcd_circuit_maker::<curve_A_pp>::new(compliance_predicate);
    compliance_step_pcd_circuit.generate_r1cs_constraints();
    let   compliance_step_pcd_circuit_cs = compliance_step_pcd_circuit.get_circuit();
    compliance_step_pcd_circuit_cs.report_linear_constraint_statistics();
    ffec::leave_block("Construct compliance step PCD circuit");

    ffec::enter_block("Generate key pair for compliance step PCD circuit");
    let mut  compliance_step_keypair = r1cs_ppzksnark_generator::<curve_A_pp>(compliance_step_pcd_circuit_cs);
    ffec::leave_block("Generate key pair for compliance step PCD circuit");

    ffec::enter_block("Construct translation step PCD circuit");
     let mut translation_step_pcd_circuit=sp_translation_step_pcd_circuit_maker::<curve_B_pp>(compliance_step_keypair.vk);
    translation_step_pcd_circuit.generate_r1cs_constraints();
    let  translation_step_pcd_circuit_cs = translation_step_pcd_circuit.get_circuit();
    translation_step_pcd_circuit_cs.report_linear_constraint_statistics();
    ffec::leave_block("Construct translation step PCD circuit");

    ffec::enter_block("Generate key pair for translation step PCD circuit");
    let  translation_step_keypair = r1cs_ppzksnark_generator::<curve_B_pp>(translation_step_pcd_circuit_cs);
    ffec::leave_block("Generate key pair for translation step PCD circuit");

    ffec::print_indent(); ffec::print_mem("in generator");
    ffec::leave_block("Call to r1cs_sp_ppzkpcd_generator");

    return r1cs_sp_ppzkpcd_keypair::<PCD_ppT>(r1cs_sp_ppzkpcd_proving_key::<PCD_ppT>(compliance_predicate,
                                                                                 (compliance_step_keypair.pk),
                                                                                 (translation_step_keypair.pk),
                                                                                 compliance_step_keypair.vk,
                                                                                 translation_step_keypair.vk),
                                            r1cs_sp_ppzkpcd_verification_key::<PCD_ppT>(compliance_step_keypair.vk,
                                                                                      translation_step_keypair.vk));
}


 pub fn r1cs_sp_ppzkpcd_prover<PCD_ppT>(pk:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
                                                      primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
                                                      auxiliary_input:r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT>,
                                                      incoming_proofs:Vec<r1cs_sp_ppzkpcd_proof<PCD_ppT> >)->r1cs_sp_ppzkpcd_proof<PCD_ppT>
{
    type FieldT_A=ffec::Fr< PCD_ppT::curve_A_pp>;
    type FieldT_B=ffec::Fr< PCD_ppT::curve_B_pp>;

    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    ffec::enter_block("Call to r1cs_sp_ppzkpcd_prover");

    let  translation_step_r1cs_vk_bits = r1cs_ppzksnark_verification_key_variable::<curve_A_pp>::get_verification_key_bits(pk.translation_step_r1cs_vk);
// #ifdef DEBUG
    print!("Outgoing message:\n");
    primary_input.outgoing_message.print();
//#endif

    ffec::enter_block("Prove compliance step");
     let mut compliance_step_pcd_circuit=sp_compliance_step_pcd_circuit_maker::<curve_A_pp>::new(pk.compliance_predicate);
    compliance_step_pcd_circuit.generate_r1cs_witness(pk.translation_step_r1cs_vk,
                                                      primary_input,
                                                      auxiliary_input,
                                                      incoming_proofs);

    let compliance_step_primary_input = compliance_step_pcd_circuit.get_primary_input();
    let compliance_step_auxiliary_input = compliance_step_pcd_circuit.get_auxiliary_input();

    let compliance_step_proof = r1cs_ppzksnark_prover::<curve_A_pp>(pk.compliance_step_r1cs_pk, compliance_step_primary_input, compliance_step_auxiliary_input);
    ffec::leave_block("Prove compliance step");

// #ifdef DEBUG
    let compliance_step_input = get_sp_compliance_step_pcd_circuit_input::<curve_A_pp>(translation_step_r1cs_vk_bits, primary_input);
    let compliance_step_ok = r1cs_ppzksnark_verifier_strong_IC::<curve_A_pp>(pk.compliance_step_r1cs_vk, compliance_step_input, compliance_step_proof);
    assert!(compliance_step_ok);
//#endif

    ffec::enter_block("Prove translation step");
    let translation_step_pcd_circuit=sp_translation_step_pcd_circuit_maker::<curve_B_pp>(pk.compliance_step_r1cs_vk);

    let  translation_step_primary_input = get_sp_translation_step_pcd_circuit_input::<curve_B_pp>(translation_step_r1cs_vk_bits, primary_input);
    translation_step_pcd_circuit.generate_r1cs_witness(translation_step_primary_input, compliance_step_proof); // TODO: potential for better naming

    let translation_step_auxiliary_input = translation_step_pcd_circuit.get_auxiliary_input();
    let translation_step_proof = r1cs_ppzksnark_prover::<curve_B_pp>(pk.translation_step_r1cs_pk, translation_step_primary_input, translation_step_auxiliary_input);
    ffec::leave_block("Prove translation step");

// #ifdef DEBUG
    let  translation_step_ok = r1cs_ppzksnark_verifier_strong_IC::<curve_B_pp>(pk.translation_step_r1cs_vk, translation_step_primary_input, translation_step_proof);
    assert!(translation_step_ok);
//#endif

    ffec::print_indent(); ffec::print_mem("in prover");
    ffec::leave_block("Call to r1cs_sp_ppzkpcd_prover");

    return translation_step_proof;
}


 pub fn r1cs_sp_ppzkpcd_online_verifier<PCD_ppT>(pvk:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>,
                                     primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
                                     proof:r1cs_sp_ppzkpcd_proof<PCD_ppT>)->bool

{
    type curve_B_pp= PCD_ppT::curve_B_pp;

    ffec::enter_block("Call to r1cs_sp_ppzkpcd_online_verifier");
    let r1cs_input = get_sp_translation_step_pcd_circuit_input::<curve_B_pp>(pvk.translation_step_r1cs_vk_bits, primary_input);
    let result = r1cs_ppzksnark_online_verifier_strong_IC(pvk.translation_step_r1cs_pvk, r1cs_input, proof);
    ffec::print_indent(); ffec::print_mem("in online verifier");
    ffec::leave_block("Call to r1cs_sp_ppzkpcd_online_verifier");

    return result;
}


pub fn r1cs_sp_ppzkpcd_process_vk<PCD_ppT>(vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>)->r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> 
{
    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    ffec::enter_block("Call to r1cs_sp_ppzkpcd_processed_verification_key");
    let compliance_step_r1cs_pvk = r1cs_ppzksnark_verifier_process_vk::<curve_A_pp>(vk.compliance_step_r1cs_vk);
    let translation_step_r1cs_pvk = r1cs_ppzksnark_verifier_process_vk::<curve_B_pp>(vk.translation_step_r1cs_vk);
    let translation_step_r1cs_vk_bits = r1cs_ppzksnark_verification_key_variable::<curve_A_pp>::get_verification_key_bits(vk.translation_step_r1cs_vk);
    ffec::leave_block("Call to r1cs_sp_ppzkpcd_processed_verification_key");

    return r1cs_sp_ppzkpcd_processed_verification_key::<PCD_ppT>((compliance_step_r1cs_pvk),
                                                               (translation_step_r1cs_pvk),
                                                               translation_step_r1cs_vk_bits);
}



 pub fn r1cs_sp_ppzkpcd_verifier<PCD_ppT>(vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
                                     primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
                              proof:r1cs_sp_ppzkpcd_proof<PCD_ppT>)->bool
{
    ffec::enter_block("Call to r1cs_sp_ppzkpcd_verifier");
    let pvk = r1cs_sp_ppzkpcd_process_vk(vk);
    let result = r1cs_sp_ppzkpcd_online_verifier(pvk, primary_input, proof);
    ffec::print_indent(); ffec::print_mem("in verifier");
    ffec::leave_block("Call to r1cs_sp_ppzkpcd_verifier");

    return result;
}




//#endif // R1CS_SP_PPZKPCD_TCC_
