// Declaration of interfaces for a SEppzkSNARK for R1CS.

// This includes:
// - pub struct for proving key
// - pub struct for verification key
// - pub struct for processed verification key
// - pub struct for key pair (proving key & verification key)
// - pub struct for proof
// - generator algorithm
// - prover algorithm
// - verifier algorithm (with strong or weak input consistency)
// - online verifier algorithm (with strong or weak input consistency)

// The implementation instantiates (a modification of) the protocol of \[GM17],
// by following extending, and optimizing the approach described in \[BCTV14].

// Acronyms:

// - R1CS = "Rank-1 Constraint Systems"
// - SEppzkSNARK = "Simulation-Extractable PreProcessing Zero-Knowledge Succinct
//     Non-interactive ARgument of Knowledge"

// References:

// \[BCTV14]:
// "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
// Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
// USENIX Security 2014,
// <http://eprint.iacr.org/2013/879>

// \[GM17]:
// "Snarky Signatures: Minimal Signatures of Knowledge from
//  Simulation-Extractable SNARKs",
// Jens Groth and Mary Maller,
// IACR-CRYPTO-2017,
// <https://eprint.iacr.org/2017/540>

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::reductions::r1cs_to_sap::r1cs_to_sap::r1cs_to_sap_get_domain;
use crate::reductions::r1cs_to_sap::r1cs_to_sap::r1cs_to_sap_instance_map_with_evaluation;
use crate::reductions::r1cs_to_sap::r1cs_to_sap::r1cs_to_sap_witness_map;
use crate::relations::arithmetic_programs::sap::sap::sap_instance_evaluation;
use crate::zk_proof_systems::ppzksnark::{
    KeyPairTConfig, ProofTConfig, ProvingKeyTConfig, VerificationKeyTConfig,
};
use ff_curves::PublicParams;
use ff_curves::{Fqk, Fr, Fr_vector, G1, G1_precomp, G1_vector, G2, G2_precomp, G2_vector};
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::OUTPUT_NEWLINE;
use ffec::scalar_multiplication::multiexp::KCConfig;
use ffec::scalar_multiplication::multiexp::{
    batch_exp, get_exp_window_size, get_window_table, inhibit_profiling_info, multi_exp,
    multi_exp_method,
};
use ffec::{One, Zero};
use fqfft::evaluation_domain::evaluation_domain::{EvaluationDomainConfig, evaluation_domain};
use rccell::RcCell;
use std::ops::{Add, Mul};
// use crate::common::data_structures::accumulation_vector;
// use crate::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint_system, r1cs_primary_input,
};
use crate::zk_proof_systems::ppzksnark::r1cs_se_ppzksnark::r1cs_se_ppzksnark_params::{
    r1cs_se_ppzksnark_auxiliary_input, r1cs_se_ppzksnark_constraint_system,
    r1cs_se_ppzksnark_primary_input,
};

//

//
/**
 * A proving key for the R1CS SEppzkSNARK.
 */
#[derive(Default, Clone)]
pub struct r1cs_se_ppzksnark_proving_key<ppT: PublicParams> {
    // G^{gamma * A_i(t)} for 0 <= i <= sap.num_variables()
    pub A_query: G1_vector<ppT>,

    // H^{gamma * A_i(t)} for 0 <= i <= sap.num_variables()
    pub B_query: G2_vector<ppT>,

    // G^{gamma^2 * C_i(t) + (alpha + beta) * gamma * A_i(t)}
    // for sap.num_inputs() + 1 < i <= sap.num_variables()
    pub C_query_1: G1_vector<ppT>,

    // G^{2 * gamma^2 * Z(t) * A_i(t)} for 0 <= i <= sap.num_variables()
    pub C_query_2: G1_vector<ppT>,

    // G^{gamma * Z(t)}
    pub G_gamma_Z: G1<ppT>,

    // H^{gamma * Z(t)}
    pub H_gamma_Z: G2<ppT>,

    // G^{(alpha + beta) * gamma * Z(t)}
    pub G_ab_gamma_Z: G1<ppT>,

    // G^{gamma^2 * Z(t)^2}
    pub G_gamma2_Z2: G1<ppT>,

    // G^{gamma^2 * Z(t) * t^i} for 0 <= i < sap.degree
    pub G_gamma2_Z_t: G1_vector<ppT>,

    pub constraint_system: r1cs_se_ppzksnark_constraint_system<ppT>,
}

impl<ppT: PublicParams> ProvingKeyTConfig for r1cs_se_ppzksnark_proving_key<ppT> {}

