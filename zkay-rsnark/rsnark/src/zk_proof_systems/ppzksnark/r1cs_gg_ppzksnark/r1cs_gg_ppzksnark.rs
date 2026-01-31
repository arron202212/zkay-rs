// Declaration of interfaces for a ppzkSNARK for R1CS with a security proof
// in the generic group (GG) model.

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

// The implementation instantiates the protocol of \[Gro16].

// Acronyms:

// - R1CS = "Rank-1 Constraint Systems"
// - ppzkSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

// References:

// \[Gro16]:
//  "On the Size of Pairing-based Non-interactive Arguments",
//  Jens Groth,
//  EUROCRYPT 2016,
//  <https://eprint.iacr.org/2016/260>

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::kc_multiexp::{kc_batch_exp, kc_multi_exp_with_mixed_addition};
use crate::knowledge_commitment::knowledge_commitment::{
    knowledge_commitment, knowledge_commitment_vector,
};
use crate::reductions::r1cs_to_qap::r1cs_to_qap::r1cs_to_qap_instance_map_with_evaluation;
use crate::reductions::r1cs_to_qap::r1cs_to_qap::r1cs_to_qap_witness_map;
use crate::relations::arithmetic_programs::qap::qap::qap_instance_evaluation;
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark_params::{
    r1cs_gg_ppzksnark_auxiliary_input, r1cs_gg_ppzksnark_constraint_system,
    r1cs_gg_ppzksnark_primary_input,
};
use crate::zk_proof_systems::ppzksnark::{
    KeyPairTConfig, ProofTConfig, ProvingKeyTConfig, VerificationKeyTConfig,
};
use ffec::scalar_multiplication::multiexp::KCConfig;
use ffec::{FieldTConfig, One, Zero};
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;

use ff_curves::{Fr, Fr_vector, G1, G2, GT};

use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::scalar_multiplication::multiexp::{
    batch_exp, batch_exp_with_coeff, get_exp_window_size, get_window_table, inhibit_profiling_info,
    multi_exp, multi_exp_method, multi_exp_with_mixed_addition,
};

use crate::common::data_structures::accumulation_vector::accumulation_vector;
use ffec::PpConfig;
use ffec::common::serialization::OUTPUT_NEWLINE;

use ff_curves::algebra::curves::public_params::{PublicParams, PublicParamsType};
use ff_curves::{G1_vector, G2_precomp};
const N: usize = 4;
use std::ops::{Add, Mul};
/******************************** Proving key ********************************/

// pub type T1<PP> = <<PP as ppTConfig>::KC as KCConfig>::T;
// pub type T2<PP> = <<PP as ppTConfig>::KC as KCConfig>::T2;
// pub type FieldT<PP> = <<PP as ppTConfig>::KC as KCConfig>::FieldT;
// pub type KnowledgeCommitmentVector<PP> = knowledge_commitment_vector<T1<PP>, T1<PP>>;
// pub type KnowledgeCommitmentVector2<PP> = knowledge_commitment_vector<T2<PP>, T1<PP>>;
// pub type KnowledgeCommitment<PP> = knowledge_commitment<T1<PP>, T1<PP>>;
// pub type KnowledgeCommitment2<PP> = knowledge_commitment<T2<PP>, T1<PP>>;
// pub type AccumulationVector<PP> = accumulation_vector<T1<PP>>;

/**
 * A proving key for the R1CS GG-ppzkSNARK.
 */
#[derive(Clone, Default)]
pub struct r1cs_gg_ppzksnark_proving_key<ppT: PublicParams>
// where
//     <ppT as ff_curves::PublicParams>::G2: PpConfig,
//     <ppT as ff_curves::PublicParams>::G1: PpConfig,
{
    pub alpha_g1: G1<ppT>,
    pub beta_g1: G1<ppT>,
    pub beta_g2: G2<ppT>,
    pub delta_g1: G1<ppT>,
    pub delta_g2: G2<ppT>,

    pub A_query: G1_vector<ppT>, // this could be a sparse vector if we had multiexp for those
    pub B_query: knowledge_commitment_vector<G2<ppT>, G1<ppT>>,
    pub H_query: G1_vector<ppT>,
    pub L_query: G1_vector<ppT>,

    pub constraint_system: r1cs_gg_ppzksnark_constraint_system<ppT>,
}
impl<ppT: PublicParams> ProvingKeyTConfig for r1cs_gg_ppzksnark_proving_key<ppT> {}

impl<ppT: PublicParams> r1cs_gg_ppzksnark_proving_key<ppT> {
    pub fn new(
        alpha_g1: G1<ppT>,
        beta_g1: G1<ppT>,
        beta_g2: G2<ppT>,
        delta_g1: G1<ppT>,
        delta_g2: G2<ppT>,
        A_query: G1_vector<ppT>,
        B_query: knowledge_commitment_vector<G2<ppT>, G1<ppT>>,
        H_query: G1_vector<ppT>,
        L_query: G1_vector<ppT>,
        constraint_system: r1cs_gg_ppzksnark_constraint_system<ppT>,
    ) -> Self {
        Self {
            alpha_g1,
            beta_g1,
            beta_g2,
            delta_g1,
            delta_g2,
            A_query,
            B_query,
            H_query,
            L_query,
            constraint_system,
        }
    }

