/** @file
 *****************************************************************************

 Declaration of interfaces for a *multi-predicate* ppzkPCD for R1CS.

 This includes:
 - class for proving key
 - class for verification key
 - class for processed verification key
 - class for key pair (proving key & verification key)
 - class for proof
 - generator algorithm
 - prover algorithm
 - verifier algorithm
 - online verifier algorithm

 The implementation follows, extends, and optimizes the approach described
 in \[CTV15]. Thus, PCD is constructed from two "matched" ppzkSNARKs for R1CS.

 Acronyms:

 "R1CS" = "Rank-1 Constraint Systems"
 "ppzkSNARK" = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
 "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

 References:

 \[CTV15]:
 "Cluster Computing in Zero Knowledge",
 Alessandro Chiesa, Eran Tromer, Madars Virza,

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_MP_PPZKPCD_HPP_
// #define R1CS_MP_PPZKPCD_HPP_

use  <memory>
use  <vector>

use crate::common::data_structures::set_commitment;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/ppzkpcd_compliance_predicate;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/r1cs_mp_ppzkpcd_params;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;



/******************************** Proving key ********************************/

template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_proving_key;

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk);

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk);

/**
 * A proving key for the R1CS (multi-predicate) ppzkPCD.
 */
template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_proving_key {
public:
    type typename PCD_ppT::curve_A_pp A_pp;
    type typename PCD_ppT::curve_B_pp B_pp;

    std::vector<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> > compliance_predicates;

    std::vector<r1cs_ppzksnark_proving_key<A_pp> > compliance_step_r1cs_pks;
    std::vector<r1cs_ppzksnark_proving_key<B_pp> > translation_step_r1cs_pks;

    std::vector<r1cs_ppzksnark_verification_key<A_pp> > compliance_step_r1cs_vks;
    std::vector<r1cs_ppzksnark_verification_key<B_pp> > translation_step_r1cs_vks;

    set_commitment commitment_to_translation_step_r1cs_vks;
    std::vector<set_membership_proof> compliance_step_r1cs_vk_membership_proofs;

    std::map<size_t, size_t> compliance_predicate_name_to_idx;

    r1cs_mp_ppzkpcd_proving_key() {};
    r1cs_mp_ppzkpcd_proving_key(const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &other) = default;
    r1cs_mp_ppzkpcd_proving_key(r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &&other) = default;
    r1cs_mp_ppzkpcd_proving_key(const std::vector<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> > &compliance_predicates,
                                const std::vector<r1cs_ppzksnark_proving_key<A_pp> > &compliance_step_r1cs_pk,
                                const std::vector<r1cs_ppzksnark_proving_key<B_pp> > &translation_step_r1cs_pk,
                                const std::vector<r1cs_ppzksnark_verification_key<A_pp> > &compliance_step_r1cs_vk,
                                const std::vector<r1cs_ppzksnark_verification_key<B_pp> > &translation_step_r1cs_vk,
                                const set_commitment &commitment_to_translation_step_r1cs_vks,
                                const std::vector<set_membership_proof> &compliance_step_r1cs_vk_membership_proofs,
                                const std::map<size_t, size_t> &compliance_predicate_name_to_idx) :
    compliance_predicates(compliance_predicates),
        compliance_step_r1cs_pks(compliance_step_r1cs_pks),
        translation_step_r1cs_pks(translation_step_r1cs_pks),
        compliance_step_r1cs_vks(compliance_step_r1cs_vks),
        translation_step_r1cs_vks(translation_step_r1cs_vks),
        commitment_to_translation_step_r1cs_vks(commitment_to_translation_step_r1cs_vks),
        compliance_step_r1cs_vk_membership_proofs(compliance_step_r1cs_vk_membership_proofs),
        compliance_predicate_name_to_idx(compliance_predicate_name_to_idx)
    {}

    r1cs_mp_ppzkpcd_proving_key<PCD_ppT>& operator=(const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &other) = default;

    size_t size_in_bits() const;

    bool is_well_formed() const;

    bool operator==(const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &other) const;
    friend std::ostream& operator<< <PCD_ppT>(std::ostream &out, const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk);
    friend std::istream& operator>> <PCD_ppT>(std::istream &in, r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk);
};


/******************************* Verification key ****************************/

template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_verification_key;

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk);

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk);

/**
 * A verification key for the R1CS (multi-predicate) ppzkPCD.
 */
