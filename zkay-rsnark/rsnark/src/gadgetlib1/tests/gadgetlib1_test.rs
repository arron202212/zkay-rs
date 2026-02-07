//  Unit tests for gadgetlib1 - main() for running all tests

// use  <gtest/gtest.h>

// use crate::gadgetlib1::examples::simple_example;
// use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark;
use crate::gadgetlib1::examples::simple_example::gen_r1cs_example_from_protoboard;
use crate::gadgetlib1::protoboard::PBConfig;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark::run_r1cs_ppzksnark;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ff_curves::default_ec_pp;

//  {
// (gadgetLib1,Integration)
type FieldT = Fr<default_ec_pp>;
fn test<PB: PBConfig>() {
    // Create an example constraint system and translate to libsnark format
    default_ec_pp::init_public_params();
    let example = gen_r1cs_example_from_protoboard::<FieldT, PB>(100);
    let mut test_serialization = false;
    // Run ppzksnark. Jump into function for breakdown
    let mut bit = run_r1cs_ppzksnark::<default_ec_pp>(&example, test_serialization);
    assert!(bit);
}

fn main<PB: PBConfig>(argc: i32, argv: &[&str]) -> i32 {
    // ::testing::InitGoogleTest(&argc, argv);
    // return RUN_ALL_TESTS();
    test::<PB>();
    0
}
