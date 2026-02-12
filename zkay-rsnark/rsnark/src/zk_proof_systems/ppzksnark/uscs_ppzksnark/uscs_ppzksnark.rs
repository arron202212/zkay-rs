//  Declaration of interfaces for a ppzkSNARK for USCS.

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

//  The implementation instantiates the protocol of \[DFGK14], by following
//  extending, and optimizing the approach described in \[BCTV14].

//  Acronyms:

//  - "ppzkSNARK" = "Pre-Processing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
//  - "USCS" = "Unitary-Square Constraint Systems"

//  References:

//  \[BCTV14]:
//  "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
//  Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
//  USENIX Security 2014,
//  <http://eprint.iacr.org/2013/879>

//  \[DFGK14]:
//  "Square Span Programs with Applications to Succinct NIZK Arguments"
//  George Danezis, Cedric Fournet, Jens Groth, Markulf Kohlweiss,
//  ASIACRYPT 2014,
//  <http://eprint.iacr.org/2014/718>

// use ff_curves::algebra::curves::public_params;

// use crate::common::data_structures::accumulation_vector;
// use crate::knowledge_commitment::knowledge_commitment;
// use crate::relations::constraint_satisfaction_problems/uscs/uscs;
// use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/uscs_ppzksnark_params;
use crate::common::data_structures::accumulation_vector::accumulation_vector;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::reductions::uscs_to_ssp::uscs_to_ssp::{
    uscs_to_ssp_instance_map_with_evaluation, uscs_to_ssp_witness_map,
};
use crate::zk_proof_systems::ppzksnark::uscs_ppzksnark::uscs_ppzksnark_params::{
    uscs_ppzksnark_auxiliary_input, uscs_ppzksnark_constraint_system, uscs_ppzksnark_primary_input,
};
use ff_curves::Fr;
use ff_curves::{G1, G1_precomp, G1_vector, G2, G2_precomp, G2_vector, GT};
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::scalar_multiplication::multiexp::{
    batch_exp, batch_exp_with_coeff, batch_to_special, get_exp_window_size, get_window_table,
    inhibit_profiling_info, multi_exp, multi_exp_method, multi_exp_with_mixed_addition,
};
use ffec::{One, PpConfig, Zero};
use std::ops::Mul;

// pub struct uscs_ppzksnark_proving_key;

// std::ostream& operator<<(std::ostream &out, pk:&uscs_ppzksnark_proving_key<ppT>);

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the USCS ppzkSNARK.
 */
#[derive(Default, Clone)]
pub struct uscs_ppzksnark_proving_key<ppT: ppTConfig> {
    pub V_g1_query: G1_vector<ppT>,
    pub alpha_V_g1_query: G1_vector<ppT>,
    pub H_g1_query: G1_vector<ppT>,
    pub V_g2_query: G2_vector<ppT>,

    pub constraint_system: uscs_ppzksnark_constraint_system<ppT>,
}
impl<ppT: ppTConfig> uscs_ppzksnark_proving_key<ppT> {
    // uscs_ppzksnark_proving_key() {};
    // uscs_ppzksnark_proving_key<ppT>& operator=(other:&uscs_ppzksnark_proving_key<ppT>) = default;
    // uscs_ppzksnark_proving_key(other:&uscs_ppzksnark_proving_key<ppT>) = default;
    // uscs_ppzksnark_proving_key(uscs_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(
        V_g1_query: G1_vector<ppT>,
        alpha_V_g1_query: G1_vector<ppT>,
        H_g1_query: G1_vector<ppT>,
        V_g2_query: G2_vector<ppT>,
        constraint_system: uscs_ppzksnark_constraint_system<ppT>,
    ) -> Self {
        Self {
            V_g1_query,
            alpha_V_g1_query,
            H_g1_query,
            V_g2_query,
            constraint_system,
        }
    }

    pub fn G1_size(&self) -> usize {
        self.V_g1_query.len() + self.alpha_V_g1_query.len() + self.H_g1_query.len()
    }

    pub fn G2_size(&self) -> usize {
        self.V_g2_query.len()
    }

    pub fn G1_sparse_size(&self) -> usize {
        self.G1_size()
    }

    pub fn G2_sparse_size(&self) -> usize {
        self.G2_size()
    }

    pub fn size_in_bits(&self) -> usize {
        G1::<ppT>::size_in_bits() * self.G1_size() + G2::<ppT>::size_in_bits() * self.G2_size()
    }

    pub fn print_size(&self) {
        print_indent();
        print!("* G1 elements in PK: {}\n", self.G1_size());
        print_indent();
        print!("* Non-zero G1 elements in PK: {}\n", self.G1_sparse_size());
        print_indent();
        print!("* G2 elements in PK: {}\n", self.G2_size());
        print_indent();
        print!("* Non-zero G2 elements in PK: {}\n", self.G2_sparse_size());
        print_indent();
        print!("* PK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(other:&uscs_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pk:&uscs_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_proving_key<ppT> &pk);
}

// pub struct uscs_ppzksnark_verification_key;

// std::ostream& operator<<(std::ostream &out, vk:&uscs_ppzksnark_verification_key<ppT>);

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_verification_key<ppT> &vk);

