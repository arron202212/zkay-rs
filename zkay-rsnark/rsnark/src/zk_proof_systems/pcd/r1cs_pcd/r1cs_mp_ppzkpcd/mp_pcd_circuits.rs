/** @file
 *****************************************************************************

 Declaration of functionality for creating and using the two PCD circuits in
 a multi-predicate PCD construction.

 The implementation follows, extends, and optimizes the approach described
 in \[CTV15]. At high level, there is a "compliance step" circuit and a
 "translation step" circuit, for each compliance predicate. For more details,
 see \[CTV15].


 References:

 \[CTV15]:
 "Cluster Computing in Zero Knowledge",
 Alessandro Chiesa, Eran Tromer, Madars Virza

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MP_PCD_CIRCUITS_HPP_
// #define MP_PCD_CIRCUITS_HPP_

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::gadget_from_r1cs;
use crate::gadgetlib1::gadgets::hashes::crh_gadget;
use crate::gadgetlib1::gadgets::set_commitment::set_commitment_gadget;
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;



/**************************** Compliance step ********************************/

/**
 * A compliance-step PCD circuit.
 *
 * The circuit is an R1CS that checks compliance (for the given compliance predicate)
 * and validity of previous proofs.
 */
pub trait FieldTConfig{
type FieldT;
}
impl<ppT>  FieldTConfig for mp_compliance_step_pcd_circuit_maker<ppT> 
{
type FieldT=ffec::Fr<ppT> ;
}
pub struct  mp_compliance_step_pcd_circuit_maker<ppT> {

    

compliance_predicate:    r1cs_pcd_compliance_predicate<FieldT>,

pb:    protoboard<FieldT>,

zero:    pb_variable<FieldT>,

block_for_outgoing_message:    RcCell<block_variable<FieldT> >,
hash_outgoing_message:    RcCell<CRH_with_field_out_gadget<FieldT> >,

block_for_incoming_messages:    Vec<block_variable<FieldT> >,
commitment_and_incoming_message_digests:    Vec<pb_variable_array<FieldT> >,
unpack_commitment_and_incoming_message_digests:    Vec<multipacking_gadget<FieldT> >,
commitment_and_incoming_messages_digest_bits:    Vec<pb_variable_array<FieldT> >,
hash_incoming_messages:    Vec<CRH_with_field_out_gadget<FieldT> >,

translation_step_vks:    Vec<r1cs_ppzksnark_verification_key_variable<ppT> >,
translation_step_vks_bits:    Vec<pb_variable_array<FieldT> >,

outgoing_message_type:    pb_variable<FieldT>,
outgoing_message_payload:    pb_variable_array<FieldT>,
outgoing_message_vars:    pb_variable_array<FieldT>,

arity:    pb_variable<FieldT>,
incoming_message_types:    Vec<pb_variable<FieldT> >,
incoming_message_payloads:    Vec<pb_variable_array<FieldT> >,
incoming_message_vars:    Vec<pb_variable_array<FieldT> >,

local_data:    pb_variable_array<FieldT>,
cp_witness:    pb_variable_array<FieldT>,
compliance_predicate_as_gadget:    RcCell<gadget_from_r1cs<FieldT> >,

outgoing_message_bits:    pb_variable_array<FieldT>,
unpack_outgoing_message:    RcCell<multipacking_gadget<FieldT> >,

incoming_messages_bits:    Vec<pb_variable_array<FieldT> >,
unpack_incoming_messages:    Vec<multipacking_gadget<FieldT> >,

mp_compliance_step_pcd_circuit_input:    pb_variable_array<FieldT>,
padded_translation_step_vk_and_outgoing_message_digest:    pb_variable_array<FieldT>,
padded_commitment_and_incoming_messages_digest:    Vec<pb_variable_array<FieldT> >,

commitment:    RcCell<set_commitment_variable<FieldT, CRH_with_bit_out_gadget<FieldT> > >,
membership_proofs:    Vec<set_membership_proof_variable<FieldT, CRH_with_bit_out_gadget<FieldT> > >,
membership_checkers:    Vec<set_commitment_gadget<FieldT, CRH_with_bit_out_gadget<FieldT> > >,
membership_check_results:    pb_variable_array<FieldT>,
common_type:    pb_variable<FieldT>,
common_type_check_aux:    pb_variable_array<FieldT>,

verifier_input:    Vec<pb_variable_array<FieldT> >,
proof:    Vec<r1cs_ppzksnark_proof_variable<ppT> >,
verification_results:    pb_variable_array<FieldT>,
verifier:    Vec<r1cs_ppzksnark_verifier_gadget<ppT> >,


}

