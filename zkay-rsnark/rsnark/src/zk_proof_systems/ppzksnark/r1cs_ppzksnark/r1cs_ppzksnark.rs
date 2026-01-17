//  Declaration of interfaces for a ppzkSNARK for R1CS.

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

//  The implementation instantiates (a modification of) the protocol of \[PGHR13],
//  by following extending, and optimizing the approach described in \[BCTV14].

//  Acronyms:

//  - R1CS = "Rank-1 Constraint Systems"
//  - ppzkSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

//  References:

//  \[BCTV14]:
//  "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
//  Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
//  USENIX Security 2014,
//  <http://eprint.iacr.org/2013/879>

//  \[PGHR13]:
//  "Pinocchio: Nearly practical verifiable computation",
//  Bryan Parno, Craig Gentry, Jon Howell, Mariana Raykova,
//  IEEE S&P 2013,
//  <https://eprint.iacr.org/2013/279>

use crate::common::data_structures::accumulation_vector::accumulation_vector;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::kc_multiexp::{kc_batch_exp, kc_multi_exp_with_mixed_addition};
use crate::knowledge_commitment::knowledge_commitment::{
    knowledge_commitment, knowledge_commitment_vector,
};
use crate::reductions::r1cs_to_qap::r1cs_to_qap::{
    r1cs_to_qap_instance_map_with_evaluation, r1cs_to_qap_witness_map,
};
use crate::relations::arithmetic_programs::qap::qap::qap_instance_evaluation;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint_system;
use ffec::scalar_multiplication::multiexp::KCConfig;

use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark_params::{
    r1cs_ppzksnark_auxiliary_input, r1cs_ppzksnark_constraint_system, r1cs_ppzksnark_primary_input,
};
use ff_curves::{
    Fr, Fr_vector, G1, G1_precomp, G1_vector, G2, G2_precomp, PublicParams, PublicParamsType,
};
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::OUTPUT_NEWLINE;
use ffec::scalar_multiplication::multiexp::{
    batch_exp, get_exp_window_size, get_window_table, inhibit_profiling_info, multi_exp,
    multi_exp_method, multi_exp_with_mixed_addition,
};
use ffec::{One, PpConfig, Zero};
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::fmt;
use std::ops::{Add, Mul, Sub};
/******************************** Proving key ********************************/

/**
 * A proving key for the R1CS ppzkSNARK.
 */

#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_proving_key<ppT: ppTConfig>
// where
//     <ppT as PublicParamsType>::Fp_type: FieldTConfig,
//     <ppT as ff_curves::PublicParams>::Fr: FieldTConfig,
{
    pub A_query: KnowledgeCommitmentVector<ppT>,
    pub B_query: KnowledgeCommitmentVector2<ppT>,
    pub C_query: KnowledgeCommitmentVector<ppT>,
    pub H_query: G1_vector<ppT>,
    pub K_query: G1_vector<ppT>,

    pub constraint_system: r1cs_ppzksnark_constraint_system<ppT>,
}
impl<ppT: ppTConfig> r1cs_ppzksnark_proving_key<ppT>
// where
//     <ppT as PublicParamsType>::Fp_type: FieldTConfig,
//     <ppT as ff_curves::PublicParams>::Fr: FieldTConfig,
{
    // r1cs_ppzksnark_proving_key() {};
    // r1cs_ppzksnark_proving_key<ppT>& operator=(&other:r1cs_ppzksnark_proving_key<ppT>) = default;
    // r1cs_ppzksnark_proving_key(&other:r1cs_ppzksnark_proving_key<ppT>) = default;
    // r1cs_ppzksnark_proving_key(r1cs_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(
        A_query: KnowledgeCommitmentVector<ppT>,
        B_query: KnowledgeCommitmentVector2<ppT>,
        C_query: KnowledgeCommitmentVector<ppT>,
        H_query: G1_vector<ppT>,
        K_query: G1_vector<ppT>,
        constraint_system: r1cs_ppzksnark_constraint_system<ppT>,
    ) -> Self
// where
    //     <ppT as ff_curves::PublicParams>::Fr: FieldTConfig,
    {
        Self {
            A_query,
            B_query,
            C_query,
            H_query,
            K_query,
            constraint_system,
        }
    }

    pub fn g1_size(&self) -> usize {
        2 * (self.A_query.domain_size() + self.C_query.domain_size())
            + self.B_query.domain_size()
            + self.H_query.len()
            + self.K_query.len()
    }

    pub fn g2_size(&self) -> usize {
        self.B_query.domain_size()
    }

    pub fn g1_sparse_size(&self) -> usize {
        2 * (self.A_query.len() + self.C_query.len())
            + self.B_query.len()
            + self.H_query.len()
            + self.K_query.len()
    }

    pub fn g2_sparse_size(&self) -> usize {
        self.B_query.len()
    }

    pub fn size_in_bits(&self) -> usize {
        self.A_query.size_in_bits()
            + self.B_query.size_in_bits()
            + self.C_query.size_in_bits()
            + ffec::size_in_bits(&self.H_query)
            + ffec::size_in_bits(&self.K_query)
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

    // bool operator==(&other:r1cs_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pk:r1cs_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_proving_key<ppT> &pk);
}

/******************************* Verification key ****************************/

/**
 * A verification key for the R1CS ppzkSNARK.
 */
