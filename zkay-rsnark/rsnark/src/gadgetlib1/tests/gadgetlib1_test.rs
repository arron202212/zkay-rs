/** @file
 *****************************************************************************
 Unit tests for gadgetlib1 - main() for running all tests
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <gtest/gtest.h>

use libsnark/gadgetlib1/examples/simple_example;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark;

namespace {

TEST(gadgetLib1,Integration) {
    type ffec::Fr<ffec::default_ec_pp> FieldT;
    // Create an example constraint system and translate to libsnark format
    ffec::default_ec_pp::init_public_params();
    const auto example = crate::gen_r1cs_example_from_protoboard<FieldT>(100);
    const bool test_serialization = false;
    // Run ppzksnark. Jump into function for breakdown
    const bool bit = crate::run_r1cs_ppzksnark<ffec::default_ec_pp>(example, test_serialization);
    EXPECT_TRUE(bit);
};

}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