template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_verification_key {
public:
    type typename PCD_ppT::curve_A_pp A_pp;
    type typename PCD_ppT::curve_B_pp B_pp;

    std::vector<r1cs_ppzksnark_verification_key<A_pp> > compliance_step_r1cs_vks;
    std::vector<r1cs_ppzksnark_verification_key<B_pp> > translation_step_r1cs_vks;
    set_commitment commitment_to_translation_step_r1cs_vks;

    r1cs_mp_ppzkpcd_verification_key() = default;
    r1cs_mp_ppzkpcd_verification_key(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &other) = default;
    r1cs_mp_ppzkpcd_verification_key(r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &&other) = default;
    r1cs_mp_ppzkpcd_verification_key(const std::vector<r1cs_ppzksnark_verification_key<A_pp> > &compliance_step_r1cs_vks,
                                     const std::vector<r1cs_ppzksnark_verification_key<B_pp> > &translation_step_r1cs_vks,
                                     const set_commitment &commitment_to_translation_step_r1cs_vks) :
        compliance_step_r1cs_vks(compliance_step_r1cs_vks),
        translation_step_r1cs_vks(translation_step_r1cs_vks),
        commitment_to_translation_step_r1cs_vks(commitment_to_translation_step_r1cs_vks)
    {}

    r1cs_mp_ppzkpcd_verification_key<PCD_ppT>& operator=(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &other) = default;

    size_t size_in_bits() const;

    bool operator==(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &other) const;
    friend std::ostream& operator<< <PCD_ppT>(std::ostream &out, const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk);
    friend std::istream& operator>> <PCD_ppT>(std::istream &in, r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk);
};


/************************* Processed verification key **************************/

template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_processed_verification_key;

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk);

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk);

/**
 * A processed verification key for the R1CS (multi-predicate) ppzkPCD.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_processed_verification_key {
public:
    type typename PCD_ppT::curve_A_pp A_pp;
    type typename PCD_ppT::curve_B_pp B_pp;

    std::vector<r1cs_ppzksnark_processed_verification_key<A_pp> > compliance_step_r1cs_pvks;
    std::vector<r1cs_ppzksnark_processed_verification_key<B_pp> > translation_step_r1cs_pvks;
    set_commitment commitment_to_translation_step_r1cs_vks;

    r1cs_mp_ppzkpcd_processed_verification_key() = default;
    r1cs_mp_ppzkpcd_processed_verification_key(const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &other) = default;
    r1cs_mp_ppzkpcd_processed_verification_key(r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &&other) = default;
    r1cs_mp_ppzkpcd_processed_verification_key(std::vector<r1cs_ppzksnark_processed_verification_key<A_pp> > &&compliance_step_r1cs_pvks,
                                               std::vector<r1cs_ppzksnark_processed_verification_key<B_pp> > &&translation_step_r1cs_pvks,
                                               const set_commitment &commitment_to_translation_step_r1cs_vks) :
        compliance_step_r1cs_pvks((compliance_step_r1cs_pvks)),
        translation_step_r1cs_pvks((translation_step_r1cs_pvks)),
        commitment_to_translation_step_r1cs_vks(commitment_to_translation_step_r1cs_vks)
    {};

    r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>& operator=(const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &other) = default;

    size_t size_in_bits() const;

    bool operator==(const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &other) const;
    friend std::ostream& operator<< <PCD_ppT>(std::ostream &out, const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk);
    friend std::istream& operator>> <PCD_ppT>(std::istream &in, r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk);
};


/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS (multi-predicate) ppzkPC, which consists of a proving key and a verification key.
 */
template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_keypair {
public:
    r1cs_mp_ppzkpcd_proving_key<PCD_ppT> pk;
    r1cs_mp_ppzkpcd_verification_key<PCD_ppT> vk;

    r1cs_mp_ppzkpcd_keypair() = default;
    r1cs_mp_ppzkpcd_keypair(r1cs_mp_ppzkpcd_keypair<PCD_ppT> &&other) = default;
    r1cs_mp_ppzkpcd_keypair(r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &&pk,
                            r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &&vk) :
        pk((pk)),
        vk((vk))
    {};
};


/*********************************** Proof ***********************************/

template<typename ppT>
class r1cs_mp_ppzkpcd_proof;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_proof<ppT> &proof);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_proof<ppT> &proof);

/**
 * A proof for the R1CS (multi-predicate) ppzkPCD.
 */
