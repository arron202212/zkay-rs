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

use ff_curves::algebra::curves::public_params;

use crate::common::data_structures::accumulation_vector;
use crate::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark_params;

//

/******************************** Proving key ********************************/

// pub fn
// pub struct r1cs_ppzksnark_proving_key;

// pub fn
// std::ostream& operator<<(std::ostream &out, &pk:r1cs_ppzksnark_proving_key<ppT>);

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the R1CS ppzkSNARK.
 */

struct r1cs_ppzksnark_proving_key<ppT> {
    A_query: knowledge_commitment_vector<ffec::G1<ppT>, ffec::G1<ppT>>,
    B_query: knowledge_commitment_vector<ffec::G2<ppT>, ffec::G1<ppT>>,
    C_query: knowledge_commitment_vector<ffec::G1<ppT>, ffec::G1<ppT>>,
    H_query: ffec::G1_vector<ppT>,
    K_query: ffec::G1_vector<ppT>,

    constraint_system: r1cs_ppzksnark_constraint_system<ppT>,
}
impl r1cs_ppzksnark_proving_key {
    // r1cs_ppzksnark_proving_key() {};
    // r1cs_ppzksnark_proving_key<ppT>& operator=(&other:r1cs_ppzksnark_proving_key<ppT>) = default;
    // r1cs_ppzksnark_proving_key(&other:r1cs_ppzksnark_proving_key<ppT>) = default;
    // r1cs_ppzksnark_proving_key(r1cs_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(
        A_query: knowledge_commitment_vector<ffec::G1<ppT>, ffec::G1<ppT>>,
        B_query: knowledge_commitment_vector<ffec::G2<ppT>, ffec::G1<ppT>>,
        C_query: knowledge_commitment_vector<ffec::G1<ppT>, ffec::G1<ppT>>,
        H_query: ffec::G1_vector<ppT>,
        K_query: ffec::G1_vector<ppT>,
        constraint_system: r1cs_ppzksnark_constraint_system<ppT>,
    ) -> Self {
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
        return 2 * (A_query.domain_size() + C_query.domain_size())
            + B_query.domain_size()
            + H_query.len()
            + K_query.len();
    }

    pub fn g2_size(&self) -> usize {
        return B_query.domain_size();
    }

    pub fn g1_sparse_size(&self) -> usize {
        return 2 * (A_query.len() + C_query.len()) + B_query.len() + H_query.len() + K_query.len();
    }

    pub fn g2_sparse_size(&self) -> usize {
        return B_query.len();
    }

    pub fn size_in_bits(&self) -> usize {
        return A_query.size_in_bits()
            + B_query.size_in_bits()
            + C_query.size_in_bits()
            + ffec::size_in_bits(H_query)
            + ffec::size_in_bits(K_query);
    }

    fn print_size(&self) {
        ffec::print_indent();
        print!("* G1 elements in PK: {}\n", self.G1_size());
        ffec::print_indent();
        print!("* Non-zero G1 elements in PK: {}\n", self.G1_sparse_size());
        ffec::print_indent();
        print!("* G2 elements in PK: {}\n", self.G2_size());
        ffec::print_indent();
        print!("* Non-zero G2 elements in PK: {}\n", self.G2_sparse_size());
        ffec::print_indent();
        print!("* PK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(&other:r1cs_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pk:r1cs_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_proving_key<ppT> &pk);
}

/******************************* Verification key ****************************/

// pub fn
// pub struct r1cs_ppzksnark_verification_key;

// pub fn
// std::ostream& operator<<(std::ostream &out, &vk:r1cs_ppzksnark_verification_key<ppT>);

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_verification_key<ppT> &vk);

/**
 * A verification key for the R1CS ppzkSNARK.
 */
struct r1cs_ppzksnark_verification_key<ppT> {
    alphaA_g2: ffec::G2<ppT>,
    alphaB_g1: ffec::G1<ppT>,
    alphaC_g2: ffec::G2<ppT>,
    gamma_g2: ffec::G2<ppT>,
    gamma_beta_g1: ffec::G1<ppT>,
    gamma_beta_g2: ffec::G2<ppT>,
    rC_Z_g2: ffec::G2<ppT>,

    encoded_IC_query: accumulation_vector<ffec::G1<ppT>>,
}
impl<ppT> r1cs_ppzksnark_verification_key<ppT> {
    // r1cs_ppzksnark_verification_key() = default;
    pub fn new(
        alphaA_g2: ffec::G2<ppT>,
        alphaB_g1: ffec::G1<ppT>,
        alphaC_g2: ffec::G2<ppT>,
        gamma_g2: ffec::G2<ppT>,
        gamma_beta_g1: ffec::G1<ppT>,
        gamma_beta_g2: ffec::G2<ppT>,
        rC_Z_g2: ffec::G2<ppT>,
        eIC: accumulation_vector<ffec::G1<ppT>>,
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
        return 2 + encoded_IC_query.len();
    }

    pub fn g2_size(&self) -> usize {
        return 5;
    }

    pub fn size_in_bits(&self) -> usize {
        return (2 * ppT::G1::size_in_bits()
            + encoded_IC_query.size_in_bits()
            + 5 * ppT::G2::size_in_bits());
    }

