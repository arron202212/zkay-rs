// Declaration of functionality that runs the RAM ppzkSNARK for
// a given RAM example.

// use crate::relations::ram_computations::rams::examples::ram_examples;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::rams::examples::ram_examples::ram_example;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::{
    ram_ppzksnark_generator, ram_ppzksnark_proof, ram_ppzksnark_prover, ram_ppzksnark_proving_key,
    ram_ppzksnark_verification_key, ram_ppzksnark_verifier,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::ram_ppzksnark_machine_pp;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::RamConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::serialization::reserialize;
use ffec::log2;
use std::ops::Mul;

/**
 * Runs the ppzkSNARK (generator, prover, and verifier) for a given
 * RAM example (specified by an architecture, boot trace, auxiliary input, and time bound).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
//
// bool run_ram_ppzksnark(example:&ram_example<ram_ppzksnark_machine_pp<ram_ppzksnark_ppT> >,
//                        test_serialization:bool);

// use common::profiling;

// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;

/**
 * The code below provides an example of all stages of running a RAM ppzkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the ppzkSNARK generator on input a given
 *     architecture and bounds on the computation.
 * (2) The "prover", which runs the ppzkSNARK prover on input the proving key,
 *     a boot trace, and an auxiliary input.
 * (3) The "verifier", which runs the ppzkSNARK verifier on input the verification key,
 *     a boot trace, and a proof.
 */
//
pub fn run_ram_ppzksnark<ram_ppzksnark_ppT: RamPptConfig>(
    example: &ram_example<ram_ppzksnark_machine_pp<ram_ppzksnark_ppT>>,
    test_serialization: bool,
) -> bool
where
    knowledge_commitment<
        <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
        <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ram_ppzksnark_ppT as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
                <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
        <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ram_ppzksnark_ppT as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
                <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<ram_ppzksnark_ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    enter_block("Call to run_ram_ppzksnark", false);

    print!("This run uses an example with the following parameters:\n");
    example.ap.print();
    print!(
        "* Primary input size bound (L): {}\n",
        example.boot_trace_size_bound
    );
    print!("* Time bound (T): {}\n", example.time_bound);
    print!(
        "Hence, log2(L+2*T) equals {}\n",
        log2(example.boot_trace_size_bound + 2 * example.time_bound)
    );

    println!("RAM ppzkSNARK Generator");
    let mut keypair = ram_ppzksnark_generator::<ram_ppzksnark_ppT>(
        &example.ap,
        example.boot_trace_size_bound,
        example.time_bound,
    );
    print!("\n");
    print_indent();
    println!("after generator");

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<ram_ppzksnark_proving_key<ram_ppzksnark_ppT>>(&keypair.pk);
        keypair.vk = reserialize::<ram_ppzksnark_verification_key<ram_ppzksnark_ppT>>(&keypair.vk);
        leave_block("Test serialization of keys", false);
    }

    println!("RAM ppzkSNARK Prover");
    let mut proof = ram_ppzksnark_prover::<ram_ppzksnark_ppT>(
        &keypair.pk,
        &example.boot_trace,
        &example.auxiliary_input,
    );
    print!("\n");
    print_indent();
    println!("after prover");

    if test_serialization {
        enter_block("Test serialization of proof", false);
        proof = reserialize::<ram_ppzksnark_proof<ram_ppzksnark_ppT>>(&proof);
        leave_block("Test serialization of proof", false);
    }

    println!("RAM ppzkSNARK Verifier");
    let ans = ram_ppzksnark_verifier::<ram_ppzksnark_ppT>(&keypair.vk, &example.boot_trace, &proof);
    print!("\n");
    print_indent();
    println!("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    leave_block("Call to run_ram_ppzksnark", false);

    ans
}