#[derive(Default, Clone)]
pub struct r1cs_ppzksnark_verification_key<ppT: ppTConfig>
// where
//     <ppT as PublicParamsType>::G1_type: PpConfig,
{
    pub alphaA_g2: G2<ppT>,
    pub alphaB_g1: G1<ppT>,
    pub alphaC_g2: G2<ppT>,
    pub gamma_g2: G2<ppT>,
    pub gamma_beta_g1: G1<ppT>,
    pub gamma_beta_g2: G2<ppT>,
    pub rC_Z_g2: G2<ppT>,

    pub encoded_IC_query: AccumulationVector<ppT>,
}
impl<ppT: ppTConfig> r1cs_ppzksnark_verification_key<ppT>
// where
//     <ppT as PublicParamsType>::G1_type: PpConfig,
{
    // r1cs_ppzksnark_verification_key() = default;
    pub fn new(
        alphaA_g2: G2<ppT>,
        alphaB_g1: G1<ppT>,
        alphaC_g2: G2<ppT>,
        gamma_g2: G2<ppT>,
        gamma_beta_g1: G1<ppT>,
        gamma_beta_g2: G2<ppT>,
        rC_Z_g2: G2<ppT>,
        eIC: AccumulationVector<ppT>,
    ) -> Self {
        Self {
            alphaA_g2,
            alphaB_g1,
            alphaC_g2,
            gamma_g2,
            gamma_beta_g1,
            gamma_beta_g2,
            rC_Z_g2,
            encoded_IC_query: eIC,
        }
    }

    pub fn g1_size(&self) -> usize {
        2 + self.encoded_IC_query.len()
    }

    pub fn g2_size(&self) -> usize {
        5
    }

    pub fn size_in_bits(&self) -> usize {
        (2 * G1::<ppT>::size_in_bits()
            + self.encoded_IC_query.size_in_bits()
            + 5 * G2::<ppT>::size_in_bits())
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in VK: {}\n", self.g1_size());
        print_indent();
        print!("* G2 elements in VK: {}\n", self.g2_size());
        print_indent();
        print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(&other:r1cs_ppzksnark_verification_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &vk:r1cs_ppzksnark_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_verification_key<ppT> &vk);

    // static r1cs_ppzksnark_verification_key<ppT> dummy_verification_key(input_size:usize);
}

/************************ Processed verification key *************************/

/**
 * A processed verification key for the R1CS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
#[derive(Default, Clone)]
pub struct r1cs_ppzksnark_processed_verification_key<ppT: ppTConfig>
// where
//     <ppT as PublicParamsType>::G1_type: PpConfig,
{
    pub pp_G2_one_precomp: G2_precomp<ppT>,
    pub vk_alphaA_g2_precomp: G2_precomp<ppT>,
    pub vk_alphaB_g1_precomp: G1_precomp<ppT>,
    pub vk_alphaC_g2_precomp: G2_precomp<ppT>,
    pub vk_rC_Z_g2_precomp: G2_precomp<ppT>,
    pub vk_gamma_g2_precomp: G2_precomp<ppT>,
    pub vk_gamma_beta_g1_precomp: G1_precomp<ppT>,
    pub vk_gamma_beta_g2_precomp: G2_precomp<ppT>,

    pub encoded_IC_query: AccumulationVector<ppT>,
    // bool operator==(&other:r1cs_ppzksnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pvk:r1cs_ppzksnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_processed_verification_key<ppT> &pvk);
}

/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS ppzkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_keypair<ppT: ppTConfig>
// where
//     <ppT as PublicParamsType>::Fp_type: FieldTConfig,
//     <ppT as PublicParamsType>::G1_type: PpConfig,
//     <ppT as ff_curves::PublicParams>::Fr: FieldTConfig,
{
    pub pk: r1cs_ppzksnark_proving_key<ppT>,
    pub vk: r1cs_ppzksnark_verification_key<ppT>,
}
impl<ppT: ppTConfig> r1cs_ppzksnark_keypair<ppT>
// where
//     <ppT as PublicParamsType>::Fp_type: FieldTConfig,
//     <ppT as PublicParamsType>::G1_type: PpConfig,
{
    // r1cs_ppzksnark_keypair() = default;
    // r1cs_ppzksnark_keypair(&other:r1cs_ppzksnark_keypair<ppT>) = default;
    pub fn new(
        pk: r1cs_ppzksnark_proving_key<ppT>,
        vk: r1cs_ppzksnark_verification_key<ppT>,
    ) -> Self {
        Self { pk, vk }
    }

    // r1cs_ppzksnark_keypair(r1cs_ppzksnark_keypair<ppT> &&other) = default;
}

pub type T1<PP> = <<PP as ppTConfig>::KC as KCConfig>::T;
pub type T2<PP> = <<PP as ppTConfig>::KC as KCConfig>::T2;
pub type FieldT<PP> = <<PP as ppTConfig>::KC as KCConfig>::FieldT;
pub type KnowledgeCommitmentVector<PP> = knowledge_commitment_vector<T1<PP>, T1<PP>>;
pub type KnowledgeCommitmentVector2<PP> = knowledge_commitment_vector<T2<PP>, T1<PP>>;
pub type KnowledgeCommitment<PP> = knowledge_commitment<T1<PP>, T1<PP>>;
pub type KnowledgeCommitment2<PP> = knowledge_commitment<T2<PP>, T1<PP>>;
pub type AccumulationVector<PP> = accumulation_vector<T1<PP>>;

/*********************************** Proof ***********************************/

/**
 * A proof for the R1CS ppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_proof<ppT: ppTConfig> {
    pub g_A: KnowledgeCommitment<ppT>,
    pub g_B: KnowledgeCommitment2<ppT>,
    pub g_C: KnowledgeCommitment<ppT>,
    pub g_H: G1<ppT>,
    pub g_K: G1<ppT>,
}
impl<ppT: ppTConfig> r1cs_ppzksnark_proof<ppT> {
    pub fn default() -> Self {
        // invalid proof with valid curve points
        Self {
            g_A: knowledge_commitment::new(G1::<ppT>::one(), G1::<ppT>::one()),
            g_B: knowledge_commitment::new(G2::<ppT>::one(), G1::<ppT>::one()),
            g_C: knowledge_commitment::new(G1::<ppT>::one(), G1::<ppT>::one()),
            g_H: G1::<ppT>::one(),
            g_K: G1::<ppT>::one(),
        }
    }
    pub fn new(
        g_A: KnowledgeCommitment<ppT>,
        g_B: KnowledgeCommitment2<ppT>,
        g_C: KnowledgeCommitment<ppT>,
        g_H: G1<ppT>,
        g_K: G1<ppT>,
    ) -> Self
// where
    //     <ppT as ff_curves::PublicParams>::G2: PpConfig,
    {
        Self {
            g_A,
            g_B,
            g_C,
            g_H,
            g_K,
        }
    }

    pub fn g1_size() -> usize {
        7
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
        self.g_A.g.is_well_formed()
            && self.g_A.h.is_well_formed()
            && self.g_B.g.is_well_formed()
            && self.g_B.h.is_well_formed()
            && self.g_C.g.is_well_formed()
            && self.g_C.h.is_well_formed()
            && self.g_H.is_well_formed()
            && self.g_K.is_well_formed()
    }

    // bool operator==(&other:r1cs_ppzksnark_proof<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &proof:r1cs_ppzksnark_proof<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_proof<ppT> &proof);
}

/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the R1CS ppzkSNARK.
 *
 * Given a R1CS constraint system CS, this algorithm produces proving and verification keys for CS.
 */

pub fn r1cs_ppzksnark_generator<ppT: ppTConfig>(
    cs: &r1cs_ppzksnark_constraint_system<ppT>,
) -> r1cs_ppzksnark_keypair<ppT>
// where
//     <ppT as PublicParamsType>::Fp_type: FieldTConfig,
//     <ppT as PublicParamsType>::G1_type: PpConfig,
//     <ppT as ff_curves::PublicParams>::Fr: FieldTConfig,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G2:
//         Add<Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
//     ED: fqfft::evaluation_domain::evaluation_domain::evaluation_domain<
//             <ppT as ff_curves::PublicParams>::Fr,
//         >,
{
    enter_block("Call to r1cs_ppzksnark_generator", false);

    /* make the B_query "lighter" if possible */
    let mut cs_copy = cs.clone();
    cs_copy.swap_AB_if_beneficial();

    /* draw random element at which the QAP is evaluated */
    let t = Fr::<ppT>::random_element();

    let qap_inst: qap_instance_evaluation<_> =
        r1cs_to_qap_instance_map_with_evaluation::<Fr<ppT>, pb_variable, pb_linear_combination>(
            &cs_copy, &t,
        );

    print_indent();
    print!("* QAP number of variables: {}\n", qap_inst.num_variables());
    print_indent();
    print!("* QAP pre degree: {}\n", cs_copy.constraints.len());
    print_indent();
    print!("* QAP degree: {}\n", qap_inst.degree());
    print_indent();
    print!(
        "* QAP number of input variables: {}\n",
        qap_inst.num_inputs()
    );

    enter_block("Compute query densities", false);
    let (mut non_zero_At, mut non_zero_Bt, mut non_zero_Ct, mut non_zero_Ht) = (0, 0, 0, 0);
    for i in 0..qap_inst.num_variables() + 1 {
        if !qap_inst.At[i].is_zero() {
            non_zero_At += 1;
        }
        if !qap_inst.Bt[i].is_zero() {
            non_zero_Bt += 1;
        }
        if !qap_inst.Ct[i].is_zero() {
            non_zero_Ct += 1;
        }
    }
    for i in 0..qap_inst.degree() + 1 {
        if !qap_inst.Ht[i].is_zero() {
            non_zero_Ht += 1;
        }
    }
    leave_block("Compute query densities", false);

    let mut At = qap_inst.At.clone(); // qap_inst.At is now in unspecified state, but we do not use it later
    let mut Bt = qap_inst.Bt.clone(); // qap_inst.Bt is now in unspecified state, but we do not use it later
    let mut Ct = qap_inst.Ct.clone(); // qap_inst.Ct is now in unspecified state, but we do not use it later
    let mut Ht = qap_inst.Ht.clone(); // qap_inst.Ht is now in unspecified state, but we do not use it later

    /* append Zt to At,Bt,Ct with */
    At.push(qap_inst.Zt.clone());
    Bt.push(qap_inst.Zt.clone());
    Ct.push(qap_inst.Zt.clone());

    let (alphaA, alphaB, alphaC, rA, rB, beta, gamma) = (
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
    );
    let rC = rA.clone() * rB.clone();

    // consrtuct the same-coefficient-check query (must happen before zeroing out the prefix of At)
    let mut Kt = Fr_vector::<ppT>::default();
    Kt.reserve(qap_inst.num_variables() + 4);
    for i in 0..qap_inst.num_variables() + 1 {
        Kt.push(
            beta.clone()
                * (rA.clone() * At[i].clone()
                    + rB.clone() * Bt[i].clone()
                    + rC.clone() * Ct[i].clone()),
        );
    }
    Kt.push(beta.clone() * rA.clone() * qap_inst.Zt.clone());
    Kt.push(beta.clone() * rB.clone() * qap_inst.Zt.clone());
    Kt.push(beta.clone() * rC.clone() * qap_inst.Zt.clone());

    /* zero out prefix of At and stick it into IC coefficients */
    let mut IC_coefficients = Fr_vector::<ppT>::default();
    IC_coefficients.reserve(qap_inst.num_inputs() + 1);
    for i in 0..qap_inst.num_inputs() + 1 {
        IC_coefficients.push(At[i].clone());
        assert!(!IC_coefficients[i].is_zero());
        At[i] = Fr::<ppT>::zero();
    }

    let g1_exp_count = 2 * (non_zero_At - qap_inst.num_inputs() + non_zero_Ct)
        + non_zero_Bt
        + non_zero_Ht
        + Kt.len();
    let g2_exp_count = non_zero_Bt;

    let g1_window = get_exp_window_size::<G1<ppT>>(g1_exp_count);
    let g2_window = get_exp_window_size::<G2<ppT>>(g2_exp_count);
    print_indent();
    print!("* G1 window: {}\n", g1_window);
    print_indent();
    print!("* G2 window: {}\n", g2_window);

    // // #ifdef MULTICORE
    //     let  chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    // //#endif

    enter_block("Generating G1 multiexp table", false);
    let g1_table = get_window_table(Fr::<ppT>::size_in_bits(), g1_window, G1::<ppT>::one());
    leave_block("Generating G1 multiexp table", false);

    enter_block("Generating G2 multiexp table", false);
    let g2_table = get_window_table(Fr::<ppT>::size_in_bits(), g2_window, G2::<ppT>::one());
    leave_block("Generating G2 multiexp table", false);

    enter_block("Generate R1CS proving key", false);

    enter_block("Generate knowledge commitments", false);
    enter_block("Compute the A-query", false);
    let A_query = kc_batch_exp::<
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
    >(
        Fr::<ppT>::size_in_bits(),
        g1_window,
        g1_window,
        &g1_table,
        &g1_table,
        &rA,
        &(rA.clone() * alphaA.clone()),
        &At,
        chunks,
    );
    leave_block("Compute the A-query", false);

    enter_block("Compute the B-query", false);
    let B_query = kc_batch_exp::<
        <<ppT as ppTConfig>::KC as KCConfig>::T2,
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
    >(
        Fr::<ppT>::size_in_bits(),
        g2_window,
        g1_window,
        &g2_table,
        &g1_table,
        &rB,
        &(rB.clone() * alphaB.clone()),
        &Bt,
        chunks,
    );
    leave_block("Compute the B-query", false);

    enter_block("Compute the C-query", false);
    let C_query = kc_batch_exp::<
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
    >(
        Fr::<ppT>::size_in_bits(),
        g1_window,
        g1_window,
        &g1_table,
        &g1_table,
        &rC,
        &(rC.clone() * alphaC.clone()),
        &Ct,
        chunks,
    );
    leave_block("Compute the C-query", false);

    enter_block("Compute the H-query", false);
    let H_query = batch_exp::<
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
    >(Fr::<ppT>::size_in_bits(), g1_window, &g1_table, &Ht);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(H_query);
    // //#endif
    leave_block("Compute the H-query", false);

    enter_block("Compute the K-query", false);
    let K_query = batch_exp::<
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
    >(Fr::<ppT>::size_in_bits(), g1_window, &g1_table, &Kt);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<G1<ppT> >(K_query);
    // //#endif
    leave_block("Compute the K-query", false);

    leave_block("Generate knowledge commitments", false);

    leave_block("Generate R1CS proving key", false);

    enter_block("Generate R1CS verification key", false);
    let alphaA_g2 = G2::<ppT>::one() * alphaA.clone();
    let alphaB_g1 = G1::<ppT>::one() * alphaB.clone();
    let alphaC_g2 = G2::<ppT>::one() * alphaC.clone();
    let gamma_g2 = G2::<ppT>::one() * gamma.clone();
    let gamma_beta_g1 = G1::<ppT>::one() * (gamma.clone() * beta.clone());
    let gamma_beta_g2 = G2::<ppT>::one() * (gamma.clone() * beta.clone());
    let rC_Z_g2 = G2::<ppT>::one() * (rC.clone() * qap_inst.Zt.clone());

    enter_block("Encode IC query for R1CS verification key", false);
    let encoded_IC_base = G1::<ppT>::one() * (rA.clone() * IC_coefficients[0].clone());
    let mut multiplied_IC_coefficients = Fr_vector::<ppT>::default();
    multiplied_IC_coefficients.reserve(qap_inst.num_inputs());
    for i in 1..qap_inst.num_inputs() + 1 {
        multiplied_IC_coefficients.push(rA.clone() * IC_coefficients[i].clone());
    }
    let encoded_IC_values = batch_exp::<
        <<ppT as ppTConfig>::KC as KCConfig>::T,
        <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
    >(
        Fr::<ppT>::size_in_bits(),
        g1_window,
        &g1_table,
        &multiplied_IC_coefficients,
    );

    leave_block("Encode IC query for R1CS verification key", false);
    leave_block("Generate R1CS verification key", false);

    leave_block("Call to r1cs_ppzksnark_generator", false);

    let mut encoded_IC_query =
        accumulation_vector::<G1<ppT>>::new_with_vec((encoded_IC_base), (encoded_IC_values));

    let mut vk = r1cs_ppzksnark_verification_key::<ppT>::new(
        alphaA_g2,
        alphaB_g1,
        alphaC_g2,
        gamma_g2,
        gamma_beta_g1,
        gamma_beta_g2,
        rC_Z_g2,
        encoded_IC_query,
    );
    let mut pk = r1cs_ppzksnark_proving_key::<ppT>::new(
        A_query, B_query, C_query, H_query, K_query, cs_copy,
    );

    pk.print_size();
    vk.print_size();

    r1cs_ppzksnark_keypair::<ppT>::new(pk, vk)
}

/**
 * A prover algorithm for the R1CS ppzkSNARK.
 *
 * Given a R1CS primary input X and a R1CS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the R1CS constraint system that was given as input to the generator algorithm.
 */

pub fn r1cs_ppzksnark_prover<ppT: ppTConfig>(
    pk: &r1cs_ppzksnark_proving_key<ppT>,
    primary_input: &r1cs_ppzksnark_primary_input<ppT>,
    auxiliary_input: &r1cs_ppzksnark_auxiliary_input<ppT>,
) -> r1cs_ppzksnark_proof<ppT>
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
        >, // where
           //     <ppT as PublicParamsType>::Fp_type: FieldTConfig,
           //     <ppT as ff_curves::PublicParams>::Fr: FieldTConfig,
           //     <ppT as ff_curves::PublicParams>::Fr: Mul<
           //             knowledge_commitment<
           //                 <ppT as ff_curves::PublicParams>::G1,
           //                 <ppT as ff_curves::PublicParams>::G1,
           //             >,
           //             Output = knowledge_commitment<
           //                 <ppT as ff_curves::PublicParams>::G1,
           //                 <ppT as ff_curves::PublicParams>::G1,
           //             >,
           //         >,
           //     <ppT as ff_curves::PublicParams>::Fr:
           //         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
           //     <ppT as ff_curves::PublicParams>::Fr: Mul<
           //             knowledge_commitment<
           //                 <ppT as ff_curves::PublicParams>::G2,
           //                 <ppT as ff_curves::PublicParams>::G1,
           //             >,
           //             Output = knowledge_commitment<
           //                 <ppT as ff_curves::PublicParams>::G2,
           //                 <ppT as ff_curves::PublicParams>::G1,
           //             >,
           //         >,
           //     ED: evaluation_domain<<ppT as PublicParams>::Fr>,
{
    enter_block("Call to r1cs_ppzksnark_prover", false);

    // // #ifdef DEBUG
    //     assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
    // //#endif
    let (d1, d2, d3) = (
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
        Fr::<ppT>::random_element(),
    );

    enter_block("Compute the polynomial H", false);
    let qap_wit = r1cs_to_qap_witness_map::<
        <ppT as ff_curves::PublicParams>::Fr,
        pb_variable,
        pb_linear_combination,
    >(
        &pk.constraint_system,
        &primary_input,
        &auxiliary_input,
        &d1,
        &d2,
        &d3,
    );
    leave_block("Compute the polynomial H", false);

    // // #ifdef DEBUG
    //     Fr::<ppT>::random_element(:Fr<ppT> t =);
    //     qap_instance_evaluation<Fr<ppT> > qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    //     assert!(qap_inst.is_satisfied(qap_wit));
    // //#endif

    let mut g_A = pk.A_query[0].clone()
        + pk.A_query[qap_wit.num_variables() + 1].clone() * qap_wit.d1.clone();
    let mut g_B = pk.B_query[0].clone()
        + pk.B_query[qap_wit.num_variables() + 1].clone() * qap_wit.d2.clone();
    let mut g_C = pk.C_query[0].clone()
        + pk.C_query[qap_wit.num_variables() + 1].clone() * qap_wit.d3.clone();

    let mut g_H = G1::<ppT>::zero();
    let mut g_K = pk.K_query[0].clone()
        + pk.K_query[qap_wit.num_variables() + 1].clone() * qap_wit.d1.clone()
        + pk.K_query[qap_wit.num_variables() + 2].clone() * qap_wit.d2.clone()
        + pk.K_query[qap_wit.num_variables() + 3].clone() * qap_wit.d3.clone();

    // // #ifdef DEBUG
    //     for i in 0..qap_wit.num_inputs() + 1
    //     {
    //         assert!(pk.A_query[i].g == G1::<ppT>::zero());
    //     }
    //     assert!(pk.A_query.domain_size() == qap_wit.num_variables()+2);
    //     assert!(pk.B_query.domain_size() == qap_wit.num_variables()+2);
    //     assert!(pk.C_query.domain_size() == qap_wit.num_variables()+2);
    //     assert!(pk.H_query.len() == qap_wit.degree()+1);
    //     assert!(pk.K_query.len() == qap_wit.num_variables()+4);
    // //#endif

    // // #ifdef MULTICORE
    //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    // //#endif

    enter_block("Compute the proof", false);

    enter_block("Compute answer to A-query", false);
    g_A = g_A
        + kc_multi_exp_with_mixed_addition::<
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
            { multi_exp_method::multi_exp_method_bos_coster },
        >(
            &pk.A_query,
            1,
            1 + qap_wit.num_variables(),
            &qap_wit.coefficients_for_ABCs[..qap_wit.num_variables()],
            chunks,
        );
    leave_block("Compute answer to A-query", false);

    enter_block("Compute answer to B-query", false);
    g_B = g_B
        + kc_multi_exp_with_mixed_addition::<
            <<ppT as ppTConfig>::KC as KCConfig>::T2,
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
            { multi_exp_method::multi_exp_method_bos_coster },
        >(
            &pk.B_query,
            1,
            1 + qap_wit.num_variables(),
            &qap_wit.coefficients_for_ABCs[..qap_wit.num_variables()],
            chunks,
        );
    leave_block("Compute answer to B-query", false);

    enter_block("Compute answer to C-query", false);
    g_C = g_C
        + kc_multi_exp_with_mixed_addition::<
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
            { multi_exp_method::multi_exp_method_bos_coster },
        >(
            &pk.C_query,
            1,
            1 + qap_wit.num_variables(),
            &qap_wit.coefficients_for_ABCs[..qap_wit.num_variables()],
            chunks,
        );
    leave_block("Compute answer to C-query", false);

    enter_block("Compute answer to H-query", false);
    g_H = g_H
        + multi_exp::<
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
            { multi_exp_method::multi_exp_method_BDLO12 },
        >(
            &pk.H_query[..qap_wit.degree() + 1],
            &qap_wit.coefficients_for_H[..qap_wit.degree() + 1],
            chunks,
        );
    leave_block("Compute answer to H-query", false);

    enter_block("Compute answer to K-query", false);
    g_K = g_K
        + multi_exp_with_mixed_addition::<
            <<ppT as ppTConfig>::KC as KCConfig>::T,
            <<ppT as ppTConfig>::KC as KCConfig>::FieldT,
            { multi_exp_method::multi_exp_method_bos_coster },
        >(
            &pk.K_query[1..1 + qap_wit.num_variables()],
            &qap_wit.coefficients_for_ABCs[..qap_wit.num_variables()],
            chunks,
        );
    leave_block("Compute answer to K-query", false);

    leave_block("Compute the proof", false);

    leave_block("Call to r1cs_ppzksnark_prover", false);

    let proof = r1cs_ppzksnark_proof::<ppT>::new(g_A, g_B, g_C, g_H, g_K);
    r1cs_ppzksnark_proof::<ppT>::print_size();

    proof
}

/*
Below are four variants of verifier algorithm for the R1CS ppzkSNARK.

These are the four cases that arise from the following two choices:

(1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
    In the latter case, we call the algorithm an "online verifier".

(2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
    Strong input consistency requires that |primary_input| = CS.num_inputs, whereas
    weak input consistency requires that |primary_input| <= CS.num_inputs (and
    the primary input is implicitly padded with zeros up to length CS.num_inputs).
*/

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */

pub fn r1cs_ppzksnark_verifier_weak_IC<ppT: ppTConfig>(
    vk: &r1cs_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_ppzksnark_primary_input<ppT>,
    proof: &r1cs_ppzksnark_proof<ppT>,
) -> bool
// where
//     <ppT as PublicParamsType>::G1_type: PpConfig,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
{
    enter_block("Call to r1cs_ppzksnark_verifier_weak_IC", false);
    let pvk = r1cs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let result = r1cs_ppzksnark_online_verifier_weak_IC::<ppT>(&pvk, &primary_input, &proof);
    leave_block("Call to r1cs_ppzksnark_verifier_weak_IC", false);
    result
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_ppzksnark_verifier_strong_IC<ppT: ppTConfig>(
    vk: &r1cs_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_ppzksnark_primary_input<ppT>,
    proof: &r1cs_ppzksnark_proof<ppT>,
) -> bool
// where
//     <ppT as PublicParamsType>::G1_type: PpConfig,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
{
    enter_block("Call to r1cs_ppzksnark_verifier_strong_IC", false);
    let pvk = r1cs_ppzksnark_verifier_process_vk::<ppT>(vk);
    let result = r1cs_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &primary_input, &proof);
    leave_block("Call to r1cs_ppzksnark_verifier_strong_IC", false);
    result
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
pub fn r1cs_ppzksnark_verifier_process_vk<ppT: ppTConfig>(
    vk: &r1cs_ppzksnark_verification_key<ppT>,
) -> r1cs_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to r1cs_ppzksnark_verifier_process_vk", false);

    let mut pvk = r1cs_ppzksnark_processed_verification_key::<ppT>::default();
    pvk.pp_G2_one_precomp = ppT::precompute_G2(&G2::<ppT>::one());
    pvk.vk_alphaA_g2_precomp = ppT::precompute_G2(&vk.alphaA_g2);
    pvk.vk_alphaB_g1_precomp = ppT::precompute_G1(&vk.alphaB_g1);
    pvk.vk_alphaC_g2_precomp = ppT::precompute_G2(&vk.alphaC_g2);
    pvk.vk_rC_Z_g2_precomp = ppT::precompute_G2(&vk.rC_Z_g2);
    pvk.vk_gamma_g2_precomp = ppT::precompute_G2(&vk.gamma_g2);
    pvk.vk_gamma_beta_g1_precomp = ppT::precompute_G1(&vk.gamma_beta_g1);
    pvk.vk_gamma_beta_g2_precomp = ppT::precompute_G2(&vk.gamma_beta_g2);

    pvk.encoded_IC_query = vk.encoded_IC_query.clone();

    leave_block("Call to r1cs_ppzksnark_verifier_process_vk", false);

    pvk
}
/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

pub fn r1cs_ppzksnark_online_verifier_weak_IC<ppT: ppTConfig>(
    pvk: &r1cs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &r1cs_ppzksnark_primary_input<ppT>,
    proof: &r1cs_ppzksnark_proof<ppT>,
) -> bool
// where
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1: Add<&'a <ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
{
    enter_block("Call to r1cs_ppzksnark_online_verifier_weak_IC", false);
    assert!(pvk.encoded_IC_query.domain_size() >= primary_input.len());

    enter_block("Compute input-dependent part of A", false);
    let accumulated_IC = pvk
        .encoded_IC_query
        .accumulate_chunk::<ppT::Fr>(primary_input, 0);
    let acc = &accumulated_IC.first;
    leave_block("Compute input-dependent part of A", false);

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
    enter_block("Check knowledge commitment for A is valid", false);
    let proof_g_A_g_precomp = ppT::precompute_G1(&proof.g_A.g);
    let proof_g_A_h_precomp = ppT::precompute_G1(&proof.g_A.h);
    let kc_A_1 = ppT::miller_loop(&proof_g_A_g_precomp, &pvk.vk_alphaA_g2_precomp);
    let kc_A_2 = ppT::miller_loop(&proof_g_A_h_precomp, &pvk.pp_G2_one_precomp);
    let kc_A = ppT::final_exponentiation(&(kc_A_1 * kc_A_2.unitary_inverse()));
    if kc_A != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Knowledge commitment for A query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for A is valid", false);

    enter_block("Check knowledge commitment for B is valid", false);
    let proof_g_B_g_precomp = ppT::precompute_G2(&proof.g_B.g);
    let proof_g_B_h_precomp = ppT::precompute_G1(&proof.g_B.h);
    let kc_B_1 = ppT::miller_loop(&pvk.vk_alphaB_g1_precomp, &proof_g_B_g_precomp);
    let kc_B_2 = ppT::miller_loop(&proof_g_B_h_precomp, &pvk.pp_G2_one_precomp);
    let kc_B = ppT::final_exponentiation(&(kc_B_1 * kc_B_2.unitary_inverse()));
    if kc_B != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Knowledge commitment for B query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for B is valid", false);

    enter_block("Check knowledge commitment for C is valid", false);
    let proof_g_C_g_precomp = ppT::precompute_G1(&proof.g_C.g);
    let proof_g_C_h_precomp = ppT::precompute_G1(&proof.g_C.h);
    let kc_C_1 = ppT::miller_loop(&proof_g_C_g_precomp, &pvk.vk_alphaC_g2_precomp);
    let kc_C_2 = ppT::miller_loop(&proof_g_C_h_precomp, &pvk.pp_G2_one_precomp);
    let kc_C = ppT::final_exponentiation(&(kc_C_1 * kc_C_2.unitary_inverse()));
    if kc_C != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Knowledge commitment for C query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for C is valid", false);

    enter_block("Check QAP divisibility", false);
    // check that g^((A+acc)*B)=g^(H*\Prod(t-\sigma)+C)
    // equivalently, via pairings, that e(g^(A+acc), g^B) = e(g^H, g^Z) + e(g^C, g^1)
    let proof_g_A_g_acc_precomp = ppT::precompute_G1(&(proof.g_A.g.clone() + acc.clone()));
    let proof_g_H_precomp = ppT::precompute_G1(&proof.g_H);
    let QAP_1 = ppT::miller_loop(&proof_g_A_g_acc_precomp, &proof_g_B_g_precomp);
    let QAP_23 = ppT::double_miller_loop(
        &proof_g_H_precomp,
        &pvk.vk_rC_Z_g2_precomp,
        &proof_g_C_g_precomp,
        &pvk.pp_G2_one_precomp,
    );
    let QAP = ppT::final_exponentiation(&(QAP_1 * QAP_23.unitary_inverse()));
    if QAP != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility", false);

    enter_block("Check same coefficients were used", false);
    let proof_g_K_precomp = ppT::precompute_G1(&proof.g_K);
    let proof_g_A_g_acc_C_precomp =
        ppT::precompute_G1(&((proof.g_A.g.clone() + acc.clone()) + proof.g_C.g.clone()));
    let K_1 = ppT::miller_loop(&proof_g_K_precomp, &pvk.vk_gamma_g2_precomp);
    let K_23 = ppT::double_miller_loop(
        &proof_g_A_g_acc_C_precomp,
        &pvk.vk_gamma_beta_g2_precomp,
        &pvk.vk_gamma_beta_g1_precomp,
        &proof_g_B_g_precomp,
    );
    let K = ppT::final_exponentiation(&(K_1 * K_23.unitary_inverse()));
    if K != ppT::GT::one() {
        if !inhibit_profiling_info {
            print_indent();
            print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    leave_block("Check same coefficients were used", false);
    leave_block("Online pairing computations", false);
    leave_block("Call to r1cs_ppzksnark_online_verifier_weak_IC", false);

    result
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_ppzksnark_online_verifier_strong_IC<ppT: ppTConfig>(
    pvk: &r1cs_ppzksnark_processed_verification_key<ppT>,
    primary_input: &r1cs_ppzksnark_primary_input<ppT>,
    proof: &r1cs_ppzksnark_proof<ppT>,
) -> bool
// where
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
{
    let mut result = true;
    enter_block("Call to r1cs_ppzksnark_online_verifier_strong_IC", false);

    if pvk.encoded_IC_query.domain_size() != primary_input.len() {
        print_indent();
        print!(
            "Input length differs from expected (got {}, expected {}).\n",
            primary_input.len(),
            pvk.encoded_IC_query.domain_size()
        );
        result = false;
    } else {
        result = r1cs_ppzksnark_online_verifier_weak_IC::<ppT>(pvk, primary_input, proof);
    }

    leave_block("Call to r1cs_ppzksnark_online_verifier_strong_IC", false);
    result
}

/****************************** Miscellaneous ********************************/

/**
 * For debugging purposes (of r1cs_ppzksnark_r1cs_ppzksnark_verifier_gadget):
 *
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a non-processed verification key,
 * (2) has weak input consistency, and
 * (3) uses affine coordinates for elliptic-curve computations.
 */

pub fn r1cs_ppzksnark_affine_verifier_weak_IC<ppT: ppTConfig>(
    vk: &r1cs_ppzksnark_verification_key<ppT>,
    primary_input: &r1cs_ppzksnark_primary_input<ppT>,
    proof: &r1cs_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_ppzksnark_affine_verifier_weak_IC", false);
    assert!(vk.encoded_IC_query.domain_size() >= primary_input.len());

    let pvk_pp_G2_one_precomp = ppT::affine_ate_precompute_G2(&G2::<ppT>::one());
    let pvk_vk_alphaA_g2_precomp = ppT::affine_ate_precompute_G2(&vk.alphaA_g2);
    let pvk_vk_alphaB_g1_precomp = ppT::affine_ate_precompute_G1(&vk.alphaB_g1);
    let pvk_vk_alphaC_g2_precomp = ppT::affine_ate_precompute_G2(&vk.alphaC_g2);
    let pvk_vk_rC_Z_g2_precomp = ppT::affine_ate_precompute_G2(&vk.rC_Z_g2);
    let pvk_vk_gamma_g2_precomp = ppT::affine_ate_precompute_G2(&vk.gamma_g2);
    let pvk_vk_gamma_beta_g1_precomp = ppT::affine_ate_precompute_G1(&vk.gamma_beta_g1);
    let pvk_vk_gamma_beta_g2_precomp = ppT::affine_ate_precompute_G2(&vk.gamma_beta_g2);

    enter_block("Compute input-dependent part of A", false);
    let accumulated_IC = vk
        .encoded_IC_query
        .accumulate_chunk::<ppT::Fr>(primary_input, 0);
    assert!(accumulated_IC.is_fully_accumulated());
    let acc = &accumulated_IC.first;
    leave_block("Compute input-dependent part of A", false);

    let mut result = true;
    enter_block("Check knowledge commitment for A is valid", false);
    let proof_g_A_g_precomp = ppT::affine_ate_precompute_G1(&proof.g_A.g);
    let proof_g_A_h_precomp = ppT::affine_ate_precompute_G1(&proof.g_A.h);
    let kc_A_miller = ppT::affine_ate_e_over_e_miller_loop(
        &proof_g_A_g_precomp,
        &pvk_vk_alphaA_g2_precomp,
        &proof_g_A_h_precomp,
        &pvk_pp_G2_one_precomp,
    );
    let kc_A = ppT::final_exponentiation(&kc_A_miller);

    if kc_A != ppT::GT::one() {
        print_indent();
        print!("Knowledge commitment for A query incorrect.\n");
        result = false;
    }
    leave_block("Check knowledge commitment for A is valid", false);

    enter_block("Check knowledge commitment for B is valid", false);
    let proof_g_B_g_precomp = ppT::affine_ate_precompute_G2(&proof.g_B.g);
    let proof_g_B_h_precomp = ppT::affine_ate_precompute_G1(&proof.g_B.h);
    let kc_B_miller = ppT::affine_ate_e_over_e_miller_loop(
        &pvk_vk_alphaB_g1_precomp,
        &proof_g_B_g_precomp,
        &proof_g_B_h_precomp,
        &pvk_pp_G2_one_precomp,
    );
    let kc_B = ppT::final_exponentiation(&kc_B_miller);
    if kc_B != ppT::GT::one() {
        print_indent();
        print!("Knowledge commitment for B query incorrect.\n");
        result = false;
    }
    leave_block("Check knowledge commitment for B is valid", false);

    enter_block("Check knowledge commitment for C is valid", false);
    let proof_g_C_g_precomp = ppT::affine_ate_precompute_G1(&proof.g_C.g);
    let proof_g_C_h_precomp = ppT::affine_ate_precompute_G1(&proof.g_C.h);
    let kc_C_miller = ppT::affine_ate_e_over_e_miller_loop(
        &proof_g_C_g_precomp,
        &pvk_vk_alphaC_g2_precomp,
        &proof_g_C_h_precomp,
        &pvk_pp_G2_one_precomp,
    );
    let kc_C = ppT::final_exponentiation(&kc_C_miller);
    if kc_C != ppT::GT::one() {
        print_indent();
        print!("Knowledge commitment for C query incorrect.\n");
        result = false;
    }
    leave_block("Check knowledge commitment for C is valid", false);

    enter_block("Check QAP divisibility", false);
    let proof_g_A_g_acc_precomp =
        ppT::affine_ate_precompute_G1(&(proof.g_A.g.clone() + acc.clone()));
    let proof_g_H_precomp = ppT::affine_ate_precompute_G1(&proof.g_H);
    let QAP_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        &proof_g_H_precomp,
        &pvk_vk_rC_Z_g2_precomp,
        &proof_g_C_g_precomp,
        &pvk_pp_G2_one_precomp,
        &proof_g_A_g_acc_precomp,
        &proof_g_B_g_precomp,
    );
    let QAP = ppT::final_exponentiation(&QAP_miller);
    if QAP != ppT::GT::one() {
        print_indent();
        print!("QAP divisibility check failed.\n");
        result = false;
    }
    leave_block("Check QAP divisibility", false);

    enter_block("Check same coefficients were used", false);
    let proof_g_K_precomp = ppT::affine_ate_precompute_G1(&proof.g_K);
    let proof_g_A_g_acc_C_precomp =
        ppT::affine_ate_precompute_G1(&((proof.g_A.g.clone() + acc.clone()) + proof.g_C.g.clone()));
    let K_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        &proof_g_A_g_acc_C_precomp,
        &pvk_vk_gamma_beta_g2_precomp,
        &pvk_vk_gamma_beta_g1_precomp,
        &proof_g_B_g_precomp,
        &proof_g_K_precomp,
        &pvk_vk_gamma_g2_precomp,
    );
    let K = ppT::final_exponentiation(&K_miller);
    if K != ppT::GT::one() {
        print_indent();
        print!("Same-coefficient check failed.\n");
        result = false;
    }
    leave_block("Check same coefficients were used", false);

    leave_block("Call to r1cs_ppzksnark_affine_verifier_weak_IC", false);

    result
}

// use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;

// use algebra::scalar_multiplication::multiexp;
// use common::profiling;
// use common::utils;

// // #ifdef MULTICORE
// use  <omp.h>
// //#endif

// use crate::knowledge_commitment::kc_multiexp;
// use crate::reductions::r1cs_to_qap::r1cs_to_qap;

impl<ppT: ppTConfig> PartialEq for r1cs_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A_query == other.A_query
            && self.B_query == other.B_query
            && self.C_query == other.C_query
            && self.H_query == other.H_query
            && self.K_query == other.K_query
            && self.constraint_system == other.constraint_system
    }
}

