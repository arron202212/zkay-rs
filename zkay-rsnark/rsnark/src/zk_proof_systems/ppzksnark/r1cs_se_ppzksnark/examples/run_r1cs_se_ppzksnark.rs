//  Declaration of functionality that runs the R1CS SEppzkSNARK for
//  a given R1CS example.

// use common::default_types::ec_pp;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::r1cs_example;
use crate::zk_proof_systems::PptConfig;
use crate::zk_proof_systems::ppzksnark::r1cs_se_ppzksnark::r1cs_se_ppzksnark::{
    r1cs_se_ppzksnark_generator, r1cs_se_ppzksnark_online_verifier_strong_IC,
    r1cs_se_ppzksnark_processed_verification_key, r1cs_se_ppzksnark_proof,
    r1cs_se_ppzksnark_prover, r1cs_se_ppzksnark_proving_key, r1cs_se_ppzksnark_verification_key,
    r1cs_se_ppzksnark_verifier_process_vk, r1cs_se_ppzksnark_verifier_strong_IC,
};
use ff_curves::Fr;
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::ops::{Add, Mul};

/**
 * Runs the SEppzkSNARK (generator, prover, and verifier) for a given
 * R1CS example (specified by a constraint system, input, and witness).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */

// bool run_r1cs_se_ppzksnark(example:&r1cs_example<Fr<ppT> >,
//                            test_serialization:bool);

/**
 * The code below provides an example of all stages of running a R1CS SEppzkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the SEppzkSNARK generator on input a given
 *     constraint system CS to create a proving and a verification key for CS.
 * (2) The "prover", which runs the SEppzkSNARK prover on input the proving key,
 *     a primary input for CS, and an auxiliary input for CS.
 * (3) The "verifier", which runs the SEppzkSNARK verifier on input the verification key,
 *     a primary input for CS, and a proof.
 */

pub fn run_r1cs_se_ppzksnark<
    ppT: PptConfig<Fr = FieldT>,
    const NN: usize,
    FieldT: FieldTConfig,
    ED: evaluation_domain<FieldT>,
>(
    example: &r1cs_example<Fr<ppT>, pb_variable, pb_linear_combination>,
    test_serialization: bool,
) -> bool
where
    for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
        Add<Output = <ppT as ff_curves::PublicParams>::G1>,
    for<'a> &'a <ppT as ff_curves::PublicParams>::G2:
        Add<Output = <ppT as ff_curves::PublicParams>::G2>,
    <ppT as ff_curves::PublicParams>::Fr:
        Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
    <ppT as ff_curves::PublicParams>::Fr:
        Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
{
    enter_block("Call to run_r1cs_se_ppzksnark", false);

    println!("R1CS SEppzkSNARK Generator");
    let mut keypair =
        r1cs_se_ppzksnark_generator::<ppT, NN, FieldT, ED>(&example.constraint_system);
    print!("\n");
    print_indent();
    println!("after generator");

    println!("Preprocess verification key");
    let mut pvk = r1cs_se_ppzksnark_verifier_process_vk::<ppT, NN, FieldT, ED>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<r1cs_se_ppzksnark_proving_key<ppT>>(&keypair.pk);
        keypair.vk = reserialize::<r1cs_se_ppzksnark_verification_key<ppT>>(&keypair.vk);
        pvk = reserialize::<r1cs_se_ppzksnark_processed_verification_key<ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    println!("R1CS SEppzkSNARK Prover");
    let mut proof = r1cs_se_ppzksnark_prover::<ppT, NN, FieldT, ED>(
        &keypair.pk,
        &example.primary_input,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<r1cs_se_ppzksnark_proof<ppT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("R1CS SEppzkSNARK Verifier");
    let mut ans = r1cs_se_ppzksnark_verifier_strong_IC::<ppT, NN, FieldT, ED>(
        &keypair.vk,
        &example.primary_input,
        &proof,
    );
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    println!("R1CS SEppzkSNARK Online Verifier");
    let mut ans2 = r1cs_se_ppzksnark_online_verifier_strong_IC::<ppT, NN, FieldT, ED>(
        &pvk,
        &example.primary_input,
        &proof,
    );
    assert!(ans == ans2);

    leave_block("Call to run_r1cs_se_ppzksnark", false);

    ans
}
