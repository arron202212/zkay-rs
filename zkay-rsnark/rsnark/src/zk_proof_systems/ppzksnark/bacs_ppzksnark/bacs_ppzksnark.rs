/** @file
 *****************************************************************************

 Declaration of interfaces for a ppzkSNARK for BACS.

 This includes:
 - pub struct for proving key
 - pub struct for verification key
 - pub struct for processed verification key
 - pub struct for key pair (proving key & verification key)
 - pub struct for proof
 - generator algorithm
 - prover algorithm
 - verifier algorithm (with strong or weak input consistency)
 - online verifier algorithm (with strong or weak input consistency)

 The implementation is a straightforward combination of:
 (1) a BACS-to-R1CS reduction, and
 (2) a ppzkSNARK for R1CS.


 Acronyms:

 - BACS = "Bilinear Arithmetic Circuit Satisfiability"
 - R1CS = "Rank-1 Constraint System"
 - ppzkSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BACS_PPZKSNARK_HPP_
// #define BACS_PPZKSNARK_HPP_

use crate::relations::circuit_satisfaction_problems::bacs::bacs;
use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/bacs_ppzksnark_params;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;



/******************************** Proving key ********************************/


pub struct bacs_ppzksnark_proving_key;


std::ostream& operator<<(std::ostream &out, pk:&bacs_ppzksnark_proving_key<ppT>);


std::istream& operator>>(std::istream &in, bacs_ppzksnark_proving_key<ppT> &pk);

/**
 * A proving key for the BACS ppzkSNARK.
 */

pub struct bacs_ppzksnark_proving_key {

    bacs_ppzksnark_circuit<ppT> circuit;
    r1cs_ppzksnark_proving_key<ppT> r1cs_pk;

    bacs_ppzksnark_proving_key() {};
    bacs_ppzksnark_proving_key(other:&bacs_ppzksnark_proving_key<ppT>) = default;
    bacs_ppzksnark_proving_key(bacs_ppzksnark_proving_key<ppT> &&other) = default;
    bacs_ppzksnark_proving_key(circuit:&bacs_ppzksnark_circuit<ppT>,
                               r1cs_pk:&r1cs_ppzksnark_proving_key<ppT>)->Self
       circuit,r1cs_pk
    {}
    bacs_ppzksnark_proving_key(bacs_ppzksnark_circuit<ppT> &&circuit,
                               r1cs_ppzksnark_proving_key<ppT> &&r1cs_pk)->Self
        circuit((circuit)), r1cs_pk((r1cs_pk))
    {}

    bacs_ppzksnark_proving_key<ppT>& operator=(other:&bacs_ppzksnark_proving_key<ppT>) = default;

    usize G1_size() const
    {
        return r1cs_pk.G1_size();
    }

    usize G2_size() const
    {
        return r1cs_pk.G2_size();
    }

    usize G1_sparse_size() const
    {
        return r1cs_pk.G1_sparse_size();
    }

    usize G2_sparse_size() const
    {
        return r1cs_pk.G2_sparse_size();
    }

    usize size_in_bits() const
    {
        return r1cs_pk.size_in_bits();
    }

    pub fn  print_size() const
    {
        r1cs_pk.print_size();
    }

    bool operator==(other:&bacs_ppzksnark_proving_key<ppT>) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, pk:&bacs_ppzksnark_proving_key<ppT>);
    friend std::istream& operator>> <ppT>(std::istream &in, bacs_ppzksnark_proving_key<ppT> &pk);
};


/******************************* Verification key ****************************/

/**
 * A verification key for the BACS ppzkSNARK.
 */

using bacs_ppzksnark_verification_key = r1cs_ppzksnark_verification_key<ppT>;


/************************ Processed verification key *************************/

/**
 * A processed verification key for the BACS ppzkSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */

using bacs_ppzksnark_processed_verification_key = r1cs_ppzksnark_processed_verification_key<ppT>;


/********************************** Key pair *********************************/

/**
 * A key pair for the BACS ppzkSNARK, which consists of a proving key and a verification key.
 */

pub struct bacs_ppzksnark_keypair {

    bacs_ppzksnark_proving_key<ppT> pk;
    bacs_ppzksnark_verification_key<ppT> vk;

    bacs_ppzksnark_keypair() {};
    bacs_ppzksnark_keypair(bacs_ppzksnark_keypair<ppT> &&other) = default;
    bacs_ppzksnark_keypair(pk:&bacs_ppzksnark_proving_key<ppT>,
                           vk:&bacs_ppzksnark_verification_key<ppT>)->Self
       pk,
        vk(vk)
    {}