/**
 * A verification key for the USCS ppzkSNARK.
 */

#[derive(Default, Clone)]
pub struct uscs_ppzksnark_verification_key<ppT: ppTConfig> {
    pub tilde_g2: G2<ppT>,
    pub alpha_tilde_g2: G2<ppT>,
    pub Z_g2: G2<ppT>,

    pub encoded_IC_query: accumulation_vector<G1<ppT>>,
}
impl<ppT: ppTConfig> uscs_ppzksnark_verification_key<ppT> {
    // uscs_ppzksnark_verification_key() = default;
    pub fn new(
        tilde_g2: G2<ppT>,
        alpha_tilde_g2: G2<ppT>,
        Z_g2: G2<ppT>,
        eIC: accumulation_vector<G1<ppT>>,
    ) -> Self {
        Self {
            tilde_g2,
            alpha_tilde_g2,
            Z_g2,
            encoded_IC_query: eIC,
        }
    }

    pub fn G1_size(&self) -> usize {
        self.encoded_IC_query.len()
    }

    pub fn G2_size(&self) -> usize {
        3
    }

    pub fn size_in_bits(&self) -> usize {
        self.encoded_IC_query.size_in_bits() + 3 * G2::<ppT>::size_in_bits()
    }

    pub fn print_size(&self) {
        print_indent();
        print!("* G1 elements in VK: {}\n", self.G1_size());
        print_indent();
        print!("* G2 elements in VK: {}\n", self.G2_size());
        print_indent();
        print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(other:&uscs_ppzksnark_verification_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, vk:&uscs_ppzksnark_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_verification_key<ppT> &vk);

    // static uscs_ppzksnark_verification_key<ppT> dummy_verification_key(input_size:usize);
    pub fn dummy_verification_key(input_size: usize) -> Self {
        let mut result = uscs_ppzksnark_verification_key::<ppT>::default();
        result.tilde_g2 = Fr::<ppT>::random_element() * G2::<ppT>::one();
        result.alpha_tilde_g2 = Fr::<ppT>::random_element() * G2::<ppT>::one();
        result.Z_g2 = Fr::<ppT>::random_element() * G2::<ppT>::one();

        let mut base = Fr::<ppT>::random_element() * G1::<ppT>::one();
        let mut v = vec![];
        for i in 0..input_size {
            v.push(Fr::<ppT>::random_element() * G1::<ppT>::one());
        }

        result.encoded_IC_query = accumulation_vector::<G1<ppT>>::from(v);

        result
    }
}

// pub struct uscs_ppzksnark_processed_verification_key;

// std::ostream& operator<<(std::ostream &out, pvk:&uscs_ppzksnark_processed_verification_key<ppT>);

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_processed_verification_key<ppT> &pvk);

