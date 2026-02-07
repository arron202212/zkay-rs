//  Test program that exercises the ppzkSNARK (first generator, then
//  prover, then verifier) on a synthetic USCS instance.

use crate::common::default_types::uscs_ppzksnark_pp::default_uscs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::circuit_satisfaction_problems::tbcs::examples::tbcs_examples::generate_tbcs_example;
use crate::relations::constraint_satisfaction_problems::uscs::examples::uscs_examples::generate_uscs_example_with_binary_input;
use crate::zk_proof_systems::ppzksnark::uscs_ppzksnark::examples::run_uscs_ppzksnark::run_uscs_ppzksnark;
use ff_curves::{Fr, PublicParams};
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;

pub fn test_uscs_ppzksnark<ppT: ppTConfig>(num_constraints: usize, input_size: usize) {
    print_header("(enter) Test USCS ppzkSNARK");

    let mut test_serialization = true;
    let example = generate_uscs_example_with_binary_input::<
        Fr<ppT>,
        pb_variable,
        pb_linear_combination,
    >(num_constraints, input_size);
    let mut bit = run_uscs_ppzksnark::<ppT>(&example, test_serialization);
    assert!(bit);

    print_header("(leave) Test USCS ppzkSNARK");
}

fn main<default_uscs_ppzksnark_pp: ppTConfig>() -> i32 {
    default_uscs_ppzksnark_pp::init_public_params();
    start_profiling();

    test_uscs_ppzksnark::<default_uscs_ppzksnark_pp>(1000, 100);
    0
}
