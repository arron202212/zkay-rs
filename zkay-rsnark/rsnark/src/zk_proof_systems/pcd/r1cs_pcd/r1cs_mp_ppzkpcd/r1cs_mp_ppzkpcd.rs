/** @file
 *****************************************************************************

 Declaration of interfaces for a *multi-predicate* ppzkPCD for R1CS.

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
 in \[CTV15]. Thus, PCD is constructed from two "matched" ppzkSNARKs for R1CS.

 Acronyms:

 "R1CS" = "Rank-1 Constraint Systems"
 "ppzkSNARK" = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
 "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

 References:

 \[CTV15]:
 "Cluster Computing in Zero Knowledge",
 Alessandro Chiesa, Eran Tromer, Madars Virza,

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_MP_PPZKPCD_HPP_
// #define R1CS_MP_PPZKPCD_HPP_

// 
// 

use crate::common::data_structures::set_commitment;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd_params;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;



/******************************** Proving key ********************************/

/**
 * A proving key for the R1CS (multi-predicate) ppzkPCD.
 */
// 
 type A_pp= PCD_ppT::curve_A_pp ;
    type B_pp= PCD_ppT::curve_B_pp ;
pub struct  r1cs_mp_ppzkpcd_proving_key<PCD_ppT> {

   

compliance_predicates:    Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> >,

compliance_step_r1cs_pks:    Vec<r1cs_ppzksnark_proving_key<A_pp> >,
translation_step_r1cs_pks:    Vec<r1cs_ppzksnark_proving_key<B_pp> >,

compliance_step_r1cs_vks:    Vec<r1cs_ppzksnark_verification_key<A_pp> >,
translation_step_r1cs_vks:    Vec<r1cs_ppzksnark_verification_key<B_pp> >,

commitment_to_translation_step_r1cs_vks:    set_commitment,
compliance_step_r1cs_vk_membership_proofs:    Vec<set_membership_proof>,

compliance_predicate_name_to_idx:    BTreeMap<usize, usize>,
}
impl r1cs_mp_ppzkpcd_proving_key<PCD_ppT> {

    pub fn new(compliance_predicates:Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> >,
                                compliance_step_r1cs_pk:Vec<r1cs_ppzksnark_proving_key<A_pp> >,
                                translation_step_r1cs_pk:Vec<r1cs_ppzksnark_proving_key<B_pp> >,
                                compliance_step_r1cs_vk:Vec<r1cs_ppzksnark_verification_key<A_pp> >,
                                translation_step_r1cs_vk:Vec<r1cs_ppzksnark_verification_key<B_pp> >,
                                commitment_to_translation_step_r1cs_vks:set_commitment,
                                compliance_step_r1cs_vk_membership_proofs:Vec<set_membership_proof>,
                                compliance_predicate_name_to_idx:BTreeMap<usize, usize>) ->Self
   
    {
Self{
 compliance_predicates,
        compliance_step_r1cs_pks,
        translation_step_r1cs_pks,
        compliance_step_r1cs_vks,
        translation_step_r1cs_vks,
        commitment_to_translation_step_r1cs_vks,
        compliance_step_r1cs_vk_membership_proofs,
        compliance_predicate_name_to_idx
}}

    
}


/******************************* Verification key ****************************/


/**
 * A verification key for the R1CS (multi-predicate) ppzkPCD.
 */
// 
//    type A_pp= PCD_ppT::curve_A_pp ;
//     type B_pp= PCD_ppT::curve_B_pp ;
pub struct  r1cs_mp_ppzkpcd_verification_key<PCD_ppT>  {

 

compliance_step_r1cs_vks:    Vec<r1cs_ppzksnark_verification_key<A_pp> >,
translation_step_r1cs_vks:    Vec<r1cs_ppzksnark_verification_key<B_pp> >,
commitment_to_translation_step_r1cs_vks:    set_commitment,
}
impl r1cs_mp_ppzkpcd_verification_key<PCD_ppT>  {
   
