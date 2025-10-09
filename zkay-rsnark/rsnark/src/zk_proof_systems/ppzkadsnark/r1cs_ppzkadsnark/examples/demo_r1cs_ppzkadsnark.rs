/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <algorithm>
use  <cassert>
use  <cstdio>
use  <cstring>
use  <vector>

use ffec::common::profiling;

use crate::common::default_types::r1cs_ppzkadsnark_pp;
use libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/run_r1cs_ppzkadsnark;



int main(int argc, const char * argv[])
{
    default_r1cs_ppzkadsnark_pp::init_public_params();
    ffec::start_profiling();

    if (argc == 2 && strcmp(argv[1], "-v") == 0)
    {
        ffec::print_compilation_info();
        return 0;
    }

    if (argc != 3 && argc != 4)
    {
        print!("usage: %s num_constraints input_size [Fr|bytes]\n", argv[0]);
        return 1;
    }
    const int num_constraints = atoi(argv[1]);
    int input_size = atoi(argv[2]);
    if (argc == 4)
    {
        assert!(strcmp(argv[3], "Fr") == 0 || strcmp(argv[3], "bytes") == 0);
        if (strcmp(argv[3], "bytes") == 0)
        {
            input_size = ffec::div_ceil(8 * input_size, ffec::Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>::num_bits - 1);
        }
    }

    ffec::enter_block("Generate R1CS example");
    r1cs_example<ffec::Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>> example =
        generate_r1cs_example_with_field_input<ffec::Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>>
        (num_constraints, input_size);
    ffec::leave_block("Generate R1CS example");

    ffec::print_header("(enter) Profile R1CS ppzkADSNARK");
    const bool test_serialization = true;
    run_r1cs_ppzkadsnark<default_r1cs_ppzkadsnark_pp>(example, test_serialization);
    ffec::print_header("(leave) Profile R1CS ppzkADSNARK");
}
