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

use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::{ knowledge_commitment_vector};
use crate::reductions::r1cs_to_qap::r1cs_to_qap::r1cs_to_qap_instance_map_with_evaluation;
use crate::reductions::r1cs_to_qap::r1cs_to_qap::r1cs_to_qap_witness_map;
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark_params::{
    r1cs_gg_ppzksnark_auxiliary_input, r1cs_gg_ppzksnark_constraint_system,
    r1cs_gg_ppzksnark_primary_input,
};
 
use ff_curves::Fr;
use ff_curves::GT;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::scalar_multiplication::multiexp::{
    batch_exp, batch_exp_with_coeff, get_exp_window_size, get_window_table, inhibit_profiling_info,
    multi_exp, multi_exp_method, multi_exp_with_mixed_addition,
};

use crate::common::data_structures::accumulation_vector::accumulation_vector;
use ff_curves::PpConfig;
use ffec::common::serialization::OUTPUT_NEWLINE;

use ff_curves::algebra::curves::public_params::{PublicParams, PublicParamsType};
use ff_curves::{G1_vector, G2_precomp};
const N: usize = 4;
/******************************** Proving key ********************************/

/**
 * A proving key for the R1CS GG-ppzkSNARK.
 */

struct r1cs_gg_ppzksnark_proving_key<ppT: PublicParams>
where
    <ppT as ff_curves::PublicParams>::G2: PpConfig, <ppT as ff_curves::PublicParams>::G1: PpConfig
{
    alpha_g1: ppT::G1,
    beta_g1: ppT::G1,
    beta_g2: ppT::G2,
    delta_g1: ppT::G1,
    delta_g2: ppT::G2,

    A_query: G1_vector<ppT>, // this could be a sparse vector if we had multiexp for those
    B_query: knowledge_commitment_vector<ppT::G2, ppT::G1>,
    H_query: G1_vector<ppT>,
    L_query: G1_vector<ppT>,

    constraint_system: r1cs_gg_ppzksnark_constraint_system<ppT, pb_variable, pb_linear_combination>,
}

impl<ppT: PublicParams> r1cs_gg_ppzksnark_proving_key<ppT> {
    pub fn new(
        alpha_g1: ppT::G1,
        beta_g1: ppT::G1,
        beta_g2: ppT::G2,
        delta_g1: ppT::G1,
        delta_g2: ppT::G2,
        A_query: G1_vector<ppT>,
        B_query: knowledge_commitment_vector< ppT::G2, ppT::G1>,
        H_query: G1_vector<ppT>,
        L_query: G1_vector<ppT>,
        constraint_system: r1cs_gg_ppzksnark_constraint_system<
            ppT,
            pb_variable,
            pb_linear_combination,
        >,
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
        return 1
            + self.A_query.len()
            + self.B_query.domain_size()
            + self.H_query.len()
            + self.L_query.len();
    }

    pub fn g2_size(&self) -> usize {
        return 1 + self.B_query.domain_size();
    }

    pub fn g1_sparse_size(&self) -> usize {
        return 1
            + self.A_query.len()
            + self.B_query.len()
            + self.H_query.len()
            + self.L_query.len();
    }

    pub fn g2_sparse_size(&self) -> usize {
        return 1 + self.B_query.len();
    }

