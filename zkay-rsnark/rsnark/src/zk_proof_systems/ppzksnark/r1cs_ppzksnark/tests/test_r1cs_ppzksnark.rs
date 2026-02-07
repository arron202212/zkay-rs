//  Test program that exercises the ppzkSNARK (first generator, then
//  prover, then verifier) on a synthetic R1CS instance.

use crate::common::default_types::r1cs_ppzksnark_pp::default_r1cs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::generate_r1cs_example_with_binary_input;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark::run_r1cs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ff_curves::{G1, G2};
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use std::ops::Mul;

pub fn test_r1cs_ppzksnark<ppT: ppTConfig>(num_constraints: usize, input_size: usize)
where
    knowledge_commitment<G2<ppT>, G1<ppT>>:
        Mul<Fr<ppT>, Output = knowledge_commitment<G2<ppT>, G1<ppT>>>,
    knowledge_commitment<G1<ppT>, G1<ppT>>:
        Mul<Fr<ppT>, Output = knowledge_commitment<G1<ppT>, G1<ppT>>>,
{
    print_header("(enter) Test R1CS ppzkSNARK");

    let mut test_serialization = true;
    let example = generate_r1cs_example_with_binary_input::<
        Fr<ppT>,
        pb_variable,
        pb_linear_combination,
    >(num_constraints, input_size);
    let mut bit = run_r1cs_ppzksnark::<ppT>(&example, test_serialization);
    assert!(bit);

    print_header("(leave) Test R1CS ppzkSNARK");
}

fn main<default_r1cs_ppzksnark_pp: ppTConfig>() -> i32
where
    knowledge_commitment<G2<default_r1cs_ppzksnark_pp>, G1<default_r1cs_ppzksnark_pp>>: Mul<
            Fr<default_r1cs_ppzksnark_pp>,
            Output = knowledge_commitment<
                G2<default_r1cs_ppzksnark_pp>,
                G1<default_r1cs_ppzksnark_pp>,
            >,
        >,
    knowledge_commitment<G1<default_r1cs_ppzksnark_pp>, G1<default_r1cs_ppzksnark_pp>>: Mul<
            Fr<default_r1cs_ppzksnark_pp>,
            Output = knowledge_commitment<
                G1<default_r1cs_ppzksnark_pp>,
                G1<default_r1cs_ppzksnark_pp>,
            >,
        >,
{
    default_r1cs_ppzksnark_pp::init_public_params();
    start_profiling();

    test_r1cs_ppzksnark::<default_r1cs_ppzksnark_pp>(1000, 100);
    0
}