template<typename PCD_ppT>
class r1cs_mp_ppzkpcd_proof {
public:
    size_t compliance_predicate_idx;
    r1cs_ppzksnark_proof<typename PCD_ppT::curve_B_pp> r1cs_proof;

    r1cs_mp_ppzkpcd_proof() = default;
    r1cs_mp_ppzkpcd_proof(const size_t compliance_predicate_idx,
                          const r1cs_ppzksnark_proof<typename PCD_ppT::curve_B_pp> &r1cs_proof) :
        compliance_predicate_idx(compliance_predicate_idx),
        r1cs_proof(r1cs_proof)
    {}

    size_t size_in_bits() const;

    bool operator==(const r1cs_mp_ppzkpcd_proof<PCD_ppT> &other) const;
    friend std::ostream& operator<< <PCD_ppT>(std::ostream &out, const r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof);
    friend std::istream& operator>> <PCD_ppT>(std::istream &in, r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof);
};


/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the R1CS (multi-predicate) ppzkPCD.
 *
 * Given a vector of compliance predicates, this algorithm produces proving and verification keys for the vector.
 */
template<typename PCD_ppT>
r1cs_mp_ppzkpcd_keypair<PCD_ppT> r1cs_mp_ppzkpcd_generator(const std::vector<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> > &compliance_predicates);

/**
 * A prover algorithm for the R1CS (multi-predicate) ppzkPCD.
 *
 * Given a proving key, name of chosen compliance predicate, inputs for the
 * compliance predicate, and proofs for the predicate's input messages, this
 * algorithm produces a proof (of knowledge) that attests to the compliance of
 * the output message.
 */
template <typename PCD_ppT>
r1cs_mp_ppzkpcd_proof<PCD_ppT> r1cs_mp_ppzkpcd_prover(const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk,
                                                      const size_t compliance_predicate_name,
                                                      const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> &primary_input,
                                                      const r1cs_mp_ppzkpcd_auxiliary_input<PCD_ppT> &auxiliary_input,
                                                      const std::vector<r1cs_mp_ppzkpcd_proof<PCD_ppT> > &incoming_proofs);

/*
  Below are two variants of verifier algorithm for the R1CS (multi-predicate) ppzkPCD.

  These are the two cases that arise from whether the verifier accepts a
  (non-processed) verification key or, instead, a processed verification key.
  In the latter case, we call the algorithm an "online verifier".
*/

/**
 * A verifier algorithm for the R1CS (multi-predicate) ppzkPCD that
 * accepts a non-processed verification key.
 */
template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_verifier(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk,
                              const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> &primary_input,
                              const r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof);

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
template<typename PCD_ppT>
r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> r1cs_mp_ppzkpcd_process_vk(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk);

/**
 * A verifier algorithm for the R1CS (multi-predicate) ppzkPCD that
 * accepts a processed verification key.
 */
template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_online_verifier(const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk,
                                     const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> &primary_input,
                                     const r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof);



use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/r1cs_mp_ppzkpcd;

//#endif // R1CS_MP_PPZKPCD_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a *multi-predicate* ppzkPCD for R1CS.

 See r1cs_mp_ppzkpcd.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_MP_PPZKPCD_TCC_
// #define R1CS_MP_PPZKPCD_TCC_

use  <algorithm>
use  <cassert>
use  <iostream>

use ffec::common::profiling;
use ffec::common::utils;

use libsnark/common/libsnark_serialization;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/mp_pcd_circuits;



template<typename PCD_ppT>
size_t r1cs_mp_ppzkpcd_proving_key<PCD_ppT>::size_in_bits() const
{
    const size_t num_predicates = compliance_predicates.size();

    size_t result = 0;
    for (size_t i = 0; i < num_predicates; ++i)
    {
        result += (compliance_predicates[i].size_in_bits() +
                   compliance_step_r1cs_pks[i].size_in_bits() +
                   translation_step_r1cs_pks[i].size_in_bits() +
                   compliance_step_r1cs_vks[i].size_in_bits() +
                   translation_step_r1cs_vks[i].size_in_bits() +
                   compliance_step_r1cs_vk_membership_proofs[i].size_in_bits());
    }
    result += commitment_to_translation_step_r1cs_vks.size();

    return result;
}

