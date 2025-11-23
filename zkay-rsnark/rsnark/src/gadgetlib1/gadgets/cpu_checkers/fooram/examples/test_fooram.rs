/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use ffec::common::utils;

use crate::common::default_types::r1cs_ppzkpcd_pp;
use crate::common::default_types::r1cs_ppzksnark_pp;
use crate::relations::ram_computations::rams::fooram::fooram_params;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark;



pub trait  default_fooram_zksnark_pp {
// 
    type PCD_pp=default_r1cs_ppzkpcd_pp;
    type FieldT= PCD_pp::scalar_field_A;
    type machine_pp=ram_fooram<FieldT>;

    fn init_public_params() { PCD_pp::init_public_params(); }
}

pub trait  default_fooram_ppzksnark_pp {
// 
    type snark_pp=default_r1cs_ppzksnark_pp;
    type FieldT=ffec::Fr<default_r1cs_ppzksnark_pp>;
    type machine_pp=ram_fooram<FieldT>;

    fn  init_public_params() { snark_pp::init_public_params(); }
}





// 
pub fn  profile_ram_zksnark<ppT>(w:usize)
{
    // type ramT=ram_zksnark_machine_pp<ppT>;

    let  example=ram_example::<ramT>::new();
    example.ap = ram_architecture_params::<ramT>(w);
    example.boot_trace_size_bound = 0;
    example.time_bound = 10;
    let test_serialization = true;
    let bit = run_ram_zksnark::<ppT>(example, test_serialization);
    assert!(bit);
}

// 
pub fn profile_ram_ppzksnark<ppT>(w:usize)
{
    // type ramT=ram_ppzksnark_machine_pp<ppT>;

    let  example=ram_example::<ramT>::new();
    example.ap = ram_architecture_params::<ramT>(w);
    example.boot_trace_size_bound = 0;
    example.time_bound = 100;
    let test_serialization = true;
    let bit = run_ram_ppzksnark::<ppT>(example, test_serialization);
    assert!(bit);
}

fn main( argc:i32,argv:[&str])->i32
{
    // //ffec::UNUSED(argv);
    start_profiling();
    default_fooram_ppzksnark_pp::init_public_params();
    default_fooram_zksnark_pp::init_public_params();

    if argc == 1
    {
        profile_ram_zksnark::<default_fooram_zksnark_pp>(32);
    }
    else
    {
        profile_ram_ppzksnark::<default_fooram_ppzksnark_pp>(8);
    }
}
