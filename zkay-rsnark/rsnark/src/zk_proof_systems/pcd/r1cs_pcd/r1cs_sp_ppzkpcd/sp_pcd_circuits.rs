/** @file
 *****************************************************************************

 Declaration of functionality for creating and using the two PCD circuits in
 a single-predicate PCD construction.

 The implementation follows, extends, and optimizes the approach described
 in \[BCTV14]. At high level, there is a "compliance step" circuit and a
 "translation step" circuit. For more details see Section 4 of \[BCTV14].


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

//#ifndef SP_PCD_CIRCUITS_HPP_
// #define SP_PCD_CIRCUITS_HPP_

use crate::gadgetlib1::gadgets/gadget_from_r1cs;
use crate::gadgetlib1::gadgets::hashes::crh_gadget;
use crate::gadgetlib1::gadgets/pairing/pairing_params;
use crate::gadgetlib1::gadgets/verifiers/r1cs_ppzksnark_verifier_gadget;
use crate::gadgetlib1::protoboard;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/cp_handler;



/**************************** Compliance step ********************************/

/**
 * A compliance-step PCD circuit.
 *
 * The circuit is an R1CS that checks compliance (for the given compliance predicate)
 * and validity of previous proofs.
 */
template<typename ppT>
class sp_compliance_step_pcd_circuit_maker {
public:
    type ffec::Fr<ppT> FieldT;

    r1cs_pcd_compliance_predicate<FieldT> compliance_predicate;

    protoboard<FieldT> pb;

    pb_variable<FieldT> zero;

    std::shared_ptr<block_variable<FieldT> > block_for_outgoing_message;
    std::shared_ptr<CRH_with_field_out_gadget<FieldT> > hash_outgoing_message;

    std::vector<block_variable<FieldT> > blocks_for_incoming_messages;
    std::vector<pb_variable_array<FieldT> > sp_translation_step_vk_and_incoming_message_payload_digests;
    std::vector<multipacking_gadget<FieldT> > unpack_sp_translation_step_vk_and_incoming_message_payload_digests;
    std::vector<pb_variable_array<FieldT> > sp_translation_step_vk_and_incoming_message_payload_digest_bits;
    std::vector<CRH_with_field_out_gadget<FieldT> > hash_incoming_messages;

    std::shared_ptr<r1cs_ppzksnark_verification_key_variable<ppT> > sp_translation_step_vk;
    pb_variable_array<FieldT> sp_translation_step_vk_bits;

    pb_variable<FieldT> outgoing_message_type;
    pb_variable_array<FieldT> outgoing_message_payload;
    pb_variable_array<FieldT> outgoing_message_vars;

    pb_variable<FieldT> arity;
    std::vector<pb_variable<FieldT> > incoming_message_types;
    std::vector<pb_variable_array<FieldT> > incoming_message_payloads;
    std::vector<pb_variable_array<FieldT> > incoming_message_vars;

    pb_variable_array<FieldT> local_data;
    pb_variable_array<FieldT> cp_witness;
    std::shared_ptr<gadget_from_r1cs<FieldT> > compliance_predicate_as_gadget;

    pb_variable_array<FieldT> outgoing_message_bits;
    std::shared_ptr<multipacking_gadget<FieldT> > unpack_outgoing_message;

    std::vector<pb_variable_array<FieldT> > incoming_messages_bits;
    std::vector<multipacking_gadget<FieldT> > unpack_incoming_messages;

    pb_variable_array<FieldT> sp_compliance_step_pcd_circuit_input;
    pb_variable_array<FieldT> padded_translation_step_vk_and_outgoing_message_digest;
    std::vector<pb_variable_array<FieldT> > padded_translation_step_vk_and_incoming_messages_digests;

    std::vector<pb_variable_array<FieldT> > verifier_input;
    std::vector<r1cs_ppzksnark_proof_variable<ppT> > proof;
    pb_variable<FieldT> verification_result;
    std::vector<r1cs_ppzksnark_verifier_gadget<ppT> > verifiers;