/*************************** Translation step ********************************/

/**
 * A translation-step PCD circuit.
 *
 * The circuit is an R1CS that checks validity of previous proofs.
 */

impl<ppT>  FieldTConfig for mp_translation_step_pcd_circuit_maker<ppT> 
{
type FieldT=ffec::Fr<ppT> ;
}
pub struct  mp_translation_step_pcd_circuit_maker<ppT> {

    

pb:    protoboard<FieldT>,

mp_translation_step_pcd_circuit_input:    pb_variable_array<FieldT>,
unpacked_mp_translation_step_pcd_circuit_input:    pb_variable_array<FieldT>,
verifier_input:    pb_variable_array<FieldT>,
unpack_mp_translation_step_pcd_circuit_input:    RcCell<multipacking_gadget<FieldT> >,

hardcoded_compliance_step_vk:    RcCell<r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> >,
proof:    RcCell<r1cs_ppzksnark_proof_variable<ppT> >,
online_verifier:    RcCell<r1cs_ppzksnark_online_verifier_gadget<ppT> >,

 
}

// /****************************** Input maps ***********************************/

// /**
//  * Obtain the primary input for a compliance-step PCD circuit.
//  */
// 
// r1cs_primary_input<ffec::Fr<ppT> > get_mp_compliance_step_pcd_circuit_input(commitment_to_translation_step_r1cs_vks:set_commitment,
//                                                                       primary_input:r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<ppT> >);

// /**
//  * Obtain the primary input for a translation-step PCD circuit.
//  */
// 
// r1cs_primary_input<ffec::Fr<ppT> > get_mp_translation_step_pcd_circuit_input(commitment_to_translation_step_r1cs_vks:set_commitment,
//                                                                        primary_input:r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<other_curve<ppT> > >);



// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::mp_pcd_circuits;

//#endif // MP_PCD_CIRCUITS_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality for creating and using the two PCD circuits in
 a multi-predicate PCD construction.

 See mp_pcd_circuits.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MP_PCD_CIRCUITS_TCC_
// #define MP_PCD_CIRCUITS_TCC_

// use  <algorithm>

use ffec::common::utils;

use crate::gadgetlib1::constraint_profiling;


