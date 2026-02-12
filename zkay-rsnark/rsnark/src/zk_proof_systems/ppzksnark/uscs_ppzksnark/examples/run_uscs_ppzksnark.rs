//  Declaration of functionality that runs the USCS ppzkSNARK for
//  a given USCS example.

// use ff_curves::algebra::curves::public_params;

// use crate::relations::constraint_satisfaction_problems/uscs/examples/uscs_examples;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::uscs::examples::uscs_examples::uscs_example;
use crate::zk_proof_systems::ppzksnark::uscs_ppzksnark::uscs_ppzksnark::{
    uscs_ppzksnark_generator, uscs_ppzksnark_online_verifier_strong_IC,
    uscs_ppzksnark_processed_verification_key, uscs_ppzksnark_proof, uscs_ppzksnark_prover,
    uscs_ppzksnark_proving_key, uscs_ppzksnark_verification_key,
    uscs_ppzksnark_verifier_process_vk, uscs_ppzksnark_verifier_strong_IC,
};
use ff_curves::Fr;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use std::ops::Mul;

/**
 * Runs the ppzkSNARK (generator, prover, and verifier) for a given
 * USCS example (specified by a constraint system, input, and witness).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */

// bool run_uscs_ppzksnark(example:&uscs_example<Fr<ppT> >,
//                         test_serialization:bool);

// use common::profiling;

// use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/uscs_ppzksnark;

/**
 * The code below provides an example of all stages of running a USCS ppzkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the ppzkSNARK generator on input a given
 *     constraint system CS to create a proving and a verification key for CS.
 * (2) The "prover", which runs the ppzkSNARK prover on input the proving key,
 *     a primary input for CS, and an auxiliary input for CS.
 * (3) The "verifier", which runs the ppzkSNARK verifier on input the verification key,
 *     a primary input for CS, and a proof.
 */

pub fn run_uscs_ppzksnark<ppT: ppTConfig>(
    example: &uscs_example<Fr<ppT>, pb_variable, pb_linear_combination>,
    test_serialization: bool,
) -> bool {
    enter_block("Call to run_uscs_ppzksnark", false);

    println!("USCS ppzkSNARK Generator");
    let mut keypair = uscs_ppzksnark_generator::<ppT>(&example.constraint_system);
    print!("\n");
    print_indent();
    println!("after generator");

    println!("Preprocess verification key");
    let mut pvk = uscs_ppzksnark_verifier_process_vk::<ppT>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<uscs_ppzksnark_proving_key<ppT>>(&keypair.pk);
        keypair.vk = reserialize::<uscs_ppzksnark_verification_key<ppT>>(&keypair.vk);
        pvk = reserialize::<uscs_ppzksnark_processed_verification_key<ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    println!("USCS ppzkSNARK Prover");
    let mut proof = uscs_ppzksnark_prover::<ppT>(
        &keypair.pk,
        &example.primary_input,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<uscs_ppzksnark_proof<ppT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("USCS ppzkSNARK Verifier");
    let ans = uscs_ppzksnark_verifier_strong_IC::<ppT>(&keypair.vk, &example.primary_input, &proof);
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    println!("USCS ppzkSNARK Online Verifier");
    let ans2 =
        uscs_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &example.primary_input, &proof);
    assert!(ans == ans2);

    leave_block("Call to run_uscs_ppzksnark", false);

    ans
}
