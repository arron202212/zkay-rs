/** @file
*****************************************************************************

Declaration of interfaces for a ppzkSNARK for R1CS with a security proof
in the generic group (GG) model.

This includes:
- class for proving key
- class for verification key
- class for processed verification key
- class for key pair (proving key & verification key)
- class for proof
- generator algorithm
- prover algorithm
- verifier algorithm (with strong or weak input consistency)
- online verifier algorithm (with strong or weak input consistency)

The implementation instantiates the protocol of \[Gro16].


Acronyms:

- R1CS = "Rank-1 Constraint Systems"
- ppzkSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

References:

\[Gro16]:
 "On the Size of Pairing-based Non-interactive Arguments",
 Jens Groth,
 EUROCRYPT 2016,
 <https://eprint.iacr.org/2016/260>


*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// #ifndef R1CS_GG_PPZKSNARK_HPP_
// #define R1CS_GG_PPZKSNARK_HPP_

// use  <memory>

use  <libff/algebra/curves/public_params.hpp>

use  <libsnark/common/data_structures/accumulation_vector.hpp>
use  <libsnark/knowledge_commitment/knowledge_commitment.hpp>
use  <libsnark/relations/constraint_satisfaction_problems/r1cs/r1cs.hpp>
use  <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark_params.hpp>

// namespace libsnark {

/******************************** Proving key ********************************/

// pub fn 
// class r1cs_gg_ppzksnark_proving_key;

// pub fn 
// std::ostream& operator<<(std::ostream &out, &pk:r1cs_gg_ppzksnark_proving_key<ppT>);

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the R1CS GG-ppzkSNARK.
 */

struct r1cs_gg_ppzksnark_proving_key<ppT> {

alpha_g1:    libff::G1<ppT>,
beta_g1:    libff::G1<ppT>,
beta_g2:    libff::G2<ppT>,
delta_g1:    libff::G1<ppT>,
delta_g2:    libff::G2<ppT>,

A_query:    libff::G1_vector<ppT>, // this could be a sparse vector if we had multiexp for those
B_query:    knowledge_commitment_vector<libff::G2<ppT>, libff::G1<ppT> >,
H_query:    libff::G1_vector<ppT>,
L_query:    libff::G1_vector<ppT>,

constraint_system:    r1cs_gg_ppzksnark_constraint_system<ppT>,
}

impl<ppT> r1cs_gg_ppzksnark_proving_key<ppT> {
    // r1cs_gg_ppzksnark_proving_key() {};
    // r1cs_gg_ppzksnark_proving_key<ppT>& operator=(&other:r1cs_gg_ppzksnark_proving_key<ppT>) = default;
    // r1cs_gg_ppzksnark_proving_key(&other:r1cs_gg_ppzksnark_proving_key<ppT>) = default;
    // r1cs_gg_ppzksnark_proving_key(r1cs_gg_ppzksnark_proving_key<ppT> &&other) = default;
    pub fn new(libff::G1<ppT> &&alpha_g1,
                                  libff::G1<ppT> &&beta_g1,
                                  libff::G2<ppT> &&beta_g2,
                                  libff::G1<ppT> &&delta_g1,
                                  libff::G2<ppT> &&delta_g2,
                                  libff::G1_vector<ppT> &&A_query,
                                  knowledge_commitment_vector<libff::G2<ppT>, libff::G1<ppT> > &&B_query,
                                  libff::G1_vector<ppT> &&H_query,
                                  libff::G1_vector<ppT> &&L_query,
                                  r1cs_gg_ppzksnark_constraint_system<ppT> &&constraint_system) :
        alpha_g1(std::move(alpha_g1)),
        beta_g1(std::move(beta_g1)),
        beta_g2(std::move(beta_g2)),
        delta_g1(std::move(delta_g1)),
        delta_g2(std::move(delta_g2)),
        A_query(std::move(A_query)),
        B_query(std::move(B_query)),
        H_query(std::move(H_query)),
        L_query(std::move(L_query)),
        constraint_system(std::move(constraint_system))
    {};

     pub fn g1_size(&self)->usize
    {
        return 1 + A_query.size() + B_query.domain_size() + H_query.size() + L_query.size();
    }

   pub fn g2_size(&self)->usize
    {
        return 1 + B_query.domain_size();
    }

     pub fn g1_sparse_size(&self) ->usize
    {
        return 1 + A_query.size() + B_query.size() + H_query.size() + L_query.size();
    }

    pub fn  g2_sparse_size(&self) ->usize
    {
        return 1 + B_query.size();
    }

     pub fn size_in_bits(&self)->usize
    {
        return (libff::size_in_bits(A_query) + B_query.size_in_bits() +
                libff::size_in_bits(H_query) + libff::size_in_bits(L_query) +
                1 * libff::G1<ppT>::size_in_bits() + 1 * libff::G2<ppT>::size_in_bits());
    }

    fn print_size(&self) 
    {
        libff::print_indent(); printf!("* G1 elements in PK: %zu\n", this->G1_size());
        libff::print_indent(); printf!("* Non-zero G1 elements in PK: %zu\n", this->G1_sparse_size());
        libff::print_indent(); printf!("* G2 elements in PK: %zu\n", this->G2_size());
        libff::print_indent(); printf!("* Non-zero G2 elements in PK: %zu\n", this->G2_sparse_size());
        libff::print_indent(); printf!("* PK size in bits: %zu\n", this->size_in_bits());
    }

