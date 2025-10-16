/** @file
 *****************************************************************************

 Declaration of interfaces for a ppzkSNARK for USCS.

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

 The implementation instantiates the protocol of \[DFGK14], by following
 extending, and optimizing the approach described in \[BCTV14].


 Acronyms:

 - "ppzkSNARK" = "Pre-Processing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
 - "USCS" = "Unitary-Square Constraint Systems"

 References:

 \[BCTV14]:
 "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
 Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
 USENIX Security 2014,
 <http://eprint.iacr.org/2013/879>

 \[DFGK14]:
 "Square Span Programs with Applications to Succinct NIZK Arguments"
 George Danezis, Cedric Fournet, Jens Groth, Markulf Kohlweiss,
 ASIACRYPT 2014,
 <http://eprint.iacr.org/2014/718>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_PPZKSNARK_HPP_
// #define USCS_PPZKSNARK_HPP_

use  <memory>

use ffec::algebra::curves::public_params;

use crate::common::data_structures::accumulation_vector;
use crate::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems/uscs/uscs;
use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/uscs_ppzksnark_params;



/******************************** Proving key ********************************/

template<typename ppT>
class uscs_ppzksnark_proving_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_proving_key<ppT> &pk);

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the USCS ppzkSNARK.
 */
template<typename ppT>
class uscs_ppzksnark_proving_key {
public:
    ffec::G1_vector<ppT> V_g1_query;
    ffec::G1_vector<ppT> alpha_V_g1_query;
    ffec::G1_vector<ppT> H_g1_query;
    ffec::G2_vector<ppT> V_g2_query;

    uscs_ppzksnark_constraint_system<ppT> constraint_system;

    uscs_ppzksnark_proving_key() {};
    uscs_ppzksnark_proving_key<ppT>& operator=(const uscs_ppzksnark_proving_key<ppT> &other) = default;
    uscs_ppzksnark_proving_key(const uscs_ppzksnark_proving_key<ppT> &other) = default;
    uscs_ppzksnark_proving_key(uscs_ppzksnark_proving_key<ppT> &&other) = default;
    uscs_ppzksnark_proving_key(ffec::G1_vector<ppT> &&V_g1_query,
                               ffec::G1_vector<ppT> &&alpha_V_g1_query,
                               ffec::G1_vector<ppT> &&H_g1_query,
                               ffec::G2_vector<ppT> &&V_g2_query,
                               uscs_ppzksnark_constraint_system<ppT> &&constraint_system) :
        V_g1_query((V_g1_query)),
        alpha_V_g1_query((alpha_V_g1_query)),
        H_g1_query((H_g1_query)),
        V_g2_query((V_g2_query)),
        constraint_system((constraint_system))
    {};

    size_t G1_size() const
    {
        return V_g1_query.size() + alpha_V_g1_query.size() + H_g1_query.size();
    }

    size_t G2_size() const
    {
        return V_g2_query.size();
    }

    size_t G1_sparse_size() const
    {
        return G1_size();
    }

    size_t G2_sparse_size() const
    {
        return G2_size();
    }

    size_t size_in_bits() const
    {
        return ffec::G1<ppT>::size_in_bits() * G1_size() + ffec::G2<ppT>::size_in_bits() * G2_size();
    }

    void print_size() const
    {
        ffec::print_indent(); print!("* G1 elements in PK: {}\n", self.G1_size());
        ffec::print_indent(); print!("* Non-zero G1 elements in PK: {}\n", self.G1_sparse_size());
        ffec::print_indent(); print!("* G2 elements in PK: {}\n", self.G2_size());
        ffec::print_indent(); print!("* Non-zero G2 elements in PK: {}\n", self.G2_sparse_size());
        ffec::print_indent(); print!("* PK size in bits: {}\n", self.size_in_bits());
    }

    bool operator==(const uscs_ppzksnark_proving_key<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const uscs_ppzksnark_proving_key<ppT> &pk);
    friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_proving_key<ppT> &pk);
};


/******************************* Verification key ****************************/

template<typename ppT>
class uscs_ppzksnark_verification_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_verification_key<ppT> &vk);

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_verification_key<ppT> &vk);

/**
 * A verification key for the USCS ppzkSNARK.
 */
template<typename ppT>
class uscs_ppzksnark_verification_key {
public:
    ffec::G2<ppT> tilde_g2;
    ffec::G2<ppT> alpha_tilde_g2;
    ffec::G2<ppT> Z_g2;

    accumulation_vector<ffec::G1<ppT> > encoded_IC_query;

    uscs_ppzksnark_verification_key() = default;
    uscs_ppzksnark_verification_key(const ffec::G2<ppT> &tilde_g2,
                                    const ffec::G2<ppT> &alpha_tilde_g2,
                                    const ffec::G2<ppT> &Z_g2,
                                    const accumulation_vector<ffec::G1<ppT> > &eIC) :
        tilde_g2(tilde_g2),
        alpha_tilde_g2(alpha_tilde_g2),
        Z_g2(Z_g2),
        encoded_IC_query(eIC)
    {};

