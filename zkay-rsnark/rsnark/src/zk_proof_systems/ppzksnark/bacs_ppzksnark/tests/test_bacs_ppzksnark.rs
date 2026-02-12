//  Test program that exercises the ppzkSNARK (first generator, then
//  prover, then verifier) on a synthetic BACS instance.

use crate::common::default_types::bacs_ppzksnark_pp::default_bacs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::circuit_satisfaction_problems::bacs::examples::bacs_examples::generate_bacs_example;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::ppzksnark::bacs_ppzksnark::examples::run_bacs_ppzksnark::run_bacs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use std::ops::Mul;

pub fn test_bacs_ppzksnark<ppT: ppTConfig>(
    primary_input_size: usize,
    auxiliary_input_size: usize,
    num_gates: usize,
    num_outputs: usize,
) where
    knowledge_commitment<
        <ppT as ff_curves::PublicParams>::G1,
        <ppT as ff_curves::PublicParams>::G1,
    >: Mul<
            <ppT as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <ppT as ff_curves::PublicParams>::G1,
                <ppT as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <ppT as ff_curves::PublicParams>::G2,
        <ppT as ff_curves::PublicParams>::G1,
    >: Mul<
            <ppT as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <ppT as ff_curves::PublicParams>::G2,
                <ppT as ff_curves::PublicParams>::G1,
            >,
        >,
{
    print_header("(enter) Test BACS ppzkSNARK");

    let mut test_serialization = true;
    let example = generate_bacs_example::<Fr<ppT>, pb_variable, pb_linear_combination>(
        primary_input_size,
        auxiliary_input_size,
        num_gates,
        num_outputs,
    );
    // #ifdef DEBUG
    example.circuit.print();

    let mut bit = run_bacs_ppzksnark::<ppT>(&example, test_serialization);
    assert!(bit);

    print_header("(leave) Test BACS ppzkSNARK");
}

fn main<default_bacs_ppzksnark_pp: ppTConfig>() -> i32
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

    test_bacs_ppzksnark::<default_bacs_ppzksnark_pp>(10, 10, 20, 5);
    0
}