impl<ppT: ppTConfig> fmt::Display for r1cs_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}",
            self.A_query,
            self.B_query,
            self.C_query,
            self.H_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.K_query
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .concat(),
            self.constraint_system,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.A_query;
//     in >> pk.B_query;
//     in >> pk.C_query;
//     in >> pk.H_query;
//     in >> pk.K_query;
//     in >> pk.constraint_system;

//     return in;
// }

impl<ppT: ppTConfig> PartialEq for r1cs_ppzksnark_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.alphaA_g2 == other.alphaA_g2
            && self.alphaB_g1 == other.alphaB_g1
            && self.alphaC_g2 == other.alphaC_g2
            && self.gamma_g2 == other.gamma_g2
            && self.gamma_beta_g1 == other.gamma_beta_g1
            && self.gamma_beta_g2 == other.gamma_beta_g2
            && self.rC_Z_g2 == other.rC_Z_g2
            && self.encoded_IC_query == other.encoded_IC_query
    }
}

impl<ppT: ppTConfig> fmt::Display for r1cs_ppzksnark_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.alphaA_g2,
            self.alphaB_g1,
            self.alphaC_g2,
            self.gamma_g2,
            self.gamma_beta_g1,
            self.gamma_beta_g2,
            self.rC_Z_g2,
            self.encoded_IC_query,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.alphaA_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.alphaB_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.alphaC_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_beta_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_beta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.rC_Z_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.encoded_IC_query;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }
impl<ppT: ppTConfig> r1cs_ppzksnark_processed_verification_key<ppT>
// where
//     <ppT as PublicParamsType>::G1_type: PpConfig,
{
    pub fn size_in_bits(&self) -> usize {
        0
    }
}
impl<ppT: ppTConfig> PartialEq for r1cs_ppzksnark_processed_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pp_G2_one_precomp == other.pp_G2_one_precomp
            && self.vk_alphaA_g2_precomp == other.vk_alphaA_g2_precomp
            && self.vk_alphaB_g1_precomp == other.vk_alphaB_g1_precomp
            && self.vk_alphaC_g2_precomp == other.vk_alphaC_g2_precomp
            && self.vk_rC_Z_g2_precomp == other.vk_rC_Z_g2_precomp
            && self.vk_gamma_g2_precomp == other.vk_gamma_g2_precomp
            && self.vk_gamma_beta_g1_precomp == other.vk_gamma_beta_g1_precomp
            && self.vk_gamma_beta_g2_precomp == other.vk_gamma_beta_g2_precomp
            && self.encoded_IC_query == other.encoded_IC_query
    }
}

impl<ppT: ppTConfig> fmt::Display for r1cs_ppzksnark_processed_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.pp_G2_one_precomp,
            self.vk_alphaA_g2_precomp,
            self.vk_alphaB_g1_precomp,
            self.vk_alphaC_g2_precomp,
            self.vk_rC_Z_g2_precomp,
            self.vk_gamma_g2_precomp,
            self.vk_gamma_beta_g1_precomp,
            self.vk_gamma_beta_g2_precomp,
            self.encoded_IC_query,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.pp_G2_one_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaA_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaB_g1_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaC_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_rC_Z_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_beta_g1_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_beta_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.encoded_IC_query;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: ppTConfig> PartialEq for r1cs_ppzksnark_proof<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.g_A == other.g_A
            && self.g_B == other.g_B
            && self.g_C == other.g_C
            && self.g_H == other.g_H
            && self.g_K == other.g_K
    }
}