    size_t G1_size() const
    {
        return encoded_IC_query.size();
    }

    size_t G2_size() const
    {
        return 3;
    }

    size_t size_in_bits() const
    {
        return encoded_IC_query.size_in_bits() + 3 * ffec::G2<ppT>::size_in_bits();
    }

    void print_size() const
    {
        ffec::print_indent(); print!("* G1 elements in VK: {}\n", self.G1_size());
        ffec::print_indent(); print!("* G2 elements in VK: {}\n", self.G2_size());
        ffec::print_indent(); print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    bool operator==(const uscs_ppzksnark_verification_key<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const uscs_ppzksnark_verification_key<ppT> &vk);
    friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_verification_key<ppT> &vk);

    static uscs_ppzksnark_verification_key<ppT> dummy_verification_key(const size_t input_size);
};


/************************ Processed verification key *************************/

template<typename ppT>
class uscs_ppzksnark_processed_verification_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_processed_verification_key<ppT> &pvk);

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_processed_verification_key<ppT> &pvk);

/**
 * A processed verification key for the USCS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
template<typename ppT>
class uscs_ppzksnark_processed_verification_key {
public:
    ffec::G1_precomp<ppT> pp_G1_one_precomp;
    ffec::G2_precomp<ppT> pp_G2_one_precomp;
    ffec::G2_precomp<ppT> vk_tilde_g2_precomp;
    ffec::G2_precomp<ppT> vk_alpha_tilde_g2_precomp;
    ffec::G2_precomp<ppT> vk_Z_g2_precomp;
    ffec::GT<ppT> pairing_of_g1_and_g2;

    accumulation_vector<ffec::G1<ppT> > encoded_IC_query;

    bool operator==(const uscs_ppzksnark_processed_verification_key &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const uscs_ppzksnark_processed_verification_key<ppT> &pvk);
    friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_processed_verification_key<ppT> &pvk);
};


/********************************** Key pair *********************************/

/**
 * A key pair for the USCS ppzkSNARK, which consists of a proving key and a verification key.
 */
template<typename ppT>
class uscs_ppzksnark_keypair {
public:
    uscs_ppzksnark_proving_key<ppT> pk;
    uscs_ppzksnark_verification_key<ppT> vk;

    uscs_ppzksnark_keypair() {};
    uscs_ppzksnark_keypair(uscs_ppzksnark_proving_key<ppT> &&pk,
                           uscs_ppzksnark_verification_key<ppT> &&vk) :
        pk((pk)),
        vk((vk))
    {}

    uscs_ppzksnark_keypair(uscs_ppzksnark_keypair<ppT> &&other) = default;
};


/*********************************** Proof ***********************************/

template<typename ppT>
class uscs_ppzksnark_proof;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_proof<ppT> &proof);

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_proof<ppT> &proof);

/**
 * A proof for the USCS ppzkSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
template<typename ppT>
class uscs_ppzksnark_proof {
public:
    ffec::G1<ppT> V_g1;
    ffec::G1<ppT> alpha_V_g1;
    ffec::G1<ppT> H_g1;
    ffec::G2<ppT> V_g2;

    uscs_ppzksnark_proof()
    {
        // invalid proof with valid curve points
        self.V_g1 = ffec::G1<ppT> ::one();
        self.alpha_V_g1 = ffec::G1<ppT> ::one();
        self.H_g1 = ffec::G1<ppT> ::one();
        self.V_g2 = ffec::G2<ppT> ::one();
    }
    uscs_ppzksnark_proof(ffec::G1<ppT> &&V_g1,
                         ffec::G1<ppT> &&alpha_V_g1,
                         ffec::G1<ppT> &&H_g1,
                         ffec::G2<ppT> &&V_g2) :
        V_g1((V_g1)),
        alpha_V_g1((alpha_V_g1)),
        H_g1((H_g1)),
        V_g2((V_g2))
    {};

    size_t G1_size() const
    {
        return 3;
    }

    size_t G2_size() const
    {
        return 1;
    }

    size_t size_in_bits() const
    {
        return G1_size() * ffec::G1<ppT>::size_in_bits() + G2_size() * ffec::G2<ppT>::size_in_bits();
    }

    void print_size() const
    {
        ffec::print_indent(); print!("* G1 elements in proof: {}\n", self.G1_size());
        ffec::print_indent(); print!("* G2 elements in proof: {}\n", self.G2_size());
        ffec::print_indent(); print!("* Proof size in bits: {}\n", self.size_in_bits());
    }

    bool is_well_formed() const
    {
        return (V_g1.is_well_formed() &&
                alpha_V_g1.is_well_formed() &&
                H_g1.is_well_formed() &&
                V_g2.is_well_formed());
    }

    bool operator==(const uscs_ppzksnark_proof<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const uscs_ppzksnark_proof<ppT> &proof);
    friend std::istream& operator>> <ppT>(std::istream &in, uscs_ppzksnark_proof<ppT> &proof);
};


/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the USCS ppzkSNARK.
 *
 * Given a USCS constraint system CS, this algorithm produces proving and verification keys for CS.
 */
