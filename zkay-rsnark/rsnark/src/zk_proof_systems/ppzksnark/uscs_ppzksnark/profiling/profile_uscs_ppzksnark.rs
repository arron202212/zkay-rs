//  Profiling program that exercises the ppzkSNARK (first generator, then prover,
//  then verifier) on a synthetic USCS instance.

//  The command

//      $ libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/profiling/profile_uscs_ppzksnark 1000 10 Fr

//  exercises the ppzkSNARK (first generator, then prover, then verifier) on an USCS instance with 1000 equations and an input consisting of 10 field elements.

//  (If you get the error `zmInit ERR:can't protect`, see the discussion [above](#elliptic-curve-choices).)

//  The command

//      $ libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/profiling/profile_uscs_ppzksnark 1000 10 bytes

//  does the same but now the input consists of 10 bytes.

// use common::profiling;
// use common::utils;

// use crate::common::default_types::uscs_ppzksnark_pp;
// use crate::relations::constraint_satisfaction_problems/uscs/examples/uscs_examples;
// use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/examples/run_uscs_ppzksnark;
use crate::common::default_types::uscs_ppzksnark_pp::default_uscs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::uscs::examples::uscs_examples::generate_uscs_example_with_field_input;
use crate::zk_proof_systems::ppzksnark::uscs_ppzksnark::examples::run_uscs_ppzksnark::run_uscs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{enter_block, leave_block, print_compilation_info, start_profiling};

fn main<default_uscs_ppzksnark_pp: ppTConfig>(argc: i32, argv: &[&str]) -> i32 {
    default_uscs_ppzksnark_pp::init_public_params();
    start_profiling();

    if argc == 2 && argv[1] == "-v" {
        print_compilation_info();
        return 0;
    }

    if argc != 3 {
        print!("usage: {} num_constraints input_size\n", argv[0]);
        return 1;
    }

    let num_constraints = argv[1].parse::<usize>().unwrap();
    let input_size = argv[2].parse::<usize>().unwrap();

    enter_block("Generate USCS example", false);
    let example = generate_uscs_example_with_field_input::<
        Fr<default_uscs_ppzksnark_pp>,
        pb_variable,
        pb_linear_combination,
    >(num_constraints, input_size);
    leave_block("Generate USCS example", false);

    println!("(enter) Profile USCS ppzkSNARK");
    let mut test_serialization = true;
    run_uscs_ppzksnark::<default_uscs_ppzksnark_pp>(&example, test_serialization);
    println!("(leave) Profile USCS ppzkSNARK");
    0
}