impl<ppT: PublicParams> r1cs_se_ppzksnark_proving_key<ppT> {
    // r1cs_se_ppzksnark_proving_key() {};
    // r1cs_se_ppzksnark_proving_key<ppT>& operator=(other:&r1cs_se_ppzksnark_proving_key<ppT>) = default;
    // r1cs_se_ppzksnark_proving_key(other:&r1cs_se_ppzksnark_proving_key<ppT>) = default;
    // r1cs_se_ppzksnark_proving_key(r1cs_se_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(
        A_query: G1_vector<ppT>,
        B_query: G2_vector<ppT>,
        C_query_1: G1_vector<ppT>,
        C_query_2: G1_vector<ppT>,
        G_gamma_Z: G1<ppT>,
        H_gamma_Z: G2<ppT>,
        G_ab_gamma_Z: G1<ppT>,
        G_gamma2_Z2: G1<ppT>,
        G_gamma2_Z_t: G1_vector<ppT>,
        constraint_system: r1cs_se_ppzksnark_constraint_system<ppT>,
    ) -> Self {
        Self {
            A_query,
            B_query,
            C_query_1,
            C_query_2,
            G_gamma_Z,
            H_gamma_Z,
            G_ab_gamma_Z,
            G_gamma2_Z2,
            G_gamma2_Z_t,
            constraint_system,
        }
    }

    pub fn G1_size(&self) -> usize {
        self.A_query.len()
            + self.C_query_1.len()
            + self.C_query_2.len()
            + 3
            + self.G_gamma2_Z_t.len()
    }

    pub fn G2_size(&self) -> usize {
        self.B_query.len() + 1
    }

    pub fn size_in_bits(&self) -> usize {
        self.G1_size() * ppT::G1::size_in_bits() + self.G2_size() * ppT::G2::size_in_bits()
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in PK: {}\n", self.G1_size());
        print_indent();
        print!("* G2 elements in PK: {}\n", self.G2_size());
        print_indent();
        print!("* PK size in bits: {}\n", self.size_in_bits());
    }
}

/**
 * A verification key for the R1CS SEppzkSNARK.
 */
#[derive(Default, Clone)]
pub struct r1cs_se_ppzksnark_verification_key<ppT: PublicParams> {
    // H
    pub H: G2<ppT>,

    // G^{alpha}
    pub G_alpha: G1<ppT>,

    // H^{beta}
    pub H_beta: G2<ppT>,

    // G^{gamma}
    pub G_gamma: G1<ppT>,

    // H^{gamma}
    pub H_gamma: G2<ppT>,

    // G^{gamma * A_i(t) + (alpha + beta) * A_i(t)}
    // for 0 <= i <= sap.num_inputs()
    pub query: G1_vector<ppT>,
}
impl<ppT: PublicParams> VerificationKeyTConfig for r1cs_se_ppzksnark_verification_key<ppT> {}

impl<ppT: PublicParams> r1cs_se_ppzksnark_verification_key<ppT> {
    // r1cs_se_ppzksnark_verification_key() = default;
    pub fn new(
        H: G2<ppT>,
        G_alpha: G1<ppT>,
        H_beta: G2<ppT>,
        G_gamma: G1<ppT>,
        H_gamma: G2<ppT>,
        query: G1_vector<ppT>,
    ) -> Self {
        Self {
            H,
            G_alpha,
            H_beta,
            G_gamma,
            H_gamma,
            query,
        }
    }

    pub fn G1_size(&self) -> usize {
        2 + self.query.len()
    }

    pub fn G2_size(&self) -> usize {
        3
    }

    pub fn size_in_bits(&self) -> usize {
        (self.G1_size() * ppT::G1::size_in_bits() + self.G2_size() * ppT::G2::size_in_bits())
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in VK: {}\n", self.G1_size());
        print_indent();
        print!("* G2 elements in VK: {}\n", self.G2_size());
        print_indent();
        print!("* VK size in bits: {}\n", self.size_in_bits());
    }
}