template<typename ppT>
uscs_ppzksnark_keypair<ppT> uscs_ppzksnark_generator(const uscs_ppzksnark_constraint_system<ppT> &cs);

/**
 * A prover algorithm for the USCS ppzkSNARK.
 *
 * Given a USCS primary input X and a USCS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the USCS constraint system that was given as input to the generator algorithm.
 */
template<typename ppT>
uscs_ppzksnark_proof<ppT> uscs_ppzksnark_prover(const uscs_ppzksnark_proving_key<ppT> &pk,
                                                const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                                const uscs_ppzksnark_auxiliary_input<ppT> &auxiliary_input);

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
template<typename ppT>
bool uscs_ppzksnark_verifier_weak_IC(const uscs_ppzksnark_verification_key<ppT> &vk,
                                     const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                     const uscs_ppzksnark_proof<ppT> &proof);

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */
template<typename ppT>
bool uscs_ppzksnark_verifier_strong_IC(const uscs_ppzksnark_verification_key<ppT> &vk,
                                       const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                       const uscs_ppzksnark_proof<ppT> &proof);

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
template<typename ppT>
uscs_ppzksnark_processed_verification_key<ppT> uscs_ppzksnark_verifier_process_vk(const uscs_ppzksnark_verification_key<ppT> &vk);

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */
template<typename ppT>
bool uscs_ppzksnark_online_verifier_weak_IC(const uscs_ppzksnark_processed_verification_key<ppT> &pvk,
                                            const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                            const uscs_ppzksnark_proof<ppT> &proof);

/**
 * A verifier algorithm for the USCS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */
template<typename ppT>
bool uscs_ppzksnark_online_verifier_strong_IC(const uscs_ppzksnark_processed_verification_key<ppT> &pvk,
                                              const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                              const uscs_ppzksnark_proof<ppT> &proof);



use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/uscs_ppzksnark;

