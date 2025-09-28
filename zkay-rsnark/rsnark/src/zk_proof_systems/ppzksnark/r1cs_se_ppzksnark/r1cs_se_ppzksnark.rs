/** @file
 *****************************************************************************

 Declaration of interfaces for a SEppzkSNARK for R1CS.

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

 The implementation instantiates (a modification of) the protocol of \[GM17],
 by following extending, and optimizing the approach described in \[BCTV14].


 Acronyms:

 - R1CS = "Rank-1 Constraint Systems"
 - SEppzkSNARK = "Simulation-Extractable PreProcessing Zero-Knowledge Succinct
     Non-interactive ARgument of Knowledge"

 References:

 \[BCTV14]:
 "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
 Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
 USENIX Security 2014,
 <http://eprint.iacr.org/2013/879>

 \[GM17]:
 "Snarky Signatures: Minimal Signatures of Knowledge from
  Simulation-Extractable SNARKs",
 Jens Groth and Mary Maller,
 IACR-CRYPTO-2017,
 <https://eprint.iacr.org/2017/540>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef R1CS_SE_PPZKSNARK_HPP_
#define R1CS_SE_PPZKSNARK_HPP_

use  <memory>

use  <libff/algebra/curves/public_params.hpp>

use  <libsnark/common/data_structures/accumulation_vector.hpp>
use  <libsnark/knowledge_commitment/knowledge_commitment.hpp>
use  <libsnark/relations/constraint_satisfaction_problems/r1cs/r1cs.hpp>
use  <libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark_params.hpp>

// namespace libsnark {

// /******************************** Proving key ********************************/


/**
 * A proving key for the R1CS SEppzkSNARK.
 */

struct r1cs_se_ppzksnark_proving_key<ppT> {

    // G^{gamma * A_i(t)} for 0 <= i <= sap.num_variables()
A_query:    libff::G1_vector<ppT>,

    // H^{gamma * A_i(t)} for 0 <= i <= sap.num_variables()
B_query:    libff::G2_vector<ppT>,

    // G^{gamma^2 * C_i(t) + (alpha + beta) * gamma * A_i(t)}
    // for sap.num_inputs() + 1 < i <= sap.num_variables()
C_query_1:    libff::G1_vector<ppT>,

    // G^{2 * gamma^2 * Z(t) * A_i(t)} for 0 <= i <= sap.num_variables()
C_query_2:    libff::G1_vector<ppT>,

    // G^{gamma * Z(t)}
G_gamma_Z:    libff::G1<ppT>,

    // H^{gamma * Z(t)}
H_gamma_Z:    libff::G2<ppT>,

    // G^{(alpha + beta) * gamma * Z(t)}
G_ab_gamma_Z:    libff::G1<ppT>,

    // G^{gamma^2 * Z(t)^2}
G_gamma2_Z2:    libff::G1<ppT>,

    // G^{gamma^2 * Z(t) * t^i} for 0 <= i < sap.degree
G_gamma2_Z_t:    libff::G1_vector<ppT>,

constraint_system:    r1cs_se_ppzksnark_constraint_system<ppT>,

}
impl<ppT> r1cs_se_ppzksnark_proving_key{
    // r1cs_se_ppzksnark_proving_key() {};
    // r1cs_se_ppzksnark_proving_key<ppT>& operator=(other:&r1cs_se_ppzksnark_proving_key<ppT>) = default;
    // r1cs_se_ppzksnark_proving_key(other:&r1cs_se_ppzksnark_proving_key<ppT>) = default;
    // r1cs_se_ppzksnark_proving_key(r1cs_se_ppzksnark_proving_key<ppT> &&other) = default;
    r1cs_se_ppzksnark_proving_key(
A_query:&        libff::G1_vector<ppT>,
B_query:&        libff::G2_vector<ppT>,
C_query_1:&        libff::G1_vector<ppT>,
C_query_2:&        libff::G1_vector<ppT>,
G_gamma_Z:&        libff::G1<ppT>,
H_gamma_Z:&        libff::G2<ppT>,
G_ab_gamma_Z:&        libff::G1<ppT>,
G_gamma2_Z2:&        libff::G1<ppT>,
G_gamma2_Z_t:&        libff::G1_vector<ppT>,
constraint_system:&        r1cs_se_ppzksnark_constraint_system<ppT>,
) :
        A_query(std::move(A_query)),
        B_query(std::move(B_query)),
        C_query_1(std::move(C_query_1)),
        C_query_2(std::move(C_query_2)),
        G_gamma_Z(G_gamma_Z),
        H_gamma_Z(H_gamma_Z),
        G_ab_gamma_Z(G_ab_gamma_Z),
        G_gamma2_Z2(G_gamma2_Z2),
        G_gamma2_Z_t(std::move(G_gamma2_Z_t)),
        constraint_system(std::move(constraint_system))
    {};

