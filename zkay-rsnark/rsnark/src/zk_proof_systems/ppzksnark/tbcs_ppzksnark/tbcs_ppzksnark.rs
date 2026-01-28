//  Declaration of interfaces for a ppzkSNARK for TBCS.

//  This includes:
//  - pub struct for proving key
//  - pub struct for verification key
//  - pub struct for processed verification key
//  - pub struct for key pair (proving key & verification key)
//  - pub struct for proof
//  - generator algorithm
//  - prover algorithm
//  - verifier algorithm (with strong or weak input consistency)
//  - online verifier algorithm (with strong or weak input consistency)

//  The implementation is a straightforward combination of:
//  (1) a TBCS-to-USCS reduction, and
//  (2) a ppzkSNARK for USCS.

//  Acronyms:

//  - TBCS = "Two-input Boolean Circuit Satisfiability"
//  - USCS = "Unitary-Square Constraint System"
//  - ppzkSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

// use crate::relations::circuit_satisfaction_problems::tbcs::tbcs;
// use libsnark/zk_proof_systems/ppzksnark/tbcs_ppzksnark/tbcs_ppzksnark_params;
// use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/uscs_ppzksnark;

 use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::zk_proof_systems::ppzksnark::tbcs_ppzksnark::tbcs_ppzksnark_params::{tbcs_ppzksnark_auxiliary_input,tbcs_ppzksnark_circuit,tbcs_ppzksnark_primary_input};
use crate::reductions::tbcs_to_uscs::tbcs_to_uscs::{tbcs_to_uscs_instance_map,tbcs_to_uscs_witness_map};
use ffec::field_utils::field_utils::convert_bit_vector_to_field_element_vector;
 use ff_curves::Fr;
use ffec::common::profiling::{enter_block,leave_block};

/******************************** Proving key ********************************/

// pub struct tbcs_ppzksnark_proving_key;

// std::ostream& operator<<(std::ostream &out, pk:&tbcs_ppzksnark_proving_key<ppT>);

// std::istream& operator>>(std::istream &in, tbcs_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the TBCS ppzkSNARK.
 */
#[derive(Default,Clone)]
pub struct tbcs_ppzksnark_proving_key<ppT: ppTConfig> {
    // type FieldT=Fr<ppT>;
    circuit: tbcs_ppzksnark_circuit,
    uscs_pk: uscs_ppzksnark_proving_key<ppT>,
}
impl<ppT: ppTConfig> tbcs_ppzksnark_proving_key<ppT> {
    // tbcs_ppzksnark_proving_key() {};
    // tbcs_ppzksnark_proving_key(other:&tbcs_ppzksnark_proving_key<ppT>) = default;
    // tbcs_ppzksnark_proving_key(tbcs_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(circuit: tbcs_ppzksnark_circuit, uscs_pk: uscs_ppzksnark_proving_key<ppT>) -> Self {
        Self { circuit, uscs_pk }
    }
    // tbcs_ppzksnark_proving_key(tbcs_ppzksnark_circuit &&circuit,
    //                            uscs_ppzksnark_proving_key<ppT> &&uscs_pk)->Self
    //     circuit((circuit)), uscs_pk((uscs_pk))
    // {}

    // tbcs_ppzksnark_proving_key<ppT>& operator=(other:&tbcs_ppzksnark_proving_key<ppT>) = default;

    pub fn G1_size(&self) -> usize {
        self.uscs_pk.G1_size()
    }

    pub fn G2_size(&self) -> usize {
        self.uscs_pk.G2_size()
    }

    pub fn G1_sparse_size(&self) -> usize {
        self.uscs_pk.G1_sparse_size()
    }

    pub fn G2_sparse_size(&self) -> usize {
        self.uscs_pk.G2_sparse_size()
    }

    pub fn size_in_bits(&self) -> usize {
        self.uscs_pk.size_in_bits()
    }

    pub fn print_size(&self) {
        self.uscs_pk.print_size();
    }

    // bool operator==(other:&tbcs_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pk:&tbcs_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, tbcs_ppzksnark_proving_key<ppT> &pk);
}

/******************************* Verification key ****************************/

/**
 * A verification key for the TBCS ppzkSNARK.
 */

pub type tbcs_ppzksnark_verification_key<ppT> = uscs_ppzksnark_verification_key<ppT>;

/************************ Processed verification key *************************/

/**
 * A processed verification key for the TBCS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */

pub type tbcs_ppzksnark_processed_verification_key<ppT> =
    uscs_ppzksnark_processed_verification_key<ppT>;

/********************************** Key pair *********************************/