//#endif // USCS_PPZKSNARK_HPP_
/** @file
 *****************************************************************************
 Implementation of interfaces for a ppzkSNARK for USCS.

 See uscs_ppzksnark.hpp .
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_PPZKSNARK_TCC_
// #define USCS_PPZKSNARK_TCC_

use  <algorithm>
use  <cassert>
use  <functional>
use  <iostream>
use  <sstream>

 use ffec::algebra::scalar_multiplication::multiexp;
use ffec::common::profiling;
use ffec::common::utils;

// #ifdef MULTICORE
use  <omp.h>
//#endif

use libsnark/reductions/uscs_to_ssp/uscs_to_ssp;
use crate::relations::arithmetic_programs/ssp/ssp;



template<typename ppT>
bool uscs_ppzksnark_proving_key<ppT>::operator==(const uscs_ppzksnark_proving_key<ppT> &other) const
{
    return (self.V_g1_query == other.V_g1_query &&
            self.alpha_V_g1_query == other.alpha_V_g1_query &&
            self.H_g1_query == other.H_g1_query &&
            self.V_g2_query == other.V_g2_query &&
            self.constraint_system == other.constraint_system);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_proving_key<ppT> &pk)
{
    out << pk.V_g1_query;
    out << pk.alpha_V_g1_query;
    out << pk.H_g1_query;
    out << pk.V_g2_query;
    out << pk.constraint_system;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_proving_key<ppT> &pk)
{
    in >> pk.V_g1_query;
    in >> pk.alpha_V_g1_query;
    in >> pk.H_g1_query;
    in >> pk.V_g2_query;
    in >> pk.constraint_system;

    return in;
}

template<typename ppT>
bool uscs_ppzksnark_verification_key<ppT>::operator==(const uscs_ppzksnark_verification_key<ppT> &other) const
{
    return (self.tilde_g2 == other.tilde_g2 &&
            self.alpha_tilde_g2 == other.alpha_tilde_g2 &&
            self.Z_g2 == other.Z_g2 &&
            self.encoded_IC_query == other.encoded_IC_query);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_verification_key<ppT> &vk)
{
    out << vk.tilde_g2 << OUTPUT_NEWLINE;
    out << vk.alpha_tilde_g2 << OUTPUT_NEWLINE;
    out << vk.Z_g2 << OUTPUT_NEWLINE;
    out << vk.encoded_IC_query << OUTPUT_NEWLINE;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_verification_key<ppT> &vk)
{
    in >> vk.tilde_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.alpha_tilde_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.Z_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.encoded_IC_query;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename ppT>
bool uscs_ppzksnark_processed_verification_key<ppT>::operator==(const uscs_ppzksnark_processed_verification_key<ppT> &other) const
{
    return (self.pp_G1_one_precomp == other.pp_G1_one_precomp &&
            self.pp_G2_one_precomp == other.pp_G2_one_precomp &&
            self.vk_tilde_g2_precomp == other.vk_tilde_g2_precomp &&
            self.vk_alpha_tilde_g2_precomp == other.vk_alpha_tilde_g2_precomp &&
            self.vk_Z_g2_precomp == other.vk_Z_g2_precomp &&
            self.pairing_of_g1_and_g2 == other.pairing_of_g1_and_g2 &&
            self.encoded_IC_query == other.encoded_IC_query);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_processed_verification_key<ppT> &pvk)
{
    out << pvk.pp_G1_one_precomp << OUTPUT_NEWLINE;
    out << pvk.pp_G2_one_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_tilde_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_alpha_tilde_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_Z_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.pairing_of_g1_and_g2 << OUTPUT_NEWLINE;
    out << pvk.encoded_IC_query << OUTPUT_NEWLINE;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_processed_verification_key<ppT> &pvk)
{
    in >> pvk.pp_G1_one_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.pp_G2_one_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_tilde_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_alpha_tilde_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_Z_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.pairing_of_g1_and_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.encoded_IC_query;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename ppT>
bool uscs_ppzksnark_proof<ppT>::operator==(const uscs_ppzksnark_proof<ppT> &other) const
{
    return (self.V_g1 == other.V_g1 &&
            self.alpha_V_g1 == other.alpha_V_g1 &&
            self.H_g1 == other.H_g1 &&
            self.V_g2 == other.V_g2);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const uscs_ppzksnark_proof<ppT> &proof)
{
    out << proof.V_g1 << OUTPUT_NEWLINE;
    out << proof.alpha_V_g1 << OUTPUT_NEWLINE;
    out << proof.H_g1 << OUTPUT_NEWLINE;
    out << proof.V_g2 << OUTPUT_NEWLINE;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, uscs_ppzksnark_proof<ppT> &proof)
{
    in >> proof.V_g1;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.alpha_V_g1;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.H_g1;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.V_g2;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename ppT>
uscs_ppzksnark_verification_key<ppT> uscs_ppzksnark_verification_key<ppT>::dummy_verification_key(const size_t input_size)
{
    uscs_ppzksnark_verification_key<ppT> result;
    result.tilde_g2       = ffec::Fr<ppT>::random_element() * ffec::G2<ppT>::one();
    result.alpha_tilde_g2 = ffec::Fr<ppT>::random_element() * ffec::G2<ppT>::one();
    result.Z_g2           = ffec::Fr<ppT>::random_element() * ffec::G2<ppT>::one();

    ffec::G1<ppT> base = ffec::Fr<ppT>::random_element() * ffec::G1<ppT>::one();
    ffec::G1_vector<ppT> v;
    for i in 0..input_size
    {
        v.push(ffec::Fr<ppT>::random_element() * ffec::G1<ppT>::one());
    }

    result.encoded_IC_query = accumulation_vector<ffec::G1<ppT> >(v);

    return result;
}

template <typename ppT>
uscs_ppzksnark_keypair<ppT> uscs_ppzksnark_generator(const uscs_ppzksnark_constraint_system<ppT> &cs)
{
    ffec::enter_block("Call to uscs_ppzksnark_generator");

    /* draw random element at which the SSP is evaluated */

    const  ffec::Fr<ppT> t = ffec::Fr<ppT>::random_element();

    /* perform USCS-to-SSP reduction */

    ssp_instance_evaluation<ffec::Fr<ppT> > ssp_inst = uscs_to_ssp_instance_map_with_evaluation(cs, t);

    ffec::print_indent(); print!("* SSP number of variables: {}\n", ssp_inst.num_variables());
    ffec::print_indent(); print!("* SSP pre degree: {}\n", cs.num_constraints());
    ffec::print_indent(); print!("* SSP degree: {}\n", ssp_inst.degree());
    ffec::print_indent(); print!("* SSP number of input variables: {}\n", ssp_inst.num_inputs());

    /* construct various tables of FieldT elements */

    ffec::Fr_vector<ppT> Vt_table = (ssp_inst.Vt); // ssp_inst.Vt is now in unspecified state, but we do not use it later
    ffec::Fr_vector<ppT> Ht_table = (ssp_inst.Ht); // ssp_inst.Ht is now in unspecified state, but we do not use it later

    Vt_table.push(ssp_inst.Zt);

    ffec::Fr_vector<ppT> Xt_table = ffec::Fr_vector<ppT>(Vt_table.begin(), Vt_table.begin() + ssp_inst.num_inputs() + 1);
    ffec::Fr_vector<ppT> Vt_table_minus_Xt_table = ffec::Fr_vector<ppT>(Vt_table.begin() + ssp_inst.num_inputs() + 1, Vt_table.end());

    /* sanity checks */

    assert!(Vt_table.size() == ssp_inst.num_variables() + 2);
    print!("Ht_table.size() = {}, ssp_inst.degree() + 1 = {}\n", Ht_table.size(), ssp_inst.degree() + 1);
    assert!(Ht_table.size() == ssp_inst.degree() + 1);
    assert!(Xt_table.size() == ssp_inst.num_inputs() + 1);
    assert!(Vt_table_minus_Xt_table.size() == ssp_inst.num_variables() + 2 - ssp_inst.num_inputs() - 1);
    for i in 0..ssp_inst.num_inputs()+1
    {
        assert!(!Xt_table[i].is_zero());
    }

    const ffec::Fr<ppT> alpha = ffec::Fr<ppT>::random_element();

    ffec::enter_block("Generate USCS proving key");

    const size_t g1_exp_count = Vt_table.size() + Vt_table_minus_Xt_table.size() + Ht_table.size();
    const size_t g2_exp_count = Vt_table_minus_Xt_table.size();

    size_t g1_window = ffec::get_exp_window_size<ffec::G1<ppT> >(g1_exp_count);
    size_t g2_window = ffec::get_exp_window_size<ffec::G2<ppT> >(g2_exp_count);

    ffec::print_indent(); print!("* G1 window: {}\n", g1_window);
    ffec::print_indent(); print!("* G2 window: {}\n", g2_window);

    ffec::enter_block("Generating G1 multiexp table");
    ffec::window_table<ffec::G1<ppT> > g1_table = get_window_table(ffec::Fr<ppT>::size_in_bits(), g1_window, ffec::G1<ppT>::one());
    ffec::leave_block("Generating G1 multiexp table");

    ffec::enter_block("Generating G2 multiexp table");
    ffec::window_table<ffec::G2<ppT> > g2_table = get_window_table(ffec::Fr<ppT>::size_in_bits(), g2_window, ffec::G2<ppT>::one());
    ffec::leave_block("Generating G2 multiexp table");

    ffec::enter_block("Generate proof components");

    ffec::enter_block("Compute the query for V_g1", false);
    ffec::G1_vector<ppT> V_g1_query = batch_exp(ffec::Fr<ppT>::size_in_bits(), g1_window, g1_table, Vt_table_minus_Xt_table);