impl mp_compliance_step_pcd_circuit_maker<ppT>{

pub fn new(compliance_predicate:r1cs_pcd_compliance_predicate<FieldT>,
                                                                                max_number_of_predicates:usize) ->Self
    
{
    /* calculate some useful sizes */
    let  digest_size = CRH_with_field_out_gadget::<FieldT>::get_digest_len();
    let outgoing_msg_size_in_bits = field_logsize() * (1 + compliance_predicate.outgoing_message_payload_length);
    assert!(compliance_predicate.has_equal_input_lengths());
    let translation_step_vk_size_in_bits = r1cs_ppzksnark_verification_key_variable::<ppT>::size_in_bits(mp_translation_step_pcd_circuit_maker::<other_curve::<ppT> >::input_size_in_elts());
    let padded_verifier_input_size = mp_translation_step_pcd_circuit_maker::<other_curve::<ppT> >::input_capacity_in_bits();
    let commitment_size = set_commitment_gadget::<FieldT, CRH_with_bit_out_gadget::<FieldT> >::root_size_in_bits();

    let output_block_size = commitment_size + outgoing_msg_size_in_bits;
    let max_incoming_payload_length = *std::max_element(compliance_predicate.incoming_message_payload_lengths.begin(), compliance_predicate.incoming_message_payload_lengths.end());
    let max_input_block_size = commitment_size + field_logsize() * (1 + max_incoming_payload_length);

    CRH_with_bit_out_gadge::<FieldT>::sample_randomness(std::max(output_block_size, max_input_block_size));

    /* allocate input of the compliance MP_PCD circuit */
    mp_compliance_step_pcd_circuit_input.allocate(pb, input_size_in_elts(), "mp_compliance_step_pcd_circuit_input");

    /* allocate inputs to the compliance predicate */
    outgoing_message_type.allocate(pb, "outgoing_message_type");
    outgoing_message_payload.allocate(pb, compliance_predicate.outgoing_message_payload_length, "outgoing_message_payload");

    outgoing_message_vars.insert(outgoing_message_vars.end(), outgoing_message_type);
    outgoing_message_vars.insert(outgoing_message_vars.end(), outgoing_message_payload.begin(), outgoing_message_payload.end());

    arity.allocate(pb, "arity");

    incoming_message_types.resize(compliance_predicate.max_arity);
    incoming_message_payloads.resize(compliance_predicate.max_arity);
    incoming_message_vars.resize(compliance_predicate.max_arity);
    for i in 0..compliance_predicate.max_arity
    {
        incoming_message_types[i].allocate(pb, FMT("", "incoming_message_type_{}", i));
        incoming_message_payloads[i].allocate(pb, compliance_predicate.incoming_message_payload_lengths[i], FMT("", "incoming_message_payloads_{}", i));

        incoming_message_vars[i].insert(incoming_message_vars[i].end(), incoming_message_types[i]);
        incoming_message_vars[i].insert(incoming_message_vars[i].end(), incoming_message_payloads[i].begin(), incoming_message_payloads[i].end());
    }

    local_data.allocate(pb, compliance_predicate.local_data_length, "local_data");
    cp_witness.allocate(pb, compliance_predicate.witness_length, "cp_witness");

    /* convert compliance predicate from a constraint system into a gadget */
    let mut  incoming_messages_concat=pb_variable_array::<FieldT>::new();
    for i in 0..compliance_predicate.max_arity
    {
        incoming_messages_concat.insert(incoming_messages_concat.end(), incoming_message_vars[i].begin(), incoming_message_vars[i].end());
    }

    compliance_predicate_as_gadget.reset(gadget_from_r1cs::<FieldT>::new(pb,
        [ outgoing_message_vars,
          pb_variable_array::<FieldT>(1, arity),
          incoming_messages_concat,
          local_data,
          cp_witness] ,
            compliance_predicate.constraint_system, "compliance_predicate_as_gadget"));

    /* unpack messages to bits */
    outgoing_message_bits.allocate(pb, outgoing_msg_size_in_bits, "outgoing_message_bits");
    unpack_outgoing_message.reset(multipacking_gadget::<FieldT>::new(pb, outgoing_message_bits, outgoing_message_vars, field_logsize(), "unpack_outgoing_message"));

    incoming_messages_bits.resize(compliance_predicate.max_arity);
    for i in 0..compliance_predicate.max_arity
    {
        let incoming_msg_size_in_bits = field_logsize() * (1 + compliance_predicate.incoming_message_payload_lengths[i]);

        incoming_messages_bits[i].allocate(pb, incoming_msg_size_in_bits, FMT("", "incoming_messages_bits_{}", i));
        unpack_incoming_messages.push(multipacking_gadget::<FieldT>(pb, incoming_messages_bits[i], incoming_message_vars[i], field_logsize(), FMT("", "unpack_incoming_messages_{}", i)));
    }

    /* allocate digests */
    commitment_and_incoming_message_digests.resize(compliance_predicate.max_arity);
    for i in 0..compliance_predicate.max_arity
    {
        commitment_and_incoming_message_digests[i].allocate(pb, digest_size, FMT("", "commitment_and_incoming_message_digests_{}", i));
    }

    /* allocate commitment, verification key(s) and membership checker(s)/proof(s) */
    commitment.reset( set_commitment_variable::<FieldT, CRH_with_bit_out_gadget::<FieldT> >::new(pb, commitment_size, "commitment"));

    ffec::print_indent(); print!("* {} perform same type optimization for compliance predicate with type {}\n",
                           if compliance_predicate.relies_on_same_type_inputs {"Will"} else{"Will NOT"},
                           compliance_predicate.types);
    if compliance_predicate.relies_on_same_type_inputs
    {
        /* only one set_commitment_gadget is needed */
        common_type.allocate(pb, "common_type");
        common_type_check_aux.allocate(pb, compliance_predicate.accepted_input_types.len(), "common_type_check_aux");

        translation_step_vks_bits.resize(1);
        translation_step_vks_bits[0].allocate(pb, translation_step_vk_size_in_bits, "translation_step_vk_bits");
        membership_check_results.allocate(pb, 1, "membership_check_results");

        membership_proofs.push(set_membership_proof_variable<FieldT, CRH_with_bit_out_gadget<FieldT>>(pb,
                                                                                                              max_number_of_predicates,
                                                                                                              "membership_proof"));
        membership_checkers.push(set_commitment_gadget<FieldT, CRH_with_bit_out_gadget<FieldT>>(pb,
                                                                                                        max_number_of_predicates,
                                                                                                        translation_step_vks_bits[0],
                                                                                                        *commitment,
                                                                                                        membership_proofs[0],
                                                                                                        membership_check_results[0], "membership_checker"));
    }
    else
    {
        /* check for max_arity possibly different VKs */
        translation_step_vks_bits.resize(compliance_predicate.max_arity);
        membership_check_results.allocate(pb, compliance_predicate.max_arity, "membership_check_results");

        for i in 0..compliance_predicate.max_arity
        {
            translation_step_vks_bits[i].allocate(pb, translation_step_vk_size_in_bits, FMT("", "translation_step_vks_bits_{}", i));

            membership_proofs.push(set_membership_proof_variable::<FieldT, CRH_with_bit_out_gadget::<FieldT> >(pb,
                                                                                                                   max_number_of_predicates,
                                                                                                                 FMT("", "membership_proof_{}", i)));
            membership_checkers.push(set_commitment_gadget::<FieldT, CRH_with_bit_out_gadget::<FieldT> >(pb,
                                                                                                             max_number_of_predicates,
                                                                                                             translation_step_vks_bits[i],
                                                                                                             *commitment,
                                                                                                             membership_proofs[i],
                                                                                                             membership_check_results[i],
                                                                                                           FMT("", "membership_checkers_{}", i)));
        }
    }

    /* allocate blocks */
    block_for_outgoing_message.reset(block_variable::<FieldT>::new(pb,  [commitment.bits, outgoing_message_bits] , "block_for_outgoing_message"));

    for i in 0..compliance_predicate.max_arity
    {
        block_for_incoming_messages.push(block_variable::<FieldT>(pb,  [commitment.bits, incoming_messages_bits[i]] , FMT("", "block_for_incoming_messages_{}", i)));
    }

    /* allocate hash checkers */
    hash_outgoing_message.reset(CRH_with_field_out_gadget::<FieldT>::new(pb, output_block_size, *block_for_outgoing_message, mp_compliance_step_pcd_circuit_input, "hash_outgoing_message"));

    for i in 0..compliance_predicate.max_arity
    {
        let input_block_size = commitment_size + incoming_messages_bits[i].len();
        hash_incoming_messages.push(CRH_with_field_out_gadget::<FieldT>(pb, input_block_size, block_for_incoming_messages[i], commitment_and_incoming_message_digests[i], FMT("", "hash_incoming_messages_{}", i)));
    }

    /* allocate useful zero variable */
    zero.allocate(pb, "zero");

    /* prepare arguments for the verifier */
    if compliance_predicate.relies_on_same_type_inputs
    {
        translation_step_vks.push(r1cs_ppzksnark_verification_key_variable::<ppT>(pb, translation_step_vks_bits[0], mp_translation_step_pcd_circuit_maker::<other_curve::<ppT> >::input_size_in_elts(), "translation_step_vk"));
    }
    else
    {
        for i in 0..compliance_predicate.max_arity
        {
            translation_step_vks.push(r1cs_ppzksnark_verification_key_variable::<ppT>(pb, translation_step_vks_bits[i], mp_translation_step_pcd_circuit_maker::<other_curve::<ppT> >::input_size_in_elts(), FMT("", "translation_step_vks_{}", i)));
        }
    }

    verification_results.allocate(pb, compliance_predicate.max_arity, "verification_results");
    commitment_and_incoming_messages_digest_bits.resize(compliance_predicate.max_arity);

    for i in 0..compliance_predicate.max_arity
    {
        commitment_and_incoming_messages_digest_bits[i].allocate(pb, digest_size * field_logsize(), FMT("", "commitment_and_incoming_messages_digest_bits_{}", i));
        unpack_commitment_and_incoming_message_digests.push(multipacking_gadget::<FieldT>(pb,
                                                                                                commitment_and_incoming_messages_digest_bits[i],
                                                                                                commitment_and_incoming_message_digests[i],
                                                                                                field_logsize(),
                                                                                              FMT("", "unpack_commitment_and_incoming_message_digests_{}", i)));

        verifier_input.push(commitment_and_incoming_messages_digest_bits[i]);
        while (verifier_input[i].len() < padded_verifier_input_size)
        {
            verifier_input[i].push(zero);
        }

        proof.push(r1cs_ppzksnark_proof_variable::<ppT>(pb, FMT("", "proof_{}", i)));
        let mut vk_to_be_used = if compliance_predicate.relies_on_same_type_inputs {translation_step_vks[0]} else{translation_step_vks[i]};
        verifier.push(r1cs_ppzksnark_verifier_gadget::<ppT>(pb,
                                                                  vk_to_be_used,
                                                                  verifier_input[i],
                                                                  mp_translation_step_pcd_circuit_maker::<other_curve::<ppT> >::field_capacity(),
                                                                  proof[i],
                                                                  verification_results[i],
                                                                FMT("", "verifier_{}", i)));
    }

    pb.set_input_sizes(input_size_in_elts());
    Self{compliance_predicate}
}


pub fn generate_r1cs_constraints()
{
    let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();
    let dimension = knapsack_dimension::<FieldT>::dimension;
    ffec::print_indent(); print!("* Knapsack dimension: {}\n", dimension);

    ffec::print_indent(); print!("* Compliance predicate arity: {}\n", compliance_predicate.max_arity);
    ffec::print_indent(); print!("* Compliance predicate outgoing payload length: {}\n", compliance_predicate.outgoing_message_payload_length);
    ffec::print_indent(); print!("* Compliance predicate incoming payload lengths:");
    for l in &compliance_predicate.incoming_message_payload_lengths
    {
        print!(" {}", l);
    }
    print!("\n");
    ffec::print_indent(); print!("* Compliance predicate local data length: {}\n", compliance_predicate.local_data_length);
    ffec::print_indent(); print!("* Compliance predicate witness length: {}\n", compliance_predicate.witness_length);

    PROFILE_CONSTRAINTS(pb, "booleanity");;
    {
        PROFILE_CONSTRAINTS(pb, "booleanity: unpack outgoing_message");;
        {
            unpack_outgoing_message.generate_r1cs_constraints(true);
        }

        PROFILE_CONSTRAINTS(pb, "booleanity: unpack s incoming_messages");;
        {
            for i in 0..compliance_predicate.max_arity
            {
                unpack_incoming_messages[i].generate_r1cs_constraints(true);
            }
        }

        PROFILE_CONSTRAINTS(pb, "booleanity: unpack verification key");;
        {
            for i in 0..translation_step_vks.len()
            {
                translation_step_vks[i].generate_r1cs_constraints(true);
            }
        }
    }

    PROFILE_CONSTRAINTS(pb, "(1+s) copies of hash");;
    {
        ffec::print_indent(); print!("* Digest-size: {}\n", digest_size);
        hash_outgoing_message.generate_r1cs_constraints();

        for i in 0..compliance_predicate.max_arity
        {
            hash_incoming_messages[i].generate_r1cs_constraints();
        }
    }

    PROFILE_CONSTRAINTS(pb, "s copies of repacking circuit for verifier");;
    {
        for i in 0..compliance_predicate.max_arity
        {
            unpack_commitment_and_incoming_message_digests[i].generate_r1cs_constraints(true);
        }
    }

    PROFILE_CONSTRAINTS(pb, "set membership check");;
    {
        for membership_proof in &membership_proofs
        {
            membership_proof.generate_r1cs_constraints();
        }

        for membership_checker in &membership_checkers
        {
            membership_checker.generate_r1cs_constraints();
        }
    }

    PROFILE_CONSTRAINTS(pb, "compliance predicate");;
    {
        compliance_predicate_as_gadget.generate_r1cs_constraints();
    }

    PROFILE_CONSTRAINTS(pb, "s copies of verifier for translated proofs");;
    {
        PROFILE_CONSTRAINTS(pb, "check that s proofs lie on the curve");;
        {
            for i in 0..compliance_predicate.max_arity
            {
                proof[i].generate_r1cs_constraints();
            }
        }

        for i in 0..compliance_predicate.max_arity
        {
            verifier[i].generate_r1cs_constraints();
        }
    }

    PROFILE_CONSTRAINTS(pb, "miscellaneous");;
    {
        generate_r1cs_equals_const_constraint::<FieldT>(pb, zero, FieldT::zero(), "zero");

        PROFILE_CONSTRAINTS(pb, "check that s proofs lie on the curve");;
        {
            for i in 0..compliance_predicate.max_arity
            {
                generate_boolean_r1cs_constraint::<FieldT>(pb, verification_results[i], FMT("", "verification_results_{}", i));
            }
        }

        /* either type = 0 or proof verified w.r.t. a valid verification key */
        PROFILE_CONSTRAINTS(pb, "check that s messages have valid proofs (or are base case)");;
        {
            for i in 0..compliance_predicate.max_arity
            {
                pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(incoming_message_types[i], 1 - verification_results[i], 0), FMT("", "not_base_case_implies_valid_proof_{}", i));
            }
        }

        if compliance_predicate.relies_on_same_type_inputs
        {
            PROFILE_CONSTRAINTS(pb, "check that all non-base case messages are of same type and that VK is validly selected");;
            {
                for i in 0..compliance_predicate.max_arity
                {
                    pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(incoming_message_types[i], incoming_message_types[i] - common_type, 0), FMT("", "non_base_types_equal_{}", i));
                }

                pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(common_type, 1 - membership_check_results[0], 0), "valid_vk_for_the_common_type");

                
                for  (i,it) in  compliance_predicate.accepted_input_types.iter().enumerate()
                {
                    pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(( if i == 0  {common_type} else{ common_type_check_aux[i-1]}),
                                                                   common_type - FieldT(*it),
                                                                   (if i == compliance_predicate.accepted_input_types.len() - 1  {0 * ONE }else {common_type_check_aux[i]})),
                                         FMT("", "common_type_in_prescribed_set_{}_must_equal_{}", i, *it));
                    
                }
            }
        }
        else
        {
            PROFILE_CONSTRAINTS(pb, "check that all s messages have validly selected VKs");;
            {
                for i in 0..compliance_predicate.max_arity
                {
                    pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(incoming_message_types[i], 1 - membership_check_results[i], 0), FMT("", "not_base_case_implies_valid_vk_{}", i));
                }
            }
        }
        pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, outgoing_message_type, FieldT(compliance_predicate.types)), "enforce_outgoing_type");
    }

    PRINT_CONSTRAINT_PROFILING();
    ffec::print_indent(); print!("* Number of constraints in mp_compliance_step_pcd_circuit: {}\n", pb.num_constraints());
}


 pub fn get_circuit() ->r1cs_constraint_system<ffec::Fr<ppT> >
{
    return pb.get_constraint_system();
}


 pub fn get_primary_input() ->r1cs_primary_input<ffec::Fr<ppT> >
{
    return pb.primary_input();
}


 pub fn get_auxiliary_input() ->r1cs_auxiliary_input<ffec::Fr<ppT> >
{
    return pb.auxiliary_input();
}