template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_proving_key<PCD_ppT>::is_well_formed() const
{
    const size_t num_predicates = compliance_predicates.size();

    bool result;
    result = result && (compliance_step_r1cs_pks.size() == num_predicates);
    result = result && (translation_step_r1cs_pks.size() == num_predicates);
    result = result && (compliance_step_r1cs_vks.size() == num_predicates);
    result = result && (translation_step_r1cs_vks.size() == num_predicates);
    result = result && (compliance_step_r1cs_vk_membership_proofs.size() == num_predicates);

    return result;
}

template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_proving_key<PCD_ppT>::operator==(const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &other) const
{
    return (self.compliance_predicates == other.compliance_predicates &&
            self.compliance_step_r1cs_pks == other.compliance_step_r1cs_pks &&
            self.translation_step_r1cs_pks == other.translation_step_r1cs_pks &&
            self.compliance_step_r1cs_vks == other.compliance_step_r1cs_vks &&
            self.translation_step_r1cs_vks == other.translation_step_r1cs_vks &&
            self.commitment_to_translation_step_r1cs_vks == other.commitment_to_translation_step_r1cs_vks &&
            self.compliance_step_r1cs_vk_membership_proofs == other.compliance_step_r1cs_vk_membership_proofs &&
            self.compliance_predicate_name_to_idx == other.compliance_predicate_name_to_idx);
}

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk)
{
    out << pk.compliance_predicates;
    out << pk.compliance_step_r1cs_pks;
    out << pk.translation_step_r1cs_pks;
    out << pk.compliance_step_r1cs_vks;
    out << pk.translation_step_r1cs_vks;
    output_bool_vector(out, pk.commitment_to_translation_step_r1cs_vks);
    out << pk.compliance_step_r1cs_vk_membership_proofs;
    out << pk.compliance_predicate_name_to_idx;

    return out;
}

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk)
{
    in >> pk.compliance_predicates;
    in >> pk.compliance_step_r1cs_pks;
    in >> pk.translation_step_r1cs_pks;
    in >> pk.compliance_step_r1cs_vks;
    in >> pk.translation_step_r1cs_vks;
    input_bool_vector(in, pk.commitment_to_translation_step_r1cs_vks);
    in >> pk.compliance_step_r1cs_vk_membership_proofs;
    in >> pk.compliance_predicate_name_to_idx;

    return in;
}

template<typename PCD_ppT>
size_t r1cs_mp_ppzkpcd_verification_key<PCD_ppT>::size_in_bits() const
{
    const size_t num_predicates = compliance_step_r1cs_vks.size();

    size_t result = 0;
    for (size_t i = 0; i < num_predicates; ++i)
    {
        result += (compliance_step_r1cs_vks[i].size_in_bits() +
                   translation_step_r1cs_vks[i].size_in_bits());
    }

    result += commitment_to_translation_step_r1cs_vks.size();

    return result;
}

template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_verification_key<PCD_ppT>::operator==(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &other) const
{
    return (self.compliance_step_r1cs_vks == other.compliance_step_r1cs_vks &&
            self.translation_step_r1cs_vks == other.translation_step_r1cs_vks &&
            self.commitment_to_translation_step_r1cs_vks == other.commitment_to_translation_step_r1cs_vks);
}

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk)
{
    out << vk.compliance_step_r1cs_vks;
    out << vk.translation_step_r1cs_vks;
    ffec::output_bool_vector(out, vk.commitment_to_translation_step_r1cs_vks);

    return out;
}

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk)
{
    in >> vk.compliance_step_r1cs_vks;
    in >> vk.translation_step_r1cs_vks;
    ffec::input_bool_vector(in, vk.commitment_to_translation_step_r1cs_vks);

    return in;
}

template<typename PCD_ppT>
size_t r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>::size_in_bits() const
{
    const size_t num_predicates = compliance_step_r1cs_pvks.size();

    size_t result = 0;
    for (size_t i = 0; i < num_predicates; ++i)
    {
        result += (compliance_step_r1cs_pvks[i].size_in_bits() +
                   translation_step_r1cs_pvks[i].size_in_bits());
    }

    result += commitment_to_translation_step_r1cs_vks.size();

    return result;
}

