//  Declaration of functionality that runs the TBCS ppzkSNARK for
//  a given TBCS example.

// use crate::relations::circuit_satisfaction_problems/tbcs/examples/tbcs_examples;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::circuit_satisfaction_problems::tbcs::examples::tbcs_examples::tbcs_example;
use crate::zk_proof_systems::ppzksnark::tbcs_ppzksnark::tbcs_ppzksnark::{
    tbcs_ppzksnark_generator, tbcs_ppzksnark_online_verifier_strong_IC,
    tbcs_ppzksnark_processed_verification_key, tbcs_ppzksnark_proof, tbcs_ppzksnark_prover,
    tbcs_ppzksnark_proving_key, tbcs_ppzksnark_verification_key,
    tbcs_ppzksnark_verifier_process_vk, tbcs_ppzksnark_verifier_strong_IC,
};
use ff_curves::Fr;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use std::ops::Mul;

/**
 * Runs the ppzkSNARK (generator, prover, and verifier) for a given
 * TBCS example (specified by a circuit, primary input, and auxiliary input).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */

// bool run_tbcs_ppzksnark(example:&tbcs_example,
//                         test_serialization:bool);

// use common::profiling;

// use libsnark/zk_proof_systems/ppzksnark/tbcs_ppzksnark/tbcs_ppzksnark;

/**
 * The code below provides an example of all stages of running a TBCS ppzkSNARK.
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

pub fn run_tbcs_ppzksnark<ppT: ppTConfig>(example: &tbcs_example, test_serialization: bool) -> bool
where
    knowledge_commitment<
        <ppT as ff_curves::PublicParams>::G1,
        <ppT as ff_curves::PublicParams>::G1,
    >: Mul<
            <ppT as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <ppT as ff_curves::PublicParams>::G1,
                <ppT as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <ppT as ff_curves::PublicParams>::G2,
        <ppT as ff_curves::PublicParams>::G1,
    >: Mul<
            <ppT as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <ppT as ff_curves::PublicParams>::G2,
                <ppT as ff_curves::PublicParams>::G1,
            >,
        >,
{
    enter_block("Call to run_tbcs_ppzksnark", false);

    println!("TBCS ppzkSNARK Generator");
    let mut keypair = tbcs_ppzksnark_generator::<ppT>(&example.circuit);
    print!("\n");
    print_indent();
    println!("after generator");

    println!("Preprocess verification key");
    let mut pvk = tbcs_ppzksnark_verifier_process_vk::<ppT>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<tbcs_ppzksnark_proving_key<ppT>>(&keypair.pk);
        keypair.vk = reserialize::<tbcs_ppzksnark_verification_key<ppT>>(&keypair.vk);
        pvk = reserialize::<tbcs_ppzksnark_processed_verification_key<ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    println!("TBCS ppzkSNARK Prover");
    let mut proof = tbcs_ppzksnark_prover::<ppT>(
        &keypair.pk,
        &example.primary_input,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<tbcs_ppzksnark_proof<ppT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("TBCS ppzkSNARK Verifier");
    let ans = tbcs_ppzksnark_verifier_strong_IC::<ppT>(&keypair.vk, &example.primary_input, &proof);
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    println!("TBCS ppzkSNARK Online Verifier");
    let ans2 =
        tbcs_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &example.primary_input, &proof);
    assert!(ans == ans2);

    leave_block("Call to run_tbcs_ppzksnark", false);

    ans
}