pub fn generate_r1cs_witness(commitment_to_translation_step_r1cs_vks:set_commitment,
                                                                      mp_translation_step_pcd_circuit_vks:Vec<r1cs_ppzksnark_verification_key<other_curve<ppT> > >,
                                                                      vk_membership_proofs:Vec<set_membership_proof>,
                                                                      compliance_predicate_primary_input:r1cs_pcd_compliance_predicate_primary_input<FieldT>,
                                                                      compliance_predicate_auxiliary_input:r1cs_pcd_compliance_predicate_auxiliary_input<FieldT>,
                                                                      translation_step_proofs:Vec<r1cs_ppzksnark_proof<other_curve<ppT> > >)
{
    self.pb.clear_values();
    self.pb.val(zero) = FieldT::zero();

    compliance_predicate_as_gadget.generate_r1cs_witness(compliance_predicate_primary_input.as_r1cs_primary_input(),
                                                          compliance_predicate_auxiliary_input.as_r1cs_auxiliary_input(compliance_predicate.incoming_message_payload_lengths));

    unpack_outgoing_message.generate_r1cs_witness_from_packed();
    for i in 0..compliance_predicate.max_arity
    {
        unpack_incoming_messages[i].generate_r1cs_witness_from_packed();
    }

    for i in 0..translation_step_vks.len()
    {
        translation_step_vks[i].generate_r1cs_witness(mp_translation_step_pcd_circuit_vks[i]);
    }

    commitment.generate_r1cs_witness(commitment_to_translation_step_r1cs_vks);

    if compliance_predicate.relies_on_same_type_inputs
    {
        /* all messages (except base case) must be of the same type */
        self.pb.val(common_type) = FieldT::zero();
        let mut  nonzero_type_idx = 0;
        for i in 0..compliance_predicate.max_arity
        {
            if self.pb.val(incoming_message_types[i]) == 0
            {
                continue;
            }

            if self.pb.val(common_type).is_zero()
            {
                self.pb.val(common_type) = self.pb.val(incoming_message_types[i]);
                nonzero_type_idx = i;
            }
            else
            {
                assert!(self.pb.val(common_type) == self.pb.val(incoming_message_types[i]));
            }
        }

        self.pb.val(membership_check_results[0]) = if self.pb.val(common_type).is_zero() {FieldT::zero()} else{FieldT::one()};
        membership_proofs[0].generate_r1cs_witness(vk_membership_proofs[nonzero_type_idx]);
        membership_checkers[0].generate_r1cs_witness();

       
        for (i,it) in compliance_predicate.accepted_input_types.iter().enumerate()
        {
            pb.val(common_type_check_aux[i]) = ((if i == 0 ? {pb.val(common_type)} else {pb.val(common_type_check_aux[i-1])}) *
                                                (pb.val(common_type) - FieldT(*it)));
        }
    }
    else
    {
        for i in 0..membership_checkers.len()
        {
            self.pb.val(membership_check_results[i])=  (if self.pb.val(incoming_message_types[i]).is_zero() {FieldT::zero()} else{FieldT::one()});
            membership_proofs[i].generate_r1cs_witness(vk_membership_proofs[i]);
            membership_checkers[i].generate_r1cs_witness();
        }
    }

    hash_outgoing_message.generate_r1cs_witness();
    for i in 0..compliance_predicate.max_arity
    {
        hash_incoming_messages[i].generate_r1cs_witness();
        unpack_commitment_and_incoming_message_digests[i].generate_r1cs_witness_from_packed();
    }

    for i in 0..compliance_predicate.max_arity
    {
        proof[i].generate_r1cs_witness(translation_step_proofs[i]);
        verifier[i].generate_r1cs_witness();
    }

// #ifdef DEBUG
    get_circuit(); // force generating constraints
    assert!(self.pb.is_satisfied());
//#endif
}


 pub fn field_logsize()->usize
{
    return ppT::Fr::size_in_bits();
}