    // bool operator==(&other:r1cs_gg_ppzksnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pk:r1cs_gg_ppzksnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_proving_key<ppT> &pk);
}


/******************************* Verification key ****************************/

// pub fn 
// class r1cs_gg_ppzksnark_verification_key;

// pub fn 
// std::ostream& operator<<(std::ostream &out, &vk:r1cs_gg_ppzksnark_verification_key<ppT>);

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_verification_key<ppT> &vk);

/**
 * A verification key for the R1CS GG-ppzkSNARK.
 */

struct r1cs_gg_ppzksnark_verification_key<ppT> {

alpha_g1_beta_g2:    libff::GT<ppT>,
gamma_g2:    libff::G2<ppT>,
delta_g2:    libff::G2<ppT>,

gamma_ABC_g1:    accumulation_vector<libff::G1<ppT> >,
}

impl<ppT> r1cs_gg_ppzksnark_verification_key<ppT> {
    // r1cs_gg_ppzksnark_verification_key() = default;
    pub fn new(alpha_g1_beta_g2:libff::GT<ppT>,
                                       gamma_g2:libff::G2<ppT>,
                                       delta_g2:libff::G2<ppT>,
                                       gamma_ABC_g1:accumulation_vector<libff::G1<ppT> >,) :
        alpha_g1_beta_g2(alpha_g1_beta_g2),
        gamma_g2(gamma_g2),
        delta_g2(delta_g2),
        gamma_ABC_g1(gamma_ABC_g1)
    {};

     pub fn g1_size(&self)->usize
    {
        return gamma_ABC_g1.size();
    }

   pub fn g2_size(&self)->usize
    {
        return 2;
    }

    pub fn gt_size(&self) ->usize
    {
        return 1;
    }

     pub fn size_in_bits(&self)->usize
    {
        // TODO: include GT size
        return (gamma_ABC_g1.size_in_bits() + 2 * libff::G2<ppT>::size_in_bits());
    }

    fn print_size(&self) 
    {
        libff::print_indent(); printf!("* G1 elements in VK: %zu\n", this->G1_size());
        libff::print_indent(); printf!("* G2 elements in VK: %zu\n", this->G2_size());
        libff::print_indent(); printf!("* GT elements in VK: %zu\n", this->GT_size());
        libff::print_indent(); printf!("* VK size in bits: %zu\n", this->size_in_bits());
    }

    // bool operator==(&other:r1cs_gg_ppzksnark_verification_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &vk:r1cs_gg_ppzksnark_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_verification_key<ppT> &vk);

    // static r1cs_gg_ppzksnark_verification_key<ppT> dummy_verification_key(input_size:size_t);
};


/************************ Processed verification key *************************/

// struct r1cs_gg_ppzksnark_processed_verification_key;

// pub fn 
// std::ostream& operator<<(std::ostream &out, &pvk:r1cs_gg_ppzksnark_processed_verification_key<ppT>);

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_processed_verification_key<ppT> &pvk);

/**
 * A processed verification key for the R1CS GG-ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
struct r1cs_gg_ppzksnark_processed_verification_key {

vk_alpha_g1_beta_g2:    libff::GT<ppT>,
vk_gamma_g2_precomp:    libff::G2_precomp<ppT>,
vk_delta_g2_precomp:    libff::G2_precomp<ppT>,

gamma_ABC_g1:    accumulation_vector<libff::G1<ppT> >,

    // bool operator==(&other:r1cs_gg_ppzksnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &pvk:r1cs_gg_ppzksnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_processed_verification_key<ppT> &pvk);
};


/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS GG-ppzkSNARK, which consists of a proving key and a verification key.
 */
struct r1cs_gg_ppzksnark_keypair<ppT> {
pk:    r1cs_gg_ppzksnark_proving_key<ppT>,
vk:    r1cs_gg_ppzksnark_verification_key<ppT>,
}
impl<ppT> r1cs_gg_ppzksnark_keypair<ppT>{
    // r1cs_gg_ppzksnark_keypair() = default;
    // r1cs_gg_ppzksnark_keypair(&other:r1cs_gg_ppzksnark_keypair<ppT>) = default;
    pub fn new(r1cs_gg_ppzksnark_proving_key<ppT> &&pk,
                              r1cs_gg_ppzksnark_verification_key<ppT> &&vk) :
        pk(std::move(pk)),
        vk(std::move(vk))
    {}

    // r1cs_gg_ppzksnark_keypair(r1cs_gg_ppzksnark_keypair<ppT> &&other) = default;
};


/*********************************** Proof ***********************************/

// struct r1cs_gg_ppzksnark_proof;

// pub fn 
// std::ostream& operator<<(std::ostream &out, &proof:r1cs_gg_ppzksnark_proof<ppT>);

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_proof<ppT> &proof);

