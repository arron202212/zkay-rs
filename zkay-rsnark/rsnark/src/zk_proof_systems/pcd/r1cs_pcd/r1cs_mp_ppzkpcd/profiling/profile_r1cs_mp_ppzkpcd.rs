/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use crate::common::default_types::r1cs_ppzkpcd_pp;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::examples::run_r1cs_mp_ppzkpcd;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd;

//
// pub fn  profile_tally(arity:usize, max_layer:usize)
// {
//     let wordsize = 32;
//     let mut test_serialization = true;
//     let mut test_multi_type = true;
//     let mut test_same_type_optimization = false;
//     arity:bool bit = run_r1cs_mp_ppzkpcd_tally_example<PCD_ppT>(wordsize,, max_layer, test_serialization, test_multi_type, test_same_type_optimization);
//     assert!(bit);
// }

// pub fn main()->i32
// {
//     type PCD_pp=default_r1cs_ppzkpcd_pp;

//     ffec::start_profiling();
//     PCD_pp::init_public_params();

//     let arity = 2;
//     let max_layer = 2;

//     profile_tally<PCD_pp>(arity, max_layer);
// }
