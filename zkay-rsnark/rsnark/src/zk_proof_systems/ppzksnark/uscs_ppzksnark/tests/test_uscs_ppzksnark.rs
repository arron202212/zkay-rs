/** @file
 *****************************************************************************
 Test program that exercises the ppzkSNARK (first generator, then
 prover, then verifier) on a synthetic USCS instance.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <cassert>
use  <cstdio>

use ffec::common::profiling;
use ffec::common::utils;

use crate::common::default_types::uscs_ppzksnark_pp;
use crate::relations::constraint_satisfaction_problems/uscs/examples/uscs_examples;
use libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/examples/run_uscs_ppzksnark;




pub fn  test_uscs_ppzksnark(usize num_constraints,
                         usize input_size)
{
    ffec::print_header("(enter) Test USCS ppzkSNARK");

    let mut test_serialization = true;
    uscs_example<ffec::Fr<ppT> > example = generate_uscs_example_with_binary_input<ffec::Fr<ppT> >(num_constraints, input_size);
    let mut bit = run_uscs_ppzksnark<ppT>(example, test_serialization);
    assert!(bit);

    ffec::print_header("(leave) Test USCS ppzkSNARK");
}

int main()
{
    default_uscs_ppzksnark_pp::init_public_params();
    ffec::start_profiling();

    test_uscs_ppzksnark<default_uscs_ppzksnark_pp>(1000, 100);
}