/**
 * A key pair for the TBCS ppzkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Default,Clone)]
pub struct tbcs_ppzksnark_keypair<ppT: ppTConfig> {
    pk: tbcs_ppzksnark_proving_key<ppT>,
    vk: tbcs_ppzksnark_verification_key<ppT>,
}
impl<ppT: ppTConfig> tbcs_ppzksnark_keypair<ppT> {
    // tbcs_ppzksnark_keypair() {};
    // tbcs_ppzksnark_keypair(tbcs_ppzksnark_keypair<ppT> &&other) = default;
    pub fn new(
        pk: tbcs_ppzksnark_proving_key<ppT>,
        vk: tbcs_ppzksnark_verification_key<ppT>,
    ) -> Self {
        Self { pk, vk }
    }

    // tbcs_ppzksnark_keypair(tbcs_ppzksnark_proving_key<ppT> &&pk,
    //                        tbcs_ppzksnark_verification_key<ppT> &&vk)->Self
    //     pk((pk)),
    //     vk((vk))
    // {}
}

/*********************************** Proof ***********************************/

/**
 * A proof for the TBCS ppzkSNARK.
 */

pub type tbcs_ppzksnark_proof<ppT> = uscs_ppzksnark_proof<ppT>;

/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the TBCS ppzkSNARK.
 *
 * Given a TBCS circuit C, this algorithm produces proving and verification keys for C.
 */

// tbcs_ppzksnark_keypair<ppT> tbcs_ppzksnark_generator(circuit:&tbcs_ppzksnark_circuit);

/**
 * A prover algorithm for the TBCS ppzkSNARK.
 *
 * Given a TBCS primary input X and a TBCS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that C(X,Y)=0''.
 * Above, C is the TBCS circuit that was given as input to the generator algorithm.
 */

// tbcs_ppzksnark_proof<ppT> tbcs_ppzksnark_prover(pk:&tbcs_ppzksnark_proving_key<ppT>,
//                                                 primary_input:&tbcs_ppzksnark_primary_input,
//                                                 auxiliary_input:&tbcs_ppzksnark_auxiliary_input);

/*
Below are four variants of verifier algorithm for the TBCS ppzkSNARK.

These are the four cases that arise from the following two choices:

(1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
    In the latter case, we call the algorithm an "online verifier".

(2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
    Strong input consistency requires that |primary_input| = C.num_inputs, whereas
    weak input consistency requires that |primary_input| <= C.num_inputs (and
    the primary input is implicitly padded with zeros up to length C.num_inputs).
*/

/**
 * A verifier algorithm for the TBCS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */

// bool tbcs_ppzksnark_verifier_weak_IC(vk:&tbcs_ppzksnark_verification_key<ppT>,
//                                      primary_input:&tbcs_ppzksnark_primary_input,
//                                      proof:&tbcs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the TBCS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

// bool tbcs_ppzksnark_verifier_strong_IC(vk:&tbcs_ppzksnark_verification_key<ppT>,
//                                        primary_input:&tbcs_ppzksnark_primary_input,
//                                        proof:&tbcs_ppzksnark_proof<ppT>);

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

// tbcs_ppzksnark_processed_verification_key<ppT> tbcs_ppzksnark_verifier_process_vk(vk:&tbcs_ppzksnark_verification_key<ppT>);