    pub fn G1_size(&self)->usize
    {
         A_query.size() + C_query_1.size() + C_query_2.size() + 3
               + G_gamma2_Z_t.size()
    }

    pub fn G2_size(&self)->usize
    {
         B_query.size() + 1
    }

    pub fn size_in_bits(&self)->usize
    {
         G1_size() * libff::G1<ppT>::size_in_bits() +
               G2_size() * libff::G2<ppT>::size_in_bits()
    }

    fn print_size(&self) 
    {
        libff::print_indent(); printf!("* G1 elements in PK: %zu\n", self.G1_size());
        libff::print_indent(); printf!("* G2 elements in PK: %zu\n", self.G2_size());
        libff::print_indent(); printf!("* PK size in bits: %zu\n", self.size_in_bits());
    }

   
};


/******************************* Verification key ****************************/

/**
 * A verification key for the R1CS SEppzkSNARK.
 */

struct r1cs_se_ppzksnark_verification_key<ppT> {

    // H
H:    libff::G2<ppT>,

    // G^{alpha}
G_alpha:    libff::G1<ppT>,

    // H^{beta}
H_beta:    libff::G2<ppT>,

    // G^{gamma}
G_gamma:    libff::G1<ppT>,

    // H^{gamma}
H_gamma:    libff::G2<ppT>,

    // G^{gamma * A_i(t) + (alpha + beta) * A_i(t)}
    // for 0 <= i <= sap.num_inputs()
query:    libff::G1_vector<ppT>,
}
impl<ppT> r1cs_se_ppzksnark_verification_key<ppT> {
    // r1cs_se_ppzksnark_verification_key() = default;
    r1cs_se_ppzksnark_verification_key(
        H:&libff::G2<ppT>
        G_alpha:&libff::G1<ppT>
        H_beta:&libff::G2<ppT>
        G_gamma:&libff::G1<ppT>
        H_gamma:&libff::G2<ppT>
query:        libff::G1_vector<ppT>,
        ) :
        H(H),
        G_alpha(G_alpha),
        H_beta(H_beta),
        G_gamma(G_gamma),
        H_gamma(H_gamma),
        query(std::move(query))
    {};

    pub fn G1_size(&self)->usize
    {
        return 2 + query.size();
    }

    pub fn G2_size(&self)->usize
    {
        return 3;
    }

    pub fn size_in_bits(&self)->usize
    {
        return (G1_size() * libff::G1<ppT>::size_in_bits() +
                G2_size() * libff::G2<ppT>::size_in_bits());
    }

     fn print_size(&self) 
    {
        libff::print_indent(); printf!("* G1 elements in VK: %zu\n",
            self.G1_size());
        libff::print_indent(); printf!("* G2 elements in VK: %zu\n",
            self.G2_size());
        libff::print_indent(); printf!("* VK size in bits: %zu\n",
            self.size_in_bits());
    }

    
};

/************************ Processed verification key *************************/

/**
 * A processed verification key for the R1CS SEppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */

struct r1cs_se_ppzksnark_processed_verification_key<ppT> {

G_alpha:    libff::G1<ppT>,
H_beta:    libff::G2<ppT>,
G_alpha_H_beta_ml:    libff::Fqk<ppT>,
G_gamma_pc:    libff::G1_precomp<ppT>,
H_gamma_pc:    libff::G2_precomp<ppT>,
H_pc:    libff::G2_precomp<ppT>,

query:    libff::G1_vector<ppT>,

};

/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS SEppzkSNARK, which consists of a proving key and a verification key.
 */

struct r1cs_se_ppzksnark_keypair<ppT> {
pk:    r1cs_se_ppzksnark_proving_key<ppT>,
vk:    r1cs_se_ppzksnark_verification_key<ppT>,
}
impl r1cs_se_ppzksnark_keypair<ppT> {
    // r1cs_se_ppzksnark_keypair() = default;
    // r1cs_se_ppzksnark_keypair(other:&r1cs_se_ppzksnark_keypair<ppT>) = default;
    pub fn new(
pk:        r1cs_se_ppzksnark_proving_key<ppT>,
vk:                              r1cs_se_ppzksnark_verification_key<ppT>,
) :
        pk(std::move(pk)),
        vk(std::move(vk))
    {}

    // r1cs_se_ppzksnark_keypair(r1cs_se_ppzksnark_keypair<ppT> &&other) = default;
};


/*********************************** Proof ***********************************/


/**
 * A proof for the R1CS SEppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */

struct r1cs_se_ppzksnark_proof<ppT> {
A:    libff::G1<ppT>,
B:    libff::G2<ppT>,
C:    libff::G1<ppT>,
}

 impl<ppT>  r1cs_se_ppzksnark_proof<ppT> {
    // r1cs_se_ppzksnark_proof()
    // {}
    r1cs_se_ppzksnark_proof(
A:        libff::G1<ppT>,
B:        libff::G2<ppT>,
C:        libff::G1<ppT>,
) :
        A(std::move(A)),
        B(std::move(B)),
        C(std::move(C))
    {};

    pub fn G1_size(&self)->usize
    {
        return 2;
    }

    pub fn G2_size(&self)->usize
    {
        return 1;
    }

    pub fn size_in_bits(&self)->usize
    {
        return G1_size() * libff::G1<ppT>::size_in_bits() +
               G2_size() * libff::G2<ppT>::size_in_bits();
    }

     fn print_size(&self) 
    {
        libff::print_indent(); printf!("* G1 elements in proof: %zu\n",
            self.G1_size());
        libff::print_indent(); printf!("* G2 elements in proof: %zu\n",
            self.G2_size());
        libff::print_indent(); printf!("* Proof size in bits: %zu\n",
            self.size_in_bits());
    }

     is_well_formed(&self) ->bool
    {
        return (A.is_well_formed() && B.is_well_formed() &&
                C.is_well_formed());
    }

   
};