/**
 * A proof for the R1CS GG-ppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
struct r1cs_gg_ppzksnark_proof<ppT> {

g_A:    libff::G1<ppT>,
g_B:    libff::G2<ppT>,
g_C:    libff::G1<ppT>,
}
impl<ppT>  r1cs_gg_ppzksnark_proof<ppT> {
    pub fn default()
    {
        // invalid proof with valid curve points
        this->g_A = libff::G1<ppT>::one();
        this->g_B = libff::G2<ppT>::one();
        this->g_C = libff::G1<ppT>::one();
    }
    pub fn new(libff::G1<ppT> &&g_A,
                            libff::G2<ppT> &&g_B,
                            libff::G1<ppT> &&g_C) :
        g_A(std::move(g_A)),
        g_B(std::move(g_B)),
        g_C(std::move(g_C))
    {};

     pub fn g1_size(&self)->usize
    {
        return 2;
    }

   pub fn g2_size(&self)->usize
    {
        return 1;
    }

     pub fn size_in_bits(&self)->usize
    {
        return G1_size() * libff::G1<ppT>::size_in_bits() + G2_size() * libff::G2<ppT>::size_in_bits();
    }

    fn print_size(&self) 
    {
        libff::print_indent(); printf!("* G1 elements in proof: %zu\n", this->G1_size());
        libff::print_indent(); printf!("* G2 elements in proof: %zu\n", this->G2_size());
        libff::print_indent(); printf!("* Proof size in bits: %zu\n", this->size_in_bits());
    }

    fn is_well_formed(&self) ->bool
    {
        return (g_A.is_well_formed() &&
                g_B.is_well_formed() &&
                g_C.is_well_formed());
    }

    // bool operator==(&other:r1cs_gg_ppzksnark_proof<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, &proof:r1cs_gg_ppzksnark_proof<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_gg_ppzksnark_proof<ppT> &proof);
};


/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the R1CS GG-ppzkSNARK.
 *
 * Given a R1CS constraint system CS, this algorithm produces proving and verification keys for CS.
 */

pub fn 
 r1cs_gg_ppzksnark_generator<ppT>(&r1cs:r1cs_gg_ppzksnark_constraint_system<ppT>)->r1cs_gg_ppzksnark_keypair<ppT>
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_generator");

    /* Make the B_query "lighter" if possible */
    r1cs_gg_ppzksnark_constraint_system<ppT> r1cs_copy(r1cs);
    r1cs_copy.swap_AB_if_beneficial();

    /* Generate secret randomness */
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> t =);
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> alpha =);
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> beta =);
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> gamma =);
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> delta =);
    gamma.inverse(:libff::Fr<ppT> gamma_inverse =);
    delta.inverse(:libff::Fr<ppT> delta_inverse =);

    /* A quadratic arithmetic program evaluated at t. */
    qap_instance_evaluation<libff::Fr<ppT> > qap = r1cs_to_qap_instance_map_with_evaluation(r1cs_copy, t);

    libff::print_indent(); printf!("* QAP number of variables: %zu\n", qap.num_variables());
    libff::print_indent(); printf!("* QAP pre degree: %zu\n", r1cs_copy.constraints.size());
    libff::print_indent(); printf!("* QAP degree: %zu\n", qap.degree());
    libff::print_indent(); printf!("* QAP number of input variables: %zu\n", qap.num_inputs());

    libff::enter_block("Compute query densities");
    size_t non_zero_At = 0;
    size_t non_zero_Bt = 0;
    for i in 0..qap.num_variables() + 1
    {
        if !qap.At[i].is_zero()
        {
            ++non_zero_At;
        }
        if !qap.Bt[i].is_zero()
        {
            ++non_zero_Bt;
        }
    }
    libff::leave_block("Compute query densities");

    /* qap.{At,Bt,Ct,Ht} are now in unspecified state, but we do not use them later */
    libff::Fr_vector<ppT> At = std::move(qap.At);
    libff::Fr_vector<ppT> Bt = std::move(qap.Bt);
    libff::Fr_vector<ppT> Ct = std::move(qap.Ct);
    libff::Fr_vector<ppT> Ht = std::move(qap.Ht);

    /* The gamma inverse product component: (beta*A_i(t) + alpha*B_i(t) + C_i(t)) * gamma^{-1}. */
    libff::enter_block("Compute gamma_ABC for R1CS verification key");
    libff::Fr_vector<ppT> gamma_ABC;
    gamma_ABC.reserve(qap.num_inputs());

    Ct[0]:libff::Fr<ppT> gamma_ABC_0 = (beta * At[0] + alpha * Bt[0] +) * gamma_inverse;
    for i in 1..qap.num_inputs() + 1
    {
        gamma_ABC.emplace_back((beta * At[i] + alpha * Bt[i] + Ct[i]) * gamma_inverse);
    }
    libff::leave_block("Compute gamma_ABC for R1CS verification key");

    /* The delta inverse product component: (beta*A_i(t) + alpha*B_i(t) + C_i(t)) * delta^{-1}. */
    libff::enter_block("Compute L query for R1CS proving key");
    libff::Fr_vector<ppT> Lt;
    Lt.reserve(qap.num_variables() - qap.num_inputs());

    qap.num_inputs(:size_t Lt_offset =) + 1;
    for i in 0..qap.num_variables() - qap.num_inputs()
    {
        Lt.emplace_back((beta * At[Lt_offset + i] + alpha * Bt[Lt_offset + i] + Ct[Lt_offset + i]) * delta_inverse);
    }
    libff::leave_block("Compute L query for R1CS proving key");

    /**
     * Note that H for Groth's proof system is degree d-2, but the QAP
     * reduction returns coefficients for degree d polynomial H (in
     * style of PGHR-type proof systems)
     */
    Ht.resize(Ht.size() - 2);