    sp_compliance_step_pcd_circuit_maker(const r1cs_pcd_compliance_predicate<FieldT> &compliance_predicate);
    void generate_r1cs_constraints();
    r1cs_constraint_system<FieldT> get_circuit() const;

    void generate_r1cs_witness(const r1cs_ppzksnark_verification_key<other_curve<ppT> > &translation_step_pcd_circuit_vk,
                               const r1cs_pcd_compliance_predicate_primary_input<FieldT> &compliance_predicate_primary_input,
                               const r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> &compliance_predicate_auxiliary_input,
                               const std::vector<r1cs_ppzksnark_proof<other_curve<ppT> > > &incoming_proofs);
    r1cs_primary_input<FieldT> get_primary_input() const;
    r1cs_auxiliary_input<FieldT> get_auxiliary_input() const;

    static size_t field_logsize();
    static size_t field_capacity();
    static size_t input_size_in_elts();
    static size_t input_capacity_in_bits();
    static size_t input_size_in_bits();
};

/*************************** Translation step ********************************/

/**
 * A translation-step PCD circuit.
 *
 * The circuit is an R1CS that checks validity of previous proofs.
 */
template<typename ppT>
class sp_translation_step_pcd_circuit_maker {
public:
    type ffec::Fr<ppT> FieldT;

    protoboard<FieldT> pb;

    pb_variable_array<FieldT> sp_translation_step_pcd_circuit_input;
    pb_variable_array<FieldT> unpacked_sp_translation_step_pcd_circuit_input;
    pb_variable_array<FieldT> verifier_input;
    std::shared_ptr<multipacking_gadget<FieldT> > unpack_sp_translation_step_pcd_circuit_input;

    std::shared_ptr<r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> > hardcoded_sp_compliance_step_vk;
    std::shared_ptr<r1cs_ppzksnark_proof_variable<ppT> > proof;
    std::shared_ptr<r1cs_ppzksnark_online_verifier_gadget<ppT> > online_verifier;

    sp_translation_step_pcd_circuit_maker(const r1cs_ppzksnark_verification_key<other_curve<ppT> > &compliance_step_vk);
    void generate_r1cs_constraints();
    r1cs_constraint_system<FieldT> get_circuit() const;

    void generate_r1cs_witness(const r1cs_primary_input<ffec::Fr<ppT> > translation_step_input,
                               const r1cs_ppzksnark_proof<other_curve<ppT> > &compliance_step_proof);
    r1cs_primary_input<FieldT> get_primary_input() const;
    r1cs_auxiliary_input<FieldT> get_auxiliary_input() const;

    static size_t field_logsize();
    static size_t field_capacity();
    static size_t input_size_in_elts();
    static size_t input_capacity_in_bits();
    static size_t input_size_in_bits();
};

/****************************** Input maps ***********************************/

/**
 * Obtain the primary input for a compliance-step PCD circuit.
 */
template<typename ppT>
r1cs_primary_input<ffec::Fr<ppT> > get_sp_compliance_step_pcd_circuit_input(const ffec::bit_vector &sp_translation_step_vk_bits,
                                                                      const r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<ppT> > &primary_input);

/**
 * Obtain the primary input for a translation-step PCD circuit.
 */
template<typename ppT>
r1cs_primary_input<ffec::Fr<ppT> > get_sp_translation_step_pcd_circuit_input(const ffec::bit_vector &sp_translation_step_vk_bits,
                                                                       const r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<other_curve<ppT> > > &primary_input);



use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/sp_pcd_circuits;

//#endif // SP_PCD_CIRCUITS_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality for creating and using the two PCD circuits in
 a single-predicate PCD construction.

 See sp_pcd_circuits.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SP_PCD_CIRCUITS_TCC_
// #define SP_PCD_CIRCUITS_TCC_

use ffec::common::utils;

use crate::gadgetlib1::constraint_profiling;



