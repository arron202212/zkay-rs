/** @file
 *****************************************************************************
 Test program that exercises the ppzkSNARK (first generator, then
 prover, then verifier) on a synthetic BACS instance.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <cassert>
use  <cstdio>

use ffec::common::profiling;

use crate::common::default_types::bacs_ppzksnark_pp;
use crate::relations::circuit_satisfaction_problems/bacs/examples/bacs_examples;
use libsnark/zk_proof_systems/ppzksnark/bacs_ppzksnark/examples/run_bacs_ppzksnark;




pub fn  test_bacs_ppzksnark(primary_input_size:usize,
                         auxiliary_input_size:usize,
                         num_gates:usize,
                         num_outputs:usize)
{
    ffec::print_header("(enter) Test BACS ppzkSNARK");

    let mut test_serialization = true;
    auxiliary_input_size:bacs_example<ffec::Fr<ppT> > example = generate_bacs_example<ffec::Fr<ppT> >(primary_input_size,, num_gates, num_outputs);
// #ifdef DEBUG
    example.circuit.print();
//#endif
    let mut bit = run_bacs_ppzksnark<ppT>(example, test_serialization);
    assert!(bit);

    ffec::print_header("(leave) Test BACS ppzkSNARK");
}

int main()
{
    default_bacs_ppzksnark_pp::init_public_params();
    ffec::start_profiling();

    test_bacs_ppzksnark<default_bacs_ppzksnark_pp>(10, 10, 20, 5);
}
