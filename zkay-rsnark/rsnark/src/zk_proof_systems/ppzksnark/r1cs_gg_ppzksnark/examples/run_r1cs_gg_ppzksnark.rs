/** @file
 *****************************************************************************

 Declaration of functionality that runs the R1CS GG-ppzkSNARK for
 a given R1CS example.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_GG_PPZKSNARK_HPP_
// #define RUN_R1CS_GG_PPZKSNARK_HPP_

use ffec::algebra::curves::public_params;

use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;



/**
 * Runs the ppzkSNARK (generator, prover, and verifier) for a given
 * R1CS example (specified by a constraint system, input, and witness).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
// template<typename ppT>
// bool run_r1cs_gg_ppzksnark(const r1cs_example<ffec::Fr<ppT> > &example,
//                         const bool test_serialization);



// use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples/run_r1cs_gg_ppzksnark;

//#endif // RUN_R1CS_GG_PPZKSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality that runs the R1CS GG-ppzkSNARK for
 a given R1CS example.

 See run_r1cs_gg_ppzksnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_GG_PPZKSNARK_TCC_
// #define RUN_R1CS_GG_PPZKSNARK_TCC_

// use  <sstream>
// use  <type_traits>

use ffec::common::profiling;

use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark;



// template<typename ppT>
// typename std::enable_if<ppT::has_affine_pairing, void>::type
// test_affine_verifier(const r1cs_gg_ppzksnark_verification_key<ppT> &vk,
//                      const r1cs_gg_ppzksnark_primary_input<ppT> &primary_input,
//                      const r1cs_gg_ppzksnark_proof<ppT> &proof,
//                      const bool expected_answer)
// {
//     ffec::print_header("R1CS GG-ppzkSNARK Affine Verifier");
//     const bool answer = r1cs_gg_ppzksnark_affine_verifier_weak_IC<ppT>(vk, primary_input, proof);
//     assert!(answer == expected_answer);
// }

// template<typename ppT>
// typename std::enable_if<!ppT::has_affine_pairing, void>::type
// test_affine_verifier(const r1cs_gg_ppzksnark_verification_key<ppT> &vk,
//                      const r1cs_gg_ppzksnark_primary_input<ppT> &primary_input,
//                      const r1cs_gg_ppzksnark_proof<ppT> &proof,
//                      const bool expected_answer)
// {
//     ffec::print_header("R1CS GG-ppzkSNARK Affine Verifier");
//     ffec::UNUSED(vk, primary_input, proof, expected_answer);
//     print!("Affine verifier is not supported; not testing anything.\n");
// }

// /**
//  * The code below provides an example of all stages of running a R1CS GG-ppzkSNARK.
//  *
//  * Of course, in a real-life scenario, we would have three distinct entities,
//  * mangled into one in the demonstration below. The three entities are as follows.
//  * (1) The "generator", which runs the ppzkSNARK generator on input a given
//  *     constraint system CS to create a proving and a verification key for CS.
//  * (2) The "prover", which runs the ppzkSNARK prover on input the proving key,
//  *     a primary input for CS, and an auxiliary input for CS.
//  * (3) The "verifier", which runs the ppzkSNARK verifier on input the verification key,
//  *     a primary input for CS, and a proof.
//  */
// template<typename ppT>
// bool run_r1cs_gg_ppzksnark(const r1cs_example<ffec::Fr<ppT> > &example,
//                         const bool test_serialization)
// {
//     ffec::enter_block("Call to run_r1cs_gg_ppzksnark");

//     ffec::print_header("R1CS GG-ppzkSNARK Generator");
//     r1cs_gg_ppzksnark_keypair<ppT> keypair = r1cs_gg_ppzksnark_generator<ppT>(example.constraint_system);
//     print!("\n"); ffec::print_indent(); ffec::print_mem("after generator");

//     ffec::print_header("Preprocess verification key");
//     r1cs_gg_ppzksnark_processed_verification_key<ppT> pvk = r1cs_gg_ppzksnark_verifier_process_vk<ppT>(keypair.vk);

//     if test_serialization
//     {
//         ffec::enter_block("Test serialization of keys");
//         keypair.pk = ffec::reserialize<r1cs_gg_ppzksnark_proving_key<ppT> >(keypair.pk);
//         keypair.vk = ffec::reserialize<r1cs_gg_ppzksnark_verification_key<ppT> >(keypair.vk);
//         pvk = ffec::reserialize<r1cs_gg_ppzksnark_processed_verification_key<ppT> >(pvk);
//         ffec::leave_block("Test serialization of keys");
//     }

//     ffec::print_header("R1CS GG-ppzkSNARK Prover");
//     r1cs_gg_ppzksnark_proof<ppT> proof = r1cs_gg_ppzksnark_prover<ppT>(keypair.pk, example.primary_input, example.auxiliary_input);
//     print!("\n"); ffec::print_indent(); ffec::print_mem("after prover");

//     if test_serialization
//     {
//         ffec::enter_block("Test serialization of proof");
//         proof = ffec::reserialize<r1cs_gg_ppzksnark_proof<ppT> >(proof);
//         ffec::leave_block("Test serialization of proof");
//     }

//     ffec::print_header("R1CS GG-ppzkSNARK Verifier");
//     const bool ans = r1cs_gg_ppzksnark_verifier_strong_IC<ppT>(keypair.vk, example.primary_input, proof);
//     print!("\n"); ffec::print_indent(); ffec::print_mem("after verifier");
//     print!("* The verification result is: %s\n", if ans {"PASS"} else{"FAIL"});

//     ffec::print_header("R1CS GG-ppzkSNARK Online Verifier");
//     const bool ans2 = r1cs_gg_ppzksnark_online_verifier_strong_IC<ppT>(pvk, example.primary_input, proof);
//     assert!(ans == ans2);

//     test_affine_verifier<ppT>(keypair.vk, example.primary_input, proof, ans);

//     ffec::leave_block("Call to run_r1cs_gg_ppzksnark");

//     return ans;
// }



//#endif // RUN_R1CS_GG_PPZKSNARK_TCC_
