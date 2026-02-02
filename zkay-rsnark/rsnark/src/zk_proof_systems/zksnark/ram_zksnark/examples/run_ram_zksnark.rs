// Declaration of functionality that runs the RAM zkSNARK for
// a given RAM example.

use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::rams::examples::ram_examples;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_instruction;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::CPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark::ram_examples::ram_example;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark::ram_zksnark_generator;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark::{
    ram_zksnark_proof, ram_zksnark_prover, ram_zksnark_proving_key, ram_zksnark_verification_key,
    ram_zksnark_verifier,
};
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::RamConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::ram_zksnark_machine_pp;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use std::ops::Mul;
// use common::profiling;

// use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark;

/**
 * Runs the zkSNARK (generator, prover, and verifier) for a given
 * RAM example (specified by an architecture, boot trace, auxiliary input, and time bound).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
//
// bool run_ram_zksnark(example:&ram_example<ram_zksnark_machine_pp<RamT> >,
//                      test_serialization:bool);

/**
 * The code below provides an example of all stages of running a RAM zkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the zkSNARK generator on input a given
 *     architecture.
 * (2) The "prover", which runs the zkSNARK prover on input the proving key,
 *     a boot trace, and an auxiliary input.
 * (3) The "verifier", which runs the zkSNARK verifier on input the verification key,
 *     a boot trace, a time bound, and a proof.
 */
//
pub fn run_ram_zksnark<RamT: RamConfig>(
    example: &ram_example<ram_zksnark_machine_pp<RamT>>,
    test_serialization: bool,
) -> bool
where
    <RamT as RamConfig>::machine_pp: CPHConfig,
    knowledge_commitment<
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =  knowledge_commitment<
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =knowledge_commitment<
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
        <<RamT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<RamT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
        >,
{
    enter_block("Call to run_ram_zksnark", false);

    print!("This run uses an example with the following parameters:\n");
    example.ap.print();
    print!("* Time bound (T): {}\n", example.time_bound);

    println!("RAM zkSNARK Generator");
    let mut keypair = ram_zksnark_generator::<RamT>(&example.ap);
    print!("\n");
    print_indent();
    println!("after generator");

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<ram_zksnark_proving_key<RamT>>(&keypair.pk);
        keypair.vk = reserialize::<ram_zksnark_verification_key<RamT>>(&keypair.vk);
        leave_block("Test serialization of keys", false);
    }

    println!("RAM zkSNARK Prover");
    let mut proof = ram_zksnark_prover::<RamT>(
        &keypair.pk,
        &example.boot_trace,
        example.time_bound,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<ram_zksnark_proof<RamT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("RAM zkSNARK Verifier");
    let ans =
        ram_zksnark_verifier::<RamT>(&keypair.vk, &example.boot_trace, example.time_bound, &proof);
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    leave_block("Call to run_ram_zksnark", false);

    return ans;
}
