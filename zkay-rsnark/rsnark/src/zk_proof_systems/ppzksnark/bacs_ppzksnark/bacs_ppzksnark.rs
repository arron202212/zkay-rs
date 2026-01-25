//  Declaration of interfaces for a ppzkSNARK for BACS.

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
//  (1) a BACS-to-R1CS reduction, and
//  (2) a ppzkSNARK for R1CS.

//  Acronyms:

//  - BACS = "Bilinear Arithmetic Circuit Satisfiability"
//  - R1CS = "Rank-1 Constraint System"
//  - ppzkSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

// use crate::relations::circuit_satisfaction_problems::bacs::bacs;
// use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/bacs_ppzksnark_params;
// use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::reductions::bacs_to_r1cs::bacs_to_r1cs::{
    bacs_to_r1cs_instance_map, bacs_to_r1cs_witness_map,
};
use crate::relations::circuit_satisfaction_problems::bacs::bacs::{
    bacs_auxiliary_input, bacs_circuit, bacs_primary_input,
};
use crate::zk_proof_systems::ppzksnark::bacs_ppzksnark::bacs_ppzksnark_params::{
    bacs_ppzksnark_auxiliary_input, bacs_ppzksnark_circuit, bacs_ppzksnark_primary_input,
};
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark_generator;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_online_verifier_strong_IC, r1cs_ppzksnark_proving_key,
    r1cs_ppzksnark_verification_key,
};
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_online_verifier_weak_IC, r1cs_ppzksnark_processed_verification_key,
    r1cs_ppzksnark_proof, r1cs_ppzksnark_prover, r1cs_ppzksnark_verifier_process_vk,
};
use ff_curves::Fr;
use ffec::common::profiling::{enter_block, leave_block};
use std::ops::Mul;

/******************************** Proving key ********************************/

// pub struct bacs_ppzksnark_proving_key;

// std::ostream& operator<<(std::ostream &out, pk:&bacs_ppzksnark_proving_key<ppT>);

// std::istream& operator>>(std::istream &in, bacs_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the BACS ppzkSNARK.
 */
#[derive(Default, Clone)]
pub struct bacs_ppzksnark_proving_key<ppT: ppTConfig> {
    pub circuit: bacs_ppzksnark_circuit<ppT>,
    pub r1cs_pk: r1cs_ppzksnark_proving_key<ppT>,
}
impl<ppT: ppTConfig> bacs_ppzksnark_proving_key<ppT> {
    // bacs_ppzksnark_proving_key() {};
    // bacs_ppzksnark_proving_key(other:&bacs_ppzksnark_proving_key<ppT>) = default;
    // bacs_ppzksnark_proving_key(bacs_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(
        circuit: bacs_ppzksnark_circuit<ppT>,
        r1cs_pk: r1cs_ppzksnark_proving_key<ppT>,
    ) -> Self {
        Self { circuit, r1cs_pk }
    }
    // bacs_ppzksnark_proving_key(bacs_ppzksnark_circuit<ppT> &&circuit,
    //                            r1cs_ppzksnark_proving_key<ppT> &&r1cs_pk)->Self
    //     circuit((circuit)), r1cs_pk((r1cs_pk))
    // {}

    // bacs_ppzksnark_proving_key<ppT>& operator=(other:&bacs_ppzksnark_proving_key<ppT>) = default;

    pub fn G1_size(&self) -> usize {
        self.r1cs_pk.g1_size()
    }

    pub fn G2_size(&self) -> usize {
        self.r1cs_pk.g2_size()
    }

    pub fn G1_sparse_size(&self) -> usize {
        self.r1cs_pk.g1_sparse_size()
    }

    pub fn G2_sparse_size(&self) -> usize {
        self.r1cs_pk.g2_sparse_size()
    }

    pub fn size_in_bits(&self) -> usize {
        self.r1cs_pk.size_in_bits()
    }

    pub fn print_size(&self) {
        self.r1cs_pk.print_size();
    }

    // bool operator==(other:&bacs_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pk:&bacs_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, bacs_ppzksnark_proving_key<ppT> &pk);
}

/******************************* Verification key ****************************/

/**
 * A verification key for the BACS ppzkSNARK.
 */

pub type bacs_ppzksnark_verification_key<ppT> = r1cs_ppzksnark_verification_key<ppT>;

/************************ Processed verification key *************************/

/**
 * A processed verification key for the BACS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */

pub type bacs_ppzksnark_processed_verification_key<ppT> =
    r1cs_ppzksnark_processed_verification_key<ppT>;

/********************************** Key pair *********************************/