template<typename ppT>
sp_compliance_step_pcd_circuit_maker<ppT>::sp_compliance_step_pcd_circuit_maker(const r1cs_pcd_compliance_predicate<FieldT> &compliance_predicate) :
    compliance_predicate(compliance_predicate)
{
    /* calculate some useful sizes */
    assert!(compliance_predicate.is_well_formed());
    assert!(compliance_predicate.has_equal_input_and_output_lengths());

    const size_t compliance_predicate_arity = compliance_predicate.max_arity;
    const size_t digest_size = CRH_with_field_out_gadget<FieldT>::get_digest_len();
    const size_t msg_size_in_bits = field_logsize() * (1+compliance_predicate.outgoing_message_payload_length);
    const size_t sp_translation_step_vk_size_in_bits = r1cs_ppzksnark_verification_key_variable<ppT>::size_in_bits(sp_translation_step_pcd_circuit_maker<other_curve<ppT> >::input_size_in_elts());
    const size_t padded_verifier_input_size = sp_translation_step_pcd_circuit_maker<other_curve<ppT> >::input_capacity_in_bits();

    print!("other curve input size = {}\n", sp_translation_step_pcd_circuit_maker<other_curve<ppT> >::input_size_in_elts());
    print!("translation_vk_bits = {}\n", sp_translation_step_vk_size_in_bits);
    print!("padded verifier input size = {}\n", padded_verifier_input_size);

    const size_t block_size = msg_size_in_bits + sp_translation_step_vk_size_in_bits;
    CRH_with_bit_out_gadget<FieldT>::sample_randomness(block_size);

    /* allocate input of the compliance PCD circuit */
    sp_compliance_step_pcd_circuit_input.allocate(pb, input_size_in_elts(), "sp_compliance_step_pcd_circuit_input");

    /* allocate inputs to the compliance predicate */
    outgoing_message_type.allocate(pb, "outgoing_message_type");
    outgoing_message_payload.allocate(pb, compliance_predicate.outgoing_message_payload_length, "outgoing_message_payload");

    outgoing_message_vars.insert(outgoing_message_vars.end(), outgoing_message_type);
    outgoing_message_vars.insert(outgoing_message_vars.end(), outgoing_message_payload.begin(), outgoing_message_payload.end());

    arity.allocate(pb, "arity");

    incoming_message_types.resize(compliance_predicate_arity);
    incoming_message_payloads.resize(compliance_predicate_arity);
    incoming_message_vars.resize(compliance_predicate_arity);
    for i in 0..compliance_predicate_arity
    {
        incoming_message_types[i].allocate(pb, FMT("", "incoming_message_type_{}", i));
        incoming_message_payloads[i].allocate(pb, compliance_predicate.outgoing_message_payload_length, FMT("", "incoming_message_payloads_{}", i));

        incoming_message_vars[i].insert(incoming_message_vars[i].end(), incoming_message_types[i]);
        incoming_message_vars[i].insert(incoming_message_vars[i].end(), incoming_message_payloads[i].begin(), incoming_message_payloads[i].end());
    }

    local_data.allocate(pb, compliance_predicate.local_data_length, "local_data");
    cp_witness.allocate(pb, compliance_predicate.witness_length, "cp_witness");

    /* convert compliance predicate from a constraint system into a gadget */
    pb_variable_array<FieldT> incoming_messages_concat;
    for i in 0..compliance_predicate_arity
    {
        incoming_messages_concat.insert(incoming_messages_concat.end(), incoming_message_vars[i].begin(), incoming_message_vars[i].end());
    }

    compliance_predicate_as_gadget.reset(new gadget_from_r1cs<FieldT>(pb,
        { outgoing_message_vars,
          pb_variable_array<FieldT>(1, arity),
          incoming_messages_concat,
          local_data,
          cp_witness },
            compliance_predicate.constraint_system, "compliance_predicate_as_gadget"));

    /* unpack messages to bits */
    outgoing_message_bits.allocate(pb, msg_size_in_bits, "outgoing_message_bits");
    unpack_outgoing_message.reset(new multipacking_gadget<FieldT>(pb, outgoing_message_bits, outgoing_message_vars, field_logsize(), "unpack_outgoing_message"));

    incoming_messages_bits.resize(compliance_predicate_arity);
    for i in 0..compliance_predicate_arity
    {
        incoming_messages_bits[i].allocate(pb, msg_size_in_bits, FMT("", "incoming_messages_bits_{}", i));
        unpack_incoming_messages.push(multipacking_gadget<FieldT>(pb, incoming_messages_bits[i], incoming_message_vars[i], field_logsize(), FMT("", "unpack_incoming_messages_{}", i)));
    }

    /* allocate digests */
    sp_translation_step_vk_and_incoming_message_payload_digests.resize(compliance_predicate_arity);
    for i in 0..compliance_predicate_arity
    {
        sp_translation_step_vk_and_incoming_message_payload_digests[i].allocate(pb, digest_size, FMT("", "sp_translation_step_vk_and_incoming_message_payload_digests_{}", i));
    }

    /* allocate blocks */
    sp_translation_step_vk_bits.allocate(pb, sp_translation_step_vk_size_in_bits, "sp_translation_step_vk_bits");

    block_for_outgoing_message.reset(new block_variable<FieldT>(pb, {
                sp_translation_step_vk_bits,
                outgoing_message_bits }, "block_for_outgoing_message"));

    for i in 0..compliance_predicate_arity
    {
        blocks_for_incoming_messages.push(block_variable<FieldT>(pb, {
                    sp_translation_step_vk_bits,
                    incoming_messages_bits[i] }, FMT("", "blocks_for_incoming_messages_zu", i)));
    }

    /* allocate hash checkers */
    hash_outgoing_message.reset(new CRH_with_field_out_gadget<FieldT>(pb, block_size, *block_for_outgoing_message, sp_compliance_step_pcd_circuit_input, "hash_outgoing_message"));

    for i in 0..compliance_predicate_arity
    {
        hash_incoming_messages.push(CRH_with_field_out_gadget<FieldT>(pb, block_size, blocks_for_incoming_messages[i], sp_translation_step_vk_and_incoming_message_payload_digests[i], FMT("", "hash_incoming_messages_{}", i)));
    }

    /* allocate useful zero variable */
    zero.allocate(pb, "zero");

    /* prepare arguments for the verifier */
    sp_translation_step_vk.reset(new r1cs_ppzksnark_verification_key_variable<ppT>(pb, sp_translation_step_vk_bits, sp_translation_step_pcd_circuit_maker<other_curve<ppT> >::input_size_in_elts(), "sp_translation_step_vk"));

    verification_result.allocate(pb, "verification_result");
    sp_translation_step_vk_and_incoming_message_payload_digest_bits.resize(compliance_predicate_arity);

    for i in 0..compliance_predicate_arity
    {
        sp_translation_step_vk_and_incoming_message_payload_digest_bits[i].allocate(pb, digest_size * field_logsize(), FMT("", "sp_translation_step_vk_and_incoming_message_payload_digest_bits_{}", i));
        unpack_sp_translation_step_vk_and_incoming_message_payload_digests.push(multipacking_gadget<FieldT>(pb,
                                                                                                            sp_translation_step_vk_and_incoming_message_payload_digest_bits[i],
                                                                                                            sp_translation_step_vk_and_incoming_message_payload_digests[i],
                                                                                                            field_logsize(),
                                                                                                            FMT("", "unpack_sp_translation_step_vk_and_incoming_message_payload_digests_{}", i)));

        verifier_input.push(sp_translation_step_vk_and_incoming_message_payload_digest_bits[i]);
        while (verifier_input[i].size() < padded_verifier_input_size)
        {
            verifier_input[i].push(zero);
        }

        proof.push(r1cs_ppzksnark_proof_variable<ppT>(pb, FMT("", "proof_{}", i)));
        verifiers.push(r1cs_ppzksnark_verifier_gadget<ppT>(pb,
                                                    *sp_translation_step_vk,
                                                    verifier_input[i],
                                                    sp_translation_step_pcd_circuit_maker<other_curve<ppT> >::field_capacity(),
                                                    proof[i],
                                                    verification_result,
                                                    FMT("", "verifiers_{}", i)));
    }

    pb.set_input_sizes(input_size_in_elts());
    print!("done compliance\n");
}

