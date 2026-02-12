// Declaration of functionality for creating and using the two PCD circuits in
// a multi-predicate PCD construction.

// The implementation follows, extends, and optimizes the approach described
// in \[CTV15]. At high level, there is a "compliance step" circuit and a
// "translation step" circuit, for each compliance predicate. For more details,
// see \[CTV15].

// References:

// \[CTV15]:
// "Cluster Computing in Zero Knowledge",
// Alessandro Chiesa, Eran Tromer, Madars Virza

// use crate::gadgetlib1::gadget;
// use crate::gadgetlib1::gadgets::gadget_from_r1cs;
// use crate::gadgetlib1::gadgets::hashes::crh_gadget;
// use crate::gadgetlib1::gadgets::set_commitment::set_commitment_gadget;
// use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget;
// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;

use crate::common::data_structures::merkle_tree::HashTConfig;
use crate::common::data_structures::set_commitment::{set_commitment, set_membership_proof};
use crate::gadgetlib1::constraint_profiling::PRINT_CONSTRAINT_PROFILING;
use crate::gadgetlib1::constraint_profiling::PROFILE_CONSTRAINTS;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_boolean_r1cs_constraint, generate_r1cs_equals_const_constraint, multipacking_gadget,
    multipacking_gadgets,
};
use crate::gadgetlib1::gadgets::gadget_from_r1cs::{gadget_from_r1cs, gadget_from_r1css};
use crate::gadgetlib1::gadgets::hashes::crh_gadget::{
    CRH_with_bit_out_gadget, CRH_with_bit_out_gadgets, CRH_with_field_out_gadget,
    CRH_with_field_out_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::hash_io::{block_variable, block_variables};
use crate::gadgetlib1::gadgets::hashes::knapsack::knapsack_gadget::knapsack_dimension;
use crate::gadgetlib1::gadgets::pairing::pairing_params::other_curve;
use crate::gadgetlib1::gadgets::pairing::pairing_params::pairing_selector;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::gadgets::set_commitment::set_commitment_gadget::set_commitment_variables;
use crate::gadgetlib1::gadgets::set_commitment::set_commitment_gadget::{
    set_commitment_gadget, set_commitment_gadgets, set_commitment_variable,
};
use crate::gadgetlib1::gadgets::set_commitment::set_membership_proof_variable::{
    set_membership_proof_variable, set_membership_proof_variables,
};
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget::{
    r1cs_ppzksnark_online_verifier_gadget, r1cs_ppzksnark_online_verifier_gadgets,
    r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable,
    r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables,
    r1cs_ppzksnark_proof_variable, r1cs_ppzksnark_proof_variables,
    r1cs_ppzksnark_verification_key_variable, r1cs_ppzksnark_verification_key_variables,
    r1cs_ppzksnark_verifier_gadget, r1cs_ppzksnark_verifier_gadgets,
};
use crate::gadgetlib1::pb_variable::ONE;
use crate::gadgetlib1::pb_variable::pb_linear_combination;
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint_system, r1cs_primary_input,
};
use crate::relations::variable::linear_combination;
use crate::relations::variable::variable;

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::LocalDataConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::MessageConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::r1cs_pcd_compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::r1cs_pcd_compliance_predicate_auxiliary_input;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::r1cs_pcd_compliance_predicate_primary_input;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_proof, r1cs_ppzksnark_verification_key,
};
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::bit_vector;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::div_ceil;
use ffec::field_utils::field_utils::{
    convert_field_element_to_bit_vector, pack_bit_vector_into_field_element_vector1,
};
use ffec::{One, Zero};
use rccell::RcCell;
use std::ops::Mul;

/**
 * A compliance-step PCD circuit.
 *
 * The circuit is an R1CS that checks compliance (for the given compliance predicate)
 * and validity of previous proofs.
 */
// pub trait FieldTConfig {
//     type ppT::FieldT;
// }
// impl<ppT> FieldTConfig for mp_compliance_step_pcd_circuit_maker<ppT> {
//     type ppT::FieldT = Fr<ppT>;
// }
pub struct mp_compliance_step_pcd_circuit_maker<ppT: ppTConfig> {
    pub compliance_predicate: r1cs_pcd_compliance_predicate<ppT::FieldT, ppT>,
    pub pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
    pub zero: variable<ppT::FieldT, pb_variable>,
    pub block_for_outgoing_message: RcCell<block_variables<ppT::FieldT, ppT::PB>>,
    pub hash_outgoing_message: RcCell<CRH_with_field_out_gadgets<ppT::FieldT, ppT::PB>>,
    pub block_for_incoming_messages: Vec<block_variables<ppT::FieldT, ppT::PB>>,
    pub commitment_and_incoming_message_digests: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub unpack_commitment_and_incoming_message_digests:
        Vec<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    pub commitment_and_incoming_messages_digest_bits: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub hash_incoming_messages: Vec<CRH_with_field_out_gadgets<ppT::FieldT, ppT::PB>>,
    pub translation_step_vks: Vec<r1cs_ppzksnark_verification_key_variables<ppT>>,
    pub translation_step_vks_bits: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub outgoing_message_type: variable<ppT::FieldT, pb_variable>,
    pub outgoing_message_payload: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub outgoing_message_vars: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub arity: variable<ppT::FieldT, pb_variable>,
    pub incoming_message_types: Vec<variable<ppT::FieldT, pb_variable>>,
    pub incoming_message_payloads: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub incoming_message_vars: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub local_data: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub cp_witness: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub compliance_predicate_as_gadget: RcCell<gadget_from_r1css<ppT::FieldT, ppT::PB>>,
    pub outgoing_message_bits: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub unpack_outgoing_message: RcCell<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    pub incoming_messages_bits: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub unpack_incoming_messages: Vec<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    pub mp_compliance_step_pcd_circuit_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub padded_translation_step_vk_and_outgoing_message_digest:
        pb_variable_array<ppT::FieldT, ppT::PB>,
    pub padded_commitment_and_incoming_messages_digest:
        Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub commitment: RcCell<set_commitment_variables<ppT::FieldT, ppT::PB>>,
    pub membership_proofs: Vec<
        set_membership_proof_variables<
            ppT::FieldT,
            ppT::PB,
            CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
        >,
    >,
    pub membership_checkers: Vec<
        set_commitment_gadgets<
            ppT::FieldT,
            ppT::PB,
            CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
        >,
    >,
    pub membership_check_results: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub common_type: variable<ppT::FieldT, pb_variable>,
    pub common_type_check_aux: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub verifier_input: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub proof: Vec<r1cs_ppzksnark_proof_variables<ppT>>,
    pub verification_results: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub verifier: Vec<r1cs_ppzksnark_verifier_gadgets<ppT>>,
}