/***************************** Main algorithms *******************************/

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
pub fn 
r1cs_se_ppzksnark_prover<ppT>(pk:&r1cs_se_ppzksnark_proving_key<ppT>,
                                                primary_input:&r1cs_se_ppzksnark_primary_input<ppT>,
                                                &auxiliary_input:r1cs_se_ppzksnark_auxiliary_input<ppT>)->r1cs_se_ppzksnark_proof<ppT> 
{
    libff::enter_block("Call to r1cs_se_ppzksnark_prover");

// #ifdef DEBUG
//     assert(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
// #endif

    let  d1 = libff::Fr<ppT>::random_element(),
     let    d2 = libff::Fr<ppT>::random_element();

    libff::enter_block("Compute the polynomial H");
    let  sap_wit = r1cs_to_sap_witness_map(
        pk.constraint_system, primary_input, auxiliary_input, d1, d2);
    libff::leave_block("Compute the polynomial H");

// #ifdef DEBUG
//     libff::Fr<ppT>::random_element(:libff::Fr<ppT> t =);
//     sap_instance_evaluation<libff::Fr<ppT> > sap_inst = r1cs_to_sap_instance_map_with_evaluation(pk.constraint_system, t);
//     assert(sap_inst.is_satisfied(sap_wit));
// #endif

// #ifdef DEBUG
//     assert(pk.A_query.size() == sap_wit.num_variables() + 1);
//     assert(pk.B_query.size() == sap_wit.num_variables() + 1);
//     assert(pk.C_query_1.size() == sap_wit.num_variables() - sap_wit.num_inputs());
//     assert(pk.C_query_2.size() == sap_wit.num_variables() + 1);
//     assert(pk.G_gamma2_Z_t.size() >= sap_wit.degree() - 1);
// #endif

// #ifdef MULTICORE
//     override:size_t chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
// #else
//     const size_t chunks = 1;
// #endif

    let  r = libff::Fr<ppT>::random_element();

    libff::enter_block("Compute the proof");

    libff::enter_block("Compute answer to A-query", false);
    /**
     * compute A = G^{gamma * (\sum_{i=0}^m input_i * A_i(t) + r * Z(t))}
     *           = \prod_{i=0}^m (G^{gamma * A_i(t)})^{input_i)
     *             * (G^{gamma * Z(t)})^r
     *           = \prod_{i=0}^m A_query[i]^{input_i} * G_gamma_Z^r
     */
    let  A = r * pk.G_gamma_Z +
        pk.A_query[0] + // i = 0 is a special case because input_i = 1
        sap_wit.d1 * pk.G_gamma_Z + // ZK-patch
        libff::multi_exp<libff::G1<ppT>,
                         libff::Fr<ppT>,
                         libff::multi_exp_method_BDLO12>(
            pk.A_query.begin() + 1,
            pk.A_query.end(),
            sap_wit.coefficients_for_ACs.begin(),
            sap_wit.coefficients_for_ACs.end(),
            chunks);

    libff::leave_block("Compute answer to A-query", false);

    libff::enter_block("Compute answer to B-query", false);
    /**
     * compute B exactly as A, except with H as the base
     */
    let  B = r * pk.H_gamma_Z +
        pk.B_query[0] + // i = 0 is a special case because input_i = 1
        sap_wit.d1 * pk.H_gamma_Z + // ZK-patch
        libff::multi_exp<libff::G2<ppT>,
                         libff::Fr<ppT>,
                         libff::multi_exp_method_BDLO12>(
            pk.B_query.begin() + 1,
            pk.B_query.end(),
            sap_wit.coefficients_for_ACs.begin(),
            sap_wit.coefficients_for_ACs.end(),
            chunks);
    libff::leave_block("Compute answer to B-query", false);

    libff::enter_block("Compute answer to C-query", false);
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
    let  C = libff::multi_exp<libff::G1<ppT>,
                                        libff::Fr<ppT>,
                                        libff::multi_exp_method_BDLO12>(
            pk.C_query_1.begin(),
            pk.C_query_1.end(),
            sap_wit.coefficients_for_ACs.begin() + sap_wit.num_inputs(),
            sap_wit.coefficients_for_ACs.end(),
            chunks) +
        (r * r) * pk.G_gamma2_Z2 +
        r * pk.G_ab_gamma_Z +
        sap_wit.d1 * pk.G_ab_gamma_Z + // ZK-patch
        r * pk.C_query_2[0] + // i = 0 is a special case for C_query_2
        (r + r) * sap_wit.d1 * pk.G_gamma2_Z2 + // ZK-patch for C_query_2
        r * libff::multi_exp<libff::G1<ppT>,
                             libff::Fr<ppT>,
                             libff::multi_exp_method_BDLO12>(
            pk.C_query_2.begin() + 1,
            pk.C_query_2.end(),
            sap_wit.coefficients_for_ACs.begin(),
            sap_wit.coefficients_for_ACs.end(),
            chunks) +
        sap_wit.d2 * pk.G_gamma2_Z_t[0] + // ZK-patch
        libff::multi_exp<libff::G1<ppT>,
                          libff::Fr<ppT>,
                          libff::multi_exp_method_BDLO12>(
            pk.G_gamma2_Z_t.begin(),
            pk.G_gamma2_Z_t.end(),
            sap_wit.coefficients_for_H.begin(),
            sap_wit.coefficients_for_H.end(),
            chunks);
    libff::leave_block("Compute answer to C-query", false);

    libff::leave_block("Compute the proof");

    libff::leave_block("Call to r1cs_se_ppzksnark_prover");

    r1cs_se_ppzksnark_proof<ppT> proof = r1cs_se_ppzksnark_proof<ppT>(
        std::move(A), std::move(B), std::move(C));
    proof.print_size();

    return proof;
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
pub fn r1cs_se_ppzksnark_verifier_weak_IC<ppT>(vk:&r1cs_se_ppzksnark_verification_key<ppT>,
                                        primary_input:&r1cs_se_ppzksnark_primary_input<ppT>,
                                        &proof:r1cs_se_ppzksnark_proof<ppT>)->bool
{
    libff::enter_block("Call to r1cs_se_ppzksnark_verifier_weak_IC");
    let  pvk = r1cs_se_ppzksnark_verifier_process_vk<ppT>(vk);
    let  result = r1cs_se_ppzksnark_online_verifier_weak_IC<ppT>(pvk, primary_input, proof);
    libff::leave_block("Call to r1cs_se_ppzksnark_verifier_weak_IC");
    return result;
}

/**
 * A verifier algorithm for the R1CS SEppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */
pub fn  r1cs_se_ppzksnark_verifier_strong_IC<ppT>(vk:&r1cs_se_ppzksnark_verification_key<ppT>,
                                          primary_input:&r1cs_se_ppzksnark_primary_input<ppT>,
                                          proof:&r1cs_se_ppzksnark_proof<ppT>)->bool
{
    libff::enter_block("Call to r1cs_se_ppzksnark_verifier_strong_IC");
    let  pvk = r1cs_se_ppzksnark_verifier_process_vk<ppT>(vk);
    let  result = r1cs_se_ppzksnark_online_verifier_strong_IC<ppT>(pvk, primary_input, proof);
    libff::leave_block("Call to r1cs_se_ppzksnark_verifier_strong_IC");
    return result;
}

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
pub fn  r1cs_se_ppzksnark_verifier_process_vk<ppT> (vk:&r1cs_se_ppzksnark_verification_key<ppT>)->r1cs_se_ppzksnark_processed_verification_key<ppT>
{
    libff::enter_block("Call to r1cs_se_ppzksnark_verifier_process_vk");

    let  G_alpha_pc = ppT::precompute_G1(vk.G_alpha);
   let  H_beta_pc = ppT::precompute_G2(vk.H_beta);

    let  pvk;
    pvk.G_alpha = vk.G_alpha;
    pvk.H_beta = vk.H_beta;
    pvk.G_alpha_H_beta_ml = ppT::miller_loop(G_alpha_pc, H_beta_pc);
    pvk.G_gamma_pc = ppT::precompute_G1(vk.G_gamma);
    pvk.H_gamma_pc = ppT::precompute_G2(vk.H_gamma);
    pvk.H_pc = ppT::precompute_G2(vk.H);

    pvk.query = vk.query;

    libff::leave_block("Call to r1cs_se_ppzksnark_verifier_process_vk");

    return pvk;
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */
fn  r1cs_se_ppzksnark_online_verifier_weak_IC<ppT>(pvk:&r1cs_se_ppzksnark_processed_verification_key<ppT>,
                                               primary_input:&r1cs_se_ppzksnark_primary_input<ppT>,
                                               &proof:r1cs_se_ppzksnark_proof<ppT>)->bool
{
    libff::enter_block("Call to r1cs_se_ppzksnark_online_verifier_weak_IC");

    let  result = true;

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

    libff::enter_block("Pairing computations");

// #ifdef MULTICORE
//     override:size_t chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
// #else
//     const size_t chunks = 1;
// #endif

    libff::enter_block("Check first test");
    /**
     * e(A*G^{alpha}, B*H^{beta}) = e(G^{alpha}, H^{beta}) * e(G^{psi}, H^{gamma})
     *                              * e(C, H)
     * where psi = \sum_{i=0}^l input_i pvk.query[i]
     */
    let  G_psi = pvk.query[0] +
        libff::multi_exp<libff::G1<ppT>,
                         libff::Fr<ppT>,
                         libff::multi_exp_method_bos_coster>(
            pvk.query.begin() + 1, pvk.query.end(),
            primary_input.begin(), primary_input.end(),
            chunks);

    let  test1_l = ppT::miller_loop(ppT::precompute_G1(proof.A + pvk.G_alpha),
                                               ppT::precompute_G2(proof.B + pvk.H_beta)),
                    test1_r1 = pvk.G_alpha_H_beta_ml,
                    test1_r2 = ppT::miller_loop(ppT::precompute_G1(G_psi),
                                                pvk.H_gamma_pc),
                    test1_r3 = ppT::miller_loop(ppT::precompute_G1(proof.C),
                                                pvk.H_pc);
    let  test1 = ppT::final_exponentiation(
        test1_l.unitary_inverse() * test1_r1 * test1_r2 * test1_r3);

    if test1 != libff::GT<ppT>::one()
    {
        if !libff::inhibit_profiling_info
        {
            libff::print_indent(); printf!("First test failed.\n");
        }
        result = false;
    }
    libff::leave_block("Check first test");

    libff::enter_block("Check second test");
    /**
     * e(A, H^{gamma}) = e(G^{gamma}, B)
     */
    let  test2_l = ppT::miller_loop(ppT::precompute_G1(proof.A),
                                               pvk.H_gamma_pc),
                    test2_r = ppT::miller_loop(pvk.G_gamma_pc,
                                               ppT::precompute_G2(proof.B));
    let  test2 = ppT::final_exponentiation(
        test2_l * test2_r.unitary_inverse());

    if test2 != libff::GT<ppT>::one()
    {
        if !libff::inhibit_profiling_info
        {
            libff::print_indent(); printf!("Second test failed.\n");
        }
        result = false;
    }
    libff::leave_block("Check second test");
    libff::leave_block("Pairing computations");
    libff::leave_block("Call to r1cs_se_ppzksnark_online_verifier_weak_IC");

    return result;
}

/**
 * A verifier algorithm for the R1CS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */
pub fn  r1cs_se_ppzksnark_online_verifier_strong_IC<ppT>(pvk:&r1cs_se_ppzksnark_processed_verification_key<ppT>,
                                                 primary_input:&r1cs_se_ppzksnark_primary_input<ppT>,
                                                 &proof:r1cs_se_ppzksnark_proof<ppT>)->bool
{
    libff::enter_block("Call to r1cs_se_ppzksnark_online_verifier_strong_IC");
    let  result = true;

    if pvk.query.size() != primary_input.size() + 1
    {
        libff::print_indent();
        printf!("Input length differs from expected (got %zu, expected %zu).\n",
            primary_input.size(), pvk.query.size());
        result = false;
    }
    else
    {
        result = r1cs_se_ppzksnark_online_verifier_weak_IC(pvk, primary_input, proof);
    }

    libff::leave_block("Call to r1cs_se_ppzksnark_online_verifier_strong_IC");
    return result;
}

} // libsnark

// use  <libsnark/zk_proof_systems/ppzksnark/r1cs_se_ppzksnark/r1cs_se_ppzksnark.tcc>

// #endif // R1CS_SE_PPZKSNARK_HPP_




/** @file
*****************************************************************************

Implementation of interfaces for a SEppzkSNARK for R1CS.

See r1cs_se_ppzksnark.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// #ifndef R1CS_SE_PPZKSNARK_TCC_
// #define R1CS_SE_PPZKSNARK_TCC_

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
use  <libsnark/reductions/r1cs_to_sap/r1cs_to_sap.hpp>

// namespace libsnark {

impl<ppT> PartialEq for r1cs_se_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A_query == other.A_query &&
            self.B_query == other.B_query &&
            self.C_query_1 == other.C_query_1 &&
            self.C_query_2 == other.C_query_2 &&
            self.G_gamma_Z == other.G_gamma_Z &&
            self.H_gamma_Z == other.H_gamma_Z &&
            self.G_ab_gamma_Z == other.G_ab_gamma_Z &&
            self.G_gamma2_Z2 == other.G_gamma2_Z2 &&
            self.G_gamma2_Z_t == other.G_gamma2_Z_t &&
            self.constraint_system == other.constraint_system
    }
}

use std::fmt;
impl<ppT> fmt::Display for r1cs_se_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
pk.A_query,
pk.B_query,
pk.C_query_1,
pk.C_query_2,
pk.G_gamma_Z,
pk.H_gamma_Z,
pk.G_ab_gamma_Z,
pk.G_gamma2_Z2,
pk.G_gamma2_Z_t,
pk.constraint_system,)
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


impl<ppT> PartialEq for r1cs_se_ppzksnark_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.H == other.H &&
            self.G_alpha == other.G_alpha &&
            self.H_beta == other.H_beta &&
            self.G_gamma == other.G_gamma &&
            self.H_gamma == other.H_gamma &&
            self.query == other.query
    }
}


impl<ppT> fmt::Display for r1cs_se_ppzksnark_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",vk.H ,
vk.G_alpha ,
vk.H_beta ,
vk.G_gamma ,
vk.H_gamma ,
vk.query ,)
    }
}


// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_verification_key<ppT> &vk)
// {
//     in >> vk.H;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.G_alpha;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.H_beta;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.G_gamma;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.H_gamma;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> vk.query;
//     libff::consume_OUTPUT_NEWLINE(in);

//     return in;
// }



impl<ppT> PartialEq for r1cs_se_ppzksnark_processed_verification_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.G_alpha == other.G_alpha &&
            self.H_beta == other.H_beta &&
            self.G_alpha_H_beta_ml == other.G_alpha_H_beta_ml &&
            self.G_gamma_pc == other.G_gamma_pc &&
            self.H_gamma_pc == other.H_gamma_pc &&
            self.H_pc == other.H_pc &&
            self.query == other.query
    }
}



impl<ppT> fmt::Display for r1cs_se_ppzksnark_processed_verification_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}", pvk.G_alpha ,
pvk.H_beta ,
pvk.G_alpha_H_beta_ml ,
pvk.G_gamma_pc ,
pvk.H_gamma_pc ,
pvk.H_pc ,
pvk.query ,
vk.query ,
        )
    }
}

// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.G_alpha;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.H_beta;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.G_alpha_H_beta_ml;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.G_gamma_pc;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.H_gamma_pc;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.H_pc;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> pvk.query;
//     libff::consume_OUTPUT_NEWLINE(in);
//     return in;
// }



impl<ppT> PartialEq for r1cs_se_ppzksnark_proof<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A &&
            self.B == other.B &&
            self.C == other.C
    }
}





impl<ppT> fmt::Display for r1cs_se_ppzksnark_proof<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {


        write!(f, "{}{}{}", proof.A ,
proof.B ,
proof.C ,)
    }
}


// pub fn 
// std::istream& operator>>(std::istream &in, r1cs_se_ppzksnark_proof<ppT> &proof)
// {
//     in >> proof.A;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> proof.B;
//     libff::consume_OUTPUT_NEWLINE(in);
//     in >> proof.C;
//     libff::consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT> r1cs_se_ppzksnark_verification_key<ppT>{
pub fn  dummy_verification_key(&self, input_size:size_t )->r1cs_se_ppzksnark_verification_key<ppT>
{
    let mut  result=r1cs_se_ppzksnark_verification_key<ppT>::new();
    result.H = libff::Fr<ppT>::random_element() * libff::G2<ppT>::one();
    result.G_alpha = libff::Fr<ppT>::random_element() * libff::G1<ppT>::one();
    result.H_beta = libff::Fr<ppT>::random_element() * libff::G2<ppT>::one();
    result.G_gamma = libff::Fr<ppT>::random_element() * libff::G1<ppT>::one();
    result.H_gamma = libff::Fr<ppT>::random_element() * libff::G2<ppT>::one();

    let  mut v=libff::G1_vector<ppT>::new();
    for i in 0..=input_size
    {
        v.emplace_back(libff::Fr<ppT>::random_element() * libff::G1<ppT>::one());
    }
    result.query = std::move(v);

    return result;
}
}
pub fn  r1cs_se_ppzksnark_generator<ppT>(cs:r1cs_se_ppzksnark_constraint_system<ppT>,)->r1cs_se_ppzksnark_keypair<ppT> 
{
    libff::enter_block("Call to r1cs_se_ppzksnark_generator");

    /**
     * draw random element t at which the SAP is evaluated.
     * it should be the case that Z(t) != 0
     */
    let  domain =
        r1cs_to_sap_get_domain(cs);
    let mut t;
    do {
        t = libff::Fr<ppT>::random_element();
    } while (domain->compute_vanishing_polynomial(t).is_zero());

    let  sap_inst = r1cs_to_sap_instance_map_with_evaluation(cs, t);

    libff::print_indent(); printf!("* SAP number of variables: %zu\n", sap_inst.num_variables());
    libff::print_indent(); printf!("* SAP pre degree: %zu\n", cs.constraints.size());
    libff::print_indent(); printf!("* SAP degree: %zu\n", sap_inst.degree());
    libff::print_indent(); printf!("* SAP number of input variables: %zu\n", sap_inst.num_inputs());

    libff::enter_block("Compute query densities");
    let  mut non_zero_At = 0;
    for  i in 0.. =sap_inst.num_variables()
    {
        if !sap_inst.At[i].is_zero()
        {
            non_zero_At+=1;
        }
    }
    libff::leave_block("Compute query densities");

    let  At = std::move(sap_inst.At);
    let Ct = std::move(sap_inst.Ct);
    let Ht = std::move(sap_inst.Ht);
    /**
     * sap_inst.{A,C,H}t are now in an unspecified state,
     * but we do not use them below
     */

    let alpha = libff::Fr<ppT>::random_element(),
     let   beta = libff::Fr<ppT>::random_element(),
     let   gamma = libff::Fr<ppT>::random_element();
   let G = libff::G1<ppT>::random_element();
   let H = libff::G2<ppT>::random_element();

    libff::enter_block("Generating G multiexp table");
    let G_exp_count = sap_inst.num_inputs() + 1 // verifier_query
                         + non_zero_At // A_query
                         + sap_inst.degree() + 1 // G_gamma2_Z_t
                         // C_query_1
                         + sap_inst.num_variables() - sap_inst.num_inputs()
                         + sap_inst.num_variables() + 1, // C_query_2
           G_window = libff::get_exp_window_size<libff::G1<ppT> >(G_exp_count);
    libff::print_indent(); printf!("* G window: %zu\n", G_window);
    let G_table = get_window_table(
        libff::Fr<ppT>::size_in_bits(), G_window, G);
    libff::leave_block("Generating G multiexp table");

    libff::enter_block("Generating H_gamma multiexp table");
    letH_gamma = gamma * H;
    let H_gamma_exp_count = non_zero_At, // B_query
     let      H_gamma_window = libff::get_exp_window_size<libff::G2<ppT> >(H_gamma_exp_count);
    libff::print_indent(); printf!("* H_gamma window: %zu\n", H_gamma_window);
    letH_gamma_table = get_window_table(
        libff::Fr<ppT>::size_in_bits(), H_gamma_window, H_gamma);
    libff::leave_block("Generating H_gamma multiexp table");

    libff::enter_block("Generate R1CS verification key");
    let G_alpha = alpha * G;
    let H_beta = beta * H;

   let mut  tmp_exponents=libff::Fr_vector<ppT>::new();
    tmp_exponents.reserve(sap_inst.num_inputs() + 1);
    for  i in 0..= sap_inst.num_inputs()
    {
        tmp_exponents.emplace_back(gamma * Ct[i] + (alpha + beta) * At[i]);
    }
    let  verifier_query = libff::batch_exp<libff::G1<ppT>,
                                                            libff::Fr<ppT> >(
        libff::Fr<ppT>::size_in_bits(),
        G_window,
        G_table,
        tmp_exponents);
    tmp_exponents.clear();

    libff::leave_block("Generate R1CS verification key");

    libff::enter_block("Generate R1CS proving key");

    libff::enter_block("Compute the A-query", false);
    tmp_exponents.reserve(sap_inst.num_variables() + 1);
    for  i in 0.. At.size()
    {
        tmp_exponents.emplace_back(gamma * At[i]);
    }

    libff::G1_vector<ppT> A_query = libff::batch_exp<libff::G1<ppT>,
                                                     libff::Fr<ppT> >(
        libff::Fr<ppT>::size_in_bits(),
        G_window,
        G_table,
        tmp_exponents);
    tmp_exponents.clear();
// #ifdef USE_MIXED_ADDITION
//     libff::batch_to_special<libff::G1<ppT> >(A_query);
// #endif
    libff::leave_block("Compute the A-query", false);

    libff::enter_block("Compute the B-query", false);
    let  B_query = libff::batch_exp<libff::G2<ppT>,
                                                     libff::Fr<ppT> >(
        libff::Fr<ppT>::size_in_bits(),
        H_gamma_window,
        H_gamma_table,
        At);
// #ifdef USE_MIXED_ADDITION
//     libff::batch_to_special<libff::G2<ppT> >(B_query);
// #endif
    libff::leave_block("Compute the B-query", false);

    libff::enter_block("Compute the G_gamma-query", false);
    libff::G1<ppT> G_gamma = gamma * G;
    libff::G1<ppT> G_gamma_Z = sap_inst.Zt * G_gamma;
    libff::G2<ppT> H_gamma_Z = sap_inst.Zt * H_gamma;
    libff::G1<ppT> G_ab_gamma_Z = (alpha + beta) * G_gamma_Z;
    libff::G1<ppT> G_gamma2_Z2 = (sap_inst.Zt * gamma) * G_gamma_Z;

    tmp_exponents.reserve(sap_inst.degree() + 1);

    /* Compute the vector G_gamma2_Z_t := Z(t) * t^i * gamma^2 * G */
    let gamma2_Z_t = sap_inst.Zt * gamma.squared();
    for i in 0..sap_inst.degree() + 1
    {
        tmp_exponents.emplace_back(gamma2_Z_t);
        gamma2_Z_t *= t;
    }
    libff::G1_vector<ppT> G_gamma2_Z_t = libff::batch_exp<libff::G1<ppT>,
                                                          libff::Fr<ppT> >(
        libff::Fr<ppT>::size_in_bits(),
        G_window,
        G_table,
        tmp_exponents);
    tmp_exponents.clear();
// #ifdef USE_MIXED_ADDITION
//     libff::batch_to_special<libff::G1<ppT> >(G_gamma2_Z_t);
// #endif
    libff::leave_block("Compute the G_gamma-query", false);

    libff::enter_block("Compute the C_1-query", false);
    tmp_exponents.reserve(sap_inst.num_variables() - sap_inst.num_inputs());
    for i in sap_inst..=sap_inst.num_variables()
    {
        tmp_exponents.emplace_back(gamma *
            (gamma * Ct[i] + (alpha + beta) * At[i]));
    }
    let C_query_1 = libff::batch_exp<libff::G1<ppT>,
                                                       libff::Fr<ppT> >(
        libff::Fr<ppT>::size_in_bits(),
        G_window,
        G_table,
        tmp_exponents);
    tmp_exponents.clear();
// #ifdef USE_MIXED_ADDITION
//     libff::batch_to_special<libff::G1<ppT> >(C_query_1);
// #endif
    libff::leave_block("Compute the C_1-query", false);

    libff::enter_block("Compute the C_2-query", false);
    tmp_exponents.reserve(sap_inst.num_variables() + 1);
    let mut double_gamma2_Z = gamma * gamma * sap_inst.Zt;
    double_gamma2_Z = double_gamma2_Z + double_gamma2_Z;
    for i in 0..=sap_inst.num_variables()
    {
        tmp_exponents.emplace_back(double_gamma2_Z * At[i]);
    }
    let  C_query_2 = libff::batch_exp<libff::G1<ppT>,
                                                       libff::Fr<ppT> >(
        libff::Fr<ppT>::size_in_bits(),
        G_window,
        G_table,
        tmp_exponents);
    tmp_exponents.clear();
// #ifdef USE_MIXED_ADDITION
//     libff::batch_to_special<libff::G1<ppT> >(C_query_2);
// #endif
    libff::leave_block("Compute the C_2-query", false);

    libff::leave_block("Generate R1CS proving key");

    libff::leave_block("Call to r1cs_se_ppzksnark_generator");

    let  vk =
        r1cs_se_ppzksnark_verification_key<ppT>(H, G_alpha, H_beta, G_gamma,
            H_gamma, std::move(verifier_query));

   let  cs_copy=cs.clone();

   let  pk = r1cs_se_ppzksnark_proving_key<ppT>(
        std::move(A_query), std::move(B_query), std::move(C_query_1),
        std::move(C_query_2), G_gamma_Z, H_gamma_Z, G_ab_gamma_Z, G_gamma2_Z2,
        std::move(G_gamma2_Z_t), std::move(cs_copy));

    pk.print_size();
    vk.print_size();

    return r1cs_se_ppzksnark_keypair<ppT>(std::move(pk), std::move(vk));
}






// } // libsnark
// #endif // R1CS_SE_PPZKSNARK_TCC_