template<typename ppT>
void sp_compliance_step_pcd_circuit_maker<ppT>::generate_r1cs_constraints()
{
    const size_t digest_size = CRH_with_bit_out_gadget<FieldT>::get_digest_len();
    const size_t dimension = knapsack_dimension<FieldT>::dimension;
    ffec::print_indent(); print!("* Knapsack dimension: {}\n", dimension);

    const size_t compliance_predicate_arity = compliance_predicate.max_arity;
    ffec::print_indent(); print!("* Compliance predicate arity: {}\n", compliance_predicate_arity);
    ffec::print_indent(); print!("* Compliance predicate payload length: {}\n", compliance_predicate.outgoing_message_payload_length);
    ffec::print_indent(); print!("* Compliance predicate local data length: {}\n", compliance_predicate.local_data_length);
    ffec::print_indent(); print!("* Compliance predicate witness length: {}\n", compliance_predicate.witness_length);

    PROFILE_CONSTRAINTS(pb, "booleanity")
    {
        PROFILE_CONSTRAINTS(pb, "booleanity: unpack outgoing_message")
        {
            unpack_outgoing_message->generate_r1cs_constraints(true);
        }

        PROFILE_CONSTRAINTS(pb, "booleanity: unpack s incoming_message")
        {
            for i in 0..compliance_predicate_arity
            {
                unpack_incoming_messages[i].generate_r1cs_constraints(true);
            }
        }

        PROFILE_CONSTRAINTS(pb, "booleanity: unpack verification key")
        {
            sp_translation_step_vk->generate_r1cs_constraints(true);
        }
    }

    PROFILE_CONSTRAINTS(pb, "(1+s) copies of hash")
    {
        ffec::print_indent(); print!("* Digest-size: {}\n", digest_size);
        hash_outgoing_message->generate_r1cs_constraints();

        for i in 0..compliance_predicate_arity
        {
            hash_incoming_messages[i].generate_r1cs_constraints();
        }
    }

    PROFILE_CONSTRAINTS(pb, "s copies of repacking circuit")
    {
        for i in 0..compliance_predicate_arity
        {
            unpack_sp_translation_step_vk_and_incoming_message_payload_digests[i].generate_r1cs_constraints(true);
        }
    }

    PROFILE_CONSTRAINTS(pb, "compliance predicate")
    {
        compliance_predicate_as_gadget->generate_r1cs_constraints();
    }

    PROFILE_CONSTRAINTS(pb, "s copies of verifier for translated proofs")
    {
        PROFILE_CONSTRAINTS(pb, "check that s proofs lie on the curve")
        {
            for i in 0..compliance_predicate_arity
            {
                proof[i].generate_r1cs_constraints();
            }
        }

        for i in 0..compliance_predicate_arity
        {
            verifiers[i].generate_r1cs_constraints();
        }
    }

    PROFILE_CONSTRAINTS(pb, "miscellaneous")
    {
        generate_r1cs_equals_const_constraint<FieldT>(pb, zero, FieldT::zero(), "zero");
        generate_boolean_r1cs_constraint<FieldT>(pb, verification_result, "verification_result");

        /* type * (1-verification_result) = 0 */
        pb.add_r1cs_constraint(r1cs_constraint<FieldT>(incoming_message_types[0], 1 - verification_result, 0), "not_base_case_implies_valid_proofs");

        /* all types equal */
        for i in 1..compliance_predicate.max_arity
        {
            pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, incoming_message_types[0], incoming_message_types[i]),
                                   FMT("", "type_%zu_equal_to_type_0", i));
        }

        pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, arity, compliance_predicate_arity), "full_arity");
        pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, outgoing_message_type, FieldT(compliance_predicate.type)), "enforce_outgoing_type");
    }

    PRINT_CONSTRAINT_PROFILING();
    ffec::print_indent(); print!("* Number of constraints in sp_compliance_step_pcd_circuit: {}\n", pb.num_constraints());
}

