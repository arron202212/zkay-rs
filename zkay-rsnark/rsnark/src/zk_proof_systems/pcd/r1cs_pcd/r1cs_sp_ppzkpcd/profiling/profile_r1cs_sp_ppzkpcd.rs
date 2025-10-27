/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use crate::common::default_types::r1cs_ppzkpcd_pp;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::examples::run_r1cs_sp_ppzkpcd;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd;



// 
// pub fn  profile_tally(arity:usize, max_layer:usize)
// {
//     let wordsize = 32;
//     let mut test_serialization = true;
//     arity:bool bit = run_r1cs_sp_ppzkpcd_tally_example<PCD_ppT>(wordsize,, max_layer, test_serialization);
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
