/** @file
 *****************************************************************************

 Declaration of functionality that runs the BACS ppzkSNARK for
 a given BACS example.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_BACS_PPZKSNARK_HPP_
// #define RUN_BACS_PPZKSNARK_HPP_

use ffec::algebra::curves::public_params;

use crate::relations::circuit_satisfaction_problems/bacs/examples/bacs_examples;



/**
 * Runs the ppzkSNARK (generator, prover, and verifier) for a given
 * BACS example (specified by a circuit, primary input, and auxiliary input).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
template<typename ppT>
bool run_bacs_ppzksnark(const bacs_example<ffec::Fr<ppT> > &example,
                        const bool test_serialization);



use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/examples/run_bacs_ppzksnark;

//#endif // RUN_BACS_PPZKSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality that runs the BACS ppzkSNARK for
 a given BACS example.

 See run_bacs_ppzksnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_BACS_PPZKSNARK_TCC_
// #define RUN_BACS_PPZKSNARK_TCC_

use  <sstream>

use ffec::common::profiling;

use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/bacs_ppzksnark;



/**
 * The code below provides an example of all stages of running a BACS ppzkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the ppzkSNARK generator on input a given
 *     circuit C to create a proving and a verification key for C.
 * (2) The "prover", which runs the ppzkSNARK prover on input the proving key,
 *     a primary input for C, and an auxiliary input for C.
 * (3) The "verifier", which runs the ppzkSNARK verifier on input the verification key,
 *     a primary input for C, and a proof.
 */
template<typename ppT>
bool run_bacs_ppzksnark(const bacs_example<ffec::Fr<ppT> > &example,
                        const bool test_serialization)
{
    ffec::enter_block("Call to run_bacs_ppzksnark");

    ffec::print_header("BACS ppzkSNARK Generator");
    bacs_ppzksnark_keypair<ppT> keypair = bacs_ppzksnark_generator<ppT>(example.circuit);
    print!("\n"); ffec::print_indent(); ffec::print_mem("after generator");

    ffec::print_header("Preprocess verification key");
    bacs_ppzksnark_processed_verification_key<ppT> pvk = bacs_ppzksnark_verifier_process_vk<ppT>(keypair.vk);

    if test_serialization
    {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize<bacs_ppzksnark_proving_key<ppT> >(keypair.pk);
        keypair.vk = ffec::reserialize<bacs_ppzksnark_verification_key<ppT> >(keypair.vk);
        pvk = ffec::reserialize<bacs_ppzksnark_processed_verification_key<ppT> >(pvk);
        ffec::leave_block("Test serialization of keys");
    }

    ffec::print_header("BACS ppzkSNARK Prover");
    bacs_ppzksnark_proof<ppT> proof = bacs_ppzksnark_prover<ppT>(keypair.pk, example.primary_input, example.auxiliary_input);
    print!("\n"); ffec::print_indent(); ffec::print_mem("after prover");

    if test_serialization
    {
        ffec::enter_block("Test serialization of proof");
        proof = ffec::reserialize<bacs_ppzksnark_proof<ppT> >(proof);
        ffec::leave_block("Test serialization of proof");
    }

    ffec::print_header("BACS ppzkSNARK Verifier");
    bool ans = bacs_ppzksnark_verifier_strong_IC<ppT>(keypair.vk, example.primary_input, proof);
    print!("\n"); ffec::print_indent(); ffec::print_mem("after verifier");
    print!("* The verification result is: %s\n", (ans ? "PASS" : "FAIL"));

    ffec::print_header("BACS ppzkSNARK Online Verifier");
    bool ans2 = bacs_ppzksnark_online_verifier_strong_IC<ppT>(pvk, example.primary_input, proof);
    assert!(ans == ans2);

    ffec::leave_block("Call to run_bacs_ppzksnark");

    return ans;
}



//#endif // RUN_BACS_PPZKSNARK_TCC_
