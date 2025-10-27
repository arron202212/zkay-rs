/** @file
 *****************************************************************************
 Profiling program that exercises the ppzkSNARK (first generator, then prover,
 then verifier) on a synthetic USCS instance.

 The command

     $ libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/profiling/profile_uscs_ppzksnark 1000 10 Fr

 exercises the ppzkSNARK (first generator, then prover, then verifier) on an USCS instance with 1000 equations and an input consisting of 10 field elements.

 (If you get the error `zmInit ERR:can't protect`, see the discussion [above](#elliptic-curve-choices).)

 The command

     $ libsnark/zk_proof_systems/ppzksnark/uscs_ppzksnark/profiling/profile_uscs_ppzksnark 1000 10 bytes

 does the same but now the input consists of 10 bytes.

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



int main(int argc, const char * argv[])
{
    default_uscs_ppzksnark_pp::init_public_params();
    ffec::start_profiling();

    if argc == 2 && strcmp(argv[1], "-v") == 0
    {
        ffec::print_compilation_info();
        return 0;
    }

    if argc != 3
    {
        print!("usage: {} num_constraints input_size\n", argv[0]);
        return 1;
    }

    let num_constraints= atoi(argv[1]);
    let input_size= atoi(argv[2]);

    ffec::enter_block("Generate USCS example");
    uscs_example<ffec::Fr<default_uscs_ppzksnark_pp> > example = generate_uscs_example_with_field_input<ffec::Fr<default_uscs_ppzksnark_pp> >(num_constraints, input_size);
    ffec::leave_block("Generate USCS example");

    ffec::print_header("(enter) Profile USCS ppzkSNARK");
    let mut test_serialization = true;
    run_uscs_ppzksnark<default_uscs_ppzksnark_pp>(example, test_serialization);
    ffec::print_header("(leave) Profile USCS ppzkSNARK");
}