    pub fn new(compliance_step_r1cs_vks:Vec<r1cs_ppzksnark_verification_key<A_pp> >,
                                     translation_step_r1cs_vks:Vec<r1cs_ppzksnark_verification_key<B_pp> >,
                                     commitment_to_translation_step_r1cs_vks:set_commitment) ->Self
        
    {
Self{compliance_step_r1cs_vks,
        translation_step_r1cs_vks,
        commitment_to_translation_step_r1cs_vks}}

}


/************************* Processed verification key **************************/

/**
 * A processed verification key for the R1CS (multi-predicate) ppzkPCD.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
// 
// type A_pp= PCD_ppT::curve_A_pp ;
//     type B_pp= PCD_ppT::curve_B_pp ;
pub struct  r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> {

    

compliance_step_r1cs_pvks:    Vec<r1cs_ppzksnark_processed_verification_key<A_pp> >,
translation_step_r1cs_pvks:    Vec<r1cs_ppzksnark_processed_verification_key<B_pp> >,
commitment_to_translation_step_r1cs_vks:    set_commitment,
}
impl r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> {
    pub fn new(
compliance_step_r1cs_pvks:Vec<r1cs_ppzksnark_processed_verification_key<A_pp> >,
translation_step_r1cs_pvks:                                               Vec<r1cs_ppzksnark_processed_verification_key<B_pp> >,
                                               commitment_to_translation_step_r1cs_vks:set_commitment) ->Self
        
    {
Self{compliance_step_r1cs_pvks,
        translation_step_r1cs_pvks,
        commitment_to_translation_step_r1cs_vks}
}

   
}


/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS (multi-predicate) ppzkPC, which consists of a proving key and a verification key.
 */

pub struct  r1cs_mp_ppzkpcd_keypair<PCD_ppT>  {

pk:    r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
vk:    r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
}
impl r1cs_mp_ppzkpcd_keypair<PCD_ppT>  {

    pub fn new(
pk:    r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
vk:                            r1cs_mp_ppzkpcd_verification_key<PCD_ppT>) ->Self
        
    {
    Self{pk,
        vk}}
}


/*********************************** Proof ***********************************/



/**
 * A proof for the R1CS (multi-predicate) ppzkPCD.
 */
// 
pub struct  r1cs_mp_ppzkpcd_proof<PCD_ppT> {

compliance_predicate_idx:    usize,
r1cs_proof:    r1cs_ppzksnark_proof< PCD_ppT::curve_B_pp>,
}
impl r1cs_mp_ppzkpcd_proof<PCD_ppT> {
 
    pub fn new(compliance_predicate_idx:usize ,
                          r1cs_proof:r1cs_ppzksnark_proof< PCD_ppT::curve_B_pp>) ->Self
        
    {
    Self{compliance_predicate_idx,
        r1cs_proof
    }}


}


/***************************** Main algorithms *******************************/

// /**
//  * A generator algorithm for the R1CS (multi-predicate) ppzkPCD.
//  *
//  * Given a vector of compliance predicates, this algorithm produces proving and verification keys for the vector.
//  */
// 
// r1cs_mp_ppzkpcd_keypair<PCD_ppT> r1cs_mp_ppzkpcd_generator(compliance_predicates:Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> >);

// /**
//  * A prover algorithm for the R1CS (multi-predicate) ppzkPCD.
//  *
//  * Given a proving key, name of chosen compliance predicate, inputs for the
//  * compliance predicate, and proofs for the predicate's input messages, this
//  * algorithm produces a proof (of knowledge) that attests to the compliance of
//  * the output message.
//  */
// 
// r1cs_mp_ppzkpcd_proof<PCD_ppT> r1cs_mp_ppzkpcd_prover(pk:r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
//                                                       compliance_predicate_name:usize ,
//                                                       primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
//                                                       auxiliary_input:r1cs_mp_ppzkpcd_auxiliary_input<PCD_ppT>,
//                                                       incoming_proofs:Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >);