pub fn field_capacity()->usize
{
    return ppT::Fr::capacity();
}


pub fn input_size_in_elts()->usize
{
    let digest_size = CRH_with_field_out_gadget::<FieldT>::get_digest_len();
    return digest_size;
}


pub fn input_capacity_in_bits()->usize
{
    return input_size_in_elts() * field_capacity();
}


pub fn input_size_in_bits()->usize
{
    return input_size_in_elts() * field_logsize();
}
}

impl mp_translation_step_pcd_circuit_maker<ppT>{

pub fn new(compliance_step_vk:r1cs_ppzksnark_verification_key<other_curve<ppT> >)->Self
{
    /* allocate input of the translation MP_PCD circuit */
    mp_translation_step_pcd_circuit_input.allocate(pb, input_size_in_elts(), "mp_translation_step_pcd_circuit_input");

    /* unpack translation step MP_PCD circuit input */
    unpacked_mp_translation_step_pcd_circuit_input.allocate(pb, mp_compliance_step_pcd_circuit_maker::<other_curve::<ppT> >::input_size_in_bits(), "unpacked_mp_translation_step_pcd_circuit_input");
    unpack_mp_translation_step_pcd_circuit_input.reset(multipacking_gadget::<FieldT>::new(pb, unpacked_mp_translation_step_pcd_circuit_input, mp_translation_step_pcd_circuit_input, field_capacity(), "unpack_mp_translation_step_pcd_circuit_input"));

    /* prepare arguments for the verifier */
    hardcoded_compliance_step_vk.reset(r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable::<ppT>::new(pb, compliance_step_vk, "hardcoded_compliance_step_vk"));
    proof.reset(r1cs_ppzksnark_proof_variable::<ppT>::new(pb, "proof"));

    /* verify previous proof */
    online_verifier.reset(r1cs_ppzksnark_online_verifier_gadget::<ppT>::new(pb,
                                                                         *hardcoded_compliance_step_vk,
                                                                         unpacked_mp_translation_step_pcd_circuit_input,
                                                                         mp_compliance_step_pcd_circuit_maker::<other_curve::<ppT> >::field_logsize(),
                                                                         *proof,
                                                                         ONE, // must always accept
                                                                         "verifier"));

    pb.set_input_sizes(input_size_in_elts());
}


pub fn generate_r1cs_constraints()
{
    PROFILE_CONSTRAINTS(pb, "repacking: unpack circuit input");;
    {
        unpack_mp_translation_step_pcd_circuit_input.generate_r1cs_constraints(true);
    }

    PROFILE_CONSTRAINTS(pb, "verifier for compliance proofs");;
    {
        PROFILE_CONSTRAINTS(pb, "check that proof lies on the curve");;
        {
            proof.generate_r1cs_constraints();
        }

        online_verifier.generate_r1cs_constraints();
    }

    PRINT_CONSTRAINT_PROFILING();
    ffec::print_indent(); print!("* Number of constraints in mp_translation_step_pcd_circuit: {}\n", pb.num_constraints());
}


pub fn get_circuit() ->r1cs_constraint_system<ffec::Fr<ppT> > 
{
    return pb.get_constraint_system();
}


pub fn generate_r1cs_witness(translation_step_input:r1cs_primary_input<ffec::Fr<ppT> >,
                                                                       prev_proof:r1cs_ppzksnark_proof<other_curve<ppT> >)
{
    self.pb.clear_values();
    mp_translation_step_pcd_circuit_input.fill_with_field_elements(pb, translation_step_input);
    unpack_mp_translation_step_pcd_circuit_input.generate_r1cs_witness_from_packed();

    proof.generate_r1cs_witness(prev_proof);
    online_verifier.generate_r1cs_witness();

// #ifdef DEBUG
    get_circuit(); // force generating constraints
    assert!(self.pb.is_satisfied());
//#endif
}


