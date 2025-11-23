/** @file
 *****************************************************************************

 Template aliasing for prettifying R1CS PCD interfaces.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PPZKPCD_COMPLIANCE_PREDICATE_HPP_
// #define PPZKPCD_COMPLIANCE_PREDICATE_HPP_

use ff_curves::algebra::curves::public_params;

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;



/* template aliasing for R1CS (multi-predicate) ppzkPCD: */


type  r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> = r1cs_pcd_compliance_predicate<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_mp_ppzkpcd_message<PCD_ppT> = r1cs_pcd_message<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_mp_ppzkpcd_local_data<PCD_ppT> = r1cs_pcd_local_data<ffec::Fr< PCD_ppT::curve_A_pp> >;


type  r1cs_mp_ppzkpcd_variable_assignment<PCD_ppT> = r1cs_variable_assignment<ffec::Fr< PCD_ppT::curve_A_pp> >;

// }

//#endif // PPZKPCD_COMPLIANCE_PREDICATE_HPP_

