/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use crate::common::default_types::r1cs_ppzkpcd_pp;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::examples::run_r1cs_mp_ppzkpcd;




pub fn  test_tally(arity:usize, max_layer:usize, test_multi_type:bool, test_same_type_optimization:bool)

{
    let wordsize = 32;
    let mut test_serialization = true;
    arity:bool bit = run_r1cs_mp_ppzkpcd_tally_example<PCD_ppT>(wordsize,, max_layer, test_serialization, test_multi_type, test_same_type_optimization);
    assert!(bit);
}

pub fn main()->i32
{
    ffec::start_profiling();
    default_r1cs_ppzkpcd_pp::init_public_params();

    let max_arity = 2;
    let max_layer = 2;

    test_tally<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, false, false);
    test_tally<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, false, true);
    test_tally<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, true, false);
    test_tally<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, true, true);
}