 pub fn get_primary_input() ->r1cs_primary_input<ffec::Fr<ppT> >
{
    return pb.primary_input();
}


pub fn get_auxiliary_input() ->r1cs_auxiliary_input<ffec::Fr<ppT> > 
{
    return pb.auxiliary_input();
}


 pub fn field_logsize()->usize
{
    return ppT::Fr::size_in_bits();
}


pub fn field_capacity()->usize
{
    return ppT::Fr::capacity();
}


pub fn input_size_in_elts()->usize
{
    return ffec::div_ceil(mp_compliance_step_pcd_circuit_maker::<other_curve::<ppT> >::input_size_in_bits(), Self::field_capacity());
}


pub fn input_capacity_in_bits()->usize
{
    return input_size_in_elts() * field_capacity();
}


pub fn input_size_in_bits()->usize
{
    return input_size_in_elts() * field_logsize();
}
}


 pub fn get_mp_compliance_step_pcd_circuit_input(commitment_to_translation_step_r1cs_vks:set_commitment,
                                                                      primary_input:r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<ppT> >)->r1cs_primary_input<ffec::Fr<ppT> >
{
    ffec::enter_block("Call to get_mp_compliance_step_pcd_circuit_input");
    type FieldT=ffec::Fr<ppT> ;

    let  outgoing_message_as_va = primary_input.outgoing_message.as_r1cs_variable_assignment();
    let mut  msg_bits=vec![];
    for elt in &outgoing_message_as_va
    {
        let elt_bits = ffec::convert_field_element_to_bit_vector(elt);
        msg_bits.insert(msg_bits.end(), elt_bits.begin(), elt_bits.end());
    }

    let mut  block=vec![];
    block.insert(block.end(), commitment_to_translation_step_r1cs_vks.begin(), commitment_to_translation_step_r1cs_vks.end());
    block.insert(block.end(), msg_bits.begin(), msg_bits.end());

    ffec::enter_block("Sample CRH randomness");
    CRH_with_field_out_gadget::<FieldT>::sample_randomness(block.len());
    ffec::leave_block("Sample CRH randomness");

    let  digest = CRH_with_field_out_gadget::<FieldT>::get_hash(block);
    ffec::leave_block("Call to get_mp_compliance_step_pcd_circuit_input");

    return digest;
}