template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>::operator==(const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &other) const
{
    return (self.compliance_step_r1cs_pvks == other.compliance_step_r1cs_pvks &&
            self.translation_step_r1cs_pvks == other.translation_step_r1cs_pvks &&
            self.commitment_to_translation_step_r1cs_vks == other.commitment_to_translation_step_r1cs_vks);
}

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk)
{
    out << pvk.compliance_step_r1cs_pvks;
    out << pvk.translation_step_r1cs_pvks;
    ffec::output_bool_vector(out, pvk.commitment_to_translation_step_r1cs_vks);

    return out;
}

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk)
{
    in >> pvk.compliance_step_r1cs_pvks;
    in >> pvk.translation_step_r1cs_pvks;
    ffec::input_bool_vector(in, pvk.commitment_to_translation_step_r1cs_vks);

    return in;
}

template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_proof<PCD_ppT>::operator==(const r1cs_mp_ppzkpcd_proof<PCD_ppT> &other) const
{
    return (self.compliance_predicate_idx == other.compliance_predicate_idx &&
            self.r1cs_proof == other.r1cs_proof);
}

template<typename PCD_ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof)
{
    out << proof.compliance_predicate_idx << "\n";
    out << proof.r1cs_proof;

    return out;
}

template<typename PCD_ppT>
std::istream& operator>>(std::istream &in, r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof)
{
    in >> proof.compliance_predicate_idx;
    ffec::consume_newline(in);
    in >> proof.r1cs_proof;

    return in;
}

template<typename PCD_ppT>
r1cs_mp_ppzkpcd_keypair<PCD_ppT> r1cs_mp_ppzkpcd_generator(const std::vector<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> > &compliance_predicates)
{
    assert!(ffec::Fr<typename PCD_ppT::curve_A_pp>::mod == ffec::Fq<typename PCD_ppT::curve_B_pp>::mod);
    assert!(ffec::Fq<typename PCD_ppT::curve_A_pp>::mod == ffec::Fr<typename PCD_ppT::curve_B_pp>::mod);

    type typename PCD_ppT::curve_A_pp curve_A_pp;
    type typename PCD_ppT::curve_B_pp curve_B_pp;

    type ffec::Fr<curve_A_pp> FieldT_A;
    type ffec::Fr<curve_B_pp> FieldT_B;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_generator");

    r1cs_mp_ppzkpcd_keypair<PCD_ppT> keypair;
    const size_t translation_input_size = mp_translation_step_pcd_circuit_maker<curve_B_pp>::input_size_in_elts();
    const size_t vk_size_in_bits = r1cs_ppzksnark_verification_key_variable<curve_A_pp>::size_in_bits(translation_input_size);
    print!("{} {}\n", translation_input_size, vk_size_in_bits);

    set_commitment_accumulator<CRH_with_bit_out_gadget<FieldT_A> > all_translation_vks(compliance_predicates.size(), vk_size_in_bits);

    ffec::enter_block("Perform type checks");
    std::map<size_t, size_t> type_counts;

    for (auto &cp : compliance_predicates)
    {
        type_counts[cp.type] += 1;
    }

    for (auto &cp : compliance_predicates)
    {
        if (cp.relies_on_same_type_inputs)
        {
            for (size_t type : cp.accepted_input_types)
            {
                assert!(type_counts[type] == 1); /* each of accepted_input_types must be unique */
            }
        }
        else
        {
            assert!(cp.accepted_input_types.empty());
        }
    }
    ffec::leave_block("Perform type checks");

    for (size_t i = 0; i < compliance_predicates.size(); ++i)
    {
        ffec::enter_block(FMT("", "Process predicate {} (with name {} and type {})", i, compliance_predicates[i].name, compliance_predicates[i].type));
        assert!(compliance_predicates[i].is_well_formed());

        ffec::enter_block("Construct compliance step PCD circuit");
        mp_compliance_step_pcd_circuit_maker<curve_A_pp> mp_compliance_step_pcd_circuit(compliance_predicates[i], compliance_predicates.size());
        mp_compliance_step_pcd_circuit.generate_r1cs_constraints();
        r1cs_constraint_system<FieldT_A> mp_compliance_step_pcd_circuit_cs = mp_compliance_step_pcd_circuit.get_circuit();
        ffec::leave_block("Construct compliance step PCD circuit");

        ffec::enter_block("Generate key pair for compliance step PCD circuit");
        r1cs_ppzksnark_keypair<curve_A_pp> mp_compliance_step_keypair = r1cs_ppzksnark_generator<curve_A_pp>(mp_compliance_step_pcd_circuit_cs);
        ffec::leave_block("Generate key pair for compliance step PCD circuit");

        ffec::enter_block("Construct translation step PCD circuit");
        mp_translation_step_pcd_circuit_maker<curve_B_pp> mp_translation_step_pcd_circuit(mp_compliance_step_keypair.vk);
        mp_translation_step_pcd_circuit.generate_r1cs_constraints();
        r1cs_constraint_system<FieldT_B> mp_translation_step_pcd_circuit_cs = mp_translation_step_pcd_circuit.get_circuit();
        ffec::leave_block("Construct translation step PCD circuit");

        ffec::enter_block("Generate key pair for translation step PCD circuit");
        r1cs_ppzksnark_keypair<curve_B_pp> mp_translation_step_keypair = r1cs_ppzksnark_generator<curve_B_pp>(mp_translation_step_pcd_circuit_cs);
        ffec::leave_block("Generate key pair for translation step PCD circuit");

        ffec::enter_block("Augment set of translation step verification keys");
        const ffec::bit_vector vk_bits = r1cs_ppzksnark_verification_key_variable<curve_A_pp>::get_verification_key_bits(mp_translation_step_keypair.vk);
        all_translation_vks.add(vk_bits);
        ffec::leave_block("Augment set of translation step verification keys");

        ffec::enter_block("Update r1cs_mp_ppzkpcd keypair");
        keypair.pk.compliance_predicates.push(compliance_predicates[i]);
        keypair.pk.compliance_step_r1cs_pks.push(mp_compliance_step_keypair.pk);
        keypair.pk.translation_step_r1cs_pks.push(mp_translation_step_keypair.pk);
        keypair.pk.compliance_step_r1cs_vks.push(mp_compliance_step_keypair.vk);
        keypair.pk.translation_step_r1cs_vks.push(mp_translation_step_keypair.vk);
        const size_t cp_name = compliance_predicates[i].name;
        assert!(keypair.pk.compliance_predicate_name_to_idx.find(cp_name) == keypair.pk.compliance_predicate_name_to_idx.end()); // all names must be distinct
        keypair.pk.compliance_predicate_name_to_idx[cp_name] = i;

        keypair.vk.compliance_step_r1cs_vks.push(mp_compliance_step_keypair.vk);
        keypair.vk.translation_step_r1cs_vks.push(mp_translation_step_keypair.vk);
        ffec::leave_block("Update r1cs_mp_ppzkpcd keypair");

        ffec::leave_block(FMT("", "Process predicate {} (with name {} and type {})", i, compliance_predicates[i].name, compliance_predicates[i].type));
    }

    ffec::enter_block("Compute set commitment and corresponding membership proofs");
    const set_commitment cm = all_translation_vks.get_commitment();
    keypair.pk.commitment_to_translation_step_r1cs_vks = cm;
    keypair.vk.commitment_to_translation_step_r1cs_vks = cm;
    for (size_t i = 0; i < compliance_predicates.size(); ++i)
    {
        const ffec::bit_vector vk_bits = r1cs_ppzksnark_verification_key_variable<curve_A_pp>::get_verification_key_bits(keypair.vk.translation_step_r1cs_vks[i]);
        const set_membership_proof proof = all_translation_vks.get_membership_proof(vk_bits);

        keypair.pk.compliance_step_r1cs_vk_membership_proofs.push(proof);
    }
    ffec::leave_block("Compute set commitment and corresponding membership proofs");

    ffec::print_indent(); ffec::print_mem("in generator");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_generator");

    return keypair;
}

