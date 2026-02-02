//  Profiling program that exercises the ppzkSNARK (first generator, then prover,
//  then verifier) on a synthetic BACS instance.

//  The command

//      $ libsnark/zk_proof_systems/bacs_ppzksnark/profiling/profile_bacs_ppzksnark 1000 10

//  exercises the ppzkSNARK (first generator, then prover, then verifier) on an BACS instance with 1000 gates and an input consisting of 10 Field elements

//  (If you get the error `zmInit ERR:can't protect`, see the discussion [above](#elliptic-curve-choices).)

// use common::profiling;

// use crate::common::default_types::bacs_ppzksnark_pp;
// use crate::relations::circuit_satisfaction_problems/bacs/examples/bacs_examples;
// use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/examples/run_bacs_ppzksnark;
use crate::common::default_types::bacs_ppzksnark_pp::default_bacs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::circuit_satisfaction_problems::bacs::examples::bacs_examples::generate_bacs_example;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::generate_r1cs_example_with_field_input;
use crate::zk_proof_systems::ppzksnark::bacs_ppzksnark::examples::run_bacs_ppzksnark::run_bacs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ff_curves::default_ec_pp;
use ffec::common::profiling::{enter_block, leave_block, print_compilation_info, start_profiling};
use ffec::div_ceil;
use std::ops::Mul;

fn main<default_bacs_ppzksnark_pp: ppTConfig>(argc: i32, argv: &[&str]) -> i32
where
    knowledge_commitment<
        <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G1,
        <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <default_bacs_ppzksnark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G1,
                <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G2,
        <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <default_bacs_ppzksnark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G2,
                <default_bacs_ppzksnark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    default_bacs_ppzksnark_pp::init_public_params();
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

    enter_block("Generate BACS example", false);
    let example =
        generate_bacs_example::<Fr<default_bacs_ppzksnark_pp>, pb_variable, pb_linear_combination>(
            primary_input_size,
            auxiliary_input_size,
            num_gates,
            num_outputs,
        );
    leave_block("Generate BACS example", false);

    println!("(enter) Profile BACS ppzkSNARK");
    let mut test_serialization = true;
    run_bacs_ppzksnark::<default_bacs_ppzksnark_pp>(&example, test_serialization);
    println!("(leave) Profile BACS ppzkSNARK");
    0
}