    pub fn g1_size(&self) -> usize {
        1 + self.A_query.len()
            + self.B_query.domain_size()
            + self.H_query.len()
            + self.L_query.len()
    }

    pub fn g2_size(&self) -> usize {
        1 + self.B_query.domain_size()
    }

    pub fn g1_sparse_size(&self) -> usize {
        1 + self.A_query.len() + self.B_query.len() + self.H_query.len() + self.L_query.len()
    }

    pub fn g2_sparse_size(&self) -> usize {
        1 + self.B_query.len()
    }

    pub fn size_in_bits(&self) -> usize {
        ffec::size_in_bits(&self.A_query)
            + self.B_query.size_in_bits()
            + ffec::size_in_bits(&self.H_query)
            + ffec::size_in_bits(&self.L_query)
            + 1 * G1::<ppT>::size_in_bits()
            + 1 * G2::<ppT>::size_in_bits()
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in PK: {}\n", self.g1_size());
        print_indent();
        print!("* Non-zero G1 elements in PK: {}\n", self.g1_sparse_size());
        print_indent();
        print!("* G2 elements in PK: {}\n", self.g2_size());
        print_indent();
        print!("* Non-zero G2 elements in PK: {}\n", self.g2_sparse_size());
        print_indent();
        print!("* PK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(&other:r1cs_gg_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pk:r1cs_gg_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_proving_key<ppT> &pk);
}

/******************************* Verification key ****************************/

/**
 * A verification key for the R1CS GG-ppzkSNARK.
 */
#[derive(Clone, Default)]
pub struct r1cs_gg_ppzksnark_verification_key<ppT: PublicParams>
// where
//     <ppT as ff_curves::PublicParams>::G1: PpConfig,
{
    pub alpha_g1_beta_g2: GT<ppT>,
    pub gamma_g2: G2<ppT>,
    pub delta_g2: G2<ppT>,

    pub gamma_ABC_g1: accumulation_vector<G1<ppT>>,
}
impl<ppT: PublicParams> VerificationKeyTConfig for r1cs_gg_ppzksnark_verification_key<ppT> {}

impl<ppT: PublicParams> r1cs_gg_ppzksnark_verification_key<ppT> {
    // r1cs_gg_ppzksnark_verification_key() = default;
    pub fn new(
        alpha_g1_beta_g2: GT<ppT>,
        gamma_g2: G2<ppT>,
        delta_g2: G2<ppT>,
        gamma_ABC_g1: accumulation_vector<G1<ppT>>,
    ) -> Self {
        Self {
            alpha_g1_beta_g2,
            gamma_g2,
            delta_g2,
            gamma_ABC_g1,
        }
    }

    pub fn g1_size(&self) -> usize {
        self.gamma_ABC_g1.len()
    }

    pub fn g2_size(&self) -> usize {
        2
    }

    pub fn gt_size(&self) -> usize {
        1
    }

    pub fn size_in_bits(&self) -> usize {
        // TODO: include GT size
        (self.gamma_ABC_g1.size_in_bits() + 2 * G2::<ppT>::size_in_bits())
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in VK: {}\n", self.g1_size());
        print_indent();
        print!("* G2 elements in VK: {}\n", self.g2_size());
        print_indent();
        print!("* GT elements in VK: {}\n", self.gt_size());
        print_indent();
        print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(&other:r1cs_gg_ppzksnark_verification_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &vk:r1cs_gg_ppzksnark_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_verification_key<ppT> &vk);

    // static r1cs_gg_ppzksnark_verification_key<ppT> dummy_verification_key(input_size:usize);
}

/************************ Processed verification key *************************/

/**
 * A processed verification key for the R1CS GG-ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
#[derive(Clone, Default)]
pub struct r1cs_gg_ppzksnark_processed_verification_key<ppT: PublicParams> {
    pub vk_alpha_g1_beta_g2: GT<ppT>,
    pub vk_gamma_g2_precomp: G2_precomp<ppT>,
    pub vk_delta_g2_precomp: G2_precomp<ppT>,

    pub gamma_ABC_g1: accumulation_vector<G1<ppT>>,
    // bool operator==(&other:r1cs_gg_ppzksnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pvk:r1cs_gg_ppzksnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_processed_verification_key<ppT> &pvk);
}

/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS GG-ppzkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Clone, Default)]
pub struct r1cs_gg_ppzksnark_keypair<ppT: PublicParams> {
    pub pk: r1cs_gg_ppzksnark_proving_key<ppT>,
    pub vk: r1cs_gg_ppzksnark_verification_key<ppT>,
}
impl<ppT: PublicParams> KeyPairTConfig for r1cs_gg_ppzksnark_keypair<ppT> {
    type PK = r1cs_gg_ppzksnark_proving_key<ppT>;
    type VK = r1cs_gg_ppzksnark_verification_key<ppT>;
    fn vk(&self) -> &Self::VK {
        &self.vk
    }
    fn pk(&self) -> &Self::PK {
        &self.pk
    }
}

impl<ppT: PublicParams> r1cs_gg_ppzksnark_keypair<ppT> {
    // r1cs_gg_ppzksnark_keypair() = default;
    // r1cs_gg_ppzksnark_keypair(&other:r1cs_gg_ppzksnark_keypair<ppT>) = default;
    pub fn new(
        pk: r1cs_gg_ppzksnark_proving_key<ppT>,
        vk: r1cs_gg_ppzksnark_verification_key<ppT>,
    ) -> Self {
        Self { pk, vk }
    }

    // r1cs_gg_ppzksnark_keypair(r1cs_gg_ppzksnark_keypair<ppT> &&other) = default;
}

/*********************************** Proof ***********************************/

/**
 * A proof for the R1CS GG-ppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
#[derive(Clone)]
pub struct r1cs_gg_ppzksnark_proof<ppT: PublicParams> {
    pub g_A: G1<ppT>,
    pub g_B: G2<ppT>,
    pub g_C: G1<ppT>,
}
impl<ppT: PublicParams> ProofTConfig for r1cs_gg_ppzksnark_proof<ppT> {}

impl<ppT: PublicParams> Default for r1cs_gg_ppzksnark_proof<ppT> {
    fn default() -> Self {
        // invalid proof with valid curve points
        Self {
            g_A: G1::<ppT>::one(),
            g_B: G2::<ppT>::one(),
            g_C: G1::<ppT>::one(),
        }
    }
}
impl<ppT: PublicParams> r1cs_gg_ppzksnark_proof<ppT> {
    pub fn new(g_A: G1<ppT>, g_B: G2<ppT>, g_C: G1<ppT>) -> Self {
        Self { g_A, g_B, g_C }
    }

    pub fn g1_size() -> usize {
        2
    }

    pub fn g2_size() -> usize {
        1
    }

    pub fn size_in_bits() -> usize {
        Self::g1_size() * G1::<ppT>::size_in_bits() + Self::g2_size() * G2::<ppT>::size_in_bits()
    }

    fn print_size() {
        print_indent();
        print!("* G1 elements in proof: {}\n", Self::g1_size());
        print_indent();
        print!("* G2 elements in proof: {}\n", Self::g2_size());
        print_indent();
        print!("* Proof size in bits: {}\n", Self::size_in_bits());
    }

    fn is_well_formed(&self) -> bool {
        return (self.g_A.is_well_formed()
            && self.g_B.is_well_formed()
            && self.g_C.is_well_formed());
    }

    // bool operator==(&other:r1cs_gg_ppzksnark_proof<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &proof:r1cs_gg_ppzksnark_proof<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_proof<ppT> &proof);
}

/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the R1CS GG-ppzkSNARK.
 *
 * Given a R1CS constraint system CS, this algorithm produces proving and verification keys for CS.
 */

pub fn r1cs_gg_ppzksnark_generator<ppT: PublicParams>(
    r1cs: &r1cs_gg_ppzksnark_constraint_system<ppT>,
) -> r1cs_gg_ppzksnark_keypair<ppT>
// where
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G2:
//         Add<Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     ED: fqfft::evaluation_domain::evaluation_domain::evaluation_domain<
//             <ppT as ff_curves::PublicParams>::Fr,
//         >,
{
    enter_block("Call to r1cs_gg_ppzksnark_generator", false);

    /* Make the B_query "lighter" if possible */
    let mut r1cs_copy = r1cs.clone();
    r1cs_copy.swap_AB_if_beneficial();

    /* Generate secret randomness */
    let t = Fr::<ppT>::random_element();
    let alpha = Fr::<ppT>::random_element();
    let beta = Fr::<ppT>::random_element();
    let gamma = Fr::<ppT>::random_element();
    let delta = Fr::<ppT>::random_element();
    let gamma_inverse = gamma.inverse();
    let delta_inverse = delta.inverse();

    /* A quadratic arithmetic program evaluated at t. */
    let qap = r1cs_to_qap_instance_map_with_evaluation::<Fr<ppT>, pb_variable, pb_linear_combination>(
        &r1cs_copy, &t,
    );

    print_indent();
    print!("* QAP number of variables: {}\n", qap.num_variables());
    print_indent();
    print!("* QAP pre degree: {}\n", r1cs_copy.constraints.len());
    print_indent();
    print!("* QAP degree: {}\n", qap.degree());
    print_indent();
    print!("* QAP number of input variables: {}\n", qap.num_inputs());

    enter_block("Compute query densities", false);
    let mut non_zero_At = 0;
    let mut non_zero_Bt = 0;
    for i in 0..qap.num_variables() + 1 {
        if !qap.At[i].is_zero() {
            non_zero_At += 1;
        }
        if !qap.Bt[i].is_zero() {
            non_zero_Bt += 1;
        }
    }
    leave_block("Compute query densities", false);

    /* qap.{At,Bt,Ct,Ht} are now in unspecified state, but we do not use them later */
    let mut At = qap.At.clone();
    let mut Bt = qap.Bt.clone();
    let mut Ct = qap.Ct.clone();
    let mut Ht = qap.Ht.clone();

    /* The gamma inverse product component: (beta*A_i(t) + alpha*B_i(t) + C_i(t)) * gamma^{-1}. */
    enter_block("Compute gamma_ABC for R1CS verification key", false);
    let mut gamma_ABC = Fr_vector::<ppT>::default();
    gamma_ABC.reserve(qap.num_inputs());

    let gamma_ABC_0 =
        (beta.clone() * At[0].clone() + alpha.clone() * Bt[0].clone() + Ct[0].clone())
            * gamma_inverse.clone();
    for i in 1..qap.num_inputs() + 1 {
        gamma_ABC.push(
            (beta.clone() * At[i].clone() + alpha.clone() * Bt[i].clone() + Ct[i].clone())
                * gamma_inverse.clone(),
        );
    }
    leave_block("Compute gamma_ABC for R1CS verification key", false);

    /* The delta inverse product component: (beta*A_i(t) + alpha*B_i(t) + C_i(t)) * delta^{-1}. */
    enter_block("Compute L query for R1CS proving key", false);
    let mut Lt = Fr_vector::<ppT>::default();
    Lt.reserve(qap.num_variables() - qap.num_inputs());

    let mut Lt_offset = qap.num_inputs() + 1;
    for i in 0..qap.num_variables() - qap.num_inputs() {
        Lt.push(
            (beta.clone() * At[Lt_offset + i].clone()
                + alpha.clone() * Bt[Lt_offset + i].clone()
                + Ct[Lt_offset + i].clone())
                * delta_inverse.clone(),
        );
    }
    leave_block("Compute L query for R1CS proving key", false);

    /**
     * Note that H for Groth's proof system is degree d-2, but the QAP
     * reduction returns coefficients for degree d polynomial H (in
     * style of PGHR-type proof systems)
     */
    Ht.resize(Ht.len() - 2, Fr::<ppT>::default());

    // // #ifdef MULTICORE
    //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    // //#endif

    enter_block("Generating G1 MSM window table", false);
    let g1_generator = G1::<ppT>::random_element();
    let g1_scalar_count = non_zero_At + non_zero_Bt + qap.num_variables();
    let g1_scalar_size = Fr::<ppT>::size_in_bits();
    let g1_window_size = get_exp_window_size::<G1<ppT>>(g1_scalar_count);

    print_indent();
    print!("* G1 window: {}\n", g1_window_size);
    let g1_table = get_window_table(g1_scalar_size, g1_window_size, g1_generator.clone());
    leave_block("Generating G1 MSM window table", false);

    enter_block("Generating G2 MSM window table", false);
    let G2_gen = G2::<ppT>::random_element();
    let g2_scalar_count = non_zero_Bt;
    let g2_scalar_size = Fr::<ppT>::size_in_bits();
    let g2_window_size = get_exp_window_size::<G2<ppT>>(g2_scalar_count);

    print_indent();
    print!("* G2 window: {}\n", g2_window_size);
    let g2_table = get_window_table(g2_scalar_size, g2_window_size, G2_gen.clone());
    leave_block("Generating G2 MSM window table", false);

    enter_block("Generate R1CS proving key", false);
    let mut alpha_g1 = g1_generator.clone() * alpha.clone();
    let mut beta_g1 = g1_generator.clone() * beta.clone();
    let mut beta_g2 = G2_gen.clone() * beta.clone();
    let mut delta_g1 = g1_generator.clone() * delta.clone();
    let mut delta_g2 = G2_gen.clone() * delta.clone();

    enter_block("Generate queries", false);
    enter_block("Compute the A-query", false);
    let A_query = batch_exp::<G1<ppT>, Fr<ppT>>(g1_scalar_size, g1_window_size, &g1_table, &At);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(A_query);
    // //#endif
    leave_block("Compute the A-query", false);

    enter_block("Compute the B-query", false);
    let mut B_query = kc_batch_exp::<G2<ppT>, G1<ppT>, Fr<ppT>>(
        Fr::<ppT>::size_in_bits(),
        g2_window_size,
        g1_window_size,
        &g2_table,
        &g1_table,
        &Fr::<ppT>::one(),
        &Fr::<ppT>::one(),
        &Bt,
        chunks,
    );
    // NOTE: if USE_MIXED_ADDITION is defined,
    // kc_batch_exp will convert its output to special form internally
    leave_block("Compute the B-query", false);

    enter_block("Compute the H-query", false);
    let mut H_query = batch_exp_with_coeff::<G1<ppT>, Fr<ppT>>(
        g1_scalar_size,
        g1_window_size,
        &g1_table,
        &(qap.Zt.clone() * delta_inverse.clone()),
        &Ht,
    );
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(H_query);
    // //#endif
    leave_block("Compute the H-query", false);

    enter_block("Compute the L-query", false);
    let mut L_query = batch_exp::<G1<ppT>, Fr<ppT>>(g1_scalar_size, g1_window_size, &g1_table, &Lt);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(L_query);
    // //#endif
    leave_block("Compute the L-query", false);
    leave_block("Generate queries", false);

    leave_block("Generate R1CS proving key", false);

    enter_block("Generate R1CS verification key", false);
    let mut alpha_g1_beta_g2 = ppT::reduced_pairing(&alpha_g1, &beta_g2);
    let mut gamma_g2 = G2_gen * gamma;

    enter_block("Encode gamma_ABC for R1CS verification key", false);
    let mut gamma_ABC_g1_0 = g1_generator * gamma_ABC_0;
    let mut gamma_ABC_g1_values =
        batch_exp::<G1<ppT>, Fr<ppT>>(g1_scalar_size, g1_window_size, &g1_table, &gamma_ABC);
    leave_block("Encode gamma_ABC for R1CS verification key", false);
    leave_block("Generate R1CS verification key", false);

    leave_block("Call to r1cs_gg_ppzksnark_generator", false);

    let mut gamma_ABC_g1 =
        accumulation_vector::<G1<ppT>>::new_with_vec(gamma_ABC_g1_0, gamma_ABC_g1_values);

    let mut vk = r1cs_gg_ppzksnark_verification_key::<ppT>::new(
        alpha_g1_beta_g2,
        gamma_g2,
        delta_g2.clone(),
        gamma_ABC_g1,
    );

    let mut pk = r1cs_gg_ppzksnark_proving_key::<ppT>::new(
        alpha_g1.clone(),
        beta_g1,
        beta_g2.clone(),
        delta_g1,
        delta_g2,
        A_query,
        B_query,
        H_query,
        L_query,
        r1cs_copy,
    );

    pk.print_size();
    vk.print_size();

    r1cs_gg_ppzksnark_keypair::<ppT>::new(pk, vk)
}
/**
 * A prover algorithm for the R1CS GG-ppzkSNARK.
 *
 * Given a R1CS primary input X and a R1CS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the R1CS constraint system that was given as input to the generator algorithm.
 */

pub fn r1cs_gg_ppzksnark_prover<ppT: PublicParams>(
    pk: &r1cs_gg_ppzksnark_proving_key<ppT>,
    primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
    auxiliary_input: &r1cs_gg_ppzksnark_auxiliary_input<ppT>,
) -> r1cs_gg_ppzksnark_proof<ppT>
// where
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::Fr>,
//     <ppT as ff_curves::PublicParams>::G1:
//         Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::G1:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::G2:
//         Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::G1:
//         Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::G1>,
//     ED: fqfft::evaluation_domain::evaluation_domain::evaluation_domain<
//             <ppT as ff_curves::PublicParams>::Fr,
//         >,
{
    enter_block("Call to r1cs_gg_ppzksnark_prover", false);

    // // #ifdef DEBUG
    //     assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
    // //#endif

    enter_block("Compute the polynomial H", false);
    let qap_wit = r1cs_to_qap_witness_map::<Fr<ppT>, pb_variable, pb_linear_combination>(
        &pk.constraint_system,
        &primary_input,
        &auxiliary_input,
        &Fr::<ppT>::zero(),
        &Fr::<ppT>::zero(),
        &Fr::<ppT>::zero(),
    );

    /* We are dividing degree 2(d-1) polynomial by degree d polynomial
    and not adding a PGHR-style ZK-patch, so our H is degree d-2 */
    assert!(!qap_wit.coefficients_for_H[qap_wit.degree() - 2].is_zero());
    assert!(qap_wit.coefficients_for_H[qap_wit.degree() - 1].is_zero());
    assert!(qap_wit.coefficients_for_H[qap_wit.degree()].is_zero());
    leave_block("Compute the polynomial H", false);

    // // #ifdef DEBUG
    //     let t =Fr::<ppT>::random_element();
    //     qap_instance_evaluation<Fr<ppT> > qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    //     assert!(qap_inst.is_satisfied(qap_wit));
    // //#endif

    /* Choose two random field elements for prover zero-knowledge. */
    let r = Fr::<ppT>::random_element();
    let s = Fr::<ppT>::random_element();

    // // #ifdef DEBUG
    //     assert!(qap_wit.coefficients_for_ABCs.len() == qap_wit.num_variables());
    //     assert!(pk.A_query.len() == qap_wit.num_variables()+1);
    //     assert!(pk.B_query.domain_size() == qap_wit.num_variables()+1);
    //     assert!(pk.H_query.len() == qap_wit.degree() - 1);
    //     assert!(pk.L_query.len() == qap_wit.num_variables() - qap_wit.num_inputs());
    // //#endif

    // // #ifdef MULTICORE
    //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    // //#endif

    enter_block("Compute the proof", false);

    enter_block("Compute evaluation to A-query", false);
    // TODO: sort out indexing
    let mut const_padded_assignment = vec![Fr::<ppT>::one()];
    const_padded_assignment.extend(qap_wit.coefficients_for_ABCs.clone());

    let evaluation_At = multi_exp_with_mixed_addition::<
        G1<ppT>,
        Fr<ppT>,
        { multi_exp_method::multi_exp_method_BDLO12 },
    >(
        &pk.A_query[..qap_wit.num_variables() + 1],
        &const_padded_assignment[..qap_wit.num_variables() + 1],
        chunks,
    );
    leave_block("Compute evaluation to A-query", false);

    enter_block("Compute evaluation to B-query", false);
    let evaluation_Bt = kc_multi_exp_with_mixed_addition::<
        G2<ppT>,
        G1<ppT>,
        Fr<ppT>,
        { multi_exp_method::multi_exp_method_BDLO12 },
    >(
        &pk.B_query,
        0,
        qap_wit.num_variables() + 1,
        &const_padded_assignment[..qap_wit.num_variables() + 1],
        chunks,
    );
    leave_block("Compute evaluation to B-query", false);

    enter_block("Compute evaluation to H-query", false);
    let evaluation_Ht = multi_exp::<G1<ppT>, Fr<ppT>, { multi_exp_method::multi_exp_method_BDLO12 }>(
        &pk.H_query[..(qap_wit.degree() - 1)],
        &qap_wit.coefficients_for_H[..(qap_wit.degree() - 1)],
        chunks,
    );
    leave_block("Compute evaluation to H-query", false);

    enter_block("Compute evaluation to L-query", false);
    let evaluation_Lt = multi_exp_with_mixed_addition::<
        G1<ppT>,
        Fr<ppT>,
        { multi_exp_method::multi_exp_method_BDLO12 },
    >(
        &pk.L_query,
        &const_padded_assignment[qap_wit.num_inputs() + 1..qap_wit.num_variables() + 1],
        chunks,
    );
    leave_block("Compute evaluation to L-query", false);

    /* A = alpha + sum_i(a_i*A_i(t)) + r*delta */
    let g1_A = pk.alpha_g1.clone() + evaluation_At.clone() + pk.delta_g1.clone() * r.clone();

    /* B = beta + sum_i(a_i*B_i(t)) + s*delta */
    let g1_B = pk.beta_g1.clone() + evaluation_Bt.h.clone() + pk.delta_g1.clone() * s.clone();
    let g2_B = pk.beta_g2.clone() + evaluation_Bt.g.clone() + pk.delta_g2.clone() * s.clone();

    /* C = sum_i(a_i*((beta*A_i(t) + alpha*B_i(t) + C_i(t)) + H(t)*Z(t))/delta) + A*s + r*b - r*s*delta */
    let g1_C = evaluation_Ht.clone()
        + evaluation_Lt.clone()
        + g1_A.clone() * s.clone()
        + g1_B.clone() * r.clone()
        - pk.delta_g1.clone() * (r.clone() * s.clone());

    leave_block("Compute the proof", false);

    leave_block("Call to r1cs_gg_ppzksnark_prover", false);

    let proof = r1cs_gg_ppzksnark_proof::<ppT>::new(g1_A, g2_B, g1_C);
    r1cs_gg_ppzksnark_proof::<ppT>::print_size();

    proof
}

/*
  Below are four variants of verifier algorithm for the R1CS GG-ppzkSNARK.

  These are the four cases that arise from the following two choices:

  (1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
  In the latter case, we call the algorithm an "online verifier".

  (2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
  Strong input consistency requires that |primary_input| = CS.num_inputs, whereas
  weak input consistency requires that |primary_input| <= CS.num_inputs (and
  the primary input is implicitly padded with zeros up to length CS.num_inputs).
*/

/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */

pub fn r1cs_gg_ppzksnark_verifier_weak_IC<ppT: PublicParams>(
    vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: &r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_verifier_weak_IC", false);
    let pvk = r1cs_gg_ppzksnark_verifier_process_vk::<ppT>(&vk);
    let result = r1cs_gg_ppzksnark_online_verifier_weak_IC::<ppT>(&pvk, &primary_input, &proof);
    leave_block("Call to r1cs_gg_ppzksnark_verifier_weak_IC", false);
    return result;
}

/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_gg_ppzksnark_verifier_strong_IC<ppT: PublicParams>(
    vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: &r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_verifier_strong_IC", false);
    let pvk = r1cs_gg_ppzksnark_verifier_process_vk::<ppT>(&vk);
    let result = r1cs_gg_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &primary_input, &proof);
    leave_block("Call to r1cs_gg_ppzksnark_verifier_strong_IC", false);
    result
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

pub fn r1cs_gg_ppzksnark_verifier_process_vk<ppT: PublicParams>(
    vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
) -> r1cs_gg_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to r1cs_gg_ppzksnark_verifier_process_vk", false);

    let mut pvk = r1cs_gg_ppzksnark_processed_verification_key::<ppT>::default();
    pvk.vk_alpha_g1_beta_g2 = vk.alpha_g1_beta_g2.clone();
    pvk.vk_gamma_g2_precomp = ppT::precompute_G2(&vk.gamma_g2);
    pvk.vk_delta_g2_precomp = ppT::precompute_G2(&vk.delta_g2);
    pvk.gamma_ABC_g1 = vk.gamma_ABC_g1.clone();

    leave_block("Call to r1cs_gg_ppzksnark_verifier_process_vk", false);

    pvk
}

/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

pub fn r1cs_gg_ppzksnark_online_verifier_weak_IC<ppT: PublicParams>(
    pvk: &r1cs_gg_ppzksnark_processed_verification_key<ppT>,
    primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: &r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_online_verifier_weak_IC", false);
    assert!(pvk.gamma_ABC_g1.domain_size() >= primary_input.len());

    enter_block("Accumulate input", false);
    let accumulated_IC = pvk
        .gamma_ABC_g1
        .accumulate_chunk::<Fr<ppT>>(primary_input, 0);
    let acc = &accumulated_IC.first;
    leave_block("Accumulate input", false);

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

    enter_block("Online pairing computations", false);
    enter_block("Check QAP divisibility", false);
    let proof_g_A_precomp = ppT::precompute_G1(&proof.g_A);
    let proof_g_B_precomp = ppT::precompute_G2(&proof.g_B);
    let proof_g_C_precomp = ppT::precompute_G1(&proof.g_C);
    let acc_precomp = ppT::precompute_G1(&acc);

    let QAP1 = ppT::miller_loop(&proof_g_A_precomp, &proof_g_B_precomp);
    let QAP2 = ppT::double_miller_loop(
        &acc_precomp,
        &pvk.vk_gamma_g2_precomp,
        &proof_g_C_precomp,
        &pvk.vk_delta_g2_precomp,
    );
    let QAP = ppT::final_exponentiation(&(QAP1 * QAP2.unitary_inverse()));

    if QAP != pvk.vk_alpha_g1_beta_g2 {
        if !inhibit_profiling_info {
            print_indent();
            print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility", false);
    leave_block("Online pairing computations", false);

    leave_block("Call to r1cs_gg_ppzksnark_online_verifier_weak_IC", false);

    result
}
/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_gg_ppzksnark_online_verifier_strong_IC<ppT: PublicParams>(
    pvk: &r1cs_gg_ppzksnark_processed_verification_key<ppT>,
    primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: &r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    let mut result = true;
    enter_block("Call to r1cs_gg_ppzksnark_online_verifier_strong_IC", false);

    if pvk.gamma_ABC_g1.domain_size() != primary_input.len() {
        print_indent();
        print!(
            "Input length differs from expected (got {}, expected {}).\n",
            primary_input.len(),
            pvk.gamma_ABC_g1.domain_size()
        );
        result = false;
    } else {
        result = r1cs_gg_ppzksnark_online_verifier_weak_IC::<ppT>(&pvk, &primary_input, &proof);
    }

    leave_block("Call to r1cs_gg_ppzksnark_online_verifier_strong_IC", false);
    result
}

/****************************** Miscellaneous ********************************/

/**
 * For debugging purposes (of r1cs_gg_ppzksnark_r1cs_gg_ppzksnark_verifier_gadget):
 *
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a non-processed verification key,
 * (2) has weak input consistency, and
 * (3) uses affine coordinates for elliptic-curve computations.
 */

pub fn r1cs_gg_ppzksnark_affine_verifier_weak_IC<ppT: PublicParams>(
    vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: &r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_affine_verifier_weak_IC", false);
    assert!(vk.gamma_ABC_g1.domain_size() >= primary_input.len());

    let pvk_vk_gamma_g2_precomp = ppT::affine_ate_precompute_G2(&vk.gamma_g2);
    let pvk_vk_delta_g2_precomp = ppT::affine_ate_precompute_G2(&vk.delta_g2);

    enter_block("Accumulate input", false);
    let accumulated_IC = vk
        .gamma_ABC_g1
        .accumulate_chunk::<Fr<ppT>>(&primary_input, 0);
    let acc = &accumulated_IC.first;
    leave_block("Accumulate input", false);

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

    enter_block("Check QAP divisibility", false);
    let proof_g_A_precomp = ppT::affine_ate_precompute_G1(&proof.g_A);
    let proof_g_B_precomp = ppT::affine_ate_precompute_G2(&proof.g_B);
    let proof_g_C_precomp = ppT::affine_ate_precompute_G1(&proof.g_C);
    let acc_precomp = ppT::affine_ate_precompute_G1(&acc);

    let QAP_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        &acc_precomp,
        &pvk_vk_gamma_g2_precomp,
        &proof_g_C_precomp,
        &pvk_vk_delta_g2_precomp,
        &proof_g_A_precomp,
        &proof_g_B_precomp,
    );
    let QAP = ppT::final_exponentiation(&QAP_miller.unitary_inverse());

    if QAP != vk.alpha_g1_beta_g2 {
        if !inhibit_profiling_info {
            print_indent();
            print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility", false);

    leave_block("Call to r1cs_gg_ppzksnark_affine_verifier_weak_IC", false);

    result
}

impl<ppT: PublicParams> PartialEq for r1cs_gg_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.alpha_g1 == other.alpha_g1
            && self.beta_g1 == other.beta_g1
            && self.beta_g2 == other.beta_g2
            && self.delta_g1 == other.delta_g1
            && self.delta_g2 == other.delta_g2
            && self.A_query == other.A_query
            && self.B_query == other.B_query
            && self.H_query == other.H_query
            && self.L_query == other.L_query
            && self.constraint_system == other.constraint_system
    }
}

impl<ppT: PublicParams> fmt::Display for r1cs_gg_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{}{}{}{}",
            self.alpha_g1,
            self.beta_g1,
            self.beta_g2,
            self.delta_g1,
            self.delta_g2,
            self.A_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.B_query,
            self.H_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.L_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.constraint_system,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.alpha_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.beta_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.beta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.delta_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.delta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pk.A_query;
//     in >> pk.B_query;
//     in >> pk.H_query;
//     in >> pk.L_query;
//     in >> pk.constraint_system;

//     return in;
// }

impl<ppT: PublicParams> PartialEq for r1cs_gg_ppzksnark_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.alpha_g1_beta_g2 == other.alpha_g1_beta_g2
            && self.gamma_g2 == other.gamma_g2
            && self.delta_g2 == other.delta_g2
            && self.gamma_ABC_g1 == other.gamma_ABC_g1
    }
}

impl<ppT: PublicParams> fmt::Display for r1cs_gg_ppzksnark_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.alpha_g1_beta_g2, self.gamma_g2, self.delta_g2, self.gamma_ABC_g1,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.alpha_g1_beta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.delta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_ABC_g1;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: PublicParams> PartialEq for r1cs_gg_ppzksnark_processed_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.vk_alpha_g1_beta_g2 == other.vk_alpha_g1_beta_g2
            && self.vk_gamma_g2_precomp == other.vk_gamma_g2_precomp
            && self.vk_delta_g2_precomp == other.vk_delta_g2_precomp
            && self.gamma_ABC_g1 == other.gamma_ABC_g1
    }
}

