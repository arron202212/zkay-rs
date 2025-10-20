/** @file
 *****************************************************************************

 Declaration of functionality that runs the R1CS ppzkADSNARK for
 a given R1CS example.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_PPZKADSNARK_HPP_
// #define RUN_R1CS_PPZKADSNARK_HPP_

use algebra::curves::public_params;

use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
use libsnark::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params;



/**
 * Runs the ppzkADSNARK (generator, prover, and verifier) for a given
 * R1CS example (specified by a constraint system, input, and witness).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
// template<typename ppT>
// bool run_r1cs_ppzkadsnark(example:r1cs_example<Fr<snark_pp<ppT>> >,
//                           test_serialization:bool);



use libsnark::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::run_r1cs_ppzkadsnark;

//#endif // RUN_R1CS_PPZKADSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality that runs the R1CS ppzkADSNARK for
 a given R1CS example.

 See run_r1cs_ppzkadsnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_PPZKADSNARK_TCC_
// #define RUN_R1CS_PPZKADSNARK_TCC_

// use  <sstream>
// use  <type_traits>

use common::profiling;

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::prf::aes_ctr_prf;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::signature::ed25519_signature;
use libsnark::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark;



/**
 * The code below provides an example of all stages of running a R1CS ppzkADSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the ppzkADSNARK generator on input a given
 *     constraint system CS to create a proving and a verification key for CS.
 * (2) The "prover", which runs the ppzkADSNARK prover on input the proving key,
 *     a primary input for CS, and an auxiliary input for CS.
 * (3) The "verifier", which runs the ppzkADSNARK verifier on input the verification key,
 *     a primary input for CS, and a proof.
 */
// template<typename ppT>
 pub fn run_r1cs_ppzkadsnark<ppT>(example:r1cs_example<Fr<snark_pp<ppT>> >,
                          test_serialization:bool)->bool
{
    enter_block("Call to run_r1cs_ppzkadsnark",false);

    let  auth_keys = r1cs_ppzkadsnark_auth_generator::<ppT>();

    print_header("R1CS ppzkADSNARK Generator");
    let keypair = r1cs_ppzkadsnark_generator::<ppT>(example.constraint_system,auth_keys.pap);
    print!("\n"); print_indent(); print_mem("after generator");

    print_header("Preprocess verification key");
    let  pvk = r1cs_ppzkadsnark_verifier_process_vk::<ppT>(keypair.vk);

    if test_serialization
    {
        enter_block("Test serialization of keys",false);
        keypair.pk = reserialize::<r1cs_ppzkadsnark_proving_key::<ppT> >(keypair.pk);
        keypair.vk = reserialize::<r1cs_ppzkadsnark_verification_key::<ppT> >(keypair.vk);
        pvk = reserialize::<r1cs_ppzkadsnark_processed_verification_key::<ppT> >(pvk);
        leave_block("Test serialization of keys",false);
    }

    print_header("R1CS ppzkADSNARK Authenticate");
    let mut  data=Vec::with_capacity(example.constraint_system.num_inputs());
    let mut  labels=Vec::with_capacity(example.constraint_system.num_inputs());
    for i in 0..example.constraint_system.num_inputs() {
        labels.push(labelT());
        data.push(example.primary_input[i]);
    }
    let mut  auth_data =
        r1cs_ppzkadsnark_auth_sign::<ppT>(data,auth_keys.sak,labels);

    print_header("R1CS ppzkADSNARK Verify Symmetric");
    let  auth_res =
        r1cs_ppzkadsnark_auth_verify::<ppT>(data,auth_data,auth_keys.sak,labels);
    print!("* The verification result is: {}\n", (if auth_res  {"PASS"} else{ "FAIL"}));

    print_header("R1CS ppzkADSNARK Verify Public");
    let  auth_resp =
        r1cs_ppzkadsnark_auth_verify::<ppT>(data,auth_data,auth_keys.pak,labels);
    assert! (auth_res == auth_resp);

    print_header("R1CS ppzkADSNARK Prover");
    let mut proof = r1cs_ppzkadsnark_prover::<ppT>(keypair.pk, example.primary_input, example.auxiliary_input,auth_data);
    print!("\n"); print_indent(); print_mem("after prover");

    if test_serialization
    {
        enter_block("Test serialization of proof",false);
        proof = reserialize::<r1cs_ppzkadsnark_proof::<ppT> >(proof);
        leave_block("Test serialization of proof",false);
    }

    print_header("R1CS ppzkADSNARK Symmetric Verifier");
    let  mut ans = r1cs_ppzkadsnark_verifier::<ppT>(keypair.vk, proof,auth_keys.sak,labels);
    print!("\n"); print_indent(); print_mem("after verifier");
    print!("* The verification result is: {}\n", (if ans { "PASS"} else{ "FAIL"}));

    print_header("R1CS ppzkADSNARK Symmetric Online Verifier");
    let mut  ans2 = r1cs_ppzkadsnark_online_verifier::<ppT>(pvk, proof,auth_keys.sak,labels);
    assert!(ans == ans2);

    print_header("R1CS ppzkADSNARK Public Verifier");
    ans = r1cs_ppzkadsnark_verifier::<ppT>(keypair.vk, auth_data, proof,auth_keys.pak,labels);
    print!("\n"); print_indent(); print_mem("after verifier");
    print!("* The verification result is: {}\n", (if ans  {"PASS" }else {"FAIL"}));

    print_header("R1CS ppzkADSNARK Public Online Verifier");
    ans2 = r1cs_ppzkadsnark_online_verifier::<ppT>(pvk, auth_data, proof,auth_keys.pak,labels);
    assert!(ans == ans2);

    leave_block("Call to run_r1cs_ppzkadsnark",false);

    return ans;
}



//#endif // RUN_R1CS_PPZKADSNARK_TCC_