template<typename ppT>
r1cs_constraint_system<ffec::Fr<ppT> > sp_compliance_step_pcd_circuit_maker<ppT>::get_circuit() const
{
    return pb.get_constraint_system();
}

template<typename ppT>
r1cs_primary_input<ffec::Fr<ppT> > sp_compliance_step_pcd_circuit_maker<ppT>::get_primary_input() const
{
    return pb.primary_input();
}

template<typename ppT>
r1cs_auxiliary_input<ffec::Fr<ppT> > sp_compliance_step_pcd_circuit_maker<ppT>::get_auxiliary_input() const
{
    return pb.auxiliary_input();
}

template<typename ppT>
void sp_compliance_step_pcd_circuit_maker<ppT>::generate_r1cs_witness(const r1cs_ppzksnark_verification_key<other_curve<ppT> > &sp_translation_step_pcd_circuit_vk,
                                                                      const r1cs_pcd_compliance_predicate_primary_input<FieldT> &compliance_predicate_primary_input,
                                                                      const r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> &compliance_predicate_auxiliary_input,
                                                                      const std::vector<r1cs_ppzksnark_proof<other_curve<ppT> > > &incoming_proofs)
{
    const size_t compliance_predicate_arity = compliance_predicate.max_arity;
    self.pb.clear_values();
    self.pb.val(zero) = FieldT::zero();

    compliance_predicate_as_gadget->generate_r1cs_witness(compliance_predicate_primary_input.as_r1cs_primary_input(),
                                                          compliance_predicate_auxiliary_input.as_r1cs_auxiliary_input(compliance_predicate.incoming_message_payload_lengths));
    self.pb.val(arity) = FieldT(compliance_predicate_arity);
    unpack_outgoing_message->generate_r1cs_witness_from_packed();
    for i in 0..compliance_predicate_arity
    {
        unpack_incoming_messages[i].generate_r1cs_witness_from_packed();
    }

    sp_translation_step_vk->generate_r1cs_witness(sp_translation_step_pcd_circuit_vk);
    hash_outgoing_message->generate_r1cs_witness();
    for i in 0..compliance_predicate_arity
    {
        hash_incoming_messages[i].generate_r1cs_witness();
        unpack_sp_translation_step_vk_and_incoming_message_payload_digests[i].generate_r1cs_witness_from_packed();
    }

    for i in 0..compliance_predicate_arity
    {
        proof[i].generate_r1cs_witness(incoming_proofs[i]);
        verifiers[i].generate_r1cs_witness();
    }

    if self.pb.val(incoming_message_types[0]) != FieldT::zero()
    {
        self.pb.val(verification_result) = FieldT::one();
    }

// #ifdef DEBUG
    generate_r1cs_constraints(); // force generating constraints
    assert!(self.pb.is_satisfied());
//#endif
}