// #ifdef USE_MIXED_ADDITION
    ffec::batch_to_special<ffec::G1<ppT> >(V_g1_query);
//#endif
    ffec::leave_block("Compute the query for V_g1", false);

    ffec::enter_block("Compute the query for alpha_V_g1", false);
    ffec::G1_vector<ppT> alpha_V_g1_query = batch_exp_with_coeff(ffec::Fr<ppT>::size_in_bits(), g1_window, g1_table, alpha, Vt_table_minus_Xt_table);
// #ifdef USE_MIXED_ADDITION
    ffec::batch_to_special<ffec::G1<ppT> >(alpha_V_g1_query);
//#endif
    ffec::leave_block("Compute the query for alpha_V_g1", false);

    ffec::enter_block("Compute the query for H_g1", false);
    ffec::G1_vector<ppT> H_g1_query = batch_exp(ffec::Fr<ppT>::size_in_bits(), g1_window, g1_table, Ht_table);
// #ifdef USE_MIXED_ADDITION
    ffec::batch_to_special<ffec::G1<ppT> >(H_g1_query);
//#endif
    ffec::leave_block("Compute the query for H_g1", false);

    ffec::enter_block("Compute the query for V_g2", false);
    ffec::G2_vector<ppT> V_g2_query = batch_exp(ffec::Fr<ppT>::size_in_bits(), g2_window, g2_table, Vt_table);
// #ifdef USE_MIXED_ADDITION
    ffec::batch_to_special<ffec::G2<ppT> >(V_g2_query);