// /*
//   Below are two variants of verifier algorithm for the R1CS (multi-predicate) ppzkPCD.

//   These are the two cases that arise from whether the verifier accepts a
//   (non-processed) verification key or, instead, a processed verification key.
//   In the latter case, we call the algorithm an "online verifier".
// */

// /**
//  * A verifier algorithm for the R1CS (multi-predicate) ppzkPCD that
//  * accepts a non-processed verification key.
//  */
// 
// bool r1cs_mp_ppzkpcd_verifier(vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
//                               primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
//                               proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>);

// /**
//  * Convert a (non-processed) verification key into a processed verification key.
//  */
// 
// r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> r1cs_mp_ppzkpcd_process_vk(vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>);

// /**
//  * A verifier algorithm for the R1CS (multi-predicate) ppzkPCD that
//  * accepts a processed verification key.
//  */
// 
// bool r1cs_mp_ppzkpcd_online_verifier(pvk:r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>,
//                                      primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
//                                      proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>);



// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd;

//#endif // R1CS_MP_PPZKPCD_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a *multi-predicate* ppzkPCD for R1CS.

 See r1cs_mp_ppzkpcd.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_MP_PPZKPCD_TCC_
// #define R1CS_MP_PPZKPCD_TCC_

// use  <algorithm>
// use  <cassert>
// use  <iostream>

use ffec::common::profiling;
use ffec::common::utils;

use crate::common::libsnark_serialization;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::mp_pcd_circuits;


impl r1cs_mp_ppzkpcd_proving_key<PCD_ppT>{

pub fn size_in_bits()->usize
{
    let num_predicates = compliance_predicates.len();

    let mut result = 0;
    for i in 0..num_predicates
    {
        result += (compliance_predicates[i].size_in_bits() +
                   compliance_step_r1cs_pks[i].size_in_bits() +
                   translation_step_r1cs_pks[i].size_in_bits() +
                   compliance_step_r1cs_vks[i].size_in_bits() +
                   translation_step_r1cs_vks[i].size_in_bits() +
                   compliance_step_r1cs_vk_membership_proofs[i].size_in_bits());
    }
    result += commitment_to_translation_step_r1cs_vks.len();

    return result;
}


pub fn is_well_formed()->bool
{
    let num_predicates = compliance_predicates.len();

    let mut  result=false;
    result = result && (compliance_step_r1cs_pks.len() == num_predicates);
    result = result && (translation_step_r1cs_pks.len() == num_predicates);
    result = result && (compliance_step_r1cs_vks.len() == num_predicates);
    result = result && (translation_step_r1cs_vks.len() == num_predicates);
    result = result && (compliance_step_r1cs_vk_membership_proofs.len() == num_predicates);

    return result;
}
}
// 
// bool r1cs_mp_ppzkpcd_proving_key<PCD_ppT>::operator==(other:r1cs_mp_ppzkpcd_proving_key<PCD_ppT>) const
// {
//     return (self.compliance_predicates == other.compliance_predicates &&
//             self.compliance_step_r1cs_pks == other.compliance_step_r1cs_pks &&
//             self.translation_step_r1cs_pks == other.translation_step_r1cs_pks &&
//             self.compliance_step_r1cs_vks == other.compliance_step_r1cs_vks &&
//             self.translation_step_r1cs_vks == other.translation_step_r1cs_vks &&
//             self.commitment_to_translation_step_r1cs_vks == other.commitment_to_translation_step_r1cs_vks &&
//             self.compliance_step_r1cs_vk_membership_proofs == other.compliance_step_r1cs_vk_membership_proofs &&
//             self.compliance_predicate_name_to_idx == other.compliance_predicate_name_to_idx);
// }