template<typename ppT>
size_t sp_compliance_step_pcd_circuit_maker<ppT>::field_logsize()
{
    return ffec::Fr<ppT>::size_in_bits();
}

template<typename ppT>
size_t sp_compliance_step_pcd_circuit_maker<ppT>::field_capacity()
{
    return ffec::Fr<ppT>::capacity();
}

template<typename ppT>
size_t sp_compliance_step_pcd_circuit_maker<ppT>::input_size_in_elts()
{
    const size_t digest_size = CRH_with_field_out_gadget<FieldT>::get_digest_len();
    return digest_size;
}

template<typename ppT>
size_t sp_compliance_step_pcd_circuit_maker<ppT>::input_capacity_in_bits()
{
    return input_size_in_elts() * field_capacity();
}

template<typename ppT>
size_t sp_compliance_step_pcd_circuit_maker<ppT>::input_size_in_bits()
{
    return input_size_in_elts() * field_logsize();
}

template<typename ppT>
sp_translation_step_pcd_circuit_maker<ppT>::sp_translation_step_pcd_circuit_maker(const r1cs_ppzksnark_verification_key<other_curve<ppT> > &sp_compliance_step_vk)
{
    /* allocate input of the translation PCD circuit */
    sp_translation_step_pcd_circuit_input.allocate(pb, input_size_in_elts(), "sp_translation_step_pcd_circuit_input");

    /* unpack translation step PCD circuit input */
    unpacked_sp_translation_step_pcd_circuit_input.allocate(pb, sp_compliance_step_pcd_circuit_maker<other_curve<ppT> >::input_size_in_bits(), "unpacked_sp_translation_step_pcd_circuit_input");
    unpack_sp_translation_step_pcd_circuit_input.reset(new multipacking_gadget<FieldT>(pb, unpacked_sp_translation_step_pcd_circuit_input, sp_translation_step_pcd_circuit_input, field_capacity(), "unpack_sp_translation_step_pcd_circuit_input"));

    /* prepare arguments for the verifier */
    hardcoded_sp_compliance_step_vk.reset(new r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT>(pb, sp_compliance_step_vk, "hardcoded_sp_compliance_step_vk"));
    proof.reset(new r1cs_ppzksnark_proof_variable<ppT>(pb, "proof"));

    /* verify previous proof */
    online_verifier.reset(new r1cs_ppzksnark_online_verifier_gadget<ppT>(pb,
                                                          *hardcoded_sp_compliance_step_vk,
                                                          unpacked_sp_translation_step_pcd_circuit_input,
                                                          sp_compliance_step_pcd_circuit_maker<other_curve<ppT> >::field_logsize(),
                                                          *proof,
                                                          ONE, // must always accept
                                                          "verifier"));
    pb.set_input_sizes(input_size_in_elts());

    print!("done translation\n");
}