    bacs_ppzksnark_keypair(bacs_ppzksnark_proving_key<ppT> &&pk,
                           bacs_ppzksnark_verification_key<ppT> &&vk)->Self
        pk((pk)),
        vk((vk))
    {}
};


/*********************************** Proof ***********************************/

/**
 * A proof for the BACS ppzkSNARK.
 */

using bacs_ppzksnark_proof = r1cs_ppzksnark_proof<ppT>;


/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the BACS ppzkSNARK.
 *
 * Given a BACS circuit C, this algorithm produces proving and verification keys for C.
 */

bacs_ppzksnark_keypair<ppT> bacs_ppzksnark_generator(circuit:&bacs_ppzksnark_circuit<ppT>);

/**
 * A prover algorithm for the BACS ppzkSNARK.
 *
 * Given a BACS primary input X and a BACS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that C(X,Y)=0''.
 * Above, C is the BACS circuit that was given as input to the generator algorithm.
 */

bacs_ppzksnark_proof<ppT> bacs_ppzksnark_prover(pk:&bacs_ppzksnark_proving_key<ppT>,
                                                primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                                auxiliary_input:&bacs_ppzksnark_auxiliary_input<ppT>);

/*
 Below are four variants of verifier algorithm for the BACS ppzkSNARK.

 These are the four cases that arise from the following two choices:

 (1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
     In the latter case, we call the algorithm an "online verifier".

 (2) The verifier checks for "weak" input consistency or, instead, "strong" input consistency.
     Strong input consistency requires that |primary_input| = C.num_inputs, whereas
     weak input consistency requires that |primary_input| <= C.num_inputs (and
     the primary input is implicitly padded with zeros up to length C.num_inputs).
 */

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has weak input consistency.
 */