/**
 * A translation-step PCD circuit.
 *
 * The circuit is an R1CS that checks validity of previous proofs.
 */

// impl<ppT> FieldTConfig for mp_translation_step_pcd_circuit_maker<ppT> {
//     type ppT::FieldT = Fr<ppT>;
// }
pub struct mp_translation_step_pcd_circuit_maker<ppT: ppTConfig> {
    pub pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
    pub mp_translation_step_pcd_circuit_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub unpacked_mp_translation_step_pcd_circuit_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub verifier_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub unpack_mp_translation_step_pcd_circuit_input:
        RcCell<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    pub hardcoded_compliance_step_vk:
        RcCell<r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>>,
    pub proof: RcCell<r1cs_ppzksnark_proof_variables<ppT>>,
    pub online_verifier: RcCell<r1cs_ppzksnark_online_verifier_gadgets<ppT>>,
}

//
// /**
//  * Obtain the primary input for a compliance-step PCD circuit.
//  */
//
// r1cs_primary_input<Fr<ppT> > get_mp_compliance_step_pcd_circuit_input(commitment_to_translation_step_r1cs_vks:set_commitment,
//                                                                       primary_input:r1cs_pcd_compliance_predicate_primary_input<Fr<ppT> >);

// /**
//  * Obtain the primary input for a translation-step PCD circuit.
//  */
//
// r1cs_primary_input<Fr<ppT> > get_mp_translation_step_pcd_circuit_input(commitment_to_translation_step_r1cs_vks:set_commitment,
//                                                                        primary_input:r1cs_pcd_compliance_predicate_primary_input<Fr<other_curve<ppT> > >);

// use  <algorithm>
// use common::utils;

// use crate::gadgetlib1::constraint_profiling;

impl<ppT: ppTConfig> mp_compliance_step_pcd_circuit_maker<ppT> {
    pub fn new(
        compliance_predicate: r1cs_pcd_compliance_predicate<ppT::FieldT, ppT>,
        max_number_of_predicates: usize,
    ) -> Self {
        let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
        /* calculate some useful sizes */
        let digest_size = CRH_with_field_out_gadgets::<ppT::FieldT, ppT::PB>::get_digest_len();
        let outgoing_msg_size_in_bits =
            Self::field_logsize() * (1 + compliance_predicate.outgoing_message_payload_length);
        assert!(compliance_predicate.has_equal_input_lengths());
        let translation_step_vk_size_in_bits =
            r1cs_ppzksnark_verification_key_variable::<ppT>::size_in_bits(
                mp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_elts(),
            );
        let padded_verifier_input_size =
            mp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_capacity_in_bits();
        let commitment_size = set_commitment_gadget::<
            ppT::FieldT,
            ppT::PB,
            CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
        >::root_size_in_bits();

        let output_block_size = commitment_size + outgoing_msg_size_in_bits;
        let max_incoming_payload_length = *compliance_predicate
            .incoming_message_payload_lengths
            .iter()
            .max()
            .unwrap();
        let max_input_block_size =
            commitment_size + Self::field_logsize() * (1 + max_incoming_payload_length);

        CRH_with_bit_out_gadget::<ppT::FieldT, ppT::PB>::sample_randomness(std::cmp::max(
            output_block_size,
            max_input_block_size,
        ));

        /* allocate input of the compliance MP_PCD circuit */
        let mut mp_compliance_step_pcd_circuit_input =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        mp_compliance_step_pcd_circuit_input.allocate(
            &pb,
            Self::input_size_in_elts(),
            "mp_compliance_step_pcd_circuit_input",
        );

        /* allocate inputs to the compliance predicate */
        let mut outgoing_message_type = variable::<ppT::FieldT, pb_variable>::default();
        let mut outgoing_message_payload = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        outgoing_message_type.allocate(&pb, "outgoing_message_type");
        outgoing_message_payload.allocate(
            &pb,
            compliance_predicate.outgoing_message_payload_length,
            "outgoing_message_payload",
        );
        let mut outgoing_message_vars = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        outgoing_message_vars
            .contents
            .push(outgoing_message_type.clone());
        outgoing_message_vars
            .contents
            .extend(outgoing_message_payload.clone());
        let mut arity = variable::<ppT::FieldT, pb_variable>::default();
        arity.allocate(&pb, "arity");

        let mut incoming_message_types =
            vec![variable::<ppT::FieldT, pb_variable>::default(); compliance_predicate.max_arity];
        let mut incoming_message_payloads = vec![
            pb_variable_array::<ppT::FieldT, ppT::PB>::default(
            );
            compliance_predicate.max_arity
        ];
        let mut incoming_message_vars = vec![
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
            compliance_predicate.max_arity
        ];
        for i in 0..compliance_predicate.max_arity {
            incoming_message_types[i]
                .allocate(&pb, prefix_format!("", "incoming_message_type_{}", i));
            incoming_message_payloads[i].allocate(
                &pb,
                compliance_predicate.incoming_message_payload_lengths[i],
                prefix_format!("", "incoming_message_payloads_{}", i),
            );

            incoming_message_vars[i]
                .contents
                .push(incoming_message_types[i].clone());
            incoming_message_vars[i]
                .contents
                .extend(incoming_message_payloads[i].clone());
        }
        let mut local_data = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        local_data.allocate(&pb, compliance_predicate.local_data_length, "local_data");
        let mut cp_witness = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        cp_witness.allocate(&pb, compliance_predicate.witness_length, "cp_witness");

        /* convert compliance predicate from a constraint system into a gadget */
        let mut incoming_messages_concat = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        for i in 0..compliance_predicate.max_arity {
            incoming_messages_concat
                .contents
                .extend(incoming_message_vars[i].clone());
        }

        let compliance_predicate_as_gadget =
            RcCell::new(gadget_from_r1cs::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                vec![
                    outgoing_message_vars.clone(),
                    pb_variable_array::<ppT::FieldT, ppT::PB>::new(vec![arity.clone()]),
                    incoming_messages_concat,
                    local_data.clone(),
                    cp_witness.clone(),
                ],
                compliance_predicate.constraint_system.clone(),
                "compliance_predicate_as_gadget".to_owned(),
            ));