    pub fn size_in_bits(&self) -> usize {
        self.A_query.size_in_bits()
            + self.B_query.size_in_bits()
            + self.H_query.size_in_bits()
            + self.L_query.size_in_bits()
            + 1 * ppT::G1::size_in_bits()
            + 1 * ppT::G2::size_in_bits()
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in PK: {}\n", Self::g1_size());
        print_indent();
        print!("* Non-zero G1 elements in PK: {}\n", self.G1_sparse_size());
        print_indent();
        print!("* G2 elements in PK: {}\n", Self::g2_size());
        print_indent();
        print!("* Non-zero G2 elements in PK: {}\n", self.G2_sparse_size());
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

struct r1cs_gg_ppzksnark_verification_key<ppT: PublicParams> where <ppT as ff_curves::PublicParams>::G1: PpConfig {
    alpha_g1_beta_g2: GT<ppT>,
    gamma_g2: ppT::G2,
    delta_g2: ppT::G2,

    gamma_ABC_g1: accumulation_vector<ppT::G1>,
}

impl<ppT: PublicParams> r1cs_gg_ppzksnark_verification_key<ppT> {
    // r1cs_gg_ppzksnark_verification_key() = default;
    pub fn new(
        alpha_g1_beta_g2: GT<ppT>,
        gamma_g2: ppT::G2,
        delta_g2: ppT::G2,
        gamma_ABC_g1: accumulation_vector<ppT::G1>,
    ) -> Self {
        Self {
            alpha_g1_beta_g2,
            gamma_g2,
            delta_g2,
            gamma_ABC_g1,
        }
    }

    pub fn g1_size(&self) -> usize {
        return self.gamma_ABC_g1.len();
    }

    pub fn g2_size(&self) -> usize {
        return 2;
    }

    pub fn gt_size(&self) -> usize {
        return 1;
    }

    pub fn size_in_bits(&self) -> usize {
        // TODO: include GT size
        return (self.gamma_ABC_g1.size_in_bits() + 2 * ppT::G2::size_in_bits());
    }

    fn print_size(&self) {
        print_indent();
        print!("* G1 elements in VK: {}\n", Self::g1_size());
        print_indent();
        print!("* G2 elements in VK: {}\n", Self::g2_size());
        print_indent();
        print!("* GT elements in VK: {}\n", self.GT_size());
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
struct r1cs_gg_ppzksnark_processed_verification_key<ppT: PublicParams> {
    vk_alpha_g1_beta_g2: GT<ppT>,
    vk_gamma_g2_precomp: G2_precomp<ppT>,
    vk_delta_g2_precomp: G2_precomp<ppT>,

    gamma_ABC_g1: accumulation_vector<ppT::G1>,
    // bool operator==(&other:r1cs_gg_ppzksnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pvk:r1cs_gg_ppzksnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_processed_verification_key<ppT> &pvk);
}

/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS GG-ppzkSNARK, which consists of a proving key and a verification key.
 */
struct r1cs_gg_ppzksnark_keypair<ppT: PublicParams> {
    pk: r1cs_gg_ppzksnark_proving_key<ppT>,
    vk: r1cs_gg_ppzksnark_verification_key<ppT>,
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
struct r1cs_gg_ppzksnark_proof<ppT: PublicParams> {
    g_A: ppT::G1,
    g_B: ppT::G2,
    g_C: ppT::G1,
}
impl<ppT: PublicParams> r1cs_gg_ppzksnark_proof<ppT> {
    pub fn default() -> Self {
        // invalid proof with valid curve points
        Self {
            g_A: ppT::G1::one(),
            g_B: ppT::G2::one(),
            g_C: ppT::G1::one(),
        }
    }
    pub fn new(g_A: ppT::G1, g_B: ppT::G2, g_C: ppT::G1) -> Self {
        Self { g_A, g_B, g_C }
    }

    pub fn g1_size() -> usize {
         2
    }

    pub fn g2_size() -> usize {
         1
    }

    pub fn size_in_bits() -> usize {
         Self::g1_size() * ppT::G1::size_in_bits() + Self::g2_size() * ppT::G2::size_in_bits()
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
    &r1cs: r1cs_gg_ppzksnark_constraint_system<ppT, pb_variable, pb_linear_combination>,
) -> r1cs_gg_ppzksnark_keypair<ppT> {
    enter_block("Call to r1cs_gg_ppzksnark_generator");

    /* Make the B_query "lighter" if possible */
    let mut r1cs_copy = r1cs.clone();
    r1cs_copy.swap_AB_if_beneficial();

    /* Generate secret randomness */
    let t = ppT::Fr::random_element();
    let alpha = ppT::Fr::random_element();
    let beta = ppT::Fr::random_element();
    let gamma = ppT::Fr::random_element();
    let delta = ppT::Fr::random_element();
    let gamma_inverse = gamma.inverse();
    let delta_inverse = delta.inverse();

    /* A quadratic arithmetic program evaluated at t. */
    let qap = r1cs_to_qap_instance_map_with_evaluation(r1cs_copy, t);

    print_indent();
    print!("* QAP number of variables: {}\n", qap.num_variables());
    print_indent();
    print!("* QAP pre degree: {}\n", r1cs_copy.constraints.len());
    print_indent();
    print!("* QAP degree: {}\n", qap.degree());
    print_indent();
    print!("* QAP number of input variables: {}\n", qap.num_inputs());

    enter_block("Compute query densities");
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
    leave_block("Compute query densities");

    /* qap.{At,Bt,Ct,Ht} are now in unspecified state, but we do not use them later */
    let At = qap.At;
    let Bt = qap.Bt;
    let Ct = qap.Ct;
    let Ht = qap.Ht;

    /* The gamma inverse product component: (beta*A_i(t) + alpha*B_i(t) + C_i(t)) * gamma^{-1}. */
    enter_block("Compute gamma_ABC for R1CS verification key");
    let mut gamma_ABC = ppT::Fr_vector();
    gamma_ABC.reserve(qap.num_inputs());

    let gamma_ABC_0 = (beta * At[0] + alpha * Bt[0] + Ct[0]) * gamma_inverse;
    for i in 1..qap.num_inputs() + 1 {
        gamma_ABC.push((beta * At[i] + alpha * Bt[i] + Ct[i]) * gamma_inverse);
    }
    leave_block("Compute gamma_ABC for R1CS verification key");

    /* The delta inverse product component: (beta*A_i(t) + alpha*B_i(t) + C_i(t)) * delta^{-1}. */
    enter_block("Compute L query for R1CS proving key");
    let mut Lt = ppT::Fr_vector();
    Lt.reserve(qap.num_variables() - qap.num_inputs());

    let mut Lt_offset = qap.num_inputs() + 1;
    for i in 0..qap.num_variables() - qap.num_inputs() {
        Lt.push(
            (beta * At[Lt_offset + i] + alpha * Bt[Lt_offset + i] + Ct[Lt_offset + i])
                * delta_inverse,
        );
    }
    leave_block("Compute L query for R1CS proving key");

    /**
     * Note that H for Groth's proof system is degree d-2, but the QAP
     * reduction returns coefficients for degree d polynomial H (in
     * style of PGHR-type proof systems)
     */
    Ht.resize(Ht.len() - 2);

    // // #ifdef MULTICORE
    //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    let chunks = 1;
    // //#endif

    enter_block("Generating G1 MSM window table");
    let g1_generator = ppT::G1::random_element();
    let g1_scalar_count = non_zero_At + non_zero_Bt + qap.num_variables();
    let g1_scalar_size = ppT::Fr::size_in_bits();
    let g1_window_size = get_exp_window_size::<ppT::G1>(g1_scalar_count);

    print_indent();
    print!("* G1 window: {}\n", g1_window_size);
    let g1_table = get_window_table(g1_scalar_size, g1_window_size, g1_generator);
    leave_block("Generating G1 MSM window table");

    enter_block("Generating G2 MSM window table");
    let G2_gen = ppT::G2::random_element();
    let g2_scalar_count = non_zero_Bt;
    let g2_scalar_size = ppT::Fr::size_in_bits();
    let g2_window_size = get_exp_window_size::<ppT::G2>(g2_scalar_count);

    print_indent();
    print!("* G2 window: {}\n", g2_window_size);
    let g2_table = get_window_table(g2_scalar_size, g2_window_size, G2_gen);
    leave_block("Generating G2 MSM window table");

    enter_block("Generate R1CS proving key");
    let mut alpha_g1 = alpha * g1_generator;
    let mut beta_g1 = beta * g1_generator;
    let mut beta_g2 = beta * G2_gen;
    let mut delta_g1 = delta * g1_generator;
    let mut delta_g2 = delta * G2_gen;

    enter_block("Generate queries");
    enter_block("Compute the A-query", false);
    let A_query = batch_exp(g1_scalar_size, g1_window_size, g1_table, At);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<ppT::G1 >(A_query);
    // //#endif
    leave_block("Compute the A-query", false);

    enter_block("Compute the B-query", false);
    let mut B_query = kc_batch_exp(
        ppT::Fr::size_in_bits(),
        g2_window_size,
        g1_window_size,
        g2_table,
        g1_table,
        ppT::Fr::one(),
        ppT::Fr::one(),
        Bt,
        chunks,
    );
    // NOTE: if USE_MIXED_ADDITION is defined,
    // kc_batch_exp will convert its output to special form internally
    leave_block("Compute the B-query", false);

    enter_block("Compute the H-query", false);
    let mut H_query = batch_exp_with_coeff(
        g1_scalar_size,
        g1_window_size,
        g1_table,
        qap.Zt * delta_inverse,
        Ht,
    );
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<ppT::G1 >(H_query);
    // //#endif
    leave_block("Compute the H-query", false);

    enter_block("Compute the L-query", false);
    let mut L_query = batch_exp(g1_scalar_size, g1_window_size, g1_table, Lt);
    // // #ifdef USE_MIXED_ADDITION
    //     batch_to_special<ppT::G1 >(L_query);
    // //#endif
    leave_block("Compute the L-query", false);
    leave_block("Generate queries");

    leave_block("Generate R1CS proving key");

    enter_block("Generate R1CS verification key");
    let mut alpha_g1_beta_g2 = ppT::reduced_pairing(alpha_g1, beta_g2);
    let mut gamma_g2 = gamma * G2_gen;

    enter_block("Encode gamma_ABC for R1CS verification key");
    let mut gamma_ABC_g1_0 = gamma_ABC_0 * g1_generator;
    let mut gamma_ABC_g1_values = batch_exp(g1_scalar_size, g1_window_size, g1_table, gamma_ABC);
    leave_block("Encode gamma_ABC for R1CS verification key");
    leave_block("Generate R1CS verification key");

    leave_block("Call to r1cs_gg_ppzksnark_generator");

    let mut gamma_ABC_g1 = accumulation_vector::<ppT::G1>::new(gamma_ABC_g1_0, gamma_ABC_g1_values);

    let mut vk = r1cs_gg_ppzksnark_verification_key::<ppT>(
        alpha_g1_beta_g2,
        gamma_g2,
        delta_g2,
        gamma_ABC_g1,
    );

    let mut pk = r1cs_gg_ppzksnark_proving_key::<ppT>(
        alpha_g1, beta_g1, beta_g2, delta_g1, delta_g2, A_query, B_query, H_query, L_query,
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
    pk: r1cs_gg_ppzksnark_proving_key<ppT>,
    primary_input: r1cs_gg_ppzksnark_primary_input<ppT>,
    auxiliary_input: r1cs_gg_ppzksnark_auxiliary_input<ppT>,
) -> r1cs_gg_ppzksnark_proof<ppT> {
    enter_block("Call to r1cs_gg_ppzksnark_prover");

    // // #ifdef DEBUG
    //     assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
    // //#endif

    enter_block("Compute the polynomial H");
    let qap_wit = r1cs_to_qap_witness_map(
        pk.constraint_system,
        primary_input,
        auxiliary_input,
        ppT::Fr::zero(),
        ppT::Fr::zero(),
    );

    /* We are dividing degree 2(d-1) polynomial by degree d polynomial
    and not adding a PGHR-style ZK-patch, so our H is degree d-2 */
    assert!(!qap_wit.coefficients_for_H[qap_wit.degree() - 2].is_zero());
    assert!(qap_wit.coefficients_for_H[qap_wit.degree() - 1].is_zero());
    assert!(qap_wit.coefficients_for_H[qap_wit.degree()].is_zero());
    leave_block("Compute the polynomial H");

    // // #ifdef DEBUG
    //     let t =ppT::Fr::random_element();
    //     qap_instance_evaluation<Fr<ppT> > qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    //     assert!(qap_inst.is_satisfied(qap_wit));
    // //#endif

    /* Choose two random field elements for prover zero-knowledge. */
    let r = ppT::Fr::random_element();
    let s = ppT::Fr::random_element();

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

    enter_block("Compute the proof");

    enter_block("Compute evaluation to A-query", false);
    // TODO: sort out indexing
    let mut const_padded_assignment = ppT::Fr_vector::new(1, ppT::Fr::one());
    const_padded_assignment.extend(qap_wit.coefficients_for_ABCs.clone());

    let evaluation_At = multi_exp_with_mixed_addition::<
        ppT::G1,
        Fr<ppT>,
        { multi_exp_method::multi_exp_method_BDLO12 },
    >(
        pk.A_query[..qap_wit.num_variables() + 1],
        const_padded_assignment[..qap_wit.num_variables() + 1],
        chunks,
    );
    leave_block("Compute evaluation to A-query", false);

    enter_block("Compute evaluation to B-query", false);
    let evaluation_Bt = kc_multi_exp_with_mixed_addition::<
        ppT::G2,
        ppT::G1,
        Fr<ppT>,
        { multi_exp_method::multi_exp_method_BDLO12 },
    >(
        pk.B_query,
        0,
        qap_wit.num_variables() + 1,
        const_padded_assignment[..qap_wit.num_variables() + 1],
        chunks,
    );
    leave_block("Compute evaluation to B-query", false);

    enter_block("Compute evaluation to H-query", false);
    let evaluation_Ht = multi_exp::<ppT::G1, ppT::Fr, { multi_exp_method::multi_exp_method_BDLO12 }>(
        pk.H_query[..(qap_wit.degree() - 1)],
        qap_wit.coefficients_for_H[..(qap_wit.degree() - 1)],
        chunks,
    );
    leave_block("Compute evaluation to H-query", false);

    enter_block("Compute evaluation to L-query", false);
    let evaluation_Lt = multi_exp_with_mixed_addition::<
        ppT::G1,
        ppT::Fr,
        { multi_exp_method::multi_exp_method_BDLO12 },
    >(
        pk.L_query,
        const_padded_assignment[qap_wit.num_inputs() + 1..qap_wit.num_variables() + 1],
        chunks,
    );
    leave_block("Compute evaluation to L-query", false);

    /* A = alpha + sum_i(a_i*A_i(t)) + r*delta */
    let g1_A = pk.alpha_g1 + evaluation_At + r * pk.delta_g1;

    /* B = beta + sum_i(a_i*B_i(t)) + s*delta */
    let g1_B = pk.beta_g1 + evaluation_Bt.h + s * pk.delta_g1;
    let g2_B = pk.beta_g2 + evaluation_Bt.g + s * pk.delta_g2;

    /* C = sum_i(a_i*((beta*A_i(t) + alpha*B_i(t) + C_i(t)) + H(t)*Z(t))/delta) + A*s + r*b - r*s*delta */
    let g1_C = evaluation_Ht + evaluation_Lt + s * g1_A + r * g1_B - (r * s) * pk.delta_g1;

    leave_block("Compute the proof");

    leave_block("Call to r1cs_gg_ppzksnark_prover");

    let proof = r1cs_gg_ppzksnark_proof::<ppT>::new(g1_A, g2_B, g1_C);
    proof.print_size();

    return proof;
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
    vk: r1cs_gg_ppzksnark_verification_key<ppT>,
    primary_input: r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_verifier_weak_IC");
    let pvk = r1cs_gg_ppzksnark_verifier_process_vk::<ppT>(vk);
    let result = r1cs_gg_ppzksnark_online_verifier_weak_IC::<ppT>(pvk, primary_input, proof);
    leave_block("Call to r1cs_gg_ppzksnark_verifier_weak_IC");
    return result;
}

/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_gg_ppzksnark_verifier_strong_IC<ppT: PublicParams>(
    vk: r1cs_gg_ppzksnark_verification_key<ppT>,
    primary_input: r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_verifier_strong_IC");
    let pvk = r1cs_gg_ppzksnark_verifier_process_vk::<ppT>(vk);
    let result = r1cs_gg_ppzksnark_online_verifier_strong_IC::<ppT>(pvk, primary_input, proof);
    leave_block("Call to r1cs_gg_ppzksnark_verifier_strong_IC");
    return result;
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

pub fn r1cs_gg_ppzksnark_verifier_process_vk<ppT: PublicParams>(
    &vk: r1cs_gg_ppzksnark_verification_key<ppT>,
) -> r1cs_gg_ppzksnark_processed_verification_key<ppT> {
    enter_block("Call to r1cs_gg_ppzksnark_verifier_process_vk");

    let mut pvk = r1cs_gg_ppzksnark_processed_verification_key::<ppT>::new();
    pvk.vk_alpha_g1_beta_g2 = vk.alpha_g1_beta_g2;
    pvk.vk_gamma_g2_precomp = ppT::precompute_G2(vk.gamma_g2);
    pvk.vk_delta_g2_precomp = ppT::precompute_G2(vk.delta_g2);
    pvk.gamma_ABC_g1 = vk.gamma_ABC_g1;

    leave_block("Call to r1cs_gg_ppzksnark_verifier_process_vk");

    return pvk;
}

/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

pub fn r1cs_gg_ppzksnark_online_verifier_weak_IC<ppT: PublicParams>(
    pvk: r1cs_gg_ppzksnark_processed_verification_key<ppT>,
    primary_input: r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_online_verifier_weak_IC");
    assert!(pvk.gamma_ABC_g1.domain_size() >= primary_input.len());

    enter_block("Accumulate input");
    let accumulated_IC = pvk
        .gamma_ABC_g1
        .accumulate_chunk::<Fr<ppT>>(primary_input, 0);
    let acc = &accumulated_IC.first;
    leave_block("Accumulate input");

    let mut result = true;

    enter_block("Check if the proof is well-formed");
    if !proof.is_well_formed() {
        if !inhibit_profiling_info {
            print_indent();
            print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    leave_block("Check if the proof is well-formed");

    enter_block("Online pairing computations");
    enter_block("Check QAP divisibility");
    let proof_g_A_precomp = ppT::precompute_G1();
    let proof_g_B_precomp = ppT::precompute_G2();
    let proof_g_C_precomp = ppT::precompute_G1();
    let acc_precomp = ppT::precompute_G1();

    let QAP1 = ppT::miller_loop(proof_g_A_precomp, proof_g_B_precomp);
    let QAP2 = ppT::double_miller_loop(
        acc_precomp,
        pvk.vk_gamma_g2_precomp,
        proof_g_C_precomp,
        pvk.vk_delta_g2_precomp,
    );
    let QAP = ppT::final_exponentiation(QAP1 * QAP2.unitary_inverse());

    if QAP != pvk.vk_alpha_g1_beta_g2 {
        if !inhibit_profiling_info {
            print_indent();
            print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility");
    leave_block("Online pairing computations");

    leave_block("Call to r1cs_gg_ppzksnark_online_verifier_weak_IC");

    return result;
}
/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_gg_ppzksnark_online_verifier_strong_IC<ppT: PublicParams>(
    pvk: r1cs_gg_ppzksnark_processed_verification_key<ppT>,
    primary_input: r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    let mut result = true;
    enter_block("Call to r1cs_gg_ppzksnark_online_verifier_strong_IC");

    if pvk.gamma_ABC_g1.domain_size() != primary_input.len() {
        print_indent();
        print!(
            "Input length differs from expected (got {}, expected {}).\n",
            primary_input.len(),
            pvk.gamma_ABC_g1.domain_size()
        );
        result = false;
    } else {
        result = r1cs_gg_ppzksnark_online_verifier_weak_IC(pvk, primary_input, proof);
    }

    leave_block("Call to r1cs_gg_ppzksnark_online_verifier_strong_IC");
    return result;
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
    vk: r1cs_gg_ppzksnark_verification_key<ppT>,
    primary_input: r1cs_gg_ppzksnark_primary_input<ppT>,
    proof: r1cs_gg_ppzksnark_proof<ppT>,
) -> bool {
    enter_block("Call to r1cs_gg_ppzksnark_affine_verifier_weak_IC");
    assert!(vk.gamma_ABC_g1.domain_size() >= primary_input.len());

    let pvk_vk_gamma_g2_precomp = ppT::affine_ate_precompute_G2(vk.gamma_g2);
    let pvk_vk_delta_g2_precomp = ppT::affine_ate_precompute_G2(vk.delta_g2);

    enter_block("Accumulate input");
    let accumulated_IC = vk
        .gamma_ABC_g1
        .accumulate_chunk::<ppT::Fr>(primary_input, 0);
    let acc = &accumulated_IC.first;
    leave_block("Accumulate input");

    let mut result = true;

    enter_block("Check if the proof is well-formed");
    if !proof.is_well_formed() {
        if !inhibit_profiling_info {
            print_indent();
            print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    leave_block("Check if the proof is well-formed");

    enter_block("Check QAP divisibility");
    let proof_g_A_precomp = ppT::affine_ate_precompute_G1(proof.g_A);
    let proof_g_B_precomp = ppT::affine_ate_precompute_G2(proof.g_B);
    let proof_g_C_precomp = ppT::affine_ate_precompute_G1(proof.g_C);
    let acc_precomp = ppT::affine_ate_precompute_G1(acc);

    let QAP_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        acc_precomp,
        pvk_vk_gamma_g2_precomp,
        proof_g_C_precomp,
        pvk_vk_delta_g2_precomp,
        proof_g_A_precomp,
        proof_g_B_precomp,
    );
    let QAP = ppT::final_exponentiation(QAP_miller.unitary_inverse());

    if QAP != vk.alpha_g1_beta_g2 {
        if !inhibit_profiling_info {
            print_indent();
            print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility");

    leave_block("Call to r1cs_gg_ppzksnark_affine_verifier_weak_IC");

    return result;
}

//

use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark;

// //#endif // R1CS_GG_PPZKSNARK_HPP_

/** @file
*****************************************************************************

Implementation of interfaces for a ppzkSNARK for R1CS.

See r1cs_gg_ppzksnark.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// //#ifndef R1CS_GG_PPZKSNARK_TCC_
// // #define R1CS_GG_PPZKSNARK_TCC_

// use  <algorithm>
// use  <cassert>
// use  <functional>
// use  <iostream>
// use  <sstream>
// use ffec::algebra::scalar_multiplication::multiexp;
// use ffec::common::profiling;
// use common::utils;

// // #ifdef MULTICORE
// use  <omp.h>
// //#endif

// use crate::knowledge_commitment::kc_multiexp;
// use crate::reductions::r1cs_to_qap::r1cs_to_qap;

//

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
            self.pk.alpha_g1,
            self.pk.beta_g1,
            self.pk.beta_g2,
            self.pk.delta_g1,
            self.pk.delta_g2,
            self.pk.A_query,
            self.pk.B_query,
            self.pk.H_query,
            self.pk.L_query,
            self.pk.constraint_system,
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
            self.vk.alpha_g1_beta_g2, self.vk.gamma_g2, self.vk.delta_g2, self.vk.gamma_ABC_g1,
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
            self.pvk.vk_alpha_g1_beta_g2,
            self.pvk.vk_gamma_g2_precomp,
            self.pvk.vk_delta_g2_precomp,
            self.pvk.gamma_ABC_g1,
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
            self.proof.g_A, self.proof.g_B, self.proof.g_C,
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
    pub fn dummy_verification_key(input_size: usize) -> r1cs_gg_ppzksnark_verification_key<ppT> {
        let result = r1cs_gg_ppzksnark_verification_key::<ppT>();
        result.alpha_g1_beta_g2 = ppT::Fr::random_element() * ppT::GT::random_element();
        result.gamma_g2 = ppT::G2::random_element();
        result.delta_g2 = ppT::G2::random_element();

        let base = ppT::G1::random_element();
        let mut v = ppT::G1_vector();
        for i in 0..input_size {
            v.push(ppT::G1::random_element());
        }

        result.gamma_ABC_g1 = accumulation_vector::<ppT::G1>(base, v);

        return result;
    }
}