/**
 * A processed verification key for the USCS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
#[derive(Default, Clone)]
pub struct uscs_ppzksnark_processed_verification_key<ppT: ppTConfig> {
    pub pp_G1_one_precomp: G1_precomp<ppT>,
    pub pp_G2_one_precomp: G2_precomp<ppT>,
    pub vk_tilde_g2_precomp: G2_precomp<ppT>,
    pub vk_alpha_tilde_g2_precomp: G2_precomp<ppT>,
    pub vk_Z_g2_precomp: G2_precomp<ppT>,
    pub pairing_of_g1_and_g2: GT<ppT>,

    pub encoded_IC_query: accumulation_vector<G1<ppT>>,
    // bool operator==(other:&uscs_ppzksnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pvk:&uscs_ppzksnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_processed_verification_key<ppT> &pvk);
}

/**
 * A key pair for the USCS ppzkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Default, Clone)]
pub struct uscs_ppzksnark_keypair<ppT: ppTConfig> {
    pub pk: uscs_ppzksnark_proving_key<ppT>,
    pub vk: uscs_ppzksnark_verification_key<ppT>,
}
impl<ppT: ppTConfig> uscs_ppzksnark_keypair<ppT> {
    // uscs_ppzksnark_keypair() {};
    pub fn new(
        pk: uscs_ppzksnark_proving_key<ppT>,
        vk: uscs_ppzksnark_verification_key<ppT>,
    ) -> Self {
        Self { pk, vk }
    }

    // uscs_ppzksnark_keypair(uscs_ppzksnark_keypair<ppT> &&other) = default;
}

// pub struct uscs_ppzksnark_proof;

// std::ostream& operator<<(std::ostream &out, proof:&uscs_ppzksnark_proof<ppT>);

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_proof<ppT> &proof);

/**
 * A proof for the USCS ppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
#[derive(Clone)]
pub struct uscs_ppzksnark_proof<ppT: ppTConfig> {
    pub V_g1: G1<ppT>,
    pub alpha_V_g1: G1<ppT>,
    pub H_g1: G1<ppT>,
    pub V_g2: G2<ppT>,
}
impl<ppT: ppTConfig> Default for uscs_ppzksnark_proof<ppT> {
    fn default() -> Self {
        // invalid proof with valid curve points
        Self {
            V_g1: G1::<ppT>::one(),
            alpha_V_g1: G1::<ppT>::one(),
            H_g1: G1::<ppT>::one(),
            V_g2: G2::<ppT>::one(),
        }
    }
}

impl<ppT: ppTConfig> uscs_ppzksnark_proof<ppT> {
    pub fn new(V_g1: G1<ppT>, alpha_V_g1: G1<ppT>, H_g1: G1<ppT>, V_g2: G2<ppT>) -> Self {
        Self {
            V_g1,
            alpha_V_g1,
            H_g1,
            V_g2,
        }
    }

    pub fn G1_size(&self) -> usize {
        3
    }

    pub fn G2_size(&self) -> usize {
        1
    }

    pub fn size_in_bits(&self) -> usize {
        self.G1_size() * G1::<ppT>::size_in_bits() + self.G2_size() * G2::<ppT>::size_in_bits()
    }

    pub fn print_size(&self) {
        print_indent();
        print!("* G1 elements in proof: {}\n", self.G1_size());
        print_indent();
        print!("* G2 elements in proof: {}\n", self.G2_size());
        print_indent();
        print!("* Proof size in bits: {}\n", self.size_in_bits());
    }

    pub fn is_well_formed(&self) -> bool {
        self.V_g1.is_well_formed()
            && self.alpha_V_g1.is_well_formed()
            && self.H_g1.is_well_formed()
            && self.V_g2.is_well_formed()
    }

    // bool operator==(other:&uscs_ppzksnark_proof<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, proof:&uscs_ppzksnark_proof<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_proof<ppT> &proof);
}

/**
 * A generator algorithm for the USCS ppzkSNARK.
 *
 * Given a USCS constraint system CS, this algorithm produces proving and verification keys for CS.
 */

// uscs_ppzksnark_keypair<ppT> uscs_ppzksnark_generator(cs:&uscs_ppzksnark_constraint_system<ppT>);

/**
 * A prover algorithm for the USCS ppzkSNARK.
 *
 * Given a USCS primary input X and a USCS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the USCS constraint system that was given as input to the generator algorithm.
 */

// uscs_ppzksnark_proof<ppT> uscs_ppzksnark_prover(pk:&uscs_ppzksnark_proving_key<ppT>,
//                                                 primary_input:&uscs_ppzksnark_primary_input<ppT>,
//                                                 auxiliary_input:&uscs_ppzksnark_auxiliary_input<ppT>);

/*
Below are four variants of verifier algorithm for the USCS ppzkSNARK.

These are the four cases that arise from the following two choices:

(1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
    In the latter case, we call the algorithm an "online verifier".

(2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
    Strong input consistency requires that |primary_input| = CS.num_inputs, whereas
    weak input consistency requires that |primary_input| <= CS.num_inputs (and
    the primary input is implicitly padded with zeros up to length CS.num_inputs).
*/

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */

// bool uscs_ppzksnark_verifier_weak_IC(vk:&uscs_ppzksnark_verification_key<ppT>,
//                                      primary_input:&uscs_ppzksnark_primary_input<ppT>,
//                                      proof:&uscs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

// bool uscs_ppzksnark_verifier_strong_IC(vk:&uscs_ppzksnark_verification_key<ppT>,
//                                        primary_input:&uscs_ppzksnark_primary_input<ppT>,
//                                        proof:&uscs_ppzksnark_proof<ppT>);

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

// uscs_ppzksnark_processed_verification_key<ppT> uscs_ppzksnark_verifier_process_vk(vk:&uscs_ppzksnark_verification_key<ppT>);

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