/**
 * A processed verification key for the R1CS SEppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
#[derive(Default, Clone)]
pub struct r1cs_se_ppzksnark_processed_verification_key<ppT: PublicParams> {
    pub G_alpha: G1<ppT>,
    pub H_beta: G2<ppT>,
    pub G_alpha_H_beta_ml: Fqk<ppT>,
    pub G_gamma_pc: G1_precomp<ppT>,
    pub H_gamma_pc: G2_precomp<ppT>,
    pub H_pc: G2_precomp<ppT>,

    pub query: G1_vector<ppT>,
}

/**
 * A key pair for the R1CS SEppzkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Default, Clone)]
pub struct r1cs_se_ppzksnark_keypair<ppT: PublicParams> {
    pub pk: r1cs_se_ppzksnark_proving_key<ppT>,
    pub vk: r1cs_se_ppzksnark_verification_key<ppT>,
}
impl<ppT: PublicParams> KeyPairTConfig for r1cs_se_ppzksnark_keypair<ppT> {
    type PK = r1cs_se_ppzksnark_proving_key<ppT>;
    type VK = r1cs_se_ppzksnark_verification_key<ppT>;
    fn vk(&self) -> &Self::VK {
        &self.vk
    }
    fn pk(&self) -> &Self::PK {
        &self.pk
    }
}
impl<ppT: PublicParams> r1cs_se_ppzksnark_keypair<ppT> {
    // r1cs_se_ppzksnark_keypair() = default;
    // r1cs_se_ppzksnark_keypair(other:&r1cs_se_ppzksnark_keypair<ppT>) = default;
    pub fn new(
        pk: r1cs_se_ppzksnark_proving_key<ppT>,
        vk: r1cs_se_ppzksnark_verification_key<ppT>,
    ) -> Self {
        Self { pk, vk }
    }

    // r1cs_se_ppzksnark_keypair(r1cs_se_ppzksnark_keypair<ppT> &&other) = default;
}

/**
 * A proof for the R1CS SEppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
#[derive(Default, Clone)]
pub struct r1cs_se_ppzksnark_proof<ppT: PublicParams> {
    pub A: G1<ppT>,
    pub B: G2<ppT>,
    pub C: G1<ppT>,
}
impl<ppT: PublicParams> ProofTConfig for r1cs_se_ppzksnark_proof<ppT> {}
impl<ppT: PublicParams> r1cs_se_ppzksnark_proof<ppT> {
    // r1cs_se_ppzksnark_proof()
    // {}
    pub fn new(A: G1<ppT>, B: G2<ppT>, C: G1<ppT>) -> Self {
        Self { A, B, C }
    }

    pub fn G1_size() -> usize {
        2
    }

    pub fn G2_size() -> usize {
        1
    }

    pub fn size_in_bits() -> usize {
        Self::G1_size() * ppT::G1::size_in_bits() + Self::G2_size() * ppT::G2::size_in_bits()
    }

    fn print_size() {
        print_indent();
        print!("* G1 elements in proof: {}\n", Self::G1_size());
        print_indent();
        print!("* G2 elements in proof: {}\n", Self::G2_size());
        print_indent();
        print!("* Proof size in bits: {}\n", Self::size_in_bits());
    }

    fn is_well_formed(&self) -> bool {
        self.A.is_well_formed() && self.B.is_well_formed() && self.C.is_well_formed()
    }
}

/**
 * A generator algorithm for the R1CS SEppzkSNARK.
 *
 * Given a R1CS constraint system CS, this algorithm produces proving and verification keys for CS.
 */
// pub fn
// r1cs_se_ppzksnark_keypair<ppT> r1cs_se_ppzksnark_generator(cs:&r1cs_se_ppzksnark_constraint_system<ppT>);

/**
 * A prover algorithm for the R1CS SEppzkSNARK.
 *
 * Given a R1CS primary input X and a R1CS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the R1CS constraint system that was given as input to the generator algorithm.
 */