// 
// std::ostream& operator<<(std::ostream &out, pk:r1cs_mp_ppzkpcd_proving_key<PCD_ppT>)
// {
//     out << pk.compliance_predicates;
//     out << pk.compliance_step_r1cs_pks;
//     out << pk.translation_step_r1cs_pks;
//     out << pk.compliance_step_r1cs_vks;
//     out << pk.translation_step_r1cs_vks;
//     output_bool_vector(out, pk.commitment_to_translation_step_r1cs_vks);
//     out << pk.compliance_step_r1cs_vk_membership_proofs;
//     out << pk.compliance_predicate_name_to_idx;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk)
// {
//     in >> pk.compliance_predicates;
//     in >> pk.compliance_step_r1cs_pks;
//     in >> pk.translation_step_r1cs_pks;
//     in >> pk.compliance_step_r1cs_vks;
//     in >> pk.translation_step_r1cs_vks;
//     input_bool_vector(in, pk.commitment_to_translation_step_r1cs_vks);
//     in >> pk.compliance_step_r1cs_vk_membership_proofs;
//     in >> pk.compliance_predicate_name_to_idx;

//     return in;
// }
impl r1cs_mp_ppzkpcd_verification_key<PCD_ppT>{

pub fn size_in_bits()->usize
{
    let  num_predicates = compliance_step_r1cs_vks.len();

    let mut  result = 0;
    for i in 0..num_predicates
    {
        result += (compliance_step_r1cs_vks[i].size_in_bits() +
                   translation_step_r1cs_vks[i].size_in_bits());
    }

    result += commitment_to_translation_step_r1cs_vks.len();

    return result;
}
}

// 
// bool r1cs_mp_ppzkpcd_verification_key<PCD_ppT>::operator==(other:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>) const
// {
//     return (self.compliance_step_r1cs_vks == other.compliance_step_r1cs_vks &&
//             self.translation_step_r1cs_vks == other.translation_step_r1cs_vks &&
//             self.commitment_to_translation_step_r1cs_vks == other.commitment_to_translation_step_r1cs_vks);
// }

// 
// std::ostream& operator<<(std::ostream &out, vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>)
// {
//     out << vk.compliance_step_r1cs_vks;
//     out << vk.translation_step_r1cs_vks;
//     ffec::output_bool_vector(out, vk.commitment_to_translation_step_r1cs_vks);

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk)
// {
//     in >> vk.compliance_step_r1cs_vks;
//     in >> vk.translation_step_r1cs_vks;
//     ffec::input_bool_vector(in, vk.commitment_to_translation_step_r1cs_vks);

//     return in;
// }

impl r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>{

pub fn size_in_bits()->usize
{
    let  num_predicates = compliance_step_r1cs_pvks.len();

    let mut  result = 0;
    for i in 0..num_predicates
    {
        result += (compliance_step_r1cs_pvks[i].size_in_bits() +
                   translation_step_r1cs_pvks[i].size_in_bits());
    }

    result += commitment_to_translation_step_r1cs_vks.len();

    return result;
}
}

// 
// bool r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>::operator==(other:r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>) const
// {
//     return (self.compliance_step_r1cs_pvks == other.compliance_step_r1cs_pvks &&
//             self.translation_step_r1cs_pvks == other.translation_step_r1cs_pvks &&
//             self.commitment_to_translation_step_r1cs_vks == other.commitment_to_translation_step_r1cs_vks);
// }

// 
// std::ostream& operator<<(std::ostream &out, pvk:r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>)
// {
//     out << pvk.compliance_step_r1cs_pvks;
//     out << pvk.translation_step_r1cs_pvks;
//     ffec::output_bool_vector(out, pvk.commitment_to_translation_step_r1cs_vks);

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk)
// {
//     in >> pvk.compliance_step_r1cs_pvks;
//     in >> pvk.translation_step_r1cs_pvks;
//     ffec::input_bool_vector(in, pvk.commitment_to_translation_step_r1cs_vks);

//     return in;
// }

