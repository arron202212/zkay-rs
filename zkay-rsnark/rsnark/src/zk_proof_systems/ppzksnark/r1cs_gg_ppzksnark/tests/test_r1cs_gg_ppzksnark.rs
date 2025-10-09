/** @file
 *****************************************************************************
 Test program that exercises the ppzkSNARK (first generator, then
 prover, then verifier) on a synthetic R1CS instance.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <cassert>
use  <cstdio>

use ffec::common::profiling;
use ffec::common::utils;

use crate::common::default_types::r1cs_gg_ppzksnark_pp;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples/run_r1cs_gg_ppzksnark;



template<typename ppT>
void test_r1cs_gg_ppzksnark(size_t num_constraints,
                         size_t input_size)
{
    ffec::print_header("(enter) Test R1CS GG-ppzkSNARK");

    const bool test_serialization = true;
    r1cs_example<ffec::Fr<ppT> > example = generate_r1cs_example_with_binary_input<ffec::Fr<ppT> >(num_constraints, input_size);
    const bool bit = run_r1cs_gg_ppzksnark<ppT>(example, test_serialization);
    assert!(bit);

    ffec::print_header("(leave) Test R1CS GG-ppzkSNARK");
}

int main()
{
    default_r1cs_gg_ppzksnark_pp::init_public_params();
    ffec::start_profiling();

    test_r1cs_gg_ppzksnark<default_r1cs_gg_ppzksnark_pp>(1000, 100);
}
