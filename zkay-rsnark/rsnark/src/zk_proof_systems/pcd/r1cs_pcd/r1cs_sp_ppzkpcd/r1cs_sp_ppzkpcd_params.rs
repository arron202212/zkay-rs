/** @file
 *****************************************************************************

 Parameters for *single-predicate* ppzkPCD for R1CS.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_SP_PPZKPCD_PARAMS_HPP_
// #define R1CS_SP_PPZKPCD_PARAMS_HPP_

use ff_curves::algebra::curves::public_params;

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params;




type  r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT> = r1cs_pcd_compliance_predicate<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_sp_ppzkpcd_message<PCD_ppT> = r1cs_pcd_message<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_sp_ppzkpcd_local_data<PCD_ppT> = r1cs_pcd_local_data<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_sp_ppzkpcd_primary_input<PCD_ppT> = r1cs_pcd_compliance_predicate_primary_input<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT> = r1cs_pcd_compliance_predicate_auxiliary_input<ffec::Fr< PCD_ppT::curve_A_pp> >;



//#endif // R1CS_SP_PPZKPCD_PARAMS_HPP_