    fn print_size(&self) {
        ffec::print_indent();
        print!("* G1 elements in VK: {}\n", self.G1_size());
        ffec::print_indent();
        print!("* G2 elements in VK: {}\n", self.G2_size());
        ffec::print_indent();
        print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(&other:r1cs_ppzksnark_verification_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &vk:r1cs_ppzksnark_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_verification_key<ppT> &vk);

    // static r1cs_ppzksnark_verification_key<ppT> dummy_verification_key(input_size:usize);
}

/************************ Processed verification key *************************/

// pub fn
// pub struct r1cs_ppzksnark_processed_verification_key;

// pub fn
// std::ostream& operator<<(std::ostream &out, &pvk:r1cs_ppzksnark_processed_verification_key<ppT>);

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_processed_verification_key<ppT> &pvk);

/**
 * A processed verification key for the R1CS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
struct r1cs_ppzksnark_processed_verification_key<ppT> {
    pp_G2_one_precomp: ffec::G2_precomp<ppT>,
    vk_alphaA_g2_precomp: ffec::G2_precomp<ppT>,
    vk_alphaB_g1_precomp: ffec::G1_precomp<ppT>,
    vk_alphaC_g2_precomp: ffec::G2_precomp<ppT>,
    vk_rC_Z_g2_precomp: ffec::G2_precomp<ppT>,
    vk_gamma_g2_precomp: ffec::G2_precomp<ppT>,
    vk_gamma_beta_g1_precomp: ffec::G1_precomp<ppT>,
    vk_gamma_beta_g2_precomp: ffec::G2_precomp<ppT>,

    encoded_IC_query: accumulation_vector<ffec::G1<ppT>>,
    // bool operator==(&other:r1cs_ppzksnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pvk:r1cs_ppzksnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzksnark_processed_verification_key<ppT> &pvk);
}

/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS ppzkSNARK, which consists of a proving key and a verification key.
 */
struct r1cs_ppzksnark_keypair<ppT> {
    pk: r1cs_ppzksnark_proving_key<ppT>,
    vk: r1cs_ppzksnark_verification_key<ppT>,
}
impl<ppT> r1cs_ppzksnark_keypair<ppT> {
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

/*********************************** Proof ***********************************/

// struct r1cs_ppzksnark_proof;

// pub fn
// std::ostream& operator<<(std::ostream &out, &proof:r1cs_ppzksnark_proof<ppT>);

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_proof<ppT> &proof);

