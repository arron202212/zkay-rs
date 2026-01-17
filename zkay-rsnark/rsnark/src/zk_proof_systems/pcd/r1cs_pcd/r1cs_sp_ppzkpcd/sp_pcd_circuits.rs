// Declaration of functionality for creating and using the two PCD circuits in
// a single-predicate PCD construction.

// The implementation follows, extends, and optimizes the approach described
// in \[BCTV14]. At high level, there is a "compliance step" circuit and a
// "translation step" circuit. For more details see Section 4 of \[BCTV14].

// References:

// \[BCTV14]:
// "Scalable Zero Knowledge via Cycles of Elliptic Curves",
// Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
// CRYPTO 2014,
// <http://eprint.iacr.org/2014/595>

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
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
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
    convert_field_element_to_bit_vector, pack_bit_vector_into_field_element_vector,
};
use ffec::{One, Zero};
use rccell::RcCell;
use std::ops::Mul;

/**************************** Compliance step ********************************/

/**
 * A compliance-step PCD circuit.
 *
 * The circuit is an R1CS that checks compliance (for the given compliance predicate)
 * and validity of previous proofs.
 */
//
type FieldT<ppT> = Fr<ppT>;

pub struct sp_compliance_step_pcd_circuit_maker<ppT: ppTConfig> {
    pub compliance_predicate: r1cs_pcd_compliance_predicate<ppT::FieldT, ppT>,
    pub pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
    pub zero: variable<ppT::FieldT, pb_variable>,
    pub block_for_outgoing_message: RcCell<block_variables<ppT::FieldT, ppT::PB>>,
    pub hash_outgoing_message: RcCell<CRH_with_field_out_gadgets<ppT::FieldT, ppT::PB>>,
    pub blocks_for_incoming_messages: Vec<block_variables<ppT::FieldT, ppT::PB>>,
    pub sp_translation_step_vk_and_incoming_message_payload_digests:
        Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub unpack_sp_translation_step_vk_and_incoming_message_payload_digests:
        Vec<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    pub sp_translation_step_vk_and_incoming_message_payload_digest_bits:
        Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub hash_incoming_messages: Vec<CRH_with_field_out_gadgets<ppT::FieldT, ppT::PB>>,
    pub sp_translation_step_vk: RcCell<r1cs_ppzksnark_verification_key_variables<ppT>>,
    pub sp_translation_step_vk_bits: pb_variable_array<ppT::FieldT, ppT::PB>,
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
    pub sp_compliance_step_pcd_circuit_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub padded_translation_step_vk_and_outgoing_message_digest:
        pb_variable_array<ppT::FieldT, ppT::PB>,
    pub padded_translation_step_vk_and_incoming_messages_digests:
        Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub verifier_input: Vec<pb_variable_array<ppT::FieldT, ppT::PB>>,
    pub proof: Vec<r1cs_ppzksnark_proof_variables<ppT>>,
    pub verification_result: variable<ppT::FieldT, pb_variable>,
    pub verifiers: Vec<r1cs_ppzksnark_verifier_gadgets<ppT>>,
}

/*************************** Translation step ********************************/

/**
 * A translation-step PCD circuit.
 *
 * The circuit is an R1CS that checks validity of previous proofs.
 */
// type FieldT<ppT>=Fr<ppT>;
pub struct sp_translation_step_pcd_circuit_maker<ppT: ppTConfig> {
    pub pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
    pub sp_translation_step_pcd_circuit_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub unpacked_sp_translation_step_pcd_circuit_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub verifier_input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub unpack_sp_translation_step_pcd_circuit_input:
        RcCell<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    pub hardcoded_sp_compliance_step_vk:
        RcCell<r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>>,
    pub proof: RcCell<r1cs_ppzksnark_proof_variables<ppT>>,
    pub online_verifier: RcCell<r1cs_ppzksnark_online_verifier_gadgets<ppT>>,
}

/****************************** Input maps ***********************************/

// /**
//  * Obtain the primary input for a compliance-step PCD circuit.
//  */
//
// r1cs_primary_input<Fr<ppT> > get_sp_compliance_step_pcd_circuit_input(sp_translation_step_vk_bits:&bit_vector,
//                                                                       primary_input:&r1cs_pcd_compliance_predicate_primary_input<Fr<ppT> >);

