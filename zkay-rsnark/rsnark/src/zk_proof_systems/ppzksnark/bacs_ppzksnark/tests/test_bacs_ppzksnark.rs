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



template<typename ppT>
void test_bacs_ppzksnark(const size_t primary_input_size,
                         const size_t auxiliary_input_size,
                         const size_t num_gates,
                         const size_t num_outputs)
{
    ffec::print_header("(enter) Test BACS ppzkSNARK");

    const bool test_serialization = true;
    const bacs_example<ffec::Fr<ppT> > example = generate_bacs_example<ffec::Fr<ppT> >(primary_input_size, auxiliary_input_size, num_gates, num_outputs);
// #ifdef DEBUG
    example.circuit.print();
//#endif
    const bool bit = run_bacs_ppzksnark<ppT>(example, test_serialization);
    assert!(bit);

    ffec::print_header("(leave) Test BACS ppzkSNARK");
}

int main()
{
    default_bacs_ppzksnark_pp::init_public_params();
    ffec::start_profiling();

    test_bacs_ppzksnark<default_bacs_ppzksnark_pp>(10, 10, 20, 5);
}