// bool uscs_ppzksnark_online_verifier_weak_IC(pvk:&uscs_ppzksnark_processed_verification_key<ppT>,
//                                             primary_input:&uscs_ppzksnark_primary_input<ppT>,
//                                             proof:&uscs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

// bool uscs_ppzksnark_online_verifier_strong_IC(pvk:&uscs_ppzksnark_processed_verification_key<ppT>,
//                                               primary_input:&uscs_ppzksnark_primary_input<ppT>,
//                                               proof:&uscs_ppzksnark_proof<ppT>);

//  use algebra::scalar_multiplication::multiexp;
// use common::profiling;
// use common::utils;

// #ifdef MULTICORE
// use  <omp.h>

// use libsnark/reductions/uscs_to_ssp/uscs_to_ssp;
// use crate::relations::arithmetic_programs::ssp::ssp;

// bool uscs_ppzksnark_proving_key<ppT>::operator==(other:&uscs_ppzksnark_proving_key<ppT>) const
// {
//     return (self.V_g1_query == other.V_g1_query &&
//             self.alpha_V_g1_query == other.alpha_V_g1_query &&
//             self.H_g1_query == other.H_g1_query &&
//             self.V_g2_query == other.V_g2_query &&
//             self.constraint_system == other.constraint_system);
// }

// std::ostream& operator<<(std::ostream &out, pk:&uscs_ppzksnark_proving_key<ppT>)
// {
//     out << pk.V_g1_query;
//     out << pk.alpha_V_g1_query;
//     out << pk.H_g1_query;
//     out << pk.V_g2_query;
//     out << pk.constraint_system;

//     return out;
// }

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.V_g1_query;
//     in >> pk.alpha_V_g1_query;
//     in >> pk.H_g1_query;
//     in >> pk.V_g2_query;
//     in >> pk.constraint_system;

//     return in;
// }

// bool uscs_ppzksnark_verification_key<ppT>::operator==(other:&uscs_ppzksnark_verification_key<ppT>) const
// {
//     return (self.tilde_g2 == other.tilde_g2 &&
//             self.alpha_tilde_g2 == other.alpha_tilde_g2 &&
//             self.Z_g2 == other.Z_g2 &&
//             self.encoded_IC_query == other.encoded_IC_query);
// }

// std::ostream& operator<<(std::ostream &out, vk:&uscs_ppzksnark_verification_key<ppT>)
// {
//     out << vk.tilde_g2 << OUTPUT_NEWLINE;
//     out << vk.alpha_tilde_g2 << OUTPUT_NEWLINE;
//     out << vk.Z_g2 << OUTPUT_NEWLINE;
//     out << vk.encoded_IC_query << OUTPUT_NEWLINE;

//     return out;
// }

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.tilde_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.alpha_tilde_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.Z_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.encoded_IC_query;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

// bool uscs_ppzksnark_processed_verification_key<ppT>::operator==(other:&uscs_ppzksnark_processed_verification_key<ppT>) const
// {
//     return (self.pp_G1_one_precomp == other.pp_G1_one_precomp &&
//             self.pp_G2_one_precomp == other.pp_G2_one_precomp &&
//             self.vk_tilde_g2_precomp == other.vk_tilde_g2_precomp &&
//             self.vk_alpha_tilde_g2_precomp == other.vk_alpha_tilde_g2_precomp &&
//             self.vk_Z_g2_precomp == other.vk_Z_g2_precomp &&
//             self.pairing_of_g1_and_g2 == other.pairing_of_g1_and_g2 &&
//             self.encoded_IC_query == other.encoded_IC_query);
// }

// std::ostream& operator<<(std::ostream &out, pvk:&uscs_ppzksnark_processed_verification_key<ppT>)
// {
//     out << pvk.pp_G1_one_precomp << OUTPUT_NEWLINE;
//     out << pvk.pp_G2_one_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_tilde_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_alpha_tilde_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_Z_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.pairing_of_g1_and_g2 << OUTPUT_NEWLINE;
//     out << pvk.encoded_IC_query << OUTPUT_NEWLINE;

//     return out;
// }

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.pp_G1_one_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.pp_G2_one_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_tilde_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alpha_tilde_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_Z_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.pairing_of_g1_and_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.encoded_IC_query;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

// bool uscs_ppzksnark_proof<ppT>::operator==(other:&uscs_ppzksnark_proof<ppT>) const
// {
//     return (self.V_g1 == other.V_g1 &&
//             self.alpha_V_g1 == other.alpha_V_g1 &&
//             self.H_g1 == other.H_g1 &&
//             self.V_g2 == other.V_g2);
// }

