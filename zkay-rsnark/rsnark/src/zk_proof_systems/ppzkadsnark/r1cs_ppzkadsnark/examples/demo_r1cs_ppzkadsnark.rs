// /**
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// use  <algorithm>
// use  <cassert>
// use  <cstdio>
// use  <cstring>
//

use ffec::common::profiling;

use crate::common::default_types::r1cs_ppzkadsnark_pp;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::run_r1cs_ppzkadsnark;

pub fn main(argc: i32, argv: [&str]) -> i32 {
    default_r1cs_ppzkadsnark_pp::init_public_params();
    start_profiling();

    if argc == 2 && argv[1] == "-v" {
        print_compilation_info();
        return 0;
    }

    if argc != 3 && argc != 4 {
        print!("usage: {} num_constraints input_size [Fr|bytes]\n", argv[0]);
        return 1;
    }
    let num_constraints = atoi(argv[1]);
    let input_size = atoi(argv[2]);
    if argc == 4 {
        assert!(argv[3] == "Fr" || argv[3] == "bytes");
        if argv[3] == "bytes" {
            input_size = div_ceil(
                8 * input_size,
                Fr::<snark_pp<default_r1cs_ppzkadsnark_pp>>::num_bits - 1,
            );
        }
    }

    enter_block("Generate R1CS example");
    let example = generate_r1cs_example_with_field_input::<Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>>(
        num_constraints,
        input_size,
    );
    leave_block("Generate R1CS example");

    print_header("(enter) Profile R1CS ppzkADSNARK");
    let test_serialization = true;
    run_r1cs_ppzkadsnark::<default_r1cs_ppzkadsnark_pp>(example, test_serialization);
    print_header("(leave) Profile R1CS ppzkADSNARK");
}