impl<ppT: ppTConfig> fmt::Display for r1cs_ppzksnark_proof<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.g_A, self.g_B, self.g_C, self.g_H, self.g_K,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.g_A;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_B;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_C;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_H;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_K;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT: ppTConfig> r1cs_ppzksnark_verification_key<ppT> {
    pub fn dummy_verification_key(input_size: usize) -> r1cs_ppzksnark_verification_key<ppT>
// where
    //     <ppT as ff_curves::PublicParams>::Fr: Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
    //     <ppT as ff_curves::PublicParams>::Fr: Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
    {
        let mut result = r1cs_ppzksnark_verification_key::<ppT>::default();
        result.alphaA_g2 = G2::<ppT>::one() * Fr::<ppT>::random_element();
        result.alphaB_g1 = G1::<ppT>::one() * Fr::<ppT>::random_element();
        result.alphaC_g2 = G2::<ppT>::one() * Fr::<ppT>::random_element();
        result.gamma_g2 = G2::<ppT>::one() * Fr::<ppT>::random_element();
        result.gamma_beta_g1 = G1::<ppT>::one() * Fr::<ppT>::random_element();
        result.gamma_beta_g2 = G2::<ppT>::one() * Fr::<ppT>::random_element();
        result.rC_Z_g2 = G2::<ppT>::one() * Fr::<ppT>::random_element();

        let base = G1::<ppT>::one() * Fr::<ppT>::random_element();
        let mut v = G1_vector::<ppT>::default();
        for i in 0..input_size {
            v.push(G1::<ppT>::one() * Fr::<ppT>::random_element());
        }

        result.encoded_IC_query = accumulation_vector::<G1<ppT>>::new_with_vec(base, v);

        result
    }
}