// /**
//  * Obtain the primary input for a translation-step PCD circuit.
//  */
//
// r1cs_primary_input<Fr<ppT> > get_sp_translation_step_pcd_circuit_input(sp_translation_step_vk_bits:&bit_vector,
//                                                                        primary_input:&r1cs_pcd_compliance_predicate_primary_input<Fr<other_curve<ppT> > >);

// use common::utils;

// use crate::gadgetlib1::constraint_profiling;

impl<ppT: ppTConfig> sp_compliance_step_pcd_circuit_maker<ppT> {
    pub fn new(compliance_predicate: r1cs_pcd_compliance_predicate<ppT::FieldT, ppT>) -> Self {
        /* calculate some useful sizes */
        let pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
        assert!(compliance_predicate.is_well_formed());
        assert!(compliance_predicate.has_equal_input_and_output_lengths());

        let compliance_predicate_arity = compliance_predicate.max_arity;
        let digest_size = CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::get_digest_len();
        let msg_size_in_bits =
            Self::field_logsize() * (1 + compliance_predicate.outgoing_message_payload_length);
        let sp_translation_step_vk_size_in_bits =
            r1cs_ppzksnark_verification_key_variable::<ppT>::size_in_bits(
                sp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_elts(),
            );
        let padded_verifier_input_size =
            sp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_capacity_in_bits();

        print!(
            "other curve input size = {}\n",
            sp_translation_step_pcd_circuit_maker::<other_curve::<ppT>>::input_size_in_elts()
        );
        print!(
            "translation_vk_bits = {}\n",
            sp_translation_step_vk_size_in_bits
        );
        print!(
            "padded verifier input size = {}\n",
            padded_verifier_input_size
        );

        let block_size = msg_size_in_bits + sp_translation_step_vk_size_in_bits;
        CRH_with_bit_out_gadget::<ppT::FieldT, ppT::PB>::sample_randomness(block_size);

        /* allocate input of the compliance PCD circuit */
        let mut sp_compliance_step_pcd_circuit_input =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        sp_compliance_step_pcd_circuit_input.allocate(
            &pb,
            Self::input_size_in_elts(),
            "sp_compliance_step_pcd_circuit_input",
        );

        /* allocate inputs to the compliance predicate */
        let mut outgoing_message_type = variable::<ppT::FieldT, pb_variable>::default();
        outgoing_message_type.allocate(&pb, "outgoing_message_type".to_owned());
        let mut outgoing_message_payload = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
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
        arity.allocate(&pb, "arity".to_owned());

        let mut incoming_message_types =
            vec![variable::<ppT::FieldT, pb_variable>::default(); compliance_predicate_arity];
        let mut incoming_message_payloads =
            vec![pb_variable_array::<ppT::FieldT, ppT::PB>::default(); compliance_predicate_arity];
        let mut incoming_message_vars =
            vec![pb_variable_array::<ppT::FieldT, ppT::PB>::default(); compliance_predicate_arity];
        for i in 0..compliance_predicate_arity {
            incoming_message_types[i]
                .allocate(&pb, prefix_format!("", "incoming_message_type_{}", i));
            incoming_message_payloads[i].allocate(
                &pb,
                compliance_predicate.outgoing_message_payload_length,
                &prefix_format!("", "incoming_message_payloads_{}", i),
            );

            incoming_message_vars[i]
                .contents
                .push(incoming_message_types[i].clone());
            incoming_message_vars[i]
                .contents
                .extend(incoming_message_payloads[i].clone());
        }

        let mut local_data = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        let mut cp_witness = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        local_data.allocate(&pb, compliance_predicate.local_data_length, "local_data");
        cp_witness.allocate(&pb, compliance_predicate.witness_length, "cp_witness");

        /* convert compliance predicate from a constraint system into a gadget */
        let mut incoming_messages_concat = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        for i in 0..compliance_predicate_arity {
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
                    incoming_messages_concat.clone(),
                    local_data.clone(),
                    cp_witness.clone(),
                ],
                compliance_predicate.constraint_system.clone(),
                "compliance_predicate_as_gadget".to_owned(),
            ));

        /* unpack messages to bits */
        let mut outgoing_message_bits = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        outgoing_message_bits.allocate(&pb, msg_size_in_bits, "outgoing_message_bits");
        let unpack_outgoing_message =
            RcCell::new(multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                outgoing_message_bits.clone().into(),
                outgoing_message_vars.clone().into(),
                Self::field_logsize(),
                "unpack_outgoing_message".to_owned(),
            ));

        let mut incoming_messages_bits =
            vec![pb_variable_array::<ppT::FieldT, ppT::PB>::default(); compliance_predicate_arity];
        let mut unpack_incoming_messages = vec![];
        for i in 0..compliance_predicate_arity {
            incoming_messages_bits[i].allocate(
                &pb,
                msg_size_in_bits,
                &prefix_format!("", "incoming_messages_bits_{}", i),
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
        let mut sp_translation_step_vk_and_incoming_message_payload_digests =
            vec![pb_variable_array::<ppT::FieldT, ppT::PB>::default(); compliance_predicate_arity];
        for i in 0..compliance_predicate_arity {
            sp_translation_step_vk_and_incoming_message_payload_digests[i].allocate(
                &pb,
                digest_size,
                &prefix_format!(
                    "",
                    "sp_translation_step_vk_and_incoming_message_payload_digests_{}",
                    i,
                ),
            );
        }

        /* allocate blocks */
        let mut sp_translation_step_vk_bits = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        sp_translation_step_vk_bits.allocate(
            &pb,
            sp_translation_step_vk_size_in_bits,
            "sp_translation_step_vk_bits",
        );

        let block_for_outgoing_message = RcCell::new(block_variable::<ppT::FieldT, ppT::PB>::new2(
            pb.clone(),
            vec![
                sp_translation_step_vk_bits.clone(),
                outgoing_message_bits.clone(),
            ],
            "block_for_outgoing_message".to_owned(),
        ));
        let mut blocks_for_incoming_messages = vec![];
        for i in 0..compliance_predicate_arity {
            blocks_for_incoming_messages.push(block_variable::<ppT::FieldT, ppT::PB>::new2(
                pb.clone(),
                vec![
                    sp_translation_step_vk_bits.clone(),
                    incoming_messages_bits[i].clone(),
                ],
                prefix_format!("", "blocks_for_incoming_messages_zu{}", i).to_owned(),
            ));
        }

        /* allocate hash checkers */
        let hash_outgoing_message =
            RcCell::new(CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                block_size,
                block_for_outgoing_message.borrow().clone(),
                sp_compliance_step_pcd_circuit_input.clone().into(),
                "hash_outgoing_message".to_owned(),
            ));
        let mut hash_incoming_messages = vec![];
        for i in 0..compliance_predicate_arity {
            hash_incoming_messages.push(CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                block_size,
                blocks_for_incoming_messages[i].clone(),
                sp_translation_step_vk_and_incoming_message_payload_digests[i]
                    .clone()
                    .into(),
                prefix_format!("", "hash_incoming_messages_{}", i),
            ));
        }

        /* allocate useful zero variable */
        let mut zero = variable::<ppT::FieldT, pb_variable>::default();
        zero.allocate(&pb, "zero".to_owned());

        /* prepare arguments for the verifier */
        let sp_translation_step_vk =
            RcCell::new(r1cs_ppzksnark_verification_key_variable::<ppT>::new(
                pb.clone(),
                sp_translation_step_vk_bits.clone(),
                sp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_elts(),
                "sp_translation_step_vk".to_owned(),
            ));
        let mut verification_result = variable::<ppT::FieldT, pb_variable>::default();
        verification_result.allocate(&pb, "verification_result".to_owned());
        let mut sp_translation_step_vk_and_incoming_message_payload_digest_bits =
            vec![pb_variable_array::<ppT::FieldT, ppT::PB>::default(); compliance_predicate_arity];
        let mut unpack_sp_translation_step_vk_and_incoming_message_payload_digests = vec![];
        let mut verifier_input = vec![];
        let mut proof = vec![];
        let mut verifiers = vec![];
        for i in 0..compliance_predicate_arity {
            sp_translation_step_vk_and_incoming_message_payload_digest_bits[i].allocate(
                &pb,
                digest_size * Self::field_logsize(),
                &prefix_format!(
                    "",
                    "sp_translation_step_vk_and_incoming_message_payload_digest_bits_{}",
                    i,
                ),
            );
            unpack_sp_translation_step_vk_and_incoming_message_payload_digests.push(
                multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
                    pb.clone(),
                    sp_translation_step_vk_and_incoming_message_payload_digest_bits[i]
                        .clone()
                        .into(),
                    sp_translation_step_vk_and_incoming_message_payload_digests[i]
                        .clone()
                        .into(),
                    Self::field_logsize(),
                    prefix_format!(
                        "",
                        "unpack_sp_translation_step_vk_and_incoming_message_payload_digests_{}",
                        i,
                    ),
                ),
            );

            verifier_input
                .push(sp_translation_step_vk_and_incoming_message_payload_digest_bits[i].clone());
            if verifier_input[i].len() < padded_verifier_input_size {
                verifier_input[i]
                    .contents
                    .resize(padded_verifier_input_size, zero.clone());
            }

            proof.push(r1cs_ppzksnark_proof_variable::<ppT>::new(
                pb.clone(),
                prefix_format!("", "proof_{}", i),
            ));
            verifiers.push(r1cs_ppzksnark_verifier_gadget::<ppT>::new(
                pb.clone(),
                sp_translation_step_vk.borrow().clone(),
                verifier_input[i].clone(),
                sp_translation_step_pcd_circuit_maker::<other_curve<ppT>>::field_capacity(),
                proof[i].clone(),
                verification_result.clone(),
                prefix_format!("", "verifiers_{}", i),
            ));
        }

        pb.borrow_mut().set_input_sizes(Self::input_size_in_elts());
        let padded_translation_step_vk_and_outgoing_message_digest =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        let padded_translation_step_vk_and_incoming_messages_digests = vec![];
        print!("done compliance\n");
        Self {
            compliance_predicate,
            pb,
            zero,
            block_for_outgoing_message,
            hash_outgoing_message,
            blocks_for_incoming_messages,
            sp_translation_step_vk_and_incoming_message_payload_digests,
            unpack_sp_translation_step_vk_and_incoming_message_payload_digests,
            sp_translation_step_vk_and_incoming_message_payload_digest_bits,
            hash_incoming_messages,
            sp_translation_step_vk,
            sp_translation_step_vk_bits,
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
            sp_compliance_step_pcd_circuit_input,
            padded_translation_step_vk_and_outgoing_message_digest,
            padded_translation_step_vk_and_incoming_messages_digests,
            verifier_input,
            proof,
            verification_result,
            verifiers,
        }
    }

    pub fn generate_r1cs_constraints(&self) {
        let digest_size = CRH_with_bit_out_gadget::<ppT::FieldT, ppT::PB>::get_digest_len();
        let dimension = knapsack_dimension::<FieldT<ppT>>::dimension;
        print_indent();
        print!("* Knapsack dimension: {}\n", dimension);

        let compliance_predicate_arity = self.compliance_predicate.max_arity;
        print_indent();
        print!(
            "* Compliance predicate arity: {}\n",
            compliance_predicate_arity
        );
        print_indent();
        print!(
            "* Compliance predicate payload length: {}\n",
            self.compliance_predicate.outgoing_message_payload_length
        );
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

            PROFILE_CONSTRAINTS(&self.pb, "booleanity: unpack s incoming_message");
            {
                for i in 0..compliance_predicate_arity {
                    self.unpack_incoming_messages[i].generate_r1cs_constraints(true);
                }
            }

            PROFILE_CONSTRAINTS(&self.pb, "booleanity: unpack verification key");
            {
                self.sp_translation_step_vk
                    .borrow()
                    .generate_r1cs_constraints(true);
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "(1+s) copies of hash");
        {
            print_indent();
            print!("* Digest-size: {}\n", digest_size);
            self.hash_outgoing_message
                .borrow()
                .generate_r1cs_constraints();

            for i in 0..compliance_predicate_arity {
                self.hash_incoming_messages[i].generate_r1cs_constraints();
            }
        }

        PROFILE_CONSTRAINTS(&self.pb, "s copies of repacking circuit");
        {
            for i in 0..compliance_predicate_arity {
                self.unpack_sp_translation_step_vk_and_incoming_message_payload_digests[i]
                    .generate_r1cs_constraints(true);
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
                for i in 0..compliance_predicate_arity {
                    self.proof[i].generate_r1cs_constraints();
                }
            }

            for i in 0..compliance_predicate_arity {
                self.verifiers[i].generate_r1cs_constraints();
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
            generate_boolean_r1cs_constraint::<ppT::FieldT, ppT::PB>(
                &self.pb,
                &(self.verification_result.clone().into()),
                "verification_result".to_owned(),
            );

            /* type * (1-verification_result) = 0 */
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                    self.incoming_message_types[0].clone().into(),
                    (linear_combination::<ppT::FieldT, pb_variable, pb_linear_combination>::from(
                        ppT::FieldT::from(1),
                    ) - self.verification_result.clone())
                    .into(),
                    ppT::FieldT::from(0).into(),
                ),
                "not_base_case_implies_valid_proofs".to_owned(),
            );

            /* all types equal */
            for i in 1..self.compliance_predicate.max_arity {
                self.pb.borrow_mut().add_r1cs_constraint(
                    r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                        ppT::FieldT::from(1).into(),
                        self.incoming_message_types[0].clone().into(),
                        self.incoming_message_types[i].clone().into(),
                    ),
                    prefix_format!("", "type_{}_equal_to_type_0", i),
                );
            }

            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<ppT::FieldT, pb_variable, pb_linear_combination>::new(
                    ppT::FieldT::from(1).into(),
                    self.arity.clone().into(),
                    ppT::FieldT::from(compliance_predicate_arity).into(),
                ),
                "full_arity".to_owned(),
            );
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
            "* Number of constraints in sp_compliance_step_pcd_circuit: {}\n",
            self.pb.borrow().num_constraints()
        );
    }

    pub fn get_circuit(
        &self,
    ) -> r1cs_constraint_system<ppT::FieldT, pb_variable, pb_linear_combination> {
        self.pb.borrow().get_constraint_system()
    }

    pub fn get_primary_input(&self) -> r1cs_primary_input<ppT::FieldT> {
        self.pb.borrow().primary_input()
    }

    pub fn get_auxiliary_input(&self) -> r1cs_auxiliary_input<ppT::FieldT> {
        self.pb.borrow().auxiliary_input()
    }

    pub fn generate_r1cs_witness(
        &self,
        sp_translation_step_pcd_circuit_vk: &r1cs_ppzksnark_verification_key<other_curve<ppT>>,
        compliance_predicate_primary_input: &r1cs_pcd_compliance_predicate_primary_input<
            ppT::FieldT,
            ppT::M,
        >,
        compliance_predicate_auxiliary_input: &r1cs_pcd_compliance_predicate_auxiliary_input<
            ppT::FieldT,
            ppT::M,
            ppT::LD,
        >,
        incoming_proofs: &Vec<r1cs_ppzksnark_proof<other_curve<ppT>>>,
    )
    // where
    //     <P as pairing_selector<ppT>>::other_curve_type: ppTConfig + PublicParams,
    //     LD: LocalDataConfig<<P as pairing_selector<ppT>>::FieldT>,
    //     M: MessageConfig<<P as pairing_selector<ppT>>::FieldT>,
    {
        let compliance_predicate_arity = self.compliance_predicate.max_arity;
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
        *self.pb.borrow_mut().val_ref(&self.arity) = ppT::FieldT::from(compliance_predicate_arity);
        self.unpack_outgoing_message
            .borrow()
            .generate_r1cs_witness_from_packed();
        for i in 0..compliance_predicate_arity {
            self.unpack_incoming_messages[i].generate_r1cs_witness_from_packed();
        }

        self.sp_translation_step_vk
            .borrow()
            .generate_r1cs_witness(sp_translation_step_pcd_circuit_vk);
        self.hash_outgoing_message.borrow().generate_r1cs_witness();
        for i in 0..compliance_predicate_arity {
            self.hash_incoming_messages[i].generate_r1cs_witness();
            self.unpack_sp_translation_step_vk_and_incoming_message_payload_digests[i]
                .generate_r1cs_witness_from_packed();
        }

        for i in 0..compliance_predicate_arity {
            self.proof[i].generate_r1cs_witness(&incoming_proofs[i]);
            self.verifiers[i].generate_r1cs_witness();
        }

        if self.pb.borrow().val(&self.incoming_message_types[0]) != ppT::FieldT::zero() {
            *self.pb.borrow_mut().val_ref(&self.verification_result) = ppT::FieldT::one();
        }

        // #ifdef DEBUG
        self.generate_r1cs_constraints(); // force generating constraints
        assert!(self.pb.borrow().is_satisfied());
        //#endif
    }

    pub fn field_logsize() -> usize {
        Fr::<ppT>::size_in_bits()
    }

    pub fn field_capacity() -> usize {
        Fr::<ppT>::capacity()
    }

    pub fn input_size_in_elts() -> usize {
        let digest_size = CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::get_digest_len();
        digest_size
    }

    pub fn input_capacity_in_bits() -> usize {
        Self::input_size_in_elts() * Self::field_capacity()
    }

    pub fn input_size_in_bits() -> usize {
        Self::input_size_in_elts() * Self::field_logsize()
    }
}

