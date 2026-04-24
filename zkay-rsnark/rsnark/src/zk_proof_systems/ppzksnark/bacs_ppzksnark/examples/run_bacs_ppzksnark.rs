//  Declaration of functionality that runs the BACS ppzkSNARK for
//  a given BACS example.

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::circuit_satisfaction_problems::bacs::examples::bacs_examples::bacs_example;
use crate::zk_proof_systems::ppzksnark::bacs_ppzksnark::bacs_ppzksnark::{
    bacs_ppzksnark_generator, bacs_ppzksnark_online_verifier_strong_IC,
    bacs_ppzksnark_processed_verification_key, bacs_ppzksnark_proof, bacs_ppzksnark_prover,
    bacs_ppzksnark_proving_key, bacs_ppzksnark_verification_key,
    bacs_ppzksnark_verifier_process_vk, bacs_ppzksnark_verifier_strong_IC,
};
use ff_curves::Fr;
use ffec::common::profiling::print_indent;
use ffec::common::serialization::reserialize;
use std::ops::Mul;
use tracing::{Level, span};

// /**
//  * The code below provides an example of all stages of running a BACS ppzkSNARK.
//  *
//  * Of course, in a real-life scenario, we would have three distinct entities,
//  * mangled into one in the demonstration below. The three entities are as follows.
//  * (1) The "generator", which runs the ppzkSNARK generator on input a given
//  *     circuit C to create a proving and a verification key for C.
//  * (2) The "prover", which runs the ppzkSNARK prover on input the proving key,
//  *     a primary input for C, and an auxiliary input for C.
//  * (3) The "verifier", which runs the ppzkSNARK verifier on input the verification key,
//  *     a primary input for C, and a proof.
//  */
// /**
//  * Runs the ppzkSNARK (generator, prover, and verifier) for a given
//  * BACS example (specified by a circuit, primary input, and auxiliary input).
//  *
//  * Optionally, also test the serialization routines for keys and proofs.
//  * (This takes additional time.)
//  */
pub fn run_bacs_ppzksnark<ppT: ppTConfig>(
    example: &bacs_example<Fr<ppT>, pb_variable, pb_linear_combination>,
    test_serialization: bool,
) -> bool
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
    let span = span!(Level::TRACE, "Call to run_bacs_ppzksnark").entered();

    println!("BACS ppzkSNARK Generator");
    let mut keypair = bacs_ppzksnark_generator::<ppT>(&example.circuit);
    print!("\n");
    print_indent();
    println!("after generator");

    println!("Preprocess verification key");
    let mut pvk = bacs_ppzksnark_verifier_process_vk::<ppT>(&keypair.vk);

    if test_serialization {
        let span = span!(Level::TRACE, "Test serialization of keys").entered();
        keypair.pk = reserialize::<bacs_ppzksnark_proving_key<ppT>>(&keypair.pk);
        keypair.vk = reserialize::<bacs_ppzksnark_verification_key<ppT>>(&keypair.vk);
        pvk = reserialize::<bacs_ppzksnark_processed_verification_key<ppT>>(&pvk);
        span.exit();
    }

    println!("BACS ppzkSNARK Prover");
    let mut proof = bacs_ppzksnark_prover::<ppT>(
        &keypair.pk,
        &example.primary_input,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        let span = span!(Level::TRACE, "Test serialization of proof").entered();
        proof = reserialize::<bacs_ppzksnark_proof<ppT>>(&proof);
        span.exit();
    }

    println!("BACS ppzkSNARK Verifier");
    let ans = bacs_ppzksnark_verifier_strong_IC::<ppT>(&keypair.vk, &example.primary_input, &proof);
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    println!("BACS ppzkSNARK Online Verifier");
    let ans2 =
        bacs_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &example.primary_input, &proof);
    assert!(ans == ans2);

    span.exit();

    ans
}