#ifdef MULTICORE
    override:size_t chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
#else
    const size_t chunks = 1;
#endif

    libff::enter_block("Generating G1 MSM window table");
    libff::G1<ppT>::random_element(:libff::G1<ppT> g1_generator =);
    qap.num_variables(:size_t g1_scalar_count = non_zero_At + non_zero_Bt +);
    libff::Fr<ppT>::size_in_bits(:size_t g1_scalar_size =);
    >(g1_scalar_count:size_t g1_window_size = libff::get_exp_window_size<libff::G1<ppT>);

    libff::print_indent(); printf!("* G1 window: %zu\n", g1_window_size);
    libff::window_table<libff::G1<ppT> > g1_table = libff::get_window_table(g1_scalar_size, g1_window_size, g1_generator);
    libff::leave_block("Generating G1 MSM window table");

    libff::enter_block("Generating G2 MSM window table");
    libff::G2<ppT>::random_element(:libff::G2<ppT> G2_gen =);
    const size_t g2_scalar_count = non_zero_Bt;
    libff::Fr<ppT>::size_in_bits(:size_t g2_scalar_size =);
    size_t g2_window_size = libff::get_exp_window_size<libff::G2<ppT> >(g2_scalar_count);

    libff::print_indent(); printf!("* G2 window: %zu\n", g2_window_size);
    libff::window_table<libff::G2<ppT> > g2_table = libff::get_window_table(g2_scalar_size, g2_window_size, G2_gen);
    libff::leave_block("Generating G2 MSM window table");

    libff::enter_block("Generate R1CS proving key");
    libff::G1<ppT> alpha_g1 = alpha * g1_generator;
    libff::G1<ppT> beta_g1 = beta * g1_generator;
    libff::G2<ppT> beta_g2 = beta * G2_gen;
    libff::G1<ppT> delta_g1 = delta * g1_generator;
    libff::G2<ppT> delta_g2 = delta * G2_gen;

    libff::enter_block("Generate queries");
    libff::enter_block("Compute the A-query", false);
    libff::G1_vector<ppT> A_query = batch_exp(g1_scalar_size, g1_window_size, g1_table, At);
#ifdef USE_MIXED_ADDITION
    libff::batch_to_special<libff::G1<ppT> >(A_query);
#endif
    libff::leave_block("Compute the A-query", false);

    libff::enter_block("Compute the B-query", false);
    knowledge_commitment_vector<libff::G2<ppT>, libff::G1<ppT> > B_query = kc_batch_exp(libff::Fr<ppT>::size_in_bits(), g2_window_size, g1_window_size, g2_table, g1_table, libff::Fr<ppT>::one(), libff::Fr<ppT>::one(), Bt, chunks);
    // NOTE: if USE_MIXED_ADDITION is defined,
    // kc_batch_exp will convert its output to special form internally
    libff::leave_block("Compute the B-query", false);

    libff::enter_block("Compute the H-query", false);
    libff::G1_vector<ppT> H_query = batch_exp_with_coeff(g1_scalar_size, g1_window_size, g1_table, qap.Zt * delta_inverse, Ht);
#ifdef USE_MIXED_ADDITION
    libff::batch_to_special<libff::G1<ppT> >(H_query);
#endif
    libff::leave_block("Compute the H-query", false);

    libff::enter_block("Compute the L-query", false);
    libff::G1_vector<ppT> L_query = batch_exp(g1_scalar_size, g1_window_size, g1_table, Lt);
#ifdef USE_MIXED_ADDITION
    libff::batch_to_special<libff::G1<ppT> >(L_query);
#endif
    libff::leave_block("Compute the L-query", false);
    libff::leave_block("Generate queries");

    libff::leave_block("Generate R1CS proving key");

    libff::enter_block("Generate R1CS verification key");
    libff::GT<ppT> alpha_g1_beta_g2 = ppT::reduced_pairing(alpha_g1, beta_g2);
    libff::G2<ppT> gamma_g2 = gamma * G2_gen;

    libff::enter_block("Encode gamma_ABC for R1CS verification key");
    libff::G1<ppT> gamma_ABC_g1_0 = gamma_ABC_0 * g1_generator;
    libff::G1_vector<ppT> gamma_ABC_g1_values = batch_exp(g1_scalar_size, g1_window_size, g1_table, gamma_ABC);
    libff::leave_block("Encode gamma_ABC for R1CS verification key");
    libff::leave_block("Generate R1CS verification key");

    libff::leave_block("Call to r1cs_gg_ppzksnark_generator");

    accumulation_vector<libff::G1<ppT> > gamma_ABC_g1(std::move(gamma_ABC_g1_0), std::move(gamma_ABC_g1_values));

    r1cs_gg_ppzksnark_verification_key<ppT> vk = r1cs_gg_ppzksnark_verification_key<ppT>(alpha_g1_beta_g2,
                                                                                         gamma_g2,
                                                                                         delta_g2,
                                                                                         gamma_ABC_g1);

    r1cs_gg_ppzksnark_proving_key<ppT> pk = r1cs_gg_ppzksnark_proving_key<ppT>(std::move(alpha_g1),
                                                                               std::move(beta_g1),
                                                                               std::move(beta_g2),
                                                                               std::move(delta_g1),
                                                                               std::move(delta_g2),
                                                                               std::move(A_query),
                                                                               std::move(B_query),
                                                                               std::move(H_query),
                                                                               std::move(L_query),
                                                                               std::move(r1cs_copy));

    pk.print_size();
    vk.print_size();

    return r1cs_gg_ppzksnark_keypair<ppT>(std::move(pk), std::move(vk));
}
/**
 * A prover algorithm for the R1CS GG-ppzkSNARK.
 *
 * Given a R1CS primary input X and a R1CS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the R1CS constraint system that was given as input to the generator algorithm.
 */

