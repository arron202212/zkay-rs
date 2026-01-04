// Declaration of functionality that runs the R1CS ppzkADSNARK for
// a given R1CS example.

use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::r1cs_example;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark::{
    r1cs_ppzkadsnark_auth_generator, r1cs_ppzkadsnark_auth_sign, r1cs_ppzkadsnark_auth_verify,
    r1cs_ppzkadsnark_auth_verify2, r1cs_ppzkadsnark_generator, r1cs_ppzkadsnark_online_verifier,
    r1cs_ppzkadsnark_online_verifier2, r1cs_ppzkadsnark_processed_verification_key,
    r1cs_ppzkadsnark_proof, r1cs_ppzkadsnark_prover, r1cs_ppzkadsnark_proving_key,
    r1cs_ppzkadsnark_verification_key, r1cs_ppzkadsnark_verifier,
    r1cs_ppzkadsnark_verifier_process_vk, r1cs_ppzkadsnark_verifier2,
};
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    labelT, r1cs_ppzkadsnark_ppTConfig, snark_pp,
};
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_prf::PrfConfig;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_signature::{
    SigConfig, kpT,
};
use ff_curves::PublicParams;
use ff_curves::{Fr, G2};
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::ops::{Add, Mul};

/**
 * Runs the ppzkADSNARK (generator, prover, and verifier) for a given
 * R1CS example (specified by a constraint system, input, and witness).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
//
// bool run_r1cs_ppzkadsnark(example:r1cs_example<Fr<snark_pp<ppT>> >,
//                           test_serialization:bool);
// use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::run_r1cs_ppzkadsnark;

// use ffec::common::profiling;

// use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::prf::aes_ctr_prf;
// use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::signature::ed25519_signature;
// use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark;

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
//
pub fn run_r1cs_ppzkadsnark<
    ppT: r1cs_ppzkadsnark_ppTConfig,
    Sig: SigConfig<ppT>,
    Prf: PrfConfig<ppT>,
    const NN: usize,
    FieldT: FieldTConfig,
    ED: evaluation_domain<FieldT>,
>(
    example: r1cs_example<Fr<snark_pp<ppT>>, pb_variable, pb_linear_combination>,
    test_serialization: bool,
) -> bool
where
    ED: evaluation_domain<<<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as PublicParams>::Fr>,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1: Add<
        Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2: Add<
        Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
    >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2: Mul<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2: FieldTConfig,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2: Mul<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2: Add<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
        Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
    >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    for<'a> <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            &'a FieldT,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Add<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
        Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
    >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            knowledge_commitment<
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
            Output = knowledge_commitment<
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            knowledge_commitment<
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
            Output = knowledge_commitment<
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G1,
        >,
    for<'a> <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
            &'a FieldT,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Add<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::G2,
        >,
    for<'a> &'a <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr: Add<
            <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
            Output = <<ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp as ff_curves::PublicParams>::Fr,
        >,
{
    enter_block("Call to run_r1cs_ppzkadsnark", false);

    let auth_keys = r1cs_ppzkadsnark_auth_generator::<ppT, Sig, Prf, NN, FieldT, ED>();

    println!("R1CS ppzkADSNARK Generator");
    let mut keypair = r1cs_ppzkadsnark_generator::<ppT, NN, FieldT, ED>(
        &example.constraint_system,
        &auth_keys.pap,
    );
    print!("\n");
    print_indent();
    println!("after generator");

    println!("Preprocess verification key");
    let mut pvk = r1cs_ppzkadsnark_verifier_process_vk::<ppT, NN, FieldT, ED>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<r1cs_ppzkadsnark_proving_key<ppT>>(&keypair.pk);
        keypair.vk = reserialize::<r1cs_ppzkadsnark_verification_key<ppT>>(&keypair.vk);
        pvk = reserialize::<r1cs_ppzkadsnark_processed_verification_key<ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    println!("R1CS ppzkADSNARK Authenticate");
    let mut data = Vec::with_capacity(example.constraint_system.num_inputs());
    let mut labels = Vec::with_capacity(example.constraint_system.num_inputs());
    for i in 0..example.constraint_system.num_inputs() {
        labels.push(labelT::default());
        data.push(example.primary_input[i].clone());
    }
    let mut auth_data =
        r1cs_ppzkadsnark_auth_sign::<ppT, Sig, Prf, NN, FieldT, ED>(&data, &auth_keys.sak, &labels);

    println!("R1CS ppzkADSNARK Verify Symmetric");
    let auth_res = r1cs_ppzkadsnark_auth_verify::<ppT, Prf, NN, FieldT, ED>(
        &data,
        &auth_data,
        &auth_keys.sak,
        &labels,
    );
    print!(
        "* The verification result is: {}\n",
        (if auth_res { "PASS" } else { "FAIL" })
    );

    println!("R1CS ppzkADSNARK Verify Public");
    let auth_resp = r1cs_ppzkadsnark_auth_verify2::<ppT, Sig, NN, FieldT, ED>(
        &data,
        &auth_data,
        &auth_keys.pak,
        &labels,
    );
    assert!(auth_res == auth_resp);

    println!("R1CS ppzkADSNARK Prover");
    let mut proof = r1cs_ppzkadsnark_prover::<ppT, NN, FieldT, ED>(
        &keypair.pk,
        &example.primary_input,
        &example.auxiliary_input,
        &auth_data,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<r1cs_ppzkadsnark_proof<ppT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("R1CS ppzkADSNARK Symmetric Verifier");
    let mut ans = r1cs_ppzkadsnark_verifier::<ppT, Prf, NN, FieldT, ED>(
        &keypair.vk,
        &proof,
        &auth_keys.sak,
        &labels,
    );
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        (if ans { "PASS" } else { "FAIL" })
    );

    println!("R1CS ppzkADSNARK Symmetric Online Verifier");
    let mut ans2 = r1cs_ppzkadsnark_online_verifier::<ppT, Prf, NN, FieldT, ED>(
        &pvk,
        &proof,
        &auth_keys.sak,
        &labels,
    );
    assert!(ans == ans2);

    println!("R1CS ppzkADSNARK Public Verifier");
    ans = r1cs_ppzkadsnark_verifier2::<ppT, Sig, NN, FieldT, ED>(
        &keypair.vk,
        &auth_data,
        &proof,
        &auth_keys.pak,
        &labels,
    );
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        (if ans { "PASS" } else { "FAIL" })
    );

    println!("R1CS ppzkADSNARK Public Online Verifier");
    ans2 = r1cs_ppzkadsnark_online_verifier2::<ppT, Sig, NN, FieldT, ED>(
        &pvk,
        &auth_data,
        &proof,
        &auth_keys.pak,
        &labels,
    );
    assert!(ans == ans2);

    leave_block("Call to run_r1cs_ppzkadsnark", false);

    ans
}