/**
 * A proof for the R1CS ppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
struct r1cs_ppzksnark_proof<ppT> {
    g_A: knowledge_commitment<ffec::G1<ppT>, ffec::G1<ppT>>,
    g_B: knowledge_commitment<ffec::G2<ppT>, ffec::G1<ppT>>,
    g_C: knowledge_commitment<ffec::G1<ppT>, ffec::G1<ppT>>,
    g_H: ffec::G1<ppT>,
    g_K: ffec::G1<ppT>,
}
impl<ppT> r1cs_ppzksnark_proof<ppT> {
    pub fn default() -> Self {
        // invalid proof with valid curve points
        Self {
            g_A: knowledge_commitment::new(ppT::G1::one(), ppT::G1::one()),
            g_B: knowledge_commitment::new(ppT::G2::one(), ppT::G1::one()),
            g_C: lknowledge_commitment::new(ppT::G1::one(), ppT::G1::one()),
            g_H: ppT::G1::one(),
            g_K: ppT::G1::one(),
        }
    }
    pub fn new_paras(
        g_A: knowledge_commitment<ffec::G1<ppT>, ffec::G1<ppT>>,
        g_B: knowledge_commitment<ppT::G2, ffec::G1<ppT>>,
        g_C: knowledge_commitment<ffec::G1<ppT>, ffec::G1<ppT>>,
        g_H: ffec::G1<ppT>,
        g_K: ffec::G1<ppT>,
    ) -> Self {
        Self {
            g_A,
            g_B,
            g_C,
            g_H,
            g_K,
        }
    }

    pub fn g1_size(&self) -> usize {
        return 7;
    }

    pub fn g2_size(&self) -> usize {
        return 1;
    }

    pub fn size_in_bits(&self) -> usize {
        return G1_size() * ppT::G1::size_in_bits() + G2_size() * ppT::G2::size_in_bits();
    }

    fn print_size(&self) {
        ffec::print_indent();
        print!("* G1 elements in proof: {}\n", self.G1_size());
        ffec::print_indent();
        print!("* G2 elements in proof: {}\n", self.G2_size());
        ffec::print_indent();
        print!("* Proof size in bits: {}\n", self.size_in_bits());
    }

    fn is_well_formed(&self) -> bool {
        return (g_A.g.is_well_formed()
            && g_A.h.is_well_formed()
            && g_B.g.is_well_formed()
            && g_B.h.is_well_formed()
            && g_C.g.is_well_formed()
            && g_C.h.is_well_formed()
            && g_H.is_well_formed()
            && g_K.is_well_formed());
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

pub fn r1cs_ppzksnark_generator<ppT>(
    cs: r1cs_ppzksnark_constraint_system<ppT>,
) -> r1cs_ppzksnark_keypair<ppT> {
    ffec::enter_block("Call to r1cs_ppzksnark_generator");

    /* make the B_query "lighter" if possible */
    let cs_copy = cs.clone();
    cs_copy.swap_AB_if_beneficial();

    /* draw random element at which the QAP is evaluated */
    let t = ppT::Fr::random_element();

    let qap_inst = r1cs_to_qap_instance_map_with_evaluation(cs_copy, t);

    ffec::print_indent();
    print!("* QAP number of variables: {}\n", qap_inst.num_variables());
    ffec::print_indent();
    print!("* QAP pre degree: {}\n", cs_copy.constraints.len());
    ffec::print_indent();
    print!("* QAP degree: {}\n", qap_inst.degree());
    ffec::print_indent();
    print!(
        "* QAP number of input variables: {}\n",
        qap_inst.num_inputs()
    );

    ffec::enter_block("Compute query densities");
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
    ffec::leave_block("Compute query densities");

    let mut At = qap_inst.At; // qap_inst.At is now in unspecified state, but we do not use it later
    let mut Bt = qap_inst.Bt; // qap_inst.Bt is now in unspecified state, but we do not use it later
    let mut Ct = qap_inst.Ct; // qap_inst.Ct is now in unspecified state, but we do not use it later
    let mut Ht = qap_inst.Ht; // qap_inst.Ht is now in unspecified state, but we do not use it later

    /* append Zt to At,Bt,Ct with */
    At.push(qap_inst.Zt);
    Bt.push(qap_inst.Zt);
    Ct.push(qap_inst.Zt);

    let (alphaA, alphaB, alphaC, rA, rB, beta, gamma) = (
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
    );
    let rC = rA * rB;

    // consrtuct the same-coefficient-check query (must happen before zeroing out the prefix of At)
    let mut Kt = ppT::Fr_vector::new();
    Kt.reserve(qap_inst.num_variables() + 4);
    for i in 0..qap_inst.num_variables() + 1 {
        Kt.push(beta * (rA * At[i] + rB * Bt[i] + rC * Ct[i]));
    }
    Kt.push(beta * rA * qap_inst.Zt);
    Kt.push(beta * rB * qap_inst.Zt);
    Kt.push(beta * rC * qap_inst.Zt);

    /* zero out prefix of At and stick it into IC coefficients */
    let mut IC_coefficients = ppT::Fr_vector::new();
    IC_coefficients.reserve(qap_inst.num_inputs() + 1);
    for i in 0..qap_inst.num_inputs() + 1 {
        IC_coefficients.push(At[i]);
        assert!(!IC_coefficients[i].is_zero());
        At[i] = ppT::Fr::zero();
    }

    let g1_exp_count = 2 * (non_zero_At - qap_inst.num_inputs() + non_zero_Ct)
        + non_zero_Bt
        + non_zero_Ht
        + Kt.len();
    let g2_exp_count = non_zero_Bt;

    let g1_window = ffec::get_exp_window_size::<ppT::G1>(g1_exp_count);
    let g2_window = ffec::get_exp_window_size::<ppT::G2>(g2_exp_count);
    ffec::print_indent();
    print!("* G1 window: {}\n", g1_window);
    ffec::print_indent();
    print!("* G2 window: {}\n", g2_window);

    // // #ifdef MULTICORE
    //     let  chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
    // #else
    //     let  usize chunks = 1;
    // //#endif

    ffec::enter_block("Generating G1 multiexp table");
    let g1_table = get_window_table(ppT::Fr::size_in_bits(), g1_window, ppT::G1::one());
    ffec::leave_block("Generating G1 multiexp table");

    ffec::enter_block("Generating G2 multiexp table");
    let g2_table = get_window_table(ppT::Fr::size_in_bits(), g2_window, ppT::G2::one());
    ffec::leave_block("Generating G2 multiexp table");

    ffec::enter_block("Generate R1CS proving key");

    ffec::enter_block("Generate knowledge commitments");
    ffec::enter_block("Compute the A-query", false);
    let A_query = kc_batch_exp(
        ppT::Fr::size_in_bits(),
        g1_window,
        g1_window,
        g1_table,
        g1_table,
        rA,
        rA * alphaA,
        At,
        chunks,
    );
    ffec::leave_block("Compute the A-query", false);

    ffec::enter_block("Compute the B-query", false);
    let B_query = kc_batch_exp(
        ppT::Fr::size_in_bits(),
        g2_window,
        g1_window,
        g2_table,
        g1_table,
        rB,
        rB * alphaB,
        Bt,
        chunks,
    );
    ffec::leave_block("Compute the B-query", false);

    ffec::enter_block("Compute the C-query", false);
    let C_query = kc_batch_exp(
        ppT::Fr::size_in_bits(),
        g1_window,
        g1_window,
        g1_table,
        g1_table,
        rC,
        rC * alphaC,
        Ct,
        chunks,
    );
    ffec::leave_block("Compute the C-query", false);

    ffec::enter_block("Compute the H-query", false);
    let H_query = batch_exp(ppT::Fr::size_in_bits(), g1_window, g1_table, Ht);
    // // #ifdef USE_MIXED_ADDITION
    //     ffec::batch_to_special<ffec::G1<ppT> >(H_query);
    // //#endif
    ffec::leave_block("Compute the H-query", false);

    ffec::enter_block("Compute the K-query", false);
    let K_query = batch_exp(ppT::Fr::size_in_bits(), g1_window, g1_table, Kt);
    // // #ifdef USE_MIXED_ADDITION
    //     ffec::batch_to_special<ffec::G1<ppT> >(K_query);
    // //#endif
    ffec::leave_block("Compute the K-query", false);

    ffec::leave_block("Generate knowledge commitments");

    ffec::leave_block("Generate R1CS proving key");

    ffec::enter_block("Generate R1CS verification key");
    let alphaA_g2 = alphaA * ppT::G2::one();
    let alphaB_g1 = alphaB * ppT::G1::one();
    let alphaC_g2 = alphaC * ppT::G2::one();
    let gamma_g2 = gamma * ppT::G2::one();
    let gamma_beta_g1 = (gamma * beta) * ppT::G1::one();
    let gamma_beta_g2 = (gamma * beta) * ppT::G2::one();
    let rC_Z_g2 = (rC * qap_inst.Zt) * ppT::G2::one();

    ffec::enter_block("Encode IC query for R1CS verification key");
    let encoded_IC_base = (rA * IC_coefficients[0]) * ppT::G1::one();
    let mut multiplied_IC_coefficients = ppT::Fr_vector::new();
    multiplied_IC_coefficients.reserve(qap_inst.num_inputs());
    for i in 1..qap_inst.num_inputs() + 1 {
        multiplied_IC_coefficients.push(rA * IC_coefficients[i]);
    }
    let encoded_IC_values = batch_exp(
        ppT::Fr::size_in_bits(),
        g1_window,
        g1_table,
        multiplied_IC_coefficients,
    );

    ffec::leave_block("Encode IC query for R1CS verification key");
    ffec::leave_block("Generate R1CS verification key");

    ffec::leave_block("Call to r1cs_ppzksnark_generator");

    let mut encoded_IC_query =
        accumulation_vector::<ppT::G1>::new((encoded_IC_base), (encoded_IC_values));

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