/**
 * A verifier algorithm for the TBCS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

// bool tbcs_ppzksnark_online_verifier_weak_IC(pvk:&tbcs_ppzksnark_processed_verification_key<ppT>,
//                                             primary_input:&tbcs_ppzksnark_primary_input,
//                                             proof:&tbcs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the TBCS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

// bool tbcs_ppzksnark_online_verifier_strong_IC(pvk:&tbcs_ppzksnark_processed_verification_key<ppT>,
//                                               primary_input:&tbcs_ppzksnark_primary_input,
//                                               proof:&tbcs_ppzksnark_proof<ppT>);

// use libsnark/reductions/tbcs_to_uscs/tbcs_to_uscs;

// bool tbcs_ppzksnark_proving_key<ppT>::operator==(other:&tbcs_ppzksnark_proving_key<ppT>) const
// {
//     return (self.circuit == other.circuit &&
//             self.uscs_pk == other.uscs_pk);
// }

// std::ostream& operator<<(std::ostream &out, pk:&tbcs_ppzksnark_proving_key<ppT>)
// {
//     out << pk.circuit << OUTPUT_NEWLINE;
//     out << pk.uscs_pk << OUTPUT_NEWLINE;

//     return out;
// }

// std::istream& operator>>(std::istream &in, tbcs_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.circuit;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.uscs_pk;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

type FieldT<ppT> = Fr<ppT>;
pub fn tbcs_ppzksnark_generator<ppT: ppTConfig>(
    circuit: &tbcs_ppzksnark_circuit,
) -> tbcs_ppzksnark_keypair<ppT> {
    enter_block("Call to tbcs_ppzksnark_generator", false);
    let uscs_cs = tbcs_to_uscs_instance_map::<FieldT<ppT>>(circuit);
    let uscs_keypair = uscs_ppzksnark_generator::<ppT>(uscs_cs);
    leave_block("Call to tbcs_ppzksnark_generator", false);

    tbcs_ppzksnark_keypair::<ppT>::new(
        tbcs_ppzksnark_proving_key::<ppT>(circuit, uscs_keypair.pk),
        uscs_keypair.vk,
    )
}

pub fn tbcs_ppzksnark_prover<ppT: ppTConfig>(
    pk: &tbcs_ppzksnark_proving_key<ppT>,
    primary_input: &tbcs_ppzksnark_primary_input,
    auxiliary_input: &tbcs_ppzksnark_auxiliary_input,
) -> tbcs_ppzksnark_proof<ppT> {
    // type FieldT=Fr<ppT>;

    enter_block("Call to tbcs_ppzksnark_prover", false);
    let uscs_va =
        tbcs_to_uscs_witness_map::<FieldT<ppT>>(pk.circuit, primary_input, auxiliary_input);
    let uscs_pi = convert_bit_vector_to_field_element_vector::<FieldT<ppT>>(primary_input);
    let uscs_ai = &uscs_va[primary_input.len()..]; // TODO: faster to just change bacs_to_r1cs_witness_map into two :(
    let uscs_proof = uscs_ppzksnark_prover::<ppT>(pk.uscs_pk, uscs_pi, uscs_ai);
    leave_block("Call to tbcs_ppzksnark_prover", false);

    uscs_proof
}

pub fn tbcs_ppzksnark_verifier_process_vk<ppT: ppTConfig>(
    vk: &tbcs_ppzksnark_verification_key<ppT>,
) -> tbcs_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to tbcs_ppzksnark_verifier_process_vk", false);
    let pvk = uscs_ppzksnark_verifier_process_vk::<ppT>(vk);
    leave_block("Call to tbcs_ppzksnark_verifier_process_vk", false);

    pvk
}

pub fn tbcs_ppzksnark_verifier_weak_IC<ppT: ppTConfig>(
    vk: &tbcs_ppzksnark_verification_key<ppT>,
    primary_input: &tbcs_ppzksnark_primary_input,
    proof: &tbcs_ppzksnark_proof<ppT>,
) -> bool {
    // type FieldT=Fr<ppT>;
    enter_block("Call to tbcs_ppzksnark_verifier_weak_IC", false);
    let uscs_input = convert_bit_vector_to_field_element_vector::<FieldT<ppT>>(primary_input);
    let pvk = tbcs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let bit = uscs_ppzksnark_online_verifier_weak_IC::<ppT>(pvk, uscs_input, proof);
    leave_block("Call to tbcs_ppzksnark_verifier_weak_IC", false);

    bit
}

pub fn tbcs_ppzksnark_verifier_strong_IC<ppT: ppTConfig>(
    vk: &tbcs_ppzksnark_verification_key<ppT>,
    primary_input: &tbcs_ppzksnark_primary_input,
    proof: &tbcs_ppzksnark_proof<ppT>,
) -> bool {
    // type FieldT=Fr<ppT>;
    enter_block("Call to tbcs_ppzksnark_verifier_strong_IC", false);
    let pvk = tbcs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let uscs_input = convert_bit_vector_to_field_element_vector::<FieldT<ppT>>(primary_input);
    let bit = uscs_ppzksnark_online_verifier_strong_IC::<ppT>(pvk, uscs_input, proof);
    leave_block("Call to tbcs_ppzksnark_verifier_strong_IC", false);

    bit
}

pub fn tbcs_ppzksnark_online_verifier_weak_IC<ppT: ppTConfig>(
    pvk: &tbcs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &tbcs_ppzksnark_primary_input,
    proof: &tbcs_ppzksnark_proof<ppT>,
) -> bool {
    // type FieldT=Fr<ppT>;
    enter_block("Call to tbcs_ppzksnark_online_verifier_weak_IC", false);
    let uscs_input = convert_bit_vector_to_field_element_vector::<FieldT<ppT>>(primary_input);
    let bit = uscs_ppzksnark_online_verifier_weak_IC::<ppT>(pvk, uscs_input, proof);
    leave_block("Call to tbcs_ppzksnark_online_verifier_weak_IC", false);

    bit
}

pub fn tbcs_ppzksnark_online_verifier_strong_IC<ppT: ppTConfig>(
    pvk: &tbcs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &tbcs_ppzksnark_primary_input,
    proof: &tbcs_ppzksnark_proof<ppT>,
) -> bool {
    // type FieldT=Fr<ppT>;
    enter_block("Call to tbcs_ppzksnark_online_verifier_strong_IC", false);
    let uscs_input = convert_bit_vector_to_field_element_vector::<FieldT<ppT>>(primary_input);
    let bit = uscs_ppzksnark_online_verifier_strong_IC::<ppT>(pvk, uscs_input, proof);
    leave_block("Call to tbcs_ppzksnark_online_verifier_strong_IC", false);

    bit
}
