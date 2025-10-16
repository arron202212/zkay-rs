/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <algorithm>
use  <cstring>
use  <fstream>
use  <iostream>
use  <sstream>
use  <string>

use ffec::common::profiling;

use crate::common::default_types::ram_ppzksnark_pp;
use crate::relations::ram_computations/rams/examples/ram_examples;
use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark;



int main(int argc, const char * argv[])
{
    ram_ppzksnark_snark_pp<default_ram_ppzksnark_pp>::init_public_params();
    ffec::start_profiling();

    if argc == 2 && strcmp(argv[1], "-v") == 0
    {
        ffec::print_compilation_info();
        return 0;
    }

    if argc != 6
    {
        print!("usage: %s word_size reg_count program_size input_size time_bound\n", argv[0]);
        return 1;
    }

    const size_t w = atoi(argv[1]),
                 k = atoi(argv[2]),
                 program_size = atoi(argv[3]),
                 input_size = atoi(argv[4]),
                 time_bound = atoi(argv[5]);

    type ram_ppzksnark_machine_pp<default_ram_ppzksnark_pp> machine_ppT;

    const ram_ppzksnark_architecture_params<default_ram_ppzksnark_pp> ap(w, k);

    ffec::enter_block("Generate RAM example");
    const size_t boot_trace_size_bound = program_size + input_size;
    const bool satisfiable = true;
    ram_example<machine_ppT> example = gen_ram_example_complex<machine_ppT>(ap, boot_trace_size_bound, time_bound, satisfiable);
    ffec::leave_block("Generate RAM example");

    ffec::print_header("(enter) Profile RAM ppzkSNARK");
    const bool test_serialization = true;
    run_ram_ppzksnark<default_ram_ppzksnark_pp>(example, test_serialization);
    ffec::print_header("(leave) Profile RAM ppzkSNARK");
}