// std::ostream& operator<<(std::ostream &out, proof:&uscs_ppzksnark_proof<ppT>)
// {
//     out << proof.V_g1 << OUTPUT_NEWLINE;
//     out << proof.alpha_V_g1 << OUTPUT_NEWLINE;
//     out << proof.H_g1 << OUTPUT_NEWLINE;
//     out << proof.V_g2 << OUTPUT_NEWLINE;

//     return out;
// }

// std::istream& operator>>(std::istream &in, uscs_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.V_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.alpha_V_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.H_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.V_g2;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

pub fn uscs_ppzksnark_generator<ppT: ppTConfig>(
    cs: &uscs_ppzksnark_constraint_system<ppT>,
) -> uscs_ppzksnark_keypair<ppT> {
    enter_block("Call to uscs_ppzksnark_generator", false);

    /* draw random element at which the SSP is evaluated */

    let t = Fr::<ppT>::random_element();

    /* perform USCS-to-SSP reduction */

    let mut ssp_inst = uscs_to_ssp_instance_map_with_evaluation(cs, &t);

    print_indent();
    print!("* SSP number of variables: {}\n", ssp_inst.num_variables());
    print_indent();
    print!("* SSP pre degree: {}\n", cs.num_constraints());
    print_indent();
    print!("* SSP degree: {}\n", ssp_inst.degree());
    print_indent();
    print!(
        "* SSP number of input variables: {}\n",
        ssp_inst.num_inputs()
    );

    /* construct various tables of FieldT elements */

    let mut Vt_table = (ssp_inst.Vt.clone()); // ssp_inst.Vt is now in unspecified state, but we do not use it later
    let mut Ht_table = (ssp_inst.Ht.clone()); // ssp_inst.Ht is now in unspecified state, but we do not use it later

    Vt_table.push(ssp_inst.Zt.clone());

    let mut Xt_table = Vt_table[..ssp_inst.num_inputs() + 1].to_vec();
    let mut Vt_table_minus_Xt_table = Vt_table[ssp_inst.num_inputs() + 1..].to_vec();

    /* sanity checks */

    assert!(Vt_table.len() == ssp_inst.num_variables() + 2);
    print!(
        "Ht_table.len() = {}, ssp_inst.degree() + 1 = {}\n",
        Ht_table.len(),
        ssp_inst.degree() + 1
    );
    assert!(Ht_table.len() == ssp_inst.degree() + 1);
    assert!(Xt_table.len() == ssp_inst.num_inputs() + 1);
    assert!(
        Vt_table_minus_Xt_table.len() == ssp_inst.num_variables() + 2 - ssp_inst.num_inputs() - 1
    );
    for i in 0..ssp_inst.num_inputs() + 1 {
        assert!(!Xt_table[i].is_zero());
    }

    let alpha = Fr::<ppT>::random_element();

    enter_block("Generate USCS proving key", false);

    let g1_exp_count = Vt_table.len() + Vt_table_minus_Xt_table.len() + Ht_table.len();
    let g2_exp_count = Vt_table_minus_Xt_table.len();

    let g1_window = get_exp_window_size::<G1<ppT>>(g1_exp_count);
    let g2_window = get_exp_window_size::<G2<ppT>>(g2_exp_count);

    print_indent();
    print!("* G1 window: {}\n", g1_window);
    print_indent();
    print!("* G2 window: {}\n", g2_window);

    enter_block("Generating G1 multiexp table", false);
    let g1_table = get_window_table(Fr::<ppT>::size_in_bits(), g1_window, G1::<ppT>::one());
    leave_block("Generating G1 multiexp table", false);

    enter_block("Generating G2 multiexp table", false);
    let g2_table = get_window_table(Fr::<ppT>::size_in_bits(), g2_window, G2::<ppT>::one());
    leave_block("Generating G2 multiexp table", false);

    enter_block("Generate proof components", false);

    enter_block("Compute the query for V_g1", false);
    let mut V_g1_query = batch_exp(
        Fr::<ppT>::size_in_bits(),
        g1_window,
        &g1_table,
        &Vt_table_minus_Xt_table,
    );
    // #ifdef USE_MIXED_ADDITION
    batch_to_special::<G1<ppT>>(&mut V_g1_query);

    leave_block("Compute the query for V_g1", false);

    enter_block("Compute the query for alpha_V_g1", false);
    let mut alpha_V_g1_query = batch_exp_with_coeff(
        Fr::<ppT>::size_in_bits(),
        g1_window,
        &g1_table,
        &alpha,
        &Vt_table_minus_Xt_table,
    );
    // #ifdef USE_MIXED_ADDITION
    batch_to_special::<G1<ppT>>(&mut alpha_V_g1_query);

    leave_block("Compute the query for alpha_V_g1", false);

    enter_block("Compute the query for H_g1", false);
    let mut H_g1_query = batch_exp(Fr::<ppT>::size_in_bits(), g1_window, &g1_table, &Ht_table);
    // #ifdef USE_MIXED_ADDITION
    batch_to_special::<G1<ppT>>(&mut H_g1_query);

    leave_block("Compute the query for H_g1", false);

    enter_block("Compute the query for V_g2", false);
    let mut V_g2_query = batch_exp(Fr::<ppT>::size_in_bits(), g2_window, &g2_table, &Vt_table);
    // #ifdef USE_MIXED_ADDITION
    batch_to_special::<G2<ppT>>(&mut V_g2_query);

    leave_block("Compute the query for V_g2", false);

    leave_block("Generate proof components", false);

    leave_block("Generate USCS proving key", false);

    enter_block("Generate USCS verification key", false);

    let tilde = Fr::<ppT>::random_element();
    let tilde_g2 = tilde.clone() * G2::<ppT>::one();
    let alpha_tilde_g2 = (alpha * tilde) * G2::<ppT>::one();
    let Z_g2 = ssp_inst.Zt * G2::<ppT>::one();

    enter_block("Encode IC query for USCS verification key", false);
    let encoded_IC_base = Xt_table[0].clone() * G1::<ppT>::one();
    let encoded_IC_values = batch_exp(
        Fr::<ppT>::size_in_bits(),
        g1_window,
        &g1_table,
        &Xt_table[1..].to_vec(),
    );
    leave_block("Encode IC query for USCS verification key", false);

    leave_block("Generate USCS verification key", false);

    leave_block("Call to uscs_ppzksnark_generator", false);

    let encoded_IC_query =
        accumulation_vector::<G1<ppT>>::new_with_vec(encoded_IC_base, encoded_IC_values);

    let vk = uscs_ppzksnark_verification_key::<ppT>::new(
        tilde_g2,
        alpha_tilde_g2,
        Z_g2,
        encoded_IC_query,
    );

    let cs_copy = cs.clone();
    let pk = uscs_ppzksnark_proving_key::<ppT>::new(
        V_g1_query,
        alpha_V_g1_query,
        H_g1_query,
        V_g2_query,
        cs_copy,
    );

    pk.print_size();
    vk.print_size();

    uscs_ppzksnark_keypair::<ppT>::new(pk, vk)
}