impl<ppT: PublicParams> fmt::Display for r1cs_gg_ppzksnark_processed_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.vk_alpha_g1_beta_g2,
            self.vk_gamma_g2_precomp,
            self.vk_delta_g2_precomp,
            self.gamma_ABC_g1,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.vk_alpha_g1_beta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_delta_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.gamma_ABC_g1;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: PublicParams> PartialEq for r1cs_gg_ppzksnark_proof<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.g_A == other.g_A && self.g_B == other.g_B && self.g_C == other.g_C
    }
}

use std::fmt;
impl<ppT: PublicParams> fmt::Display for r1cs_gg_ppzksnark_proof<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.g_A, self.g_B, self.g_C,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.g_A;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_B;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_C;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: PublicParams> r1cs_gg_ppzksnark_verification_key<ppT> {
    pub fn dummy_verification_key(input_size: usize) -> r1cs_gg_ppzksnark_verification_key<ppT>
// where
    //     <ppT as ff_curves::PublicParams>::Fr: Mul<<ppT as ff_curves::PublicParams>::GT, Output = <ppT as ff_curves::PublicParams>::GT>,
    {
        let mut result = r1cs_gg_ppzksnark_verification_key::<ppT>::default();
        result.alpha_g1_beta_g2 = GT::<ppT>::random_element() * Fr::<ppT>::random_element();
        result.gamma_g2 = G2::<ppT>::random_element();
        result.delta_g2 = G2::<ppT>::random_element();

        let base = G1::<ppT>::random_element();
        let mut v = G1_vector::<ppT>::default();
        for i in 0..input_size {
            v.push(G1::<ppT>::random_element());
        }

        result.gamma_ABC_g1 = accumulation_vector::<G1<ppT>>::new_with_vec(base, v);

        result
    }
}