template<typename ppT>
void sp_translation_step_pcd_circuit_maker<ppT>::generate_r1cs_constraints()
{
    PROFILE_CONSTRAINTS(pb, "repacking: unpack circuit input")
    {
        unpack_sp_translation_step_pcd_circuit_input->generate_r1cs_constraints(true);
    }

    PROFILE_CONSTRAINTS(pb, "verifier for compliance proofs")
    {
        PROFILE_CONSTRAINTS(pb, "check that proof lies on the curve")
        {
            proof->generate_r1cs_constraints();
        }

        online_verifier->generate_r1cs_constraints();
    }

    PRINT_CONSTRAINT_PROFILING();
    ffec::print_indent(); print!("* Number of constraints in sp_translation_step_pcd_circuit: {}\n", pb.num_constraints());
}

template<typename ppT>
r1cs_constraint_system<ffec::Fr<ppT> > sp_translation_step_pcd_circuit_maker<ppT>::get_circuit() const
{
    return pb.get_constraint_system();
}

template<typename ppT>
void sp_translation_step_pcd_circuit_maker<ppT>::generate_r1cs_witness(const r1cs_primary_input<ffec::Fr<ppT> > sp_translation_step_input,
                                                                       const r1cs_ppzksnark_proof<other_curve<ppT> > &compliance_step_proof)
{
    self.pb.clear_values();
    sp_translation_step_pcd_circuit_input.fill_with_field_elements(pb, sp_translation_step_input);
    unpack_sp_translation_step_pcd_circuit_input->generate_r1cs_witness_from_packed();

    proof->generate_r1cs_witness(compliance_step_proof);
    online_verifier->generate_r1cs_witness();

// #ifdef DEBUG
    generate_r1cs_constraints(); // force generating constraints

    print!("Input to the translation circuit:\n");
    for i in 0..self.pb.num_inputs()
    {
        self.pb.val(pb_variable<FieldT>(i+1)).print();
    }

    assert!(self.pb.is_satisfied());
//#endif
}

template<typename ppT>
r1cs_primary_input<ffec::Fr<ppT> > sp_translation_step_pcd_circuit_maker<ppT>::get_primary_input() const
{
    return pb.primary_input();
}

template<typename ppT>
r1cs_auxiliary_input<ffec::Fr<ppT> > sp_translation_step_pcd_circuit_maker<ppT>::get_auxiliary_input() const
{
    return pb.auxiliary_input();
}

template<typename ppT>
size_t sp_translation_step_pcd_circuit_maker<ppT>::field_logsize()
{
    return ffec::Fr<ppT>::size_in_bits();
}