pub fn uscs_ppzksnark_prover<ppT: ppTConfig>(
    pk: &uscs_ppzksnark_proving_key<ppT>,
    primary_input: &uscs_ppzksnark_primary_input<ppT>,
    auxiliary_input: &uscs_ppzksnark_auxiliary_input<ppT>,
) -> uscs_ppzksnark_proof<ppT> {
    enter_block("Call to uscs_ppzksnark_prover", false);

    let d = Fr::<ppT>::random_element();

    enter_block("Compute the polynomial H", false);
    let ssp_wit =
        uscs_to_ssp_witness_map(&pk.constraint_system, primary_input, auxiliary_input, &d);
    leave_block("Compute the polynomial H", false);

    /* sanity checks */
    assert!(
        pk.constraint_system
            .is_satisfied(primary_input, auxiliary_input)
    );
    assert!(pk.V_g1_query.len() == ssp_wit.num_variables() + 2 - ssp_wit.num_inputs() - 1);
    assert!(pk.alpha_V_g1_query.len() == ssp_wit.num_variables() + 2 - ssp_wit.num_inputs() - 1);
    assert!(pk.H_g1_query.len() == ssp_wit.degree() + 1);
    assert!(pk.V_g2_query.len() == ssp_wit.num_variables() + 2);

    // #ifdef DEBUG
    let t = Fr::<ppT>::random_element();
    let ssp_inst = uscs_to_ssp_instance_map_with_evaluation(&pk.constraint_system, &t);
    assert!(ssp_inst.is_satisfied(&ssp_wit));

    let mut V_g1 = ssp_wit.d.clone() * pk.V_g1_query[pk.V_g1_query.len() - 1].clone();
    let mut alpha_V_g1 =
        ssp_wit.d.clone() * pk.alpha_V_g1_query[pk.alpha_V_g1_query.len() - 1].clone();
    let mut H_g1 = G1::<ppT>::zero();
    let mut V_g2 = pk.V_g2_query[0].clone()
        + ssp_wit.d.clone() * pk.V_g2_query[pk.V_g2_query.len() - 1].clone();

    // #ifdef MULTICORE
    // override:usize chunks = omp_get_max_threads(); // to, set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;

    // MAYBE LATER: do queries 1,2,4 at once for slightly better speed

    enter_block("Compute the proof", false);

    enter_block("Compute V_g1, the 1st component of the proof", false);
    V_g1 = V_g1
        + multi_exp_with_mixed_addition::<
            G1<ppT>,
            Fr<ppT>,
            { multi_exp_method::multi_exp_method_BDLO12 },
        >(
            &pk.V_g1_query[..(ssp_wit.num_variables() - ssp_wit.num_inputs())],
            &ssp_wit.coefficients_for_Vs[ssp_wit.num_inputs()..ssp_wit.num_variables()],
            chunks,
        );
    leave_block("Compute V_g1, the 1st component of the proof", false);

    enter_block("Compute alpha_V_g1, the 2nd component of the proof", false);
    alpha_V_g1 = alpha_V_g1
        + multi_exp_with_mixed_addition::<
            G1<ppT>,
            Fr<ppT>,
            { multi_exp_method::multi_exp_method_BDLO12 },
        >(
            &pk.alpha_V_g1_query[..(ssp_wit.num_variables() - ssp_wit.num_inputs())],
            &ssp_wit.coefficients_for_Vs[ssp_wit.num_inputs()..ssp_wit.num_variables()],
            chunks,
        );
    leave_block("Compute alpha_V_g1, the 2nd component of the proof", false);

    enter_block("Compute H_g1, the 3rd component of the proof", false);
    H_g1 = H_g1
        + multi_exp::<G1<ppT>, Fr<ppT>, { multi_exp_method::multi_exp_method_BDLO12 }>(
            &pk.H_g1_query[..ssp_wit.degree() + 1],
            &ssp_wit.coefficients_for_H[..ssp_wit.degree() + 1],
            chunks,
        );
    leave_block("Compute H_g1, the 3rd component of the proof", false);

    enter_block("Compute V_g2, the 4th component of the proof", false);
    V_g2 = V_g2
        + multi_exp::<G2<ppT>, Fr<ppT>, { multi_exp_method::multi_exp_method_BDLO12 }>(
            &pk.V_g2_query[1..ssp_wit.num_variables() + 1],
            &ssp_wit.coefficients_for_Vs[..ssp_wit.num_variables()],
            chunks,
        );
    leave_block("Compute V_g2, the 4th component of the proof", false);

    leave_block("Compute the proof", false);

    leave_block("Call to uscs_ppzksnark_prover", false);

    let proof = uscs_ppzksnark_proof::<ppT>::new(V_g1, alpha_V_g1, H_g1, V_g2);

    proof.print_size();

    proof
}

