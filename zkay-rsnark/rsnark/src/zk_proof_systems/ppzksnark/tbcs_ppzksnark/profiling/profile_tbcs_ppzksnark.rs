//  Profiling program that exercises the ppzkSNARK (first generator, then prover,
//  then verifier) on a synthetic TBCS instance.

//  The command

//      $ libsnark/tbcs_ppzksnark/examples/profile_tbcs_ppzksnark 1000 10

//  exercises the ppzkSNARK (first generator, then prover, then verifier) on an TBCS instance with 1000 gates and an input consisting of 10 bits.

// use common::profiling;
// use common::utils;

// use crate::common::default_types::tbcs_ppzksnark_pp;
// use crate::relations::circuit_satisfaction_problems/tbcs/examples/tbcs_examples;
// use libsnark/zk_proof_systems/ppzksnark/tbcs_ppzksnark/examples/run_tbcs_ppzksnark;
use crate::common::default_types::tbcs_ppzksnark_pp::default_tbcs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::circuit_satisfaction_problems::tbcs::examples::tbcs_examples::generate_tbcs_example;
use crate::zk_proof_systems::ppzksnark::tbcs_ppzksnark::examples::run_tbcs_ppzksnark::run_tbcs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{enter_block, leave_block, print_compilation_info, start_profiling};
use std::ops::Mul;

fn main<default_tbcs_ppzksnark_pp: ppTConfig>(argc: i32, argv: &[&str]) -> i32
where
    knowledge_commitment<
        <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G1,
        <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <default_tbcs_ppzksnark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G1,
                <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G2,
        <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <default_tbcs_ppzksnark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G2,
                <default_tbcs_ppzksnark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    default_tbcs_ppzksnark_pp::init_public_params();
    start_profiling();

    if argc == 2 && argv[1] == "-v" {
        print_compilation_info();
        return 0;
    }

    if argc != 3 {
        print!("usage: {} num_gates primary_input_size\n", argv[0]);
        return 1;
    }
    let num_gates = argv[1].parse::<usize>().unwrap();
    let primary_input_size = argv[2].parse::<usize>().unwrap();

    let auxiliary_input_size = 0;
    let num_outputs = num_gates / 2;

    enter_block("Generate TBCS example", false);
    let example = generate_tbcs_example(
        primary_input_size,
        auxiliary_input_size,
        num_gates,
        num_outputs,
    );
    leave_block("Generate TBCS example", false);

    println!("(enter) Profile TBCS ppzkSNARK");
    let mut test_serialization = true;
    run_tbcs_ppzksnark::<default_tbcs_ppzksnark_pp>(&example, test_serialization);
    println!("(leave) Profile TBCS ppzkSNARK");
    0
}