        /* unpack messages to bits */
        let mut outgoing_message_bits = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        outgoing_message_bits.allocate(&pb, outgoing_msg_size_in_bits, "outgoing_message_bits");
        let unpack_outgoing_message =
            RcCell::new(multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                outgoing_message_bits.clone().into(),
                outgoing_message_vars.clone().into(),
                Self::field_logsize(),
                "unpack_outgoing_message".to_owned(),
            ));

        let mut incoming_messages_bits = vec![
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
            compliance_predicate.max_arity
        ];
        let mut unpack_incoming_messages = vec![];
        for i in 0..compliance_predicate.max_arity {
            let incoming_msg_size_in_bits = Self::field_logsize()
                * (1 + compliance_predicate.incoming_message_payload_lengths[i]);

            incoming_messages_bits[i].allocate(
                &pb,
                incoming_msg_size_in_bits,
                prefix_format!("", "incoming_messages_bits_{}", i),
            );
            unpack_incoming_messages.push(multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                incoming_messages_bits[i].clone().into(),
                incoming_message_vars[i].clone().into(),
                Self::field_logsize(),
                prefix_format!("", "unpack_incoming_messages_{}", i),
            ));
        }

        /* allocate digests */
        let mut commitment_and_incoming_message_digests =
            vec![
                pb_variable_array::<ppT::FieldT, ppT::PB>::default();
                compliance_predicate.max_arity
            ];
        for i in 0..compliance_predicate.max_arity {
            commitment_and_incoming_message_digests[i].allocate(
                &pb,
                digest_size,
                prefix_format!("", "commitment_and_incoming_message_digests_{}", i),
            );
        }

        /* allocate commitment, verification key(s) and membership checker(s)/proof(s) */
        let commitment = RcCell::new(set_commitment_variable::<ppT::FieldT, ppT::PB>::new(
            pb.clone(),
            commitment_size,
            "commitment".to_owned(),
        ));

        print_indent();
        print!(
            "* {} perform same type optimization for compliance predicate with type {}\n",
            if compliance_predicate.relies_on_same_type_inputs {
                "Will"
            } else {
                "Will NOT"
            },
            compliance_predicate.types
        );
        let mut common_type = variable::<ppT::FieldT, pb_variable>::default();
        let mut common_type_check_aux = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        let mut membership_check_results = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        let mut translation_step_vks_bits = vec![];
        let mut membership_proofs = vec![];
        let mut membership_checkers = vec![];
        if compliance_predicate.relies_on_same_type_inputs {
            /* only one set_commitment_gadget is needed */
            common_type.allocate(&pb, "common_type");
            common_type_check_aux.allocate(
                &pb,
                compliance_predicate.accepted_input_types.len(),
                "common_type_check_aux",
            );

            translation_step_vks_bits
                .resize(1, pb_variable_array::<ppT::FieldT, ppT::PB>::default());
            translation_step_vks_bits[0].allocate(
                &pb,
                translation_step_vk_size_in_bits,
                "translation_step_vk_bits",
            );
            membership_check_results.allocate(&pb, 1, "membership_check_results");

            membership_proofs.push(set_membership_proof_variable::<
                ppT::FieldT,
                ppT::PB,
                CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
            >::new(
                pb.clone(),
                max_number_of_predicates,
                "membership_proof".to_owned(),
            ));
            membership_checkers.push(set_commitment_gadget::<
                ppT::FieldT,
                ppT::PB,
                CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
            >::new(
                pb.clone(),
                max_number_of_predicates,
                translation_step_vks_bits[0].clone(),
                commitment.borrow().clone(),
                membership_proofs[0].clone(),
                membership_check_results[0].clone().into(),
                "membership_checker".to_owned(),
            ));
        } else {
            /* check for max_arity possibly different VKs */
            translation_step_vks_bits.resize(
                compliance_predicate.max_arity,
                pb_variable_array::<ppT::FieldT, ppT::PB>::default(),
            );
            membership_check_results.allocate(
                &pb,
                compliance_predicate.max_arity,
                "membership_check_results",
            );

            for i in 0..compliance_predicate.max_arity {
                translation_step_vks_bits[i].allocate(
                    &pb,
                    translation_step_vk_size_in_bits,
                    prefix_format!("", "translation_step_vks_bits_{}", i),
                );

                membership_proofs.push(set_membership_proof_variable::<
                    ppT::FieldT,
                    ppT::PB,
                    CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
                >::new(
                    pb.clone(),
                    max_number_of_predicates,
                    prefix_format!("", "membership_proof_{}", i),
                ));
                membership_checkers.push(set_commitment_gadget::<
                    ppT::FieldT,
                    ppT::PB,
                    CRH_with_bit_out_gadgets<ppT::FieldT, ppT::PB>,
                >::new(
                    pb.clone(),
                    max_number_of_predicates,
                    translation_step_vks_bits[i].clone(),
                    commitment.borrow().clone(),
                    membership_proofs[i].clone(),
                    membership_check_results[i].clone().into(),
                    prefix_format!("", "membership_checkers_{}", i),
                ));
            }
        }

        /* allocate blocks */
        let block_for_outgoing_message = RcCell::new(block_variable::<ppT::FieldT, ppT::PB>::new2(
            pb.clone(),
            vec![
                commitment.borrow().t.bits.clone(),
                outgoing_message_bits.clone(),
            ],
            "block_for_outgoing_message".to_owned(),
        ));
        let mut block_for_incoming_messages = vec![];
        for i in 0..compliance_predicate.max_arity {
            block_for_incoming_messages.push(block_variable::<ppT::FieldT, ppT::PB>::new2(
                pb.clone(),
                vec![
                    commitment.borrow().t.bits.clone(),
                    incoming_messages_bits[i].clone(),
                ],
                prefix_format!("", "block_for_incoming_messages_{}", i),
            ));
        }

        /* allocate hash checkers */
        let hash_outgoing_message =
            RcCell::new(CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                output_block_size,
                block_for_outgoing_message.borrow().clone(),
                mp_compliance_step_pcd_circuit_input.clone().into(),
                "hash_outgoing_message".to_owned(),
            ));
        let mut hash_incoming_messages = vec![];
        for i in 0..compliance_predicate.max_arity {
            let input_block_size = commitment_size + incoming_messages_bits[i].len();
            hash_incoming_messages.push(CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                input_block_size,
                block_for_incoming_messages[i].clone(),
                commitment_and_incoming_message_digests[i].clone().into(),
                prefix_format!("", "hash_incoming_messages_{}", i),
            ));
        }

        /* allocate useful zero variable */
        let mut zero = variable::<ppT::FieldT, pb_variable>::default();
        zero.allocate(&pb, "zero");

        /* prepare arguments for the verifier */
        let mut translation_step_vks = vec![];
        if compliance_predicate.relies_on_same_type_inputs {
            translation_step_vks.push(r1cs_ppzksnark_verification_key_variable::<ppT>::new(
                pb.clone(),
                translation_step_vks_bits[0].clone(),
                mp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_elts(),
                "translation_step_vk".to_owned(),
            ));
        } else {
            for i in 0..compliance_predicate.max_arity {
                translation_step_vks.push(r1cs_ppzksnark_verification_key_variable::<ppT>::new(
                    pb.clone(),
                    translation_step_vks_bits[i].clone(),
                    mp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_elts(),
                    prefix_format!("", "translation_step_vks_{}", i),
                ));
            }
        }
        let mut verification_results = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        verification_results.allocate(&pb, compliance_predicate.max_arity, "verification_results");
        let mut commitment_and_incoming_messages_digest_bits =
            vec![
                pb_variable_array::<ppT::FieldT, ppT::PB>::default();
                compliance_predicate.max_arity
            ];
        let mut unpack_commitment_and_incoming_message_digests = vec![];
        let mut verifier_input = vec![];
        let mut proof = vec![];
        let mut verifier = vec![];
        for i in 0..compliance_predicate.max_arity {
            commitment_and_incoming_messages_digest_bits[i].allocate(
                &pb,
                digest_size * Self::field_logsize(),
                prefix_format!("", "commitment_and_incoming_messages_digest_bits_{}", i),
            );
            unpack_commitment_and_incoming_message_digests.push(multipacking_gadget::<
                ppT::FieldT,
                ppT::PB,
            >::new(
                pb.clone(),
                commitment_and_incoming_messages_digest_bits[i]
                    .clone()
                    .into(),
                commitment_and_incoming_message_digests[i].clone().into(),
                Self::field_logsize(),
                prefix_format!("", "unpack_commitment_and_incoming_message_digests_{}", i),
            ));

            verifier_input.push(commitment_and_incoming_messages_digest_bits[i].clone());
            while (verifier_input[i].len() < padded_verifier_input_size) {
                verifier_input[i].contents.push(zero.clone());
            }

            proof.push(r1cs_ppzksnark_proof_variable::<ppT>::new(
                pb.clone(),
                prefix_format!("", "proof_{}", i),
            ));
            let mut vk_to_be_used = if compliance_predicate.relies_on_same_type_inputs {
                translation_step_vks[0].clone()
            } else {
                translation_step_vks[i].clone()
            };
            verifier.push(r1cs_ppzksnark_verifier_gadget::<ppT>::new(
                pb.clone(),
                vk_to_be_used,
                verifier_input[i].clone(),
                mp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::field_capacity(),
                proof[i].clone(),
                verification_results[i].clone(),
                prefix_format!("", "verifier_{}", i),
            ));
        }

        pb.borrow_mut().set_input_sizes(Self::input_size_in_elts());
        Self {
            compliance_predicate,
            pb,
            zero,
            block_for_outgoing_message,
            hash_outgoing_message,
            block_for_incoming_messages,
            commitment_and_incoming_message_digests,
            unpack_commitment_and_incoming_message_digests,
            commitment_and_incoming_messages_digest_bits,
            hash_incoming_messages,
            translation_step_vks,
            translation_step_vks_bits,
            outgoing_message_type,
            outgoing_message_payload,
            outgoing_message_vars,
            arity,
            incoming_message_types,
            incoming_message_payloads,
            incoming_message_vars,
            local_data,
            cp_witness,
            compliance_predicate_as_gadget,
            outgoing_message_bits,
            unpack_outgoing_message,
            incoming_messages_bits,
            unpack_incoming_messages,
            mp_compliance_step_pcd_circuit_input,
            padded_translation_step_vk_and_outgoing_message_digest: pb_variable_array::<
                <ppT as ppTConfig>::FieldT,
                <ppT as ppTConfig>::PB,
            >::default(),
            padded_commitment_and_incoming_messages_digest: vec![],
            commitment,
            membership_proofs,
            membership_checkers,
            membership_check_results,
            common_type,
            common_type_check_aux,
            verifier_input,
            proof,
            verification_results,
            verifier,
        }
    }

    pub fn generate_r1cs_constraints(&self) {
        let digest_size = CRH_with_bit_out_gadgets::<ppT::FieldT, ppT::PB>::get_digest_len();
        let dimension = knapsack_dimension::<ppT::FieldT>::dimension;
        print_indent();
        print!("* Knapsack dimension: {}\n", dimension);

        print_indent();
        print!(
            "* Compliance predicate arity: {}\n",
            self.compliance_predicate.max_arity
        );
        print_indent();
        print!(
            "* Compliance predicate outgoing payload length: {}\n",
            self.compliance_predicate.outgoing_message_payload_length
        );
        print_indent();
        print!("* Compliance predicate incoming payload lengths:");
        for l in &self.compliance_predicate.incoming_message_payload_lengths {
            print!(" {}", l);
        }
        print!("\n");
        print_indent();
        print!(
            "* Compliance predicate local data length: {}\n",
            self.compliance_predicate.local_data_length
        );
        print_indent();
        print!(
            "* Compliance predicate witness length: {}\n",
            self.compliance_predicate.witness_length
        );

        PROFILE_CONSTRAINTS(&self.pb, "booleanity");
        {
            PROFILE_CONSTRAINTS(&self.pb, "booleanity: unpack outgoing_message");
            {
                self.unpack_outgoing_message
                    .borrow()
                    .generate_r1cs_constraints(true);
            }

            PROFILE_CONSTRAINTS(&self.pb, "booleanity: unpack s incoming_messages");
            {
                for i in 0..self.compliance_predicate.max_arity {
                    self.unpack_incoming_messages[i].generate_r1cs_constraints(true);
                }
            }

            PROFILE_CONSTRAINTS(&self.pb, "booleanity: unpack verification key");
            {
                for i in 0..self.translation_step_vks.len() {
                    self.translation_step_vks[i].generate_r1cs_constraints(true);
                }
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "(1+s) copies of hash");
        {
            print_indent();
            print!("* Digest-size: {}\n", digest_size);
            self.hash_outgoing_message
                .borrow()
                .generate_r1cs_constraints(true);

            for i in 0..self.compliance_predicate.max_arity {
                self.hash_incoming_messages[i].generate_r1cs_constraints(true);
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "s copies of repacking circuit for verifier");
        {
            for i in 0..self.compliance_predicate.max_arity {
                self.unpack_commitment_and_incoming_message_digests[i]
                    .generate_r1cs_constraints(true);
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "set membership check");
        {
            for membership_proof in &self.membership_proofs {
                membership_proof.generate_r1cs_constraints();
            }

            for membership_checker in &self.membership_checkers {
                membership_checker.generate_r1cs_constraints();
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "compliance predicate");
        {
            self.compliance_predicate_as_gadget
                .borrow_mut()
                .generate_r1cs_constraints();
        }

        PROFILE_CONSTRAINTS(&self.pb, "s copies of verifier for translated proofs");
        {
            PROFILE_CONSTRAINTS(&self.pb, "check that s proofs lie on the curve");
            {
                for i in 0..self.compliance_predicate.max_arity {
                    self.proof[i].generate_r1cs_constraints();
                }
            }

            for i in 0..self.compliance_predicate.max_arity {
                self.verifier[i].generate_r1cs_constraints();
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "miscellaneous");
        {
            generate_r1cs_equals_const_constraint::<ppT::FieldT, ppT::PB>(
                &self.pb,
                &(self.zero.clone().into()),
                &ppT::FieldT::zero(),
                "zero".to_owned(),
            );

            PROFILE_CONSTRAINTS(&self.pb, "check that s proofs lie on the curve");
            {
                for i in 0..self.compliance_predicate.max_arity {
                    generate_boolean_r1cs_constraint::<ppT::FieldT, ppT::PB>(
                        &self.pb,
                        &(self.verification_results[i].clone().into()),
                        prefix_format!("", "verification_results_{}", i),
                    );
                }
            }

            /* either type = 0 or proof verified w.r.t. a valid verification key */
            PROFILE_CONSTRAINTS(
                &self.pb,
                "check that s messages have valid proofs (or are base case)",
            );
            {
                for i in 0..self.compliance_predicate.max_arity {
                    self.pb.borrow_mut().add_r1cs_constraint(
                        r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                            self.incoming_message_types[i].clone().into(),
                            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        ppT::FieldT::from(1),
                    ) - self.verification_results[i].clone(),
                            ppT::FieldT::from(0).into(),
                        ),
                        prefix_format!("", "not_base_case_implies_valid_proof_{}", i),
                    );
                }
            }

            if self.compliance_predicate.relies_on_same_type_inputs {
                PROFILE_CONSTRAINTS(
                    &self.pb,
                    "check that all non-base case messages are of same type and that VK is validly selected",
                );
                {
                    for i in 0..self.compliance_predicate.max_arity {
                        self.pb.borrow_mut().add_r1cs_constraint(
                            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                                self.incoming_message_types[i].clone().into(),
                                self.incoming_message_types[i].clone()
                                    - self.common_type.clone().into(),
                                ppT::FieldT::from(0).into(),
                            ),
                            prefix_format!("", "non_base_types_equal_{}", i),
                        );
                    }

                    self.pb.borrow_mut().add_r1cs_constraint(
                        r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                            self.common_type.clone().into(),
                            linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        ppT::FieldT::from(1),
                    ) - self.membership_check_results[0].clone(),
                           ppT::FieldT::from(0).into(),
                        ),
                        "valid_vk_for_the_common_type".to_owned(),
                    );

                    for (i, &it) in self
                        .compliance_predicate
                        .accepted_input_types
                        .iter()
                        .enumerate()
                    {
                        self.pb.borrow_mut().add_r1cs_constraint(
                            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                                (if i == 0 {
                                    self.common_type.clone()
                                } else {
                                    self.common_type_check_aux[i - 1].clone()
                                })
                                .into(),
                                (linear_combination::<
                                    ppT::FieldT,
                                    pb_variable,
                                    pb_linear_combination,
                                >::from(self.common_type.clone())
                                    - linear_combination::<
                                        ppT::FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(
                                        ppT::FieldT::from(it)
                                    )),
                                (if i == self.compliance_predicate.accepted_input_types.len() - 1 {
                                    linear_combination::<
                                        ppT::FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(ppT::FieldT::from(
                                        0,
                                    )) * variable::<ppT::FieldT, pb_variable>::from(ONE)
                                } else {
                                    self.common_type_check_aux[i].clone().into()
                                }),
                            ),
                            prefix_format!(
                                "",
                                "common_type_in_prescribed_set_{}_must_equal_{}",
                                i,
                                it
                            ),
                        );
                    }
                }
            } else {
                PROFILE_CONSTRAINTS(
                    &self.pb,
                    "check that all s messages have validly selected VKs",
                );
                {
                    for i in 0..self.compliance_predicate.max_arity {
                        self.pb.borrow_mut().add_r1cs_constraint(
                            r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                                self.incoming_message_types[i].clone().into(),
                                linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        ppT::FieldT::from(1),
                    ) - self.membership_check_results[i].clone(),
                                ppT::FieldT::from(0).into(),
                            ),
                            prefix_format!("", "not_base_case_implies_valid_vk_{}", i),
                        );
                    }
                }
            }
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                    ppT::FieldT::from(1).into(),
                    self.outgoing_message_type.clone().into(),
                    ppT::FieldT::from(self.compliance_predicate.types).into(),
                ),
                "enforce_outgoing_type".to_owned(),
            );
        }

        PRINT_CONSTRAINT_PROFILING();
        print_indent();
        print!(
            "* Number of constraints in mp_compliance_step_pcd_circuit: {}\n",
            self.pb.borrow().num_constraints()
        );
    }

    pub fn get_circuit(
        &self,
    ) -> r1cs_constraint_system<Fr<ppT>, pb_variable, pb_linear_combination> {
        self.pb.borrow().get_constraint_system()
    }

    pub fn get_primary_input(&self) -> r1cs_primary_input<Fr<ppT>> {
        self.pb.borrow().primary_input()
    }

    pub fn get_auxiliary_input(&self) -> r1cs_auxiliary_input<Fr<ppT>> {
        self.pb.borrow().auxiliary_input()
    }

    pub fn generate_r1cs_witness(
        &self,
        commitment_to_translation_step_r1cs_vks: &set_commitment,
        mp_translation_step_pcd_circuit_vks: &Vec<
            r1cs_ppzksnark_verification_key<other_curve<ppT>>,
        >,
        vk_membership_proofs: &Vec<set_membership_proof>,
        compliance_predicate_primary_input: &r1cs_pcd_compliance_predicate_primary_input<
            ppT::FieldT,
            ppT::M,
        >,
        compliance_predicate_auxiliary_input: &r1cs_pcd_compliance_predicate_auxiliary_input<
            ppT::FieldT,
            ppT::M,
            ppT::LD,
        >,
        translation_step_proofs: &Vec<r1cs_ppzksnark_proof<other_curve<ppT>>>,
    ) {
        self.pb.borrow_mut().clear_values();
        *self.pb.borrow_mut().val_ref(&self.zero) = ppT::FieldT::zero();

        self.compliance_predicate_as_gadget
            .borrow()
            .generate_r1cs_witness(
                &compliance_predicate_primary_input.as_r1cs_primary_input(),
                &compliance_predicate_auxiliary_input.as_r1cs_auxiliary_input(
                    &self.compliance_predicate.incoming_message_payload_lengths,
                ),
            );

        self.unpack_outgoing_message
            .borrow()
            .generate_r1cs_witness_from_packed();
        for i in 0..self.compliance_predicate.max_arity {
            self.unpack_incoming_messages[i].generate_r1cs_witness_from_packed();
        }

        for i in 0..self.translation_step_vks.len() {
            self.translation_step_vks[i]
                .generate_r1cs_witness(&mp_translation_step_pcd_circuit_vks[i]);
        }

        self.commitment
            .borrow()
            .generate_r1cs_witness(&commitment_to_translation_step_r1cs_vks);

        if self.compliance_predicate.relies_on_same_type_inputs {
            /* all messages (except base case) must be of the same type */
            *self.pb.borrow_mut().val_ref(&self.common_type) = ppT::FieldT::zero();
            let mut nonzero_type_idx = 0;
            for i in 0..self.compliance_predicate.max_arity {
                if self.pb.borrow().val(&self.incoming_message_types[i]) == ppT::FieldT::zero() {
                    continue;
                }

                if self.pb.borrow().val(&self.common_type).is_zero() {
                    *self.pb.borrow_mut().val_ref(&self.common_type) =
                        self.pb.borrow().val(&self.incoming_message_types[i]);
                    nonzero_type_idx = i;
                } else {
                    assert!(
                        self.pb.borrow().val(&self.common_type)
                            == self.pb.borrow().val(&self.incoming_message_types[i])
                    );
                }
            }

            *self
                .pb
                .borrow_mut()
                .val_ref(&self.membership_check_results[0]) =
                if self.pb.borrow().val(&self.common_type).is_zero() {
                    ppT::FieldT::zero()
                } else {
                    ppT::FieldT::one()
                };
            self.membership_proofs[0]
                .generate_r1cs_witness(&vk_membership_proofs[nonzero_type_idx]);
            self.membership_checkers[0].generate_r1cs_witness();

            for (i, &it) in self
                .compliance_predicate
                .accepted_input_types
                .iter()
                .enumerate()
            {
                *self.pb.borrow_mut().val_ref(&self.common_type_check_aux[i]) =
                    ((if i == 0 {
                        self.pb.borrow().val(&self.common_type)
                    } else {
                        self.pb.borrow().val(&self.common_type_check_aux[i - 1])
                    }) * (self.pb.borrow().val(&self.common_type) - ppT::FieldT::from(it)));
            }
        } else {
            for i in 0..self.membership_checkers.len() {
                *self
                    .pb
                    .borrow_mut()
                    .val_ref(&self.membership_check_results[i]) = (if self
                    .pb
                    .borrow()
                    .val(&self.incoming_message_types[i])
                    .is_zero()
                {
                    ppT::FieldT::zero()
                } else {
                    ppT::FieldT::one()
                });
                self.membership_proofs[i].generate_r1cs_witness(&vk_membership_proofs[i]);
                self.membership_checkers[i].generate_r1cs_witness();
            }
        }

        self.hash_outgoing_message.borrow().generate_r1cs_witness();
        for i in 0..self.compliance_predicate.max_arity {
            self.hash_incoming_messages[i].generate_r1cs_witness();
            self.unpack_commitment_and_incoming_message_digests[i]
                .generate_r1cs_witness_from_packed();
        }

        for i in 0..self.compliance_predicate.max_arity {
            self.proof[i].generate_r1cs_witness(&translation_step_proofs[i]);
            self.verifier[i].generate_r1cs_witness();
        }

        // #ifdef DEBUG
        self.get_circuit(); // force generating constraints
        assert!(self.pb.borrow().is_satisfied());
    }

    pub fn field_logsize() -> usize {
        ppT::Fr::size_in_bits()
    }

    pub fn field_capacity() -> usize {
        ppT::Fr::capacity()
    }

    pub fn input_size_in_elts() -> usize {
        let digest_size = CRH_with_field_out_gadgets::<ppT::FieldT, ppT::PB>::get_digest_len();
        digest_size
    }

    pub fn input_capacity_in_bits() -> usize {
        Self::input_size_in_elts() * Self::field_capacity()
    }

    pub fn input_size_in_bits() -> usize {
        Self::input_size_in_elts() * Self::field_logsize()
    }
}