template<typename ppT>
size_t sp_translation_step_pcd_circuit_maker<ppT>::field_capacity()
{
    return ffec::Fr<ppT>::capacity();
}

template<typename ppT>
size_t sp_translation_step_pcd_circuit_maker<ppT>::input_size_in_elts()
{
    return ffec::div_ceil(sp_compliance_step_pcd_circuit_maker<other_curve<ppT> >::input_size_in_bits(), sp_translation_step_pcd_circuit_maker<ppT>::field_capacity());
}

template<typename ppT>
size_t sp_translation_step_pcd_circuit_maker<ppT>::input_capacity_in_bits()
{
    return input_size_in_elts() * field_capacity();
}

template<typename ppT>
size_t sp_translation_step_pcd_circuit_maker<ppT>::input_size_in_bits()
{
    return input_size_in_elts() * field_logsize();
}

template<typename ppT>
r1cs_primary_input<ffec::Fr<ppT> > get_sp_compliance_step_pcd_circuit_input(const ffec::bit_vector &sp_translation_step_vk_bits,
                                                                      const r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<ppT> > &primary_input)
{
    ffec::enter_block("Call to get_sp_compliance_step_pcd_circuit_input");
    type ffec::Fr<ppT> FieldT;

    const r1cs_variable_assignment<FieldT> outgoing_message_as_va = primary_input.outgoing_message->as_r1cs_variable_assignment();
    ffec::bit_vector msg_bits;
    for elt in &outgoing_message_as_va
    {
        const ffec::bit_vector elt_bits = ffec::convert_field_element_to_bit_vector(elt);
        msg_bits.insert(msg_bits.end(), elt_bits.begin(), elt_bits.end());
    }

    ffec::bit_vector block;
    block.insert(block.end(), sp_translation_step_vk_bits.begin(), sp_translation_step_vk_bits.end());
    block.insert(block.end(), msg_bits.begin(), msg_bits.end());

    ffec::enter_block("Sample CRH randomness");
    CRH_with_field_out_gadget<FieldT>::sample_randomness(block.size());
    ffec::leave_block("Sample CRH randomness");

    const std::vector<FieldT> digest = CRH_with_field_out_gadget<FieldT>::get_hash(block);
    ffec::leave_block("Call to get_sp_compliance_step_pcd_circuit_input");

    return digest;
}

template<typename ppT>
r1cs_primary_input<ffec::Fr<ppT> > get_sp_translation_step_pcd_circuit_input(const ffec::bit_vector &sp_translation_step_vk_bits,
                                                                       const r1cs_pcd_compliance_predicate_primary_input<ffec::Fr<other_curve<ppT> > > &primary_input)
{
    ffec::enter_block("Call to get_sp_translation_step_pcd_circuit_input");
    type ffec::Fr<ppT> FieldT;

    const std::vector<ffec::Fr<other_curve<ppT> > > sp_compliance_step_pcd_circuit_input = get_sp_compliance_step_pcd_circuit_input<other_curve<ppT> >(sp_translation_step_vk_bits, primary_input);
    ffec::bit_vector sp_compliance_step_pcd_circuit_input_bits;
    for elt in &sp_compliance_step_pcd_circuit_input
    {
        const ffec::bit_vector elt_bits = ffec::convert_field_element_to_bit_vector<ffec::Fr<other_curve<ppT> > >(elt);
        sp_compliance_step_pcd_circuit_input_bits.insert(sp_compliance_step_pcd_circuit_input_bits.end(), elt_bits.begin(), elt_bits.end());
    }

    sp_compliance_step_pcd_circuit_input_bits.resize(sp_translation_step_pcd_circuit_maker<ppT>::input_capacity_in_bits(), false);

    const r1cs_primary_input<FieldT> result = ffec::pack_bit_vector_into_field_element_vector<FieldT>(sp_compliance_step_pcd_circuit_input_bits, sp_translation_step_pcd_circuit_maker<ppT>::field_capacity());
    ffec::leave_block("Call to get_sp_translation_step_pcd_circuit_input");

    return result;
}



//#endif // SP_PCD_CIRCUITS_TCC_