pub fn uscs_ppzksnark_verifier_process_vk<ppT: ppTConfig>(
    vk: &uscs_ppzksnark_verification_key<ppT>,
) -> uscs_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to uscs_ppzksnark_verifier_process_vk", false);

    let mut pvk = uscs_ppzksnark_processed_verification_key::<ppT>::default();

    pvk.pp_G1_one_precomp = ppT::precompute_G1(&G1::<ppT>::one());
    pvk.pp_G2_one_precomp = ppT::precompute_G2(&G2::<ppT>::one());

    pvk.vk_tilde_g2_precomp = ppT::precompute_G2(&vk.tilde_g2);
    pvk.vk_alpha_tilde_g2_precomp = ppT::precompute_G2(&vk.alpha_tilde_g2);
    pvk.vk_Z_g2_precomp = ppT::precompute_G2(&vk.Z_g2);

    pvk.pairing_of_g1_and_g2 = ppT::miller_loop(&pvk.pp_G1_one_precomp, &pvk.pp_G2_one_precomp);

    pvk.encoded_IC_query = vk.encoded_IC_query.clone();

    leave_block("Call to uscs_ppzksnark_verifier_process_vk", false);

    pvk
}

pub fn uscs_ppzksnark_online_verifier_weak_IC<ppT: ppTConfig>(
    pvk: &uscs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &uscs_ppzksnark_primary_input<ppT>,
    proof: &uscs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to uscs_ppzksnark_online_verifier_weak_IC", false);
    assert!(pvk.encoded_IC_query.domain_size() >= primary_input.len());

    enter_block("Compute input-dependent part of V", false);
    let accumulated_IC = pvk
        .encoded_IC_query
        .accumulate_chunk::<Fr<ppT>>(primary_input, 0);
    assert!(accumulated_IC.is_fully_accumulated());
    let acc = accumulated_IC.first;
    leave_block("Compute input-dependent part of V", false);

    let mut result = true;

    enter_block("Check if the proof is well-formed", false);
    if !proof.is_well_formed() {
        if !inhibit_profiling_info {
            print_indent();
            print!("At least one of the proof components is not well-formed.\n");
        }
        result = false;
    }
    leave_block("Check if the proof is well-formed", false);

    enter_block("Online pairing computations", false);

    enter_block("Check knowledge commitment for V is valid", false);
    let proof_V_g1_with_acc_precomp = ppT::precompute_G1(&(proof.V_g1.clone() + acc.clone()));
    let proof_V_g2_precomp = ppT::precompute_G2(&proof.V_g2);
    let V_1 = ppT::miller_loop(&proof_V_g1_with_acc_precomp, &pvk.pp_G2_one_precomp);
    let V_2 = ppT::miller_loop(&pvk.pp_G1_one_precomp, &proof_V_g2_precomp);
    let V = ppT::final_exponentiation(&(V_1 * V_2.unitary_inverse()));
    if V != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Knowledge commitment for V invalid.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for V is valid", false);

    enter_block("Check SSP divisibility", false); // i.e., check that V^2=H*Z+1
    let proof_H_g1_precomp = ppT::precompute_G1(&proof.H_g1);
    let SSP_1 = ppT::miller_loop(&proof_V_g1_with_acc_precomp, &proof_V_g2_precomp);
    let SSP_2 = ppT::miller_loop(&proof_H_g1_precomp, &pvk.vk_Z_g2_precomp);
    let SSP = ppT::final_exponentiation(
        &(SSP_1.unitary_inverse() * SSP_2.clone() * pvk.pairing_of_g1_and_g2.clone()),
    );
    if SSP != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("SSP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check SSP divisibility", false);

    enter_block("Check same coefficients were used", false);
    let proof_V_g1_precomp = ppT::precompute_G1(&proof.V_g1);
    let proof_alpha_V_g1_precomp = ppT::precompute_G1(&proof.alpha_V_g1);
    let alpha_V_1 = ppT::miller_loop(&proof_V_g1_precomp, &pvk.vk_alpha_tilde_g2_precomp);
    let alpha_V_2 = ppT::miller_loop(&proof_alpha_V_g1_precomp, &pvk.vk_tilde_g2_precomp);
    let alpha_V = ppT::final_exponentiation(&(alpha_V_1 * alpha_V_2.unitary_inverse()));
    if alpha_V != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    leave_block("Check same coefficients were used", false);

    leave_block("Online pairing computations", false);

    leave_block("Call to uscs_ppzksnark_online_verifier_weak_IC", false);

    result
}