template <typename PCD_ppT>
r1cs_mp_ppzkpcd_proof<PCD_ppT> r1cs_mp_ppzkpcd_prover(const r1cs_mp_ppzkpcd_proving_key<PCD_ppT> &pk,
                                                      const size_t compliance_predicate_name,
                                                      const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> &primary_input,
                                                      const r1cs_mp_ppzkpcd_auxiliary_input<PCD_ppT> &auxiliary_input,
                                                      const std::vector<r1cs_mp_ppzkpcd_proof<PCD_ppT> > &prev_proofs)
{
    type typename PCD_ppT::curve_A_pp curve_A_pp;
    type typename PCD_ppT::curve_B_pp curve_B_pp;

    type ffec::Fr<curve_A_pp> FieldT_A;
    type ffec::Fr<curve_B_pp> FieldT_B;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_prover");

// #ifdef DEBUG
    print!("Compliance predicate name: {}\n", compliance_predicate_name);
//#endif
    auto it = pk.compliance_predicate_name_to_idx.find(compliance_predicate_name);
    assert!(it != pk.compliance_predicate_name_to_idx.end());
    const size_t compliance_predicate_idx = it->second;

// #ifdef DEBUG
    print!("Outgoing message:\n");
    primary_input.outgoing_message->print();
//#endif

    ffec::enter_block("Prove compliance step");
    assert!(compliance_predicate_idx < pk.compliance_predicates.size());
    assert!(prev_proofs.size() <= pk.compliance_predicates[compliance_predicate_idx].max_arity);

    const size_t arity = prev_proofs.size();
    const size_t max_arity = pk.compliance_predicates[compliance_predicate_idx].max_arity;

    if (pk.compliance_predicates[compliance_predicate_idx].relies_on_same_type_inputs)
    {
        const size_t input_predicate_idx = prev_proofs[0].compliance_predicate_idx;
        for (size_t i = 1; i < arity; ++i)
        {
            assert!(prev_proofs[i].compliance_predicate_idx == input_predicate_idx);
        }
    }

    std::vector<r1cs_ppzksnark_proof<curve_B_pp> > padded_proofs(max_arity);
    for (size_t i = 0; i < arity; ++i)
    {
        padded_proofs[i] = prev_proofs[i].r1cs_proof;
    }

    std::vector<r1cs_ppzksnark_verification_key<curve_B_pp> > translation_step_vks;
    std::vector<set_membership_proof> membership_proofs;

    for (size_t i = 0; i < arity; ++i)
    {
        const size_t input_predicate_idx = prev_proofs[i].compliance_predicate_idx;
        translation_step_vks.push(pk.translation_step_r1cs_vks[input_predicate_idx]);
        membership_proofs.push(pk.compliance_step_r1cs_vk_membership_proofs[input_predicate_idx]);

// #ifdef DEBUG
        if (auxiliary_input.incoming_messages[i]->type != 0)
        {
            print!("check proof for message {}\n", i);
            const r1cs_primary_input<FieldT_B> translated_msg = get_mp_translation_step_pcd_circuit_input<curve_B_pp>(pk.commitment_to_translation_step_r1cs_vks,
                                                                                                                      auxiliary_input.incoming_messages[i]);
            const bool bit = r1cs_ppzksnark_verifier_strong_IC<curve_B_pp>(translation_step_vks[i], translated_msg, padded_proofs[i]);
            assert!(bit);
        }
        else
        {
            print!("message {} is base case\n", i);
        }
//#endif
    }

    /* pad with dummy vks/membership proofs */
    for (size_t i = arity; i < max_arity; ++i)
    {
        print!("proof {} will be a dummy\n", arity);
        translation_step_vks.push(pk.translation_step_r1cs_vks[0]);
        membership_proofs.push(pk.compliance_step_r1cs_vk_membership_proofs[0]);
    }

    mp_compliance_step_pcd_circuit_maker<curve_A_pp> mp_compliance_step_pcd_circuit(pk.compliance_predicates[compliance_predicate_idx], pk.compliance_predicates.size());

    mp_compliance_step_pcd_circuit.generate_r1cs_witness(pk.commitment_to_translation_step_r1cs_vks,
                                                         translation_step_vks,
                                                         membership_proofs,
                                                         primary_input,
                                                         auxiliary_input,
                                                         padded_proofs);

    const r1cs_primary_input<FieldT_A> compliance_step_primary_input = mp_compliance_step_pcd_circuit.get_primary_input();
    const r1cs_auxiliary_input<FieldT_A> compliance_step_auxiliary_input = mp_compliance_step_pcd_circuit.get_auxiliary_input();
    const r1cs_ppzksnark_proof<curve_A_pp> compliance_step_proof = r1cs_ppzksnark_prover<curve_A_pp>(pk.compliance_step_r1cs_pks[compliance_predicate_idx], compliance_step_primary_input, compliance_step_auxiliary_input);
    ffec::leave_block("Prove compliance step");

// #ifdef DEBUG
    const r1cs_primary_input<FieldT_A> compliance_step_input = get_mp_compliance_step_pcd_circuit_input<curve_A_pp>(pk.commitment_to_translation_step_r1cs_vks, primary_input.outgoing_message);
    const bool compliance_step_ok = r1cs_ppzksnark_verifier_strong_IC<curve_A_pp>(pk.compliance_step_r1cs_vks[compliance_predicate_idx], compliance_step_input, compliance_step_proof);
    assert!(compliance_step_ok);
//#endif

    ffec::enter_block("Prove translation step");
    mp_translation_step_pcd_circuit_maker<curve_B_pp> mp_translation_step_pcd_circuit(pk.compliance_step_r1cs_vks[compliance_predicate_idx]);

    const r1cs_primary_input<FieldT_B> translation_step_primary_input = get_mp_translation_step_pcd_circuit_input<curve_B_pp>(pk.commitment_to_translation_step_r1cs_vks, primary_input);
    mp_translation_step_pcd_circuit.generate_r1cs_witness(translation_step_primary_input, compliance_step_proof);
    const r1cs_auxiliary_input<FieldT_B> translation_step_auxiliary_input = mp_translation_step_pcd_circuit.get_auxiliary_input();

    const r1cs_ppzksnark_proof<curve_B_pp> translation_step_proof = r1cs_ppzksnark_prover<curve_B_pp>(pk.translation_step_r1cs_pks[compliance_predicate_idx], translation_step_primary_input, translation_step_auxiliary_input);

    ffec::leave_block("Prove translation step");

// #ifdef DEBUG
    const bool translation_step_ok = r1cs_ppzksnark_verifier_strong_IC<curve_B_pp>(pk.translation_step_r1cs_vks[compliance_predicate_idx], translation_step_primary_input, translation_step_proof);
    assert!(translation_step_ok);
//#endif

    ffec::print_indent(); ffec::print_mem("in prover");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_prover");

    r1cs_mp_ppzkpcd_proof<PCD_ppT> result;
    result.compliance_predicate_idx = compliance_predicate_idx;
    result.r1cs_proof = translation_step_proof;
    return result;
}