pub fn r1cs_se_ppzksnark_prover<ppT: PublicParams>(
    pk: &r1cs_se_ppzksnark_proving_key<ppT>,
    primary_input: &r1cs_se_ppzksnark_primary_input<ppT>,
    auxiliary_input: &r1cs_se_ppzksnark_auxiliary_input<ppT>,
) -> r1cs_se_ppzksnark_proof<ppT>
// where
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     ED: evaluation_domain<<ppT as PublicParams>::Fr>,
{
    enter_block("Call to r1cs_se_ppzksnark_prover", false);

    // // #ifdef DEBUG
    //     assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
    //

    let (d1, d2) = (ppT::Fr::random_element(), ppT::Fr::random_element());

    enter_block("Compute the polynomial H", false);
    let sap_wit = r1cs_to_sap_witness_map::<Fr<ppT>, pb_variable, pb_linear_combination>(
        &pk.constraint_system,
        &primary_input,
        &auxiliary_input,
        &d1,
        &d2,
    );
    leave_block("Compute the polynomial H", false);

    // // #ifdef DEBUG
    //     ppT::Fr::random_element(:Fr<ppT> t =);
    //     sap_instance_evaluation<Fr<ppT> > sap_inst = r1cs_to_sap_instance_map_with_evaluation(pk.constraint_system, t);
    //     assert!(sap_inst.is_satisfied(sap_wit));
    //

    // // #ifdef DEBUG
    //     assert!(pk.A_query.len() == sap_wit.num_variables() + 1);
    //     assert!(pk.B_query.len() == sap_wit.num_variables() + 1);
    //     assert!(pk.C_query_1.len() == sap_wit.num_variables() - sap_wit.num_inputs());
    //     assert!(pk.C_query_2.len() == sap_wit.num_variables() + 1);
    //     assert!(pk.G_gamma2_Z_t.len() >= sap_wit.degree() - 1);
    //

    // // #ifdef MULTICORE
    //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    //

    let r = ppT::Fr::random_element();

    enter_block("Compute the proof", false);

    enter_block("Compute answer to A-query", false);
    /**
     * compute A = G^{gamma * (\sum_{i=0}^m input_i * A_i(t) + r * Z(t))}
     *           = \prod_{i=0}^m (G^{gamma * A_i(t)})^{input_i)
     *             * (G^{gamma * Z(t)})^r
     *           = \prod_{i=0}^m A_query[i]^{input_i} * G_gamma_Z^r
     */
    let A = pk.G_gamma_Z.clone()*r.clone() +
        pk.A_query[0].clone() + // i = 0 is a special case because input_i = 1
        pk.G_gamma_Z.clone()* sap_wit.d1.clone()  + // ZK-patch
        multi_exp::<G1<ppT>,Fr<ppT>,
                         {multi_exp_method::multi_exp_method_BDLO12}>(
            &pk.A_query[1..],
            &sap_wit.coefficients_for_ACs,
            chunks);

    leave_block("Compute answer to A-query", false);

    enter_block("Compute answer to B-query", false);
    /**
     * compute B exactly as A, except with H as the base
     */
    let B = pk.H_gamma_Z.clone()*r.clone()  +
        pk.B_query[0].clone() + // i = 0 is a special case because input_i = 1
         pk.H_gamma_Z.clone()*sap_wit.d1.clone()  + // ZK-patch
        multi_exp::<G2<ppT>,Fr<ppT>,
                         {multi_exp_method::multi_exp_method_BDLO12}>(
            &pk.B_query[1..],
            &sap_wit.coefficients_for_ACs,
            chunks);
    leave_block("Compute answer to B-query", false);

    enter_block("Compute answer to C-query", false);
    /**
     * compute C = G^{f(input) +
     *                r^2 * gamma^2 * Z(t)^2 +
     *                r * (alpha + beta) * gamma * Z(t) +
     *                2 * r * gamma^2 * Z(t) * \sum_{i=0}^m input_i A_i(t) +
     *                gamma^2 * Z(t) * H(t)}
     * where G^{f(input)} = \prod_{i=l+1}^m C_query_1 * input_i
     * and G^{2 * r * gamma^2 * Z(t) * \sum_{i=0}^m input_i A_i(t)} =
     *              = \prod_{i=0}^m C_query_2 * input_i
     */
    let C = multi_exp::<G1<ppT>,Fr<ppT>,
                                        {multi_exp_method::multi_exp_method_BDLO12}>(
            &pk.C_query_1,
            &sap_wit.coefficients_for_ACs[sap_wit.num_inputs()..],
            chunks) +
         pk.G_gamma2_Z2.clone() *(r.clone() * r.clone()) +
         pk.G_ab_gamma_Z.clone()*r.clone()  +
         pk.G_ab_gamma_Z.clone()*sap_wit.d1.clone()  + // ZK-patch
         pk.C_query_2[0].clone()*r.clone()  + // i = 0 is a special case for C_query_2
        pk.G_gamma2_Z2.clone()* (r.clone() + r.clone()) * sap_wit.d1.clone()  + // ZK-patch for C_query_2
         multi_exp::<G1<ppT>,Fr<ppT>,
                             {multi_exp_method::multi_exp_method_BDLO12}>(
            &pk.C_query_2[1..],
            &sap_wit.coefficients_for_ACs,
            chunks) *r.clone() +
         pk.G_gamma2_Z_t[0].clone()*sap_wit.d2.clone()  + // ZK-patch
        multi_exp::<G1<ppT>,Fr<ppT>,
                         { multi_exp_method::multi_exp_method_BDLO12}>(
            &pk.G_gamma2_Z_t,
            &sap_wit.coefficients_for_H,
            chunks);
    leave_block("Compute answer to C-query", false);

    leave_block("Compute the proof", false);

    leave_block("Call to r1cs_se_ppzksnark_prover", false);

    let proof = r1cs_se_ppzksnark_proof::<ppT>::new(A, B, C);
    r1cs_se_ppzksnark_proof::<ppT>::print_size();

    proof
}

/*
Below are four variants of verifier algorithm for the R1CS SEppzkSNARK.

These are the four cases that arise from the following two choices:

(1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
    In the latter case, we call the algorithm an "online verifier".

(2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
    Strong input consistency requires that |primary_input| = CS.num_inputs, whereas
    weak input consistency requires that |primary_input| <= CS.num_inputs (and
    the primary input is implicitly padded with zeros up to length CS.num_inputs).
*/

/**
 * A verifier algorithm for the R1CS SEppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */
pub fn r1cs_se_ppzksnark_verifier_weak_IC<ppT: PublicParams>(
    vk: &r1cs_se_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_se_ppzksnark_primary_input<ppT>,
    proof: &r1cs_se_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_se_ppzksnark_verifier_weak_IC", false);
    let pvk = r1cs_se_ppzksnark_verifier_process_vk::<ppT>(&vk);
    let result = r1cs_se_ppzksnark_online_verifier_weak_IC::<ppT>(&pvk, &primary_input, &proof);
    leave_block("Call to r1cs_se_ppzksnark_verifier_weak_IC", false);
    result
}

/**
 * A verifier algorithm for the R1CS SEppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */
pub fn r1cs_se_ppzksnark_verifier_strong_IC<ppT: PublicParams>(
    vk: &r1cs_se_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_se_ppzksnark_primary_input<ppT>,
    proof: &r1cs_se_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_se_ppzksnark_verifier_strong_IC", false);
    let pvk = r1cs_se_ppzksnark_verifier_process_vk::<ppT>(&vk);
    let result = r1cs_se_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &primary_input, &proof);
    leave_block("Call to r1cs_se_ppzksnark_verifier_strong_IC", false);
    result
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
pub fn r1cs_se_ppzksnark_verifier_process_vk<ppT: PublicParams>(
    vk: &r1cs_se_ppzksnark_verification_key<ppT>,
) -> r1cs_se_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to r1cs_se_ppzksnark_verifier_process_vk", false);

    let G_alpha_pc = ppT::precompute_G1(&vk.G_alpha);
    let H_beta_pc = ppT::precompute_G2(&vk.H_beta);

    let mut pvk = r1cs_se_ppzksnark_processed_verification_key::<ppT>::default();
    pvk.G_alpha = vk.G_alpha.clone();
    pvk.H_beta = vk.H_beta.clone();
    pvk.G_alpha_H_beta_ml = ppT::miller_loop(&G_alpha_pc, &H_beta_pc);
    pvk.G_gamma_pc = ppT::precompute_G1(&vk.G_gamma);
    pvk.H_gamma_pc = ppT::precompute_G2(&vk.H_gamma);
    pvk.H_pc = ppT::precompute_G2(&vk.H);

    pvk.query = vk.query.clone();

    leave_block("Call to r1cs_se_ppzksnark_verifier_process_vk", false);

    pvk
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */
fn r1cs_se_ppzksnark_online_verifier_weak_IC<ppT: PublicParams>(
    pvk: &r1cs_se_ppzksnark_processed_verification_key<ppT>,
    primary_input: &r1cs_se_ppzksnark_primary_input<ppT>,
    proof: &r1cs_se_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_se_ppzksnark_online_verifier_weak_IC", false);

    let mut result = true;

    enter_block("Check if the proof is well-formed", false);
    if !proof.is_well_formed() {
        if !inhibit_profiling_info {
            print_indent();
            print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    leave_block("Check if the proof is well-formed", false);

    enter_block("Pairing computations", false);

    // // #ifdef MULTICORE
    //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    //

    enter_block("Check first test", false);
    /**
     * e(A*G^{alpha}, B*H^{beta}) = e(G^{alpha}, H^{beta}) * e(G^{psi}, H^{gamma})
     *                              * e(C, H)
     * where psi = \sum_{i=0}^l input_i pvk.query[i]
     */
    let G_psi = pvk.query[0].clone()
        + multi_exp::<G1<ppT>, Fr<ppT>, { multi_exp_method::multi_exp_method_bos_coster }>(
            &pvk.query[1..],
            primary_input,
            chunks,
        );

    let test1_l = ppT::miller_loop(
        &ppT::precompute_G1(&(proof.A.clone() + pvk.G_alpha.clone())),
        &ppT::precompute_G2(&(proof.B.clone() + pvk.H_beta.clone())),
    );
    let test1_r1 = pvk.G_alpha_H_beta_ml.clone();
    let test1_r2 = ppT::miller_loop(&ppT::precompute_G1(&G_psi), &pvk.H_gamma_pc);
    let test1_r3 = ppT::miller_loop(&ppT::precompute_G1(&proof.C), &pvk.H_pc);
    let test1 =
        ppT::final_exponentiation(&(test1_l.unitary_inverse() * test1_r1 * test1_r2 * test1_r3));

    if test1 != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("First test failed.\n");
        }
        result = false;
    }
    leave_block("Check first test", false);

    enter_block("Check second test", false);
    /**
     * e(A, H^{gamma}) = e(G^{gamma}, B)
     */
    let test2_l = ppT::miller_loop(&ppT::precompute_G1(&proof.A), &pvk.H_gamma_pc);
    let test2_r = ppT::miller_loop(&pvk.G_gamma_pc, &ppT::precompute_G2(&proof.B));
    let test2 = ppT::final_exponentiation(&(test2_l * test2_r.unitary_inverse()));

    if test2 != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Second test failed.\n");
        }
        result = false;
    }
    leave_block("Check second test", false);
    leave_block("Pairing computations", false);
    leave_block("Call to r1cs_se_ppzksnark_online_verifier_weak_IC", false);

    result
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */
pub fn r1cs_se_ppzksnark_online_verifier_strong_IC<ppT: PublicParams>(
    pvk: &r1cs_se_ppzksnark_processed_verification_key<ppT>,
    primary_input: &r1cs_se_ppzksnark_primary_input<ppT>,
    proof: &r1cs_se_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_se_ppzksnark_online_verifier_strong_IC", false);
    let mut result = true;

    if pvk.query.len() != primary_input.len() + 1 {
        print_indent();
        print!(
            "Input length differs from expected (got {}, expected {}).\n",
            primary_input.len(),
            pvk.query.len()
        );
        result = false;
    } else {
        result = r1cs_se_ppzksnark_online_verifier_weak_IC::<ppT>(pvk, primary_input, proof);
    }

    leave_block("Call to r1cs_se_ppzksnark_online_verifier_strong_IC", false);
    result
}

