//  Test program that exercises the ppzkSNARK (first generator, then
//  prover, then verifier) on a synthetic TBCS instance.

use crate::common::default_types::tbcs_ppzksnark_pp::default_tbcs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::circuit_satisfaction_problems::tbcs::examples::tbcs_examples::generate_tbcs_example;
use crate::zk_proof_systems::ppzksnark::tbcs_ppzksnark::examples::run_tbcs_ppzksnark::run_tbcs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use std::ops::Mul;

pub fn test_tbcs_ppzksnark<ppT: ppTConfig>(
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
    print_header("(enter) Test TBCS ppzkSNARK");

    let mut test_serialization = true;
    let example = generate_tbcs_example(
        primary_input_size,
        auxiliary_input_size,
        num_gates,
        num_outputs,
    );
    // #ifdef DEBUG
    example.circuit.print();

    let mut bit = run_tbcs_ppzksnark::<ppT>(&example, test_serialization);
    assert!(bit);

    print_header("(leave) Test TBCS ppzkSNARK");
}

fn main<default_tbcs_ppzksnark_pp: ppTConfig>() -> i32
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

    test_tbcs_ppzksnark::<default_tbcs_ppzksnark_pp>(10, 10, 20, 5);
    0
}