bool bacs_ppzksnark_verifier_weak_IC(vk:&bacs_ppzksnark_verification_key<ppT>,
                                     primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                     proof:&bacs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a non-processed verification key, and
 * (2) has strong input consistency.
 */

bool bacs_ppzksnark_verifier_strong_IC(vk:&bacs_ppzksnark_verification_key<ppT>,
                                       primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                       proof:&bacs_ppzksnark_proof<ppT>);

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */

bacs_ppzksnark_processed_verification_key<ppT> bacs_ppzksnark_verifier_process_vk(vk:&bacs_ppzksnark_verification_key<ppT>);

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has weak input consistency.
 */

bool bacs_ppzksnark_online_verifier_weak_IC(pvk:&bacs_ppzksnark_processed_verification_key<ppT>,
                                            primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                            proof:&bacs_ppzksnark_proof<ppT>);

/**
 * A verifier algorithm for the BACS ppzkSNARK that:
 * (1) accepts a processed verification key, and
 * (2) has strong input consistency.
 */

bool bacs_ppzksnark_online_verifier_strong_IC(pvk:&bacs_ppzksnark_processed_verification_key<ppT>,
                                              primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                              proof:&bacs_ppzksnark_proof<ppT>);



use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/bacs_ppzksnark;

//#endif // BACS_PPZKSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a ppzkSNARK for BACS.

 See bacs_ppzksnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BACS_PPZKSNARK_TCC_
// #define BACS_PPZKSNARK_TCC_

use crate::reductions::bacs_to_r1cs::bacs_to_r1cs;





bool bacs_ppzksnark_proving_key<ppT>::operator==(other:&bacs_ppzksnark_proving_key<ppT>) const
{
    return (self.circuit == other.circuit &&
            self.r1cs_pk == other.r1cs_pk);
}


std::ostream& operator<<(std::ostream &out, pk:&bacs_ppzksnark_proving_key<ppT>)
{
    out << pk.circuit << OUTPUT_NEWLINE;
    out << pk.r1cs_pk << OUTPUT_NEWLINE;

    return out;
}


std::istream& operator>>(std::istream &in, bacs_ppzksnark_proving_key<ppT> &pk)
{
    in >> pk.circuit;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pk.r1cs_pk;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}



bacs_ppzksnark_keypair<ppT> bacs_ppzksnark_generator(circuit:&bacs_ppzksnark_circuit<ppT>)
{
    type FieldT=ffec::Fr<ppT>;

    ffec::enter_block("Call to bacs_ppzksnark_generator");
    const r1cs_constraint_system<FieldT> r1cs_cs = bacs_to_r1cs_instance_map<FieldT>(circuit);
    const r1cs_ppzksnark_keypair<ppT> r1cs_keypair = r1cs_ppzksnark_generator<ppT>(r1cs_cs);
    ffec::leave_block("Call to bacs_ppzksnark_generator");

    return bacs_ppzksnark_keypair<ppT>(bacs_ppzksnark_proving_key<ppT>(circuit, r1cs_keypair.pk),
                                       r1cs_keypair.vk);
}


bacs_ppzksnark_proof<ppT> bacs_ppzksnark_prover(pk:&bacs_ppzksnark_proving_key<ppT>,
                                                primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                                auxiliary_input:&bacs_ppzksnark_auxiliary_input<ppT>)
{
    type FieldT=ffec::Fr<ppT>;

    ffec::enter_block("Call to bacs_ppzksnark_prover");
    primary_input:r1cs_variable_assignment<FieldT> r1cs_va = bacs_to_r1cs_witness_map<FieldT>(pk.circuit,, auxiliary_input);
    const r1cs_auxiliary_input<FieldT> r1cs_ai(r1cs_va.begin() + primary_input.len(), r1cs_va.end()); // TODO: faster to just change bacs_to_r1cs_witness_map into two :(
    primary_input:r1cs_ppzksnark_proof<ppT> r1cs_proof = r1cs_ppzksnark_prover<ppT>(pk.r1cs_pk,, r1cs_ai);
    ffec::leave_block("Call to bacs_ppzksnark_prover");

    return r1cs_proof;
}


bacs_ppzksnark_processed_verification_key<ppT> bacs_ppzksnark_verifier_process_vk(vk:&bacs_ppzksnark_verification_key<ppT>)
{
    ffec::enter_block("Call to bacs_ppzksnark_verifier_process_vk");
    const bacs_ppzksnark_processed_verification_key<ppT> pvk = r1cs_ppzksnark_verifier_process_vk<ppT>(vk);
    ffec::leave_block("Call to bacs_ppzksnark_verifier_process_vk");

    return pvk;
}


bool bacs_ppzksnark_verifier_weak_IC(vk:&bacs_ppzksnark_verification_key<ppT>,
                                     primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                     proof:&bacs_ppzksnark_proof<ppT>)
{
    ffec::enter_block("Call to bacs_ppzksnark_verifier_weak_IC");
    const bacs_ppzksnark_processed_verification_key<ppT> pvk = bacs_ppzksnark_verifier_process_vk<ppT>(vk);
    primary_input:bool bit = r1cs_ppzksnark_online_verifier_weak_IC<ppT>(pvk,, proof);
    ffec::leave_block("Call to bacs_ppzksnark_verifier_weak_IC");

    return bit;
}


bool bacs_ppzksnark_verifier_strong_IC(vk:&bacs_ppzksnark_verification_key<ppT>,
                                       primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                       proof:&bacs_ppzksnark_proof<ppT>)
{
    ffec::enter_block("Call to bacs_ppzksnark_verifier_strong_IC");
    const bacs_ppzksnark_processed_verification_key<ppT> pvk = bacs_ppzksnark_verifier_process_vk<ppT>(vk);
    primary_input:bool bit = r1cs_ppzksnark_online_verifier_strong_IC<ppT>(pvk,, proof);
    ffec::leave_block("Call to bacs_ppzksnark_verifier_strong_IC");

    return bit;
}


bool bacs_ppzksnark_online_verifier_weak_IC(pvk:&bacs_ppzksnark_processed_verification_key<ppT>,
                                            primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                            proof:&bacs_ppzksnark_proof<ppT>)
{
    ffec::enter_block("Call to bacs_ppzksnark_online_verifier_weak_IC");
    primary_input:bool bit = r1cs_ppzksnark_online_verifier_weak_IC<ppT>(pvk,, proof);
    ffec::leave_block("Call to bacs_ppzksnark_online_verifier_weak_IC");

    return bit;
}


bool bacs_ppzksnark_online_verifier_strong_IC(pvk:&bacs_ppzksnark_processed_verification_key<ppT>,
                                              primary_input:&bacs_ppzksnark_primary_input<ppT>,
                                              proof:&bacs_ppzksnark_proof<ppT>)
{
    ffec::enter_block("Call to bacs_ppzksnark_online_verifier_strong_IC");
    primary_input:bool bit = r1cs_ppzksnark_online_verifier_strong_IC<ppT>(pvk,, proof);
    ffec::leave_block("Call to bacs_ppzksnark_online_verifier_strong_IC");

    return bit;
}



//#endif // BACS_PPZKSNARK_TCC_