impl<ppT: ppTConfig> mp_translation_step_pcd_circuit_maker<ppT> {
    pub fn new(compliance_step_vk: r1cs_ppzksnark_verification_key<other_curve<ppT>>) -> Self {
        let pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
        /* allocate input of the translation MP_PCD circuit */
        let mut mp_translation_step_pcd_circuit_input =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        mp_translation_step_pcd_circuit_input.allocate(
            &pb,
            Self::input_size_in_elts(),
            "mp_translation_step_pcd_circuit_input",
        );

        /* unpack translation step MP_PCD circuit input */
        let mut unpacked_mp_translation_step_pcd_circuit_input =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        unpacked_mp_translation_step_pcd_circuit_input.allocate(
            &pb,
            mp_compliance_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_bits(),
            "unpacked_mp_translation_step_pcd_circuit_input",
        );
        let unpack_mp_translation_step_pcd_circuit_input =
            RcCell::new(multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                unpacked_mp_translation_step_pcd_circuit_input
                    .clone()
                    .into(),
                mp_translation_step_pcd_circuit_input.clone().into(),
                Self::field_capacity(),
                "unpack_mp_translation_step_pcd_circuit_input".to_owned(),
            ));

        /* prepare arguments for the verifier */
        let hardcoded_compliance_step_vk = RcCell::new(
            r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable::<ppT>::new(
                pb.clone(),
                compliance_step_vk.clone(),
                "hardcoded_compliance_step_vk".to_owned(),
            ),
        );
        let proof = RcCell::new(r1cs_ppzksnark_proof_variable::<ppT>::new(
            pb.clone(),
            "proof".to_owned(),
        ));

