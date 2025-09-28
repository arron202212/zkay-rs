/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <iostream>
use  <sstream>

use  "depends/gtest/googletest/include/gtest/gtest.h"

use  <libsnark/common/default_types/r1cs_ppzksnark_pp.hpp>
use  <libsnark/gadgetlib2/examples/simple_example.hpp>
use  <libsnark/gadgetlib2/gadget.hpp>
use  <libsnark/gadgetlib2/pp.hpp>
use  <libsnark/gadgetlib2/protoboard.hpp>
use  <libsnark/relations/constraint_satisfaction_problems/r1cs/examples/r1cs_examples.hpp>
use  <libsnark/zk_proof_systems/ppzksnark/r1cs_ppzksnark/examples/run_r1cs_ppzksnark.hpp>

using namespace gadgetlib2;

namespace {

TEST(gadgetLib2,Integration) {
    using namespace libsnark;

    initPublicParamsFromDefaultPp();
    const r1cs_example<libff::Fr<default_r1cs_ppzksnark_pp> > example = gen_r1cs_example_from_gadgetlib2_protoboard(100);
    const bool test_serialization = false;

    const bool bit = run_r1cs_ppzksnark<default_r1cs_ppzksnark_pp>(example, test_serialization);
    EXPECT_TRUE(bit);
};

}
