//  Profiling program that exercises the ppzkSNARK (first generator, then prover,
//  then verifier) on a synthetic R1CS instance.

//  The command

//      $ libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/profiling/profile_r1cs_gg_ppzksnark 1000 10 Fr

//  exercises the ppzkSNARK (first generator, then prover, then verifier) on an R1CS instance with 1000 equations and an input consisting of 10 field elements.

//  (If you get the error `zmInit ERR:can't protect`, see the discussion [above](#elliptic-curve-choices).)

//  The command

//      $ libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/profiling/profile_r1cs_gg_ppzksnark 1000 10 bytes

//  does the same but now the input consists of 10 bytes.

// use common::profiling;
// use common::utils;

// use crate::common::default_types::r1cs_gg_ppzksnark_pp;
// use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
// use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples::run_r1cs_gg_ppzksnark;
use crate::common::default_types::r1cs_gg_ppzksnark_pp::default_r1cs_gg_ppzksnark_pp;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::generate_r1cs_example_with_field_input;
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples::run_r1cs_gg_ppzksnark::run_r1cs_gg_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ff_curves::default_ec_pp;
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_compilation_info, start_profiling};
use ffec::div_ceil;

fn main(argc: i32, argv: &[&str]) -> i32 {
    default_r1cs_gg_ppzksnark_pp::init_public_params();
    start_profiling();

    if argc == 2 && argv[1] == "-v" {
        print_compilation_info();
        return 0;
    }

    if argc != 3 && argc != 4 {
        print!("usage: {} num_constraints input_size [Fr|bytes]\n", argv[0]);
        return 1;
    }
    let num_constraints = argv[1].parse::<usize>().unwrap();
    let mut input_size = argv[2].parse::<usize>().unwrap();
    if argc == 4 {
        assert!(argv[3] == "Fr" || argv[3] == "bytes");
        if argv[3] == "bytes" {
            input_size = div_ceil(8 * input_size, Fr::<default_ec_pp>::capacity()).unwrap();
        }
    }

    enter_block("Generate R1CS example", false);
    let example = generate_r1cs_example_with_field_input::<
        Fr<default_r1cs_gg_ppzksnark_pp>,
        pb_variable,
        pb_linear_combination,
    >(num_constraints, input_size);
    leave_block("Generate R1CS example", false);

    println!("(enter) Profile R1CS GG-ppzkSNARK");
    let mut test_serialization = true;
    run_r1cs_gg_ppzksnark::<default_r1cs_gg_ppzksnark_pp>(&example, test_serialization);
    println!("(leave) Profile R1CS GG-ppzkSNARK");
    0
}
