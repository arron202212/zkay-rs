/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <sstream>

use crate::common::default_types::ram_zksnark_pp;
use crate::relations::ram_computations::rams::examples::ram_examples;
use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark;




pub fn  test_ram_zksnark(w:usize,
                      k:usize,
                      boot_trace_size_bound:usize,
                      time_bound:usize)
{
    type ramT=ram_zksnark_machine_pp<ppT>;
    const ram_architecture_params<ramT> ap(w, k);
    boot_trace_size_bound:ram_example<ramT> example = gen_ram_example_complex<ramT>(ap,, time_bound, true);
    let mut test_serialization = true;
    let mut ans = run_ram_zksnark<ppT>(example, test_serialization);
    assert!(ans);
}

pub fn main()->i32
{
    ffec::start_profiling();
    ram_zksnark_PCD_pp<default_ram_zksnark_pp>::init_public_params();

    let w = 32;
    let k = 16;

    let boot_trace_size_bound = 20;
    let time_bound = 10;

    test_ram_zksnark<default_ram_zksnark_pp>(w, k, boot_trace_size_bound, time_bound);
}