// 
// bool r1cs_mp_ppzkpcd_proof<PCD_ppT>::operator==(other:r1cs_mp_ppzkpcd_proof<PCD_ppT>) const
// {
//     return (self.compliance_predicate_idx == other.compliance_predicate_idx &&
//             self.r1cs_proof == other.r1cs_proof);
// }

// 
// std::ostream& operator<<(std::ostream &out, proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>)
// {
//     out << proof.compliance_predicate_idx << "\n";
//     out << proof.r1cs_proof;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof)
// {
//     in >> proof.compliance_predicate_idx;
//     ffec::consume_newline(in);
//     in >> proof.r1cs_proof;

//     return in;
// }



pub fn r1cs_mp_ppzkpcd_generator<PCD_ppT>(compliance_predicates:Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> >)->r1cs_mp_ppzkpcd_keypair<PCD_ppT> 
{
    assert!(ffec::Fr::< PCD_ppT::curve_A_pp>::modulo == ffec::Fq::< PCD_ppT::curve_B_pp>::modulo);
    assert!(ffec::Fq::< PCD_ppT::curve_A_pp>::modulo == ffec::Fr::< PCD_ppT::curve_B_pp>::modulo);

    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    type FieldT_A=ffec::Fr<curve_A_pp>;
    type FieldT_B=ffec::Fr<curve_B_pp>;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_generator");

    let mut  keypair=r1cs_mp_ppzkpcd_keypair::<PCD_ppT>::new();
    let  translation_input_size = mp_translation_step_pcd_circuit_maker::<curve_B_pp>::input_size_in_elts();
    let   vk_size_in_bits = r1cs_ppzksnark_verification_key_variable::<curve_A_pp>::size_in_bits(translation_input_size);
    print!("{} {}\n", translation_input_size, vk_size_in_bits);

    let mut  all_translation_vks=set_commitment_accumulator::<CRH_with_bit_out_gadget::<FieldT_A> >::new(compliance_predicates.len(), vk_size_in_bits);

    ffec::enter_block("Perform type checks");
    let mut  type_counts=HashMap::new();

    for cp in &compliance_predicates
    {
        type_counts[cp.types] += 1;
    }

    for cp in &compliance_predicates
    {
        if cp.relies_on_same_type_inputs
        {
            for types in &cp.accepted_input_types
            {
                assert!(type_counts[types] == 1); /* each of accepted_input_types must be unique */
            }
        }
        else
        {
            assert!(cp.accepted_input_types.empty());
        }
    }
    ffec::leave_block("Perform type checks");

    for i in 0..compliance_predicates.len()
    {
        ffec::enter_block(FMT("", "Process predicate {} (with name {} and type {})", i, compliance_predicates[i].name, compliance_predicates[i].types));
        assert!(compliance_predicates[i].is_well_formed());

        ffec::enter_block("Construct compliance step PCD circuit");
        let mut  mp_compliance_step_pcd_circuit=mp_compliance_step_pcd_circuit_maker::<curve_A_pp>::new(compliance_predicates[i], compliance_predicates.len());
        mp_compliance_step_pcd_circuit.generate_r1cs_constraints();
        let mut  mp_compliance_step_pcd_circuit_cs = mp_compliance_step_pcd_circuit.get_circuit();
        ffec::leave_block("Construct compliance step PCD circuit");

        ffec::enter_block("Generate key pair for compliance step PCD circuit");
        let mut  mp_compliance_step_keypair = r1cs_ppzksnark_generator::<curve_A_pp>(mp_compliance_step_pcd_circuit_cs);
        ffec::leave_block("Generate key pair for compliance step PCD circuit");

        ffec::enter_block("Construct translation step PCD circuit");
        let mut  mp_translation_step_pcd_circuit=mp_translation_step_pcd_circuit_maker::<curve_B_pp>::new(mp_compliance_step_keypair.vk);
        mp_translation_step_pcd_circuit.generate_r1cs_constraints();
        let  mp_translation_step_pcd_circuit_cs = mp_translation_step_pcd_circuit.get_circuit();
        ffec::leave_block("Construct translation step PCD circuit");

        ffec::enter_block("Generate key pair for translation step PCD circuit");
        let mut  mp_translation_step_keypair = r1cs_ppzksnark_generator::<curve_B_pp>::new(mp_translation_step_pcd_circuit_cs);
        ffec::leave_block("Generate key pair for translation step PCD circuit");

        ffec::enter_block("Augment set of translation step verification keys");
        let vk_bits = r1cs_ppzksnark_verification_key_variable::<curve_A_pp>::get_verification_key_bits(mp_translation_step_keypair.vk);
        all_translation_vks.add(vk_bits);
        ffec::leave_block("Augment set of translation step verification keys");

        ffec::enter_block("Update r1cs_mp_ppzkpcd keypair");
        keypair.pk.compliance_predicates.push(compliance_predicates[i]);
        keypair.pk.compliance_step_r1cs_pks.push(mp_compliance_step_keypair.pk);
        keypair.pk.translation_step_r1cs_pks.push(mp_translation_step_keypair.pk);
        keypair.pk.compliance_step_r1cs_vks.push(mp_compliance_step_keypair.vk);
        keypair.pk.translation_step_r1cs_vks.push(mp_translation_step_keypair.vk);
        let cp_name = compliance_predicates[i].name;
        assert!(keypair.pk.compliance_predicate_name_to_idx.find(cp_name) == keypair.pk.compliance_predicate_name_to_idx.end()); // all names must be distinct
        keypair.pk.compliance_predicate_name_to_idx[cp_name] = i;

        keypair.vk.compliance_step_r1cs_vks.push(mp_compliance_step_keypair.vk);
        keypair.vk.translation_step_r1cs_vks.push(mp_translation_step_keypair.vk);
        ffec::leave_block("Update r1cs_mp_ppzkpcd keypair");

        ffec::leave_block(FMT("", "Process predicate {} (with name {} and type {})", i, compliance_predicates[i].name, compliance_predicates[i].types));
    }

    ffec::enter_block("Compute set commitment and corresponding membership proofs");
    let cm = all_translation_vks.get_commitment();
    keypair.pk.commitment_to_translation_step_r1cs_vks = cm;
    keypair.vk.commitment_to_translation_step_r1cs_vks = cm;
    for i in 0..compliance_predicates.len()
    {
        let vk_bits = r1cs_ppzksnark_verification_key_variable::<curve_A_pp>::get_verification_key_bits(keypair.vk.translation_step_r1cs_vks[i]);
        let proof = all_translation_vks.get_membership_proof(vk_bits);

        keypair.pk.compliance_step_r1cs_vk_membership_proofs.push(proof);
    }
    ffec::leave_block("Compute set commitment and corresponding membership proofs");

    ffec::print_indent(); ffec::print_mem("in generator");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_generator");

    return keypair;
}


 pub fn r1cs_mp_ppzkpcd_prover<PCD_ppT>(pk:r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
                                                      compliance_predicate_name:usize,
                                                      primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
                                                      auxiliary_input:r1cs_mp_ppzkpcd_auxiliary_input<PCD_ppT>,
                                                      prev_proofs:Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >)->r1cs_mp_ppzkpcd_proof<PCD_ppT>
{
    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    type FieldT_A=ffec::Fr<curve_A_pp>;
    type FieldT_B=ffec::Fr<curve_B_pp>;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_prover");

// #ifdef DEBUG
    print!("Compliance predicate name: {}\n", compliance_predicate_name);
//#endif
    let mut it = pk.compliance_predicate_name_to_idx.find(compliance_predicate_name);
    assert!(it != pk.compliance_predicate_name_to_idx.end());
    let compliance_predicate_idx = it.1;

// #ifdef DEBUG
    print!("Outgoing message:\n");
    primary_input.outgoing_message.print();
//#endif

    ffec::enter_block("Prove compliance step");
    assert!(compliance_predicate_idx < pk.compliance_predicates.len());
    assert!(prev_proofs.len() <= pk.compliance_predicates[compliance_predicate_idx].max_arity);

    let arity = prev_proofs.len();
    let max_arity = pk.compliance_predicates[compliance_predicate_idx].max_arity;

    if pk.compliance_predicates[compliance_predicate_idx].relies_on_same_type_inputs
    {
        let input_predicate_idx = prev_proofs[0].compliance_predicate_idx;
        for i in 1..arity
        {
            assert!(prev_proofs[i].compliance_predicate_idx == input_predicate_idx);
        }
    }

    let mut  padded_proofs=vec![r1cs_ppzksnark_proof::<curve_B_pp> ::new();max_arity];
    for i in 0..arity
    {
        padded_proofs[i] = prev_proofs[i].r1cs_proof;
    }

    let mut translation_step_vks=vec![];
    let mut membership_proofs=vec![];

    for i in 0..arity
    {
        let input_predicate_idx = prev_proofs[i].compliance_predicate_idx;
        translation_step_vks.push(pk.translation_step_r1cs_vks[input_predicate_idx]);
        membership_proofs.push(pk.compliance_step_r1cs_vk_membership_proofs[input_predicate_idx]);

// #ifdef DEBUG
        if auxiliary_input.incoming_messages[i].types != 0
        {
            print!("check proof for message {}\n", i);
            let translated_msg = get_mp_translation_step_pcd_circuit_input::<curve_B_pp>(pk.commitment_to_translation_step_r1cs_vks,
                                                                                                                      auxiliary_input.incoming_messages[i]);
            let bit = r1cs_ppzksnark_verifier_strong_IC::<curve_B_pp>(translation_step_vks[i], translated_msg, padded_proofs[i]);
            assert!(bit);
        }
        else
        {
            print!("message {} is base case\n", i);
        }
//#endif
    }

    /* pad with dummy vks/membership proofs */
    for i in arity..max_arity
    {
        print!("proof {} will be a dummy\n", arity);
        translation_step_vks.push(pk.translation_step_r1cs_vks[0]);
        membership_proofs.push(pk.compliance_step_r1cs_vk_membership_proofs[0]);
    }

    let mut mp_compliance_step_pcd_circuit= mp_compliance_step_pcd_circuit_maker::<curve_A_pp>::new(pk.compliance_predicates[compliance_predicate_idx], pk.compliance_predicates.len());

    mp_compliance_step_pcd_circuit.generate_r1cs_witness(pk.commitment_to_translation_step_r1cs_vks,
                                                         translation_step_vks,
                                                         membership_proofs,
                                                         primary_input,
                                                         auxiliary_input,
                                                         padded_proofs);

    let compliance_step_primary_input = mp_compliance_step_pcd_circuit.get_primary_input();
    let compliance_step_auxiliary_input = mp_compliance_step_pcd_circuit.get_auxiliary_input();
    let compliance_step_proof = r1cs_ppzksnark_prover::<curve_A_pp>(pk.compliance_step_r1cs_pks[compliance_predicate_idx], compliance_step_primary_input, compliance_step_auxiliary_input);
    ffec::leave_block("Prove compliance step");

// #ifdef DEBUG
    let compliance_step_input = get_mp_compliance_step_pcd_circuit_input::<curve_A_pp>(pk.commitment_to_translation_step_r1cs_vks, primary_input.outgoing_message);
    let compliance_step_ok = r1cs_ppzksnark_verifier_strong_IC::<curve_A_pp>(pk.compliance_step_r1cs_vks[compliance_predicate_idx], compliance_step_input, compliance_step_proof);
    assert!(compliance_step_ok);
//#endif

    ffec::enter_block("Prove translation step");
    let mut  mp_translation_step_pcd_circuit=mp_translation_step_pcd_circuit_maker::<curve_B_pp>::new(pk.compliance_step_r1cs_vks[compliance_predicate_idx]);

    let translation_step_primary_input = get_mp_translation_step_pcd_circuit_input::<curve_B_pp>(pk.commitment_to_translation_step_r1cs_vks, primary_input);
    mp_translation_step_pcd_circuit.generate_r1cs_witness(translation_step_primary_input, compliance_step_proof);
    let translation_step_auxiliary_input = mp_translation_step_pcd_circuit.get_auxiliary_input();

    let translation_step_proof = r1cs_ppzksnark_prover::<curve_B_pp>(pk.translation_step_r1cs_pks[compliance_predicate_idx], translation_step_primary_input, translation_step_auxiliary_input);

    ffec::leave_block("Prove translation step");

// #ifdef DEBUG
    let translation_step_ok = r1cs_ppzksnark_verifier_strong_IC::<curve_B_pp>(pk.translation_step_r1cs_vks[compliance_predicate_idx], translation_step_primary_input, translation_step_proof);
    assert!(translation_step_ok);
//#endif

    ffec::print_indent(); ffec::print_mem("in prover");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_prover");

    let mut  result=r1cs_mp_ppzkpcd_proof::<PCD_ppT>::new();
    result.compliance_predicate_idx = compliance_predicate_idx;
    result.r1cs_proof = translation_step_proof;
    return result;
}