/**
 * A key pair for the BACS ppzkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Default, Clone)]
pub struct bacs_ppzksnark_keypair<ppT: ppTConfig> {
    pub pk: bacs_ppzksnark_proving_key<ppT>,
    pub vk: bacs_ppzksnark_verification_key<ppT>,
}
impl<ppT: ppTConfig> bacs_ppzksnark_keypair<ppT> {
    // bacs_ppzksnark_keypair() {};
    // bacs_ppzksnark_keypair(bacs_ppzksnark_keypair<ppT> &&other) = default;
    pub fn new(
        pk: bacs_ppzksnark_proving_key<ppT>,
        vk: bacs_ppzksnark_verification_key<ppT>,
    ) -> Self {
        Self { pk, vk }
    }

    // bacs_ppzksnark_keypair(bacs_ppzksnark_proving_key<ppT> &&pk,
    //                        bacs_ppzksnark_verification_key<ppT> &&vk)->Self
    //     pk((pk)),
    //     vk((vk))
    // {}
}

/*********************************** Proof ***********************************/

/**
 * A proof for the BACS ppzkSNARK.
 */

pub type bacs_ppzksnark_proof<ppT> = r1cs_ppzksnark_proof<ppT>;

/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the BACS ppzkSNARK.
 *
 * Given a BACS circuit C, this algorithm produces proving and verification keys for C.
 */

// bacs_ppzksnark_keypair<ppT> bacs_ppzksnark_generator(circuit:&bacs_ppzksnark_circuit<ppT>);

/**
 * A prover algorithm for the BACS ppzkSNARK.
 *
 * Given a BACS primary input X and a BACS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that C(X,Y)=0''.
 * Above, C is the BACS circuit that was given as input to the generator algorithm.
 */

// bacs_ppzksnark_proof<ppT> bacs_ppzksnark_prover(pk:&bacs_ppzksnark_proving_key<ppT>,
//                                                 primary_input:&bacs_ppzksnark_primary_input<ppT>,
//                                                 auxiliary_input:&bacs_ppzksnark_auxiliary_input<ppT>);

/*
Below are four variants of verifier algorithm for the BACS ppzkSNARK.

These are the four cases that arise from the following two choices:

(1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
    In the latter case, we call the algorithm an "online verifier".

(2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
    Strong input consistency requires that |primary_input| = C.num_inputs, whereas
    weak input consistency requires that |primary_input| <= C.num_inputs (and
    the primary input is implicitly padded with zeros up to length C.num_inputs).
*/

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */

// bool bacs_ppzksnark_verifier_weak_IC(vk:&bacs_ppzksnark_verification_key<ppT>,
//                                      primary_input:&bacs_ppzksnark_primary_input<ppT>,
//                                      proof:&bacs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

// bool bacs_ppzksnark_verifier_strong_IC(vk:&bacs_ppzksnark_verification_key<ppT>,
//                                        primary_input:&bacs_ppzksnark_primary_input<ppT>,
//                                        proof:&bacs_ppzksnark_proof<ppT>);

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

// bacs_ppzksnark_processed_verification_key<ppT> bacs_ppzksnark_verifier_process_vk(vk:&bacs_ppzksnark_verification_key<ppT>);

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

// bool bacs_ppzksnark_online_verifier_weak_IC(pvk:&bacs_ppzksnark_processed_verification_key<ppT>,
//                                             primary_input:&bacs_ppzksnark_primary_input<ppT>,
//                                             proof:&bacs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

// bool bacs_ppzksnark_online_verifier_strong_IC(pvk:&bacs_ppzksnark_processed_verification_key<ppT>,
//                                               primary_input:&bacs_ppzksnark_primary_input<ppT>,
//                                               proof:&bacs_ppzksnark_proof<ppT>);

// use crate::reductions::bacs_to_r1cs::bacs_to_r1cs;

// bool bacs_ppzksnark_proving_key<ppT>::operator==(other:&bacs_ppzksnark_proving_key<ppT>) const
// {
//     return (self.circuit == other.circuit &&
//             self.r1cs_pk == other.r1cs_pk);
// }

// std::ostream& operator<<(std::ostream &out, pk:&bacs_ppzksnark_proving_key<ppT>)
// {
//     out << pk.circuit << OUTPUT_NEWLINE;
//     out << pk.r1cs_pk << OUTPUT_NEWLINE;

//     return out;
// }

// std::istream& operator>>(std::istream &in, bacs_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.circuit;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.r1cs_pk;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

type FieldT<ppT> = Fr<ppT>;