pub fn 
 r1cs_gg_ppzksnark_prover<ppT>(pk:r1cs_gg_ppzksnark_proving_key<ppT>,
                                                      primary_input:r1cs_gg_ppzksnark_primary_input<ppT>,
                                                      auxiliary_input:r1cs_gg_ppzksnark_auxiliary_input<ppT>,)->r1cs_gg_ppzksnark_proof<ppT>
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_prover");

// #ifdef DEBUG
//     assert(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
// #endif

    libff::enter_block("Compute the polynomial H");
    libff::Fr<ppT>::zero():qap_witness<libff::Fr<ppT> > qap_wit = r1cs_to_qap_witness_map(pk.constraint_system, primary_input, auxiliary_input, libff::Fr<ppT>::zero(), libff::Fr<ppT>::zero());

    /* We are dividing degree 2(d-1) polynomial by degree d polynomial
       and not adding a PGHR-style ZK-patch, so our H is degree d-2 */
    assert(!qap_wit.coefficients_for_H[qap_wit.degree()-2].is_zero());
    assert(qap_wit.coefficients_for_H[qap_wit.degree()-1].is_zero());
    assert(qap_wit.coefficients_for_H[qap_wit.degree()].is_zero());
    libff::leave_block("Compute the polynomial H");

#ifdef DEBUG
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> t =);
    qap_instance_evaluation<libff::Fr<ppT> > qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    assert(qap_inst.is_satisfied(qap_wit));
#endif

    /* Choose two random field elements for prover zero-knowledge. */
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> r =);
    libff::Fr<ppT>::random_element(:libff::Fr<ppT> s =);

#ifdef DEBUG
    assert(qap_wit.coefficients_for_ABCs.size() == qap_wit.num_variables());
    assert(pk.A_query.size() == qap_wit.num_variables()+1);
    assert(pk.B_query.domain_size() == qap_wit.num_variables()+1);
    assert(pk.H_query.size() == qap_wit.degree() - 1);
    assert(pk.L_query.size() == qap_wit.num_variables() - qap_wit.num_inputs());
#endif

#ifdef MULTICORE
    override:size_t chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
#else
    const size_t chunks = 1;
#endif

    libff::enter_block("Compute the proof");

    libff::enter_block("Compute evaluation to A-query", false);
    // TODO: sort out indexing
    libff::Fr_vector<ppT> const_padded_assignment(1, libff::Fr<ppT>::one());
    const_padded_assignment.insert(const_padded_assignment.end(), qap_wit.coefficients_for_ABCs.begin(), qap_wit.coefficients_for_ABCs.end());

    libff::G1<ppT> evaluation_At = libff::multi_exp_with_mixed_addition<libff::G1<ppT>,
                                                                        libff::Fr<ppT>,
                                                                        libff::multi_exp_method_BDLO12>(
        pk.A_query.begin(),
        pk.A_query.begin() + qap_wit.num_variables() + 1,
        const_padded_assignment.begin(),
        const_padded_assignment.begin() + qap_wit.num_variables() + 1,
        chunks);
    libff::leave_block("Compute evaluation to A-query", false);

    libff::enter_block("Compute evaluation to B-query", false);
    knowledge_commitment<libff::G2<ppT>, libff::G1<ppT> > evaluation_Bt = kc_multi_exp_with_mixed_addition<libff::G2<ppT>,
                                                                                                           libff::G1<ppT>,
                                                                                                           libff::Fr<ppT>,
                                                                                                           libff::multi_exp_method_BDLO12>(
        pk.B_query,
        0,
        qap_wit.num_variables() + 1,
        const_padded_assignment.begin(),
        const_padded_assignment.begin() + qap_wit.num_variables() + 1,
        chunks);
    libff::leave_block("Compute evaluation to B-query", false);

    libff::enter_block("Compute evaluation to H-query", false);
    libff::G1<ppT> evaluation_Ht = libff::multi_exp<libff::G1<ppT>,
                                                    libff::Fr<ppT>,
                                                    libff::multi_exp_method_BDLO12>(
        pk.H_query.begin(),
        pk.H_query.begin() + (qap_wit.degree() - 1),
        qap_wit.coefficients_for_H.begin(),
        qap_wit.coefficients_for_H.begin() + (qap_wit.degree() - 1),
        chunks);
    libff::leave_block("Compute evaluation to H-query", false);

    libff::enter_block("Compute evaluation to L-query", false);
    libff::G1<ppT> evaluation_Lt = libff::multi_exp_with_mixed_addition<libff::G1<ppT>,
                                                                        libff::Fr<ppT>,
                                                                        libff::multi_exp_method_BDLO12>(
        pk.L_query.begin(),
        pk.L_query.end(),
        const_padded_assignment.begin() + qap_wit.num_inputs() + 1,
        const_padded_assignment.begin() + qap_wit.num_variables() + 1,
        chunks);
    libff::leave_block("Compute evaluation to L-query", false);

    /* A = alpha + sum_i(a_i*A_i(t)) + r*delta */
    libff::G1<ppT> g1_A = pk.alpha_g1 + evaluation_At + r * pk.delta_g1;

    /* B = beta + sum_i(a_i*B_i(t)) + s*delta */
    libff::G1<ppT> g1_B = pk.beta_g1 + evaluation_Bt.h + s * pk.delta_g1;
    libff::G2<ppT> g2_B = pk.beta_g2 + evaluation_Bt.g + s * pk.delta_g2;

    /* C = sum_i(a_i*((beta*A_i(t) + alpha*B_i(t) + C_i(t)) + H(t)*Z(t))/delta) + A*s + r*b - r*s*delta */
    libff::G1<ppT> g1_C = evaluation_Ht + evaluation_Lt + s *  g1_A + r * g1_B - (r * s) * pk.delta_g1;

    libff::leave_block("Compute the proof");

    libff::leave_block("Call to r1cs_gg_ppzksnark_prover");

    r1cs_gg_ppzksnark_proof<ppT> proof = r1cs_gg_ppzksnark_proof<ppT>(std::move(g1_A), std::move(g2_B), std::move(g1_C));
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