        /* verify previous proof */
        let online_verifier = RcCell::new(r1cs_ppzksnark_online_verifier_gadget::<ppT>::new(
            pb.clone(),
            hardcoded_compliance_step_vk.borrow().clone(),
            unpacked_mp_translation_step_pcd_circuit_input.clone(),
            mp_compliance_step_pcd_circuit_maker::<other_curve<ppT>>::field_logsize(),
            proof.borrow().clone(),
            variable::<ppT::FieldT, pb_variable>::from(ONE), // must always accept
            "verifier".to_owned(),
        ));

        pb.borrow_mut().set_input_sizes(Self::input_size_in_elts());
        Self {
            pb,
            mp_translation_step_pcd_circuit_input,
            unpacked_mp_translation_step_pcd_circuit_input,
            verifier_input:
                pb_variable_array::<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB>::default(),
            unpack_mp_translation_step_pcd_circuit_input,
            hardcoded_compliance_step_vk,
            proof,
            online_verifier,
        }
    }

    pub fn generate_r1cs_constraints(&self) {
        PROFILE_CONSTRAINTS(&self.pb, "repacking: unpack circuit input");
        {
            self.unpack_mp_translation_step_pcd_circuit_input
                .borrow()
                .generate_r1cs_constraints(true);
        }

        PROFILE_CONSTRAINTS(&self.pb, "verifier for compliance proofs");
        {
            PROFILE_CONSTRAINTS(&self.pb, "check that proof lies on the curve");
            {
                self.proof.borrow().generate_r1cs_constraints();
            }

            self.online_verifier.borrow().generate_r1cs_constraints();
        }

        PRINT_CONSTRAINT_PROFILING();
        print_indent();
        print!(
            "* Number of constraints in mp_translation_step_pcd_circuit: {}\n",
            self.pb.borrow().num_constraints()
        );
    }

    pub fn get_circuit(
        &self,
    ) -> r1cs_constraint_system<Fr<ppT>, pb_variable, pb_linear_combination> {
        self.pb.borrow().get_constraint_system()
    }

    pub fn generate_r1cs_witness(
        &self,
        translation_step_input: &r1cs_primary_input<Fr<ppT>>,
        prev_proof: &r1cs_ppzksnark_proof<other_curve<ppT>>,
    ) {
        self.pb.borrow_mut().clear_values();
        self.mp_translation_step_pcd_circuit_input
            .fill_with_field_elements(&self.pb, &translation_step_input);
        self.unpack_mp_translation_step_pcd_circuit_input
            .borrow()
            .generate_r1cs_witness_from_packed();

        self.proof.borrow().generate_r1cs_witness(prev_proof);
        self.online_verifier.borrow().generate_r1cs_witness();

        // #ifdef DEBUG
        self.get_circuit(); // force generating constraints
        assert!(self.pb.borrow().is_satisfied());
    }

    pub fn get_primary_input(&self) -> r1cs_primary_input<Fr<ppT>> {
        self.pb.borrow().primary_input()
    }

    pub fn get_auxiliary_input(&self) -> r1cs_auxiliary_input<Fr<ppT>> {
        self.pb.borrow().auxiliary_input()
    }

    pub fn field_logsize() -> usize {
        ppT::Fr::size_in_bits()
    }

    pub fn field_capacity() -> usize {
        ppT::Fr::capacity()
    }

    pub fn input_size_in_elts() -> usize {
        div_ceil(
            mp_compliance_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_bits(),
            Self::field_capacity(),
        )
        .unwrap()
    }

    pub fn input_capacity_in_bits() -> usize {
        Self::input_size_in_elts() * Self::field_capacity()
    }

    pub fn input_size_in_bits() -> usize {
        Self::input_size_in_elts() * Self::field_logsize()
    }
}