// use algebra::scalar_multiplication::multiexp;
// use common::profiling;
// use common::utils;

// // #ifdef MULTICORE
// use  <omp.h>
//

// use crate::knowledge_commitment::kc_multiexp;
// use crate::reductions::r1cs_to_sap::r1cs_to_sap;

//

impl<ppT: PublicParams> PartialEq for r1cs_se_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A_query == other.A_query
            && self.B_query == other.B_query
            && self.C_query_1 == other.C_query_1
            && self.C_query_2 == other.C_query_2
            && self.G_gamma_Z == other.G_gamma_Z
            && self.H_gamma_Z == other.H_gamma_Z
            && self.G_ab_gamma_Z == other.G_ab_gamma_Z
            && self.G_gamma2_Z2 == other.G_gamma2_Z2
            && self.G_gamma2_Z_t == other.G_gamma2_Z_t
            && self.constraint_system == other.constraint_system
    }
}

use std::fmt;
impl<ppT: PublicParams> fmt::Display for r1cs_se_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}",
            self.A_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.B_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.C_query_1
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.C_query_2
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.G_gamma_Z,
            self.H_gamma_Z,
            self.G_ab_gamma_Z,
            self.G_gamma2_Z2,
            self.G_gamma2_Z_t
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.constraint_system,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.A_query;
//     in >> pk.B_query;
//     in >> pk.C_query_1;
//     in >> pk.C_query_2;
//     in >> pk.G_gamma_Z;
//     in >> pk.H_gamma_Z;
//     in >> pk.G_ab_gamma_Z;
//     in >> pk.G_gamma2_Z2;
//     in >> pk.G_gamma2_Z_t;
//     in >> pk.constraint_system;

//     return in;
// }

impl<ppT: PublicParams> PartialEq for r1cs_se_ppzksnark_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.H == other.H
            && self.G_alpha == other.G_alpha
            && self.H_beta == other.H_beta
            && self.G_gamma == other.G_gamma
            && self.H_gamma == other.H_gamma
            && self.query == other.query
    }
}

impl<ppT: PublicParams> fmt::Display for r1cs_se_ppzksnark_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.H,
            self.G_alpha,
            self.H_beta,
            self.G_gamma,
            self.H_gamma,
            self.query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.H;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.G_alpha;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.H_beta;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.G_gamma;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.H_gamma;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.query;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: PublicParams> PartialEq for r1cs_se_ppzksnark_processed_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.G_alpha == other.G_alpha
            && self.H_beta == other.H_beta
            && self.G_alpha_H_beta_ml == other.G_alpha_H_beta_ml
            && self.G_gamma_pc == other.G_gamma_pc
            && self.H_gamma_pc == other.H_gamma_pc
            && self.H_pc == other.H_pc
            && self.query == other.query
    }
}

impl<ppT: PublicParams> fmt::Display for r1cs_se_ppzksnark_processed_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.G_alpha,
            self.H_beta,
            self.G_alpha_H_beta_ml,
            self.G_gamma_pc,
            self.H_gamma_pc,
            self.H_pc,
            self.query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.G_alpha;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.H_beta;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.G_alpha_H_beta_ml;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.G_gamma_pc;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.H_gamma_pc;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.H_pc;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.query;
//     consume_OUTPUT_NEWLINE(in);
//     return in;
// }

impl<ppT: PublicParams> PartialEq for r1cs_se_ppzksnark_proof<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A && self.B == other.B && self.C == other.C
    }
}

