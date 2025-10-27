/** @file
 *****************************************************************************
 Test program that exercises the ppzkSNARK (first generator, then
 prover, then verifier) on a synthetic TBCS instance.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <cassert>
use  <cstdio>

use ffec::common::profiling;

use crate::common::default_types::tbcs_ppzksnark_pp;
use crate::relations::circuit_satisfaction_problems/tbcs/examples/tbcs_examples;
use libsnark/zk_proof_systems/ppzksnark/tbcs_ppzksnark/examples/run_tbcs_ppzksnark;




pub fn  test_tbcs_ppzksnark(primary_input_size:usize,
                         auxiliary_input_size:usize,
                         num_gates:usize,
                         num_outputs:usize)
{
    ffec::print_header("(enter) Test TBCS ppzkSNARK");

    let mut test_serialization = true;
    auxiliary_input_size:tbcs_example example = generate_tbcs_example(primary_input_size,, num_gates, num_outputs);
// #ifdef DEBUG
    example.circuit.print();
//#endif
    let mut bit = run_tbcs_ppzksnark<ppT>(example, test_serialization);
    assert!(bit);

    ffec::print_header("(leave) Test TBCS ppzkSNARK");
}

int main()
{
    default_tbcs_ppzksnark_pp::init_public_params();
    ffec::start_profiling();

    test_tbcs_ppzksnark<default_tbcs_ppzksnark_pp>(10, 10, 20, 5);
}