impl<ppT: ppTConfig> sp_translation_step_pcd_circuit_maker<ppT> {
    pub fn new(sp_compliance_step_vk: r1cs_ppzksnark_verification_key<other_curve<ppT>>) -> Self
// where <<P as pairing_selector<ppT>>::other_curve_type as ff_curves::PublicParams>::Fr: Mul<<ppT as ff_curves::PublicParams>::Fr,Output=<<P as pairing_selector<ppT>>::other_curve_type as ff_curves::PublicParams>::Fr>,
    {
        /* allocate input of the translation PCD circuit */
        let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
        let mut sp_translation_step_pcd_circuit_input =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        sp_translation_step_pcd_circuit_input.allocate(
            &pb,
            Self::input_size_in_elts(),
            "sp_translation_step_pcd_circuit_input",
        );

        /* unpack translation step PCD circuit input */
        let mut unpacked_sp_translation_step_pcd_circuit_input =
            pb_variable_array::<ppT::FieldT, ppT::PB>::default();

        unpacked_sp_translation_step_pcd_circuit_input.allocate(
            &pb,
            sp_compliance_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_bits(),
            "unpacked_sp_translation_step_pcd_circuit_input",
        );
        let unpack_sp_translation_step_pcd_circuit_input =
            RcCell::new(multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
                pb.clone(),
                unpacked_sp_translation_step_pcd_circuit_input
                    .clone()
                    .into(),
                sp_translation_step_pcd_circuit_input.clone().into(),
                Self::field_capacity(),
                "unpack_sp_translation_step_pcd_circuit_input".to_owned(),
            ));