template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_online_verifier(const r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk,
                                     const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> &primary_input,
                                     const r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof)
{
    type typename PCD_ppT::curve_B_pp curve_B_pp;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_online_verifier");
    const r1cs_primary_input<ffec::Fr<curve_B_pp> > r1cs_input = get_mp_translation_step_pcd_circuit_input<curve_B_pp>(pvk.commitment_to_translation_step_r1cs_vks, primary_input);
    const bool result = r1cs_ppzksnark_online_verifier_strong_IC(pvk.translation_step_r1cs_pvks[proof.compliance_predicate_idx], r1cs_input, proof.r1cs_proof);

    ffec::print_indent(); ffec::print_mem("in online verifier");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_online_verifier");
    return result;
}

template<typename PCD_ppT>
r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> r1cs_mp_ppzkpcd_process_vk(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk)
{
    type typename PCD_ppT::curve_A_pp curve_A_pp;
    type typename PCD_ppT::curve_B_pp curve_B_pp;

    ffec::enter_block("Call to r1cs_mp_ppzkpcd_processed_verification_key");

    r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> result;
    result.commitment_to_translation_step_r1cs_vks = vk.commitment_to_translation_step_r1cs_vks;

    for (size_t i = 0; i < vk.compliance_step_r1cs_vks.size(); ++i)
    {
        const r1cs_ppzksnark_processed_verification_key<curve_A_pp> compliance_step_r1cs_pvk = r1cs_ppzksnark_verifier_process_vk<curve_A_pp>(vk.compliance_step_r1cs_vks[i]);
        const r1cs_ppzksnark_processed_verification_key<curve_B_pp> translation_step_r1cs_pvk = r1cs_ppzksnark_verifier_process_vk<curve_B_pp>(vk.translation_step_r1cs_vks[i]);

        result.compliance_step_r1cs_pvks.push(compliance_step_r1cs_pvk);
        result.translation_step_r1cs_pvks.push(translation_step_r1cs_pvk);
    }
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_processed_verification_key");

    return result;
}


template<typename PCD_ppT>
bool r1cs_mp_ppzkpcd_verifier(const r1cs_mp_ppzkpcd_verification_key<PCD_ppT> &vk,
                              const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> &primary_input,
                              const r1cs_mp_ppzkpcd_proof<PCD_ppT> &proof)
{
    ffec::enter_block("Call to r1cs_mp_ppzkpcd_verifier");
    r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> pvk = r1cs_mp_ppzkpcd_process_vk(vk);
    const bool result = r1cs_mp_ppzkpcd_online_verifier(pvk, primary_input, proof);

    ffec::print_indent(); ffec::print_mem("in verifier");
    ffec::leave_block("Call to r1cs_mp_ppzkpcd_verifier");
    return result;
}




//#endif // R1CS_MP_PPZKPCD_TCC_