pub fn 
 r1cs_gg_ppzksnark_verifier_weak_IC<ppT>(vk:r1cs_gg_ppzksnark_verification_key<ppT>,
                                        primary_input:r1cs_gg_ppzksnark_primary_input<ppT>,
                                        proof:r1cs_gg_ppzksnark_proof<ppT>,)->bool
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_verifier_weak_IC");
    r1cs_gg_ppzksnark_processed_verification_key<ppT> pvk = r1cs_gg_ppzksnark_verifier_process_vk<ppT>(vk);
    bool result = r1cs_gg_ppzksnark_online_verifier_weak_IC<ppT>(pvk, primary_input, proof);
    libff::leave_block("Call to r1cs_gg_ppzksnark_verifier_weak_IC");
    return result;
}


/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

pub fn 
 r1cs_gg_ppzksnark_verifier_strong_IC<ppT>(vk:r1cs_gg_ppzksnark_verification_key<ppT>,
                                          primary_input:r1cs_gg_ppzksnark_primary_input<ppT>,
                                          proof:r1cs_gg_ppzksnark_proof<ppT>,)->bool
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_verifier_strong_IC");
    r1cs_gg_ppzksnark_processed_verification_key<ppT> pvk = r1cs_gg_ppzksnark_verifier_process_vk<ppT>(vk);
    bool result = r1cs_gg_ppzksnark_online_verifier_strong_IC<ppT>(pvk, primary_input, proof);
    libff::leave_block("Call to r1cs_gg_ppzksnark_verifier_strong_IC");
    return result;
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

pub fn 
 r1cs_gg_ppzksnark_verifier_process_vk<ppT>(&vk:r1cs_gg_ppzksnark_verification_key<ppT>)->r1cs_gg_ppzksnark_processed_verification_key<ppT>
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_verifier_process_vk");

    r1cs_gg_ppzksnark_processed_verification_key<ppT> pvk;
    pvk.vk_alpha_g1_beta_g2 = vk.alpha_g1_beta_g2;
    pvk.vk_gamma_g2_precomp = ppT::precompute_G2(vk.gamma_g2);
    pvk.vk_delta_g2_precomp = ppT::precompute_G2(vk.delta_g2);
    pvk.gamma_ABC_g1 = vk.gamma_ABC_g1;

    libff::leave_block("Call to r1cs_gg_ppzksnark_verifier_process_vk");

    return pvk;
}

/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

