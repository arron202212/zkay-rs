// Declaration of functionality that runs the R1CS GG-ppzkSNARK for
// a given R1CS example.
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::r1cs_example;
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark::{
    r1cs_gg_ppzksnark_affine_verifier_weak_IC, r1cs_gg_ppzksnark_generator,
    r1cs_gg_ppzksnark_online_verifier_strong_IC, r1cs_gg_ppzksnark_processed_verification_key,
    r1cs_gg_ppzksnark_proof, r1cs_gg_ppzksnark_prover, r1cs_gg_ppzksnark_proving_key,
    r1cs_gg_ppzksnark_verification_key, r1cs_gg_ppzksnark_verifier_process_vk,
    r1cs_gg_ppzksnark_verifier_strong_IC,
};
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark_params::r1cs_gg_ppzksnark_primary_input;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::ops::{Add, Mul};

pub trait TestAffineVerifier {
    fn test_affine_verifier<ppT: PublicParams>(
        vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
        primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
        proof: &r1cs_gg_ppzksnark_proof<ppT>,
        expected_answer: bool,
    );
}
pub struct TestAffineVerifiers<const HAS_AFFINE_PAIRING: bool>;
impl TestAffineVerifier for TestAffineVerifiers<true> {
    // std::enable_if<ppT::has_affine_pairing, pub fn >::type
    fn test_affine_verifier<ppT: PublicParams>(
        vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
        primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
        proof: &r1cs_gg_ppzksnark_proof<ppT>,
        expected_answer: bool,
    ) {
        println!("R1CS GG-ppzkSNARK Affine Verifier");
        let answer = r1cs_gg_ppzksnark_affine_verifier_weak_IC::<ppT>(&vk, &primary_input, &proof);
        assert!(answer == expected_answer);
    }
}
impl TestAffineVerifier for TestAffineVerifiers<false> {
    // std::enable_if<!ppT::has_affine_pairing, pub fn >::type
    fn test_affine_verifier<ppT: PublicParams>(
        vk: &r1cs_gg_ppzksnark_verification_key<ppT>,
        primary_input: &r1cs_gg_ppzksnark_primary_input<ppT>,
        proof: &r1cs_gg_ppzksnark_proof<ppT>,
        expected_answer: bool,
    ) {
        println!("R1CS GG-ppzkSNARK Affine Verifier");
        //UNUSED(vk, primary_input, proof, expected_answer);
        print!("Affine verifier is not supported; not testing anything.\n");
    }
}

/**
 * The code below provides an example of all stages of running a R1CS GG-ppzkSNARK.
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

pub fn run_r1cs_gg_ppzksnark<ppT: PublicParams>(
    example: &r1cs_example<Fr<ppT>, pb_variable, pb_linear_combination>,
    test_serialization: bool,
) -> bool
// where
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G1:
//         Add<Output = <ppT as ff_curves::PublicParams>::G1>,
//     for<'a> &'a <ppT as ff_curves::PublicParams>::G2:
//         Add<Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G1, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::Fr:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::G1:
//         Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::G1>,
//     <ppT as ff_curves::PublicParams>::G1:
//         Mul<<ppT as ff_curves::PublicParams>::G2, Output = <ppT as ff_curves::PublicParams>::G2>,
//     <ppT as ff_curves::PublicParams>::G2:
//         Mul<<ppT as ff_curves::PublicParams>::Fr, Output = <ppT as ff_curves::PublicParams>::G2>,
//     ED: fqfft::evaluation_domain::evaluation_domain::evaluation_domain<
//             <ppT as ff_curves::PublicParams>::Fr,
//         >,
{
    enter_block("Call to run_r1cs_gg_ppzksnark", false);

    println!("R1CS GG-ppzkSNARK Generator");
    let mut keypair = r1cs_gg_ppzksnark_generator::<ppT>(&example.constraint_system);
    print!("\n");
    print_indent();
    println!("after generator");

    println!("Preprocess verification key");
    let mut pvk = r1cs_gg_ppzksnark_verifier_process_vk::<ppT>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<r1cs_gg_ppzksnark_proving_key<ppT>>(&keypair.pk);
        keypair.vk = reserialize::<r1cs_gg_ppzksnark_verification_key<ppT>>(&keypair.vk);
        pvk = reserialize::<r1cs_gg_ppzksnark_processed_verification_key<ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    println!("R1CS GG-ppzkSNARK Prover");
    let mut proof = r1cs_gg_ppzksnark_prover::<ppT>(
        &keypair.pk,
        &example.primary_input,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<r1cs_gg_ppzksnark_proof<ppT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("R1CS GG-ppzkSNARK Verifier");
    let mut ans =
        r1cs_gg_ppzksnark_verifier_strong_IC::<ppT>(&keypair.vk, &example.primary_input, &proof);
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    println!("R1CS GG-ppzkSNARK Online Verifier");
    let mut ans2 =
        r1cs_gg_ppzksnark_online_verifier_strong_IC::<ppT>(&pvk, &example.primary_input, &proof);
    assert!(ans == ans2);
    if ppT::has_affine_pairing {
        TestAffineVerifiers::<true>::test_affine_verifier::<ppT>(
            &keypair.vk,
            &example.primary_input,
            &proof,
            ans,
        );
    } else {
        TestAffineVerifiers::<false>::test_affine_verifier::<ppT>(
            &keypair.vk,
            &example.primary_input,
            &proof,
            ans,
        );
    }

    leave_block("Call to run_r1cs_gg_ppzksnark", false);

    ans
}