pub fn r1cs_ppzksnark_prover<ppT>(
    pk: r1cs_ppzksnark_proving_key<ppT>,
    primary_input: r1cs_ppzksnark_primary_input<ppT>,
    auxiliary_input: r1cs_ppzksnark_auxiliary_input<ppT>,
) -> r1cs_ppzksnark_proof<ppT> {
    ffec::enter_block("Call to r1cs_ppzksnark_prover");

    // // #ifdef DEBUG
    //     assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
    // //#endif
    let (d1, d2, d3) = (
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
        ppT::Fr::random_element(),
    );

    ffec::enter_block("Compute the polynomial H");
    let qap_wit =
        r1cs_to_qap_witness_map(pk.constraint_system, primary_input, auxiliary_input, d1, d3);
    ffec::leave_block("Compute the polynomial H");

    // // #ifdef DEBUG
    //     ppT::Fr::random_element(:ffec::Fr<ppT> t =);
    //     qap_instance_evaluation<ffec::Fr<ppT> > qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    //     assert!(qap_inst.is_satisfied(qap_wit));
    // //#endif

    let g_A = pk.A_query[0] + qap_wit.d1 * pk.A_query[qap_wit.num_variables() + 1];
    let g_B = pk.B_query[0] + qap_wit.d2 * pk.B_query[qap_wit.num_variables() + 1];
    let g_C = pk.C_query[0] + qap_wit.d3 * pk.C_query[qap_wit.num_variables() + 1];

    let g_H = ppT::G1::zero();
    let g_K = (pk.K_query[0]
        + qap_wit.d1 * pk.K_query[qap_wit.num_variables() + 1]
        + qap_wit.d2 * pk.K_query[qap_wit.num_variables() + 2]
        + qap_wit.d3 * pk.K_query[qap_wit.num_variables() + 3]);

    // // #ifdef DEBUG
    //     for i in 0..qap_wit.num_inputs() + 1
    //     {
    //         assert!(pk.A_query[i].g == ppT::G1::zero());
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
    //     let chunks = 1;
    // //#endif

    ffec::enter_block("Compute the proof");

    ffec::enter_block("Compute answer to A-query", false);
    g_A = g_A
        + kc_multi_exp_with_mixed_addition::<
            ppT::G1,
            ppT::G1,
            ppT::Fr,
            ffec::multi_exp_method_bos_coster,
        >(
            pk.A_query,
            1,
            1 + qap_wit.num_variables(),
            qap_wit.coefficients_for_ABCs.begin(),
            qap_wit.coefficients_for_ABCs.begin() + qap_wit.num_variables(),
            chunks,
        );
    ffec::leave_block("Compute answer to A-query", false);

    ffec::enter_block("Compute answer to B-query", false);
    g_B = g_B
        + kc_multi_exp_with_mixed_addition::<
            ffec::G2<ppT>,
            ffec::G1<ppT>,
            ffec::Fr<ppT>,
            ffec::multi_exp_method_bos_coster,
        >(
            pk.B_query,
            1,
            1 + qap_wit.num_variables(),
            qap_wit.coefficients_for_ABCs.begin(),
            qap_wit.coefficients_for_ABCs.begin() + qap_wit.num_variables(),
            chunks,
        );
    ffec::leave_block("Compute answer to B-query", false);

    ffec::enter_block("Compute answer to C-query", false);
    g_C = g_C
        + kc_multi_exp_with_mixed_addition::<
            ffec::G1<ppT>,
            ffec::G1<ppT>,
            ffec::Fr<ppT>,
            ffec::multi_exp_method_bos_coster,
        >(
            pk.C_query,
            1,
            1 + qap_wit.num_variables(),
            qap_wit.coefficients_for_ABCs.begin(),
            qap_wit.coefficients_for_ABCs.begin() + qap_wit.num_variables(),
            chunks,
        );
    ffec::leave_block("Compute answer to C-query", false);

    ffec::enter_block("Compute answer to H-query", false);
    g_H = g_H
        + ffec::multi_exp::<ffec::G1<ppT>, ffec::Fr<ppT>, ffec::multi_exp_method_BDLO12>(
            pk.H_query.begin(),
            pk.H_query.begin() + qap_wit.degree() + 1,
            qap_wit.coefficients_for_H.begin(),
            qap_wit.coefficients_for_H.begin() + qap_wit.degree() + 1,
            chunks,
        );
    ffec::leave_block("Compute answer to H-query", false);

    ffec::enter_block("Compute answer to K-query", false);
    g_K = g_K
        + ffec::multi_exp_with_mixed_addition::<
            ffec::G1<ppT>,
            ffec::Fr<ppT>,
            ffec::multi_exp_method_bos_coster,
        >(
            pk.K_query.begin() + 1,
            pk.K_query.begin() + 1 + qap_wit.num_variables(),
            qap_wit.coefficients_for_ABCs.begin(),
            qap_wit.coefficients_for_ABCs.begin() + qap_wit.num_variables(),
            chunks,
        );
    ffec::leave_block("Compute answer to K-query", false);

    ffec::leave_block("Compute the proof");

    ffec::leave_block("Call to r1cs_ppzksnark_prover");

    let proof = r1cs_ppzksnark_proof::<ppT>(g_A, g_B, g_C, g_H, g_K);
    proof.print_size();

    return proof;
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

pub fn r1cs_ppzksnark_verifier_weak_IC<ppT>(
    vk: r1cs_ppzksnark_verification_key<ppT>,
    primary_input: r1cs_ppzksnark_primary_input<ppT>,
    proof: r1cs_ppzksnark_proof<ppT>,
) -> bool {
    ffec::enter_block("Call to r1cs_ppzksnark_verifier_weak_IC");
    let pvk = r1cs_ppzksnark_verifier_process_vk::<ppT>::new(vk);
    let result = r1cs_ppzksnark_online_verifier_weak_IC::<ppT>::new(pvk, primary_input, proof);
    ffec::leave_block("Call to r1cs_ppzksnark_verifier_weak_IC");
    return result;
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_ppzksnark_verifier_strong_IC<ppT>(
    vk: r1cs_ppzksnark_verification_key<ppT>,
    primary_input: r1cs_ppzksnark_primary_input<ppT>,
    &proof: r1cs_ppzksnark_proof<ppT>,
) -> bool {
    ffec::enter_block("Call to r1cs_ppzksnark_verifier_strong_IC");
    let pvk = r1cs_ppzksnark_verifier_process_vk::<ppT>::new(vk);
    let result = r1cs_ppzksnark_online_verifier_strong_IC::<ppT>::new(pvk, primary_input, proof);
    ffec::leave_block("Call to r1cs_ppzksnark_verifier_strong_IC");
    return result;
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
pub fn r1cs_ppzksnark_verifier_process_vk<ppT>(
    vk: r1cs_ppzksnark_verification_key<ppT>,
) -> r1cs_ppzksnark_processed_verification_key<ppT> {
    ffec::enter_block("Call to r1cs_ppzksnark_verifier_process_vk");

    let pvk = r1cs_ppzksnark_processed_verification_key::<ppT>::new();
    pvk.pp_G2_one_precomp = ppT::precompute_G2(ppT::G2::one());
    pvk.vk_alphaA_g2_precomp = ppT::precompute_G2(vk.alphaA_g2);
    pvk.vk_alphaB_g1_precomp = ppT::precompute_G1(vk.alphaB_g1);
    pvk.vk_alphaC_g2_precomp = ppT::precompute_G2(vk.alphaC_g2);
    pvk.vk_rC_Z_g2_precomp = ppT::precompute_G2(vk.rC_Z_g2);
    pvk.vk_gamma_g2_precomp = ppT::precompute_G2(vk.gamma_g2);
    pvk.vk_gamma_beta_g1_precomp = ppT::precompute_G1(vk.gamma_beta_g1);
    pvk.vk_gamma_beta_g2_precomp = ppT::precompute_G2(vk.gamma_beta_g2);

    pvk.encoded_IC_query = vk.encoded_IC_query;

    ffec::leave_block("Call to r1cs_ppzksnark_verifier_process_vk");

    return pvk;
}
/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

pub fn r1cs_ppzksnark_online_verifier_weak_IC<ppT>(
    pvk: r1cs_ppzksnark_processed_verification_key<ppT>,
    primary_input: r1cs_ppzksnark_primary_input<ppT>,
    proof: r1cs_ppzksnark_proof<ppT>,
) -> bool {
    ffec::enter_block("Call to r1cs_ppzksnark_online_verifier_weak_IC");
    assert!(pvk.encoded_IC_query.domain_size() >= primary_input.len());

    ffec::enter_block("Compute input-dependent part of A");
    let accumulated_IC = pvk.encoded_IC_query.accumulate_chunk::<ppT::Fr>(
        primary_input.begin(),
        primary_input.end(),
        0,
    );
    let acc = &accumulated_IC.first;
    ffec::leave_block("Compute input-dependent part of A");

    let mut result = true;

    ffec::enter_block("Check if the proof is well-formed");
    if !proof.is_well_formed() {
        if !ffec::inhibit_profiling_info {
            ffec::print_indent();
            print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    ffec::leave_block("Check if the proof is well-formed");

    ffec::enter_block("Online pairing computations");
    ffec::enter_block("Check knowledge commitment for A is valid");
    let proof_g_A_g_precomp = ppT::precompute_G1(proof.g_A.g);
    let proof_g_A_h_precomp = ppT::precompute_G1(proof.g_A.h);
    let kc_A_1 = ppT::miller_loop(proof_g_A_g_precomp, pvk.vk_alphaA_g2_precomp);
    let kc_A_2 = ppT::miller_loop(proof_g_A_h_precomp, pvk.pp_G2_one_precomp);
    let kc_A = ppT::final_exponentiation(kc_A_1 * kc_A_2.unitary_inverse());
    if kc_A != ppT::GT::one() {
        if !ffec::inhibit_profiling_info {
            ffec::print_indent();
            print!("Knowledge commitment for A query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for A is valid");

    ffec::enter_block("Check knowledge commitment for B is valid");
    let proof_g_B_g_precomp = ppT::precompute_G2(proof.g_B.g);
    let proof_g_B_h_precomp = ppT::precompute_G1(proof.g_B.h);
    let kc_B_1 = ppT::miller_loop(pvk.vk_alphaB_g1_precomp, proof_g_B_g_precomp);
    let kc_B_2 = ppT::miller_loop(proof_g_B_h_precomp, pvk.pp_G2_one_precomp);
    let kc_B = ppT::final_exponentiation(kc_B_1 * kc_B_2.unitary_inverse());
    if kc_B != ppT::GT::one() {
        if !ffec::inhibit_profiling_info {
            ffec::print_indent();
            print!("Knowledge commitment for B query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for B is valid");

    ffec::enter_block("Check knowledge commitment for C is valid");
    let proof_g_C_g_precomp = ppT::precompute_G1(proof.g_C.g);
    let proof_g_C_h_precomp = ppT::precompute_G1(proof.g_C.h);
    let kc_C_1 = ppT::miller_loop(proof_g_C_g_precomp, pvk.vk_alphaC_g2_precomp);
    let kc_C_2 = ppT::miller_loop(proof_g_C_h_precomp, pvk.pp_G2_one_precomp);
    let kc_C = ppT::final_exponentiation(kc_C_1 * kc_C_2.unitary_inverse());
    if kc_C != ppT::GT::one() {
        if !ffec::inhibit_profiling_info {
            ffec::print_indent();
            print!("Knowledge commitment for C query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for C is valid");

    ffec::enter_block("Check QAP divisibility");
    // check that g^((A+acc)*B)=g^(H*\Prod(t-\sigma)+C)
    // equivalently, via pairings, that e(g^(A+acc), g^B) = e(g^H, g^Z) + e(g^C, g^1)
    let proof_g_A_g_acc_precomp = ppT::precompute_G1(proof.g_A.g + acc);
    let proof_g_H_precomp = ppT::precompute_G1(proof.g_H);
    let QAP_1 = ppT::miller_loop(proof_g_A_g_acc_precomp, proof_g_B_g_precomp);
    let QAP_23 = ppT::double_miller_loop(
        proof_g_H_precomp,
        pvk.vk_rC_Z_g2_precomp,
        proof_g_C_g_precomp,
        pvk.pp_G2_one_precomp,
    );
    let QAP = ppT::final_exponentiation(QAP_1 * QAP_23.unitary_inverse());
    if QAP != ppT::GT::one() {
        if !ffec::inhibit_profiling_info {
            ffec::print_indent();
            print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check QAP divisibility");

    ffec::enter_block("Check same coefficients were used");
    let proof_g_K_precomp = ppT::precompute_G1(proof.g_K);
    let proof_g_A_g_acc_C_precomp = ppT::precompute_G1((proof.g_A.g + acc) + proof.g_C.g);
    let K_1 = ppT::miller_loop(proof_g_K_precomp, pvk.vk_gamma_g2_precomp);
    let K_23 = ppT::double_miller_loop(
        proof_g_A_g_acc_C_precomp,
        pvk.vk_gamma_beta_g2_precomp,
        pvk.vk_gamma_beta_g1_precomp,
        proof_g_B_g_precomp,
    );
    let K = ppT::final_exponentiation(K_1 * K_23.unitary_inverse());
    if K != ppT::GT::one() {
        if !ffec::inhibit_profiling_info {
            ffec::print_indent();
            print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check same coefficients were used");
    ffec::leave_block("Online pairing computations");
    ffec::leave_block("Call to r1cs_ppzksnark_online_verifier_weak_IC");

    return result;
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

pub fn r1cs_ppzksnark_online_verifier_strong_IC<ppT>(
    pvk: r1cs_ppzksnark_processed_verification_key<ppT>,
    primary_input: r1cs_ppzksnark_primary_input<ppT>,
    proof: r1cs_ppzksnark_proof<ppT>,
) -> bool {
    let result = true;
    ffec::enter_block("Call to r1cs_ppzksnark_online_verifier_strong_IC");

    if pvk.encoded_IC_query.domain_size() != primary_input.len() {
        ffec::print_indent();
        print!(
            "Input length differs from expected (got {}, expected {}).\n",
            primary_input.len(),
            pvk.encoded_IC_query.domain_size()
        );
        result = false;
    } else {
        result = r1cs_ppzksnark_online_verifier_weak_IC(pvk, primary_input, proof);
    }

    ffec::leave_block("Call to r1cs_ppzksnark_online_verifier_strong_IC");
    return result;
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

pub fn r1cs_ppzksnark_affine_verifier_weak_IC<ppT>(
    vk: r1cs_ppzksnark_verification_key<ppT>,
    primary_input: r1cs_ppzksnark_primary_input<ppT>,
    proof: r1cs_ppzksnark_proof<ppT>,
) -> bool {
    ffec::enter_block("Call to r1cs_ppzksnark_affine_verifier_weak_IC");
    assert!(vk.encoded_IC_query.domain_size() >= primary_input.len());

    let pvk_pp_G2_one_precomp = ppT::affine_ate_precompute_G2(ppT::G2::one());
    let pvk_vk_alphaA_g2_precomp = ppT::affine_ate_precompute_G2(vk.alphaA_g2);
    let pvk_vk_alphaB_g1_precomp = ppT::affine_ate_precompute_G1(vk.alphaB_g1);
    let pvk_vk_alphaC_g2_precomp = ppT::affine_ate_precompute_G2(vk.alphaC_g2);
    let pvk_vk_rC_Z_g2_precomp = ppT::affine_ate_precompute_G2(vk.rC_Z_g2);
    let pvk_vk_gamma_g2_precomp = ppT::affine_ate_precompute_G2(vk.gamma_g2);
    let pvk_vk_gamma_beta_g1_precomp = ppT::affine_ate_precompute_G1(vk.gamma_beta_g1);
    let pvk_vk_gamma_beta_g2_precomp = ppT::affine_ate_precompute_G2(vk.gamma_beta_g2);

    ffec::enter_block("Compute input-dependent part of A");
    let accumulated_IC = vk.encoded_IC_query.accumulate_chunk::<ppT::Fr>(
        primary_input.begin(),
        primary_input.end(),
        0,
    );
    assert!(accumulated_IC.is_fully_accumulated());
    let acc = &accumulated_IC.first;
    ffec::leave_block("Compute input-dependent part of A");

    let result = true;
    ffec::enter_block("Check knowledge commitment for A is valid");
    let proof_g_A_g_precomp = ppT::affine_ate_precompute_G1(proof.g_A.g);
    let proof_g_A_h_precomp = ppT::affine_ate_precompute_G1(proof.g_A.h);
    let kc_A_miller = ppT::affine_ate_e_over_e_miller_loop(
        proof_g_A_g_precomp,
        pvk_vk_alphaA_g2_precomp,
        proof_g_A_h_precomp,
        pvk_pp_G2_one_precomp,
    );
    let kc_A = ppT::final_exponentiation(kc_A_miller);

    if kc_A != ppT::GT::one() {
        ffec::print_indent();
        print!("Knowledge commitment for A query incorrect.\n");
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for A is valid");

    ffec::enter_block("Check knowledge commitment for B is valid");
    let proof_g_B_g_precomp = ppT::affine_ate_precompute_G2(proof.g_B.g);
    let proof_g_B_h_precomp = ppT::affine_ate_precompute_G1(proof.g_B.h);
    let kc_B_miller = ppT::affine_ate_e_over_e_miller_loop(
        pvk_vk_alphaB_g1_precomp,
        proof_g_B_g_precomp,
        proof_g_B_h_precomp,
        pvk_pp_G2_one_precomp,
    );
    let kc_B = ppT::final_exponentiation(kc_B_miller);
    if kc_B != ppT::GT::one() {
        ffec::print_indent();
        print!("Knowledge commitment for B query incorrect.\n");
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for B is valid");

    ffec::enter_block("Check knowledge commitment for C is valid");
    let proof_g_C_g_precomp = ppT::affine_ate_precompute_G1(proof.g_C.g);
    let proof_g_C_h_precomp = ppT::affine_ate_precompute_G1(proof.g_C.h);
    let kc_C_miller = ppT::affine_ate_e_over_e_miller_loop(
        proof_g_C_g_precomp,
        pvk_vk_alphaC_g2_precomp,
        proof_g_C_h_precomp,
        pvk_pp_G2_one_precomp,
    );
    let kc_C = ppT::final_exponentiation(kc_C_miller);
    if kc_C != ppT::GT::one() {
        ffec::print_indent();
        print!("Knowledge commitment for C query incorrect.\n");
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for C is valid");

    ffec::enter_block("Check QAP divisibility");
    let proof_g_A_g_acc_precomp = ppT::affine_ate_precompute_G1(proof.g_A.g + acc);
    let proof_g_H_precomp = ppT::affine_ate_precompute_G1(proof.g_H);
    let QAP_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        proof_g_H_precomp,
        pvk_vk_rC_Z_g2_precomp,
        proof_g_C_g_precomp,
        pvk_pp_G2_one_precomp,
        proof_g_A_g_acc_precomp,
        proof_g_B_g_precomp,
    );
    let QAP = ppT::final_exponentiation(QAP_miller);
    if QAP != ppT::GT::one() {
        ffec::print_indent();
        print!("QAP divisibility check failed.\n");
        result = false;
    }
    ffec::leave_block("Check QAP divisibility");

    ffec::enter_block("Check same coefficients were used");
    let proof_g_K_precomp = ppT::affine_ate_precompute_G1(proof.g_K);
    let proof_g_A_g_acc_C_precomp =
        ppT::affine_ate_precompute_G1((proof.g_A.g + acc) + proof.g_C.g);
    let K_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        proof_g_A_g_acc_C_precomp,
        pvk_vk_gamma_beta_g2_precomp,
        pvk_vk_gamma_beta_g1_precomp,
        proof_g_B_g_precomp,
        proof_g_K_precomp,
        pvk_vk_gamma_g2_precomp,
    );
    let K = ppT::final_exponentiation(K_miller);
    if K != ppT::GT::one() {
        ffec::print_indent();
        print!("Same-coefficient check failed.\n");
        result = false;
    }
    ffec::leave_block("Check same coefficients were used");

    ffec::leave_block("Call to r1cs_ppzksnark_affine_verifier_weak_IC");

    return result;
}

//

use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;

// //#endif // R1CS_PPZKSNARK_HPP_

/** @file
*****************************************************************************

Implementation of interfaces for a ppzkSNARK for R1CS.

See r1cs_ppzksnark.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// //#ifndef R1CS_PPZKSNARK_TCC_
// // #define R1CS_PPZKSNARK_TCC_

// use  <algorithm>
// use  <cassert>
// use  <functional>
// use  <iostream>
// use  <sstream>
use ffec::algebra::scalar_multiplication::multiexp;
use ffec::common::profiling;
use ffec::common::utils;

// // #ifdef MULTICORE
// use  <omp.h>
// //#endif

use crate::knowledge_commitment::kc_multiexp;
use crate::reductions::r1cs_to_qap::r1cs_to_qap;

//

impl<ppT> PartialEq for r1cs_ppzksnark_proving_key<ppT> {
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

impl<ppT> fmt::Display for r1cs_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}",
            pk.A_query, pk.B_query, pk.C_query, pk.H_query, pk.K_query, pk.constraint_system,
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

impl<ppT> PartialEq for r1cs_ppzksnark_verification_key<ppT> {
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

impl<ppT> fmt::Display for r1cs_ppzksnark_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            vk.alphaA_g2,
            vk.alphaB_g1,
            vk.alphaC_g2,
            vk.gamma_g2,
            vk.gamma_beta_g1,
            vk.gamma_beta_g2,
            vk.rC_Z_g2,
            vk.encoded_IC_query,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.alphaA_g2;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.alphaB_g1;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.alphaC_g2;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_g2;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_beta_g1;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_beta_g2;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.rC_Z_g2;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> vk.encoded_IC_query;
//     ffec::consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT> PartialEq for r1cs_ppzksnark_processed_verification_key<ppT> {
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

impl<ppT> fmt::Display for r1cs_ppzksnark_processed_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            pvk.pp_G2_one_precomp,
            pvk.vk_alphaA_g2_precomp,
            pvk.vk_alphaB_g1_precomp,
            pvk.vk_alphaC_g2_precomp,
            pvk.vk_rC_Z_g2_precomp,
            pvk.vk_gamma_g2_precomp,
            pvk.vk_gamma_beta_g1_precomp,
            pvk.vk_gamma_beta_g2_precomp,
            pvk.encoded_IC_query,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.pp_G2_one_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaA_g2_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaB_g1_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaC_g2_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_rC_Z_g2_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_g2_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_beta_g1_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_beta_g2_precomp;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.encoded_IC_query;
//     ffec::consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT> PartialEq for r1cs_ppzksnark_proof<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.g_A == other.g_A
            && self.g_B == other.g_B
            && self.g_C == other.g_C
            && self.g_H == other.g_H
            && self.g_K == other.g_K
    }
}

impl<ppT> fmt::Display for r1cs_ppzksnark_proof<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            proof.g_A, proof.g_B, proof.g_C, proof.g_H, proof.g_K,
        )
    }
}

// pub fn
// std::istream& operator>>(std::istream &in, r1cs_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.g_A;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_B;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_C;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_H;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_K;
//     ffec::consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT> r1cs_ppzksnark_verification_key<ppT> {
    pub fn dummy_verification_key(input_size: usize) -> r1cs_ppzksnark_verification_key<ppT> {
        let mut result = r1cs_ppzksnark_verification_key::<ppT>::new();
        result.alphaA_g2 = ppT::Fr::random_element() * ppT::G2::one();
        result.alphaB_g1 = ppT::Fr::random_element() * ppT::G1::one();
        result.alphaC_g2 = ppT::Fr::random_element() * ppT::G2::one();
        result.gamma_g2 = ppT::Fr::random_element() * ppT::G2::one();
        result.gamma_beta_g1 = ppT::Fr::random_element() * ppT::G1::one();
        result.gamma_beta_g2 = ppT::Fr::random_element() * ppT::G2::one();
        result.rC_Z_g2 = ppT::Fr::random_element() * ppT::G2::one();

        let base = ppT::Fr::random_element() * ppT::G1::one();
        let mut v = ppT::G1_vector::new();
        for i in 0..input_size {
            v.push(ppT::Fr::random_element() * ppT::G1::one());
        }

        result.encoded_IC_query = accumulation_vector::<ppT::G1>(base, v);

        return result;
    }
}

//
// //#endif // R1CS_PPZKSNARK_TCC_
