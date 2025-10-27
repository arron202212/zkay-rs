/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <iostream>
use  <sstream>

use  "depends/gtest/googletest/include/gtest/gtest.h"

use crate::common::default_types::r1cs_ppzksnark_pp;
use crate::gadgetlib2::examples::simple_example;
use crate::gadgetlib2::gadget;
use crate::gadgetlib2::pp;
use crate::gadgetlib2::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark;

using namespace gadgetlib2;

namespace {

TEST(gadgetLib2,Integration) {
    

    initPublicParamsFromDefaultPp();
    const r1cs_example<ffec::Fr<default_r1cs_ppzksnark_pp> > example = gen_r1cs_example_from_gadgetlib2_protoboard(100);
    let mut test_serialization = false;

    let mut bit = run_r1cs_ppzksnark<default_r1cs_ppzksnark_pp>(example, test_serialization);
    EXPECT_TRUE(bit);
};

}