pub fn uscs_ppzksnark_verifier_weak_IC<ppT: ppTConfig>(
    vk: &uscs_ppzksnark_verification_key<ppT>,
    primary_input: &uscs_ppzksnark_primary_input<ppT>,
    proof: &uscs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to uscs_ppzksnark_verifier_weak_IC", false);
    let pvk = uscs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let result = uscs_ppzksnark_online_verifier_weak_IC::<ppT>(&pvk, primary_input, proof);
    leave_block("Call to uscs_ppzksnark_verifier_weak_IC", false);
    result
}

pub fn uscs_ppzksnark_online_verifier_strong_IC<ppT: ppTConfig>(
    pvk: &uscs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &uscs_ppzksnark_primary_input<ppT>,
    proof: &uscs_ppzksnark_proof<ppT>,
) -> bool {
    let mut result = true;
    enter_block("Call to uscs_ppzksnark_online_verifier_strong_IC", false);

    if pvk.encoded_IC_query.domain_size() != primary_input.len() {
        print_indent();
        print!(
            "Input length differs from expected (got {}, expected {}).\n",
            primary_input.len(),
            pvk.encoded_IC_query.domain_size()
        );
        result = false;
    } else {
        result = uscs_ppzksnark_online_verifier_weak_IC(pvk, primary_input, proof);
    }

    leave_block("Call to uscs_ppzksnark_online_verifier_strong_IC", false);
    result
}

pub fn uscs_ppzksnark_verifier_strong_IC<ppT: ppTConfig>(
    vk: &uscs_ppzksnark_verification_key<ppT>,
    primary_input: &uscs_ppzksnark_primary_input<ppT>,
    proof: &uscs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to uscs_ppzksnark_verifier_strong_IC", false);
    let pvk = uscs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let result = uscs_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, primary_input, proof);
    leave_block("Call to uscs_ppzksnark_verifier_strong_IC", false);
    result
}