pub fn 
 r1cs_gg_ppzksnark_online_verifier_weak_IC<ppT>(pvk:r1cs_gg_ppzksnark_processed_verification_key<ppT>,
                                               primary_input:r1cs_gg_ppzksnark_primary_input<ppT>,
                                               proof:r1cs_gg_ppzksnark_proof<ppT>,)->bool
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_online_verifier_weak_IC");
    assert(pvk.gamma_ABC_g1.domain_size() >= primary_input.size());

    libff::enter_block("Accumulate input");
    primary_input.end():accumulation_vector<libff::G1<ppT> > accumulated_IC = pvk.gamma_ABC_g1.template accumulate_chunk<libff::Fr<ppT> >(primary_input.begin(), 0);
    const libff::G1<ppT> &acc = accumulated_IC.first;
    libff::leave_block("Accumulate input");

    bool result = true;

    libff::enter_block("Check if the proof is well-formed");
    if !proof.is_well_formed()
    {
        if !libff::inhibit_profiling_info
        {
            libff::print_indent(); printf!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    libff::leave_block("Check if the proof is well-formed");

    libff::enter_block("Online pairing computations");
    libff::enter_block("Check QAP divisibility");
    ppT::precompute_G1(proof.g_A:libff::G1_precomp<ppT> proof_g_A_precomp =);
    ppT::precompute_G2(proof.g_B:libff::G2_precomp<ppT> proof_g_B_precomp =);
    ppT::precompute_G1(proof.g_C:libff::G1_precomp<ppT> proof_g_C_precomp =);
    ppT::precompute_G1(acc:libff::G1_precomp<ppT> acc_precomp =);

    ppT::miller_loop(proof_g_A_precomp:libff::Fqk<ppT> QAP1 =  proof_g_B_precomp);
    const libff::Fqk<ppT> QAP2 = ppT::double_miller_loop(
        acc_precomp, pvk.vk_gamma_g2_precomp,
        proof_g_C_precomp, pvk.vk_delta_g2_precomp);
    QAP2.unitary_inverse():libff::GT<ppT> QAP = ppT::final_exponentiation(QAP1 *);

    if QAP != pvk.vk_alpha_g1_beta_g2
    {
        if !libff::inhibit_profiling_info
        {
            libff::print_indent(); printf!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    libff::leave_block("Check QAP divisibility");
    libff::leave_block("Online pairing computations");

    libff::leave_block("Call to r1cs_gg_ppzksnark_online_verifier_weak_IC");

    return result;
}
/**
 * A verifier algorithm for the R1CS GG-ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

pub fn 
 r1cs_gg_ppzksnark_online_verifier_strong_IC<ppT>(pvk:r1cs_gg_ppzksnark_processed_verification_key<ppT>,
                                                 primary_input:r1cs_gg_ppzksnark_primary_input<ppT>,
                                                 proof:r1cs_gg_ppzksnark_proof<ppT>,)->bool
{
    bool result = true;
    libff::enter_block("Call to r1cs_gg_ppzksnark_online_verifier_strong_IC");

    if pvk.gamma_ABC_g1.domain_size() != primary_input.size()
    {
        libff::print_indent(); printf!("Input length differs from expected (got %zu, expected %zu).\n", primary_input.size(), pvk.gamma_ABC_g1.domain_size());
        result = false;
    }
    else
    {
        result = r1cs_gg_ppzksnark_online_verifier_weak_IC(pvk, primary_input, proof);
    }

    libff::leave_block("Call to r1cs_gg_ppzksnark_online_verifier_strong_IC");
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


pub fn 
 r1cs_gg_ppzksnark_affine_verifier_weak_IC<ppT>(vk:r1cs_gg_ppzksnark_verification_key<ppT>,
                                               primary_input:r1cs_gg_ppzksnark_primary_input<ppT>,
                                               proof:r1cs_gg_ppzksnark_proof<ppT>,)->bool
{
    libff::enter_block("Call to r1cs_gg_ppzksnark_affine_verifier_weak_IC");
    assert(vk.gamma_ABC_g1.domain_size() >= primary_input.size());

    libff::affine_ate_G2_precomp<ppT> pvk_vk_gamma_g2_precomp = ppT::affine_ate_precompute_G2(vk.gamma_g2);
    libff::affine_ate_G2_precomp<ppT> pvk_vk_delta_g2_precomp = ppT::affine_ate_precompute_G2(vk.delta_g2);

    libff::enter_block("Accumulate input");
    primary_input.end():accumulation_vector<libff::G1<ppT> > accumulated_IC = vk.gamma_ABC_g1.template accumulate_chunk<libff::Fr<ppT> >(primary_input.begin(), 0);
    const libff::G1<ppT> &acc = accumulated_IC.first;
    libff::leave_block("Accumulate input");

    bool result = true;

    libff::enter_block("Check if the proof is well-formed");
    if !proof.is_well_formed()
    {
        if !libff::inhibit_profiling_info
        {
            libff::print_indent(); printf!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    libff::leave_block("Check if the proof is well-formed");

    libff::enter_block("Check QAP divisibility");
    ppT::affine_ate_precompute_G1(proof.g_A:libff::affine_ate_G1_precomp<ppT> proof_g_A_precomp =);
    ppT::affine_ate_precompute_G2(proof.g_B:libff::affine_ate_G2_precomp<ppT> proof_g_B_precomp =);
    ppT::affine_ate_precompute_G1(proof.g_C:libff::affine_ate_G1_precomp<ppT> proof_g_C_precomp =);
    ppT::affine_ate_precompute_G1(acc:libff::affine_ate_G1_precomp<ppT> acc_precomp =);

    const libff::Fqk<ppT> QAP_miller = ppT::affine_ate_e_times_e_over_e_miller_loop(
        acc_precomp, pvk_vk_gamma_g2_precomp,
        proof_g_C_precomp, pvk_vk_delta_g2_precomp,
        proof_g_A_precomp,  proof_g_B_precomp);
    ppT::final_exponentiation(QAP_miller.unitary_inverse():libff::GT<ppT> QAP =);

    if QAP != vk.alpha_g1_beta_g2
    {
        if !libff::inhibit_profiling_info
        {
            libff::print_indent(); printf!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    libff::leave_block("Check QAP divisibility");

    libff::leave_block("Call to r1cs_gg_ppzksnark_affine_verifier_weak_IC");

    return result;
}

// } // libsnark

use  <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.tcc>

// #endif // R1CS_GG_PPZKSNARK_HPP_



/** @file
*****************************************************************************

Implementation of interfaces for a ppzkSNARK for R1CS.

See r1cs_gg_ppzksnark.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// #ifndef R1CS_GG_PPZKSNARK_TCC_
// #define R1CS_GG_PPZKSNARK_TCC_

// use  <algorithm>
// use  <cassert>
// use  <functional>
// use  <iostream>
// use  <sstream>

use  <libff/algebra/scalar_multiplication/multiexp.hpp>
use  <libff/common/profiling.hpp>
use  <libff/common/utils.hpp>

// #ifdef MULTICORE
// use  <omp.h>
// #endif

use  <libsnark/knowledge_commitment/kc_multiexp.hpp>
use  <libsnark/reductions/r1cs_to_qap/r1cs_to_qap.hpp>

// namespace libsnark {

impl<ppT> PartialEq for r1cs_gg_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        this->alpha_g1 == other.alpha_g1 &&
            this->beta_g1 == other.beta_g1 &&
            this->beta_g2 == other.beta_g2 &&
            this->delta_g1 == other.delta_g1 &&
            this->delta_g2 == other.delta_g2 &&
            this->A_query == other.A_query &&
            this->B_query == other.B_query &&
            this->H_query == other.H_query &&
            this->L_query == other.L_query &&
            this->constraint_system == other.constraint_system
    }
}

impl<ppT> fmt::Display for r1cs_gg_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
    out << pk.alpha_g1 << OUTPUT_NEWLINE;
    out << pk.beta_g1 << OUTPUT_NEWLINE;
    out << pk.beta_g2 << OUTPUT_NEWLINE;
    out << pk.delta_g1 << OUTPUT_NEWLINE;
    out << pk.delta_g2 << OUTPUT_NEWLINE;
    out << pk.A_query;
    out << pk.B_query;
    out << pk.H_query;
    out << pk.L_query;
    out << pk.constraint_system;
)
    }
}

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_proving_key<ppT> &pk)
// {
//     in >> pk.alpha_g1;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pk.beta_g1;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pk.beta_g2;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pk.delta_g1;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pk.delta_g2;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pk.A_query;
//     in >> pk.B_query;
//     in >> pk.H_query;
//     in >> pk.L_query;
//     in >> pk.constraint_system;

//     return in;
// }


impl<ppT> PartialEq for r1cs_gg_ppzksnark_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
     this->alpha_g1_beta_g2 == other.alpha_g1_beta_g2 &&
            this->gamma_g2 == other.gamma_g2 &&
            this->delta_g2 == other.delta_g2 &&
            this->gamma_ABC_g1 == other.gamma_ABC_g1
    }
}


impl<ppT> fmt::Display for r1cs_gg_ppzksnark_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
    out << vk.alpha_g1_beta_g2 << OUTPUT_NEWLINE;
    out << vk.gamma_g2 << OUTPUT_NEWLINE;
    out << vk.delta_g2 << OUTPUT_NEWLINE;
    out << vk.gamma_ABC_g1 << OUTPUT_NEWLINE;
)
    }
}


// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.alpha_g1_beta_g2;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_g2;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.delta_g2;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_ABC_g1;
//     libff::consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT> PartialEq for r1cs_gg_ppzksnark_processed_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        this->vk_alpha_g1_beta_g2 == other.vk_alpha_g1_beta_g2 &&
            this->vk_gamma_g2_precomp == other.vk_gamma_g2_precomp &&
            this->vk_delta_g2_precomp == other.vk_delta_g2_precomp &&
            this->gamma_ABC_g1 == other.gamma_ABC_g1
    }
}


impl<ppT> fmt::Display for r1cs_gg_ppzksnark_processed_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
  out << pvk.vk_alpha_g1_beta_g2 << OUTPUT_NEWLINE;
    out << pvk.vk_gamma_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_delta_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.gamma_ABC_g1 << OUTPUT_NEWLINE;
)
    }
}


// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.vk_alpha_g1_beta_g2;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_g2_precomp;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_delta_g2_precomp;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.gamma_ABC_g1;
//     libff::consume_OUTPUT_NEWLINE(in);

//     return in;
// }


impl<ppT> PartialEq for r1cs_gg_ppzksnark_proof<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        this->g_A == other.g_A &&
            this->g_B == other.g_B &&
            this->g_C == other.g_C
    }
}



use std::fmt;
impl<ppT> fmt::Display for r1cs_gg_ppzksnark_proof<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
 out << proof.g_A << OUTPUT_NEWLINE;
    out << proof.g_B << OUTPUT_NEWLINE;
    out << proof.g_C << OUTPUT_NEWLINE;
)
    }
}

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_gg_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.g_A;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_B;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_C;
//     libff::consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl r1cs_gg_ppzksnark_verification_key<ppT>{
pub fn 
 dummy_verification_key<ppT>(input_size:size_t)->r1cs_gg_ppzksnark_verification_key<ppT>
{
    r1cs_gg_ppzksnark_verification_key<ppT> result;
    result.alpha_g1_beta_g2 = libff::Fr<ppT>::random_element() * libff::GT<ppT>::random_element();
    result.gamma_g2 = libff::G2<ppT>::random_element();
    result.delta_g2 = libff::G2<ppT>::random_element();

    libff::G1<ppT> base = libff::G1<ppT>::random_element();
    libff::G1_vector<ppT> v;
    for i in 0..input_size
    {
        v.emplace_back(libff::G1<ppT>::random_element());
    }

    result.gamma_ABC_g1 = accumulation_vector<libff::G1<ppT> >(std::move(base), std::move(v));

    return result;
}

}



// } // libsnark
// #endif // R1CS_GG_PPZKSNARK_TCC_