pub fn bacs_ppzksnark_generator<ppT: ppTConfig>(
    circuit: &bacs_ppzksnark_circuit<ppT>,
) -> bacs_ppzksnark_keypair<ppT> {
    // type FieldT=Fr<ppT>;

    enter_block("Call to bacs_ppzksnark_generator", false);
    let r1cs_cs =
        bacs_to_r1cs_instance_map::<FieldT<ppT>, pb_variable, pb_linear_combination>(circuit);
    let r1cs_keypair = r1cs_ppzksnark_generator::<ppT>(&r1cs_cs);
    leave_block("Call to bacs_ppzksnark_generator", false);

    bacs_ppzksnark_keypair::<ppT>::new(
        bacs_ppzksnark_proving_key::<ppT>::new(circuit.clone(), r1cs_keypair.pk),
        r1cs_keypair.vk,
    )
}

pub fn bacs_ppzksnark_prover<ppT: ppTConfig>(
    pk: &bacs_ppzksnark_proving_key<ppT>,
    primary_input: &bacs_ppzksnark_primary_input<ppT>,
    auxiliary_input: &bacs_ppzksnark_auxiliary_input<ppT>,
) -> bacs_ppzksnark_proof<ppT>
where
    knowledge_commitment<
        <ppT as ff_curves::PublicParams>::G1,
        <ppT as ff_curves::PublicParams>::G1,
    >: Mul<
            <ppT as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <ppT as ff_curves::PublicParams>::G1,
                <ppT as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <ppT as ff_curves::PublicParams>::G2,
        <ppT as ff_curves::PublicParams>::G1,
    >: Mul<
            <ppT as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <ppT as ff_curves::PublicParams>::G2,
                <ppT as ff_curves::PublicParams>::G1,
            >,
        >,
{
    // type FieldT=Fr<ppT>;

    enter_block("Call to bacs_ppzksnark_prover", false);
    let r1cs_va = bacs_to_r1cs_witness_map::<FieldT<ppT>, pb_variable, pb_linear_combination>(
        &pk.circuit,
        primary_input,
        auxiliary_input,
    );
    let r1cs_ai = r1cs_va[primary_input.len()..].to_vec(); // TODO: faster to just change bacs_to_r1cs_witness_map into two :(
    let r1cs_proof = r1cs_ppzksnark_prover::<ppT>(&pk.r1cs_pk, primary_input, &r1cs_ai);
    leave_block("Call to bacs_ppzksnark_prover", false);

    r1cs_proof
}

pub fn bacs_ppzksnark_verifier_process_vk<ppT: ppTConfig>(
    vk: &bacs_ppzksnark_verification_key<ppT>,
) -> bacs_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to bacs_ppzksnark_verifier_process_vk", false);
    let pvk = r1cs_ppzksnark_verifier_process_vk::<ppT>(vk);
    leave_block("Call to bacs_ppzksnark_verifier_process_vk", false);

    pvk
}

pub fn bacs_ppzksnark_verifier_weak_IC<ppT: ppTConfig>(
    vk: &bacs_ppzksnark_verification_key<ppT>,
    primary_input: &bacs_ppzksnark_primary_input<ppT>,
    proof: &bacs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to bacs_ppzksnark_verifier_weak_IC", false);
    let pvk = bacs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let bit = r1cs_ppzksnark_online_verifier_weak_IC::<ppT>(&pvk, primary_input, proof);
    leave_block("Call to bacs_ppzksnark_verifier_weak_IC", false);

    bit
}

pub fn bacs_ppzksnark_verifier_strong_IC<ppT: ppTConfig>(
    vk: &bacs_ppzksnark_verification_key<ppT>,
    primary_input: &bacs_ppzksnark_primary_input<ppT>,
    proof: &bacs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to bacs_ppzksnark_verifier_strong_IC", false);
    let pvk = bacs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let bit = r1cs_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, primary_input, proof);
    leave_block("Call to bacs_ppzksnark_verifier_strong_IC", false);

    bit
}

pub fn bacs_ppzksnark_online_verifier_weak_IC<ppT: ppTConfig>(
    pvk: &bacs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &bacs_ppzksnark_primary_input<ppT>,
    proof: &bacs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to bacs_ppzksnark_online_verifier_weak_IC", false);
    let bit = r1cs_ppzksnark_online_verifier_weak_IC::<ppT>(pvk, primary_input, proof);
    leave_block("Call to bacs_ppzksnark_online_verifier_weak_IC", false);

    bit
}

pub fn bacs_ppzksnark_online_verifier_strong_IC<ppT: ppTConfig>(
    pvk: &bacs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &bacs_ppzksnark_primary_input<ppT>,
    proof: &bacs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to bacs_ppzksnark_online_verifier_strong_IC", false);
    let bit = r1cs_ppzksnark_online_verifier_strong_IC::<ppT>(pvk, primary_input, proof);
    leave_block("Call to bacs_ppzksnark_online_verifier_strong_IC", false);

    bit
}