pub fn get_mp_compliance_step_pcd_circuit_input<ppT: ppTConfig>(
    commitment_to_translation_step_r1cs_vks: &set_commitment,
    primary_input: &r1cs_pcd_compliance_predicate_primary_input<Fr<ppT>, ppT::M>,
) -> r1cs_primary_input<Fr<ppT>> {
    leave_block("Call to get_mp_compliance_step_pcd_circuit_input", false);
    //type ppT::FieldT = Fr<ppT>;

    let outgoing_message_as_va = primary_input
        .outgoing_message
        .borrow()
        .as_r1cs_variable_assignment();
    let mut msg_bits = vec![];
    for elt in &outgoing_message_as_va {
        let elt_bits = convert_field_element_to_bit_vector(elt);
        msg_bits.extend(elt_bits.clone());
    }

    let mut block = vec![];
    block.extend(commitment_to_translation_step_r1cs_vks.clone());
    block.extend(msg_bits.clone());

    leave_block("Sample CRH randomness", false);
    CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::sample_randomness(block.len());
    leave_block("Sample CRH randomness", false);

    let digest = CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::get_hash_for_field(&block);
    leave_block("Call to get_mp_compliance_step_pcd_circuit_input", false);

    digest
}

pub fn get_mp_translation_step_pcd_circuit_input<ppT: ppTConfig>(
    commitment_to_translation_step_r1cs_vks: &set_commitment,
    primary_input: &r1cs_pcd_compliance_predicate_primary_input<
        Fr<other_curve<ppT>>,
        <<<ppT as ppTConfig>::P as pairing_selector>::other_curve_type as ppTConfig>::M,
    >,
) -> r1cs_primary_input<Fr<ppT>> {
    leave_block("Call to get_mp_translation_step_pcd_circuit_input", false);
    // type ppT::FieldT = Fr<ppT>;

    let mp_compliance_step_pcd_circuit_input =
        get_mp_compliance_step_pcd_circuit_input::<other_curve<ppT>>(
            commitment_to_translation_step_r1cs_vks,
            primary_input,
        );
    let mut mp_compliance_step_pcd_circuit_input_bits = vec![];
    for elt in &mp_compliance_step_pcd_circuit_input {
        let elt_bits = convert_field_element_to_bit_vector::<Fr<other_curve<ppT>>>(elt);
        mp_compliance_step_pcd_circuit_input_bits.extend(elt_bits.clone());
    }

    mp_compliance_step_pcd_circuit_input_bits.resize(
        mp_compliance_step_pcd_circuit_maker::<ppT>::input_capacity_in_bits(),
        false,
    );

    let result = pack_bit_vector_into_field_element_vector1::<ppT::FieldT>(
        &mp_compliance_step_pcd_circuit_input_bits,
        mp_compliance_step_pcd_circuit_maker::<ppT>::field_capacity(),
    );
    leave_block("Call to get_mp_translation_step_pcd_circuit_input", false);

    result
}