pub fn get_mp_translation_step_pcd_circuit_input(commitment_to_translation_step_r1cs_vks:set_commitment,
                                                                       primary_input:r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<other_curve<ppT> > >)->r1cs_primary_input<ffec::Fr<ppT> > 
{
    ffec::enter_block("Call to get_mp_translation_step_pcd_circuit_input");
    type FieldT=ffec::Fr<ppT> ;

    let  mp_compliance_step_pcd_circuit_input = get_mp_compliance_step_pcd_circuit_input::<other_curve::<ppT> >(commitment_to_translation_step_r1cs_vks, primary_input);
    let mut mp_compliance_step_pcd_circuit_input_bits=vec![];
    for elt in &mp_compliance_step_pcd_circuit_input
    {
        let  elt_bits = ffec::convert_field_element_to_bit_vector::<ffec::Fr::<other_curve::<ppT> > >(elt);
        mp_compliance_step_pcd_circuit_input_bits.insert(mp_compliance_step_pcd_circuit_input_bits.end(), elt_bits.begin(), elt_bits.end());
    }

    mp_compliance_step_pcd_circuit_input_bits.resize(Self::input_capacity_in_bits(), false);

    let   result = ffec::pack_bit_vector_into_field_element_vector::<FieldT>(mp_compliance_step_pcd_circuit_input_bits, Self::field_capacity());
    ffec::leave_block("Call to get_mp_translation_step_pcd_circuit_input");

    return result;
}



//#endif // MP_PCD_CIRCUITS_TCC_
