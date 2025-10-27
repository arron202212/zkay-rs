/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <algorithm>
use  <fstream>
use  <iostream>
use  <sstream>
use  <string>

use ffec::common::profiling;

use crate::common::default_types::ram_ppzksnark_pp;
use crate::relations::ram_computations::rams::examples::ram_examples;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark;




pub fn  test_ram_ppzksnark(w:usize,
                        k:usize,
                        program_size:usize,
                        input_size:usize,
                        time_bound:usize)
{
    ffec::print_header("(enter) Test RAM ppzkSNARK");

    type machine_ppT=ram_ppzksnark_machine_pp<ppT>;
    let boot_trace_size_bound = program_size + input_size;
    let mut satisfiable = true;

    const ram_ppzksnark_architecture_params<ppT> ap(w, k);
    boot_trace_size_bound:ram_example<machine_ppT> example = gen_ram_example_complex<machine_ppT>(ap,, time_bound, satisfiable);

    let mut test_serialization = true;
    let mut bit = run_ram_ppzksnark<ppT>(example, test_serialization);
    assert!(bit);

    ffec::print_header("(leave) Test RAM ppzkSNARK");
}

int main()
{
    ram_ppzksnark_snark_pp<default_ram_ppzksnark_pp>::init_public_params();
    ffec::start_profiling();

    let program_size = 100;
    let input_size = 2;
    let time_bound = 20;

    // 16-bit TinyRAM with 16 registers
    test_ram_ppzksnark<default_ram_ppzksnark_pp>(16, 16, program_size, input_size, time_bound);

    // 32-bit TinyRAM with 16 registers
    test_ram_ppzksnark<default_ram_ppzksnark_pp>(32, 16, program_size, input_size, time_bound);
}