//#endif
    ffec::leave_block("Compute the query for V_g2", false);

    ffec::leave_block("Generate proof components");

    ffec::leave_block("Generate USCS proving key");

    ffec::enter_block("Generate USCS verification key");

    const ffec::Fr<ppT> tilde    = ffec::Fr<ppT>::random_element();
    ffec::G2<ppT> tilde_g2       = tilde * ffec::G2<ppT>::one();
    ffec::G2<ppT> alpha_tilde_g2 = (alpha * tilde) * ffec::G2<ppT>::one();
    ffec::G2<ppT> Z_g2           = ssp_inst.Zt * ffec::G2<ppT>::one();

    ffec::enter_block("Encode IC query for USCS verification key");
    ffec::G1<ppT> encoded_IC_base = Xt_table[0] * ffec::G1<ppT>::one();
    ffec::G1_vector<ppT> encoded_IC_values = batch_exp(ffec::Fr<ppT>::size_in_bits(), g1_window, g1_table, ffec::Fr_vector<ppT>(Xt_table.begin() + 1, Xt_table.end()));
    ffec::leave_block("Encode IC query for USCS verification key");

    ffec::leave_block("Generate USCS verification key");

    ffec::leave_block("Call to uscs_ppzksnark_generator");

    accumulation_vector<ffec::G1<ppT> > encoded_IC_query((encoded_IC_base), (encoded_IC_values));

    uscs_ppzksnark_verification_key<ppT> vk = uscs_ppzksnark_verification_key<ppT>(tilde_g2,
                                                                                   alpha_tilde_g2,
                                                                                   Z_g2,
                                                                                   encoded_IC_query);

    uscs_ppzksnark_constraint_system<ppT> cs_copy = cs;
    uscs_ppzksnark_proving_key<ppT> pk = uscs_ppzksnark_proving_key<ppT>((V_g1_query),
                                                                         (alpha_V_g1_query),
                                                                         (H_g1_query),
                                                                         (V_g2_query),
                                                                         (cs_copy));

    pk.print_size();
    vk.print_size();

    return uscs_ppzksnark_keypair<ppT>((pk), (vk));
}