pub fn  r1cs_mp_ppzkpcd_online_verifier<PCD_ppT>(pvk:r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>,
                                     primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
                                     proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>)->bool
{
    type curve_B_pp= PCD_ppT::curve_B_pp;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_online_verifier");
    let r1cs_input = get_mp_translation_step_pcd_circuit_input::<curve_B_pp>(pvk.commitment_to_translation_step_r1cs_vks, primary_input);
    let result = r1cs_ppzksnark_online_verifier_strong_IC(pvk.translation_step_r1cs_pvks[proof.compliance_predicate_idx], r1cs_input, proof.r1cs_proof);

    ffec::print_indent(); ffec::print_mem("in online verifier");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_online_verifier");
    return result;
}


 pub fn r1cs_mp_ppzkpcd_process_vk<PCD_ppT>(vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>)->r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>
{
    type curve_A_pp= PCD_ppT::curve_A_pp;
    type curve_B_pp= PCD_ppT::curve_B_pp;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_processed_verification_key");

    let mut result= r1cs_mp_ppzkpcd_processed_verification_key::<PCD_ppT>::new();
    result.commitment_to_translation_step_r1cs_vks = vk.commitment_to_translation_step_r1cs_vks;

    for i in 0..vk.compliance_step_r1cs_vks.len()
    {
        let compliance_step_r1cs_pvk = r1cs_ppzksnark_verifier_process_vk::<curve_A_pp>(vk.compliance_step_r1cs_vks[i]);
        let translation_step_r1cs_pvk = r1cs_ppzksnark_verifier_process_vk::<curve_B_pp>(vk.translation_step_r1cs_vks[i]);

        result.compliance_step_r1cs_pvks.push(compliance_step_r1cs_pvk);
        result.translation_step_r1cs_pvks.push(translation_step_r1cs_pvk);
    }
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_processed_verification_key");

    return result;
}



pub fn  r1cs_mp_ppzkpcd_verifier<PCD_ppT>(vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
                              primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
                              proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>)->bool
{
    ffec::enter_block("Call to r1cs_mp_ppzkpcd_verifier");
    let  pvk = r1cs_mp_ppzkpcd_process_vk(vk);
    let result = r1cs_mp_ppzkpcd_online_verifier(pvk, primary_input, proof);

    ffec::print_indent(); ffec::print_mem("in verifier");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_verifier");
    return result;
}




//#endif // R1CS_MP_PPZKPCD_TCC_