impl<ppT: PublicParams> fmt::Display for r1cs_se_ppzksnark_proof<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.A, self.B, self.C,)
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.A;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.B;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.C;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: PublicParams> r1cs_se_ppzksnark_verification_key<ppT> {
    pub fn dummy_verification_key(
        &self,
        input_size: usize,
    ) -> r1cs_se_ppzksnark_verification_key<ppT>
    where
        <ppT as ff_curves::PublicParams>::Fr: Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
        <ppT as ff_curves::PublicParams>::Fr: Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
    {
        let mut result = r1cs_se_ppzksnark_verification_key::<ppT>::default();
        result.H = ppT::Fr::random_element() * ppT::G2::one();
        result.G_alpha = ppT::Fr::random_element() * ppT::G1::one();
        result.H_beta = ppT::Fr::random_element() * ppT::G2::one();
        result.G_gamma = ppT::Fr::random_element() * ppT::G1::one();
        result.H_gamma = ppT::Fr::random_element() * ppT::G2::one();

        let mut v = G1_vector::<ppT>::default();
        for i in 0..=input_size {
            v.push(ppT::Fr::random_element() * ppT::G1::one());
        }
        result.query = (v);

        return result;
    }
}
pub fn r1cs_se_ppzksnark_generator<ppT: PublicParams>(
    cs: &r1cs_se_ppzksnark_constraint_system<ppT>,
) -> r1cs_se_ppzksnark_keypair<ppT>
// where
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G2:
//         Add<Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<FieldT, Output = <ppT as ff_curves::PublicParams>::Fr>,
//     <ppT as ff_curves::PublicParams>::G1:
//         Mul<FieldT, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::G2:
//         Mul<FieldT, Output = <ppT as ff_curves::PublicParams>::G2>,
{
    enter_block("Call to r1cs_se_ppzksnark_generator", false);

    /**
     * draw random element t at which the SAP is evaluated.
     * it should be the case that Z(t) != 0
     */
    let domain = r1cs_to_sap_get_domain(&cs);
    let mut t;
    loop {
        t = Fr::<ppT>::random_element();
        if !domain
            .borrow_mut()
            .compute_vanishing_polynomial(&t)
            .is_zero()
        {
            break;
        }
    }

    let sap_inst: sap_instance_evaluation<_> = r1cs_to_sap_instance_map_with_evaluation(&cs, &t);

    print_indent();
    print!("* SAP number of variables: {}\n", sap_inst.num_variables());
    print_indent();
    print!("* SAP pre degree: {}\n", cs.constraints.len());
    print_indent();
    print!("* SAP degree: {}\n", sap_inst.degree());
    print_indent();
    print!(
        "* SAP number of input variables: {}\n",
        sap_inst.num_inputs()
    );

    enter_block("Compute query densities", false);
    let mut non_zero_At = 0;
    for i in 0..=sap_inst.num_variables() {
        if !sap_inst.At[i].is_zero() {
            non_zero_At += 1;
        }
    }
    leave_block("Compute query densities", false);

    let At = (sap_inst.At.clone());
    let Ct = (sap_inst.Ct.clone());
    let Ht = (sap_inst.Ht.clone());
    /**
     * sap_inst.{A,C,H}t are now in an unspecified state,
     * but we do not use them below
     */
    let (alpha, beta, gamma, G, H) = (
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::G1::random_element(),
        ppT::G2::random_element(),
    );

    enter_block("Generating G multiexp table", false);
    let G_exp_count = sap_inst.num_inputs() + 1 // verifier_query
                         + non_zero_At // A_query
                         + sap_inst.degree() + 1 // G_gamma2_Z_t
                         // C_query_1
                         + sap_inst.num_variables()
        - sap_inst.num_inputs()
        + sap_inst.num_variables()
        + 1; // C_query_2
    let G_window = get_exp_window_size::<ppT::G1>(G_exp_count);
    print_indent();
    print!("* G window: {}\n", G_window);
    let G_table = get_window_table(ppT::Fr::size_in_bits(), G_window, G.clone());
    leave_block("Generating G multiexp table", false);

    enter_block("Generating H_gamma multiexp table", false);
    let mut H_gamma = H.clone() * gamma.clone();
    let mut H_gamma_exp_count = non_zero_At; // B_query
    let mut H_gamma_window = get_exp_window_size::<ppT::G2>(H_gamma_exp_count);
    print_indent();
    print!("* H_gamma window: {}\n", H_gamma_window);
    let mut H_gamma_table =
        get_window_table(ppT::Fr::size_in_bits(), H_gamma_window, H_gamma.clone());
    leave_block("Generating H_gamma multiexp table", false);

    enter_block("Generate R1CS verification key", false);
    let mut G_alpha = G.clone() * alpha.clone();
    let mut H_beta = H.clone() * beta.clone();

    let mut tmp_exponents = Fr_vector::<ppT>::default();
    tmp_exponents.reserve(sap_inst.num_inputs() + 1);
    for i in 0..=sap_inst.num_inputs() {
        tmp_exponents
            .push(gamma.clone() * Ct[i].clone() + (alpha.clone() + beta.clone()) * At[i].clone());
    }
    let verifier_query =
        batch_exp::<G1<ppT>, Fr<ppT>>(ppT::Fr::size_in_bits(), G_window, &G_table, &tmp_exponents);
    tmp_exponents.clear();

    leave_block("Generate R1CS verification key", false);

    enter_block("Generate R1CS proving key", false);

    enter_block("Compute the A-query", false);
    tmp_exponents.reserve(sap_inst.num_variables() + 1);
    for i in 0..At.len() {
        tmp_exponents.push(gamma.clone() * At[i].clone());
    }

    let A_query =
        batch_exp::<G1<ppT>, Fr<ppT>>(ppT::Fr::size_in_bits(), G_window, &G_table, &tmp_exponents);
    tmp_exponents.clear();
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(A_query);
    //
    leave_block("Compute the A-query", false);

    enter_block("Compute the B-query", false);
    let B_query =
        batch_exp::<G2<ppT>, Fr<ppT>>(ppT::Fr::size_in_bits(), H_gamma_window, &H_gamma_table, &At);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G2<ppT> >(B_query);
    //
    leave_block("Compute the B-query", false);

    enter_block("Compute the G_gamma-query", false);
    let G_gamma = G.clone() * gamma.clone();
    let G_gamma_Z = G_gamma.clone() * sap_inst.Zt.clone();
    let H_gamma_Z = H_gamma.clone() * sap_inst.Zt.clone();
    let G_ab_gamma_Z = G_gamma_Z.clone() * (alpha.clone() + beta.clone());
    let G_gamma2_Z2 = G_gamma_Z.clone() * (gamma.clone() * sap_inst.Zt.clone());

    tmp_exponents.reserve(sap_inst.degree() + 1);

    /* Compute the vector G_gamma2_Z_t := Z(t) * t^i * gamma^2 * G */
    let mut gamma2_Z_t = gamma.squared() * sap_inst.Zt.clone();
    for i in 0..sap_inst.degree() + 1 {
        tmp_exponents.push(gamma2_Z_t.clone());
        gamma2_Z_t *= t.clone();
    }
    let G_gamma2_Z_t =
        batch_exp::<G1<ppT>, Fr<ppT>>(ppT::Fr::size_in_bits(), G_window, &G_table, &tmp_exponents);
    tmp_exponents.clear();
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(G_gamma2_Z_t);
    //
    leave_block("Compute the G_gamma-query", false);

    enter_block("Compute the C_1-query", false);
    tmp_exponents.reserve(sap_inst.num_variables() - sap_inst.num_inputs());
    for i in sap_inst.num_inputs() + 1..=sap_inst.num_variables() {
        tmp_exponents.push(
            gamma.clone()
                * (gamma.clone() * Ct[i].clone() + (alpha.clone() + beta.clone()) * At[i].clone()),
        );
    }
    let C_query_1 =
        batch_exp::<G1<ppT>, Fr<ppT>>(ppT::Fr::size_in_bits(), G_window, &G_table, &tmp_exponents);
    tmp_exponents.clear();
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(C_query_1);
    //
    leave_block("Compute the C_1-query", false);

    enter_block("Compute the C_2-query", false);
    tmp_exponents.reserve(sap_inst.num_variables() + 1);
    let mut double_gamma2_Z = gamma.clone() * gamma.clone() * sap_inst.Zt.clone();
    double_gamma2_Z = double_gamma2_Z.clone() + double_gamma2_Z.clone();
    for i in 0..=sap_inst.num_variables() {
        tmp_exponents.push(double_gamma2_Z.clone() * At[i].clone());
    }
    let C_query_2 =
        batch_exp::<G1<ppT>, Fr<ppT>>(ppT::Fr::size_in_bits(), G_window, &G_table, &tmp_exponents);
    tmp_exponents.clear();
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(C_query_2);
    //
    leave_block("Compute the C_2-query", false);

    leave_block("Generate R1CS proving key", false);

    leave_block("Call to r1cs_se_ppzksnark_generator", false);

    let vk = r1cs_se_ppzksnark_verification_key::<ppT>::new(
        H,
        G_alpha,
        H_beta,
        G_gamma,
        H_gamma.clone(),
        (verifier_query),
    );

    let cs_copy = cs.clone();

    let pk = r1cs_se_ppzksnark_proving_key::<ppT>::new(
        (A_query),
        (B_query),
        (C_query_1),
        (C_query_2),
        G_gamma_Z,
        H_gamma_Z,
        G_ab_gamma_Z,
        G_gamma2_Z2,
        (G_gamma2_Z_t),
        (cs_copy),
    );

    pk.print_size();
    vk.print_size();

    r1cs_se_ppzksnark_keypair::<ppT>::new((pk), (vk))
}