        /* prepare arguments for the verifier */
        let hardcoded_sp_compliance_step_vk = RcCell::new(
            r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable::<ppT>::new(
                pb.clone(),
                sp_compliance_step_vk,
                "hardcoded_sp_compliance_step_vk".to_owned(),
            ),
        );
        let proof = RcCell::new(r1cs_ppzksnark_proof_variable::<ppT>::new(
            pb.clone(),
            "proof".to_owned(),
        ));

        /* verify previous proof */
        let online_verifier = RcCell::new(r1cs_ppzksnark_online_verifier_gadget::<ppT>::new(
            pb.clone(),
            hardcoded_sp_compliance_step_vk.borrow().clone(),
            unpacked_sp_translation_step_pcd_circuit_input.clone(),
            sp_compliance_step_pcd_circuit_maker::<other_curve<ppT>>::field_logsize(),
            proof.borrow().clone(),
            variable::<ppT::FieldT, pb_variable>::from(ONE), // must always accept
            "verifier".to_owned(),
        ));
        pb.borrow_mut().set_input_sizes(Self::input_size_in_elts());
        let verifier_input = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        print!("done translation\n");

        Self {
            pb,
            sp_translation_step_pcd_circuit_input,
            unpacked_sp_translation_step_pcd_circuit_input,
            verifier_input,
            unpack_sp_translation_step_pcd_circuit_input,
            hardcoded_sp_compliance_step_vk,
            proof,
            online_verifier,
        }
    }

    pub fn generate_r1cs_constraints(&self) {
        PROFILE_CONSTRAINTS(&self.pb, "repacking: unpack circuit input");
        {
            self.unpack_sp_translation_step_pcd_circuit_input
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
            "* Number of constraints in sp_translation_step_pcd_circuit: {}\n",
            self.pb.borrow().num_constraints()
        );
    }

    pub fn get_circuit(
        &self,
    ) -> r1cs_constraint_system<ppT::FieldT, pb_variable, pb_linear_combination> {
        self.pb.borrow().get_constraint_system()
    }

    pub fn generate_r1cs_witness(
        &self,
        sp_translation_step_input: &r1cs_primary_input<ppT::FieldT>,
        compliance_step_proof: &r1cs_ppzksnark_proof<other_curve<ppT>>,
    )
    //  where
    //     <P as pairing_selector<ppT>>::other_curve_type: ppTConfig,
    {
        self.pb.borrow_mut().clear_values();
        self.sp_translation_step_pcd_circuit_input
            .fill_with_field_elements(&self.pb, sp_translation_step_input);
        self.unpack_sp_translation_step_pcd_circuit_input
            .borrow()
            .generate_r1cs_witness_from_packed();

        self.proof
            .borrow()
            .generate_r1cs_witness(compliance_step_proof);
        self.online_verifier.borrow().generate_r1cs_witness();

        // #ifdef DEBUG
        self.generate_r1cs_constraints(); // force generating constraints

        print!("Input to the translation circuit:\n");
        for i in 0..self.pb.borrow().num_inputs() {
            self.pb
                .borrow()
                .val(&variable::<ppT::FieldT, pb_variable>::from(i + 1))
                .print();
        }

        assert!(self.pb.borrow().is_satisfied());
        //#endif
    }

    pub fn get_primary_input(&self) -> r1cs_primary_input<ppT::FieldT> {
        self.pb.borrow().primary_input()
    }

    pub fn get_auxiliary_input(&self) -> r1cs_auxiliary_input<ppT::FieldT> {
        self.pb.borrow().auxiliary_input()
    }

    pub fn field_logsize() -> usize {
        Fr::<ppT>::size_in_bits()
    }

    pub fn field_capacity() -> usize {
        Fr::<ppT>::capacity()
    }

    pub fn input_size_in_elts() -> usize {
        div_ceil(
            sp_compliance_step_pcd_circuit_maker::<other_curve<ppT>>::input_size_in_bits(),
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

pub fn get_sp_compliance_step_pcd_circuit_input<ppT: ppTConfig>(
    sp_translation_step_vk_bits: &bit_vector,
    primary_input: &r1cs_pcd_compliance_predicate_primary_input<ppT::FieldT, ppT::M>,
) -> r1cs_primary_input<ppT::FieldT> {
    enter_block("Call to get_sp_compliance_step_pcd_circuit_input", false);
    // type FieldT<ppT>=ppT::FieldT;

    let outgoing_message_as_va = primary_input
        .outgoing_message
        .borrow()
        .as_r1cs_variable_assignment();
    let mut msg_bits: Vec<_> = vec![];
    for elt in &outgoing_message_as_va {
        let elt_bits = convert_field_element_to_bit_vector(elt);
        msg_bits.extend(elt_bits);
    }

    let mut block = vec![];
    block.extend(sp_translation_step_vk_bits);
    block.extend(msg_bits);

    enter_block("Sample CRH randomness", false);
    CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::sample_randomness(block.len());
    leave_block("Sample CRH randomness", false);

    let digest = CRH_with_field_out_gadget::<ppT::FieldT, ppT::PB>::get_hash(block);
    leave_block("Call to get_sp_compliance_step_pcd_circuit_input", false);

    digest
}

pub fn get_sp_translation_step_pcd_circuit_input<ppT: ppTConfig>(
    sp_translation_step_vk_bits: &bit_vector,
    primary_input: &r1cs_pcd_compliance_predicate_primary_input<
        Fr<other_curve<ppT>>,
        <<<ppT as ppTConfig>::P as pairing_selector>::other_curve_type as ppTConfig>::M,
    >,
) -> r1cs_primary_input<Fr<ppT>>
// where
//     <P as pairing_selector<ppT>>::other_curve_type: ff_curves::PublicParams,
//     M: MessageConfig<
//         <<P as pairing_selector<ppT>>::other_curve_type as ff_curves::PublicParams>::Fr,
//     >,
{
    enter_block("Call to get_sp_translation_step_pcd_circuit_input", false);
    // type FieldT<ppT>=ppT::FieldT;

    let sp_compliance_step_pcd_circuit_input = get_sp_compliance_step_pcd_circuit_input::<
        other_curve<ppT>,
    >(sp_translation_step_vk_bits, primary_input);
    let mut sp_compliance_step_pcd_circuit_input_bits = vec![];
    for elt in &sp_compliance_step_pcd_circuit_input {
        let elt_bits = convert_field_element_to_bit_vector::<Fr<other_curve<ppT>>>(elt);
        sp_compliance_step_pcd_circuit_input_bits.extend(&elt_bits);
    }

    sp_compliance_step_pcd_circuit_input_bits.resize(
        sp_translation_step_pcd_circuit_maker::<ppT>::input_capacity_in_bits(),
        false,
    );

    let result = pack_bit_vector_into_field_element_vector::<ppT::FieldT>(
        &sp_compliance_step_pcd_circuit_input_bits,
        sp_translation_step_pcd_circuit_maker::<ppT>::field_capacity(),
    );
    leave_block("Call to get_sp_translation_step_pcd_circuit_input", false);

    result
}