template <typename ppT>
uscs_ppzksnark_proof<ppT> uscs_ppzksnark_prover(const uscs_ppzksnark_proving_key<ppT> &pk,
                                                const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                                const uscs_ppzksnark_auxiliary_input<ppT> &auxiliary_input)
{
    ffec::enter_block("Call to uscs_ppzksnark_prover");

    const ffec::Fr<ppT> d = ffec::Fr<ppT>::random_element();

    ffec::enter_block("Compute the polynomial H");
    const ssp_witness<ffec::Fr<ppT> > ssp_wit = uscs_to_ssp_witness_map(pk.constraint_system, primary_input, auxiliary_input, d);
    ffec::leave_block("Compute the polynomial H");

    /* sanity checks */
    assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
    assert!(pk.V_g1_query.size() == ssp_wit.num_variables() + 2 - ssp_wit.num_inputs() - 1);
    assert!(pk.alpha_V_g1_query.size() == ssp_wit.num_variables() + 2 - ssp_wit.num_inputs() - 1);
    assert!(pk.H_g1_query.size() == ssp_wit.degree() + 1);
    assert!(pk.V_g2_query.size() == ssp_wit.num_variables() + 2);

// #ifdef DEBUG
    const ffec::Fr<ppT> t = ffec::Fr<ppT>::random_element();
    ssp_instance_evaluation<ffec::Fr<ppT> > ssp_inst = uscs_to_ssp_instance_map_with_evaluation(pk.constraint_system, t);
    assert!(ssp_inst.is_satisfied(ssp_wit));
//#endif

    ffec::G1<ppT> V_g1       = ssp_wit.d*pk.V_g1_query[pk.V_g1_query.size()-1];
    ffec::G1<ppT> alpha_V_g1 = ssp_wit.d*pk.alpha_V_g1_query[pk.alpha_V_g1_query.size()-1];
    ffec::G1<ppT> H_g1       = ffec::G1<ppT>::zero();
    ffec::G2<ppT> V_g2       = pk.V_g2_query[0]+ssp_wit.d*pk.V_g2_query[pk.V_g2_query.size()-1];

// #ifdef MULTICORE
    const size_t chunks = omp_get_max_threads(); // to override, set OMP_NUM_THREADS env var or call omp_set_num_threads()
#else
    const size_t chunks = 1;
//#endif

    // MAYBE LATER: do queries 1,2,4 at once for slightly better speed

    ffec::enter_block("Compute the proof");

    ffec::enter_block("Compute V_g1, the 1st component of the proof", false);
    V_g1 = V_g1 + ffec::multi_exp_with_mixed_addition<ffec::G1<ppT>,
                                                       ffec::Fr<ppT>,
                                                       ffec::multi_exp_method_BDLO12>(
        pk.V_g1_query.begin(), pk.V_g1_query.begin()+(ssp_wit.num_variables()-ssp_wit.num_inputs()),
        ssp_wit.coefficients_for_Vs.begin()+ssp_wit.num_inputs(), ssp_wit.coefficients_for_Vs.begin()+ssp_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute V_g1, the 1st component of the proof", false);

    ffec::enter_block("Compute alpha_V_g1, the 2nd component of the proof", false);
    alpha_V_g1 = alpha_V_g1 + ffec::multi_exp_with_mixed_addition<ffec::G1<ppT>,
                                                                   ffec::Fr<ppT>,
                                                                   ffec::multi_exp_method_BDLO12>(
        pk.alpha_V_g1_query.begin(), pk.alpha_V_g1_query.begin()+(ssp_wit.num_variables()-ssp_wit.num_inputs()),
        ssp_wit.coefficients_for_Vs.begin()+ssp_wit.num_inputs(), ssp_wit.coefficients_for_Vs.begin()+ssp_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute alpha_V_g1, the 2nd component of the proof", false);

    ffec::enter_block("Compute H_g1, the 3rd component of the proof", false);
    H_g1 = H_g1 + ffec::multi_exp<ffec::G1<ppT>,
                                   ffec::Fr<ppT>,
                                   ffec::multi_exp_method_BDLO12>(
        pk.H_g1_query.begin(), pk.H_g1_query.begin()+ssp_wit.degree()+1,
        ssp_wit.coefficients_for_H.begin(), ssp_wit.coefficients_for_H.begin()+ssp_wit.degree()+1,
        chunks);
    ffec::leave_block("Compute H_g1, the 3rd component of the proof", false);

    ffec::enter_block("Compute V_g2, the 4th component of the proof", false);
    V_g2 = V_g2 + ffec::multi_exp<ffec::G2<ppT>,
                                   ffec::Fr<ppT>,
                                   ffec::multi_exp_method_BDLO12>(
        pk.V_g2_query.begin()+1, pk.V_g2_query.begin()+ssp_wit.num_variables()+1,
        ssp_wit.coefficients_for_Vs.begin(), ssp_wit.coefficients_for_Vs.begin()+ssp_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute V_g2, the 4th component of the proof", false);

    ffec::leave_block("Compute the proof");

    ffec::leave_block("Call to uscs_ppzksnark_prover");

    uscs_ppzksnark_proof<ppT> proof = uscs_ppzksnark_proof<ppT>((V_g1), (alpha_V_g1), (H_g1), (V_g2));

    proof.print_size();

    return proof;
}

template <typename ppT>
uscs_ppzksnark_processed_verification_key<ppT> uscs_ppzksnark_verifier_process_vk(const uscs_ppzksnark_verification_key<ppT> &vk)
{
    ffec::enter_block("Call to uscs_ppzksnark_verifier_process_vk");

    uscs_ppzksnark_processed_verification_key<ppT> pvk;

    pvk.pp_G1_one_precomp         = ppT::precompute_G1(ffec::G1<ppT>::one());
    pvk.pp_G2_one_precomp         = ppT::precompute_G2(ffec::G2<ppT>::one());

    pvk.vk_tilde_g2_precomp       = ppT::precompute_G2(vk.tilde_g2);
    pvk.vk_alpha_tilde_g2_precomp = ppT::precompute_G2(vk.alpha_tilde_g2);
    pvk.vk_Z_g2_precomp           = ppT::precompute_G2(vk.Z_g2);

    pvk.pairing_of_g1_and_g2      = ppT::miller_loop(pvk.pp_G1_one_precomp,pvk.pp_G2_one_precomp);

    pvk.encoded_IC_query = vk.encoded_IC_query;

    ffec::leave_block("Call to uscs_ppzksnark_verifier_process_vk");

    return pvk;
}

template <typename ppT>
bool uscs_ppzksnark_online_verifier_weak_IC(const uscs_ppzksnark_processed_verification_key<ppT> &pvk,
                                            const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                            const uscs_ppzksnark_proof<ppT> &proof)
{
    ffec::enter_block("Call to uscs_ppzksnark_online_verifier_weak_IC");
    assert!(pvk.encoded_IC_query.domain_size() >= primary_input.size());

    ffec::enter_block("Compute input-dependent part of V");
    const accumulation_vector<ffec::G1<ppT> > accumulated_IC = pvk.encoded_IC_query.accumulate_chunk<ffec::Fr<ppT> >(primary_input.begin(), primary_input.end(), 0);
    assert!(accumulated_IC.is_fully_accumulated());
    const ffec::G1<ppT> &acc = accumulated_IC.first;
    ffec::leave_block("Compute input-dependent part of V");

    bool result = true;

    ffec::enter_block("Check if the proof is well-formed");
    if !proof.is_well_formed()
    {
        if !ffec::inhibit_profiling_info
        {
            ffec::print_indent(); print!("At least one of the proof components is not well-formed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check if the proof is well-formed");

    ffec::enter_block("Online pairing computations");

    ffec::enter_block("Check knowledge commitment for V is valid");
    ffec::G1_precomp<ppT> proof_V_g1_with_acc_precomp = ppT::precompute_G1(proof.V_g1 + acc);
    ffec::G2_precomp<ppT> proof_V_g2_precomp = ppT::precompute_G2(proof.V_g2);
    ffec::Fqk<ppT> V_1 = ppT::miller_loop(proof_V_g1_with_acc_precomp,    pvk.pp_G2_one_precomp);
    ffec::Fqk<ppT> V_2 = ppT::miller_loop(pvk.pp_G1_one_precomp, proof_V_g2_precomp);
    ffec::GT<ppT> V = ppT::final_exponentiation(V_1 * V_2.unitary_inverse());
    if V != ffec::GT::<ppT>::one()
    {
        if !ffec::inhibit_profiling_info
        {
            ffec::print_indent(); print!("Knowledge commitment for V invalid.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for V is valid");

    ffec::enter_block("Check SSP divisibility"); // i.e., check that V^2=H*Z+1
    ffec::G1_precomp<ppT> proof_H_g1_precomp = ppT::precompute_G1(proof.H_g1);
    ffec::Fqk<ppT> SSP_1  = ppT::miller_loop(proof_V_g1_with_acc_precomp,  proof_V_g2_precomp);
    ffec::Fqk<ppT> SSP_2  = ppT::miller_loop(proof_H_g1_precomp, pvk.vk_Z_g2_precomp);
    ffec::GT<ppT> SSP = ppT::final_exponentiation(SSP_1.unitary_inverse() * SSP_2 * pvk.pairing_of_g1_and_g2);
    if SSP != ffec::GT::<ppT>::one()
    {
        if !ffec::inhibit_profiling_info
        {
            ffec::print_indent(); print!("SSP divisibility check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check SSP divisibility");

    ffec::enter_block("Check same coefficients were used");
    ffec::G1_precomp<ppT> proof_V_g1_precomp = ppT::precompute_G1(proof.V_g1);
    ffec::G1_precomp<ppT> proof_alpha_V_g1_precomp = ppT::precompute_G1(proof.alpha_V_g1);
    ffec::Fqk<ppT> alpha_V_1 = ppT::miller_loop(proof_V_g1_precomp, pvk.vk_alpha_tilde_g2_precomp);
    ffec::Fqk<ppT> alpha_V_2 = ppT::miller_loop(proof_alpha_V_g1_precomp, pvk.vk_tilde_g2_precomp);
    ffec::GT<ppT> alpha_V = ppT::final_exponentiation(alpha_V_1 * alpha_V_2.unitary_inverse());
    if alpha_V != ffec::GT::<ppT>::one()
    {
        if !ffec::inhibit_profiling_info
        {
            ffec::print_indent(); print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check same coefficients were used");

    ffec::leave_block("Online pairing computations");

    ffec::leave_block("Call to uscs_ppzksnark_online_verifier_weak_IC");

    return result;
}

template<typename ppT>
bool uscs_ppzksnark_verifier_weak_IC(const uscs_ppzksnark_verification_key<ppT> &vk,
                                     const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                     const uscs_ppzksnark_proof<ppT> &proof)
{
    ffec::enter_block("Call to uscs_ppzksnark_verifier_weak_IC");
    uscs_ppzksnark_processed_verification_key<ppT> pvk = uscs_ppzksnark_verifier_process_vk<ppT>(vk);
    bool result = uscs_ppzksnark_online_verifier_weak_IC<ppT>(pvk, primary_input, proof);
    ffec::leave_block("Call to uscs_ppzksnark_verifier_weak_IC");
    return result;
}

template<typename ppT>
bool uscs_ppzksnark_online_verifier_strong_IC(const uscs_ppzksnark_processed_verification_key<ppT> &pvk,
                                              const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                              const uscs_ppzksnark_proof<ppT> &proof)
{
    bool result = true;
    ffec::enter_block("Call to uscs_ppzksnark_online_verifier_strong_IC");

    if pvk.encoded_IC_query.domain_size() != primary_input.size()
    {
        ffec::print_indent(); print!("Input length differs from expected (got {}, expected {}).\n", primary_input.size(), pvk.encoded_IC_query.domain_size());
        result = false;
    }
    else
    {
        result = uscs_ppzksnark_online_verifier_weak_IC(pvk, primary_input, proof);
    }

    ffec::leave_block("Call to uscs_ppzksnark_online_verifier_strong_IC");
    return result;
}

template<typename ppT>
bool uscs_ppzksnark_verifier_strong_IC(const uscs_ppzksnark_verification_key<ppT> &vk,
                                       const uscs_ppzksnark_primary_input<ppT> &primary_input,
                                       const uscs_ppzksnark_proof<ppT> &proof)
{
    ffec::enter_block("Call to uscs_ppzksnark_verifier_strong_IC");
    uscs_ppzksnark_processed_verification_key<ppT> pvk = uscs_ppzksnark_verifier_process_vk<ppT>(vk);
    bool result = uscs_ppzksnark_online_verifier_strong_IC<ppT>(pvk, primary_input, proof);
    ffec::leave_block("Call to uscs_ppzksnark_verifier_strong_IC");
    return result;
}



//#endif // USCS_PPZKSNARK_TCC_
